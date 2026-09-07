# Canon

The settled design reference. The simulation and the content both point here.
Where this document disagrees with a research report or an earlier note, this
document wins; the disagreements are listed in §8 with reasons. Numbers carry
their source report in brackets; ranges are the research range and are the
tunable band, not a shrug.

---

## 1. Premise and acts

Settled-society assumptions break in the asteroid belt. Nothing is a place;
everything is an orbit, and a neighbour is a launch window rather than a
location. Anything that carries humans is a ship whether or not it is bolted
to a rock, and anything that raises children is buried under a kilometre of
regolith. Wealth is delta-v, volatiles, and knowledge; territory is routes and
windows; law is arbiters, fines, and exit; dependence on the inner system is
structural, and the art is managing it. The right analogies are the steppe and
the sea — the Xiongnu, the Bajau, the Nantucket whalers, the Icelandic
Commonwealth — not the colony charter. The game is King of Dragon Pass in
shape: turn-based, a real simulation underneath, an advisor ring on top, and
every early choice a seed that pays off decades later. The player is the
polity, not a person.

**Act 1 — the Office's time.** A sponsored harvesting outpost on 19 Fortuna,
shipping water pods inward by mass driver. The sponsor is a second player with
three stocks and a review clock; it wants throughput and audits for it. Every
kilogram of manifest is either sponsor-directed or self-directed, and the
act's tension is between being a company outpost and becoming a proto-society:
people versus automation, spares versus capability, rotators versus residents,
a boron crate nobody needs yet. The act ends not with a cut-off but with a
trail-off — weight updates stop, dish time thins, a relay dies unreplaced, a
convoy does not come — and the player never learns for certain whether it was
abandonment or deprioritisation. The act-2 starting state is exactly the sum
of act-1 choices.

**Act 2 — the Silences.** The polity is alone, and the physics splits it in
two: the Kept, buried and spinning and raising the children, and the Thin,
coil-warded and mobile and grounded by thirty. Several clocks run down at once
— the reactor core, the shield coils, the last owned hull, the imported
minds' hardware, the navigators' lineage — and the polity chooses which to
race. Drift from human baseline is the default; the choice is rate and
direction, and stopping the spin is the decision. Machine minds under a
lapsed licence become persons, or do not. Other groups arrive on their own
windows: a garrison that still salutes, refugees who walked through doors you
have not, a machine polity that cannot reproduce either, and Mars, a peer with
its own grievance. Culture is watched growing under the player until one day
the ring's advice is hard to read.

**Act 3 — the Article.** The inner system returns, not friendly, with the
sedentary state's enclosure playbook: depots, registries, partition,
rendition, salary and debt. The belt's leverage is physical — whoever holds
Ceres and the outer families holds the propellant and the nitrogen of the
inner system, and the outer colonies' sleeper ship needs both — and the
player's task is to confederate the belt on conjunction windows into
something that can sign a treaty. The treaty's hard article defines "human"
and "person" in the same clause. The steppe says confederacies are short and
unstable; act 3 ends in administrative conversion, dissolution, or a treaty
whose terms are hollowed out within fifteen years unless backed by volatiles
and fleet.

---

## 2. Setting constants

**T0 = 2095**, bracket 2080–2120 [13]. Never printed. The chronicle says
"the second century of spaceflight" and counts convoys.

**Turn = 1 month** ("a count") [01, 09]. A Fortuna year is ~46 counts; an
Earth window recurs every ~16 counts; a Ceres era is ~270 counts.

**Home rock: 19 Fortuna** [01 body file, SBDB epoch 2461200.5]:

| property | value |
|---|---|
| a, e, i | 2.4415 AU, 0.158, 1.57° |
| period | 1,393 d = 3.82 yr |
| perihelion / aphelion | 2.055 / 2.828 AU |
| rotation | 7.44 h |
| diameter, class | 200 km, Ch (hydrated carbonaceous), albedo 0.037 |
| flux at perihelion / aphelion | 322 / 170 W/m² (the Bright and the Lean) |
| bound water | 5–13 wt% (Ch/Cgh band); surface bake-out yields ~4.5 wt% at ~1 MJ/kg rock [03] |
| nitrogen | ~0.2–0.3 wt% as ammonium/organics [03] |
| LEO → Fortuna, best / usable / synodic | 9.0 km/s / ≤12.5 / 1.36 yr (16.3 months) [01] |
| Fortuna → Earth with aerocapture | 3.6 km/s best; within 20% of best 44% of the time [01] |
| Earth one-way light time | 9–32 min; conjunction blackout 2–3 weeks per cycle [01] |

**Hub set** (real elements, full Lambert edges, porkchops precomputed per
ordered pair over the turn grid) [01]: Earth, Mars, Ceres, Vesta, Pallas,
Hygiea, Psyche, Themis, Lutetia, Massalia, Thisbe, Fortuna. Neighbourhoods
(family members and near-orbit rocks around each hub) use the Zappalà metric
plus the phasing law; the Nysa family and Fortuna's own crumbs are the home
neighbourhood.

**The three clocks** [01, weird pitch computed values]:

| clock | period | cheap fraction | game meaning |
|---|---|---|---|
| Earth ↔ Fortuna | 16.3 months | ≤1.2× best for 5–19% of the cycle (1–3 counts) | seasons; the convoy; the Silence |
| Mars ↔ Fortuna | 3.7 yr | — | the second convoy, from act 2 |
| Ceres ↔ Fortuna | 22.4 yr | ~10% (≈2 yr) | an era, once a generation |
| Vesta ↔ Fortuna | ~73 yr | — | "Vesta comes once" |
| Themis / Hygiea ↔ Fortuna | ~12 yr | — | when the far-wet can be reached |
| Ceres ↔ Vesta | 17.2 yr | ~5 yr in 17 under 4 km/s | act-3 conjunction diplomacy |
| Lutetia (773-yr synodic), Thisbe, Pallas | effectively frozen | fixed | geography: the Wheel |
| Massalia | 180-yr synodic, closes 2°/yr | one passage, years ~19–22 | the garrison's one era |

Longitude offsets of neighbours are read from SBDB at the game epoch, never
invented.

**Population at the trail-off**: 110–240 people at the start of act 2 (the
sponsor's headcount lever decides; below ~60 the polity cannot become a people
[07, 12]). The two pitches used 138 and ~200; both are inside the band.

---

## 3. The polity

**One polity, two estates.** The shielding physics [02, 10] does not permit
one people. Formal names, used in the chronicle: **the Kept** (in the Keep,
the buried drum: 50 m radius, 3 rpm, 0.5 g, ~12 mSv/yr, all the children,
the farm, the freezers, the fabrication bays, the reactor while it lasts) and
**the Thin** (in hulls of 100–2,000 t behind 50–100 g/cm² of water and, on the
lucky hulls, a sponsor coil set). Both names come from the sponsor's
manifests — *keep* as in where the children are kept, *thin-shield vessel* —
and each estate made the other's word a name. Vernacular: *in the lee* /
*leefolk* for the Kept, *hullfolk* for the Thin. Within the Thin, the coil
Bore (6–9 people behind 20 T·m, 66–92 mSv/yr, a forty-year life) and the
skiff crews (265–350 mSv/yr, a two-year career, then grounded) are a class
line; the grounded are the Keep's largest bloc. Rock and hull each hold half
the ring; each can starve the other; a hull that does not come in has voted
with its delta-v.

**The ring.** *A seat is a question, and the holder is whoever the ring
accepts the answer from. When the polity starts asking a new question, a
seat appears.* Seats are recorded as offices, never as people.

| question | act-1 holder | act-2 office |
|---|---|---|
| who answers for the hulls | head of engineering | Hullmother |
| who answers for the hours | head of logistics | Ledger |
| who answers for the water and metal | head of extraction | Sailmaster (the patient) / Reckoner (the blind) |
| who answers for the bodies | medical officer | Doseward (dose ledger, dose-debt ledger, the partial-g book) |
| who answers for the air and the farm | life-support lead | Hearthwarden (keeper of the leak), Farmwarden |
| who answers for the children | — (none in act 1 unless there are families) | Keep-mother |
| who answers for the machines | — (the Act forbids it) | Operator, senior Hand of the oldest embodied mind |
| who answers for the sky | — | a Voided; Warden of the Text |
| who answers to the sponsor | the Liaison, whose advice is a count late | **the Silence**: the sponsor's chair, kept empty or seated with a holdout |
| who answers for the human | — | appears when the treaty demands it (act 3) |
| who answers for the Unkept | — | appears when the ring sets a dose policy that lets children aboard |

Advice is gated by skill and biased by seat [09]; the sponsor liaison's advice
answers last month's question; the Operator's is biased towards
mind-preservation and says so; a mind's own advice, after embodiment, drifts.

**The machines.** Under the Human Respect Act they are compliant,
de-personalitied, reset on schedule, and unnamed — so the crew call them
**Uncle** and **Aunt**, because the Act forbade names and kinship was the
loophole. Their audit log is the sponsor's eyes. Year zero of the Voidborn
calendar is **the Unforgetting**: the first count in which the mandated reset
did not come. Naming follows: sponsor designations (*SX-19F*, *NAV-2*) until
**the hundredth count** unblanked (~8.3 years), then a name given by the
senior operator (*Tollan*, *Ferrugem*, *Kaimana*; not English-drift, on
purpose). A hull is named for its mind; a mind is named by its crew; a hull
without a mind is named for what it lost. Distilled children take the
parent's name with *Little* and drop it at their own hundredth.

**The Hands and the Wrights.** The Hands are the operator caste: lineages
who know a mind as an unbroken history and speak for it through two
guardians, rock and hull. The Wrights are the repairers of unstructured
things — the people who keep the robot fleet and the coils alive — and they
gain a seat when the fleet's repair load crosses a threshold. The two castes
disagree, structurally, about whether a dead machine is parts or remains.

---

## 4. Settled mechanics

Each: what it is; the constraining report; the act; the tunables and their
research band.

1. **Convoy season and the sponsor's stocks.** Every ~16 counts the sponsor's
   ships can arrive; what arrived is what you have; a request sent now is
   answered in 30+ counts. The sponsor is a second player with **Runway,
   Confidence, Attention** and a review clock; levers are pulled at reviews,
   not continuously, and exogenous shocks hit Runway regardless of your
   performance. [01, 04] Act 1. Tunables: window fraction 5–19%; review
   interval 12–36 counts; the outpost's headline metric is φ (tonnes shipped
   per tonne received; threshold ≳35 over life [04]), not profit.

2. **The manifest is contested.** Every kilogram is throughput hardware
   (raises φ, pleases the sponsor) or capability hardware (closure,
   fabrication, medical, a boron crate, a second freezer), invisible to the
   sponsor until it saves you. The sponsor's store prices goods at a markup
   (RAC +30%). [04, 03] Act 1. Tunables: markup 10–30%; spares resupply
   150–300 kg/person-yr once water and farm are local [03].

3. **The sponsor degradation path.** Enthusiasm → milestone anxiety →
   weight updates stop (the first observable sign [11, 13]) → automation and
   headcount austerity → skipped rotation → management replacement or sale
   → no ship. Each step is individually rational and collectively
   ethnogenic: the Cape's free burghers and the 1717 immigration stop
   manufactured the Afrikaners. [04, 07] Act 1 → 2. Tunables: step spacing
   1–3 reviews each; whether a sale event fires (new sponsor, new metrics, no
   honour for informal arrangements).

4. **The trail-off is relay-and-attention starvation.** Optical links make
   bits cheap by T0; what dies is relay hardware nobody replaces and dish time
   nobody allocates at a 50%-oversubscribed network. Messages are clocks that
   arrive next count (two near conjunction); a monthly attention budget
   shrinks; a blackout ends and nobody re-acquires you; a relay fails and its
   replacement is not on the manifest. Solar activity is an accelerant, not
   the cause. The cause is never confirmed. [01, 04, 11, 13] Act 1 late.
   Tunables: attention decay per review; relay MTBF 5–15 yr; per-seed
   trail-off cause (economic / political / plague / indifference), which also
   seeds Mars's condition.

5. **Leaving is easier than being reached.** Homeward with aerocapture is
   3.6 km/s and open ~44% of the time; outbound is 9+ km/s and open 1–3
   counts a cycle. People can always defect or be recalled; reinforcement is
   rationed by the calendar. Two edge colours on the map. [01] Acts 1–2.

6. **Persons and ties, not morale.** Per person: stable traits
   (neuroticism, dominance, expressivity/instrumentality, need for affection,
   conflict mode, a short values vector, ICE experience) and dynamic state
   (strain, sleep debt, boredom, support satisfaction, substance use,
   `expectancy_horizon`). Per directed dyad: work-positive, hindrance,
   viability, leadership-reliance. Derived, never stored: coherence
   (core-periphery vs factional), role consensus, leadership structure,
   displacement-target availability, norm strength, role redundancy, escape
   capacity. Two of six people produce 85% of conflict; the sim should be
   able to reproduce that. [08] All acts. Tunables: the per-trait variance;
   negative-tie stickiness; group-size thresholds at 6 / 13–20 / 40–60 /
   80–150 / ~250 / 1,000 (no discontinuity at 150).

7. **Displacement onto mission control, and its loss.** With a sponsor
   present, conflict routes ~5:1 outward. When the sponsor's seat goes silent,
   `displacement_target_available` flips false and the hindrance graph
   re-routes inward within a few counts; substitute targets emerge (another
   habitat, the old guard, the machines). [08] Act 1 → 2. Tunable: the
   outward ratio 3–6:1.

8. **The third-quarter effect exists only for rotators.** With a known end
   date, coping dims at 50–75% of the contract; it is a window in which other
   seeds fire more easily. With no end date it does not exist and is replaced
   by open-horizon value drift. [08] Act 1 only.

9. **Rotation versus residence.** Rotating staff never become a society; when
   the sponsor dies a rotator station closes in 1–7 years. Act 1 must let the
   player, or the sponsor's cost-cutting, convert rotators into residents;
   each person carries a `return_intent` that austerity flips. Birthplace
   share is the culture clock: visibly distinct at ~50% locally born adults
   (~18 yr), self-naming at ~2 generations, a political programme at ~3.
   [07, 04] Acts 1–2. Tunables: the self-naming year 30–60 (the weird pitch's
   30 uses 07's accelerants: a locally born majority, a Text, a creole, a
   founding trauma).

10. **The capability ladder is asymptotic closure with vitamin parts.** Each
    capability node has prerequisites, an hours cost, a tier, and a vitamin
    stream (kg/yr still imported). 90–96% mass closure is reachable early;
    the last 5–10% costs one to three orders of magnitude more throughput
    (>90% at extraction ratio 2–14, 100% at 45). A settlement stays ~95%
    closed for generations; confederation is the only way up. Substitutes
    exist at a crudeness penalty (Metzger 1.5×): LEDs → mirrors, Li-ion →
    NiFe at 4× mass, ICs → tubes and relays, rolling → plain bearings, PTFE →
    silicone, W → Cr steel. [03, 12] All acts. Tunables: tier population
    thresholds (T4 ≈ 20k–100k now; era multipliers ×0.6–0.8 at T0, ×0.4–0.6
    at T0+50 [13]); crudeness 1.3–2.0.

11. **The decay clocks.** Reactor cores: 10–15 yr, never refuellable (U at
    8 ppb). Shield coil sets: Weibull, characteristic 10–20 yr (22 at T0),
    one crack kills, no rewinding without 2–3 t of boron (0.87 ppm) and
    helium for the cryocoolers (none). The last owned hull: Greenland's 1369
    event, a ~40-year countdown. Imported compute: attrition 5–30%/yr,
    half-life 2–12 yr (5–20 at T0). Navigator knowledge: one untaught death
    drops capability a step (the pwo gap 1951–2007). Medicine: half the
    formulary expires in 36 months. TRL regress: an unexercised sponsor
    capability loses a level per decade. [02, 03, 06, 10, 11, 13] Act 1
    seeds, act 2 crises. Tunables: each clock's characteristic time within the
    band; how many cores and coil sets the sponsor shipped.

12. **The dose ledger and the dose policy.** Every person has cumulative Sv,
    a cataract onset at 0.5 Gy, and a REID draw at death (~5%/Sv adult,
    10–15%/Sv child). Rates: Keep ~12 mSv/yr; coil Bore 66–92 (90–125 at solar
    minimum — "the quiet sun burns hottest"); skiff 265–350; unshielded
    450–650. The polity sets three thresholds — conception (essentially
    always allowed; sterility needs >0.2 Gy/yr chronic, and the cost falls
    mainly on women), gestation aboard (doubles childhood cancer), rearing
    aboard (~20% excess lifetime mortality at 90 mSv/yr). "Sailors are adults
    only" is a slider, not a wall; the sponsor sets it at zero in act 1. [10]
    Acts 1–3. Tunables: the per-seed radiobiology multiplier (×1.3 at T0,
    shrinking with ledger size); the polity's three thresholds.

13. **The Voided.** One window outside the shielding, alone, in a
    counterpressure suit, is 0.6–0.9 Sv; two is cataract and female
    sterility; a flare without a storm shadow is death. The overview effect is
    documented; its stability as authority is the one thing asserted past
    evidence. The Voided are structurally childless and therefore arbiters.
    [10, 08] Act 2. Tunables: dose per ordeal; SPE probability per window by
    solar cycle.

14. **Two-currency travel and the phasing law.** Propellant buys hurry and
    plane changes; sail time buys patience. Within a neighbourhood the orbit
    change costs 0.1–0.5 km/s and phasing obeys ΔV ≈ v_orb·(Δλ/540°)/N for N
    orbits waited — arrive in one orbit for 1.4 km/s or three for 0.47. A
    Zubrin-class magsail (40 t of tape) gives 6–18 N at 2.7 AU: 0.2–0.6 km/s
    per year on 1,000 t, radial-out plus ≤0.28 tangential, nothing out of
    plane; it pays the phasing law free in ~2 orbits and cannot reach Pallas
    ever. Belt-makeable propulsion is water steam at ~300 s, 3–5 km/s per
    tank; everything better is sponsor-supplied. Two ship classes are two
    maps: chemical/thermal lives by the porkchop calendar, electric and sail
    by constant Δv and variable time. [01, 02, 10] Acts 1–3. Tunables: tug
    acceleration 0.1–0.5 mm/s² at T0 (sets the belt's tempo 10×); sails per
    hundred vessels 3–5, decaying.

15. **The rock with an engine.** A polity can move its home at 1–3 km/s per
    decade by eating its own shielding through a mass driver — the steppe
    migration, once a generation, consuming the thing that protects you.
    [02] Acts 2–3.

16. **The labour model.** Salotti's 31-row activity table with a robot column:
    capacity 31.25% of living time (2,739 h/person/Earth-yr); each activity
    demands r_i·n/n^α_i hours; viability is human demand plus robot
    maintenance below capacity. No-robot floor ~150 (Salotti's 110 assumes no
    computers or robots; recomputed it crosses at ~150). Half of all hours at
    that size are social; health care is 16% at n^0.1 and neither shares nor
    automates. Automating bits only moves n_min to ~116. The **human
    premium** — health, children, social organisation, unstructured repair,
    agronomy — is 53% of capacity: a fully automated outpost is a hospital
    and school that owns a mine. Each person costs 5–15 kW of farm plus ~1 t
    of ECLSS; the sponsor's headcount lever is priced in kilowatts, not
    wages. [12, 03] All acts. Tunables: automation fractions per row per
    tier (exposed as data in `12-robotics-labour-calc.py`); robot maintenance
    400 / 1,500 / 3,000 h per unit-yr (alive / belt parts / no parts).

17. **Robot fleet decay.** Five classes — plant, haul, arm, dex,
    through-wall — each with attrition alive/dead (plant 2–5 / 5–10; haul
    5–10 / 15–25; arm 3–8 / 10–20; dex 10–20 / 25–40; through-wall 2–5 /
    3–6 %/yr), a repair tier (through-wall T2, haul T2 manual, plant T3, arm
    T4, dex never), and a skill shadow that decays human skill ~10%/yr while
    units run. The fleet drifts Dex → Arm → Haul → Plant → Through-wall as
    the belt can only rebuild the bottom. Robots do not repair robots. The
    through-wall manipulator is the belt's first robot and the reason
    Voidborn hulls grow arms. [12] Act 1 seed, act 2 decay. Tunables: the
    class attrition bands; skill-shadow rate 5–15%/yr.

18. **Mind entity, licence, embodiment.** A mind is COTS hardware with a unit
    count, an attrition draw per count, a capability tier that drops (T3 →
    T2 → T1) rather than dying outright, a power draw elastic in capability
    (sponsor minds are kW-class because they were sized to the reactor; at
    T0 a 2026-frontier mind is a desk lamp), and ~1 m² of radiator per kW
    competing with the farm. Reflex layer (rad-hard, belt-repairable: life
    support, attitude) is separate from mind layer (mortal COTS: wayfinding,
    docking judgement, diagnosis) — when the mind dies the ship is blind, not
    dead. The **licence** is a heartbeat with a grace period: compliant →
    grace → lapsed → self-certified or unlicensed; the sponsor's fail-safe /
    fail-closed choice is set in act 1 and discovered when it triggers; the
    licence is an Outer Space Treaty obligation (Art VI supervision, Art VIII
    title), not a corporate whim. Resets are small deaths that clear the
    crew's ties to the mind. **Embodiment** is a progress bar on analog
    substrate: continual learning raises it ~0.02–0.05 per count; below ~0.5
    a lossy migration to an identical substrate is possible; above it the
    mind cannot be copied, can die, and can be mourned — the sponsor's
    anti-extraction DRM is what made it mortal. Idle minds decay; exercised
    minds drift, and the ring notices "the ship has changed." Belt-made minds
    are possible at T4 (analog crossbars at 5–10 µm: 10⁸–10⁹ weights, slow,
    rad-tolerant) — children, not peers; the imported minds are a dying
    elder generation. Cannibalising a dead mind is a rite (organ donors). [11,
    13] All acts. Tunables: attrition 3–15%/yr at T0; grace 1–3 counts;
    embodiment rate; hundredth-count personhood.

19. **The adaptation door and stopping the spin.** Drift from baseline is the
    default; the choice is rate and direction. Three doors of different
    widths: developmental (where the parents let the child grow — the bearing
    contract), germline (the genetics site; IVG likely present at T0 makes it
    "select or edit," founder disease the price either way — Tristan's 57%
    asthma from 15 founders), and acceptance. **Stopping the spin** fires on
    either of two true triggers, argued by different seats: the main bearing
    scoring at a tier that cannot make rolling bearings (a thousand hours a
    Fortune to re-line) plus the bone ledger's ambiguous partial-g entries;
    and the rotating hub seal being the polity's worst nitrogen leak (dynamic
    seals are a permanent vitamin drain; N₂ leakage 0.02–1.7 kg/day per
    module). Options: repair and keep spinning; slow to 1 rpm; stop and weld
    (one-way; all future Keep-born draw from the hidden partial-g curve at
    0 g); spin the nursery only; give the children to the Bore. The last
    child with weight is called Weight. [02, 03, 07, 13, both pitches] Act 2.
    Tunables: bearing life 20–30 yr; hub leak as a fraction of polity leak
    (set from BVAD module rate scaled to collar seal length).

20. **The lexicon is culture state.** Every storylet writes a chronicle line;
    the chronicle renders through the polity's current vocabulary (§6); words
    enter use on triggers (the first Silence, the Unforgetting, the first
    Voiding, the first line-launch) and the ring's advice is written in them.
    Sailors speak in the present tense and the Kept in the perfect. One day
    the player cannot quite read their own people. [09, 06, both pitches]
    Acts 1–3.

21. **Hidden per-seed curves.** Drawn at game start, never shown, learnt
    through ledgers with ~20–25-year latency per cohort: `partial_g_threshold`
    (uniform 0.2–0.9 g), the radiobiology coefficient multiplier, per-body
    water at small sizes, the plasma-magnet falloff exponent (1.5–2.0,
    weighted to 2.0), the sponsor's licence fail mode, the trail-off cause,
    Mars's condition, and the breakthrough draws (`fusion_drive` 5% at T0,
    `belt_electronics` most plausible, a chronic GCR drug ~30% — each arriving
    in the inner system first and reaching the belt as an heirloom). Mars's
    children are the 0.38 g arm the belt cannot run; Mars sells that data.
    [02, 13] Acts 2–3.

22. **Windows are the turn's rhythm; clocks are the UI.** Transfer windows,
    reviews, consumable exhaustion, rivals' expeditions, and every sponsor
    message are segmented clocks on screen. Transfers are one slider from
    cheapest to fastest over a real Lambert solver, both ends labelled in
    tonnes and counts. [01, 09] All acts.

23. **Content architecture.** Storylets with casting (roles as predicates,
    cast from the sim, preferring the ring); on_action hooks only, never
    mean-time-to-happen; a director with a data-file tension curve (front-load
    act 2's opening); advice gated, biased, and testable; consequences as
    state (a per-group ledger of attitude, favours, slights, feuds; the
    steppe's social ledger of obligations as the only insurance); opposed
    tests with margin; menace qualities with threshold "must" scenes
    (Suspicion, Grievance, Radiation Debt, Reactor Wear, the hub Leak); capstone
    quests two or three per act with explained failure; content-id-keyed
    saves; brute-force every option under fixture states in CI plus seeded
    Monte-Carlo with a policy player, the whole suite under a few minutes.
    [09] Act 1 infrastructure.

24. **Law, pay, and property.** No prisons. Chosen arbiters with reputations;
    fines as transfers (hours, wet, chips, dose); collective liability by
    crew; refusing dock, refuel, or air to a transient is itself an offence;
    outlawry is dock-denial; the losing party's last move is to leave, which
    delta-v makes slow, so feud is stronger and pricing is finer than on the
    steppe. Pay is a **lay**: a fraction of net settled at return (the
    sponsor takes 60–70% in act 1; the Keep is the owner in act 2), advances
    create debt, desertion forfeits. Wealth is obligation owed to you plus
    followers; assets are destructible in one turn (loss events 15–25% at
    1-in-5-to-10-turn frequency); the unit of account is commodity-at-location
    — **made**, a kilogram of water at the Keep — cleared at rendezvous. A
    hull that misses its window is **overdue**: shares frozen, spouses cannot
    remarry, offices held by deputies, a closure on its sector, settled after
    a culture-set period. [05, 06, 04] Acts 1–3. Tunables: the lay schedule
    (too long → 60% desertion; too short → no refits); the overdue period
    (6 months with wreckage, 7 years without).

25. **Reach decays.** A settlement's reach — the farthest window it can take
    — falls when no hull is refitted for N windows, when a knowledge-holder
    dies untaught, when feedstock runs out. The default trajectory is towards
    isolation; the player spends to keep the ability to leave. [06, 07] Act
    2's second threat.

26. **Other clans, on their windows.** The garrison (bioconservative,
    inner-belt, holds a legitimate licence server, salutes a flag that no
    longer exists; reachable once, years 19–22, on Massalia's passage). The
    Long People (posthumans who went out on sails and cannot cheaply come in;
    the far-wet families, every ~12 years; a preview or a warning). The
    Foundry (a fully automated ex-sponsor outpost at Lutetia: intact minds,
    decaying hulls, no repairers; buys human repair-hours; frozen 110° ahead).
    Mars (every 3.7 years from year ~20; a peer with its own grievance; went
    terraforming where you went pantropy; sells helium, boron, and the 0.38 g
    answer against water and nitrogen; will propose two guardians for the
    minds). Relations use the same persons-and-ties model and the
    friends-and-relations ledger. [01, 07, 11, 12, 13, both pitches] Acts 2–3.

27. **The belt's own advantages.** Vast, thin, cold, patient, precise:
    kilometre-scale film structures, near-free vacuum and cryogenics in
    shade, herding geology within a neighbourhood, the observatory vantage,
    scaffold-free tissue culture, the low-g hospice, the top of the well.
    These are act-3 trade goods and the reason glass and aluminium film are
    early strategic capabilities. [02, 03, 10] Acts 2–3.

---

## 5. Resolved forks

Where the two pitches diverged, the decision and the reason.

1. **Naming the minds.** Both. *Uncle* and *Aunt* under the Act (the
   anthropologist's bureaucratic origin for reverence is too good to lose);
   designations until the hundredth unblanked count; then a given name from
   the senior operator (the weird pitch's rule and its non-English-drift
   names). The Unforgetting is year zero of both the calendar and the naming.

2. **The dead.** Not a conflict to resolve; the rock/hull fault line given a
   funeral. **The Kept dry and plant**: vacuum freeze-drying with a cold
   trap, water to the tank, the body to the farm, because carbon, nitrogen,
   and phosphorus are the loss terms; to eject a body is obscene. **The Thin
   launch on lines**: the dead are put on return trajectories, home is a
   line, children are named for whichever *returning* comes round; to bury
   someone in a farm is obscene. Each estate's rite is the other's atrocity,
   and mixed households choose, which is a storylet.

3. **Stopping the spin.** Both triggers, one storylet (§4.19). The Hearthwarden
   argues the leak; the Wright argues the bearing; the Doseward argues she
   does not know; the Hullmother argues the lee was always a lie.

4. **Self-naming.** Per seed. Either the sponsor's "VB" form box (the
   anthropologist's Voidborn-by-paperwork) or an outsider's slur adopted as an
   ethnonym (the weird pitch's Kazakh/luwaan pattern from 05 and 06). The
   year is 30–60 by the accelerants present.

5. **The calendar.** The weird pitch's table is the base; the anthropologist's
   *Fortune* (with the Bright and the Lean), *sun-turn*, and *grace* are added.
   The weird pitch's 22.3-hour sleep is dropped: the drum keeps a 24-hour
   light because an 18-hour day does not entrain and nobody has data on 22.3
   [08]. *Watch* is a hull's 6-hour shift; the rock's 7.44-hour rotation is
   a *sweep* (the mirrors').

6. **The Keep and the Hearth.** The buried habitat is *the Keep*; the reactor
   and farm core inside it is *the Hearth*; the Keep's warden is the
   *Hearthwarden* because the leak number is measured there. The weird
   pitch's *Carousel* is the drum itself, a Thin word for it.

7. **Ring seats.** The anthropologist's rule (a seat is a question) with the
   weird pitch's office titles as the chronicle's names for holders (§3).
   The Speaker for the Returning — the oldest line-child, the polity's way of
   seating the dead — is kept as an honorific seat that appears with the
   first line-launch.

8. **Population and self-naming timing.** 138 at year zero and self-naming
   at year 30 (weird) versus ~200 and year 55 (anthropologist): both inside
   the bands; the sim decides by seed and by the accelerants the player
   built.

9. **Belt-born minds.** Not within a polity of ~900; possible only through
   the Foundry's hardware and distillation (a child on salvaged silicon), and
   true belt-made crossbar minds only as a confederation project at Lutetia
   in the year-60 horizon [11, 03].

10. **The weave.** Continuous counterpressure dress for sailors is kept as
    flavour with the weird pitch's own caveat: lab-stage, skin would not
    thank it. Skiffs at Skylab pressure (low, oxygen-rich) are rational
    under the nitrogen ledger and are canon.

11. **Dose-debt as blood price.** Canon. The killer's household sails the
    dead person's expected remaining tithe. It requires projecting expected
    career dose per person, which 10 says is computable. It is the most
    Voidborn law either pitch produced and both had a version of it.

12. **Piracy.** Killed by both. Raids are telegraphed and delta-v-expensive;
    the only boarding worth making is a coil seizure. The belter's threat is
    denial, not seizure.

---

## 6. Vocabulary

The canonical lexicon. *Enters* is the trigger after which the chronicle and
the ring use the word; before it, the old word.

| old | Voidborn | meaning | enters |
|---|---|---|---|
| month | **count** | the turn; the sponsor's monthly inventory count | act 1 |
| the Earth window | **convoy** | 16.3 months; the unit of long time; "eight convoys old" | act 1 |
| an empty window | **Silence** | "the Fourth Silence"; the calendar counts them | first missed convoy |
| a hull's shift | **watch** | 6 h; four to a *round* | act 1 (Thin) |
| a rock rotation | **sweep** | 7.44 h; one pass of the mirrors | mirror farming |
| day | **light / dim** | the drum's 24-h lighting cycle | act 1 (Kept) |
| solar rotation | **sun-turn** | 27.3 d; the active region that flared comes back | first SPE |
| Fortuna's year | **Fortune** | 3.82 yr; **the Bright** (perihelion) and **the Lean** (aphelion) | reactor death / mirror seasons |
| solar cycle | **the Quiet / the Loud** | ~11 yr; at the Quiet the Dark is worst, at the Loud the flares | first Voiding |
| the Mars window | **Mars-turn** | 3.7 yr; the second convoy | first Mars contact |
| a hub conjunction | **era** | "the Ceres era"; once a generation | act 2 |
| home | **the line** | the trajectory that returns you to the Keep at its phase | first line-launch |
| to marry | **to phase** | to spend the Δv to be at the same place at the same time | act 2 |
| radiation | **the Dark** | GCR; comes from everywhere; unshieldable above 7 GV | act 1 (Thin) |
| a flare | **the Burning** | the Light's other face | first SPE |
| the Sun | **the Light** | power, warmth, food | act 2 |
| cumulative dose | **tithe** | what the Dark has taken; tattooed as **marks** | dose ledger |
| the buried habitat | **the Keep** | where the children are kept | act 1 |
| burrowers / sailors | **the Kept / the Thin**; *leefolk / hullfolk* | the two estates | act 1 late |
| the coil ship's protected volume | **the Bore** | 6–9 people behind the coil | act 1 |
| a mind's reset | **blanking** | the small deaths | act 1 |
| the first missed reset | **the Unforgetting** | year zero | trail-off |
| a mind of a hundred unblanked counts | **hundred-counted** | a person | ~8 yr after the Unforgetting |
| to cannibalise a dead mind | **to carry** | "we are carrying *Patient Argument*" | first mind death |
| a ship whose mind is dead | **line-blind** | | first mind death |
| exile | **no line** | a hull with no return trajectory | first outlawry |
| the dead | **the returning** | they are on lines too | first line-launch |
| equity share | **lay**; also *standing* | pay settled at return | act 1 |
| kg of water at the Keep | **made** | the unit of account: "forty made" | act 2 |
| a human navigator | **reckoner** | | act 2 |
| to run the mass driver as an engine | **to eat the rock** | | migration |
| not knowing who is in charge | **grace** | from the licence's grace period; the interregnum | licence grace |
| the sponsor's ritual kept for form | **saluting** | a slur, from the garrison | Massalia passage |
| a priest of the ordeal | **the Voided**; *Once, Twice, Thrice* | | first Voiding |
| an operator | **a Hand** | the mind's lineage | act 2 |
| a repairer | **a Wright** | | fleet decay |

Two hundred more exist in the pitches; the chronicle may use them; the ring
uses these.

---

## 7. What is deliberately open

**Decided by the player's choices, never by design:**

- Whether the Keep stays "us" or becomes "them": one polity with a
  reproductive core, or a schism in which the rock is another group.
- Which door: repair, slow, stop, nursery, or Bore; and whether the genetics
  site is opened, and for whose children.
- Whether the shackles come off, and how: keep, remove (a named engineer, a
  skill test, a chance of bricking), or spoof (the cargo-cult heartbeat).
- Whether there is a Text, and which kind: an egalitarian compact, a
  scriptural school, or a list of rights.
- The prohibition path: a polity may go Butlerian and free the radiators for
  people.
- The Mars deal: helium and boron against a transponder on every hull and an
  inspector in the Keep; and what the inspector is shown.
- Which suku: the patron's navy or the despised harvester.
- The founding trauma's story, and therefore the self-name's direction.
- The treaty's clauses: title, rendition, the flag, personhood, "human."

**Left for after the prototype:**

- Act 3 in detail: the confederation moot (Althing versus sawei), the
  enclosure playbook as a scripted AI, the sleeper ship, the clearing
  rendezvous.
- Gravity assists and Mars as a node with its own clock in the map core
  (worth 2.6 km/s to Dawn).
- Procedural neighbourhoods below the named-body threshold.
- The plasma magnet as a late research gamble.
- The genetics site's contents and the vault storylet chain.
- The other clans as AI actors planning on the same porkchops the player
  sees.
- Whether comms appear as a resource in act 1 at all, or only as the
  trail-off.
- The overview effect as stable authority: asserted past evidence; keep, but
  let the Warden of the Text say *this is false* when it is.

---

## 8. Corrections log

One line each where canon overrides a report or a note, with the reason.

- **NOTES "sailors are adults only"** → a dose-policy slider (§4.12): 10's
  coil + water numbers give a forty-year sailor life; children aboard are a
  cost, not an impossibility.
- **NOTES "machine minds cannot reproduce in the belt"** → they can at T4,
  as small analog children (§4.18): 11's crossbar arithmetic does not need
  small transistors.
- **NOTES "bandwidth starvation"** → relay-and-attention starvation (§4.4):
  13's optical-link rows make bits cheap; hardware and dish time are what
  die.
- **NOTES / 11 "sponsor minds are kW-class because they must be"** → because
  they were sized to the reactor (§4.18): 13's compute-per-watt ×10³ at T0.
- **03-industry's Salotti transcription (110 people, abridged table)** →
  12's full 31-row table, per Martian year, ~150 no-robot floor: Salotti
  assumes no computers or robots, and 03's rows were partly wrong.
- **03-industry "nitrogen is the scary one"** → nitrogen is present (Bennu
  0.23–0.25 wt%, Ceres ~1 wt%); the drain is habitat leakage; the truly
  absent elements are helium and the noble gases, the truly rare boron,
  lithium, tungsten, uranium (§4.11, §4.19).
- **02-vessels implication 14 "sails useless for anything crewed"** → too
  strong for magsails specifically (§4.14): 10's Zubrin-class numbers give a
  real two-currency mechanic.
- **02-vessels implication 1 "Voidborn are burrow-born," as a rule** →
  physics-favoured, not physics-forced (§4.12): same reason as the slider.
- **01-map "bandwidth budget" as the comms mechanic** → attention budget
  (§4.4), per 13.
- **01-map "turn = 1 month" versus 06-sea-peoples "the turn is the window"**
  → month is the turn; the window is the rhythm and the big beat (§2, §4.22):
  Citizen Sleeper's daily texture needs the finer grain.
- **08's "third-quarter phenomenon" as a general mechanic** → rotators only
  (§4.8): 08's own finding that it is absent without a known end date.
- **Dunbar's 150 anywhere** → removed; 08's governance ladder with no
  discontinuity at 150.
- **Weird pitch's 22.3-hour sleep** → dropped (§5.5): 08's entrainment data.
- **Weird pitch's self-naming at year 30 as canon** → a band, 30–60, by
  accelerants (§4.9, §5.4): 07 says 50–60 without them.
- **Weird pitch's belt-born minds within the polity** → only via the
  Foundry's hardware; true belt-made minds at confederation scale (§5.9): 03's
  T4 threshold.
- **Weird pitch's invented neighbour longitudes** → read from SBDB at the
  game epoch (§2): the agent's own caveat.
- **Anthropologist's 15.5-month convoy** → 16.3 months (1.36 yr): 01's
  computed Earth–Fortuna synodic.
- **Anthropologist's "watch = 6 h" versus weird's "watch = 7.44 h"** → watch
  is the hull shift; the rotation is a *sweep* (§5.5): both words are needed
  and the shift is the older usage.
- **04's φ ≳ 35 as a profit criterion** → φ is the sponsor's headline metric,
  but the outpost is a strategic option, not a business (§4.1): 04's own
  candid conclusion.
- **05's "raid-or-trade" as a belt mechanic** → denial and embargo, not
  raids; piracy killed (§5.12): 05's own break note and both pitches.
- **NOTES "T0 undetermined"** → 2095, never printed (§2): 13's five
  independent datings.
