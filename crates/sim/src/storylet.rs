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
struct RawCondition {
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
            Self::NotFired(id) => !game.fired.get(id).is_some_and(|r| r.count > 0),
            Self::StageMin(s) => game.sponsor.stage.index() >= *s,
            Self::StageMax(s) => game.sponsor.stage.index() <= *s,
        }
    }
}

/// How a role is cast.
#[derive(Debug, Clone, PartialEq)]
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
}

/// Advice one seat gives about one option, written by the author.
#[derive(Debug, Clone, PartialEq)]
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

fn parse_condition(file: &str, r: RawCondition) -> Result<Condition, ContentError> {
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
    Ok(Cast {
        role: r.role,
        by,
        min_skill: r.min_skill,
        rotator: r.rotator,
        optional: r.optional,
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
                            return Err(invalid(format!("stance must be for/against, not {other}")));
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
    pub fn cast(&self, game: &Game, seats: &BTreeMap<Seat, PersonId>) -> Option<Casting> {
        let mut used: Vec<PersonId> = Vec::new();
        let mut roles = BTreeMap::new();
        for Cast {
            role,
            by,
            min_skill,
            rotator,
            optional,
        } in &self.cast
        {
            let eligible = |p: &crate::person::Person| {
                if used.contains(&p.id) {
                    return false;
                }
                match rotator {
                    Some(true) => p.contract_end().is_some(),
                    Some(false) => p.contract_end().is_none(),
                    None => true,
                }
            };
            let pick = match by {
                CastBy::Seat(seat) => seats
                    .get(seat)
                    .map(|&id| game.person(id))
                    .filter(|p| eligible(p) && p.skill(seat.skill()) >= *min_skill)
                    .map(|p| p.id),
                CastBy::Skill(skill) => game
                    .present()
                    .filter(|p| eligible(p) && p.skill(*skill) >= *min_skill)
                    .max_by_key(|p| (p.skill(*skill), std::cmp::Reverse(p.id)))
                    .map(|p| p.id),
                CastBy::Anyone => game
                    .present()
                    .filter(|p| eligible(p))
                    .max_by_key(|p| {
                        (
                            (p.condition.strain * 1000.0) as u32,
                            std::cmp::Reverse(p.id),
                        )
                    })
                    .map(|p| p.id),
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

/// Roles filled for one firing.
#[derive(Debug, Clone, PartialEq, Default)]
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
            out = out.replace(&format!("{{{role}}}"), &game.person(*id).name);
        }
        out = out.replace("{outpost}", &game.outpost_name);
        let mind = game
            .minds
            .iter()
            .find(|m| m.alive)
            .map_or_else(|| "the mind".to_owned(), |m| m.display().to_owned());
        out = out.replace("{mind}", &mind);
        out.replace("{turn}", &game.turn.to_string())
    }
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
        }
    }
    let line = casting.render(game, &option.chronicle);
    game.chronicle(line, Some(&storylet.id));
    let rec = game.fired.entry(storylet.id.clone()).or_default();
    rec.count += 1;
    rec.last = Some(game.turn);
}
