//! Projects: clocks the polity fills with labour. People and robots are the dice.

use crate::person::{PersonId, Skill};
use crate::state::{Game, RobotClass};
use crate::storylet::{Condition, ContentError};
use az::{Az, SaturatingAs};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Content identifier of a project.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectId(pub String);

/// A die: one adult, or one robot unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DieId {
    /// A person.
    Person(PersonId),
    /// The k-th unit of a robot class.
    Robot(RobotClass, u32),
}

impl DieId {
    /// Wire form: `p:12` or `r:haul:3`.
    #[must_use]
    pub fn key(self) -> String {
        match self {
            Self::Person(PersonId(n)) => format!("p:{n}"),
            Self::Robot(class, k) => format!("r:{}:{k}", class.key()),
        }
    }

    /// Parses the wire form.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split(':');
        match (parts.next(), parts.next(), parts.next()) {
            (Some("p"), Some(n), None) => n.parse().ok().map(|n| Self::Person(PersonId(n))),
            (Some("r"), Some(class), Some(k)) => {
                Some(Self::Robot(RobotClass::parse(class)?, k.parse().ok()?))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProject {
    id: String,
    title: String,
    domain: String,
    #[serde(default)]
    structured: bool,
    #[serde(default)]
    standing: bool,
    #[serde(default)]
    segments: u32,
    #[serde(default = "six")]
    pips_per_segment: u32,
    #[serde(default)]
    pips_needed: f64,
    #[serde(default)]
    pips_per_person: f64,
    #[serde(default)]
    manual: bool,
    #[serde(default)]
    once: bool,
    #[serde(default)]
    spares: f64,
    #[serde(default)]
    min_skill: u8,
    #[serde(default)]
    when: Vec<crate::storylet::RawCondition>,
    #[serde(default)]
    on_complete_flag: Option<String>,
    #[serde(default)]
    on_complete_lexicon: Option<String>,
    #[serde(default)]
    repeat_segments: Option<u32>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    on_complete_add: Option<QualityAdd>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct QualityAdd {
    q: String,
    value: f64,
}

const fn six() -> u32 {
    6
}

/// A project as defined by content.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectDef {
    /// Identifier.
    pub id: ProjectId,
    /// Title, in the act-1 register.
    pub title: String,
    /// One line of description.
    pub description: String,
    /// Skill that sets a die's face here.
    pub domain: Skill,
    /// Structured work: any robot fits. Otherwise only dex units.
    pub structured: bool,
    /// Never completes; its pips are a rate.
    pub standing: bool,
    /// Segments to fill (one-time projects).
    pub segments: u32,
    /// Pips that fill one segment.
    pub pips_per_segment: u32,
    /// Pips at which a standing project runs at full rate.
    pub pips_needed: f64,
    /// Extra pips needed per present person (standing projects that scale).
    pub pips_per_person: f64,
    /// Only opened by content, never by its conditions.
    pub manual: bool,
    /// After completion it does not reopen.
    pub once: bool,
    /// Spares consumed on completion.
    pub spares: f64,
    /// A die needs at least this skill to sit here.
    pub min_skill: u8,
    /// Opens when these hold (unless manual).
    pub when: Vec<Condition>,
    /// Flag set on completion; content fires the completion scene off it.
    pub on_complete_flag: Option<String>,
    /// Lexicon trigger on completion.
    pub on_complete_lexicon: Option<String>,
    /// Segments when it reopens after completion.
    pub repeat_segments: Option<u32>,
    /// A quality to add to on completion (e.g. a through-wall unit).
    pub on_complete_add: Option<(crate::quality::Quality, f64)>,
}

impl ProjectDef {
    /// Parses one project file.
    pub fn from_toml(file: &str, text: &str) -> Result<Self, ContentError> {
        let raw: RawProject = toml::from_str(text).map_err(|source| ContentError::Toml {
            file: file.to_owned(),
            source,
        })?;
        let invalid = |m: String| ContentError::Invalid {
            file: file.to_owned(),
            message: m,
        };
        let domain = Skill::parse(&raw.domain)
            .ok_or_else(|| invalid(format!("unknown domain {}", raw.domain)))?;
        if !raw.standing && raw.segments == 0 {
            return Err(invalid("a one-time project needs segments".into()));
        }
        if raw.standing && raw.pips_needed <= 0.0 {
            return Err(invalid("a standing project needs pips_needed".into()));
        }
        let when = raw
            .when
            .into_iter()
            .map(|c| crate::storylet::parse_condition(file, c))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            id: ProjectId(raw.id),
            title: raw.title,
            description: raw.description,
            domain,
            structured: raw.structured,
            standing: raw.standing,
            segments: raw.segments,
            pips_per_segment: raw.pips_per_segment.max(1),
            pips_needed: raw.pips_needed,
            pips_per_person: raw.pips_per_person,
            manual: raw.manual,
            once: raw.once,
            spares: raw.spares,
            min_skill: raw.min_skill,
            when,
            on_complete_flag: raw.on_complete_flag,
            on_complete_lexicon: raw.on_complete_lexicon,
            repeat_segments: raw.repeat_segments,
            on_complete_add: match raw.on_complete_add {
                None => None,
                Some(QualityAdd { q, value }) => {
                    let quality = crate::quality::Quality::parse(&q)
                        .filter(|x| x.writable())
                        .ok_or_else(|| {
                            invalid(format!("on_complete_add: unknown or read-only quality {q}"))
                        })?;
                    Some((quality, value))
                }
            },
        })
    }
}

/// The live state of one open project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectState {
    /// Which project.
    pub id: ProjectId,
    /// Segments filled.
    pub filled: u32,
    /// Segments total (0 for standing).
    pub segments: u32,
    /// Pips carried toward the next segment.
    pub carry: u32,
    /// Count opened.
    pub opened: u32,
    /// Times completed.
    pub completions: u32,
    /// Pips delivered this count (a rate for standing projects).
    pub pips_this_count: u32,
    /// Fraction of full rate this count (standing projects), 0-1.
    pub rate: f64,
    /// Setback or bonus this count, for the chronicle.
    pub roll: Roll,
}

/// The one roll a project makes each count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Roll {
    /// Nothing special.
    #[default]
    Plain,
    /// A bonus segment.
    Bonus,
    /// A lost segment.
    Setback,
}

/// The mass driver's position: the second standing control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThrowMode {
    /// Ship everything above the reserve.
    #[default]
    Ship,
    /// Ship only what the tank can spare this count.
    HoldAtReserve,
    /// Stop the driver.
    Stop,
}

/// The manifest split: the first standing control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ManifestSplit {
    /// Pods, propellant, robots.
    Throughput,
    /// Half and half.
    #[default]
    Balanced,
    /// Spares, medicine, tooling.
    Capability,
    /// Ask for people.
    People,
}

impl ManifestSplit {
    /// Share of a convoy's tonnage that is capability hardware.
    #[must_use]
    pub const fn capability_share(self) -> f64 {
        match self {
            Self::Throughput => 0.1,
            Self::Balanced => 0.45,
            Self::Capability => 0.7,
            Self::People => 0.3,
        }
    }
}

/// The hand's sort and eviction order: the third standing control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RosterOrder {
    /// Best face for the project first; upkeep eats the least skilled.
    #[default]
    Skill,
    /// Least strained first; upkeep eats the most strained.
    Strain,
    /// By name.
    Name,
}

/// The three standing controls, plus whether the engine deals free dice itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Controls {
    /// Manifest split.
    pub manifest: ManifestSplit,
    /// Throw position.
    pub throw: ThrowMode,
    /// Roster order.
    pub roster: RosterOrder,
    /// Deal unassigned free dice onto standing projects automatically (CLI policies).
    pub auto_deal: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            manifest: ManifestSplit::Balanced,
            throw: ThrowMode::Ship,
            roster: RosterOrder::Skill,
            auto_deal: true,
        }
    }
}

/// A die as the player sees it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DieView {
    /// Wire id.
    pub id: String,
    /// Name, or the robot class.
    pub label: String,
    /// Face for the given domain (persons), or pips a robot gives; 0-6.
    pub face: u8,
    /// Every non-zero face by domain, so the player can see where a die is worth more.
    #[serde(default)]
    pub faces: std::collections::BTreeMap<String, u8>,
    /// Strain, for the dulling.
    pub strain: f64,
    /// Where it sits: a project id, or "hand", or "upkeep".
    pub place: String,
    /// Robot or person.
    pub robot: bool,
}

/// The hand and the rail after this count's deal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Hand {
    /// Adults present.
    pub adults: u32,
    /// Dice eaten by upkeep.
    pub eaten: u32,
    /// Dice free to assign (in the hand or on projects).
    pub free: u32,
    /// Upkeep dice the outpost could not supply this count.
    pub shortfall: u32,
    /// Robot dice available.
    pub robots: u32,
    /// Every die with its place.
    pub dice: Vec<DieView>,
}

/// Face of a person for a domain, after strain.
#[must_use]
pub fn face(game: &Game, id: PersonId, domain: Skill) -> u8 {
    let p = game.person(id);
    let raw = p.skill(domain);
    if p.condition.strain > 0.5 {
        raw.saturating_sub(1)
    } else {
        raw
    }
}

fn robot_dice(game: &Game) -> Vec<(DieId, u8)> {
    let mut out = Vec::new();
    for class in RobotClass::ALL {
        let n = game.robots.count(class).round().saturating_as::<u32>();
        for k in 0..n {
            out.push((DieId::Robot(class, k), class.pips()));
        }
    }
    out
}

fn robot_fits(class: RobotClass, def: &ProjectDef) -> bool {
    def.structured || class == RobotClass::Dex
}

/// Recomputes who is eaten by upkeep, evicts over-assignment, deals free dice if asked,
/// and returns the hand. Called once per count before projects progress.
pub fn deal(game: &mut Game, defs: &[ProjectDef], upkeep_h: f64, per_die: f64) -> Vec<String> {
    let adults: Vec<PersonId> = game
        .present()
        .filter(|p| p.age_counts(game.turn) >= 18 * 12)
        .map(|p| p.id)
        .collect();
    let n = adults.len().saturating_as::<u32>();
    let robot_units = robot_dice(game);
    let assigned_robot_hours = |assignments: &std::collections::BTreeMap<DieId, ProjectId>| -> f64 {
        assignments
            .keys()
            .filter_map(|d| match d {
                DieId::Robot(c, _) => Some(c.hours()),
                DieId::Person(_) => None,
            })
            .sum()
    };
    let all_robot_hours: f64 = robot_units
        .iter()
        .map(|(d, _)| match d {
            DieId::Robot(c, _) => c.hours(),
            DieId::Person(_) => 0.0,
        })
        .sum();
    let eaten_for = |assignments: &std::collections::BTreeMap<DieId, ProjectId>| -> u32 {
        let covered = (all_robot_hours - assigned_robot_hours(assignments)).max(0.0);
        ((upkeep_h - covered) / per_die)
            .ceil()
            .max(0.0)
            .saturating_as::<u32>()
    };
    let order = game.controls.roster;
    let mut ranked = adults.clone();
    ranked.sort_by(|&a, &b| {
        let pa = game.person(a);
        let pb = game.person(b);
        match order {
            RosterOrder::Skill => {
                let sa: u8 = pa.skills.values().copied().max().unwrap_or(0);
                let sb: u8 = pb.skills.values().copied().max().unwrap_or(0);
                sb.cmp(&sa).then(a.cmp(&b))
            }
            RosterOrder::Strain => pa
                .condition
                .strain
                .total_cmp(&pb.condition.strain)
                .then(a.cmp(&b)),
            RosterOrder::Name => pa.name.cmp(&pb.name).then(a.cmp(&b)),
        }
    });
    let mut assignments = game.assignments.clone();
    assignments.retain(|die, pid| match die {
        DieId::Person(p) => adults.contains(p) && game.projects.iter().any(|s| &s.id == pid),
        DieId::Robot(class, k) => {
            let n = game.robots.count(*class).round().saturating_as::<u32>();
            *k < n && game.projects.iter().any(|s| &s.id == pid)
        }
    });
    for (die, pid) in assignments.clone() {
        let Some(def) = defs.iter().find(|d| d.id == pid) else {
            assignments.remove(&die);
            continue;
        };
        let fits = match die {
            DieId::Person(p) => face(game, p, def.domain) >= def.min_skill,
            DieId::Robot(class, _) => robot_fits(class, def),
        };
        if !fits {
            assignments.remove(&die);
        }
    }
    let eaten_needed = eaten_for(&assignments);
    let eaten = eaten_needed.min(n);
    let shortfall = eaten_needed.saturating_sub(n);
    let free = n - eaten;
    let mut kept: Vec<PersonId> = ranked.iter().copied().take(free.az::<usize>()).collect();
    let mut assigned_people: Vec<PersonId> = assignments
        .keys()
        .filter_map(|d| match d {
            DieId::Person(p) => Some(*p),
            DieId::Robot(_, _) => None,
        })
        .collect();
    let face_on_project = |p: PersonId| -> u8 {
        assignments
            .get(&DieId::Person(p))
            .and_then(|pid| defs.iter().find(|d| &d.id == pid))
            .map_or(0, |d| face(game, p, d.domain))
    };
    assigned_people.sort_by(|&a, &b| face_on_project(b).cmp(&face_on_project(a)).then(a.cmp(&b)));
    let mut free_set: Vec<PersonId> = Vec::new();
    for p in &assigned_people {
        if free_set.len() < free.az::<usize>() {
            free_set.push(*p);
        }
    }
    for p in ranked {
        if free_set.len() >= free.az::<usize>() {
            break;
        }
        if !free_set.contains(&p) {
            free_set.push(p);
        }
    }
    kept.clear();
    kept.extend(free_set.iter().copied());
    let mut evicted: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
    assignments.retain(|die, pid| match die {
        DieId::Person(p) => {
            let keep = kept.contains(p);
            if !keep {
                *evicted.entry(pid.0.clone()).or_insert(0) += 1;
            }
            keep
        }
        DieId::Robot(_, _) => true,
    });
    let mut lines: Vec<String> = evicted
        .iter()
        .map(|(pid, n)| {
            let title = defs
                .iter()
                .find(|d| d.id.0 == *pid)
                .map_or(pid.as_str(), |d| d.title.as_str());
            format!("Upkeep recalled {n} from {title}.")
        })
        .collect();
    if game.controls.auto_deal {
        let mut standing: Vec<&ProjectDef> = defs
            .iter()
            .filter(|d| d.standing && game.projects.iter().any(|s| s.id == d.id))
            .collect();
        standing.sort_by(|a, b| a.id.cmp(&b.id));
        let mut one_time: Vec<&ProjectDef> = game
            .projects
            .iter()
            .filter_map(|s| defs.iter().find(|d| d.id == s.id && !d.standing))
            .collect();
        one_time.sort_by_key(|d| {
            game.projects
                .iter()
                .find(|s| s.id == d.id)
                .map_or(0, |s| s.opened)
        });
        for &p in &kept {
            if assignments.contains_key(&DieId::Person(p)) {
                continue;
            }
            let staffed = |d: &ProjectDef| assignments.values().filter(|t| **t == d.id).count();
            if let Some(d) = one_time
                .iter()
                .find(|d| staffed(d) < 3 && face(game, p, d.domain) >= d.min_skill.max(1))
            {
                assignments.insert(DieId::Person(p), d.id.clone());
                continue;
            }
            let n_present = game.present().count().az::<f64>();
            let best = standing
                .iter()
                .filter(|d| face(game, p, d.domain) >= d.min_skill)
                .filter(|d| {
                    let have: u32 = assignments
                        .iter()
                        .filter(|(_, t)| **t == d.id)
                        .map(|(die, _)| match die {
                            DieId::Person(q) => u32::from(face(game, *q, d.domain)),
                            DieId::Robot(c, _) => u32::from(c.pips()),
                        })
                        .sum();
                    f64::from(have) < (d.pips_needed + d.pips_per_person * n_present) * 1.2
                })
                .max_by_key(|d| {
                    let load: u32 = assignments
                        .values()
                        .filter(|x| **x == d.id)
                        .count()
                        .saturating_as::<u32>();
                    (face(game, p, d.domain), std::cmp::Reverse(load))
                });
            if let Some(d) = best {
                assignments.insert(DieId::Person(p), d.id.clone());
            }
        }
        {
            for (die, _) in robot_dice(game) {
                if assignments.contains_key(&die) {
                    continue;
                }
                let DieId::Robot(class, _) = die else {
                    continue;
                };
                let mut trial = assignments.clone();
                trial.insert(die, ProjectId(String::new()));
                if n.saturating_sub(eaten_for(&trial)) < 3 {
                    continue;
                }
                let n_present = game.present().count().az::<f64>();
                let target = standing.iter().filter(|d| robot_fits(class, d)).find(|d| {
                    let have: u32 = assignments
                        .iter()
                        .filter(|(_, t)| **t == d.id)
                        .map(|(die, _)| match die {
                            DieId::Person(q) => u32::from(face(game, *q, d.domain)),
                            DieId::Robot(c, _) => u32::from(c.pips()),
                        })
                        .sum();
                    f64::from(have) < d.pips_needed + d.pips_per_person * n_present
                });
                if let Some(d) = target {
                    assignments.insert(die, d.id.clone());
                }
            }
        }
    }
    game.assignments = assignments;
    game.hand = Hand {
        adults: n,
        eaten,
        free,
        shortfall,
        robots: robot_units.len().saturating_as::<u32>(),
        dice: Vec::new(),
    };
    refresh_hand(game, defs);
    if game.turn.is_multiple_of(6) {
        let visible = |game: &Game, id: &str| {
            !crate::tutorial::active(game) || game.flags.contains(&format!("ui:project:{id}"))
        };
        for def in defs.iter().filter(|d| d.standing) {
            let open = game.projects.iter().any(|s| s.id == def.id);
            let staffed = game.assignments.values().any(|t| *t == def.id);
            if open && !staffed && visible(game, &def.id.0) {
                lines.push(format!("{} idle: no dice assigned.", def.title));
            }
        }
        for def in defs.iter().filter(|d| !d.standing) {
            let Some(state) = game.projects.iter().find(|s| s.id == def.id) else {
                continue;
            };
            let staffed = game.assignments.values().any(|t| *t == def.id);
            if staffed
                && visible(game, &def.id.0)
                && state.pips_this_count < def.pips_per_segment / 2
            {
                let months = def
                    .pips_per_segment
                    .checked_div(state.pips_this_count)
                    .map_or_else(|| "no".to_owned(), |m| m.to_string());
                lines.push(format!(
                    "{} at {} pips a month: {months} months to the next segment.",
                    def.title, state.pips_this_count
                ));
            }
        }
    }
    lines
}

/// Rebuilds the dice list of the hand from the current assignments without re-dealing.
pub fn refresh_hand(game: &mut Game, defs: &[ProjectDef]) {
    let adults: Vec<PersonId> = game
        .present()
        .filter(|p| p.age_counts(game.turn) >= 18 * 12)
        .map(|p| p.id)
        .collect();
    let eaten_ids: std::collections::BTreeSet<PersonId> = game
        .hand
        .dice
        .iter()
        .filter(|d| !d.robot && d.place == "upkeep")
        .filter_map(|d| DieId::parse(&d.id))
        .filter_map(|d| match d {
            DieId::Person(p) => Some(p),
            DieId::Robot(_, _) => None,
        })
        .collect();
    let eaten_ids = if game.hand.dice.is_empty() {
        let free = game.hand.free.az::<usize>();
        adults.iter().copied().skip(free).collect()
    } else {
        eaten_ids
    };
    let mut dice = Vec::new();
    for p in &adults {
        let assigned = game.assignments.get(&DieId::Person(*p)).cloned();
        let place = if eaten_ids.contains(p) && assigned.is_none() {
            "upkeep".to_owned()
        } else {
            assigned
                .as_ref()
                .map_or_else(|| "hand".to_owned(), |pid| pid.0.clone())
        };
        let person = game.person(*p);
        let domain = assigned
            .as_ref()
            .and_then(|pid| defs.iter().find(|d| &d.id == pid))
            .map(|d| d.domain);
        let faces: std::collections::BTreeMap<String, u8> = Skill::ALL
            .into_iter()
            .map(|sk| (sk.key().to_owned(), face(game, *p, sk)))
            .filter(|(_, f)| *f > 0)
            .collect();
        let shown = domain.map_or_else(
            || faces.values().copied().max().unwrap_or(0),
            |d| face(game, *p, d),
        );
        dice.push(DieView {
            id: DieId::Person(*p).key(),
            label: person.name.clone(),
            face: shown,
            faces,
            strain: person.condition.strain,
            place,
            robot: false,
        });
    }
    for (die, pips) in robot_dice(game) {
        let DieId::Robot(class, _) = die else {
            continue;
        };
        dice.push(DieView {
            id: die.key(),
            label: class.key().to_owned(),
            face: pips,
            faces: std::collections::BTreeMap::new(),
            strain: 0.0,
            place: game
                .assignments
                .get(&die)
                .map_or_else(|| "upkeep".to_owned(), |pid| pid.0.clone()),
            robot: true,
        });
    }
    game.hand.dice = dice;
    preview_rates(game, defs);
}

/// Recomputes each open project's pips and rate from the current assignments, so a
/// placement shows its effect at once; `progress` recomputes them for real next count.
pub fn preview_rates(game: &mut Game, defs: &[ProjectDef]) {
    let n_present = game.present().count().az::<f64>();
    let mut per_project: std::collections::BTreeMap<ProjectId, u32> =
        std::collections::BTreeMap::new();
    for (die, pid) in &game.assignments {
        let Some(def) = defs.iter().find(|d| &d.id == pid) else {
            continue;
        };
        let pips = match die {
            DieId::Person(p) => u32::from(face(game, *p, def.domain)),
            DieId::Robot(class, _) => u32::from(class.pips()),
        };
        *per_project.entry(pid.clone()).or_insert(0) += pips;
    }
    for state in &mut game.projects {
        let Some(def) = defs.iter().find(|d| d.id == state.id) else {
            continue;
        };
        let pips = per_project.get(&state.id).copied().unwrap_or(0);
        state.pips_this_count = pips;
        if def.standing {
            let needed = def.pips_needed + def.pips_per_person * n_present;
            state.rate = if needed > 0.0 {
                (f64::from(pips) / needed).min(1.5)
            } else {
                1.0
            };
        }
    }
}

/// Opens projects whose conditions hold and closes none; content opens manual ones.
pub fn open_eligible(game: &mut Game, defs: &[ProjectDef]) {
    let lesson = crate::tutorial::active(game) && !game.flags.contains("tutorial_open");
    for def in defs {
        if lesson && !def.standing && def.id.0 != "dig_keep" {
            continue;
        }
        if def.manual || game.projects.iter().any(|s| s.id == def.id) {
            continue;
        }
        if def.once
            && game
                .counters
                .get(&format!("completed:{}", def.id.0))
                .is_some_and(|&c| c > 0.0)
        {
            continue;
        }
        if def.when.iter().all(|c| c.holds(game)) {
            open(game, def, None);
        }
    }
}

/// Opens a project.
pub fn open(game: &mut Game, def: &ProjectDef, segments: Option<u32>) {
    if game.projects.iter().any(|s| s.id == def.id) {
        return;
    }
    game.projects.push(ProjectState {
        id: def.id.clone(),
        filled: 0,
        segments: segments.unwrap_or(def.segments),
        carry: 0,
        opened: game.turn,
        completions: 0,
        pips_this_count: 0,
        rate: 0.0,
        roll: Roll::Plain,
    });
}

/// Progresses every open project by its dice, rolls once each, completes what fills.
///
/// Returns chronicle lines for setbacks and completions.
pub fn progress(game: &mut Game, defs: &[ProjectDef], rng: &mut impl Rng) -> Vec<String> {
    let mut lines = Vec::new();
    let n_present = game.present().count().az::<f64>();
    let mut completed: Vec<ProjectId> = Vec::new();
    for i in 0..game.projects.len() {
        let pid = game.projects[i].id.clone();
        let Some(def) = defs.iter().find(|d| d.id == pid) else {
            continue;
        };
        let mut pips: u32 = 0;
        let mut best: Option<(u8, PersonId)> = None;
        for (die, target) in &game.assignments {
            if *target != pid {
                continue;
            }
            match die {
                DieId::Person(p) => {
                    let f = face(game, *p, def.domain);
                    pips += u32::from(f);
                    if best.is_none_or(|(b, _)| f > b) {
                        best = Some((f, *p));
                    }
                }
                DieId::Robot(class, _) => pips += u32::from(class.pips()),
            }
        }
        let best_info = best.map(|(f, p)| {
            (
                f,
                game.person(p).condition.strain,
                game.person(p).name.clone(),
            )
        });
        let state = &mut game.projects[i];
        state.pips_this_count = pips;
        state.roll = Roll::Plain;
        if def.standing {
            let needed = def.pips_needed + def.pips_per_person * n_present;
            state.rate = if needed > 0.0 {
                (f64::from(pips) / needed).min(1.5)
            } else {
                1.0
            };
            continue;
        }
        if pips == 0 {
            continue;
        }
        let total = state.carry + pips;
        let gained = total / def.pips_per_segment;
        state.carry = total % def.pips_per_segment;
        state.filled = (state.filled + gained).min(state.segments);
        if let Some((f, strain, name)) = best_info {
            let d = rng.random_range(1..=6u8);
            let bonus_on = if f >= 4 { 5 } else { 6 };
            if d >= bonus_on {
                state.roll = Roll::Bonus;
                state.filled = (state.filled + 1).min(state.segments);
            } else if d == 1 && (f <= 2 || strain > 0.6) {
                state.roll = Roll::Setback;
                state.filled = state.filled.saturating_sub(1);
                lines.push(format!("{name}: setback on {}; a segment lost.", def.title));
            }
        }
        if state.filled >= state.segments {
            completed.push(pid.clone());
        }
    }
    for pid in completed {
        let Some(def) = defs.iter().find(|d| d.id == pid) else {
            continue;
        };
        let Some(idx) = game.projects.iter().position(|s| s.id == pid) else {
            continue;
        };
        let completions = game.projects[idx].completions + 1;
        game.projects.remove(idx);
        game.assignments.retain(|_, t| *t != pid);
        *game
            .counters
            .entry(format!("completed:{}", pid.0))
            .or_insert(0.0) += 1.0;
        game.stocks.spares = (game.stocks.spares - def.spares).max(0.0);
        if let Some(flag) = &def.on_complete_flag {
            game.flags.insert(flag.clone());
        }
        if let Some(l) = &def.on_complete_lexicon {
            game.lexicon_triggers.insert(l.clone());
        }
        if let Some((q, v)) = def.on_complete_add {
            let cur = game.quality(q);
            game.set_quality(q, cur + v);
        }
        lines.push(format!("{} complete.", def.title));
        if let Some(seg) = def.repeat_segments {
            open(game, def, Some(seg));
            if let Some(s) = game.projects.iter_mut().find(|s| s.id == pid) {
                s.completions = completions;
            }
        }
    }
    lines
}

/// Rate of a standing project this count, 0 if not open.
#[must_use]
pub fn rate(game: &Game, id: &str) -> f64 {
    game.projects
        .iter()
        .find(|s| s.id.0 == id)
        .map_or(0.0, |s| s.rate)
}

/// Assigns a die to a project or back to the hand. Errors are plain strings for the driver.
pub fn assign(game: &mut Game, defs: &[ProjectDef], die: &str, target: &str) -> Result<(), String> {
    let die = DieId::parse(die).ok_or_else(|| format!("unknown die {die}"))?;
    if target == "hand" {
        game.assignments.remove(&die);
        refresh_hand(game, defs);
        return Ok(());
    }
    let pid = ProjectId(target.to_owned());
    if !game.projects.iter().any(|s| s.id == pid) {
        return Err(format!("project {target} is not open"));
    }
    let def = defs
        .iter()
        .find(|d| d.id == pid)
        .ok_or_else(|| format!("unknown project {target}"))?;
    match die {
        DieId::Person(p) => {
            if !game
                .people
                .get(p.0.az::<usize>())
                .is_some_and(|x| x.alive && x.present)
            {
                return Err("that person is not present".into());
            }
            if game
                .hand
                .dice
                .iter()
                .any(|d| d.id == die.key() && d.place == "upkeep")
            {
                return Err("that person is eaten by upkeep this count".into());
            }
            if face(game, p, def.domain) < def.min_skill {
                return Err(format!(
                    "{} needs skill {} in {}",
                    def.title,
                    def.min_skill,
                    def.domain.key()
                ));
            }
        }
        DieId::Robot(class, k) => {
            if k >= game.robots.count(class).round().saturating_as::<u32>() {
                return Err("no such robot".into());
            }
            if !robot_fits(class, def) {
                return Err(format!("{} does not take {} units", def.title, class.key()));
            }
        }
    }
    game.assignments.insert(die, pid);
    refresh_hand(game, defs);
    Ok(())
}

/// Loads project definitions bundled into the binary.
pub fn bundled() -> Result<Vec<ProjectDef>, ContentError> {
    let mut out = Vec::new();
    for (file, text) in crate::content::BUNDLED_PROJECTS {
        out.push(ProjectDef::from_toml(file, text)?);
    }
    let mut ids: Vec<&str> = out.iter().map(|d| d.id.0.as_str()).collect();
    ids.sort_unstable();
    if ids.windows(2).any(|w| w[0] == w[1]) {
        return Err(ContentError::Invalid {
            file: "projects".into(),
            message: "duplicate project id".into(),
        });
    }
    Ok(out)
}

/// Serde helper so `BTreeMap<DieId, ProjectId>` survives JSON (keys must be strings).
pub mod assignment_map {
    use super::{DieId, ProjectId};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    /// Serializes as `{ "p:3": "bake_out" }`.
    pub fn serialize<S: Serializer>(
        map: &BTreeMap<DieId, ProjectId>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let out: BTreeMap<String, &str> =
            map.iter().map(|(k, v)| (k.key(), v.0.as_str())).collect();
        out.serialize(ser)
    }

    /// Parses the wire form back.
    pub fn deserialize<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<BTreeMap<DieId, ProjectId>, D::Error> {
        let raw: BTreeMap<String, String> = BTreeMap::deserialize(de)?;
        raw.into_iter()
            .map(|(k, v)| {
                DieId::parse(&k)
                    .map(|d| (d, ProjectId(v)))
                    .ok_or_else(|| serde::de::Error::custom(format!("bad die id {k}")))
            })
            .collect()
    }
}
