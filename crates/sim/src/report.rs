//! Human-readable summaries of a game: the state the polity carries forward.

use crate::person::Birthplace;
use crate::quality::Quality;
use crate::ring::seats;
use crate::state::{Game, Licence};

/// A summary of the polity at this count, rendered through the lexicon.
#[must_use]
pub fn summary(game: &Game) -> String {
    let q = |x: Quality| game.quality(x);
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&crate::lexicon::render(game, &s));
        out.push('\n');
    };
    line(format!("{} at count {}", game.outpost_name, game.turn));
    let present: Vec<_> = game.present().collect();
    let earth = present
        .iter()
        .filter(|p| p.birthplace == Birthplace::Earth)
        .count();
    let mars = present
        .iter()
        .filter(|p| p.birthplace == Birthplace::Mars)
        .count();
    let orbit = present
        .iter()
        .filter(|p| p.birthplace == Birthplace::Orbit)
        .count();
    let belt = present
        .iter()
        .filter(|p| p.birthplace == Birthplace::Belt)
        .count();
    line(format!(
        "People: {} present ({} residents, {} on contract); born on Earth {}, Mars {}, in orbit {}, in the belt {}. Dead: {}.",
        present.len(),
        q(Quality::Residents),
        q(Quality::Rotators),
        earth,
        mars,
        orbit,
        belt,
        game.people.iter().filter(|p| !p.alive).count()
    ));
    line(format!(
        "Mean strain {:.2}, coherence {:.2}, conflict concentration {:.2}, mean dose {:.2} Sv, {} with cataracts.",
        q(Quality::MeanStrain),
        q(Quality::Coherence),
        q(Quality::ConflictConcentration),
        q(Quality::MeanDose),
        present.iter().filter(|p| p.condition.cataracts).count()
    ));
    line("The ring:".to_owned());
    for (seat, id) in seats(game) {
        let p = game.person(id);
        line(format!(
            "  {} — {} ({}, skill {})",
            seat.question(),
            p.name,
            seat.title(),
            p.skill(seat.skill())
        ));
    }
    let licence = match game.licence {
        Licence::Compliant => "compliant".to_owned(),
        Licence::Grace { remaining } => format!("in grace ({remaining} counts left)"),
        Licence::Lapsed => "lapsed".to_owned(),
        Licence::SelfCertified => "self-certified".to_owned(),
        Licence::Unlicensed => "unlicensed".to_owned(),
    };
    line(format!("Minds ({licence}):"));
    for m in &game.minds {
        line(format!(
            "  {} — {}, {:.0}% of its units, embodiment {:.2}, {} counts unblanked",
            m.display(),
            if m.alive { "alive" } else { "dark" },
            100.0 * m.units / m.units_at_start,
            m.embodiment,
            m.counts_unblanked
        ));
    }
    line(format!(
        "Robots: plant {:.1}, haul {:.1}, arm {:.1}, dex {:.1}, through-wall {:.1}.",
        game.robots.plant,
        game.robots.haul,
        game.robots.arm,
        game.robots.dex,
        game.robots.through_wall
    ));
    line(format!(
        "Stocks: water {:.0} t, propellant {:.0} t, nitrogen {:.0} kg, spares {:.0}, boron {:.0} kg, helium {:.0} kg, medicine {:.0}, food margin {:.0} counts. Closure {:.2}.",
        game.stocks.water_t,
        game.stocks.propellant_t,
        game.stocks.nitrogen_kg,
        game.stocks.spares,
        game.stocks.boron_kg,
        game.stocks.helium_kg,
        game.stocks.medicine,
        game.stocks.food_margin_counts,
        game.closure
    ));
    line(format!(
        "Power {:.0} of {:.0} kW; reactor {} counts left; relay {:.2}.",
        game.power.capacity_kw, game.power.demand_kw, game.power.reactor_life, game.relay_health
    ));
    line(format!(
        "Sponsor: stage {}, confidence {:.2}, attention {:.2}, runway {:.0}; {} convoys arrived, {} missed; shipped {:.0} t against {:.0} t received (φ {:.1}).",
        game.sponsor.stage.index(),
        game.sponsor.confidence,
        game.sponsor.attention,
        game.sponsor.runway,
        game.convoys_arrived,
        game.missed_convoys,
        game.shipped_t,
        game.received_t,
        q(Quality::Phi)
    ));
    line(format!(
        "Menaces: suspicion {:.1}, grievance {:.1}, leak {:.1}, reactor wear {:.1}.",
        game.menace.suspicion, game.menace.grievance, game.menace.leak, game.menace.reactor_wear
    ));
    let mut flags: Vec<&str> = game
        .flags
        .iter()
        .map(String::as_str)
        .filter(|f| !f.starts_with("leaving:"))
        .collect();
    flags.sort_unstable();
    line(format!("Flags: {}.", flags.join(", ")));
    let mut words: Vec<&str> = game.lexicon_triggers.iter().map(String::as_str).collect();
    words.sort_unstable();
    line(format!("Words in use after: {}.", words.join(", ")));
    out
}
