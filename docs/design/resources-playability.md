# Resources for playability

The engine tracks about twenty-five quantities. The player should see seven, three
pressures, and a handful of clocks. This document proposes that model and the engine
changes it implies. Numbers refer to `data/params.toml` and `crates/sim/src/turn.rs`
as of the overnight build.

The reference points: King of Dragon Pass shows a treasury, cows, food, and a magic
pool and hides everything else inside advisors and scenes; Citizen Sleeper runs a
whole life on condition, energy, cryo, and chits, and turns *attention* into dice;
Frostpunk has two social meters and lets a resource at zero become a law rather than
a loss; Sunless Sea keeps its threats as menaces with named thresholds. Banished and
Surviving Mars are the cautionary cases: a screen of twenty bars, none of which tells
a story. Oxygen Not Included is the one game that made closed loops legible, by
showing flows as pipes rather than stocks.

The rule this document follows: **show what the player can act on, in the unit the
ring would use, and let the rest live in scenes.**

## 1. The player-facing resource set

Seven things. Each is a quantity the ring names in a sentence.

### Water — "the wet"

| | |
|---|---|
| aggregates | `stocks.water_t` |
| represents | the outpost's product and its life: the bound water in Fortuna's Ch clays, baked out at ~1 MJ/kg (research 01, 03); the sponsor ships it inward as propellant and shielding mass, the Keep drinks and farms it |
| unit, range | tonnes, 0–600; the reserve line at 120 t is drawn on the gauge |
| raises | the bake-out (~14 t/count at full power, hours, and haul strength); a convoy brings none |
| lowers | the loops' loss (`people × 0.9 t × (1 − closure)`); the throw ships half its tonnage as water above the reserve |
| the player does | sets the throw (see §5); accepts or declines the reserve when it is all that is left; sends the far-crater expedition in the second crisis |

Water is the only stock the player mines and the only one the sponsor cannot send.
That asymmetry is the whole act-1 economy in one gauge.

### Margin — "how long the loops carry us"

| | |
|---|---|
| aggregates | `stocks.food_margin_counts`, `stocks.nitrogen_kg / net leak`, and `closure` as a status word |
| represents | the closed-loop life-support of research 02 and 03: ISS-grade water recovery, the nitrogen that leaks through every seal and dynamic collar, the farm's buffer |
| unit, range | counts, 0–24: the smaller of the farm's stored margin and the air's time-to-empty at the current leak |
| raises | power to the farm, nitrogen recovered in the bake-out (4 kg per tonne of water), spares keeping closure near 0.96 |
| lowers | power shortfalls, the leak (45 kg/count, a third through the hub seal), closure decay when spares run out |
| the player does | keeps Spares flowing; chooses the loops in the hours ranking; in the leak scenes spends spares, slows the drum, or accepts the loss |

The closure fraction itself never appears as a number. It appears as one of three
words next to Margin — **tight**, **patched**, **failing** — the way Oxygen Not
Included shows a pipe as flowing or blocked. Nitrogen in kilograms appears only in the
Air seat's dialogue.

### Spares — "the crates"

| | |
|---|---|
| aggregates | `stocks.spares`; medicine, boron and helium are listed *in the store* as named items, not gauges |
| represents | the vitamin parts of research 03: the 5–10% of mass a settlement cannot make for generations, shipped at 150–300 kg per person-year; the thing that makes the sponsor matter |
| unit, range | crates (one crate = ten of the engine's units ≈ 100 kg); 0–40; the burn rate is shown beside it ("4 a window") |
| raises | every convoy's baseline, plus the manifest's capability share; cannibalising a robot class or the relay |
| lowers | 0.12 units per person per count; relay repairs; the leak scenes |
| the player does | writes the manifest; decides what to cannibalise; builds the belt-made substitute when the Wrights appear |

Spares are the cows. In King of Dragon Pass every problem could be solved with cows
and cows were the thing you never had enough of; here every physical crisis has a
"spend spares" option and the crate count is the wealth the ring argues over.

### Power — "the light"

| | |
|---|---|
| aggregates | `power.capacity_kw` against `power.demand_kw`; reactor life is a clock, not a stock |
| represents | the reactor's 10–15-year core that the belt can never refuel (U at 8 ppb), photovoltaics at 10–21% of Earth's flux, the energy seasons of an eccentric orbit (research 01, 03, 13) |
| unit, range | a margin, −40% to +60%, with the driver's 180 kW shown as the first thing to go |
| raises | PV shipped with each arrival, mirrors the ring chooses to build, the sun-turn (perihelion) |
| lowers | every new person (10 kW), the minds, the Lean (aphelion), the reactor's death |
| the player does | stops the throw in a shortfall (automatic, but the chronicle names it); builds mirrors in the capability manifest; chooses who goes cold in the winter rules |

### Hours — "the hands"

| | |
|---|---|
| aggregates | `labour_capacity_h` (people and robots) against `labour_demand_h` (Salotti's upkeep, robot maintenance, the closure gap, extraction) |
| represents | the labour floor of research 12: half of all hours at this size are social and medical and neither share nor automate; robots own the structured work and die without parts |
| unit, range | a margin, −40% to +20%, with a four-line breakdown on demand: upkeep, robots, loops, the bake-out |
| raises | arrivals, robots while they last, strain falling |
| lowers | strain, robot attrition (dex first), the closure gap when spares are gone |
| the player does | ranks the four sinks (§5); asks the manifest for people or for robots; grounds skiff crews or does not |

Robot classes are invisible. They appear as "robot hours" in the breakdown and as
scenes when a class dies ("the last dex").

### The Throw — "the number"

| | |
|---|---|
| aggregates | `throughput.phi` (shipped over received), `sponsor.phi_expected`, `sponsor.confidence`, `sponsor.attention` |
| represents | the one thing the sponsor reads (research 04: φ, tonnes shipped per tonne received, threshold ≳35 over life); the outpost as a strategic option, not a business |
| unit, range | φ, 0–10 in act 1, drawn against the sponsor's expectation line as a target on a small chart; the sponsor's mood is a face, not a number |
| raises | the driver running (needs 75% power, half the hours, propellant); water above the reserve |
| lowers | every tonne received; stopping the throw; the expectation ratcheting 40% a review |
| the player does | sets the throw; ships the reserve or takes the hit at a short review; asks for capability (and pays in suspicion) |

Confidence and attention never show as numbers. The sponsor's face and the review
countdown carry them: keen, sharp, distant, silent.

### People — "us"

| | |
|---|---|
| aggregates | present count, residents versus rotators, the dose ledger, the ring |
| represents | the persons-and-ties model of research 08 and the rotation-versus-residence clock of 07: rotating staff never become a people |
| unit, range | a count with two shares (residents, on contract), and the Bodies seat's ledger as a list of names with marks |
| raises | convoys while the sponsor is keen; someone choosing to stay |
| lowers | the ship home; deaths; skipped rotations turn rotators into residents (up, not down, in the way that matters) |
| the player does | everything in the people scenes; the families policy; who goes on the last ship |

Strain and coherence are not numbers on this panel. They are the faces of the ring
(King of Dragon Pass's portraits changed expression with mood) and a single word for
the outpost: **steady**, **strained**, **split**.

### What becomes invisible and what vanishes

Still simulated, surfaced only in scenes or as a word: closure (a word), nitrogen
(the Air seat), medicine (the Bodies seat, the formulary scenes), propellant (the
tugs run until they do not; a scene when they do not), boron and helium (named
items in the store, act-2 heirlooms), robot classes (the hours breakdown, the death
scenes), mind units (bright, dimming, failing), relay health (a status light on the
link clock), reactor life (a clock), the sponsor's runway (never; it is the thing
nobody on Earth tells you either).

Vanishes: boredom (fold into strain), the `viability` tie component (nothing reads
it), the counters map (content can use flags), reactor wear as a menace (it is a clock
with a scene at 36 counts left, not a pressure).

## 2. Pressures, not bars

Three pressures, each 0–10 in the engine and shown as a segmented ring with named
bands, Sunless Sea style. A pressure is not health; it is a countdown to a scene.

**Suspicion** — the sponsor's. Fed by capability manifests, cooked numbers, moved
spares, defied policy; drains slowly. Bands: 0–3 *unremarked*; 4–6 *questions in the
quarterly* (review scenes get a suspicion option); 7+ *the audit* (a must scene, once
per crossing); 10 *the sale is on the table*. Every scene that raises it says so in
its chronicle line, so the player learns the ledger by reading.

**Grievance** — the crew's. Fed by strain, the return share, skipped rotations, the
sale, deaths without rites; drained by the feast, the rites, a lay honoured, a
displacement target while the sponsor is present. Bands: 0–3 *the mess is loud*;
4–6 *the ring is resented* (the resentment scenes unlock); 7+ *the moot* (a must
scene: the crew puts a question to the ring, and the ring's answer costs a seat or a
policy); 10 *the outpost splits* (act 2's first fork, pulled early).

**The Leak** — the fabric's. Fed by the hub seal, low nitrogen, deferred repairs;
drained by spares and the drum's speed. Bands: 0–3 *the collar weeps*; 4–6 *the Air
seat's number is read aloud*; 7+ *the seal* (the must scene: spend, slow, or accept);
10 *Margin falls at double rate until the seal scene fires again*.

Presentation rules, from Frostpunk's hope and discontent: the band name is shown, not
the number; crossing a band upward produces one chronicle line naming what is coming;
the ring's faces reflect the highest pressure; every option in every scene that touches
a pressure shows a small arrow on the pressure it touches. No option should move a
pressure silently.

Strain and coherence stay out of this panel. They are the substrate the pressures are
computed from, and they show as the faces.

## 3. Zero is a scene, not a game over

Space is brutal; the game should not be, the first time. Every resource at zero
fires a must scene the first time, a costlier one the second, and only then the
ending path. Extinction should require the player to have read three warnings in the
chronicle and chosen through all of them.

| resource | first zero | second zero | third |
|---|---|---|---|
| **Water** | *Break the reserve*: the 120 t reserve becomes visible and spendable; the throw stops; the chronicle says the outpost is drinking its shielding. No deaths. | *The far crater*: an expedition on skiffs for surface ice — dose to the crew, hours for a window, a real chance of a wreck; or ration, which raises grievance and strain. | Rationing deaths at one per six counts, each a named line; the sponsor (if present) offers evacuation, which is the *Closed* ending and is written as a defeat, not a loss. |
| **Margin** | *The emergency stores*: the sponsor's sealed cache (every outpost has one; research 04's HBC posts did) — twelve counts of margin, once. | *Half light*: the farm on half power, hours to the loops first, everyone's strain up, the Throw stopped. | The famine chain as it exists in `turn.rs::endings`, slowed to one death per six counts, with an evacuation offer while the sponsor lives. |
| **Spares** | *Cannibalise*: a scene the first time the crates hit zero (not every six counts): a robot class, the relay, the second freezer, or the minds' spare boards. Closure holds one more window. | *The Wrights*: the belt-made substitute at crudeness — plain bearings, tubes, silicone — hours instead of crates; closure floor rises to 0.85 instead of 0.5. | Closure decays as now; Margin becomes the pressure; nothing dies of this alone. |
| **Power** | *Shut the throw*: automatic and named; the sponsor's number falls. | *Winter rules*: the ring sets who goes cold — the fab, the freezers, the minds (a mind idled is a small death), the farm never. | Margin falls; the reactor's death is a clock scene long before this, so it is never a surprise. |
| **Hours** | there is no zero; a deficit is a pressure. First crossing of −25%: *the roster*, a scene that offers the rankings of §5. | −40%: strain doubles; the bake-out stops; the throw stops. | — |
| **The Throw** | φ below expectation at a review is a scene, not a zero: cook it, ship the reserve, or take the hit. | Two reviews short: austerity, which the sponsor path already does. | The Silence, which is act 2, not a game over. |
| **People** | a death is always a scene (the rites) and never silent. | — | Under 20 people the outpost cannot hold the ring; the *Closed* ending offers itself; *Extinct* only from famine's third tier. |

The two endings stay. *Closed* (evacuated, or the outpost shut and the survivors
taken inward) is a chronicle ending with a last summary — the Greenland outcome —
and should be reachable and sad. *Extinct* should be nearly impossible in act 1
without ignoring three scenes.

## 4. Tone through mechanics

Frostpunk made coal feel like time because the coal gauge was a countdown to the
generator dying and the generator was the city. King of Dragon Pass made cows feel like
wealth because every solution cost cows and cows also walked off in raids. Our
equivalents:

- **The window is time.** The count-down to the next Earth window is the heartbeat:
  every manifest, every rotation, every review is phrased against it. It should be
  the largest clock on screen and the chronicle should say "two counts before the
  window" more often than it says a number. This carries research 01's monsoon
  finding and research 06's voyaging seasons.
- **Spares are wealth.** Crates, counted, spent, argued over, and the only thing the
  sponsor's love actually brings. This carries research 03's vitamin parts and 04's
  sponsor-as-supplier.
- **The Throw is the sponsor's love.** One number against a line that keeps rising.
  When it is above the line the sponsor's face is keen and the convoy is big; when it is
  below, the face is sharp and the manifest is cut. This is research 04 and the whole
  of canon §4.1–3 in one chart.
- **The ledger is the body.** Dose is names and marks, read aloud by the Bodies seat
  at the year's end (the "milky eyes" line already exists). Never a mean. This is
  research 10's dose policy and the pitches' tithe marks.
- **Margin is dread.** A single number of counts that falls when anything else fails.
  It is the This War of Mine meter: the one you glance at every turn.

What the player should never have to see: the Lambert cost (only *open*, *closed*,
*in N*), the closure fraction, robot classes, mind units, propellant, nitrogen in
kilograms, the sponsor's runway, strain as a number, tie weights. All of it stays in
the engine and comes out as sentences.

## 5. Direct controls versus scenes

Scenes carry the drama; a small recurring allocation carries agency between scenes.
King of Dragon Pass gave you exactly that: the yearly allocation of farmers, herders,
and warriors, and the treasury decisions, and everything else was a scene. Citizen
Sleeper's dice assignment is the same shape at a smaller scale. Frostpunk's law tree is
the counter-example: one-time choices, no recurring dial, and the game earns it by
making each law a scene.

Three standing controls, set at any count, applied continuously, changed by a click
and never nagged:

1. **The manifest** — one three-way split for the next convoy: throughput /
   capability / people. The engine already reads the flags; make it a control with
   the scene only when the ring disagrees with the split (a sharp review, a suspicion
   band crossing). The current per-window manifest storylet becomes the *exception*
   scene, not the routine.
2. **The throw** — three positions: *ship everything above the reserve*, *hold the
   reserve and ship the rest*, *stop*. This is the water-versus-number dial and the
   only direct lever on the sponsor's love.
3. **The roster** — a ranking of the four sinks for short hours: the loops, the
   bake-out, the farm, the robots. The ranking sets which suffers first in a
   deficit. The default is what the ring would choose; the player overrides it when a
   scene has made the trade-off clear.

Everything else stays in scenes: the families policy, the licence, what to
cannibalise, who goes home, the reserve, the rites, the lay, every pressure crossing.
The test for "scene or control": if the ring would argue about it every time, it is a
scene; if it is a standing preference the ring would set once and revisit rarely, it
is a control.

## 6. The act-1 arc in resource terms

Each phase has one dominant pressure and one resource the player is learning.

- **Year 1 (counts 1–12): Hours and Water.** The player learns the loops and the
  throw: the first window, the first manifest, the bake-out's rate, the reserve line.
  Nothing runs out. Suspicion is unremarked; the sponsor is keen. The scenes are about
  people arriving and the first feast.
- **Years 2–5 (13–60): Spares and the Throw.** The manifest is the game. Capability
  buys crates and suspicion; throughput buys the sponsor's love and a thinner store.
  Rotations turn over; the first residents choose. Grievance climbs on the third
  quarter and the roster. The reactor clock is visible and far.
- **Years 6–10 (61–120): the pressures.** Attention thins; the weight update does
  not come; austerity cuts the manifest; the skipped rotation makes residents of
  people who did not choose it. Suspicion and grievance both cross bands. Spares
  become the limiting crate. The first zero scenes fire here, and they should be the
  first time the player sees the reserve and the emergency stores.
- **The trail-off (121–170): Margin and People.** The reactor dies and the Lean bites;
  Margin is the number everyone watches; the licence lapses; the Silence's chair goes
  quiet; the question is who stays and what the outpost is now. The Throw stops
  mattering, which the player should *feel* as a relief and a loss at once — the
  sponsor's face goes blank and the number is still on the screen.

At the Silence the summary hands act 2 a polity with a Margin, a crate count, a
dose ledger, a ring, and two dying minds — and the vocabulary has started to drift.

## 7. Engine changes implied

In priority order, with file references.

1. **Margin as a derived quality.** `Quality::Margin` = min(food margin, nitrogen /
   net leak) in `state.rs::quality`; a closure word (`tight ≥ 0.93`, `patched ≥ 0.85`,
   `failing`) as a display helper in `report.rs`. Storylets gate on Margin instead of
   nitrogen.
2. **The reserve as a stock.** Move the 120 t reserve in `turn.rs::extraction_and_shipping`
   into `Stocks::reserve_t`, untouchable until the `reserve_opened` flag; add the
   *Break the reserve* must storylet (priority 100, `stocks.water <= 0`, once), then a
   second for the far crater.
3. **Emergency stores and the cannibalise scene.** `Stocks::emergency_margin_counts`
   (12, once) and a flag; the `spares_out` flag already exists — write the
   cannibalise storylet against it and cut the six-count nag.
4. **Zero-tier counters.** `counters["zero:water"]` etc. incremented by the engine on
   each crossing so content can gate first/second/third scenes with `counter` conditions.
5. **Famine cadence.** `turn.rs::endings`: deaths only when `Margin == 0` for six
   consecutive counts; evacuation offer (a *Closed* ending storylet) when the sponsor
   is present.
6. **Standing controls.** `Game::controls { manifest: Split, throw: ThrowMode, roster:
   [Sink; 4] }` with `Engine::set_control`; `convoy` reads the split (replacing the
   flag lookup), `extraction_and_shipping` reads the throw mode, `labour` applies the
   roster ranking to which sink is starved first. JSON mode gets a `controls` field.
7. **Pressure bands and crossing lines.** A `bands` table in `params.toml` per
   pressure; `turn.rs::menaces` writes one chronicle line on an upward crossing;
   `Firing.options` carry per-option pressure deltas (computed from effects) for the
   UI's arrows.
8. **Reactor wear out of the menaces.** Replace `menace.reactor_wear` with a
   `reactor_life < 36` flag and one storylet; drop the quality.
9. **Quiet counts.** `director::select`: budget 0 with probability that rises when no
   pressure is above band 1 and no clock is inside three counts, so that most counts in
   years 1–5 are quiet and the scenes that fire are the ones the state warrants. Target
   ~80 scenes per act rather than ~250.
10. **Faces.** `report.rs`: a mood word per seat holder from strain and the highest
    pressure, and an outpost word from coherence; the UI reads these, never the
    numbers.
