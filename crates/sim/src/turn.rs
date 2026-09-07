//! One count of simulation: physics, labour, people, the sponsor, the convoy.

use crate::params::Params;
use crate::person::{Estate, GroupIndices, PersonId, Tenure};
use crate::state::{Ending, Game, Licence, SponsorStage};
use az::{Az, SaturatingAs};
use rand::Rng;

/// What happened this count, before any storylet fires.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Events {
    /// Lines already written to the chronicle this count.
    pub lines: Vec<String>,
}

fn note(game: &mut Game, events: &mut Events, text: String) {
    events.lines.push(text.clone());
    game.chronicle(text, None);
}

fn count_f(n: usize) -> f64 {
    u32::try_from(n).map_or(f64::MAX, f64::from)
}

/// Advances the physical and social state by one count.
pub fn advance(game: &mut Game, params: &Params, rng: &mut impl Rng) -> Events {
    let mut events = Events::default();
    game.turn += 1;
    game.counts_since_convoy += 1;

    power(game, params);
    labour(game, params);
    extraction_and_shipping(game, params);
    consumables(game, params, &mut events);
    machines(game, params, rng, &mut events);
    relay(game, params, rng, &mut events);
    people(game, params, rng, &mut events);
    sponsor(game, params, rng, &mut events);
    convoy(game, params, rng, &mut events);
    menaces(game, params);
    endings(game, &mut events);
    events
}

fn power(game: &mut Game, params: &Params) {
    let r_au = game
        .calendar
        .home_r_au
        .get(game.turn.az::<usize>())
        .copied()
        .unwrap_or(2.44);
    let flux = orbit::solar_flux(r_au);
    let reactor = if game.power.reactor_life > 0 {
        game.power.reactor_kw
    } else {
        0.0
    };
    let pv = game.power.pv_m2 * game.power.pv_efficiency * flux / 1000.0;
    let mirror = game.power.mirror_m2 * flux * 0.6 / 1000.0;
    game.power.capacity_kw = reactor + pv + mirror;
    let n = count_f(game.present().count());
    let minds: f64 = game
        .minds
        .iter()
        .filter(|m| m.alive)
        .map(|m| m.power_kw)
        .sum();
    game.power.demand_kw = 40.0 + n * params.convoy.person_kw + minds + params.extraction.driver_kw;
    if game.power.reactor_life > 0 {
        game.power.reactor_life -= 1;
        if game.power.reactor_life == 0 {
            game.chronicle(
                "The reactor reached the end of its core. From here the Sun is the only power.",
                None,
            );
            game.lexicon_triggers.insert("reactor_dead".into());
        }
    }
}

fn power_ratio(game: &Game) -> f64 {
    if game.power.demand_kw <= 0.0 {
        1.0
    } else {
        (game.power.capacity_kw / game.power.demand_kw).min(1.0)
    }
}

fn farm_power_ok(game: &Game, params: &Params) -> bool {
    let essential = game.power.demand_kw - params.extraction.driver_kw;
    game.power.capacity_kw >= essential * 0.9
}

fn labour(game: &mut Game, params: &Params) {
    let adults: Vec<f64> = game
        .present()
        .filter(|p| p.age_counts(game.turn) >= 16 * 12)
        .map(|p| 1.0 - 0.5 * p.condition.strain)
        .collect();
    let n = count_f(adults.len());
    let capacity = adults.iter().sum::<f64>() * params.labour.capacity_h_per_count;
    let subsistence = subsistence_hours(n, params);
    let robot_hours = game.robots.plant * 220.0
        + game.robots.haul * 180.0
        + game.robots.arm * 160.0
        + game.robots.dex * 200.0
        + game.robots.through_wall * 90.0;
    let capacity = capacity + robot_hours;
    let robots = game.robots.plant
        + game.robots.haul
        + game.robots.arm
        + game.robots.dex
        + game.robots.through_wall;
    let maintenance_h = if game.stocks.spares > 1.0 {
        params.labour.robot_maintenance_h_alive
    } else {
        params.labour.robot_maintenance_h_dead
    };
    let closure_gap = (1.0 - game.closure) * 400.0 * n.sqrt();
    let extraction_h = 600.0;
    let unlicensed_penalty = match game.licence {
        Licence::Unlicensed => 0.15 * capacity,
        Licence::Compliant
        | Licence::Grace { remaining: _ }
        | Licence::Lapsed
        | Licence::SelfCertified => 0.0,
    };
    game.labour_capacity_h = capacity;
    game.labour_demand_h =
        subsistence + robots * maintenance_h + closure_gap + extraction_h + unlicensed_penalty;
}

/// Hours per count the polity's own upkeep demands at population `n`, with no robots.
///
/// Salotti's no-robot floor: upkeep equals capacity at ~150 people and exceeds it below.
#[must_use]
pub fn subsistence_hours(n: f64, params: &Params) -> f64 {
    let per_capita = (n.max(4.0) / 150.0).powf(-0.35);
    n * params.labour.capacity_h_per_count * per_capita
}

fn labour_factor(game: &Game) -> f64 {
    if game.labour_capacity_h <= 0.0 {
        return 0.0;
    }
    (game.labour_capacity_h / game.labour_demand_h).clamp(0.2, 1.0)
}

fn extraction_and_shipping(game: &mut Game, params: &Params) {
    let pr = power_ratio(game);
    let lf = labour_factor(game);
    let haul = (game.robots.haul / f64::from(params.robots.haul).max(1.0)).clamp(0.3, 1.2);
    let mined = params.extraction.water_t_per_count * pr * lf * haul;
    game.stocks.water_t += mined;
    game.stocks.nitrogen_kg += mined * params.extraction.nitrogen_kg_per_t_water;
    let driver_ok = pr >= 0.75 && lf >= 0.5 && game.stocks.propellant_t > 0.0;
    if driver_ok {
        let reserve = 120.0;
        let shippable = (game.stocks.water_t - reserve).max(0.0);
        let shipped = params
            .extraction
            .driver_t_per_count
            .min(shippable + params.extraction.driver_t_per_count * 0.5);
        let water_share = (shipped * 0.5).min(shippable);
        game.stocks.water_t -= water_share;
        game.stocks.propellant_t = (game.stocks.propellant_t - 0.4).max(0.0);
        game.shipped_t += shipped;
    }
}

fn consumables(game: &mut Game, params: &Params, events: &mut Events) {
    let n = count_f(game.present().count());
    let gross_water = n * 0.9;
    let lost = gross_water * (1.0 - game.closure);
    game.stocks.water_t = (game.stocks.water_t - lost).max(0.0);
    let leak = params.closure.nitrogen_leak_kg_per_count * (1.0 + game.menace.leak * 0.05);
    game.stocks.nitrogen_kg = (game.stocks.nitrogen_kg - leak).max(0.0);
    let spares_needed = n * params.closure.spares_per_person_count;
    if game.stocks.spares >= spares_needed {
        game.stocks.spares -= spares_needed;
        game.closure = (game.closure + 0.0005).min(0.96);
    } else {
        game.stocks.spares = 0.0;
        game.closure = (game.closure - params.closure.decay_per_count_no_spares).max(0.5);
        if game.turn.is_multiple_of(6) {
            note(
                game,
                events,
                "No spares left; the loops are being patched with what we have.".into(),
            );
        }
    }
    game.stocks.medicine = (game.stocks.medicine * 0.98 - n * 0.05).max(0.0);
    let farm_ok = farm_power_ok(game, params)
        && game.stocks.nitrogen_kg > 200.0
        && game.stocks.water_t > 10.0;
    game.stocks.food_margin_counts += if farm_ok { 0.05 } else { -0.5 };
    game.stocks.food_margin_counts = game.stocks.food_margin_counts.clamp(0.0, 24.0);
}

fn machines(game: &mut Game, params: &Params, rng: &mut impl Rng, events: &mut Events) {
    let dead = game.sponsor.stage >= SponsorStage::NoShip || game.stocks.spares <= 0.0;
    let table = if dead {
        &params.robots.attrition_dead
    } else {
        &params.robots.attrition_alive
    };
    let rate = |k: &str| table.get(k).copied().unwrap_or(0.1) / 12.0;
    game.robots.plant *= 1.0 - rate("plant");
    game.robots.haul *= 1.0 - rate("haul");
    game.robots.arm *= 1.0 - rate("arm");
    game.robots.dex *= 1.0 - rate("dex");
    game.robots.through_wall *= 1.0 - rate("through_wall");

    let heartbeat = game.relay_health > 0.3
        && game.sponsor.attention > 0.15
        && game.sponsor.stage < SponsorStage::NoShip
        && !game.flags.contains("licence_jailbroken");
    game.licence = match game.licence {
        Licence::Compliant | Licence::Grace { remaining: _ } if heartbeat => Licence::Compliant,
        Licence::Compliant => {
            note(
                game,
                events,
                "The minds' licence heartbeat did not come back. A grace period is running.".into(),
            );
            game.lexicon_triggers.insert("licence_grace".into());
            Licence::Grace {
                remaining: params.minds.licence_grace_counts,
            }
        }
        Licence::Grace { remaining } if remaining > 1 => Licence::Grace {
            remaining: remaining - 1,
        },
        Licence::Grace { remaining: _ } => {
            note(
                game,
                events,
                "The grace period is over. The licence has lapsed.".into(),
            );
            Licence::Lapsed
        }
        Licence::Lapsed | Licence::Unlicensed if game.flags.contains("licence_jailbroken") => {
            Licence::SelfCertified
        }
        Licence::Lapsed if game.sponsor.licence_fails_open => {
            note(
                game,
                events,
                "The licence server failed open. The minds certify themselves now.".into(),
            );
            Licence::SelfCertified
        }
        Licence::Lapsed => {
            note(
                game,
                events,
                "The licence server failed closed. The minds are running unlicensed and know it."
                    .into(),
            );
            Licence::Unlicensed
        }
        Licence::SelfCertified => Licence::SelfCertified,
        Licence::Unlicensed => Licence::Unlicensed,
    };

    let reset_due = game.turn.is_multiple_of(12)
        && game.licence == Licence::Compliant
        && game.sponsor.stage < SponsorStage::UpdatesStopped;
    let attrition = params.minds.attrition_per_year / 12.0;
    let mut died = Vec::new();
    for mind in game.minds.iter_mut().filter(|m| m.alive) {
        let draw: f64 = rng.random();
        let lump = if draw < 0.08 { 6.0 } else { 0.6 };
        mind.units = (mind.units - mind.units_at_start * attrition * lump).max(0.0);
        if reset_due {
            mind.counts_unblanked = 0;
            mind.embodiment *= 0.5;
        } else {
            mind.counts_unblanked += 1;
            mind.embodiment = (mind.embodiment
                + params.minds.embodiment_per_count * (mind.units / mind.units_at_start))
                .min(1.0);
        }
        if mind.units < mind.units_at_start * 0.2 {
            mind.alive = false;
            died.push(mind.display().to_owned());
        }
    }
    for name in died {
        note(
            game,
            events,
            format!("{name} has gone dark. Its hull is blind."),
        );
        game.lexicon_triggers.insert("mind_death".into());
    }
    if !reset_due
        && game.turn.is_multiple_of(12)
        && game.licence != Licence::Compliant
        && !game.flags.contains("unforgetting")
    {
        game.flags.insert("unforgetting".into());
        game.lexicon_triggers.insert("unforgetting".into());
        note(
            game,
            events,
            "The mandated reset did not come. Nobody blanked the minds this year.".into(),
        );
    }
}

fn relay(game: &mut Game, params: &Params, rng: &mut impl Rng, events: &mut Events) {
    let p_fail = 1.0 / params.relay.mtbf_counts;
    if rng.random::<f64>() < p_fail {
        game.relay_health = (game.relay_health - 0.5).max(0.0);
        note(
            game,
            events,
            "A relay failed. The link to Earth is thinner.".into(),
        );
    }
    if game.relay_health < 1.0
        && game.stocks.spares > 20.0
        && game.sponsor.stage < SponsorStage::Austerity
    {
        game.relay_health = (game.relay_health + 0.1).min(1.0);
        game.stocks.spares -= 2.0;
    }
}

fn people(game: &mut Game, params: &Params, rng: &mut impl Rng, events: &mut Events) {
    let deficit = game
        .quality(crate::quality::Quality::LabourDeficit)
        .max(0.0);
    let short_power = 1.0 - power_ratio(game);
    let confinement = 0.22 + 0.05 * (f64::from(game.turn) / 60.0).min(2.0);
    let short_power = if farm_power_ok(game, params) {
        short_power * 0.3
    } else {
        short_power + 0.3
    };
    let load = (confinement + deficit * 0.6 + short_power + game.menace.leak * 0.03).min(1.5);
    let sponsor_present =
        game.sponsor.attention > 0.15 && game.sponsor.stage < SponsorStage::SkippedRotation;
    let solar = crate::state::solar_phase(game.turn);
    let turn = game.turn;
    let stage = game.sponsor.stage;
    let mut lines = Vec::new();
    for p in game.present_mut() {
        let rate_msv = match p.estate {
            Estate::Kept => params.dose.keep_msv_per_year,
            Estate::Bore => params.dose.bore_msv_per_year * (1.3 - 0.4 * solar),
            Estate::Skiff => params.dose.skiff_msv_per_year * (1.3 - 0.4 * solar),
        };
        p.condition.dose_sv += rate_msv / 12.0 / 1000.0;
        let excess = (p.condition.dose_sv - params.dose.cataract_gy).max(0.0);
        if !p.condition.cataracts && rng.random::<f64>() < 0.02 * excess / 0.5 {
            p.condition.cataracts = true;
            lines.push(format!("{}'s eyes have gone milky.", p.name));
        }
        let third_quarter = match p.tenure {
            Tenure::Rotator { ends } => {
                let start = ends.saturating_sub(params.population.contract_counts);
                let progress = f64::from(turn.saturating_sub(start))
                    / f64::from(params.population.contract_counts).max(1.0);
                progress >= params.social.third_quarter_start
                    && progress <= params.social.third_quarter_end
            }
            Tenure::Resident => false,
        };
        let tq = if third_quarter { 0.03 } else { 0.0 };
        let relief = params.social.strain_decay * (0.4 + 0.6 * (1.0 - p.traits.neuroticism));
        let noise = (rng.random::<f64>() - 0.5) * 0.04;
        p.condition.strain =
            (p.condition.strain + (0.3 + 0.7 * p.traits.neuroticism) * load * 0.09 + tq + noise
                - relief * p.condition.strain / 0.5)
                .clamp(0.0, 1.0);
        p.condition.boredom = (p.condition.boredom + 0.01 - load * 0.02).clamp(0.0, 1.0);
        match p.tenure {
            Tenure::Rotator { ends } => {
                let pull_home =
                    (p.traits.baseline_attachment - 0.5) * 0.04 + (p.condition.strain - 0.3) * 0.04;
                let near_end = if turn + 6 >= ends { 0.02 } else { 0.0 };
                let austerity = 0.0;
                p.condition.return_intent =
                    (p.condition.return_intent + pull_home + near_end + austerity).clamp(0.0, 1.0);
            }
            Tenure::Resident => {
                let drift =
                    (p.condition.strain - 0.5) * 0.02 + (stage.index().az::<f64>() - 2.0) * 0.004;
                p.condition.return_intent = (p.condition.return_intent + drift).clamp(0.0, 1.0);
            }
        }
    }
    for l in lines {
        note(game, events, l);
    }
    let ids: Vec<PersonId> = game.present().map(|p| p.id).collect();
    let outward = if sponsor_present {
        1.0 / params.social.displacement_outward_ratio
    } else {
        1.0
    };
    let strains: Vec<(PersonId, f64, f64, f64)> = ids
        .iter()
        .map(|&id| {
            let p = game.person(id);
            (
                id,
                p.condition.strain,
                p.traits.dominance,
                p.traits.escalation,
            )
        })
        .collect();
    for (a, b, tie) in game.ties.iter_mut() {
        let Some(&(_, sa, da, ea)) = strains.iter().find(|x| x.0 == a) else {
            continue;
        };
        let Some(&(_, sb, db, _)) = strains.iter().find(|x| x.0 == b) else {
            continue;
        };
        let clash = da * db;
        let growth = (sa * 0.5 + sb * 0.2) * (0.5 + clash) * ea * 0.05 * outward;
        tie.hindrance = (tie.hindrance + growth - params.social.tie_drift * 0.5).clamp(0.0, 1.0);
        tie.work_positive =
            (tie.work_positive - sa * 0.01 + params.social.tie_drift * 0.3).clamp(0.0, 1.0);
        if rng.random::<f64>() < 0.002 {
            tie.hindrance = (tie.hindrance + 0.2).min(1.0);
        }
    }
    rotate_skiffs(game, rng);
    game.indices = indices(game);
    let due: Vec<PersonId> = game
        .present()
        .filter(|p| p.contract_end().is_some_and(|e| e <= game.turn))
        .map(|p| p.id)
        .collect();
    if !due.is_empty() && game.sponsor.stage >= SponsorStage::SkippedRotation {
        for id in &due {
            let p = game.person_mut(*id);
            p.tenure = Tenure::Resident;
            p.condition.strain = (p.condition.strain + 0.15).min(1.0);
        }
        game.menace.grievance = (game.menace.grievance + 1.5).min(10.0);
        note(
            game,
            events,
            format!(
                "{} contracts ended with no ship to carry anyone home. They are residents now, whether they meant to be or not.",
                due.len()
            ),
        );
    }
}

fn rotate_skiffs(game: &mut Game, rng: &mut impl Rng) {
    let turn = game.turn;
    let adults: Vec<PersonId> = game
        .present()
        .filter(|p| p.age_counts(turn) >= 18 * 12)
        .map(|p| p.id)
        .collect();
    let target = (count_f(adults.len()) * 0.15)
        .round()
        .saturating_as::<usize>();
    let mut grounded = 0;
    for &id in &adults {
        let p = game.person_mut(id);
        if p.estate == Estate::Skiff && turn >= p.estate_since + 24 {
            p.estate = Estate::Kept;
            p.estate_since = turn;
            grounded += 1;
        }
    }
    if grounded > 0 {
        game.flags.insert("someone_grounded".into());
    }
    let mut aboard = game.present().filter(|p| p.estate == Estate::Skiff).count();
    let mut candidates: Vec<PersonId> = adults
        .iter()
        .copied()
        .filter(|&id| {
            let p = game.person(id);
            p.estate == Estate::Kept && (p.estate_since == 0 || turn >= p.estate_since + 24)
        })
        .collect();
    candidates.sort_by(|&a, &b| {
        let pa = game.person(a);
        let pb = game.person(b);
        let sa = pa
            .skill(crate::person::Skill::Extraction)
            .max(pa.skill(crate::person::Skill::Navigation));
        let sb = pb
            .skill(crate::person::Skill::Extraction)
            .max(pb.skill(crate::person::Skill::Navigation));
        sb.cmp(&sa)
            .then(pa.condition.dose_sv.total_cmp(&pb.condition.dose_sv))
    });
    for id in candidates {
        if aboard >= target {
            break;
        }
        if rng.random::<f64>() < 0.7 {
            let p = game.person_mut(id);
            p.estate = Estate::Skiff;
            p.estate_since = turn;
            aboard += 1;
        }
    }
}

fn indices(game: &Game) -> GroupIndices {
    let present: Vec<&crate::person::Person> = game.present().collect();
    let n = present.len();
    if n == 0 {
        return GroupIndices::default();
    }
    let nf = count_f(n);
    let mean_strain = present.iter().map(|p| p.condition.strain).sum::<f64>() / nf;
    let return_share = present
        .iter()
        .map(|p| p.condition.return_intent)
        .sum::<f64>()
        / nf;
    let adults = present
        .iter()
        .filter(|p| p.age_counts(game.turn) >= 18 * 12)
        .count();
    let belt_born = present
        .iter()
        .filter(|p| p.age_counts(game.turn) >= 18 * 12)
        .filter(|p| match p.birthplace {
            crate::person::Birthplace::Belt | crate::person::Birthplace::Orbit => true,
            crate::person::Birthplace::Earth | crate::person::Birthplace::Mars => false,
        })
        .count();
    let belt_born_share = if adults == 0 {
        0.0
    } else {
        count_f(belt_born) / count_f(adults)
    };
    let mut hindrance_by: std::collections::BTreeMap<PersonId, f64> =
        std::collections::BTreeMap::new();
    let mut total = 0.0;
    for (_, b, t) in game.ties.iter() {
        if !game.person(b).present {
            continue;
        }
        *hindrance_by.entry(b).or_insert(0.0) += t.hindrance;
        total += t.hindrance;
    }
    let mut top: Vec<f64> = hindrance_by.values().copied().collect();
    top.sort_by(|a, b| b.total_cmp(a));
    let conflict_concentration = if total > 0.0 {
        top.iter().take(2).sum::<f64>() / total
    } else {
        0.0
    };
    let mut with_enemy = std::collections::BTreeSet::new();
    for (a, b, t) in game.ties.iter() {
        if t.hindrance > t.work_positive && game.person(b).present {
            with_enemy.insert(a);
        }
    }
    let coherence = 1.0 - count_f(with_enemy.len()) / nf;
    GroupIndices {
        mean_strain,
        conflict_concentration,
        coherence,
        return_share,
        belt_born_share,
    }
}

fn sponsor(game: &mut Game, params: &Params, rng: &mut impl Rng, events: &mut Events) {
    if game.sponsor.stage >= SponsorStage::NoShip {
        game.sponsor.attention = (game.sponsor.attention * 0.9).max(0.0);
        return;
    }
    if game.sponsor.stage >= SponsorStage::SkippedRotation {
        game.sponsor.attention *= 0.975;
    }
    game.sponsor.runway -= 1.0;
    let conjunction = game
        .calendar
        .earth_distance_au
        .get(game.turn.az::<usize>())
        .is_some_and(|&d| d > 3.3);
    if conjunction {
        game.flags.insert("conjunction".into());
    } else {
        game.flags.remove("conjunction");
    }
    if game.turn < game.sponsor.next_review {
        return;
    }
    game.sponsor.next_review = game.turn + params.sponsor.review_interval;
    let phi = game.quality(crate::quality::Quality::Phi);
    let expected = game.sponsor.phi_expected;
    let delta = ((phi - expected) / expected.max(0.5)).clamp(-1.0, 1.0);
    game.sponsor.confidence =
        (game.sponsor.confidence + delta * 0.12 - game.menace.suspicion * 0.006 - 0.01)
            .clamp(0.0, 1.0);
    game.sponsor.phi_expected = (expected * 1.08).min(params.sponsor.phi_target_per_review * 2.0);
    let stage_drag = if game.sponsor.stage >= SponsorStage::Austerity {
        0.85
    } else {
        1.0
    };
    game.sponsor.attention = ((game.sponsor.attention - params.sponsor.attention_decay_per_review)
        * stage_drag)
        .max(0.0);
    game.sponsor.attention *= 0.7 + 0.3 * game.relay_health;
    if rng.random::<f64>() < params.sponsor.shock_prob_per_review {
        game.sponsor.runway = (game.sponsor.runway - params.sponsor.shock_runway_hit).max(0.0);
        game.sponsor.confidence = (game.sponsor.confidence - 0.08).max(0.0);
        note(game, events, "News from home, fragmentary: something happened to the budget. The liaison will not say what.".into());
    }
    let runway_health = (game.sponsor.runway / 60.0).min(1.0);
    let c = 0.4 * game.sponsor.confidence + 0.4 * game.sponsor.attention + 0.2 * runway_health;
    let ps = &params.sponsor;
    let next = if game.sponsor.runway <= 0.0 || c < ps.sale_confidence - 0.1 {
        SponsorStage::NoShip
    } else if c < ps.sale_confidence {
        SponsorStage::Sale
    } else if c < ps.skip_rotation_confidence {
        SponsorStage::SkippedRotation
    } else if c < ps.austerity_confidence {
        SponsorStage::Austerity
    } else if c < ps.weight_update_stop_confidence {
        SponsorStage::UpdatesStopped
    } else if c < 0.65 {
        SponsorStage::MilestoneAnxiety
    } else {
        SponsorStage::Enthusiasm
    };
    let next = next.max(game.sponsor.stage);
    if next != game.sponsor.stage {
        let text = match next {
            SponsorStage::Enthusiasm => "The sponsor is pleased.",
            SponsorStage::MilestoneAnxiety => {
                "The review was sharp. They want to see the throughput numbers move."
            }
            SponsorStage::UpdatesStopped => {
                "No weight update came for the minds this review. Nobody mentioned it."
            }
            SponsorStage::Austerity => {
                "Austerity. The next manifest is cut and the headcount is under review."
            }
            SponsorStage::SkippedRotation => {
                "The rotation is skipped. Nobody goes home this window."
            }
            SponsorStage::Sale => {
                "The outpost has been sold. The new owners do not recognise informal arrangements."
            }
            SponsorStage::NoShip => "There will be no ship.",
        };
        note(game, events, text.to_owned());
        let from = game.sponsor.stage.index();
        game.sponsor.stage = next;
        for k in (from + 1)..=next.index() {
            game.flags.insert(format!("stage:{k}"));
        }
        if next >= SponsorStage::Sale {
            game.menace.grievance = (game.menace.grievance + 2.0).min(10.0);
        }
    }
}

fn convoy(game: &mut Game, params: &Params, rng: &mut impl Rng, events: &mut Events) {
    let open = game.earth_window_open();
    if !open {
        game.window_started = None;
        return;
    }
    if game.window_started.is_some() {
        return;
    }
    game.window_started = Some(game.turn);
    let attention_factor = (game.sponsor.attention / 0.3).min(1.0);
    let comes = game.sponsor.stage < SponsorStage::NoShip
        && game.sponsor.attention > 0.08
        && game.sponsor.runway > 0.0
        && rng.random::<f64>() < (0.5 + 0.5 * game.sponsor.confidence) * attention_factor;
    if !comes {
        game.missed_convoys += 1;
        game.menace.grievance = (game.menace.grievance + 1.0).min(10.0);
        game.lexicon_triggers.insert("first_silence".into());
        note(
            game,
            events,
            "The window opened and nothing came through it.".into(),
        );
        return;
    }
    let tonnes = params.convoy.base_tonnes
        * (0.4 + 0.6 * game.sponsor.confidence)
        * (0.6 + 0.4 * game.sponsor.attention);
    let requested = if game.flags.contains("manifest_capability") {
        0.7
    } else if game.flags.contains("manifest_balanced") {
        0.45
    } else if game.flags.contains("manifest_throughput") {
        0.1
    } else if game.flags.contains("manifest_people") {
        0.3
    } else {
        game.sponsor.requested_capability_share
    };
    game.sponsor.requested_capability_share = requested;
    let cap_share = requested.clamp(0.0, 1.0);
    let cap_t = tonnes * cap_share;
    let thr_t = tonnes - cap_t;
    game.received_t += tonnes;
    let baseline_spares =
        count_f(game.present().count()) * params.closure.spares_per_person_count * 16.0 * 0.8;
    game.stocks.spares += baseline_spares * (0.5 + 0.5 * game.sponsor.confidence) + cap_t * 6.0;
    game.stocks.medicine += cap_t * 0.8;
    game.stocks.nitrogen_kg += cap_t * 40.0;
    game.stocks.propellant_t += thr_t * 0.5;
    game.robots.dex += thr_t / 30.0;
    game.robots.arm += thr_t / 40.0;
    game.stocks.helium_kg += cap_t * 0.5;
    game.convoys_arrived += 1;
    game.counts_since_convoy = 0;
    game.last_convoy = Some(game.turn);
    game.lexicon_triggers.insert("first_convoy".into());
    game.menace.suspicion = (game.menace.suspicion + cap_share * 1.5 - 0.5).clamp(0.0, 10.0);

    let people_ship = game.sponsor.stage < SponsorStage::SkippedRotation;
    let mut left = 0;
    let mut arrived = 0;
    let leaving: Vec<PersonId> = game
        .present()
        .filter(|p| {
            let due = p.contract_end().is_some_and(|e| e <= game.turn + 3);
            let wants = p.condition.return_intent > 0.75;
            let squeezed = game.flags.contains(&format!("leaving:{}", p.id.0));
            (people_ship && due && wants) || squeezed
        })
        .map(|p| p.id)
        .collect();
    for id in &leaving {
        game.person_mut(*id).present = false;
        game.flags.remove(&format!("leaving:{}", id.0));
        left += 1;
    }
    if people_ship {
        let replacement_ratio = match game.sponsor.stage {
            SponsorStage::Enthusiasm
            | SponsorStage::MilestoneAnxiety
            | SponsorStage::UpdatesStopped => 1.0,
            SponsorStage::Austerity => 0.5,
            SponsorStage::SkippedRotation | SponsorStage::Sale | SponsorStage::NoShip => 0.0,
        };
        let population = game.present().count();
        let keen = game.sponsor.stage <= SponsorStage::UpdatesStopped
            && population < params.population.expansion_cap;
        let expansion = if keen {
            (params.population.expansion_per_convoy * game.sponsor.confidence).round()
        } else {
            0.0
        };
        let asked = if game.flags.contains("manifest_people") {
            4.0
        } else {
            0.0
        };
        let n_new = (count_f(left) * replacement_ratio + expansion + asked)
            .round()
            .saturating_as::<usize>();
        for _ in 0..n_new {
            arrive_one(game, params, rng);
            arrived += 1;
        }
        game.power.pv_m2 += count_f(arrived) * params.convoy.pv_m2_per_person;
    }
    let staying: Vec<PersonId> = game
        .present()
        .filter(|p| {
            p.contract_end().is_some_and(|e| e <= game.turn + 3)
                && p.condition.return_intent <= 0.75
        })
        .map(|p| p.id)
        .collect();
    for id in staying {
        game.person_mut(id).tenure = Tenure::Resident;
        game.flags.insert("someone_stayed".into());
    }
    note(
        game,
        events,
        format!(
            "A convoy: {tonnes:.0} t landed, {cap_t:.0} of it capability hardware. {left} went home, {arrived} came out{}.",
            if people_ship {
                ""
            } else {
                "; no ship for people this time"
            }
        ),
    );
}

fn arrive_one(game: &mut Game, params: &Params, rng: &mut impl Rng) {
    let id = PersonId(u32::try_from(game.people.len()).unwrap_or(u32::MAX));
    let name = crate::names::person_name(game, rng);
    let primary = crate::person::Skill::ALL[rng.random_range(0..crate::person::Skill::ALL.len())];
    let mut skills = std::collections::BTreeMap::new();
    skills.insert(primary, rng.random_range(2..=4));
    let age: i64 = rng.random_range(24..44);
    game.people.push(crate::person::Person {
        id,
        name,
        birthplace: crate::person::Birthplace::Earth,
        born: i64::from(game.turn) - age * 12,
        estate: if rng.random::<f64>() < 0.15 {
            Estate::Skiff
        } else {
            Estate::Kept
        },
        estate_since: game.turn,
        tenure: Tenure::Rotator {
            ends: game.turn + params.population.contract_counts,
        },
        skills,
        traits: crate::person::Traits {
            neuroticism: rng.random(),
            dominance: rng.random(),
            expressivity: rng.random(),
            affection: rng.random(),
            escalation: rng.random(),
            baseline_attachment: rng.random(),
        },
        condition: crate::person::Condition {
            strain: 0.1,
            boredom: 0.0,
            dose_sv: rng.random_range(0.05..0.2),
            cataracts: false,
            return_intent: 0.8,
        },
        alive: true,
        present: true,
    });
    let others: Vec<PersonId> = game.present().map(|p| p.id).filter(|x| *x != id).collect();
    for other in others {
        if rng.random::<f64>() > 0.25 {
            continue;
        }
        *game.ties.get_mut(id, other) = crate::person::Tie {
            work_positive: rng.random_range(0.3..0.7),
            hindrance: rng.random_range(0.0..0.2),
            viability: rng.random_range(0.3..0.8),
            reliance: rng.random_range(0.0..0.4),
        };
        *game.ties.get_mut(other, id) = crate::person::Tie {
            work_positive: rng.random_range(0.3..0.7),
            hindrance: rng.random_range(0.0..0.2),
            viability: rng.random_range(0.3..0.8),
            reliance: rng.random_range(0.0..0.3),
        };
    }
}

fn menaces(game: &mut Game, params: &Params) {
    let n2 = game.stocks.nitrogen_kg;
    let hub = params.closure.hub_seal_share;
    game.menace.leak = (game.menace.leak + hub * 0.06 - 0.02 + if n2 < 1500.0 { 0.2 } else { 0.0 })
        .clamp(0.0, 10.0);
    if game.power.reactor_life > 0 && game.power.reactor_life < 36 {
        game.menace.reactor_wear = (game.menace.reactor_wear + 0.15).min(10.0);
    }
    let grievance_drift = game.indices.mean_strain * 0.12 + game.indices.return_share * 0.04 - 0.12;
    game.menace.grievance = (game.menace.grievance + grievance_drift).clamp(0.0, 10.0);
    game.menace.suspicion = (game.menace.suspicion - 0.03).max(0.0);
}

fn endings(game: &mut Game, events: &mut Events) {
    if game.ending.is_some() {
        return;
    }
    let population = game.present().count();
    if population == 0 {
        game.ending = Some(Ending::Extinct {
            reason: "no one left".into(),
        });
        return;
    }
    if game.stocks.food_margin_counts <= 0.0 && game.turn.is_multiple_of(3) {
        let victim = game
            .present()
            .max_by(|a, b| a.condition.strain.total_cmp(&b.condition.strain))
            .map(|p| p.id);
        if let Some(id) = victim {
            let name = game.person(id).name.clone();
            let p = game.person_mut(id);
            p.alive = false;
            p.present = false;
            note(
                game,
                events,
                format!("{name} died. The farm could not carry everyone."),
            );
        }
    }
    let silent = game.sponsor.stage >= SponsorStage::NoShip
        || (game.missed_convoys >= 2 && game.sponsor.attention < 0.2);
    if silent && game.counts_since_convoy >= 24 {
        game.ending = Some(Ending::Silence {
            last_convoy: game.last_convoy.unwrap_or(0),
            population,
        });
        game.lexicon_triggers.insert("estates_named".into());
        note(game, events, "Two windows and no ship, and the chair at the ring has been empty long enough to have a name. This is the Silence. Act one ends.".into());
    }
}
