# space-sim

A realistic asteroid-belt settlement simulator in the King of Dragon Pass mould:
turn-based, a real simulation underneath, an advisor ring on top.

The premise: settled-society assumptions break in the belt. The better analogies
are nomadic and maritime cultures. Three acts — a sponsored harvesting outpost,
the slow loss of the sponsor and the drift into something Voidborn, and the
confederation of the belt when the inner system comes back.

## Layout

| path | what |
|---|---|
| `NOTES.md` | running design decisions, in the order they were made |
| `docs/design/canon.md` | the settled design reference the code and content point at |
| `docs/design/storylet-format.md` | the content format |
| `docs/design/agent-protocol.md` | how to drive a game over JSON, and the batch tools |
| `docs/PROGRESS.md` | state of the build and what comes next |
| `docs/design/voidborn-*.md` | the two creative pitches |
| `docs/research/` | thirteen research reports (physics, economics, industry, history, games) |
| `crates/orbit` | Kepler propagation, Lambert transfers, the body catalogue; validated against JPL Horizons |
| `crates/sim` | the headless deterministic simulation: persons and ties, sponsor, storylets, ring, chronicle |
| `crates/cli` | `space-sim`: play interactively, drive over JSON, or run Monte-Carlo batches |
| `crates/web` | wasm-bindgen bridge: the game in the browser, state in the wasm |
| `web/` | the page: clock rail, the hand, scene, ring, chronicle; deploys to GitHub Pages on a version tag |
| `data/` | bodies, parameters, storylets |

## Running

```sh
cargo build --release
./target/release/space-sim play --seed 7            # interactive text
./target/release/space-sim play --json --seed 7     # one JSON object per prompt; choices on stdin
./target/release/space-sim run --seed 7 --trace     # headless, random policy, status every 6 counts
./target/release/space-sim montecarlo --games 200 --policy ring
./target/release/space-sim validate                 # parse every storylet
./target/release/space-sim calendar --counts 200    # the Earth window table
cargo test --release                                # Horizons validation, determinism, content checks

wasm-pack build crates/web --target web --out-dir ../../web/pkg
python3 -m http.server -d web 8765                  # then open http://localhost:8765/
node web/build.mjs                                  # deploy bundle in web/out (CI does this)
```

Every game starts with the tutorial: the first crewed hull arriving at a robot-built
site. `--scenario act1` on the CLI starts from an established outpost instead.

Everything in the simulation is headless, deterministic per seed, and meant to be
driven and tested by an AI agent; the UI is a thin layer on top.
