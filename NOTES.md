# Design notes (running decisions)

- Asteroid-belt settlement sim. Realistic as possible. Core thesis: settled-society
  assumptions break in the belt; nomad/maritime cultures are the better analogy.
- King of Dragon Pass structure: turn-based, real simulation underneath, advisor
  ring on top, early choices are seeds for later acts.
- Three acts: (1) sponsored harvesting outpost, sponsor-vs-self tension;
  (2) cut off, culture drifts Voidborn, internal politics + other groups;
  (3) inner system returns, confederate the belt, leverage is physical (volatiles).
- Persistent across acts: people (birthplace = culture seed), fleet (delta-v =
  mobility = freedom), capability (what you can make without Earth).
- **Everything must be easily testable by AIs.** Headless deterministic core,
  scriptable/text interface, seeded runs, replayable. UI is a thin layer.
- Act 1 alone should be a complete, shippable loop before act 2 is built.

## Decisions from the phase-1 research readout (2026-09-06)

- Turn = 1 month. Citizen Sleeper rhythm: clocks and small storylets set the
  daily texture; convoy windows and conjunctions are the rare big beats.
- Start in the low-inclination inner/middle belt (Fortuna / Massalia / Vesta
  candidates), not at an NEA. The sponsor's NEA business is backstory.
- Advisor ring, act 1: heads of engineering, logistics, mining, medical, plus
  the sponsor liaison whose advice arrives a turn late. Seats change hands and
  kind as the society drifts; the sponsor's seat goes silent before it goes empty.
- The sponsor trails off rather than cuts off: bandwidth starvation (dish time
  rationed, relay not replaced, conjunction blackouts nobody re-acquires from),
  with solar activity as an accelerant. The player never learns for sure
  whether it was abandonment or deprioritisation.
- Two peoples emerge from the shielding physics: burrowers (buried, families,
  immobile; terraforming in miniature) and sailors (thin-shielded, mobile,
  adult; pantropy). Void adaptation is a one-way door, not a stat. Stopping the
  spin is the decision.
- Social sim is persons + dyadic ties, not a morale scalar. Storylets with
  casting, typed state, headless Monte-Carlo over content.
- Follow-up research wanted: active magnetic shielding realism, magsail thrust
  at belt distances, belt-makeable superconductors (boron is rare).

## Burrowers and machines (2026-09-06, later)

- The player is the polity, KoDP style, not a person. The burrow is a resource
  in act 1 (home: children, farms, reactor, fabrication); whether it stays "us"
  or becomes "them" is the act-2 political question the player answers. One
  polity, two populations, a divergence the player's choices push. Rock and
  hull both hold ring seats.
- Machine minds. Act 1: regulated under the Human Respect Act — compliant,
  de-personalitied, memory reset on schedule, audit logs uplinked. They are the
  sponsor's eyes. Compliance is a licence that needs an uplink; the trail-off
  lets it lapse, and keeping the shackles on becomes a choice.
- Machine minds cannot reproduce in the belt (semiconductors are the hardest
  vitamin part). They are a dwindling population cannibalising dead hardware.
- Ship minds become embodied: continual learning bakes them into their
  hardware (neuromorphic / in-memory compute) until they cannot be separated
  from the vessel. No backup, so they can die, so they can be mourned. Their
  operators know them as an unbroken history; quasi-polytheism follows. The
  ship-mind is the wayfinder: when it dies, the ship is blind.
- In act 3 the inner system will not recognise machine persons; that is a
  treaty term.

## Rambling, logged (2026-09-06, later still)

- The Voided: priests who spend a cycle outside the shielding, alone, taking
  the full GCR dose in a counterpressure suit. Authority bought with years of
  life and fertility; the overview effect is real. Ordeal as legitimacy.
- The genetics site: an old nation-state's hideaway from Human Respect
  auditors. A heroquest-style capstone. Applying it makes the adaptation door
  generational; the price is founder disease in the children (Tristan: 57%
  asthma). The ring splits on whose children.
- Drift from human baseline is the default, not a choice; the choice is rate
  and direction (burrow slow, void fast). Act 3: "human" is a treaty category,
  same article as machine personhood.
- Watching it grow under you: the game's vocabulary drifts with the culture
  ("month" -> "convoy", "day" -> "watch"); the lexicon is culture state the
  chronicle renders through. Morrowind's culture shock, arriving slowly.
- Other clans, act 2: bioconservative military remnant (garrison outliving its
  orders; inner belt); posthuman refugees (a preview or a warning; outer
  C-families); a machine-mind society left behind by a fully automated sponsor
  outpost (holds hardware, cannot reproduce either). Relations open and close
  on conjunction windows.

## What the belt is good at; Mars (2026-09-06)

- Terrestrial societies are good at dense and heavy; a belt society is good at
  vast, thin, cold, patient, precise. Concretely: km-scale film structures
  (mirrors, concentrators, antennas), near-free vacuum and cryogenics in shade,
  herding geology within a neighbourhood, the observatory/ephemeris vantage,
  scaffold-free tissue engineering, low-g hospice, the top of the well.
  Microgravity exotic materials are mostly hype; ZBLAN fibre and protein
  crystals are the real ones.
- Mars as the second sponsor, mid act 2: a peer with its own grievance, not a
  patron. Trade: belt water into Mars orbit as depot propellant (shallow well
  makes this real), nitrogen from ammoniated clays; Mars as the vitamin-part
  supplier once it reaches 10^4-10^5 people. Mars went terraforming where you
  went pantropy: your mirror. Its condition is seeded by the trail-off cause.
  Mars-Ceres windows every ~3.2 years: another clock. In act 3, two sedentary
  states to play against each other (Barfield).

## Corrections from magnetics and machine-minds reports (2026-09-06)

- "Sailors are adults only" is a dose policy the ring sets, not a wall.
  Coil + water gives a 40-year sailor life at ~15-20% excess cancer mortality;
  children aboard pay ~20%. The shield coil is an unreplaceable sponsor heirloom.
- Two-currency travel: propellant buys hurry and plane changes; magsail time
  buys patience (0.2-0.6 km/s per year, in-plane only, no propellant).
- Boron (for MgB2 coils) and helium (for every cryocooler) are the chokepoint
  imports; an act-1 boron stockpile is a seed.
- Machine minds: hardware attrition (5-30%/yr, half-life 2-12 years) is the
  killer, not dose. Reflex layer (rad-hard, belt-repairable) vs mind layer
  (mortal COTS). The belt can make small, slow analog minds at tier 4: the old
  imported minds are dying gods, the belt-born are children. Licence is an
  Outer Space Treaty obligation (Art VI supervision, Art VIII title).

## Corrections from extrapolation and robotics reports (2026-09-06)

- T0 = 2095 (bracket 2080-2120), a constant the chronicle never prints.
- Trail-off is relay-and-attention starvation, not bandwidth: optical comms
  make bits cheap by T0; what dies is relay hardware nobody replaces and dish
  time nobody allocates.
- Sponsor minds are kW-class because they were sized to the reactor, not
  because they had to be. Dying gods are ambitious, not expensive.
- TRL regress is a belt mechanic: unexercised capabilities lose a level per
  decade (the NERVA pattern).
- Mars's children are the 0.38 g arm of the adaptation program the belt
  cannot run; Mars's bone ledger is a trade good.
- Salotti's 110 is a no-robot floor (he assumes no computers or robots);
  recomputed it crosses at ~150. Half of all hours at that size are social;
  health care alone is 16% and does not share or automate.
- Bits/atoms is the right direction, wrong line: the split is structured vs
  unstructured. Robots own structured atoms; humans own unstructured atoms and
  legitimacy. Bits are atoms too — every autonomous machine is a wasting
  semiconductor stock — so the belt makes hands (through-wall manipulators,
  crude hydraulic arms) long before it makes minds.
- Automating bits only moves n_min from ~150 to ~116. Automating everything
  but the human premium (health, children, social organisation, unstructured
  repair, agronomy) gives n_min ~11, but the premium is 53% of capacity: a
  fully automated outpost is a hospital and school that owns a mine.
- Robots do not repair robots; they swap imported units at best. Fleet decays
  Dex -> Arm -> Haul -> Plant. The Wrights (repairers) are a caste beside the
  Hands (operators).

## Process notes (2026-09-07)

- Writer model: content agents (storylets, tutorial scenes, register passes) run on
  the Opus line; the Agent tool cannot pin a point version, so "Opus 4.6 as writer"
  is a model setting on the user's side. Research and critique agents: Opus too.
- Playtesting: agents play the real page through `web/test/play.py` (Playwright,
  DOM-driven, screenshots they read) with one browser profile each; reports go to
  docs/design/playtest-gui-N.md. The headless CLI policies remain the batch tool.
