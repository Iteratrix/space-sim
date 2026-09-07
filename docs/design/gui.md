# The GUI

One design, proposed for building. A web front (Rust compiled to wasm, the `sim`
crate linked directly, no server), laid out for a desktop browser and reflowed for
a phone in portrait. The organising idea, in one sentence: **the screen is a
ring around a chronicle, and everything else is a clock — some clocks count down
at you, and some you fill with people.**

Two kinds of clock, and the whole game is the race between them. **Countdowns**
tick on their own: the Earth window, the review, contracts, the reactor, the
licence's grace, conjunction, the Sun. **Projects** advance only when the player
assigns labour to them each count — "Dig the Keep", "Align the driver", "Re-line
the bearing", "Teach the reckoning", "Build a through-wall arm" — Citizen
Sleeper's clocks with the dice replaced by the roster. Each present adult is one
die per count, its face set by their skill in the project's domain and dulled by
strain; robots are dice that only fit structured projects and vanish as the fleet
decays. Subsistence (Salotti's upkeep) eats most of the dice before the player
sees them; what is left is the hand. Assignments persist as routines, so the
player re-deals only when something changes: a die dies, a die arrives, a clock
completes, a countdown gets close.

The resources pass settled the ledger: seven visible resources (**Water, Margin,
Spares, Power, Hours, the Throw, People**), three pressures as segmented rings
with named bands (**Suspicion, Grievance, the Leak**), and three standing
controls (**the manifest split, the throw position, the roster ranking**). This
design is built around those.

The reference points, and what each one is for:

- **King of Dragon Pass / Six Ages** — how little UI a rich simulation needs. One
  scene screen: a picture, a paragraph, the ring's faces and speech, a short list
  of choices. Management screens exist but you visit them; you *live* in scenes.
  The ring is both the playing piece and the interface.
- **Citizen Sleeper** — clocks as segmented rings, every clock on screen, so time
  pressure is legible without a number. A map of nodes with the text panel beside
  it. Persistence by clocks and flags.
- **Frostpunk** — two meters at the bottom of the world (hope, discontent) that
  everything else is in service of; the book of laws as the record of what you
  decided you were.
- **Fallen London / Sunless Sea** — storylet cards, qualities as the only state the
  player sees, menaces that rise toward a threshold you can watch.
- **Highfleet** — diegetic instruments: the radio, the map table, the ship's
  gauges. Information arrives the way it would arrive.
- **Terra Invicta / Kerbal** — the delta-v map and the porkchop, and the lesson
  that a player needs one slider with two labelled ends, not the plot.
- **Crusader Kings 3** — the event window (portrait, text, options with tooltips)
  and the character panel; and the caution that a hundred panels is a job.
- **Roadwarden / Wildermyth** — text first, map beside, the record as the reward.
- **Alpha Centauri, Dwarf Fortress** — cautions only: a datasheet is not a game.

What this design refuses: a resource bar of nine icons with numbers; a tech tree;
a base-builder map of the burrow; portraits with stat blocks. What it keeps: faces
and voices, clocks, a book.

---

## 1. The screen

### Desktop

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  FORTUNA STATION · count 41 · the fourth convoy is in 3        [☰] [Chronicle]│
├─────────────┬────────────────────────────────────────────────┬───────────────┤
│  CLOCKS     │  SCENE                                          │  THE RING     │
│             │                                                 │               │
│  ◔ Earth    │   The Question the Policy Was Written For      │  ┌───┐ hulls  │
│    window   │                                                 │  │ o │ Zara  │
│  ◑ review   │   Eli Ibarra has asked the ring for a child.    │  └───┘ Novak  │
│  ◕ reactor  │   Not permission, exactly; the sponsor's        │   "Take it    │
│  ○ contract │   medical policy says no children on station    │    to the     │
│    (11)     │   and does not say who would stop one. Yara     │    Bodies..." │
│  ◒ Sun      │   Haddad has the numbers: the Keep runs at      │               │
│             │   twelve millisieverts a year ...               │  ┌───┐ hours  │
│  ─────────  │                                                 │  │ o │ Vik    │
│  LEDGER     │   ▸ Read the policy aloud and say no            │  └───┘ Acht.  │
│  water  ▮▮▮▮│   ▸ Say yes, and write it down as ours          │   "Nine times │
│  air    ▮▮▮ │   ▸ Say nothing; the policy was for a coil ship │    in ten..." │
│  parts  ▮▮  │                                                 │               │
│  power  ▮▮▮▮│                                                 │  ┌───┐ bodies │
│  hours  ▮▮▮ │                                                 │  │ o │ Yara   │
│  ─────────  │                                                 │  └───┘ Haddad │
│  PRESSURES  │                                                 │   "Twelve a   │
│  suspicion ·│                                                 │    year is    │
│  grievance ·│                                                 │    less..."   │
│  the leak  ·│                                                 │               │
│  the sponsor│                                                 │  ┌───┐ air    │
│  ○○○○●●     │                                                 │  ┌───┐ liaison│
│             │                                                 │  ╎   ╎ (late) │
└─────────────┴────────────────────────────────────────────────┴───────────────┘
```

Three columns, fixed roles:

- **Left, the clocks and the ledger** (always visible, never scrolls). Five to
  seven clocks at the top, the resource ledger as short bars beneath, the
  pressures beneath that, the sponsor's stage as a track of dots at the bottom.
  Nothing here has a number on it by default. Hover or tap gives the number and
  one sentence ("Water: 388 t. The loops lose 9 t a count. The bake-out makes 12.").
- **Centre, the scene** (the only thing that changes every count). Title,
  situation text with the cast's names as quiet links, the options as a list.
  When there is no scene this count, the centre shows the count's events as a
  short chronicle fragment and a single **End count** button; the game never
  advances on its own.
- **Right, the ring** (always visible). One card per seat, top to bottom in a
  fixed order. Each card: a face, the seat's *question* as its label (never the
  office title in act 2), the holder's name, and their counsel for the current
  scene as one or two sentences. Cards do not reorder; a seat that appears is
  added at the bottom, a seat that goes silent stays in place and empties.

Header: outpost name, the count, one sentence that is the game's own summary of
what matters next ("the fourth convoy is in 3", "the reactor has a year", "no ship
came"), a menu, and the Chronicle button. The chronicle opens as a full-height
drawer over the centre column, not a new page.

On demand only (from the menu or by clicking the thing): the **Roster**, the
**Fleet**, the **Map**, the **Manifest**, the end-of-act **Summary**. These are
KoDP's management screens: you visit them; they are never required to play.

### Phone (portrait)

```
┌────────────────────────┐
│ FORTUNA · 41 · convoy 3│
│ ◔ ◑ ◕ ○ ◒   water▮▮▮▮ │  ← one strip: clocks as small rings, two bars
├────────────────────────┤
│ The Question the       │
│ Policy Was Written For │
│                        │
│ Eli Ibarra has asked   │
│ the ring for a child.  │
│ ...                    │
│                        │
│ ▸ Read the policy      │
│   aloud and say no     │
│ ▸ Say yes, and write   │
│   it down as ours      │
│ ▸ Say nothing          │
├────────────────────────┤
│ [Ring ▴]  [Ledger] [Ch]│
└────────────────────────┘
```

The ring becomes a bottom sheet: swipe up to see the seats' counsel for the
scene you are looking at, each seat one line with the face, the question and the
sentence. The clocks compress to a strip of five small rings; tapping the strip
opens the full clock panel. The ledger and pressures are a second sheet. The
chronicle is the third. Nothing in the phone layout is a different *thing* from
the desktop; it is the same three regions with two of them folded.

---

## 2. The ring

The ring is the interface, as it was in KoDP. Canon's rule — a seat is a question
and the holder is whoever the ring accepts the answer from — is shown literally.

**A seat card.**

```
┌────────────────────────────┐
│ ┌────┐  who answers for    │
│ │face│  the bodies         │
│ └────┘  Yara Haddad        │
│  ◆◆◆◇                      │
│ "Twelve a year is less     │
│  than some cities. The     │
│  policy was written for    │
│  a coil ship."             │
│                            │
│  favours: Say yes          │
└────────────────────────────┘
```

- **The question, not the title.** In act 1 the title is a hover ("medical
  officer"); the question is the label. This is what lets a seat change hands and
  kind without the UI changing shape: in act 2 the same card reads "who answers
  for the bodies — the Doseward".
- **Faces**, not portraits with stats. Procedural but stable per person (a seeded
  silhouette, hair, a scar, a suit collar for the Thin, a cataract's milk in the
  eyes once it comes). A face carries the dose ledger without a number: the
  milk-eyed elder is a fact of the picture.
- **Counsel** is one or two sentences in the holder's voice, then a single line
  naming the option they favour. Authored advice and generated advice are not
  visually distinguished — the generated line is written to read as speech
  ("Leans toward the second") and the point is that the player should not be able
  to tell which is which from the chrome. What distinguishes them is quality, and
  quality is the player's problem to learn.
- **Trust without a stat sheet.** The four diamonds under the name are the holder's
  *standing*, not their skill: they fill as the holder's favoured option turns out
  well over the game (the sim can score an option after the fact: did the thing
  the seat warned about happen?). Standing is the game's memory of whose advice
  worked. It starts at two for everyone. Skill-gated wrongness therefore shows up
  the way it should — the head of extraction with logistics skill 1 gives confident
  bad advice about hours, and over ten counts his diamonds on hours questions
  drain. Standing is per seat-holder, not per person, so a new holder starts fresh.
  Hover on the diamonds says which recent counsel earned or cost them.
- **Conflict in the ring.** When two seats favour different options, their cards
  each draw a faint line to their option in the scene (a KoDP touch: the ring
  points). When all seats agree, the option is quietly marked "the ring is of one
  mind", which in a game about a splitting society is a thing you will come to
  miss.

**A seat goes silent.** The Liaison's card does not disappear when attention
fails. It stays in its place and its face fades to an outline; its counsel line
becomes the count-late message when one exists ("The reply to the manifest of
count 25 has arrived") and then nothing. At the Silence the card is renamed by the
lexicon: "who answers to the sponsor" becomes "the Silence", the outline stays,
and the card's only content is the count since the last ship. A holdout can be
seated there by a storylet; the outline gains a face again and the label does
not change. Frostpunk's empty chair, kept on screen.

**A seat appears.** Machines, Children, Sky: a card slides in at the bottom of the
column with its question and no holder, and stays empty until the ring seats
someone — the storylet that creates the seat is also the one that fills it. The
first time this happens the game says, once, in the header: "The ring is asking a
new question." Nothing else explains it.

**Seats in act 2.** The offices rename through the lexicon (Hullmother, Ledger,
Doseward, Hearthwarden, Keep-mother, Operator, the Warden of the Text). The
Operator's card carries a second, smaller face: the mind it speaks for, with the
mind's name once it has one. When the Operator's advice drifts after embodiment
the card gets the single most important line in the act-2 UI, from the weird
pitch: *"I am biased. I told you I was when you seated me."* shown as a permanent
footer under the Operator's counsel.

---

## 3. Scenes

A scene is a CK3 event window and a KoDP scene at once: text, cast, options,
counsel, and then its line in the chronicle.

**Arrival.** A new scene does not pop up; the centre column *turns to it* — the
previous count's events scroll up out of view, the title settles. A must scene
(priority ≥ 100) announces itself only by its title being set in the chronicle's
display face (the heavier serif) and by the clocks column dimming everything but
the clock that caused it (the leak's pressure, the licence's grace). No red
border, no siren. The player should learn that the quiet ones are the bad ones.

**Text.** The situation paragraph, 80–160 words, with cast names as quiet links.
Clicking a name opens that person's card over the ring column (face, estate,
tenure, the seats they hold, their ties to the other cast as three words each:
"relies on", "cannot work with", "would sail with"). No numbers on the card
except the dose ledger, which is shown as it would be shown in-fiction: a mark
count on the forearm in the face art, and a number on hover.

**Options.** Two to five, as a list, each a label and a one-line description. No
predicted effects. This is the KoDP rule and it is load-bearing: the counsel is
where you learn what an option does, and the counsel can be wrong. The single
concession: an option gated by a `requires` that fails is shown greyed with the
requirement in plain words ("needs spares"), because a player must be able to see
the door they cannot open.

**Counsel** lives in the ring column, not under the options — the seats speak
from their chairs. The lines between cards and options (§2) do the pointing.

**After the choice.** The options collapse; the chronicle line is written into
the scene in the chronicle's face, with the count; the ledger and pressures
update with a half-second ease so a jump is visible; any clock that moved ticks.
Then either the second scene of the count turns in, or the **End count** button
appears.

**Two scenes in one count.** Sequenced, never side by side. A small "and then" rule
between them in the centre column. The must scene always comes first.

**No scene.** The centre shows the count's engine events as the chronicle would
record them ("A convoy: 45 t landed..."), and End count. About a third of counts
should be like this once the director is retuned toward KoDP density; the quiet
counts are where the clocks do the talking.

---

## 4. Clocks and windows

Citizen Sleeper's insight is that a clock does not need a number if its segments
are on screen. Every clock here is a ring of segments, filling clockwise, with a
symbol in the middle and a one-word label. Full means the thing happens.

```
   ◔ Earth window        segments = counts until the window opens; fills, then
                          the ring turns solid for the counts the window is open,
                          then empties and starts again. A ship glyph sits in the
                          centre only while a convoy is actually inbound.
   ◑ review              16 segments; fills to the next review.
   ○ contract            the nearest contract end; the number of people on it
                          in the centre.
   ◕ reactor             fills over the core's life; red segments for the last
                          three years. Then the symbol becomes the Sun.
   ◒ the Sun             the eleven-year cycle drawn as a single slow ring; the
                          bright half is the storm half.
   ◐ grace               appears only while the licence is in grace: two segments.
   ◌ conjunction         appears only while Earth is behind the Sun: a hollow ring.
```

Later clocks slot in below: **Mars** (3.7 years), **Ceres** (22 years, drawn
mostly empty for most of a life), **Vesta** ("once"), a family's **passage**.
A clock that will not fill in the player's lifetime is drawn anyway; that is the
point of it.

**The porkchop is never shown.** What the player needs from it is exactly two
facts: when the window opens, and what it costs to go now versus then. The first
is the clock. The second is the **Map**, on demand:

```
┌─────────────────────────────────────────────────────┐
│  THE MAP                                            │
│                                                     │
│        · Vesta (73 yr)                              │
│                     ○ Ceres  ◔ 22 yr                │
│   ● Fortuna ──────────────── Earth ◔ 3 counts        │
│      ╲ Massalia (passes, years 19-22)               │
│       · Lutetia (fixed, ahead)                      │
│                                                     │
│   Fortuna → Earth                                   │
│   cheapest ●────────────────────────○ fastest        │
│   9 counts, 3.6 km/s          4 counts, 8.1 km/s     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

An orrery, not to scale, with the hubs as dots, each labelled with its clock. The
lines are the edges the polity can currently take; grey lines are edges it cannot
(no hull with the delta-v). Selecting an edge gives Terra Invicta's one slider:
cheapest to fastest, both ends labelled in counts and km/s, computed live by the
Lambert solver for the current count. No colour plot, no axes. The slider is the
porkchop, collapsed to the one line a person ever reads off it.

**Two currencies, once hulls exist.** The slider gets a second, quieter scale
under it: the propellant it will burn in tonnes at the left end and the sail-time
it will take at the right, so that a sail hull's slider is the same slider with
the fast end greyed ("no sail can hurry"). Patience-is-propellant is then a thing
the player *sees* every time they move the handle: pull left and the tonnes fall
and the counts rise. The phasing law inside a neighbourhood appears as a third
choice on the same slider ("wait one orbit / three orbits") when the destination
is a neighbour rather than a hub.

**What the clocks column does during a count.** When End count is pressed the
clocks tick in sequence over about a second, in the order the engine resolves
them: the window, the Sun, the reactor, the contracts, the review. A clock that
completes flares once and its symbol changes if it has a next state. This is the
whole turn animation; there is no other.

---

## 5. Resources and pressures

The engine tracks eleven stocks. The screen shows five, as bars without numbers,
and the rest live behind them. This assumes the resources-playability pass lands
on something like the following aggregation; if it aggregates differently the
column simply shows its five.

**The ledger (assumed aggregation):**

| bar | aggregates | what "low" means |
|---|---|---|
| **water** | water | the tank against the reserve line |
| **air** | nitrogen, with the leak's share drawn as a hatched tail | the farm's floor |
| **parts** | spares, medicine, and the closure they maintain | the vitamin dependence |
| **power** | capacity against demand, the reactor's share shaded | the farm's priority; the driver stalls first |
| **hours** | labour capacity against demand, robot hours shaded | the deficit that raises strain |

Propellant, boron, helium, food margin: hidden until a storylet or a fleet makes
them matter; then they appear as a sixth or seventh bar with a small "new" mark,
the way a Frostpunk resource appears when the building that uses it is built.
Throughput (φ) is not a bar at all; it is the sponsor's number and lives on the
sponsor's track.

Each bar is a horizontal strip of six cells that fill left to right; the reserve
line (water's 120 t, nitrogen's 200 kg) is drawn as a notch. A bar draining below
the notch pulses once per count. Hover: the number, the count's net change, and
the one sentence that says why ("The loops lose 9 t a count; the bake-out makes
12; the throw takes 10 above the line.").

**Emergency reserves.** If the playability pass adopts "break open the reserves"
on first exhaustion instead of consequences, the ledger shows it as the notch
moving: the bar empties to the notch, a scene fires, and the notch drops to zero
with a bracket labelled "reserve, opened count 61". The second exhaustion has no
notch to move. The UI's job is to make the first one feel like a door that only
opens once.

**Pressures.** Three or four menaces, drawn as Fallen London menaces: a short
vertical track of ten cells with a threshold mark at seven, filling upward.

```
   suspicion   grievance   the leak   (reactor wear appears in the reactor's last years)
      ·           ▮           ▮
      ·           ▮           ·
      ─ 7 ─       ─ 7 ─       ─ 7 ─
      ▮           ▮           ▮
      ▮           ▮           ▮
```

They are labelled with the polity's words (the lexicon renames "the leak" to "the
weeping collar" when the content does). Reaching the threshold is the must scene;
the UI does nothing at the threshold except let the scene come. Frostpunk's lesson
is that two meters the whole game is about are better than ten; here they are
grievance (inward) and suspicion (outward), and the leak is the physical one that
will decide the spin.

**The sponsor's track.** The seven stages as dots along the bottom of the column,
the current one filled, the ones behind it dark. No labels until a stage is
reached, then the stage's chronicle line is the hover. A player who has not read
the design will see six dark dots ahead and understand that this ends.

---

## 6. The chronicle and the lexicon

The chronicle is the game's primary artefact — the thing the player made — and
the GUI should treat it like KoDP's saga and Wildermyth's storybook: the reward
for playing, readable on its own.

**Presentation.** A drawer over the centre column, full height, the counts as
margin numbers, engine lines in a lighter weight and storylet lines in the
chronicle's face. Convoys and the Silence get a rule across the page. It is
searchable by name (click a name anywhere in the game to see their lines). It is
exportable as a text file, which the engine already produces.

**The lexicon.** Words drift in the chronicle as the engine's `render` already
does, and the UI does three more things:

1. **Hover restores.** Any drifted word is set in a faintly different ink; hover or
   long-press shows the sponsor-era word beneath it ("count · *month*"). The
   player can always get back to the old word. They will do it less and less.
2. **The chrome drifts too.** The clock labels, the seat questions, the pressure
   names and the End count button are all rendered through the same lexicon. The
   first count after the first Silence, the "Earth window" clock reads "the
   convoy" and the button says "End count". By act 2 the interface is written in
   the polity's words and the player learned them by reading the chronicle. This
   is the whole culture-shock mechanic delivered by string substitution; it costs
   nothing and it is the thing people will talk about.
3. **Older entries do not re-render.** A chronicle line is rendered with the
   lexicon of the count it was written in and frozen. Scrolling back up the
   chronicle is scrolling back into the old language. (The engine renders at
   display time today; the front should snapshot the vocabulary per entry.)

**The end-of-act summary** is a page of the chronicle in a different face: the
polity's state as the engine's `report::summary` already writes it, set as a
ledger page with the ring's final faces across the top and the lexicon's word
list at the bottom under the heading "Words in use". It is the save's cover page
and the thing a player screenshots.

---

## 7. Fleet and automation over time

Act 1 has one hull (the one that brought you), a robot fleet, and two minds. Act 2
has hulls that leave and come back, a fleet that decays, and minds that are
people. The Fleet screen has to grow from a footnote to a second map without
changing shape.

**The Fleet screen** (on demand):

```
┌──────────────────────────────────────────────────────────────┐
│  THE FLEET                                                   │
│                                                              │
│  HULLS                                                       │
│   ▣ Fortuna Station (the Keep)         spin 3 rpm · 0.5 g    │
│   ▢ SX-19F "Tollan" · coil hull        at the Keep · ready   │
│      ◆◆◆◇ embodiment    ◔ licence: grace (2)   units 62%     │
│   ▢ skiff 2 · no mind                  outbound → Nysa · ◔ 4 │
│                                                              │
│  ROBOTS      dex ▮▮▮▮      arm ▮▮▮▮▮▮    haul ▮▮▮▮▮▮▮        │
│              plant ▮▮▮▮▮▮  through-wall ▮▮                    │
│              "the Wrights keep the arms alive; the dex will   │
│               not be replaced"                                │
│                                                              │
│  ROUTINES                                                    │
│   bake-out     [ full | steady | idle ]   14 t/count          │
│   the throw    [ ship | hold at reserve | stop ]              │
│   the loops    [ spares first | let it decay ]                │
│   the skiffs   [ crews of 10% | 5% | grounded ]              │
└──────────────────────────────────────────────────────────────┘
```

**Hulls** as a list, each a line: name (the mind's name once it has one, the
designation before, "what it lost" for a blind hull), class, where it is, and its
clock if it is in transit (a small ring: counts to arrival). A hull's row expands
to its card: delta-v left as a bar, propellant and sail as the two currencies,
the crew as faces, and the mind.

**Minds** are shown on their hull, never as a separate list; a mind without a hull
is a card in the Keep. Three facts, three glyphs: **embodiment** as four diamonds
(the same mark as a seat's standing, on purpose — a mind earns its place the way a
person does); **licence** as a state word with the grace clock beside it when
running; **units** as a percentage that only ever falls. Naming is an event, not a
control: when the storylet names a mind, the Fleet row's designation crossfades to
the name and the chronicle line is the one that says so. After embodiment passes
0.5 the diamonds lock (a small padlock: this mind cannot be copied) and the row
gains the line "cannot be backed up", which is the most important sentence about
machines in the game and should be in the UI where a player will reread it.

**Robots** as five bars with one sentence from the Wrights under them. Bars only
ever shorten, except for dex and arm on convoy days, and the sentence changes as
classes fall (the content already has the scenes; the sentence is drawn from the
last one that fired).

**Routines.** These are the only direct controls in the game, and they should stay
few: four or five three-way toggles that set the engine's automatic behaviour
between scenes. They are what "automation" means to the player — not a
programming interface but standing orders. In act 1 the sponsor sets most of them
(greyed, with the sponsor's choice shown: "skiffs: crews of 10% — sponsor
policy") and the manifest scene is the one the player owns. As the sponsor's
attention fails, toggles unlock one at a time, each unlock a chronicle line ("Nobody
on Earth is setting the bake-out rate any more"). By act 2 the player holds them
all, and the Hands and Wrights appear as two more rows with their own standing
diamonds. A routine that has not been touched for a long time gets the routine's
holder's face beside it: the polity does this now, not you.

This is deliberately less than Frostpunk's law book and much less than a
Rimworld work-priority grid. Canon's automation story is that the belt makes hands
before minds and that robots do not repair robots; the UI for that is a bar
shortening under a sentence, not a scheduler.

---

## 8. The tutorial

The proposed vertical slice is the arrival: robots went first and did what they
could; the first crewed hull arrives at a partly built automated site. The
tutorial is not a separate mode — it is act 0, a short arc of authored scenes with
the same screen, in which the UI's regions light up one at a time as the fiction
introduces them.

**Count 0 — the approach.** The screen is dark except the centre column and one
clock: the hull's own arrival ring, nearly full. The scene is the last day
aboard: the ring is five faces who have been in a can together for nine months,
and the scene asks one question (who goes down first) whose only purpose is to
make the player click a name, see a card, and choose. The right column lights as
they answer: this is the ring.

**Counts 1–3 — the site.** The robots' work is presented as the engine's events
in the centre: what the automated effort built, what it did not (the arms are
dead; the through-wall unit is the only one working; the bake-out plant is
running at a third). The ledger lights bar by bar as each is explained by a
seat: water when the Air seat reads the tank, hours when the Hours seat counts
the arms. The first must scene is the first routine: the bake-out toggle unlocks
with the Extraction seat's counsel, and the Fleet screen opens for the first time
with three rows.

**Counts 4–8 — the first burrow.** A capstone-style scene sequence: where to dig,
how deep, whether to spin. Each is a scene with counsel that disagrees, and each
lights a pressure: the leak appears with the collar; grievance appears when the
first watch schedule is argued. The reactor clock appears when the core is
landed. The player is not told what a pressure is; the Bodies seat says "watch
the collar" and the bar is there the next count.

**Counts 9–14 — the window.** The Earth window clock lights with a ship glyph:
the sponsor's first convoy is inbound. The manifest scene is the tutorial's
centre — the request for the *second* convoy has to go up before the first has
landed, which is the game's whole logic of lag in one decision. The review clock
appears with the Liaison's card, whose counsel is visibly a count late ("They ask
how the arms are. The arms were dead when we landed.").

**Counts 15–18 — the first payload.** The throw is assembled; the routine
unlocks; the first pods go; φ appears on the sponsor's track as the one number.
The Sun clock lights during a storm scene. The arc ends with the convoy landing —
the clocks column ticking through its whole sequence for the first time — and the
first chronicle drawer opening on its own, showing the eighteen lines the player
wrote. Then the header says, once: "This is the chronicle. It is the only thing
that will be left of you." And the game continues into act 1 without a break.

Nothing in the tutorial is explained by a tooltip that would not exist later; the
seats do the explaining, in character, which is how the ring earns its standing
diamonds in the first hour. If the resources pass adopts emergency reserves, the
tutorial's one deliberate scarcity (water at the reserve line in count 6) is where
the player learns that the notch moves once.

---

## 9. Build plan

**The smallest slice worth building.** One screen, desktop layout, with: the
clocks column (Earth window, review, reactor, contract, Sun), the ledger as five
bars with hover, the three pressures, the sponsor's track, the scene column with
options and End count, the ring column with cards and counsel, and the chronicle
drawer. No Map, no Fleet, no faces (a seeded two-colour glyph in place of each
face), no lexicon hover. That is act 1 as it exists today, playable in a browser,
and it is perhaps three days of front-end work against the existing engine.

Then in order: faces; the lexicon hover and drifting chrome; the Map with the one
slider (the Lambert solver is already in the wasm); the Fleet screen with
routines (needs the engine's routines, which do not exist yet — they are the
parameters the sponsor currently sets); the phone reflow; the tutorial arc as
content plus the region-lighting choreography.

**The wasm / page split.** The `sim` crate compiles to wasm with `wasm-bindgen`
exposing five functions and nothing else:

```
new_game(seed) -> handle
advance(handle) -> JSON        // the same object play --json prints: status, events, firings, ending
resolve(handle, firing_id, option_id) -> JSON   // { chronicle }
view(handle) -> JSON           // qualities, clocks, ring seats, ledger, pressures, lexicon triggers,
                               // chronicle entries with their per-count vocabulary snapshot
save(handle) -> JSON / load(json) -> handle
```

`advance` and `resolve` exist as the JSON protocol already (`docs/design/agent-
protocol.md`); `view` is new and small — it is `report::summary` returned as
structure rather than prose, plus the calendar slice the clocks need and the
`transfer::best_at` call for the slider. The page is plain TypeScript with no
framework: three columns, a drawer, a handful of SVG rings. The state lives in the
wasm; the page is a renderer and an input queue. Saves go to `localStorage` as the
engine's JSON, which the CLI can also load — one save format, both ends.

**What the protocol already gives the page for free:** the firing's rendered text
with names filled in, the available options with tags, the counsel with the
favoured option index, the chronicle line per resolution, the ending and the
summary. What it does not yet give, and the engine should add before the page is
built: the calendar slice (open/closed per count for the next 60), per-entry
lexicon snapshots, the ring's seats as a list with the question and the holder,
and a `view` of the ledger with per-count deltas and the one-sentence "why" for
each bar (those sentences are the engine's to write, since only it knows the
flows).

**Two rules for whoever builds it.** No number is shown that a hover could show
instead. No panel is added that a scene could deliver instead.
