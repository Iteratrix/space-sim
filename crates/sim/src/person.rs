//! Persons, their traits, and the directed ties between them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Index of a person in the roster. Stable for the life of a game; the dead keep theirs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PersonId(pub u32);

/// Where a person was born; the culture clock counts birthplaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Birthplace {
    /// Earth.
    Earth,
    /// Mars.
    Mars,
    /// Cislunar or orbital habitats.
    Orbit,
    /// Born in the belt.
    Belt,
}

/// Which estate a person lives in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Estate {
    /// In the Keep, behind rock, at 0.5 g.
    Kept,
    /// Aboard a coil hull.
    Bore,
    /// Aboard a skiff or tug.
    Skiff,
}

/// Contract status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tenure {
    /// On a fixed contract, expecting a ship home at the given count.
    Rotator {
        /// Count at which the contract ends.
        ends: u32,
    },
    /// Intends to stay.
    Resident,
}

/// A skill domain; the ring's seats are keyed on these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Skill {
    /// Hulls, coils, the drum, the driver.
    Engineering,
    /// Hours, manifests, the lay.
    Logistics,
    /// Water, metal, the bake-out.
    Extraction,
    /// The bodies: dose, bones, the formulary.
    Medical,
    /// Air, water loop, the farm.
    LifeSupport,
    /// Trajectories, windows, the reckoning.
    Navigation,
    /// The farm and the biology.
    Agronomy,
    /// Disputes, morale, the moot.
    Social,
    /// The minds and the robots.
    Operations,
}

impl Skill {
    /// Every skill, in a fixed order.
    pub const ALL: [Self; 9] = [
        Self::Engineering,
        Self::Logistics,
        Self::Extraction,
        Self::Medical,
        Self::LifeSupport,
        Self::Navigation,
        Self::Agronomy,
        Self::Social,
        Self::Operations,
    ];

    /// The lower-case identifier used in data files.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Engineering => "engineering",
            Self::Logistics => "logistics",
            Self::Extraction => "extraction",
            Self::Medical => "medical",
            Self::LifeSupport => "life_support",
            Self::Navigation => "navigation",
            Self::Agronomy => "agronomy",
            Self::Social => "social",
            Self::Operations => "operations",
        }
    }

    /// Parses a data-file identifier.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.key() == s)
    }
}

/// Stable personality traits, each in `[0, 1]`. Drawn once; never change.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Traits {
    /// Tendency to strain under load.
    pub neuroticism: f64,
    /// Wants to lead; clashes with other dominants.
    pub dominance: f64,
    /// Warmth and expressiveness versus task focus.
    pub expressivity: f64,
    /// Need for affection and inclusion.
    pub affection: f64,
    /// Tendency to escalate rather than withdraw in conflict.
    pub escalation: f64,
    /// Attachment to Earth-normal ways.
    pub baseline_attachment: f64,
}

/// Dynamic state, changes every count.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Condition {
    /// Accumulated psychological strain, `[0, 1]`.
    pub strain: f64,
    /// Boredom, `[0, 1]`.
    pub boredom: f64,
    /// Cumulative dose in sieverts.
    pub dose_sv: f64,
    /// Whether cataracts have set in.
    pub cataracts: bool,
    /// Intent to go home when the ship comes, `[0, 1]`.
    pub return_intent: f64,
}

/// A person.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    /// Identifier.
    pub id: PersonId,
    /// Given name.
    pub name: String,
    /// Birthplace.
    pub birthplace: Birthplace,
    /// Count of birth (negative for those born before the outpost).
    pub born: i64,
    /// Estate.
    pub estate: Estate,
    /// Count at which the person entered their current estate.
    #[serde(default)]
    pub estate_since: u32,
    /// Contract.
    pub tenure: Tenure,
    /// Skills, 0-5.
    #[serde(with = "skill_map")]
    pub skills: BTreeMap<Skill, u8>,
    /// Personality.
    pub traits: Traits,
    /// Present condition.
    pub condition: Condition,
    /// Alive.
    pub alive: bool,
    /// Present at the outpost (false once shipped home).
    pub present: bool,
}

impl Person {
    /// Skill level in a domain, 0 if untrained.
    #[must_use]
    pub fn skill(&self, skill: Skill) -> u8 {
        self.skills.get(&skill).copied().unwrap_or(0)
    }

    /// Age in counts at the given count.
    #[must_use]
    pub fn age_counts(&self, now: u32) -> i64 {
        i64::from(now) - self.born
    }

    /// Whether this person is a rotator with a contract end.
    #[must_use]
    pub const fn contract_end(&self) -> Option<u32> {
        match self.tenure {
            Tenure::Rotator { ends } => Some(ends),
            Tenure::Resident => None,
        }
    }
}

/// A directed tie from one person to another.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Tie {
    /// Positive working relationship, `[0, 1]`.
    pub work_positive: f64,
    /// Perceived hindrance, `[0, 1]`.
    pub hindrance: f64,
    /// Would want this person on the next voyage, `[0, 1]`.
    pub viability: f64,
    /// Relies on this person's lead, `[0, 1]`.
    pub reliance: f64,
}

/// The sparse tie graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(from = "TieList", into = "TieList")]
pub struct Ties {
    edges: BTreeMap<(PersonId, PersonId), Tie>,
}

#[derive(Serialize, Deserialize)]
struct TieList(Vec<(PersonId, PersonId, Tie)>);

impl From<TieList> for Ties {
    fn from(TieList(list): TieList) -> Self {
        Self {
            edges: list.into_iter().map(|(a, b, t)| ((a, b), t)).collect(),
        }
    }
}

impl From<Ties> for TieList {
    fn from(ties: Ties) -> Self {
        Self(
            ties.edges
                .into_iter()
                .map(|((a, b), t)| (a, b, t))
                .collect(),
        )
    }
}

impl Ties {
    /// The tie from `a` to `b`, default if none recorded.
    #[must_use]
    pub fn get(&self, a: PersonId, b: PersonId) -> Tie {
        self.edges.get(&(a, b)).copied().unwrap_or_default()
    }

    /// Mutable tie from `a` to `b`, created if absent.
    pub fn get_mut(&mut self, a: PersonId, b: PersonId) -> &mut Tie {
        self.edges.entry((a, b)).or_default()
    }

    /// All recorded ties.
    pub fn iter(&self) -> impl Iterator<Item = (PersonId, PersonId, &Tie)> {
        self.edges.iter().map(|(&(a, b), t)| (a, b, t))
    }

    /// Mutable iteration over all recorded ties.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PersonId, PersonId, &mut Tie)> {
        self.edges.iter_mut().map(|(&(a, b), t)| (a, b, t))
    }

    /// Number of recorded ties.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Whether no ties are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

/// Derived group indices, computed fresh each count and never stored as state.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupIndices {
    /// Mean strain over present persons.
    pub mean_strain: f64,
    /// Share of hindrance mass concentrated on the two most-hindering persons.
    pub conflict_concentration: f64,
    /// Coherence: 1 = core-periphery, 0 = two factions of equal size.
    pub coherence: f64,
    /// Share of persons intending to leave.
    pub return_share: f64,
    /// Share of present adults born in the belt or orbit.
    pub belt_born_share: f64,
}

mod skill_map {
    use super::Skill;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    pub fn serialize<S: Serializer>(map: &BTreeMap<Skill, u8>, ser: S) -> Result<S::Ok, S::Error> {
        let out: BTreeMap<&str, u8> = map.iter().map(|(k, v)| (k.key(), *v)).collect();
        out.serialize(ser)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<BTreeMap<Skill, u8>, D::Error> {
        let raw: BTreeMap<String, u8> = BTreeMap::deserialize(de)?;
        raw.into_iter()
            .map(|(k, v)| {
                Skill::parse(&k)
                    .map(|s| (s, v))
                    .ok_or_else(|| serde::de::Error::custom(format!("unknown skill {k}")))
            })
            .collect()
    }
}
