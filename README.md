# space-sim

A realistic asteroid-belt settlement simulator in the King of Dragon Pass mould:
turn-based, a real simulation underneath, an advisor ring on top.

The premise: settled-society assumptions break in the belt. The better analogies
are nomadic and maritime cultures. Three acts — a sponsored harvesting outpost,
the slow loss of the sponsor and the drift into something Voidborn, and the
confederation of the belt when the inner system comes back.

Design decisions so far live in [NOTES.md](NOTES.md). Research reports from the
first phase (physics, economics, industry, and the historical analogies) live in
[docs/research/](docs/research/).

Everything in the simulation is meant to be headless, deterministic, and
testable by an AI agent; the UI is a thin layer on top.
