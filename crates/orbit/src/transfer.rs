//! Transfer costs: Hohmann, Edelbaum, phasing, synodic periods, and Lambert porkchops.

use crate::kepler::{Elements, Jd, State};
use crate::lambert;
use crate::{DAY_S, MU_SUN};
use az::{Az, SaturatingAs};

/// Gravitational parameter of Earth, km³/s².
pub const MU_EARTH: f64 = 398_600.441_8;
/// Radius of a 200 km low Earth orbit, km.
pub const R_LEO: f64 = 6378.137 + 200.0;

/// How the departure leg is priced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Departure {
    /// Hyperbolic excess only: the vessel is already free of the body.
    Excess,
    /// Escape from a 200 km low Earth orbit.
    FromLeo,
}

/// How the arrival leg is priced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arrival {
    /// Hyperbolic excess only: no capture burn is modelled.
    Excess,
    /// Capture into a circular orbit of the given radius about a body with the given GM.
    Capture {
        /// Gravitational parameter of the target, km³/s².
        gm: f64,
        /// Orbit radius after capture, km.
        radius_km: f64,
    },
    /// Arrival is free (aerocapture at a body with an atmosphere).
    Aerocapture,
}

/// Cost of one departure-date / time-of-flight cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cost {
    /// Departure burn, km/s.
    pub depart_kms: f64,
    /// Arrival burn, km/s.
    pub arrive_kms: f64,
    /// Time of flight in days.
    pub tof_days: f64,
}

impl Cost {
    /// Total delta-v, km/s.
    #[must_use]
    pub fn total_kms(&self) -> f64 {
        self.depart_kms + self.arrive_kms
    }
}

/// Delta-v to leave a circular orbit of radius `r` about a body of parameter `mu` with excess `vinf`.
#[must_use]
pub fn escape_dv(vinf: f64, mu: f64, r: f64) -> f64 {
    (vinf * vinf + 2.0 * mu / r).sqrt() - (mu / r).sqrt()
}

/// Two-impulse Hohmann transfer between the semi-major axes of two orbits, coplanar and circular.
#[must_use]
pub fn hohmann(from: &Elements, to: &Elements) -> Cost {
    let ra = from.a_km;
    let rb = to.a_km;
    let va = (MU_SUN / ra).sqrt();
    let vb = (MU_SUN / rb).sqrt();
    let at = f64::midpoint(ra, rb);
    let depart_kms = ((MU_SUN * (2.0 / ra - 1.0 / at)).sqrt() - va).abs();
    let arrive_kms = (vb - (MU_SUN * (2.0 / rb - 1.0 / at)).sqrt()).abs();
    let tof_days = std::f64::consts::PI * (at.powi(3) / MU_SUN).sqrt() / DAY_S;
    Cost {
        depart_kms,
        arrive_kms,
        tof_days,
    }
}

/// Edelbaum low-thrust delta-v between two circular orbits with a plane change, km/s.
#[must_use]
pub fn edelbaum(from: &Elements, to: &Elements) -> f64 {
    let va = from.circular_speed();
    let vb = to.circular_speed();
    let di = (from.i - to.i).abs();
    (va * va + vb * vb - 2.0 * va * vb * (std::f64::consts::FRAC_PI_2 * di).cos()).sqrt()
}

/// Synodic period between two orbits, in years.
#[must_use]
pub fn synodic_years(a: &Elements, b: &Elements) -> f64 {
    let ta = a.period_years();
    let tb = b.period_years();
    (1.0 / (1.0 / ta - 1.0 / tb)).abs()
}

/// Delta-v to shift phase by `delta_lambda` radians in `n_orbits` orbits at circular speed `v_orb`.
///
/// This is the neighbourhood "hurry or wait" law: the phasing burn is spread over the
/// number of orbits the vessel is willing to wait.
#[must_use]
pub fn phasing_dv(v_orb: f64, delta_lambda: f64, n_orbits: f64) -> f64 {
    v_orb * (delta_lambda.abs() / (3.0 * std::f64::consts::PI)) / n_orbits
}

/// Prices a single Lambert arc departing at `depart` and arriving `tof_days` later.
#[must_use]
pub fn price(
    from: &Elements,
    to: &Elements,
    depart: Jd,
    tof_days: f64,
    dep: Departure,
    arr: Arrival,
) -> Option<Cost> {
    let State { r: r1, v: va } = from.state_at(depart);
    let State { r: r2, v: vb } = to.state_at(depart.plus_days(tof_days));
    let lambert::Transfer { v1, v2 } = lambert::solve(r1, r2, tof_days * DAY_S)?;
    let vinf1 = (v1 - va).norm();
    let vinf2 = (v2 - vb).norm();
    let depart_kms = match dep {
        Departure::Excess => vinf1,
        Departure::FromLeo => escape_dv(vinf1, MU_EARTH, R_LEO),
    };
    let arrive_kms = match arr {
        Arrival::Excess => vinf2,
        Arrival::Capture { gm, radius_km } => escape_dv(vinf2, gm, radius_km),
        Arrival::Aerocapture => 0.0,
    };
    Some(Cost {
        depart_kms,
        arrive_kms,
        tof_days,
    })
}

/// Cheapest transfer departing at `depart` over the given range of flight times.
#[must_use]
pub fn best_at(
    from: &Elements,
    to: &Elements,
    depart: Jd,
    tof_range_days: (f64, f64),
    tof_step_days: f64,
    dep: Departure,
    arr: Arrival,
) -> Option<Cost> {
    let (lo, hi) = tof_range_days;
    let steps = ((hi - lo) / tof_step_days).ceil().saturating_as::<u32>();
    (0..steps)
        .map(|k| lo + k.az::<f64>() * tof_step_days)
        .filter_map(|tof| price(from, to, depart, tof, dep, arr))
        .min_by(|a, b| a.total_kms().total_cmp(&b.total_kms()))
}
