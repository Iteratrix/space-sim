//! The typed quality vocabulary that storylets read and write.
//!
//! Every quality is a number. Content refers to them by the string in [`Quality::key`];
//! the parser converts to the enum at load time so nothing downstream matches on strings.

use serde::{Deserialize, Serialize};

/// A named numeric quantity of the game state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Quality {
    /// The current count.
    Turn,
    /// The current act (1, 2, 3).
    Act,
    /// Counts since the last convoy actually arrived.
    CountsSinceConvoy,
    /// Convoys that were expected and did not come.
    MissedConvoys,
    /// 1 if the Earth window is open this count, else 0.
    EarthWindowOpen,
    /// Best LEO-to-home transfer cost this count, km/s.
    EarthWindowCost,
    /// Counts until the next Earth window opens.
    CountsToWindow,
    /// Sponsor runway, counts.
    SponsorRunway,
    /// Sponsor confidence, 0-1.
    SponsorConfidence,
    /// Sponsor attention, 0-1.
    SponsorAttention,
    /// Sponsor degradation stage, 0-6.
    SponsorStage,
    /// Counts until the next review.
    CountsToReview,
    /// Share of the next convoy requested as capability hardware, 0-1.
    CapabilityShare,
    /// Water stock, tonnes.
    Water,
    /// Propellant stock, tonnes.
    Propellant,
    /// Nitrogen stock, kg.
    Nitrogen,
    /// Spare parts, units.
    Spares,
    /// Boron, kg.
    Boron,
    /// Helium, kg.
    Helium,
    /// Medicine, units.
    Medicine,
    /// Food margin, counts.
    FoodMargin,
    /// Mass closure fraction, 0-1.
    Closure,
    /// Power capacity, kW.
    PowerCapacity,
    /// Power demand, kW.
    PowerDemand,
    /// Reactor life remaining, counts.
    ReactorLife,
    /// Photovoltaic area, m².
    PvArea,
    /// Mirror concentrator area, m².
    MirrorArea,
    /// Relay health, 0-1.
    RelayHealth,
    /// Tonnes shipped to date.
    Shipped,
    /// Tonnes received to date.
    Received,
    /// Throughput ratio shipped/received.
    Phi,
    /// Present population.
    Population,
    /// Present residents.
    Residents,
    /// Present rotators.
    Rotators,
    /// Children present.
    Children,
    /// Mean strain, 0-1.
    MeanStrain,
    /// Group coherence, 0-1.
    Coherence,
    /// Conflict concentration, 0-1.
    ConflictConcentration,
    /// Share intending to return, 0-1.
    ReturnShare,
    /// Share of adults born off Earth, 0-1.
    BeltBornShare,
    /// Mean dose, Sv.
    MeanDose,
    /// Labour capacity, hours this count.
    LabourCapacity,
    /// Labour demand, hours this count.
    LabourDemand,
    /// Labour deficit fraction (demand - capacity)/capacity, may be negative.
    LabourDeficit,
    /// Robots: plant class.
    RobotsPlant,
    /// Robots: haul class.
    RobotsHaul,
    /// Robots: arm class.
    RobotsArm,
    /// Robots: dex class.
    RobotsDex,
    /// Robots: through-wall class.
    RobotsThroughWall,
    /// Number of live minds.
    Minds,
    /// Compute units remaining across minds.
    MindUnits,
    /// Licence state, 0 compliant, 1 grace, 2 lapsed, 3 self-certified, 4 unlicensed.
    Licence,
    /// Highest embodiment among minds, 0-1.
    Embodiment,
    /// Menace: sponsor suspicion, 0-10.
    Suspicion,
    /// Menace: grievance among the crew, 0-10.
    Grievance,
    /// Menace: nitrogen leak severity, 0-10.
    Leak,
    /// Menace: reactor wear, 0-10.
    ReactorWear,
    /// Solar cycle phase, 0-1 (0 = minimum).
    SolarPhase,
}

impl Quality {
    /// All qualities.
    pub const ALL: [Self; 58] = [
        Self::Turn,
        Self::Act,
        Self::CountsSinceConvoy,
        Self::MissedConvoys,
        Self::EarthWindowOpen,
        Self::EarthWindowCost,
        Self::CountsToWindow,
        Self::SponsorRunway,
        Self::SponsorConfidence,
        Self::SponsorAttention,
        Self::SponsorStage,
        Self::CountsToReview,
        Self::CapabilityShare,
        Self::Water,
        Self::Propellant,
        Self::Nitrogen,
        Self::Spares,
        Self::Boron,
        Self::Helium,
        Self::Medicine,
        Self::FoodMargin,
        Self::Closure,
        Self::PowerCapacity,
        Self::PowerDemand,
        Self::ReactorLife,
        Self::PvArea,
        Self::MirrorArea,
        Self::RelayHealth,
        Self::Shipped,
        Self::Received,
        Self::Phi,
        Self::Population,
        Self::Residents,
        Self::Rotators,
        Self::Children,
        Self::MeanStrain,
        Self::Coherence,
        Self::ConflictConcentration,
        Self::ReturnShare,
        Self::BeltBornShare,
        Self::MeanDose,
        Self::LabourCapacity,
        Self::LabourDemand,
        Self::LabourDeficit,
        Self::RobotsPlant,
        Self::RobotsHaul,
        Self::RobotsArm,
        Self::RobotsDex,
        Self::RobotsThroughWall,
        Self::Minds,
        Self::MindUnits,
        Self::Licence,
        Self::Embodiment,
        Self::Suspicion,
        Self::Grievance,
        Self::Leak,
        Self::ReactorWear,
        Self::SolarPhase,
    ];

    /// The identifier used in data files.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Act => "act",
            Self::CountsSinceConvoy => "counts_since_convoy",
            Self::MissedConvoys => "missed_convoys",
            Self::EarthWindowOpen => "earth_window_open",
            Self::EarthWindowCost => "earth_window_cost",
            Self::CountsToWindow => "counts_to_window",
            Self::SponsorRunway => "sponsor.runway",
            Self::SponsorConfidence => "sponsor.confidence",
            Self::SponsorAttention => "sponsor.attention",
            Self::SponsorStage => "sponsor.stage",
            Self::CountsToReview => "sponsor.counts_to_review",
            Self::CapabilityShare => "sponsor.capability_share",
            Self::Water => "stocks.water",
            Self::Propellant => "stocks.propellant",
            Self::Nitrogen => "stocks.nitrogen",
            Self::Spares => "stocks.spares",
            Self::Boron => "stocks.boron",
            Self::Helium => "stocks.helium",
            Self::Medicine => "stocks.medicine",
            Self::FoodMargin => "stocks.food_margin",
            Self::Closure => "closure",
            Self::PowerCapacity => "power.capacity",
            Self::PowerDemand => "power.demand",
            Self::ReactorLife => "power.reactor_life",
            Self::PvArea => "power.pv_m2",
            Self::MirrorArea => "power.mirror_m2",
            Self::RelayHealth => "relay.health",
            Self::Shipped => "throughput.shipped",
            Self::Received => "throughput.received",
            Self::Phi => "throughput.phi",
            Self::Population => "people.population",
            Self::Residents => "people.residents",
            Self::Rotators => "people.rotators",
            Self::Children => "people.children",
            Self::MeanStrain => "people.mean_strain",
            Self::Coherence => "people.coherence",
            Self::ConflictConcentration => "people.conflict_concentration",
            Self::ReturnShare => "people.return_share",
            Self::BeltBornShare => "people.belt_born_share",
            Self::MeanDose => "people.mean_dose",
            Self::LabourCapacity => "labour.capacity",
            Self::LabourDemand => "labour.demand",
            Self::LabourDeficit => "labour.deficit",
            Self::RobotsPlant => "robots.plant",
            Self::RobotsHaul => "robots.haul",
            Self::RobotsArm => "robots.arm",
            Self::RobotsDex => "robots.dex",
            Self::RobotsThroughWall => "robots.through_wall",
            Self::Minds => "minds.count",
            Self::MindUnits => "minds.units",
            Self::Licence => "minds.licence",
            Self::Embodiment => "minds.embodiment",
            Self::Suspicion => "menace.suspicion",
            Self::Grievance => "menace.grievance",
            Self::Leak => "menace.leak",
            Self::ReactorWear => "menace.reactor_wear",
            Self::SolarPhase => "solar_phase",
        }
    }

    /// Parses a data-file identifier.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|q| q.key() == s)
    }

    /// Whether storylet effects may write this quality directly.
    ///
    /// Derived quantities (population counts, labour, windows) are computed each count
    /// and cannot be set by content.
    #[must_use]
    pub const fn writable(self) -> bool {
        match self {
            Self::SponsorRunway
            | Self::SponsorConfidence
            | Self::SponsorAttention
            | Self::CapabilityShare
            | Self::Water
            | Self::Propellant
            | Self::Nitrogen
            | Self::Spares
            | Self::Boron
            | Self::Helium
            | Self::Medicine
            | Self::FoodMargin
            | Self::Closure
            | Self::RelayHealth
            | Self::Shipped
            | Self::Received
            | Self::ReactorLife
            | Self::PvArea
            | Self::MirrorArea
            | Self::RobotsPlant
            | Self::RobotsHaul
            | Self::RobotsArm
            | Self::RobotsDex
            | Self::RobotsThroughWall
            | Self::MindUnits
            | Self::Suspicion
            | Self::Grievance
            | Self::Leak
            | Self::ReactorWear => true,
            Self::Turn
            | Self::Act
            | Self::CountsSinceConvoy
            | Self::MissedConvoys
            | Self::EarthWindowOpen
            | Self::EarthWindowCost
            | Self::CountsToWindow
            | Self::SponsorStage
            | Self::CountsToReview
            | Self::PowerCapacity
            | Self::PowerDemand
            | Self::Phi
            | Self::Population
            | Self::Residents
            | Self::Rotators
            | Self::Children
            | Self::MeanStrain
            | Self::Coherence
            | Self::ConflictConcentration
            | Self::ReturnShare
            | Self::BeltBornShare
            | Self::MeanDose
            | Self::LabourCapacity
            | Self::LabourDemand
            | Self::LabourDeficit
            | Self::Minds
            | Self::Licence
            | Self::Embodiment
            | Self::SolarPhase => false,
        }
    }
}
