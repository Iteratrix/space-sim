//! The director: which storylets fire this count.

use crate::params::Params;
use crate::person::PersonId;
use crate::ring::{self, Counsel, Seat};
use crate::state::Game;
use crate::storylet::{Casting, Storylet};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A storylet ready to be presented: cast, with its available options and the ring's counsel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Firing {
    /// Storylet id.
    pub id: String,
    /// Title.
    pub title: String,
    /// Rendered situation text.
    pub text: String,
    /// Role to person.
    pub roles: BTreeMap<String, PersonId>,
    /// Option indices that are available, with their rendered labels and texts.
    pub options: Vec<FiringOption>,
    /// The ring's counsel.
    pub counsel: Vec<Counsel>,
}

/// One available option, rendered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiringOption {
    /// Index into the storylet's option list.
    pub index: usize,
    /// Option id.
    pub id: String,
    /// Label.
    pub label: String,
    /// Description.
    pub text: String,
    /// Tags.
    pub tags: Vec<String>,
}

fn tension(game: &Game, params: &Params) -> f64 {
    let year = (game.turn / 12) as usize;
    let curve = &params.director.tension_curve;
    curve
        .get(year)
        .or_else(|| curve.last())
        .copied()
        .unwrap_or(1.0)
}

/// Chooses up to `max_storylets_per_count` storylets for this count.
pub fn select(
    game: &Game,
    content: &[Storylet],
    params: &Params,
    rng: &mut impl Rng,
) -> Vec<Firing> {
    let seats = ring::seats(game);
    let mut eligible: Vec<(&Storylet, Casting)> = content
        .iter()
        .filter(|s| s.conditions_hold(game) && s.history_permits(game))
        .filter_map(|s| s.cast(game, &seats).map(|c| (s, c)))
        .filter(|(s, _)| s.available_options(game).next().is_some())
        .collect();
    eligible.sort_by(|a, b| {
        b.0.priority
            .cmp(&a.0.priority)
            .then_with(|| a.0.id.cmp(&b.0.id))
    });
    let mut chosen: Vec<(&Storylet, Casting)> = Vec::new();
    let max = params.director.max_storylets_per_count;
    let musts: Vec<usize> = eligible
        .iter()
        .enumerate()
        .filter(|(_, (s, _))| s.priority >= 100)
        .map(|(i, _)| i)
        .collect();
    for i in musts.into_iter().rev() {
        if chosen.len() < max {
            chosen.push(eligible.remove(i));
        }
    }
    let t = tension(game, params);
    let budget = if rng.random::<f64>() < 0.35 * t {
        max
    } else {
        1
    };
    while chosen.len() < budget && !eligible.is_empty() {
        let total: f64 = eligible.iter().map(|(s, _)| s.weight).sum();
        if total <= 0.0 {
            break;
        }
        let mut pick = rng.random::<f64>() * total;
        let mut idx = 0;
        for (i, (s, _)) in eligible.iter().enumerate() {
            pick -= s.weight;
            if pick <= 0.0 {
                idx = i;
                break;
            }
        }
        chosen.push(eligible.remove(idx));
    }
    chosen
        .into_iter()
        .map(|(s, casting)| {
            let available: Vec<(usize, &crate::storylet::Option_)> =
                s.available_options(game).collect();
            let counsel = ring::counsel(game, s, &available, &seats, rng);
            Firing {
                id: s.id.clone(),
                title: s.title.clone(),
                text: casting.render(game, &s.text),
                roles: casting.roles.clone(),
                options: available
                    .iter()
                    .map(|(i, o)| FiringOption {
                        index: *i,
                        id: o.id.clone(),
                        label: casting.render(game, &o.label),
                        text: casting.render(game, &o.text),
                        tags: o.tags.clone(),
                    })
                    .collect(),
                counsel,
            }
        })
        .collect()
}

/// Who holds each seat, by name, for display.
#[must_use]
pub fn ring_summary(game: &Game) -> Vec<(Seat, String)> {
    ring::seats(game)
        .into_iter()
        .map(|(seat, id)| (seat, game.person(id).name.clone()))
        .collect()
}
