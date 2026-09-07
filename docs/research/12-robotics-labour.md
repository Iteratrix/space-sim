# 12 — Robotics and the division of labour

Premise under test: machine minds are good at bits, humans are good at atoms.
How automated should a belt outpost be, where do humans add value, and what
does that do to the society? Builds on 02 (Salotti's 110), 03 (tiers, the
dependency graph, rolling bearings at T4), 04 (the sponsor's automation
lever) and 11 (hardware attrition 5–30 %/yr, reflex vs mind layer).

Short version of the verdict (argued in §1.7): the premise is right about
the *direction* and wrong about the *line*. The real split is not bits vs
atoms but **structured vs unstructured**. Robots already own structured atoms
(haulage, process plant, fixed-station fabrication) and minds own bits.
Humans own *unstructured* atoms — repair, improvisation, working on the
machines themselves — and they own legitimacy. And "bits" are not free of
atoms: every mind and every autonomous robot in the belt is a wasting stock
of semiconductors, so the most bit-like things in the polity are the most
import-dependent. The human premium is not the hand; it is the *closed loop*
— a human repairs humans, reproduces humans, and learns a never-before-seen
repair from a single failure. No robot fleet in the game's horizon does any
of those three.

Flags: **(calc)** my arithmetic; **(recollection)** not verified this pass;
**(estimate)** my number where no source exists.

## 1. Findings

### 1.1 Moravec's paradox, forty years on

Moravec (1988): "it is comparatively easy to make computers exhibit adult
level performance on intelligence tests or playing checkers, and difficult or
impossible to give them the skills of a one-year-old when it comes to
perception and mobility." Pinker's gloss: "the hard problems are easy and
the easy problems are hard." The evolutionary explanation — sensorimotor
skill is a billion years old and compiled, abstract reasoning is 10⁵ years
old and interpreted — is why the paradox has aged well for *manipulation*
and badly for *mobility and perception*. The state of play in 2026:

- **Structured mobility is solved.** Waymo runs ~3,000–3,900 robotaxis in
  10 US metros at ~500,000 paid rides/week, 200 million driverless miles,
  with an IIHS crash rate 68 % below human drivers. What it still needs is
  a *remote-assistance* desk (US and Philippines) that "do[es] not drive the
  vehicles, simply providing guidance … which [the vehicle] can accept or
  reject." Note the architecture: full local autonomy for the reflex, a
  human for the 1-in-10⁴ judgment call, and that human is *seconds* away.
- **Structured heavy atoms are solved.** Rio Tinto's Pilbara fleet was 80
  autonomous Komatsu trucks by 2018, having moved 1 Gt; AutoHaul made the
  first fully driverless heavy-haul rail run in December 2018 over a
  ~1,700 km network, run from Perth ~1,500 km away **(recollection)**.
  Claimed unit-cost savings are ~15 % **(recollection)**. Resolute's Syama
  mine (Mali, 2019) is the first fully autonomous underground mine —
  Sandvik AutoMine trucks, loaders and drills from a surface control room —
  but charging explosives, ground support and *all maintenance* stayed
  human **(recollection)**. FANUC's lights-out plant builds ~50 robots per
  24 h shift and runs unattended for up to 30 days; the Wikipedia summary is
  candid that maintenance, setup and material supply remain human and that
  "complete automation remains largely aspirational."
- **Structured warehouse atoms are half-solved.** Amazon had >200,000 drive
  units by 2019 and ~1 million by 2025 against ~1.5 million employees
  **(recollection for 2025)**; the architecture still carries the pod *to a
  human* who does the pick. Picking arbitrary objects from clutter is the
  canonical still-hard problem.
- **Learned manipulation policies are real and brittle.** Open X-Embodiment
  pooled 22 robot types, 21 institutions, 527 skills, 160,266 tasks, and
  found positive cross-embodiment transfer. π₀ (Physical Intelligence,
  2024) trains a flow-matching action head on a VLM across 8 robot
  configurations and folds laundry (1.0 success on shirt folding in-house
  vs 0 for OpenVLA), busses tables (0.875 on the hard set), assembles boxes;
  the authors say "generalist robot policies are still in their infancy, and
  we have a long way to go," naming long-horizon reasoning, self-improvement,
  robustness and safety as open. Gemini Robotics (2025) claims adaptation to
  unseen environments and new embodiments and "learning new short-horizon
  tasks from as few as 100 demonstrations." One hundred demonstrations of a
  *rare* repair is the whole problem: a failure that has happened once has
  one demonstration, performed by the human who improvised it.
- **Humanoids are a bet, not a tool.** Optimus is 57 kg, 20 kg payload,
  22-DOF hands (Gen 3); its public demos were partly teleoperated and Tesla
  "was criticized for not being transparent about this." Electric Atlas
  (2024) is scheduled for Hyundai's line in 2028 doing parts sequencing — a
  structured task. Rodney Brooks calls humanoid general assistants "pure
  fantasy thinking"; the engineering constraint named is torque density.
  The honest trend: humanoid *form* is converging on factory work, and
  factory work is structured. Specialised machines (arms on rails, mobile
  bases with one gripper, ROVs) do the same work with fewer joints to
  break, and every joint is a bearing (03: rolling bearings are tier 4).
- **Unstructured environments are where robots still die.** The DARPA
  Robotics Challenge finals (2015) — drive a vehicle, cross rubble, clear
  debris, open a door, climb a ladder, cut a wall, turn a valve, connect a
  hose, under deliberately degraded comms — were finished by 3 of ~23 teams
  (KAIST in 44 min 28 s); the memorable output was the fall reel. Ten years
  later there is no deployed robot that does the DRC course unattended.
  Fukushima Daiichi is the sustained test: remote heavy equipment cleared
  debris from April 2011, but inside the buildings Quince lost its tether
  in Unit 2 (2011) **(recollection)**, the 2017 Unit 2 camera probe
  estimated 530 Sv/h from its own image noise and the follow-up "scorpion"
  crawler stuck on debris and was abandoned **(recollection)**; first
  contact with fuel debris was 2019, and the first retrieval, in 2024, was
  aborted on the first attempt because the *humans* assembled the push-pipes
  in the wrong order, then recovered ~0.7 g of debris in November
  **(recollection)**. Thirteen years, ~$180 bn budgeted, 30–40 years
  planned, grams retrieved. And the origin story of the whole field:
  Chernobyl's roof, where the Soviet STR-1 (lunar-rover heritage) worked for
  tens of hours and the German MF-2/"Joker" died of radiation within hours,
  after which ~3,500 soldiers cleared the graphite in 40–90 s shifts
  **(recollection)**. They were called *biorobots*. That word is the belt's
  future in one line.

Where robots are already better than people: repetitive, heavy, dangerous,
long-duration, in *known* environments, with a spares pipeline. Where they
still fail: fine repair of something not in the training set, improvisation
under an unmodelled fault, and — decisively — working on themselves. No
fielded robot repairs robots beyond module swap.

### 1.2 Teleoperation across light-lag: Earth is dead, the belt is dead

Ferrell's 1965 delayed-manipulation experiments found that with a
transmission delay operators abandon continuous control for "move and
wait," and completion time grows roughly linearly with delay; the switch
happens between ~0.3 and 1 s **(recollection)**. Lunokhod was driven in real
time at 2.5 s and was famously exhausting. At 10–46 min one-way from Earth
and 0–50 min belt-to-belt (01-map), *every* manipulation task is
autonomous or local. There is no intermediate. The Waymo remote desk that
answers in seconds does not exist for the belt; the only remote desk is the
sponsor's planning cycle, which is the Mars model:

- **Mars rovers** are the ceiling for Earth-supervised autonomy. Ground
  uploads a plan per sol; AutoNav executes. Perseverance's record is ~700 m
  in a sol (February 2023) with the majority of campaign distance driven
  autonomously **(recollection)**, 44.98 km total by August 2026 — an
  average of ~22 m per sol over the mission **(calc)**. The rover has never
  been touched. Its manipulation is one arm, one drill, one set of tubes.
  Curiosity's drill feed failed in December 2016; the workaround
  (feed-extended drilling) took until May 2018 — seventeen months of
  Earth engineering time to recover one degree of freedom, with no hand on
  site. Opportunity ran 5,352 sols against a 90-sol design (57×), 45.16 km,
  ended in RAM-only mode with a stuck arm joint and a dust storm. Ingenuity
  flew 72 times against a plan of 5 on a Snapdragon 801 phone SoC and died
  when its vision navigation lost features over rippled sand. Lesson: with
  no hands, machines *degrade gracefully into uselessness* over 5–15 years,
  and every recovery costs months of expert time at the far end of the
  link — time the trailed-off sponsor stops selling.
- **ISS** is the ceiling for space robots *with* hands nearby. Dextre
  (1,662 kg, two 7-joint arms with tool changers and force sensing) has
  been ground-operated since its first job in February 2011, swapping
  Remote Power Control Modules, cameras, pumps and bus switching units while
  the crew sleeps, and it does eliminate spacewalks — for ORUs *designed*
  to be swapped by it. Canadarm2 (1,800 kg, 17.6 m, 7 DOF) needed both
  Latching End Effectors replaced by astronaut EVA in October 2017 and
  January 2018 **(recollection)**: the robot could not fix the robot.
  Robonaut 2, the only dexterous humanoid flown, was powered up in 2011,
  got legs in 2014, developed an electrical fault the ground could not
  clear, came home in May 2018 and never flew again; it is in a museum.
  Astrobee (three ~10 kg cubes **(recollection)**) does inventory,
  documentation and cargo shuffling under ground or crew control — bits and
  light atoms. The ISS pattern is exactly the DRC pattern: robots do the
  structured swaps, humans do the unplanned repairs, *and humans repair the
  robots*.
- **Undersea** is the closest terrestrial analogue to EVA: work-class ROVs
  need a topside crew and a tether; "resident" autonomous ROVs (Saipem's
  Hydrone, Ocean Infinity's lean-crewed fleet) cut the ship but not the
  maintainers **(recollection)**. Antarctic autonomous observatories at
  Dome A (PLATO) ran unattended for a season at a time and were serviced by
  the traverse crew each summer **(recollection)**: unattended for months,
  not years.

What this does to the sponsor's "automated outpost": 04 already has the
economic form (the sponsor wants a machine; the machine needs a crew; the
crew has interests). The robotics evidence sharpens it. An outpost with no
hands is a Mars rover: it works until the first unmodelled fault, then it
sends telemetry about the fault for 5–15 years. A machine-only sponsor
outpost (the act-2 "machine-mind society" clan) is therefore a real thing
with a real shape: it *holds* hardware, it *does* structured production
until its haulers and pumps wear out, and it cannot do the DRC course on
itself. Its rational act-2 move is to buy human hands. That is a trade
mechanic (§3).

### 1.3 Robots as a population

A robot is a stack of vitamin parts: rolling bearings and harmonic drives
(03: T4), permanent-magnet motors (Alnico at T3, NdFeB never), encoders and
force sensors (semiconductor), batteries (Li-ion is an import; NiFe at 4×
mass), cameras and lidar (imaging sensors are beyond the tube tier), and a
perception computer (11: COTS, 5–30 %/yr attrition). Industrial arms last
10–15 years *with* an OEM spares pipeline **(recollection)**; there are
~4.66 million of them in service worldwide (IFR, end 2024), every one of
them maintained by a human technician on a schedule. Take away the pipeline
and the Mars numbers apply: 5–15 years of graceful degradation and no
recovery from the first hard mechanical fault.

**Attrition by class, no spares (estimate).** Rates per year of units lost
or downgraded, sponsor-alive (spares crate on each ship) vs sponsor-dead:

| Class | What it is | Vitamins | Alive | Dead | Belt-repairable at |
|---|---|---|---|---|---|
| Plant | fixed process automation: pumps, valves, furnace and electrolyser loops, PID | seals, sensors, control electronics | 2–5 % | 5–10 % | T2 crude (mechanical governors, relays), T3 (tubes) |
| Haul | mobile bulk machines on the rubble pile: excavator, bagger, hauler, mass-driver loader; 0.5–5 t, 5–50 kW | bearings, motors, cutting edges, perception compute if autonomous | 5–10 % | 15–25 % | T2 as a *human-driven* or wire-guided machine; autonomy needs a T1 mind (imported, or belt analog at T4) |
| Arm | fixed 6–7-DOF manipulator, 100–1,000 kg, 1–5 kW, 0.1 mm repeatability | harmonic drives, encoders, motor drivers | 3–8 % | 10–20 % | T4 at spec; T3 crude hydraulic arm with tube servo at ~1 mm |
| Dex | mobile dexterous / humanoid, 60–90 kg, 0.5–1 kW active, 2–4 h battery, learned policies | everything: actuators, batteries, cameras, COTS compute | 10–20 % | 25–40 % | never at spec within the game |
| Through-wall | master–slave manipulator, mechanical or hydraulic, human in the loop, 100–300 kg | seals, cables | 2–5 % | 3–6 % | T2 |

The last row is the important one. Argonne's master–slave manipulators
date from 1945–49 (Goertz; the MSM-8 of every hot-cell newsreel). They are
pure mechanism: a human's hands carried through a wall with force
reflection and zero latency, no electronics, and they have run in hot cells
for seventy years. In the belt a through-hull manipulator turns EVA — the
most expensive human activity, in a suit that is a T4 vitamin capital item
(03) — into shirt-sleeve work. It is the belt-native robot, and it is not
autonomous at all. Nickel-carbonyl handling (03: toxic gas, continuous seal
drain) is literally hot-cell work.

**What a belt-repairable robot looks like.** Ellery's minimal-diversity set
(03 §1.2) already describes it: printed universal motors without rare-earth
magnets, plain bearings in silicone oil, tube op-amps and relay logic,
magnetic-core memory, selenium photocells. Add hydraulics (silicone fluid at
T3, water-glycol before that) because hydraulic actuators tolerate crude
machining better than gear trains, and you have a 1950s hot-cell arm with a
1960s process-control cabinet: heavy (Metzger's crudeness factor 1.5 on
mass and maintenance hours), slow, precise to a millimetre, deaf and blind
(no imaging sensor at the tube tier — it works by touch, limit switches and
a human eye), and utterly incapable of the DRC course. It is a *hand
multiplier*, not a hand replacement. **Can robots repair robots at a belt
tier?** No: module swap on ORU-designed hardware at best (Dextre), and the
ORUs are the imports. The repairer of last resort is a human at every tier
the game reaches. Even the machine-mind clan's own survival plan needs
biorobots.

**Human vs robot, side by side (calc/estimate).** Hours are per Earth year.

| | Human | Imported dexterous robot | Belt-made crude arm (T3) |
|---|---|---|---|
| Mass incl. support | 80 kg + ~1 t ECLSS + 50–500 t hab/shielding (02) | 60–90 kg + ~200 kg charger and spares | 300–800 kg |
| Power | 120 W metabolic; 5–15 kWe farm + 1–2 kW ECLSS ≈ 7–17 kW all-in (03) | 0.5–1 kW active, ~0.1 kW idle | 1–3 kW |
| Useful hours / yr | 2,739 (Salotti's 31.25 % of 8,766) | 2,000–4,000 (battery, downtime, recoveries) | 3,000–6,000 under human direction |
| Lifetime | 60–80 yr, ~45 productive | 5–10 yr with spares, 2–4 without | 20–40 yr, rebuildable |
| Reproduction | 9 months + 18 years; ~3,000 h/yr of raising + education (Salotti); cost is food and time | import: 2–4 yr transit, sponsor's price, never after cut-off; belt: never at spec | 2,000–5,000 machinist-hours **(estimate)** |
| Repairs itself | yes, with T3 medicine | no | no |
| Repairs others | yes | ORU swap only | no |
| Learns a rare task | from one instance | needs ~100 demonstrations (Gemini Robotics) | not at all |
| Radiation | 0.3–0.5 Sv/yr on a sailor (02) | TID irrelevant; SEU resets (11) | none |
| Vacuum work | suit (T4 vitamin) or through-wall | native | native |

Two things fall out that the design brief had backwards. First, **humans are
not cheap atoms**: a marginal human costs ~10 kW of infrastructure and
hundreds of tonnes of shielding; a robot hour costs a tenth of a human hour
in energy. The human premium is not cheapness. Second, the robot's
energy-per-hour edge is bought with a 2–10 year lifetime and zero
reproduction; the human's cost is bought with a 45-year productive life and
a reproduction rate that is *free* in vitamins. Over a 30-year act-2
horizon, one human delivers ~80,000 useful hours and one child; one robot
delivers 10,000–30,000 hours and one corpse to cannibalise.

### 1.4 The labour model: Salotti's table, automated

03-industry's transcription of Salotti is incomplete and partly wrong; the
full tables (fetched from the Europe PMC full text) are 31 activities in
five domains. The units are **per Martian year** (16,487 h); capacity is
31.25 % of living time × n = **5,152 h per person per Martian year**
(2,739 per Earth year; multiply requirements by 0.532 to convert). Demand
for activity i is r_i · n^(1−a_i). Three things the brief needs to know:

1. **Salotti assumes no robots.** Verbatim: "It is nevertheless assumed here
   that such challenges can be faced and that all activities for survival
   are possible without computers and robots." He raises the question —
   "if global productivity gains enabled by computers, robots and other
   modern tools, are greater than the additional time requirements for
   their production and maintenance" — and leaves it. So 110 is the
   *biorobot baseline*: a society of hands with no machines. Every
   automation scenario below is a departure from it.
2. **Recomputing with his central values gives n ≈ 150, not 110 (calc).**
   At n = 110 demand is 643,000 h against 567,000 capacity; the crossing is
   at ~150. The gap is inside his ±50 % CIs and probably reflects a
   rounding or an activity he weighted differently in Fig. 2; treat the
   no-robot floor as **110–150**.
3. **Half the work at n = 110 is social, and it does not share.** Health
   care (n^0.1) alone is 16 % of all hours — 937 h per person per Martian
   year; raising children, education, social organisation and "other social"
   are another 30 %. These are the activities with the *lowest* sharing
   exponents, so they are nearly per-capita and do not shrink with scale,
   and they are the least automatable. Industry (mining, metal, objects,
   chemicals, glass, clothes, other: n^0.6) is 30 % and is where robots
   help.

**Automation fractions.** I assigned each activity a fraction A of hours
that robots take over, per industrial tier (03's T1–T4) and sponsor state
(alive = spares and replacements keep arriving; dead = steady state on
belt-repairable machines only). Robot hours = A × demand; robot units =
robot hours / 6,000 per Martian year; each unit costs human maintenance of
400 h/yr (alive; industrial norm of a few technicians per ten robots) or
1,500 h/yr (dead; parts must be made). Grouped table at n = 110, hours per
Martian year; **P** marks the human premium (irreplaceable hand or
judgment on a decades horizon):

| Activity group (Salotti rows) | Demand | A: T1a/T2a/T3a/T4a | A: T2d/T3d/T4d | T2-alive human / robot | T3-dead human / robot | P |
|---|---|---|---|---|---|---|
| Farm and biology (agriculture, organisms, wastes, agronomy, other) | 80,450 | .5/.6/.7/.8 (agri); .1–.3 (agronomy, organisms) | .3–.6 (agri); 0–.2 | 51,900 / 28,550 | 60,900 / 19,560 | P for agronomy, animals, disease |
| Air and water loops | 4,630 | .8/.8/.8/.9 | .5/.6/.7 | 930 / 3,700 | 1,850 / 2,780 | — (plant class; tube-controllable) |
| Power (PV, electricity, thermal, methane, other) | 45,620 | .7–.9; other .2–.3 | .3–.8 | 17,750 / 27,870 | 24,160 / 21,460 | — |
| Mining | 24,920 | .8/.85/.9/.9 | .5/.6/.7 | 3,740 / 21,180 | 9,970 / 14,950 | — (haul class) |
| Metal production | 36,970 | .7/.8/.85/.9 | .4/.5/.6 | 7,390 / 29,580 | 18,480 / 18,480 | — (plant) |
| Metallic objects (fabrication, machining) | 36,970 | .4/.5/.6/.7 | .1/.2/.4 | 18,480 / 18,480 | 29,580 / 7,390 | **P** (one-offs, fitting) |
| Chemical industry | 36,970 | .7/.8/.85/.9 | .4/.5/.6 | 7,390 / 29,580 | 18,480 / 18,480 | — (plant) |
| Glass/ceramics + clothes | 24,650 | .5/.6/.7/.8 | .2/.3/.5 | 9,860 / 14,790 | 17,250 / 7,390 | — |
| Industry "other incl. innovation" | 36,970 | .2/.2/.3/.3 | .1/.1/.2 | 29,580 / 7,390 | 33,270 / 3,700 | **P** (process improvisation) |
| Construction, concrete, equipping | 15,270 | .3–.8 | .1–.6 | 10,510 / 4,760 | 12,730 / 2,530 | P for equipping |
| Building maintenance + other | 51,270 | .1/.1/.2/.3 | 0/0/.1 | 46,140 / 5,130 | 51,270 / 0 | **P** (unstructured repair) |
| Raising children + education | 50,110 | 0 (babies); .3–.4 (education) | 0; .1–.2 | 47,160 / 2,950 | 48,140 / 1,970 | **P** |
| Health care | 103,120 | .1/.1/.15/.2 | 0/.05/.1 | 92,810 / 10,310 | 97,960 / 5,160 | **P** |
| Meals, social organisation, culture, other social | 95,480 | .1–.3 | 0–.3 | 81,250 / 14,230 | 90,710 / 4,770 | **P** (organisation, legitimacy) |
| **Total** | **643,390** | | | **424,890 / 218,500** (34 % automated; 36 robot units; 14,600 h maintenance) | **514,750 / 128,630** (20 %; 21 units; 32,200 h maintenance) | |

**Minimum viable population per scenario (calc, central values).** The
no-robot floor is 150 in my recomputation (Salotti: 110).

| Scenario | n_min | Robot units at n_min | People per robot at n = 110 | Automated share at n = 110 |
|---|---|---|---|---|
| T1 sponsor alive (imported machines, crew tier) | 63 | 26 | 3.3 | 31 % |
| T2 alive | 57 | 27 | 3.0 | 34 % |
| T3 alive | 44 | 28 | 2.5 | 40 % |
| T4 alive | 37 | 28 | 2.3 | 45 % |
| T2 dead (belt-repairable fleet only) | 113 | 16 | 7.0 | 15 % |
| T3 dead | 101 | 21 | 5.1 | 20 % |
| T4 dead | 84 | 27 | 3.7 | 28 % |

Sensitivity that matters: the maintenance cost per robot. At T2-alive,
n_min goes 54 → 57 → 62 → 72 → 95 as maintenance rises 200 → 400 → 800 →
1,500 → 3,000 h per unit-year. A fleet that costs 3,000 human hours per
robot to keep alive (the sponsor-dead, no-parts case for imported Dex-class
machines) is worth almost nothing: it returns 6,000 robot hours for 3,000
human hours and the human hours are the scarce ones.

Two limiting cases answer the brief's direct question:

- **Bits fully automated, no hands.** If minds take over all planning,
  reckoning, inventory, scheduling and record-keeping — roughly 10 % of
  every activity's hours **(estimate)** — n_min moves from 150 to **116**.
  Bits are ~10 % of the labour of a subsistence society. The clerks,
  planners and navigators were never the headcount.
- **Hands fully automated except the premium.** If robots did *everything*
  except health care, child-raising, social organisation, building
  maintenance, agronomy, animal husbandry and equipping, n_min falls to
  **11** — but the premium activities alone are 2,750 h per person per
  Martian year at n = 110, 53 % of capacity. A fully automated outpost is
  a hospital and a school that owns a mine.

So: **the minimum viable population barely moves when bits are automated,
falls by half when structured atoms are automated with a live sponsor, and
climbs back to ~100 within a decade of cut-off** as the fleet decays to
what the belt can rebuild. The 110 floor is a *sponsor-dead* number, which
is exactly what act 2 is.

### 1.5 Judgment and novelty

**Where humans add value beyond hands.** The recurrent structure of every
famous save is *unmodelled failure + improvised physical fix + a decision
someone had to own*: Apollo 13's CO₂ scrubber adapter built from the
checklist's own cover and duct tape; Mir's 1997 oxygen-candle fire fought
with wet towels while the Soyuz escape route was cut off by the fire itself;
the ISS 2013 ammonia-pump leak located by EVA. In each case the *diagnosis*
was substantially remote (Houston, TsUP) and the *fix* was hands on site.
Modern minds move the diagnosis fully local (DS1's Livingstone fault
model-based diagnosis in 1999; 11 §1.6) and are better than a tired crew at
it. What stays human is (a) the physical improvisation, (b) the decision to
accept a risk on behalf of the people who will die if it is wrong — the
Mir crew, not TsUP, chose not to abandon — and (c) being the party whose
choice counts. 11 already has the law: under OST Art VI/VIII the sponsor
state supervises and owns the machines forever unless it agrees otherwise;
a treaty is signed by humans and binds humans. A machine polity's
representatives at the act-3 table are humans or nothing.

**The reverse: what humans are bad at in the belt.** Trajectory reckoning
(optical navigation against a catalogue is DS1 AutoNav, not a sextant);
vigilance — Bainbridge (1983), citing Mackworth (1950): "it is impossible
for even a highly motivated human being to maintain effective visual
attention towards a source of information on which very little happens, for
more than about half an hour"; inventory reconciliation across ten
thousand items and a decade; the chronicle as *record* (the anthropologist
pitch already has the mind as the exact record and the human as the story).
A belt that goes Butlerian (11 #16) pays for these in hours: a human
navigator line and a human vigilance rota are new Salotti rows at perhaps
1,000–2,000 h per Martian year each **(estimate)**, and the vigilance one is
physiologically impossible to staff well.

**Bainbridge's ironies are the act-1 → act-2 mechanism.** "The designer who
tries to eliminate the operator still leaves the operator to do the tasks
which the designer cannot think how to automate … the operator can be left
with an arbitrary collection of tasks, and little thought may have been
given to providing support for them." "Physical skills deteriorate when they
are not used, particularly the refinements of gain and timing. This means
that a formerly experienced operator who has been monitoring an automated
process may now be an inexperienced one. If he takes over he may set the
process into oscillation." "It is the most successful automated systems,
with rare need for manual intervention, which may need the greatest
investment in human operator training." An outpost that automates hard in
act 1 arrives at cut-off with a crew that has watched pumps for eight years
and cannot run them. The Fitts-list approach — "assigning to man and
machine the tasks they are best at" — is exactly the bits/atoms premise,
and Bainbridge's judgment on it in 1983 was that it "is no longer
sufficient," because taking away the easy parts makes the hard parts
harder.

### 1.6 Social consequences

**What humans do all day.** Read the labour table at T2-dead: of 515,000
human hours, 98,000 are health care, 48,000 raising and teaching children,
51,000 maintaining the built fabric, 91,000 meals and social organisation,
61,000 farming and biology. Industry is 130,000 — a quarter. A society of
hands is a society of *carers and fixers*, not miners, and its prestige
economy will follow the scarce skill: whoever can fix what the sponsor
made. The anthropologist pitch's Hands (mind operators) are one caste; the
labour table wants at least two more: the **Wrights** (repairers of last
resort; the people who do the DRC course on a dead hauler) and the
**Reckoners** if the polity ever goes Butlerian. Prestige-per-hour: a
Wright's hour substitutes for ~2 robot-hours of lost production plus the
capital; a caring hour substitutes for nothing and is 45 % of all work.
That is a class fault line built into the physics, and it is the same one
every terrestrial society has, with the sponsor-made robots standing in
for land.

**Skill decay when a mind does the reckoning.** Bainbridge again; the
maritime precedent is GPS deskilling (celestial navigation dropped from
the US Naval Academy syllabus in 1998 and restored in 2015 as a
cyber-resilience measure **(recollection)**). The sim should carry a skill
ledger per person per domain that decays when unexercised and is
re-acquired at a cost; the act-2 cliff is that every skill the robots owned
is at zero in every human at the moment the robots die.

**Who owns the robots.** In act 1 the sponsor does (OST Art VIII title;
04's manifest is the sponsor's store). Robot-hours are the polity's largest
capital and the sponsor's cheapest lever: every automation package ships
with a licence (11 #2) and a headcount cut (04 #4). At cut-off the fleet is
salvage, and salvage law is the first property law the Voidborn write: is a
dead hauler's motor the mine's, the Wright who pulled it, or the Hands whose
mind drove it? 11 #11 already has the organ-donor rite for minds; robots
are the mundane version, and the mundane version is where the property
argument actually happens.

**The sponsor's headcount lever as a cultural fork.** Two outposts with the
same φ: one at 40 people and 30 robots, one at 110 people and 10 robots.
The first is cheaper, pleases the sponsor, and at cut-off is 40 monitors
with decayed skills, a fleet with a 3–5 year half-life, and a population
below every social floor in 07 (~40 is St Kilda). It does not become a
people; it becomes the machine-mind clan's human remnant, or nothing. The
second is expensive, has a skill ledger, has children, and is Salotti's
society. 07's rule — automated outposts never become peoples — is not a
moral; it is the arithmetic above.

### 1.7 Verdict on the premise

**Where "minds do bits, humans do atoms" holds.** Minds beat humans at
reckoning, vigilance, inventory, diagnosis and the record, and nothing in
the belt changes that. Humans beat every fielded and every plausible
2050s robot at unstructured repair, at improvising with what is in the
room, at repairing the repairers, and at reproducing the workforce. The
sponsor's fantasy of a hands-free outpost dies at the first fault the
designer did not model, and Mars rover history says that is 2–5 years in.

**Where it breaks.**

1. *The line is structured vs unstructured, not bits vs atoms.* Robots own
   structured atoms outright — haulage, plant, fixed-station fabrication,
   driving — and that is 30–45 % of Salotti's hours. Humans keep the
   unstructured 55–70 %, most of which is not industry at all.
2. *Bits are atoms.* A mind is a wasting stock of sub-micron silicon
   (11); an autonomous hauler's perception computer is the same stock. The
   things that do bits are the most import-dependent objects in the polity.
   After cut-off the belt can make hands (crude arms, through-wall
   manipulators, human-driven machines at T2–T3) long before it can make a
   mind (T4, 20k–100k people). The Voidborn end up with *more* hands per
   mind every year, not fewer.
3. *Humans are not the cheap option.* Ten kilowatts and hundreds of tonnes
   per person against a kilowatt per robot. The human premium is
   generality, self-repair and reproduction, and it is bought dearly.
4. *Judgment is not a human monopoly; legitimacy is.* Minds diagnose and
   plan as well or better. What humans keep is the standing to decide and
   to be bound — the treaty party, the person who accepts the risk and dies
   of it.

The one-line version for the design: **the belt automates the mine and
keeps the hospital; the sponsor pays for the first and the Voidborn inherit
the second.**

## 2. Numbers the sim needs

| Parameter | Value / range | Unit | Source |
|---|---|---|---|
| Working-time capacity | 31.25 % of living time = 5,152 per Martian yr = 2,739 per Earth yr | h/person-yr | Salotti 2020 |
| Salotti activity table (31 rows, 5 domains) | see §1.4 and `labour.py`; e.g. health 1,500 n^0.1; mining 3,802 n^0.6; metal 5,640 n^0.6; babies 1,500 n^0.3 (capped 8 at n ≥ 1024) | h per Martian yr, exponent | Salotti 2020 full text |
| No-robot minimum population | 110 (published) / ~150 (recomputed, central values) | people | Salotti; calc |
| Share of hours: health / all social / industry at n = 110 | 16 / 46 / 30 | % | calc |
| n_min, sponsor alive, T1/T2/T3/T4 | 63 / 57 / 44 / 37 | people | calc (§1.4 assumptions) |
| n_min, sponsor dead, T2/T3/T4 | 113 / 101 / 84 | people | calc |
| n_min with only bits automated (10 % of all hours) | 116 | people | calc |
| Premium (non-automatable) demand at n = 110 | 2,750 (53 % of capacity) | h/person per Martian yr | calc |
| Robot productive hours | 6,000 per Martian yr (≈3,200 per Earth yr) at 36 % duty; range 2,000–4,000 Dex, 4,000–7,000 Haul/Plant | h/unit-yr | estimate; mining availability |
| Robot maintenance, human hours per unit | 400 (alive) / 1,500 (dead) / 3,000 (dead, Dex-class) | h/unit-yr | estimate; industrial norms |
| Fleet attrition by class, alive / dead | Plant 2–5 / 5–10; Haul 5–10 / 15–25; Arm 3–8 / 10–20; Dex 10–20 / 25–40; Through-wall 2–5 / 3–6 | %/yr | estimate; 11's compute attrition; rover lifetimes |
| Belt-repairable tier by class | Plant T2–T3; Haul T2 (manual), T4 (autonomous); Arm T3 crude / T4 spec; Dex never; Through-wall T2 | tier | 03 graph |
| Crudeness factor for belt-made robots | 1.5 on mass, power, maintenance hours | × | Metzger 2013 via 03 |
| Belt-made crude arm cost | 2,000–5,000 machinist-h; 300–800 kg; 1–3 kW; ~1 mm precision | — | estimate |
| Through-wall manipulator | 100–300 kg, 0–1 kW, zero latency, no electronics, 70-yr precedent | — | Argonne MSM history |
| Dexterous robot | 57 (Optimus) – 85 (Atlas hydraulic) kg; 20 kg payload; 22 DOF hand; 0.5–1 kW | — | Wikipedia |
| Dextre / Canadarm2 | 1,662 / 1,800 kg; 7 joints per arm; ground-operated | — | Wikipedia |
| Human all-in power | 7–17 kW (farm 5–15 + ECLSS 1–2); 120 W metabolic | kW | 03; calc |
| Human productive life / hours | ~45 yr; ~80,000 h over a 30-yr act 2 | — | calc |
| Robot life without spares | 2–4 yr (Dex), 5–15 yr graceful degradation (rover analogue) | yr | Opportunity 14.5 yr; Curiosity 14 yr; Ingenuity 3 yr |
| Teleoperation breakdown delay | 0.3–1 s → move-and-wait; completion time ∝ delay | s | Ferrell 1965 (recollection) |
| Light-lag Earth / intra-belt | 10–46 / 0–50 | min one-way | 01-map |
| Rover autonomy throughput | ~700 m/sol record; ~22 m/sol mission average | m | JPL (recollection); calc from 44.98 km |
| Earth-side recovery time for one lost DOF | 17 months | — | Curiosity drill 2016–18 |
| Vigilance limit | ~30 min | min | Mackworth 1950 via Bainbridge |
| Bits share of subsistence labour | ~10 | % | estimate |
| Autonomous mining fleet | 80 trucks, 1 Gt by 2018 | — | Wikipedia (Rio Tinto) |
| Lights-out factory | 50 robots/day, 30 days unattended, maintenance human | — | Wikipedia |
| Learned-policy few-shot | ~100 demonstrations per new short task | demos | Gemini Robotics |
| DRC finals 2015 | 3 of ~23 teams completed 8 tasks; 44 min 28 s | — | Wikipedia |
| Fukushima | 530 Sv/h (Unit 2, 2017); grams retrieved by 2024; $180 bn; 30–40 yr | — | Wikipedia; recollection |
| Chernobyl biorobots | ~3,500 men, 40–90 s shifts | — | recollection |

### 2.1 Robot-fleet entity (parallel to 11 §2.1 Mind)

```
RobotFleet {
  class: plant | haul | arm | dex | throughwall
  units: N                                  # each fails independently
  origin: imported | belt_made | hybrid     # belt_made carries crudeness 1.5
  mass_kg_per_unit: plant n/a | haul 500-5000 | arm 100-1000 | dex 60-90 | throughwall 100-300
  power_kW_per_unit: active 1-50 (haul) | 1-5 (arm) | 0.5-1 (dex) | 0-1 (throughwall); idle 0.05-0.2
  hours_per_unit_yr: 2000-4000 (dex) | 4000-7000 (haul, plant, arm) | as human-directed (throughwall)
  capability: structured_only | semi | unstructured   # dex is the only 'unstructured', at 50-90 % task success
  autonomy_needs: mind_tier T0 (plant, throughwall) | T1 perception (haul, arm autonomous, dex)
                  -> binds to a Mind entity; if no Mind of that tier, unit runs human-directed at 0.5x hours
  attrition_per_yr: {alive, dead} per class table §1.3; belt_factor for units outside shielding x1.5
  repair_tier: industry tier at which the polity can rebuild the unit at spec (plant T3, haul T2/T4, arm T4, dex T5, throughwall T2)
  maintenance_h_per_unit_yr: 400 (spares flowing) | 1500 (belt-made parts) | 3000 (imported, no parts)
  spares_stock, cannibalisation_yield 0.3-0.6, vitamins: [bearings, motors, encoders, batteries, cameras, compute]
  battery: li_ion (imported, 5-10 yr calendar) | nife (belt, 4x mass) | tethered
  licence: inherits Mind licence state when autonomy_needs >= T1 (sponsor kill-switch applies)
  ownership: sponsor | polity | salvage_disputed
  skill_shadow: [activities this fleet performs]  # human skill in these decays while units > 0
}
```

Derived: fleet half-life = ln2 / attrition; automated share of each Salotti
activity = min(A_max[class, tier, sponsor_state], units × hours / demand);
maintenance load added to the human ledger; skill_shadow drives the decay
in the per-person skill ledger.

## 3. Implications for mechanics

Each: mechanic — evidence — act.

1. **Salotti with a robot column.** Implement the 31-row table (not 03's
   abridged one), per Martian year or converted, with an automation fraction
   per row driven by RobotFleet units and class, plus a maintenance row
   that scales with units and sponsor state. The polity's viability is
   human demand + maintenance < 5,152 n. — Salotti full text; §1.4 calc. —
   All acts; the act-1 loop's core equation.

2. **The sponsor's automation package is a headcount swap with a hidden
   half-life.** At each review (04 #1) the sponsor offers robot crates
   against a headcount cut; accepting raises φ and Confidence, lowers n,
   and adds units whose attrition is 5–20 %/yr and whose spares arrive
   only while the ships do. The player sees the productivity, not the
   half-life. — Rio Tinto, Amazon, lights-out; rover lifetimes; 11 #1. —
   Act 1 seed, act 2 crisis.

3. **Skill shadow and the Bainbridge takeover.** Every automated activity
   casts a shadow: human skill in it decays (per-person ledger, e.g. −10 %
   per year unexercised **(estimate)**). When units fail, the activity
   returns to humans at the shadowed skill level: crude penalties on
   yield, a chance of an "oscillation" incident (pump cavitation,
   electrolyser flood, furnace freeze) on the first manual months. A
   drills-and-rotation policy (costs hours now) keeps skills warm. —
   Bainbridge 1983; GPS deskilling. — Act 1 choice, act 2 payoff.

4. **Vigilance is a machine job; if the minds go, it is a rota nobody can
   staff.** Monitoring is a Salotti row at zero hours while a T0/T1 mind
   holds it; without one it becomes 1,000–2,000 h per Martian year with an
   incident rate that rises with rota length past 30 minutes. This is the
   concrete cost of 11 #16's Butlerian branch. — Mackworth via Bainbridge.
   — Act 2.

5. **Through-wall manipulators as the belt's first robot.** A T2
   capability node (`throughwall_manipulators`: prereqs bulk_alloys_steel,
   machine_tools_basic, seals) that converts EVA hours into shirt-sleeve
   hours at 1:1.5 and removes suit wear from the vitamin stream. Cheap,
   dumb, and the reason Voidborn hulls grow arms. — Argonne MSM 1945–;
   Fukushima/Chernobyl teleop record; 03 spacesuits at T4. — Act 1 late,
   act 2 core.

6. **Fleet decay as a per-turn draw with cannibalisation and downgrade.**
   Each unit rolls attrition monthly; a failed unit yields 30–60 % of a
   spare; a Haul unit that loses its perception compute downgrades to
   human-driven (halves hours, adds a driver row) rather than dying. The
   fleet's composition drifts from Dex → Arm → Haul → Plant → Through-wall
   as the belt can only rebuild the bottom of the list. — §1.3 table; 11
   attrition; Curiosity/Opportunity degradation. — Act 2.

7. **The last dexterous robot.** When Dex units reach 1, fire a scene: keep
   it for the one job only it can do (the reactor cell, the outside work
   with no through-wall reach), strip it for the actuators that keep three
   arms alive, or send it with a hull as a trade good to the machine-mind
   clan. — Robonaut's museum; organ donors (11 #11). — Act 2.

8. **Robot-repair as a Wright's craft and a caste.** A `wright` skill line
   (unstructured repair) with a ring seat once the fleet's repair load
   exceeds N hours; Wrights' prestige rises with fleet age; conflict with
   the Hands over whether a dead hauler is parts or a mind's body. —
   Canadarm2 LEE EVAs; §1.6. — Act 2 → 3.

9. **The machine-mind clan buys hands.** The fully automated ex-sponsor
   outpost (NOTES) is a Mars rover the size of a town: intact minds, decaying
   hulls, no repairers. Its standing trade offer is robot-hours and spares
   (it holds hardware) for human repair-hours and, eventually, people.
   Accepting seeds a human caste inside a machine polity; refusing lets it
   die and leaves its hardware as salvage on a conjunction window. —
   §1.2; Fukushima; 07's "never become peoples." — Act 2 other-clan, act 3
   leverage.

10. **Automation level as the act-1 cultural fork, scored.** Track
    `hands_ratio` = human hours / (human + robot hours). Below ~0.6 at
    cut-off, the polity fails 07's social floor (n < ~60) and the skill
    ledger is hollow: it cannot become Voidborn; it becomes a remnant. Above
    ~0.8 it was expensive and the sponsor trailed off sooner. Show the ratio
    on the advisor ring as an argument between engineering ("we need the
    haulers") and medical ("we need the people"). — §1.4 n_min tables; 04
    #4. — Act 1.

11. **Rare-failure learning is a human monopoly, and it is a storylet
    generator.** Robots need ~100 demonstrations; a human needs one
    failure. Each novel fault (catalogue from ISS/rover history: seal
    rupture, feed brake debris, tether snag, vision loss over featureless
    terrain) is a storylet cast on a Wright with a skill test; success
    writes a *procedure* into the polity's lore that later makes the same
    fault routine (and, at T3+, automatable). Lore is how a society of
    hands turns improvisation into structure. — Gemini Robotics 100-demo
    figure; Curiosity's 17-month recovery. — All acts.

12. **Salvage law.** The first property dispute after cut-off is over
    sponsor-owned machines: fire a ring scene that sets the rule (polity
    owns; finder owns; lineage of the operating mind owns) and persists it
    as the seed of act-2 property norms and the act-3 rendition clause (OST
    Art VIII: the sponsor still owns them). — 11 #13; 06 lay systems. —
    Act 2 opening, act 3 payoff.

13. **Humans are the expensive atoms; make the farm the constraint on
    headcount.** Each person costs 5–15 kWe or ~100 m² mirror plus ~1 t
    ECLSS; each robot costs ~1 kW. The sponsor's headcount lever should be
    presented in power and shielding mass, not wages: "one more person is
    ten kilowatts you do not have." — 03 farm power; §1.3 table. — Act 1.

### Event seeds, tagged

Act 1 (automation vs headcount):
- *The crate or the berth.* Review turn: sponsor offers three Haul units and
  a Dex unit against four returned berths. Engineering wants the haulers;
  medical points out the four are the only people who have run the
  electrolyser by hand.
- *Night watch.* A pump loop the reflex layer has run for six years trips at
  03:00 in a conjunction blackout; the watchkeeper has never done it
  manually. Skill-shadow test; outcome seeds a training-rota policy.
- *Teleop theatre.* The sponsor's liaison asks the outpost to "stand down"
  a task so a Houston operator can drive the Arm over 34 minutes of lag for
  a publicity demonstration. Waste a week or refuse and lose Attention.
- *The through-wall proposal.* A machinist offers to build hot-cell arms
  from steel and seals for the carbonyl plant instead of the sponsor's
  robot; cheaper, uglier, and it lowers the outpost's "autonomy score" the
  sponsor reads from the manifest.
- *The licence in the hauler.* The Haul units' autonomy stack shares the
  minds' licence heartbeat (11 #2). First missed heartbeat: the haulers
  park themselves. Drive them by hand (find drivers), spoof, or wait.

Act 2 (decay of the fleet):
- *Half-life.* The chronicle notes the fleet has halved. The ring argues
  cannibalisation order: keep the arms (fabrication) or the haulers
  (throughput).
- *The last Dex.* As mechanic 7.
- *Biorobots.* A job only a Dex could do — inside the reactor shielding,
  under a dose that would kill a person in hours — with no Dex left. Send
  people in 90-second shifts (dose ledger, Voided-style legitimacy for the
  volunteers) or lose the reactor.
- *The Wright's seat.* Fleet repair load passes a threshold; the Wrights
  demand a ring seat; the Hands object that the Wrights are gutting minds'
  bodies for bearings.
- *The rover town.* A conjunction window opens to the machine-mind clan.
  It offers a working Arm and a crate of encoders for two Wrights on a
  five-year lay. The two are named; they have families in the rock.
- *Manual again.* The last autonomous hauler loses its perception compute;
  it becomes a vehicle. Someone has to learn to drive on a rubble pile.
  Injury Poisson stream from 02 #10 rises.

Act 3:
- *Rendition.* The inner system's draft treaty lists the sponsor's robot
  serial numbers alongside its minds (OST Art VIII). Returning the haulers
  is cheap; returning the Arm the Wrights rebuilt three times from belt
  steel is the argument about whether a thing rebuilt is the same thing.

## 4. Open questions

- **Automation fractions are mine.** The A values in §1.4 are reasoned per
  activity from the class table, not sourced. The right calibration source
  would be labour-productivity series for mining (tonnes per employee-hour
  before/after autonomous haulage) and process industry (operators per
  loop), which the search budget did not reach. Design decision: expose A
  as data and let the design team tune n_min targets (110 sponsor-dead is
  the natural anchor).
- **Salotti's 110 vs my 150.** Same tables, central values, different
  crossing. Either he weighted something in Fig. 2 that the tables do not
  show, or I have mis-set one exponent. Decide whether the game uses the
  published number or the recomputed one; the difference is inside his
  error bars.
- **Robot maintenance hours.** 400/1,500/3,000 h per unit-year are
  estimates; the sensitivity is large (n_min 54 → 95 across the range).
  Amazon or FANUC maintenance headcount per robot would pin the alive
  value; nothing pins the dead value.
- **Can a T3 belt make a perception computer for a hauler?** 11 says analog
  wayfinder minds at T4. If a T3 tube-and-relay polity can wire-guide or
  beacon-follow a hauler (1960s AGV technology), Haul stays semi-autonomous
  after cut-off and the T2d/T3d numbers improve by ~10 people. A design
  call, and a good one to make in favour of the belt.
- **Dose-limited biorobot work.** The reactor-cell scenario needs a dose
  rate inside the shield and a job-time, which 02 and 10 do not give.
- **Learned policies in the belt.** If the sponsor ships a Dex fleet with a
  frozen VLA policy, does the policy degrade with the compute (11 #1) or
  fail cleanly? And can the polity *collect its own demonstrations* to
  fine-tune (100 per task, on hardware it is losing)? That would let a
  Wright's improvisation be taught to a Dex once — the closed loop the
  verdict says does not exist. Probably a T4 capability, but it is the one
  place the premise could be beaten.
- **Whether the Wrights and the Hands are one caste or two.** The
  anthropologist pitch has the Hands as mind-operators; the labour table
  says the scarce skill is unstructured mechanical repair, which is a
  different person. Creative decision with ring-seat consequences.

## 5. Sources

- Salotti, "Minimum Number of Settlers for Survival on Another Planet",
  Sci. Rep. 10, 9700 (2020) — full text via Europe PMC REST
  (PMC7297723 fullTextXML) — the 31-row activity tables, 31.25 % capacity,
  the explicit no-robots assumption. Recomputation script at
  `12-robotics-labour-calc.py` (same directory; prints the baseline, per-scenario n_min, sensitivities and the grouped table).
- Bainbridge, "Ironies of Automation", Automatica 19(6), 1983 — mirror at
  ckrybus.com — the deskilling and vigilance ironies quoted in §1.5.
- Wikipedia, Moravec's paradox — the original 1988 quote, Pinker, Minsky,
  Newell's and Narayanan's criticisms.
- Wikipedia, Robonaut — 2011 activation, 2014 legs, electrical fault, May
  2018 return, museum.
- Wikipedia, Dextre — 1,662 kg, ground operation, first task Feb 2011, ORU
  swaps replacing spacewalks.
- Wikipedia, Mobile Servicing System — Canadarm2 1,800 kg, 17.6 m, 7 DOF,
  ground control.
- Wikipedia, Astrobee (robot) — inventory/documentation/cargo roles,
  perching arm, ground or crew control.
- Wikipedia, Perseverance (rover) — 44.98 km by Aug 2026; TRN; sample
  count.
- Wikipedia, Opportunity (rover) — 5,352 sols vs 90, 45.16 km, RAM-only
  mode, end in dust storm.
- Wikipedia, Curiosity (rover) — wheel damage, drill feed failure Dec 2016
  and recovery May 2018, computer swaps, 37.4 km at 14 yr.
- Wikipedia, Ingenuity (helicopter) — 72 flights vs 5 planned, Snapdragon
  801, vision-navigation loss over featureless sand.
- Wikipedia, Waymo — ~3,000–3,900 vehicles, ~500k rides/week, 200M miles,
  IIHS 68 % lower crash rate, remote assistance "does not drive."
- Wikipedia, DARPA Robotics Challenge — eight tasks, 2015 results, degraded
  comms rules.
- Wikipedia, Optimus (robot) — 57 kg, 20 kg payload, 22-DOF hands,
  teleoperation controversy, Brooks's criticism.
- Wikipedia, Atlas (robot) — hydraulic 80–85 kg; electric 2024; Hyundai
  2028.
- Wikipedia, Lights out (manufacturing) — FANUC 50 robots/day, 30 days
  unattended; human maintenance remains.
- Wikipedia, Amazon Robotics — >200k drive units by 2019; pods carried to
  human pickers.
- Wikipedia, Industrial robot — 4.66 M in operation end 2024 (IFR).
- Wikipedia, Rio Tinto (corporation) — 80 autonomous Komatsu trucks, 1 Gt
  by 2018; $518 M rail automation.
- Wikipedia, Remote manipulator — Argonne 1945 contract, MSM-8, through-wall
  mechanical linkage.
- Wikipedia, Fukushima nuclear accident cleanup; Fukushima Daiichi (Unit 2)
  — remote heavy equipment 2011, 530 Sv/h 2017, $180 bn, 30–40 yr.
- Open X-Embodiment, arXiv 2310.08864 — 22 embodiments, 21 institutions,
  527 skills, 160,266 tasks.
- π₀ blog, pi.website/blog/pi0 (Physical Intelligence, 2024) — 8 robot
  configurations, task list, success rates vs OpenVLA/Octo, "still in their
  infancy."
- Gemini Robotics, arXiv 2503.20020 — ~100 demonstrations per new
  short-horizon task; unseen environments and embodiments.
- Recollection-only, to verify next pass: Ferrell 1965 (IEEE Trans. Human
  Factors in Electronics, "Remote manipulation with transmission delay");
  Sheridan 1993 "Space teleoperation through time delay"; Rio Tinto AutoHaul
  first driverless run Dec 2018 and Perth operations centre; Resolute
  Mining Syama AutoMine 2019; Canadarm2 LEE EVAs Oct 2017 / Jan 2018;
  Chernobyl STR-1/Joker and the ~3,500 roof "biorobots"; Fukushima Quince
  (2011), scorpion (2017), 2024 pipe-order abort and 0.7 g retrieval; USNA
  celestial navigation 1998/2015; Amazon 750k (2023) / 1M (2025) robots.
- Project reports drawn on: 02-vessels §4 and #9; 03-industry §1.2–1.3,
  §2.1 graph, #2 and #4; 04-economics §1.3, §1.8, #4; 07-cutoff-outposts
  demographics; 11-machine-minds §1.1, §1.4, §1.6, §2.1, #1, #2, #9, #11,
  #14–16; design/voidborn-anthropologist.md (the Hands).
