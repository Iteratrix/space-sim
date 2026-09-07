# 10 — Active magnetic shielding, magnetic sails, and belt-makeable superconductors

Follow-up to 02-vessels.md (the sailor/burrower split) and 03-industry.md (vitamin
parts). Three questions: can sailors live a full life aboard, is magsail thrust
meaningful at 2.2–3.3 AU, can the belt ever make its own coils. Primary sources
read in full: NASA NIAC MAARSS Phase I (2013) and Phase II (2019), ESA ARSSEM
(2012), NIAC CREW HaT Phase I (2024), Andrews & Zubrin (1990). SR2S, Spillantini,
Hoffman et al. and the plasma-magnet reports could not be fetched this session
(search budget exhausted); where I use them I say "recollection". Everything
marked (calc) is in `10-magnetics-calc.py` beside this file.

## 1. Findings

### 1.1 Active shielding: what the serious studies concluded

**The physics that sets everything.** A particle of rigidity R (GV) in a field B
(T) bends with radius r = R/(0.3·B) metres. A shield is characterised by its
bending power ∫B·dL in tesla-metres. MAARSS-I's GEANT4 runs show what that buys:
an 8 T·m system deflects 3 GV protons clear of the habitat, bends 5 GV protons
"slightly", and shows "no indication of a significant deflection of the 7 GV
protons" (MAARSS-I §6.6; kinetic-energy cutoff 4.7 GeV for protons). Dose-weighted
GCR sits at 1–10 GeV/n (rigidity 2–20 GV for protons, 4–40 GV for He and
heavier), so a practical magnet removes the lower half of the dose and leaves
the upper half untouched. Everyone who has done the calculation properly gets
the same shape: 30–50 % reduction for the first ~10 T·m, another 25 % for
2.5× more bending power, and a hard floor set by particles above ~7 GV that no
buildable magnet touches.

**The studies, with their own numbers.**

| Study | Configuration | Bending power | Superconductor, T_op | Magnet system mass | Result |
|---|---|---|---|---|---|
| Hoffman, Fisher, Batishchev 2005 (NIAC Ph. I; recollection) | toroid/solenoid trades | ~10 T·m class | LTS/HTS | ~10² t | GCR factor ~2 at most; concluded structure mass and stray field dominate; comparable passive mass |
| ESA ARSSEM, Battiston et al. 2012 (arXiv 1209.1907) | 15 toroidal geometries; racetrack toroid vs Double-Helix (12 coils, 18 m long) | 4–5 T·m | HTS tape, ~1 t per coil | 12 × 2.16 t + 4.9 t plates ×1.2 margin = **37 t**; +2 t H₂, 3.2 t thermal, <2 t power | "combined effect of passive and active shielding would provide a **40 % reduction** of the GCR dose with respect to empty space"; "up to a factor of 10 … could, in principle, be developed" |
| NASA MAARSS Phase I, Westover et al. 2013 | **6+1**: six expandable solenoids 8 m Ø × 20 m at 1 T around a 6 m × 10 m habitat, plus a compensator coil | 8 T·m | YBCO tape at 40 K; 503 kg conductor per coil | 6 coils **36 t** (conductor + strongback + blanket), **49.5 t with contingency**; 7.5 t cold mass per coil; compensator 2 t | Barrel-region BFO dose **309 mSv/yr**, total **451 mSv/yr** with "nearly naked" endcaps (solar-min 1977 spectrum). SPE (Oct 1989, 180 h): BFO ~50 mSv; a 36 t coil set equals **141 t of 75 cm polyethylene** for SPE. Raising to 20 T·m: −25 % barrel dose for ~2× mass, but only −5 % total because trapped low-energy particles are funnelled into the endcaps. "Real GCR reduction has been achieved however, the reduction is not significant enough to provide significantly greater mission duration." |
| NASA MAARSS Phase II, 2019 | same 6+1; trade space B × field-thickness with HZETRN | 8–100 T·m scanned | YBCO 5 cm × 0.1 mm tape, Je 87 kA/cm² at 40 K assumed (3× 77 K value) | **1 T × 10 m: SC 34.5 t, structure 14.9 t, thermal 4.6 t, power 0.2 t, radiator 5.9 t, arrays 0.9 t ≈ 61 t.** **4 T × 4.75 m (19 T·m): ~200 t** for a 150 mSv/yr point dose. 8 T·m, 6-coil system 50 t within a 106 t vehicle | "**does not offer dramatic improvements over passive shielding**"; needs better Je and lighter structure. 10 T × 2 m gives ~150 mSv/yr, 2 T × 10 m ~200 mSv/yr. Passive JWST-class sunshade can hold coils at 30 K with net *negative* external load (−41 to −76 W), so cryocoolers are only for the 380 W habitat leak. |
| SR2S (EU FP7 2013–15, CERN/INFN/CEA; recollection) | "pumpkin" toroid of MgB₂ racetracks around a ~10 m habitat | ~4–5 T·m class | MgB₂ at 10–20 K; CERN wound and tested MgB₂ coils | tens of tonnes coil + structure | ~40–50 % GCR reduction modelled; project ended with a technology roadmap, not a design that closed; cryogenics and mechanics flagged as the hard parts |
| Sailer 2019 (arXiv 1902.10122) | split toroid | — | — | 80.8 t of wire for 0.47 T | <0.22 mT in crew area |
| CREW HaT, D'Onghia et al. 2024 (NIAC 2022 Ph. I, NTRS 20250002403) | 8-coil Halbach torus, coils 8 m × 4 m ellipses, 10⁷ A·turns each, 10 T peak on the winding | sized for 50 % attenuation of 600 MeV protons | REBCO 4 mm tape at 40 K, 80 A/tape, 125,000 turns per coil (2,632 with CORC cable), ~1000 km of tape per coil | **~15 t per coil, ~120 t for eight**; heat load 190 W at 40 K is "a factor of four" above available cooling | Cuts 400–500 MeV protons by half and **total GCR exposure by 20 %**: 0.8 mSv/day solar min, 0.4 solar max; "an additional passive shielding layer provides equivalent mitigation to our active shielding system at a considerable mass cost" (≈100 t of Al/PE) |

**Honest summary for a 100–500 t magnet system** (MAARSS-II trade tables,
ARSSEM, CREW HaT): ~60 t of 1 T coils (8–10 T·m) cuts the barrel-region GCR
dose by 25–50 %; ~200 t at 4 T (~20 T·m) cuts it by ~65–75 %; ~500 t at 8 T
(~40 T·m) by ~85 %; beyond that the residual is >7 GV primaries and endcap
leakage and no mass helps. Against *SPE protons* (< 1 GV) a magnet is superb —
36 t of coil ≈ 141 t of polyethylene — but 20–50 g/cm² of water does the SPE
job too, so the magnet's real product is GCR, exactly where it is weakest.
Per tonne, a magnet is worth roughly **1–3 tonnes of water/polyethylene**
against GCR (CREW HaT: 120 t ≈ 100 t passive; MAARSS: 200 t magnet → 150
mSv/yr, versus ~450 t of water for the same dose on the same 6 × 10 m cylinder,
calc). Not ten. Not free.

**What no study puts in the abstract.**

- *Volume.* The protected volume is the bore. MAARSS protects a 6 m × 10 m
  cylinder (280 m³, 6–9 people at 30–50 m³/person); coil mass scales with the
  bore diameter and roughly with B²·V for structure, so a 16 m bore at 1.5 T
  (MAARSS-II "expandable" case) is ~4× the mass. Magnets are a technology for
  small crews.
- *Endcaps and secondaries.* Solenoids funnel sub-cutoff particles along the
  axis; MAARSS-I found the endcaps "dominated by the contribution of the low
  energy protons trapped by the solenoid shield" and needs 50 cm of
  polyethylene (~47 g/cm²) at each end; the 20 T·m variant lost most of its
  barrel gain there. ARSSEM's Monte Carlo shows secondary showers off the coil
  structure (3, 5 and 7 GV proton event displays) and warns that the material
  crossed by GCR must be kept minimal — the coil's own strongback is a thin
  aluminium-like shield with the neutron penalty 02-vessels already describes.
  Budget +10–20 % on the residual dose for coil-structure secondaries.
- *Stray field.* Toroids put tesla-level fields through the habitat and were
  rejected on those grounds (MAARSS-I §10). The 6+1 compensator coil nulls
  the habitat field; Sailer reports <0.2 mT. ICNIRP static-field limits are 2 T
  head/trunk occupational, 400 mT public, but the practical ceiling is ~0.5 mT
  for pacemakers, ~1 mT for vacuum-tube electronics and CRTs (a belt at the
  tube tier per 03-industry cares), and any ferromagnetic tool near a 1–10 T
  winding is a projectile. External fringe fields also act on approaching
  ferromagnetic craft (MAARSS-II analysed a capsule closing at 0.1 m/s).
- *Stored energy and quench.* 1 T over six 8 m × 20 m solenoids stores 2.4 GJ;
  at 4 T, 38 GJ (~9 t TNT) (calc). CREW HaT quotes 98 H inductance per coil.
  A quench must dump that into a resistor and radiator or the winding melts;
  CREW HaT notes a single crack anywhere in ~1000 km of tape kills the coil.
  Quench detection and dump switches are electronics; 1960s LTS magnets did it
  with voltage taps, relays and a dump resistor, so tube-tier is possible.
- *Cryogenics.* Heat loads: 300 K habitat → 30 K coil through MLI is ~500 W
  (MAARSS-II Table 14.2; e* = 0.002), ~0.3 W through a JWST-quality five-layer
  shield (e* = 10⁻⁶), plus ~380 W conducted from the habitat, plus joints and
  leads (CREW HaT: 190 W at 40 K). Removing ~500 W at 20–30 K needs 56 kW of
  cryocooler input and 3–5 t of coolers (MAARSS-II Tables 14.3–14.4:
  Stirling GPC-2 150 W at 20 K for 22 kW, 600 kg; Cryomech AL325 100 W at 25 K
  for 11 kW, 470 kg). MAARSS-II therefore recommends **passive cooling by
  sunshade** at 1 AU and cryocoolers only for the habitat leak. At 2.7 AU solar
  input is 7.3× lower and the sky is 3 K; a 20–30 K coil behind a multi-layer
  aluminium shade with a separate habitat-side shade is a passive design, at
  the price of very slow cool-down (1,275 MJ per 7.5 t coil from 300 K;
  weeks to months radiatively) and quench on any pointing or plume upset.
  Every cryocooler in the table is a helium machine; 03-industry lists helium
  as truly absent in the belt.

### 1.2 Magnet plus water: what a sailor's dose becomes

Model (calc): free-space barrel dose 650 mSv/yr at solar minimum (MAARSS 1977
spectrum; CREW HaT/OLTARIS give 428, RAD measured 670 inside a spacecraft —
use a 450–650 band), solar-cycle average ×0.75, belt radial gradient +5 %
ignored; water factor from 02-vessels' synthesis (20/50/100/300 g/cm² →
0.65/0.48/0.32/0.10); magnet factor from MAARSS-I Table 8.2 mid-band (8/20/40
T·m → 0.62/0.27/0.13 of the 10 g/cm² case); combined sub-multiplicatively
(magnet factor^0.85) because water and field both remove the low-rigidity end;
endcaps at ~50 g/cm² PE add 20–40 mSv/yr. A 6 m × 10 m cylinder has 245 m² of
hull, so water at 50/100/300 g/cm² is 123/245/735 t.

| Shield | mSv/yr solar min / cycle average | Years to 600 mSv | 40-year adult dose | Excess cancer mortality (~5 %/Sv) |
|---|---|---|---|---|
| 20 g/cm² water, no magnet (02's skiff) | 460 / 350 | 1.7 | 14 Sv | not a life (plus deterministic effects) |
| 50 g/cm² water | 350 / 265 | 2.3 | 10.6 Sv | ~50 % |
| 100 g/cm² water | 250 / 185 | 3.2 | 7.4 Sv | ~35 % |
| 300 g/cm² water (735 t) | 105 / 80 | 7.6 | 3.1 Sv | ~15 % |
| 8 T·m (60 t) + 50 g/cm² | 230 / 170 | 3.5 | 6.8 Sv | ~35 % |
| 20 T·m (~200 t) + 50 g/cm² | 125 / 92 | 6.5 | 3.7 Sv | ~18 % |
| 20 T·m + 100 g/cm² | 90 / 66 | 9 | 2.7 Sv | ~13 % |
| 40 T·m (~500 t) + 100 g/cm² | 57 / 43 | 14 | 1.7 Sv | ~9 % |
| burrow, 1000 g/cm² regolith-equivalent | ~13 / 10 | 60 | 0.4 Sv | ~2 % (Earth: 0.1–0.3) |

Same mass, no magnet: 450 t of water on that cylinder (180 g/cm²) gives ~130
mSv/yr average. So a sponsor-built 20 T·m coil buys a mobile crew roughly a
factor 1.5–2 in dose per tonne carried, and lets a 300–350 t shield package
protect a 6-person bore at ~70–90 mSv/yr instead of ~150.

**What that dose means for a life.** 70–120 mSv/yr average is 4–6× the ICRP
occupational limit (20 mSv/yr) and 70–120× the public limit; a 40-year adult
career accumulates 3–5 Sv. Consequences, using ICRP 103 / BEIR VII figures:
cancer mortality +15–25 % absolute (adult risk coefficient ~5 %/Sv; NASA's REID
model gives similar numbers with a 95 %-CL multiplier of ~2–3); lens opacity
threshold 0.5 Gy absorbed — at Q ≈ 3 that is ~35 mGy/yr, so cataracts by year
15–20 (surgery is a `medical_level` 2 procedure); cardiovascular excess at
>0.5 Gy is now accepted by ICRP; heavy-ion CNS effects (cognition, mood) are
the unknown NASA still lists as a red risk. Fertility: temporary male sterility
needs 0.15 Gy acute or 0.4 Gy/yr chronic, permanent 3.5–6 Gy acute or 2 Gy/yr;
ovarian sterility 2.5–6 Gy acute or >0.2 Gy/yr chronic. At 35 mGy/yr no sailor
is sterilised. Heritable effects: ICRP 0.2 %/Sv → ~1 % excess genetic disease in
the children of a 4 Sv parent. Pregnancy aboard: nine months at 90 mSv/yr
equivalent is ~25 mGy absorbed to the fetus — below the 100 mGy malformation
threshold but a doubling of childhood leukaemia risk (Oxford Survey: ~40 %
excess per 10 mGy; baseline ~0.2 %), and the heavy-ion component has no
epidemiology at all. Children raised aboard: per-unit-dose cancer risk 2–3× the
adult value (BEIR VII 10–15 %/Sv for exposure before age 10), so 18 years at
90 mSv/yr is 1.6 Sv and ~20 % excess lifetime mortality, plus unknown
developmental effects.

**The verdict on the hybrid.** Without a magnet, 02's split is exactly right:
sailors are adults with 2–3-year careers or a 50 % cancer lottery. With a
sponsor-built 20 T·m coil set and 50–100 g/cm² of water, an adult can spend a
whole life aboard at the cost of roughly a decade of life expectancy — the
historical price of mining, whaling or smoking, which cultures have accepted
before. Conceiving aboard is fine; gestating aboard doubles a child's leukaemia
risk; raising children aboard is a 20 % lifetime penalty on them. Reproduction
in burrows is therefore still what the physics *favours*, but it is no longer
what the physics *forces* — it becomes a policy the colony sets, and a
Voidborn sailor culture that keeps its children aboard is a coherent, costly
choice rather than a rules violation. The catch, developed in 1.4: the coil that
makes this possible is a sponsor artefact with a finite life.

### 1.3 Magnetic sails, plasma magnets and electric sails at 2.2–3.3 AU

**The wind.** Solar wind: 250–750 km/s (typically 400–500), 3–10 protons/cm³
at 1 AU, dynamic pressure 1–6 nPa (Andrews & Zubrin used 0.5/1.0/2.0 nPa for
quiet/average/high). Density falls as 1/r², speed is flat beyond 1 AU, so
p_dyn ∝ 1/r²: 0.42 nPa at 2.2 AU, 0.28 at 2.7, 0.19 at 3.3 (calc, 2 nPa base).
The wind is radial at 400+ km/s against an orbital speed of 18 km/s, so the
apparent wind is within 2.5° of radial everywhere in the belt.

**Pure magsail (Andrews & Zubrin 1988–91).** A bare superconducting loop's
magnetosphere stands off where B²/2μ₀ = p_dyn; for a dipole B falls as r⁻³, so
standoff L ∝ m^{1/3}·p_dyn^{−1/6} and thrust F = C_d·½p·πL² ∝ p_dyn^{2/3} ∝
**r^{−4/3}** — gentler than a solar sail's r⁻². Andrews & Zubrin's kinetic
model: drag 264.5 N and lift 74.1 N (L/D 0.28) at quiet-sun 1 AU for their
reference loop; all their minimum-mass loops carry 57.7 kA with a 720 N hoop
force regardless of size; the 1990 NTRS paper quotes "an average thrust of
250 Newtons at a radius of one A.U." Freeland (2015) re-integrated and found
the thrust "optimistic by a factor of 3.1 due to a numerical integration
error". Nishida's MHD runs give C_d 3.6 at zero tilt rising to 5 at 90°, with
much less lift than the kinetic estimate. My first-principles model (calc,
C_d 3.6, 450 km/s, 6/cm³):

| Loop | Tape mass at Je 2×10⁹ A/m² (+ sunshade) | Stored energy | Standoff, thrust at 1 AU | at 2.2 AU | at 2.7 AU | at 3.3 AU |
|---|---|---|---|---|---|---|
| R 10 km, 50 kA | 13 t (+2) | 0.24 GJ | 35 km, 14 N | 46 km, 5.0 N | 49 km, 3.8 N | 53 km, 2.9 N |
| R 32 km, 50 kA (Zubrin-class) | 40 t (+6) | 0.8 GJ | 77 km, 67 N | 100 km, 24 N | 107 km, 18 N | 114 km, 14 N |
| R 50 km, 58 kA | 73 t (+9) | 1.8 GJ | 108 km, 135 N | 141 km, 47 N | 151 km, 36 N | 161 km, 28 N |

Apply Freeland's ÷3.1 as the pessimistic edge: the Zubrin-class loop gives
**6–18 N at 2.7 AU for ~50 t of hardware**. Two belt-specific penalties: the
proton gyroradius at the magnetopause grows as 1/B_mp ∝ r, from 66 km at 1 AU
to 178 km at 2.7 AU, so the 100 km bubble is smaller than the ion gyroradius
and the interaction turns kinetic (Fujita's particle simulations show thrust
falling when L/r_gi < 1 — take another ×0.5–1); and the wind's factor-of-2–4
variability is now your engine's variability. Zubrin assumed Je 2×10¹⁰ A/m²;
today's REBCO at 20–30 K self-field gives 1–3×10⁹, so the 200 km of loop is
~1,200 km of 4 mm tape (~40 t) — CREW HaT-scale quantities of sponsor tape.
Hoop tension is trivial (a few kN, calc); the mass is conductor and
thermal protection (Zubrin's 0.029 kg/m TPS), and cooling is the same
sunshade problem as 1.1 spread over 200 km of cable.

On a 1,000 t vessel that is 0.006–0.02 mm/s²: **0.2–0.6 km/s per year**, 1 km/s
in 1.7–5 years; on 10,000 t, 0.02–0.06 km/s per year. Compare 02-vessels'
1 MWe NEP: 61 N and 269 kg/day of propellant. The magsail is a third to a
tenth of a megawatt reactor's thrust for no propellant, no reactor fuel and no
radiators. That is meaningful — for the right jobs.

**Which jobs.** Thrust is radial-outward drag plus (kinetic estimate) up to 0.28
of it tangential, either sense, by tilting the dipole. Radial thrust alone
acts like reducing the Sun's gravity by β = a/g_sun; at 2.7 AU g_sun is 0.81
mm/s², so 0.02 mm/s² is β = 0.025: switched on, the orbit becomes an ellipse
with e ≈ β and aphelion 5 % further out; switched off at aphelion, you have
raised your orbit for free. Over one 4.4-year orbit the sail delivers ~2.8
km/s of accumulated a·t (calc). Against 01-map's phasing law (ΔV ≈
v_orb·Δλ/540°/N, i.e. 6 km/s to close 180° in one orbit, 3 km/s in two), a
Zubrin-class magsail makes **same-orbit phasing free in two orbits** and a
2.5→2.8 AU radial hop (1.1 km/s) free in a year or two. It cannot change
inclination at all (no out-of-plane force from a radial wind), so the 0.31
km/s-per-degree plane-change cost that 02 identifies as the real belt Δv
budget is untouched; and sunward travel uses only the lift component, so
going in is 3–4× slower than going out. A magsail is a phasing and
station-keeping engine for the patient, not a transport engine.

**Plasma magnet / M2P2.** Winglee's M2P2 (2000) claimed that injecting plasma
into a small coil's field inflates a 10–30 km bubble from a metre-scale coil
at ~1 kW, because the field falls as r⁻¹ rather than r⁻³; with f_o = 1 the
bubble grows as 1/√p_dyn and **thrust is independent of solar distance**.
Khazanov, Cattell and Toivanen's independent models (2003–05) found the
falloff cannot be better than r⁻² and the thrust "dramatically overestimated"
(tens to hundreds of times); Slough's plasma magnet (RMF-driven current disc,
NIAC 2004–06) was restated at f_o = 2 by Kirtley & Slough in 2011–12, and
JAXA's MPS laboratory programme (Funaki, Yamakawa, Ueno) measured f_o between
1.5 and 2 with thrust gains of order a few, best "when the injected plasma had
a lower density and velocity". With f_o = 2, thrust scales as 1/r (bubble
L ∝ p^{−1/4}), like an electric sail. The sensitivity is brutal (calc, MAARSS
1 T, 4 m coil as the seed): f_o = 2 gives a 25 km bubble and **0.95 N** at 2.7
AU; f_o = 1.5 gives 450 km and 320 N. Freeze/Greason's "Wind Rider" claims
(2017–22) sit at the optimistic end and have not been tested in space. For the
sim: a plasma magnet is a 5–10 m coil plus tens of kW of RMF power and a
plasma feed, giving 1–10 N at 2.7 AU with a hidden ×0.1–×30 uncertainty
parameter — perfect material for a per-seed unknown, not a known constant.

**Electric sail (Janhunen).** 50–100 tethers of 25–50 µm aluminium wire, 20 km
each, held at 20 kV by an electron gun; baseline 540 W for 0.5 N at 1 AU (up
to ~1 N in later papers, ~500 nN/m of tether), thrust ∝ r⁻¹ to r^{−7/6} because
the Debye sheath widens as the wind thins; thrust vector up to 30° off radial;
spin-stabilised. At 2.7 AU a 2,000 km e-sail gives **0.3–0.4 N** (calc): 1 km/s
in ~1 year on a 10 t pod, 10 years on 100 t. Tether count is limited by
dynamics and deployment (ESTCube-1 and Aalto-1 both failed to unreel), so
e-sails do not scale to crewed tonnage. Their virtue is elemental: aluminium
wire, a tube-tier electron gun and a 20 kV supply — a T2–T3 belt product.

**Solar sail.** At 2.7 AU radiation pressure is 1.24 µPa: 1 km² of 5 µm
aluminium foil (13.5 t, no polymer needed) yields 1.2 N. On a 1,000 t vessel
that is 27 years per km/s; on a 30 t pod, ~300 days. Belt-native (03's Al
film at T2), useless for people, as 02 already says.

**Can the shield coil be the sail?** No. Shield coils are compact and strong
(1–10 T over 4–8 m: dipole moment ~10⁹–10¹⁰ A·m²); sails are vast and weak
(Zubrin: 1.6×10¹⁴ A·m² at 10⁻⁶ T). The MAARSS 6+1 array as a magsail has a 2
km bubble and 0.07 N at 1 AU (calc). Conversely a magsail's 70 nT bubble is
0.01 T·m of bending power — it deflects keV solar wind and nothing that
matters for dose. The plasma magnet is the only overlap: it can inflate a
shield coil's field into a ~25 km bubble (f_o = 2) for ~1 N at 2.7 AU — a
station-keeping trickle, at tens of kW.

### 1.4 Superconductors against belt abundances

CI-chondrite abundances (Lodders 2003 via the Wikipedia data page, converted
to ppm by mass; calc): Mg 9.9 %, Al 0.87 %, Ca 0.93 %, Ni 1.1 %, Cr 2,670 ppm,
K 560, Ti 440, Cu 126, V 57, Ge 33, Se 19, Ga 10, Sr 7.8, Pb 2.5, Ba 2.35, Te
2.3, As 1.9, Sn 1.7, Y 1.6, Mo 0.93, B 0.87, Nb 0.25, La 0.24, Ag 0.20, Gd
0.20, Tl 0.14, Bi 0.12, Ta 0.014. Siderophiles (Ni, Co, Cu, Ga, Ge, As, Mo, W,
Ag, PGM) concentrate into metal and the carbonyl residue (irons carry As
4–20 ppm, Ga 1–100, Ge 0.01–500, Mo 6–8, W 0.5–3 ppm); chalcophiles (Se, Te)
into troilite; lithophiles (B, Y, Ba, Sr, Nb, Ta, Ti, REE) stay dispersed in
silicates with no ore-forming process — except that Y and the REE ride in the
phosphate (merrillite/apatite, ~0.3–0.5 wt% of chondrite), which a colony
already leaches for fertiliser phosphorus.

| Material | Composition, T_c, usable T_op | Elemental problem (belt) | Conductor form and making | Belt verdict |
|---|---|---|---|---|
| NbTi | Nb-47wt%Ti, 9.2 K, 4.2 K | Nb 0.25 ppm, no concentrator; needs liquid helium | multifilament in Cu, drawn | **never**; helium-cooled anyway |
| Nb₃Sn | 18 K, 4.2 K | Nb, Sn 1.7 ppm, helium | bronze/internal-tin, 650 °C react | **never** |
| MgB₂ | 39 K; usable 10–20 K (Hc2 ~3 T ⟂ at 20 K, 15 T at 4 K; Jc 10⁹ A/m² at 4.2 K, 5 T) | Mg abundant; **B 0.87 ppm** (03's rare list); wire is ~50 wt% B in the MgB₂ fraction; a 30 t wire set (15–30 % fill) needs ~2–3 t of boron ≈ 3 Mt of rock at 100 % recovery | powder-in-tube in Fe/Ni/Monel sheath, drawn, reacted 650–950 °C; 18 km of wire made an MRI magnet in 2006; CERN's 20 kA cables (SR2S/HL-LHC links) | **makeable at T3–T4 if boron is imported** (2–3 t per coil set: a vitamin by mass, a chokepoint by chemistry); T_op ≤ 20 K needs passive sunshade cooling or helium coolers |
| REBCO tape (Y/Gd-Ba₂Cu₃O₇) | 92 K; usable 20–65 K, Hc2 >100 T; 700–2,000 A/mm² in the layer | Per 100 t of tape (calc from a 4 mm × 0.1 mm architecture): ~53 t Hastelloy (Ni-Cr-Mo-W — replaceable by local Ni-Cr or Ni-W as in RABiTS), ~43 t Cu stabiliser (126 ppm; substitutable by Al, which CREW HaT proposed), ~4 t **Ag** (0.2 ppm, not in metal; unavoidable cap layer), ~0.7 t REBCO containing ~0.1 t Y (from phosphate leach; mixed Y/Gd works), ~0.3 t **Ba** (2.35 ppm dispersed; no substitute) | 1 µm epitaxial layer on IBAD-MgO or textured-metal buffers by MOCVD/PLD/MOD over hundreds of km, oxygen anneal; production of 300 km in 9 months (SuperOx 2021) is a fab | **never in the game** (T5: semiconductor-class thin-film precision plus Ag and Ba) |
| Bi-2212/2223 (BSCCO) | 85–110 K; usable 20–77 K | Bi 0.12 ppm, Sr 7.8, Ag sheath ~70 wt% | Ag-sheathed PIT | **never** |
| Iron-based: FeSe / Fe(Se,Te) | 8 K / 14 K | Fe, Se 19 ppm (troilite), Te 2.3 ppm — all local | PIT and coated-conductor work in Japan/China | chemically native, but T_op < 8 K = helium: **no** |
| Iron-based 122 / 1144: (Ba,K)Fe₂As₂ 38 K; **CaKFe₄As₄ 35 K** (Hc2 >70 T) | usable 15–25 K | 1144 is Ca 7 %, K 7 %, Fe 40 %, **As 54 %** by mass; As 1.9 ppm bulk but 4–20 ppm in metal and enriched in carbonyl residue; 5 t of 1144 needs ~2.7 t As ≈ 200–500 kt of metal processed; arsenic chemistry is a hazard, not a barrier | Ag- or Cu-sheathed PIT tapes; 100 m lengths with Jc ~1.5×10⁵ A/cm² at 4.2 K, 10 T (IEE-CAS 2018–20); at 20 K expect 5–10× below REBCO | **the only fully belt-native superconductor chemistry**; T4 (fine chemistry + PIT wire mill + 20 K cooling), crudeness ×5–10 on coil mass |
| Nb, Pb, V, Ta elemental | 9.3, 7.2, 5.4, 4.5 K | Pb 2.5 ppm; all need helium | — | no |

**Cooling is a second chokepoint.** Every 4–40 K cryocooler flown is a helium
machine (Stirling, pulse tube, GM, sorption), rated for >10,000 h to ~10 years
(AIRS coolers have run since 2002). Hydrogen is a workable Stirling/pulse-tube
gas above its 20 K boiling point, so a belt-built H₂-cycle cooler can plausibly
hold 30–40 K (REBCO, 1144) but not the ≤20 K that MgB₂ needs at useful fields.
The way out is what MAARSS-II recommended for a different reason: **passive
cooling behind a JWST-class shade**. Aluminium foil layers are T2; polyimide
film is T3 organic chemistry; the coil then has no moving parts and no helium,
but must be shaded from the habitat as well as the Sun, cannot be warmed and
recooled quickly, and quenches on any thermal upset. At 2.7 AU (177 W/m²) this
is easier than at 1 AU by 7×.

**Lifetime of an imported coil.** Radiation damage is a non-issue: REBCO and
MgB₂ degrade at fast-neutron fluences of ~10²² m⁻²; GCR is ~10¹³ m⁻² per year.
What kills coils: (i) mechanical — a tape crack in ~1000 km of conductor
(CREW HaT), delamination on thermal cycling, creep of the strongback under
magnetic pressure; (ii) joints — REBCO has no true persistent-mode joint, so
the current decays through nΩ joints and must be topped up by a flux pump
(MAARSS-II baselines a YBCO flux pump, ~2 kW), which is power electronics;
(iii) cooling — cooler death or shade damage (micrometeoroids through five
layers) leads to a quench; (iv) the quench itself — 2–40 GJ dumped. A
reasonable sim prior: 10–20 year characteristic life per coil set with a
rising hazard, spares being tape (sponsor), helium (sponsor), and flux-pump /
quench electronics (chip stock, per 03). A dead coil is 35–100 t of
Ni-alloy, copper and silver scrap — worth stripping.

**Is the coil a vitamin part?** Yes, of the worst kind: it is the single most
massive vitamin in the fleet (35–100 t of tape per shield set; 40 t per
Zubrin-class magsail), it cannot be rewound locally, and its consumables
(helium, chips) are themselves vitamins. The only belt path is MgB₂ wire from
imported boron with passive cooling — which converts a 60 t import into a 3 t
import plus a T3–T4 wire mill — or, generations later, arsenide wire. Those
are act-3 goals, not act-2 rescues.

### 1.5 Anything else that makes small-vessel mobility cheaper

- **Water is both shield and propellant.** A sailor ship carrying 120 t of
  water at 50 g/cm² can steam any of it away at 190–320 s; 02's mass-driver
  rock-eating is the same duality at habitat scale. Every kilometre per second
  bought this way thins the shield — the dose ledger should charge for it.
- **Spin-tether slings.** A spinning habitat is a momentum bank. At 2 rpm a
  10 km tether from the rim has a tip speed of 2.1 km/s (calc); Zylon/UHMWPE
  tapered tethers reach 2–3 km/s characteristic velocity, so a burrow-adjacent
  spun station can throw 10 t pods at km/s and catch incoming ones to restore
  spin (a 10⁶ t ring at 224 m loses ~2 % of its angular momentum per 10 t pod
  at 2 km/s). This is a belt-native, propellantless launcher for cargo and
  suicidally brave sailors; it needs tether fibre (organic chemistry, T3) and
  metre-accurate rendezvous.
- **Gravity assists** are nearly worthless: Ceres at v∞ 2 km/s and 500 km
  periapsis bends the trajectory 3.5° for 0.12 km/s (calc); Vesta less.
- **Aerobraking**: no atmospheres in the belt; the only aerocapture is at Mars
  or Earth, which 02 already books for cargo arrival.
- **Electric sails and foil sails** for pods (above): cheap, slow, native.
- **Solar-wind–powered generators** (Greason's "repetitively stroked plasma
  magnet") are speculative; ignore.

## 2. Numbers the sim needs

| Parameter | Value / range | Unit | Source |
|---|---|---|---|
| Proton bending radius | R_GV / (0.3·B_T) | m | physics |
| 8 T·m cutoff: deflected / marginal / undeflected protons | 3 / 5 / 7 GV (4.7 GeV KE) | — | MAARSS-I §6.6 |
| GCR barrel dose factor vs bending power (relative to same passive shield) | 8 T·m 0.5–0.75; 20 T·m 0.19–0.37; 40 T·m 0.10–0.18; 100 T·m ~0.065 | — | MAARSS-I Tables 8.1–8.2 |
| Field strength vs thickness at equal T·m | higher B wins: 8T×1m 0.49, 4T×2m 0.55, 2T×4m 0.64, 1T×8m 0.75 | — | MAARSS-I Table 8.2 |
| Endcap dose at 25 g/cm² Al (unshielded by field) | 55 | mSv/yr | MAARSS-I Table 8.3 |
| Endcap passive requirement | ~47 (50 cm PE) | g/cm² | MAARSS-I §6.8 |
| Coil-structure secondaries penalty | +10–20 % of residual | — | ARSSEM/MAARSS, my estimate |
| Free-space GCR, solar min / cycle avg | 450–650 / ×0.75 | mSv/yr | MAARSS 1977, OLTARIS, RAD |
| Solar-max/min dose ratio | 0.4–0.6 | — | ARSSEM, CREW HaT |
| Magnet system mass, 1 T × 10 m (6+1, 8 m bore, 20 m) | 61 (SC 34.5, structure 15, thermal 4.6, radiator 5.9) | t | MAARSS-II Table 15.1 |
| Magnet system mass, 4 T × 4.75 m (~19 T·m) | ~200 | t | MAARSS-II §15.2 |
| Magnet system mass, 8 coils × 10⁷ A, 10 T peak (CREW HaT) | ~120 (15 per coil) | t | CREW HaT §4.1 |
| ARSSEM DH toroid 4–5 T·m | 37 magnet + 2 H₂ + 3.2 thermal + 2 power | t | ARSSEM §on mass |
| Magnet ≈ passive equivalence, GCR | 1 t magnet ≈ 1–3 t water/PE | — | CREW HaT, MAARSS-II (calc) |
| Magnet ≈ passive equivalence, SPE | 36 t coil ≈ 141 t PE (75 cm) | — | MAARSS-I §10 |
| Protected bore per MAARSS set | 6 m × 10 m, 280 m³, 6–9 people | — | MAARSS |
| Coil mass scaling | ∝ bore diameter × length (SC) + ∝ B²·V (structure) | — | MAARSS-II trade |
| Stored energy, six 8 m × 20 m solenoids | 2.4 (1 T) / 38 (4 T) | GJ | calc |
| CREW HaT coil inductance | 98 (tape) / 0.74 (CORC) | H | CREW HaT |
| Habitat stray field target / pacemaker / tube electronics | <0.2 / 0.5 / ~1 | mT | Sailer; ICNIRP; practice |
| ICNIRP static limits: occupational head–trunk / limbs / public | 2 / 8 / 0.4 | T | ICNIRP 2009 |
| Heat load 300 K→30 K: MLI (e* 0.002) / JWST shield (10⁻⁶) / habitat conduction | ~500 / 0.3 / 380 | W | MAARSS-II Tables 14.2 |
| Cryocooler at 20–25 K: cooling / input / mass | 100–150 W / 11–22 kW / 470–600 kg | — | MAARSS-II Table 14.3 |
| Cryocooler system for 500 W at 20–30 K | 56 kW, 3–5 t | — | MAARSS-II Table 14.4 |
| Cool-down enthalpy per 7.5 t coil | 1,275 | MJ | MAARSS-I §3.2 |
| Coil cold mass / conductor per 8 m × 20 m coil | 7,500 / 503 | kg | MAARSS-I |
| Tape per CREW HaT coil | ~1,000 | km | CREW HaT |
| REBCO Je assumed at 40 K / measured at 40 K, 10 T | 870 A/mm² / 80 A per 4 mm tape | — | MAARSS-II; CREW HaT |
| Coil-set lifetime prior | 10–20 yr characteristic, rising hazard | yr | my synthesis |
| Radiation damage threshold vs GCR fluence | 10²² vs 10¹³ per yr | m⁻² | literature (recollection) |
| Solar wind p_dyn at 1 / 2.2 / 2.7 / 3.3 AU | 2.0 / 0.42 / 0.28 / 0.19 (×0.5–3 variability) | nPa | calc from 6/cm³, 450 km/s |
| Magnetopause field / proton gyroradius at 1 / 2.7 AU | 71 nT, 66 km / 26 nT, 178 km | — | calc |
| Magsail thrust scaling | ∝ r^{−4/3}; e-sail and plasma magnet (f_o=2) ∝ r⁻¹; solar sail ∝ r⁻² | — | physics |
| Zubrin-class loop (R 32 km, 50 kA): tape mass / thrust at 2.7 AU | 40 t (Je 2×10⁹) / 6–18 N | — | calc; Freeland ÷3.1 |
| Andrews–Zubrin reference: drag / lift / L/D, quiet Sun 1 AU | 264.5 / 74.1 N / 0.28 | — | NTRS 19910012840 |
| Optimum magsail current / hoop force | 57.7 kA / 720 N | — | Andrews–Zubrin |
| Magsail C_d vs tilt | 3.6 (0°) → 5 (90°) | — | Nishida 2005 |
| Kinetic penalty when L < r_gi | ×0.5–1 | — | Fujita (recollection) |
| Magsail accel on 1,000 / 10,000 t at 2.7 AU | 0.006–0.02 / 0.0006–0.002 | mm/s² | calc |
| Δv per year, 1,000 t vessel, magsail at 2.7 AU | 0.2–0.6 | km/s | calc |
| β = a/g_sun at 2.7 AU for 0.02 mm/s² | 0.025 (e ≈ 0.025 induced) | — | calc |
| Plasma magnet from 1 T, 4 m coil at 2.7 AU | 0.95 N (f_o 2) … 320 N (f_o 1.5); hidden ×0.1–×30 | N | calc |
| E-sail: 2,000 km of tether, 20 kV, ~540 W | 0.5–1 N at 1 AU; 0.3–0.4 N at 2.7 AU; ≤30° off radial | — | Janhunen; calc |
| Solar sail: 1 km² 5 µm Al foil | 13.5 t, 1.24 N at 2.7 AU | — | calc |
| Spin-tether tip speed, 2 rpm, 10 km | 2.1 | km/s | calc |
| Ceres flyby Δv (v∞ 2 km/s, r_p 500 km) | 0.12 | km/s | calc |
| CI abundances (ppm by mass) | B 0.87, Nb 0.25, Y 1.6, Ba 2.35, Ag 0.20, Bi 0.12, Se 19, Te 2.3, As 1.9, Cu 126, Mo 0.93, Ti 440, V 57 | ppm | Lodders 2003 (calc) |
| REBCO tape mass split | Hastelloy 53 %, Cu 43 %, Ag 3.7 %, REBCO 0.7 % | — | calc |
| Boron per MgB₂ coil set (30 t wire) | 2–3 | t | calc |
| Arsenic per 5 t of CaKFe₄As₄ | 2.7 | t | calc |
| Iron-based wire Jc (4.2 K, 10 T) | 1.5×10⁵ | A/cm² | IEE-CAS 2018–20 (recollection) |
| Career limit (sponsor) / occupational / public | 600 total / 20 per yr / 1 per yr | mSv | NASA 2022; ICRP |
| Cancer mortality coefficient adult / child | ~5 / 10–15 | %/Sv | ICRP 103; BEIR VII |
| Lens opacity threshold | 0.5 | Gy | ICRP 2011 |
| Fetal malformation threshold / childhood cancer excess | 100 mGy / ~40 % per 10 mGy | — | ICRP; Oxford Survey |
| Sterility thresholds (testes temp/perm; ovaries perm) | 0.15 Gy, 3.5–6 Gy (chronic 0.4, 2 Gy/yr); 2.5–6 Gy (chronic >0.2 Gy/yr) | — | ICRP 103 |
| Heritable risk | 0.2 | %/Sv | ICRP 103 |
| Sailor dose with 20 T·m + 50–100 g/cm² water | 66–92 cycle-avg (90–125 solar min) | mSv/yr | calc |
| Sailor dose without magnet, 20–50 g/cm² | 265–350 | mSv/yr | calc |

## 3. Implications for mechanics

1. **The coil is a vessel component with a life, not a shield stat (acts 1–3).**
   Add `coil_set` to `Vessel`: `bending_Tm` (8/20/40), `bore_m3` (280 at MAARSS
   scale; mass scales with bore), `mass_t` (60/200/500), `t_op_K`, `cooling`
   (helium-cryocooler | H₂-cryocooler | passive-shade), `stored_GJ`, `age_yr`,
   `hazard` (Weibull, characteristic 10–20 yr), `flux_pump_ok`, `quench_electronics_ok`.
   Dose = base × water_factor × magnet_factor^0.85 + endcap term. Evidence:
   MAARSS-II Table 15.1 and §15.2; CREW HaT §4.1; the quench/joint/crack
   failure modes. Effect on act 1: coil sets are the sponsor's most expensive
   gift and arrive with named ships; on act 2: "the magnets are dying" is the
   slow clock on sailor life; on act 3: whoever can ship tape or boron sets
   terms for the whole mobile population.

2. **Dose policy is a colony decision with a ledger, and children aboard are a
   choice (act 2).** Replace the binary `fertility_allowed` with three
   thresholds the polity sets: conception (essentially always allowed —
   sterility needs >0.2 Gy/yr), gestation aboard (doubles childhood cancer at
   ~25 mGy fetal; sponsor forbids), child-rearing aboard (≈20 % excess lifetime
   mortality at 90 mSv/yr; sponsor forbids). Track per person cumulative Sv,
   cataract onset (0.5 Gy absorbed), and a REID draw at death. A coil ship at
   66–92 mSv/yr makes a lifelong sailor culture *possible* at ~10 years of life
   expectancy; a skiff at 265–350 does not. This is where the Voidborn split in
   NOTES becomes a slider the player pushes rather than a wall. Evidence: 1.2
   table; ICRP 103 thresholds; BEIR VII age coefficients.

3. **Magnet-or-water is a real trade only for mobile vessels (act 1–2).** One
   tonne of coil ≈ 1–3 t of water against GCR and ≈ 4 t against SPE, but the
   water is belt-made, doubles as propellant, scales to any volume and never
   quenches, while the coil protects a 280 m³ bore for 6–9 people and dies. For
   burrows the magnet is pointless (regolith is free); for a 500–2,000 t sailor
   ship it doubles shielding per tonne and therefore Δv. Give the sponsor a
   catalogue line "shield coil set, 60/200 t" that competes with "200 t of
   tankage" in the act-1 manifest. Evidence: CREW HaT ≈100 t passive; MAARSS
   141 t PE; calc.

4. **Sail time as a currency beside Δv (acts 2–3).** A Zubrin-class magsail (40
   t of tape, 6–18 N at 2.7 AU) delivers 0.2–0.6 km/s per year to a 1,000 t
   vessel, radial-out plus ≤0.28 tangential, nothing out of plane. Model it as
   a continuous acceleration with a per-turn Δv accrual that can pay 01-map's
   phasing law (free same-orbit phasing in ~2 orbits; free 0.3 AU radial hops in
   1–2 years) and coplanar Hohmann legs, but cannot pay inclination. The
   design consequence is a two-currency travel system: propellant buys hurry
   and plane changes, sail time buys patience. Sailor vessels become the
   nomads of low-inclination families; high-inclination rocks (Pallas) stay
   unreachable. Evidence: 1.3 tables; Andrews–Zubrin L/D; Nishida.

5. **The magsail is another sponsor tape artefact — the same clock (act 2).**
   A magsail is 1,200 km of REBCO with a 200 km sunshade, quenchable by a
   plume, a shadow or a micrometeoroid, holding 0.2–1.8 GJ. It ages like a
   shield coil and cannot be rewound. Fleet with sails: 3–5 sails per hundred
   vessels at the start of act 2, decaying. Late-game the belt's own
   propellantless options are aluminium: foil sails (1 km² = 13.5 t = 1.2 N)
   and e-sails (0.3 N per 2,000 km of wire) for pods, and spin-tether slings
   (2 km/s tips at 2 rpm × 10 km) at spun stations — none of which move a
   crewed hull. Evidence: 1.3, 1.5.

6. **Plasma magnet as a per-seed unknown (act 3 research).** Its thrust at 2.7
   AU spans 1–300 N depending on a field-falloff exponent nobody has measured
   in flight. Implement as a hidden `f_o` drawn from 1.5–2.0 per seed with the
   prior weighted toward 2.0 (the independent models), discoverable only by
   building one (a 5–10 m coil, tens of kW, a plasma feed) — a late research
   gamble whose payoff is either a station-keeping trickle or a belt-changing
   engine. Evidence: Khazanov/Cattell/Toivanen critiques; JAXA f_o 1.5–2.

7. **Boron as an act-1 seed for act-3 coils (acts 1, 3).** MgB₂ wire is T3–T4
   metallurgy (powder-in-tube in nickel or iron, drawn, reacted) and its
   cooling can be passive at 2.7 AU behind aluminium-foil shades — but each
   coil set needs 2–3 t of boron, which the belt holds at 0.87 ppm. Make
   boron (as B₄C or borax) a sponsor-catalogue vitamin in act 1 that nobody
   needs yet; a colony that stockpiled it can, at T4 with ~10⁴ people, rewind
   its dying magnets. Without it the only native chemistry is CaKFe₄As₄ from
   carbonyl-residue arsenic at 5–10× the mass — a confederation-scale project.
   Evidence: 1.4 table; 03-industry tiers and boron flag.

8. **Helium is the coil's second leash (act 2).** Every flown cryocooler at
   20–40 K runs on helium; hydrogen-cycle coolers can plausibly hold ≥30 K
   (REBCO, arsenides) but not MgB₂'s 20 K. A coil set's `cooling` field
   determines which vitamin it consumes: helium stock (sponsor), H₂-cooler
   (T3–T4 local, needs bearings/seals), or passive shade (T2 foil, slow
   cool-down, quench-prone). Passive-shade conversion is the cheap act-2
   retrofit that trades reliability for independence. Evidence: MAARSS-II §14.

9. **Stray-field texture (all acts).** Coil ships have a 0.5 mT line: no
   pacemakers, no CRTs, non-magnetic tools, docking approaches that respect the
   fringe field, and a quench alarm that everybody aboard understands (a 4 T
   set holds 9 t of TNT). This is cheap storylet material — the tube-era
   electronics of 03's Voidborn are field-sensitive, so sailor ships keep their
   radios in the compensated zone. Evidence: MAARSS-I §10; ICNIRP; 03.

10. **Water as shield-and-propellant with a dose charge (acts 1–2).** When a
    sailor ship steams its shield water, recompute the water factor and let
    the medical advisor object. Evidence: 1.5; 02's steam numbers.

## 4. Open questions

- **The combined magnet + water factor.** My sub-multiplicative exponent (0.85)
  is a guess; MAARSS tabulated the magnet only against 5–10 g/cm². An OLTARIS
  or GEANT4 run of a 20 T·m solenoid with 50–100 g/cm² of water would fix the
  sailor dose to ±20 % instead of ±40 %.
- **SR2S final numbers.** The FP7 project's pumpkin design (MgB₂, ~10 m) has
  published masses and dose reductions I could not retrieve this session;
  they should replace my recollection in 1.1 and confirm whether MgB₂ at
  10–20 K closed or not.
- **Magsail kinetic regime at 2.7 AU.** The bubble (100 km) is smaller than the
  ion gyroradius (178 km). Fujita/Nishida-type simulations at that ratio
  would set the ×0.5–1 factor; if it is ×0.2, magsails are station-keeping
  only.
- **Lift.** Zubrin's L/D 0.28 is a kinetic estimate; MHD gives less. Sunward
  performance decides whether sailors can ever come back in without steam.
- **Plasma magnet f_o.** No flight data. Decide whether the sim treats it as a
  hidden draw (item 6) or excludes it.
- **Child dose policy.** Does the game let a Voidborn sailor culture raise
  children aboard at a modelled 20 % penalty, or does it hard-code the burrow?
  Item 2 argues for the slider; NOTES currently says the split is physics.
- **Iron-based wire at 20 K.** Performance and length data for 1144/122 PIT
  conductors at 20–25 K are thin; my ×5–10 crudeness is reasoned, not sourced.
- **Passive coil cooling near a warm habitat.** MAARSS-II's negative heat load
  assumed the coils see only the sunshade and space; a habitat-side shade
  with e* 10⁻⁶ over 200 m² of coil surface has never been built. Needs a
  thermal model before "passive-shade" is allowed as a `cooling` value.
- **Spin-tether slings.** 2 km/s tips at 2 rpm are within fibre limits on
  paper; dynamics of catch/release on a crewed spun station are unstudied.

## 5. Sources

- Westover, Meinke, Battiston, Burger et al., *MAARSS Phase I final report*, NIAC 2013, NTRS 20130000761 — https://ntrs.nasa.gov/api/citations/20130000761/downloads/20130000761.pdf — 6+1 architecture, 8 T·m, 36–50 t, 309/451 mSv/yr, SPE ≈ 141 t PE, endcap funnelling, Tables 8.1–8.3 dose vs B × thickness.
- Westover, Meinke, Nerolich, Washburn, Battiston et al., *MAARSS Phase II final report*, NIAC 2019, NTRS 20190002579 — https://ntrs.nasa.gov/api/citations/20190002579/downloads/20190002579.pdf — system trade (1T×10m 61 t; 4T×4.75m 200 t), thermal loads and cryocooler tables, passive sunshade recommendation, "no dramatic improvement" conclusion.
- Battiston, Burger, Calvelli, Musenich et al., *Active Radiation Shield for Space Exploration Missions (ARSSEM)*, ESA study, arXiv:1209.1907 (2012) — https://arxiv.org/abs/1209.1907 — 15 toroidal geometries, Double-Helix 37 t at 4–5 T·m, 40 % combined reduction, secondaries and thin-shield warnings, dose-by-species tables.
- D'Onghia, Desiati, Bednarz, Pfotenhauer et al., *CREW HaT Phase I final report*, NIAC 2022, NTRS 20250002403 — https://ntrs.nasa.gov/api/citations/20250002403/downloads/NIAC_2022_PhI_DOnghia_CREWHat.pdf — 8 × 10⁷ A Halbach torus, 10 T peak at 40 K, 120 t, 190 W heat load, 20 % total GCR reduction ≈ 100 t passive, 1000 km tape per coil, crack/quench discussion.
- Sailer, *Magnetic Shielding for Interplanetary Travel*, arXiv:1902.10122 (2019) — https://arxiv.org/abs/1902.10122 — split toroid, 80.8 t wire, 0.47 T, <0.22 mT in crew area.
- Andrews & Zubrin, *Use of magnetic sails for advanced exploration missions*, NASA CP (1990), NTRS 19910012840 — https://ntrs.nasa.gov/api/citations/19910012840/downloads/19910012840.pdf — 264.5 N drag / 74.1 N lift / L/D 0.28, 250 N average, 57.7 kA optimum loops, 720 N hoop force, TPS 0.029 kg/m.
- Wikipedia, *Magnetic sail* — https://en.wikipedia.org/wiki/Magnetic_sail — synthesis of Freeland's ×3.1 correction, Nishida's C_d, f_o falloff exponents for magsail/M2P2/plasma magnet/MPS, solar-wind parameters, critiques of M2P2.
- Wikipedia, *Mini-magnetospheric plasma propulsion* — https://en.wikipedia.org/wiki/Mini-magnetospheric_plasma_propulsion — M2P2 claims and the Khazanov/Cattell/Toivanen counter-models.
- Wikipedia, *Electric sail* — https://en.wikipedia.org/wiki/Electric_sail — 0.5 N for 540 W baseline, 50–100 × 20 km tethers, 30° thrust angle, ESTCube-1/Aalto-1 failures.
- Wikipedia, *Solar wind* — https://en.wikipedia.org/wiki/Solar_wind — 1–6 nPa, 3–10/cm³, 1/r² density.
- Perakis & Hein, *Combining Magnetic and Electric Sails for Interstellar Deceleration*, arXiv:1603.03015 (2016) — https://arxiv.org/abs/1603.03015 — magsail force formula and mass model (Je 2×10¹⁰ assumption, 6000 kg/m³, 15 % tether mass).
- Gros, *Universal scaling relation for magnetic sails*, arXiv:1707.02801 (2017) — https://arxiv.org/abs/1707.02801 — magsail scaling and critical current 1.55 MA.
- Wikipedia, *Abundances of the elements (data page)* — https://en.wikipedia.org/wiki/Abundances_of_the_elements_(data_page) — solar-system atom fractions (Lodders) converted here to ppm by mass.
- Wikipedia, *Magnesium diboride* — https://en.wikipedia.org/wiki/Magnesium_diboride — 39 K, Hc2, PIT wire, 18 km MRI magnet, CERN 20 kA cables.
- Wikipedia, *Iron-based superconductor* — https://en.wikipedia.org/wiki/Iron-based_superconductor — family compositions and T_c (LaFeAsO 26 K, Ba₀.₆K₀.₄Fe₂As₂ 38 K, FeSe, Hc2 43 T).
- Wikipedia, *Yttrium barium copper oxide* — https://en.wikipedia.org/wiki/Yttrium_barium_copper_oxide — IBAD/RABiTS/MOD routes, SuperOx 700–2000 A/mm², 300 km in 9 months.
- Wikipedia, *Cryocooler* — https://en.wikipedia.org/wiki/Cryocooler — helium as the working fluid of Stirling/pulse-tube/GM machines.
- ICRP Publication 103 (2007) and ICRP statement on tissue reactions (2011); BEIR VII (2006) — dose thresholds for sterility, lens, fetus; age-dependent cancer coefficients (cited from memory; standard values).
- NAS 2021 *Space Radiation and Astronaut Health* — 600 mSv career limit (already in 02-vessels sources).
- Recollection only, to be verified: SR2S project outputs (Musenich, Calvelli, Bruce & Baudouy 2014–16); Spillantini 2011 *Adv. Space Res.*; Hoffman, Fisher & Batishchev NIAC 2005; Slough & Kirtley plasma-magnet NIAC 2005–06; Funaki et al. JAXA MPS papers; Fujita 2004 kinetic magsail simulations; IEE-CAS iron-based tape results.
