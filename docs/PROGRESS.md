# Progress and handoff

Written for whoever picks this up next (human or agent). Newest at the top.

## Day 3 (2026-09-11): onramp v2

- **Diagnosis.** The v1 tutorial taught by telling, gated on the calendar, ran 43
  counts, and could be clicked through without performing a mechanic (playtests 3-4).
- **Three required counts.** Count 1: one scene, two options, End count. Count 2: the
  hand and one short clock ("Shelter, first chamber", 6 segments); End count is held —
  the button reads the requirement — until a rated die is on it. Count 3: a robot die
  and the bake-out's rate ring; held until a robot is placed. Then the game is open.
  `crates/sim/src/tutorial.rs`: `require:place_person:<project>` /
  `require:place_robot:<project>` flags a scene sets; the engine clears them on the real
  state (assign, resolve, advance), treats a completed project as satisfied and an
  unopened one as pending; `view.required` carries the text; the CLI's policies satisfy
  requirements mechanically (by the project's domain face) and deal free dice from
  count 4 so headless runs exercise the clocks.
- **Just-in-time disclosure, engine-owned.** `tutorial::disclose` reveals each part of
  the screen the first time the state makes it matter (ring on the first seat, RSW
  clock within six counts, water when it moves, a pressure when it leaves band 0,
  spares with the first convoy, the mass driver on alignment, crew with the chamber,
  power/reactor when tight, margin when low, contracts within four counts, roster and
  auto-deal at twenty crew, the sponsor track at the first review). No condition-opened
  projects clutter the lesson counts; standing projects are open from the start.
- **Plain labels** with the acronym in parentheses ("Shelter (KEEP)", "Mass driver
  (MDLS)", "Resupply window (RSW)"); parenthesised acronyms never drift.
- **Ten tutorial scenes** (Opus), every report under 80 words, every scene a decision.
- **Page**: End count held with the requirement as its label; idle standing clocks
  look off; a lifted die shows its face beside every clock; the last count's lines
  carry into the next count's events; captions accumulate (last two per section);
  "favoured" only on a unique plurality of the ring.
- **Acceptance** (`docs/design/playtest-gui-5.md`, `-6.md`): held at count 2; count 3
  failed once (the bake-out was suppressed during the lesson — fixed); the careful
  reader read ≈340 words before the game opened against v1's ~2,500 and called it the
  same game. A second impatient run (`-7.md`) is the re-check.

## Day 2, night: two more playtests, the feel gate, calm months

- Playtests 3 (the careful reader) and 4 (the impatient player) on v0.1.4 — most
  reveals landed from the screen alone; the throw lesson failed structurally
  (clock opened empty, nobody asked to staff it); the game "never raised its voice"
  when the plant sat idle. Fixed: upkeep evicts the worst face and says so; idle
  standing projects reported; scenes can place robots (`assign_robots`); projects
  opened by a scene are droppable at once; a lifted die shows its face beside each
  clock; clocks show their pips the moment dice land; the events pane survives a
  reload; the tutorial rewritten again (the throw lesson names operations and
  staffs the MDLS; directives enacted; single-option briefings given a choice).
- **The feel gate** (`space-sim feel`, in CI): quiet streaks, never-fired,
  never-chosen, silent seats, endings — limits 5 (tutorial) and 9 (act 1).
- **Twelve calm-month scenes** (`40-calm-*`): act 1's worst silence 12 → 7-9;
  attention decay retuned so act 1 stays near fifteen years.

## Day 2, evening: the report register and progressive disclosure

- **The report register.** Act 1's narrator is the station mind under the HRA giving
  status reports to the Commander: every scene, option, chronicle line and relayed
  recommendation, plus the engine's own lines, the view's sentences, the mood words and
  the project descriptions (writers' guide §2b). The seats' questions and the Silence
  line keep their own voices on purpose. Content agents run on the Opus line.
- **Progressive disclosure** (`docs/design/tutorial-disclosure.md`): twenty lessons
  across the forty-three counts; the page starts as the scene column and End count
  and reveals sections, bars, clocks, controls, robot dice and pressure rings on
  `ui:*` flags that scenes set, each with a one-line caption from the mind; "Start,
  show everything" skips it. Tutorial v3 (27 scenes) sets the flags in the plan's
  order; the tutorial starts with auto-deal off and only the KEEP open; a scene can
  place a die (`assign`), open a project, set any standing control by flag.
- `web/test/play.py` now reports only what is visible, so an agent playtests what a
  person sees.

## Day 2, afternoon: the loop closes

- **Agents play the page.** `web/test/play.py` drives the real browser (Playwright,
  DOM-driven, one action per call, one profile per agent, screenshots they read).
  Two GUI playtests (`docs/design/playtest-gui-1.md`, `-2.md`) found the tutorial
  narrating dice it never moved, controls lagging their scenes, robots unassignable
  from the page, dice vanishing on completion, unpersisted assignments, bars showing
  stocks not flows. All the engine/page items are fixed; the content went back to a
  writer.
- **Writers' guide** (`docs/design/writers-guide.md`): the bible every content agent
  reads first — the three registers with the vocabulary ladder, how each seat speaks,
  the anatomy of a scene with annotated examples, the tutorial's rules, projects, a
  twelve-point checklist. Content agents run on the Opus line.
- **Tutorial v2**: twenty scenes; the ring is empty at MM 1 and gains a seat as each
  question arises (`seat:*` flags gate seats during the tutorial); allocation is asked,
  not narrated; a missed-window scene; `tutorial_open` lets texture in after RSW-1;
  three teaching texture scenes (pressures, roster order, throw position); the
  handoff is the moment the language turns (the engine holds `first_convoy` until
  `tutorial_done`).
- Engine additions for writers: `top = true` exact casting, the `assign` effect,
  `roster_*` flags, sentence-case lexicon drift, one scene per tutorial count,
  `on_complete_add` on projects, ledger deltas, rotation strips logged.
- **Research 14** (`docs/research/14-closed-loop-gamedev.md`): nobody has published
  this loop; Play2Code (source-blind GUI player + coder, 30%→72%) and MeepleLM
  (persona critics) are the nearest; recommended next: a jury of disjoint model
  families as critic, world-state variants in the sweep, five fixed playtester
  personas with an MDA diary, and "feel" assertions on the diary (no more than N
  quiet counts) as a CI gate.

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
