# The tutorial as progressive disclosure

The problem, from two playtests: the tutorial tells a story but never explains the
game. The page opens with every region lit — five countdowns, four project clocks,
twenty-five dice, seven bars, three pressure rings, the sponsor's dots — and the
first scene asks the Commander to make a decision about a system nobody has named.
Playtest 2's player put three `·` dice on projects before learning what a face was;
playtest 1's player never learned what a pressure was at all.

The fix is the one Citizen Sleeper and Frostpunk both use: the screen starts nearly
empty and each mechanic appears at the moment it first matters, with one line from
the voice that runs the station. In act 1 that voice is the station's machine mind,
SX-19F, reporting to the Commander. Every explanation below is the mind's briefing,
in the report register: clipped, technocratic, no metaphor, one new noun per report.

Design principles, so the writer and the page agree:

1. **A mechanic appears when the mind first has to report on it**, never before. Until
   then it is not greyed, not hinted, not present. The engine runs it regardless.
2. **The mind explains what a thing is and what a control does, once.** It never
   says whether a choice was good. The consequence does that, one count later.
3. **One lesson per scene, one scene per count.** A lesson is: a region appears, the
   mind names it, the Commander does one thing with it, the next count shows the
   effect.
4. **The reveal is the caption, not a tour.** A region slides in with a single line
   under its heading, in the mind's voice, and the line stays until the next reveal.
   No modal, no arrow, no "click here".
5. **The full act-1 screen is the handoff's reward.** At MM 43 everything is lit, the
   language turns, and the mind stops briefing.

## 1. The reveal order

Twenty lessons over the tutorial's forty-three counts. Each entry: **count** it
lands in, **appears** (exact regions, bars, clocks, controls), **the Commander
must** (the one action that passes the lesson), **the mind says** (a sample in the
register; the writer will do better), **flags** the scene sets.

The page starts showing only: the header (station name, count, the mind's headline),
the scene column, and the **End count** button. Nothing in the rail, nothing in the
ring column (the ring column is present but empty, as it is now).

### L1 — The count and the log. MM 1.
**Appears:** the scene column (already), End count (already), the **Chronicle**
button. Nothing else.
**Must:** choose an option, then press End count.
**Mind:** "SX-19F reporting. Mission month 1. Crew of fourteen landed. This screen
is the station log; each entry is one month. Decisions are logged when made. End
count advances the month."
**Flags:** `ui:chronicle`.
**Scene:** `tut_arrival` (survives; its text becomes the mind's site-handover
report; it still sets `seat:hulls`, `seat:hours`, but see L4 — the ring column
stays hidden until then, so the seats exist in the engine before the player sees
the column).

### L2 — The window. MM 2.
**Appears:** `#countdowns` with the **RSW** clock only (`ui:clock:rsw`).
**Must:** nothing; read it and End count.
**Mind:** "Resupply Window 1 opens in ten months. The ring fills as the window
approaches. A ship can leave Earth for this station only while it is open; nothing
sent outside a window arrives."
**Flags:** `ui:countdowns`, `ui:clock:rsw`.
**Scene:** new, `tut_window` — a short report scene with a single option ("Noted")
so the count has a scene but no decision. Alternative: fold into `tut_hand`'s
opening paragraph and skip the extra scene; the lesson order below assumes the
separate scene because the hand is a big enough lesson on its own.

### L3 — The hand, people only. MM 3.
**Appears:** `#hand` (`ui:hand`), showing **only person dice**, with the count
caption ("12 of 14 free"). No robots yet. No projects yet. Auto-deal is forced
**off** during the tutorial until L14; the control itself is hidden.
**Must:** nothing yet — read the dice. (The scene explains faces.)
**Mind:** "Crew allocation board. Each crew member is one unit of labour per month;
the number is their rating in the task they are assigned to. Two of fourteen are
consumed by station upkeep this month and are shown dimmed. The rest are free."
**Flags:** `ui:hand`.
**Scene:** `tut_hand` (survives, rewritten): it currently teaches the split; it
becomes the reading lesson. Its options become "Review the board" style
acknowledgements plus the existing `seat:extraction` introduction moves to L6.

### L4 — A project clock, and dragging a die. MM 4.
**Appears:** `#projects` (`ui:projects`) with **one** clock: `dig_keep` (KEEP
excavation), opened this count by the scene (`open_project:dig_keep`), 0/24. The
bake-out and the throw are *not shown yet* even though the engine has them open;
their dice appear in the hand as free (see gating rule G3).
**Must:** drag at least one die onto the KEEP clock. The next count's scene checks
`counter` "keep_pips"? — not available; instead the L5 scene is gated on the
project having any dice: no such condition exists either. Use the simplest honest
gate: L5 fires unconditionally at MM 5 and its text branches by reading nothing;
the *consequence* is visible on the clock itself (segments filled or not). The
mind's L5 report says "KEEP excavation: N segments this month" from the clock the
Commander can see. (Engine note, §6: a `project:<id>.dice` condition would let the
scene react; optional.)
**Mind:** "Project clock: KEEP excavation. Kilotonne-class Excavated Environmental
Protection — the buried habitat. Twenty-four segments. A die placed on the clock
adds its rating each month; six points fill a segment. Drag one die from the board
onto the clock."
**Flags:** `ui:projects`, `ui:project:dig_keep`, `open_project:dig_keep`.
**Scene:** new, `tut_first_project` (replaces the allocation half of the old
`tut_hand`). One option ("Open the clock") so the decision is the drag.

### L5 — The clock moves; the roll. MM 5.
**Appears:** nothing new; the KEEP clock shows filled segments.
**Must:** nothing; read the result.
**Mind:** "KEEP excavation advanced. Each clock makes one roll per month on its
best die: a high roll adds a segment, a low roll on a poor or strained die loses
one. The roll is the only luck on the board."
**Flags:** `tut_roll_explained`.
**Scene:** `tut_dead_dex` (survives, moved here as the count's story: the dead
dexterous unit) with the roll paragraph prepended to the mind's report. This is
also where **robots** are introduced — see L6.

### L6 — Robots, and the first standing project. MM 6.
**Appears:** robot dice in the hand (square, dimmed on upkeep); the **bake-out**
clock (`ui:project:bake_out`) with its rate ring.
**Must:** drag one robot (or one person) onto the bake-out.
**Mind:** "Autonomous units: eleven. A unit is a fixed-rating die that fits
structured work only. Unassigned units carry station upkeep, which frees crew.
Standing clock: volatile extraction. It has no end; its ring is a rate — full ring,
full output. It needs ten points of dice."
**Flags:** `ui:robots`, `ui:project:bake_out`, `seat:extraction` (the question
"who answers for the water and metal" arises here; the ring column is still
hidden, so this is invisible until L8).
**Scene:** new, `tut_units` — the old `tut_hand` "BOP" content lives here.

### L7 — Upkeep eats the hand. MM 7.
**Appears:** the **Hours** bar as the first ledger bar (`ui:ledger`,
`ui:bar:hours`), because the hand caption is the thing the player has been reading
and the bar is its flow.
**Must:** nothing; End count.
**Mind:** "Upkeep first. Each month the station consumes labour before any is free:
air, water, food, repair. Two of fourteen this month; more as the crew grows, fewer
as units cover it. The Hours bar is the free fraction."
**Flags:** `ui:ledger`, `ui:bar:hours`.
**Scene:** `tut_keep_race` (survives, at MM 7 as now): the CED ledger report is
the story; the upkeep paragraph is the lesson. Sets `seat:bodies`.

### L8 — The ring. MM 8.
**Appears:** `#ring` (`ui:ring`) with the seats that already exist: hulls, hours,
extraction, bodies — four cards, each with the question, the holder, and counsel
on this scene.
**Must:** choose an option after reading the counsel.
**Mind:** "Station Management Board. A seat is a question; the holder is whoever
answers it. Counsel is advice, not instruction. It can be wrong. The Commander
decides."
**Flags:** `ui:ring`.
**Scene:** new, `tut_board` — a small real decision with disagreeing counsel (the
shelter question from the old `tut_keep_race`'s options can move here). Note: the
old scheme revealed seats one at a time inside an always-visible column; this
scheme keeps the column hidden until four questions exist, then shows them
together, and reveals the fifth and sixth (air, liaison) one at a time afterward.
The reason: the first three counts already teach three things; a column with one
card at MM 1 reads as broken.

### L9 — Water, the first stock. MM 9.
**Appears:** **Water** bar (`ui:bar:water`).
**Must:** nothing.
**Mind:** "Water: 399 tonnes. The bake-out makes it; the loops lose what they
cannot close; the driver will ship it. The reserve line is 120 tonnes. Water is
the station's product and its life."
**Flags:** `ui:bar:water`, `seat:air`.
**Scene:** `tut_nitrogen` (survives at MM 9): the N2 make-up log is the story; the
mind's paragraph introduces the bar. The Air seat's card appears in the ring
column with a caption.

Why water first among the stocks: it is the only resource the player has already
acted on (the bake-out clock at L6), so the bar closes a loop rather than opening
one. Spares, power, margin, throw, people follow in the order the player can
*cause* them to change.

### L10 — The manifest split. MM 10.
**Appears:** the **manifest** select on the RSW clock (`ui:control:manifest`); the
**Liaison** seat card.
**Must:** set the split (or accept the scene's option, which sets the flag).
**Mind:** "Resupply request must transmit this month for RSW-1. Every tonne is
throughput hardware, which the sponsor reads, or capability hardware, which nobody
on Earth reads until it matters. The request is answered by the ship after next."
**Flags:** `ui:control:manifest`, `seat:liaison`, `manifest_*`.
**Scene:** `tut_manifest` (survives; move from MM 4 to MM 10, with `turn_max 11`).
The lesson "the answer arrives in the next window" stays.

### L11 — The window opens; spares. MM 12.
**Appears:** **Spares** bar (`ui:bar:spares`); the RSW clock fills and the convoy
lands (engine).
**Must:** nothing.
**Mind:** "RSW-1 landed: 42 tonnes. Spares: the parts the station cannot make.
Consumed at four crates a month; each RSW brings a baseline. When the crate count
reaches zero the loops fail slowly."
**Flags:** `ui:bar:spares`, `tutorial_open` (as now).
**Scene:** `tut_first_rsw` / `tut_rsw_empty` (survive). With `tutorial_open`, act-1
texture may fire from here; the page keeps hidden regions hidden regardless, so a
texture scene about power before L13 simply reads as a report about something the
Commander has not yet been shown — acceptable, and the writer's guide should tell
act-1 texture to avoid naming bars by name.

### L12 — Alignment complete; the throw and its position. MM 6–20 (when it fires).
**Appears:** the **throw** clock (`ui:project:throw`) with its position select
(`ui:control:throw`); the **Throw** bar (`ui:bar:throw`, φ).
**Must:** pick a position (the scene's options set `throw_*`).
**Mind:** "MDLS alignment complete. Mass driver operations is a standing clock; its
dice set tonnes per month. Position: Ship — everything above the reserve; Hold —
only what the tank can spare; Stop. Throughput ratio, tonnes shipped per tonne
received, is the one number the sponsor reads."
**Flags:** `ui:project:throw`, `ui:control:throw`, `ui:bar:throw`, `throw_*`.
**Scene:** `tut_driver_aligned` (survives). Because it fires on `driver_aligned`,
its count varies; if it fires before L4 (possible if auto-deal were on — it is
forced off in the tutorial, and `align_driver` is not opened until L12b below), the
order still holds.

Change to the arc: `align_driver` is opened by `tut_first_rsw` (L11), not at
start, so the driver is aligned *after* the first window — "pods with nowhere to go
until the driver is true" — which makes L12 land around MM 14–18 and keeps the
first ten counts to one project. `tut_driver_still` (MM 24 fallback) survives.

### L13 — The review and the sponsor. MM 14 (as now, `counts_to_review ≤ 2`).
**Appears:** the **review** clock (`ui:clock:review`); the **sponsor track**
(`ui:sponsor`) with the seven dots, the mood word, φ against expectation.
**Must:** choose how to report.
**Mind:** "Quarterly review in two months. The sponsor scores throughput ratio
against its expectation, which rises. Confidence sets what the next RSW carries.
Seven stages from nominal to no further arrivals; the station is at the first."
**Flags:** `ui:clock:review`, `ui:sponsor`.
**Scene:** `tut_review` (survives).

### L14 — Power and the reactor. MM 15.
**Appears:** **Power** bar (`ui:bar:power`); **reactor** clock (`ui:clock:reactor`).
**Must:** nothing.
**Mind:** "Power: 526 of 372 kilowatts. Reactor core life 135 months; not
refuellable from belt material. Photovoltaic area is the remainder. The driver is
first to go dark when power is short; the farm is last."
**Flags:** `ui:bar:power`, `ui:clock:reactor`.
**Scene:** new, `tut_power` — a report scene with one option; or attach to the
first act-1 texture scene that fires after L13. Prefer the report scene: it is
short, and the reactor clock is one of the three clocks the whole game is about.

### L15 — Margin. MM 16–18.
**Appears:** **Margin** bar (`ui:bar:margin`) with its closure word.
**Must:** nothing.
**Mind:** "Consumables margin: eight months. The smaller of the farm's buffer and
the nitrogen make-up time. The word beside it is closure: tight, patched, failing.
Margin is the number the station dies by."
**Flags:** `ui:bar:margin`.
**Scene:** new, `tut_margin`, placed on the first count after L14 with no other
scene (gate `turn_min 16`, `turn_max 20`, `fired tut_power`).

### L16 — The KEEP is dug; the crew moves; dose. MM 13–22 (when `keep_dug`).
**Appears:** nothing new on the rail; the **People** bar (`ui:bar:people`) with its
mean dose in the hover.
**Must:** choose the move.
**Mind:** "KEEP pressurised. Surface dose 120 millisieverts a year; KEEP dose 12.
Cumulative effective dose is kept per person and shown on the crew bar. Cataracts
above half a gray; the sponsor's career limit is 600 millisieverts."
**Flags:** `ui:bar:people`.
**Scene:** `tut_keep_done` (survives).

### L17 — The first pressure. MM 19–23.
**Appears:** `#pressures` (`ui:pressures`) with **one** ring: whichever is highest
— usually the N2 make-up (`ui:pressure:leak`); the other two appear when each first
leaves its lowest band (`ui:pressure:suspicion`, `ui:pressure:grievance` set by the
engine's band-crossing lines? — no: the page can gate them on the view's `band > 0`
without a flag; see §5).
**Must:** nothing; read the band name.
**Mind:** "Pressure: nitrogen make-up. Bands: tight, weeping, rationed, the collar.
A pressure that reaches its last band forces a decision the board cannot defer.
Every option in a decision moves at least one pressure; the direction is not
shown."
**Flags:** `ui:pressures`.
**Scene:** `tut_pressures` (survives, becomes once: `once = true`, `turn_min 19`).

### L18 — Contracts and rotation. MM 20–27.
**Appears:** the **contract** clock (`ui:clock:contract`).
**Must:** nothing until MM 27's decision.
**Mind:** "Crew contracts: thirty months. At expiry a crew member rotates home on
the next RSW unless they choose residence. The contract clock shows the next
expiry."
**Flags:** `ui:clock:contract`.
**Scene:** new one-option report at MM 20–22 (`tut_contracts_clock`), then
`tut_contract` (survives at MM 27) is the decision.

### L19 — Roster order and auto-deal. MM 22–34.
**Appears:** the **roster** select and the **auto-deal** switch in the hand
(`ui:control:roster`, `ui:control:autodeal`). Auto-deal is offered, not forced.
**Must:** nothing; the option sets `roster_*`.
**Mind:** "Roster order sets which crew upkeep consumes first: lowest rating,
highest strain, or by name. Auto-deal lets SX-19F place free dice on standing
clocks each month. The Commander may leave it on and override by hand."
**Flags:** `ui:control:roster`, `ui:control:autodeal`, `roster_*`.
**Scene:** `tut_roster` (survives, `once = true`).

### L20 — Expansion, the Sun, and the handoff. MM 28 and MM 43.
**Appears at MM 28:** the **solar** clock (`ui:clock:sun`) — the last countdown,
introduced with the storm risk in the expansion report. **Appears at MM 43:**
everything; the mind's captions clear; the lexicon turns (`first_convoy` fires
because `tutorial_done` is set).
**Must (MM 43):** the handoff choice.
**Mind (MM 28):** "Solar cycle: rising toward maximum. Storm risk up, galactic dose
down. Crew augmentation: twelve arriving. Upkeep will consume more dice than the
units cover; the Hours bar will fall."
**Mind (MM 43):** none — the handoff scene is the crew's voice, and it is the first
line of the log that is not a report.
**Flags:** `ui:clock:sun` (MM 28), `ui:all`, `tutorial_done` (MM 43).
**Scenes:** `tut_expansion`, `tut_handoff` (survive).

**Left for act 1 to teach by consequence** (no lesson, no caption): grace and
conjunction clocks (they appear when the engine creates them; the engine's own
line explains them); the reactor's end; the second and third pressures; the
manifest's effect on stocks; strain dulling a die (the die is drawn half-shaded;
the hover says why); the standing diamonds when they exist; the lexicon beyond the
handoff.

**Counts with no lesson** (MM 11, 13, 17, 21, 23–26 depending on fires) are quiet
counts or act-1 texture after `tutorial_open`: End count and the clocks. That is
deliberate — a third of the tutorial should be watching clocks fill.

## 2. Gating rules

**G1 — Hidden means absent.** A gated region is not rendered at all (not greyed,
not collapsed). The rail shrinks to what exists; the scene column and the ring
column keep their places. Playtest 1 found the pressures below the fold; with the
rail this short for the first twenty counts, nothing is.

**G2 — The engine runs everything.** Hiding is display only. Water falls, spares
burn, pressures move, the sponsor reviews, whether or not the bar is shown. The
lessons are ordered so that nothing hidden can *fail* before it is shown in a
normally played tutorial: water starts at ~400 t with the bake-out running by MM 6
(L6 asks for a die on it; if the Commander refuses, the mind's L9 report says the
tank is falling and why); spares start at ~218 and cannot reach zero before L11;
power is in surplus until the crew grows (L14 precedes L20's expansion); the
reactor has 150 months; pressures start at zero. If a hidden thing goes wrong
anyway (a shock on the sponsor, a relay failure), the engine's chronicle line
still appears in the events column — the page never suppresses events — and the
mind's next report names the region and reveals it early (see G5).

**G3 — Hidden projects and their dice.** The bake-out and the throw are open in the
engine from MM 1 (the site is running). Until L6/L12 their clocks are hidden and
any dice the engine has on them show in the hand as **free** — the page relabels a
die whose `place` is a hidden project as `hand` — but they are not draggable-off
(the engine keeps them there). Simplest honest rule: the tutorial scenario starts
those two projects with no dice and auto-deal off, so there is nothing to hide; the
engine already leaves robots on upkeep. (Engine note: the tutorial should not
auto-deal at all before L19; today the scenario inherits `auto_deal: true` — see
§6.)

**G4 — The reveal.** When a `ui:*` flag first appears in `view.flags`, the page
inserts the region with a caption line beneath its heading: the mind's sentence,
taken from the scene (see §5 for how the page gets the text). The caption persists
until the next reveal replaces it; the last caption clears at `ui:all`. A revealed
region never hides again.

**G5 — Early reveal on failure.** If the engine writes a chronicle line that names
a hidden region's subject ("Spares inventory zero", "Relay unit failure", a band
crossing, "Reactor core end-of-life"), the page reveals that region with the
engine line as its caption. The mapping is a short table in the page (line prefix
→ region). This is the only reveal not driven by a scene.

**G6 — Skip.** A **Skip tutorial** control in the New-game dialog (not on the
screen) starts the tutorial scenario and immediately sets `tutorial_done` and
`ui:all` through a scene? No — content cannot be invoked by the page. Instead the
page sets a local flag `skipTutorial` and treats every `ui:*` gate as open; the
engine still runs the scripted scenes (they are the arc, not the lessons) but the
mind's captions are suppressed. A returning player who has finished the tutorial
once gets the skip offered by default (`localStorage` remembers `tutorial_done`).
If the engine gains a `skip_tutorial` scenario later, the page uses it; see §6.

**G7 — Act 1 start (`--scenario act1`)** shows everything from count 1: the page
treats the absence of the `tutorial` flag as `ui:all`.

## 3. The scenes

Order, with count, lesson, and status. Twenty-two scripted scenes, one per count at
most; priorities as now (must = 100+; the engine limits the tutorial to one scene a
count).

| MM | scene | status | lesson | flags to add |
|---|---|---|---|---|
| 1 | `tut_arrival` | survives, mind's voice | L1 | `ui:chronicle` |
| 2 | `tut_window` | **new** | L2 | `ui:countdowns`, `ui:clock:rsw` |
| 3 | `tut_hand` | survives, becomes the reading lesson | L3 | `ui:hand` |
| 4 | `tut_first_project` | **new** (from old `tut_hand`) | L4 | `ui:projects`, `ui:project:dig_keep`, `open_project:dig_keep` |
| 5 | `tut_dead_dex` | survives, roll paragraph added | L5 | `tut_roll_explained` |
| 6 | `tut_units` | **new** (from old `tut_hand` BOP) | L6 | `ui:robots`, `ui:project:bake_out`, `seat:extraction` |
| 7 | `tut_keep_race` | survives; options move to L8 | L7 | `ui:ledger`, `ui:bar:hours`, `seat:bodies` |
| 8 | `tut_board` | **new** (shelter decision) | L8 | `ui:ring` |
| 9 | `tut_nitrogen` | survives | L9 | `ui:bar:water`, `seat:air` |
| 10 | `tut_manifest` | survives, moved from 4 | L10 | `ui:control:manifest`, `seat:liaison` |
| 12 | `tut_first_rsw` / `tut_rsw_empty` | survive; open `align_driver` here | L11 | `ui:bar:spares`, `open_project:align_driver`, `tutorial_open` |
| 14 | `tut_review` | survives | L13 | `ui:clock:review`, `ui:sponsor` |
| 15 | `tut_power` | **new** | L14 | `ui:bar:power`, `ui:clock:reactor` |
| 16–20 | `tut_margin` | **new** | L15 | `ui:bar:margin` |
| var. | `tut_driver_aligned` | survives | L12 | `ui:project:throw`, `ui:control:throw`, `ui:bar:throw` |
| var. | `tut_keep_done` | survives | L16 | `ui:bar:people` |
| 19+ | `tut_pressures` | survives, `once` | L17 | `ui:pressures` |
| 20–22 | `tut_contracts_clock` | **new**, one option | L18 | `ui:clock:contract` |
| 22+ | `tut_roster` | survives, `once` | L19 | `ui:control:roster`, `ui:control:autodeal` |
| 24 | `tut_driver_still` | survives (fallback) | — | — |
| 25+ | `tut_throw_position` | survives, `once`; gate `flag = throw_taught` | L12 reprise | — |
| 27 | `tut_contract` | survives | L18 decision | — |
| 28 | `tut_expansion` | survives | L20a | `ui:clock:sun` |
| 28–44 | `tut_rsw_missed` | survives | — | — |
| 43 | `tut_handoff` | survives | L20b | `ui:all`, `tutorial_done` |
| — | `tut_mind_log` (MM 5 today) | **moves to MM 11** (a quiet count) | story only | — |

Removed: nothing; `tut_hand` is split into three. Net: twenty-two scenes, of which
six are one-option reports (L2, L4, L14, L15, L18, and the roll paragraph rides on
`tut_dead_dex`). One-option scenes are the price of one lesson per count; keep
them under sixty words each.

The arc is unchanged: robots built the site; the crew arrives; the KEEP is dug
before the first storm; RSW-1 lands with the request's answer; the driver is
aligned after it; the review; a contract; the expansion; the handoff.

## 4. Explaining without lecturing

**The naming rule.** A control is named exactly once, in the report that reveals
it, in the form *label: what it is; what to do with it* — "Allocation board: one
die per crew member; drag a die onto a clock." — and never again. Later scenes
refer to the thing by its label only ("the board", "the KEEP clock", "the split").
If a player forgets, the hover on the region repeats the caption (the page keeps
the reveal caption as the region's `title`).

**What the mind may explain.** What a stat *is* (its unit, what raises and lowers
it, the line that matters — the reserve, the career limit, the last band). What a
control *does*, mechanically. What the engine did last month ("KEEP advanced two
segments; one lost to a setback"). What is scheduled (a window, a review, a
contract).

**What the mind may not say.** Whether an option is wise; what an option will do to
a pressure or a stock (the counsel may guess; the mind reports outcomes only);
anything about the future beyond the clocks; anything in the second person that
is not an instruction ("the Commander should" is banned; "recommend" belongs to
the seats).

**How the consequence teaches.** Every lesson's *do* has a visible result the next
count without a scene: the segment count on the clock, the rate ring, the bar's
delta arrow, the caption's number changing. The writer's test for a lesson: delete
the mind's explanation and ask whether a player who did the thing could still see
what it did. If not, the lesson needs a better consequence, not a longer report.

**The register for lessons.** Reports are the mind's; the register is canon §9's
strictest voice: acronym expanded on first use, MM numbers, no contractions, no
metaphor, "nominal / off-nominal", present tense for state and past tense for what
happened. Three sentences is the ceiling for an explanation; the fourth sentence
is the instruction, when there is one.

## 5. Page work implied

**Gates.** Each element below is rendered only when the named flag is in
`view.flags`, or when `ui:all` is, or when the `tutorial` flag is absent.

| element | flag |
|---|---|
| `#btn-chronicle` | `ui:chronicle` |
| `#countdowns` section | `ui:countdowns` |
| countdown by id (`rsw`, `review`, `reactor`, `contract`, `sun`) | `ui:clock:<id>` (grace and conjunction: always, when present) |
| `#projects` section | `ui:projects` |
| project clock by id | `ui:project:<id>` (any project not listed in a lesson — `through_wall_arm`, `repair_relay`, `reline_bearing`, `mirror_field`, `teach_reckoning` — shows once `ui:projects` is set; they only open by their own conditions) |
| `#hand` section | `ui:hand` |
| robot dice in the hand | `ui:robots` |
| `#roster` select | `ui:control:roster` |
| `#auto-deal` switch | `ui:control:autodeal` |
| `#ledger` section | `ui:ledger` |
| bar by id | `ui:bar:<id>` |
| `#pressures` section | `ui:pressures` |
| pressure ring by id | `ui:pressures` and (the ring's `band > 0` or `ui:pressure:<id>`) |
| `#sponsor-track` | `ui:sponsor` |
| `#ring` column | `ui:ring` (seats within it already gate on `seat:*` in the engine) |
| manifest select on the RSW clock | `ui:control:manifest` |
| throw select on the throw clock | `ui:control:throw` |

**Captions.** One `<div class="reveal">` per section, under the `<h3>`, filled when
the section's flag first appears. The text comes from the scene: add an optional
`caption` string to the storylet's `[[option.effect]]` vocabulary? That is an
engine change; avoid it. Instead the page carries the twenty captions in a small
table keyed by flag (they are UI copy, not content), written in the mind's register
by the same writer. Reveal captions for G5 (early reveal on failure) use the engine
line itself.

**Reveal behaviour.** A newly rendered section gets a `.revealed` class for one
count (a brief brighten, no motion); the caption stays until the next reveal.

**The hand before `ui:robots`.** Filter `view.hand.dice` by `!robot`; the caption
count ("12 of 14 free") already excludes robots.

**Hidden projects' dice (G3).** If a die's `place` is a project without
`ui:project:<id>` and without `ui:all`, render it in the hand as free and
non-draggable, with the hover "assigned by the station" — or, preferred, avoid the
case by the engine change in §6 (no auto-deal before L19).

**Skip.** In the New-game dialog: a checkbox "Skip the tutorial's briefings" that
sets `localStorage.skipTutorial = 1`; when set, every gate is treated as open and
captions are not shown. Offered checked by default once `tutorial_done` has been
seen in a save.

**View fields missing.** None strictly. Nice to have: `view.hand.dice[].place`
already carries the project id; countdowns carry `id`; bars carry `id`; controls
do not need ids (they are fixed elements). The pressure rings carry `id` and
`band`. The only addition worth making is `view.tutorial: { active: bool, lesson:
string | null }` derived from flags, so the page does not parse `ui:*` strings —
optional.

**Phone.** The strip in gui.md §1 compresses to what is revealed; with two clocks
and one bar it is nearly empty for the first ten counts, which is right.

## 6. Engine work implied

Preferably none; three small items would make the lessons cleaner:

1. **Tutorial scenario: `auto_deal` starts off** and standing projects start with no
   dice (the robots on upkeep), so nothing is hidden in the hand before L4/L6.
   One line in `setup::new_game_scenario`. Without it the page needs the G3
   relabelling.
2. **`open_project:align_driver` moves from setup to the RSW-1 scene** (a content
   flag the engine already honours; remove the flag from the scenario's initial
   set). One line.
3. **Optional: a `project` condition** — `[[when]] project = "dig_keep"` with
   `dice_min = 1` or `filled_min = 1` — so L5's report can branch on whether the
   Commander placed a die. Without it the mind reports the number from the clock
   and the branch is unnecessary; skip unless the writer wants it.
4. **Optional: a `skip_tutorial` scenario** that starts at act 1's state after the
   handoff (population ~28, the flags set, `tutorial_done`), for returning players
   who want act 1 without the arc. The page's local skip (G6) covers the briefings
   only.

Nothing else: `ui:*` flags are ordinary flags; the view already exposes them; the
`seat:*` gating, `open_project:*`, `manifest_*`, `throw_*`, `roster_*` and
`tutorial_open` all exist.
