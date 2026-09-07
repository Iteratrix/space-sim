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
