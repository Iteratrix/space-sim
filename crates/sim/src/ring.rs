//! The advisor ring: a seat is a question; the holder is whoever the ring accepts the answer from.

use crate::person::{PersonId, Skill};
use crate::state::Game;
use crate::storylet::{Option_, Stance, Storylet};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A seat on the ring, keyed on the question it answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Seat {
    /// Who answers for the hulls.
    Hulls,
    /// Who answers for the hours.
    Hours,
    /// Who answers for the water and metal.
    Extraction,
    /// Who answers for the bodies.
    Bodies,
    /// Who answers for the air and the farm.
    Air,
    /// Who answers for the machines (act 2; the Act forbids it in act 1).
    Machines,
    /// Who answers for the children.
    Children,
    /// Who answers to the sponsor: the liaison, a count late.
    Liaison,
}

impl Seat {
    /// Every seat.
    pub const ALL: [Self; 8] = [
        Self::Hulls,
        Self::Hours,
        Self::Extraction,
        Self::Bodies,
        Self::Air,
        Self::Machines,
        Self::Children,
        Self::Liaison,
    ];

    /// Data-file key.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Hulls => "hulls",
            Self::Hours => "hours",
            Self::Extraction => "extraction",
            Self::Bodies => "bodies",
            Self::Air => "air",
            Self::Machines => "machines",
            Self::Children => "children",
            Self::Liaison => "liaison",
        }
    }

    /// The question, as the chronicle names it.
    #[must_use]
    pub const fn question(self) -> &'static str {
        match self {
            Self::Hulls => "who answers for the hulls",
            Self::Hours => "who answers for the hours",
            Self::Extraction => "who answers for the water and metal",
            Self::Bodies => "who answers for the bodies",
            Self::Air => "who answers for the air and the farm",
            Self::Machines => "who answers for the machines",
            Self::Children => "who answers for the children",
            Self::Liaison => "who answers to the sponsor",
        }
    }

    /// The act-1 office title.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Hulls => "head of engineering",
            Self::Hours => "head of logistics",
            Self::Extraction => "head of extraction",
            Self::Bodies => "medical officer",
            Self::Air => "life-support lead",
            Self::Machines => "operator",
            Self::Children => "keep-mother",
            Self::Liaison => "sponsor liaison",
        }
    }

    /// The skill that qualifies a holder.
    #[must_use]
    pub const fn skill(self) -> Skill {
        match self {
            Self::Hulls => Skill::Engineering,
            Self::Hours | Self::Liaison => Skill::Logistics,
            Self::Extraction => Skill::Extraction,
            Self::Bodies => Skill::Medical,
            Self::Air => Skill::LifeSupport,
            Self::Machines => Skill::Operations,
            Self::Children => Skill::Social,
        }
    }

    /// Parses a data-file key.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.key() == s)
    }

    /// Whether the seat exists in the given act and state.
    #[must_use]
    pub fn exists(self, game: &Game) -> bool {
        match self {
            Self::Hulls | Self::Hours | Self::Extraction | Self::Bodies | Self::Air => true,
            Self::Machines => game.act >= 2 || game.flags.contains("machines_seat"),
            Self::Children => game.quality(crate::quality::Quality::Children) > 0.0,
            Self::Liaison => game.act == 1 && game.sponsor.attention > 0.05,
        }
    }

    /// How strongly this seat favours each option tag. Positive is for, negative against.
    #[must_use]
    pub fn bias(self, tag: &str) -> f64 {
        let table: &[(&str, f64)] = match self {
            Self::Hulls => &[
                ("capability", 0.6),
                ("hulls", 0.8),
                ("risk", -0.5),
                ("throughput", 0.1),
                ("people", 0.1),
                ("sponsor", -0.2),
                ("spares", 0.7),
            ],
            Self::Hours => &[
                ("throughput", 0.7),
                ("hours", 0.8),
                ("cost", -0.8),
                ("capability", 0.2),
                ("risk", -0.3),
                ("sponsor", 0.3),
            ],
            Self::Extraction => &[
                ("throughput", 0.8),
                ("water", 0.7),
                ("risk", 0.1),
                ("capability", 0.3),
                ("people", -0.1),
            ],
            Self::Bodies => &[
                ("people", 0.8),
                ("dose", -0.9),
                ("risk", -0.7),
                ("throughput", -0.3),
                ("medicine", 0.8),
                ("families", 0.3),
            ],
            Self::Air => &[
                ("air", 0.9),
                ("nitrogen", 0.9),
                ("capability", 0.5),
                ("risk", -0.5),
                ("closure", 0.8),
                ("throughput", -0.2),
            ],
            Self::Machines => &[
                ("machines", 0.9),
                ("sponsor", -0.6),
                ("risk", -0.2),
                ("capability", 0.3),
            ],
            Self::Children => &[
                ("families", 0.9),
                ("people", 0.6),
                ("dose", -0.8),
                ("risk", -0.6),
                ("sponsor", -0.3),
            ],
            Self::Liaison => &[
                ("sponsor", 0.9),
                ("throughput", 0.7),
                ("capability", -0.5),
                ("machines", -0.6),
                ("families", -0.6),
                ("independence", -0.9),
                ("cost", -0.5),
            ],
        };
        table
            .iter()
            .find(|(t, _)| *t == tag)
            .map_or(0.0, |(_, b)| *b)
    }
}

/// Who holds each seat this count.
#[must_use]
pub fn seats(game: &Game) -> BTreeMap<Seat, PersonId> {
    let mut out = BTreeMap::new();
    let mut taken: Vec<PersonId> = Vec::new();
    for seat in Seat::ALL {
        if !seat.exists(game) {
            continue;
        }
        let best = game
            .present()
            .filter(|p| !taken.contains(&p.id) && p.age_counts(game.turn) >= 18 * 12)
            .max_by_key(|p| (p.skill(seat.skill()), std::cmp::Reverse(p.id)));
        if let Some(p) = best {
            taken.push(p.id);
            out.insert(seat, p.id);
        }
    }
    out
}

/// One piece of advice, rendered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counsel {
    /// Seat.
    pub seat: Seat,
    /// Holder's name.
    pub holder: String,
    /// Which option index they favour.
    pub favours: usize,
    /// For or against as text.
    pub text: String,
    /// Whether the advice is authored (true) or generated from biases (false).
    pub authored: bool,
}

/// The ring's advice on a storylet's available options.
pub fn counsel(
    game: &Game,
    storylet: &Storylet,
    available: &[(usize, &Option_)],
    seats: &BTreeMap<Seat, PersonId>,
    rng: &mut impl Rng,
) -> Vec<Counsel> {
    let mut out = Vec::new();
    for (&seat, &holder) in seats {
        let person = game.person(holder);
        let skill = person.skill(seat.skill());
        let scored: Vec<(usize, f64)> = available
            .iter()
            .map(|(i, o)| (*i, o.tags.iter().map(|t| seat.bias(t)).sum::<f64>()))
            .collect();
        let Some(&(mut best, _)) = scored.iter().max_by(|a, b| a.1.total_cmp(&b.1)) else {
            continue;
        };
        let wrong_chance = match skill {
            0 => 0.5,
            1 => 0.35,
            2 => 0.2,
            3 => 0.1,
            _ => 0.03,
        };
        let authored_for = available
            .iter()
            .find(|(i, o)| {
                o.advice
                    .iter()
                    .any(|a| a.seat == seat && a.stance == Stance::For)
                    && *i != usize::MAX
            })
            .map(|(i, _)| *i);
        if let Some(i) = authored_for {
            best = i;
        }
        if scored.len() > 1 && rng.random::<f64>() < wrong_chance {
            let others: Vec<usize> = scored
                .iter()
                .map(|(i, _)| *i)
                .filter(|i| *i != best)
                .collect();
            best = others[rng.random_range(0..others.len())];
        }
        let authored = storylet.options[best]
            .advice
            .iter()
            .find(|a| a.seat == seat && a.stance == Stance::For)
            .map(|a| a.text.clone());
        let warning = available
            .iter()
            .filter(|(i, _)| *i != best)
            .find_map(|(_, o)| {
                o.advice
                    .iter()
                    .find(|a| a.seat == seat && a.stance == Stance::Against)
                    .map(|a| format!(" Against \"{}\": {}", o.label, a.text))
            })
            .unwrap_or_default();
        let (text, authored) = authored.map_or_else(
            || {
                (
                    format!("Leans toward \"{}\".", storylet.options[best].label) + &warning,
                    !warning.is_empty(),
                )
            },
            |t| (format!("{t}{warning}"), true),
        );
        out.push(Counsel {
            seat,
            holder: person.name.clone(),
            favours: best,
            text,
            authored,
        });
    }
    out
}
