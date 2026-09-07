//! WASM bridge: JSON in, JSON out, state held in the wasm.
//!
//! Games live in a table keyed by handle. Every call returns a JSON string the page
//! parses; errors come back as `{"error": "..."}`.

use serde::Serialize;
use sim::Engine;
use std::cell::RefCell;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
    static GAMES: RefCell<BTreeMap<u32, sim::Game>> = const { RefCell::new(BTreeMap::new()) };
    static PENDING: RefCell<BTreeMap<u32, Vec<sim::Firing>>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT: RefCell<u32> = const { RefCell::new(1) };
}

#[derive(Serialize)]
struct Err {
    error: String,
}

fn err(msg: impl Into<String>) -> String {
    serde_json::to_string(&Err { error: msg.into() }).unwrap_or_default()
}

fn json<T: Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_else(|e| err(e.to_string()))
}

fn with_engine<R>(f: impl FnOnce(&Engine) -> R) -> Result<R, String> {
    ENGINE.with(|cell| {
        let mut e = cell.borrow_mut();
        if e.is_none() {
            *e = Some(Engine::bundled().map_err(|x| x.to_string())?);
        }
        Ok(f(e.as_ref().expect("engine set")))
    })
}

fn with_game<R>(handle: u32, f: impl FnOnce(&Engine, &mut sim::Game) -> R) -> Result<R, String> {
    with_engine(|engine| {
        GAMES.with(|g| {
            let mut games = g.borrow_mut();
            let game = games
                .get_mut(&handle)
                .ok_or_else(|| format!("no game {handle}"))?;
            Ok(f(engine, game))
        })
    })?
}

fn store(game: sim::Game) -> u32 {
    let handle = NEXT.with(|n| {
        let mut n = n.borrow_mut();
        let h = *n;
        *n += 1;
        h
    });
    GAMES.with(|g| g.borrow_mut().insert(handle, game));
    handle
}

/// Starts a game. `scenario` is `tutorial` or `act1`. Returns `{"handle": n}`.
#[wasm_bindgen]
#[must_use]
pub fn new_game(seed: u64, scenario: &str) -> String {
    let scenario = sim::setup::Scenario::parse(scenario).unwrap_or(sim::setup::Scenario::Tutorial);
    match with_engine(|e| {
        e.new_game_scenario(seed, scenario)
            .map_err(|x| x.to_string())
    }) {
        Ok(Ok(game)) => json(&serde_json::json!({ "handle": store(game) })),
        Ok(Err(e)) | Err(e) => err(e),
    }
}

#[derive(Serialize)]
struct Advanced<'a> {
    events: &'a [String],
    firings: &'a [sim::Firing],
    ending: &'a Option<sim::Ending>,
    view: sim::view::View,
}

/// Advances one count. Returns events, the scenes to resolve, the ending, and the view.
#[wasm_bindgen]
#[must_use]
pub fn advance(handle: u32) -> String {
    let result = with_game(handle, |engine, game| {
        let (events, firings) = engine.advance(game);
        let view = engine.view(game);
        let out = json(&Advanced {
            events: &events.lines,
            firings: &firings,
            ending: &game.ending,
            view,
        });
        PENDING.with(|p| p.borrow_mut().insert(handle, firings));
        out
    });
    result.unwrap_or_else(err)
}

/// Resolves a pending scene by option id. Returns `{"chronicle": "..."}`.
#[wasm_bindgen]
#[must_use]
pub fn resolve(handle: u32, firing_id: &str, option_id: &str) -> String {
    let firing = PENDING.with(|p| {
        p.borrow()
            .get(&handle)
            .and_then(|fs| fs.iter().find(|f| f.id == firing_id).cloned())
    });
    let Some(firing) = firing else {
        return err(format!("no pending scene {firing_id}"));
    };
    let result = with_game(handle, |engine, game| {
        engine.resolve_by_id(game, &firing, option_id).map_or_else(
            || err("no such option"),
            |line| json(&serde_json::json!({ "chronicle": line })),
        )
    });
    PENDING.with(|p| {
        if let Some(fs) = p.borrow_mut().get_mut(&handle) {
            fs.retain(|f| f.id != firing_id);
        }
    });
    result.unwrap_or_else(err)
}

/// Moves a die to a project or the hand. Returns the view.
#[wasm_bindgen]
#[must_use]
pub fn assign(handle: u32, die: &str, target: &str) -> String {
    let result = with_game(handle, |engine, game| {
        match engine.assign(game, die, target) {
            Ok(()) => json(&engine.view(game)),
            Err(e) => err(e),
        }
    });
    result.unwrap_or_else(err)
}

/// Sets a standing control. Returns the view.
#[wasm_bindgen]
#[must_use]
pub fn set(handle: u32, control: &str, value: &str) -> String {
    let result = with_game(handle, |engine, game| {
        match engine.set_control(game, control, value) {
            Ok(()) => json(&engine.view(game)),
            Err(e) => err(e),
        }
    });
    result.unwrap_or_else(err)
}

/// The view.
#[wasm_bindgen]
#[must_use]
pub fn view(handle: u32) -> String {
    with_game(handle, |engine, game| json(&engine.view(game))).unwrap_or_else(err)
}

/// The full chronicle, rendered.
#[wasm_bindgen]
#[must_use]
pub fn chronicle(handle: u32) -> String {
    with_game(handle, |engine, game| json(&engine.chronicle(game))).unwrap_or_else(err)
}

/// The polity summary as text.
#[wasm_bindgen]
#[must_use]
pub fn summary(handle: u32) -> String {
    with_game(handle, |_, game| json(&sim::report::summary(game))).unwrap_or_else(err)
}

/// Saves the game as JSON.
#[wasm_bindgen]
#[must_use]
pub fn save(handle: u32) -> String {
    with_game(handle, |_, game| json(game)).unwrap_or_else(err)
}

/// Loads a saved game. Returns `{"handle": n}`.
#[wasm_bindgen]
#[must_use]
pub fn load(saved: &str) -> String {
    match serde_json::from_str::<sim::Game>(saved) {
        Ok(game) => json(&serde_json::json!({ "handle": store(game) })),
        Err(e) => err(e.to_string()),
    }
}
