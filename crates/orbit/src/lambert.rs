//! Zero-revolution prograde Lambert solver (universal variables, Curtis algorithm 5.2).

use crate::MU_SUN;
use crate::vec3::Vec3;

fn stumpff_c(z: f64) -> f64 {
    if z > 1e-8 {
        (1.0 - z.sqrt().cos()) / z
    } else if z < -1e-8 {
        ((-z).sqrt().cosh() - 1.0) / (-z)
    } else {
        0.5
    }
}

fn stumpff_s(z: f64) -> f64 {
    if z > 1e-8 {
        let sz = z.sqrt();
        (sz - sz.sin()) / sz.powi(3)
    } else if z < -1e-8 {
        let sm = (-z).sqrt();
        (sm.sinh() - sm) / sm.powi(3)
    } else {
        1.0 / 6.0
    }
}

fn lowest_z_with_positive_y(y_of: &impl Fn(f64) -> f64, lo: f64, hi: f64) -> Option<f64> {
    if y_of(lo) >= 0.0 {
        return Some(lo);
    }
    if y_of(hi) < 0.0 {
        return None;
    }
    let mut lo = lo;
    let mut hi = hi;
    for _ in 0..60 {
        let mid = f64::midpoint(lo, hi);
        if y_of(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(hi)
}

/// A solved transfer: the heliocentric velocities required at departure and arrival.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transfer {
    /// Velocity on the transfer orbit at departure, km/s.
    pub v1: Vec3,
    /// Velocity on the transfer orbit at arrival, km/s.
    pub v2: Vec3,
}

/// Solves the prograde zero-revolution Lambert problem between `r1` and `r2` in `tof_s` seconds.
///
/// Returns `None` when the bisection fails to converge, which happens for geometries
/// that a zero-revolution prograde arc cannot join in the given time.
#[must_use]
pub fn solve(r1: Vec3, r2: Vec3, tof_s: f64) -> Option<Transfer> {
    let n1 = r1.norm();
    let n2 = r2.norm();
    let cz = r1.cross(r2).z;
    let cos_dth = (r1.dot(r2) / (n1 * n2)).clamp(-1.0, 1.0);
    let mut dth = cos_dth.acos();
    if cz < 0.0 {
        dth = std::f64::consts::TAU - dth;
    }
    let a = dth.sin() * (n1 * n2 / (1.0 - dth.cos())).sqrt();
    let sqrt_mu = MU_SUN.sqrt();
    let y_of = |z: f64| n1 + n2 + a * (z * stumpff_s(z) - 1.0) / stumpff_c(z).sqrt();
    let f_of = |z: f64| {
        let y = y_of(z);
        (y / stumpff_c(z)).powf(1.5) * stumpff_s(z) + a * y.sqrt() - sqrt_mu * tof_s
    };
    let four_pi2 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;
    let hi = four_pi2 * 0.999;
    let lo = lowest_z_with_positive_y(&y_of, -four_pi2, hi)?;
    let mut lo = lo;
    let mut hi = hi;
    for _ in 0..80 {
        let mid = f64::midpoint(lo, hi);
        if f_of(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let z = f64::midpoint(lo, hi);
    let y = y_of(z);
    if f_of(z).abs() >= 1e-3 * sqrt_mu * tof_s || !y.is_finite() || y <= 0.0 {
        return None;
    }
    let f = 1.0 - y / n1;
    let g = a * (y / MU_SUN).sqrt();
    let gd = 1.0 - y / n2;
    let v1 = (r2 - r1 * f) * (1.0 / g);
    let v2 = (r2 * gd - r1) * (1.0 / g);
    Some(Transfer { v1, v2 })
}
