# Progress and handoff

Written for whoever picks this up next (human or agent). Newest at the top.

## State as of the overnight build (2026-09-07, early)

**What exists and works**

- `crates/orbit`: Kepler propagation, zero-rev Lambert, Hohmann/Edelbaum/phasing, the
  79-body catalogue (`data/bodies.json`, 77 SBDB bodies + Earth and Mars). Validated
  against JPL Horizons vectors for Ceres, Earth, Mars, Fortuna, Vesta at 2030-01-01
  (`crates/orbit/tests/horizons.rs`). Clippy-clean.
- `crates/sim`: the act-1 loop, headless and deterministic per seed. Monthly counts.
  Persons with traits, condition, skills, tenure; directed ties; derived group indices.
  The sponsor as an actor (runway, confidence, attention, review clock, degradation
  stages, expansion while keen). Convoys on the real Earth-Fortuna windows. Labour after
  Salotti (corrected). Closure with vitamin parts. Reactor, relay, robot and mind
  attrition. Licence heartbeat -> grace -> lapse -> fail open/closed. Dose by estate.
  Storylets in TOML with casting, typed conditions/effects, authored advice; a director
  with priorities, weights, cooldowns; the ring's counsel with seat biases and
  skill-gated wrong advice; the chronicle rendered through a trigger-gated lexicon.
  Ending: the Silence (act-1 end), Closed, Extinct. Save/resume replays exactly
  (`crates/sim/tests/engine.rs`).
- `crates/cli` (`space-sim`): `play` (text or `--json` for agents), `run --trace`,
  `montecarlo` (endings, stage distribution, per-storylet firing rates, options never
  chosen), `validate`, `calendar`. `--save/--load` on play.
- Content: `data/storylets/*.toml`. Format in `docs/design/storylet-format.md`.
- Design: `docs/design/canon.md` is the reference; `NOTES.md` the decision log;
  `docs/research/01-13` the research; two Voidborn pitches in `docs/design/`.

**Tuned state (random policy, 100 games):** every game reaches the Silence; act 1
lasts ~145-210 counts (mean ~173); population at the trail-off 64-113 (mean ~88),
~75 of them residents; final sponsor stage spread over 3-6.

**Known gaps and next steps, in order**

1. Read `docs/design/review-engine-1.md` (the critique agent's review) and apply what
   it finds; it was written against the engine before the last tuning pass.
2. Act 1 is still a little long; canon wants ~10-13 years. Levers: `sponsor.attention_decay_per_review`,
   the composite-health weights in `turn.rs::sponsor`, the Silence rule in `turn.rs::endings`.
3. Content coverage: run `montecarlo --games 200` and look at `never fired` and
   `options never chosen`; loosen or cut.
4. The ring's generated advice is bland when unauthored; more `[[option.advice]]`.
5. Per-person flags, estate/skill conditions in `[[when]]`, and referencing a specific
   mind are missing from the storylet format (content authors asked for them).
6. Act 2 is not started. The state it needs is all in `Game`; the act-2 director,
   the Machines/Children/Sky seats, the Voided, stopping the spin, other clans, and Mars
   are storylets plus a handful of new qualities away.
7. No UI beyond the terminal. A wasm front over the JSON protocol is the natural next
   step (see the `wasm-pages-app` skill).

**How to run** — see README.md.
