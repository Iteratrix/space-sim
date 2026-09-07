//! Headless, deterministic settlement simulation.
//!
//! The [`Engine`] owns parameters, content, and the body catalogue; a [`Game`] is pure
//! state. One count is: [`Engine::advance`] (physics, people, sponsor, convoy, then the
//! director's choice of storylets), then one [`Engine::resolve`] per firing.

pub mod content;
pub mod director;
pub mod lexicon;
pub mod names;
pub mod params;
pub mod person;
pub mod quality;
pub mod ring;
pub mod setup;
pub mod state;
pub mod storylet;
pub mod turn;

pub use director::Firing;
pub use params::Params;
pub use quality::Quality;
pub use state::{Ending, Game};
pub use storylet::Storylet;

/// Everything needed to run games: parameters, content, and the map.
#[derive(Debug)]
pub struct Engine {
    /// Tunables.
    pub params: Params,
    /// Storylets.
    pub content: Vec<Storylet>,
    /// Bodies.
    pub catalogue: orbit::Catalogue,
    /// The Earth edge, precomputed once for every game.
    pub calendar: state::Calendar,
}

/// Errors from building an engine.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// Parameters.
    #[error("params: {0}")]
    Params(#[from] toml::de::Error),
    /// Content.
    #[error(transparent)]
    Content(#[from] storylet::ContentError),
    /// Catalogue.
    #[error(transparent)]
    Catalogue(#[from] orbit::CatalogueError),
    /// Setup.
    #[error(transparent)]
    Setup(#[from] setup::SetupError),
}

impl Engine {
    /// The engine with everything bundled into the binary.
    pub fn bundled() -> Result<Self, EngineError> {
        let params = Params::bundled()?;
        let catalogue = orbit::bundled_catalogue()?;
        let calendar = setup::calendar(&params, &catalogue)?;
        Ok(Self {
            params,
            content: content::bundled()?,
            catalogue,
            calendar,
        })
    }

    /// A new game from a seed.
    pub fn new_game(&self, seed: u64) -> Result<Game, EngineError> {
        Ok(setup::new_game(&self.params, &self.calendar, seed))
    }

    /// Advances one count and returns the storylets that fire.
    pub fn advance(&self, game: &mut Game) -> (turn::Events, Vec<Firing>) {
        if game.ending.is_some() {
            return (turn::Events::default(), Vec::new());
        }
        let mut rng = setup::rng_for(game, 0);
        let events = turn::advance(game, &self.params, &mut rng);
        if game.ending.is_some() {
            return (events, Vec::new());
        }
        let mut rng = setup::rng_for(game, 1);
        let firings = director::select(game, &self.content, &self.params, &mut rng);
        (events, firings)
    }

    /// Resolves a firing with the chosen option index (an index into `firing.options`).
    ///
    /// Returns the rendered chronicle line, or `None` if the id or choice is invalid.
    pub fn resolve(&self, game: &mut Game, firing: &Firing, choice: usize) -> Option<String> {
        let storylet = self.content.iter().find(|s| s.id == firing.id)?;
        let opt = firing.options.get(choice)?;
        let option = storylet.options.get(opt.index)?;
        let casting = storylet::Casting {
            roles: firing.roles.clone(),
        };
        storylet::apply(game, storylet, option, &casting);
        game.chronicle
            .last()
            .map(|e| lexicon::render(game, &e.text))
    }

    /// Renders the chronicle through the current lexicon.
    #[must_use]
    pub fn chronicle(&self, game: &Game) -> Vec<String> {
        game.chronicle
            .iter()
            .map(|e| format!("[{}] {}", e.turn, lexicon::render(game, &e.text)))
            .collect()
    }
}
