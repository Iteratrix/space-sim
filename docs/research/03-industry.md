# 03 — Industry and the self-sufficiency ladder

What a belt settlement can make in situ, in what order, and what it cannot.
Sections per BRIEF.md. Word budget spent on the dependency graph (2.1) and the
mechanics (3); the findings are compressed to what the graph needs.

Headline: the "ladder" is not a ladder. Closure is asymptotic. The 1980 NASA
study and everything since agree that 90-96% mass closure is reachable with a
modest industrial base, and the last 4-10% ("vitamin parts": microelectronics,
bearings, precision instruments, pharmaceuticals, fission fuel) costs one to
three orders of magnitude more raw-material throughput and a population the
belt will not have for generations. The game's capability axis should be
modelled as **closure fraction with a hard tail**, not as a tech tree you
finish.

## 1. Findings

### 1.1 ISRU basics

**Water from C-type material.** Unheated CM chondrite (Murchison) holds 9.8 +/-
0.8 wt% water, released between ~200 and 770 C, almost all of it as structural
OH in phyllosilicates (serpentine/saponite), not ice. CI material is ~10-13
wt%. Returned samples: Bennu ~80 vol% phyllosilicate, more hydrated than
Ryugu; Ryugu reported at roughly 7-12 wt% bulk H2O depending on method. The
important correction to naive "10% water" planning: telescopic 3-micron
hygrometry gives *surface* water contents for C-complex main-belt asteroids
averaging **4.5 wt%** (Ceres excluded), significantly below CM meteorites, and
heated CMs drop from ~13 to ~3 wt%. Plan on 3-10 wt% recoverable, body by
body, with the surface metre depleted. Extraction is simple thermal bake-out:
heat rock to 500-700 C, condense in a cold trap. Energy is dominated by heating
the rock, not the water: ~1 MJ per kg of rock (0.6 MJ sensible + ~0.3 MJ
dehydroxylation/vaporisation of the ~10% water) with no heat recovery, i.e.
**~10 MJ per kg water at 10 wt%, ~25 MJ/kg at 4 wt%**. Optical mining
(TransAstra, tested at 12 kW / "2000 suns" at White Sands) does this with
concentrated sunlight directly, no electricity. Co-products at those
temperatures: CO2, SO2/H2S, organic volatiles; the water is sour and salty
and needs cleanup (S compounds, chlorides, organics) before electrolysis.

**Propellant.** Water electrolysis costs ~5 kWh (18 MJ) per kg H2O
practically; hydrogen is 11% of the mass; liquefying H2 adds ~10-15 kWh/kg
H2. Alkaline electrolysis (Ni electrodes, KOH) is fully in-situ-able once
nickel and potassium are available; PEM electrolysis needs Pt/Ir membranes
(import, or M-type residue). Solar-thermal steam rockets (Isp ~190 s) need
only water and a mirror and are the lowest rung of propulsion.

**Oxygen from silicates.** Metalysis-FFC molten-salt electrolysis (CaCl2,
~900-950 C) removed 96-97% of the oxygen from lunar simulant, yielding
40-45 kg O2 per 100 kg regolith and a mixed metal alloy powder
(Fe-Si-Al-Mg-Ca-Ti); molten regolith electrolysis (>1600 C) gives 20-30%
yield; hydrogen reduction of ilmenite 1-2%; carbothermal 10-20%. Blue Origin's
Blue Alchemist ran molten regolith electrolysis end-to-end to >99.999% silicon
and working solar cells from simulant (2023, NASA award $34.7M, autonomous
demo planned 2026). Both processes need CaCl2 or high-temperature electrodes
and tens to hundreds of kW; both are secondary to water in the belt, where
oxygen comes free from electrolysis, but they are the route to Si, Al, Mg, Ti.

**Bulk metals.** Iron-nickel is available two ways: M-type bodies (Fe-Ni with
5-15% Ni, 0.5% Co, ~0.2% P as schreibersite, S as troilite nodules) or
magnetically separated metal grains from ordinary chondrites (10-20% metal).
The Mond/carbonyl process is the canonical zero-gravity refiner: CO over the
metal at modest temperature and pressure forms Ni(CO)4 (readily, ~50-100 C,
1-10 bar) and Fe(CO)5 (needs higher pressure, tens to ~200 bar, ~150-200 C);
the vapours are decomposed at 150-250 C to 99.9% metal, as powder, CVD
coatings, or drawn-wire feedstock, and the CO is recycled. It needs a CO
source (carbon from C-type + oxygen), pressure vessels, and seals. The residue
is enriched in everything that does not form carbonyls: PGMs (~0.5 wt% in
residue), Co, Cu, Ga, Ge, W. Elvis (2014) puts good terrestrial PGM ore at
2-6 ppm and finds that the top ~50% of IIIAB irons match it (Ir 0.3-0.9 ppm,
total PGM ~7x Ir); the point for the game is that **PGMs are a by-product of
making iron, not a separate mine**.

**Silicates, glass, ceramics.** Basalt-type melt at ~1300 C, fast quench
(~20 C/s) for glass; glass wool for insulation; basalt/glass fibre for crude
textiles; sintered regolith ceramics for substrates and furnaces. All
solar-furnace tech. Optical-quality homogeneous glass is harder but is
17th-century technology once you have a stable furnace.

**Volatiles: C, S, Cl, P, K are fine; N is scarce-but-present; He/noble gases
are absent.** CI material is ~3.5 wt% C, 5.4% S, ~700 ppm Cl (as soluble
salts; Bennu returned halite/sylvite and Na-phosphates), ~900 ppm P, ~550 ppm
K. Nitrogen: Bennu bulk N 0.23-0.25 wt% (highest of any returned sample), CI
~0.3 wt%; Orgueil water-soluble NH4+ 0.07 wt%; Bennu hot-water extracts
release abundant ammonia. Ceres' surface spectra are best fit with ~28 wt%
ammoniated phyllosilicate in the mixture, implying roughly 1 wt% N — Ceres
(and the ammoniated/cometary-like bodies in the outer belt) is the belt's
nitrogen mine. Extraction: leach ammonium salts (cheap, ~0.07 wt% yield, so
~1.4 kt rock per tonne N) or pyrolyse/oxidise the insoluble organic matter
where most of the N sits (hot, dirty, ~0.25 wt% yield). **The brief's
"nitrogen is the scary one" is half right**: it is scary because a habitat's
buffer-gas inventory (~94 kg N2 per 100 m3 at 1 atm) leaks continuously and
cannot be closed, not because it is absent. The truly absent items are helium
(Stirling working gas, cryogenics, leak detection), argon/xenon (electric
propulsion), and the truly rare ones are Li (1.5 ppm), B (0.9 ppm), F (60
ppm), Sn (1.7 ppm), W (0.09 ppm CI, ~1 ppm in irons), In, and fission fuel
(U 8 ppb, Th 29 ppb).

### 1.2 Manufacturing and self-replication

**Additive.** ESA/Airbus printed the first metal part in orbit (Aug 2024)
with wire-fed laser DED, chosen because powder beds do not work in
microgravity; polymer FDM has flown since 2014. So the belt feedstock chain
is: carbonyl Fe/Ni -> cast ingot -> rolled/drawn wire -> DED, or carbonyl CVD
directly onto mandrels. Wire drawing needs dies (tool steel or ceramic) and a
rolling mill, i.e. machine tools. The laser is a vitamin part (diode lasers
are semiconductors); an electron-beam gun is a vacuum tube and can be made in
situ at the tube tier.

**Machine tools.** The bootstrapping order is well documented from
terrestrial "make a machine shop from scrap" practice (Gingery: lathe first,
because a lathe can make most of its own parts; then shaper, mill, grinder)
and from Open Source Ecology's 50-machine Global Village Construction Set.
What does not bootstrap easily: precision reference (gauge blocks, lead
screws, measuring instruments), rolling-element bearings, cutting tools
(tungsten carbide / high-speed steel need W, Co; substitutes are carbon or
chromium steels with worse tool life), and lubricants.

**Self-replication studies.** NASA's 1980 Advanced Automation for Space
Missions (Freitas et al.) designed a 100 t lunar seed factory replicating
itself in 1 year at 0.47-11.5 MW, and introduced the vocabulary the sim
should use: qualitative / quantitative / throughput closure; matter, energy,
information closure; "vitamin parts". Its materials analysis is the key
quantitative result: an 18-element factory design needs an extraction ratio
(kg raw material processed per kg product) of R=45 for 100% closure, but
**>90% closure is achieved at R=2-14** — the last 5-10% of closure costs
1-3 orders of magnitude more processing. Metzger et al. (2013) rerun this as
a six-generation, ~12-year, 12 t seed lunar bootstrap with explicit
"crudeness factors" (Gen 3 hardware is 1.5x heavier/worse than Earth's) and
electronics targets of 90/95/99/100% local for Gens 3-6; they concede the
early generations import "electronics boxes", Gen 3 makes boards but imports
chips, Gen 4 builds lithography machines in dust-free labs, Gen 5 makes
chips. That schedule is robotic and optimistic; treat it as the fastest
conceivable path. Ellery (Carleton, 2016) attacks the same problem by
minimising material diversity: a self-replicating 3D printer from Fe, Ni, Co
(Kovar/Fernico wire and structure), silicon-steel laminations, regolith
ceramic, glass, silicone plastics, tungsten filaments, with **vacuum-tube
electronics** because a solid-state foundry (~$10B, ~30 process steps,
extensive reagent inventories) is "totally infeasible" in a self-replicating
system; he explicitly rules out printed organic electronics too. His
component set: printed universal motors (no rare-earth magnets), RCL
circuits, tube op-amps, magnetic-core memory, selenium photocells, X-ray
tubes. Note his tungsten and cobalt assumptions are lunar; in the belt W is
~1 ppm even in metal, though filament masses are milligrams.

### 1.3 The hard tail

For each item: why it is hard, the lower-tech substitute, and a population
estimate. The population figures are my synthesis (flagged in 2.1 with
confidence); the only published anchors are Salotti (110 people for
survival-grade Mars self-sufficiency with iron, glass, ceramics, chemicals,
clothes, PV production, *no* electronics manufacturing modelled), Jensen
(2024: "a typical semiconductor plant employs 2,000 people ... the
surrounding town 40,000"), and Stross (~100 million to sustain the full
present-day specialty set, half a million for civil aviation alone).

- **Semiconductors.** A modern fab needs ~30 coupled process families,
  ultrapure gases (He, Ar, NF3, SiH4), 9N silicon, photoresist organic
  chemistry, HF (fluorine 60 ppm in the belt), photomasks, precision optics,
  vibration-isolated clean rooms, and a supplier web of thousands of firms.
  Sam Zeloof made 1,200-transistor ICs in a garage with 1970s equipment, but
  bought wafers, resist, dopants, and acids — the inputs are the industry.
  Realistic belt ladder: RCL + vacuum tubes + relays + magnetic-core memory
  (Colossus/1950s class; a BBC pentode ran 232,592 h; tubes are
  radiation-hard) -> discrete transistors/diodes (1950s, needs 6N Si and a
  small clean bench; dopant P is abundant, B is not — use Al for p-type) ->
  micron-scale ICs (Metzger Gen 5). Estimate: tubes at ~500-2,000 people;
  discrete Si at ~5,000-20,000; 5-10 um ICs at ~20,000-100,000; sub-micron
  never within the game.
- **Pharmaceuticals.** Half the ISS formulary expires within 36 months;
  peptide/protein drugs last ~6 months even refrigerated; radiation and dry
  cabin air degrade solids. Small-molecule synthesis of a few hundred
  compounds is a fine-chemicals industry (10^4+ people) but the *critical*
  short list is cheaper: ether/chloroform anaesthesia, aspirin/salicylates,
  ethanol/iodine antiseptics, penicillin-class fermentation, insulin and
  growth factors from engineered microbes. NASA's Astropharmacy (B. subtilis
  spores + His-tag purification, targets filgrastim and teriparatide) is a
  deliberately low-mass on-demand route; its dependency is imported strains
  and DNA, not heavy plant.
- **Biologics/biology stocks.** Seed banks, microbial strains, crop
  cultivars, and the human/plant microbiome are irreplaceable inventories:
  cheap to carry, impossible to recreate. Seeds survive decades at -18 C but
  need grow-out cycles and shielding; strain collections need cold and
  redundancy. Confinement narrows microbiome diversity (Mars500/ISS
  studies); Biosphere 2 lost O2 from 20.9% to 14.5% in 16 months to concrete
  carbonation — biology's failure modes are chemical and slow.
- **Precision optics.** Grinding/polishing glass is Renaissance technology
  and in-situ-able early (Ellery: 90%-reflective polished nickel mirrors);
  optical-grade glass homogeneity, coatings beyond Al, and lasers (diode =
  semiconductor; CO2/HeNe = tube tech but needs He/Ne — CO2 lasers can run on
  CO2+N2 only with penalty) are the hard parts.
- **Bearings.** Rolling-element bearings are on the 1980 vitamin list and
  still belong there: hardened tool steel, sub-micron grinding/lapping,
  metrology. Substitutes: plain/journal bearings in steel or cast iron with
  silicone oil (works at low speed; bad for turbomachinery and Stirlings),
  gas bearings (need machining precision but no balls), magnetic bearings
  (need electronics). Turbopumps, Stirling convertors, gyroscopes,
  centrifuges, and machine-tool spindles are the casualties.
- **Seals.** Elastomers need organic chemistry: silicone rubber (Si + methyl
  chloride; needs Cl, Cu catalyst — feasible at chemistry tier), FT-derived
  polyolefins, or plant rubber (guayule/dandelion — a biology route).
  Fluoropolymers (PTFE, Viton) need fluorine: import. Metal gaskets (soft Al,
  Ni, Cu) cover static seals early. Dynamic seals for pumps and valves are a
  steady vitamin consumption (BVAD lists seal degradation as the dominant
  dust-driven maintenance item).
- **Catalysts.** Mostly solvable: Ni (Sabatier, hydrogenation, alkaline
  electrolysis), Fe (Haber-Bosch, Fischer-Tropsch), Co (FT), PGMs from
  carbonyl residue for oxidation/PEM. The vitamin is *catalyst manufacture*
  (supports, activation) and poisoning by the belt's ubiquitous sulfur.
- **Rare elements.** Two problems: cosmic-abundance dilution with no
  hydrothermal ore-forming processes (Elvis: only ~1/2000 NEOs are PGM-ore
  by terrestrial standards), and specific holes. Practical substitutions:
  Al or Fe/Kovar wire for Cu (126 ppm CI, ~100-300 ppm in metal; extractable
  from carbonyl residue with the PGMs); Ni-Fe (Edison) or Na-based batteries
  for Li-ion; Alnico magnets (Al, Ni, Co all local) for NdFeB; Cr steels for
  W tool steels; water/regolith shielding for LiH. No substitute: He
  (Stirlings can run on H2 or N2 with a penalty), fluorine chemistry, tin.

### 1.4 Agriculture and biology

- Closure achieved: BIOS-3 (3 crew, 180 d, 95% closure, ~32.5 m2/person for
  3,000 kcal), Lunar Palace 365 (4 crew, 370 d, 98.2% material closure,
  mealworms fed on inedible biomass, crew and plants share one atmosphere).
  NASA's planning figure is **40-50 m2 of continuously cropped area per
  person**; a soybean-only diet needs 164 m2. Productivities at KSC's Biomass
  Production Chamber: wheat 39.6, potato 27.2, soybean 15.7, lettuce 7.7
  g/m2/d dry biomass.
- Lighting is the hidden cost. Delivered PPF of 500-1,000 umol/m2/s at 1.7
  (2014 LED/HPS) to ~3 (modern) umol/J and 0.8 delivery efficiency gives
  200-700 W/m2 electrical, or **~5-15 kW average electric per person**. LEDs
  are GaN semiconductors: a vitamin. The belt-native alternative is
  daylight: at 2.7 AU sunlight is 187 W/m2, ~43% PAR by energy, ~370
  umol/m2/s — roughly half of what crops want — so ~2x concentration with
  aluminised-film mirrors and glazed growing volumes gives full-rate crops
  from ~100 m2 of mirror per person with no electricity. This makes glass
  and mirror film early-tier strategic capabilities.
- Hydroponics/aeroponics first (mass, control, no soil pathogens); soil or
  soil-like substrates later for resilience and microbiome. Protein: soy and
  insects (mealworms, black soldier fly on waste; feed conversion ~1.5-2)
  before fish (water mass, disease); dairy/meat never at small scale.
- Human metabolic interface (BVAD Rev2, 82 kg reference crew): O2 0.895
  kg/d, CO2 1.085 kg/d, dry food 0.80 kg/d (12.8 MJ), drinking + food water
  3.98 kg/d, urine 1.42 + solids 0.061, feces 0.101 water + 0.032 solids,
  respiration/perspiration water 2.95 kg/d. Nitrogen through a person is
  ~13 g/d (80 g protein), ~5 kg/yr — trivially recyclable if the loop is
  closed, but every kg of standing crop biomass and compost holds ~1-3% N by
  dry mass, so a farm's N *inventory* is tens of kg per person.

### 1.5 Power

- Solar flux: 1361/r^2 W/m2 -> 281 (2.2 AU), 187 (2.7 AU), 133 (3.2 AU).
  Imported 30% cells: 56 W/m2 at 2.7 AU; in-situ Si cells (10-15%): 19-28
  W/m2. Ten kW average for a person's farm plus 3-5 kW habitat/industry
  means **300-700 m2 of in-situ PV per person** at 2.7 AU. Mirrors +
  solar-thermal (furnaces, bake-out, steam) sidestep PV for the heavy
  thermal loads, which is where most ISRU energy goes.
- Fission: KRUSTY (2018) proved the concept at 1 kWe / ~4 kWt; Kilopower
  designs 1-10 kWe, ~25% Stirling efficiency, HEU U-8Mo core (92% U, 95%
  U-235), ~260 kg for the 1 kWe unit, 10-15 yr design life. NASA's 40 kWe
  lunar FSP concept (2022): ~250 kWt HALEU (~20% enriched UN) with YH
  moderator, Na-Mo heat pipes, 8 x 6.2 kWe Stirlings on gas bearings, 1,250
  kg LiH/W shielding, basic mass 7.4 t (10 t with margins) for 10 years —
  i.e. ~250 kg/kWe, ~10 year fuel life, and the crew must stay ~1 km away or
  bury it. Mars planning assumes four to five 10 kWe units for 40 kWe.
- **Fuel supply: there is none.** Chondrites carry 8 ppb U and 29 ppb Th.
  One kilogram of uranium is 125,000 t of rock; a small reactor core needs
  tens to hundreds of kg, and then enrichment (UF6 -> fluorine, centrifuge
  cascades = precision bearings). Thorium breeding needs a reactor and
  reprocessing. Fission cores are therefore **imported, and each one is a
  10-15 year clock**. Radioisotopes are worse: Pu-238 requires reactor
  irradiation of Np-237; Sr-90 requires reprocessing fission products.
- Stirling convertors have run >30,000 h in test; they are replaceable in
  situ only at the bearing/precision tier and need He (or H2 with penalty)
  as working gas.

### 1.6 Recycling and closure

- ISS today: water ~98% recovered (UPA 87% from urine + brine processor to
  95-98%); oxygen ~50% recovered via Sabatier (hydrogen lost as vented
  methane), 75-90% with methane pyrolysis (in development). A bioregenerative
  system closes the carbon/oxygen loop through plants, which is why food
  production matters for *air*, not just calories.
- Air leakage: BVAD nominal 0.02 kg/d/module (0.01 measured on the ground,
  2x margin); ISS has actually lost 0.45-1.7 kg/d since the 2019 Zvezda
  cracks. Old structures leak more. At 78% N2 this is 6-500 kg N2 per module
  per year, and it never comes back.
- Consumables that are not water or air (BVAD/ISS values per person-year):
  clothing 84 kg without laundry, 7 kg with; hygiene/paper/wipes ~50-100 kg;
  ECLSS spares: air subsystem 0.13 kg/d (47 kg/yr), water processor 0.33
  kg/d (120 kg/yr), WPA-related resupply ~472 kg/yr for the station
  (filters, multifiltration beds, catalysts, seals, pumps, sensors); food
  2.39 kg/d as packaged (870 kg/yr) if not grown; medicines: kilograms but
  critical; electronics: kilograms but critical.
- So a 20-person ISS-technology outpost imports on the order of **25-30
  t/yr**; the same outpost with local water, a 98%-closed farm, laundry, and
  in-situ structural/metal capability imports **~3-5 t/yr**, and that
  residual is almost entirely vitamin parts (electronics, membranes, seals,
  bearings, catalysts, drugs, spacesuit soft goods, nitrogen). The mass
  curve flattens fast; the *criticality* curve does not.

## 2. Numbers the sim needs

| Parameter | Value | Unit | Note |
|---|---|---|---|
| Water content, CI / CM / heated CM | 10-13 / 9.8 / ~3 | wt% | meteorite bulk |
| Water content, C-complex MBA surface (mean) | 4.5 (range ~1-12, +/-4) | wt% | Beck 2021; Ceres excluded |
| Bake-out temperature | 300-700 | C | most water 200-770 C |
| Bake-out energy | ~1 | MJ/kg rock | no heat recovery; ~10-25 MJ/kg water |
| Electrolysis energy | 18 (5 kWh) | MJ/kg H2O | practical |
| H2 liquefaction | 36-54 (10-15 kWh) | MJ/kg H2 | |
| FFC oxygen yield | 40-45 | kg O2 / 100 kg regolith | 900-950 C, CaCl2 |
| MRE oxygen yield | 20-30 | kg O2 / 100 kg | >1600 C |
| Carbonyl process | Ni: 50-100 C, 1-10 bar; Fe: 150-200 C, tens-200 bar | | decompose 150-250 C |
| PGM in carbonyl residue | ~0.5 | wt% | Ir 0.01-100 ppm in irons; median IIIAB ~0.3-0.9 ppm Ir |
| CI abundances (Lodders 2003) | C 3.5%, S 5.4%, N 0.3%, P 900, Cl 700, K 550, Cu 126, Zn 310, F 60, Ga 10, Ge 32, Li 1.5, B 0.9, Sn 1.7, Pb 2.5, W 0.09, U 0.008, Th 0.03 | ppm unless % | verify against Lodders table |
| Bennu bulk N / C | 0.23-0.25 / 4.5-4.7 | wt% | highest N of returned samples |
| Soluble NH4+ (Orgueil) | 0.07 | wt% | leachable N |
| Ceres ammoniated clay fraction | ~28 | wt% of spectral mixture | ~1 wt% N order of magnitude |
| N2 habitat inventory | 0.94 | kg N2 per m3 at 1 atm | 94 kg per 100 m3/person |
| Air leakage, nominal / ISS actual | 0.02 / 0.45-1.7 | kg/d per module / station | BVAD, 2019-2026 reports |
| Crew O2 / CO2 / food / water | 0.895 / 1.085 / 0.80 dry / 3.98 | kg/CM-d | BVAD Rev2 Table 3-31 |
| Water recovery (ISS) | 87-98 | % | UPA alone / with brine processor |
| O2 recovery Sabatier / pyrolysis | ~50 / 75-90 | % | |
| BLSS material closure | 95 (BIOS-3), 98.2 (Lunar Palace) | % | |
| Crop area | 40-50 (32.5 min) | m2/person | soybean-only 164 |
| Crop dry productivity | wheat 39.6, potato 27.2, soy 15.7, lettuce 7.7 | g/m2/d | KSC BPC |
| Farm lighting, electric | 5-15 | kW avg/person | 200-700 W/m2 electric; 1.7-3 umol/J |
| Daylight PAR at 2.7 AU | ~370 | umol/m2/s | need ~2x mirror concentration |
| Solar flux 2.2 / 2.7 / 3.2 AU | 281 / 187 / 133 | W/m2 | |
| In-situ Si cell efficiency | 10-15 | % | imported 28-32 |
| Fission specific mass | ~250 (40 kWe FSP), ~260 kg per 1 kWe unit | kg/kWe, kg | 10-15 yr core life |
| Stirling demonstrated life | >30,000 | h | ~3.4 yr; needs He or H2 |
| U / Th in chondrite | 8 / 29 | ppb | 125,000 t rock per kg U |
| Closure vs extraction ratio | >90% at R=2-14; 100% at R=45 (18-element design) | kg processed / kg product | AASM 1980 |
| Vitamin fraction | 4-10 | % of mass | AASM; the critical not the heavy part |
| Crudeness factor, early local hardware | 1.5 (Gen 3), 1.0 (Gen 4+) | x mass/perf penalty | Metzger 2013 |
| Robotic bootstrap schedule | 6 generations x 2 yr from 12 t seed | | optimistic upper bound |
| Working-time capacity | 31.25% of hours (8,766/yr) per person | | Salotti; 25% of pop unproductive |
| Salotti activity hours (per person-yr, sharing exponent) | mining 3,802 (n^0.6); metal 5,640 (n^0.6); metal objects 5,640 (n^0.6); chemical 5,640 (n^0.6); glass/ceramics 1,880 (n^0.6); clothes 1,880 (n^0.6); PV production 1,880 (n^0.6); agriculture 940 (n^0.3); water 750 (n^0.7); air 380 (n^0.7); raising children 1,500 (n^0.3); health care 1,500 (n^0.1); education 1,500 (n^0.6) | h, exponent | minimum n = 110 |
| Clothing consumption | 84 / 7 | kg/person-yr | no laundry / laundry |
| ECLSS spares (ISS tech) | ~170-300 | kg/person-yr | air 47 + water 120 + misc |
| Packaged food if imported | 870 | kg/person-yr | 2.39 kg/d |
| Pharmacy expiry | >50% of formulary in 36 months; peptides ~6 months | | |
| Semiconductor plant workforce | ~2,000 + 40,000 town | people | Jensen 2024 |
| Full modern specialty set | ~10^8 | people | Stross |

### 2.1 Capability dependency graph

Tiers: **T0** arrives with the sponsor / trivial. **T1** first year, crew of
6-30, imported machines. **T2** years 2-10, population 50-300, local bulk
materials, imported precision. **T3** decade+, population 500-5,000, local
control electronics (tubes), chemistry, biology platforms. **T4**
generations, population 10^4-10^5, discrete semiconductors, rolling
bearings, fine chemicals. **T5** never in the game (10^7+): modern
semiconductors, fluorine chemistry at scale, fission fuel. Population is
"people whose working time is largely committed to this and its upstream
chain"; pop confidence: H = published anchor, M = reasoned from anchors,
L = my estimate. `imports` lists what keeps flowing from outside even when
the capability is "local".

```yaml
- id: regolith_mining
  tier: T1
  prereqs: []
  pop: 3            # conf M
  power_kw: 5-20
  imports: [excavator_electronics, cutting_edges, bearings]
  substitutes: {}
  notes: Bagging/anchoring on rubble-pile bodies is the hard part, not digging.

- id: solar_concentrator
  tier: T1 (imported film) / T2 (in-situ Al film + glass)
  prereqs: [regolith_mining]           # T2 version adds aluminium_silicon, glass_ceramics
  pop: 2
  power_kw: 0                          # it *is* the power
  imports: [reflective_film until T2, actuators]
  notes: Drives bake-out, furnaces, steam rockets, daylight farming. Cheapest MW in the belt.

- id: water_bakeout
  tier: T1
  prereqs: [regolith_mining, solar_concentrator]
  pop: 3
  power_kw: thermal ~1 MJ per kg rock; 100 m2 mirror at 2.7 AU -> ~1.5 t rock/day -> 50-150 kg water/day
  yield: 3-10 wt% water; co-products CO2, SO2/H2S, organics
  imports: [cold_trap_seals, sensors]

- id: water_cleanup
  tier: T1
  prereqs: [water_bakeout]
  pop: 2
  imports: [ion_exchange_resins, activated_carbon (local at T2 from C-type char), membranes]
  notes: Sulfur/chloride/organics removal; sour water kills electrolysers.

- id: electrolysis_LOX_LH2
  tier: T1 (PEM, imported) / T2 (alkaline Ni-KOH, local)
  prereqs: [water_cleanup, power_pv_imported]
  pop: 2
  power_kw: 5 kWh per kg water; 200 kW -> ~1 t water/day
  imports: [PEM_membranes_Pt_Ir until T2, compressors, cryocoolers, He for purge]
  notes: Alkaline route needs nickel (carbonyl_metals) and potassium (salt leach).

- id: steam_propellant
  tier: T1
  prereqs: [water_cleanup, solar_concentrator]
  imports: []
  notes: Isp ~190 s. First fully closed capability in the game.

- id: power_pv_imported
  tier: T0
  prereqs: []
  degrades: 1-2 %/yr plus micrometeoroid; no replacement until solar_cells_local
  notes: 56 W/m2 at 2.7 AU (30% cells).

- id: power_fission_imported
  tier: T0
  prereqs: []
  clock_years: 10-15 (core), 3-5 (Stirling convertors, replaceable at T4)
  mass_kg_per_kwe: ~250
  imports: [core, Stirlings, He working gas, LiH shielding or use local water/regolith]
  notes: Cannot be refuelled from belt material (U 8 ppb). Hard deadline.

- id: carbon_char_CO
  tier: T2
  prereqs: [water_bakeout]             # pyrolysed C-type residue; CO from C + O2 or CO2 reduction
  pop: 3
  power_kw: 20-100 thermal
  notes: Feeds carbonyl process, Fischer-Tropsch, activated carbon, steelmaking carbon.

- id: carbonyl_metals
  tier: T2
  prereqs: [regolith_mining (M-type or magnetic separation), carbon_char_CO, pressure_vessels_imported]
  pop: 5-10
  power_kw: 50-200 (thermal + compressors)
  yield: Fe, Ni 99.9% as powder/wire/CVD; residue Co, Cu, PGM ~0.5 wt%, Ga, Ge, W
  imports: [compressor_seals, valves, Ni(CO)4 toxicity monitoring electronics]
  notes: Toxic gas handling; needs seals continuously (a vitamin drain).

- id: bulk_alloys_steel
  tier: T2
  prereqs: [carbonyl_metals, carbon_char_CO, solar_concentrator]
  pop: 5-10
  products: mild/high-carbon steel, Cr steel (chromite), silicon steel, Kovar/Fernico (Fe-Ni-Co), cast iron
  cannot: tungsten/HSS tool steel (W 1 ppm), bronze (Sn 1.7 ppm), brass (Zn ok, Cu scarce)

- id: glass_ceramics
  tier: T2
  prereqs: [regolith_mining, solar_concentrator]
  pop: 3-5
  power_kw: 20-100 thermal (1300 C melt)
  products: glass, glass wool, basalt/glass fibre, sintered regolith ceramic, crude optics
  notes: Prerequisite for mirrors, tubes, windows for daylight farming, insulation.

- id: ffc_oxygen_alloy
  tier: T2
  prereqs: [regolith_mining, salt_leach (CaCl2), power >= 100 kW]
  pop: 3-5
  yield: 40-45 kg O2 per 100 kg regolith + mixed Fe-Si-Al-Mg-Ti alloy
  imports: [inert_anodes until T3]

- id: salt_leach
  tier: T2
  prereqs: [water_bakeout, water_cleanup]
  products: NaCl, KCl, CaCl2, NH4 salts, phosphates, sulfates from C-type soluble fraction
  notes: Source of Cl, K, Ca, leachable N and P. Cheap; low yield (~0.1-1 wt% total salts).

- id: aluminium_silicon
  tier: T3
  prereqs: [ffc_oxygen_alloy, salt_leach, power >= 200 kW]
  pop: 10-20
  products: Al (wire, mirror film, gaskets), Mg, Ti sponge, metallurgical Si
  imports: [electrode materials, halide chemistry consumables]

- id: silicon_5N
  tier: T3
  prereqs: [aluminium_silicon, glass_ceramics (quartz crucibles), vacuum_tube_electronics (control)]
  pop: 10-20
  notes: Zone refining / directional solidification; Blue Alchemist proved MRE -> 5N Si on simulant.

- id: solar_cells_local
  tier: T3
  prereqs: [silicon_5N, aluminium_silicon (contacts), glass_ceramics (cover)]
  pop: 20-50   # conf M
  efficiency: 10-15 %
  imports: [dopant gases early; use P (local) and Al (local, p-type) instead of B]
  notes: Ends the PV degradation clock. Metzger has this at Gen 1 (imported machines) - optimistic.

- id: machine_tools_basic
  tier: T2
  prereqs: [bulk_alloys_steel, imported_reference_tools]
  pop: 5-10
  products: lathe, mill, shaper, drill, grinder (bootstrapped Gingery-style)
  imports: [rolling_bearings, lead_screws, gauge_blocks, cutting_inserts, measuring instruments]
  notes: Precision plateaus at ~0.05 mm without imported references; enough for pumps, valves, motors, tubes.

- id: metal_am_wire_ded
  tier: T2
  prereqs: [bulk_alloys_steel (wire drawing), machine_tools_basic, laser_imported]
  imports: [laser diode (semiconductor) until electron_beam_gun at T3]
  notes: Powder-bed does not work in microgravity; wire DED proven on ISS 2024.

- id: electric_motors
  tier: T2
  prereqs: [bulk_alloys_steel (silicon steel), carbonyl_metals or aluminium_silicon (wire), glass_ceramics (insulation), machine_tools_basic]
  pop: 5
  magnets: Alnico at T3 (Al, Ni, Co local); NdFeB never
  imports: [rolling_bearings -> plain bearings substitute, magnet wire enamel -> glass/silicone insulation]

- id: plain_bearings_seals_metal
  tier: T2
  prereqs: [bulk_alloys_steel, machine_tools_basic]
  substitutes_for: rolling_bearings (low speed only), elastomer seals (static only)
  notes: Silicone oil from polymers_silicone at T3; before that, imported lubricants.

- id: vacuum_tube_electronics
  tier: T3
  prereqs: [glass_ceramics, carbonyl_metals (Ni), bulk_alloys_steel (Kovar), tungsten_trace (W from carbonyl residue, mg per tube), machine_tools_basic]
  pop: 50-200   # conf M (Ellery: feasible; scale is the question)
  capability: analog control (PID, op-amps), radio (TWT, magnetron), X-ray tubes, electron-beam guns, Colossus-class computing with relays + magnetic core memory
  cannot: dense computing, imaging sensors, LEDs, lasers (except CO2), precise timing without quartz (quartz crystal oscillators are T3-feasible)
  notes: The "Voidborn" electronics. Reliability high if never power-cycled.

- id: batteries_NiFe
  tier: T2-T3
  prereqs: [carbonyl_metals, salt_leach (KOH via K), bulk_alloys_steel]
  substitutes_for: Li-ion (Li 1.5 ppm)
  penalty: ~4x mass per kWh, 70% round-trip

- id: polymers_silicone
  tier: T3
  prereqs: [aluminium_silicon (Si), salt_leach (Cl -> HCl/MeCl), carbon_char_CO (methanol via syngas), carbonyl_metals (Cu catalyst from residue)]
  pop: 20-50
  products: silicone oils (lubricants), rubbers (O-rings, dynamic seals), resins, insulation
  notes: Ellery's chosen plastic family; radiation tolerant; needs a working chlorine loop.

- id: polymers_hydrocarbon_FT
  tier: T3
  prereqs: [carbon_char_CO, electrolysis (H2), catalysts (Fe/Co)]
  products: waxes, fuels, polyethylene/polypropylene, solvents
  power_kw: 100s
  notes: Enables textiles, films, packaging, photoresist precursors (T4).

- id: fluorine_chemistry
  tier: T5
  prereqs: [F source (60 ppm)]
  blocks: PTFE/Viton seals, HF for semiconductors, UF6 enrichment, SF6
  notes: Effectively import-only for the whole game.

- id: catalysts_local
  tier: T2 (Ni, Fe) / T3 (PGM from residue)
  prereqs: [carbonyl_metals]
  imports: [supports, promoters, replacement after sulfur poisoning]

- id: nitrogen_supply
  tier: T2 (leach NH4 salts, ~0.07 wt%) / T3 (IOM pyrolysis, ~0.25 wt%) / Ceres-class ammoniated bodies (~1 wt%)
  prereqs: [salt_leach or carbon_char_CO, water_bakeout]
  pop: 5-10
  throughput: 1.4 kt rock per t N (leach); 400 t per t N (IOM)
  demand: leakage 6-500 kg N2/module-yr + farm inventory tens of kg/person + Haber ammonia for fertiliser
  notes: The belt's strategic volatile in Act 3.

- id: ammonia_haber
  tier: T3
  prereqs: [nitrogen_supply (N2 via NH4 oxidation or direct), electrolysis (H2), catalysts_local (Fe), pressure_vessels_local]
  notes: Only needed if N arrives as N2; leached NH4 salts skip it.

- id: agriculture_hydroponic
  tier: T2
  prereqs: [water_cleanup, nitrogen_supply or imported N, salt_leach (P, K, Ca, Mg, S), lighting (LED imported OR daylight_farming), seed_bank, glass_ceramics (for daylight)]
  pop: 1 farmer per 5-10 people (Salotti: 940 h/person-yr at n^0.3)
  area_m2_per_person: 40-50
  power: 5-15 kW/person electric (LED) or ~100 m2 mirror + glazing (daylight)
  closes: carbon/oxygen loop; food 870 kg/person-yr import avoided

- id: daylight_farming
  tier: T2
  prereqs: [solar_concentrator (in-situ), glass_ceramics, aluminium_silicon (film) or imported film]
  notes: Removes the LED vitamin; needs ~2x concentration at 2.7 AU.

- id: protein_insects
  tier: T2
  prereqs: [agriculture_hydroponic]
  notes: Mealworms/BSF on inedible biomass; proven in Lunar Palace 365.

- id: aquaculture
  tier: T3
  prereqs: [agriculture_hydroponic, large water inventory]
  notes: Low priority; disease and water mass.

- id: soil_microbiome
  tier: T2 (compost/vermiculture) -> T3 (stable soil)
  prereqs: [agriculture_hydroponic, imported inoculum]
  irreplaceable_stock: true
  notes: Loss event = collapse of nutrient cycling; cannot be re-derived from Earth once cut off.

- id: seed_bank
  tier: T0
  prereqs: []
  irreplaceable_stock: true
  maintenance: -18 C, shielded, grow-out every 5-20 yr per species
  notes: Cultivar diversity shrinks each generation without deliberate breeding.

- id: textiles
  tier: T2 (glass/basalt fibre, crude) / T3 (cotton or FT polyolefin)
  prereqs: [glass_ceramics] or [agriculture (cotton, large area)] or [polymers_hydrocarbon_FT]
  demand: 7 kg/person-yr with laundry, 84 without

- id: spacesuits
  tier: T4
  prereqs: [polymers_silicone or FT, textiles, rolling_bearings (joints), precision_optics (visor), electronics]
  imports: [bladders, bearings, visors, gloves]  # the classic vitamin capital item
  notes: Salotti flags suits as mandatory local capability; nobody has shown how.

- id: pharma_critical_list
  tier: T3
  prereqs: [polymers_hydrocarbon_FT or biology fermentation, glass_ceramics (labware), catalysts_local]
  pop: 20-100  # conf L
  products: ether/chloroform, ethanol, iodine (import), salicylates, penicillin-class, saline, oral rehydration
  cannot: most modern formulary (>50% expires in 36 months)

- id: biologics_platform
  tier: T3
  prereqs: [imported strains + DNA, agriculture (media), glass_ceramics, cold storage, vacuum_tube_electronics (control)]
  pop: 10-30
  products: insulin, filgrastim, teriparatide, enzymes, vaccines (T4)
  irreplaceable_stock: strains
  notes: NASA Astropharmacy model; the stock, not the plant, is the vitamin.

- id: precision_optics
  tier: T3
  prereqs: [glass_ceramics (optical glass), machine_tools_basic, aluminium_silicon (coatings)]
  pop: 5-10
  products: telescopes, microscopes, mirrors, lithography-grade optics at T4
  imports: [lasers (diode) -> CO2 laser tubes at T3]

- id: medical_imaging
  tier: T3
  prereqs: [vacuum_tube_electronics (X-ray tube), precision_optics, quartz piezo (ultrasound)]

- id: discrete_semiconductors
  tier: T4
  prereqs: [silicon_5N (to 9N), glass_ceramics (quartz), polymers_hydrocarbon_FT (resist precursors), salt_leach (acids: HCl, HNO3 via ammonia), fluorine_chemistry (HF; or import), clean_room, precision_optics]
  pop: 5,000-20,000  # conf L; society-level, not plant-level
  products: transistors, diodes, simple photocells, 1950s-class computing
  imports: [HF, ultrapure gases, photomasks]

- id: ic_fab_micron
  tier: T4-T5
  prereqs: [discrete_semiconductors, precision_optics (lithography), machine_tools_precision, fluorine_chemistry, He/Ar supply]
  pop: 20,000-100,000  # conf L; Jensen: 2,000/plant + 40,000 town + supplier web
  notes: Metzger Gen 5; treat as the far end of Act 3.

- id: modern_semiconductors
  tier: T5
  pop: 10^7-10^8   # Stross
  notes: Out of scope for any belt population; every chip in the game is a stock, not a flow.

- id: rolling_bearings
  tier: T4
  prereqs: [bulk_alloys_steel (bearing steel), machine_tools_precision, metrology]
  pop: 1,000-5,000  # conf L
  unlocks: turbomachinery, Stirling repair, centrifuges, machine-tool spindles, spacesuit joints

- id: machine_tools_precision
  tier: T4
  prereqs: [machine_tools_basic, rolling_bearings, precision_optics (interferometry), metrology references]

- id: fission_core_local
  tier: T5
  prereqs: [U source (none), fluorine_chemistry, rolling_bearings (centrifuges)]
  notes: Never. Reactors are imported clocks.

- id: helium_supply
  tier: T5 (none in belt)
  blocks: Stirling working gas (H2 substitute), cryogenics, leak detection, welding shield gas (use Ar? also absent; use vacuum welding)
```

Gaps a realistic settlement cannot climb for generations: **fluorine
chemistry, semiconductors beyond discretes, rolling bearings and the
precision-metrology chain, fission fuel, helium, spacesuit soft goods, the
long pharmaceutical tail, and any biological stock once lost**. Everything
else is engineering time and population.

## 3. Implications for mechanics

1. **Capability = closure fraction, not a tech tree.** (All acts.) Model each
   capability node above with prerequisites, a working-hours cost, and a
   *vitamin stream* (kg/yr of imported consumables it still needs). The
   settlement's headline number is its closure fraction by criticality, not by
   mass. Evidence: AASM 90-96% closure with 4-10% vitamins; ISS 98% water but
   50% oxygen; the mass import curve collapses from ~25 t to ~4 t per 20
   people while the critical list barely shrinks.

2. **Salotti's time-budget as the core economic equation.** (Acts 1-3.)
   Population supplies 31.25% of 8,766 h per person-year; each active
   capability consumes r_i x n / n^alpha_i hours. Survival means total demand
   below capacity. This is a one-line simulation that makes population,
   specialisation, and "what we choose to make" a single trade-off, produces
   the 110-person threshold naturally, and gives the advisor ring concrete
   things to argue about (drop glassmaking to free 1,880 h?). Use the
   published r_i and alpha_i as starting values; add electronics, medicine,
   and spacesuit lines that Salotti omitted.

3. **The reactor is a clock, not a building.** (Act 1 seed, Act 2 crisis.)
   Imported fission cores run 10-15 years; Stirlings 3-5 years; PV degrades
   1-2%/yr; none are replaceable until T3 (cells) or T4 (Stirlings), and the
   core never. Act 1 decisions (how many cores the sponsor ships, whether to
   invest in mirrors/glass) set an Act 2 deadline. Solar-thermal via mirrors
   is the belt-native escape hatch and should be cheap in hours but
   power-limited by distance (187 W/m2 at 2.7 AU).

4. **Substitution with a crudeness penalty.** (Act 2.) Each vitamin has a
   local substitute at lower performance: LEDs -> mirror farming; Li-ion ->
   NiFe (4x mass); ICs -> tubes/relays (bulk, heat, no imaging); ball
   bearings -> plain bearings (no high-speed machines); PTFE seals ->
   silicone (temperature limits); NdFeB -> Alnico; W tools -> Cr steel (tool
   life). Metzger's crudeness factor 1.5 is a usable default multiplier on
   mass, power, and maintenance hours. Going Voidborn is choosing these
   substitutions and living with their culture (a tube-and-relay society with
   its own aesthetics), not merely losing things.

5. **Stocks vs flows: the chip economy.** (Acts 2-3.) Every semiconductor in
   the settlement is a non-renewable stock. Track a "chip inventory" that is
   cannibalised from failed equipment, rationed, and becomes the medium of
   exchange between belt groups in Act 3 more readily than PGMs. Evidence:
   Ellery's "totally infeasible" foundry, Metzger's Gen 5 chips only after
   four generations of imports.

6. **Nitrogen as the strategic volatile and the leak as a sink.** (Act 1
   design choice, Act 3 leverage.) Habitat volume x leak rate x 0.78 is a
   permanent N2 drain (6-500 kg/module-yr); old modules leak more (ISS 2019+).
   Nitrogen is ~0.25 wt% in C-types and ~1 wt% on ammoniated bodies like
   Ceres. Whoever holds the ammoniated bodies and the leach/pyrolysis plant
   sells air. Low-pressure habitats (BVAD allows 48-70 kPa) trade N2 inventory
   for fire risk and suit prebreathe — a real design choice for the player.

7. **Food is power.** (Act 1.) A 98%-closed farm needs 40-50 m2 and 5-15 kW
   per person, or ~100 m2 of mirror and a glazed volume. Make the farm the
   single biggest sink in the energy budget and the reason glass and
   aluminium film are early strategic capabilities. Lunar Palace 365 justifies
   a shared crew-plant atmosphere; insects on inedible biomass are the
   default protein.

8. **The last 10% costs 10x.** (Act 2-3.) AASM's extraction-ratio result:
   >90% closure at R=2-14, 100% at R=45. Implement closure as a curve where
   each additional percent of self-sufficiency demands a larger rock
   throughput and a bigger hour budget. This is the honest reason a settlement
   stays 95% closed for generations, and why Act 3 confederation (pooling
   populations across the belt to reach T4 nodes) is the only path up.

9. **Irreplaceable biological stocks with loss events.** (All acts.) Seed
   bank, strain library, soil microbiome, crop cultivars. Cheap to carry,
   impossible to recreate once cut off. Random events (freezer failure,
   contamination, radiation) permanently prune the capability graph; the
   player's redundancy investment in Act 1 (second freezer, grow-out
   discipline) is a seed for Act 2 survival.

10. **Medicine as a countdown.** (Act 2.) Half the formulary expires in 36
    months; peptides in 6. Model a decaying drug inventory that forces either
    the T3 critical-list chemistry and biologics platform (strains imported in
    Act 1!) or an accepted rise in mortality. Health-care hours in Salotti
    scale as n^0.1 — nearly linear in population — so medicine is the
    domain where scale helps least.

11. **Spares as the true resupply.** (Act 1.) With local water and a farm,
    resupply collapses to ~150-300 kg/person-yr of ECLSS spares plus
    electronics, drugs, and suit soft goods. The sponsor's leverage in Act 1
    is *these* shipments, not food; a sponsor who wants control ships
    consumables, not capabilities (Metzger's "import electronics boxes"
    versus "build lithography machines" is exactly the sponsor-vs-self
    tension in hardware form).

12. **Laundry.** (Act 1, trivial but illustrative.) 84 vs 7 kg/person-yr.
    Small closure choices with 10x effects are good early-game tutorials for
    the closure mechanic.

## 4. Open questions

- **Main-belt water at depth.** Surface hygrometry (4.5 wt% mean) versus
  meteorite bulk (10 wt%): is the first metre depleted and the interior CM-like,
  or are many C-complex bodies genuinely drier than CM? Determines whether
  the water plant number is 10 or 25 MJ/kg. Design decision: sample per body
  from a 1-12 wt% distribution and let prospecting reveal it.
- **Nitrogen extraction efficiency from insoluble organic matter.** No
  process data; the leach yield (0.07 wt%) is the only measured number. Needs
  a design call on whether IOM pyrolysis is T2 or T3.
- **Vacuum-tube ECLSS.** Can a life-support system be controlled by
  tube/relay electronics at acceptable reliability and mass? Historically
  yes (1950s-60s industrial control), but nobody has designed one. Decide the
  crudeness penalty for T3 control.
- **Stirling working gas.** Hydrogen instead of helium is possible with
  embrittlement and leakage penalties; no numbers found. Affects whether
  imported reactors are repairable at T4.
- **Population estimates for T4 nodes.** No published minimum for a
  discrete-semiconductor or bearing industry as a closed chain; my 10^3-10^5
  figures are reasoned, not sourced. The sim could instead expose these as
  tunable constants and let the design team pick where Act 3 ends.
- **Copper.** 126 ppm in CI, chalcophile, partitions to sulfides and the
  carbonyl residue; nobody has costed its recovery. If copper is genuinely a
  vitamin, most electrical plant changes; if Al/Kovar wiring is adequate,
  it is a minor penalty. Decide by fiat.
- **Skills decay and information closure.** AASM's third closure type.
  Nothing here models the loss of know-how across generations when a
  capability is dormant; KoDP-style "lore" mechanics would need a source.
- **Whether semiconductors are a hard cap or a slow T4.** Ellery says never
  in a small system; Metzger says Gen 5 in ~10 years with robotics. The
  honest middle: discretes at 10^4 people, micron ICs only by confederation.

## 5. Sources

- NASA CP-2255 *Advanced Automation for Space Missions* (1980/82), ch. 5.3
  on closure — https://en.wikisource.org/wiki/Advanced_Automation_for_Space_Missions/Chapter_5.3 — closure definitions, 90-96% / vitamin parts, extraction-ratio curve, 100 t seed factory.
- Metzger, Muscatello, Mueller, Mantovani, *Affordable, Rapid Bootstrapping of the Space Industry and Solar System Civilization*, J. Aerospace Eng. 2013 — https://arxiv.org/abs/1612.03238 — generation table, crudeness factor, electronics import schedule.
- Ellery, *Are Self-Replicating Machines Feasible?*, J. Spacecraft & Rockets 53(2) 2016 — https://carleton.ca/ceser/wp-content/uploads/Are-self-replicating-machines-feasible.pdf — minimal material set, vacuum-tube electronics rationale, motors, silicone plastics.
- Salotti, *Minimum Number of Settlers for Survival on Another Planet*, Sci. Rep. 2020 — https://www.nature.com/articles/s41598-020-66740-0 (full text via Europe PMC PMC7297723) — time-budget model, activity hour tables, sharing exponents, n=110.
- Jensen, *Design Limits on Large Space Stations* (2024) — https://arxiv.org/pdf/2408.00152 — population targets, semiconductor plant workforce anchor.
- Stross, *Insufficient data* (2010) — http://www.antipope.org/charlie/blog-static/2010/07/insufficient-data.html — 100M-person argument for full modern specialty set.
- NASA/TP-2015-218570 Rev2 *Life Support Baseline Values and Assumptions Document* (2022) — https://ntrs.nasa.gov/api/citations/20210024855/downloads/BVAD_2.15.22-final.pdf — Table 3-31 metabolic values, Table 4-1 leakage, lighting efficiencies, crop areas, clothing and spares rates.
- Ewert et al., *Astronaut Mass Balance for Long Duration Missions*, ICES-2019-126 — https://ntrs.nasa.gov/api/citations/20190027563/downloads/20190027563.pdf — per-person daily mass balance.
- *Status of ISS Water Management and Recovery*, ICES-2023-097 — UPA 87%, brine processor to 95-98%.
- SpaceNews / SpacePolicyOnline on the Zvezda leak (2019-2026) — https://spacenews.com/nasa-monitoring-increased-leak-in-russian-iss-module/ — real leak rates 0.45-1.7 kg/d.
- Beck et al., *"Water" abundance at the surface of C-complex main-belt asteroids* (2021) — https://arxiv.org/pdf/2011.00279 — 4.5 wt% mean surface water, Ceres organics/NH4.
- Glavin, Dworkin et al., *Abundant ammonia and nitrogen-rich soluble organic matter in samples from asteroid Bennu*, Nat. Astron. 2025 (PMC11842271) — Bennu N 0.23-0.25 wt%, C 4.5-4.7 wt%.
- De Sanctis et al., *Ammoniated phyllosilicates ... on (1) Ceres*, Nature 2015 — https://www.nature.com/articles/nature16172 — Ceres as nitrogen reservoir.
- Nitrogen in Orgueil (GCA 2024) — https://www.sciencedirect.com/science/article/pii/S0016703724005180 — soluble NH4+ 0.07 wt%.
- Duan et al. 2021 / NTRS 20200001812 on thermal decomposition of Murchison — 9.8 wt% water loss 200-770 C.
- Lomax et al., *The Metalysis-FFC process for the efficient extraction of oxygen on the lunar surface* (2019) — https://www.hou.usra.edu/meetings/lunarisru2019/presentations/5023_Lomax.pdf — yield comparison table.
- Blue Origin, *Blue Alchemist* (2023) — https://www.blueorigin.com/news/blue-alchemist-powers-our-lunar-future — MRE to 5N silicon and working cells.
- Elvis, *How Many Ore-Bearing Asteroids?*, PSS 2014 — https://arxiv.org/pdf/1312.4450 — PGM richness distribution, ore-grade thresholds, 1/2000 NEOs.
- Mond process (Wikipedia / Britannica) and NSS *Asteroid Resources* — https://nss.org/settlement/nasa/spaceresvol3/asterres1.htm — carbonyl conditions, C1/C2 water and organics, PGM residue.
- Lodders 2003, *Solar system abundances and condensation temperatures of the elements*, ApJ 591 — CI abundance table (values quoted from memory; verify).
- Gibson et al., *Kilopower Project: The KRUSTY Fission Power Experiment* (Nuclear Technology 2020) and Beyond NERVA Kilopower page — https://beyondnerva.wordpress.com/fission-power-systems/kilopower/ — core composition, masses, efficiency.
- Oleson et al., *A Deployable 40 kWe Lunar Fission Surface Power Concept* (2022) — https://ntrs.nasa.gov/api/citations/20220004670/downloads/40%20kW%20Deployable%20FSP%20Paper_FINAL.pdf — HALEU design, mass table, 10-year life.
- Lovering / Morgan & Lovering on U and Th in chondrites — https://pubmed.ncbi.nlm.nih.gov/18960411/ — 8 ppb U, 29 ppb Th.
- ESA, *ESA 3D prints first metal part on the ISS* (2024) — https://www.esa.int/Newsroom/Press_Releases/ESA_3D_prints_first_metal_part_on_the_International_Space_Station — wire DED in microgravity.
- TransAstra optical mining test (US Army / Leonard David, 2015) — https://www.army.mil/article/159127/ — solar-thermal volatile extraction demo.
- Fu et al. / Lunar Palace 365 reliability paper (Acta Astronautica 2022) — https://www.sciencedirect.com/science/article/abs/pii/S0094576522006294 — 98.2% closure, 370 d, mealworms.
- BIOS-3 and closed ecological systems review (Nelson, Allen, Dempster) — https://ecotechnics.edu/wp-content/uploads/2011/08/Handbook-Envt-Engineering-Closed-system-chapter.pdf — 95% closure, 32.5 m2/person.
- Wheeler et al., NASA Biomass Production Chamber — https://www.sciencedirect.com/science/article/abs/pii/027311779500880N — crop productivities, 40-50 m2/person.
- NASA, *A Flexible, Personalized, On-Demand Astropharmacy* — https://www.nasa.gov/general/a-flexible-personalized-on-demand-astropharmacy/ — B. subtilis platform, peptide shelf life.
- *Expiration analysis of the ISS formulary*, npj Microgravity 2024 — https://www.nature.com/articles/s41526-024-00414-3 — >50% expired by 36 months.
- Zeloof, *First IC* — https://sam.zeloof.xyz/first-ic/ — what a one-person 1970s-process fab can and cannot do.
- Open Source Ecology, Global Village Construction Set — https://www.opensourceecology.org/gvcs/ — 50-machine bootstrapping set; Gingery's *Build Your Own Metal Working Shop from Scrap* for lathe-first ordering.
