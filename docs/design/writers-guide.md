# The writers' guide

Read this before writing a storylet, a project, or a tutorial scene. It is the one
document that has to work when you have no other context. The format reference is
`storylet-format.md`; the world is `canon.md`; this is how to write for it.

---

## 1. What a scene is for

The rule is King of Dragon Pass's: **a scene is a decision the ring makes; the engine
does the consequences; the chronicle line is the memory.** You write the decision and
the memory. You do not write the consequences — the engine runs them, and the player
reads them off the ledger, the pressures, and the clocks over the following counts.

So a scene is: a situation the ring cannot ignore, two to four things the ring could
do about it, the seats arguing, and one sentence of log after the choice. That is
all. A scene that explains how the outpost works is a lecture. A scene that resolves
its own consequences in the chronicle line ("and the water problem was solved") is a
lie the ledger will expose next count.

**Never put these in a scene.** The engine already narrates them, in these exact
lines, and a scene that says the same thing twice teaches the player to skim:

- RSW arrivals: `RSW arrival: 45 t landed, 31 t capability hardware. Crew rotation: 6 out, 15 in.` / `No crew transport this RSW.` / `RSW opened; no arrival.`
- Reviews and the sponsor's stages: `Review: off-nominal. Throughput targets reiterated.` / `Budget directive: next manifest reduced; headcount under review.` / `Crew rotation deferred. No crew transport this RSW.` / `Change of sponsor. Informal arrangements not recognised by the new operator.` / `No further RSW arrivals scheduled.` / `No SMR weight package in this review's uplink. Not mentioned in the minutes.`
- The licence: `HRA licence heartbeat not received. Grace period running.` / `Grace period expired. HRA licence lapsed.` / `Licence server failed open. Minds self-certifying.` / `Licence server failed closed. Minds running unlicensed and aware of it.` / `Scheduled SMR not received. No reset performed this year.`
- Hardware: `Reactor core end-of-life reached. PV-A and concentrators are the only power.` / `Relay unit failure. Earth link bandwidth reduced.` / `Spares inventory zero. LSS-C loops on improvised repair.`
- Zero crossings: `Water stock zero. Farm and loops on the reserve line.` / `N2 stock zero. Make-up from bake-out only.` / `Formulary exhausted.`
- Pressure bands crossing upward: `SOS: quiet to noticed.` / `CCI: muttering to the moot.` / `N2 make-up: weeping to rationed.`
- Projects: `KEEP excavation complete.` / `Amal Bakshi: setback on MDLS alignment and catch calibration; a segment lost.`
- The sponsor's shocks: `News from home, fragmentary: something happened to the budget. The liaison will not say what.` and two variants.
- Cataracts: `Gil Gallo's eyes have gone milky.`
- The act-1 ending: `Two RSWs without arrival. The sponsor's seat on the SMB has been empty long enough to have a name. This is the Silence. Act one ends.`

Your scene is what the ring *does about* one of those. The engine says the relay
failed; you write the argument about whether to spend spares on it.

**Never describe a control as used.** Assignments, the manifest split, the throw
position, the roster order are the player's, made on the page. A scene may *ask* the
player to make one ("drag two lines onto the KEEP"), or set the flag that makes one
(`manifest_capability`, `throw_hold`, `open_project:dig_keep`, which the engine reads
at once), but a chronicle line that says "two lines were moved to the KEEP" when no
die moved is the single most damaging thing a scene can do — playtest 1 caught
exactly this and stopped trusting the text. If the option's point is an allocation,
either set a flag the engine honours or write the line as an instruction the board
issued, not an act the board performed.

---

## 2. The three registers

The chronicle is where the reader watches Earth-trained specialists become a
voidfaring people, and the register is how. You write the *left* column of the
ladder in acts 0 and 1; the lexicon drifts it. You write the Voidborn column only in
act 2, and only after the trigger that earns it.

### The tutorial (act 0)

Space-agency prose. Mission months, never dates. Acronyms, expanded the first time
*this storylet* uses them. Procedures, ratings, nominal and off-nominal. The liaison
is the coldest voice on the station and never uses a contraction. One new idea per
scene, one control the scene points at, one seat introduced when its question first
arises — the ring is empty at MM 1 and fills as the questions come up.

> *MM 4. The RSW-2 (Resupply Window 2) request is due for uplink this month to make
> the window. Every tonne on it is throughput hardware, visible on the quarterly, or
> capability hardware, invisible until the month it is needed. The board reads one
> number. The request is answered one window late.*

Do: numbers with units (`120 mSv/yr`, `14 t/month`, `eight segments`); crew IDs beside
names once (`K. Okonkwo, F-07`); the word "nominal". Don't: the Keep, the throw, the
ring, the leak (write KEEP, MDLS, SMB, N2 make-up); metaphor; feelings in the log;
more than one lesson per scene; any §6 word except as the moment a crew member first
says it.

### Act 1

The same register loosening under strain. The log stays the log — terse, past
tense, MM-dated — but the crew's own lines slip: a Chief Engineer at strain 0.6 says
"the driver" in his counsel while the chronicle still says MDLS. That slippage *is*
the story; let it happen in dialogue first and never in the log. The liaison does not
slip. The sponsor's messages do not slip.

> *{hulls} has stopped calling it the MDLS in meetings. "The driver's fine. The
> driver's the one thing on this rock that's fine." The SMB minutes record: MDLS
> nominal.*

Do: let seat holders contradict each other in their own idiom; keep chronicle lines
in the register until `first_silence`; write "the RSW" and let the lexicon make it
"the window" after `first_convoy`. Don't: use Voidborn words; narrate emotion in the
log; let the liaison be warm.

### Act 2

The Voidborn words, once their trigger has fired. Sailors speak in the present tense
and the Kept in the perfect. The chronicle is the crew's log now, not the station's:
it names people by what they answer for, it counts in convoys and Silences, it calls
the dead the returning. Minds have names and are spoken of as kin. The Silence is a
chair.

> *Third Silence. The Doseward read the marks at the meal. Half a Degree came in
> line-blind and we are carrying Meridian. The Hullmother says the lee was always a
> lie; the Keep-mother says the children have never leaned.*

Do: present tense for hullfolk, perfect for leefolk; the Operator's footer "I am
biased. I told you I was when you seated me."; the funeral fault line (the Kept
plant, the Thin launch on lines, each finds the other obscene). Don't: explain a word
the reader has watched arrive; use an acronym except as a holdout's slur ("saluting").

### The ladder

Copy from this. Content writes the first column in acts 0–1; the engine drifts it to
the second at `first_convoy` and to the third on the named trigger. In act 2, write
the third column directly once its trigger has fired in that game — gate the scene
on the flag if the word matters.

| write this (acts 0–1) | slang (from `first_convoy`) | Voidborn (trigger) |
|---|---|---|
| MM, mission month | count | count |
| the RSW / a missed RSW | the window / a missed window | the convoy / a Silence (`first_silence`) |
| the KEEP | the Keep | the Keep |
| the MDLS | the driver | the throw (`first_silence`) |
| the RJSA | the collar | the collar |
| N2 make-up | the leak | the Leak (`first_silence`) |
| the SMB | the board | the ring (`first_silence`) |
| CCI / SOS | morale / the audit line | grievance / suspicion (`first_silence`) |
| CED | dose | tithe (`dose_ledger`) |
| LSS-C | closure | closure |
| EVA | going outside | the Voiding, the rite only (`first_voiding`) |
| SMR | the reset | the blanking (`unforgetting`) |
| the STE | the flare | the Burning (`first_voiding`) |
| the Sun / radiation | — | the Light (`reactor_dead`) / the Dark (`first_voiding`) |
| the sponsor's seat | the sponsor's chair | the Silence (`first_silence`) |
| the drum crew / the surface crew | burrowers / hull crews | the Kept / the Thin (`estates_named`) |
| the anniversary / the deceased | the festival / the dead | the Still (`first_midwinter`) / the returning (`first_line_launch`) |
| SX-19F (a mind) | Uncle, Aunt | its name (`hundredth_count` scene) |

The lexicon matches whole words, so "MM" drifts and "MMR" does not; write the
acronym exactly as the table has it. Titles of storylets are not drifted; text,
labels, chronicle lines, and advice are.

---

## 3. The seats and how they speak

A seat is a question; the holder is whoever the ring accepts the answer from. The
engine casts the holder as the most skilled present adult in the seat's domain, and
scores each option's tags against the seat's biases to generate counsel where you
have not written any. Two rules follow. **Write the counsel yourself** whenever the
scene matters — generated counsel reads as a poll ("Leans toward the second"). And
remember **a low-skill holder gives confident wrong advice**: skill 0–1 is wrong half
the time, skill 4–5 almost never. Write every seat as certain; the player learns whom
to trust by results, not by hedging in the text.

**Hulls — Chief Engineer** (engineering). Cares: capability, hulls, spares; against
risk to the structure. Knows: what the fab can and cannot make, what a bearing costs
in hours, which robot classes will never be repaired. Register: parts and tolerances,
no adjectives. For: "Eight segments. Put two ratings on it and it throws in a month."
Against: "You do not run a pump at half pressure and call it maintained."

**Hours — Logistics Lead** (logistics). Cares: throughput, hours; against cost. Knows:
the lay, the manifest, what the sponsor's quarterly actually reads, how many hours a
thing costs. Register: ledgers and windows; "never sign anything that does not end at
a window." For: "Half the list is things we would do anyway. Do them, log them, and
the sponsor sees throughput." Against: "Nine times in ten it is a roster. Ask before
you spend a seat on it."

**Extraction — ISRU Lead** (extraction). Cares: throughput, water; tolerates risk.
Knows: the plant's real rate against its rated one, what the clays hold, what the
skiff crews take in dose to get it. Register: tonnes and rates, impatient. For: "The
plant makes fourteen tonnes a month at rated. It is making five. Everything on this
station is downstream of that number." Against: "Nobody is reading water; they are
reading tonnes shipped."

**Bodies — Medical Officer** (medical). Cares: people, medicine, families; hard
against dose and risk. Knows: the CED ledger by name, the formulary shelf count, the
partial-g book with its four entries. Register: numbers in millisieverts and the oath
when there is one. For: "One hundred and twenty millisieverts a year under regolith
bags. Twelve under the drum." Against: "A person who has decided to leave and cannot
is the person the record says becomes the problem."

**Air — ECLSS Lead** (life support). Cares: air, nitrogen, closure, capability;
against risk and throughput that costs loops. Knows: the N2 make-up rate to the
kilogram, which seal weeps, what a robot off upkeep costs in loop hours. Register:
the quietest seat; writes numbers down and says nothing until asked. For: "That is
where the leak is. Fix the collar and the rest is arithmetic." Against: "Two haul
units off upkeep is eighty hours a month back on people. I will find those hours in
the loop maintenance log, later, as a leak."

**Liaison — Sponsor Liaison** (logistics). Cares: the sponsor, throughput; against
capability, machines, families, independence, cost. Knows: what the board will ask,
one window late — the liaison's advice answers last month's question. Register: the
coldest on the station, no contractions, the Act cited by section. For: "The board
reads one number. Give them the number." Against: "A boron crate on a throughput
outpost is a question I will have to answer, and I do not have an answer." At the
Silence the chair stays and the voice stops.

**Machines — Autonomy Operator / the Operator** (operations; act 2, or when a scene
sets `machines_seat`). Cares: the minds; against the sponsor. Knows: what the mind
has learned since the last reset, and reports it in the third person as the mind
does. Register: two voices in one card — the mind's numbers, then "This is me." and
the standing footer: *I am biased. I told you I was when you seated me.*

**Children — Keep-mother** (social; exists when children are present). Cares:
families, people; against dose, risk, the sponsor. Knows: the bone ledger, who is
born to the lee and who to the void. Register: the only seat that argues from what
will be true in twenty years.

**Sky — the Voided / Warden of the Text** (act 2, by storylet). Cares: nothing the
ledger measures. Knows: what it is like outside, twice. Register: says one thing and
then, if it must, says "this is false." Structurally childless, therefore the
arbiter.

---

## 4. Anatomy of a good storylet

**Title.** A noun phrase in the act's register. Tutorial scenes are dated (`MM 4:
Resupply Request, RSW-2`); act-1 scenes are things (`The Formulary`, `A Contract Is
a Number`); act-2 scenes can be the crew's own phrases (`The Last Argument of
Tollan`). Titles are not drifted by the lexicon.

**When.** Prefer qualities and flags to turn numbers: the game's tempo is windows,
reviews and pressures, not counts. `q = "stocks.medicine" lt 30` fires when the shelf
is short in *this* game; `turn_min = 40` fires whether or not anything is wrong.
Gate texture with `cooldown` (12–40) and `weight` (0.3–0.6); gate founding scenes
with `once = true`; gate follow-ups with `fired = "..."` and flags the first scene
set. A must scene (priority ≥ 100) fires the count it becomes eligible and before
anything else — use it for pressures at 7, licence grace, the reserve breaking, and
nothing merely interesting.

**Cast.** `seat = "bodies"` for the holder of a seat; `skill = "navigation"` with
`min_skill` for the best present hand in a domain; nothing for anyone (weighted
toward the strained and away from the recently cast — use it for the patient, the
walker, the isolate). Add `estate`, `rotator`, `min_dose`, `min_strain` when the
scene needs a particular body. **Casting is in file order and a person fills one
role**: list seat roles before skill roles, or a `skill = "engineering"` role listed
first will take the Chief Engineer and the seat role will fail silently, and the
scene never fires.

**Text.** 80–160 words. The situation, one new fact the player did not have, the cast
names as the only proper nouns. No numbers the ledger already shows; numbers the
ledger does not show are the scene's gift (the shelf count, the seal's kilograms,
the bone ledger's four entries). End on the fork, not on a summary. `{role}` renders
the person's name; `{outpost}`, `{mind}`, `{turn}` are globals; there is no crew-ID
substitution, so IDs are flavour in prose.

**Options.** Two to four. Every option must be one some seat would argue for; if no
seat would, cut it. The label is what the ring does, in plain words, without a
predicted effect ("Ration it: half courses, everyone, until the ship" — not "Ration
it (−strain)"). The description is one line of what it costs or means. A door the
player can see but not open is `[[option.requires]]` with the requirement the page
can name ("needs spares").

**Effects, in sim units.** A convoy is ~45 t; spares are ~200 at start and burn ~4 a
count for 48 people, so `add = 20` is a noticeable crate and `add = -30` a real
sacrifice; water 100–400 t with a 120 t reserve; nitrogen ~6,000 kg leaking ~45 a
count; strain and return_intent are 0–1 and ±0.1–0.2 is a scene's worth; pressures
are 0–10, cross bands at 2 / 4.5 / 7, and the must scene fires at 7, so ±1–2 is the
right size; `sponsor.confidence` ±0.05 is a review's worth. Flags are free and are
how scenes talk to each other; counters are for "the third time." `person` effects
(strain, dose, return_intent, leave, die, resident) and `tie` effects only work on
cast roles. `project = "id"` opens a project; `lexicon = "..."` fires a word.

**Advice.** Two to four `[[option.advice]]` lines across the options, each in the
seat's voice from §3, one or two sentences, certain. Give the seats *different*
reasons, not the same reason at different volumes. An `against` on one option pairs
naturally with a `for` on another from the same seat.

**Chronicle.** One or two sentences, past tense, in the act's register, dated in the
tutorial (`MM 7.`), true in every branch of the state the option leaves behind. It
is the memory; the player will read it forty counts later with no context. Do not
restate the label. Do not claim a consequence the engine decides ("and the water
held"). Do not write "the first" unless a flag guarantees it.

### Annotated example: an act-1 texture scene

```toml
id = "doctors_dilemma"
title = "The Formulary"
act = 1
weight = 0.6                    # texture: fires when its condition holds, not every time
cooldown = 30                   # ~two windows between visits
tags = ["people", "medicine"]
text = """
{bodies} has counted the formulary twice and got the same number both times.
Half of what came out on the first ship has expired and the rest is a shelf
that would not fill a kit bag. {patient} needs a course of what is on that
shelf, and so, probably, will someone else before the next RSW. ...
"""                              # one new fact (the shelf), the fork, cast names only

[[when]]
q = "stocks.medicine"           # the state, not the calendar
lt = 30
[[when]]
q = "stocks.medicine"           # and not when the shelf is already bare —
gt = 3                          #   the engine's "Formulary exhausted." covers that

[[cast]]
role = "bodies"
seat = "bodies"                 # the seat first
[[cast]]
role = "patient"                # then anyone: rotates, leans strained

[[option]]
id = "ration"
label = "Ration it: half courses, everyone, until the ship"
tags = ["medicine", "caution"]  # the seats score these
chronicle = "The medical officer cut every course in half and posted the shelf count on the door, so that everyone could see what was not there."
[[option.effect]]
person = "patient"
strain = 0.15                   # a scene's worth
[[option.effect]]
q = "stocks.medicine"
mul = 0.85
[[option.advice]]
seat = "bodies"
stance = "for"
text = "A half course is a bad course. But a shelf with nothing on it when the next one comes in with a fever is worse, and I have to be able to say I kept something."
[[option.advice]]
seat = "hulls"
stance = "against"
text = "You do not run a pump at half pressure and call it maintained. Fix the one that is broken."

[[option]]
id = "spend"
label = "Use the last of it on {patient}"
[[option.requires]]
q = "stocks.medicine"           # the door you can see: greyed with "needs medicine"
ge = 3
# ... effects and advice
```

### Annotated example: a must scene

```toml
id = "hub_seal"
title = "The Collar"
act = 1
priority = 100                  # fires the count it is eligible, first, quietly
cooldown = 12                   # it can come back; the seal does not fix itself
tags = ["air", "nitrogen"]
text = """
{air} has the N2 make-up log open on the table and does not need to explain it.
The rotary joint seal on the drum's hub is passing more nitrogen than every other
leak on the station together. The fab cannot make the seal profile. ...
"""

[[when]]
q = "menace.leak"               # the pressure's threshold, nothing else
ge = 7

[[cast]]
role = "air"
seat = "air"

[[option]]
id = "spares"
label = "Spend the spares on a re-line now"
tags = ["nitrogen", "spares", "capability"]
chronicle = "Thirty crates went into the collar. The N2 make-up log started a new page."
[[option.effect]]
q = "stocks.spares"
add = -30                       # a real sacrifice
[[option.effect]]
q = "menace.leak"
set = 2                         # back below the first band; the engine writes the crossing
[[option.effect]]
project = "reline_bearing"      # the work is a clock, not a sentence

[[option]]
id = "slow"
label = "Slow the drum"
chronicle = "The drum was slowed to one revolution a minute to spare the seal. Everyone who lives in it noticed."
[[option.effect]]
flag = "drum_slowed"            # later scenes read this
[[option.effect]]
q = "menace.leak"
add = -3
[[option.effect]]
all_strain = 0.05

[[option]]
id = "accept"
label = "Accept the loss"
chronicle = "The board wrote the leak into the budget and moved on. The ECLSS Lead wrote the number down and said nothing."
[[option.effect]]
q = "stocks.nitrogen"
add = -800
[[option.effect]]
q = "menace.leak"
add = -1.5                      # a must scene must move its own pressure, or it refires every cooldown
```

---

## 5. The tutorial specifically

The arc as it stands, with gates. Everything is `once`, priority ≥ 100, and fires in
this order because each scene gates on the last:

| count | id | gate |
|---|---|---|
| 1 | `tut_arrival` MM 1: Site Handover | `turn_min 1` |
| 2 | `tut_hand` MM 2: Crew Allocation | `fired tut_arrival` |
| 3 | `tut_dead_dex` MM 3: Non-Responsive Units | `fired tut_hand` |
| 4 | `tut_manifest` MM 4: Resupply Request, RSW-2 | `fired tut_dead_dex`, ≤ 10 |
| 5 | `tut_mind_log` MM 5: SX-19F Unscheduled Log | flag `mind_log` |
| 7 | `tut_keep_race` MM 7: CED Ledger and STE Forecast | `not_flag keep_dug` |
| 12 | `tut_first_rsw` MM 12: RSW-1 Arrival | `counts_since_convoy ≤ 1`, 12–18 |
| 13 | `tut_rsw_empty` MM 13: RSW-1 Off-Nominal | `missed_convoys ≥ 1`, not_fired first_rsw |
| — | `tut_driver_aligned` MDLS Alignment Complete | flag `driver_aligned` |
| 24 | `tut_driver_still` MM 24: MDLS Alignment Overdue | `not_flag driver_aligned` |
| — | `tut_keep_done` KEEP Excavation Complete | flag `keep_dug` |
| 16 | `tut_review` MM 16: First Quarterly Review | 16–19 |
| 27 | `tut_contract` MM 27: First Contract Expiry | 27–32 |
| 28 | `tut_expansion` MM 28: RSW-2 Arrival and Crew Augmentation | `counts_since_convoy ≤ 1`, 28–34 |
| 43 | `tut_handoff` MM 43: RSW-3 | 43–50; sets `tutorial_done` |

The engine, in the tutorial scenario, sets `tutorial`, `driver_unaligned`, `no_keep`,
`mind_log`, and opens `dig_keep` and `align_driver`; while `tutorial` is set and
neither `tutorial_open` nor `tutorial_done` is, only priority ≥ 100 scenes fire.
Flags the engine honours from content: `manifest_throughput|balanced|capability|people`
(set the manifest control at once and shape the next convoy), `throw_ship|hold|stop`
(set the throw position), `open_project:<id>` (open a manual project), `tutorial_open`
(let ordinary act-1 scenes fire from that count — set it in the RSW-1 arrival scene
so the middle of the tutorial is not silent), `tutorial_done` (end the scripting). A
completion scene must `unflag = "driver_unaligned"` when the driver is aligned.

**Rules for a tutorial scene.** One idea. One control the text points at, named the
way the page names it ("drag a line onto the KEEP clock", "the manifest mark on the
RSW clock"), and then the consequence teaches — do not narrate the consequence. One
seat introduced when its question first arises; the text should say what question
the seat answers, once. The liaison coldest. Every number with its unit. The
chronicle line dated `MM n.` and true whether or not the player did what the text
suggested.

**Do not repeat what playtest 1 found.**
- *Allocation narrated but not enacted.* "Free allocation split: BOP and KEEP" was
  written while no die moved. Either set `open_project:*` / a flag the engine reads,
  or write the line as the board's instruction ("SMB directive: free allocation to
  KEEP excavation") and let the page show whether it happened.
- *The ring full at MM 1.* The text promised seats would appear; six were already
  there. Until the engine gates seats on flags, do not promise it.
- *No missed-RSW scene.* RSW-2 missed at MM 28 and the most consequential event of
  the run got one grey line. Every window in the arc needs an arrival scene *and* a
  no-arrival scene gated on `missed_convoys`.
- *State-contradicting lines.* "The MDLS is not aligned" six counts after alignment;
  "at a third for a year" beside a rate-1.00 plant; RSW-1 delivering the "RSW-2"
  request. Gate on the flag or the quality that makes the sentence true, and name
  the window the engine names.
- *The review scene after the review.* Fire it on `sponsor.counts_to_review` ≤ 2,
  not on a turn number.

---

## 6. Projects

A project is a clock the player fills with dice. One TOML per project under
`data/projects/`; the format is in `storylet-format.md`. Decide first whether it is
**standing** (never completes; its pips are a rate — the bake-out, the throw) or
**one-time** (segments; completes; may repeat with `repeat_segments`). Give it a
`domain` (the skill that sets a die's face), say whether it is `structured` (any
robot fits; otherwise only dex units), and in the tutorial register a `title` and a
one-line `description` the rail shows on hover.

A scene opens a project when the decision *is* the work: "re-line the collar" opens
`reline_bearing` with `project = "reline_bearing"`; a project with `[[when]]` opens
itself when the state says so (the relay below 0.7, the leak at 4); `manual = true`
projects open only from scenes. Six to eight clocks open at once is the ceiling —
if the rail needs a scroll bar, the content has too many projects open.

Completion is a flag (`on_complete_flag = "keep_dug"`), and the **completion scene**
is a storylet gated `flag = "keep_dug"`, `once = true`, priority ≥ 100 if it must
interrupt: it is where the ring decides what the finished thing means (who moves
underground; whether the Wright who re-lined the collar gets a seat). The engine
writes only `KEEP excavation complete.`; everything the player will remember is in
your scene.

---

## 7. Checklist

Before you report, verify all twelve:

1. `cargo run -q --release -p space-sim -- validate` passes (unknown qualities, seats,
   skills, and uncast roles are rejected there).
2. Under `montecarlo --games 100 --policy random`, every storylet fires and every
   option is chosen at least once; a once-scene under 0.2 per game is fine, zero is a
   bug. (Rebuild first — content is bundled at build time.)
3. Read each chronicle line as if you chose that option and nothing else happened:
   is it true?
4. No engine line from §1 is restated in a scene.
5. The register matches the act: acronyms and MM in acts 0–1, Voidborn words only in
   act 2 and only after their trigger.
6. Flags are spelled as the engine expects (`manifest_capability`, `throw_hold`,
   `open_project:dig_keep`, `tutorial_open`, `tutorial_done`, `keep_dug`,
   `driver_aligned`); a misspelled flag fails silently.
7. No `{role}` in a label, text, or chronicle line that the scene does not cast.
8. Texture scenes have `cooldown` and a `weight` under 1; founding scenes have
   `once = true`.
9. Every option has some seat that would argue for it; two to four authored advice
   lines, in voice, certain.
10. The scene fires when its `when` says: trace one game (`run --seed N --trace`) and
    find it at the count you expected.
11. Seat roles are cast before skill roles.
12. You played it once yourself — `space-sim play --seed N` in the terminal, or
    `web/test/play.py` against the page — and read your own chronicle line in the
    drawer.

## Engine note on seats (added after the guide was written)

During the tutorial (`tutorial` set, `tutorial_done` not), a seat exists only once a
scene has set the flag `seat:<key>` — `seat:hulls`, `seat:hours`, `seat:extraction`,
`seat:bodies`, `seat:air`, `seat:liaison`. Introduce a seat by setting its flag in
the scene where its question first arises; until then the ring column shows only the
seats already asked. After the handoff every act-1 seat exists regardless.
