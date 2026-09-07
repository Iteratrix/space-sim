# Progress and handoff

Written for whoever picks this up next (human or agent). Newest at the top.

## Day 2 (2026-09-07): projects and the hand, the register, the tutorial, the web front

- **Projects and dice** (`crates/sim/src/project.rs`, `data/projects/`): content-defined
  clocks; each adult is a die whose face is their skill in the project's domain
  (dulled above 0.5 strain); robots are square dice that carry upkeep unless assigned
  and fit only structured work (dex fit anything). Upkeep after Salotti is scaled by
  how supplied the outpost is (0.65 while the sponsor is keen, 1.0 after the sale);
  the deal computes what unassigned robots cover, eats people first, keeps a hand of
  ~3 when auto-dealing, staffs one-time projects before standing ones. One skill-gated
  roll per project per count. Standing projects (`bake_out`, `throw`) are rates that
  replace the old flat extraction; closure now decays with the upkeep shortfall.
- **Standing controls**: manifest split, throw position (ship / hold at reserve /
  stop), roster order, auto-deal. `assign` and `set` verbs in the CLI and the wasm.
- **Register**: canon §9, a three-column lexicon ladder (acronym → act-1 slang →
  Voidborn word), all storylets and the engine's own lines in the NASA register, seat
  titles as job titles; chronicle entries keep a snapshot of the vocabulary of their
  count (`ChronicleEntry.words`).
- **Tutorial** (`--scenario tutorial`, the default in the browser): 14 crew arrive at
  a robot-built site with no KEEP, the driver unaligned, one mind that kept a log;
  fifteen scripted scenes (`data/storylets/00-tutorial-*.toml`) across the first
  three windows, one per count, handing off at MM 43. Only priority ≥ 100 scenes fire
  while the `tutorial` flag is set and `tutorial_done` is not.
- **View** (`crates/sim/src/view.rs`): countdowns, projects with dice, the hand, seven
  ledger bars with a word and a why, three pressures with band names, the ring, the
  sponsor's track, controls, the calendar slice, the chronicle. `Quality::Margin`.
  Zero crossings and pressure band crossings write chronicle lines; quiet counts
  (~87 lines a game instead of ~250).
- **Web front** (`crates/web`, `web/`): wasm-bindgen bridge with eight functions and
  the state in the wasm; a plain-JS page with the clock rail, the hand (drag or
  tap-tap to assign), ledger, pressures, sponsor dots, the scene, the ring, and the
  chronicle drawer; saves to localStorage every count. Smoke-tested headless with
  Playwright (`web/test/smoke.py`). Deploys to GitHub Pages on a version tag.
- Balance now: act 1 149-213 counts, 110-158 people at the Silence (random policy).

**Next**: faces for the ring; the phone reflow; standing (trust) diamonds; the Map
with the one slider; "zero is a scene" storylets against the `zero:*` counters and
the reserve; act-1 balance under the dice model; act 2.

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

**Tuned state (100 games per policy, end of the overnight build):** every game
reaches the Silence; none starves. Act 1 lasts 149-197 counts (mean ~170, about
14 years) under the random and ring policies, ~195 under "always option 1".
Population at the trail-off 88-157 (mean ~115-147 by policy), inside canon's
110-240 band; final sponsor stage spread over 1-6. Mean dose ~0.5 Sv; minds usually
outlive the act. Fifty-one storylets, all firing; two critiques and one
content-quality pass applied. 200 games run in ~4 s in release. CI is green.

**Reviews:** `docs/design/review-engine-1.md` and `review-engine-2.md`. Applied
from review 2: the famine chain (spares at zero no longer selects the no-sponsor
attrition table while the sponsor lives; farm deaths slower and only from an empty
margin), the sponsor's phi expectation now ramps from 1 by 40%/review to a cap of
8 (it started at an unreachable 6), exponential mind attrition, tie hindrance that
grows and decays at rates that let coherence move, skiff tours as a one-time career
(second tours only after 96 counts and never for seat holders), cataract onset on
Gy (Sv/2.5), the Unforgetting announced on any missed reset. Not applied: robots
as capped rather than flat hours and a human-premium term, `p_miss` by stage,
hashes and the chosen option id in saves, conjunction as a licence interrupt,
the phantom 20 t/count of bulk when the tank is empty (it is regolith; label it).

**Known gaps and next steps, in order**

1. `docs/design/review-engine-1.md` was the first critique. Applied from it: robot
   hours in the labour model (no-robot floor now ~150, tested), ties for arrivals,
   skiff rotation and a probabilistic cataract onset, cast rotation away from
   recently cast people, authored advice rendered and stance-aware, must-scene
   ordering, baseline spares on every convoy, manifest flags shaping the convoy,
   `leave` honoured after the people-ship stops, stage flags filled on jumps,
   storylets on the ending count, coherence as "who has an enemy", mind attrition
   matching its parameter. Not yet applied: content-id-keyed `resolve`, a
   params/content hash in saves, per-person predicates and flags in storylets,
   addressing a specific mind, moving the ~40 magic numbers in `turn.rs` into
   `params.toml`, Sale resetting the sponsor's metrics, an "option chosen"
   condition.
2. Act 1 is a year or two long against canon's 10-13; the levers are
   `sponsor.attention_decay_per_review`, the 1.4 expectation growth in
   `turn.rs::sponsor`, and the Silence rule in `turn.rs::endings`. Levers: `sponsor.attention_decay_per_review`,
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
