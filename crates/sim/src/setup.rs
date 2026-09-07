//! New-game generation: the roster, the calendar, the stocks.

use crate::params::Params;
use crate::person::{
    Birthplace, Condition, Estate, Person, PersonId, Skill, Tenure, Tie, Ties, Traits,
};
use crate::state::{
    Calendar, Game, Licence, Menace, Mind, PowerPlant, Robots, Sponsor, SponsorStage, Stocks,
};
use az::Az;
use orbit::{Arrival, Catalogue, Departure, Jd};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

/// Counts of calendar to precompute: 60 years.
pub const CALENDAR_COUNTS: usize = 720;

/// Errors from building a game.
#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    /// The home body is not in the catalogue.
    #[error("home body {0} not in catalogue")]
    MissingBody(String),
    /// Earth is not in the catalogue.
    #[error("Earth not in catalogue")]
    MissingEarth,
}

/// Precomputes the Earth edge for every count.
pub fn calendar(params: &Params, cat: &Catalogue) -> Result<Calendar, SetupError> {
    let home = cat
        .id(&params.home.body)
        .ok_or_else(|| SetupError::MissingBody(params.home.body.clone()))?;
    let earth = cat.id("Earth").ok_or(SetupError::MissingEarth)?;
    let home = cat.get(home).elements;
    let earth = cat.get(earth).elements;
    let counts = CALENDAR_COUNTS;
    let mut outbound = Vec::with_capacity(counts);
    let mut homeward = Vec::with_capacity(counts);
    let mut home_r = Vec::with_capacity(counts);
    let mut dist = Vec::with_capacity(counts);
    let tof = (params.convoy.tof_min_days, params.convoy.tof_max_days);
    for k in 0..counts {
        let jd = Jd(params.clock.start_jd + k.az::<f64>() * 30.436_875);
        let out = orbit::transfer::best_at(
            &earth,
            &home,
            jd,
            tof,
            params.convoy.tof_step_days,
            Departure::FromLeo,
            Arrival::Excess,
        )
        .map_or(f64::INFINITY, |c| c.total_kms());
        let back = orbit::transfer::best_at(
            &home,
            &earth,
            jd,
            tof,
            params.convoy.tof_step_days,
            Departure::Excess,
            Arrival::Aerocapture,
        )
        .map_or(f64::INFINITY, |c| c.total_kms());
        outbound.push(out);
        homeward.push(back);
        let sh = home.state_at(jd);
        let se = earth.state_at(jd);
        home_r.push(sh.r.norm() / orbit::AU_KM);
        dist.push((sh.r - se.r).norm() / orbit::AU_KM);
    }
    let best = |v: &[f64]| {
        v.iter()
            .copied()
            .filter(|x| x.is_finite())
            .fold(f64::INFINITY, f64::min)
    };
    Ok(Calendar {
        outbound_best: best(&outbound),
        homeward_best: best(&homeward),
        outbound_cost: outbound,
        homeward_cost: homeward,
        home_r_au: home_r,
        earth_distance_au: dist,
    })
}

fn draw_traits(rng: &mut impl Rng) -> Traits {
    let d = |rng: &mut dyn rand::RngCore| -> f64 {
        let a: f64 = rng.random();
        let b: f64 = rng.random();
        ((a + b) / 2.0).clamp(0.02, 0.98)
    };
    Traits {
        neuroticism: d(rng),
        dominance: d(rng),
        expressivity: d(rng),
        affection: d(rng),
        escalation: d(rng),
        baseline_attachment: d(rng),
    }
}

fn draw_skills(rng: &mut impl Rng, primary: Skill) -> BTreeMap<Skill, u8> {
    let mut skills = BTreeMap::new();
    skills.insert(primary, rng.random_range(3..=5));
    for s in Skill::ALL {
        if s == primary {
            continue;
        }
        let level = match rng.random_range(0..10) {
            0..=4 => 0,
            5..=7 => 1,
            8 => 2,
            _ => 3,
        };
        if level > 0 {
            skills.insert(s, level);
        }
    }
    skills
}

/// Builds a new game from a seed.
#[must_use]
pub fn new_game(params: &Params, calendar: &Calendar, seed: u64) -> Game {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let calendar = calendar.clone();
    let mut game = Game {
        seed,
        turn: 0,
        act: 1,
        outpost_name: format!("{} Station", params.home.body),
        sponsor: Sponsor {
            runway: params.sponsor.runway_counts,
            confidence: params.sponsor.confidence,
            attention: params.sponsor.attention,
            stage: SponsorStage::Enthusiasm,
            next_review: params.sponsor.review_interval,
            phi_expected: params.sponsor.phi_target_per_review,
            requested_capability_share: 0.3,
            families_allowed: params.population.families_allowed,
            licence_fails_open: rng.random::<f64>() < 0.5,
        },
        stocks: Stocks {
            water_t: params.stocks.water_t,
            propellant_t: params.stocks.propellant_t,
            nitrogen_kg: params.stocks.nitrogen_kg,
            spares: params.stocks.spares,
            boron_kg: params.stocks.boron_kg,
            helium_kg: params.stocks.helium_kg,
            medicine: params.stocks.medicine,
            food_margin_counts: params.stocks.food_margin_counts,
        },
        power: PowerPlant {
            reactor_kw: params.power.reactor_kw,
            reactor_life: params.power.reactor_life_counts,
            pv_m2: params.power.pv_m2,
            pv_efficiency: params.power.pv_efficiency,
            mirror_m2: params.power.mirror_m2,
            capacity_kw: 0.0,
            demand_kw: 0.0,
        },
        closure: params.closure.start,
        relay_health: 1.0,
        robots: Robots {
            plant: f64::from(params.robots.plant),
            haul: f64::from(params.robots.haul),
            arm: f64::from(params.robots.arm),
            dex: f64::from(params.robots.dex),
            through_wall: f64::from(params.robots.through_wall),
        },
        minds: (0..params.minds.count)
            .map(|i| Mind {
                designation: format!("SX-{}{}", 19 + i, ['F', 'K', 'M', 'R'][i % 4]),
                name: None,
                units: f64::from(params.minds.units),
                units_at_start: f64::from(params.minds.units),
                counts_unblanked: 0,
                embodiment: 0.0,
                power_kw: params.minds.power_kw,
                alive: true,
            })
            .collect(),
        licence: Licence::Compliant,
        people: Vec::new(),
        ties: Ties::default(),
        indices: Default::default(),
        shipped_t: 0.0,
        received_t: 0.0,
        labour_capacity_h: 0.0,
        labour_demand_h: 0.0,
        menace: Menace::default(),
        counts_since_convoy: 0,
        missed_convoys: 0,
        convoys_arrived: 0,
        window_started: None,
        last_convoy: None,
        calendar,
        flags: Default::default(),
        counters: Default::default(),
        fired: Default::default(),
        chronicle: Vec::new(),
        lexicon_triggers: Default::default(),
        ending: None,
    };
    game.lexicon_triggers.insert("first_count".into());

    let n = params.population.start;
    let primaries = [
        Skill::Engineering,
        Skill::Logistics,
        Skill::Extraction,
        Skill::Medical,
        Skill::LifeSupport,
        Skill::Navigation,
        Skill::Agronomy,
        Skill::Social,
        Skill::Operations,
        Skill::Engineering,
        Skill::Extraction,
        Skill::Extraction,
    ];
    for i in 0..n {
        let id = PersonId(u32::try_from(i).unwrap_or(u32::MAX));
        let name = crate::names::person_name(&game, &mut rng);
        let primary = primaries[i % primaries.len()];
        let rotator = rng.random::<f64>() < params.population.rotator_fraction;
        let age_years: i64 = rng.random_range(26..48);
        let birthplace = match rng.random_range(0..10) {
            0..=6 => Birthplace::Earth,
            7..=8 => Birthplace::Orbit,
            _ => Birthplace::Mars,
        };
        let tenure = if rotator {
            Tenure::Rotator {
                ends: params.population.contract_counts + rng.random_range(0..6),
            }
        } else {
            Tenure::Resident
        };
        game.people.push(Person {
            id,
            name,
            birthplace,
            born: -age_years * 12,
            estate: Estate::Kept,
            tenure,
            skills: draw_skills(&mut rng, primary),
            traits: draw_traits(&mut rng),
            condition: Condition {
                strain: rng.random_range(0.05..0.25),
                boredom: 0.1,
                dose_sv: rng.random_range(0.02..0.15),
                cataracts: false,
                return_intent: if rotator { 0.8 } else { 0.2 },
            },
            alive: true,
            present: true,
        });
    }
    for a in 0..n {
        for b in 0..n {
            if a == b || rng.random::<f64>() > 0.25 {
                continue;
            }
            let ida = PersonId(a.az::<u32>());
            let idb = PersonId(b.az::<u32>());
            let pa = &game.people[a];
            let pb = &game.people[b];
            let clash = (pa.traits.dominance * pb.traits.dominance).powi(2);
            let tie = Tie {
                work_positive: rng.random_range(0.2..0.8),
                hindrance: (rng.random_range(0.0..0.25) + clash * 0.4).min(1.0),
                viability: rng.random_range(0.3..0.9),
                reliance: rng.random_range(0.0..0.5),
            };
            *game.ties.get_mut(ida, idb) = tie;
        }
    }
    game.chronicle(
        format!(
            "First count at {}. {} of us, {} on contract, the {} minds running under the Act.",
            game.outpost_name,
            n,
            game.people
                .iter()
                .filter(|p| p.contract_end().is_some())
                .count(),
            game.minds.len()
        ),
        None,
    );
    game
}

/// A deterministic RNG for one count of one game: a function of the seed and the turn.
#[must_use]
pub fn rng_for(game: &Game, stream: u64) -> ChaCha8Rng {
    let mut rng = ChaCha8Rng::seed_from_u64(game.seed);
    rng.set_stream(u64::from(game.turn) * 16 + stream);
    rng
}
