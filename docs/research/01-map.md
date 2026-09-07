# 01 — The Map: the belt as a time-varying graph

Scope: the asteroid belt and near-Earth asteroids as nodes and edges for a turn-based
simulation. Most delta-v numbers below were computed for this report from real JPL SBDB
orbital elements with a two-body Lambert solver (method and validation in 1.6); the
population, composition and data-source numbers come from the literature and from live
queries to the JPL and NHATS APIs on 2026-09-06.

## 1. Findings

### 1.1 Physical geography

**Population.** JPL SBDB currently holds 1,416,280 main-belt asteroids (2.0 < a < 3.3 AU,
q > 1.666 AU): 387k inner (2.0–2.5), 534k middle (2.5–2.82), 495k outer (2.82–3.3); the
inner belt is over-represented because it is easier to observe. Bottke et al. (2005)
estimate the true cumulative counts as ~1.36 million bodies with D > 1 km, 680 with
D > 50 km, and 220 with D > 100 km; the size distribution has a fossil "elbow" at ~100 km.
SBDB has measured diameters (mostly NEOWISE) for 133k main-belt bodies: 189 > 100 km,
576 > 50 km, 7,715 > 10 km (complete above ~30–50 km, incomplete below). Total belt mass
is 2.39×10²¹ kg (~3% of the Moon); Ceres alone is 39%, and Ceres + Vesta + Pallas +
Hygiea are 62%. Everything else is crumbs: the remaining ~1.4 million bodies share a
third of 3% of a Moon.

**Spatial spacing.** The commonly quoted "average distance between asteroids ~1 million
km" is not right for kilometre-class bodies. Taking the belt as an annulus from 2.1 to
3.3 AU, 1 AU thick (volume ≈ 20 AU³ ≈ 6.8×10²⁵ km³; this is the Lucy team's own
volume), mean spacing (V/N)^(1/3) is:

| population | N | mean spacing | in lunar distances |
|---|---|---|---|
| D > 100 km | 220 | 0.45 AU (6.8×10⁷ km) | 176 |
| D > 10 km | ~10⁴ | 0.13 AU (1.9×10⁷ km) | 49 |
| D > 1 km | 1.36×10⁶ | 0.025 AU (3.7×10⁶ km) | 10 |
| D > 100 m | ~10⁸ (rough) | 0.006 AU (0.9×10⁶ km) | 2.3 |

So the "few hundred thousand km" figure belongs to the 100-m population. A settler at a
random km-class rock has the next km-class rock ~12 light-seconds away and the next
10-km rock a light-minute away; neither is visible to the eye. For a game, this means
spatial distance is essentially never the constraint. What matters is delta-v distance,
which is a different geometry (see below).

**Delta-v spacing (the geometry that matters).** Using the Zappalà et al. (1990) metric
(the standard proximity measure used to find asteroid families; it approximates the
impulsive cost to change from one orbit to another, ignoring phasing) over all 43,437
known bodies with D > 5 km:

- median nearest-neighbour delta-v among D > 5 km bodies: 86 m/s (10th–90th pct 39–180 m/s)
- among the 1,973 bodies with D > 20 km: 264 m/s (117–569)
- among the 615 bodies with D > 50 km: 450 m/s (195–855)

Around candidate hubs, number of known D > 5 km bodies within (0.3 / 0.5 / 1 / 2 km/s):

| hub | a, e, i | D>5 km within 0.3/0.5/1/2 km/s | D>20 km within 0.5/1/2 |
|---|---|---|---|
| Ceres | 2.77, 0.08, 10.6° | 10 / 29 / 228 / 4,173 | 2 / 15 / 234 |
| Vesta | 2.36, 0.09, 7.1° | 1 / 14 / 120 / 852 | 2 / 8 / 47 |
| Psyche | 2.93, 0.13, 3.1° | 8 / 34 / 339 / 5,516 | 1 / 10 / 262 |
| Hygiea | 3.15, 0.11, 3.8° | 56 / 226 / 1,613 / 6,435 | 5 / 61 / 291 |
| Themis | 3.14, 0.12, 0.7° | 52 / 202 / 1,148 / 4,251 | 15 / 72 / 220 |
| Eunomia | 2.64, 0.19, 11.8° | 63 / 265 / 1,096 / 3,686 | 4 / 29 / 149 |
| Flora | 2.20, 0.16, 5.9° | 5 / 19 / 155 / 792 | 1 / 2 / 16 |
| Pallas | 2.77, 0.23, 34.9° | 1 / 2 / 12 / 56 | 0 / 0 / 1 |

Two things follow. First, the belt is a graph of *neighbourhoods*: for a few hundred
m/s you can reach tens of multi-km bodies; for 1 km/s, hundreds; for 2 km/s, thousands.
Second, hubs differ enormously in degree: Hygiea/Themis (the outer, low-inclination,
water-rich region) sit inside dense families; Vesta and Flora in the inner belt are
comparatively sparse at the >5 km level (their families are mostly sub-5 km fragments);
Pallas, at 35° inclination, is an island nobody visits.

**The catch: phasing.** Zappalà delta-v ignores where the body is along its orbit. Two
bodies on near-identical orbits can be on opposite sides of the Sun, and their synodic
period is then centuries, so they never come around. Closing a longitude gap Δλ over N
orbits by a phasing manoeuvre costs approximately

    ΔV_phase ≈ v_orb × (Δλ / 540°) / N        (raise-then-lower, small Δa)

At Ceres (v_orb = 17.9 km/s, T = 4.6 yr): Δλ = 30° in one orbit costs 1.0 km/s, in
three orbits 0.33 km/s; Δλ = 180° in one orbit costs 6 km/s, in ten orbits (46 years)
0.6 km/s. Delta-v times waiting time is roughly constant: hurry is bought with propellant.
This is the core economic law of intra-neighbourhood travel and should be modelled
explicitly rather than hidden inside a Lambert solver.

**Major bodies worth a node** (elements from SBDB, epoch 2026; flux and light-lag
computed):

| body | a (AU) | e | i (°) | D (km) | class | flux at q/Q (W/m²) | one-way lag to Earth (min) | note |
|---|---|---|---|---|---|---|---|---|
| 1 Ceres | 2.766 | 0.080 | 10.6 | 939 | C | 210/153 | 13–33 | 39% of belt mass, ~25% water ice, v_esc 0.52 km/s |
| 4 Vesta | 2.361 | 0.090 | 7.1 | 523 | V | 295/205 | 9–30 | dry basaltic (HED), Fe core, v_esc 0.36 |
| 2 Pallas | 2.770 | 0.231 | 34.9 | 513 | B | 300/117 | 9–37 | i = 35° makes it the most expensive big body |
| 10 Hygiea | 3.151 | 0.107 | 3.8 | 407 | C | 172/112 | 15–38 | outer belt, huge family, hydrated |
| 16 Psyche | 2.926 | 0.135 | 3.1 | 222 | X (M) | 212/123 | 13–36 | 2.29×10¹⁹ kg, ρ = 3.98, metal-silicate mix, not a pure core |
| 704 Interamnia | 3.057 | 0.155 | 17.3 | 306 | B | 204/109 | 13–38 | |
| 52 Europa | 3.094 | 0.112 | 7.5 | 304 | C | 180/115 | 14–37 | "Europa-like" 3-µm ice group namesake |
| 511 Davida | 3.162 | 0.189 | 15.9 | 270 | C | 207/96 | 13–40 | |
| 87 Sylvia | 3.491 | 0.094 | 10.8 | 253 | X | 136/93 | 18–40 | Cybele region, two moons |
| 65 Cybele | 3.407 | 0.128 | 3.6 | 237 | Xc | 154/92 | 16–40 | water ice + organics detected |
| 15 Eunomia | 2.642 | 0.188 | 11.8 | 232 | S | 296/138 | 9–35 | biggest S-type, dense family |
| 3 Juno | 2.671 | 0.256 | 13.0 | 247 | Sk | 344/121 | 8–36 | |
| 24 Themis | 3.144 | 0.115 | 0.7 | 198 | B | 176/111 | 15–38 | surface water ice confirmed; main-belt comets in family |
| 88 Thisbe | 2.766 | 0.166 | 5.2 | 232 | B | 255/131 | 11–35 | same a as Ceres: frozen +129° ahead of it |
| 19 Fortuna | 2.442 | 0.158 | 1.6 | 200 | Ch | 322/170 | 9–32 | hydrated C in the inner belt at i = 1.6° |
| 13 Egeria | 2.576 | 0.085 | 16.5 | 203 | Ch | 245/174 | 11–32 | |
| 8 Flora | 2.202 | 0.156 | 5.9 | 147 | S | 394/210 | 7–30 | inner edge; family of ~14k |
| 20 Massalia | 2.408 | 0.144 | 0.7 | 136 | S | 320/179 | 9–31 | i = 0.7°, cheap inner-belt S |
| 44 Nysa | 2.422 | 0.150 | 3.7 | 71 | Xc/E | 321/175 | 9–32 | Nysa-Polana complex, ~19k members |
| 216 Kleopatra | 2.795 | 0.250 | 13.1 | 122 | Xe (M) | 310/111 | 9–38 | metallic dog-bone, two moons |
| 21 Lutetia | 2.434 | 0.165 | 3.1 | 98 | Xk | 329/169 | 9–32 | Rosetta flyby, ρ = 3.4 |
| 153 Hilda / 279 Thule | 3.97 / 4.27 | | | 171 / 127 | X/P | 116/67 | 20–46 | 3:2 resonance "Hildas", outer frontier |
| 624 Hektor | 5.277 | 0.024 | 18.1 | 225 | D | 51/47 | 34–53 | Trojan; beyond the game's belt |

**Families.** Asteroid families (Nesvorný HCM catalogue, PDS SBN) are delta-v-cheap,
compositionally homogeneous clusters: Nysa-Polana ~19,000 members, Vesta ~15,000,
Flora ~14,000, Eos ~9,800, Koronis ~5,900, Eunomia ~5,700, Hygiea ~4,900, Themis ~4,800
(counts as of the 2015 catalogue; ~120 families total). Family members share their
parent's composition (Vesta family = basalt; Themis/Hygiea = hydrated C/B with ice; Eos
= K-type; Koronis = S). Family velocity dispersions are ~50–300 m/s, so a family is
precisely an "archipelago" in the maritime sense: many islands, same rock, cheap to sail
between, but each on its own phase.

**Kirkwood gaps and zones.** Gaps sit at Jupiter mean-motion resonances: 4:1 at 2.065
AU, 3:1 at 2.502, 5:2 at 2.825, 7:3 at 2.958, 2:1 at 3.279 AU. They define the
conventional zones: inner (2.06–2.50), middle (2.50–2.82), outer (2.82–3.28), then the
Cybele region (3.3–3.5), Hildas at the 3:2 (3.97), Trojans at 5.2. The gaps are empty
of nodes (anything there has its eccentricity pumped on 10⁵–10⁶-year timescales) but are
not hazards on game timescales; treat them as straits with nothing in them. Crossing a
gap is just a Hohmann-like a-change: 2.5 → 2.82 AU costs about 1.1 km/s coplanar.

**Composition vs distance.** DeMeo & Carry (2014, Nature 505, 629) mapped the belt by
*mass* down to 5 km. Headlines: the C-complex holds ~60% of belt mass (DeMeo & Carry
2013); S-types dominate the inner belt at large sizes (C is only 6% of inner-belt mass at
D > 100 km) but C rises to half of inner-belt mass at 5–20 km; the middle belt is mixed
with Ceres dominating its mass; the outer belt is C/B/P/D-dominated but still holds
more than half of all S-type mass in absolute terms. Their key finding is that *every
type exists in every zone*: the belt is mixed, not a clean gradient. M/X (metal-rich)
bodies are scattered: Lutetia at 2.43, Kleopatra at 2.80, Psyche at 2.93 AU.

Where the water is, in decreasing convenience:

1. *Hydrated minerals* (phyllosilicates; Ch/Cgh types; "sharp" 3-µm band): concentrated
   at 2.5–3.3 AU (Takir & Emery 2012 group distribution; Rivkin's Ch statistics), with
   real inner-belt members (Fortuna at 2.44, Egeria at 2.58). Water is bound at ~5–13 wt%
   (CM-chondrite-like) and must be baked out at 300–900 °C.
2. *Ceres*: ~25% water ice by mass, a 3.1-µm feature since Lebofsky (1978), brines and
   surface ice in shadowed craters, an ice-rich crust 20–40 km thick. The single largest
   accessible volatile reservoir in the belt by a wide margin.
3. *Surface/near-surface ice* on outer-belt bodies: Themis (3.14 AU, Campins et al. and
   Rivkin & Emery 2010), Cybele (3.43, Licandro et al. 2011); "rounded" 3-µm ice-like
   groups dominate beyond ~3.3–3.4 AU (Cybeles, Hildas). Main-belt comets (Hsieh &
   Jewitt 2006) cluster in the Themis family and outer belt beyond ~3 AU.
4. The modern belt "snow line" (surface-ice stability) is roughly 2.7–3.0 AU; buried ice
   survives well inside that under a metre of regolith.

Volatiles other than water (CO₂, NH₃, organics) track the same outward gradient; Ceres has
ammoniated clays and carbonates, meaning nitrogen and carbon are available there, which
matters for a closed-loop life-support economy more than water does.

### 1.2 Delta-v and time

**Literature baselines (impulsive, from LEO).** Shoemaker–Helin-type figures used by
Ieva et al. (A&A 2014): Moon 6.0 km/s, Mars 6.3 km/s. Taylor et al. (2018, Acta
Astronautica 146, 73) computed two- and three-burn LEO delta-v for 596,713 main-belt
asteroids: median 9.96 km/s; 34,760 below 8.5, 3,986 below 8.0, 96 below 7.5, 4 below
7.0 km/s; the lowest is (271774) at 6.83 km/s. The 19 cheapest (< 7.3 km/s) all have
i < 4°, e ≈ 0.03–0.34, a = 1.63–2.31 AU, transfer times 280–535 days and synodic periods
510–700 days, and are 200 m–4.7 km in diameter. Their NEO sample (15,922) has a median of
9.22 km/s by the same method; NHATS-style optimisation typically finds ~0.5 km/s less.

**Computed porkchops.** For this report I built a two-body Lambert porkchop
(Curtis universal-variable formulation, zero-revolution, prograde) over the SBDB elements
and swept departure dates over 1–2 synodic cycles with 5–30 day steps and time of flight
in 5–25 day steps. Earth departures are charged from a 200-km LEO (Δv = √(v∞² + 2μ/r) −
√(μ/r)); arrivals at Ceres/Vesta are charged into a 100-km orbit; other arrivals are
v∞ (matching heliocentric velocity, which is essentially the whole cost since v_esc of
everything but the big four is < 0.25 km/s). "Best" is the minimum over the sweep;
percentiles are of the per-departure-date minimum over TOF; "within 20%/50%" is the
fraction of departure dates whose best transfer costs ≤ 1.2× / 1.5× the global best.

| edge | synodic (yr) | Hohmann coplanar | low-thrust Edelbaum | best (km/s) | TOF at best (d) | p25 | median | p75 | worst | within 20% | within 50% |
|---|---|---|---|---|---|---|---|---|---|---|---|
| LEO → Ceres orbit | 1.28 | 11.2 | 13.6 | **10.25** | 542 | 14.9 | 20.5 | 28.5 | 41 | 6% | 29% |
| LEO → Vesta orbit | 1.38 | 9.9 | 11.4 | **8.68** | 470 | 13.1 | 17.2 | 24.9 | 37 | 5% | 24% |
| LEO → Psyche | 1.25 | 11.6 | 12.5 | **9.69** | 478 | 13.8 | 19.2 | 27.9 | 42 | 11% | 31% |
| Ceres → Vesta | 17.2 | 1.47 | 2.29 | **2.70** | 1,140 | 3.9 | 5.0 | 9.8 | 11.7 | 10% | 30% |
| Vesta → Ceres | 17.2 | 1.47 | 2.29 | **2.71** | 940 | 3.8 | 6.4 | 7.6 | 19.5 | 10% | 31% |
| Ceres → Pallas | 2,120 | (0.01) | 11.7 | **12.8** | 1,190 | 13.6 | 14.4 | 15.1 | 16.9 | 81% | 100% |
| Ceres → Hygiea | 25.9 | 1.13 | 3.40 | **4.67** | 1,790 | 6.4 | 8.3 | 11.8 | 22.2 | 11% | 34% |
| Ceres → Psyche | 56.8 | 0.50 | 3.65 | **4.10** | 1,475 | 5.2 | 7.0 | 12.5 | 20.8 | 21% | 43% |
| Vesta → Psyche | 13.2 | 1.96 | 2.83 | **3.03** | 1,250 | 4.5 | 8.9 | 12.2 | 25.4 | 10% | 26% |
| Ceres → Themis | 26.3 | 1.11 | 4.80 | **3.58** | 1,175 | 5.7 | 10.1 | 15.1 | 22.9 | 6% | 21% |
| Vesta → Flora | 32.7 | 0.69 | 0.97 | **2.64** | 600 | 4.9 | 7.2 | 9.5 | 14.2 | 8% | 13% |
| Vesta → Eunomia | 23.4 | 1.06 | 2.61 | **7.62** | 1,025 | 9.2 | 11.3 | 15.8 | 23.5 | 25% | 52% |
| Ceres → Thisbe (co-orbital, +129°) | 12,600 | (0.00) | 2.63 | **10.8** (zero-rev) | 750 | 12.1 | 13.3 | 14.3 | 15.1 | 43% | 100% |
| LEO → Fortuna (inner, Ch, i 1.6°) | 1.36 | 10.2 | 10.8 | **9.03** | 525 | 12.5 | 16.4 | 23.9 | 39 | 16% | 29% |
| LEO → Massalia (inner, S, i 0.7°) | 1.37 | 10.1 | 10.6 | **8.78** | 525 | 12.3 | 16.8 | 24.0 | 38 | 15% | 32% |
| LEO → Flora (inner edge) | 1.44 | 9.4 | 10.5 | **9.88** | 405 | 12.6 | 16.3 | 23.9 | 36 | 19% | 39% |
| LEO → Eunomia (middle, i 11.8°) | 1.30 | 10.8 | 13.7 | **10.31** | 330 | 15.1 | 19.9 | 27.5 | 41 | 10% | 27% |
| LEO → Hygiea (outer) | 1.22 | 12.1 | 13.2 | **10.85** | 620 | 14.9 | 21.5 | 30.5 | 44 | 17% | 31% |
| LEO → Mars (v∞ arrival) | 2.14 | 5.6 | 5.8 | **6.22** | 310 | 10.9 | 14.2 | 21.4 | 32 | 12% | 21% |
| Mars → Vesta (v∞ to v∞) | 3.90 | 4.7 | 5.7 | **6.10** | 705 | 12.7 | 18.8 | 21.8 | 29 | 6% | 11% |
| Mars → Ceres (v∞ to v∞) | 3.18 | 6.1 | 8.0 | **7.18** | 705 | 11.7 | 18.7 | 24.7 | 39 | 6% | 16% |
| LEO → 2000 SG344 (a 0.98, H 24.7, ~40 m) | 28.8 | 0.34 | 0.35 | **3.36** | 280 | 5.0 | 6.5 | 8.2 | 12.2 | 9% | 26% |
| LEO → 2008 EV5 (a 0.96, 400 m) | 15.7 | 0.62 | 6.2 | **4.43** | 140 | 7.5 | 8.9 | 10.6 | 14.2 | 2% | 12% |
| LEO → Apophis (a 0.92, 340 m) | 7.8 | 1.2 | 3.0 | **4.73** | 80 | 7.5 | 9.1 | 11.0 | 12.9 | 3% | 18% |
| LEO → Ryugu (a 1.19, 900 m) | 4.3 | 2.5 | 5.2 | **4.83** | 470 | 7.9 | 10.3 | 13.8 | 25 | 3% | 22% |
| LEO → Bennu (a 1.13, 490 m) | 6.1 | 1.7 | 5.1 | **5.09** | 390 | 7.9 | 10.3 | 13.2 | 19.7 | 4% | 22% |
| LEO → 1996 FG3 (a 1.05, 1.2 km, C) | 13.0 | 0.8 | 1.8 | **6.30** | 290 | 8.8 | 11.8 | 14.9 | 21 | 14% | 30% |
| LEO → 2000 FJ10 (a 1.32) | 2.9 | 3.8 | 5.6 | **6.52** | 490 | 8.2 | 12.8 | 16.1 | 20.5 | 24% | 34% |
| Ceres → Earth, aerocapture (departure v∞ only) | 1.28 | | | **4.96** | 510 | 5.4 | 6.2 | 6.9 | 8.3 | 42% | 88% |
| Vesta → Earth, aerocapture | 1.38 | | | **4.12** | 470 | 5.4 | 6.2 | 7.1 | 8.4 | 14% | 49% |
| Fortuna → Earth, aerocapture | 1.36 | | | **3.61** | 520 | 4.8 | 5.6 | 6.7 | 8.5 | 17% | 44% |

(NEA rows are one-way rendezvous, LEO departure plus v∞ matching; the NHATS figures in
1.3 are round trips and so roughly double these. The return rows charge only the
departure burn, assuming Earth aerocapture or an Earth-side tug; the cheapest departures
arrive at Earth with v∞ of 7–9.6 km/s, i.e. 13–14.5 km/s entry, harder than a lunar
return. Because only departure counts, the *homeward* direction is far less seasonal than
the outbound one: Ceres → Earth stays within 20% of its best 42% of the time. The NEA sweeps cover one full synodic
cycle each, so "within 20%" for 2000 SG344 means ~9% of 29 years ≈ 2.6 years of cheap
access per cycle: cheap NEAs have long windows but rare ones.)

Reading the table:

- *Earth ↔ big belt bodies* cost 8.7–10.3 km/s from LEO at the best of each 15-month
  cycle, with 16–18-month flights. The cheap window (≤ 1.2× best) is ~5–11% of the
  cycle, i.e. **4–6 weeks every 15–16 months**; the usable window (≤ 1.5×) is ~4–5
  months. Outside it a ballistic transfer is prohibitive (the median across the cycle is
  17–20 km/s). This is an unambiguous, strong season.
- *Ceres ↔ Vesta* costs 2.7 km/s at best but the best recurs only every 17.2 years;
  across the cycle it swings 2.7 → 11.7 km/s, and the ≤ 1.2× window (~3.2 km/s) lasts
  ~1.7 years, the ≤ 1.5× window (~4 km/s) ~5 years. Flights are 2.5–3 years. The other
  hub pairs behave the same way: Vesta → Psyche 3.0 km/s best on a 13-year cycle,
  Ceres → Themis 3.6 on 26 years, Ceres → Psyche 4.1 on 57 years, Ceres → Hygiea 4.7 on
  26 years, all with 3–5-year flights and cheap phases of 1–3 years. Big-body-to-big-body
  edges in the belt do not have seasons; they have *eras*.
- *Coming home is less seasonal than going out.* If Earth arrival is free (aerocapture,
  or a tug waiting in cislunar space), the return edge costs only the departure burn:
  Ceres → Earth 5.0 km/s best, never worse than 8.3, within 20% of best 42% of the time;
  Vesta → Earth 4.1 (14%), Fortuna → Earth 3.6 (17%). The outbound edge, which has to
  pay both ends, is the sharply seasonal one. A settlement can leave for Earth in most
  years; Earth can only reach the settlement in the window.
- *Vesta → Eunomia* (7.6 km/s best) shows that a middle-belt hub can be expensive from
  the inner belt when inclination (11.8°) and eccentricity (0.19) both work against you;
  Eunomia's rich neighbourhood is bought with a costly approach.
- *Ceres → Thisbe* is the instructive failure: the Edelbaum cost is 2.6 km/s, but the
  zero-revolution Lambert optimum is 10.8 km/s, because Thisbe sits 129° ahead on the
  same-period orbit and a ≤ 4-year ballistic arc has to buy that phase with a huge orbit
  change. The phasing formula in 1.1 gives the right answer: 129° over 3 orbits (14 yr)
  costs 1.4 km/s, over 10 orbits 0.4 km/s, plus the 2.6 km/s orbit change. Same-orbit
  neighbours must be modelled as phasing problems, not as Lambert edges.
- *Co-orbital pairs* (same a): Ceres–Pallas (synodic 2,120 yr), Ceres–Thisbe (12,600
  yr), Hygiea–Themis (1,680 yr). Their relative geometry is frozen for the length of
  any game: on 2026-01-01 Thisbe sits 129° ahead of Ceres, Pallas 53° behind, Themis 130°
  ahead of Hygiea, and those offsets will not change. Pallas is *permanently* 12.8–16.9
  km/s from Ceres because of its inclination. These are fixed geography.
- The Hohmann and Edelbaum columns show the impulsive coplanar ideal and the low-thrust
  circular-to-circular ideal (including the plane change through cos(πΔi/2)); the real
  ballistic optimum sits between them because of eccentricity and inclination phasing.

**Low thrust changes the shape, not the size.** Dawn (1,218 kg, 91 mN, Isp 3,100 s,
0.075 mm/s²) delivered 10.8 km/s from its ion engines: launch Sep 2007, Mars gravity
assist Feb 2009 (worth 2.6 km/s), Vesta arrival Jul 2011, Vesta departure Sep 2012, Ceres
arrival Mar 2015. Vesta → Ceres took 2.5 years at that acceleration; the Edelbaum ideal
for that hop is 2.29 km/s, i.e. ~350 days of continuous thrust plus coasting to catch the
phase. Thrust time scales as Δv/acceleration: at 0.1 mm/s² 2.3 km/s takes 270 days and
11 km/s takes 3.5 years; at 0.5 mm/s² those drop to 53 days and 8.5 months. With low
thrust the cost of an edge is nearly independent of departure date (you can always spiral
to the target orbit) but the *time of arrival* depends on phasing: seasons move from the
cost axis to the time axis. A settlement fleet mixing chemical (fast, seasonal) and
electric (slow, steady) ships therefore experiences two different maps.

**Round trips.** Earth ↔ Ceres one-way flights are ~1.5 years each way and the windows
recur every 1.28 years, so a ballistic round trip is at least 3–4.5 years including the
wait at Ceres. A sponsor's supply ship that arrives cannot leave for a year. Belt-to-belt
round trips between big bodies are 5–6 years plus a wait of up to a decade for the return
era.

**Synodic periods (computed from SBDB semi-major axes):**

| pair | synodic (yr) | pair | synodic (yr) |
|---|---|---|---|
| Earth–Ceres | 1.28 | Ceres–Vesta | 17.2 |
| Earth–Vesta | 1.38 | Ceres–Hygiea | 25.9 |
| Earth–Psyche | 1.25 | Ceres–Themis | 26.3 |
| Earth–Hygiea | 1.22 | Ceres–Psyche | 56.8 |
| Earth–Flora/Hebe | 1.44 | Ceres–Eunomia | 64.8 |
| Earth–Mars | 2.14 | Vesta–Psyche | 13.2 |
| Earth–Eros | 2.3 | Vesta–Eunomia | 23.4 |
| Earth–Didymos | 1.9 | Vesta–Flora | 32.7 |
| Earth–2000 FJ10 / Itokawa | 2.9 | Vesta–Hebe | 92 |
| Earth–Ryugu | 4.3 | Psyche–Hygiea | 47.5 |
| Earth–Bennu | 6.1 | Ceres–Pallas | 2,120 |
| Earth–Apophis | 7.8 | Hygiea–Themis | 1,680 |
| Earth–1996 FG3 | 13.0 | Ceres–Thisbe | 12,600 |
| Earth–2008 EV5 | 15.7 | | |
| Earth–2000 SG344 | 28.8 | | |
| Earth–Kamoʻoalewa | ~820 (quasi-satellite) | | |

The pattern: the closer two orbits are in size, the cheaper the hop and the rarer the
opportunity. Delta-v accessibility and window frequency are inversely related everywhere
on the map.

### 1.3 Near-Earth asteroids vs the main belt

**Population.** 42,275 known NEAs (SBDB, Sep 2026); 867 with H < 17.75 (~> 1 km)
against an estimated total of 981 ± 19 (Harris & Chodas 2021), so the km-class census
is ~88% complete; 11,588 with H < 22 (> 140 m) against an estimated 25,000–35,000; an
estimated 840,000 > 40 m. Debiased composition matches the main-belt source regions
(MITHNEOS, Marsset et al. 2022): S/Q-dominated, with ~20–30% C-complex and an excess of
D-types. Diameters are measured for only 1,245 NEAs and spectra for 313.

**Accessibility, from NHATS (round trip, ≤ 450 days, launches 2020–2045, queried live):**

| total round-trip Δv | targets | with H < 22 (> ~140 m) | largest |
|---|---|---|---|
| ≤ 6 km/s | 711 | **0** | 2022 RB5, H 22.5 (55–245 m) |
| ≤ 8 km/s | 2,148 | 16 | Apophis (~340 m, 6.05 km/s), 2008 EV5 (400 m, 6.29), Bennu (490 m, 7.0) |
| ≤ 10 km/s | 4,248 | 72 | 2003 SD220 (0.5–2.4 km, 9.85) |
| ≤ 12 km/s | 7,037 | 199 | Anteros (2.3 km, 11.7) |

The 38 targets under 4 km/s round trip are all H > 27 (a few metres to ~20 m).
Elvis et al. (2011) defined ultra-low-Δv objects as < 4.5 km/s one-way from LEO; 65 of
6,699 NEOs known in 2010 qualified, nearly all tiny. Elvis (2014) then estimated that
only ~1 in 1,100 NEOs is water "ore" (accessible, big enough, carbonaceous, actually wet)
and ~1 in 2,000 is platinum-group-metal ore: roughly 18 water-ore NEOs larger than 100 m
and ~10 PGM-ore NEOs in the whole population.

**Why the cheap ones are cheap: they are on Earth-like orbits, so they almost never come
back.** Synodic periods with Earth: 2000 SG344 28.8 yr, 2008 EV5 15.7 yr, 1996 FG3 13.0
yr, Apophis 7.8 yr, Bennu 6.1 yr, Ryugu 4.3 yr; only the a ≈ 1.3–1.6 AU objects (2000
FJ10, Itokawa, Didymos, Eros) come round every 2–3 years, and those cost 7–10 km/s. The
first-principles version: Δv to an orbit of similar size is small, and the synodic period
1/|1/T₁ − 1/T₂| diverges as the periods converge. The NEA map is a set of isolated
points, each with a single edge to Earth that opens for a few months once a decade,
and with no cheap edges to each other (NEA–NEA nearest-neighbour Δv is km/s, not the
~100 m/s of belt neighbourhoods).

**Position.** Yes, some NEAs are cheaper than the Moon (round trip ≤ 6 km/s versus
~9 km/s LEO-to-lunar-surface-and-back), and a realistic sponsor's *industry* starts
there: that is where propellant-water and test mining happen first. But a realistic act
1 *settlement* does not start at an NEA:

1. Mass and diversity: every sub-6-km/s target is < 250 m (order 10⁷–10⁹ kg). A
   settlement that lives on it either exhausts it or has nowhere to go; there is no
   second resource type on site, no gravity, no neighbours.
2. Cadence: resupply and return windows to the cheap NEAs recur every 7–30 years. The
   sponsor–outpost tension the brief wants (act 1) needs a sponsor who *can* show up:
   the belt's big bodies have a window every 15 months; a cheap NEA does not.
3. Graph structure: there is no belt-like neighbourhood around an NEA, so "fleet =
   mobility = freedom" has nothing to act on; the only edge is back to Earth.

The alternative the evidence favours is a **low-inclination inner- or middle-belt body**:
Taylor's cheapest main-belt bodies cost 6.8–7.3 km/s from LEO, comparable to mid-tier
NEAs, and the inner belt contains 72,164 known bodies with i < 5°, e < 0.15, 2.1 < a <
2.5 AU, 101 of them larger than 10 km. 19 Fortuna (200 km, hydrated Ch, i = 1.6°, a =
2.44) and 20 Massalia (136 km, S, i = 0.7°) are the natural sponsor-outpost candidates
in the inner belt; Vesta is the obvious "big, dry, famous" hub. NEAs then work as the
prologue (the sponsor's existing NEA propellant business is why the ships exist) and,
in act 3, as the stepping stones back inward. If the design wants an NEA start
regardless, the honest version is a *drifting* start: a mining ship that has been
working NEAs is sent to the belt on a one-way contract; the NEA phase is backstory,
not gameplay.

### 1.4 Solar flux and light-lag

Solar flux is 1361 W/m² / r²: 281 W/m² at 2.2 AU (21% of Earth's), 178 at 2.77 AU
(13%), 133 at 3.2 AU (10%), 50 at Jupiter. Dawn's arrays gave 10 kW at 1 AU and 1.3 kW
at 3 AU. Photovoltaics remain workable across the belt with ~8–10× the array area per
kW of a 1-AU design (low-intensity/low-temperature effects are second-order); nuclear
becomes preferable beyond ~3 AU mostly for mass, not feasibility. Eccentric bodies have
real energy seasons: Ceres swings 210 → 153 W/m² (±16%) over its 4.6-year orbit; Pallas
300 → 117 (a factor 2.6); Juno 344 → 121; Bamberga 435 → 105 over its 4.4-year year.
For a settlement on an eccentric body, "winter" is a 2–3-year low-power phase.

Light-time is 8.3 min per AU. Earth ↔ Ceres one-way 13–33 min (round trip 25–66 min);
Earth ↔ inner belt 9–30 min; Earth ↔ outer belt 15–40 min; Earth ↔ Cybeles/Hildas up
to 46 min. Belt to belt: two bodies at ~3 AU are 0–6 AU apart, so 0–50 min one-way,
changing over months. Every synodic cycle the Sun blocks the Earth link for ~2–3 weeks
at conjunction (as for Mars). Consequences: nothing is teleoperated from Earth, ever;
Earth's control is contractual and logistical (ships, people, spares, software updates),
exercised on a 15-month cadence; within the belt, conversation between settlements is
correspondence with hours of latency, and only near neighbours (same family, same phase)
are ever within a minute. Bandwidth is the scarcer quantity: Dawn returned ~124 kbps from
3 AU to a 70-m DSN dish; a settlement without a large dish gets kbps–low-Mbps to Earth.
That is enough for text, code and compressed video, not enough for a culture to stream
Earth's culture live, which is a mechanism for the act-2 drift.

### 1.5 Data: sources, formats, practicality

- **JPL SBDB** (`https://ssd-api.jpl.nasa.gov/sbdb.api?sstr=Ceres&phys-par=1`): one
  object per call, JSON: osculating elements (a, e, i, Ω, ω, M, epoch, q, Q, period),
  orbit-quality metadata, and physical parameters (H, G, diameter, albedo, rotation
  period, GM, density, Bus/Tholen spectral class, colours) where known. No key, no
  documented rate limit; 77 objects took ~30 s.
- **JPL SBDB Query** (`sbdb_query.api`): bulk JSON with filters (`sb-cdata` JSON
  constraints on any field: `a|RG|2.0|3.3`, `diameter|GT|5`, `spec_B|DF`) and selectable
  `fields`; a 43k-row pull returned in seconds. Coverage today: diameters for 133k MBAs
  and 1,245 NEAs; spectral class for 1,227 MBAs and 313 NEAs; GM for 11 MBAs; density for 9.
  This is the practical primary source for the game's node set.
- **JPL Horizons API** (`https://ssd.jpl.nasa.gov/api/horizons.api`): numerically
  integrated ephemerides for any body and date, `EPHEM_TYPE=VECTORS` or `ELEMENTS`,
  `CENTER='@10'` for heliocentric, up to 10,000 epochs per call via `TLIST`, JSON or
  text. Best used offline to validate or to re-seed elements at a chosen game epoch, not
  at runtime.
- **NHATS API** (`ssd-api.jpl.nasa.gov/nhats.api?dv=6&dur=450`): round-trip
  accessibility tables for NEAs with H, size ranges, min Δv, min duration, and viable
  trajectory counts; CSV/JSON.
- **MPC MPCORB.DAT**: the whole catalogue (~1.4 M rows, ~200 MB) in fixed columns: packed
  designation (1–7), H (9–13), G, packed epoch (21–25), M, ω, Ω, i, e, mean motion, a
  (93–103), flags at 162–165. Fine for bulk, unnecessary for a curated set.
- **NEOWISE Diameters and Albedos V2.0** (PDS SBN, doi 10.26033/18S3-2Z54): thermal-model
  diameters/albedos for ~160k bodies; SBDB already ingests these as its `diameter`
  field, so the curated set does not need to touch it directly.
- **Nesvorný HCM families** (PDS SBN): family membership lists; the way to populate
  "neighbourhood" nodes around a hub with real, correctly-typed bodies.
- **Taxonomy**: SBDB `spec_B`/`spec_T` for the classical ~1,500; Gaia DR3 reflectance
  taxonomy (~14k objects, Mahlke-type classes) for breadth.

**Is a curated 30–100-body set with real ephemerides practical?** Yes, trivially. The
data pull for this report (77 bodies, elements + physics, JSON) is the prototype. Store
per node: designation, class, a, e, i, Ω, ω, M₀, epoch (JD TDB), H, D, albedo, spectral
class, and GM/radius for the few bodies where it matters (Ceres 62.6, Vesta 17.3,
Pallas 13.6, Hygiea 5.8, Psyche 1.5 km³/s²). Propagate with a Kepler solver (pure
function, deterministic, microseconds). Validation for this report: my two-body Ceres
position on 2030-01-01 from 2026 elements is within 0.5% of Horizons' integrated
vector (Earth within 0.03%); that is well inside anything a monthly-turn game resolves,
and errors stay at that level for decades except for bodies near resonances. Add
Earth, Mars and (for gravity assists) Jupiter from any almanac.

**What a patched-conic / Lambert approach needs from the data:** heliocentric osculating
elements per node (above); μ_sun; per-node GM and radius only for capture/landing at the
big four (v_esc 0.24–0.52 km/s; everything else is "dock"); planet GMs for LEO departure
and gravity assists; and a policy for TOF search bounds and multi-revolution handling
(zero-rev Lambert is adequate for TOF < ~1 transfer period; long slow hops within a
neighbourhood are better modelled with the phasing formula in 1.1 than with a
multi-rev Lambert). Rotation periods and pole directions are only needed for surface
operations, not for the map.

### 1.6 The map as a data structure, and how strongly edges oscillate

**Structure.** The map is *not* a graph with stored edge weights; it is a set of nodes
with orbital elements plus an edge-cost *function* `cost(A, B, t_depart, tof, mode) →
(Δv_depart, Δv_arrive)`. Two tiers fall out of the physics:

1. **Hubs** (10–30 big bodies plus Earth/Mars): full Lambert edges, precomputed as
   porkchop tables per ordered pair over the game's turn grid (departure turn × TOF turn).
   With monthly turns and TOF ≤ 48 months that is ≤ ~600 cells per pair per 50-year
   game, ~10⁵–10⁶ cells for 30 hubs: small enough to precompute and ship, or to compute
   lazily and cache (a Lambert solve is ~1 µs in Rust).
2. **Neighbourhoods** (family members and near-orbit rocks around each hub, from the
   Nesvorný lists and a Zappalà cut): constant Δv from the Zappalà metric (0.05–0.5
   km/s) plus a phasing term from the formula in 1.1 that trades Δv against wait time.
   Dozens to thousands of nodes per hub; they can be generated procedurally from the
   real family statistics if the full list is unwanted.

Low-thrust ships use the same nodes with a different cost function: near-constant
Edelbaum Δv, TOF = Δv/acceleration + a phasing wait.

**Oscillation strength, by regime (numbers from the porkchops above):**

| regime | period | cost swing over a cycle | cheap-window fraction | game-scale meaning |
|---|---|---|---|---|
| Earth → belt hub (outbound) | 1.2–1.4 yr | ~9–11 → 20–40 km/s (ballistic), i.e. open → closed | 5–19% (≤1.2×), 25–39% (≤1.5×) | true seasons: 1–3 months of "sailing season" every 15–16 months |
| belt hub → Earth (homeward, aerocapture) | 1.2–1.4 yr | 4–5 → 8.5 km/s | 14–42% / 44–88% | mild season: home is reachable most years |
| hub ↔ hub in the belt | 13–65 yr | 2.7–4.7 → 12–25 km/s | ~6–20% (1–3 yr), 20–43% (3–10 yr) | eras: a once-a-generation conjunction |
| co-orbital pairs | 10³–10⁴ yr | none | 100% | fixed geography: permanent neighbours, permanent mountains |
| inside a neighbourhood | phase-dependent | Δv ~constant; time varies | always open at a price | the hurry/wait trade |

**Is the window structure strong enough to be the game's seasons?** Yes, at the
Earth–belt scale, and it is even shaped like the maritime seasons the brief reaches for:
the Indian Ocean dhow trade ran on a 12-month monsoon cycle with ~2-month sailing windows
in each direction; the belt runs on a 15-month cycle with a 4–6-week cheap window and a
4-month usable one. Inside the belt, the equivalent structure is decadal, which is a
different and arguably better dramatic device: the Ceres–Vesta window that opens in year
12 of a campaign and closes in year 17 is an event, not a season. And the co-orbital
freezes give the map permanent features the player can learn. The recommendation is
therefore to keep the real ephemerides rather than abstracting to a static graph:
the three timescales come for free from the data, and they are the map's personality.

## 2. Numbers the sim needs

| parameter | value | unit / note |
|---|---|---|
| μ_sun | 1.32712440018×10¹¹ | km³/s² |
| AU | 1.495978707×10⁸ | km |
| light-time | 8.317 | min per AU |
| solar constant | 1361 / r² | W/m², r in AU |
| flux at 2.2 / 2.77 / 3.2 AU | 281 / 178 / 133 | W/m² |
| Ceres GM, radius, v_esc, v_circ(100 km) | 62.6, 470, 0.52, 0.33 | km³/s², km, km/s |
| Vesta GM, radius, v_esc | 17.3, 263, 0.36 | |
| Pallas GM, v_esc | 13.6, 0.33 | |
| Hygiea GM, v_esc | 5.8, 0.24 | |
| Psyche GM, v_esc, mass, density | 1.53, 0.17, 2.29×10¹⁹ kg, 3.98 g/cm³ | |
| belt total mass | 2.39×10²¹ | kg; Ceres 39%, top-4 62% |
| N(D>1 km), N(D>50 km), N(D>100 km) | 1.36×10⁶, 680, 220 | Bottke 2005 |
| known MBAs (2026) | 1,416,280 | inner 387k / middle 534k / outer 495k |
| mean spatial spacing D>1 km / D>10 km / D>100 km | 3.7×10⁶ / 1.9×10⁷ / 6.8×10⁷ | km |
| nearest-neighbour Zappalà Δv, D>5 / >20 / >50 km | 86 / 264 / 450 | m/s (median) |
| neighbours of Ceres within 0.5 / 1 / 2 km/s (D>5 km) | 29 / 228 / 4,173 | |
| neighbours of Hygiea within 0.5 / 1 / 2 km/s | 226 / 1,613 / 6,435 | |
| family velocity dispersion | 50–300 | m/s |
| phasing cost | ΔV ≈ v_orb·(Δλ/540°)/N | N = orbits waited; v_orb(Ceres) 17.9 km/s |
| Kirkwood gaps | 2.065, 2.502, 2.825, 2.958, 3.279 | AU (4:1, 3:1, 5:2, 7:3, 2:1) |
| zone bounds | 2.06–2.50 / 2.50–2.82 / 2.82–3.28 / 3.3–3.5 / 3.9–4.0 | inner / middle / outer / Cybele / Hilda, AU |
| C-complex share of belt mass | ~60% | DeMeo & Carry |
| C share of inner-belt mass at D>100 km / 5–20 km | 6% / ~50% | |
| hydrated (Ch/Cgh) concentration | 2.5–3.3 AU | ice-like 3-µm beyond ~3.3 AU |
| water in hydrated C material / Ceres | 5–13 wt% / ~25 wt% | bound vs ice |
| LEO → Ceres orbit, best / usable / synodic | 10.25 / ≤15 / 1.28 | km/s, km/s, yr; TOF 540 d |
| LEO → Vesta orbit | 8.68 / ≤13 / 1.38 | TOF 470 d |
| LEO → Psyche | 9.69 / ≤14.5 / 1.25 | TOF 480 d |
| LEO → Moon (surface) / Mars (Shoemaker-Helin) | 6.0 / 6.3 | km/s |
| lowest LEO → MBA (Taylor 2018) | 6.83; 3,986 bodies < 8.0; median 9.96 | km/s |
| Ceres ↔ Vesta best / median / worst / synodic | 2.7 / 5–6 / 12–20 / 17.2 | km/s, yr; TOF 2.6–3.1 yr |
| Ceres → Pallas (flat) | 12.8–16.9 | km/s, no cycle |
| Edelbaum Ceres–Vesta / Earth–Ceres | 2.29 / 13.6 | km/s (low thrust) |
| Earth-belt cheap window / usable window | 5–11% / 25–30% | of synodic cycle |
| hub-hub cheap window | ~10% / ~30% | of a 13–65-yr cycle |
| Dawn acceleration / Isp / total Δv | 0.075 mm/s² / 3,100 s / 10.8 km/s | |
| low-thrust time | t = Δv / a; 2.3 km/s @ 0.1 mm/s² = 270 d | |
| Earth ↔ Ceres light-lag | 13–33 | min one-way |
| Earth ↔ outer belt / Hildas light-lag | 15–40 / 20–46 | min one-way |
| belt ↔ belt light-lag | 0–50 | min one-way |
| conjunction blackout | ~2–3 weeks per synodic period | |
| Earth-link bandwidth (Dawn, 70-m DSN) | ~124 kbps at 3 AU | scales 1/d² |
| NEAs known / >1 km known (est.) / >140 m known (est.) | 42,275 / 867 (981) / 11,588 (25–35k) | |
| NHATS ≤6 km/s round trip: count / >140 m | 711 / 0 | |
| NHATS ≤8 km/s: count / >140 m | 2,148 / 16 | Apophis 6.05, 2008 EV5 6.29, Bennu 7.0 |
| ULDV (<4.5 km/s one-way) known in 2010 | 65 of 6,699 | all tiny |
| water-ore NEAs > 100 m (Elvis 2014) | ~18 | 1 in 1,100 |
| synodic: Bennu / 2008 EV5 / 2000 SG344 | 6.1 / 15.7 / 28.8 | yr |
| inner-belt low-i candidates (i<5°, e<0.15, 2.1–2.5 AU) | 72,164 known; 101 > 10 km | |
| Ceres flux swing (perihelion/aphelion) | 210 / 153 | W/m² |
| Pallas flux swing | 300 / 117 | W/m² |

## 3. Implications for mechanics

1. **Turn length = 1 month; the calendar has three clocks.** Evidence: Earth–belt
   windows recur every 15–16 months and the cheap window is 4–6 weeks wide; a Ceres year
   is 55 months; hub–hub conjunctions are 13–65 years. A monthly turn resolves the
   window (1–2 turns), makes a Ceres year ~55 turns and the Ceres–Vesta era ~200 turns.
   Acts: all.

2. **"Convoy season" as the act-1 heartbeat.** Every ~15 turns the sponsor's ships can
   arrive; what arrived is what you have for the next 15 turns; requests sent now are
   answered in 30+ turns (one-way lag of the *ship*, not the radio). Missing a window
   costs a year. The sponsor's leverage is exactly this cadence, and the act-1 end (being
   cut off) is simply a convoy that does not come. Evidence: 1.28-yr synodic, 5–19%
   cheap window, 3–4.5-yr round trips. Act 1.

2b. **Leaving is easier than being reached.** With aerocapture at Earth, the homeward
   edge is 4–5 km/s and open most of the time, while the outbound edge is 9–11 km/s and
   open a few months a cycle. Mechanically: the settlement's people can always defect
   or be recalled (a 1.5-year flight, one burn), but reinforcement and resupply are
   rationed by the calendar. This asymmetry is the sponsor-vs-self tension in physical
   form and should be visible in the UI as two different edge colours. Acts 1–2.

3. **Two ship classes = two maps.** Chemical/thermal ships (fast, seasonal: the porkchop
   map) and electric tugs (slow, steady: constant Δv, time varies). The player's
   fleet composition determines which map they live on. A settlement with only tugs
   never misses a window but never hurries; with only chemical ships it lives by the
   calendar. Evidence: Edelbaum vs Lambert results; Dawn's 2.5-year Vesta–Ceres hop.
   Acts 1–3.

4. **Neighbourhood travel is priced in time, not fuel: the hurry/wait trade.** Within a
   family, moving between rocks costs ~0.1–0.5 km/s of orbit change plus
   ΔV ≈ v·(Δλ/540°)/N of phasing. Expose this as a slider: "arrive in 1 orbit (1 km/s)
   or 3 orbits (0.33 km/s)". This is the mechanic that makes nomadism feel physical:
   patience is propellant. Acts 1–2.

5. **Hub degree is a real, uneven resource.** Ceres has ~30 multi-km neighbours within
   0.5 km/s, Hygiea/Themis/Eunomia ~200–260, Vesta ~14, Pallas 2. A settlement's
   "range" (rocks within its fleet's Δv budget) should be computed from real
   neighbourhoods, and the outer C/B families (Themis, Hygiea) should be where the
   water-rich, many-islands life is; the inner belt is where the sponsor can reach you.
   That tension (sponsor-reachable and dry vs. far, wet and dense) is the act 1→2 arc
   on the map. Acts 1–2.

6. **Fixed geography from co-orbitals.** Ceres–Thisbe (+129°), Ceres–Pallas (−53°),
   Hygiea–Themis (+130°) never change. Use them: Thisbe is Ceres's permanent
   "downstream neighbour" (cheap orbit, awkward phase), Pallas its permanent unreachable
   mountain (12.8+ km/s). Players can learn these like coastlines. Acts 2–3.

7. **Eras, not seasons, inside the belt: conjunction diplomacy.** Ceres–Vesta is cheap
   (< 4 km/s) for ~5 years in every 17. Confederation logistics in act 3 should be
   driven by these openings: an alliance, migration or war between two hub polities is
   feasible only in its window, and the AI factions should plan around the same
   porkchops the player sees. Evidence: 17.2-yr synodic, 2.7 → 11.7 km/s swing. Act 3.

8. **Start in the low-inclination inner/middle belt, not at an NEA; use NEAs as
   prologue and as act-3 stepping stones.** Candidates: Fortuna (hydrated, i 1.6°),
   Massalia (S, i 0.7°), Vesta (big, dry, famous). Evidence: no NHATS target under 6
   km/s is larger than 250 m; cheap NEAs have 7–30-year synodic periods; inner belt has
   72k low-i bodies and windows every 15 months. Act 1 (and a decision in 4).

9. **Volatile leverage has a location.** Water is concentrated at Ceres (ice, ~25%),
   in the Ch/Cgh belt at 2.5–3.3 AU (bound, 5–13%), and as ice beyond ~3.3 AU
   (Themis, Cybeles). Nitrogen/carbon (ammoniated clays, carbonates, organics) follow the
   same gradient with Ceres again dominant. Act 3's thesis (leverage is physical) is
   literally that whoever holds Ceres and the outer families holds the propellant and
   the life support of the inner system. Act 3.

10. **Communications as latency + bandwidth, not presence.** Model every message as
    arriving next turn (or two turns near conjunction), with a monthly bandwidth budget
    that shrinks with distance and with the loss of the sponsor's big dish. Cultural
    drift in act 2 can be driven partly by that budget (how much of Earth's culture you
    can still pull). Evidence: 13–46 min lag, kbps–Mbps links, 2–3-week blackouts. Act 2.

11. **Energy seasons on eccentric bodies.** Settlements on Pallas-, Juno- or
    Bamberga-class orbits see 2.5–4× flux swings over 4–5-year orbits; Ceres ±16%.
    Model power as a function of heliocentric distance each turn; eccentric homes have
    winters. Acts 1–2.

12. **The map is data-driven and testable.** Node file = SBDB JSON (elements + physics);
    propagator = Kepler; edges = Lambert/Edelbaum/phasing functions; all pure and
    deterministic, so the headless core can be validated against Horizons vectors in a
    unit test (the check in 1.5 is the template). Acts: all.

## 4. Open questions

- **Multi-revolution and gravity-assist transfers.** The porkchops here are zero-rev
  Lambert; multi-rev and Mars/Jupiter assists can cut hub–hub costs and change the
  window shape (Dawn's Mars flyby was worth 2.6 km/s). Decision: does the game model
  assists at all? If yes, Mars becomes a node with its own 2.1-yr synodic clock.
- **How many nodes, and are neighbourhoods real or procedural?** Real family lists are
  available (Nesvorný), but tens of thousands of nodes are UI-hostile. A hybrid (real
  hubs + real named neighbours down to ~20 km + procedural crumbs from the size
  distribution) needs a design decision.
- **Ship acceleration assumptions.** Whether the settlement's tugs run at 0.05 or 0.5
  mm/s² changes low-thrust hop times by 10×; this is a tech-tree decision that sets the
  whole tempo of the belt map.
- **Precision drift.** Two-body elements are fine for decades at game fidelity, but bodies
  near resonances (Hildas, anything near the 3:1) drift more. Decide whether to
  re-seed from Horizons at fixed epochs or accept drift.
- **NEA fraction of act 1.** If the design still wants an NEA opening, the honest
  version is a "prologue at the sponsor's NEA depot" with no map play; that needs a
  narrative decision rather than more research.
- **Bandwidth as a resource** is well-motivated but easy to over-design; needs a
  decision on whether comms appear at all in act 1.
- **Where exactly the water is at small sizes** (5–20 km) inside the inner belt is
  poorly constrained; DeMeo & Carry show C-types are ~half the inner-belt mass at those
  sizes, but hydration state is rarely measured. The sim will have to assign it
  statistically by class.

## 5. Sources

- JPL SBDB API, https://ssd-api.jpl.nasa.gov/doc/sbdb.html and
  https://ssd-api.jpl.nasa.gov/doc/sbdb_query.html — the node data (elements, H, D,
  albedo, class, GM); all counts in 1.1/1.3 were queried live 2026-09-06.
- JPL Horizons API, https://ssd-api.jpl.nasa.gov/doc/horizons.html — integrated
  ephemerides; used to validate the Kepler propagator.
- NHATS, https://cneos.jpl.nasa.gov/nhats/ and https://ssd-api.jpl.nasa.gov/doc/nhats.html
  — round-trip NEA accessibility tables; queried live for the counts in 1.3.
- Taylor, Elvis et al. 2018, "A Delta-V map of the known Main Belt Asteroids", Acta
  Astronautica 146, 73 (PDF: https://planet4589.org/jcm/pubs/sci/papers/2018/Taylor18.pdf)
  — LEO→MBA delta-v distribution, lowest-Δv MBA table with synodic periods and sizes.
- Ieva et al. 2014, "Low delta-V near-Earth asteroids: a survey of suitable targets",
  A&A 569, A59, https://www.aanda.org/articles/aa/full_html/2014/09/aa22283-13/aa22283-13.html
  — Shoemaker–Helin Δv for Moon/Mars, low-Δv NEA statistics.
- Elvis, McDowell, Hoffman, Binzel 2011, "Ultra-Low Delta-v Objects and the Human
  Exploration of Asteroids", https://arxiv.org/abs/1105.4152 — ULDV definition and counts.
- Elvis 2014, "How many ore-bearing asteroids?", https://arxiv.org/abs/1312.4450 —
  ore-bearing NEA fractions (1/1,100 water, 1/2,000 PGM).
- DeMeo & Carry 2014, "Solar System evolution from compositional mapping of the asteroid
  belt", Nature 505, 629, https://arxiv.org/abs/1408.2787 — mass-weighted composition by
  zone and size; "every type in every zone".
- Bottke et al. 2005 (via Morbidelli et al., "The fossilized size distribution of the
  main asteroid belt", https://lagrange.oca.eu/images/LAGRANGE/pages_perso/morby/papers/fossileSFD.pdf)
  — cumulative counts by size, 100-km elbow.
- Lucy mission, "The Density of the Asteroid Belt", https://lucy.swri.edu/MainBeltDensity.html
  — belt mass, volume, counts (note its spacing figure is inconsistent with its own
  volume; see 1.1).
- Wikipedia, "Asteroid belt", "Kirkwood gap", "16 Psyche", "Near-Earth object",
  "Dawn (spacecraft)" — mass shares, gap positions, Psyche mass/density, NEA census and
  population estimates (Harris & Chodas 2021), Dawn timeline and propulsion figures.
- Rayman et al., NASA NTRS 20170008206, "In-Flight Operation of the Dawn Ion Propulsion
  System Through Survey Science Orbit at Ceres" — 10.8 km/s delivered, Mars assist 2.6
  km/s, leg dates.
- Nesvorný HCM Asteroid Families (PDS SBN), https://sbn.psi.edu/pds/resource/nesvornyfam.html
  — family membership lists and counts.
- NEOWISE Diameters and Albedos V2.0 (PDS SBN), https://sbn.psi.edu/pds/resource/doi/neowise_2.0.html
  — diameters/albedos; the source of SBDB's diameter field.
- MPC, MPCORB format, https://minorplanetcenter.net/iau/info/MPOrbitFormat.html — bulk
  catalogue layout.
- Marsset et al. 2022, MITHNEOS debiased NEO composition, https://arxiv.org/abs/2202.13796
  — NEOs compositionally match their main-belt source regions.
- Takir & Emery 2012, Icarus 219, 641, "Outer Main Belt asteroids: identification and
  distribution of four 3-µm spectral groups" (cited from memory; ADS blocked the fetch) —
  hydrated "sharp" group at 2.5–3.3 AU, ice-like "rounded" groups beyond ~3.3 AU.
- Campins et al. 2010 and Rivkin & Emery 2010 (Nature 464, 1320/1322) — water ice on
  Themis; Licandro et al. 2011 — ice on Cybele; Hsieh & Jewitt 2006 — main-belt comets.
- Zappalà et al. 1990, AJ 100, 2030 — the Δv proximity metric used for the
  neighbourhood counts.
- Computation for this report: `porkchop.py` (universal-variable Lambert after Curtis,
  *Orbital Mechanics for Engineering Students*, Alg. 5.2; Edelbaum low-thrust formula;
  SBDB element ingestion) in the session scratchpad; results in `porkchop_results.json`.
