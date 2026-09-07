//! The structured view a front end renders.
//!
//! Clocks, projects, the hand, the ledger, the pressures, the ring, the sponsor's
//! track, the chronicle. Numbers come with the one-sentence "why" only the engine can
//! write.

use crate::lexicon;
use crate::project::{DieView, Hand};
use crate::quality::Quality;
use crate::ring::{Seat, seats};
use crate::state::{Game, Licence, SponsorStage};
use az::Az;
use serde::{Deserialize, Serialize};

/// A countdown clock.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Countdown {
    /// Identifier.
    pub id: String,
    /// Label in the current register.
    pub label: String,
    /// Segments filled.
    pub filled: u32,
    /// Segments total.
    pub segments: u32,
    /// Counts remaining, if the clock is a countdown to something.
    pub remaining: Option<u32>,
    /// Whether the clock is in its last three counts.
    pub urgent: bool,
    /// One sentence.
    pub why: String,
}

/// A project clock with its dice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectView {
    /// Identifier.
    pub id: String,
    /// Title.
    pub title: String,
    /// Description.
    pub description: String,
    /// Domain skill key.
    pub domain: String,
    /// Standing (never completes).
    pub standing: bool,
    /// Structured (any robot fits).
    pub structured: bool,
    /// Segments filled.
    pub filled: u32,
    /// Segments total (0 for standing).
    pub segments: u32,
    /// Rate this count (standing), 0-1.5.
    pub rate: f64,
    /// Pips delivered this count.
    pub pips: u32,
    /// Pips for full rate (standing).
    pub pips_needed: f64,
    /// Dice on it.
    pub dice: Vec<DieView>,
    /// Bonus, setback, or plain this count.
    pub roll: String,
}

/// One ledger bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    /// Identifier.
    pub id: String,
    /// Label in the act-1 register.
    pub label: String,
    /// Value.
    pub value: f64,
    /// Unit.
    pub unit: String,
    /// A reference for the bar's fill: capacity, reserve line, or a comfortable level.
    pub full: f64,
    /// Change since last count (approximate: from the stored previous value).
    pub delta: f64,
    /// One sentence saying why.
    pub why: String,
    /// A word for the state: nominal, tight, patched, failing, or the like.
    pub word: String,
}

/// A pressure with named bands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pressure {
    /// Identifier.
    pub id: String,
    /// Label.
    pub label: String,
    /// 0-10.
    pub value: f64,
    /// Band index 0-3.
    pub band: usize,
    /// Band name.
    pub band_name: String,
    /// Threshold for the must scene.
    pub threshold: f64,
}

/// A ring seat as displayed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeatView {
    /// Seat key.
    pub seat: String,
    /// The question the seat answers.
    pub question: String,
    /// Job title.
    pub title: String,
    /// Holder's name, if the seat is filled.
    pub holder: Option<String>,
    /// Holder's person id.
    pub holder_id: Option<u32>,
    /// Holder's skill in the seat's domain.
    pub skill: u8,
    /// Holder's strain.
    pub strain: f64,
    /// A mood word.
    pub mood: String,
    /// Whether the seat exists but is silent (the sponsor's chair).
    pub silent: bool,
}

/// The sponsor's track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SponsorView {
    /// Stage 0-6.
    pub stage: u8,
    /// Stage name in the act-1 register.
    pub stage_name: String,
    /// Confidence 0-1.
    pub confidence: f64,
    /// Attention 0-1.
    pub attention: f64,
    /// A mood word.
    pub mood: String,
    /// φ actual.
    pub phi: f64,
    /// φ expected at the next review.
    pub phi_expected: f64,
    /// Counts to the next review.
    pub counts_to_review: u32,
}

/// The whole view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    /// Count.
    pub turn: u32,
    /// Act.
    pub act: u8,
    /// Outpost name.
    pub outpost: String,
    /// Countdowns.
    pub countdowns: Vec<Countdown>,
    /// Projects.
    pub projects: Vec<ProjectView>,
    /// The hand.
    pub hand: Hand,
    /// Ledger bars.
    pub ledger: Vec<Bar>,
    /// Pressures.
    pub pressures: Vec<Pressure>,
    /// Ring seats.
    pub ring: Vec<SeatView>,
    /// Sponsor.
    pub sponsor: SponsorView,
    /// Controls as strings.
    pub controls: ControlsView,
    /// Whether the Earth window is open per count for the next 60 counts.
    pub calendar: Vec<bool>,
    /// The last chronicle entries, rendered.
    pub chronicle: Vec<String>,
    /// Lexicon triggers fired.
    pub words: Vec<String>,
    /// Flags, for the front's own gating (tutorial regions).
    pub flags: Vec<String>,
    /// Ending, if any.
    pub ending: Option<String>,
}

/// The standing controls as strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlsView {
    /// manifest split.
    pub manifest: String,
    /// throw position.
    pub throw: String,
    /// roster order.
    pub roster: String,
    /// auto-deal.
    pub auto_deal: bool,
}

fn word_for(value: f64, bands: &[(f64, &str)]) -> String {
    bands.iter().find(|(at, _)| value >= *at).map_or_else(
        || bands.last().map_or("", |b| b.1).to_owned(),
        |(_, w)| (*w).to_owned(),
    )
}

fn mood(strain: f64) -> &'static str {
    if strain < 0.25 {
        "steady"
    } else if strain < 0.45 {
        "tired"
    } else if strain < 0.65 {
        "worn"
    } else {
        "brittle"
    }
}

/// Builds the view.
#[must_use]
pub fn view(
    game: &Game,
    defs: &[crate::project::ProjectDef],
    params: &crate::params::Params,
) -> View {
    let q = |x: Quality| game.quality(x);
    let r = |s: &str| lexicon::render(game, s);
    let mut countdowns = Vec::new();
    let to_window = game.counts_to_window();
    countdowns.push(Countdown {
        id: "rsw".into(),
        label: r("RSW (resupply window)"),
        filled: 16u32.saturating_sub(to_window.min(16)),
        segments: 16,
        remaining: Some(to_window),
        urgent: to_window <= 3,
        why: if game.earth_window_open() {
            "The window is open. A ship can leave Earth for us this month.".into()
        } else {
            format!(
                "Best LEO-to-Fortuna cost is {:.1} km/s this month; the window opens at {:.1}.",
                q(Quality::EarthWindowCost),
                game.calendar.outbound_best * crate::state::WINDOW_RATIO
            )
        },
    });
    let to_review = game.sponsor.next_review.saturating_sub(game.turn);
    countdowns.push(Countdown {
        id: "review".into(),
        label: "Sponsor review".into(),
        filled: params.sponsor.review_interval.saturating_sub(to_review),
        segments: params.sponsor.review_interval,
        remaining: Some(to_review),
        urgent: to_review <= 3,
        why: format!(
            "Throughput ratio {:.1} against an expectation of {:.1}.",
            q(Quality::Phi),
            game.sponsor.phi_expected
        ),
    });
    let reactor = game.power.reactor_life;
    countdowns.push(Countdown {
        id: "reactor".into(),
        label: "Reactor core life".into(),
        filled: params.power.reactor_life_counts.saturating_sub(reactor),
        segments: params.power.reactor_life_counts,
        remaining: Some(reactor),
        urgent: reactor > 0 && reactor <= 36,
        why: if reactor == 0 {
            "Core exhausted. PV-A and concentrators only.".into()
        } else {
            format!("{reactor} months of core life. Not refuellable from belt material.")
        },
    });
    if let Some(next) = game
        .present()
        .filter_map(crate::person::Person::contract_end)
        .filter(|e| *e >= game.turn)
        .min()
    {
        let names: Vec<&str> = game
            .present()
            .filter(|p| p.contract_end() == Some(next))
            .map(|p| p.name.as_str())
            .take(3)
            .collect();
        countdowns.push(Countdown {
            id: "contract".into(),
            label: "Next contract end".into(),
            filled: params
                .population
                .contract_counts
                .saturating_sub(next - game.turn),
            segments: params.population.contract_counts,
            remaining: Some(next - game.turn),
            urgent: next - game.turn <= 3,
            why: format!("{} due for rotation at MM {next}.", names.join(", ")),
        });
    }
    let solar = q(Quality::SolarPhase);
    countdowns.push(Countdown {
        id: "sun".into(),
        label: "Solar cycle".into(),
        filled: (solar * 12.0).round().az::<u32>(),
        segments: 12,
        remaining: None,
        urgent: solar > 0.85,
        why: if solar > 0.7 {
            "Near solar maximum: storm risk up, galactic dose down.".into()
        } else {
            "Quiet Sun: galactic dose at its highest.".into()
        },
    });
    if let Licence::Grace { remaining } = game.licence {
        countdowns.push(Countdown {
            id: "grace".into(),
            label: "HRA licence grace".into(),
            filled: params.minds.licence_grace_counts.saturating_sub(remaining),
            segments: params.minds.licence_grace_counts,
            remaining: Some(remaining),
            urgent: true,
            why:
                "No heartbeat from the licence server. When grace ends the fail mode is discovered."
                    .into(),
        });
    }
    if game.flags.contains("conjunction") {
        countdowns.push(Countdown {
            id: "conjunction".into(),
            label: "Solar conjunction".into(),
            filled: 1,
            segments: 1,
            remaining: None,
            urgent: false,
            why: "Earth is behind the Sun. No link this month.".into(),
        });
    }

    let projects = game
        .projects
        .iter()
        .map(|s| {
            let def = defs.iter().find(|d| d.id == s.id);
            let dice: Vec<DieView> = game
                .hand
                .dice
                .iter()
                .filter(|d| d.place == s.id.0)
                .cloned()
                .collect();
            ProjectView {
                id: s.id.0.clone(),
                title: def.map_or_else(|| s.id.0.clone(), |d| r(&d.title)),
                description: def.map_or_else(String::new, |d| r(&d.description)),
                domain: def.map_or("", |d| d.domain.key()).to_owned(),
                standing: def.is_some_and(|d| d.standing),
                structured: def.is_some_and(|d| d.structured),
                filled: s.filled,
                segments: s.segments,
                rate: s.rate,
                pips: s.pips_this_count,
                pips_needed: def.map_or(0.0, |d| {
                    d.pips_needed + d.pips_per_person * game.present().count().az::<f64>()
                }),
                dice,
                roll: match s.roll {
                    crate::project::Roll::Plain => "plain",
                    crate::project::Roll::Bonus => "bonus",
                    crate::project::Roll::Setback => "setback",
                }
                .to_owned(),
            }
        })
        .collect();

    let n = q(Quality::Population);
    let water_loss = n * 0.9 * (1.0 - game.closure);
    let margin = q(Quality::Margin);
    let ledger = vec![
        Bar {
            id: "water".into(),
            label: "Water".into(),
            value: game.stocks.water_t,
            unit: "t".into(),
            full: 400.0,
            delta: 0.0,
            why: format!(
                "Losing {water_loss:.1} t a month to the closure gap; the reserve line is 120 t."
            ),
            word: word_for(
                game.stocks.water_t,
                &[
                    (200.0, "nominal"),
                    (120.0, "at reserve"),
                    (1.0, "below reserve"),
                    (0.0, "dry"),
                ],
            ),
        },
        Bar {
            id: "margin".into(),
            label: "CM-days (consumables margin)".into(),
            value: margin,
            unit: "months".into(),
            full: 24.0,
            delta: 0.0,
            why: format!(
                "The smaller of the farm's buffer and the N2 make-up time; LSS-C closure {:.2}.",
                game.closure
            ),
            word: word_for(
                game.closure,
                &[(0.93, "tight"), (0.85, "patched"), (0.0, "failing")],
            ),
        },
        Bar {
            id: "spares".into(),
            label: "Spares (vitamin parts)".into(),
            value: game.stocks.spares,
            unit: "crates".into(),
            full: 300.0,
            delta: 0.0,
            why: format!(
                "Burning {:.1} a month for {} people; the next RSW brings a baseline.",
                n * params.closure.spares_per_person_count,
                n
            ),
            word: word_for(
                game.stocks.spares,
                &[
                    (100.0, "nominal"),
                    (20.0, "low"),
                    (0.1, "critical"),
                    (0.0, "zero"),
                ],
            ),
        },
        Bar {
            id: "power".into(),
            label: "Power margin".into(),
            value: game.power.capacity_kw - game.power.demand_kw,
            unit: "kW".into(),
            full: game.power.demand_kw.max(1.0),
            delta: 0.0,
            why: format!(
                "{:.0} kW capacity against {:.0} kW demand; the MDLS draws {:.0} and is first to go dark.",
                game.power.capacity_kw, game.power.demand_kw, params.extraction.driver_kw
            ),
            word: if game.power.capacity_kw >= game.power.demand_kw {
                "nominal".into()
            } else if game.power.capacity_kw >= game.power.demand_kw - params.extraction.driver_kw {
                "driver dark".into()
            } else {
                "farm short".into()
            },
        },
        Bar {
            id: "hours".into(),
            label: "Hours (the hand)".into(),
            value: f64::from(game.hand.free),
            unit: "dice".into(),
            full: f64::from(game.hand.adults.max(1)),
            delta: 0.0,
            why: format!(
                "{} of {} free; {} eating and breathing; {} robot units, {} on upkeep.",
                game.hand.free,
                game.hand.adults,
                game.hand.eaten,
                game.hand.robots,
                game.hand
                    .dice
                    .iter()
                    .filter(|d| d.robot && d.place == "upkeep")
                    .count()
            ),
            word: if game.hand.shortfall > 0 {
                "short".into()
            } else if game.hand.free <= 2 {
                "thin".into()
            } else {
                "nominal".into()
            },
        },
        Bar {
            id: "throw".into(),
            label: "MDLS throughput (φ)".into(),
            value: q(Quality::Phi),
            unit: "t/t".into(),
            full: game.sponsor.phi_expected.max(1.0),
            delta: 0.0,
            why: format!(
                "{:.0} t shipped against {:.0} t received; the sponsor expects {:.1} at the next review.",
                game.shipped_t, game.received_t, game.sponsor.phi_expected
            ),
            word: if q(Quality::Phi) >= game.sponsor.phi_expected {
                "on target".into()
            } else {
                "short".into()
            },
        },
        Bar {
            id: "people".into(),
            label: "Crew".into(),
            value: n,
            unit: "people".into(),
            full: f64::from(params.population.expansion_cap.az::<u32>()),
            delta: 0.0,
            why: format!(
                "{} residents, {} on contract; mean CED {:.0} mSv.",
                q(Quality::Residents),
                q(Quality::Rotators),
                q(Quality::MeanDose) * 1000.0
            ),
            word: word_for(
                q(Quality::MeanStrain),
                &[
                    (0.6, "brittle"),
                    (0.4, "worn"),
                    (0.25, "tired"),
                    (0.0, "steady"),
                ],
            ),
        },
    ];

    let band = |v: f64, names: [&str; 4]| -> (usize, String) {
        let i = if v >= 7.0 {
            3
        } else if v >= 4.5 {
            2
        } else {
            usize::from(v >= 2.0)
        };
        (i, names[i].to_owned())
    };
    let pressures = [
        (
            "suspicion",
            "SOS (sponsor oversight status)",
            game.menace.suspicion,
            ["quiet", "noticed", "audited", "the audit"],
        ),
        (
            "grievance",
            "CCI (crew cohesion index)",
            game.menace.grievance,
            ["quiet", "muttering", "the moot", "the split"],
        ),
        (
            "leak",
            "N2 make-up",
            game.menace.leak,
            ["tight", "weeping", "rationed", "the collar"],
        ),
    ]
    .into_iter()
    .map(|(id, label, v, names)| {
        let (b, name) = band(v, names);
        Pressure {
            id: id.into(),
            label: r(label),
            value: v,
            band: b,
            band_name: r(&name),
            threshold: 7.0,
        }
    })
    .collect();

    let held = seats(game);
    let mut ring = Vec::new();
    for seat in Seat::ALL {
        let silent = seat == Seat::Liaison && game.sponsor.attention < 0.05;
        if !seat.exists(game) && !silent {
            continue;
        }
        let holder = held.get(&seat).map(|id| game.person(*id));
        ring.push(SeatView {
            seat: seat.key().into(),
            question: r(seat.question()),
            title: seat.title().into(),
            holder: holder.map(|p| p.name.clone()),
            holder_id: holder.map(|p| p.id.0),
            skill: holder.map_or(0, |p| p.skill(seat.skill())),
            strain: holder.map_or(0.0, |p| p.condition.strain),
            mood: holder.map_or("absent", |p| mood(p.condition.strain)).into(),
            silent,
        });
    }

    let stage_name = match game.sponsor.stage {
        SponsorStage::Enthusiasm => "nominal",
        SponsorStage::MilestoneAnxiety => "milestone review",
        SponsorStage::UpdatesStopped => "updates suspended",
        SponsorStage::Austerity => "budget directive",
        SponsorStage::SkippedRotation => "rotation deferred",
        SponsorStage::Sale => "change of sponsor",
        SponsorStage::NoShip => "no further arrivals",
    };
    let sponsor = SponsorView {
        stage: game.sponsor.stage.index(),
        stage_name: r(stage_name),
        confidence: game.sponsor.confidence,
        attention: game.sponsor.attention,
        mood: if game.sponsor.confidence > 0.6 {
            "satisfied"
        } else if game.sponsor.confidence > 0.35 {
            "impatient"
        } else {
            "cold"
        }
        .into(),
        phi: q(Quality::Phi),
        phi_expected: game.sponsor.phi_expected,
        counts_to_review: to_review,
    };
    let controls = ControlsView {
        manifest: match game.controls.manifest {
            crate::project::ManifestSplit::Throughput => "throughput",
            crate::project::ManifestSplit::Balanced => "balanced",
            crate::project::ManifestSplit::Capability => "capability",
            crate::project::ManifestSplit::People => "people",
        }
        .into(),
        throw: match game.controls.throw {
            crate::project::ThrowMode::Ship => "ship",
            crate::project::ThrowMode::HoldAtReserve => "hold",
            crate::project::ThrowMode::Stop => "stop",
        }
        .into(),
        roster: match game.controls.roster {
            crate::project::RosterOrder::Skill => "skill",
            crate::project::RosterOrder::Strain => "strain",
            crate::project::RosterOrder::Name => "name",
        }
        .into(),
        auto_deal: game.controls.auto_deal,
    };
    let calendar = (0..60)
        .map(|k| game.earth_window_open_at(game.turn + k))
        .collect();
    let chronicle = game
        .chronicle
        .iter()
        .rev()
        .take(40)
        .map(|e| format!("[{}] {}", e.turn, lexicon::render(game, &e.text)))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    View {
        turn: game.turn,
        act: game.act,
        outpost: game.outpost_name.clone(),
        countdowns,
        projects,
        hand: game.hand.clone(),
        ledger,
        pressures,
        ring,
        sponsor,
        controls,
        calendar,
        chronicle,
        words: game.lexicon_triggers.iter().cloned().collect(),
        flags: game.flags.iter().cloned().collect(),
        ending: game.ending.as_ref().map(|e| format!("{e:?}")),
    }
}
