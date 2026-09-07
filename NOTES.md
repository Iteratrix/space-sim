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
