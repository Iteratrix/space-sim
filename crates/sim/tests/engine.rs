//! Engine-level tests: determinism, content validation, calendar structure.

use sim::{Engine, Quality, Storylet};

fn engine() -> Engine {
    Engine::bundled().expect("bundled engine")
}

fn play(engine: &Engine, seed: u64, max_turns: u32) -> (Vec<String>, u32) {
    let mut game = engine.new_game(seed).expect("game");
    while game.ending.is_none() && game.turn < max_turns {
        let (_, firings) = engine.advance(&mut game);
        for f in &firings {
            let choice =
                (usize::try_from(game.turn).unwrap_or(0) + f.options.len()) % f.options.len();
            engine.resolve(&mut game, f, choice);
        }
    }
    (engine.chronicle(&game), game.turn)
}

#[test]
fn same_seed_same_chronicle() {
    let e = engine();
    let a = play(&e, 42, 120);
    let b = play(&e, 42, 120);
    assert_eq!(a, b);
}

#[test]
fn different_seeds_differ() {
    let e = engine();
    let a = play(&e, 1, 60);
    let b = play(&e, 2, 60);
    assert_ne!(a.0, b.0);
}

#[test]
fn save_and_resume_replays_identically() {
    let e = engine();
    let mut game = e.new_game(9).expect("game");
    for _ in 0..40 {
        let (_, firings) = e.advance(&mut game);
        for f in &firings {
            e.resolve(&mut game, f, 0);
        }
    }
    let json = serde_json::to_string(&game).expect("serialize");
    let mut restored: sim::Game = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, game);
    let mut original = game.clone();
    for _ in 0..40 {
        let (ea, fa) = e.advance(&mut original);
        let (eb, fb) = e.advance(&mut restored);
        assert_eq!(ea, eb);
        assert_eq!(fa, fb);
        for (f, g) in fa.iter().zip(fb.iter()) {
            e.resolve(&mut original, f, 0);
            e.resolve(&mut restored, g, 0);
        }
    }
    assert_eq!(original, restored);
}

#[test]
fn every_game_reaches_an_ending() {
    let e = engine();
    for seed in 100..110 {
        let (_, turns) = play(&e, seed, 400);
        assert!(turns < 400, "seed {seed} did not end by count 400");
    }
}

#[test]
fn earth_windows_recur_about_every_sixteen_counts() {
    let e = engine();
    let game = e.new_game(1).expect("game");
    let opens: Vec<u32> = (0..240)
        .filter(|&t| game.earth_window_open_at(t) && (t == 0 || !game.earth_window_open_at(t - 1)))
        .collect();
    assert!(
        opens.len() >= 12,
        "only {} windows in 240 counts",
        opens.len()
    );
    for w in opens.windows(2) {
        let gap = w[1] - w[0];
        assert!((13..=20).contains(&gap), "gap {gap} between windows {w:?}");
    }
}

#[test]
fn every_quality_key_roundtrips() {
    for q in Quality::ALL {
        assert_eq!(Quality::parse(q.key()), Some(q), "{}", q.key());
    }
}

#[test]
fn storylet_rejects_unknown_quality() {
    let text = r#"
id = "bad"
title = "Bad"
text = "x"
[[when]]
q = "stocks.unobtainium"
gt = 1
[[option]]
id = "a"
label = "A"
chronicle = "a."
"#;
    assert!(Storylet::from_toml("bad.toml", text).is_err());
}

#[test]
fn storylet_rejects_writing_a_derived_quality() {
    let text = r#"
id = "bad"
title = "Bad"
text = "x"
[[option]]
id = "a"
label = "A"
chronicle = "a."
[[option.effect]]
q = "people.population"
add = 1
"#;
    assert!(Storylet::from_toml("bad.toml", text).is_err());
}

#[test]
fn storylet_rejects_effect_on_uncast_role() {
    let text = r#"
id = "bad"
title = "Bad"
text = "x"
[[option]]
id = "a"
label = "A"
chronicle = "a."
[[option.effect]]
person = "nobody"
strain = 0.1
"#;
    assert!(Storylet::from_toml("bad.toml", text).is_err());
}

#[test]
fn bundled_content_parses_and_ids_are_unique() {
    let e = engine();
    let mut ids: Vec<&str> = e.content.iter().map(|s| s.id.as_str()).collect();
    let n = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), n);
}

#[test]
fn quality_reads_do_not_panic_at_any_count() {
    let e = engine();
    let mut game = e.new_game(3).expect("game");
    for _ in 0..200 {
        let (_, firings) = e.advance(&mut game);
        for q in Quality::ALL {
            let v = game.quality(q);
            assert!(!v.is_nan(), "{} is NaN at count {}", q.key(), game.turn);
        }
        for f in &firings {
            e.resolve(&mut game, f, 0);
        }
        if game.ending.is_some() {
            break;
        }
    }
}

#[test]
fn no_robot_labour_floor_is_about_150() {
    let e = engine();
    let cap = |n: f64| n * e.params.labour.capacity_h_per_count;
    let sub = |n: f64| sim::turn::subsistence_hours(n, &e.params);
    assert!((sub(150.0) - cap(150.0)).abs() < 1.0);
    assert!(sub(100.0) > cap(100.0));
    assert!(sub(48.0) > cap(48.0) * 1.3);
    assert!(sub(200.0) < cap(200.0));
}

#[test]
fn population_at_the_silence_is_a_people_not_a_station() {
    let e = engine();
    for seed in 300..306 {
        let mut game = e.new_game(seed).expect("game");
        while game.ending.is_none() && game.turn < 400 {
            let (_, firings) = e.advance(&mut game);
            for f in &firings {
                let choice =
                    (usize::try_from(game.turn).unwrap_or(0) + f.options.len()) % f.options.len();
                e.resolve(&mut game, f, choice);
            }
        }
        let pop = game.present().count();
        assert!(pop >= 30, "seed {seed}: only {pop} people at the Silence");
    }
}

#[test]
fn minds_usually_outlive_act_one() {
    let e = engine();
    let mut alive_at_end = 0;
    for seed in 400..410 {
        let mut game = e.new_game(seed).expect("game");
        while game.ending.is_none() && game.turn < 400 {
            let (_, firings) = e.advance(&mut game);
            for f in &firings {
                e.resolve(&mut game, f, 0);
            }
        }
        alive_at_end += game.minds.iter().filter(|m| m.alive).count();
    }
    assert!(
        alive_at_end >= 10,
        "only {alive_at_end} of 20 minds alive at the Silence"
    );
}

#[test]
fn nobody_serves_five_skiff_tours() {
    let e = engine();
    for seed in 500..505 {
        let mut game = e.new_game(seed).expect("game");
        while game.ending.is_none() && game.turn < 400 {
            let (_, firings) = e.advance(&mut game);
            for f in &firings {
                e.resolve(&mut game, f, 0);
            }
        }
        let worst = game
            .people
            .iter()
            .map(|p| p.condition.dose_sv)
            .fold(0.0, f64::max);
        assert!(worst < 2.2, "seed {seed}: someone carries {worst:.2} Sv");
    }
}
