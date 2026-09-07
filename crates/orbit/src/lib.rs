//! Orbital mechanics for the belt map.
//!
//! Pure, deterministic functions: Kepler propagation from real osculating elements,
//! a zero-revolution Lambert solver, transfer pricing, and the body catalogue.
//! Everything here is validated against JPL Horizons in the test suite.

pub mod catalogue;
pub mod kepler;
pub mod lambert;
pub mod transfer;
pub mod vec3;

pub use catalogue::{Body, BodyId, Catalogue, CatalogueError, Kind};
pub use kepler::{Elements, Jd, State};
pub use transfer::{Arrival, Cost, Departure};
pub use vec3::Vec3;

/// Gravitational parameter of the Sun, km³/s².
pub const MU_SUN: f64 = 1.327_124_400_18e11;
/// One astronomical unit in kilometres.
pub const AU_KM: f64 = 1.495_978_707e8;
/// Seconds per day.
pub const DAY_S: f64 = 86_400.0;
/// Light time per AU, in minutes.
pub const LIGHT_MIN_PER_AU: f64 = 8.316_746;
/// Solar flux at 1 AU, W/m².
pub const SOLAR_CONSTANT_W_M2: f64 = 1361.0;

/// Solar flux at a heliocentric distance in AU, W/m².
#[must_use]
pub fn solar_flux(r_au: f64) -> f64 {
    SOLAR_CONSTANT_W_M2 / (r_au * r_au)
}

/// The catalogue shipped with the crate, parsed from `data/bodies.json` at the repo root.
pub fn bundled_catalogue() -> Result<Catalogue, CatalogueError> {
    Catalogue::from_json(include_str!("../../../data/bodies.json"))
}
