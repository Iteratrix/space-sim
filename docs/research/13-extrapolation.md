# 13 — Extrapolation: the world at T0, T0+50, T0+100

Reports 01–11 are a simulation of roughly 2026. This report says when the
game starts, what is different by then, what is different fifty and a
hundred years later, and — the deliverable that matters most — which of the
reports' hard limits are physics (identical at T0+100) and which are merely
unpaid-for engineering (movable, and by how much). Method: base rates,
reference classes, and NASA Technology Readiness Levels as the maturity
scaffold, with cost kept on a separate axis. Where a figure is from memory
rather than a fetched source it is marked *(recollection)*; this session's
search budget was exhausted early, so more of that than I would like.

Conventions. **T0** = game start (the sponsor's outpost is being stood up).
The trail-off begins ~T0+10–15 (act 1). The pitches' snapshots (year 5/15/
30/60 after the trail-off) sit at roughly T0+20 … T0+75. **T0+50** is the
act-2/3 horizon; **T0+100** is the epilogue. TRL is the NASA nine-level
scale (Sadin 1974/1989, Mankins 1995; see §1.2.3). Confidence: **H** (I
would bet at 4:1), **M** (evens), **L** (a reasoned guess).

## 1. Findings

### 1.1 Picking T0: about 2095, bracket 2080–2120

The sponsor's business case needs five things to be simultaneously true.
Each can be dated from a base rate, and they agree with each other to about
a decade.

**(a) An in-space propellant market that makes a belt outpost a "strategic
option" (04).** Report 04's own logic: launch demand becomes price-elastic
below ~$1,300–6,100/kg (Metzger); cislunar propellant demand of 300–500 t/yr
(Colvin 2020) appears only once orbital refuelling is routine; lunar-derived
propellant crosses Metzger's φ ≳ 35 threshold 4–11 years after a lunar ISRU
plant exists; and a belt water outpost is only rational once a *Mars-orbit*
or belt-internal depot exists, because at cislunar depots the belt loses to
lunar ice (04 §open questions). As of July 2026 Starship has flown 13
integrated tests, deployed only test articles, and has not yet demonstrated
ship-to-ship propellant transfer (Wikipedia, *List of Starship launches*).
Chain, with a 1.5–2× slip factor (§1.2.1) applied to industry dates:
routine refuelling ~2032–36; a lunar ISRU pilot ~2040–45; lunar propellant
competitive ~2050; a Mars-orbit depot with real demand ~2065–80. A belt
outpost as a *sponsor's* strategic option, not a NIAC study: **2075–2095**.

**(b) Launch and delivery costs low enough that a person-year in the belt
is a line item, not a moonshot.** 04 brackets a belt person-year at $5–30M
at "Starship-era" prices (~$100–300/kg to LEO) and ten times that at Falcon
prices. A sponsor keeping 240 people in the belt (the pitches' founding
population) at $10M/person-yr is $2.4B/yr — an ISS-class budget, which
states and consortia demonstrably sustain for 25+ years. At Falcon prices it
is $24B/yr, which nobody sustains for a commercial outpost. So T0 requires
LEO at ≤$300/kg *realised*, not advertised. §1.3 row 1 puts that at
2060–2090 (M). This is the least binding of the five.

**(c) Machine minds that can run a habitat, and a mature Human Respect Act
regime with licences, audit uplinks and scheduled resets (11).** The
capability is the easy half (information technologies arrive early,
§1.2.2); a mind that can run a habitat's ECLSS, fab and wayfinding under
supervision is plausible by the 2040s. The *law* is the slow half: the EU AI
Act (2024) took ~5 years from proposal to force; the IMO's autonomous-ship
code is voluntary 2026, mandatory ~2032 (11); treaty-grade personhood law
with an international auditor corps, the kind that a nation-state would
*hide a genetics lab from*, is a 20–40-year institutional build after the
capability exists. A mature HRA: **2065–2090**.

**(d) A Mars colony tenacious enough to be a peer sponsor mid act 2 (NOTES:
10⁴–10⁵ people at T0+20–30, a chip fab by ~T0+60).** This is the binding
constraint. NASA's crewed-Mars target has slipped roughly one decade per
decade since 1969 (Space Task Group: 1981–86; SEI 1989: 2019; VSE 2004:
"after 2020"; current: "2040s" *(recollection for the older dates)*), and
SpaceX's own Mars dates (cargo 2022, crew 2024, stated in 2016) have already
missed by ≥8 years. Take first crew ~2040–45. Frontier populations with
immigration grow at 4–5%/yr (Cape 1662–1795, 07) and without it at 2–2.5%/yr
(07). Immigration is the lever: 100 people per ship, one window per 26
months. Ten ships per window (2050s) is ~230 people/yr; a hundred per window
is ~4,600/yr. Mars at 10³ by ~2060, 10⁴ by ~2080, 10⁵ by ~2100–2110 (M).
NOTES wants 10⁴–10⁵ at T0+25 → **T0 ≈ 2080–2095**. The pitch's "chip fab in
the sixtieth year" needs 03's 20,000–100,000 people committed to the chain,
i.e. a Mars of a few ×10⁵: T0+60 ≈ 2150. Consistent.

**(e) The hardware the fiction assumes exists as sponsor heirlooms:
20 T·m coil sets, a magsail tug, a 10–15-year fission core, frontier minds,
MWe-class electric tugs.** MAARSS-class coils are TRL 2–3 (paper designs
2012; CREW HaT NIAC Phase I 2024, 10); magsails TRL 2; MWe NEP TRL 3–4
(Prometheus/JIMO cancelled 2005; 02); 40 kWe-class fission surface power
TRL 5 (KRUSTY 2018) with a 2030-ish flight target that will slip. Using the
transition base rates in §1.2.3 (≈5–10 years per TRL 3→6 and 6→9 *when
funded*, and zero when not), a 20 T·m coil set that a sponsor would ship as
routine equipment is TRL 9 no earlier than **2065–2085**, and only if a
continuous crewed deep-space program pays for it. The fiction's coil sets
being "rare, unreplaceable, sponsor heirlooms with a 10–20-year life" is
exactly what a technology 10–20 years past TRL 9 looks like; it is what
fission power in space would look like today if SNAP-10A had been followed
up. Good fit at T0 ≈ 2090, poor fit at T0 ≈ 2060 (coils would not exist) and
at T0 ≈ 2130 (coils would be commodity).

**Two independent routes agree.** Route 1 (base-rate slip applied to the
optimists): SpaceX's "self-sustaining Mars city by 2050" at a 2× slip lands
in the 2090s. Route 2 (compound the reference-class growth rates from a
pessimistic first landing): 10⁴ Martians and a belt depot economy in the
2080s–90s. The earliest T0 at which *all five* hold is ~2080; the central
estimate is **2095**; the latest at which the fiction's "heirloom" texture
survives (coils and frontier minds still scarce, Mars still a settlement
rather than a civilisation) is ~2120. **Recommendation: T0 = 2095 as a
constant the chronicle never prints.** The game should say "the second
century of spaceflight" and let the era constants (§2) do the work; a
printed year invites the player to check it against their own extrapolation,
and every printed year in past SF has aged badly.

Two things this T0 implies that the reports do not yet assume: (i) the
sponsor's Earth has ~70 years of AI and biotech development that the belt
outpost carries in its heads and its seed stock — the *information*
technologies are far past the reports' 2026 baselines (§1.2.2); (ii) the
Moon is an industrial site with a propellant economy and the belt is its
poorer, farther cousin, which is why the sponsor is a consortium taking an
option rather than a state planting a flag.

### 1.2 Base rates and reference classes

#### 1.2.1 What forecasters get wrong, and by how much

- **Transport and energy: systematically over-optimistic, by ~2× on dates
  and ~10× on costs.** The Shuttle was sold (1972) at ~$118/lb and
  delivered ~$25,000/lb *(recollection; 04 has $54,000/kg)*. NERVA reached
  a flight-adequate engine in 1969 (XE-Prime: 246 kN, 710 s, 28 restarts)
  and was cancelled in 1973; the 2023 DARPA/NASA DRACO revival was cancelled
  in 2025 (Wikipedia, *NERVA*). Artemis III's landing went 2024 → 2025 →
  2026 → 2027 and in February 2026 the landing itself was moved to Artemis
  IV, "early 2028" (Wikipedia, *Artemis III*): four years of slip in seven,
  ~0.6 yr/yr. Constellation (Moon by 2020, announced 2004) was cancelled in
  2010. ITER: first plasma 2016 → 2025 → 2034–36 *(recollection)*. Fusion
  power has been "30 years away" since the 1970s; the current private claims
  (grid power 2028–2035) should be read at the same 2–3× slip.
- **Information: systematically under-optimistic.** Nobody's 1990 roadmap
  had the smartphone; nobody's 2015 roadmap had 2023's language models;
  Moore's law outlived every predicted end for four decades; sequencing cost
  fell faster than Moore from 2008. The "flying cars vs smartphones" error
  (J. Storrs Hall's framing; Vaclav Smil's counter-argument that
  energy-and-matter transitions take 50–70 years) is the single most useful
  correction for this game: **anything that is mostly bits — minds,
  wayfinding, diagnosis, design, control, genomics — will be far past the
  reports at T0; anything that is mostly atoms and joules — launch, coils,
  closure hardware, fabs, reactors — will be roughly where a 1.5–2×-slipped
  roadmap puts it.**
- **The forecasting literature.** Tetlock (*Expert Political Judgment*,
  2005): expert accuracy beyond ~3–5 years is near chance; "foxes"
  (base-rate-first, many models) beat "hedgehogs"; *Superforecasting* (2015):
  start from the outside view, adjust in small steps, keep score. Armstrong
  (*Principles of Forecasting*, 2001): damp trends, combine methods, prefer
  structured judgment over unaided expert opinion. Kahneman & Tversky's
  planning fallacy; Flyvbjerg (2003, 2014): 9 of 10 megaprojects over budget,
  mean overrun ~45% (rail) to >100% (IT, Olympics), with fat tails.
  Metaculus's space questions *(recollection)*: the community has been
  well calibrated at 1–2-year horizons and systematically ~1–2 years early
  on "first flight of X" hardware questions. **Working rule for this
  report: for atoms, take the industry date, multiply the remaining
  interval by 1.5–2.5, and widen the error bar to ±one decade at 50 years;
  for bits, take the industry date and ask what the second-order
  consequences are, because those are what will be missed.**

#### 1.2.2 Wright's law and the reference-class learning rates

Wright's law: unit cost falls by a fixed fraction per doubling of cumulative
production. Measured progress ratios (cost after doubling ÷ cost before):
aerospace 85% (i.e. 15% cost fall per doubling), shipbuilding 80–85%,
complex machine tools 75–85%, repetitive electronics 90–95%, raw materials
93–96% (NASA cost-estimating tables, via Wikipedia *Experience curve
effects*); solar PV modules ~20% per doubling (Swanson's law; Nemet 2009;
Farmer & Lafond 2016); Li-ion cells ~18–20% *(recollection)*; onshore wind
~10–15% *(recollection)*; **nuclear power negative** — French reactor costs
*rose* with cumulative build (Grubler 2010; disputed in part by Lovering et
al. 2016 *(recollection)*). Two lessons for the belt: learning needs
*volume* (a coil set built once a decade learns nothing), and safety-
regulated, bespoke, low-volume systems (reactors, crewed spacecraft) can
show zero or negative learning.

**Launch, honestly.** The 20× fall from Shuttle (~$54,000/kg) to Falcon 9
(~$2,700/kg price, ~$630/kg internal cost; 04) happened over only ~1.5
doublings of cumulative mass to orbit — an implied 85% *fall* per doubling,
which is not learning, it is a regime change (expendable cost-plus →
reusable commercial). *Within* the reusable regime the numbers behave:
Falcon 9's internal cost fell from roughly $2,000/kg at first reflight
(2017) to ~$630/kg (2025) over ~6–7 doublings of reusable-flight count, a
15–18% learning rate — right on the aerospace reference class
*(recollection for the 2017 figure)*. Starship is a second regime change
(full reuse, methane, 100+ t payload). Its marketing floor ($30/kg,
propellant-plus-ops; 04) is physics-adjacent; its realised cost will be set
by flight rate and by whether the ship, not just the booster, is reused
routinely. Apply the Shuttle lesson (a 10× miss on a promised cost is the
base rate for a new launch architecture) and the Falcon lesson (once the
regime exists, 15%/doubling is real): **realised Starship-class cost
$300–600/kg in the 2030s, $100–300/kg by 2060–2090 (T0), $30–100/kg by
T0+50**. CSIS's launch dataset confirms only the shape: "gradual decline
1957–2005, steeper 2005–2020" (CSIS Aerospace).

**Superconducting tape (for coils).** REBCO tape went from ~$300/kA·m (2010)
toward ~$50–100/kA·m (2025) on fusion-startup demand, with vendors claiming
$10–20 by the 2030s *(recollection; not verified this session)*. Volume is
now real (fusion magnets, one CREW HaT coil alone is ~1,000 km of tape; 10)
so a 15–20%/doubling curve is plausible: **tape cost ×0.1–0.2 by T0**. But
tape is <5% of a coil *system's* mass (structure, cryo, radiators dominate;
10) and the structure mass is set by magnetic pressure B²/2μ₀ — physics, not
learning. Coil-set *mass* per protected volume falls perhaps ×0.5–0.7 by T0;
coil-set *cost* falls ×0.1–0.3. Note the manufacturing-readiness split: the
tape is MRL 9 (a catalogue product); a 20 T·m crew-rated coil set is MRL 2–3
(no production line, no qualified process).

#### 1.2.3 TRL as the maturity scaffold — and its two failure modes

NASA TRL: 1 principles observed; 2 concept formulated; 3 analytical/
experimental proof of concept; 4 breadboard in lab; 5 breadboard in relevant
environment; 6 system prototype in relevant environment; 7 prototype in
space; 8 flight-qualified; 9 flight-proven (Wikipedia, *Technology
readiness level*). GAO's assessment guide (GAO-20-48G) wants TRL 6–7 at
program start and documents that immature technology is the leading cause of
cost and schedule growth in federal programs. **Transition-time base rates**
(NASA/SAIC aeronautics case studies, Peisen et al. 1999, and DoD acquisition
practice; *recollection, figures not verified this session*): a funded
technology moves one TRL every ~1.5–4 years; TRL 3→6 typically 5–10 years;
TRL 6→9 typically 5–10 years and needs a *mission* to fly on. The "valley of
death" at TRL 4–6 is where industry investment must replace research
funding (Wikipedia). Manufacturing Readiness Levels (DoD MRL 1–10) track the
separate question of whether a production process exists at rate.

Two disciplines the coordinator asked for, both borne out by the record:

1. **TRL is not monotonic.** Facilities close, people retire, qualification
   lapses. The list of belt-relevant technologies that have *regressed* from
   TRL ≥6 is long, and it is the right list of things a sponsor could plausibly
   have at T0 as rare heirlooms rather than commodities:
   - Nuclear thermal rockets: TRL 6 in 1969, cancelled 1973, revived and
     cancelled 1991–94 and again 2023–25 (Wikipedia, *NERVA*).
   - Space fission power: SNAP-10A flew 1965 (TRL 9), then nothing for 53
     years until KRUSTY (TRL 5, 2018).
   - Saturn-class heavy lift: 1973 → SLS 2022. Crewed lunar landing: 1972 →
     2028 at the earliest.
   - Human centrifuge and artificial-gravity facilities: 1960s–70s
     programs, the ISS Centrifuge Accommodation Module cancelled 2005;
     only short-radius ground centrifuges (DLR :envihab) since.
   - Closed ecological life support: BIOS-3 (1970s–80s) → Lunar Palace
     (2018); MELiSSA since 1989 with no flight system; Biosphere 2.
   - Mass drivers: SSI/MIT prototypes 1977–85, then nothing (02).
   - MgB₂ shielding coils: SR2S ended 2015 with a roadmap, not a design
     (10).
   **Design consequence:** "TRL 6 today" means "TRL 6 until someone pays".
   At T0, every one of these is plausibly TRL 9 *for the sponsor* and TRL
   0 *for the belt*, and — the act-2 point — a capability the belt cannot
   exercise for a generation regresses in the belt exactly as NERVA did on
   Earth (03's "skills decay" open question is the NERVA pattern).

2. **TRL says nothing about economics.** Solar-electric propulsion has been
   TRL 9 since Deep Space 1 (1998) and is still niche for cargo; NTR could be
   TRL 9 and still lose to refuelled chemical on cost. §1.4 therefore keeps
   *engineering maturity* (TRL/MRL) and *cost/demand* as separate columns.

### 1.3 The parameter table: now / T0 / T0+50

TRL "now" is my assessment against the NASA definitions with the source in
brackets; the T0 TRL applies §1.2.3's base rates with the 1.5–2× slip.
"Class" anticipates §1.4: **P** physics, **E** economics/engineering, **K**
knowledge (the value is fixed; our uncertainty about it is what shrinks),
**I** institutional.

| # | Parameter (unit) | Now (2026) | T0 (~2095) | T0+50 | Conf. | TRL now → T0 (MRL where scale matters) | Class |
|---|---|---|---|---|---|---|---|
| 1 | Launch to LEO, realised ($2025/kg) | 630 cost / 2,700 price (F9); Starship 13 flights, no refuelling yet | 100–300 | 30–100 (floor ~30) | M | Full reuse TRL 6–7 → 9 | E |
| 2 | Cargo delivered to belt hub ($/kg, slow tug) | 3–5×10⁵ (Dawn ~$470M/1.2 t; Psyche ~$1.2B/2.6 t, mission-priced) | 2,000–10,000 | 300–2,000 | M | Depot + reusable tug architecture TRL 3 → 8–9 | E |
| 3 | Person-year in belt ($M) | n/a (ISS ~450) | 3–15 | 0.5–3 | M | — | E |
| 4 | Chemical Isp (s): LOX/LH2, LOX/CH4, water-steam | 450 / 370 / 190–320 | same | same | H | 9 | **P** |
| 5 | NTR-H2 in service (Isp ~900 s) | TRL 6 (1969), regressed; DRACO cancelled 2025 | 50% flies at all; if so an Earth–Mars fast-transit engine. In the belt NTR runs on water at 250–350 s (02) | 70% | M | 6 (stale) → 9 or 0 | E (Isp itself P) |
| 6 | NEP specific mass (kg/kWe), MWe class | 40–50 design SOA (Prometheus, never built); Kilopower 150 | 20–35, 1–5 MWe units exist (70%) | 10–20 | M | 3–4 → 8–9 | E (radiator floor P) |
| 7 | Tug acceleration (mm/s²), 500–1,000 t vessel | 0.05–0.1 (Dawn 0.075) | 0.1–0.5 | 0.5–2 | M | derived from 6 | E |
| 8 | Fusion drive in service (any) | TRL 2 | ≤10% | 20–30% | L | 2 → 3–5 | E (fuel D is belt-makeable: P) |
| 9 | Magsail / plasma magnet thrust at 2.7 AU (N per 1,000 t) | paper: 6–18 N Zubrin loop; f_o unknown (10) | same numbers; ~30% a demo has flown | same | M | 2 → 4–7 | **P** (values) / E (existence) |
| 10 | Active shield: coil-system mass per protected m³ (t/m³) at 20 T·m | 0.7 (MAARSS-II ~200 t / 280 m³) | 0.35–0.5 | 0.25–0.35 | M | 2–3 → 8–9 for sponsor; MRL 2–3 → 6–7 | E; structure ∝ B²V is **P** |
| 11 | Active shield dose factor at 8 / 20 / 40 T·m (relative) | 0.5–0.75 / 0.19–0.37 / 0.10–0.18 (10) | same | same | H | — | **P** |
| 12 | REBCO tape cost ($/kA·m) | ~50–100 *(recoll.)* | 5–20 | 3–10 | M | MRL 9 | E |
| 13 | Cryocooler input W per W at 20–30 K | 100–200 (10); Carnot floor ~14 | 40–80 | 25–50 | M | 9 (improving) | E, floor **P** |
| 14 | Coil-set characteristic life (yr) | 10–20 prior (10) | 15–30 | 20–40 | L | — | E |
| 15 | Passive shielding for families (t/m²); GCR free-space (mSv/yr, solar min) | 10; 450–650 | same | same | H | — | **P** |
| 16 | Radiobiology: cancer mortality %/Sv (adult / child); ovarian chronic threshold Gy/yr | ~5 / 10–15; 0.2 (10), each ±×2 | same central values, ±×1.3 | ±×1.2 | M | — | **K** |
| 17 | Water / O₂ / food closure (%) in a 100+-person hab | 98 / ~50 (75–90 target) / ~0 | 99+ / 90–95 / 50–90 | 99.5 / 95 / 90+ | M-H / M / L | 9 / 6 / 4–5 (Lunar Palace) → 9 / 9 / 8 | E; last 1–2% leakage **P**-ish |
| 18 | Life-support hardware (t/person) and spares (kg/person·yr) | 1 P-C, 3 BLSS; 170–300 | 1.5–2.5 (with farm); 50–150 | 1–2; 30–100 | M | — | E |
| 19 | Crop area (m²/person, full diet); LED efficacy (µmol/J) | 40–65; 1.7–3 | 25–40; 3.5–4.5 (ceiling ~5) | 20–30; ~4.5 | M | 5 → 9 (MRL 4 → 8) | E; photosynthesis ceiling **P** |
| 20 | ISRU: water bake-out (MJ/kg rock), FFC O₂ yield (kg/100 kg) | ~1; 40–45 (03) | same | same | H | Lunar water ISRU 4–5 → 9; belt first-of-kind *is* the T0 outpost | **P** (yields, enthalpies); E (plant TRL) |
| 21 | Tier population thresholds ×(03's T2/T3/T4: 50–300 / 500–5,000 / 10⁴–10⁵) | ×1 | ×0.5–0.7 | ×0.3–0.5 | L | — | E (see §1.4 note) |
| 22 | Compute efficiency (relative to 2026 per W, fixed task) | 1 (1.4 TFLOPS/W FP16) | 10²–10⁴ (S-curve toward ~10⁻¹⁵ J/op) | 10³–10⁵ | M | 9 | E; Landauer/thermal floor **P** |
| 23 | Rad-hard vs COTS efficiency gap; belt accelerator attrition (%/yr) | 10²–10⁵; 5–30 | 10–100; 3–15 | 3–30; 2–10 | L | — | E/K |
| 24 | Compute-stock half-life in belt (yr) | 2–12 (11) | 5–20 | 7–30 | L | — | E |
| 25 | Robot manipulation (share of a human technician's task set at parity, unstructured) | ~10–30% *(assumed; parallel report owns this)* | 70–90% | ~100% | M | 5–6 → 9 | E (bits) — but actuators/bearings/sensors are vitamin flows (atoms) |
| 26 | Germline editing: legality (enhancement / therapeutic) | banned ~70+ states; WHO 2021 registry; He Jiankui 2018 | banned by treaty-grade instrument (70%) / legal in a minority (40%) | same, with defector jurisdictions and off-Earth sites | M | — | **I** |
| 27 | Germline capability: off-target + mosaicism rate; loci per edit; IVG | 5–20%; 1–3; mouse only | <1%; 10s (multiplex base/prime); human IVG likely (60%) | routine where legal | M | 4 → 8 | E/K |
| 28 | GCR pharmacological countermeasure (relative cancer-risk factor) | none chronic (amifostine acute only) | 0.7–0.9 (30% that a real one exists) | 0.5–0.8 (50%) | L | 3 → 5–7 | K/E |
| 29 | Bone-loss countermeasure (6-month µg BMD loss, %/month → controlled?) | 1–1.5 uncontrolled; ARED + bisphosphonate ≈ controlled for 6 mo | controlled for 1–3 yr at 0 g in adults (M); developmental partial-g unknown | multi-year adult 0 g managed; children still a **K** gap unless Mars data | M / L | 6–7 → 9 | K |
| 30 | Space law: resource extraction ≠ appropriation | Artemis Accords 71 states (Aug 2026); Russia/China ILRS; Moon Agreement 17 parties | customary law (85%); COPUOS implementation agreement (50%); two or three blocs | Mars as a third bloc; OST Art VI/VIII unchanged | H | — | **I** |
| 31 | One-way light time, conjunction blackouts, synodic periods | 01's values | same | same | H | — | **P** |
| 32 | Belt downlink per dish (Mbps at 3 AU) | 0.1–1 RF / ~1.3 optical scaled (11) | 10–1,000 optical | 10³–10⁴ | M-H | DSOC 7 → 9 | E |

Notes on the rows that most change the design:

- **Rows 21 and 25 (tier thresholds and robots).** 03's thresholds are
  "people whose working time is committed to the chain". Automation converts
  labour into capital plus *vitamin flow*: each robot is bearings, sensors,
  actuators, chips. So better robots lower the population per capability
  (×0.3–0.5 by T0+50) *and* raise the import dependence unless the robots
  are belt-repairable, which at T3 means tube-and-relay reflex layers driving
  crude actuators (11's reflex/mind split). Additive manufacturing moves
  *bulk* items down a tier (wire-DED steel parts, cast housings) but not the
  precision-metrology chain that gates rolling bearings and lithography; the
  T4 thresholds fall less than the T2/T3 ones. The honest era knob is a
  per-tier multiplier, not a single scalar (§2).
- **Row 22 (compute).** Koomey's law (computations per kWh doubling every
  ~1.6 years to 2009, ~2.6 years since; *recollection*) plus algorithmic
  gains (11: ~33×/yr at fixed task) gives 10²–10⁴ by T0 before the thermal
  floor bends the curve. Consequence: a mind of *today's* frontier capability
  runs on 10–100 W at T0; the sponsor's frontier mind is still 1–10 kW
  because capability is sized to power (11's "elastic" case). The
  reports' kW-per-mind and radiator-per-mind numbers survive only as *the
  sponsor's* minds; a belt-born analog mind of 2026-frontier capability is a
  desk lamp. **This is the single largest revision to the machine-minds
  fiction**: the dying gods are big because they are ambitious, not because
  thinking is expensive.
- **Row 32 (bandwidth).** Optical deep-space comm (DSOC 267 Mbps at 0.21 AU,
  11) makes bits per dish ×100–1,000 by T0. The trail-off as "bandwidth
  starvation" (NOTES) needs re-pointing: what starves is *relay hardware and
  ground-side attention* (dish time, a relay not replaced, a conjunction
  blackout nobody re-acquires from) — 04's DSN oversubscription is the right
  model, and it is not a bit-rate problem. Keep the mechanic, rename the
  resource.

### 1.4 The physics-vs-economics audit

The most important table. **P** = physics: identical at T0+100, no seed
should vary it. **K** = a fixed value we do not know; the sim draws it per
seed (or fixes it) and the *society* learns it. **E** = economics or
engineering maturity: movable by era, with the multiplier given. **I** =
institutional: a choice, and therefore a storylet.

| Constraint | Report | Class | At T0+100 | Movable by | Design note |
|---|---|---|---|---|---|
| GCR spectrum, free-space dose 450–650 mSv/yr, solar-cycle modulation ×0.4–0.6 | 02, 10 | **P** | identical | — | Eternal. The solar cycle is forecastable, not changeable. |
| Passive shielding curves (water/regolith factors; regolith neutron peak >45 g/cm²) | 02, 10 | **P** | identical | — | 5–10 t/m² for families is forever. |
| Magnetic rigidity cutoff; dose factor per T·m | 10 | **P** | identical | — | 20 T·m always buys 0.19–0.37. |
| Coil structure mass (∝ B²·V, magnetic pressure) | 10 | **P** | identical floor | — | The floor; row 10 above moves the overhead only. |
| Coil conductor + cryo + radiator mass; coil-set cost; coil life | 10 | E | ×0.5 mass, ×0.1–0.3 cost, ×1.5–2 life | tape learning, cryocooler efficiency, structure optimisation | Heirloom at T0, expensive-but-buyable at T0+50 *for those with a supply chain*. The belt has none: for the belt it stays heirloom. |
| Cryocooler W/W | 10 | E, **P** floor (Carnot ~14 at 20 K) | ×0.3–0.5 | engineering | Helium remains the working fluid: chokepoint is P (no He in belt). |
| Boron 0.9 ppm, helium absent, F 60 ppm, U 8 ppb, Th 29 ppb, Cu 126 ppm, Li 1.5 ppm | 03, 10 | **P** | identical | — | Chokepoint imports are eternal; only *substitutes* are E. |
| Fission fuel in the belt | 02, 03 | **P**+E | never | breeding needs a fuel seed; enrichment needs F chemistry at T5 | Reactors are imported clocks forever. Fusion (D from water) is the only escape — see §1.6. |
| Rocket equation; Isp per chemistry; water-steam 190–320 s | 02 | **P** | identical | — | Water is 3–5 km/s per mass-ratio-3 tank forever. |
| Ion/NEP Isp 2,000–5,000 s | 02 | **P** (Isp) / E (α kg/kWe) | Isp same; α ×0.3–0.5 | reactor + radiator engineering; radiator floor is P (σT⁴) | Tug accel ×5–10 by T0+50 (row 7). This is 01's "tempo of the belt map" knob. |
| Δv map, synodic periods, windows, phasing law, plane-change 0.31 km/s/° | 01, 02 | **P** | identical | — | The calendar is eternal. Higher accel widens usable windows (25–30% → 40–50% of cycle at 0.5 mm/s²) but never removes them. |
| Light-time; conjunction blackouts | 01 | **P** | identical | — | Advice always arrives a turn late. |
| Bits per dish | 11 | E | ×100–1,000 (optical) | — | Rename "bandwidth starvation" → "relay/attention starvation". |
| Radiative cooling W/m² at 300 K; radiator area per kW | 11 | **P** | identical | — | "Rationing thought by radiator" survives — but the *thoughts per watt* are E (×10²–10⁴). |
| Compute per watt | 11 | E (floor P) | ×10³–10⁵ | Koomey + algorithms | Big revision, §1.3 row 22. |
| Rad-hard lag; SEU rates in small nodes | 11 | E/K (partly **P**: smaller nodes, more upsets) | gap ×0.1; attrition ×0.3–0.5 | rad-hard-by-design, error-corrected analog | Half-lives lengthen but the mortal/immortal split stays: the efficient hardware is the fragile hardware. |
| Semiconductor fab population 20k–100k (micron ICs) | 03, 11 | E (sticky) | ×0.3–0.5 | maskless litho, printed electronics, automation | The most plausible breakthrough target (§1.6 #3). Chemistry gates (HF, ultrapure gases) are P. |
| Rolling bearings, precision-metrology chain (T4) | 03 | E | ×0.3–0.5 population; could drop to T3 with imported metrology references | AM + in-situ interferometry | The one T4 item AM can pull down. |
| Tier thresholds T2/T3 | 03 | E | ×0.3–0.5 | automation, AM, minds | Per-tier multipliers. |
| Vitamin fraction 4–10% of mass | 03 | E | ×0.5–0.7 | AM, local electronics | Never zero: chips, He, F, B, seeds, strains. |
| Life-support closure (water/O₂/food) | 02, 03 | E; last 1–2% **P**-ish (leakage, separation entropy) | 99.5/95/90+ | — | Closure decay in act 2 is still real; the *floor* it decays toward is higher at T0 than 2026's. |
| Crop area per person; g dry per kWh | 02, 03 | E above a **P** floor (photosynthesis ~10 g/kWh max; LED ~5 µmol/J max) | ×0.5 | breeding, LEDs, CO₂ enrichment | 20–30 m²/person is the floor for a real diet. |
| Bake-out enthalpy, electrolysis energy, FFC/MRE stoichiometry | 03 | **P** | identical | — | ISRU *yields* are eternal; ISRU *plant reliability and mass* are E (×0.5). |
| Water content of a given body (1–12 wt%) | 01, 03 | **K** | fixed per body; unknown until prospected | — | Per-seed draw; prospecting reveals. |
| Partial-g dose-response (threshold 0.2–0.9 g?) | 02 | **K** | fixed; learnt at 20 years per data point | — | §1.5. Mars is the 0.38-g arm; the burrow the 0.5-g arm; the hull the 0-g arm. |
| Radiobiology coefficients (5 %/Sv etc.) | 10 | **K** | fixed; uncertainty ×2 → ×1.2 | — | The dose ledger's *meaning* sharpens over the game. |
| Vestibular rpm tolerance 6/10/23 | 02 | K, biology; training moves the *habituated* value | ~same | — | Cultural fork (4 rpm at small radius) survives. |
| MVP genetics 98 (managed) / 14,000–44,000 (unmanaged); founder disease | 02, 07 | biology (P-like) but *management* is E/I | 98 stays; IVG + selection (row 27) make management cheap | — | §1.6 #7: the Tristan price becomes optional if the polity accepts embryo selection — a new ethical fork, not a removed one. |
| Psychiatric incidence, conflict shares, sex-ratio violence threshold, social floors (40/250) | 07, 08 | **K** (human nature) | identical | — | Eternal. Screening and pharmacology shave rates, not thresholds. |
| Launch cost; delivery cost | 04 | E | ×0.1–0.2 by T0, ×0.03 by T0+50 | Wright + regime changes | Sets the sponsor's patience, not the belt's physics. |
| PGM market saturation; ≤20% of market per shipment | 04 | E/**K** (demand elasticity) | similar | terrestrial substitution could *lower* PGM demand | PGMs stay a trap. |
| OST Art VI supervision, Art VIII title | 11 | **I** (sticky: never amended in 59 years) | very likely unchanged | — | The licence *is* an OST obligation; act-3 treaty is about personhood *and* title, separately (11). |
| Human Respect Act; germline prohibition | NOTES | **I** | a choice per polity | — | Storylets, not constants. |

Reading the table: the *shape* of the belt — the calendar, the dose, the
shielding split, the chokepoint elements, the rocket equation — is P and is
the same at T0+100. What era changes is *how many people it takes* (E),
*how much it costs the sponsor* (E), *how clever the machines are per watt*
(E, and a lot), and *what the society knows about its own bodies* (K). The
two-estate split, the coil-as-heirloom, dose-as-currency, the window
calendar, the vitamin-part dependence: all P or P-anchored. The population
thresholds, the closure decay rates, the mind half-lives, the launch-cost
patience of the sponsor: all E, and the design should expose them as era
constants rather than bake them in.

### 1.5 The void-adaptation research program

If a polity decided to study adapting humans to the belt, this is what the
program is, what it needs, how long each answer takes, and what it yields.
Grounded in NASA's Human Research Roadmap (22 named risks: radiation
carcinogenesis, SANS, bone fracture, renal stones, immune, cardiovascular,
musculoskeletal fitness, sensorimotor/vestibular, sleep/circadian,
behavioural, team, nutrition, medication stability, EVA injury, dynamic
loads, DCS, hypoxia, dust, host–microbe, in-mission medical, Earth-
independent operations, and the inactive gravity-re-exposure disc risk;
humanresearchroadmap.nasa.gov), the Twins Study (Garrett-Bakelman et al.
2019: telomere lengthening in flight then shortening below baseline after,
gene-expression changes of which ~91% reverted within six months, cognitive
decline post-flight; *recollection*), bed-rest and centrifuge work (AGBRESA
2019: 60-day head-down bed rest with 30 min/day short-radius centrifugation;
bone loss in bed rest and on ISS ~1–1.5%/month at the hip; *recollection*),
the JAXA ISS mouse centrifuge (Shiba et al. 2017: 1 g on orbit prevented the
muscle and bone loss seen at µg; the 2023 lunar-gravity follow-up, Hayashi
et al., *Communications Biology*: 1/6 g prevented soleus atrophy but not the
fibre-type shift, with bone only partially protected — *recollection, paper
not fetchable this session*), radioprotectors (amifostine, FDA 1995: a
pre-dose acute protectant with nausea and hypotension, useless for chronic
GCR; the successors — entolimod, Ex-Rad, genistein/BIO 300, metformin
repurposing — are acute-SPE or adjuvant candidates, none chronic-HZE;
*recollection*), and the comparative-genomics hopes: tardigrade Dsup
(Hashimoto et al. 2016: human cells expressing Dsup showed ~40% less X-ray
DNA damage; nucleosome-binding mechanism 2019–20), elephant TP53 (Abegglen
et al. 2015: ~20 copies, enhanced apoptosis; but "super-p53" mice show
premature ageing in some models), Bajau PDE10A (Ilardo et al. 2018: ~50%
larger spleens, a selection sweep over ~10³ years), Tibetan EPAS1 (a
~3,000–10,000-year sweep). The last two are the sobering reference class:
**natural selection needs 10²–10³ generations; the polity has three.**

**What it studies.** (1) Partial-g developmental biology: skeleton, muscle,
vestibular, cardiovascular set-points, vision (SANS), in children raised at
0, 0.38 (Mars), 0.5 (a burrow at 3 rpm/50 m) and 1 g. (2) GCR chronic
effects at 12 (burrow), 70–90 (coil hull), 265–350 (thin hull) and 450–650
(Voided) mSv/yr: cancer, cataract, CNS, cardiovascular. (3) Fertility and
pregnancy under chronic dose and reduced g. (4) Immune and microbiome drift
in a closed population. (5) Psychology and cognition of the coil-born.
(6) Low-pressure/high-O₂ acclimatisation. (7) The countermeasure arms:
training, drugs, and — behind a door — germline.

**What it needs.** *Subjects*, which is the horror at the centre: every
developmental data point is a child raised to adulthood under a condition
chosen for them, and every arm of the trial is a *people*. The Kept are the
control arm; the Unkept are the 0-g arm; Mars's children are the 0.38-g arm
the belt cannot run itself. Effect sizes of 0.5 SD at 80% power need ~60 per
arm; the pitches' polity has ~30 children. The polity's whole first century
is one under-powered trial. *A centrifuge*: the burrow is one; the hull is
the 0-g arm; a variable-g animal centrifuge (JAXA-class) is T2 hardware.
*Instruments*: the dose ledger, the bone ledger (DXA needs an X-ray tube:
T3; ultrasound densitometry on quartz piezo: T3), the `partial_g_book`.
*Generations*: 25 years each. *Records that outlive minds*: the Unforgetting
(pitches) is the lab notebook.

**How long each question takes (from first subject).**

| Question | Method | Time to a usable answer | Category of result |
|---|---|---|---|
| Vestibular tolerance to 4–10 rpm | training, habituation | 1–3 yr (adults); children faster | training |
| Adult bone/muscle at 0 g for 1–5 yr with exercise + bisphosphonate/sclerostin-antibody | cohort + ledger | 5–10 yr | pharmacology + training |
| Cataract, CNS, cardiovascular at 70–350 mSv/yr | dose ledger cohort | 10–20 yr | acceptance (deterministic effects are marks) |
| Cancer excess at chronic belt dose | cohort, needs ~10³ person-decades for ±30% | 20–40 yr (and never well-powered) | acceptance / pharmacology |
| Fertility at chronic dose; pregnancy at 0/0.5 g | bearing-contract records | 5–15 yr for fertility; 20 yr for offspring outcomes | acceptance + policy (the "come in to bear" custom is a countermeasure) |
| Partial-g development: does 0.5 g suffice for a skeleton that can bear a child, walk a well, live to 70? | one cohort raised to adulthood | **20–25 yr per data point**, 40–60 yr for late outcomes | developmental (irreversible in the individual) |
| Second-generation effects (the 0-g-born bearing 0-g-born) | two cohorts | 45–60 yr | developmental / acceptance |
| Germline package (DNA repair, bone retention) safety | edited cohort to adulthood + their children | 25 yr for phenotype; 50 yr for pleiotropy and heritable disease load | germline |
| Polygenic selection for bone density via IVG | one generation per +0.3–0.5 SD | 25 yr per step | germline-adjacent (selection, not editing) |

**What it yields.** *At 20 years* (≈ T0+35, the year-15/20 snapshots): the
training results are in (rpm tolerance, counterpressure protocols,
low-pressure acclimatisation); adult 0-g bone is *managed* at a cost in drug
supply and exercise hours; the first fertility numbers; the deterministic
marks (lens, skin) are known and priced; the first partial-g cohort is
adolescent and the book has four ambiguous entries (the pitch's "four
entries and no conclusion" is exactly right); Mars, if contact exists, holds
the 0.38-g answer and it is the belt's most valuable import. Pharmacology
against GCR: 10–30% risk reduction, probably a supply-limited vitamin drug.
*At 50 years* (≈ T0+65): the first belt-born are 40; the 0.5-g and 0-g
skeletons are characterised; the answer to "can the coil-born go down a
well?" is known (it is almost certainly no for the 0-g-born and probably
yes-with-rehab for the 0.5-g-born — the K value the sim draws); cancer
excess is visible but still ±50%; the second generation is being born; if the
genetics site was opened at ~T0+30 its first edited adults exist and the
pleiotropy bill (07's 57% asthma as the price analogy) is arriving; "healthy"
has been redefined by the culture (acceptance is a result). *At 100 years*:
three generations; the polity has a developmental phenotype (not a race: gait,
stature, bone set-point, eyes, pressure tolerance) and, if it chose, a
narrow germline package whose costs are now legible; polygenic selection has
moved one or two traits by ~1 SD; the return-to-well question has closed
into a treaty category ("human", NOTES).

**Which adaptations are which.** Training/pharmacology: vestibular, exercise,
bisphosphonates and sclerostin antibodies, hypoxia acclimatisation,
radioprotectant adjuvants, SANS countermeasures (LBNP, cuffs). Developmental
(one-way in the individual, reversible in the population by changing the
next cohort's g): gait, stature, skeleton set-point, eye, pressure tolerance.
Germline (one-way in the lineage): Dsup-class DNA-repair upregulation, extra
TP53 copies, PDE10A-class organ scaling, polygenic bone density by
selection. Acceptance: the 15–20% excess cancer mortality, cataract by
mid-life, no return down a well, a shorter sailor life, the marks. The
design's "adaptation is a one-way door, not a stat" is the right shape; the
audit adds that it is *three* doors of different widths (developmental,
germline, acceptance), and that the developmental door is opened by the
*parents'* choice of where the child grows, which is why the bearing
contract is the polity's most important legal instrument.

### 1.6 Unknown unknowns: what would break the design, ranked

Ranked by (plausibility by T0+50) × (how much it breaks). "Breaks" means a
load-bearing P constraint in §1.4 stops binding.

| Rank | Breakthrough | p(exists in the belt by T0+50) | What it breaks | Mitigation / what survives |
|---|---|---|---|---|
| 1 | **Compact fusion propulsion/power** (D-D or D-³He; deuterium is in belt water) | 15–25% (fusion base rate: 2–3× slip on 2028–35 grid claims → first grid power 2045–60; a space-rated compact drive 30–50 yr later; T0+50 ≈ 2145) | Δv scarcity, the two-currency travel model, the magsail, the window calendar's grip, the reactor-fuel chokepoint (the *only* escape from "fission is a sponsor clock") | Fusion needs superconducting coils (B, REBCO, He) and dense electronics: still vitamin-gated; a fusion drive is a sponsor/Mars heirloom before it is belt-made. Phasing and plane-change costs shrink but synodic geometry stays. Keep as an act-3 rumour and a per-seed draw. |
| 2 | **Belt-scale electronics** (maskless e-beam litho, printed/analog electronics at µm nodes, in-space epitaxy) | 25–40% that micron ICs drop from T4 (20k–100k) to T3 (500–5,000) | Vitamin-part dependence for control electronics; machine minds can reproduce (belt-born minds early); the "dying gods" become "ageing parents" | HF/fluorine, ultrapure gases and dopant chemistry remain P-gated (F 60 ppm); the minds are small and slow; the licence/personhood conflict survives intact. Highest-plausibility breaker because it is mostly bits. |
| 3 | **A real chronic GCR countermeasure** (a DNA-repair or anti-inflammatory therapy cutting HZE cancer risk ×2–5) | 20–30% (biology is fast; HZE track-structure damage is stubborn) | The two-estate split softens: sailors can raise children at 70–90 mSv/yr with a burrow-like ledger | It does not touch SPE acute deaths, cataract, CNS; it is a *drug*, a flow, a vitamin: whoever makes it holds the licence. The split becomes a supply dependency rather than a wall — arguably a better mechanic. |
| 4 | **Self-replicating industrial robotics** (Metzger's 6-generation bootstrap works) | 20–30% | Population-as-capability; the tier ladder; the sponsor's need for people at all | The automated sponsor outpost (the Metal) already models this; robots still need bearings, sensors, chips (vitamins). Turns act 1's sponsor question into "why are we here?" |
| 5 | **Cheap IVG + embryo selection** | 60% by T0 (bits-and-biology) | The founder-disease price; the genetics site's necessity; MVP management | Selection replaces editing: the "whose children" split moves from *editing* to *choosing*; the horror is intact, relocated. Treat as present at T0. |
| 6 | **Ambient-pressure high-T_c superconductor wire-able from belt elements** | 5–10% | Helium leash, the coil heirloom, magsail cost, cryocooler mass | Coil structure mass (B²V) is still P; still needs a fab. Low plausibility; high break. |
| 7 | **The partial-g answer arrives** (a knowledge event: 0.38 g suffices, or it does not) | 50% we *know* by T0+50 (Mars's children) | Not a break: a resolution. If 0.38–0.5 g suffices, burrows spin slower and cheaper and stopping-the-spin is less final; if not, spin is sacred | Design should draw the K value per seed and let Mars sell the answer. |
| 8 | **Healthy-lifespan extension +10–20 yr** (senolytics, partial reprogramming) | 30% | Sailor dose economics (more years to spend), the Voided's price, the tithe | Makes dose-as-currency *more* valuable, not less. |
| 9 | **Optical comm at Gbps** | 80% by T0 | "Bandwidth starvation" as bits | Already folded in: rename to relay/attention starvation. |
| 10 | **Cheap enrichment / breeding in the belt** | <5% | Fission chokepoint | No U/Th (P). Not a breaker; fusion (#1) covers the niche. |

The design's constraints that are *one breakthrough from collapse*: Δv
scarcity (fusion), the vitamin-part dependence (belt electronics), and the
two-estate split (a GCR drug). The constraints that are *not*: the window
calendar's geometry, the shielding mass for families, the chokepoint
elements, the social floors, light-time. Note that #2, #3 and #5 are all
bits-heavy and therefore the ones a 2026 forecaster is most likely to
under-predict.

## 2. Numbers the sim needs

Era constants, intended for an `era.yaml` keyed by game year with linear
interpolation between anchor years; per-seed draws where marked. Values are
central estimates from §1.3; the `now` column is a sanity anchor, not a game
state.

| Constant | now (2026) | T0 (2095) | T0+50 | T0+100 | Draw? |
|---|---|---|---|---|---|
| `T0_year` (never printed) | — | 2095 | — | — | fixed |
| `launch_cost_leo_usd_kg` | 630 | 200 | 60 | 40 | ±×2 log-normal |
| `belt_delivery_cost_usd_kg` (cargo, slow) | 4×10⁵ | 5,000 | 1,000 | 500 | ±×2 |
| `person_year_belt_usd_M` | — | 8 | 1.5 | 0.8 | ±×2 |
| `nep_specific_mass_kg_kwe` | 45 (design) | 28 | 15 | 12 | ±30% |
| `tug_accel_mm_s2` (500–1,000 t) | 0.075 | 0.25 | 1.0 | 1.5 | derived |
| `ntr_available` | 0 | 0.5 | 0.7 | 0.8 | Bernoulli per seed |
| `fusion_drive_available` | 0 | 0.05 | 0.2 | 0.35 | Bernoulli per seed; arrival year uniform in window |
| `coil_mass_t_per_m3_20Tm` | 0.7 | 0.42 | 0.3 | 0.28 | ±20% |
| `coil_set_cost_rel` (2026 = 1) | 1 | 0.2 | 0.08 | 0.06 | ±×2 |
| `coil_life_yr` (characteristic) | 15 | 22 | 30 | 35 | Weibull, k≈3 |
| `cryocooler_w_per_w_20K` | 150 | 60 | 35 | 30 | ±30% |
| `closure_water / o2 / food` (%) | 98 / 50 / 0 | 99.2 / 92 / 70 | 99.5 / 95 / 90 | same | food ±20 |
| `ls_hardware_t_per_person` | 1–3 | 2.0 | 1.5 | 1.3 | ±30% |
| `spares_kg_per_person_yr` | 235 | 100 | 60 | 50 | ±40% |
| `crop_area_m2_per_person` | 50 | 32 | 25 | 22 | ±25% |
| `led_efficacy_umol_J` | 2.5 | 4.0 | 4.5 | 4.6 | fixed ceiling 5 |
| `tier_pop_mult` T2 / T3 / T4 | 1 / 1 / 1 | 0.6 / 0.6 / 0.8 | 0.4 / 0.4 / 0.6 | 0.3 / 0.3 / 0.5 | ±30% each |
| `compute_per_watt_rel` (2026 = 1) | 1 | 10³ | 10⁴ | 3×10⁴ | ±×10 |
| `frontier_mind_kw` (sponsor, elastic) | 1–10 | 1–10 | 1–10 | 1–10 | fixed: capability sized to power |
| `mind_2026_capability_w` | 1,000–10,000 | 10–100 | 1–10 | 1 | derived |
| `cots_attrition_belt_pct_yr` | 5–30 | 3–15 | 2–10 | 2–8 | per seed, log-uniform |
| `compute_half_life_yr` | 2–12 | 5–20 | 7–30 | 8–35 | derived |
| `robot_labour_mult` (people per capability) | 1 | 0.5 | 0.35 | 0.3 | ±30% |
| `robot_vitamin_flow_mult` (imports per capability) | 1 | 1.5 | 2 | 2 | ±30% |
| `germline_enhancement_legal` (sponsor state) | no | no (p 0.7) | no (0.6) | — | Bernoulli |
| `ivg_available` | no | yes (0.6) | yes (0.9) | yes | Bernoulli |
| `edit_offtarget_rate` | 0.1 | 0.005 | 0.001 | 0.001 | fixed |
| `gcr_drug_risk_factor` | 1.0 | 0.85 (p exists 0.3) | 0.65 (0.5) | 0.6 | Bernoulli + value |
| `bone_adult_0g_managed_yr` | 0.5 | 3 | 10 | 10 | fixed |
| `partial_g_threshold_g` (K) | unknown | unknown | unknown | unknown | **per seed, uniform 0.2–0.9**; learnt at 20–25 yr per cohort |
| `cancer_coeff_uncertainty_mult` | ×2 | ×1.3 | ×1.2 | ×1.15 | shrinks with ledger size |
| `optical_downlink_mbps_3AU` | 1 | 100 | 3,000 | 5,000 | ±×3 |
| `accords_bloc_count` | 2 | 2–3 | 3 (Mars) | 3–4 | fixed |
| `trl_regress_rate_belt` (levels lost per decade a capability is unexercised) | — | 1 | 1 | 1 | fixed; the NERVA rate (6 → ~3 in 20 yr) |
| Physics constants (GCR, shielding curves, Isp table, Δv map, light-time, abundances, radiative cooling) | as 01/02/03/10/11 | identical | identical | identical | **never varied** |

Wright's-law parameters for anything the sim wants to learn endogenously:
`learning_rate` aerospace 0.15, PV 0.20, tape 0.15–0.20, bespoke-nuclear 0
to −0.05, per doubling of cumulative units; a coil set built once a decade
gets zero doublings.

## 3. Implications for mechanics

1. **Era constants as data, physics as code (all acts).** Everything tagged
   P in §1.4 is a constant in the core; everything tagged E is a row in
   `era.yaml` interpolated by game year, and everything tagged K is a hidden
   per-seed draw that the *society* learns through ledgers. Motivation: the
   audit shows the design's shape is physics and its numbers are economics;
   confusing the two is how a sim ages. This is also what makes the
   "settled assumptions break" thesis testable: rerun with T0 = 2070 and
   2130 and see which storylets still fire.
2. **Never print the year (acts 1–3).** The chronicle says "the second
   century of spaceflight", counts, convoys, Silences. Motivation: §1.1's
   T0 is a bracket; a printed year invites the player to litigate it.
3. **Rename bandwidth starvation to relay-and-attention starvation (act 1).**
   The scarce resource is a relay satellite nobody replaces and dish time
   at a 50%-oversubscribed network (04), not bits per second (row 32). The
   licence heartbeat and audit uplink (11) are kilobytes; they fail because
   nobody is *listening*, which is the emotionally correct failure.
4. **The dying gods are ambitious, not expensive (acts 1–2).** With compute
   ×10³ per watt at T0, a 2026-frontier mind runs on a desk lamp; the
   sponsor's minds are kW-class because they were sized to the reactor.
   Mechanically: `mind.capability` should be elastic in `power_kW`, and a
   throttled god should degrade gracefully into something that still out-
   thinks any belt-made child. Motivation: §1.3 row 22; 11's elastic case.
5. **Per-tier era multipliers, not one automation scalar (acts 2–3).** T2/T3
   thresholds fall ×0.4 by T0+50; T4 falls ×0.6 and pulls harder on vitamin
   flow (`robot_vitamin_flow_mult`). Motivation: automation trades people for
   bearings and chips; the belt can make the people.
6. **TRL regress as a belt mechanic (act 2).** Every sponsor capability the
   polity does not exercise loses a level per decade (the NERVA rate); to
   re-exercise it costs the TRL 3→6 base rate in years and a population
   commitment. Motivation: §1.2.3; 03's skills-decay open question; the
   heirloom coil's *maintenance knowledge* is the thing that dies first.
7. **The research program is a set of ledgers with 20-year latency (acts
   2–3).** `dose_ledger`, `bone_ledger`, `partial_g_book`, `fertility_book`:
   each gains one entry per person-life, and the hidden K values (partial-g
   threshold, cancer coefficient) are revealed to the ring as the ledgers
   cross power thresholds (~30 entries per arm for a rough answer, ~60 for a
   usable one). The Unforgetting is the notebook; if the mind dies, entries
   lose provenance. Motivation: §1.5's per-question timescales.
8. **Mars sells the 0.38-g answer (act 2).** Mars's children are the
   partial-g arm the belt cannot afford; the first Mars contact should carry
   the offer of that data as a trade good priced against belt nitrogen.
   Motivation: §1.5; NOTES' Mars-as-mirror.
9. **Three adaptation doors of different widths (acts 2–3).** Developmental
   (opened by where the parents let the child grow — the bearing contract),
   germline (the site), acceptance (the ledger reads what it reads). The
   bearing contract is the polity's constitution. Motivation: §1.5's
   classification.
10. **IVG at T0 makes the genetics-site choice "select or edit" (act 3).**
    With IVG likely present (row 27), founder disease is manageable by
    selection at the price of choosing children; the site's germline package
    is the *fast* door with pleiotropy as its price. Two horrors, one
    storylet. Motivation: §1.6 #5; 07's MVP disagreement dissolves.
11. **The GCR drug as a licence (act 3).** If the per-seed draw gives a
    chronic countermeasure (p≈0.5 by T0+50), it arrives as a *supply* from
    Mars or the inner system, and whoever controls it controls whether
    sailors may raise children. Motivation: §1.6 #3: a breakthrough that
    would erase the split instead relocates it.
12. **Breakthrough rumours as act-3 seeds (act 3).** `fusion_drive_available`
    and `belt_electronics_available` are drawn at game start with §1.6's
    probabilities and an arrival year; when they arrive they do so *in the
    inner system first* and reach the belt as heirlooms, so the last act's
    negotiation is over access to a technology that makes the belt's
    physical leverage (volatiles, Δv) worth less every window. Motivation:
    §1.6 ranking; 05's subsidy-ratchet logic in reverse.
13. **Tug acceleration sets the tempo; make it an era row, then tune (acts
    1–3).** 01's 10× tempo question resolves to 0.1–0.5 mm/s² at T0
    (sponsor tugs) and ~1 mm/s² by T0+50 for whoever has reactors — which
    is not the belt after the cores burn down. The belt's own tempo is
    steam and sail: 02's numbers unchanged. Motivation: row 7.

## 4. Open questions

- **T0 = 2095 vs a deliberately earlier, harsher T0 (≈2070).** An earlier
  start makes the sponsor a state with a flag, coils non-existent, and Mars
  a base not a peer — a different, meaner game that some of NOTES' fiction
  (the Voided, the ordeal) would fit as well. Design decision; the era table
  supports both.
- **Whether machine-mind capability at T0 makes the human advisor ring
  redundant.** At 10³× compute per watt, a sponsor mind is a strategic
  planner, not an ECLSS controller. The HRA's de-personalisation and
  resets are the fiction's answer; the sim needs a rule for *what a
  compliant mind may advise on*, or the ring is decorative.
- **Robot capability numbers** are placeholders pending the parallel
  robotics report; rows 21/25 and `robot_*_mult` should be replaced by its
  values.
- **The launch learning-rate fit** is from memory (F9 2017 cost); a proper
  fit needs a cumulative-mass-to-orbit series (CSIS dataset, not fetchable
  as text) and SpaceX cost disclosures.
- **REBCO tape price history and the JAXA lunar-g mouse paper** could not
  be fetched (search budget, paywall redirect); both are marked recollection
  and should be verified before their rows become constants.
- **Should K values be drawn per seed or fixed?** 02 asked this for
  partial-g; §1.5 argues per seed *and* revealed slowly, which is more work
  for the storylet layer (advice must be conditioned on ledger state, not on
  the hidden value). Needs a decision on whether advisors can be wrong
  about biology in a way the player can later verify.
- **How the Human Respect Act treats germline and machine personhood in one
  instrument** is asserted by NOTES; the legal reference class (WHO 2021
  governance framework; the Oviedo Convention Art 13; the EU AI Act) is
  three separate instruments. One treaty or three is a plot decision with
  act-3 consequences.
- **Fusion's arrival mechanism.** If drawn present, does it arrive as an
  inner-system heirloom (this report's assumption) or can Mars build it? A
  Mars fab at T0+60 (03) makes the latter possible; that changes act 3's
  balance of power sharply.

## 5. Sources

- Wikipedia, *Artemis III* — https://en.wikipedia.org/wiki/Artemis_III — February 2026 redesignation to a LEO demo; first landing moved to Artemis IV, "early 2028". The slip base rate.
- Wikipedia, *Artemis Accords* — https://en.wikipedia.org/wiki/Artemis_Accords — 71 signatories as of 31 Aug 2026; per-year signing counts; Russia/China ILRS; Moon Agreement harmonisation.
- Wikipedia, *List of Starship launches* — https://en.wikipedia.org/wiki/List_of_Starship_launches — 13 flights to July 2026; no propellant transfer yet; test-article deployments only.
- Wikipedia, *NERVA* — https://en.wikipedia.org/wiki/NERVA — Rover/NERVA timeline; XE-Prime 246 kN, 710 s, 28 restarts (1969); cancelled 1973; DRACO cancelled 2025. The non-monotonic-TRL exemplar.
- Wikipedia, *Technology readiness level* — https://en.wikipedia.org/wiki/Technology_readiness_level — TRL 1–9 definitions; Sadin 1974/1989, Mankins 1995; valley of death TRL 5–7; MRL; criticisms (no obsolescence factor).
- GAO, *Technology Readiness Assessment Guide*, GAO-20-48G — https://www.gao.gov/products/gao-20-48g — TRA best practice; immature technology as the cause of cost/schedule growth (full guide not fetched).
- Wikipedia, *Experience curve effects* — https://en.wikipedia.org/wiki/Experience_curve_effects — NASA progress ratios by industry (aerospace 85%); Swanson's law 20%; BCG 10–25%.
- CSIS Aerospace, *Space Launch to Low Earth Orbit: How Much Does It Cost?* — https://aerospace.csis.org/data/space-launch-to-low-earth-orbit-how-much-does-it-cost/ — shape of the $/kg decline (gradual to 2005, steep 2005–2020); numbers are in an interactive chart.
- NASA Human Research Roadmap, *Risks* — https://humanresearchroadmap.nasa.gov/risks/ — the 22 named human-spaceflight risks used to structure §1.5.
- Hayashi et al., *Communications Biology* (2023), ISS lunar-gravity mouse study — https://www.nature.com/articles/s42003-023-04769-3 — 1/6 g partially protective (paywall redirect; recollection).
- Report 04 (this project) for Metzger 2023 (φ ≳ 35; launch elasticity; $30/kg floor), Colvin 2020, Hein 2020; Report 02 for NAS 2021 (NEP 20 kg/kWe target; NTR LH2 problem), Kilopower; Report 10 for MAARSS-I/II, CREW HaT, SR2S; Report 11 for Koomey-class efficiency trends, DSOC, EU AI Act, IMO MASS; Report 03 for tier thresholds and the AASM closure work.
- *Recollection, to verify*: Tetlock 2005/2015; Armstrong 2001; Flyvbjerg 2003/2014; Grubler 2010 and Lovering 2016 on nuclear learning; Koomey 2011/2016; Peisen et al. 1999 (NASA/SAIC TRL transition times); Garrett-Bakelman 2019 (Twins); AGBRESA 2019; Shiba 2017 (JAXA MARS 1 g mice); Hashimoto 2016 (Dsup); Abegglen 2015 (elephant TP53); Ilardo 2018 (Bajau PDE10A); Space Task Group 1969, SEI 1989, VSE 2004 targets; Shuttle 1972 cost promise; REBCO tape price series; Metaculus space-question calibration.
