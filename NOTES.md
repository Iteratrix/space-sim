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
