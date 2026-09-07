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
use std::io::{BufRead, Write};

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
                out.max_turns = args.next().and_then(|v| v.parse().ok()).unwrap_or(400)
            }
            "--policy" => {
                out.policy = args
                    .next()
                    .and_then(|v| Policy::parse(&v))
                    .unwrap_or(Policy::Random);
            }
            "--json" => out.json = true,
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
        println!(
            "    {} ({}), for [{}]: {}",
            c.holder,
            c.seat.title(),
            opt,
            c.text
        );
    }
    let _ = engine;
}

fn play(engine: &Engine, args: &Args) {
    let mut game = engine.new_game(args.seed).expect("new game");
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
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
            for e in &events.lines {
                println!("  * {}", sim::lexicon::render(&game, e));
            }
        }
        if let Some(ending) = &game.ending {
            if !args.json {
                println!("\n{}", describe_ending(ending));
            }
            break;
        }
        for f in &firings {
            if !args.json {
                print_firing(engine, &game, f);
            }
            let choice = loop {
                if !args.json {
                    print!("> ");
                    std::io::stdout().flush().ok();
                }
                let Some(Ok(line)) = lines.next() else { return };
                let line = line.trim();
                if line == "q" || line == "quit" {
                    return;
                }
                if let Ok(n) = line.parse::<usize>() {
                    if n >= 1 && n <= f.options.len() {
                        break n - 1;
                    }
                }
                if !args.json {
                    println!("  choose 1-{}", f.options.len());
                }
            };
            if let Some(line) = engine.resolve(&mut game, f, choice) {
                if args.json {
                    println!("{}", serde_json::json!({ "chronicle": line }));
                } else {
                    println!("  -> {line}");
                }
            }
        }
        if firings.is_empty() && !args.json {
            let Some(Ok(line)) = lines.next() else { return };
            if line.trim() == "q" {
                return;
            }
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
    population: usize,
    residents: usize,
    closure: f64,
    stage: u8,
    chronicle: Vec<String>,
}

fn run_one(engine: &Engine, seed: u64, policy: Policy, max_turns: u32, trace: bool) -> RunResult {
    use rand::SeedableRng;
    let mut game = engine.new_game(seed).expect("new game");
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed ^ 0xC0FFEE);
    let mut fired: BTreeMap<String, u32> = BTreeMap::new();
    while game.ending.is_none() && game.turn < max_turns {
        let (_, firings) = engine.advance(&mut game);
        if trace && game.turn % 6 == 0 {
            eprintln!("{}", status_line(&game));
        }
        for f in &firings {
            let choice = policy.choose(f, &mut rng);
            *fired.entry(f.id.clone()).or_default() += 1;
            engine.resolve(&mut game, f, choice);
        }
    }
    RunResult {
        turns: game.turn,
        ending: game.ending.clone(),
        fired,
        population: game.present().count(),
        residents: game
            .present()
            .filter(|p| p.contract_end().is_none())
            .count(),
        closure: game.closure,
        stage: game.sponsor.stage.index(),
        chronicle: engine.chronicle(&game),
    }
}

fn montecarlo(engine: &Engine, args: &Args) {
    let mut endings: BTreeMap<String, usize> = BTreeMap::new();
    let mut fired: BTreeMap<String, u32> = BTreeMap::new();
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
    fired.sort_by(|a, b| b.1.cmp(&a.1));
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
            let r = run_one(&engine, args.seed, args.policy, args.max_turns, args.trace);
            if !args.quiet {
                for line in &r.chronicle {
                    println!("{line}");
                }
            }
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
