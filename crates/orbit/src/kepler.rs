//! Two-body propagation from osculating elements.

use crate::vec3::Vec3;
use crate::{AU_KM, DAY_S, MU_SUN};
use serde::{Deserialize, Serialize};

/// A Julian day number (TDB).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Jd(pub f64);

impl Jd {
    /// Adds a number of days.
    #[must_use]
    pub const fn plus_days(self, days: f64) -> Self {
        Self(self.0 + days)
    }

    /// Days elapsed from `earlier` to `self`.
    #[must_use]
    pub const fn days_since(self, earlier: Self) -> f64 {
        self.0 - earlier.0
    }
}

/// Heliocentric osculating elements in the ecliptic J2000 frame.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Elements {
    /// Semi-major axis in kilometres.
    pub a_km: f64,
    /// Eccentricity.
    pub e: f64,
    /// Inclination in radians.
    pub i: f64,
    /// Longitude of the ascending node in radians.
    pub om: f64,
    /// Argument of perihelion in radians.
    pub w: f64,
    /// Mean anomaly at `epoch` in radians.
    pub m0: f64,
    /// Epoch of the elements.
    pub epoch: Jd,
}

/// Position and velocity at an instant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    /// Heliocentric position in kilometres.
    pub r: Vec3,
    /// Heliocentric velocity in kilometres per second.
    pub v: Vec3,
}

impl Elements {
    /// Builds elements from the angles in degrees and the semi-major axis in AU.
    #[must_use]
    pub fn from_degrees(
        a_au: f64,
        e: f64,
        i_deg: f64,
        om_deg: f64,
        w_deg: f64,
        ma_deg: f64,
        epoch: Jd,
    ) -> Self {
        Self {
            a_km: a_au * AU_KM,
            e,
            i: i_deg.to_radians(),
            om: om_deg.to_radians(),
            w: w_deg.to_radians(),
            m0: ma_deg.to_radians(),
            epoch,
        }
    }

    /// Mean motion in radians per second.
    #[must_use]
    pub fn mean_motion(&self) -> f64 {
        (MU_SUN / self.a_km.powi(3)).sqrt()
    }

    /// Orbital period in days.
    #[must_use]
    pub fn period_days(&self) -> f64 {
        std::f64::consts::TAU / self.mean_motion() / DAY_S
    }

    /// Orbital period in Julian years.
    #[must_use]
    pub fn period_years(&self) -> f64 {
        self.period_days() / 365.25
    }

    /// Circular orbital speed at the semi-major axis, in km/s.
    #[must_use]
    pub fn circular_speed(&self) -> f64 {
        (MU_SUN / self.a_km).sqrt()
    }

    /// Mean anomaly at `jd`, wrapped to `[0, 2π)`.
    #[must_use]
    pub fn mean_anomaly_at(&self, jd: Jd) -> f64 {
        let dt = jd.days_since(self.epoch) * DAY_S;
        (self.m0 + self.mean_motion() * dt).rem_euclid(std::f64::consts::TAU)
    }

    /// Eccentric anomaly at `jd` by Newton iteration.
    #[must_use]
    pub fn eccentric_anomaly_at(&self, jd: Jd) -> f64 {
        let m = self.mean_anomaly_at(jd);
        let e = self.e;
        let mut ea = if e > 0.8 { std::f64::consts::PI } else { m };
        for _ in 0..50 {
            let f = ea - e * ea.sin() - m;
            let fp = 1.0 - e * ea.cos();
            let step = f / fp;
            ea -= step;
            if step.abs() < 1e-14 {
                break;
            }
        }
        ea
    }

    /// True anomaly at `jd`.
    #[must_use]
    pub fn true_anomaly_at(&self, jd: Jd) -> f64 {
        let ea = self.eccentric_anomaly_at(jd);
        let e = self.e;
        2.0 * ((1.0 + e).sqrt() * (ea / 2.0).sin()).atan2((1.0 - e).sqrt() * (ea / 2.0).cos())
    }

    /// Heliocentric state at `jd`.
    #[must_use]
    pub fn state_at(&self, jd: Jd) -> State {
        let e = self.e;
        let ea = self.eccentric_anomaly_at(jd);
        let nu = self.true_anomaly_at(jd);
        let r = self.a_km * (1.0 - e * ea.cos());
        let p = self.a_km * (1.0 - e * e);
        let h = (MU_SUN * p).sqrt();
        let rp = Vec3::new(r * nu.cos(), r * nu.sin(), 0.0);
        let vp = Vec3::new(-MU_SUN / h * nu.sin(), MU_SUN / h * (e + nu.cos()), 0.0);
        let (so, co) = self.om.sin_cos();
        let (sw, cw) = self.w.sin_cos();
        let (si, ci) = self.i.sin_cos();
        let rot = |v: Vec3| {
            Vec3::new(
                (co * cw - so * sw * ci) * v.x + (-co * sw - so * cw * ci) * v.y + (so * si) * v.z,
                (so * cw + co * sw * ci) * v.x + (-so * sw + co * cw * ci) * v.y + (-co * si) * v.z,
                (sw * si) * v.x + (cw * si) * v.y + ci * v.z,
            )
        };
        State {
            r: rot(rp),
            v: rot(vp),
        }
    }

    /// Heliocentric ecliptic longitude at `jd`, in radians wrapped to `[0, 2π)`.
    #[must_use]
    pub fn longitude_at(&self, jd: Jd) -> f64 {
        let State { r, v: _ } = self.state_at(jd);
        r.y.atan2(r.x).rem_euclid(std::f64::consts::TAU)
    }
}
