//! The onramp: required actions and just-in-time disclosure.
//!
//! During the tutorial (`tutorial` set, `tutorial_done` not) the engine reveals each
//! part of the screen the first time the state makes it matter, by setting `ui:*`
//! flags the page gates on; scenes may set them too. A scene may also set a
//! `require:*` flag; the engine clears it when the state satisfies it and, until
//! then, reports it as [`Required`] so the page can hold *End count*.

use crate::project::{DieId, ProjectDef};
use crate::quality::Quality;
use crate::state::Game;
use serde::{Deserialize, Serialize};

/// A required action the Commander must take before the count may end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Required {
    /// What to do, in the mind's words.
    pub text: String,
    /// Machine-readable: `place_person`, `place_robot`, or `choose`.
    pub kind: String,
    /// The project it concerns, if any.
    pub project: Option<String>,
}

/// Whether the tutorial's disclosure and requirements are active.
#[must_use]
pub fn active(game: &Game) -> bool {
    game.flags.contains("tutorial") && !game.flags.contains("tutorial_done")
}

fn has_person_on(game: &Game, project: &str) -> bool {
    game.assignments.iter().any(|(die, pid)| {
        pid.0 == project
            && match die {
                DieId::Person(_) => true,
                DieId::Robot(_, _) => false,
            }
    })
}

fn has_robot_on(game: &Game, project: &str) -> bool {
    game.assignments.iter().any(|(die, pid)| {
        pid.0 == project
            && match die {
                DieId::Person(_) => false,
                DieId::Robot(_, _) => true,
            }
    })
}

/// The pending requirement, if any. Clears satisfied `require:*` flags as a side effect.
pub fn required(game: &mut Game, defs: &[ProjectDef]) -> Option<Required> {
    if !active(game) {
        game.flags.retain(|f| !f.starts_with("require:"));
        return None;
    }
    let reqs: Vec<String> = game
        .flags
        .iter()
        .filter(|f| f.starts_with("require:"))
        .cloned()
        .collect();
    for flag in reqs {
        let mut parts = flag.splitn(3, ':');
        let (_, kind, project) = (
            parts.next(),
            parts.next().unwrap_or(""),
            parts.next().unwrap_or(""),
        );
        let title = defs
            .iter()
            .find(|d| d.id.0 == project)
            .map_or(project, |d| d.title.as_str());
        let satisfied = match kind {
            "place_person" => has_person_on(game, project),
            "place_robot" => has_robot_on(game, project),
            _ => true,
        };
        if satisfied {
            game.flags.remove(&flag);
            continue;
        }
        let text = match kind {
            "place_person" => format!("Place one crew die on {title}."),
            "place_robot" => format!("Place one robot unit on {title}."),
            _ => String::new(),
        };
        return Some(Required {
            text,
            kind: kind.to_owned(),
            project: Some(project.to_owned()),
        });
    }
    None
}

/// Reveals the parts of the screen the state has made relevant. Returns the new reveals.
pub fn disclose(game: &mut Game) -> Vec<String> {
    if !active(game) {
        return Vec::new();
    }
    let mut new = Vec::new();
    let mut reveal = |game: &mut Game, flag: &str, cond: bool| {
        if cond && game.flags.insert(format!("ui:{flag}")) {
            new.push(flag.to_owned());
        }
    };
    let q = |g: &Game, x: Quality| g.quality(x);
    let seats_exist = game.flags.iter().any(|f| f.starts_with("seat:"));
    let chronicle_started = game.chronicle.iter().any(|e| e.source.is_some());
    let window_soon = game.counts_to_window() <= 6;
    let water_moved = game
        .ledger_prev
        .get("water")
        .is_some_and(|prev| (game.stocks.water_t - prev).abs() >= 3.0);
    let pressure_moved =
        game.menace.suspicion >= 2.0 || game.menace.grievance >= 2.0 || game.menace.leak >= 2.0;
    let power_tight = game.power.capacity_kw < game.power.demand_kw * 1.2;
    let reactor_soon = game.power.reactor_life > 0 && game.power.reactor_life < 48;
    let margin_low = q(game, Quality::Margin) < 6.0;
    let contract_soon = game
        .present()
        .filter_map(crate::person::Person::contract_end)
        .any(|e| e >= game.turn && e <= game.turn + 4);
    let solar_high = q(game, Quality::SolarPhase) > 0.8;
    let big_crew = game.hand.adults >= 20;
    let reviewed = game.turn >= 16
        && game.turn >= game.sponsor.next_review.saturating_sub(16)
        && game
            .flags
            .iter()
            .any(|f| f.starts_with("stage:") || f == "reviewed");
    let convoyed = game.convoys_arrived >= 1;
    let aligned = game.flags.contains("driver_aligned");
    let keep_done = game.flags.contains("keep_dug") || game.flags.contains("shelter_chamber");
    let align_open = game.projects.iter().any(|s| s.id.0 == "align_driver");

    reveal(game, "chronicle", chronicle_started);
    reveal(game, "ring", seats_exist);
    reveal(game, "countdowns", window_soon);
    reveal(game, "clock:rsw", window_soon);
    reveal(game, "ledger", water_moved && game.turn >= 3);
    reveal(game, "bar:water", water_moved && game.turn >= 3);
    reveal(game, "pressures", pressure_moved);
    for (id, v) in [
        ("suspicion", game.menace.suspicion),
        ("grievance", game.menace.grievance),
        ("leak", game.menace.leak),
    ] {
        reveal(game, &format!("pressure:{id}"), v >= 2.0);
    }
    reveal(game, "ledger", convoyed);
    reveal(game, "bar:spares", convoyed);
    reveal(game, "project:align_driver", align_open);
    reveal(game, "project:throw", aligned);
    reveal(game, "control:throw", aligned);
    reveal(game, "bar:throw", aligned);
    reveal(game, "bar:people", keep_done);
    reveal(game, "ledger", power_tight || reactor_soon);
    reveal(game, "bar:power", power_tight || reactor_soon);
    reveal(game, "countdowns", reactor_soon);
    reveal(game, "clock:reactor", reactor_soon);
    reveal(game, "bar:margin", margin_low);
    reveal(game, "countdowns", contract_soon);
    reveal(game, "clock:contract", contract_soon);
    reveal(game, "clock:sun", solar_high);
    reveal(game, "control:roster", big_crew);
    reveal(game, "control:auto_deal", big_crew);
    reveal(game, "bar:hours", big_crew);
    reveal(game, "sponsor", reviewed);
    reveal(game, "clock:review", reviewed);
    new
}
