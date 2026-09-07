# Engine review 2

Snapshot: sources as of 2026-09-06 23:57 (`turn.rs`, `storylet.rs`, `ring.rs`
were edited while I read them; line numbers are from the files on disk at
23:58, and every number quoted is from a binary built at 23:58). Runs: `run
--seed 7 --trace --quiet`, `montecarlo --games 200` under `random`, `ring`,
`first`, `run --seed 21 --policy ring`, plus a scratch probe linked against
the crate for per-count series and a 40-seed death census under `first`.

Headline: the mechanical fixes landed (ties, must ordering, advice, skill-cast
rotation, `resolve_by_id`) and the tests still pass. The model is still two
countdowns in series: confidence runs out by count ~110 because φ can never
meet an expectation set at 6 when 2 is the arithmetic maximum, then attention
runs out on a fixed slope. Underneath, the fixes created a famine engine:
spares → dead-rate robot attrition → haul collapse → water 0 → one death every
three counts, in 40 of 40 games under `first` (724 farm deaths in 40 seeds),
while the driver keeps shipping twenty tonnes a count of water it does not
have. Both minds die in act 1 in every game. A third of the outpost has
cataracts at the Silence. Population at the trail-off is 64-66 (p90 77-80)
against canon's 110-240.

Monte-Carlo (200 games): random 166 counts (p10 134, p50 164, p90 197), pop
65; ring 168 (144/164/197), pop 66; first 193 (164/197/224), pop 64. Canon
§4 wants 120-156.

## 1. The first review's findings

**Fixed.**

- Arrivals get ties (`turn.rs:883-900`): 25 % density, both directions. Both
  end-state saves have no present person without a tie; mean out-degree
  17-22. Arrivals lack the founders' dominance-clash term
  (`setup.rs:287-290`), which is why the hindrance graph never seeds (see §2).
- Musts fire highest-priority first (`director.rs:76-86`).
- Authored advice is rendered through the casting (`director.rs:117`) and the
  For/Against split is right (`ring.rs:256-298`). Leftovers: `*i !=
  usize::MAX` at `ring.rs:262` is a no-op; the CLI prints counsel without the
  lexicon (`main.rs:178-183`) while it renders storylet text with it
  (`main.rs:161`).
- `resolve_by_id` (`lib.rs:105-113`) exists and `play` accepts an id
  (`main.rs:228`). But `FiringRecord` (`state.rs:213-218`) still records no
  option, so there is no `chose` condition and a chronicle cannot be
  re-derived from a save; `montecarlo`/`run` still resolve by index.
- Strain has noise (`turn.rs:398`); grievance relaxes (`turn.rs:911`);
  displacement keys on `attention > 0.15 && stage < SkippedRotation`
  (`turn.rs:348-349`) rather than 0.3.
- Cast rotation: `Anyone` (`storylet.rs:772-786`) and now `Skill`
  (`storylet.rs:750-771`) weight against `recent_cast`. Chronicles show 38-44
  distinct names; the top name (9-11 lines) is always a seat holder, since
  seats still pin to max skill with lowest id (`ring.rs:196-206`). Good
  enough for act 1.
- PV ships with every arrival (`turn.rs:810`). This ends the aphelion
  starvation of review 1, and overshoots: 2,000-2,300 kW against 1,090 demand
  under `first`; the reactor's death at count 150 is now a non-event and
  power neither binds nor matters. PV should degrade (~2 %/yr) and arrivals
  should bring `person_kw`'s worth, not 190 m² each.

**Half-fixed.**

- *Labour floor.* `subsistence_hours` (`turn.rs:137-140`) is
  `n·228·(n/150)^−0.35`, which equals capacity at 150 by construction; the
  test at `engine.rs:180-189` checks that identity, not the model. Through
  `labour()` (`turn.rs:94-131`), which adds 600 h extraction and a closure
  gap, the no-robot crossing is 157 and the sponsor-fleet crossing (4,700
  robot-hours less 858 maintenance) is 104 at closure 1, 111 at 0.9. Both in
  band. But robots are uncapped flat hours: 4 people plus 1,000 dex gives
  capacity 200,900 against demand 36,800; canon's 53 % human premium is
  absent, so is the skill shadow, and adults are 16·12 here (`turn.rs:97`)
  versus 18·12 everywhere else. With 48-87 people the deficit is +19-35 %
  from count 1 and +50-100 % after count 150 (`labour_factor`,
  `turn.rs:142-147`, then throttles mining — that is the famine chain).
- *Skiff rotation* (`turn.rs:477-531`): target 15 % of adults (484), 24-count
  tours (490), re-eligible 24 counts after grounding (505), candidates sorted
  by skill then dose (509-518). Canon (`canon.md:130`): "a two-year career,
  then grounded". Probe, seed 7: 42 people ever aboard of a 121 roster, 24
  did two or more tours, 20 spent more than 48 counts aboard, the top
  individual 97 counts over five tours (~3.2 Sv). It is a rota, and it
  re-picks the most skilled, who are also the seat holders, so the ring's
  extraction and navigation chairs are held by the most irradiated people in
  the polity. Fix: `Person.skiff_counts: u32`; eligible only while
  `skiff_counts < 24`; sort by (never aboard, skill); a `grounded` cast
  filter; count arrivals' Skiff estate toward it.
- *Cataracts* (`turn.rs:360-364`): onset is a roll now, but still Sv against a
  0.5 Gy threshold; at 2 %/count per 0.5 Sv excess, 21-29 of 61-87 present
  (28-33 %) have cataracts at the Silence. GCR quality factor 2-3 puts the
  threshold at ~1.2 Sv. Mean dose at the Silence is 0.63-0.92 Sv, which *is*
  what canon's rates and a 15 % skiff share give (0.15·0.33 + 0.85·0.012 =
  0.06 Sv/yr × 13-15 yr); the distribution is what is wrong: p50 0.31, p90
  2.5, max 3.3 Sv, founders 1.4 versus arrivals 0.5.
- *Licence* (`turn.rs:218-269`): the heartbeat is still `attention > 0.15`,
  i.e. the calendar; conjunction still does not interrupt it, so the grace
  period is only ever lived at the end. New: a `minds_idled` flag keeps the
  heartbeat alive unconditionally (218), which lets one storylet make the
  machine unreachable — `unlicensed_refusal` never fired in 200 ring games on
  the current build. *The Unforgetting* (`turn.rs:302-314`) still requires
  `licence != Compliant`, so when the stage reaches UpdatesStopped and
  `reset_due` (271-273) goes false, nothing is said: seed 21 stopped
  resetting at count 72 and printed the Unforgetting at 168. `canon.md:161`
  says year zero is the first count the reset did not come. Fire it on the
  first `turn % 12 == 0 && !reset_due`, whatever the licence. Embodiment on
  reset halves (282) rather than collapses; there are no crew-to-mind ties
  to clear.
- *Manifest* (`turn.rs:733-744`): flags shape the convoy, but
  `sponsor.capability_share` is now written *by* the convoy from the flag
  (744) — a record, not a lever; content writing it does nothing. No lag: the
  manifest storylet fires ≤ 4 counts before a window and that same window
  answers it; canon says 30+ counts.
- *Sale* (`turn.rs:680-702`) resets `phi_expected`, confidence 0.3,
  suspicion; not attention, `next_review`, or the `manifest_*`/`lay_*`
  flags. Stage steps are still uncapped (664): seed 302 went 3 → 5 in one
  review.
- *`leave` after the people-ship stops* (`turn.rs:770`): honoured only when a
  convoy arrives; after NoShip nothing arrives, so in act 2 it still does
  nothing.
- *Coherence* is now 1 − share with an enemy (`turn.rs:580-586`). It reads
  0.93-1.00 all game. The hindrance graph is empty: growth (444) at typical
  values (strain 0.4, clash 0.25, escalation 0.5, outward 0.2) is ~0.001 per
  count against a decay of 0.01 (445); the end saves have mean hindrance
  0.005-0.009, max 0.25-0.38, and work_positive 0.81. Enemies: 0-5 of 65.
  `ring_resented` (`coherence < 0.9`) fires only in the first twelve counts;
  `two_dominants` (`conflict_concentration > 0.12`) fires 0.01-0.02 per game
  and its `arbitrate` option was never chosen in 600 games. Conflict
  concentration sits at 0.05-0.16, which is 2/n for a uniform graph. The
  fossil has been replaced with a graph of friends.
- *Mind attrition* (`turn.rs:277-289`): now linear in `units_at_start` with a
  lump — expected loss 0.69 % of start per count, death (< 20 %, line 289) at
  count ~116-155 in every seed. `mind_death` fires 1.00 per game under every
  policy; both minds are dark before the Silence. Canon §4.18 has 8 %/yr
  attrition with tier drops and the imported minds as a dying elder
  generation in acts 2-3; exponential 8 %/yr reaches 20 % at 19 years. The
  fix overshot from four times too slow to certain death, and takes
  `hundredth_count` (needs embodiment ≥ 0.5 at turn ≥ 100: 0.04-0.09 per
  game, never under `first`) with it.

**Remaining.** Confidence and attention as countdowns (§2). No `p_miss` by
stage: 23 of 40 first missed windows happen at Enthusiasm (`turn.rs:715-718`:
p = (0.5 + 0.5·conf)·min(att/0.3, 1)), and `first_silence` (722) enters the
lexicon at stage 0. The homeward edge is still unused. No third-quarter
predicate: `20-people-third-quarter.toml` casts any rotator; the true
third-quarter share is a wave (0 → 25-40 % → 0 every ~30 counts) because
contracts are cohort-synchronised. Austerity does not flip rotators: `let
austerity = 0.0;` at `turn.rs:389` is a dead variable. Convoys still ship only
dex and arm (`turn.rs:755-756`); no haul, no repair tier, no skill shadow.
Phantom shipping (`turn.rs:158-167`): `shipped = min(40, shippable + 20)` and
`water_share = min(shipped/2, shippable)`, so an empty tank ships 20 t/count;
seed 7's φ rose 6.6 → 7.6 over forty counts with water at 0. Boron and helium
are never consumed; medicine reaches < 2 by count 30 (`turn.rs:196`) and
nothing in the engine reads it. `strains.iter().find` is still O(n·E)
(`turn.rs:437-440`). `counts_to_window` still returns 60 past the calendar
(`state.rs:556-560`). Magic numbers and unread params as before; no
params/content hash in saves.

## 2. New problems the fixes introduced

**The famine chain.** Under `first`, 724 deaths in 40 seeds (18 per game), 39
of 40 games with a death, 40 of 40 with water at 0. Seed 302: 21 dead, all
"The farm could not carry everyone" (`turn.rs:927-943`), none by storylet —
the only lethal option, `no_evacuation/wait_for_ship`, is fourth in its list,
and `first` never reaches it. The chain: spares hit 0 by count ~100 (0.12 per
person-count is 125 per window at n = 65; a throughput manifest delivers ~79)
→ `spares <= 0` selects the *dead* attrition table (`turn.rs:205`) while the
sponsor is alive → haul falls 8 → 1.1-2.3 by count 150 and the haul factor
clamps at 0.3 (152) → mining is 14·pr·lf·0.3 ≈ 2-3 t/count against
consumption n·0.9·(1 − closure) = 7-12 t as closure decays without spares
(184) → water 0 at count 108-144 → farm fails (197-200) → the most-strained
person dies every third count (928-931). Meanwhile φ climbs on phantom water.
`first` is worst because its throughput manifests add +0.04 confidence each
(`10-sponsor-manifest.toml:59`), so the sponsor lives to count 197 while the
outpost starves. It is an engine problem: the water/haul/spares model makes
any population above ~40 unsustainable once the fleet decays, the reverse of
canon's floor at 150. Fixes, in order: `shipped ≤ shippable` and consumption
met from mining before the driver takes anything; ship haul (thr_t/25) and let
arm rebuild haul at T2; mining scales with people and labour surplus, not only
down; `dead` means `stage >= NoShip` — no spares already raises maintenance
hours (114-118), it should not also double attrition; famine cuts margin,
then strain, and kills only after six counts at zero.

**Sponsor pacing is two countdowns.** `phi_expected` starts at 6 and rises
8 %/review to 12 (`turn.rs:625`). At review 1 (count 16) φ ≤ 16·40/320 = 2; at
the observed 25 t/count average φ is 4.7 by review 9 when the expectation is
11. Across 21 reviews in two probed seeds φ beat expectation zero times, so
`delta` (621) is −0.5 to −1 every review, confidence falls 0.07-0.09 per
review and is 0.00 by count 100-115 in every trace. After that the composite
is `0.4·attention + 0.2` (runway_health stays 1.0 until count 180) against
thresholds 0.55/0.45/0.35/0.25, so the stages are attention crossing 0.875,
0.625, 0.375, 0.125 on a −0.06/review slope with 0.85 drag after austerity:
stage 2 at count ~64-114, 3 at ~130-150, 4 at ~165-180, Silence 24 counts
after the last convoy. That is why act 1 is 164-197 counts and why policy
barely moves it. Fix: seed `phi_expected` from the achievable —
`driver_t_per_count·counts / received_t` scaled by 0.6 — and raise it only
when throughput hardware is shipped; judge the *trend*; give attention inputs
(+0.05 on a beaten target, +0.1 on a reported death, −0.1 per conjunction not
re-acquired); `p_miss` by stage; cap the stage step at +1 per review.

**Skiff rotation × dose × cataracts** — see §1. The mean is consistent with
canon; the rota concentrates 2.5-3.3 Sv on the twenty most useful people and
the Sv/Gy conflation then blinds a third of the outpost.

**Baseline spares** (`turn.rs:749-751`): `n·0.12·16·0.8·(0.5 + 0.5·conf)`. At
n = 65 and conf 0 that is 50 per window against 125 consumed; a throughput
manifest adds 19, capability 134. The choice matters — the baseline is a
floor, not a substitute. But scaling it by confidence halves it at exactly the
point (count ~100) when the sponsor stops answering capability requests; make
it stage-scaled (1.0/1.0/0.8/0.5/0.3/0/0).

**Coherence and conflict concentration do not move** — see §1. Fix the
constants first: growth ×10 (or decay ÷10), seed hindrance for arrivals with
the founders' clash term, add a per-count chance that one strained pair's
hindrance jumps by 0.3, and let `work_positive` fall when hindrance rises.
Then make coherence a modularity on the positive graph.

**Population** peaks at 70-87 under `first` and 64-66 at the Silence under
every policy; expansion (`turn.rs:794-795`) is `12·confidence`, and
confidence is 0 by count 110. The population test asserts ≥ 30.

**`minds_idled`** (`turn.rs:218`) is a content flag that switches the licence
machine off. Storylets should write `minds.licence`-adjacent state through a
typed effect, not a magic flag the engine reads.

## 3. Determinism and save integrity

- `estate_since` has `#[serde(default)]` (`person.rs:153-154`). An old save
  loads founders correctly (0) and arrivals wrongly (0 = "never rotated",
  immediately eligible). Unavoidable, but the save cannot tell, because
  `Game` still carries no `params_hash`, `content_hash`, or calendar hash.
  `Engine.calendar` is cloned into every save (720×4 f64: 680-870 KB) and
  loading under a changed `params.toml` silently diverges. Not applied from
  review 1.
- Nothing else is missing: every field `turn.rs` and `director.rs` read
  (`recent_cast`, `flags`, `counters`, `fired`, `lexicon_triggers`,
  `window_started`) is serialised. What is missing for *meaning*:
  `FiringRecord` has no option id; the CLI policy RNG (`main.rs:344`, `seed ^
  0xC0FFEE`, sequential) is outside the save, so `run --load` cannot
  reproduce a Monte-Carlo game past the load point. The famine death now sets
  `first_death` (`turn.rs:937`) but not the `deaths` counter content uses.
- RNG: `rng_for` (`setup.rs:315-319`) still streams `turn·16 + {0, 1}`;
  `advance` builds stream 0 before `turn += 1` (`lib.rs:81`, `turn.rs:28`),
  so count N's physics uses stream (N−1)·16 and its director N·16+1 — per
  (seed, turn), asymmetric, undocumented, deterministic. The new consumers
  since review 1 — skiff rolls (`turn.rs:523-529`), cataract roll (362),
  strain noise (398), hindrance lumps (448), `weighted_pick` on stream 1 —
  all draw in roster or `BTreeMap` order, so replay holds; the resume test
  passes (13/13, 0.7 s). Note that `rotate_skiffs` draws a variable number of
  rolls until the target is met, so the physics stream's consumption depends
  on state; harmless for replay, fatal for any future mid-turn checkpoint.
- Saves are JSON with shortest-round-trip floats; no NaN can enter within
  720 counts. Past 720, `counts_to_window` returns 60 and `home_r_au` falls
  back to 2.44 (`turn.rs:50-51`) silently.

## 4. Three engine features act 2 needs

**Hulls as entities with a location and a transfer in progress.**

```rust
pub struct HullId(pub u32);
pub enum Place {
    At(BodyId),
    Transit { from: BodyId, to: BodyId, depart: u32, arrive: u32, dv_kms: f64, sail: bool },
}
pub struct Hull {
    id: HullId, name: Option<String>, class: HullClass,   // Coil | Skiff | Tug | Sail
    mind: Option<usize>,                                   // index into game.minds
    place: Place, crew: BTreeSet<PersonId>,
    propellant_t: f64, water_t: f64, cargo: Stocks,
    counts_since_refit: u32, reach_kms: f64, overdue_since: Option<u32>,
}
```

`Person.aboard: Option<HullId>` replaces `Estate::Skiff/Bore` (estate becomes
derived: Kept if `None`, else the hull's class). `Game.hulls: Vec<Hull>`;
`Calendar` grows to `edges: BTreeMap<(BodyId, BodyId), EdgeCalendar>` computed
lazily from `orbit::transfer::best_at` and hashed into the save.
`Engine::plan(hull, to, depart, slider)` returns `(dv_kms, counts)` along a
Lambert sweep; the two currencies are `Propulsion::Steam { isp_s: 300,
tank_kms: 3.0..5.0 }` and `Propulsion::Sail { kms_per_year: 0.2..0.6,
plane_change: false }`. Effects `hull_depart`, `hull_recall`; qualities
`hulls.away`, `hulls.overdue`, `reach.kms`; cast filter `aboard = "role"`.
Dose comes from `Place` (Transit unshielded 550). Overdue after N windows sets
`overdue_since` and freezes the crew's shares (§4.24). Reach decays when
`counts_since_refit` passes a threshold. Ties should become `Ties<Entity>`
with `Entity::Person | Entity::Mind` so mind death and resets can clear them.

**Machines / Children / Sky seats on triggers.** `Seat::exists`
(`ring.rs:110-118`) already gates Machines on a flag and Children on a count.
Data: `Game.seats: BTreeMap<Seat, SeatState { opened: Option<u32>, appointed:
Option<PersonId> }>` so a seat can be appointed (seed 7's "deputy") rather
than always max-skill; `seats()` prefers `appointed` if present and
qualified. Triggers in the engine, not content: Machines opens on the
Unforgetting or the first mind death; Children on the first birth; Sky (who
answers for the outside — Navigation skill, biases for `risk` +0.3, `sponsor`
−0.5, `independence` +0.8) on the first Voiding or the first overdue hull.
Effects `open_seat = "sky"`, `appoint = ["seat", "role"]`. Counsel weights
authority by `turn − opened`: a seat younger than 12 counts doubles its
wrong-answer chance.

**Births and children with the hidden partial-g curve.**

```rust
pub struct Hidden {                       // drawn in new_game, serialised, never printed
    partial_g_threshold: f64,             // U(0.2, 0.9)
    radiobiology_mult: f64, plasma_exponent: f64,
    trailoff_cause: Cause, mars_condition: Mars, breakthroughs: BTreeSet<String>,
}
pub struct Polity { conception: bool, gestation_aboard: bool, rearing_aboard: bool,
                    families_allowed: bool, spin_g: f64 }
// on Person:
parents: Option<(PersonId, PersonId)>, gestated_g: f64, reared_g: f64, bone: f64,
```

Birth: for each pair with `work_positive > 0.6` both ways, both adult, the
mother's dose under `polity` thresholds, p = 0.02/12 per count while
`families_allowed`; the child is `Birthplace::Belt`, `born = turn`, no
skills, excluded from labour under 16·12 and seats under 18·12 (already
gated). `bone` is `(reared_g >= hidden.partial_g_threshold) as f64 + noise`,
and `people.bone_ledger` surfaces only as the mean over a cohort of ≥ 5 aged
≥ 20 years — the 20-25-year latency of §4.21. `spin_g = 0` on "stop and
weld" makes every later Keep-born draw at 0 g and closes the question.
`Quality::Children > 0` then opens the Children seat by itself.
`Sponsor.families_allowed` moves into `Polity`.

## 5. Tests

In priority order; the state today in brackets.

1. Water conservation: per count `shipped ≤ shippable`, and over a game
   Σ shipped ≤ start + Σ mined − Σ consumed. [Fails: phantom 20 t.]
2. Labour through `labour()`, not `subsistence_hours`: a `Game` with n adults
   at strain 0, closure 1, no robots has |deficit| < 5 % at n ∈ [150, 160];
   with the sponsor fleet at n ∈ [105, 120]; with 4 people and 1,000 dex the
   deficit is still > 0. [Third fails: no human premium.]
3. Skiff career: over 200 counts no person exceeds 26 counts aboard, and no
   one aboard at count 100 was aboard at count 60. [Fails.]
4. Dose: Kept-only for 15 years gives a cataract share < 5 %; one 24-count
   tour < 20 %. [Fails.]
5. The Unforgetting fires on the first multiple of 12 with no reset, under
   any licence state, and never while resets run. [Fails.]
6. Sponsor: over 100 seeds φ beats expectation at ≥ 20 % of reviews;
   confidence at count 100 spans ≥ 0.3 between p10 and p90. [Fails: 0 and
   0.00-0.07.]
7. The first missed window is at stage ≥ 2 in ≥ 90 % of seeds. [Fails: 23 of
   40 at stage 0.]
8. Hindrance alive: at count 120, ≥ 5 % of present persons have an enemy and
   `conflict_concentration ≥ 0.2` in half the seeds. [Fails.]
9. Minds: with no storylets, both minds alive at count 156 in ≥ 80 % of
   seeds; embodiment ≥ 0.5 reached in ≥ 50 %. [Fails: 0 %.]
10. Monte-Carlo acceptance (review 1 #7, tightened): Silence p10-p90 within
    [120, 170] under `ring`; population p10 ≥ 100; farm deaths p90 ≤ 3 under
    every policy. [Fails on all three.]
11. Brute force every storylet × option under four fixtures (count 1, 60,
    150, post-Silence): no panic, no stray `{`, every non-optional role cast.
    [Missing.]
12. Save with a mutated `params.toml` is refused. [Needs the hash field.]
13. `resolve_by_id` with an unavailable id returns `None` and leaves `Game`
    equal to its clone.

Weak existing tests: `no_robot_labour_floor_is_about_150`
(`engine.rs:180-189`) asserts the formula's identity at its own anchor and
cannot fail for any exponent; `population_at_the_silence_is_a_people_not_a_station`
(191-206) asserts ≥ 30 under a name that promises canon's 110 — raise it or
rename it; `every_game_reaches_an_ending` only bounds 400;
`save_and_resume_replays_identically` resolves option 0 on both sides, so it
never exercises the weighted casts on divergent branches — add a ring-vote
choice; `quality_reads_do_not_panic_at_any_count` runs one seed.

What to do first: make `shipped ≤ shippable` and stop the dead-rate attrition
while the sponsor lives; ship haul; seed `phi_expected` from the achievable;
make the mind attrition exponential; cap skiff careers at 24 counts; fire the
Unforgetting on the first skipped reset. Then run tests 1, 6, 9, 10 and tune
until they pass.
