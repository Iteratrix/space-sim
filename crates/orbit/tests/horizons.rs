//! Validation against JPL Horizons integrated vectors at JD 2462502.5 (2030-01-01 TDB),
//! heliocentric ecliptic J2000, km and km/s. Fetched 2026-09-06 from the Horizons API.

use orbit::{Catalogue, Jd, Vec3, bundled_catalogue};

const JD_2030: Jd = Jd(2_462_502.5);

struct Reference {
    name: &'static str,
    r: Vec3,
    v: Vec3,
    tol_r: f64,
    tol_v: f64,
}

const REFS: &[Reference] = &[
    Reference {
        name: "Ceres",
        r: Vec3::new(
            4.227_234_362_293_558e8,
            -1.158_180_366_136_358e8,
            -8.152_202_471_420_151e7,
        ),
        v: Vec3::new(
            4.038_364_378_431_361,
            16.072_225_064_621_08,
            -0.231_007_912_412_916_7,
        ),
        tol_r: 0.006,
        tol_v: 0.006,
    },
    Reference {
        name: "Earth",
        r: Vec3::new(
            -2.600_847_757_810_495e7,
            1.447_900_488_833_384e8,
            -9.532_206_645_891_07e3,
        ),
        v: Vec3::new(
            -29.815_051_927_415_13,
            -5.371_451_831_814_229,
            -5.573_338_788_711_357e-4,
        ),
        tol_r: 0.003,
        tol_v: 0.003,
    },
    Reference {
        name: "Mars",
        r: Vec3::new(
            1.912_791_417_330_595e8,
            -7.798_058_850_777_282e7,
            -6.323_760_608_033_922e6,
        ),
        v: Vec3::new(
            10.066_894_478_013_66,
            24.509_014_150_592_23,
            0.266_885_231_984_627_3,
        ),
        tol_r: 0.01,
        tol_v: 0.01,
    },
    Reference {
        name: "Fortuna",
        r: Vec3::new(
            -3.987_123_838_579_816e8,
            6.142_612_521_585_729e7,
            -7.084_686_616_293_32e6,
        ),
        v: Vec3::new(
            -4.613_083_964_033_871,
            -16.527_643_167_748_77,
            0.324_097_996_523_913_2,
        ),
        tol_r: 0.006,
        tol_v: 0.006,
    },
    Reference {
        name: "Vesta",
        r: Vec3::new(
            3.250_582_492_918_579e8,
            -1.185_901_435_013_306e8,
            -3.607_179_231_387_807e7,
        ),
        v: Vec3::new(
            8.192_822_804_494_854,
            17.827_155_187_884_45,
            -1.524_999_052_257_745,
        ),
        tol_r: 0.006,
        tol_v: 0.006,
    },
];

fn catalogue() -> Catalogue {
    bundled_catalogue().expect("bundled catalogue parses")
}

#[test]
fn propagated_positions_match_horizons() {
    let cat = catalogue();
    for Reference {
        name,
        r,
        v,
        tol_r,
        tol_v,
    } in REFS
    {
        let id = cat
            .id(name)
            .unwrap_or_else(|| panic!("{name} in catalogue"));
        let state = cat.get(id).elements.state_at(JD_2030);
        let rel_r = (state.r - *r).norm() / r.norm();
        let rel_v = (state.v - *v).norm() / v.norm();
        assert!(
            rel_r < *tol_r,
            "{name}: position error {rel_r:.4} exceeds {tol_r}"
        );
        assert!(
            rel_v < *tol_v,
            "{name}: velocity error {rel_v:.4} exceeds {tol_v}"
        );
    }
}

#[test]
fn lambert_recovers_a_body_own_orbit() {
    let cat = catalogue();
    let ceres = &cat.get(cat.id("Ceres").expect("Ceres")).elements;
    let t1 = JD_2030;
    let tof_days = 400.0;
    let s1 = ceres.state_at(t1);
    let s2 = ceres.state_at(t1.plus_days(tof_days));
    let transfer = orbit::lambert::solve(s1.r, s2.r, tof_days * orbit::DAY_S).expect("solves");
    assert!(
        (transfer.v1 - s1.v).norm() < 1e-3,
        "v1 mismatch {:?}",
        transfer.v1 - s1.v
    );
    assert!(
        (transfer.v2 - s2.v).norm() < 1e-3,
        "v2 mismatch {:?}",
        transfer.v2 - s2.v
    );
}

#[test]
fn hohmann_earth_mars_is_about_five_and_a_half() {
    let cat = catalogue();
    let earth = &cat.get(cat.id("Earth").expect("Earth")).elements;
    let mars = &cat.get(cat.id("Mars").expect("Mars")).elements;
    let cost = orbit::transfer::hohmann(earth, mars);
    assert!(
        (cost.total_kms() - 5.6).abs() < 0.15,
        "total {}",
        cost.total_kms()
    );
    assert!((cost.tof_days - 259.0).abs() < 5.0, "tof {}", cost.tof_days);
}

#[test]
fn synodic_periods_match_the_map_report() {
    let cat = catalogue();
    let el = |n: &str| cat.get(cat.id(n).expect(n)).elements;
    let earth_fortuna = orbit::transfer::synodic_years(&el("Earth"), &el("Fortuna"));
    assert!(
        (earth_fortuna - 1.36).abs() < 0.03,
        "Earth-Fortuna {earth_fortuna}"
    );
    let ceres_vesta = orbit::transfer::synodic_years(&el("Ceres"), &el("Vesta"));
    assert!(
        (ceres_vesta - 17.2).abs() < 1.0,
        "Ceres-Vesta {ceres_vesta}"
    );
}

#[test]
fn earth_to_fortuna_window_exists_in_a_synodic_cycle() {
    let cat = catalogue();
    let earth = cat.get(cat.id("Earth").expect("Earth")).elements;
    let fortuna = cat.get(cat.id("Fortuna").expect("Fortuna")).elements;
    let mut best = f64::INFINITY;
    let mut worst: f64 = 0.0;
    for step in 0..50 {
        let depart = JD_2030.plus_days(f64::from(step) * 10.0);
        if let Some(c) = orbit::transfer::best_at(
            &earth,
            &fortuna,
            depart,
            (150.0, 900.0),
            10.0,
            orbit::Departure::FromLeo,
            orbit::Arrival::Excess,
        ) {
            best = best.min(c.total_kms());
            worst = worst.max(c.total_kms());
        }
    }
    assert!(best < 11.0 && best > 7.0, "best {best}");
    assert!(
        worst > best * 1.5,
        "windows should oscillate: best {best} worst {worst}"
    );
}
