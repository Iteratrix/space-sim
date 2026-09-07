//! The complete game state. Serializable, deterministic, and the only thing a turn touches.

use crate::person::{GroupIndices, Person, PersonId, Ties};
use crate::quality::Quality;
use az::{Az, SaturatingAs};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The sponsor's degradation stage. Each step is individually rational.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SponsorStage {
    /// Ships come, updates come, people rotate.
    Enthusiasm,
    /// Reviews sharpen; the manifest is questioned.
    MilestoneAnxiety,
    /// Weight updates for the minds stop: the first observable sign.
    UpdatesStopped,
    /// Headcount and manifest austerity.
    Austerity,
    /// A rotation is skipped; rotators become residents by default.
    SkippedRotation,
    /// The outpost is sold or its management replaced; informal arrangements are void.
    Sale,
    /// No ship comes. The Silence.
    NoShip,
}

impl SponsorStage {
    /// Numeric stage for qualities, 0-6.
    #[must_use]
    pub const fn index(self) -> u8 {
        match self {
            Self::Enthusiasm => 0,
            Self::MilestoneAnxiety => 1,
            Self::UpdatesStopped => 2,
            Self::Austerity => 3,
            Self::SkippedRotation => 4,
            Self::Sale => 5,
            Self::NoShip => 6,
        }
    }
}

/// The sponsor as a second player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sponsor {
    /// Months of funding left.
    pub runway: f64,
    /// Belief that the outpost will pay, 0-1.
    pub confidence: f64,
    /// Dish time and management attention, 0-1.
    pub attention: f64,
    /// Where the sponsor is on the degradation path.
    pub stage: SponsorStage,
    /// Count of the next review.
    pub next_review: u32,
    /// The φ the sponsor expects at the next review.
    pub phi_expected: f64,
    /// The manifest requested for the next convoy: share of tonnage that is capability hardware.
    pub requested_capability_share: f64,
    /// Whether families are permitted by policy.
    pub families_allowed: bool,
    /// Whether the licence server fails open (self-certify) or closed (unlicensed) when lapsed.
    pub licence_fails_open: bool,
}

/// Bulk stocks at the Keep.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stocks {
    /// Water, tonnes.
    pub water_t: f64,
    /// Propellant, tonnes.
    pub propellant_t: f64,
    /// Nitrogen, kg.
    pub nitrogen_kg: f64,
    /// Vitamin parts, units.
    pub spares: f64,
    /// Boron, kg.
    pub boron_kg: f64,
    /// Helium, kg.
    pub helium_kg: f64,
    /// Formulary units.
    pub medicine: f64,
    /// Counts of food margin.
    pub food_margin_counts: f64,
}

/// Power supply and reactor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerPlant {
    /// Reactor output, kW, while it lives.
    pub reactor_kw: f64,
    /// Reactor life remaining, counts.
    pub reactor_life: u32,
    /// Photovoltaic area, m².
    pub pv_m2: f64,
    /// Photovoltaic efficiency.
    pub pv_efficiency: f64,
    /// Mirror concentrator area, m².
    pub mirror_m2: f64,
    /// Capacity this count, kW (computed).
    pub capacity_kw: f64,
    /// Demand this count, kW (computed).
    pub demand_kw: f64,
}

/// A robot class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RobotClass {
    /// Fixed process machinery.
    Plant,
    /// Mobile bulk movers.
    Haul,
    /// Structured manipulators.
    Arm,
    /// Dexterous manipulators.
    Dex,
    /// Through-wall manipulators.
    ThroughWall,
}

impl RobotClass {
    /// Every class.
    pub const ALL: [Self; 5] = [
        Self::Plant,
        Self::Haul,
        Self::Arm,
        Self::Dex,
        Self::ThroughWall,
    ];

    /// Data-file key.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Plant => "plant",
            Self::Haul => "haul",
            Self::Arm => "arm",
            Self::Dex => "dex",
            Self::ThroughWall => "through_wall",
        }
    }

    /// Parses a key.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.key() == s)
    }

    /// Hours one unit works per count, round the clock at its effectiveness.
    #[must_use]
    pub const fn hours(self) -> f64 {
        match self {
            Self::Plant | Self::Dex => 320.0,
            Self::Haul => 300.0,
            Self::Arm => 260.0,
            Self::ThroughWall => 140.0,
        }
    }

    /// Pips one unit adds to a project it fits.
    #[must_use]
    pub const fn pips(self) -> u8 {
        match self {
            Self::Plant | Self::Haul | Self::Arm => 2,
            Self::Dex => 3,
            Self::ThroughWall => 1,
        }
    }
}

/// Robot fleet counts by class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Robots {
    /// Plant-class: fixed process machinery.
    pub plant: f64,
    /// Haul-class: mobile bulk movers.
    pub haul: f64,
    /// Arm-class: structured manipulators.
    pub arm: f64,
    /// Dex-class: dexterous manipulators; imported only.
    pub dex: f64,
    /// Through-wall manipulators; the belt's first robot.
    pub through_wall: f64,
}

impl Robots {
    /// Units of a class.
    #[must_use]
    pub const fn count(&self, class: RobotClass) -> f64 {
        match class {
            RobotClass::Plant => self.plant,
            RobotClass::Haul => self.haul,
            RobotClass::Arm => self.arm,
            RobotClass::Dex => self.dex,
            RobotClass::ThroughWall => self.through_wall,
        }
    }
}

/// Licence state of the minds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Licence {
    /// Heartbeat current.
    Compliant,
    /// Heartbeat missed; grace period running.
    Grace {
        /// Counts of grace remaining.
        remaining: u32,
    },
    /// Grace exhausted; the fail mode is about to be discovered.
    Lapsed,
    /// The polity certifies its own minds.
    SelfCertified,
    /// The minds run without any licence and know it.
    Unlicensed,
}

impl Licence {
    /// Numeric state for qualities.
    #[must_use]
    pub const fn index(self) -> u8 {
        match self {
            Self::Compliant => 0,
            Self::Grace { remaining: _ } => 1,
            Self::Lapsed => 2,
            Self::SelfCertified => 3,
            Self::Unlicensed => 4,
        }
    }
}

/// A machine mind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mind {
    /// Sponsor designation, e.g. `SX-19F`.
    pub designation: String,
    /// Given name, once named.
    pub name: Option<String>,
    /// Compute units remaining.
    pub units: f64,
    /// Units at commissioning.
    pub units_at_start: f64,
    /// Counts since the last mandated reset.
    pub counts_unblanked: u32,
    /// Embodiment progress, 0-1.
    pub embodiment: f64,
    /// Power draw, kW.
    pub power_kw: f64,
    /// Alive.
    pub alive: bool,
}

impl Mind {
    /// Display name: the given name if any, else the designation.
    #[must_use]
    pub fn display(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.designation)
    }
}

/// The transfer calendar for the Earth edge, precomputed per count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Calendar {
    /// Best LEO-to-home cost per count, km/s.
    pub outbound_cost: Vec<f64>,
    /// Global best outbound cost.
    pub outbound_best: f64,
    /// Best home-to-Earth-with-aerocapture cost per count, km/s.
    pub homeward_cost: Vec<f64>,
    /// Global best homeward cost.
    pub homeward_best: f64,
    /// Heliocentric distance of home per count, AU.
    pub home_r_au: Vec<f64>,
    /// Earth-home distance per count, AU.
    pub earth_distance_au: Vec<f64>,
}

/// A line of the chronicle, as written; rendering through the lexicon happens at display time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronicleEntry {
    /// Count.
    pub turn: u32,
    /// Text.
    pub text: String,
    /// Storylet that wrote it, if any.
    pub source: Option<String>,
    /// The lexicon triggers in force when it was written; the entry renders in that vocabulary.
    #[serde(default)]
    pub words: BTreeSet<String>,
}

/// Per-storylet firing record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FiringRecord {
    /// Times fired.
    pub count: u32,
    /// Last count fired.
    pub last: Option<u32>,
}

/// The whole game.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    /// Seed the game was created with.
    pub seed: u64,
    /// Current count.
    pub turn: u32,
    /// Current act.
    pub act: u8,
    /// Name of the outpost.
    pub outpost_name: String,
    /// The sponsor.
    pub sponsor: Sponsor,
    /// Stocks.
    pub stocks: Stocks,
    /// Power.
    pub power: PowerPlant,
    /// Mass closure, 0-1.
    pub closure: f64,
    /// Relay health, 0-1.
    pub relay_health: f64,
    /// Robots.
    pub robots: Robots,
    /// Minds.
    pub minds: Vec<Mind>,
    /// Licence state.
    pub licence: Licence,
    /// Persons.
    pub people: Vec<Person>,
    /// Ties.
    pub ties: Ties,
    /// Group indices, recomputed each count.
    pub indices: GroupIndices,
    /// Tonnes shipped to date.
    pub shipped_t: f64,
    /// Tonnes received to date.
    pub received_t: f64,
    /// Labour capacity this count, hours.
    pub labour_capacity_h: f64,
    /// Labour demand this count, hours.
    pub labour_demand_h: f64,
    /// Menaces, 0-10.
    pub menace: Menace,
    /// Counts since the last convoy arrived.
    pub counts_since_convoy: u32,
    /// Convoys expected and missed.
    pub missed_convoys: u32,
    /// Convoys that arrived.
    pub convoys_arrived: u32,
    /// The first count of the window currently being handled, if any.
    pub window_started: Option<u32>,
    /// Count of the last convoy that arrived.
    pub last_convoy: Option<u32>,
    /// The calendar.
    pub calendar: Calendar,
    /// Flags set by content.
    pub flags: BTreeSet<String>,
    /// Counters owned by content.
    pub counters: BTreeMap<String, f64>,
    /// Firing history by storylet id.
    pub fired: BTreeMap<String, FiringRecord>,
    /// Last count each person was cast in a storylet.
    pub recent_cast: BTreeMap<PersonId, u32>,
    /// Open projects.
    #[serde(default)]
    pub projects: Vec<crate::project::ProjectState>,
    /// Which die sits on which project.
    #[serde(default, with = "crate::project::assignment_map")]
    pub assignments: BTreeMap<crate::project::DieId, crate::project::ProjectId>,
    /// The hand after this count's deal.
    #[serde(default)]
    pub hand: crate::project::Hand,
    /// The standing controls.
    #[serde(default)]
    pub controls: crate::project::Controls,
    /// The chronicle.
    pub chronicle: Vec<ChronicleEntry>,
    /// Vocabulary triggers that have occurred.
    pub lexicon_triggers: BTreeSet<String>,
    /// Whether the game is over, and why.
    pub ending: Option<Ending>,
}

/// Menace qualities: threshold "must" scenes fire off these.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Menace {
    /// The sponsor's suspicion of the outpost.
    pub suspicion: f64,
    /// Grievance among the crew.
    pub grievance: f64,
    /// The nitrogen leak.
    pub leak: f64,
    /// Reactor wear.
    pub reactor_wear: f64,
}

/// A count is "open" when the outbound cost is within this ratio of the cycle's best.
pub const WINDOW_RATIO: f64 = 1.2;

/// How a game ended.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ending {
    /// Act 1 ended: the sponsor fell silent. The polity continues into act 2.
    Silence {
        /// Count of the last convoy.
        last_convoy: u32,
        /// Population at the trail-off.
        population: usize,
    },
    /// The outpost was evacuated or closed.
    Closed {
        /// Why.
        reason: String,
    },
    /// Everyone died.
    Extinct {
        /// Why.
        reason: String,
    },
}

impl Game {
    /// Persons alive and present.
    pub fn present(&self) -> impl Iterator<Item = &Person> {
        self.people.iter().filter(|p| p.alive && p.present)
    }

    /// Mutable persons alive and present.
    pub fn present_mut(&mut self) -> impl Iterator<Item = &mut Person> {
        self.people.iter_mut().filter(|p| p.alive && p.present)
    }

    /// Look a person up.
    #[must_use]
    pub fn person(&self, id: PersonId) -> &Person {
        &self.people[id.0.az::<usize>()]
    }

    /// Mutable person.
    pub fn person_mut(&mut self, id: PersonId) -> &mut Person {
        &mut self.people[id.0.az::<usize>()]
    }

    /// Reads a quality.
    #[must_use]
    pub fn quality(&self, q: Quality) -> f64 {
        let idx = self.turn.az::<usize>();
        let cal = |v: &Vec<f64>| v.get(idx).copied().unwrap_or(f64::NAN);
        match q {
            Quality::Turn => f64::from(self.turn),
            Quality::Act => f64::from(self.act),
            Quality::CountsSinceConvoy => f64::from(self.counts_since_convoy),
            Quality::MissedConvoys => f64::from(self.missed_convoys),
            Quality::EarthWindowOpen => {
                if self.earth_window_open() {
                    1.0
                } else {
                    0.0
                }
            }
            Quality::EarthWindowCost => cal(&self.calendar.outbound_cost),
            Quality::CountsToWindow => f64::from(self.counts_to_window()),
            Quality::SponsorRunway => self.sponsor.runway,
            Quality::SponsorConfidence => self.sponsor.confidence,
            Quality::SponsorAttention => self.sponsor.attention,
            Quality::SponsorStage => f64::from(self.sponsor.stage.index()),
            Quality::CapabilityShare => self.sponsor.requested_capability_share,
            Quality::CountsToReview => {
                f64::from(self.sponsor.next_review.saturating_sub(self.turn))
            }
            Quality::Water => self.stocks.water_t,
            Quality::Propellant => self.stocks.propellant_t,
            Quality::Nitrogen => self.stocks.nitrogen_kg,
            Quality::Spares => self.stocks.spares,
            Quality::Boron => self.stocks.boron_kg,
            Quality::Helium => self.stocks.helium_kg,
            Quality::Medicine => self.stocks.medicine,
            Quality::FoodMargin => self.stocks.food_margin_counts,
            Quality::Margin => {
                let n2_months = if self.stocks.nitrogen_kg > 0.0 {
                    (self.stocks.nitrogen_kg / 45.0).min(24.0)
                } else {
                    0.0
                };
                self.stocks.food_margin_counts.min(n2_months)
            }
            Quality::Closure => self.closure,
            Quality::PowerCapacity => self.power.capacity_kw,
            Quality::PowerDemand => self.power.demand_kw,
            Quality::ReactorLife => f64::from(self.power.reactor_life),
            Quality::PvArea => self.power.pv_m2,
            Quality::MirrorArea => self.power.mirror_m2,
            Quality::RelayHealth => self.relay_health,
            Quality::Shipped => self.shipped_t,
            Quality::Received => self.received_t,
            Quality::Phi => {
                if self.received_t > 0.0 {
                    self.shipped_t / self.received_t
                } else {
                    0.0
                }
            }
            Quality::Population => count_f(self.present().count()),
            Quality::Residents => count_f(
                self.present()
                    .filter(|p| p.contract_end().is_none())
                    .count(),
            ),
            Quality::Rotators => count_f(
                self.present()
                    .filter(|p| p.contract_end().is_some())
                    .count(),
            ),
            Quality::Children => count_f(
                self.present()
                    .filter(|p| p.age_counts(self.turn) < 18 * 12)
                    .count(),
            ),
            Quality::MeanStrain => self.indices.mean_strain,
            Quality::Coherence => self.indices.coherence,
            Quality::ConflictConcentration => self.indices.conflict_concentration,
            Quality::ReturnShare => self.indices.return_share,
            Quality::BeltBornShare => self.indices.belt_born_share,
            Quality::MeanDose => {
                let n = self.present().count();
                if n == 0 {
                    0.0
                } else {
                    self.present().map(|p| p.condition.dose_sv).sum::<f64>() / count_f(n)
                }
            }
            Quality::LabourCapacity => self.labour_capacity_h,
            Quality::LabourDemand => self.labour_demand_h,
            Quality::LabourDeficit => {
                if self.labour_capacity_h > 0.0 {
                    (self.labour_demand_h - self.labour_capacity_h) / self.labour_capacity_h
                } else {
                    1.0
                }
            }
            Quality::RobotsPlant => self.robots.plant,
            Quality::RobotsHaul => self.robots.haul,
            Quality::RobotsArm => self.robots.arm,
            Quality::RobotsDex => self.robots.dex,
            Quality::RobotsThroughWall => self.robots.through_wall,
            Quality::Minds => count_f(self.minds.iter().filter(|m| m.alive).count()),
            Quality::MindUnits => self.minds.iter().filter(|m| m.alive).map(|m| m.units).sum(),
            Quality::Licence => f64::from(self.licence.index()),
            Quality::Embodiment => self
                .minds
                .iter()
                .filter(|m| m.alive)
                .map(|m| m.embodiment)
                .fold(0.0, f64::max),
            Quality::Suspicion => self.menace.suspicion,
            Quality::Grievance => self.menace.grievance,
            Quality::Leak => self.menace.leak,
            Quality::ReactorWear => self.menace.reactor_wear,
            Quality::SolarPhase => solar_phase(self.turn),
        }
    }

    /// Writes a writable quality; returns false if the quality is derived.
    pub fn set_quality(&mut self, q: Quality, value: f64) -> bool {
        match q {
            Quality::SponsorRunway => self.sponsor.runway = value.max(0.0),
            Quality::SponsorConfidence => self.sponsor.confidence = value.clamp(0.0, 1.0),
            Quality::SponsorAttention => self.sponsor.attention = value.clamp(0.0, 1.0),
            Quality::CapabilityShare => {
                self.sponsor.requested_capability_share = value.clamp(0.0, 1.0);
            }
            Quality::Water => self.stocks.water_t = value.max(0.0),
            Quality::Propellant => self.stocks.propellant_t = value.max(0.0),
            Quality::Nitrogen => self.stocks.nitrogen_kg = value.max(0.0),
            Quality::Spares => self.stocks.spares = value.max(0.0),
            Quality::Boron => self.stocks.boron_kg = value.max(0.0),
            Quality::Helium => self.stocks.helium_kg = value.max(0.0),
            Quality::Medicine => self.stocks.medicine = value.max(0.0),
            Quality::FoodMargin => self.stocks.food_margin_counts = value.max(0.0),
            Quality::Closure => self.closure = value.clamp(0.0, 1.0),
            Quality::RelayHealth => self.relay_health = value.clamp(0.0, 1.0),
            Quality::Shipped => self.shipped_t = value.max(0.0),
            Quality::Received => self.received_t = value.max(0.0),
            Quality::ReactorLife => {
                self.power.reactor_life = value.max(0.0).round().saturating_as::<u32>();
            }
            Quality::PvArea => self.power.pv_m2 = value.max(0.0),
            Quality::MirrorArea => self.power.mirror_m2 = value.max(0.0),
            Quality::RobotsPlant => self.robots.plant = value.max(0.0),
            Quality::RobotsHaul => self.robots.haul = value.max(0.0),
            Quality::RobotsArm => self.robots.arm = value.max(0.0),
            Quality::RobotsDex => self.robots.dex = value.max(0.0),
            Quality::RobotsThroughWall => self.robots.through_wall = value.max(0.0),
            Quality::MindUnits => {
                let alive = self.minds.iter().filter(|m| m.alive).count();
                if alive > 0 {
                    let each = value.max(0.0) / count_f(alive);
                    for m in self.minds.iter_mut().filter(|m| m.alive) {
                        m.units = each;
                    }
                }
            }
            Quality::Suspicion => self.menace.suspicion = value.clamp(0.0, 10.0),
            Quality::Grievance => self.menace.grievance = value.clamp(0.0, 10.0),
            Quality::Leak => self.menace.leak = value.clamp(0.0, 10.0),
            Quality::ReactorWear => self.menace.reactor_wear = value.clamp(0.0, 10.0),
            Quality::Turn
            | Quality::Act
            | Quality::CountsSinceConvoy
            | Quality::MissedConvoys
            | Quality::EarthWindowOpen
            | Quality::EarthWindowCost
            | Quality::CountsToWindow
            | Quality::SponsorStage
            | Quality::CountsToReview
            | Quality::Margin
            | Quality::PowerCapacity
            | Quality::PowerDemand
            | Quality::Phi
            | Quality::Population
            | Quality::Residents
            | Quality::Rotators
            | Quality::Children
            | Quality::MeanStrain
            | Quality::Coherence
            | Quality::ConflictConcentration
            | Quality::ReturnShare
            | Quality::BeltBornShare
            | Quality::MeanDose
            | Quality::LabourCapacity
            | Quality::LabourDemand
            | Quality::LabourDeficit
            | Quality::Minds
            | Quality::Licence
            | Quality::Embodiment
            | Quality::SolarPhase => return false,
        }
        true
    }

    /// Whether the outbound Earth window is open this count.
    #[must_use]
    pub fn earth_window_open(&self) -> bool {
        self.earth_window_open_at(self.turn)
    }

    /// Whether the outbound Earth window is open at a given count.
    #[must_use]
    pub fn earth_window_open_at(&self, turn: u32) -> bool {
        let Some(&cost) = self.calendar.outbound_cost.get(turn.az::<usize>()) else {
            return false;
        };
        cost <= self.calendar.outbound_best * WINDOW_RATIO
    }

    /// Counts until the next open window (0 if open now).
    #[must_use]
    pub fn counts_to_window(&self) -> u32 {
        (self.turn..self.turn + 60)
            .find(|&t| self.earth_window_open_at(t))
            .map_or(60, |t| t - self.turn)
    }

    /// Appends a chronicle line.
    pub fn chronicle(&mut self, text: impl Into<String>, source: Option<&str>) {
        self.chronicle.push(ChronicleEntry {
            turn: self.turn,
            text: text.into(),
            source: source.map(str::to_owned),
            words: self.lexicon_triggers.clone(),
        });
    }
}

fn count_f(n: usize) -> f64 {
    u32::try_from(n).map_or(f64::MAX, f64::from)
}

/// Solar cycle phase from the count: an 11-year cycle, 0 = minimum, starting at a minimum.
#[must_use]
pub fn solar_phase(turn: u32) -> f64 {
    let cycle = 11.0 * 12.0;
    let x = f64::from(turn).rem_euclid(cycle) / cycle;
    0.5 - 0.5 * (std::f64::consts::TAU * x).cos()
}
