# 02 — Vessels and transport

What a human-carrying vessel or habitat in the belt realistically is, and how bulk cargo actually moves. Numbers marked (calc) are my own calculations from standard physics (Hohmann/vis-viva, rocket equation, hoop stress) using the constants in the Numbers section; everything else is sourced.

## Findings

### 1. Propulsion: what actually works at 2–3 AU

**The belt is far from the Sun and far from itself.** Solar flux at 2.77 AU (Ceres) is 177 W/m², 13% of 1 AU; at 3.3 AU it is 125 W/m² (calc). Hohmann Earth→Ceres is 11.2 km/s total impulsive Δv and 1.29 years one way; Earth→2.2 AU (inner belt) is 9.4 km/s and 1.0 year (calc). Intra-belt coplanar hops are cheap in Δv (2.2→2.5 AU: 1.2 km/s; 2.5→3.0 AU: 1.6 km/s; 2.2→3.2 AU: 3.4 km/s) but slow (1.8–2.3 years per Hohmann leg) and the *synodic periods between belt orbits are enormous*: 2.2↔3.2 AU repeats every 7.6 years, 2.5↔2.77 AU every 28 years, 2.77↔3.0 AU every 41 years (calc). Two rocks at similar semi-major axes are neighbours for decades and then not for decades. The belt is not a neighbourhood; it is an ocean with very slow tides. Inclination is the hidden cost: circular speed at 2.77 AU is 17.9 km/s, so a 1° plane change costs 0.31 km/s and 10° costs 3.1 km/s (calc). Belt inclinations run 0–20°+; plane changes, not radial hops, dominate intra-belt Δv budgets.

**Chemical.** LOX/LH2 Isp ≈ 450 s, LOX/CH4 ≈ 360–380 s, storables ≈ 320 s; thrust-to-weight 10–100 (engine level). The belt's chemical propellant is water: electrolysed H2/O2 if you have the power and cryogenic storage, or plain superheated steam through a nuclear or solar heat exchanger at Isp 190–320 s for 1000–3000 K, with theoretical 640–660 s if you dissociate water at ~4000 K (Wikipedia "Steam rocket"; Thermal rocket). Cryogenic storage is the problem for anything multi-year: passive boil-off of LH2 ≈ 0.13%/day (3.8%/month), LOX ≈ 0.016%/day (0.5%/month); zero-boil-off needs a cryocooler and pays for itself beyond roughly 5–64 days in LEO (Wikipedia "Propellant depot", citing NASA tests). Storing hydrogen for a 2-year belt transit means active cooling or 60% loss. A belt fleet that runs on water-as-water (steam) or water electrolysed on demand avoids this, at the cost of Isp ≈ 300 s and mass ratio 2.8 for 3 km/s, 4.9 for 5 km/s (calc).

**Nuclear thermal (NTP).** Isp ≈ 900 s at 2700 K hydrogen (NASA baseline; Rover/NERVA demonstrated 820–875 s; Pewee 875 s). Thrust-to-weight of the engine ~3–5; DRACO flight demo (2027, LEU fuel) is a modest 10–20 kN class. NTP's Achilles heel is the same LH2 storage problem: the NAS Mars study needs 7–21 tanks of 10 t LH2 held at 20 K with "minimal boiloff" over 2 years assembly plus 2 years mission and states current cryocoolers cannot reliably do this (NAS 2021, ch. 2). NTP on water instead of hydrogen drops to ~190–350 s (the water molecule limits exhaust velocity) — that is a nuclear steam rocket, which is the realistic ISRU-fed high-thrust engine in the belt. A bimodal reactor (thrust + electricity) is attractive for exactly that reason.

**Nuclear electric (NEP).** The NAS 2021 baseline: 1–2 MWe, Isp ≥ 2000 s, thruster efficiency > 50%, total system specific mass target 20 kg/kWe (≤ 5 kg/kWe for the EP subsystem, ≤ 15 for reactor + conversion + radiators + PMAD), reactor outlet ~1200 K, 4% burnup over 4 years. State of the art is nowhere near: Prometheus/JIMO (2003–05) was 200 kWe at ~40–50 kg/kWe, with the radiator alone at 10.1 kg/kWe; Kilopower is 10 kWe at ~1500 kg unshielded (150 kg/kWe). Realistic decades-out range: 20–60 kg/kWe for MWe-class. NEP's decisive feature for this game: **the reactor fuel (LEU/HEU) is the one thing the belt cannot make** — uranium enrichment is a sponsor-controlled chokepoint. Thorium/uranium exist in asteroids but at ppb levels, and enrichment or breeding is heavy industry.

**Solar electric (SEP).** Psyche: 20 kW-class array (~18 kW at 1 AU end-of-life), four SPT-140 Hall thrusters, 0.9–4.5 kW discharge per thruster, 58.5 mN/kW measured in flight (234 mN at 4 kW), Isp ~1800 s, 1030 kg xenon total of which 885 kg is deterministic cruise. About 400 days out the array can no longer feed full thruster power; at capture near 2.7 AU the thruster runs at ~1.7 kW, and the design had to be qualified down to 0.9 kW because the mission "will need to operate from 1 AU up to 3.33 AU" (Snyder et al. 2019). Solar arrays are 7.7–15 kg/kWe at 1 AU (BVAD); at 2.77 AU that is effectively 60–115 kg/kWe (calc) — 1–5× worse than a mediocre reactor. SEP in the belt is viable for small craft and patient cargo, and it needs no fuel from Earth (xenon is a problem — krypton, argon, or iodine are the belt-plausible propellants; water-fed electrothermal thrusters at a few hundred seconds are commercially claimed but I could not verify flight numbers).

**Thrust reality for large masses (calc, η = 0.6).** 1 MWe at Isp 2000 s gives 61 N, consuming 269 kg/day; on a 500 t ship that is 0.12 mm/s² and 10 km/s takes ~950 days of continuous thrust. At 2 MWe and Isp 2500 s: 98 N, 590 days. A 10,000 t habitat with 1 MWe moves at ~0.5 km/s per year. Electric propulsion moves ships; it does not move towns.

**Solar sails.** Solar Cruiser: 1666 m², characteristic acceleration 0.17 mm/s² at 1 AU; far-term goal 1 g/m² areal density (~6 mm/s²). At 2.77 AU acceleration falls by 7.7×: 0.022 mm/s² for a Solar Cruiser-class sail, 1 km/s in 520 days (calc). Sails are a belt-manufacturable propellantless option (thin metal film) for unmanned, unhurried cargo and station-keeping, and essentially useless for anything with people on it or for the inner-belt plane changes.

**Slow-and-cheap vs. fast.** Fast (chemical/NTP, ~1–1.5 yr Earth↔belt) costs mass ratio 3–10 and Earth-made cryogenics; slow (SEP/NEP, 2–4 yr with spiral penalties; low-thrust Δv ≈ |v1 − v2| ≈ 12 km/s Earth→Ceres) costs propellant mass ratio ~1.5–2 and time, which for people is life support mass (6 kg/person-day open loop) and radiation dose (see §2). Cargo that does not breathe should always go slow. People should go as fast as the radiation budget requires, which turns out to be the binding constraint, not propellant.

### 2. Spin gravity and radiation

**Spin.** Radius for 1 g: 894 m at 1 rpm, 224 m at 2 rpm, 99 m at 3 rpm, 56 m at 4 rpm, 25 m at 6 rpm; for 0.38 g divide by 2.6 (calc). The traditional comfort limit was 6 rpm (Stone & Letko); Clément et al. 2015 report tolerance to 10 rpm with progressive exposure and up to 23 rpm after habituation of motion sickness, and that adaptation to 10 rpm is "relatively easy and quick" for repeated movements. Coriolis on a 1 m/s walk: 0.043 g at 2 rpm, 0.085 g at 4 rpm, 0.13 g at 6 rpm (calc) — annoying, not disabling. Head-to-foot gravity gradient for a 2 m person at 1 g: 0.9% at 2 rpm, 3.6% at 4 rpm, 8% at 6 rpm (calc). Kalpana One (Globus) picks 2 rpm, 250 m radius, and admits "the 2 rpm figure is" conservative; a decades-out belt design can plausibly use 3–4 rpm and 50–100 m radius, which changes the structure from a city to a tethered pair or a ring of modules. Structural cost: hull hoop stress from spinning the *shielding* dominates. For a steel hull (σ = 250 MPa) at r = 100 m, 1 g, carrying 5 t/m² of shielding, the hull needs ~160 kg/m² for spin load plus ~220 kg/m² for 70 kPa pressure; at r = 225 m it is 370 + 495 kg/m². Aluminium halves that; UHMWPE fibre makes it negligible (calc). Structure is 5–10% of shielding mass; shielding is the mass.

**Partial gravity: we know almost nothing.** Clément 2015: lunar-level (0.16 g) simulation by head-up tilt "was not effective for preventing cardiovascular deconditioning"; Mars-level (0.38 g) data are one suspended-weight gait study; vestibular perception threshold is 0.16–0.5 g; bed-rest + centrifuge studies used 1–2 g at the heart and showed cardiovascular benefit but inconclusive bone/muscle protection. There are zero human data on years at 0.2–0.5 g, and none at all on gestation or childhood. The honest simulation treats the partial-g dose–response as an *unknown drawn per game seed*, not a known curve.

**Radiation dose.** MSL/RAD measured 1.84 ± 0.3 mSv/day in cruise (inside a spacecraft, 2011–12, near solar maximum but a weak one) and 0.64 mSv/day on the Mars surface (0.21 mGy/day, Q ≈ 3.1) (Reitz/Hassler; Zeitlin 2013). Guo et al. 2015 estimate that at strong solar maximum the cruise rate could be as low as one quarter of that; solar-minimum rates are ~1.5–2× the RAD value. GCR intensity also rises slowly with heliocentric distance (radial gradient of order 2–4%/AU from Pioneer/Voyager era measurements — my recollection, not re-verified here); the belt is ~5% worse than Mars orbit. Unshielded: ~670 mSv/year at RAD's rate (calc). NASA's 2021–22 career limit is 600 mSv effective dose, all ages and sexes, set at 3% REID for a 35-year-old female (NAS 2021). **A person living behind spacecraft-grade shielding reaches the NASA career limit in about 11 months.** Terrestrial public limit is 1 mSv/yr, radiation worker 20 mSv/yr.

**Shielding.** Slaba et al. (NASA TP-2013-217983, free-space section): the first 20 g/cm² reduces dose equivalent 45–65% and effective dose 25–35%; for aluminium-like materials (and regolith, which "performs similar to aluminum") there is "no significant benefit from shielding after 45 g/cm²" and dose *rises* out to a peak just beyond 100 g/cm² from secondary neutrons; meaningful reduction needs "several hundred g/cm²". Hydrogenous materials (water, polyethylene) do better: polyethylene halves dose equivalent versus aluminium at the same areal density. Kalpana One quotes ~4.5 t/m² for adults outside flare events and 10 t/m² (Earth's atmosphere equivalent) where children are involved, and observes radiation shielding "dominates the mass of most space settlement designs". A cylinder of radius 100 m, length 200 m has ~1.9 × 10⁵ m² of hull; at 5 t/m² that is ~1 million tonnes of shielding (calc). Rules of thumb the sim can use (my synthesis, ±30%): 20 g/cm² water → ~65% of unshielded; 100 g/cm² → ~30–35%; 300 g/cm² → ~10%; 1000 g/cm² (10 m of water, 5 m of regolith) → ~2%, i.e. roughly Earth sea level (calc, from the curves above). Only the last is compatible with lifelong residence and children.

**What dose limits imply.** Crew on a thinly shielded ship (20–50 g/cm²) accumulate 300–500 mSv/year: a career of 1–2 years of transit exposure, then permanent grounding. A population that reproduces needs sub-20 mSv/year, i.e. hundreds of g/cm² — which means either a very heavy habitat or living *inside* a rock. Careers therefore split into "sailors" (mobile, dosed, short careers, adults only) and "burrowers" (immobile, shielded, families). That split is a physical fact, not a design choice.

### 3. Life support closure

**What the ISS actually does (Williamson et al. ICES 2023; BVAD 2022).** Water: UPA recovers 87% from US urine (target 90%; was 75% in 2009 after calcium sulphate precipitation killed a distiller), 70% from Russian-pretreated urine; the Brine Processor Assembly (2021) pushes urine-water recovery to 95–98% and total system recovery to ~98%. 2008–2018 the UPA produced 16,175 kg of distillate against 2,215 kg of water resupply. But the WPA's expendable multifiltration beds and ion exchange bed still cost ~472 kg/year of resupply for 6 crew, and brine is disposed of with its container. Oxygen: OGA electrolyses water; the Sabatier CRA recovered only ~47% of O2 from CO2 and has been out of service since 2017 (the H2 is lost as methane); ESA's ACLS runs Sabatier for ~3 crew. NASA's exploration target is ≥75% O2 recovery (Bosch/plasma pyrolysis), not yet flown. In the belt O2 closure is a non-issue *if* water is abundant — split water, vent CO2 — but that turns O2 closure into a *carbon* loss, and carbon is what food is made of.

**Consumables per person-day (BVAD 2022 Table 3-x, 82 kg reference crew).** O2 0.895 kg, CO2 1.085 kg, dry food 0.80 kg (ISS packaged planning value 2.39 kg), potable water 3.217 kg plus 0.76 kg water in food; outputs: urine water 1.42 kg (range 0.8–2.45), respiration/perspiration 2.946 kg, metabolic heat ~18 MJ/day peak (208 W). Open-loop total ≈ 5.7 kg/person-day, ~2.1 t/person-year. Crew time for ECLSS maintenance on ISS-class systems: 3.0–3.3 h/day for a crew of 2–3 (Russell & Klaus), i.e. 1–1.5 h/person-day.

**Hardware mass.** ISS-class physicochemical (WRS + OGA + CDRA + racks) is of order 1 t/person for 6 crew (my estimate from rack masses; BVAD does not give a clean number). MELiSSA-derived bioregenerative for a 6-crew, 780-day Mars mission: 18.1 t, i.e. 3 t/person (Lasseur et al.). BVAD Table 4-88 (Drysdale 1999, pre-LED) gives plant growth chamber hardware ~90 kg/m² plus 20 kg/m² crops, 2.1 kW/m² lighting (HPS), 13 crew-hours/m²-year. BVAD Table 4-92: a full diet from all exploration crops needs **65 m²/person** (soybean alone 46 m² — protein is the area hog), salad + carbohydrate crops only 19.5 m²/person, salad only 1.35 m². The MELiSSA stoichiometric model (Ciurans et al. 2023) closes 12 of 14 compounds at 781 g dry food/person-day (675 g higher plants + 100 g *Limnospira*) — but tracks only CHON; phosphorus and potassium are not closed, and BVAD estimates 90–100 kg/person-year of fertiliser salts. Lighting energy: modern LEDs 1.66 µmol/J; wheat-class PPF of 1000–1500 µmol/m²-s is 0.6–0.9 kW/m²; BVAD's biomass-per-energy range is 1.6–10 g dry/kWh, so 781 g/day costs 3–20 kWe continuous per person for lighting alone (calc). Realistic: 5–40 kWe/person for a full diet, plus 65 m² × 100 kg/m² ≈ 6.5 t/person of chamber hardware. That is why nobody has closed food: BIOS-3 (3 crew, 180 days, 1972–73) recycled 85% of water, plants supplied ~25% of air, meat was imported, and it drew **400 kW** for three people; Yuegong-1 (4 crew, 370 days, 2017–18, 58 m² of crops) achieved total O2 and water recycling and grew all plants and mealworms — the best result on record, on 160 m² of floor for four people. Biosphere 2 (8 people, 1.27 ha, 1991–93) lost O2 to concrete curing and needed oxygen injection and food rationing.

**Failure modes (documented, not hypothetical).** Precipitation chemistry (CaSO₄ in the UPA distiller 2010); peristaltic tube rupture and planetary-gear failure in pumps (~2,200–3,500 h); elastomer seal leaks in catalytic reactors; vacuum leaks; biofilm and microbial contamination of water loops; multifiltration bed breakthrough; brine handling; single-crop disease in monoculture chambers; trace contaminant accumulation; "recalcitrant organic matter and precipitates" accumulating in bioreactors (Ciurans). Jones (ICES 2019) is blunt: ISS experience showed failure rates higher than predicted, ISS planners multiply by K-factors, and "achieving high reliability requires more than providing spares" — common-cause failures, design errors (Apollo 13 needed an improvised spare), and infant mortality of new systems are not fixed by redundancy. The closure fraction is not a number you buy; it is a number that decays unless maintained.

### 4. Minimum viable crew

Three different questions, three different numbers:

- **Work-time coverage.** Salotti (2020, Sci. Rep.) compares hours needed for all survival activities against available labour and gets **110** settlers for Mars with near-total ISRU; the number is driven by the "make-things-from-scratch" burden and would rise sharply with less automation. This is the act-1/act-2 number: an outpost under ~100 cannot maintain its own ECLSS, power, propulsion, fabrication, medicine and agriculture simultaneously without importing spares and skills.
- **Genetics.** Marin & Beluffi (2018) get 98 (49 pairs) as the floor for a 6,300-year voyage under *strict* breeding management (32 gives 0% success). Smith (2014, Project Hyperion) argues for 14,000–44,000 for a 5-generation voyage without such management, to survive genetic drift plus one catastrophe. Conservation biology's 50/500 rule (50 to avoid inbreeding depression, 500 against drift) has an inbreeding-aware meta-analysis median MVP of ~4,000 for vertebrates. A frozen gamete/embryo bank collapses the genetic requirement to the social one (~100), at the cost of an ultra-cold freezer that must never fail for a century — a wonderful single point of failure for a game.
- **Medical capacity.** NASA's own planning estimate is one major medical event requiring a practitioner per 3-year, 5–7 person mission (~0.06/person-year); US submarine patrols show 1.9–2.3 medical evacuations per 1,000 person-months (~2.5%/person-year) and 157 acute encounters per 100,000 person-days; ANARE Antarctic stations ~2.6 illnesses and ~2 injuries per person-year (NAS "Safe Passage" 2001). A 100-person outpost should expect ~6 serious events per year and ~2–3 that on Earth would mean evacuation, with no evacuation possible. Surgery capability, imaging, and a pharmacy with 5-year shelf life become fleet parameters.

### 5. Mass drivers and cargo logistics

**Physics.** A mass driver is an electric launcher: energy per kg is ½v²; the propellant (payload) mass flow is decoupled from energy, which comes from the Sun or a reactor (NASA SP-428). The 1977 lunar baseline (O'Neill/Kolm): 10.5 kg payloads at 2.4 km/s, 4 Hz, 42 kg/s, 650,000 t/year, 1,000 g acceleration, 488 m long, 96.4% efficient, 125 MW, 3,130 t total mass (NASA SP-428 III-3). The asteroid-retrieval version (SP-428 IV-2): exhaust 8 km/s, 800 g, 4 kg/s, ~2,500 t, 19–100 MW, used as a *reaction engine* to push a 10⁶ t fragment home over 2–4 years with the asteroid's own dust as propellant; the 5.4-year baseline mission returned 535,000 t to high Earth orbit for 10,000 t launched from Earth. Mass Driver I (1977) demonstrated 30 g with liquid-nitrogen-cooled coils.

**Belt energetics (calc).** Escape velocity from a 10 km rubble pile is 6 m/s and from a 1 km body 0.6 m/s — launching off a small asteroid is free; the cost is the heliocentric transfer. Belt→Earth Hohmann from 2.77 AU: 4.86 km/s launch, 11.8 MJ/kg (3.3 kWh/kg; 3.9 kWh/kg at 85% efficiency), 1.29-year flight, **arriving at 1 AU with 6.3 km/s excess over circular** — that must be removed by the receiver. Belt→Mars: 2.83 km/s, 4 MJ/kg, 1.57 years, arrival excess 3.3 km/s. Intra-belt 2.77→2.2 AU: 1.06 km/s, 0.6 MJ/kg, 2 years. Throughput: 1 kg/s continuous at 4.9 km/s needs ~14 MWe and delivers 31,500 t/year; 10 kg/s needs 140 MWe for 315,000 t/year. For comparison an NEP tug at Isp 3000 s, η = 0.6 spends ~130 MJ per kg of *payload* for the same 5 km/s (0.185 kg propellant at 720 MJ/kg) — the mass driver is ~10× more energy-efficient and needs no ship, but the tug can brake at the far end.

**Aiming and catching is the real problem.** The 1975 lunar study needed launch velocity errors ≤ 10⁻⁴ m/s along-track and 10⁻³ m/s crosswise for a 10 kg payload to hit a 1 km net at L2 after a ~2-day flight (NASA SP-413 ch. 5; catcher 220 t, 0.43 km² catch area, 5 launches/s). Over a 1.3-year interplanetary flight (4 × 10⁷ s) those same errors become 4 km along-track and 40 km crosswise, before perturbations, outgassing, and solar radiation pressure on a small pellet. **Passive ballistic pellets across interplanetary distances cannot be caught.** Realistic bulk cargo is therefore (a) large pods (10–1000 t) with a beacon and cold-gas or electric trim thrusters doing mm/s mid-course corrections, launched by mass driver and *met* by a tug at the far end; or (b) slow tugs/sails; or (c) the SP-428 pattern: move the whole rock with a mass-driver engine. At Earth the 6.3 km/s arrival excess can be dumped by aerocapture if the pod is a ballistic ice-and-regolith brick; at a belt destination there is no atmosphere, so the receiver must spend Δv or run a mass driver in reverse (electromagnetic capture is symmetric in principle, but nobody has built one). Net: the *launch* end is cheap, the *arrival* end is where the cost and the leverage sit. Whoever owns the catchers owns the trade.

**Time.** Minimum-energy belt→Earth is 1.3–1.6 years, plus waiting for the launch window (Earth–belt synodic period 1.24–1.28 years), plus the 1–2-year lead time to mine and pack. First delivery of a newly ordered cargo is 3–4 years out; steady-state flow is continuous once the pipeline is full. Intra-belt: 2 years per hop, and windows that may not recur for decades.

**Cyclers.** An Earth–Mars Aldrin cycler needs no propulsion in the circular model, has 146–153-day legs and taxi V∞ of 4.7 km/s at Earth and 5.0 at Mars. An Earth–belt cycler in a 3-year resonant orbit (a = 2.08 AU, aphelion 3.16 AU) has V∞ ≈ 6.9 km/s at Earth and ≈ 5.1 km/s relative to a circular belt orbit at aphelion (calc), visits once every 3 years, and touches only one point of the belt. Cyclers make sense only as a sponsor–colony shuttle on a fixed route, with expensive taxis at both ends; they never make sense for belt-internal traffic.

### 6. Docking, refuelling, mobility

Autonomous docking and storable-propellant transfer are mature (Progress to Salyut/Mir/ISS since 1978; Orbital Express hydrazine transfer 2007; a Chinese satellite-to-satellite test 2016); cryogenic transfer is not (Starship intra-vehicle demo 2024, inter-vehicle demo slated 2026). The IDSS docking standard is the plausible template. In the belt the transferable commodity is *water* (storable, dense, shieldable), electrolysed or boiled at the point of use; anything cryogenic is a sponsor technology.

**Can a habitat be a ship?** Only a small one. Compare per person: a thinly shielded mobile hab (20–50 g/cm² water, ~50–100 t/person including structure and consumables) versus a shielded residential hab (5–10 t/m² of hull; at 50 m² of hull per person, 250–500 t/person). Moving 500 t/person at 1 mm/s² takes 500 kN per 1,000 people — that is a 40 t LOX/LH2-class thrust for hours per burn, or a 5 MWe NEP thrusting continuously for years at Δv ≈ 1 km/s per year. The mobile penalty is not the engine; it is that **the shielding mass you need to live is 5–10× the mass you can afford to accelerate**. The one architecture that resolves this is the SP-428 one: a habitat *inside* a 10⁴–10⁶ t rock, with a mass-driver engine eating the rock as propellant at 8 km/s. It moves at 1–3 km/s per *decade* and gets lighter as it goes. That is a nomad's yurt, not a ship: it relocates between seasons, not between ports.

Fleet reality: (i) parked, buried, breeding habitats (immobile, decades-long); (ii) crewed ships of 100–2,000 t with 20–50 g/cm² shielding, adult-only, dose-limited careers, Δv 5–15 km/s; (iii) uncrewed cargo pods and tugs at Δv 3–12 km/s and 1–4-year transits; (iv) the occasional rock-with-an-engine.

### 7. Parametric habitat/ship model

Each fleet object is a `Vessel` with the parameters below; derived quantities in the Numbers section. The intent is that "delta-v = mobility = freedom" is computed, not assigned.

**Identity / structure**
- `kind`: ship | pod | tug | habitat | rock-with-engine
- `dry_mass_t` — structure + systems, excluding shielding, propellant, consumables, cargo
- `shield_areal_gcm2` (0–1000) and `shield_material` (water | regolith | polyethylene | rock-burial). Shield mass = areal × hull area; burial sets mass 0 and mobility 0.
- `hull_area_m2`, `pressurized_volume_m3` (ISS ~25 m³/person habitable; long-duration 50–100 m³/person)
- `spin_radius_m`, `spin_rpm` → `g_level` (0 if none). Coriolis comfort penalty above 4 rpm, health unknowns below ~0.4 g.

**Propulsion**
- `engine_type`: chemical-cryo | chemical-water-steam | ntp-h2 | ntp-water | nep | sep | sail | mass-driver-engine
- `isp_s`, `thrust_kN` (or `power_kWe` and `efficiency` for electric), `engine_mass_t`
- `propellant_type` (H2/O2, water, xenon/krypton/argon, regolith) and `propellant_t`
- `boiloff_frac_per_day` (0.0013 LH2, 0.00016 LOX, 0 water) and `zbo_cooler_power_kWe`
- Derived: `delta_v_kms = isp·g·ln(m_wet/m_dry_total)`, `accel_mm_s2`, `days_per_km_s`

**Power**
- `power_source`: solar | fission | rtg; `power_kWe_1AU` (solar) or `power_kWe` (fission); `reactor_fuel_years_remaining` (sponsor chokepoint)
- Solar available = `power_kWe_1AU / r_AU²` × degradation

**Life support**
- `crew_capacity`, `crew`
- `closure_water` (0.6 open-ish … 0.98 ISS+BPA … 0.99+), `closure_o2` (0.47 Sabatier … 0.75 Bosch … 1.0 electrolysis-from-water), `closure_food` (0 … 0.2 salad … 0.55 Yuegong-1 phase 1 … 0.8–1.0), `closure_n`, `closure_pk` (fertiliser salts)
- `eclss_hardware_t` (≈ 1 t/person physicochemical + 3–7 t/person per unit of food closure), `eclss_power_kWe` (1–3 kWe/person P-C; +5–40 kWe/person for food), `crop_area_m2` (19.5 salad+carb, 65 full diet per person)
- `spares_mass_t`, `spares_fraction_per_year` (starting ~5–10% of hardware mass per year; K-factor 1.5–3 for new designs), `maintenance_h_per_person_day` (1–1.5)
- `consumables_t`, derived daily draw = crew × Σ rate_i × (1 − closure_i)
- `closure_decay_per_year` if maintenance hours or spares are short

**Radiation / health**
- `dose_rate_mSv_day` = base(solar_cycle) × shield_factor(areal, material) × (1 + 0.03·(r_AU − 1))
- Per-person `cumulative_dose_mSv`, `career_limit_mSv` (600 sponsor rule; the colony may set its own), `fertility_allowed` flag when dose rate < ~0.05 mSv/day
- `medical_level` (0 first aid … 3 surgery+imaging); event rate 0.06 major/person-year, 0.025 evacuation-class/person-year

**Crew / society**
- `min_skill_coverage` (Salotti-style hours ledger across ECLSS, power, propulsion, fab, medicine, agriculture, governance)
- `gene_bank` (bool) and `gene_bank_freezer_reliability`
- `effective_population` for genetics (census × ~0.3)

**Key relationships**
1. Shield mass ≫ everything else for any habitat with families: mass ∝ hull area × areal density; hull area ∝ r·L; living area ∝ r·L too, so shielding per person is roughly constant (Kalpana: 10 t/m² → ~20 t/person shielding + 10 structure + 10 cooling at very large scale; small habs are worse, 50–500 t/person).
2. Mobility ∝ thrust / (dry + shield + propellant); habitats with > 100 g/cm² are immobile in practice.
3. Δv purchasable with belt-made propellant (water, ~300 s) is 3–5 km/s per mass-ratio-3 tank; higher-Isp mobility needs sponsor-supplied reactor fuel or electric thrusters + big power.
4. Time in transit × dose rate × (1/shield factor) is the constraint on crewed missions, not propellant.
5. Food closure costs ~65 m², 6.5 t, and 5–40 kWe per person; water and O2 closure are nearly free where water is abundant; carbon, nitrogen, P and K are the loss terms.
6. Closure is a maintained state: without spares and hours it decays; failures are lumpy and correlated.

## Numbers the sim needs

| Parameter | Value / range | Unit | Source |
|---|---|---|---|
| Solar flux | 1361 / r² → 281 (2.2 AU), 177 (2.77), 125 (3.3) | W/m² | calc |
| Hohmann Δv Earth→2.2 / 2.5 / 2.77 / 3.0 / 3.3 AU | 9.4 / 10.4 / 11.2 / 11.7 / 12.3 | km/s | calc |
| Hohmann transit Earth→belt | 1.0–1.6 | yr | calc |
| Low-thrust Δv Earth→Ceres | ≈ 11.9 (≈ v1 − v2) | km/s | calc |
| Intra-belt hop Δv (coplanar) | 1.2 (2.2→2.5), 1.6 (2.5→3.0), 3.4 (2.2→3.2) | km/s | calc |
| Intra-belt hop time | 1.8–2.3 | yr | calc |
| Intra-belt synodic period | 7.6 (2.2↔3.2), 28 (2.5↔2.77), 41 (2.77↔3.0) | yr | calc |
| Plane change at 2.77 AU | 0.31 per degree (small angles) | km/s | calc |
| Earth–belt synodic period | 1.24–1.28 | yr | calc |
| Isp: storable / LOX-CH4 / LOX-LH2 | 320 / 370 / 450 | s | standard |
| Isp: steam thermal (1000–3000 K) / dissociated (4000 K) | 190–320 / 640–660 | s | Wikipedia Steam rocket |
| Isp: NTP-H2 | 900 (design), 875 (demonstrated) | s | NAS 2021 |
| Isp: Hall (SPT-140) / NEP target | ~1800 / ≥ 2000 | s | Snyder 2019; NAS 2021 |
| Hall thrust/power | 58.5 | mN/kW | Snyder 2019 |
| Psyche array / thruster power at 2.7 AU | 18–21 kW at 1 AU / ~1.7 kW | kW | Snyder 2019 |
| NEP specific mass: target / SOA / Kilopower | 20 / 40–50 / 150 | kg/kWe | NAS 2021; Kilopower |
| NEP radiator specific mass | 10.1 | kg/kWe | NAS 2021 |
| Solar array specific mass (1 AU) | 7.7–15 (×7.7 at 2.77 AU) | kg/kWe | BVAD |
| LH2 / LOX passive boil-off | 0.13 / 0.016 | %/day | Wikipedia Propellant depot |
| Sail characteristic accel (Solar Cruiser / far-term) | 0.17 / ~6 | mm/s² at 1 AU | search sources |
| EP thrust 1 MWe, Isp 2000, η 0.6 | 61 N, 269 kg/day | — | calc |
| Spin radius for 1 g at 2 / 3 / 4 / 6 rpm | 224 / 99 / 56 / 25 | m | calc |
| Rotation tolerance | 6 classic; 10 progressive; 23 habituated | rpm | Clément 2015 |
| Hull mass, steel, r=100 m, 1 g, 5 t/m² shield | ~160 (spin) + ~220 (70 kPa) | kg/m² | calc |
| GCR dose rate, spacecraft-shielded, cruise | 1.84 ± 0.3 (solar max ~0.5; solar min ~2.5–3) | mSv/day | RAD; Guo 2015 |
| GCR radial gradient | ~2–4 (unverified) | %/AU | recollection |
| Shield factor: 20 / 100 / 300 / 1000 g/cm² water | ~0.65 / ~0.32 / ~0.10 / ~0.02 | — | synthesis of Slaba 2013 |
| Regolith/aluminium: no benefit beyond | 45 (dose rises to peak >100) | g/cm² | Slaba 2013 |
| Shielding for adults / families | 4.5 / 10 | t/m² | Kalpana One |
| Career dose limit (NASA 2022) | 600 | mSv | NAS 2021 |
| Terrestrial public / worker limit | 1 / 20 | mSv/yr | ICRP |
| O2 / CO2 / dry food / potable water / food water | 0.895 / 1.085 / 0.80 / 3.217 / 0.76 | kg/person-day | BVAD 2022 |
| Urine water / respiration+perspiration water | 1.42 (0.8–2.45) / 2.946 | kg/person-day | BVAD 2022 |
| Packaged food ISS planning | 2.39 | kg/person-day | BVAD 2022 |
| Metabolic heat | ~150–210 | W/person | BVAD 2022 |
| ISS water recovery: UPA / total (with BPA) | 87 (90 target) / 98 | % | ICES 2023 |
| ISS O2 recovery from CO2 (Sabatier) / target | 47 / ≥ 75 | % | search; NASA |
| WPA expendables resupply (6 crew) | 472 | kg/yr | BVAD (Carrasquillo 2005) |
| ECLSS maintenance crew time | 3.0–3.3 per 2–3 crew | h/day | Russell & Klaus |
| P-C ECLSS hardware | ~1 | t/person | estimate |
| BLSS hardware (MELiSSA, 6 crew 780 d) | 3 | t/person | Lasseur |
| Plant chamber hardware / power (HPS) | ~90 kg/m² / 2.1 kW/m² | — | BVAD Table 4-88 |
| LED efficacy | 1.66 | µmol/J | BVAD (Nelson & Bugbee) |
| Biomass per energy | 1.6–10 | g dry/kWh | BVAD |
| Crop area: salad / salad+carb / full diet | 1.35 / 19.5 / 65 | m²/person | BVAD Table 4-92 |
| Diet, dry | 781 (675 plants + 100 algae), 3000 kcal | g/person-day | Ciurans 2023 |
| Fertiliser salts | 90–100 | kg/person-yr | BVAD |
| Yuegong-1 | 4 crew, 370 d, 58 m² crops, O2/water 100%, plants+mealworms | — | Wikipedia |
| BIOS-3 | 3 crew, 180 d, 400 kW, water 85% | — | Wikipedia |
| Min settlers (work-time) | 110 | people | Salotti 2020 |
| Min crew (genetic, managed) / (unmanaged, 5 gen) | 98 / 14,000–44,000 | people | Marin & Beluffi; Smith 2014 |
| MVP meta-analysis (vertebrates) | ~4,000 | people | Wikipedia MVP |
| Major medical events | ~0.06 | /person-yr | NAS Safe Passage |
| Evacuation-class events (submarine) | 1.9–2.3 per 1000 person-months ≈ 0.025 | /person-yr | NAS Safe Passage |
| Mass driver lunar baseline | 42 kg/s, 2.4 km/s, 125 MW, 3,130 t, 488 m, 96% eff | — | SP-428 III-3 |
| Mass driver engine (asteroid) | 8 km/s, 4 kg/s, 2,500 t, 19–100 MW | — | SP-428 IV-2 |
| Launch accuracy needed | 1e-4 along / 1e-3 cross | m/s | SP-413 ch. 5 |
| Catcher (L2) | 220 t, 0.43 km² | — | SP-413 ch. 5 |
| Belt→Earth pellet: launch Δv / energy / arrival excess / time | 4.86 / 11.8 (3.9 kWh/kg at 85%) / 6.3 / 1.29 | km/s, MJ/kg, km/s, yr | calc |
| Belt→Mars pellet | 2.83 / 4.0 / 3.3 / 1.57 | same | calc |
| Escape velocity 10 km / 1 km body | 6 / 0.6 | m/s | calc |
| Belt cycler (3-yr) taxi V∞ Earth / belt | ~6.9 / ~5.1 | km/s | calc |
| Mobile hab budget | ≤ 50 g/cm², 50–100 t/person; residential 250–500 t/person | — | synthesis |

## Implications for mechanics

1. **Two fleet classes, physically forced (all acts).** *Sailors*: ships ≤ 2,000 t, 20–50 g/cm², adults, per-person cumulative dose ticking at 0.3–0.5 Sv/yr, career over at the limit the colony chooses (600 mSv sponsor rule in act 1; the colony can raise it in act 2 and pay in cancer decades later). *Burrowers*: habitats in or under rock, > 300 g/cm², where children can be born. The birthplace-as-culture-seed persistence rule falls out: Voidborn are burrow-born. Evidence: RAD 1.84 mSv/day; NASA 600 mSv; Slaba's several-hundred-g/cm² requirement.

2. **Δv is a currency with two mints (acts 1–3).** Water-steam/electrolysis gives the colony ~3–5 km/s per tank at 300 s from its own volatiles. Anything better needs sponsor imports: LH2 (boils away), reactor fuel (finite `reactor_fuel_years_remaining`), xenon. Act 1's sponsor tension: the ships that make you free run on fuel you cannot make. Act 2 cut-off: high-Isp mobility decays as reactor cores burn down (4% burnup / 4 years per NAS baseline is a usable clock); the colony falls back to steam and mass drivers. Act 3 leverage: whoever can enrich or breed fuel, or whoever has volatiles at the catchers, sets terms.

3. **Windows, not distances (act 2–3 map).** Model belt travel on synodic windows and inclination differences, not on a distance map. Neighbours drift apart over decades; a good window to a specific rock is an event worth an advisor announcement ("the Vesta-family conjunction in 14 years"). Evidence: computed synodic periods of 8–41 years and 0.31 km/s per degree of inclination.

4. **Cargo is a pipeline with a 3–4-year first-delivery latency (act 1 core loop).** Mass driver launch is cheap (3.9 kWh/kg) and the sponsor receives 1.3–1.6 years later; payment/credit arrives with a further ~1.3-year comm-and-return lag. Early turns are spent building the pipeline; the sponsor's pressure is about throughput (kg/s) and reliability of the *catch*, which the sponsor controls at their end. Evidence: SP-428 numbers; arrival excess 6.3 km/s at Earth; SP-413 accuracy requirements showing passive pellets cannot be caught interplanetary — cargo must be guided pods.

5. **Catcher ownership as act-3 leverage.** The expensive end of any bulk route is arrival. A faction that operates receivers (tugs, aerocapture at Mars/Earth, reverse mass drivers) taxes the flow. Confederation politics can be literally about who runs the docks. Evidence: launch KE vs arrival Δv asymmetry (calc).

6. **Closure as a maintained state with decay and lumpy failure (acts 1–2).** Each closure fraction has a hardware mass, a spares draw (start 5–10%/yr, K-factor 1.5–3 for anything new), and crew-hours (1–1.5 h/person-day). Short either and closure drifts down; failures arrive as discrete events with a catalogue drawn from ISS history (precipitation, pump tube rupture, seal leak, vacuum leak, bed breakthrough, biofilm, crop disease). Act 2 begins when the spares pipeline closes. Evidence: ICES 2023; Jones 2019; BVAD 472 kg/yr expendables.

7. **Food closure is the big investment, and protein is the hard part (act 2).** 65 m², ~6.5 t and 5–40 kWe per person; soybean alone is 46 m². Mealworm/insect protein and algae (Yuegong-1, MELiSSA *Limnospira*) cut area but shift culture. This is where "what you can make without Earth" first bites hard, and where a fission reactor for grow lights (vs. sunlight at 13%) becomes a political object. Evidence: BVAD Table 4-92; Yuegong-1; BIOS-3's 400 kW.

8. **Carbon, nitrogen, P and K are the real loss terms (act 2–3).** Where water is plentiful O2 closure is trivial (electrolysis) but every vented CO2 molecule is lost carbon; fertiliser salts run 90–100 kg/person-year. Carbonaceous-chondrite volatiles (C, N, and water) become the trade goods; a colony on a dry S-type is a customer forever. Evidence: BVAD; Ciurans 2023 (P, K not modelled); SP-428 IV-2 composition (50 kt water, 20 kt carbon compounds per 500 kt).

9. **Population thresholds as tiered goals (act 1→2→3).** ~100 for a work-time-viable outpost (Salotti); ~100 with a gene bank or ~4,000+ without for a self-reproducing Voidborn population; 10⁴+ for a confederation that can afford specialisation. Make the gene-bank freezer an explicit object with a failure probability; make the skill ledger (hours by domain) the underlying sim that the "we lack a doctor" advisor reads from. Evidence: Salotti; Marin & Beluffi; Smith 2014; MVP meta-analysis.

10. **Medical events as a Poisson stream (all acts).** ~0.06 major/person-year and ~0.025 evacuation-class/person-year; outcome depends on `medical_level`, and a sailor ship at level 0–1 loses people. Evidence: NAS Safe Passage tables.

11. **Partial-g health as a per-seed hidden parameter (act 2).** Nobody knows whether 0.3 g is enough for bone, cardiovascular, or gestation. Draw the dose–response curve from a prior at game start and let the colony discover it through outcomes; the advisor's medical officer can only report. Evidence: Clément 2015 (Moon-level tilt ineffective; no human partial-g data).

12. **Spin choice as a cultural fork (act 1–2).** 2 rpm/224 m is a city and a million tonnes; 4 rpm/56 m is a tethered pair or a ring that a small colony can build from cable and water bags, with Coriolis you learn to live with (10 rpm adaptation is documented). Voidborn at 4–6 rpm and 0.5 g would find Earth-normal habitats disorienting and Earth itself crushing. Evidence: Clément; hull-stress calc.

13. **The rock-with-an-engine as the act-2/3 "migration" move.** A colony can move its entire home at ~1–3 km/s per decade by eating its own shielding through a mass driver at 8 km/s (SP-428 IV-2: 2,500 t engine, 19–100 MW, 10⁶ t body over 2–4 years for ~5 km/s). This is a once-a-generation decision — the steppe migration analogue — and it consumes the thing that protects you. Evidence: SP-428; mobility calc.

14. **Sails and slow cargo as the belt's own technology (act 2).** Metal-film sails and steam tugs are makeable in the belt; they are useless for people and fine for 2–4-year cargo. A "patience" economy where cargo cost is measured in years rather than kilograms fits the nomad framing. Evidence: Solar Cruiser 0.17 mm/s², 7.7× penalty at 2.77 AU.

## Open questions

- **Partial gravity biology.** No human data between 0 and 1 g beyond days. Design decision: is the hidden dose–response drawn per seed, or fixed for the game? What are the prior's bounds (e.g. threshold anywhere from 0.2 to 0.9 g)?
- **GCR radial gradient and solar-cycle modulation** should be set from a proper source (Ulysses/Pioneer/Voyager radial gradients; Guo 2015 modulation model). I used ~3%/AU and a 0.25–1.5× solar-cycle band from memory and one abstract.
- **Shield-factor curve.** I synthesised the water/regolith factor table from Slaba 2013's qualitative statements and Kalpana One's 4.5–10 t/m². A tabulated effective-dose vs g/cm² curve for water and regolith at solar min/max (OLTARIS output) would replace my ±30% guesses.
- **Reverse mass-driver capture.** Physically symmetric, never built. Do we allow it in the tech tree, and at what accuracy cost?
- **Reactor fuel in the belt.** Is there any credible act-3 path to enrichment or breeding (thorium in asteroid regolith is ~tens of ppb)? If not, fission is permanently a sponsor technology and the confederation's mobility ceiling is steam + solar-electric + sails.
- **Water-fed electric thrusters.** Commercial claims (a few hundred seconds, microwave electrothermal) could not be verified from primary sources in this pass; if real, they give the belt an Earth-independent ~500–800 s option that changes item 2 above.
- **What Δv does "freedom" actually require?** With windows and inclinations dominating, the right mobility metric may be "how many rocks are reachable within N years at ≤ X km/s", not raw Δv. The sim should compute reachability sets.
- **Habitat pressure.** Kalpana One uses Shimla-altitude pressure (~70 kPa); lower pressure at higher O2 fraction cuts hull mass and raises fire risk. Design decision with cultural flavour (Voidborn lungs).
- **Spares vs. fabrication.** At what point does in-situ manufacture (metal printing from Fe-Ni) substitute for the spares pipeline? That belongs to the industry/capability report, but the closure-decay mechanic depends on it.

## Sources

- NASA SP-428 *Space Resources and Space Settlements* (1979), ch. III-3 Mass Drivers III: Engineering — https://nss.org/settlement/nasa/spaceres/III-3.html — lunar mass driver baseline: 42 kg/s, 125 MW, 3,130 t, 96% efficiency.
- NASA SP-428, ch. IV-2 Retrieval of Asteroidal Materials — https://nss.org/settlement/nasa/spaceres/IV-2.html — mass-driver reaction engine (8 km/s, 4 kg/s, 2,500 t), 10⁶ t retrieval scenario, Δv budget, transit years, returned composition.
- NASA SP-413 *Space Settlements: A Design Study* (1975), ch. 5 — https://nss.org/settlement/nasa/75SummerStudy/Chapt5.html — mass catcher, launch accuracy 1e-4/1e-3 m/s, 220 t catcher, 5 launches/s.
- National Academies, *Space Nuclear Propulsion for Human Mars Exploration* (2021), ch. 2 (NTP) and ch. 3 (NEP) — https://www.nationalacademies.org/read/25977/chapter/4 and /chapter/5 — Isp 900 s, LH2 storage problem; NEP 20 kg/kWe target, 1–2 MWe, Prometheus SOA, radiator 10.1 kg/kWe.
- DARPA DRACO program page — https://www.darpa.mil/research/programs/demonstration-rocket-for-agile-cislunar-operations — 2027 NTP demo, LEU fuel, 10–20 kN class.
- Snyder et al., "Electric Propulsion for the Psyche Mission", IEPC-2019-244 — https://electricrocket.org/2019/244.pdf — SPT-140 throttle range 0.9–4.5 kW, 58.5 mN/kW, xenon budget, power vs range to 3.33 AU.
- Kilopower/KRUSTY paper (Nuclear Technology 2020) — https://www.tandfonline.com/doi/full/10.1080/00295450.2020.1722554 — 10 kWe at ~1,500 kg unshielded, 15-year life.
- Wikipedia, *Propellant depot* — https://en.wikipedia.org/wiki/Propellant_depot — boil-off 0.13%/day LH2, 0.016%/day LOX; refuelling history (Progress, Orbital Express 2007, Starship 2024/2026).
- Wikipedia, *Steam rocket* / *Thermal rocket* — https://en.wikipedia.org/wiki/Steam_rocket — water Isp 190–320 s (1000–3000 K), 640–660 s dissociated.
- Solar Cruiser / sail parameter sources (search summary; ScienceDirect topic page) — https://www.sciencedirect.com/topics/mathematics/solar-sail — 0.17 mm/s² for 1,666 m²; 1 g/m² far-term.
- Clément, Bukley & Paloski, "Artificial gravity as a countermeasure…", Front. Syst. Neurosci. 2015 — https://www.frontiersin.org/journals/systems-neuroscience/articles/10.3389/fnsys.2015.00092/full — rpm tolerance (6/10/23), partial-g ignorance, bed-rest centrifuge results.
- Globus & Arora, *The Kalpana One Orbital Space Settlement Revised* (2007) — https://nss.org/wp-content/uploads/Kalpana-One-2007.pdf — 250 m, 2 rpm, 4.5–10 t/m² shielding, ~7 Mt total, cylinder minimises shielding per living area.
- Reitz, Hassler et al., MSL-RAD ICES 2016 — https://elib.dlr.de/106090/1/ME-SBA-2016-Reitz_MSL-RAD_ICES_2016.pdf — 1.84 mSv/day cruise, 0.64 mSv/day surface, Q = 3.1, several-hundred-g/cm² shielding remark.
- Guo et al., "Variations of dose rate observed by MSL/RAD in transit to Mars", A&A 2015 — https://arxiv.org/abs/1503.06631 — solar-maximum rate could be ¼ of RAD's measurement.
- Slaba et al., *Radiation Shielding Optimization on Mars*, NASA TP-2013-217983 — https://spaceradiation.larc.nasa.gov/nasapapers/NASA-TP-2013-217983.pdf — first 20 g/cm² cuts 45–65% dose equivalent; no benefit past 45 g/cm² for Al-like; peak >100 g/cm²; polyethylene 2× better.
- National Academies, *Space Radiation and Astronaut Health* (2021) — https://www.nationalacademies.org/read/26155/chapter/5 — 600 mSv career limit, 3% REID, 35-year-old female basis.
- NASA *Life Support Baseline Values and Assumptions Document* Rev 2 (2022) — https://ntrs.nasa.gov/api/citations/20210024855/downloads/BVAD_2.15.22-final.pdf — all per-person-day consumables, Table 4-88 chamber ESM, Table 4-92 crop area per person, lighting efficacies, array kg/kWe, fertiliser.
- Williamson et al., "Status of ISS Water Management and Recovery", ICES-2023-097 — https://ntrs.nasa.gov/api/citations/20230006217/downloads/ICES%202023-097%20Status%20of%20ISS%20Water%20Management%20and%20Recovery.pdf — UPA 87%, BPA to 98%, Sabatier down since 2017, ORU failure modes and lifetimes.
- Jones, "High Reliability Requires More Than Providing Spares", ICES-2019 — https://ntrs.nasa.gov/api/citations/20190027319/downloads/20190027319.pdf — K-factors, common-cause failures, limits of spares logistics.
- Russell & Klaus, "Maintenance, reliability and policies for orbital space station life support systems" (2007) — https://www.sciencedirect.com/science/article/abs/pii/S0951832006001050 — 3.0–3.3 h/day ECLSS maintenance for 2–3 crew.
- Ciurans et al., "Stoichiometric model of a fully closed bioregenerative life support system", Front. Astron. Space Sci. 2023 — https://www.frontiersin.org/journals/astronomy-and-space-sciences/articles/10.3389/fspas.2023.1198689/full — 781 g dry diet, CHON closure, P/K not modelled.
- MELiSSA mass estimate (18.1 t for 6 crew, 780 d) via ESA/MELiSSA summaries — https://www.esa.int/Enabling_Support/Space_Engineering_Technology/Melissa/Closed_Loop_Compartments — 3 t/person BLSS.
- Wikipedia, *Yuegong-1* and *BIOS-3* — https://en.wikipedia.org/wiki/Yuegong-1 ; https://en.wikipedia.org/wiki/BIOS-3 — best closure records to date and their power/area cost.
- Salotti, "Minimum Number of Settlers for Survival on Another Planet", Sci. Rep. 2020 — https://www.nature.com/articles/s41598-020-66740-0 — 110 settlers via work-time ledger.
- Marin & Beluffi, "Computing the minimal crew for a multi-generational space journey", JBIS 2018 — https://www.researchgate.net/publication/327174459 — 98 people under managed breeding; 32 fails.
- Smith, "Estimation of a genetically viable population for multigenerational interstellar voyaging", Acta Astronautica 2014 — https://www.sciencedirect.com/science/article/abs/pii/S0094576513004669 — 14,000–44,000 for 5 generations without gene banks.
- Wikipedia, *Minimum viable population* — https://en.wikipedia.org/wiki/Minimum_viable_population — 50/500 rule, ~4,000 meta-analysis median.
- National Academies, *Safe Passage: Astronaut Care for Exploration Missions* (2001), ch. 3 — https://www.nationalacademies.org/read/10218/chapter/5 — medical event rates (Shuttle, Mir, submarine, Antarctic), one major event per Mars mission.
- Wikipedia, *Mars cycler* — https://en.wikipedia.org/wiki/Mars_cycler — Aldrin cycler legs and V∞ 4.7/5.0 km/s, used as calibration for the belt-cycler calc.
