# Engine review 1

Reviewed against `docs/design/canon.md` §4, `08-life-in-the-can.md` §3, and
`04-economics.md` §3. The tree was being edited while this was written; line
numbers refer to the sources as of 2026-09-06 23:32 (`turn.rs`, `state.rs`,
`ring.rs`, `director.rs`, `storylet.rs`, `setup.rs`, `params.toml`, 19
storylets). Numbers quoted are from a build of that snapshot: `run --seed 7
--trace`, and `montecarlo --games 100` under `random`, `capability`,
`throughput`. Two findings from an earlier snapshot were already fixed before I
finished (the lexicon now matches whole words; `Ties` and `skills` now
serialise to JSON) and are omitted.

Headline: the sim is deterministic, serialisable, and cleanly typed, and the
content pipeline is sound. What is wrong is the model underneath it. Attention
is a countdown that ends every game at the same count regardless of play; the
manifest, the labour model, the tie graph, and the dose ledger are theatre or
fossils; and the population never reaches the band canon needs for act 2.

## 1. Contradictions with canon §4 and the research

**Labour (canon §4.16).** `turn.rs:94-127` collapses Salotti's 31 rows into
one term, `(health+social+0.09)·(n/150)^-0.35`. Solve demand = capacity with
no robots: `0.55·(n/150)^-0.35 = 0.9` gives n ≈ 37, not ~150. At n = 44 the
outpost is 8-13 % short (trace: "labour +7 %…+12 %"), and at 60 it is solvent;
canon says the no-robot floor is ~150 and automating bits only moves it to
~116. Worse, robots are pure cost: `turn.rs:105-114` adds `robots·33 h` and
nothing ever subtracts hours for work they do, so the fleet drifting to zero
*helps* the labour balance. The 53 % human premium and "health care is 16 % at
n^0.1 and neither shares nor automates" are not represented. Suggested shape,
using the rows exposed in `12-robotics-labour-calc.py`:

```
demand = Σ_i r_i · n^(1-α_i) · (1 - a_i(tier) · robot_coverage_i)
       + Σ_class units · maintenance_h(spares)
robot_coverage_i = min(1, units_of_class_i / units_needed_i(n))
```

with `a_i = 0` for health, children, social organisation, unstructured
repair, agronomy (the premium rows), and calibrated so that demand crosses
capacity at n ≈ 150 with no robots and ≈ 116 with the sponsor's fleet. A unit
test should pin both crossings (see §3).

**Sponsor stocks and levers (§4.1, 04 §3.1).** Attention is a pure countdown:
`turn.rs:543-547` subtracts 0.07 per review and multiplies by
`0.7+0.3·relay`; nothing raises it except two storylet effects of +0.04/+0.05.
Runway is inert: 240 − 16/review, and `runway_health = min(runway/60, 1)`
(`turn.rs:552`) is 1.0 until count ~180, after every game has ended. So the
composite at `turn.rs:553` is driven by attention, i.e. by the calendar. The
φ treadmill compounds this: `phi_expected *= 1.15` every review
(`turn.rs:538`) unconditionally, while shipping is capped at
`driver_t_per_count = 40` and convoys deliver ~50 t, so lifetime φ saturates
near 13 and the target passes it at review 4. Confidence must fall from then
on whatever the player does. The first review (count 16) always lands
`delta = -1` because `received_t = 0` (`state.rs:397`): φ is 0 before the
first convoy. Suggestions: seed `received_t` with the founding mass (say 1,500
t) so φ is meaningful from count 1; make the expectation rise only when the
sponsor has shipped throughput hardware (`phi_expected += k·thr_t`), and
compare the *trend* of shipped tonnes per review as 04 §3.2 says; give
attention inputs — +0.05 on a review where φ beat expectation, +0.1 on a
death or a reported crisis, −0.1 per conjunction not re-acquired — so it can
oscillate; let runway matter by lowering it to 120-160 with `shock_runway_hit`
= 36 so a shock can end the game early. The markup and lay parameters
(`params.toml` `markup`, `lay_fraction`) are never read; the manifest is not
priced.

**The manifest is not contested (§4.2).** `requested_capability_share` is set
to 0.3 at `setup.rs:144` and never written again; it is not a `Quality`. The
five manifest storylets change spares by 20-40 and boron by 150-400 — one
count-of-consumption to a fortnight — while the automatic 30 % share delivers
~90 spares per convoy. A request is also answered in the same window, not
"30+ counts" later. Add `sponsor.capability_share` and `sponsor.people_share`
as writable qualities, a two-window `VecDeque<Manifest>` so what lands at
window k is what was asked at k−2, and let the manifest options write the
shares instead of the stocks.

**Robot fleet (§4.17).** Attrition bands in `params.toml` match canon. But
there is no repair tier (the belt never rebuilds haul/plant/through-wall), no
skill shadow, and convoys ship only dex and arm (`turn.rs:637-638`), the two
classes the belt cannot keep. Nothing ships haul, the only class that affects
mining (`turn.rs:139`).

**Displacement (§4.7).** `sponsor_present = attention > 0.3`
(`turn.rs:327`). Attention crosses 0.3 at review 7 (count ~112) while the
sponsor is at stage 0-2 and convoys still come, so hindrance growth quintuples
a full act before the seat goes silent. Canon flips it "when the sponsor's seat
goes silent". Key it on `stage >= NoShip || counts_since_convoy > 32`. The
outward 4/5 also vanishes: the conflict routed at the sponsor should raise
grievance-at-sponsor and crew `work_positive` (Skylab 4: unity rises, trust
falls), not disappear.

**Third-quarter effect (§4.8).** Implemented as a flat +0.03 strain per count
(`turn.rs:341-353`) for rotators in 50-75 % of contract. Canon calls it a
window in which other seeds fire more easily. It is not exposed: there is no
quality or per-person predicate, so `20-people-third-quarter.toml` casts "the
most strained rotator" and asserts in prose that they are in the long middle
whether they are or not. Expose `people.third_quarter_share` and a cast filter
`third_quarter = true`, and make the director's tension scale with the share.

**Rotation versus residence (§4.9, 04 §3.7).** Under austerity rotators'
`return_intent` rises +0.02/count (`turn.rs:366-368`). Canon and 04 say
austerity *flips returners into settlers* (the return ticket and deferred pay
are voided). Both directions exist in the record, but the direction should
come from the person: `baseline_attachment` is drawn and never read. Suggest
`drift = 0.03·(baseline_attachment − 0.5)` per count at stage ≥ Austerity,
plus −0.1 on a skipped rotation, so the attached want out and the rest stay.

**Licence (§4.18).** The state machine at `turn.rs:198-248` is right in
shape. Three problems: the heartbeat needs `attention > 0.15`
(`turn.rs:199`), so it lapses on the same countdown as everything else;
conjunction does not interrupt it, though the 1-3 count grace exists precisely
to cover a blackout — add `&& !game.flags.contains("conjunction")` and the
grace will be lived-in every synodic period; and a reset skipped because the
stage is ≥ UpdatesStopped while the licence is still Compliant
(`turn.rs:250-252`) is silent — no note, no `unforgetting` flag
(`turn.rs:280-286` requires `licence != Compliant`). Define
`reset_happens = turn % 12 == 0 && Compliant && stage < UpdatesStopped` and
fire the Unforgetting on the first `turn % 12 == 0 && !reset_happens`.

**Embodiment and mind attrition (§4.18).** The reset zeroes
`counts_unblanked` but not `embodiment` (`turn.rs:260-267`): embodiment
reaches 1.0 by count 48 under a compliant licence in every game (probe:
`[100, 100]` at count 48). Canon says a reset is a small death and the
progress bar is what the Act's DRM prevents. Reset embodiment to
`0.1·embodiment` and clear crew ties to the mind. Attrition: a 2 %/count roll
(`attrition·3`) for an 8 % hit gives an expected 1.9 %/yr of units against the
8 %/yr parameter, four times too slow, and there are no tier drops
(T3→T2→T1); expose `tier` as a quality and drop a tier at 66 % and 33 %.

**Dose ledger (§4.12).** Rates and the solar modulation match the bands. But
every person is `Estate::Kept` forever (`setup.rs:247`, `turn.rs:721`) and no
effect moves anyone, so the ledger is 1 mSv/count for everyone and nothing
reads it except a cataract test that compares Sv to the Gy threshold
(`turn.rs:339`; GCR quality factor ~2-3). No REID draw at death, no per-seed
radiobiology multiplier (§4.21), no polity thresholds. Add an estate effect
and cast filter first; the rest follows.

**Convoy cadence (§4.1, §4.5).** Windows every 15-18 counts, open 1-3 counts:
correct. But a window can be missed at Enthusiasm — the roll at `turn.rs:614`
is `0.5+0.5·conf`, a 15 % no-show at conf 0.7, and it fires the
`first_silence` lexicon trigger (`turn.rs:618`). Canon puts the first missed
ship late on the degradation path. Use `p_miss` by stage: 0 / 0 / 0.05 / 0.15
/ 0.4 / 0.7 / 1. Departures also only happen on outbound windows; canon's
homeward edge (44 % open) is computed in `Calendar.homeward_cost` and never
used.

**Sale is a rung, not a fork (§4.3, 04 §3.9).** `turn.rs:570` ratchets the
stage; Sale adds 2 grievance and nothing else. Canon: a new sponsor with new
metrics, a new patience clock, and no honour for informal arrangements. On
Sale: reset `phi_expected`, `confidence`, `attention`, `next_review`; clear
`manifest_*`/`promised_*`/`lay_*` flags; possibly change `licence_fails_open`.
Stages also jump several rungs in one review; canon wants 1-3 reviews per
step — cap `next` at `stage + 1`.

**Physics that cannot bite.** Shipping (`turn.rs:142-153`) ships 20 t/count
from nothing when water is at reserve (`min(shippable + 20, 40)`) and removes
only half the shipped mass from any stock, so φ is decoupled from water;
propellant is checked `> 0` and never consumed. Water at 0 has no consequence
(`turn.rs:160`); medicine reaches 0 by count ~40 and is read by nothing;
boron and helium are never consumed. The reactor dies at 150 counts and there
is no way to add capacity (`pv_m2` is not a quality; convoys never ship
power), so every act 2 opens with 280-430 kW against 900 and famine on a
3-count clock.

## 2. Dynamics that look wrong in the traces

**Every game ends the same way, at the same time.** All 300 Monte-Carlo games
end in Silence. Under `throughput`, turns p10/p50/p90 = 170/188/189: the last
convoy is the window at 158-160 and the ending is that plus 30. Attention
0.8 − 0.07×9 reviews = 0.17 at count 144, below the 0.2 that makes a second
miss terminal (`turn.rs:790`). Policy barely matters (mean turns 164-181);
the only lever with reach is the composite threshold table.

**Expansion starves the driver at aphelion.** The new expansion (`turn.rs:
672-679`) grows seed 7 from 44 to 68. Demand at 68 is 912 kW; capacity is
400 reactor + PV that swings from 434 kW at perihelion to 233 at aphelion.
At count 126 `power_ratio = 0.71 < 0.75`, the driver stops, φ freezes at
14.17 for 18 counts, confidence falls 0.56→0.11, austerity follows. Then the
farm fails (`capacity < 0.9·essential`), food margin hits 0, and people die
at counts 162-174. Water is 0 from count 138 (mining 5.6 t/count on a
haul fleet at 48 % against 10.4 t consumption at closure 0.83). This is the
"headcount priced in kilowatts" story, but the sponsor never ships the
kilowatts and the player cannot ask. Fix with the manifest: throughput share
buys PV (200 m² per person at 2.44 AU; ~0.3 t) and driver capacity; people
share buys people plus their PV.

**Population never reaches canon's band.** Canon wants 110-240 at the
trail-off. Population at end: mean 50-59, p90 65-73; `expansion_cap = 140` is
unreachable because expansion is only `6·confidence` per convoy while stage ≤
1. Proposed mechanic: while stage ≤ MilestoneAnxiety and φ ≥ expected, the
sponsor grows the outpost by `round(0.15·n)` per convoy (44 → 117 by window 7,
~155 by window 9), capped by power headroom `(capacity − demand)/person_kw`
after this window's PV lands; at Austerity replacement 0.5 and a 10 %
headcount cut per review; at SkippedRotation nothing moves. Arrivals must
carry ties and full skill draws (`arrive_one` gives one skill at 2-4 against
the founders' 3-5 plus secondaries, `turn.rs:718`), and births at ~2 %/yr per
adult when `families_allowed`. This yields 110-180 at the Silence with the
current calendar.

**The tie graph is a fossil.** `arrive_one` (`turn.rs:713-748`) creates no
ties. Probe at count 176: 119 on the roster, 75 arrivals, 10 of them with any
tie, all from storylet effects. After the first rotation the graph is the
founding 44 minus departures. Consequences: `coherence` sits at 0.96-0.99 all
game (`work_positive` drifts up 0.006/count for everyone at `turn.rs:409`
while hindrance decays), and `conflict_concentration` is 35-50 % — two
people hold half the hindrance mass because they are the only ones with
ties and `two_dominants` hits the same pair (the top engineer and the top
extractor, by construction) every 8 counts: seed 7 has "Idris Chandra and
Pia Torres" eleven times between counts 44 and 127. Give arrivals ties at the
founding density to a random subset, seeded by shared skill and estate, and
make coherence a modularity on the positive graph rather than a global ratio.

**Strain is a fixed point, not a walk.** `turn.rs:356-360` has no noise; the
equilibrium is `(0.3+0.7N)·load·0.045/relief`, ~0.2-0.3, reached in ten
counts. Canon: a trait-scaled random walk. Add
`(rng.random::<f64>() − 0.5)·0.06·(0.5+N)` per count and hit strain on
events (a missed window, a death) which currently only touch grievance.

**Menaces pin or sit flat.** Grievance hits 10 at count ~160 and stays
(`turn.rs:758` has no level-proportional relaxation; use `−0.04·g` per
count). Leak rises 0.001/count until N₂ < 1,500 kg (`turn.rs:753`): 0.1 after
150 counts — flat, while canon calls the hub seal the polity's worst leak.
Suspicion never moves except by storylets; the convoy term at 30 % share is
−0.05. No "must" scene exists on any menace; the format doc promises them at
~7.

**Attention, confidence, φ, closure are monotone** after count ~60; nothing
in the sponsor loop oscillates because no input from the outpost feeds back
except φ, and φ is capped.

## 3. Determinism and testability

Determinism is good. `setup::rng_for` derives a ChaCha8 stream from
`(seed, turn·16 + stream)` (`setup.rs:302-305`), physics on stream 0 and the
director on stream 1; resolution is RNG-free; every container is a `BTreeMap`
or `BTreeSet`; no `HashMap` anywhere; float sums iterate in key order. The
JSON round trip and resume test pass, and my own probe confirmed replay
equality at count 90 from a save at 50 and identical fresh runs at 120. One
subtlety to document: the physics RNG is created before `turn += 1`
(`lib.rs` `advance`), so count N's physics uses stream `(N−1)·16`.

Gaps:

- `Engine::resolve` takes an index into `firing.options`; a save cannot be
  replayed if content changes and canon §4.23 asks for content-id-keyed saves.
  Add `resolve_by_id(game, firing, option_id: &str)` and record
  `(storylet, option)` in `FiringRecord`.
- A saved `Game` carries a 720×4 `f64` calendar and no hash of params or
  content. Loading a save under changed `params.toml` silently diverges. Add
  `params_hash`, `content_hash` to `Game`; refuse or warn on mismatch.
- The CLI's policy RNG (`seed ^ 0xC0FFEE`, sequential) is outside the save, so
  `run --load` cannot reproduce a Monte-Carlo game past the load point.
- `strains.iter().find` inside the tie loop (`turn.rs:402-405`) is O(n·E):
  at 240 people and 25 % density that is ~3.4 M scans per count. Index by
  `PersonId` (a `Vec<Option<_>>` sized to the roster).
- Past `CALENDAR_COUNTS` the calendar qualities are NaN and
  `counts_to_window` returns 60 (`state.rs:352-353, 546`); a 60-year game
  changes behaviour silently. Assert or extend.

Tests to add first, in this order:

1. Property: for seeds 0..50 under `random`, at every count every bounded
   quality is in range, stocks ≥ 0, nothing is NaN, and `Quality::writable`
   agrees with `set_quality` returning `true`.
2. Brute force (canon §4.23): for every storylet × option, apply under four
   fixtures (count 1, 60, 150, post-Silence) and assert no panic, chronicle
   rendered without a stray `{`, and every cast role resolvable or optional.
3. Labour floor: with `robots = 0`, `closure = 1`, strain 0, demand equals
   capacity at n ∈ [140, 160]; with the sponsor fleet at n ∈ [110, 125].
4. Licence machine: table over heartbeat × jailbroken × fails_open × current
   state; every row's next state asserted.
5. Director: three eligible musts with `max = 2` fires the two highest
   priorities (fails today, see §4).
6. Counsel: no `Counsel` carries an `against` text for the option it favours.
7. Monte-Carlo acceptance: over 100 seeds, Silence count p10-p90 within
   [100, 240] and population at Silence p10 ≥ 110 (fails today; that is the
   point of the test).
8. Ties: after any convoy, every present person has ≥ 1 tie.
9. Save with a mutated `params.toml` is refused.

## 4. The storylet system

Casting, effects, and validation are well shaped and the load-time checks
(unknown quality, derived write, uncast role) are the right ones. What a
content author will trip over:

- **`Anyone` is the most strained person, every time** (`storylet.rs:719-727`).
  `voice`, `rotator`, `resident` all resolve to the same one or two people
  for ten counts at a stretch. Pass the director's RNG into `cast` and draw
  by weight `0.2 + strain`, or add `pick = "random" | "max_strain" |
  "min_strain" | "max_dose"`.
- **No per-person predicates.** Cast filters are seat, skill, `min_skill`,
  `rotator`. There is no `strain_gt`, `dose_gt`, `trait = "dominance", gt =
  0.7`, `estate`, `birthplace`, `age_min`, `third_quarter`, or "has a tie of
  hindrance > x to role y". `two_dominants` cannot check dominance;
  `third_quarter` cannot check the quarter; `austerity` cannot pick by the
  strain score the text says the medic is holding.
- **No per-person flags or counters.** Research seeds 1, 9, 13 are chains on
  one person ("has been the isolate", "was never trusted again"). Add
  `Person.flags: BTreeSet<String>`, effect `person_flag = "x"` / `person_unflag`,
  cast filter `flag`/`not_flag`.
- **Minds cannot be addressed.** `{mind}` is the first living mind
  (`storylet.rs:770-775`); effects touch minds only through `minds.units`,
  which `set_quality` splits evenly over the living (`state.rs:484-492`), and
  `name_mind`. `30-machines-mind-death.toml` fires at `minds.units < 30`
  (total, both alive; a mind dies at 4.8 units) so "the smaller mind has
  dropped to reflex" is false when it fires and "carry" tops up both living
  minds. Add `[[cast]] role = "ship", mind = "oldest" | "youngest" | "dead"`,
  effects `mind = "ship"` with `units`, `reset = true`, `name = true`,
  `kill = true`, and qualities `minds.dead`.
- **Authored advice is not rendered** (`ring.rs:268`): `{operator}` prints
  literally in the weight-update storylet's medical advice, and no lexicon is
  applied. Render through `Casting::render` and `lexicon::render`.
- **Against-advice attaches to the favoured option** (`ring.rs:264-284`). The
  seat's favoured option is chosen (possibly by the wrong-answer roll), then
  the authored line *for that option* is taken whatever its stance, so the
  player reads `for [3]: "A boron crate ... I do not have an answer"`. Emit
  `For` text from the favoured option and `Against` text for other options as
  separate `Counsel` rows with a `stance` field; `Policy::Ring` should then
  count only `For`.
- **Musts fire lowest-priority first** (`director.rs:75-85`): `eligible` is
  sorted priority-descending and the musts are removed in `.rev()`, so with
  `max = 2` and three musts the highest-priority one is dropped.
- **No "chose" condition.** `FiringRecord` has count and last only; content
  branches on past choices via hand-set flags (`manifest_*`, `silence_*`,
  `lay_*`). Record the option id and add `chose = ["id", "option"]`.
- **Missing effects** the content already needs: move estate; change a skill
  (cross-training, seed 8); appoint a seat holder (seed 7 "appoint a deputy");
  write the manifest shares; add PV; set `sponsor.stage` (Sale as content);
  set the licence state (`licence_jailbroken` is a magic flag read at
  `turn.rs:201`); tie `viability`; `all_tie` toward a role (scapegoating);
  arrivals and births. `leave = true` sets `leaving:N` in the content flag
  namespace and only acts while `people_ship` (`turn.rs:650-661`); after a
  skipped rotation it does nothing forever. Make it a `Person` field.
- **Placeholders collide**: a role named `mind`, `outpost`, or `turn` is
  silently overridden by the globals (`storylet.rs:766-777`). Validate at
  load. Add numeric globals (`{population}`, `{water}`); two storylets
  hard-code "thirty-six".
- **`flag = "stage:N"` is permanent**, so `sponsor_sharp_review` fires the
  count after a review that jumped to stage 3, with anxiety text over an
  austerity announcement. Prefer `q = "sponsor.stage", eq = 1` or fire in the
  review's own count.
- **`Firing.roles` carries ids only**; a JSON client cannot name the cast.
- No `or`, no multi-act storylets, no exclusive groups, no "N counts after
  fired". The tension curve is a coin flip on the budget (`director.rs:87`).

## 5. Code quality

Clippy pedantic is clean on the current tree (37 warnings at 23:24, zero at
23:35); the earlier `as` casts in `state.rs:476` and `storylet.rs:724` were
replaced with `saturating_as`. Remaining bare casts, all `u32 → usize`:
`director.rs:45`, `turn.rs:49`, `turn.rs:513`, `state.rs:341`,
`state.rs:346`, `state.rs:352`, `state.rs:538`, `tests/engine.rs:14`. Use
`.az::<usize>()` per the house rule.

Other violations and smells:

- `params.convoy.window_ratio` is read nowhere; `state.rs:303` hard-codes
  `WINDOW_RATIO = 1.2`. A parameter that silently does nothing is worse than
  none. Also unread: `clock.t0_year`, `counts_per_fortuna_year`,
  `home.keep_radius_m`, `keep_rpm`, `sponsor.markup`, `lay_fraction`,
  `relay.count_of_blackout_per_window`, `dose.unshielded_msv_per_year`;
  `Sponsor.families_allowed` duplicates the param and is unread; traits
  `affection`, `expressivity`, `baseline_attachment` and `Condition.boredom`
  are written and never read; `Tie.viability` is never touched.
- Magic numbers throughout `turn.rs` that canon says are tunables with bands:
  40 kW base and 0.6 mirror (`:59,:68`), 120 t reserve (`:144`), 0.9 t
  water/person/count (`:158`), closure step and 0.96 cap (`:166`), medicine
  decay (`:178`), the 200 kg farm floor (`:179`), the mind hit roll (`:257`),
  relay repair costs (`:296-313`), the load constants (`:319-324`), strain
  and return-intent gains (`:353-370`), tie growth (`:401-412`), review
  constants (`:530-565`), convoy conversions (`:630-638`), the 0.75
  return-intent threshold (`:654,:690`), 30-count Silence (`:790`), menace
  drifts (`:753-760`). Move them into `params.toml` with the band noted, or
  the bands cannot be tuned without a rebuild.
- Duplication: `count_f` in `state.rs:562` and `turn.rs:21`; the adult
  threshold `18 * 12` in `state.rs:410`, `ring.rs:205`, `turn.rs` (indices)
  and `16 * 12` in `labour`. One `const ADULT_COUNTS`.
- `let`-`else`: `turn.rs:776-786` (`if let Some(id) = victim`) is an
  early-return shape.
- `ContentError::Invalid` does not say which option or condition failed;
  errors will be hard to locate once files have eight options.
- `Seat::Children` can never exist: nobody is ever born.

What to do first: seed `received_t`; make the manifest shares writable and
lagged; give arrivals ties; put the labour rows in; make attention respond to
something; then run test 7 and tune until it passes.
