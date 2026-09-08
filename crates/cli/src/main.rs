//! Command-line driver for the settlement simulation.
//!
//! ```text
//! space-sim play [--seed N]            interactive, text
//! space-sim play --json [--seed N]     one JSON object per prompt on stdout; choices as lines on stdin
//! space-sim run [--seed N] [--policy P] [--quiet]   headless single game with a policy player
//! space-sim montecarlo [--games G] [--seed N] [--policy P]
//! space-sim validate                   parse every bundled storylet
//! space-sim calendar [--counts N]      print the Earth window table
//! ```

use az::Az;
use serde::Serialize;
use sim::director::Firing;
use sim::state::Ending;
use sim::{Engine, Game, Quality};
use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Policy {
    Random,
    First,
    Ring,
    Capability,
    Throughput,
}

impl Policy {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "random" => Some(Self::Random),
            "first" => Some(Self::First),
            "ring" => Some(Self::Ring),
            "capability" => Some(Self::Capability),
            "throughput" => Some(Self::Throughput),
            _ => None,
        }
    }

    fn choose(self, firing: &Firing, rng: &mut impl rand::Rng) -> usize {
        let n = firing.options.len();
        match self {
            Self::Random => rng.random_range(0..n),
            Self::First => 0,
            Self::Ring => {
                let mut votes: BTreeMap<usize, usize> = BTreeMap::new();
                for c in &firing.counsel {
                    *votes.entry(c.favours).or_default() += 1;
                }
                votes
                    .into_iter()
                    .max_by_key(|(_, v)| *v)
                    .and_then(|(fav, _)| firing.options.iter().position(|o| o.index == fav))
                    .unwrap_or(0)
            }
            Self::Capability | Self::Throughput => {
                let want = if self == Self::Capability {
                    "capability"
                } else {
                    "throughput"
                };
                firing
                    .options
                    .iter()
                    .position(|o| o.tags.iter().any(|t| t == want))
                    .unwrap_or_else(|| rng.random_range(0..n))
            }
        }
    }
}

struct Args {
    command: String,
    seed: u64,
    games: usize,
    counts: usize,
    policy: Policy,
    json: bool,
    save: Option<String>,
    load: Option<String>,
    scenario: sim::setup::Scenario,
    quiet: bool,
    trace: bool,
    max_turns: u32,
}

fn parse_args() -> Args {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".into());
    let mut out = Args {
        command,
        seed: 1,
        games: 200,
        counts: 120,
        policy: Policy::Random,
        json: false,
        save: None,
        load: None,
        scenario: sim::setup::Scenario::Act1,
        quiet: false,
        trace: false,
        max_turns: 400,
    };
    while let Some(a) = args.next() {
        match a.as_str() {
            "--seed" => out.seed = args.next().and_then(|v| v.parse().ok()).unwrap_or(1),
            "--games" => out.games = args.next().and_then(|v| v.parse().ok()).unwrap_or(200),
            "--counts" => out.counts = args.next().and_then(|v| v.parse().ok()).unwrap_or(120),
            "--max-turns" => {
                out.max_turns = args.next().and_then(|v| v.parse().ok()).unwrap_or(400);
            }
            "--policy" => {
                out.policy = args
                    .next()
                    .and_then(|v| Policy::parse(&v))
                    .unwrap_or(Policy::Random);
            }
            "--json" => out.json = true,
            "--scenario" => {
                out.scenario = args
                    .next()
                    .and_then(|v| sim::setup::Scenario::parse(&v))
                    .unwrap_or(sim::setup::Scenario::Act1);
            }
            "--save" => out.save = args.next(),
            "--load" => out.load = args.next(),
            "--quiet" => out.quiet = true,
            "--trace" => out.trace = true,
            _ => {}
        }
    }
    out
}

fn status_line(game: &Game) -> String {
    let q = |x: Quality| game.quality(x);
    format!(
        "count {:>3} | pop {:>3} ({} res) | water {:>5.0} t | N2 {:>5.0} kg | spares {:>4.0} | closure {:.2} | power {:.0}/{:.0} kW | labour {:+.0}% | φ {:.2} | sponsor conf {:.2} att {:.2} stage {} | window {} | strain {:.2} | griev {:.1} susp {:.1} leak {:.1}",
        game.turn,
        q(Quality::Population),
        q(Quality::Residents),
        q(Quality::Water),
        q(Quality::Nitrogen),
        q(Quality::Spares),
        q(Quality::Closure),
        q(Quality::PowerCapacity),
        q(Quality::PowerDemand),
        q(Quality::LabourDeficit) * 100.0,
        q(Quality::Phi),
        q(Quality::SponsorConfidence),
        q(Quality::SponsorAttention),
        q(Quality::SponsorStage),
        if game.earth_window_open() {
            "OPEN".to_owned()
        } else {
            format!("in {}", game.counts_to_window())
        },
        q(Quality::MeanStrain),
        q(Quality::Grievance),
        q(Quality::Suspicion),
        q(Quality::Leak),
    )
}

fn hand_line(game: &Game) -> String {
    use std::fmt::Write as _;
    let h = &game.hand;
    let mut out = format!(
        "hand: {} of {} free, {} on upkeep, {} robots |",
        h.free, h.adults, h.eaten, h.robots
    );
    for p in &game.projects {
        let n = h.dice.iter().filter(|d| d.place == p.id.0).count();
        if p.segments > 0 {
            let _ = write!(out, " {} {}/{} [{n}]", p.id.0, p.filled, p.segments);
        } else {
            let _ = write!(out, " {} rate {:.2} [{n}]", p.id.0, p.rate);
        }
    }
    out
}

fn print_firing(engine: &Engine, game: &Game, f: &Firing) {
    println!("\n=== {} ===", f.title);
    for line in f.text.trim().lines() {
        println!("  {}", sim::lexicon::render(game, line));
    }
    println!();
    for (i, o) in f.options.iter().enumerate() {
        println!("  [{}] {}", i + 1, o.label);
        if !o.text.is_empty() {
            println!("      {}", o.text);
        }
    }
    println!("\n  The ring:");
    for c in &f.counsel {
        let opt = f
            .options
            .iter()
            .position(|o| o.index == c.favours)
            .map_or(0, |i| i + 1);
        println!("    {}, for [{}]: {}", c.holder, opt, c.text);
    }
    let _ = engine;
}

fn save_game(game: &Game, path: &str) {
    match serde_json::to_string(game) {
        Ok(json) => {
            if let Err(e) = std::fs::write(path, json) {
                eprintln!("save failed: {e}");
            }
        }
        Err(e) => eprintln!("save failed: {e}"),
    }
}

fn report(r: Result<(), String>, json: bool) {
    match (r, json) {
        (Ok(()), true) => println!("{}", serde_json::json!({ "ok": true })),
        (Ok(()), false) => println!("  ok"),
        (Err(e), true) => println!("{}", serde_json::json!({ "error": e })),
        (Err(e), false) => println!("  {e}"),
    }
}

fn command(engine: &Engine, game: &mut Game, line: &str, json: bool) -> bool {
    if line == "view" {
        match serde_json::to_string(&engine.view(game)) {
            Ok(j) => println!("{j}"),
            Err(e) => eprintln!("{e}"),
        }
        return true;
    }
    if let Some(rest) = line.strip_prefix("assign ") {
        let mut it = rest.split_whitespace();
        if let (Some(die), Some(target)) = (it.next(), it.next()) {
            report(engine.assign(game, die, target), json);
        }
        return true;
    }
    if let Some(rest) = line.strip_prefix("set ") {
        let mut it = rest.split_whitespace();
        if let (Some(c), Some(v)) = (it.next(), it.next()) {
            report(engine.set_control(game, c, v), json);
        }
        return true;
    }
    false
}

fn next_line(lines: &mut std::io::Lines<std::io::StdinLock<'static>>) -> String {
    let Some(Ok(line)) = lines.next() else {
        std::process::exit(0)
    };
    let line = line.trim().to_owned();
    if line == "q" || line == "quit" {
        std::process::exit(0);
    }
    line
}

fn resolve_firings(
    engine: &Engine,
    game: &mut Game,
    firings: &[Firing],
    args: &Args,
    lines: &mut std::io::Lines<std::io::StdinLock<'static>>,
) {
    for f in firings {
        if !args.json {
            print_firing(engine, game, f);
        }
        let choice = loop {
            if !args.json {
                print!("> ");
                std::io::stdout().flush().ok();
            }
            let line = next_line(lines);
            if command(engine, game, &line, args.json) {
                continue;
            }
            if let Ok(n) = line.parse::<usize>()
                && n >= 1
                && n <= f.options.len()
            {
                break n - 1;
            }
            if let Some(i) = f.options.iter().position(|o| o.id == line) {
                break i;
            }
            if !args.json {
                println!("  choose 1-{}", f.options.len());
            }
        };
        if let Some(line) = engine.resolve(game, f, choice) {
            if args.json {
                println!("{}", serde_json::json!({ "chronicle": line }));
            } else {
                println!("  -> {line}");
            }
        }
    }
    loop {
        if !args.json {
            print!("[end] ");
            std::io::stdout().flush().ok();
        }
        let line = next_line(lines);
        if line.is_empty() || line == "end" {
            break;
        }
        if !command(engine, game, &line, args.json) && !args.json {
            println!(
                "  commands: view | assign <die> <project|hand> | set <control> <value> | end"
            );
        }
    }
}

fn play(engine: &Engine, args: &Args) {
    let mut game = args.load.as_ref().map_or_else(
        || {
            engine
                .new_game_scenario(args.seed, args.scenario)
                .expect("new game")
        },
        |path| {
            let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("load failed: {e}");
                std::process::exit(1)
            });
            serde_json::from_str(&text).unwrap_or_else(|e| {
                eprintln!("load failed: {e}");
                std::process::exit(1)
            })
        },
    );
    let mut lines = std::io::stdin().lines();
    println!("{}", engine.chronicle(&game).join("\n"));
    loop {
        let (events, firings) = engine.advance(&mut game);
        if args.json {
            let view = TurnView {
                status: status_line(&game),
                events: &events.lines,
                firings: &firings,
                ending: &game.ending,
            };
            println!("{}", serde_json::to_string(&view).expect("json"));
        } else {
            println!("\n{}", status_line(&game));
            println!("{}", hand_line(&game));
            for e in &events.lines {
                println!("  * {}", sim::lexicon::render(&game, e));
            }
        }
        if let Some(path) = &args.save {
            save_game(&game, path);
        }
        resolve_firings(engine, &mut game, &firings, args, &mut lines);
        if let Some(ending) = &game.ending {
            if args.json {
                println!(
                    "{}",
                    serde_json::json!({ "summary": sim::report::summary(&game) })
                );
            } else {
                println!(
                    "\n{}\n{}",
                    describe_ending(ending),
                    sim::report::summary(&game)
                );
            }
            break;
        }
    }
}

#[derive(Serialize)]
struct TurnView<'a> {
    status: String,
    events: &'a [String],
    firings: &'a [Firing],
    ending: &'a Option<Ending>,
}

fn describe_ending(e: &Ending) -> String {
    match e {
        Ending::Silence {
            last_convoy,
            population,
        } => {
            format!(
                "THE SILENCE. Last convoy at count {last_convoy}; {population} people remain. Act one ends."
            )
        }
        Ending::Closed { reason } => format!("CLOSED: {reason}"),
        Ending::Extinct { reason } => format!("EXTINCT: {reason}"),
    }
}

struct RunResult {
    turns: u32,
    ending: Option<Ending>,
    fired: BTreeMap<String, u32>,
    chosen: BTreeMap<String, u32>,
    population: usize,
    residents: usize,
    closure: f64,
    stage: u8,
    chronicle: Vec<String>,
    summary: String,
}

fn run_one(
    engine: &Engine,
    seed: u64,
    policy: Policy,
    max_turns: u32,
    trace: bool,
    scenario: sim::setup::Scenario,
) -> RunResult {
    use rand::SeedableRng;
    let mut game = engine.new_game_scenario(seed, scenario).expect("new game");
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0x00C0_FFEE);
    let mut fired: BTreeMap<String, u32> = BTreeMap::new();
    let mut chosen: BTreeMap<String, u32> = BTreeMap::new();
    while game.ending.is_none() && game.turn < max_turns {
        let (_, firings) = engine.advance(&mut game);
        if trace && game.turn.is_multiple_of(6) {
            eprintln!("{}", status_line(&game));
            eprintln!("{}", hand_line(&game));
        }
        for f in &firings {
            let choice = policy.choose(f, &mut rng);
            *fired.entry(f.id.clone()).or_default() += 1;
            *chosen
                .entry(format!("{}/{}", f.id, f.options[choice].id))
                .or_default() += 1;
            engine.resolve(&mut game, f, choice);
        }
    }
    RunResult {
        turns: game.turn,
        ending: game.ending.clone(),
        fired,
        chosen,
        population: game.present().count(),
        residents: game
            .present()
            .filter(|p| p.contract_end().is_none())
            .count(),
        closure: game.closure,
        stage: game.sponsor.stage.index(),
        chronicle: engine.chronicle(&game),
        summary: sim::report::summary(&game),
    }
}

fn montecarlo(engine: &Engine, args: &Args) {
    let mut endings: BTreeMap<String, usize> = BTreeMap::new();
    let mut fired: BTreeMap<String, u32> = BTreeMap::new();
    let mut chosen: BTreeMap<String, u32> = BTreeMap::new();
    let mut turns = Vec::new();
    let mut pops = Vec::new();
    let mut residents = Vec::new();
    let mut closures = Vec::new();
    let mut stages: BTreeMap<u8, usize> = BTreeMap::new();
    for g in 0..args.games {
        let r = run_one(
            engine,
            args.seed + g.az::<u64>(),
            args.policy,
            args.max_turns,
            false,
            args.scenario,
        );
        let key = match &r.ending {
            Some(Ending::Silence { .. }) => "silence",
            Some(Ending::Closed { .. }) => "closed",
            Some(Ending::Extinct { .. }) => "extinct",
            None => "timeout",
        };
        *endings.entry(key.into()).or_default() += 1;
        for (id, n) in r.fired {
            *fired.entry(id).or_default() += n;
        }
        for (id, n) in r.chosen {
            *chosen.entry(id).or_default() += n;
        }
        turns.push(f64::from(r.turns));
        pops.push(r.population.az::<f64>());
        residents.push(r.residents.az::<f64>());
        closures.push(r.closure);
        *stages.entry(r.stage).or_default() += 1;
    }
    let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len().max(1).az::<f64>();
    let pct = |v: &mut Vec<f64>, p: f64| {
        v.sort_by(f64::total_cmp);
        v[((v.len() - 1).az::<f64>() * p).round().az::<usize>()]
    };
    println!("games {} policy {:?}", args.games, args.policy);
    println!("endings: {endings:?}");
    println!("final sponsor stage: {stages:?}");
    println!(
        "turns: mean {:.0} p10 {:.0} p50 {:.0} p90 {:.0}",
        mean(&turns),
        pct(&mut turns.clone(), 0.1),
        pct(&mut turns.clone(), 0.5),
        pct(&mut turns.clone(), 0.9)
    );
    println!(
        "population at end: mean {:.1} p10 {:.0} p90 {:.0}",
        mean(&pops),
        pct(&mut pops.clone(), 0.1),
        pct(&mut pops.clone(), 0.9)
    );
    println!("residents at end: mean {:.1}", mean(&residents));
    println!("closure at end: mean {:.3}", mean(&closures));
    println!("storylets fired (per game):");
    let total_games = args.games.az::<f64>();
    let mut fired: Vec<_> = fired.into_iter().collect();
    fired.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (id, n) in &fired {
        println!("  {:<32} {:>6.2}", id, f64::from(*n) / total_games);
    }
    let never: Vec<&str> = engine
        .content
        .iter()
        .map(|s| s.id.as_str())
        .filter(|id| !fired.iter().any(|(f, _)| f == id))
        .collect();
    if !never.is_empty() {
        println!("never fired: {never:?}");
    }
    let never_chosen: Vec<String> = engine
        .content
        .iter()
        .flat_map(|s| s.options.iter().map(move |o| format!("{}/{}", s.id, o.id)))
        .filter(|k| !chosen.contains_key(k))
        .collect();
    if !never_chosen.is_empty() {
        println!(
            "options never chosen ({}): {never_chosen:?}",
            never_chosen.len()
        );
    }
}

fn calendar(engine: &Engine, args: &Args) {
    let game = engine.new_game(1).expect("new game");
    println!("count  out_kms  home_kms  r_au  dist_au  open");
    for t in 0..args.counts.min(game.calendar.outbound_cost.len()) {
        println!(
            "{:>5}  {:>7.2}  {:>8.2}  {:>4.2}  {:>7.2}  {}",
            t,
            game.calendar.outbound_cost[t],
            game.calendar.homeward_cost[t],
            game.calendar.home_r_au[t],
            game.calendar.earth_distance_au[t],
            if game.earth_window_open_at(t.az::<u32>()) {
                "*"
            } else {
                ""
            }
        );
    }
    println!(
        "best outbound {:.2}, best homeward {:.2}",
        game.calendar.outbound_best, game.calendar.homeward_best
    );
}

fn main() {
    let args = parse_args();
    if args.command == "validate" {
        match Engine::bundled() {
            Ok(e) => {
                println!("{} storylets ok", e.content.len());
                for s in &e.content {
                    println!(
                        "  {} ({} options, {} roles, priority {})",
                        s.id,
                        s.options.len(),
                        s.cast.len(),
                        s.priority
                    );
                }
            }
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
        return;
    }
    let engine = match Engine::bundled() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    match args.command.as_str() {
        "play" => play(&engine, &args),
        "run" => {
            let r = run_one(
                &engine,
                args.seed,
                args.policy,
                args.max_turns,
                args.trace,
                args.scenario,
            );
            if !args.quiet {
                for line in &r.chronicle {
                    println!("{line}");
                }
            }
            println!("\n{}", r.summary);
            println!(
                "\n-- {} turns, ending {:?}, population {}, residents {}, closure {:.2}, stage {}",
                r.turns,
                r.ending.as_ref().map(describe_ending),
                r.population,
                r.residents,
                r.closure,
                r.stage
            );
        }
        "montecarlo" => montecarlo(&engine, &args),
        "calendar" => calendar(&engine, &args),
        _ => {
            println!(
                "usage: space-sim <play|run|montecarlo|validate|calendar> [--seed N] [--games G] [--policy random|first|ring|capability|throughput] [--json] [--quiet] [--counts N] [--max-turns N]"
            );
        }
    }
}
