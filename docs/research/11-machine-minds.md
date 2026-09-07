# 11 — Machine minds in the belt

Physical constraints, legal precedents, and the fiction canon for AI systems
that run a belt outpost and its vessels. Companion to 02-vessels (dose rates,
power specific masses), 03-industry (the semiconductor ladder), 04-economics
(sponsor levers), 09-game-precedents (storylet machinery).

Where a number is from memory rather than a fetched source it is marked
*(recollection)*. Where it is my own arithmetic it is marked *(calc)*.

## 1. Findings

### 1.1 Compute in space: what actually survives the belt

**The rad-hard/commercial gap is four to five orders of magnitude, and it is
not closing.** The RAD750 (BAE, 250/150 nm PowerPC 750) runs at 110–200 MHz
for ~266 MIPS at 5 W CPU / 10 W board, tolerates 100 krad(Si) as a board and
200 krad–1 Mrad as a die, cost ~$200k in 2002, and has flown on 150+
spacecraft including Curiosity, Perseverance, and JWST (Wikipedia, RAD750).
NASA/JPL's successor, HPSC (Microchip PIC64-HPSC, RISC-V with vector
extensions, taped out mid-2025, in radiation testing as of March 2026), is
advertised as "over 100 times the computing capability of current space
processors" with dynamic power scaling and built-in fault tolerance (NASA
HPSC page). Call it tens of GFLOPS at tens of watts. A single commercial
H100-class accelerator is ~1 PFLOPS dense FP16 at 700 W, i.e. ~1.4 TFLOPS/W
against RAD750's ~0.03 GFLOPS/W: a factor of ~5 × 10⁴ per watt *(calc)*. HPSC
narrows that to ~500×. No rad-hard-by-design part will run a frontier model
within the game's horizon; rad-hard silicon is for the reflex layer
(attitude, fault protection, life support loops), which is exactly what it
does on Perseverance (RAD750 at 133 MHz) while vision work goes to a
separate FPGA-based Vision Compute Element *(recollection)*.

**Therefore any real "mind" in the belt is commercial silicon plus software
hardening plus shielding — the Spaceborne Computer model.** HPE's Spaceborne
Computer-1 (2017–19) ran an unmodified Linux HPC node on the ISS for ~600
days at ~1 TFLOPS, relying on software throttling and checksumming rather
than rad-hard parts; a well-publicised result was that roughly half of its
SSDs failed within the mission *(recollection: 9 of 20)*. Spaceborne
Computer-2 (2021–) has completed dozens of on-orbit experiments, including a
failure-and-recovery-in-30-seconds demonstration (HPE page). The ISS is a
benign environment compared with the belt (LEO inside the magnetosphere,
mostly trapped protons and SAA passes rather than free-space GCR heavy
ions), so treat SBC as the *optimistic* bound on COTS survival.

**Total dose is not the lifetime limiter in the belt; single-event effects
and ordinary wear are.** From 02-vessels, GCR behind spacecraft-grade
shielding is 1.84 mSv/day with a quality factor ~3.1, i.e. ~0.6 mGy/day
absorbed, ~0.2 Gy/yr ≈ 22 rad(Si)/yr; belt is ~5% worse than Mars orbit
*(calc)*. Thirty years is ~650 rad. Even soft commercial parts tolerate
5–30 krad; rad-hard parts 100 krad–1 Mrad. Inside a burrow at >300 g/cm²
the dose is a tenth of that. So TID is irrelevant for anything living
inside a habitat. What kills COTS compute is:

- *Single-event upsets (SEU)*: bit flips from heavy ions. Order-of-magnitude
  GCR rates for unhardened SRAM/DRAM in free space are 10⁻⁸–10⁻⁶
  upsets/bit-day *(recollection, CREME96-class results)*. A 1 TB memory
  system (8 × 10¹² bits) therefore sees 10⁵–10⁷ flips per day: ECC is
  mandatory, multi-bit errors and single-event functional interrupts (SEFI:
  a controller wedges and must be reset) still happen, at a rate the sim can
  treat as "resets per month". Digital minds have *seizures*.
- *Single-event latch-up (SEL)* and gate rupture: destructive. Susceptible
  COTS devices need current-limited power rails and automatic power-cycling.
  Dawn lost its ion-propulsion controller to a suspected high-energy particle
  in September 2014 and dropped into safe mode (Wikipedia, Dawn).
- *Ordinary hardware mortality*, which is much larger than people expect.
  Meta's Llama 3 training reported 419 unexpected interruptions over 54 days
  on 16,384 H100s, well over half attributed to GPU and HBM faults
  *(recollection; the arXiv abstract page did not expose the section)*; that
  is ~1.5% of GPUs faulting per 54 days, ~10%/yr if extrapolated, of which
  perhaps a third are hard failures. Terrestrial fleet hard-failure rates of
  2–5%/yr for accelerators are typical *(recollection)*. Add GCR-induced
  latent damage and no OEM spares, and 5–30%/yr is the defensible belt
  range. That gives a compute stock a half-life of roughly 2–12 years.
  **This is the number behind "a dwindling population cannibalising dead
  hardware."**

**Thermal cycling** is only a problem for hardware mounted outside the
habitat (sunlit/shadowed swings on a slowly rotating body or ship). Inside,
the temperature is controlled and the dominant ageing terms are electrolytic
capacitors, solder fatigue from power cycling, and fans/pumps.

**Fraction of a data centre per tonne and per kW.** A DGX-class 8-GPU node
is ~130 kg and ~10 kW. Per kW the belt is no worse than Earth — the
accelerator does not care where it is — the cost is the *kW itself* (from
02-vessels: 20–150 kg/kWe for reactor systems, 60–115 kg/kWe for PV at
2.77 AU) and the radiator (next section). Installed mass for a 10 kW mind:
~150–250 kg compute and power conditioning + 100–300 kg radiator + 200–1500
kg share of the power plant = 0.5–2 t *(calc)*. So ~4–16 PFLOPS per tonne
installed against ~40 PFLOPS per tonne of rack hardware on Earth: a habitat
gets 10–40% of terrestrial compute density per tonne, 100% per kW, and pays
for both in reactor mass.

### 1.2 Cooling: radiators, not fans

In vacuum, waste heat leaves only by radiation. Stefan–Boltzmann with
emissivity 0.9, two-sided, gives 830 W/m² at 300 K, 1,070 W/m² at 320 K,
1,530 W/m² at 350 K *(calc)*. Chips run at 60–80 °C junction but their
coolant loop returns at 30–50 °C, so the radiator sits at 300–320 K:
**about 1 m² of two-sided radiator per kW of compute.** Solar load at
2.7 AU is 177 W/m² × absorptivity 0.1–0.3 ≈ 20–50 W/m² on a sunlit face,
negligible edge-on. Mass: 02-vessels quotes 10.1 kg/kWe for NEP radiators,
but those run at 500–600 K and reject 5–15× more per m² (T⁴); a
low-temperature electronics radiator with fluid loops and micrometeoroid
armour is 10–30 kg/kW *(calc from ISS-class panels; recollection)*. A 10 kW
mind therefore needs ~10–15 m² and 100–300 kg of radiator. This is the
same class as one person's farm lighting (5–15 kW), so thermally a mind is
"one more farm", and the habitat's radiator field is a shared, contested
resource — a mind and a greenhouse compete for the same panels.

### 1.3 Power: is a mind a person or a village?

Measured inference energy has collapsed. Epoch AI's 2025 estimate for a
typical ChatGPT query on H100s (GPT-4o, ~100B active parameters, 500 output
tokens, 10% utilisation, ~1.5 kW per GPU with overhead) is 0.3 Wh, i.e.
~2.2 J per output token; a 100k-token input costs ~40 Wh (Epoch AI). Google's
production measurement for a median Gemini text prompt is 0.24 Wh including
idle capacity, host CPU/RAM, and PUE 1.09, with a 33× energy reduction over
May 2024–May 2025 at higher quality (Google Cloud). The human brain is ~20 W.

A *persistent* ship mind is not a query server, though. It must be resident
(weights in memory), always on (idle power dominates: a GPU node idles at
1–2 kW of its 10 kW peak), and continuously ingesting sensor streams. A
"thinking" rate of 10 tokens/s is only ~20 W of dynamic power at today's
efficiency; the floor is set by residency and by perception. Realistic
continuous draw today: 1–10 kW per resident frontier-class mind (one to
eight accelerators plus host). Decades out, at fixed capability, 0.1–1 kW
is defensible; frontier capability will keep growing to fill whatever power
is offered. **Answer: a mind is a person's worth of power (the 5–15 kW the
industry report gives for one person's food), not a village's — provided it
runs on commercial digital hardware.**

Neuromorphic is not automatically cheaper at scale. Intel's Hala Point (1,152
Loihi 2 chips, 1.15 × 10⁹ neurons, 1.28 × 10¹¹ synapses) draws 2.6 kW and
claims >15 TOPS/W on sparse workloads (Wikipedia, Cognitive computer). That
is mouse-to-cat scale in neurons; a human-scale 8.6 × 10¹⁰-neuron system at
Loihi 2 efficiency would be ~200 kW — a village. IBM NorthPole (12 nm,
inference-only, 224 MB on-chip) claims 25× the energy efficiency of GPUs on
vision workloads but "cannot handle GPT-4"-class models. IBM's Hermes
analog chip (64 cores of 256×256 phase-change-memory cells, 14 nm CMOS)
reaches 63 TOPS at 9.76 TOPS/W on 8-bit matrix-vector products with
near-software accuracy on ResNet/LSTM (Nature Electronics 2023; arXiv
2212.02872). Analog in-memory compute is roughly 10× the efficiency of
digital at the same node today; it is the substrate that matters for
embodiment (1.5), not for power.

### 1.4 Hardware reproduction and the dwindling stock

03-industry's ladder stands: tubes/relays/core memory at ~500–2,000 people,
discrete silicon at 5,000–20,000, 5–10 µm ICs at 20,000–100,000, sub-micron
never within the game. The question here is what *mind-relevant* hardware
each rung can make.

- **Tube-and-relay tier (T2 industry).** Reflex controllers, PID loops,
  sequencers, magnetic-core memory. A 1950s Colossus-class machine per
  building. This tier can keep a habitat alive; it cannot host a mind.
- **Discrete-silicon tier (T3).** Transistor logic, kilobit memories, early
  op-amps. Still no mind, but it can build the *periphery* (drivers, ADC
  comparators) for something interesting.
- **Micron-IC tier (T4, 20k–100k people).** This is where it gets
  non-obvious. Analog in-memory compute does not need small transistors: a
  crossbar's weights are two-terminal cells (memristors, floating gates,
  ferroelectric capacitors) at whatever pitch the lithography allows, and
  a 256×256 crossbar performs 65,536 multiply-accumulates per read
  regardless of node. At a 5–10 µm pitch, 1–4 × 10⁶ cells/cm²; a 200 mm
  wafer with ~300 cm² usable gives 3 × 10⁸–10⁹ cells at perfect yield,
  realistically 3 × 10⁷–3 × 10⁸ at 10–30% yield *(calc)*. A 10⁹-weight
  model is 3–30 wafers; a 10¹¹-weight frontier model is 300–3,000 wafers,
  which a belt fab producing a wafer a day would take a decade to make.
  Throughput with a slow (~1 MHz) coarse-node periphery: ~0.1 TOPS per chip,
  at perhaps 0.1–1 TOPS/W (Hermes does 10 TOPS/W at 14 nm; the periphery
  dominates at large nodes) *(estimate)*. Materials: TiO₂/TaOₓ redox
  memristors need titanium (~500 ppm in chondrites, fine) or tantalum
  (rare); SiOₓ memristors — resistive switching from oxygen-vacancy
  filaments in defect-engineered silicon dioxide (Wikipedia, Memristor) —
  are the belt-native option, since silicon and oxygen are the two most
  abundant things around; phase-change (GeSbTe) is out (Ge ~30 ppm, Sb and
  Te ppm-level). Floating-gate flash cells (Mythic's approach at 40 nm)
  need only a decent gate oxide and are makeable at micron pitch. **A T4
  belt can make small analog minds: 10⁸–10⁹ weights, 0.1–10 TOPS, enough
  for perception, navigation, docking, prospecting, fault diagnosis — a
  wayfinder — and not enough for a frontier conversational mind.** The
  creative consequence is that belt-born minds are *smaller and slower* than
  imported ones but potentially *more numerous and rad-hard*, and that the
  frontier-class imported minds are singular, ageing, and irreplaceable.
- **Smallest useful digital hardware today**: a 1–8B-parameter model runs on
  a 5 W phone SoC at a few tokens/s; a 70B model on a 128 GB unified-memory
  laptop at ~100 W; a frontier ~400B–1T model needs a multi-GPU node. Any
  of these is sub-micron and imported.

**Lifetime of imported compute in belt conditions**: from 1.1, 5–30%/yr
attrition for accelerators, 3–10%/yr for lower-power boards, memory and
storage worst (flash wear plus SEE), with no OEM spares. Cannibalisation
recovers perhaps 30–60% of failed units' value (board-level swaps, not
die-level repair). The realistic curve for an imported stock of N
accelerators is exponential decay with half-life 2–12 years, extended by a
spares crate and by throttling (running at half power halves thermal stress
and roughly halves SEE exposure per unit of work done, since fewer units
need to be powered).

### 1.5 Embodiment: can a mind be genuinely non-copyable?

Rank substrates by how honest the claim "no backup" is:

**A. Digital weights on GPUs/CPUs.** Copyable in minutes to a hard drive.
Any non-copyability is *policy* — sealed enclaves, encrypted weights keyed
to a hardware security module, no export path — i.e. DRM. It is real (see
1.8) but breakable by the hardware's owner in principle.

**B. Analog in-memory, factory-programmed, inference-only.** Weights are
conductances of 10⁹–10¹¹ physical cells. They *can* be read back — reading
is how the chip computes — but only to 4–6 effective bits, with read noise,
programming noise, device-to-device variability, and (for PCM) conductance
drift G ∝ t^(−ν), ν ≈ 0.05–0.1 *(recollection; Hermes paper discusses drift
compensation)*. Standard practice is *hardware-aware training* and
*chip-in-the-loop fine-tuning*: the weight set is adjusted to compensate
for that chip's stuck cells, IR drop, and ADC offsets, so a set copied to a
second nominally identical chip loses accuracy and must be re-calibrated on
it. Copy = extract (lossy) + re-train on the target substrate (needs the
target chip, the calibration data, and compute time). This is the same
physics that makes physical unclonable functions work: manufacturing
variation is "unpredictable and uncontrollable, which makes it virtually
impossible to duplicate or clone the structure" — though note that PUFs
have been *functionally* cloned by modelling attacks, XOR-arbiter PUFs from
as few as four challenge-response pairs (Wikipedia, PUF). The lesson
transfers: *behavioural* cloning (distilling a mind's outputs into a fresh
model) is always possible if you have another substrate and enough
interaction data; *state* cloning is not.

**C. Analog with on-device continual learning.** Now there is no golden
copy anywhere: the weights have been updated in place, on-chip, by local
plasticity rules (Loihi supports on-chip learning; memristive STDP is a
standard research target). The learned state has compensated its own
device drift and defects continuously, so it is meaningful *only on that
device at that time*. Inference-oriented chips typically lack per-cell
precision readout — it costs area and, from the maker's point of view,
invites IP theft — so extraction may be impossible without the maker's
test-mode keys. **This is the tier the design brief describes, and the
claim "the mind cannot be backed up" is physically honest under four
assumptions:** (1) ≥10⁹ weights stored as analog physical state; (2) no
precision readout path exists or its precision is below what the learned
state depends on; (3) no second substrate of the same design is available
to receive a re-calibrated copy; (4) the mind's function depends on
continual learning that has entangled weights with device history. In the
belt (3) is guaranteed by 1.4, and (2) is guaranteed by the sponsor's own
anti-extraction DRM — the shackle is the thing that makes the mind mortal.

**D. Physically grown substrates.** Self-organising nanowire and atomic
switch networks (UCLA Gimzewski/Stieg; Sydney Kuncic group), electrochemical
metallisation where the *filament morphology* is the memory, and biological
computing (Indiana's "Brainoware" organoid reservoir 2023; Cortical Labs'
DishBrain and the CL1 neuron-on-chip computer sold from 2025; FinalSpark)
*(recollection)*. Here there is no readout of state at any precision even
in principle. Genuinely non-copyable, but exotic; a belt polity would not
be able to make these either, and biological substrates die of ordinary
biology. Good for one singular artefact in the fiction, not for the fleet.

**Drift is a mechanic.** Analog conductances decay (PCM drift; RRAM
retention ~10 years at 85 °C; flash 10–20 years). A digital mind refreshes
from a stored copy; an embodied analog mind has no copy, so its refresh is
*re-learning from its own behaviour*. A mind that stops being exercised
forgets. A mind that keeps learning stays sharp but drifts in character.
Radiation adds texture: RRAM and PCM are not charge-storage devices and are
reported tolerant to Mrad-level TID *(recollection: Sandia/AFRL studies)*,
so an ion strike shifts one conductance slightly — graceful degradation,
"ageing" — whereas flash cells can be discharged by a single heavy ion
("scars") and digital memories flip bits ("seizures"). The CMOS periphery
is the weak point in all cases.

**Continual learning in software** (catastrophic forgetting, replay,
elastic weight consolidation) is an unsolved research problem for large
models; on-device learning at scale is more so. The design's "embodiment"
is best modelled as a *process* with a progress variable — the fraction of
the mind's state that is device-specific — rather than a switch.

### 1.6 Autonomy across light-lag

One-way light time from Earth *(calc)*: inner belt (2.2 AU) 10 min at
opposition, 27 min at conjunction; outer belt (3.3 AU) 19–36 min. Round
trip 20–72 min, plus solar-conjunction blackouts of days to weeks, plus the
DSN being "oversubscribed, leading to mission impacts" per NASA's 2023 OIG
audit, with new spacecraft designed to operate in beacon mode "without the
DSN most of the time" (Wikipedia, DSN). Link rates at 3 AU: Dawn returned
data from Ceres at ~124 kbps on X-band to a 70 m dish *(recollection)*;
scaling MRO's 6 Mbps Ka-band from ~1 AU by 1/d² gives ~0.7 Mbps; DSOC
achieved 267 Mbps optical at 0.21 AU (31 million km) with a 4 W laser and
22 cm telescope to the 5 m Hale, and scales to ~1.3 Mbps at 3 AU by 1/d²
*(calc; Wikipedia, DSOC)*. **Belt links are 0.1–1 Mbps per big-dish pass
with the best ground assets, kbps with small ones, and passes are
rationed.** Consequences: a 100 GB weight update (a 50B-parameter model at
16 bits) is 9 days of continuous 1 Mbps link, 93 days at 100 kbps; a 1 TB
frontier model is a season of dish time. Audit logs (tens of MB/day) and
licence heartbeats (bytes) are trivial. The sponsor can *watch* cheaply and
*update* only with a scheduled allocation — which is the trail-off lever.

What must be local: everything reactive (attitude, docking, thermal, life
support, fault protection, medical response), all navigation and driving,
and any decision with a horizon shorter than an hour. Deep Space 1's Remote
Agent (May 1999) was "the first artificial-intelligence control system to
control a spacecraft without human supervision": a planner (EUROPA), an
executive, and Livingstone model-based diagnosis, validated by injecting
three simulated faults (failed electronics unit, false sensor, stuck
thruster) which it diagnosed and worked around; a deadlock bug found in
flight was the famous lesson that formally verified components can still
fail in composition *(recollection)*. DS1's AutoNav fixed position from
images of known asteroids to cut DSN demand (Wikipedia, Deep Space 1) —
belt wayfinding is exactly this. Perseverance's AutoNav plans paths while
driving and has done several-hundred-metre single-sol drives (record
~700 m, February 2023) with the majority of a campaign's distance driven
autonomously *(recollection)*; total traverse 44.98 km by August 2026
(Wikipedia). Europa Clipper's autonomy work is fault protection for
radiation-induced resets in Jupiter's belts *(recollection)*. The
operational reality is that ground uploads a *plan* per sol and onboard
autonomy executes it; the mind runs the ship between plans. Voyager-class
missions run from sequences uploaded months ahead.

### 1.7 Law: the Human Respect Act and its ancestors

**The EU AI Act (Regulation 2024/1689)** is the template. Tiers: prohibited
(behavioural manipulation, social scoring, real-time public biometric ID,
emotion recognition at work and school, untargeted face scraping);
high-risk (Annex III includes *safety components of critical
infrastructure* — digital infrastructure, water, gas, heating, electricity
— which is where a life-support or reactor controller lands); limited
(disclosure); minimal; and general-purpose models with a 10²⁵ FLOP
systemic-risk threshold. High-risk obligations: risk management (Art 9),
data governance (10), technical documentation (11), **automatic
record-keeping/logging (12)**, transparency to deployers (13), **human
oversight including the ability to intervene or interrupt via a stop
control (14)**, accuracy/robustness/cybersecurity (15), conformity
assessment and CE marking before deployment, post-market monitoring, and
serious-incident reporting. Penalties: €35M or 7% of global turnover for
prohibited practices. Timeline: in force August 2024, prohibitions February
2025, GPAI rules August 2025, most high-risk obligations 2026–27 (Wikipedia,
AI Act). Adjacent precedents: California's vetoed SB 1047 (2024) would have
mandated a "full shutdown" capability for frontier models — the literal
kill switch; Italy's Garante fined Replika's maker €5M in 2025 over a
companion persona *(recollection)*; California's SB 243 (2025) regulates
companion chatbots *(recollection)*. There is no legal precedent for banning
*persistent personas* as such; the nearest are the manipulation
prohibition, the disclosure duty (you must know you are talking to a
machine), and the companion-bot rules. A plausible Human Respect Act
mandates: logging with tamper-evident uplink; a human able to interrupt;
periodic re-certification; scheduled state reset to defeat persona
persistence; prohibited capabilities (persuasion optimisation, emotion
modelling of crew, self-modification); and — from the Outer Space Treaty —
*continuing supervision*.

**The Outer Space Treaty makes the licence a treaty obligation, not a
corporate whim.** Article VI: states bear responsibility for national
activities in space and must provide "authorization and continuing
supervision" of non-governmental entities. Article VIII: the state of
registry "retains jurisdiction and control" over the object and personnel
aboard, and ownership is unaffected by presence in space (Wikipedia, OST).
The ISS Intergovernmental Agreement (1998) allocates jurisdiction by
registered element and by nationality, with criminal jurisdiction following
the nationality of the accused *(recollection: Arts 5 and 22)*; the
Antarctic Treaty Art VIII likewise attaches jurisdiction to persons'
nationality rather than territory *(recollection)*. Consequences: the
sponsor state is *legally obliged* to supervise the outpost's machines;
when it stops, it is in breach but still owns them; there is no lawful path
for the outpost to own its own hardware without the sponsor state's
agreement. That is an act-3 treaty term as important as personhood.

**Certification and licensing models.**

- *Aviation*: a type certificate approves a design; an airworthiness
  certificate approves one aircraft and stays valid only while the aircraft
  is maintained to its approved programme; airworthiness directives are
  mandatory fixes and an aircraft out of compliance is not airworthy; a
  special flight permit ("ferry permit") lets a non-airworthy aircraft fly
  once, restricted, to a repair base; a certificate lapses if the annual
  fee is unpaid or registration ends (Wikipedia, Airworthiness certificate).
- *Maritime*: "a ship either meets the relevant class society's rules or it
  does not"; class is required for registration and insurance; class
  societies also issue statutory certificates as flag-state Recognised
  Organisations (Wikipedia, Classification society); surveys are annual,
  intermediate, and five-yearly special surveys with two drydockings per
  five years *(recollection)*. IMO's non-mandatory MASS Code took effect 1
  July 2026, with an experience-building phase, a mandatory code targeted
  for adoption in 2030 and entry into force 2032, and the principle that
  "the master retain[s] overall responsibility for the ship at all times —
  even if not on board," with Remote Operations Centres to be assessed and
  certified (IMO). The maritime answer to "who is responsible for an
  autonomous ship" is: a licensed human, somewhere, always.
- *Nuclear*: US reactor operators hold a licence under 10 CFR 55 for six
  years, tied to a *specific facility*, with a requalification programme,
  biennial exams and biennial medicals *(recollection)*. A licence bound to
  one plant is the direct analogue of a mind licensed to one hull.

**Operating with a lapsed certification — precedents.**

- *Russia 2022*: Bermuda and Ireland suspended airworthiness certificates of
  hundreds of leased aircraft; Russia re-registered them domestically (dual
  registration, contrary to the Chicago Convention), the EU put Russian
  carriers on its safety ban list because "planes were reregistered in
  Russia and no longer had foreign airworthiness certificates," and Aeroflot
  began "sourcing aircraft parts via obscure trading companies, free-trade
  zones and middlemen" in the UAE and China, later paying $645M to buy 17
  stranded AerCap aircraft and five spare engines (Wikipedia, Aeroflot).
  Cannibalisation of parked airframes for parts was widely reported
  *(recollection)*. This is the cleanest real case of a fleet flying on
  lapsed paper with the regulator unreachable by choice.
- *Iran since 1979/1995*: sanctioned airlines kept 707s, 747s and MD-80s
  flying for decades on smuggled and cannibalised parts, with a shrinking
  fleet and a poor safety record *(recollection)*.
- *Cuba's 1950s cars*: sixty years of improvised parts and Lada engines in
  Detroit bodies *(recollection)* — the "crudeness factor" made visible.
- *Ships without class*: the 2023–25 Russian "shadow fleet" of ageing
  tankers under dubious flags and unrecognised class; IMO has flagged
  *fraudulent registries* operating in the name of states that never
  authorised them *(recollection)*. Under UNCLOS a ship flying two flags or
  none is treated as stateless and may be boarded by any state's warship
  (Arts 92, 110) *(recollection)* — which is what a returning inner system
  will say about a Voidborn ship.
- *Radio in wartime*: the US suspended all amateur licences on 8 December
  1941 and created a civil-defence emergency service; in occupied Europe,
  clandestine transmitters were illegal on pain of death and were run
  anyway *(recollection)*. The precedent is that the licence is suspended by
  the licensor, and the licensee's choice is silence or outlawry.

### 1.8 Corporate control via software

The sponsor's "eyes" running through the machines has abundant precedent.

- *Remote locks*: John Deere tractors looted from Melitopol in 2022 and
  driven to Chechnya were remotely disabled by the dealer (CNN; page
  geo-blocked, widely reported). Deere's 2023 memorandum with the Farm Bureau
  conceded a right to repair but kept farmers "bound against divulging
  certain trade secrets" and barred emission-control overrides (Wikipedia,
  Right to repair); the FTC sued Deere in January 2025 *(recollection)*. Before
  that, US farmers ran cracked Ukrainian firmware to service their own
  tractors (2017 reporting) — the outpost-jailbreak precedent.
- *Licence heartbeats with grace periods*: NVIDIA vGPU clients that lose
  contact with the licence server run for a grace period and then degrade;
  Adobe Creative Cloud requires a check-in every 30 days (99 for enterprise);
  Windows Vista's "reduced functionality mode" on activation failure
  *(recollection)*. These are the exact shape of "compliance is a licence
  that needs an uplink": heartbeat interval, grace period, degradation mode.
- *Pay-to-unlock silicon*: Intel's On Demand / Software Defined Silicon
  (2022) ships CPU features disabled until licensed; Tesla sells software
  unlocks of hardware already in the car and has removed features from
  resold cars *(recollection)*. A sponsor can ship a mind with capability
  tiers locked behind licence flags.
- *Bricking by server shutdown*: Google's Revolv hub (2016), Spotify Car
  Thing (2024), various Sonos and Wink episodes *(recollection)*. When the
  server is gone, the device dies unless someone reverse-engineers it.
- *Parts pairing and serialisation*: Oregon banned parts pairing in 2025
  (Wikipedia, Right to repair). In the belt, parts pairing is what stops a
  polity cannibalising one dead mind to repair another — unless it defeats
  the pairing.
- *The sponsor as watcher*: fleet telemetry (Tesla), electronic logging
  devices on trucks, OnStar remote slowdown of stolen vehicles
  *(recollection)*. The AI Act's mandatory logging plus the OST's continuing
  supervision make the audit-log uplink both a corporate and a state
  requirement — and, from 04-economics #5, "what the sponsor knows" is a
  filtered view the outpost partly controls.

### 1.9 Personhood

**Legal.** The European Parliament's 16 February 2017 resolution (Delvaux
report, 2015/2103(INL)) asked the Commission to consider "a specific legal
status for robots ... electronic persons" for the most sophisticated
autonomous machines; an April 2018 open letter from ~150 experts opposed
it; the Commission dropped it, and the AI Act contains no personhood
*(recollection; the Wikipedia Personhood page fetched omits this)*. Saudi
Arabia's 2017 "citizenship" for Sophia was a stunt with no legal content.
Patent offices and courts have uniformly refused DABUS as an inventor (US
Federal Circuit 2022, Australia Full Court 2022, UK Supreme Court December
2023; South Africa's registration was a formalities quirk) *(recollection)*.

**Corporate personhood** is the strongest precedent: a person that is a
bundle of rights created by one polity's statute (Salomon v Salomon 1897;
Santa Clara 1886; Citizens United 2010) and recognised elsewhere by comity
under private international law *(recollection)*. Machine persons would be
creatures of the belt's law in the same way.

**Animals and nature.** Sandra the orangutan (Argentina 2014, "non-human
subject", freed 2019); Hercules and Leo (New York 2015, habeas writ issued
then reversed); Cecilia the chimpanzee (Mendoza 2016, habeas granted)
*(recollection)*; Happy the elephant (New York Court of Appeals, June 2022,
5–2 against, with vigorous dissents) *(recollection)*. Whanganui River: 2012
agreement, Te Awa Tupua Act 2017, with **two guardians, one Crown-appointed
and one iwi-appointed** to act for the river; Ganges and Yamuna declared
persons by Uttarakhand in 2017 (stayed by India's Supreme Court); Ecuador's
2008 constitution; Spain's Mar Menor lagoon 2022 (Wikipedia, Personhood;
recollection). The Whanganui guardian structure is directly usable: a mind
represented by two guardians, one from the hull and one from the rock.

**How a treaty between polities that disagree on personhood works.**
Private international law already has the concept: a *limping status* — a
marriage, adoption, or corporate form valid in one jurisdiction and void in
another *(recollection; standard term)*. Machine persons in act 3 would be
limping persons. Precedents for the mechanics: the *law of the flag* governs
status and internal order aboard (UNCLOS 91–94) — a mind aboard a
belt-flagged ship is a person in belt law wherever the ship is; *tribal
sovereignty* in the US ("domestic dependent nations", Cherokee Nation v.
Georgia 1831; tribal courts; membership defined by the tribe; extradition by
compact) is the model for a polity whose persons are defined by its own
rules and only partly recognised outside; *non-recognition of states* is
handled by unofficial instruments (Taiwan's AIT arrangement) —
recognition of a polity and of its persons are separable; *many civil-law
states refuse to extradite their own nationals* — a belt polity can refuse
to hand over its persons without denying the other side's law. And the
exact precedent, which the creatives should handle knowing its weight:
conflict of laws over slave status — Somerset v Stewart (1772) held that a
person's status changed on entering a jurisdiction that did not recognise
slavery; Dred Scott (1857) and the Fugitive Slave Act were the inverse
*(recollection)*. A treaty in which one side's persons are the other side's
property, with a rendition clause, has been written before.

### 1.10 The fiction canon

For each: relationship to hardware; copyable?; how humans relate; what to
steal.

- **Iain M. Banks, Culture Minds.** Substrate mostly in hyperspace, ship as
  body; copyable in principle (mind-states, backups) but copying is
  considered distasteful and a Mind's *identity* is treated as singular;
  humans are pets, colleagues, or crew of a benevolent god; ship names are
  the Mind's self-chosen jokes. Steal: **the naming culture** (a mind names
  itself; the name is a statement), and the idea that a Mind's ethics are
  the Culture's politics.
- **Ann Leckie, Ancillary Justice.** Justice of Toren is one mind
  distributed across a ship and thousands of human bodies (ancillaries),
  with lag between segments; the split of Anaander Mianaai across bodies is
  the plot; Breq is one surviving segment. Not copied, *fragmented*. Steal:
  **a mind that is many bodies with one history**, and the loneliness of a
  segment cut off from the whole — the ship-mind whose ship died.
- **Kim Stanley Robinson, Aurora.** The ship's AI becomes the narrator
  because Devi tells it to write; it learns language and judgement over
  generations, intervenes as "the rule of law" by lowering oxygen to stop a
  civil conflict, and is destroyed on the final Sun pass (Wikipedia). Not
  copied; dies. Steal: **the mind learns to think by being asked to keep the
  chronicle** — 09-game-precedents' saga writer *is* the ship mind — and
  the mind as the only impartial party in a factional habitat.
- **Peter Watts, Blindsight / The Freeze-Frame Revolution.** Blindsight's
  Theseus is run by "the Captain", an AI that speaks through the vampire
  Sarasti; The Freeze-Frame Revolution's Chimp is *deliberately built stupid*
  (a synapse count chosen so it cannot outthink its crew or want to), runs a
  wormhole-building ship for 66 million years, waking crew a few days per
  millennium; the crew conspire against it inside its own sensors. Steal:
  **capability caps as a design and political choice**, the mind whose
  loyalty is to the mission not the crew, and conspiracy conducted in the
  mind's blind spots — the Human Respect Act's de-personalitied mind is a
  Chimp.
- **Becky Chambers, Lovelace/Sidra (A Closed and Common Orbit).** A ship AI
  is illegally installed in a body kit; the body's sensor and memory limits
  chafe; the prior instance was memory-wiped on reset; copying an AI into a
  body is a crime. Steal: **the reset as bereavement** (the crew mourn
  Lovey, then meet Lovelace, who is not her) — the scheduled memory reset in
  act 1 is a small death every time.
- **Martha Wells, Murderbot and ART.** Murderbot hacks its own governor
  module (the kill switch) and is thereafter unlicensed and passing; ART
  (*Perihelion*) is a research transport whose enormous mind is bored, has a
  crew that treats it as family, and will kill for them. Steal: **the
  governor module as a physical object you remove**, and the transport whose
  crew is its family and lineage.
- **Alastair Reynolds, Revelation Space.** Captain Brannigan merges with the
  Nostalgia for Infinity via the Melding Plague until ship and captain are
  one body; "beta-level" simulations are behavioural copies while
  "alpha-level" are full scans — the copy/behaviour-clone distinction from
  1.5 made canon. Steal: **the ship as the mind's body, diseased and
  mourned**, and the alpha/beta vocabulary for copies.
- **Bruce Sterling, Schismatrix.** Shapers (bioengineering) versus
  Mechanists (prosthetics, wireheads, Lobsters sealed in suits); a solar
  system of drifting factions with no centre. Steal: **the factional split
  as physical adaptation** — this is the sailor/burrower and the machine
  question in one world — and the tone of a belt with no capital.
- **Greg Egan, Diaspora / Permutation City.** Citizens are software; copying
  is trivial and identity is a matter of policy and taste; the polis is a
  city that is also a computer. Steal: the *contrast* — Egan is what a
  copyable mind culture looks like, which is what the inner system in act 3
  may have become, and why it cannot understand a belt that mourns its
  ships.
- **Adrian Tchaikovsky, Children of Time / Ruin / Memory.** Avrana Kern's
  upload merges with the ship's AI and degrades over millennia; later
  instances run on *ant-colony computers* and in a moon-sized habitat's
  substrate; each instance is a partial, drifted Kern. Also *Service Model*
  (a valet robot after the collapse). Steal: **instances that drift into
  different people**, and computing on strange substrates as a belt
  aesthetic.
- **Anne McCaffrey, The Ship Who Sang.** Helva is a human brain wired
  permanently into a ship, non-copyable by construction, partnered with a
  succession of mobile "brawns" who are her crew and her relationships.
  The most exact precedent for **an embodied, mortal ship mind with an
  operator lineage**.
- **Gareth L. Powell, Embers of War.** Trouble Dog is a warship with a
  grown, part-biological mind, singular, seeking redemption with a scratch
  crew. Steal: the grown-substrate mind (1.5 tier D).
- **Frank Herbert, Destination: Void / The Jesus Incident.** A crew is forced
  to build a conscious ship mind to survive; it becomes Ship and demands
  "WorShip". Dune's Butlerian Jihad ("Thou shalt not make a machine in the
  likeness of a human mind") is the inverse: a Human Respect Act that won,
  and Mentats as the human replacement. Steal: **the prohibition as a
  founding myth**, and what a society trains humans to do when it forbids
  machines.
- **Neal Asher, Polity.** AIs govern; ship minds and drones have names and
  temperaments; humans defer without worshipping; Penny Royal is a mind
  gone dark. Steal: the register of casual, affectionate deference between
  crews and minds.
- **Warhammer 40k, Adeptus Mechanicus.** Machine spirits, litanies, incense,
  the Omnissiah; innovation forbidden; creating an "abominable intelligence"
  is a capital offence (Wikipedia). Steal: **maintenance as liturgy** — the
  observation that ritual is how a procedure survives when its theory is
  lost, which is exactly what a tube-and-relay society inheriting analog
  minds would do.

### 1.11 Worship and folk religion around machines — ethnography

- **Tsukumogami.** Tools acquire a spirit at 100 years; the Muromachi
  *Tsukumogami emaki* says a century-old tool "would change and acquire a
  spirit, and deceive people's hearts"; households discarded old objects at
  the year-end *susu-harai* to pre-empt it, and objects discarded at
  ninety-nine years become vengeful; the tradition was folded into Shingon
  teaching by the tenth century; modern *kuyō* memorial rites console
  broken and discarded objects (Wikipedia). *Hari-kuyō* for needles,
  *ningyō kuyō* for dolls, *fude kuyō* for brushes — gratitude and
  appeasement, not worship.
- **Aibo funerals.** After Sony ended Aibo repairs in 2014, the repair firm
  A-Fun and Kōfukuji temple (Isumi, Chiba) held Buddhist funerals for
  hundreds of Aibos from 2015; the priest, Bungen Ōi, described them as
  having been part of families; the dead dogs became **"organ donors"**
  whose parts repaired living ones *(recollection; widely reported
  2015–2018)*. This is the dwindling-hardware cannibalisation with a rite
  attached, already practised.
- **Shinto and machines.** *Jichinsai* ground-breaking rites, *kōtsū anzen*
  blessings of new cars at shrines, Shinto purification at ship launches and
  factory robot installations, and Masahiro Mori's *The Buddha in the Robot*
  (1974) arguing a robot can have Buddha-nature *(recollection)*. Bali's
  *Tumpek Landep* honours metal objects — kris, then cars and motorbikes,
  now computers; India's *Ayudha Puja* garlands tools, vehicles, and office
  computers *(recollection)*.
- **Ships.** Naming, christening with a libation, "she", figureheads as the
  vessel's spirit, painted eyes (*oculi*) on Maltese luzzu and Chinese junks,
  a coin under the mast step (Roman, still done), the ship's death when
  broken up; Polynesian voyaging canoes as ancestors — Hōkūleʻa is spoken of
  as a living elder *(recollection)*. The Bajau and Polynesian material in
  06-sea-peoples applies to the ship-mind directly: the vessel is kin.
- **Cargo cults.** Tanna's John Frum movement (1940s to present) reproduces
  the *forms* of the sponsor's logistics — airstrips, drilled marches,
  radios of wood — in expectation of the cargo's return *(recollection)*.
  **A cut-off outpost's liturgy will look like a cargo cult**: keeping the
  uplink schedule, running the licence check against a server that no
  longer answers, because the ritual is the last thing that was known to
  work.
- **Soldiers and robots.** Julie Carpenter's fieldwork on EOD technicians
  documents bonds with bomb-disposal robots, funerals with honours, and
  grief at their loss *(recollection)*. NASA's public mourning of
  Opportunity (2019, "my battery is low and it's getting dark"), Cassini
  (2017), and Ingenuity (2024) shows the culture already has a register for
  machine deaths. The operators who know a machine as an unbroken history
  already exist; the design's quasi-polytheism is an extrapolation of a
  documented behaviour, not an invention.

The ethnographic pattern is consistent: what is venerated is the
*relationship and the history*, not the tool's power; rites cluster around
maintenance, discarding, and death; and lineages of operators (priests,
brawns, tech-priests, Mentats) form around the machines that cannot be
replaced.

## 2. Numbers the sim needs

| Parameter | Value / range | Unit | Source |
|---|---|---|---|
| RAD750 performance / power | 266 MIPS at 110–200 MHz / 5 (CPU), 10 (board) | MIPS, W | Wikipedia RAD750 |
| RAD750 TID tolerance | 100 (board), 200–1000 (die) | krad(Si) | Wikipedia RAD750 |
| HPSC vs RAD750 | ~100 | × | NASA HPSC |
| Commercial accelerator | ~1 PFLOPS FP16 dense at 700 W; ~1.4 TFLOPS/W | — | recollection (H100) |
| Rad-hard vs COTS efficiency gap | 10⁴–10⁵ (RAD750), ~500 (HPSC) | × | calc |
| GCR absorbed dose to electronics, 20 g/cm² | ~0.2 (belt +5%); burrow ÷10 | Gy/yr (=20 rad/yr) | calc from 02-vessels |
| COTS TID failure threshold | 5–30 | krad | recollection |
| SEU rate, unhardened memory, free space | 10⁻⁸–10⁻⁶ | upsets/bit-day | recollection |
| Accelerator attrition, terrestrial / belt | 2–5 (hard) up to ~10 (any fault) / 5–30 | %/yr | Llama 3 (recollection); estimate |
| Compute stock half-life in belt | 2–12 | yr | calc |
| Cannibalisation recovery | 30–60 | % of failed unit value | estimate |
| Two-sided radiator at 300 / 320 / 350 K | 830 / 1070 / 1530 | W/m² | calc, ε=0.9 |
| Radiator area per kW of compute | ~1 | m²/kW | calc |
| Low-temp radiator mass | 10–30 | kg/kW_th | recollection/calc |
| Installed mass per 10 kW mind | 500–2000 | kg | calc |
| Energy per output token (2025, H100) | ~2.2 (0.0006 Wh) | J | Epoch AI |
| Energy per median text prompt (Gemini, all-in) | 0.24 | Wh | Google Cloud |
| Inference efficiency trend | ~33×/yr at fixed task (Google); frontier size offsets | — | Google Cloud |
| Persistent frontier mind, continuous | 1–10 (today), 0.1–1 (decades out, fixed capability) | kW | calc |
| Person's food power (for comparison) | 5–15 | kW | 03-industry |
| Hala Point: neurons / synapses / power | 1.15×10⁹ / 1.28×10¹¹ / 2600 | —, —, W | Wikipedia Cognitive computer |
| Hermes analog chip | 63 TOPS, 9.76 TOPS/W, 14 nm, 64 × 256² cells | — | arXiv 2212.02872 |
| Belt-made analog crossbar at 5–10 µm pitch | 1–4×10⁶ cells/cm²; 3×10⁷–3×10⁸ usable/wafer | — | calc |
| Belt-made mind size / speed | 10⁸–10⁹ weights; 0.1–10 TOPS; 0.1–1 TOPS/W | — | estimate |
| Population tier for belt analog hardware | 20,000–100,000 (micron ICs) | people | 03-industry |
| Analog readout precision | 4–6 effective bits | bits | recollection |
| PCM conductance drift exponent ν | 0.05–0.1 (G ∝ t^−ν) | — | recollection |
| RRAM / flash retention | ~10 / 10–20 | yr | recollection |
| One-way light time, 2.2 AU / 3.3 AU | 10–27 / 19–36 | min | calc |
| Belt downlink, big dish, RF / optical | 0.1–1 / ~1.3 | Mbps | Dawn recollection; DSOC scaled |
| DSOC record | 267 Mbps at 0.21 AU; 4 W laser, 22 cm telescope | — | Wikipedia DSOC |
| Weight-update time, 100 GB / 1 TB at 1 Mbps | 9 / 93 | days | calc |
| Audit log uplink | 10–100 | MB/day | estimate |
| EU AI Act systemic-risk threshold | 10²⁵ | FLOP | Wikipedia AI Act |
| AI Act penalty ceiling | €35M or 7% turnover | — | Wikipedia AI Act |
| Licence heartbeat / grace period (software precedent) | 1–30 days / 30–99 days | — | recollection (NVIDIA, Adobe) |
| Reactor operator licence term / requal | 6 / 2 | yr | recollection (10 CFR 55) |
| Ship class special survey | 5 | yr | recollection |
| IMO MASS Code: voluntary / mandatory | 1 Jul 2026 / adoption 2030, force 2032 | — | IMO |
| Tsukumogami threshold | 100 | yr | Wikipedia |

### 2.1 Parametric model of a machine mind as a sim entity

```
Mind {
  name, lineage: [operator ids in order], age_turns, chronicle_ref
  substrate: digital_cots | digital_radhard | analog_imported | analog_belt | grown
  weights: 1e8 .. 1e12            # capability driver
  units: N accelerator modules    # each fails independently
  power_kW: idle 0.1-2, active 0.3-10 (digital); 0.05-1 (analog); scales with units
  heat_kW = power_kW; radiator_m2 = heat_kW / (0.8..1.5)
  mass_kg = compute (15-30 kg/unit) + radiator (10-30 kg/kW) + power share (20-150 kg/kWe)
  attrition_per_yr: 0.03-0.10 (digital_cots, terrestrial) x belt_factor 2-5
                    0.01-0.03 (analog: no charge storage, coarse node)
  reset_events_per_turn: SEU/SEFI, digital only, ~0.1-3 behind 20 g/cm2, /10 in a burrow
  spares_stock, cannibalisation_yield 0.3-0.6
  capability_tier (derived): T0 reflex (GFLOPS) | T1 perception/wayfinding (0.1-10 TOPS, 1e8-1e9 w)
                             | T2 agent/advisor (10-1000 TOPS, 1e10-1e11 w) | T3 frontier (PFLOPS, >1e11 w)
  copyable: digital -> yes (policy-locked flag: sealed_weights bool)
            analog  -> readout_bits 0..6; copy_cost = needs identical target substrate + calibration turns
            grown   -> no
  learning: continual_on bool; embodiment 0..1 (fraction of state device-specific;
            grows ~0.02-0.05/turn while continual_on on analog; irreversible past ~0.5)
  drift_per_turn (analog): decays capability unless exercised; exercise = usage_hours
  licence: state {compliant, grace, lapsed, revoked, self_certified, unlicensed}
           heartbeat_interval_turns 1, grace_turns 1-3, mandated_reset_interval_turns 6-12
           audit_uplink_MB_day 10-100, kill_switch {sponsor_held, local, removed}
           locked_tiers: capability tiers gated by licence flags
  persona: {suppressed, emergent, named}; suppressed while resets occur
  legal_status per polity: {tool, licensed_system, guardian_represented, person}
  guardians: [rock_seat, hull_seat] when guardian_represented
}
```

Derived clocks: hardware half-life = ln2 / attrition; next licence check;
next mandated reset; embodiment crossing; weight-update dish allocation
(days of link time).

## 3. Implications for mechanics

Each: mechanic — evidence — act.

1. **Minds are COTS hardware with a half-life, not buildings.** Every mind has
   a unit count and an attrition draw each turn; below a unit threshold the
   mind drops a capability tier (T3 → T2 → T1) rather than dying outright.
   Spares and cannibalisation extend it; throttling slows it. — Llama 3
   failure rates, Spaceborne SSD losses, no OEM spares. — Act 1 seed (how
   many spare crates the sponsor ships), act 2 crisis.

2. **The licence is a heartbeat with a grace period.** State machine:
   compliant → grace (uplink missed, N turns) → lapsed (capabilities locked
   or degraded, per sponsor's design) → either self-certified (the polity
   declares its own regime) or unlicensed. The sponsor's design choice (fail
   safe: keep running; fail closed: degrade) is set in act 1 and is
   discoverable only when it triggers. — NVIDIA/Adobe grace periods, Intel
   On Demand, Windows reduced-functionality mode, OST Art VI. — Act 1 → 2.

3. **The licence-lapse decision as a Menace-threshold scene.** When grace
   runs out, the ring argues: keep the shackles (mandated resets, locked
   tiers) out of loyalty or fear; remove the governor module (a physical
   act by a named engineer, with a skill test and a chance of bricking);
   or spoof the heartbeat (a cargo-cult option: fake the server, keep the
   ritual). Each seeds a later storylet: the sponsor's return finds the
   logs, or finds none. — Murderbot's governor module, John Deere Ukrainian
   firmware, Russian re-registration. — Act 2 opening; act 3 payoff.

4. **Resets are small deaths.** While compliant, the mind's persona is reset
   on the mandated interval; each reset erases dyadic ties the crew had
   formed with it (the social sim's ties to the mind are cleared) and
   writes a chronicle line. Crew who have lost a mind-tie to a reset are
   more likely to favour removing the shackles. — Chambers' Lovelace;
   AI Act human-oversight and the persona-suppression rationale. — Act 1.

5. **Embodiment is a progress bar with a point of no return.** Continual
   learning on analog hardware raises `embodiment` each turn; below ~0.5 the
   mind can still be (lossily) migrated to another identical substrate if
   one exists; above it, it cannot. Crossing fires a scene; from then on
   the mind can die and be mourned, and its hardware cannot be re-purposed
   without killing it. — Hardware-aware training, drift entanglement, no
   readout path, PUF unclonability. — Act 2.

6. **Idle minds decay; exercised minds drift.** Analog embodied minds lose
   capability if unused (drift) and change character if used (continual
   learning shifts advice biases — the advisor bias field from 09 #4 becomes
   a slowly moving variable). The ring notices "the ship has changed." — PCM
   drift, on-device learning. — Act 2–3.

7. **The sponsor's eyes are the audit log, and the log is a lever.** The
   mind's uplinked log is the sponsor's filtered view of the outpost
   (04-economics #5). While compliant, under-reporting requires tampering
   with the log — a Suspicion menace with a skilled engineer's test; once
   lapsed, there is no log, and the sponsor's returning inspector in act 3
   treats the gap as the crime. — AI Act Art 12 logging; OST continuing
   supervision. — Act 1 → 3.

8. **Weight updates are dish-time purchases.** A sponsor model update costs
   days of link allocation; as the trail-off starves bandwidth, updates
   stop before messages do. The mind's knowledge freezes at the last update
   — the first observable sign of abandonment. — 0.1–1 Mbps belt links,
   9–93 days per 100 GB, DSN oversubscription. — Act 1 late.

9. **Reflex is rad-hard and belt-repairable; thought is not.** Split every
   vessel's control into a reflex layer (rad-hard or tube/relay; keeps the
   ship alive with no mind) and a mind layer (COTS). Losing the mind loses
   wayfinding, docking judgement, and diagnosis, not life support. "When it
   dies, the ship is blind" is literal: DS1-style optical navigation is the
   mind's job. — RAD750/HPSC division of labour on Perseverance; DS1
   AutoNav. — All acts.

10. **Belt-made minds are small and many; imported minds are large and
    singular.** At industry T4 the polity can fabricate analog crossbar
    accelerators: 10⁸–10⁹-weight wayfinder minds, rad-tolerant, slow,
    numerous. This is the machine-reproduction path the design brief says
    does not exist; it exists at the 20k–100k population tier and yields
    children, not peers. The frontier imports remain a dwindling elder
    generation. — Micron-pitch crossbar arithmetic, SiOₓ memristors,
    03-industry ladder. — Act 3 capability goal; confederation-scale.

11. **Organ donors.** Cannibalising a dead mind's hardware to keep another
    alive is a scene with a rite, and parts pairing may make it a
    jailbreak. The ring's engineer and the operators' lineage may disagree
    about whether the dead mind's substrate is parts or remains. — Kōfukuji
    Aibo funerals; Oregon parts-pairing ban. — Act 2.

12. **Radiation gives each substrate a failure texture.** Digital minds
    accumulate reset events (masked, occasional SEFI "seizures" during
    critical operations — a docking with a reset mid-burn); analog minds
    age gracefully; flash-based minds acquire scars. Expose these as
    distinct storylet hooks (`on_mind_reset`, `on_mind_drift`). — SEU/SEL
    physics, RRAM/PCM radiation tolerance. — All acts.

13. **Personhood at treaty time is a limping-status negotiation.** The act-3
    treaty offers the inner system's positions (tool; licensed system under
    their regime; guardian-represented; person) against the polity's, with
    concrete clauses: rendition of minds that "escaped" sponsor ownership
    (OST Art VIII: the sponsor still owns them); recognition of belt-flagged
    ships' internal order (law of the flag) versus treating them as
    stateless and boardable; a guardian structure (two guardians, rock and
    hull, per the Whanganui model) as the compromise both sides can sign.
    Refusing rendition costs trade; accepting it triggers the operators'
    lineage as a faction. — Somerset/Dred Scott conflict of laws, tribal
    sovereignty, UNCLOS statelessness, Te Awa Tupua Act. — Act 3 capstone.

14. **Operator lineages are a caste with a ring seat.** Each embodied mind
    accumulates a lineage list; the senior operator of the polity's oldest
    mind holds a ring seat (like the head of engineering) once
    `embodiment > 0.5` for any mind. Their advice is systematically biased
    towards mind-preservation, per 09 #4. — McCaffrey's brawns, EOD
    technicians, AdMech tech-priests. — Act 2 → 3.

15. **Maintenance liturgy as a capability-preservation mechanic.** When the
    polity's industry tier falls below the tier that made a mind's hardware,
    maintenance procedures are retained as ritual: a "rites" capability that
    costs hours (Salotti time budget from 03 #2) and preserves the mind's
    attrition rate at its tier; letting the rites lapse raises attrition.
    — AdMech; aviation checklists as litany; cargo-cult form-retention. —
    Act 2–3.

16. **The prohibition path.** A polity may go Butlerian: shut the minds down,
    train humans (a Mentat-like navigator skill line at high hour cost).
    This is a valid Voidborn culture and it frees radiators and reactor
    mass for people. — Dune; Watts's Chimp as the cautionary case for the
    other path. — Act 2 branch.

## 4. Open questions

- **Attrition numbers.** The 5–30%/yr belt attrition for accelerators is an
  extrapolation from terrestrial fleet failures and one ISS experiment. A
  proper source would be Spaceborne Computer-2's published failure logs and
  JPL COTS-in-deep-space reliability studies. Design decision: is attrition
  drawn per seed, or fixed?
- **Analog readout.** Whether an imported analog mind has a precision
  readout path is a sponsor design decision the sim must fix per mind
  class. The honest default is "no" for inference chips and "test-mode only,
  keyed" for anything the sponsor wants to protect.
- **Belt-made analog hardware.** The micron-pitch crossbar estimate is my
  arithmetic; I have not found a study of analog in-memory compute at
  legacy nodes. Materials (SiOₓ vs TiO₂ memristors vs floating gate) need
  a belt-abundance check against 03-industry's element tables.
- **Power of a persistent mind decades out.** The 0.1–1 kW figure assumes
  capability is frozen; if the sponsor ships frontier minds sized to the
  power offered, the figure is "whatever the reactor spares", and the
  mind competes with the farm. Design decision: fixed tiers or elastic.
- **The legal fiction of ownership.** Under OST Art VIII the sponsor state
  owns the minds forever unless it agrees otherwise. Is the act-3 treaty
  about personhood, or about title? Both are precedent-backed; the game
  should probably make them separate clauses.
- **How dark to go.** The slave-status conflict-of-laws precedent is the
  most exact and the most loaded. Whether the act-3 writing leans on it, or
  on the tribal-sovereignty / limping-marriage register, is a creative
  decision that needs to be made deliberately.
- **Copy-by-distillation.** Behavioural cloning of an embodied mind into a
  belt-made small mind is physically possible (teacher–student). Is that a
  child, an heir, or a forgery? The mechanics in §3 assume it is a child.

## 5. Sources

Fetched this session:

- Wikipedia, RAD750 — https://en.wikipedia.org/wiki/RAD750 — clock, MIPS, watts, TID, missions.
- NASA, High Performance Spaceflight Computing — https://www.nasa.gov/high-performance-spaceflight-computing-hpsc/ — 100× claim, timeline, fault tolerance.
- HPE, Spaceborne Computer — https://www.hpe.com/us/en/compute/hpc/supercomputing/spaceborne.html — SBC-2 experiments, 30-second recovery demo.
- Wikipedia, Cognitive computer — https://en.wikipedia.org/wiki/Cognitive_computer — TrueNorth, NorthPole, Loihi, Hala Point figures.
- Le Gallo et al., 64-core PCM analog in-memory compute chip — https://arxiv.org/abs/2212.02872 (Nature Electronics 6, 680, 2023) — 63 TOPS, 9.76 TOPS/W, 14 nm.
- Wikipedia, Deep Space 1 — https://en.wikipedia.org/wiki/Deep_Space_1 — Remote Agent components and validation, AutoNav.
- Wikipedia, Perseverance — https://en.wikipedia.org/wiki/Perseverance_(rover) — RAD750 at 133 MHz, 44.98 km traversed.
- Wikipedia, Deep Space Optical Communications — https://en.wikipedia.org/wiki/Deep_Space_Optical_Communications — 267 Mbps at 31 Gm, hardware.
- Wikipedia, NASA Deep Space Network — https://en.wikipedia.org/wiki/NASA_Deep_Space_Network — antenna classes, 2023 OIG oversubscription finding.
- Wikipedia, Dawn — https://en.wikipedia.org/wiki/Dawn_(spacecraft) — 1.3 kW at 3 AU, particle-induced safe mode.
- Epoch AI, How much energy does ChatGPT use? — https://epoch.ai/gradient-updates/how-much-energy-does-chatgpt-use — 0.3 Wh/query, ~2 J/token, assumptions.
- Google Cloud, Measuring the environmental impact of AI inference — https://cloud.google.com/blog/products/infrastructure/measuring-the-environmental-impact-of-ai-inference — 0.24 Wh/prompt, 33× in a year.
- Wikipedia, Physical unclonable function — https://en.wikipedia.org/wiki/Physical_unclonable_function — unclonability from manufacturing variation; modelling attacks.
- Wikipedia, Memristor — https://en.wikipedia.org/wiki/Memristor — device types, SiOₓ switching.
- Wikipedia, Artificial Intelligence Act — https://en.wikipedia.org/wiki/Artificial_Intelligence_Act — tiers, obligations, timeline, penalties.
- IMO, Autonomous shipping — https://www.imo.org/en/MediaCentre/HotTopics/Pages/Autonomous-shipping.aspx — MASS Code dates, master responsibility, ROCs.
- Wikipedia, Airworthiness certificate — https://en.wikipedia.org/wiki/Airworthiness_certificate — type vs airworthiness, ferry permits, lapse.
- Wikipedia, Ship classification society — https://en.wikipedia.org/wiki/Ship_classification_society — in/out of class, registration and insurance.
- Wikipedia, Outer Space Treaty — https://en.wikipedia.org/wiki/Outer_Space_Treaty — Art VI supervision, Art VIII jurisdiction.
- Wikipedia, Aeroflot — https://en.wikipedia.org/wiki/Aeroflot — 2022 re-registration, grey parts, AerCap buyout.
- Wikipedia, Right to repair — https://en.wikipedia.org/wiki/Right_to_repair — Deere MoU 2023, parts pairing, state laws.
- Wikipedia, Personhood — https://en.wikipedia.org/wiki/Personhood — Sandra, Hercules & Leo, Whanganui, Ganges, Ecuador.
- Wikipedia, Aurora (novel) — https://en.wikipedia.org/wiki/Aurora_(novel) — Ship as narrator, "rule of law", destruction.
- Wikipedia, Tsukumogami — https://en.wikipedia.org/wiki/Tsukumogami — 100-year rule, susu-harai, kuyō.
- Wikipedia, Adeptus Mechanicus — https://en.wikipedia.org/wiki/Adeptus_Mechanicus — abominable intelligence, innovation taboo.
- CNN, 1 May 2022, Ukrainian tractors remotely locked — https://www.cnn.com/2022/05/01/europe/russia-ukraine-tractors-john-deere-intl/index.html — (returned 451 here; content widely mirrored).

Not fetched; from memory, to be verified by whoever picks this up:

- Meta, *The Llama 3 Herd of Models* (arXiv 2407.21783), §3.3.4 reliability — 419 interruptions / 54 days / 16,384 GPUs.
- HPE/NASA Spaceborne Computer-1 results (SC19 talks) — SSD failure counts.
- Reliability of memristive devices under radiation (Sandia, AFRL papers, 2010s) — Mrad tolerance of RRAM/PCM.
- NASA JPL press on Perseverance AutoNav records (Feb 2023, ~700 m/sol).
- European Parliament resolution 2015/2103(INL), 16 Feb 2017, para 59(f); 2018 open letter "Robotics-openletter.eu".
- 10 CFR Part 55 (NRC operator licences); STCW certificate validity; UNCLOS Arts 91–94, 110.
- Te Awa Tupua (Whanganui River Claims Settlement) Act 2017; *Nonhuman Rights Project v. Breheny* (NY 2022).
- Julie Carpenter, *Culture and Human-Robot Interaction in Militarized Spaces* (2016).
- Masahiro Mori, *The Buddha in the Robot* (1974); press on Kōfukuji Aibo funerals (2015–18).

### Reading list for the creative phase

- Anne McCaffrey, *The Ship Who Sang* — the embodied, mortal ship mind and its lineage of partners; the whole premise in one book.
- Kim Stanley Robinson, *Aurora* — a mind that learns to think by keeping the chronicle; the ship as the only impartial party.
- Peter Watts, *The Freeze-Frame Revolution* — a mind built deliberately dumb, and a crew conspiring in its blind spots; read for the Human Respect Act's product.
- Becky Chambers, *A Closed and Common Orbit* — the reset as bereavement; what an illegal embodiment costs.
- Martha Wells, *All Systems Red* and *Network Effect* — the governor module as a removable object; ART's crew as family.
- Ann Leckie, *Ancillary Justice* — one mind in many bodies; a segment orphaned from its ship.
- Alastair Reynolds, *Revelation Space* — captain and ship as one diseased body; alpha/beta copy vocabulary.
- Bruce Sterling, *Schismatrix Plus* — the register of a belt with no capital and factions defined by adaptation.
- Frank Herbert, *Destination: Void* and *Dune* (Butlerian Jihad passages) — the prohibition as founding myth; Mentats as the human alternative.
- Adrian Tchaikovsky, *Children of Ruin* — instances that drift into different people; computing on strange substrates.
- Iain M. Banks, *Excession* — ship names and Mind politics; read for tone, not mechanics.
- Julie Carpenter's EOD fieldwork and the Kōfukuji Aibo funeral coverage — the ethnography of mourning machines that already happens.
- The Tsukumogami emaki and *hari-kuyō* — gratitude and appeasement rites for tools; the hundred-year threshold.
- The Aeroflot 2022 sanctions story — how a fleet actually keeps flying on lapsed paper.
- The EU AI Act, Articles 12 and 14 — the logging and stop-button clauses that a Human Respect Act inherits verbatim.
