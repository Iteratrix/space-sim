# 09 — Game precedents: event authoring and legible orbits

Status: complete. Sections per BRIEF.md. Where a primary source was unreachable
(Paradox wikis and forum, Pavonis forum, Hooded Horse wiki all bot-block), the
claim is marked *(well-documented, source blocked)* and backed by a first-hand
script sample pulled from GitHub or by a secondary page.

## 1. Findings

### 1.1 King of Dragon Pass / Six Ages: the clan sim, OSL, advisors, heroquests, endgame

**Architecture.** KoDP is three layers, communicating mostly downward: a UI
layer (mTropolis in 1999, UIKit on iOS; roughly "50 screens and dialogs"), a
scene layer of 500+ interactive scenes written in OSL and *compiled to
bytecode*, and a C++ engine that interprets the bytecode, runs the economic
model ("tracks food, population, cattle, etc. for every clan"), manages clan
relationships, and serializes saves. Dunham's own numbers for the iOS port:
the engine needed minimal change, the scene layer was "highly portable", and
rebuilding the UI took about half the port effort. One architectural regret
he names: "Saved games assume a specific number of scenes and treasures", so
adding content broke save compatibility on the Windows build. Lesson for us:
the save format must key content by stable id, not by index.

**OSL (Opal Scripting Language).** Designed in 1997 for a non-programmer
writer. Deliberately not Turing-complete: "multiple responses and advice",
"simple branching", *no functions*. Variables are global; single-letter ones
reset to 0 at scene start; values are floats, strings, or references to game
entities (person, clan, tribe) with dotted attribute access
(`otherClan.chief`, `c.leadership`). Lists and chainable list functions exist:
`c = StrongestMilitary(ClansWithPositiveAttitude(NeighboringClans))`. Text
has placeholders for casting and grammar (`<survivorClan.plural>`,
`<him/her>`) and *variation placeholders* for replay variety. The central
mechanic is the **test**: `test Bargaining vs. Bargaining d5 + 1 + n, bonus: b`
or `test Deception(expeditionLeader) vs Crankiness d6`. The leader with the
best relevant skill tests on behalf of the clan; outcomes are win/lose with
the margin stored in `q`. There are 15 skills: 7 base (Animals, Bargaining,
Combat, Custom, Leadership, Magic, Plants) and 8 composites (e.g. Diplomacy =
Bargaining + Custom, Strategy = Combat + Leadership).

**Anatomy of a scene** (from Dunham's worked example): precondition in
brackets (`[sora >= 0 AND .horses > 5]` — the NPC is alive and we have
horses); opening text with conditional variants; a `saga:` entry so the event
is recorded in the in-game chronicle; music; usually five responses, "none
obviously optimal or useless", some conditional, some opening further
`NewChoices`; each response applies immediate effects (`.cattle -= 5`,
`otherClan.attitude += 3`), may run a test, and writes saga text. Advice is a
list of conditional lines with priorities:
`[Animals >= 2] ... [Animals >= 3] ... [Elmal] ...`, where the number in
trailing brackets `[4]` is the response the line recommends.

**"Storylets with casting."** Emily Short's phrase for what KoDP did in 1997:
storylets whose roles (rival clan, questor, dead ring member) are cast at
runtime from the sim. Dunham's rule of thumb: prefer casting from leaders
already on the ring, because "players have typically had an opportunity to
learn a bit about their personality (as well as have invested in them)".

**Consequence tracking.** Not branches but state. Per other clan, KoDP
tracks: allied, attitude (the map colours blue→red from it), captives, favors
(numeric, mutual, "cancel out"), feud, proximity (neighbour / neighbour-of-
neighbour), raid record including streaks, *slights* ("grave offenses are
remembered for years, even by otherwise friendly clans"), trade route, and
tribute in either direction. Internally: farmer morale, weaponthane morale,
thrall morale, mood; at tribe level: tribal attitude, tribal mood, royal
jealousy, and a "kingship" score for "how well our clan is considered
suitable for leading others". Other clans have "limited AI" — they recover
warriors slowly after losses, run short of cattle to trade, keep treasures
you gave them — and Dunham is explicit that "it's not a simulation of Iron
Age magical economics. King of Dragon Pass tells a story." The sim exists to
make consequences persist and to cast scenes plausibly, not to be watched.

**Turn structure.** Five Gloranthan seasons per year, two macro actions per
season, then Sacred Time for allocating clan magic. A long game runs on the
order of 53 years; about 5 random scenes per year on top of seasonal and
triggered ones. Scene selection rules: preconditions must hold; "no scene
will randomly repeat for at least five game years"; "if a scene logically
can't repeat within one game, it won't"; heavy internal variation (the ring
member who dies is random, four possible deaths; flavour sentences drawn from
alternatives). Dunham's frame: the game is "storytelling motifs, which it
repeats in endless variation".

**Content volume.** 1,624 script objects: 464 code (non-interactive), 462
news, 614 scenes, 84 quest scripts; 544 distinct interactive events after
folding follow-ups. Six Ages: Ride Like the Wind planned 275 scenes, shipped
412, at ~402,000 words; Lights Going Out has ~36% more scenes. Original KoDP
took 33 months, Six Ages 46.

**Advisors.** The ring is both playing piece and UI. Advice is gated by
*skill thresholds* ("if no advisor meets the threshold, you won't be shown
that insight") and coloured by personality and cult; advisors "aren't
stupid", but "may tend to see only one side of certain issues". Six Ages
added quirks (Song Quoter, Xenophobe), advisor-to-advisor interaction, and
advisors who "initiate independent actions to further their own agendas".
Script form: `[Relandar Priority] The Buzzard People should try harder to fit
in. [4]`. Dunham's stated payoff: advice is "an effective way to ease you into
the lore, one frothing and sliding fact at a time", and players recruit ring
members for personality and advice quality, not just stats. Six Ages also
added in-game explanations for *why* a heroquest failed and "rationalized
success-chance calculations" because KoDP players "often didn't understand
failure reasons".

**Heroquests and endgame.** Seven heroquests are needed for the long win;
each is a ritual re-enactment of a myth with stations where the questor
chooses to follow or deviate, and tests are skill-gated (Uralda's Blessing
"is hard because the quester must have a good enough Combat skill"). Tribe
formation is diplomatic: enough neighbouring clans with positive attitude
must be persuaded to join, then a moot sets name, king, and laws (paid for in
tribute); kingship requires unifying the tribes and reconciling with the
Horse-Spawn. Dunham: "It's your relationship with the other clans that will
ultimately determine whether you can forge a tribe, and then unify the tribes
into a kingdom." Achievement telemetry: 59 tribal kings vs 22 queens among
players who reported. Player-data post: "losses greatly outnumber wins";
median mobile session 4.1 min. Six Ages' retrospective flags the risk on our
side: a victory condition tied to a plot twist "made it difficult to
communicate win conditions".

**Tooling (the part most relevant to us).** Six Ages: a scene compiler that
emits reports (all strings for spell-check, tag-usage reports, cross-build
diffs to catch typos); *tags* (`@help`, `@hurt`, `@tutorial`, `@actThree`,
`@endgame`, `@frequent`, `@infrequent`, `@triggerFromCode`, `@triggeringCode`,
over 100 in all) used for selection, for blessings that apply "+1 in scenes
tagged @foreigners", for the compiler to check that special-condition scenes
are triggered from exactly one spot, and for test setup (`@unitTestWithClan`,
`@unitTestWithPerson`, "ten tags so it can set up the right context"); a
**brute-force harness that exercises every response of every scene** (about 2
minutes for the whole game in 2016 "and getting worse"), which "can make sure
nothing crashes" but "can't find typos or situations where the wrong person is
mentioned"; the harness can be pointed at a set-up situation ("no chief"
found ~10 failures in untested scenes); a debug dialog to trigger any scene,
run script fragments, zero the food, get raided by trolls, or dump
`farmerMorale/warriorMorale/thrallMorale/mood`; crash reports carrying the
script log plus two saves. KoDP QA in 1999 was a paper notebook tracking
success and failure of every response, and Dunham notes they preferred
organic play to forced states because forcing "might introduce its own bugs".

### 1.2 Fallen London / Sunless Sea / StoryNexus: quality-based narrative

**The model.** Failbetter coined "quality-based narrative": all state is
*qualities* (integers) — inventory, skills, menaces, story progress,
reputations; a *storylet* is text plus *branches*, each branch has
requirements (quality ranges) and effects (quality changes), and content is
selected by filtering the whole database for storylets whose requirements
hold, then presenting them as a menu (location-pinned) or drawing them as
*opportunity cards* from a deck. Emily Short's summary of the strengths:
content "slots together organically without uniform coverage requirements";
order-independence ("three pieces of evidence in any sequence" without
combinatorial explosion); adding a storylet "creates fewer system-wide
effects". Weaknesses she names: "scaling challenges at both extremes" (thin
database is dull, huge database overwhelms the menu); *manual progression
bookkeeping* — the author hand-codes `A available → A complete → B available`
as quality values, "bookkeeping for functionality that many other systems
provide automatically"; and *time caves* if you branch without recombining.
Alexis Kennedy's early "narrative structures" posts describe a pattern
language of ~60 named structures on top of storylets — *ventures* (a chain of
storylets driven by one progress quality), *Mark of Cain* (an exile quality
that permanently locks a completed story), *Midnight Staircase* (grind a
preparation quality, then cash it in for variable outcomes — an explicit
attempt to make grind feel like choice), and social exchanges — and a
discovery that matters for us: "people really loved it when terrible things
happened to them"; players sought exile, prison and death because those were
the best stories. His principle: "while we control the actual chunks of the
story, the paths between them belong to the player alone."

**Numbers.** Fallen London's wiki lists 709 *pyramidal* qualities and 3,198
*discrete* ones, 1,758 quality-usage pages. Pyramidal qualities level up on
change points: reaching level L+1 from L costs L CP, so total CP to level L is
L(L+1)/2 — a built-in soft cap that shapes the grind curve. Challenges are
*broad* (chance = 0.6 × quality/difficulty, so 60% at parity, ~100% at 5/3
ratio) or *narrow* (50% at parity, ±10% per level, clamped ~10–90%); the UI
shows only a word ("chancy", "modest", "straightforward") until you learn the
bands *(well-documented on fallenlondon.wiki; formula pages blocked)*.

**Known weaknesses, stated plainly.** Grind: because progress is a quality
and qualities rise by repeating branches, the default failure mode is
repeating the same storylet dozens of times; Sunless Sea's port-to-port
economy inherited it and was the most common critique of that game, which
Failbetter later addressed with more directed "ambitions" in Sunless Skies.
Opacity: the player often cannot see which quality gated which card, so
consequences read as arbitrary; Short's Reigns note generalises the problem —
with many competing menace stats "consistently heading in any direction will
get you killed", which is anti-roleplay. Menaces (Wounds, Nightmares,
Suspicion, Scandal) are the genre's pacing device: a rising bad quality that
triggers a "must" storylet at a threshold (prison, asylum, exile) and resets,
i.e. a built-in tension-and-release loop.

**Tooling.** StoryNexus (2012–2016) was a web editor exposing exactly this
model to outside authors: define qualities, storylets, branches, requirement
ranges, effects, decks, areas; content was live-editable and the requirement
filter *was* the test. There was no offline simulator; balance was found by
play and by player-run wikis reverse-engineering the formulas. The
transferable lesson is that a system whose whole state is a typed key→number
map is trivially serialisable, diffable and fuzzable — the Failbetter data
model is the most machine-friendly of all the precedents even though
Failbetter never built the machine.

### 1.3 Crusader Kings 2/3 and Paradox event scripting

**CK2 (Clausewitz classic).** Events are blocks in text files: `id`, `desc`,
`picture`, `is_triggered_only`, `trigger = { ... }`, `mean_time_to_happen = {
months = N  mult_modifier = { factor = 0.75  owner = { war = yes } } ... }`,
`immediate = { ... }`, `option = { name  ai_chance  effects }`. A real sample
from CK2Plus (`ze_favor_events.txt`): `is_triggered_only = yes # on_yearly_pulse,
random_events`, a `trigger` over traits and voters, a `weight_multiplier` that
doubles for `gluttonous` or `hedonist`, `immediate` that picks and saves a
target (`save_event_target_as = target_voter`), then options. MTTH is an
exponential-ish per-check probability, evaluated for every candidate scope on
a fixed cadence, which produced two chronic problems the community documented
for a decade: cost scales with (events × characters) every tick, and the
"mean" is neither a guarantee nor a floor, so authors could not reason about
pacing ("MTTH 2500 months" in the prosperity sample above is a way of saying
"rarely, and I don't know when").

**CK3.** Dev Diary #30 (2020) dropped MTTH for character events; *every*
event fires from an **on_action**: a named hook the engine calls (on_birth,
on_death, on_yearly_pulse, on_marriage, on_title_gain...) or that script
calls. A hook lists `events = { }` (always), `random_events = {
chance_to_happen = 65  20 = government_conflicts.1  100 = character_events.6
... }` (a weighted draw with a no-event probability), `first_valid = { }`, an
optional `trigger`, and nested `on_actions`. The sample above is a Victoria 3
mod's yearly pulse, same engine family. Events gain `cooldown = { years = 5 }`,
`triggered_desc`/`first_valid` for conditional text, `theme`, portraits,
`weight_multiplier`, saved scopes (`scope:actor`), and scripted triggers/
effects/values for reuse. The engine ships `error.log`, a console `script_docs`
dump, `-debug_mode`, observer mode, and a `run` command for script files. What
this enables: hundreds of authors, thousands of events, cheap conditional
content, and predictable *cadence* (you know a yearly pulse fires yearly).
What it makes hard: no headless mode and no test runner — validation is
"launch the game and read error.log"; scope errors are runtime; balance is
found by playing. The community filled the gap with **CWTools** (a .NET
parser/validator with per-game config files, used by IDE plugins) and
**ck3-tiger** (loads vanilla plus mod and checks syntax, missing references,
missing localisation, scope-type consistency, even genealogical impossibilities
in history files; JSON output). The fact that two independent third-party
validators exist is the strongest evidence in this survey that *static
validation of event content is both necessary and feasible*, and that it must
know the game's scope/type system to be useful.

### 1.4 RimWorld storytellers and Dwarf Fortress: pacing emergent narrative

**RimWorld.** The storyteller is a small, entirely data-driven director. From
the wiki's transcription of `StorytellerDef` (Cassandra):
`minDaysPassed 11.0, onDays 4.6, offDays 6.0, minSpacingDays 1.9,
numIncidentsRange 1~2`, misc events `mtbDays 4.8`. So: a fixed intro (one mad
animal, one raider on day 5 hour 15), then from day 11 a square wave —
4.6 days "on" with 1–2 major threats at least 1.9 days apart, 6 days "off" —
repeating every 10.6 days regardless of saves or map, ~8.5 major threats per
60-day year. Phoebe lengthens the off phase; Randy replaces the wave with
pure MTB randomness (and players find him *easier on average, harsher in the
tail*). Threat *size* is decoupled from *timing*: `Raid Points = (Wealth
Points + Pawn Points) × Difficulty × Starting Factor × Adaption Factor`, with
wealth points linear from 0 at 14k storyteller wealth to 2,400 at 400k
(~1 point per 161 wealth; buildings count 50%), pawn points 15→140 per
colonist over the same range, floor 35, cap 10,000. The **adaption factor**
(0.4–1.47; starts 0.8; 30-day grace; grows on a curve every 12 h; loses days
when a colonist is downed or dies, scaled by population) is a "days since
last injury" sign that makes the game ease off after tragedy. Incident
selection is a weighted table per category with hard gates (infestations need
≥400 points; mechanoid weight 0 below 300), per-incident cooldowns (flashstorm
15 days, volcanic winter 140 days and not before day 60), and a
*PopulationIntent* curve that raises join-events when the colony is small
and starves them after ~20 colonists. Nothing here is emergent; it is a
tension curve, a difficulty budget, a weighted deck, and cooldowns — and it
produces stories because the *consequences* run through a rich sim.

**Dwarf Fortress.** Adams calls the game a "story generator"; the design
prioritises simulation over scripted content and a persistent generated
world (Legends mode, exportable to XML) so that stories survive the fortress.
Its pacing devices are nonetheless the same shape as RimWorld's, just
implicit: migrant waves scale with fortress wealth and reputation; sieges and
megabeasts are gated by created wealth and population; strange moods fire on
a timer; seasons bring caravans yearly. DF's known failure mode is exactly the
flat stretch the brief worries about — a mature, safe fortress becomes a
management screensaver until the player self-imposes goals — and the
community's answer (Boatmurdered-style succession play, self-set challenges)
is evidence that simulation alone does not pace. The legibility lesson is
Legends mode: emergent story only *reads* as story once the sim writes a
chronicle in prose with named actors and causes.

### 1.5 Terra Invicta, KSP, Children of a Dead Earth: making delta-v legible

**Kerbal Space Program.** Patched conics (no Lagrange points, perturbations,
or tides), and its legibility comes from three things: the map view draws the
predicted orbit *and* the encounter's conic inside the target's sphere of
influence; the **maneuver node** turns a burn into a draggable object with
prograde/normal/radial handles, a live delta-v number and burn time, and
closest-approach markers, so the player learns Δv by direct manipulation; and
the community's **delta-v map** poster (a subway map of Δv costs between
bodies) plus **Alex Moon's Launch Window Planner**: a Lambert-solver porkchop
plot with departure date on x, time of flight on y, colour = total Δv,
click-to-read ejection Δv, plane-change Δv, insertion Δv, phase and ejection
angles. Edge's line — "you'll crack a puzzle set by *reality*" — is the point:
the honesty of the physics is what makes the abstractions feel earned.

**Terra Invicta** *(dev diary and wiki blocked; from Steam discussion, the
search summary of DD#17, and play)*. Patched-conic solar system, ships with an
honest propellant/exhaust-velocity Δv budget and thrust-to-mass acceleration.
The entire orbital-mechanics UI is one **transfer planner**: pick a
destination, and a slider trades Δv against travel time along a continuum of
Lambert solutions; the planner also computes the departure date and a
position-at-time function so the ship's path is drawn. Low-acceleration ships
lose the slider and get "microthrust spirals" (an honest consequence:
you cannot do impulsive transfers at 0.2 milligee). The Steam thread shows the
design working and failing at once: an experienced player answers "use the
Transfer Planner in the intel screen to see how much Delta-V roughly you'd
need... ~70–110 [km/s?] to one of Jupiter's moons at 0.2 milligee", while the
new player could not tell "if my ship designs are at all acceptable" despite
a "Ship Performance Data" panel. The abstraction that works is *hiding the
porkchop behind a single slider whose two ends are labelled in the player's
currencies (propellant, days)*; what fails is showing raw Δv with no
reference class.

**Children of a Dead Earth.** Full N-body with a 4th-order symplectic
integrator, chosen after the developer found patched conics "diverge very
heavily" from N-body over the game's timescales; consequences: Lagrange
points and perturbations exist, Hohmann and phasing "are much harder to pull
off", and an alpha tester flattened an out-of-plane orbit for free using
Uranus's perturbation by turning stationkeeping off. The stated philosophy is
"a simulation first, and a game second. No amount of realism was compromised
to make things more fun." The result is the most honest and the least
legible of the three; players report needing external tutorials to plan a
rendezvous. CoaDE is the control case: honesty without abstraction produces
a niche audience. For us it argues for KSP-grade honesty (conics + Lambert)
with Terra Invicta-grade abstraction (one slider, two labelled ends) — and
against N-body, which buys nothing at belt scales for a turn-based game.

### 1.6 Compact turn/event structures with persistent consequence

- **Seedship** (Twine/SugarCube, 2017): 1,000 frozen colonists; a handful of
  ship stats (scanners, landing system, constructors, scientific and cultural
  databases, colonist count) each degradable; loop = jump → random event
  (usually "known penalty/reward vs unknown") → planet with visible
  attributes (green = healthy) → probe or land or move on. The ending is a
  generated prose history of the colony *computed from the final stats*
  (database damage lowers the colony's culture/science, etc.) plus a score.
  The whole design is ~30 variables and ~60 events and it is replayable; its
  strength is that every event's effect is visible on the stat panel within
  one turn. It is the smallest complete proof that "push your luck on a
  degrading vessel toward a destination you can't fully see" is a loop.
- **Citizen Sleeper** (2022, Blades in the Dark / Mothership lineage): each
  *cycle* you roll up to 5 dice, fewer as *condition* decays without
  stabiliser; dice are placed into actions with outcome bands by die value;
  *clocks* (segmented progress tracks, some ticking per cycle, some per
  action) run both your projects and the world's threats; *drives* are quest
  arcs with milestones. The sequel adds stress that breaks dice and a
  "glitch die" (80% bad / 20% best). Persistence is by clocks and flags; the
  time pressure is legible because every clock is on screen. Steal: clocks as
  the universal representation of "something is coming".
- **Roadwarden** (Ren'Py, 2022): a hard 40-day limit; per-settlement
  *attitude* and a reputation the player can inspect; survival meters (food,
  health, hygiene, money); the ending is a per-settlement ledger of what you
  changed. Critics noted some decisions "are not ultimately reflected in the
  ending" — the failure mode of writing consequences into prose rather than
  state.
- **Wildermyth** (2019–21): events are data (JSON, authored in the shipped
  in-game editor) with typed *roles* (HERO with filters on personality,
  relationship, hook, aspect, stat; `notAlreadyMatchedAs`), and *aspects*
  (facts about a hero: `missingLeftArm`, `hunter`) as the universal state
  hook; ~11 trigger classes each with a documented expected frequency
  (wilderness scouting 12–20/game; mid-mission "every other or every third
  combat"; mortal choice "when a hero falls to zero HP for the first time").
  Selection: "search the database of events for all the stories that could
  fit the situation, then pick one randomly" — with a decay rule: seeing an
  event makes it *one quarter* as likely in the next campaign; skipping it
  *doubles* it, capped at default. Outcomes are a fixed menu of mechanically
  simple effects plus a *history line* on the hero (the persistent prose
  memory). The writers' guide admits "branching events are currently pretty
  awkward" and most stories are a single choice; the design guide's rule —
  "any text that hits the player as direct narrative ought to serve the
  purpose of offering her choices" — is the right brief for our writers.
- **The Banner Saga** (2014): one currency (renown) buys both supplies and
  upgrades; supplies burn per day of travel; morale drifts with events; the
  caravan, not a hero, is the protagonist; no reload-on-defeat, characters
  die in dialogue choices. The single-currency squeeze is the cleanest
  "sponsor-vs-self" analogue in the set.

### 1.7 Testability and AI authoring

Ranked by how far each architecture already is from "an agent writes a
storylet, a validator rejects the bad ones, a headless sim measures the rest":

| System | State model | Content format | Static validation | Headless sim | Authoring tool |
|---|---|---|---|---|---|
| Fallen London / StoryNexus | flat typed qualities | storylet/branch records in DB | requirements are the schema | none | web editor |
| KoDP / Six Ages | globals + entity refs | OSL DSL → bytecode | compiler + tag reports | brute-force every response (~2 min) + fixture tags | text + compiler + debug dialog |
| CK3 / Paradox | scoped object graph | Clausewitz script | error.log; 3rd-party CWTools, tiger | none official | text editor + IDE plugins |
| Wildermyth | aspects on entities | JSON | in-editor | none public | shipped editor |
| RimWorld | Unity objects | XML defs | Unity load errors | none | XML |
| Seedship | ~30 vars | Twine passages | none | trivially possible | Twine |

The two architectures with real testing stories are Six Ages (exhaustive
response enumeration under fixture states, tags to set up context, compiler
reports as regression diffs) and Paradox-by-proxy (tiger/CWTools proving that
a validator needs the engine's type/scope model). No precedent runs headless
Monte-Carlo over content, which is the thing an AI-tested pipeline most
needs; the closest is Six Ages' "turn-by-turn economic data in reports for
balancing" from human playtests.

What an AI-writable, unit-testable storylet needs, derived from the above:

1. **Typed state, not free variables.** Failbetter's flat qualities are
   fuzzable but opaque; OSL's globals are what made Six Ages need a "no chief"
   fixture to find ten bugs. Declare every readable/writable field with a
   type and range; the validator rejects references to undeclared state.
2. **Declarative predicates and effects**, not code, for requirements and
   consequences (CK3 triggers/effects, StoryNexus requirements), with a tiny
   escape hatch for tests-with-margin (OSL `test ... vs ... d6`) that the
   engine implements, so a storylet cannot mutate state in ways the schema
   does not enumerate.
3. **Casting as part of the contract** (Wildermyth roles with filters;
   KoDP's prefer-the-ring rule): a storylet declares roles and the predicate
   each must satisfy; a static check proves each role is satisfiable from
   the declared entity types and a dynamic check reports how often it is.
4. **Triggers as named hooks with weights and cooldowns** (CK3 on_actions +
   KoDP five-year rule + Wildermyth ¼/×2 decay), never MTTH.
5. **Advice lines with gating and a recommended option index**, so a test can
   assert "an advisor with Engineering ≥ 3 recommends the option that the sim
   says is best in this state" — and separately assert the *biased* advisor
   recommends the wrong one.
6. **Effects expressed in sim units** (kg propellant, m/s Δv, person-days of
   food, attitude points), so headless runs can assert invariants like "no
   storylet creates mass from nothing" and balance can be measured, not
   guessed.
7. **A chronicle line** (KoDP `saga:`, Wildermyth history) as a required
   effect, so replays produce a readable transcript and a diff of two seeds is
   a diff of two stories.

### 1.8 Comparison of event-authoring architectures and recommendation

Four models were in play: (A) branching scripts (Twine, Banner Saga: easy to
write, combinatorially expensive, untestable beyond reachability); (B)
quality-based storylets (Failbetter: modular, order-independent, fuzzable,
but grindy and opaque, with hand-rolled progression bookkeeping); (C) hooked,
weighted, scoped events over a rich sim (CK3: predictable cadence, scalable to
thousands, needs a validator that knows the type system, no headless mode);
(D) storylets with casting on top of a sim, with a scene compiler and
brute-force response testing (KoDP/Six Ages: the closest to our brief and
the only one with a working test harness, but a bespoke DSL, global state,
and index-keyed saves).

**Recommendation: D's unit and test harness, C's trigger model, B's state
discipline, Wildermyth's data format, RimWorld's director.** Concretely:

- The unit is a *storylet with casting* (D), stored as data (Wildermyth-style
  JSON/TOML, or a small DSL that compiles to it — the compiler is where Six
  Ages got its reports and its tests; a data-first format gets them for free
  with a schema).
- All state a storylet may read or write is declared in a typed schema over
  the sim's real entities (people, vessels, habitats, sponsor, other groups,
  the map). Qualities for narrative-only progress (B) are allowed but must be
  declared too, with ranges and a category (progress / menace / reputation /
  flag), so the validator can enforce Short's bookkeeping automatically
  (e.g. a `progress` quality must have exactly one storylet that advances
  each step, and each step must be reachable).
- Firing is by *on_actions* (C): turn phases (`on_turn_start`,
  `on_season_end`), sim events (`on_vessel_arrival`, `on_supply_below`,
  `on_death`, `on_sponsor_message`), with weighted `random_events`, no-event
  chance, per-storylet cooldowns, and the KoDP/Wildermyth repetition decay.
- Above the hooks sits a *director* (RimWorld): a tension budget with on/off
  phases and an adaption factor that scales the *size* of threats by the
  settlement's wealth/capability and eases after losses, so pacing is a
  tunable curve rather than a property of the deck.
- Advice is a first-class field on each option (D), gated by advisor skill
  and coloured by personality tags, with a `recommends` index the tests can
  check.
- Tooling from day one: a validator (schema, references, role
  satisfiability, localisation placeholders, reachability), a headless runner
  that enumerates every option of every storylet under fixture states (Six
  Ages' brute force, but seeded and in CI), and a Monte-Carlo runner with a
  scripted "policy" player that reports per-storylet fire rates, per-option
  pick rates under the policy, resource trajectories, and act-transition
  rates — the numbers Six Ages only got from human playtests.

## 2. Numbers the sim needs

| Parameter | Value / range | Source | Use |
|---|---|---|---|
| Seasons per year / actions per season | 5 / 2 (+ Sacred Time) | KoDP | turn budget baseline |
| Random scenes per year | ~5 (plus seasonal/triggered) | KoDP "How Many Scenes?" | deck draw rate |
| Game length (long game) | ~53 years, several generations | KoDP | act sizing; ~265 random scenes seen |
| Scene non-repeat window | ≥5 game years | KoDP "Repeatability" | cooldown default |
| Cross-campaign decay | seen ×0.25, skipped ×2, cap 1.0 | Wildermyth | replay weighting |
| Distinct interactive scenes at ship | 544 (KoDP), 412 (Six Ages RLtW), ~560 (Lights Going Out) | KoDP, Six Ages | content target; Act 1 alone might need ~120–150 |
| Words | ~402,000 for 412 scenes (~975 words/scene incl. advice) | Six Ages | writing budget |
| Responses per scene | typically 5 | KoDP | option count |
| Advice lines | several per scene, skill-gated, one recommended option each | KoDP/Six Ages | advice schema |
| Leader skills | 15 (7 base + 8 composite) | KoDP | ring skill model |
| Test dice | d5/d6 opposed, margin `q` | KoDP | resolution |
| Brute-force test time | ~2 min for all responses of all scenes (2016) | Six Ages | CI budget ceiling |
| Storyteller cycle | intro day 5; on 4.6 d / off 6.0 d from day 11; ≥1.9 d spacing; 1–2 threats per on-phase; ~8.5 major/yr | RimWorld Cassandra | director curve |
| Misc event MTB | 4.8 days | RimWorld | background incident rate |
| Threat size | (wealth pts + pawn pts) × difficulty × start × adaption; wealth pts 0@14k→2,400@400k; 15→140 pts/colonist; floor 35, cap 10,000 | RimWorld | threat scaling |
| Adaption factor | 0.4–1.47 (start 0.8, 30-day grace) | RimWorld | mercy after loss |
| Population-intent minimum | 0.02% (Cassandra/Phoebe) vs 0.08% (Randy) per check | RimWorld | migration-event floor |
| Pyramidal quality cost | L CP to go L→L+1; L(L+1)/2 total | Fallen London | soft-cap skills/reputation |
| Broad challenge | p = 0.6 × q/d (cap 1) | Fallen London | soft skill checks |
| Narrow challenge | p = 0.5 ± 0.1/level, clamp 0.1–0.9 | Fallen London | hard skill checks |
| Named narrative patterns | ~60 | Failbetter | pattern library size to aim for |
| Heroquests to win | 7 (long game) | KoDP | number of "capstone" set pieces per act ~2–3 |
| Seedship state | 1,000 colonists, ~8 ship stats, binary-choice events | Seedship | minimal viable Act 1 vessel model |
| Dice per cycle | 5 at full condition | Citizen Sleeper | action-point analogue |
| Hard time limit | 40 days | Roadwarden | Act 1 sponsor-review horizon analogue |
| Porkchop axes | x = departure date, y = time of flight, colour = total Δv | KSP planner | transfer UI |
| Transfer slider | Δv ⇄ time along Lambert family; spirals when accel too low | Terra Invicta | transfer UI |
| CK3 pulse example | yearly pulse: chance_to_happen 65, weights 5–100 | Vic3 mod sample | on_action defaults |
| CK2 MTTH example | 2,500 months with ×0.75/×0.9 modifiers | CK2Plus sample | what *not* to do |

## 3. Implications for mechanics

Each: mechanic — evidence — act.

1. **Storylets with casting as the content unit.** Roles declared with
   predicates; cast from the sim, preferring people the player has already
   met (the ring). — KoDP/Six Ages; Wildermyth roles. — All acts; the schema
   is Act 1 infrastructure.
2. **On_action hooks, not MTTH, as the only firing path.** Turn-phase hooks
   plus sim-event hooks (`on_arrival`, `on_supply_below`, `on_death`,
   `on_sponsor_message`, `on_window_open`), weighted draws with a no-event
   chance, per-storylet cooldown ≥ N turns, cross-run decay. — CK3 DD#30,
   Vic3 sample, KoDP 5-year rule, Wildermyth ¼/×2. — Act 1.
3. **A director with an explicit tension curve.** On/off phases, minimum
   spacing, threat *size* from a capability/wealth score with an adaption
   factor that eases after deaths; misc events on an MTB. Make the curve a
   data file so acts can have different shapes (Act 2's "cut off" phase should
   front-load). — RimWorld StorytellerDef; DF's flat-stretch failure. — Act 1,
   retuned per act.
4. **Advice as a gated, biased, testable field.** Each option carries advice
   lines `[skill ≥ k]`, `[trait]`, `[faction]`, priority, and a recommended
   option; advisors below threshold say nothing; some advisors are
   systematically one-sided (sponsor loyalist vs. Voidborn) so "who to seat"
   is a real decision. Tests assert the competent advisor's recommendation
   matches the sim's expected value and the biased one's does not. — KoDP
   anatomy; Six Ages "Advice"; Six Ages' added failure explanations. —
   Act 1 (sponsor liaison vs. crew), Act 2 (factions), Act 3 (other groups).
5. **Consequences as state, never as branches.** Per-other-group ledger:
   attitude, favours (mutual, cancelling), slights (decay over years),
   feud/alliance, trade route, tribute, captives, proximity; internal morale
   by caste/cohort; a "suitability to lead" score for the Act 3 confederation.
   — KoDP "Friends-and-Relations". — Act 1 seeds; Act 2–3 pay off.
6. **Opposed tests with margin.** `test Skill(role) vs Difficulty dN`,
   winner/loser plus margin, with best-qualified-person-tests-for-the-group as
   the default caster. Show the odds word *and* the number (avoid Fallen
   London's opacity). — KoDP; Fallen London broad/narrow. — Act 1.
7. **Menace qualities with threshold "must" events.** Sponsor Suspicion,
   Crew Grievance, Radiation Debt, Reactor Wear: rising qualities that force
   a scene at a threshold and then reset — the genre's tension-release loop
   and a natural way to make "cut off" arrive as a *consequence*. —
   Failbetter menaces; Kennedy's "people loved it when terrible things
   happened". — Act 1 → Act 2 transition.
8. **Pyramidal costs for capability and reputation.** Level L→L+1 costs L
   points, giving diminishing returns without hard caps. — Fallen London. —
   All acts (capability ladder from report 03).
9. **Clocks for everything that is coming.** Transfer windows, sponsor
   review, consumable exhaustion, a rival's expedition: segmented tracks on
   screen, some ticking per turn, some per action. This is also the honest
   representation of light-lag: a sponsor message is a clock, not an instant.
   — Citizen Sleeper. — Act 1 (windows, sponsor), Act 2 (supplies), Act 3
   (rival fleets).
10. **One-slider transfers over a real Lambert solver.** Compute the porkchop
    honestly (conics + Lambert; not N-body); present it as a slider from
    "cheapest" to "fastest" with both ends labelled in propellant tonnes and
    days, a "next window in N turns" clock, and a Δv map poster of the belt
    as an in-game document. Show raw Δv only on hover. — KSP planner and Δv
    map; Terra Invicta slider and the confused-newcomer thread; CoaDE as the
    honesty-without-legibility control. — Act 1 (sponsor runs), Act 3
    (leverage is Δv).
11. **Δv as freedom, spirals as poverty.** A vessel whose acceleration drops
    below a threshold loses impulsive transfers and gets slow spirals — a
    legible way to make "the fleet is mobility is freedom" felt. — Terra
    Invicta microthrust rule. — Act 2.
12. **Single-currency squeeze for the sponsor era.** One sponsor-controlled
    budget buys both consumables and capability, so every capability purchase
    is visibly a bet against the next resupply. — Banner Saga renown. — Act 1.
13. **Capstone quests as skill-gated set pieces with explained failure.** Two
    or three per act (first ISRU closure, first unsponsored transfer, the
    confederation moot), each a multi-station scene where the caster's skills
    gate stations and failure explains itself. — KoDP heroquests; Six Ages
    failure explanations and win-condition regret. — All acts; Act 3 moot as
    endgame.
14. **A chronicle as a required effect.** Every storylet writes a saga line;
    the sim writes lines for births, deaths, arrivals; the end-of-act and
    end-of-game summaries are generated from state (Seedship). This is also
    the test transcript. — KoDP saga; DF Legends; Seedship ending. — Act 1.
15. **Tags on storylets for selection, blessings, and tests.** `@act1`,
    `@sponsor`, `@help`, `@hurt`, `@frequent`, `@triggerFromCode`,
    `@fixture:no_commander`; the validator checks that `@triggerFromCode`
    scenes are called from exactly one place; the test runner uses fixture
    tags to set up state. — Six Ages tags. — Act 1 infrastructure.
16. **Brute-force every option under fixture states in CI**, plus a seeded
    Monte-Carlo with a policy player that reports fire rates and resource
    trajectories; keep the whole suite under a few minutes. — Six Ages
    "Testing in Bulk" (2 min ceiling); Six Ages playtest economic reports. —
    Act 1.
17. **Content-id-keyed saves.** — KoDP's index-keyed save regret. — Act 1.

## 4. Open questions

- **DSL or data?** Six Ages' compiler reports and brute-force tests came from
  owning a DSL; Wildermyth's editor came from owning a JSON schema. For AI
  authors, data with a schema is easier to validate and to generate, but
  advice/variant prose is more readable in a DSL. A DSL that compiles to the
  data form (and a decompiler for round-tripping) is the likely answer; who
  owns the grammar needs a decision.
- **How much of the sim may a storylet touch?** Failbetter's answer (all
  state is qualities) is fuzzable but leads to grind; KoDP's answer (globals
  and entity attributes) needed fixtures to catch. A whitelist of writable
  fields per act, or a capability system for storylets, needs design.
- **Director vs. sim-driven threats.** RimWorld's square wave is a director
  imposing pacing; the brief's thesis wants threats to *be* the physics
  (windows, wear, solar cycle). Which threats are director-budgeted and which
  are purely emergent, and can the director only *schedule* what the sim
  already permits?
- **Advice truthfulness.** Should the "competent" advisor be computed from
  the sim's actual expected value (an oracle, testable) or hand-written
  (authored, cheaper, drifts as the sim is tuned)? Six Ages hand-writes;
  an oracle would be novel and is what makes advice unit-testable.
- **Terra Invicta's slider behind the porkchop is unverified from primary
  sources** in this session (forum and wiki blocked); confirm the planner's
  exact inputs before copying it.
- **Fallen London's formulas** are community-derived; fine as design
  inspiration, but do not cite as Failbetter's.
- **Endgame legibility.** KoDP's tribe moot and Six Ages' twist-based win
  both had communication problems; the Act 3 confederation needs a visible
  score (KoDP's "kingship") from Act 1 on.

## 5. Sources

- KoDP blog, "OSL (King of Dragon Pass scripting)" — language design, tests,
  variables, placeholders. http://kingofdragonpass.blogspot.com/2011/10/osl-king-of-dragon-pass-scripting.html
- KoDP blog, "Anatomy of a Scene" — full worked scene with preconditions,
  responses, effects, advice priorities. http://kingofdragonpass.blogspot.com/2013/08/anatomy-of-scene.html
- KoDP blog, "An Architecture Overview" and "Architecture Redux" — three
  layers, bytecode, save-format regret. http://kingofdragonpass.blogspot.com/2011/02/architecture-overview.html ; https://kingofdragonpass.blogspot.com/2013/06/architecture-redux.html
- KoDP blog, "Storylets with Casting" — selection conditions, role casting,
  prefer-the-ring rule. http://kingofdragonpass.blogspot.com/2021/08/kodp-on-web-50-years-of-text-games.html
- KoDP blog, "How Many Scenes?" — 1,624/614/544 counts, ~5 random scenes/yr,
  53-year game. http://kingofdragonpass.blogspot.com/2012/09/how-many-scenes.html
- KoDP blog, "Repeatability" — five-year rule, internal variation. https://kingofdragonpass.blogspot.com/2011/09/repeatability.html
- KoDP blog, "A Skilled Leader", "Skills, Expanded" — test syntax, 15 skills. https://kingofdragonpass.blogspot.com/2011/12/skilled-leader.html ; https://kingofdragonpass.blogspot.com/2012/05/skills-expanded.html
- KoDP blog, "Other Clans", "Friends-and-Relations" — per-clan state model,
  path to tribe and kingship. https://kingofdragonpass.blogspot.com/2011/10/other-clans.html ; https://kingofdragonpass.blogspot.com/2013/12/friends-and-relations.html
- KoDP blog, "The Debug Dialog", "A Taste of QA", "Player Data" — debug
  tools, manual QA, telemetry. https://kingofdragonpass.blogspot.com/2013/04/the-debug-dialog.html ; https://kingofdragonpass.blogspot.com/2012/02/taste-of-qa.html ; https://kingofdragonpass.blogspot.com/2013/06/player-data.html
- Six Ages blog, "Advice" — skill gating, bias, script form. https://blog.sixages.com/index.php/2021/05/18/advice/
- Six Ages blog, "Testing In Bulk", "Scene Tags", "Tags are Magic", "So It Is
  Written" — brute-force harness, tag system, compiler reports, word count. https://blog.sixages.com/index.php/2016/04/21/testing-in-bulk/ ; https://blog.sixages.com/index.php/2016/05/28/scene-tags/ ; https://blog.sixages.com/index.php/2018/04/16/tags-are-magic/ ; https://blog.sixages.com/index.php/2016/07/15/so-it-is-written/
- Game Developer, "Making Six Ages for iOS: A Retrospective" — scene counts,
  compiler, unit tests, what went wrong. https://www.gamedeveloper.com/production/making-six-ages-for-ios-a-retrospective
- GDC Vault, "Designing 'Six Ages'" — talk abstract (tools for four-Harry-
  Potters of text). https://www.gdcvault.com/play/1025740/Designing-Six-Ages-a-Storytelling
- Failbetter, "Anything Nice: King of Dragon Pass (David Dunham)" — advisors
  as lore delivery. https://www.failbettergames.com/news/anything-nice-king-of-dragon-pass-david-dunham
- Wikipedia, King of Dragon Pass — seasons, ring, heroquests, endgame. https://en.wikipedia.org/wiki/King_of_Dragon_Pass
- Emily Short, "Beyond Branching: Quality-Based, Salience-Based, and Waypoint
  Narrative Structures" — QBN strengths/weaknesses. https://emshort.blog/2016/04/12/beyond-branching-quality-based-and-salience-based-narrative-structures/
- Emily Short, "Storylets: You Want Them" and the QBN category — definitions,
  time caves, Reigns critique. https://emshort.blog/2019/11/29/storylets-you-want-them/ ; https://emshort.blog/category/quality-based-narrative/
- Kreminski & Wardrip-Fruin, "Sketching a Map of the Storylets Design Space" —
  dimensions of storylet systems. https://mkremins.github.io/publications/Storylets_SketchingAMap.pdf
- Failbetter, "Echo Bazaar Narrative Structures" parts 1–3 — pattern
  language, ventures, Mark of Cain, Midnight Staircase, "loved terrible
  things". https://www.failbettergames.com/news/echo-bazaar-narrative-structures-part-one ; ...-part-two ; ...-part-three
- Fallen London wiki, Qualities category — counts of pyramidal/discrete
  qualities, menace list. https://fallenlondon.wiki/wiki/Qualities
- CK3 Dev Diary #30 "Event Scripting" and CK3 wiki "Event modding" — on_actions
  replacing MTTH *(blocked this session; cited for the record)*. https://forum.paradoxplaza.com/forum/developer-diary/crusader-kings-3-dev-diary-30-event-scripting.1397140/ ; https://ck3.paradoxwikis.com/Event_modding
- CK2Plus `ze_favor_events.txt` and `CK2Plus_prosperity_events.txt` — real
  CK2 event and MTTH syntax. https://github.com/ck2plus/CK2Plus
- Basileia-Romaion `00_on_actions_yearly.txt` — real Clausewitz
  `random_events` pulse with weights. https://github.com/AndHope/Basileia-Romaion
- ck3-tiger and CWTools — third-party validators; what static checking of
  Paradox script needs. https://github.com/amtep/ck3-tiger ; https://github.com/cwtools/cwtools
- RimWorld wiki, "AI Storytellers", "Cassandra Classic", "Raid points",
  "Events" — StorytellerDef numbers, raid-point formula, adaption factor,
  cooldowns, PopulationIntent. https://rimworldwiki.com/wiki/AI_Storytellers ; https://rimworldwiki.com/wiki/Cassandra_Classic ; https://rimworldwiki.com/wiki/Raid_points ; https://rimworldwiki.com/wiki/Events
- Wikipedia, Dwarf Fortress — "story generator", Legends export, pacing
  devices. https://en.wikipedia.org/wiki/Dwarf_Fortress
- Alex Moon, KSP Launch Window Planner — porkchop UI and Lambert outputs. https://alexmoon.github.io/ksp/
- Wikipedia, Kerbal Space Program — patched conics, maneuver nodes, Edge
  quote. https://en.wikipedia.org/wiki/Kerbal_Space_Program
- Terra Invicta Steam discussion "No idea what I am doing" — transfer planner
  in practice and newcomer confusion. https://steamcommunity.com/app/1176470/discussions/0/554627540143071066/
- Terra Invicta Dev Diary #17 "Rocket Science" *(blocked; cited for the
  record)*. https://www.pavonisinteractive.com/phpBB3/viewtopic.php?t=29424
- Children of a Dead Earth blog, "Fun with Orbital Mechanics", "How Realistic
  Is It Actually?", "The Essence of Space Warfare" — N-body choice and
  simulation-first philosophy. https://childrenofadeadearth.wordpress.com/2016/05/17/fun-with-orbital-mechanics/ ; https://childrenofadeadearth.wordpress.com/2016/04/15/how-realistic-is-it-actually/
- Wildermyth wiki, "Event Types", "Story Inputs and Outputs", "Event", "Event
  design philosophy" — trigger classes and frequencies, roles/aspects,
  ¼/×2 decay, writing rules. https://wildermyth.com/wiki/Event_Types ; https://wildermyth.com/wiki/Story_Inputs_and_Outputs ; https://wildermyth.com/wiki/Event ; https://wildermyth.com/wiki/Event_design_philosophy
- Seedship (itch, IFDB reviews) — structure and generated ending. https://spacegoblingames.itch.io/seedship ; https://ifdb.org/viewgame?id=d8vc24jntql44fe
- Wikipedia, Citizen Sleeper and Citizen Sleeper 2 — dice, stress, glitch
  die, tabletop lineage. https://en.wikipedia.org/wiki/Citizen_Sleeper ; https://en.wikipedia.org/wiki/Citizen_Sleeper_2:_Starward_Vector
- Wikipedia, Roadwarden — 40-day limit, meters, ending-reflection critique. https://en.wikipedia.org/wiki/Roadwarden
- Wikipedia, The Banner Saga — caravan-as-protagonist, no-reload philosophy. https://en.wikipedia.org/wiki/The_Banner_Saga
