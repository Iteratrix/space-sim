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
pub mod project;
pub mod quality;
pub mod report;
pub mod ring;
pub mod setup;
pub mod state;
pub mod storylet;
pub mod turn;
pub mod view;

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
    /// Project definitions.
    pub projects: Vec<project::ProjectDef>,
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
            projects: project::bundled()?,
            catalogue,
            calendar,
        })
    }

    /// A new act-1 game from a seed.
    pub fn new_game(&self, seed: u64) -> Result<Game, EngineError> {
        Ok(setup::new_game(&self.params, &self.calendar, seed))
    }

    /// A new game from a seed in a scenario.
    pub fn new_game_scenario(
        &self,
        seed: u64,
        scenario: setup::Scenario,
    ) -> Result<Game, EngineError> {
        Ok(setup::new_game_scenario(
            &self.params,
            &self.calendar,
            seed,
            scenario,
        ))
    }

    /// Advances one count and returns the storylets that fire.
    pub fn advance(&self, game: &mut Game) -> (turn::Events, Vec<Firing>) {
        if game.ending.is_some() {
            return (turn::Events::default(), Vec::new());
        }
        let mut rng = setup::rng_for(game, 0);
        let events = turn::advance(game, &self.params, &self.projects, &mut rng);
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
        game.chronicle.last().map(lexicon::render_entry)
    }

    /// The structured view a front end renders.
    #[must_use]
    pub fn view(&self, game: &Game) -> view::View {
        view::view(game, &self.projects, &self.params)
    }

    /// Moves a die onto a project, or back to the hand (`target == "hand"`).
    pub fn assign(&self, game: &mut Game, die: &str, target: &str) -> Result<(), String> {
        project::assign(game, &self.projects, die, target)
    }

    /// Sets a standing control: `manifest`, `throw`, `roster`, or `auto_deal`.
    pub fn set_control(&self, game: &mut Game, control: &str, value: &str) -> Result<(), String> {
        use project::{ManifestSplit, RosterOrder, ThrowMode};
        match (control, value) {
            ("manifest", "throughput") => game.controls.manifest = ManifestSplit::Throughput,
            ("manifest", "balanced") => game.controls.manifest = ManifestSplit::Balanced,
            ("manifest", "capability") => game.controls.manifest = ManifestSplit::Capability,
            ("manifest", "people") => game.controls.manifest = ManifestSplit::People,
            ("throw", "ship") => game.controls.throw = ThrowMode::Ship,
            ("throw", "hold") => game.controls.throw = ThrowMode::HoldAtReserve,
            ("throw", "stop") => game.controls.throw = ThrowMode::Stop,
            ("roster", "skill") => game.controls.roster = RosterOrder::Skill,
            ("roster", "strain") => game.controls.roster = RosterOrder::Strain,
            ("roster", "name") => game.controls.roster = RosterOrder::Name,
            ("auto_deal", "on") => game.controls.auto_deal = true,
            ("auto_deal", "off") => game.controls.auto_deal = false,
            ("flag", f) if f == "tutorial_done" || f == "tutorial_open" || f.starts_with("ui:") => {
                game.flags.insert(f.to_owned());
            }
            _ => return Err(format!("unknown control or value: {control} = {value}")),
        }
        Ok(())
    }

    /// Resolves a firing by option id rather than index; the safer call for agents.
    pub fn resolve_by_id(
        &self,
        game: &mut Game,
        firing: &Firing,
        option_id: &str,
    ) -> Option<String> {
        let choice = firing.options.iter().position(|o| o.id == option_id)?;
        self.resolve(game, firing, choice)
    }

    /// Renders the chronicle through the current lexicon.
    #[must_use]
    pub fn chronicle(&self, game: &Game) -> Vec<String> {
        game.chronicle
            .iter()
            .map(|e| format!("[{}] {}", e.turn, lexicon::render_entry(e)))
            .collect()
    }
}
