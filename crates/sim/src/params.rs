//! Tunable parameters, loaded from `data/params.toml`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The whole parameter file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    /// Calendar constants.
    pub clock: Clock,
    /// The home rock and the Keep.
    pub home: Home,
    /// Starting population and contracts.
    pub population: Population,
    /// The sponsor as an actor.
    pub sponsor: Sponsor,
    /// Convoy pricing and sizing.
    pub convoy: Convoy,
    /// Starting stocks.
    pub stocks: Stocks,
    /// Life-support closure dynamics.
    pub closure: Closure,
    /// Power supply.
    pub power: Power,
    /// Relay hardware.
    pub relay: Relay,
    /// Dose rates by estate.
    pub dose: Dose,
    /// Labour capacity and demand.
    pub labour: Labour,
    /// Robot fleet.
    pub robots: Robots,
    /// Machine minds.
    pub minds: Minds,
    /// Mining and shipping.
    pub extraction: Extraction,
    /// Social dynamics.
    pub social: Social,
    /// Storylet director.
    pub director: Director,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    pub t0_year: i32,
    pub start_jd: f64,
    pub counts_per_fortuna_year: u32,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {
    pub body: String,
    pub keep_radius_m: f64,
    pub keep_rpm: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub start: usize,
    pub rotator_fraction: f64,
    pub contract_counts: u32,
    pub families_allowed: bool,
    pub expansion_per_convoy: f64,
    pub expansion_cap: usize,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sponsor {
    pub runway_counts: f64,
    pub confidence: f64,
    pub attention: f64,
    pub review_interval: u32,
    pub phi_target_per_review: f64,
    pub markup: f64,
    pub lay_fraction: f64,
    pub shock_prob_per_review: f64,
    pub shock_runway_hit: f64,
    pub attention_decay_per_review: f64,
    pub weight_update_stop_confidence: f64,
    pub austerity_confidence: f64,
    pub skip_rotation_confidence: f64,
    pub sale_confidence: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convoy {
    pub tof_min_days: f64,
    pub tof_max_days: f64,
    pub tof_step_days: f64,
    pub base_tonnes: f64,
    pub person_kw: f64,
    pub pv_m2_per_person: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stocks {
    pub water_t: f64,
    pub propellant_t: f64,
    pub nitrogen_kg: f64,
    pub spares: f64,
    pub boron_kg: f64,
    pub helium_kg: f64,
    pub medicine: f64,
    pub food_margin_counts: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Closure {
    pub start: f64,
    pub decay_per_count_no_spares: f64,
    pub spares_per_person_count: f64,
    pub nitrogen_leak_kg_per_count: f64,
    pub hub_seal_share: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Power {
    pub reactor_kw: f64,
    pub reactor_life_counts: u32,
    pub pv_m2: f64,
    pub pv_efficiency: f64,
    pub mirror_m2: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relay {
    pub mtbf_counts: f64,
    pub count_of_blackout_per_window: u32,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dose {
    pub keep_msv_per_year: f64,
    pub bore_msv_per_year: f64,
    pub skiff_msv_per_year: f64,
    pub unshielded_msv_per_year: f64,
    pub cataract_gy: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Labour {
    pub capacity_h_per_count: f64,
    pub health_share: f64,
    pub social_share: f64,
    pub robot_maintenance_h_alive: f64,
    pub robot_maintenance_h_dead: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robots {
    pub plant: u32,
    pub haul: u32,
    pub arm: u32,
    pub dex: u32,
    pub through_wall: u32,
    pub attrition_alive: BTreeMap<String, f64>,
    pub attrition_dead: BTreeMap<String, f64>,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minds {
    pub count: usize,
    pub units: u32,
    pub attrition_per_year: f64,
    pub power_kw: f64,
    pub licence_grace_counts: u32,
    pub embodiment_per_count: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extraction {
    pub water_t_per_count: f64,
    pub driver_t_per_count: f64,
    pub driver_kw: f64,
    pub nitrogen_kg_per_t_water: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Social {
    pub displacement_outward_ratio: f64,
    pub third_quarter_start: f64,
    pub third_quarter_end: f64,
    pub strain_decay: f64,
    pub tie_drift: f64,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Director {
    pub max_storylets_per_count: usize,
    pub tension_curve: Vec<f64>,
}

impl Params {
    /// Parses the parameter file.
    pub fn from_toml(text: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(text)
    }

    /// The parameters shipped in `data/params.toml`.
    pub fn bundled() -> Result<Self, toml::de::Error> {
        Self::from_toml(include_str!("../../../data/params.toml"))
    }
}
