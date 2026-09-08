//! Storylets: the data-driven content format, its parser, and evaluation against a game.
//!
//! A storylet is a TOML document with `when` conditions, `cast` roles filled from the
//! roster, and `option`s whose `effect`s are typed edits to the state. Everything a
//! storylet can read or write is declared, so content can be validated and brute-forced.

use crate::person::{PersonId, Skill};
use crate::quality::Quality;
use crate::ring::Seat;
use crate::state::{Ending, Game};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Errors raised while loading content.
#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    /// TOML syntax or shape.
    #[error("{file}: {source}")]
    Toml {
        /// File name.
        file: String,
        /// Underlying error.
        #[source]
        source: toml::de::Error,
    },
    /// A reference to something that does not exist.
    #[error("{file}: {message}")]
    Invalid {
        /// File name.
        file: String,
        /// What is wrong.
        message: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStorylet {
    id: String,
    title: String,
    #[serde(default = "one")]
    act: u8,
    #[serde(default = "one_f")]
    weight: f64,
    #[serde(default)]
    once: bool,
    #[serde(default)]
    cooldown: u32,
    #[serde(default)]
    priority: i32,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    when: Vec<RawCondition>,
    #[serde(default)]
    cast: Vec<RawCast>,
    text: String,
    option: Vec<RawOption>,
}

const fn one() -> u8 {
    1
}
const fn one_f() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawCondition {
    q: Option<String>,
    lt: Option<f64>,
    le: Option<f64>,
    gt: Option<f64>,
    ge: Option<f64>,
    eq: Option<f64>,
    flag: Option<String>,
    not_flag: Option<String>,
    counter: Option<String>,
    turn_min: Option<u32>,
    turn_max: Option<u32>,
    window: Option<String>,
    fired: Option<String>,
    not_fired: Option<String>,
    stage_min: Option<u8>,
    stage_max: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCast {
    role: String,
    seat: Option<String>,
    skill: Option<String>,
    #[serde(default)]
    min_skill: u8,
    #[serde(default)]
    rotator: Option<bool>,
    #[serde(default)]
    optional: bool,
    estate: Option<String>,
    min_dose: Option<f64>,
    min_strain: Option<f64>,
    max_strain: Option<f64>,
    #[serde(default)]
    top: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawOption {
    id: String,
    label: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    requires: Vec<RawCondition>,
    #[serde(default)]
    effect: Vec<RawEffect>,
    chronicle: String,
    #[serde(default)]
    advice: Vec<RawAdvice>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEffect {
    q: Option<String>,
    add: Option<f64>,
    set: Option<f64>,
    mul: Option<f64>,
    flag: Option<String>,
    unflag: Option<String>,
    counter: Option<String>,
    person: Option<String>,
    strain: Option<f64>,
    dose: Option<f64>,
    return_intent: Option<f64>,
    leave: Option<bool>,
    die: Option<bool>,
    resident: Option<bool>,
    tie: Option<[String; 2]>,
    hindrance: Option<f64>,
    work_positive: Option<f64>,
    reliance: Option<f64>,
    lexicon: Option<String>,
    end: Option<String>,
    reason: Option<String>,
    all_strain: Option<f64>,
    all_return_intent: Option<f64>,
    name_mind: Option<bool>,
    project: Option<String>,
    close_project: Option<String>,
    assign: Option<String>,
    assign_robots: Option<RawAssignRobots>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAssignRobots {
    class: String,
    project: String,
    #[serde(default = "one_u32")]
    count: u32,
}

const fn one_u32() -> u32 {
    1
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAdvice {
    seat: String,
    stance: String,
    text: String,
}

/// A comparison on a quality.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cmp {
    /// Less than.
    Lt(f64),
    /// Less than or equal.
    Le(f64),
    /// Greater than.
    Gt(f64),
    /// Greater than or equal.
    Ge(f64),
    /// Equal within a small tolerance.
    Eq(f64),
}

impl Cmp {
    fn holds(self, v: f64) -> bool {
        match self {
            Self::Lt(x) => v < x,
            Self::Le(x) => v <= x,
            Self::Gt(x) => v > x,
            Self::Ge(x) => v >= x,
            Self::Eq(x) => (v - x).abs() < 1e-9,
        }
    }
}

/// A typed condition.
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    /// A quality compared to a number.
    Quality(Quality, Cmp),
    /// A flag is set.
    Flag(String),
    /// A flag is not set.
    NotFlag(String),
    /// A counter compared to a number.
    Counter(String, Cmp),
    /// Turn at least.
    TurnMin(u32),
    /// Turn at most.
    TurnMax(u32),
    /// Earth window open or closed.
    Window(bool),
    /// Another storylet has fired.
    Fired(String),
    /// Another storylet has not fired.
    NotFired(String),
    /// Sponsor stage at least.
    StageMin(u8),
    /// Sponsor stage at most.
    StageMax(u8),
}

impl Condition {
    /// Whether the condition holds.
    #[must_use]
    pub fn holds(&self, game: &Game) -> bool {
        match self {
            Self::Quality(q, cmp) => cmp.holds(game.quality(*q)),
            Self::Flag(f) => game.flags.contains(f),
            Self::NotFlag(f) => !game.flags.contains(f),
            Self::Counter(c, cmp) => cmp.holds(game.counters.get(c).copied().unwrap_or(0.0)),
            Self::TurnMin(t) => game.turn >= *t,
            Self::TurnMax(t) => game.turn <= *t,
            Self::Window(open) => game.earth_window_open() == *open,
            Self::Fired(id) => game.fired.get(id).is_some_and(|r| r.count > 0),
            Self::NotFired(id) => game.fired.get(id).is_none_or(|r| r.count == 0),
            Self::StageMin(s) => game.sponsor.stage.index() >= *s,
            Self::StageMax(s) => game.sponsor.stage.index() <= *s,
        }
    }
}

/// How a role is cast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastBy {
    /// The holder of a ring seat.
    Seat(Seat),
    /// The most skilled present person in a domain.
    Skill(Skill),
    /// Anyone present.
    Anyone,
}

/// A role to be filled from the roster.
#[derive(Debug, Clone, PartialEq)]
pub struct Cast {
    /// Role name used in text as `{role}`.
    pub role: String,
    /// How to choose.
    pub by: CastBy,
    /// Minimum skill in the relevant domain.
    pub min_skill: u8,
    /// Restrict to rotators (`Some(true)`) or residents (`Some(false)`).
    pub rotator: Option<bool>,
    /// The storylet may fire without this role filled.
    pub optional: bool,
    /// Restrict to one estate.
    pub estate: Option<crate::person::Estate>,
    /// Minimum cumulative dose, Sv.
    pub min_dose: Option<f64>,
    /// Minimum strain.
    pub min_strain: Option<f64>,
    /// Maximum strain.
    pub max_strain: Option<f64>,
    /// Skill casting takes the single best holder (the one the ring seats), not a near-best pool.
    pub top: bool,
}

/// One typed edit to the state.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// Add to a quality.
    Add(Quality, f64),
    /// Set a quality.
    Set(Quality, f64),
    /// Multiply a quality.
    Mul(Quality, f64),
    /// Set a flag.
    Flag(String),
    /// Clear a flag.
    Unflag(String),
    /// Add to a counter.
    Counter(String, f64),
    /// Change a cast person's strain.
    Strain(String, f64),
    /// Add dose to a cast person, Sv.
    Dose(String, f64),
    /// Change a cast person's return intent.
    ReturnIntent(String, f64),
    /// A cast person leaves on the next ship.
    Leave(String),
    /// A cast person dies.
    Die(String),
    /// A cast person becomes a resident.
    Resident(String),
    /// Adjust a directed tie between two cast persons.
    Tie {
        /// From role.
        from: String,
        /// To role.
        to: String,
        /// Hindrance delta.
        hindrance: f64,
        /// Work-positive delta.
        work_positive: f64,
        /// Reliance delta.
        reliance: f64,
    },
    /// Fire a lexicon trigger.
    Lexicon(String),
    /// End the game.
    End(Ending),
    /// Change everyone's strain.
    AllStrain(f64),
    /// Change everyone's return intent.
    AllReturnIntent(f64),
    /// The senior operator names the oldest unnamed mind.
    NameMind,
    /// Opens a project by id.
    OpenProject(String),
    /// Closes an open project by id, abandoning its progress.
    CloseProject(String),
    /// Puts a cast person's die on a project (or back in the hand with "hand").
    Assign(String, String),
    /// Puts up to `count` unassigned robot units of a class on a project.
    AssignRobots(crate::state::RobotClass, String, u32),
}

/// Advice one seat gives about one option, written by the author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Advice {
    /// The seat speaking.
    pub seat: Seat,
    /// For or against.
    pub stance: Stance,
    /// What they say.
    pub text: String,
}

/// A stance on an option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stance {
    /// Recommends it.
    For,
    /// Warns against it.
    Against,
}

/// One choice.
#[derive(Debug, Clone, PartialEq)]
pub struct Option_ {
    /// Identifier.
    pub id: String,
    /// Short label.
    pub label: String,
    /// Longer description.
    pub text: String,
    /// Tags the ring scores by.
    pub tags: Vec<String>,
    /// Gate.
    pub requires: Vec<Condition>,
    /// Effects, applied in order.
    pub effects: Vec<Effect>,
    /// Chronicle line.
    pub chronicle: String,
    /// Authored advice.
    pub advice: Vec<Advice>,
}

/// A storylet.
#[derive(Debug, Clone, PartialEq)]
pub struct Storylet {
    /// Identifier.
    pub id: String,
    /// Title.
    pub title: String,
    /// Act it belongs to.
    pub act: u8,
    /// Weight among eligible storylets.
    pub weight: f64,
    /// Fires at most once.
    pub once: bool,
    /// Counts between firings.
    pub cooldown: u32,
    /// Higher fires first; menace scenes use this.
    pub priority: i32,
    /// Tags.
    pub tags: Vec<String>,
    /// Conditions.
    pub when: Vec<Condition>,
    /// Roles.
    pub cast: Vec<Cast>,
    /// Situation text with `{role}` placeholders.
    pub text: String,
    /// Options.
    pub options: Vec<Option_>,
}

fn parse_cmp(file: &str, r: &RawCondition) -> Result<Option<Cmp>, ContentError> {
    let cmps = [
        r.lt.map(Cmp::Lt),
        r.le.map(Cmp::Le),
        r.gt.map(Cmp::Gt),
        r.ge.map(Cmp::Ge),
        r.eq.map(Cmp::Eq),
    ];
    let mut found = None;
    for c in cmps.into_iter().flatten() {
        if found.is_some() {
            return Err(ContentError::Invalid {
                file: file.to_owned(),
                message: "a condition may carry only one comparison".into(),
            });
        }
        found = Some(c);
    }
    Ok(found)
}

pub(crate) fn parse_condition(file: &str, r: RawCondition) -> Result<Condition, ContentError> {
    let invalid = |m: &str| ContentError::Invalid {
        file: file.to_owned(),
        message: m.to_owned(),
    };
    let cmp = parse_cmp(file, &r)?;
    if let Some(q) = &r.q {
        let quality = Quality::parse(q).ok_or_else(|| invalid(&format!("unknown quality {q}")))?;
        let cmp = cmp.ok_or_else(|| invalid(&format!("quality {q} needs a comparison")))?;
        return Ok(Condition::Quality(quality, cmp));
    }
    if let Some(c) = r.counter {
        let cmp = cmp.ok_or_else(|| invalid(&format!("counter {c} needs a comparison")))?;
        return Ok(Condition::Counter(c, cmp));
    }
    if let Some(f) = r.flag {
        return Ok(Condition::Flag(f));
    }
    if let Some(f) = r.not_flag {
        return Ok(Condition::NotFlag(f));
    }
    if let Some(t) = r.turn_min {
        return Ok(Condition::TurnMin(t));
    }
    if let Some(t) = r.turn_max {
        return Ok(Condition::TurnMax(t));
    }
    if let Some(w) = r.window {
        return match w.as_str() {
            "open" => Ok(Condition::Window(true)),
            "closed" => Ok(Condition::Window(false)),
            other => Err(invalid(&format!(
                "window must be open or closed, not {other}"
            ))),
        };
    }
    if let Some(id) = r.fired {
        return Ok(Condition::Fired(id));
    }
    if let Some(id) = r.not_fired {
        return Ok(Condition::NotFired(id));
    }
    if let Some(s) = r.stage_min {
        return Ok(Condition::StageMin(s));
    }
    if let Some(s) = r.stage_max {
        return Ok(Condition::StageMax(s));
    }
    Err(invalid("empty condition"))
}

fn parse_cast(file: &str, r: RawCast) -> Result<Cast, ContentError> {
    let invalid = |m: String| ContentError::Invalid {
        file: file.to_owned(),
        message: m,
    };
    let by = match (r.seat, r.skill) {
        (Some(s), None) => {
            CastBy::Seat(Seat::parse(&s).ok_or_else(|| invalid(format!("unknown seat {s}")))?)
        }
        (None, Some(k)) => {
            CastBy::Skill(Skill::parse(&k).ok_or_else(|| invalid(format!("unknown skill {k}")))?)
        }
        (None, None) => CastBy::Anyone,
        (Some(_), Some(_)) => {
            return Err(invalid(format!("role {} has both seat and skill", r.role)));
        }
    };
    let estate = match r.estate.as_deref() {
        None => None,
        Some("kept") => Some(crate::person::Estate::Kept),
        Some("bore") => Some(crate::person::Estate::Bore),
        Some("skiff") => Some(crate::person::Estate::Skiff),
        Some(other) => return Err(invalid(format!("unknown estate {other}"))),
    };
    Ok(Cast {
        role: r.role,
        by,
        min_skill: r.min_skill,
        rotator: r.rotator,
        optional: r.optional,
        estate,
        min_dose: r.min_dose,
        min_strain: r.min_strain,
        max_strain: r.max_strain,
        top: r.top,
    })
}

fn parse_effect(file: &str, roles: &[String], r: RawEffect) -> Result<Effect, ContentError> {
    let invalid = |m: String| ContentError::Invalid {
        file: file.to_owned(),
        message: m,
    };
    let check_role = |role: &str| {
        if roles.iter().any(|x| x == role) {
            Ok(())
        } else {
            Err(invalid(format!("effect refers to uncast role {role}")))
        }
    };
    if let Some(q) = &r.q {
        let quality = Quality::parse(q).ok_or_else(|| invalid(format!("unknown quality {q}")))?;
        if !quality.writable() {
            return Err(invalid(format!(
                "quality {q} is derived and cannot be written"
            )));
        }
        return match (r.add, r.set, r.mul) {
            (Some(v), None, None) => Ok(Effect::Add(quality, v)),
            (None, Some(v), None) => Ok(Effect::Set(quality, v)),
            (None, None, Some(v)) => Ok(Effect::Mul(quality, v)),
            _ => Err(invalid(format!(
                "quality effect on {q} needs exactly one of add/set/mul"
            ))),
        };
    }
    if let Some(f) = r.flag {
        return Ok(Effect::Flag(f));
    }
    if let Some(f) = r.unflag {
        return Ok(Effect::Unflag(f));
    }
    if let Some(c) = r.counter {
        return Ok(Effect::Counter(c, r.add.unwrap_or(1.0)));
    }
    if let Some(role) = r.person {
        check_role(&role)?;
        if let Some(target) = r.assign {
            return Ok(Effect::Assign(role, target));
        }
        if let Some(v) = r.strain {
            return Ok(Effect::Strain(role, v));
        }
        if let Some(v) = r.dose {
            return Ok(Effect::Dose(role, v));
        }
        if let Some(v) = r.return_intent {
            return Ok(Effect::ReturnIntent(role, v));
        }
        if r.leave == Some(true) {
            return Ok(Effect::Leave(role));
        }
        if r.die == Some(true) {
            return Ok(Effect::Die(role));
        }
        if r.resident == Some(true) {
            return Ok(Effect::Resident(role));
        }
        return Err(invalid(format!("person effect on {role} does nothing")));
    }
    if let Some([from, to]) = r.tie {
        check_role(&from)?;
        check_role(&to)?;
        return Ok(Effect::Tie {
            from,
            to,
            hindrance: r.hindrance.unwrap_or(0.0),
            work_positive: r.work_positive.unwrap_or(0.0),
            reliance: r.reliance.unwrap_or(0.0),
        });
    }
    if let Some(l) = r.lexicon {
        return Ok(Effect::Lexicon(l));
    }
    if let Some(e) = r.end {
        let reason = r.reason.unwrap_or_default();
        return match e.as_str() {
            "closed" => Ok(Effect::End(Ending::Closed { reason })),
            "extinct" => Ok(Effect::End(Ending::Extinct { reason })),
            other => Err(invalid(format!(
                "end must be closed or extinct, not {other}"
            ))),
        };
    }
    if let Some(v) = r.all_strain {
        return Ok(Effect::AllStrain(v));
    }
    if let Some(v) = r.all_return_intent {
        return Ok(Effect::AllReturnIntent(v));
    }
    if r.name_mind == Some(true) {
        return Ok(Effect::NameMind);
    }
    if let Some(p) = r.project {
        return Ok(Effect::OpenProject(p));
    }
    if let Some(p) = r.close_project {
        return Ok(Effect::CloseProject(p));
    }
    if let Some(RawAssignRobots {
        class,
        project,
        count,
    }) = r.assign_robots
    {
        let class = crate::state::RobotClass::parse(&class)
            .ok_or_else(|| invalid(format!("unknown robot class {class}")))?;
        return Ok(Effect::AssignRobots(class, project, count));
    }
    Err(invalid("empty effect".into()))
}

impl Storylet {
    /// Parses one storylet file.
    pub fn from_toml(file: &str, text: &str) -> Result<Self, ContentError> {
        let raw: RawStorylet = toml::from_str(text).map_err(|source| ContentError::Toml {
            file: file.to_owned(),
            source,
        })?;
        let invalid = |m: String| ContentError::Invalid {
            file: file.to_owned(),
            message: m,
        };
        if raw.option.is_empty() {
            return Err(invalid("a storylet needs at least one option".into()));
        }
        let when = raw
            .when
            .into_iter()
            .map(|c| parse_condition(file, c))
            .collect::<Result<Vec<_>, _>>()?;
        let cast = raw
            .cast
            .into_iter()
            .map(|c| parse_cast(file, c))
            .collect::<Result<Vec<_>, _>>()?;
        let roles: Vec<String> = cast.iter().map(|c| c.role.clone()).collect();
        let mut options = Vec::new();
        for o in raw.option {
            let requires = o
                .requires
                .into_iter()
                .map(|c| parse_condition(file, c))
                .collect::<Result<Vec<_>, _>>()?;
            let effects = o
                .effect
                .into_iter()
                .map(|e| parse_effect(file, &roles, e))
                .collect::<Result<Vec<_>, _>>()?;
            let advice = o
                .advice
                .into_iter()
                .map(|a| {
                    let seat = Seat::parse(&a.seat)
                        .ok_or_else(|| invalid(format!("unknown seat {}", a.seat)))?;
                    let stance = match a.stance.as_str() {
                        "for" => Stance::For,
                        "against" => Stance::Against,
                        other => {
                            return Err(invalid(format!(
                                "stance must be for/against, not {other}"
                            )));
                        }
                    };
                    Ok(Advice {
                        seat,
                        stance,
                        text: a.text,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if o.chronicle.trim().is_empty() {
                return Err(invalid(format!(
                    "option {} has an empty chronicle line",
                    o.id
                )));
            }
            options.push(Option_ {
                id: o.id,
                label: o.label,
                text: o.text,
                tags: o.tags,
                requires,
                effects,
                chronicle: o.chronicle,
                advice,
            });
        }
        Ok(Self {
            id: raw.id,
            title: raw.title,
            act: raw.act,
            weight: raw.weight,
            once: raw.once,
            cooldown: raw.cooldown,
            priority: raw.priority,
            tags: raw.tags,
            when,
            cast,
            text: raw.text,
            options,
        })
    }

    /// Whether the storylet's own conditions hold, ignoring casting and history.
    #[must_use]
    pub fn conditions_hold(&self, game: &Game) -> bool {
        self.act == game.act && self.when.iter().all(|c| c.holds(game))
    }

    /// Whether history permits firing now.
    #[must_use]
    pub fn history_permits(&self, game: &Game) -> bool {
        let Some(rec) = game.fired.get(&self.id) else {
            return true;
        };
        if self.once && rec.count > 0 {
            return false;
        }
        rec.last
            .is_none_or(|last| game.turn >= last + self.cooldown)
    }

    /// Attempts to fill every role. Returns `None` if a required role cannot be cast.
    #[must_use]
    pub fn cast(
        &self,
        game: &Game,
        seats: &BTreeMap<Seat, PersonId>,
        rng: &mut impl rand::Rng,
    ) -> Option<Casting> {
        let mut used: Vec<PersonId> = Vec::new();
        let mut roles = BTreeMap::new();
        for Cast {
            role,
            by,
            min_skill,
            rotator,
            optional,
            estate,
            min_dose,
            min_strain,
            max_strain,
            top: exact,
        } in &self.cast
        {
            let eligible = |p: &crate::person::Person| {
                if used.contains(&p.id) {
                    return false;
                }
                let tenure_ok = match rotator {
                    Some(true) => p.contract_end().is_some(),
                    Some(false) => p.contract_end().is_none(),
                    None => true,
                };
                let estate_ok = estate.is_none_or(|e| p.estate == e);
                let dose_ok = min_dose.is_none_or(|d| p.condition.dose_sv >= d);
                let strain_ok = min_strain.is_none_or(|s| p.condition.strain >= s)
                    && max_strain.is_none_or(|s| p.condition.strain <= s);
                tenure_ok && estate_ok && dose_ok && strain_ok
            };
            let pick = match by {
                CastBy::Seat(seat) => seats
                    .get(seat)
                    .map(|&id| game.person(id))
                    .filter(|p| eligible(p) && p.skill(seat.skill()) >= *min_skill)
                    .map(|p| p.id),
                CastBy::Skill(skill) => {
                    let top = game
                        .present()
                        .filter(|p| eligible(p))
                        .map(|p| p.skill(*skill))
                        .max()
                        .unwrap_or(0);
                    let floor = if *exact {
                        top
                    } else {
                        top.saturating_sub(1).max(*min_skill)
                    };
                    let pool: Vec<(PersonId, f64)> = game
                        .present()
                        .filter(|p| eligible(p) && p.skill(*skill) >= floor && top >= *min_skill)
                        .map(|p| {
                            let recent = game
                                .recent_cast
                                .get(&p.id)
                                .is_some_and(|&t| game.turn < t + 8);
                            let w = if p.skill(*skill) == top { 1.0 } else { 0.4 };
                            (p.id, if recent { w * 0.15 } else { w })
                        })
                        .collect();
                    weighted_pick(&pool, rng)
                }
                CastBy::Anyone => {
                    let pool: Vec<(PersonId, f64)> = game
                        .present()
                        .filter(|p| eligible(p))
                        .map(|p| {
                            let recent = game
                                .recent_cast
                                .get(&p.id)
                                .is_some_and(|&t| game.turn < t + 8);
                            let w = 0.2 + p.condition.strain;
                            (p.id, if recent { w * 0.1 } else { w })
                        })
                        .collect();
                    weighted_pick(&pool, rng)
                }
            };
            match pick {
                Some(id) => {
                    used.push(id);
                    roles.insert(role.clone(), id);
                }
                None if *optional => {}
                None => return None,
            }
        }
        Some(Casting { roles })
    }

    /// Options whose gates hold.
    pub fn available_options<'a>(
        &'a self,
        game: &Game,
    ) -> impl Iterator<Item = (usize, &'a Option_)> {
        self.options
            .iter()
            .enumerate()
            .filter(move |(_, o)| o.requires.iter().all(|c| c.holds(game)))
    }
}

fn weighted_pick(pool: &[(PersonId, f64)], rng: &mut impl rand::Rng) -> Option<PersonId> {
    let total: f64 = pool.iter().map(|(_, w)| w).sum();
    if total <= 0.0 {
        return None;
    }
    let mut x = rng.random::<f64>() * total;
    for (id, w) in pool {
        x -= w;
        if x <= 0.0 {
            return Some(*id);
        }
    }
    pool.last().map(|(id, _)| *id)
}

/// Roles filled for one firing.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Casting {
    /// Role name to person.
    pub roles: BTreeMap<String, PersonId>,
}

impl Casting {
    /// Substitutes `{role}` placeholders with names and a few globals.
    #[must_use]
    pub fn render(&self, game: &Game, text: &str) -> String {
        let mut out = text.to_owned();
        for (role, id) in &self.roles {
            out = out.replace(&placeholder(role), &game.person(*id).name);
        }
        out = out.replace(&placeholder("outpost"), &game.outpost_name);
        let mind = game
            .minds
            .iter()
            .find(|m| m.alive)
            .map_or_else(|| "the mind".to_owned(), |m| m.display().to_owned());
        out = out.replace(&placeholder("mind"), &mind);
        out.replace(&placeholder("turn"), &game.turn.to_string())
    }
}

fn placeholder(name: &str) -> String {
    format!("{{{name}}}")
}

/// Applies an option's effects and writes its chronicle line.
pub fn apply(game: &mut Game, storylet: &Storylet, option: &Option_, casting: &Casting) {
    let person = |game: &Game, role: &str| {
        casting
            .roles
            .get(role)
            .copied()
            .filter(|id| game.person(*id).alive)
    };
    for effect in &option.effects {
        match effect {
            Effect::Add(q, v) => {
                let cur = game.quality(*q);
                game.set_quality(*q, cur + v);
            }
            Effect::Set(q, v) => {
                game.set_quality(*q, *v);
            }
            Effect::Mul(q, v) => {
                let cur = game.quality(*q);
                game.set_quality(*q, cur * v);
            }
            Effect::Flag(f) => {
                game.flags.insert(f.clone());
                match f.as_str() {
                    "manifest_throughput" => {
                        game.controls.manifest = crate::project::ManifestSplit::Throughput;
                    }
                    "manifest_balanced" => {
                        game.controls.manifest = crate::project::ManifestSplit::Balanced;
                    }
                    "manifest_capability" => {
                        game.controls.manifest = crate::project::ManifestSplit::Capability;
                    }
                    "manifest_people" => {
                        game.controls.manifest = crate::project::ManifestSplit::People;
                    }
                    "throw_ship" => game.controls.throw = crate::project::ThrowMode::Ship,
                    "throw_hold" => game.controls.throw = crate::project::ThrowMode::HoldAtReserve,
                    "throw_stop" => game.controls.throw = crate::project::ThrowMode::Stop,
                    "roster_skill" => game.controls.roster = crate::project::RosterOrder::Skill,
                    "roster_strain" => game.controls.roster = crate::project::RosterOrder::Strain,
                    "roster_name" => game.controls.roster = crate::project::RosterOrder::Name,
                    "auto_deal_on" => game.controls.auto_deal = true,
                    "auto_deal_off" => game.controls.auto_deal = false,
                    _ => {}
                }
            }
            Effect::Unflag(f) => {
                game.flags.remove(f);
            }
            Effect::Counter(c, v) => {
                *game.counters.entry(c.clone()).or_insert(0.0) += v;
            }
            Effect::Strain(role, v) => {
                if let Some(id) = person(game, role) {
                    let p = game.person_mut(id);
                    p.condition.strain = (p.condition.strain + v).clamp(0.0, 1.0);
                }
            }
            Effect::Dose(role, v) => {
                if let Some(id) = person(game, role) {
                    game.person_mut(id).condition.dose_sv += v;
                }
            }
            Effect::ReturnIntent(role, v) => {
                if let Some(id) = person(game, role) {
                    let p = game.person_mut(id);
                    p.condition.return_intent = (p.condition.return_intent + v).clamp(0.0, 1.0);
                }
            }
            Effect::Leave(role) => {
                if let Some(id) = person(game, role) {
                    game.person_mut(id).condition.return_intent = 1.0;
                    game.flags.insert(format!("leaving:{}", id.0));
                }
            }
            Effect::Die(role) => {
                if let Some(id) = person(game, role) {
                    let p = game.person_mut(id);
                    p.alive = false;
                    p.present = false;
                }
            }
            Effect::Resident(role) => {
                if let Some(id) = person(game, role) {
                    let p = game.person_mut(id);
                    p.tenure = crate::person::Tenure::Resident;
                    p.condition.return_intent *= 0.3;
                }
            }
            Effect::Tie {
                from,
                to,
                hindrance,
                work_positive,
                reliance,
            } => {
                if let (Some(a), Some(b)) = (person(game, from), person(game, to)) {
                    let t = game.ties.get_mut(a, b);
                    t.hindrance = (t.hindrance + hindrance).clamp(0.0, 1.0);
                    t.work_positive = (t.work_positive + work_positive).clamp(0.0, 1.0);
                    t.reliance = (t.reliance + reliance).clamp(0.0, 1.0);
                }
            }
            Effect::Lexicon(trigger) => {
                game.lexicon_triggers.insert(trigger.clone());
            }
            Effect::End(ending) => {
                game.ending = Some(ending.clone());
            }
            Effect::AllStrain(v) => {
                for p in game.present_mut() {
                    p.condition.strain = (p.condition.strain + v).clamp(0.0, 1.0);
                }
            }
            Effect::AllReturnIntent(v) => {
                for p in game.present_mut() {
                    p.condition.return_intent = (p.condition.return_intent + v).clamp(0.0, 1.0);
                }
            }
            Effect::NameMind => {
                crate::names::name_oldest_mind(game);
            }
            Effect::OpenProject(id) => {
                game.flags.insert(format!("open_project:{id}"));
            }
            Effect::CloseProject(id) => {
                game.projects.retain(|s| s.id.0 != *id);
                game.assignments.retain(|_, t| t.0 != *id);
            }
            Effect::AssignRobots(class, project, count) => {
                if game.projects.iter().any(|s| s.id.0 == *project) {
                    let n = game.robots.count(*class).round().max(0.0);
                    let units = n.min(f64::from(u32::MAX)).round();
                    let mut placed = 0;
                    let mut k = 0u32;
                    while f64::from(k) < units && placed < *count {
                        let die = crate::project::DieId::Robot(*class, k);
                        if let std::collections::btree_map::Entry::Vacant(slot) =
                            game.assignments.entry(die)
                        {
                            slot.insert(crate::project::ProjectId(project.clone()));
                            placed += 1;
                        }
                        k += 1;
                    }
                }
            }
            Effect::Assign(role, target) => {
                if let Some(id) = person(game, role) {
                    let die = crate::project::DieId::Person(id);
                    if target == "hand" {
                        game.assignments.remove(&die);
                    } else if game.projects.iter().any(|s| s.id.0 == *target) {
                        game.assignments
                            .insert(die, crate::project::ProjectId(target.clone()));
                    }
                }
            }
        }
    }
    for id in casting.roles.values() {
        game.recent_cast.insert(*id, game.turn);
    }
    let line = casting.render(game, &option.chronicle);
    game.chronicle(line, Some(&storylet.id));
    let rec = game.fired.entry(storylet.id.clone()).or_default();
    rec.count += 1;
    rec.last = Some(game.turn);
}
