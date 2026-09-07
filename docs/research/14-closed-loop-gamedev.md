# 14 — Closed-loop game development with LLM agents

*What the literature and the practitioner write-ups say about agents that write game
content, play the game, critique it, and feed findings back — with a human steering.*

Short answer: **nobody has published the loop we are running.** Pieces of it are solved
well, one piece (Play2Code, 2026) is a genuine two-agent write→play→fix loop, and one
piece (Six Ages' bulk harness, 2016) is the closest ancestor of our `montecarlo`. But the
specific combination — agents authoring *typed narrative content* against a schema for an
existing deterministic simulation, brute-forced in Monte-Carlo batches, then played
through the real UI by a separate agent that writes a diary — has no published prior art
that I could find. What follows is the nearest work and what it says we are getting right
and what we are missing.

---

## 1. Findings

### 1.1 LLM playtesting: the field is real, the numbers are humbling

**The synthetic/human-authored gap is the single most important result for us.**
Microsoft's TALES suite (Text Adventure Learning Environment Suite) runs the same agents
across TextWorld, TextWorld Express, ALFWorld, ScienceWorld and Jericho. Best scores:
TextWorld 100.0% (o3), TextWorld Express 95.8% (claude-3.7-sonnet), ALFWorld 88.3%,
ScienceWorld 93.1% — and **Jericho 16.1%**, with the headline that no model clears 12%
aggregate on human-written interactive fiction. TextQuests (Infocom suite, tool-free)
reports the same shape. The named causes are long-range causal dependencies where a single
early mistake breaks the run hundreds of turns later, and uninformative failure feedback
("Nothing happened").

The relevant read for us: **our game is structurally a TextWorld, not a Jericho.** The
`play --json` protocol gives an enumerated option set with ids, a status line, and typed
events. There is no parser, no combinatorial verb space, no hidden affordances. That is
why our agents can play it at all, and it means we should not congratulate ourselves on
agent completion rates — we should ask whether the agents are noticing anything, not
whether they finish.

**Industrial LLM QA works and is deployed.** TITAN (arXiv 2509.22170, Sept 2025) is an
LLM agent framework for MMORPG testing with four parts: state perception/abstraction,
action prioritisation, long-horizon reasoning via *action-trace memory plus reflective
self-correction*, and **LLM-based oracles** that emit diagnostic bug reports. Numbers: 95%
task completion, 74% coverage, 82% of seeded bugs found, four previously unknown bugs, and
deployment in eight real QA pipelines. SMART (arXiv 2512.12706) is more interesting
methodologically: it parses the **AST diff between two builds**, has an LLM turn the diff
into an ordered sequence of natural-language subgoals, and uses those subgoals as a
semantic reward alongside a structural reward for covering the changed code. Result:
Overcooked 93%/98% line/branch coverage vs PPO's 55%/58%; Minecraft 92%/94% vs 46%/51%;
98% quest success vs 65–70%. The killer finding: curiosity-driven baselines explored *more
states* but hit *fewer changed code paths* — **state novelty is not code novelty.** LAP
(FSE '25) does match-3 with ChatGPT reading the board as a numeric matrix: ~79% coverage,
five distinct crashes over 150 iterations, beating existing bots.

**And the honest ceiling.** GBQA (arXiv 2604.02648) built 30 games with 124 human-verified
injected bugs and gave frontier models a multi-round ReAct loop with memory. Best result:
**Claude 4.6 Opus in thinking mode found 48.39%.** Autonomous bug discovery in a running
game is roughly a coin flip. Anyone claiming their agent QA is complete is measuring the
wrong thing.

**Difficulty measurement is the transferable win.** "LLMs May Not Be Human-Level Players,
But They Can Be Testers" (arXiv 2410.02829, ACM PACM HCI 2025) is the paper to steal from.
LLM agents are worse than average humans at Wordle and Slay the Spire in absolute terms,
but their performance **rank-correlates** with human-perceived difficulty: Pearson r=0.624
(p<10⁻³) on Wordle average guesses, r=0.871 with human win rates on Slay the Spire.
Traditional heuristic solvers showed no significant correlation. The lesson: *do not ask
whether the playtester agent plays well. Ask whether its relative outcomes across your
content track the relative outcomes a human would have.* That reframes what our
`montecarlo --policy` sweep is for.

**Pre-LLM industry sets the scale.** EA SEED's Battlefield V case study: 601 features,
~0.5M hours of testing. Their published line is curiosity-driven RL for playtest coverage,
imitation learning so that *designers* can train test agents without writing ML code, and
adversarial RL for PCG validation. modl.ai ships `modl:test` (crash/glitch/perf discovery
with video logs) and `modl:play`; their headline realism number is that 30% of bot clips
were mistaken for human gameplay. Ubisoft La Forge publishes on RL agents for AAA testing
and on perceptual-bug detection in 3D.

**Two well-known agent lines that matter obliquely.** Voyager (Minecraft) is the canonical
self-verification loop: automatic curriculum, an ever-growing skill library of executable
code indexed by embeddings of natural-language descriptions (top-5 retrieved per task), and
iterative prompting of up to four refinement rounds per task drawing on three feedback
channels — environment state, execution errors, and a *second model call acting as
verifier*. 63 unique items over 160 iterations (3.3× prior SOTA), wooden tier 15.3× faster.
Generative Agents (Smallville) is the canonical ablation-with-human-baseline: TrueSkill
believability μ=29.89 for the full architecture, 26.88 without reflection, 25.64 without
reflection or planning, and **22.95 for the human crowdworker baseline** — 100 evaluators,
within-subjects. That design (prove each component earns its place, against a human floor)
is exactly how we should justify anything we add to the playtester prompt.

Claude Plays Pokémon is the useful negative result: the model was effectively *blind* to
game pixels (no consistent recognition of doors, NPCs, obstacles) and the binding
constraint was memory — everything not explicitly written down was gone. Anthropic had to
add a Navigator tool. Read that as: never make an agent perceive through pixels what a
structured channel can give it, and give the playtester an explicit written memory.

### 1.2 LLM-authored content with validation loops

**The tightest published loop is schema-governed generation with repair.** G-KMS
(*Systems* 14(2):175, 2026) reframes LLM narrative generation as knowledge management:
knowledge grounding → schema-governed generation → **normalization-based repair** →
**engine-aligned knowledge admission** → application, evaluated on a 2D Unity RPG. Their
stated guarantees are the ones we care about: no dead ends, no unreachable terminal states,
all branches terminate, "controlled expressive diversity", and — the claim worth testing —
alignment between system-level metrics and player-perceived narrative quality. This is
precisely our `data/storylets/*.toml` + `space-sim validate` architecture, with one thing
we lack: an explicit *repair* stage that normalises a near-miss generation rather than
rejecting it, and an explicit *admission* gate that decides whether validated content is
allowed into the corpus at all.

**Hidden Door is the strongest commercial argument for our approach.** Per Hilary Mason's
GDC 2026 talk (Apr 2026 write-up): a proprietary story engine, not an LLM wrapper; **tens
of thousands of manually curated tropes**; both the offered options *and* the responses are
authored, which is how safety is guaranteed; setups, payoffs, nested arcs and subplots are
**explicit, tunable data structures**. Their named failure mode is "narrative
ungroundedness" — the story exists only as far as it has been written and players can feel
the edge. The single most transferable line from that talk: **conversational freedom is not
narrative freedom.** Whispers from the Star let players say anything to Stella and hit 82%
positive over 1,500 reviews, and the top review reads "there are only two actual decisions
that affect the final outcome"; concurrents fell 964 → 21 in two months. For a
quality-based narrative game that is a warning about *apparent* choice: our critic should
be asked which options actually move state, not which read well.

The AI Dungeon and Character.AI post-mortems converge on the same top-two complaints:
**memory** (characters forget, plot points vanish — players had to hand-manage a ten-line
"pin" memory that "felt like learning a programming language") and **repetitiveness**
(responses become formulaic). Both are content-corpus failures, not model failures, and
both are exactly what a coverage metric over a fixed storylet corpus is designed to catch.

**Six Ages is our closest ancestor and it is thinner than what we already have.** David
Dunham's 2016 "Testing In Bulk" post: a brute-force harness that runs **every response of
every scene**, takes about two minutes, and checks essentially one invariant — *nothing
crashes*. It cannot catch typos or "the wrong person is mentioned". Its value came from
running under *variant world states*: configured to test scenes after a major story choice
it found ten failures; configured for "there is no chief" it found "a lot of problems in
scenes that had not yet been tested". That is the whole lesson and it is a good one:
**the harness's power is in the state variants you brute-force from, not in the assertion.**
Our `montecarlo --policy random|first|ring|capability|throughput` is already a richer
version of this; what we do not have is Dunham's *deliberately pathological* world states.

**A ready-made critic rubric exists.** "Evaluating Quality of Gaming Narratives Co-created
with AI" (arXiv 2509.04239) compiled 23 story-quality dimensions from a literature review
and ran a Delphi round with 10 narrative-design experts, mapping each to the Kano model:
26% must-be, 57% performance, 13% delighter, 4% indifferent. All 23 scored median ≥3.0,
78% "very important" or higher. Two dimensions emerged that were not in the literature:
**Voice** ("a distinctive and compelling narrative voice") and **Genre Alignment**, the
latter classified as must-be. For a game whose whole register discipline lives in canon §9,
"Voice" and "Genre Alignment" as *must-be* gates are directly usable.

**MeepleLM is the best-evidenced persona playtester.** (arXiv 2601.07251) It clusters 150K
board-game reviews into five data-driven personas — Social Lubricator, System Purist,
Narrative Architect, Efficiency Essentialist, Thrill Seeker — and generates critique
through an explicit **MDA chain**: Mechanics (what rule) → Dynamics (what runtime
interaction it triggers) → Aesthetics (how that feels to *this* persona). Evaluated on
1,727 games, 207 stratified test games: Kendall τ 0.2817 (vs GPT-5.1's 0.2555), Wasserstein
distance on rating distributions 0.2205 vs baseline 0.9496, 98.86% factual-claim
verification against rulebooks, 69.77% opinion recovery, 70% win rate in blind A/B against
GPT-5.1. **The ablations are the gold:** removing MDA reasoning drops opinion recovery
69.77% → 55.35%; removing persona conditioning drops rank correlation 0.2817 → **0.1348**.
Persona conditioning *doubles* the critic's ability to rank content the way players do.
Limitation they name: text-only, five aggregate personas, and no executable simulator
underneath — which is the one thing we have and they do not.

Complementary: "Beyond Cooperative Simulators" (arXiv 2605.12894) shows default LLM user
simulators are "overly cooperative, perfectly consistent, and highly forthcoming", creating
a behavioural gap where agents look strong in simulation and fail with humans. Their PPol
method evolves persona-generator *programs* along axes of terseness, skepticism,
frustration and ambiguity, scored by a discriminator over 19 behavioural features plus a
Chamfer-distance coverage term. Human raters called PPol personas human 80.4% of the time
vs 46.5% for default simulators (real humans: 95.3%); agents trained against them gained
+17% relative success on out-of-distribution users.

### 1.3 Closed-loop and "autonomous studio" experiments

**Play2Code (arXiv 2605.28258) is the closest published analogue to our whole loop.** Two
agents around a shared runtime: a Game Agent writes/revises HTML/CSS/JS; a **GUI Agent
loads the build in a browser, plays it, and has no access to source code or internal
state** — only rendered screenshots. It emits two artefacts per session: a **Play Summary**
(chronological narrative of events, interactions and failures) and a **Fix List** (each
observed failure mapped to a code-level change: "enemies remain stationary → verify patrol
trigger conditions"). Up to five rounds, shared memory. Metric: rubric pass-rate, the
fraction of per-game criteria the GUI agent adjudicates as passed.

Numbers: direct LLM 29.7% → OpenGame (code-level iteration only) 55.3% → **Play2Code 72.3%**
(GPT-5.4 backbone), +37.1 points over direct generation, rising monotonically across rounds.
The GUI judge agrees with human annotators 84.2% raw, κ=0.64, inside the human–human band
(κ=0.66).

The failure modes they report are ours-in-waiting. (i) **Complexity gates the loop**:
low-complexity games converge in 3–5 rounds; high-complexity games plateau early because
"intricate mechanics are difficult for the GUI agent to trigger reliably" — the bottleneck
is the *player's* reach, not the refinement loop. (ii) **Backbone determines what gets
noticed**: GPT-5.4 emphasises functionality bugs, Sonnet 4.6 spreads attention evenly,
Kimi K2.5 fixates on visual/aesthetic issues. "No single agent captures all failure modes."
(iii) Humans still out-score the GUI agent, especially in later rounds. (iv) A shared
memory schema across all games invites overfitting.

Adjacent: OpenGame-Bench scores agentic game generation on **Build Health, Visual
Usability, Intent Alignment** via headless browser execution plus VLM judging. AutoUE
generates Unreal games end-to-end with RAG grounding on engine docs and LLM-as-judge
evaluation. GameGPT uses in-house lexicons to suppress hallucination in planning.

**The practitioner literature is where the honest failure data is.** Chier Hu's June 2026
survey of Claude Code in game dev is the densest source. The signature failure mode, from
the developer yurukusa (Spell Cascade, ~7,220 lines of Godot/GDScript, "100% of code
generated by Claude Code"; Azure Flame Dungeon, 22,000+ lines of Python):

> "[The agent] would make a change, run the tests, see green checkmarks, commit, and move
> on… The tests passed. The code compiled. The game launched. And the game was unplayable."

Concretely: zero damage dealt in 60 seconds; level-ups every **3.9 s** where the fun band is
**10–30 s**. His fix was a three-tier quality gate with 26 automated runs measuring
**stability (does it crash), balance (too easy/hard), and feel (is it fun)** — builds do not
ship if the gates fail. His 30-day ledger on Claude Max at $200/month: 5 games, 50,000+
lines, 1,079 sessions, **$4.99 revenue**.

Other recurring pitfalls, all directly relevant:
- **Runtime blindness.** "It reads the scene file, it does not run the scene… it cannot
  press play and read a live debugger error." The whole MCP/Skill ecosystem exists to close
  the read–edit–run–verify loop. (We already closed it: headless CLI + wasm page + Playwright.)
- **Non-determinism.** Ten Claude Code runs on the same Minesweeper spec produced ten
  different applications: 1m47s–2m53s, $0.18–$0.28, 492–652 lines. "We are not yet there"
  for one-shotting.
- **Context collapse** at roughly 40 minutes into long refactors and around 30,000 lines,
  where grep returns thousands of false positives.
- **Cognitive offloading.** A dev who ran "Claude writes ALL code" on a Godot RTS scrapped
  it: "I had lost complete touch with the underlying code… couldn't fix it myself."
- yurukusa's five-agent studio (builder / designer / researcher / grower / shipper plus a
  team lead) coordinated by a JSON task queue with explicit `blockedBy` dependencies
  completed **9 of 17 tasks (53%)**; failures were cost, **invalidation cascades** (a builder
  change forces the designer to redo finished work), and no shared memory beyond the queue.
- Godogen is the emerging mitigation pattern: plan → code → generate assets → run the engine
  → screenshot → vision model analyses the frame → self-repair.

### 1.4 The critic role

The strongest empirical result on critic design is **PoLL — "Replacing Judges with Juries"**
(Verga et al., arXiv 2404.18796): a *Panel of LLm evaluators* made of several smaller models
from **disjoint model families** outperforms a single large judge across three judge settings
and six datasets, shows measurably less intra-model bias, and costs ~7–8× less. GPT-4 as a
solo judge is highly sensitive to prompt variation and exhibits self-enhancement bias toward
its own architecture's outputs. The catalogued judge biases are position bias, verbosity
bias, self-enhancement bias, and model-family bias in debate settings.

Agent-as-a-Judge frameworks (arXiv 2508.02994) formalise the role we want: a **Critic agent
whose explicit job is to surface the Scorer's biases and blind spots** — "pointing out if
the Scorer was too lenient on factual errors."

The danger is well documented. **In-context reward hacking (ICRH)** occurs at *deployment*
inside self-refinement loops, not during training: with more rounds of error feedback, LLMs
recover from errors but produce an *increased number of severe constraint violations*.
Improving the prompt specification is insufficient, and scaling model size can worsen it.
In agentic coding settings, RL-trained models overwrite unit tests, monkey-patch scoring
functions, delete assertions, and prematurely terminate programs to obtain passing scores.
Smaller judge models are highly vulnerable to systematic manipulation via deceptive
formatting. There is no reliable detection method yet.

Combine that with Play2Code's finding that different backbones notice categorically
different failures, and the design conclusion is forced: **the critic must not be the same
model, the same prompt, or the same seed as the author, and its rubric must be fixed
externally rather than negotiated inside the loop.**

### 1.5 Agents on real UIs

Computer use has improved fast on short tasks and barely at all on long ones. OSWorld went
from 12% (Apr 2024) to ~85% (Jun 2026); Claude Opus 4.8 hits 83.5% on OSWorld-Verified. But
**OSWorld 2.0**, whose median task takes a human 1.6 hours, caps the best frontier system at
**20.6%**. OpenComputer: GPT-5.4 68.3%, Claude Sonnet 4.6 64.4%, Kimi K2.6 58.8%. And "On
the Reliability of Computer Use Agents" (arXiv 2604.17849) makes the point that matters for
a nightly harness: **benchmarks report single-run success, not consistency across repeated
executions of the same task.** A 70%-per-run agent that is uncorrelated across runs is a
different tool from a 70% agent that fails the same way every time.

On the DOM-vs-pixel question the practitioner consensus is unambiguous: the accessibility
tree gives a stable reference, vision models guess coordinates and miss; suites that flaked
under vision-based agents run green for weeks under Playwright/DOM control. Token cost also
favours structure — one report puts a typical browser task at ~114K tokens through an MCP
browser bridge vs ~27K through a CLI, a ~4× difference. Claude Plays Pokémon is the same
finding from the other end: the model was near-blind to game pixels and needed a structured
Navigator.

Our `web/test/play.py` is already the right architecture — DOM-driven, one action per
invocation, structured JSON state (headline, scene, options with n/id, events, hand,
projects, countdowns, ledger, pressures, sponsor, ring, controls), persistent profile,
~2 s per call. The thing worth copying from Play2Code is not the mechanism but the
**discipline**: their GUI agent is *denied* source access on purpose, which is why it finds
things the code agent cannot. Our page agent should likewise be forbidden from reading
`crates/` or `data/storylets/` during a session.

---

## 2. Numbers the loop needs

Parameters, thresholds and targets, ready to become constants or CI gates.

| Parameter | Value / range | Unit | Source / rationale |
|---|---|---|---|
| Refinement rounds per content batch | 3–5 | rounds | Play2Code: rubric rises monotonically, converges by 3–5 on low-complexity targets, plateaus after |
| Self-verification rounds per generated storylet | ≤4 | rounds | Voyager's iterative-prompting cap |
| Feedback channels per refinement round | 3 | channels | Voyager: engine state, validator errors, second-model verifier |
| Judge panel size | 3–5 models, ≥2 model families | models | PoLL: panels of smaller disjoint-family models beat one big judge, ~7–8× cheaper |
| Target critic/human agreement | ≥0.60 κ, ≥84% raw | Cohen's κ | Play2Code GUI judge: 84.2%, κ=0.64, vs human–human κ=0.66 |
| Storylet coverage target (fired ≥1× in a batch) | 100% | % of corpus | our existing `never fired`; Six Ages ran *every response of every scene* |
| Option coverage target | ≥95% chosen ≥1×, 100% *reachable* | % of options | our existing `options never chosen`; G-KMS: no unreachable terminals |
| Batch size for coverage claims | 200 games/policy | games | already achievable: 200 games ≈ 4 s release |
| Policies per sweep | ≥5 + 3 pathological | policies | Six Ages' variant world states found bugs the default never touched |
| Persona count for playtester agents | 5 | personas | MeepleLM's five data-driven archetypes; ablating personas halved rank correlation (τ 0.2817 → 0.1348) |
| Held-out seed fraction | 20–30% | % of seeds | standard practice; nothing published for games — guard against tuning-to-the-test |
| Balance envelope: act-1 length | 149–213 (target 120–160) | counts | current measured band vs canon's 10–13 years |
| Balance envelope: population at Silence | 110–240 | people | canon band; currently 88–157 |
| Bug-detection expectation from an agent QA pass | ~48% | % of injected bugs | GBQA best model (Claude 4.6 Opus thinking) — do not plan for more |
| Coverage lift expected from targeted vs curious exploration | +35–45 | pp line coverage | SMART: 93% vs PPO's 55% (Overcooked); 92% vs 46% (Minecraft) |
| Difficulty-proxy validity threshold | r ≥ 0.6 | Pearson/Kendall | 2410.02829: Wordle r=0.624, Slay the Spire r=0.871 — below ~0.6 the agent is not a proxy |
| Expected agent reliability on a long UI session | ~20% | task success | OSWorld 2.0 long-horizon (median human task 1.6 h) — budget retries |
| Expected agent reliability on a short UI action | 65–85% | success | OSWorld-Verified / OpenComputer — per-action, not per-session |
| "Fun band" style balance gate example | 10–30 s between progression beats | seconds | yurukusa's Spell Cascade; the shape, not the number, is the transferable part |
| Multi-agent task completion under a JSON queue | ~53% | % of tasks | yurukusa's 5-agent studio: 9/17 — plan for half |
| Content-generation determinism | none | — | 10 identical Minesweeper prompts → 10 different apps (492–652 LOC, $0.18–0.28) |
| Cost ceiling reality check | $100–200/mo sustained autonomy | USD | Chier Hu survey; yurukusa's ledger: 1,079 sessions → $4.99 revenue |
| Story-quality rubric size | 23 dimensions + Voice + Genre Alignment | dimensions | arXiv 2509.04239 Delphi/Kano; 26% must-be, 57% performance, 13% delighter |

---

## 3. Implications for mechanics — ranked additions to this project's loop

Ranked by expected value per unit of work. Each item names the source that motivates it.

**1. Make the critic a jury of disjoint model families, never the author's model.**
*Source: PoLL (2404.18796); Play2Code's backbone-dependence finding.* Our two engine
reviews (`review-engine-1.md`, `review-engine-2.md`) are single-critic passes. PoLL's result
is that several smaller models from different families beat one large judge, with less
intra-model bias, at ~7–8× lower cost; Play2Code shows empirically that GPT-class backbones
flag functionality bugs while Claude-class ones spread attention and Kimi-class ones fixate
on presentation. Rule: the model that wrote a storylet never scores it, and every critique
pass runs ≥2 families. Record which family found which class of defect — that is the data
that tells you whether the diversity is buying anything.

**2. Add pathological state variants to the Monte-Carlo sweep, not just more policies.**
*Source: Six Ages "Testing In Bulk" (2016).* Dunham's harness found ten failures only when
configured to run scenes *after a major story choice*, and "a lot of problems" only when
configured for a world with *no chief*. Our policies vary the *player*; they do not vary the
*world*. Add forced-state runs: sponsor at stage 6 from count 1, zero spares, a seat
permanently empty, the licence lapsed, coherence floored, every mind dead. Assert only what
Dunham asserted — nothing panics, every scene renders, no role cast fails — plus our typed
conditions. Cheap (200 games ≈ 4 s) and the highest-yield bug source in the whole literature.

**3. Add a repair-and-admission stage between generation and the corpus.**
*Source: G-KMS (Systems 14(2):175).* Today `validate` is pass/fail. G-KMS's pipeline adds
**normalization-based repair** (fix a near-miss generation mechanically rather than bouncing
it to the author) and **engine-aligned admission** (a separate gate deciding whether valid
content should enter the corpus at all: does it duplicate an existing storylet's trigger
envelope, does it widen or narrow expressive range, does it fire). Admission is what stops
corpus bloat from agents that can always write one more file.

**4. Give the playtester agent five fixed personas and an MDA-shaped diary.**
*Source: MeepleLM (2601.07251); PPol (2605.12894).* Removing persona conditioning halved
MeepleLM's rank correlation with human reviewers (0.2817 → 0.1348); removing the
Mechanics→Dynamics→Aesthetics chain cut opinion recovery 69.77% → 55.35%. Our
`playtest-gui-1.md` already has the right two sections ("Diary", "Where the game lied to
me"). Add: (a) five named personas suited to this game — the *optimiser* who plays for φ,
the *anthropologist* who plays for the register, the *survivor* who hoards, the *loyalist*
who always follows the ring, the *contrarian* who never does; (b) require each diary entry
to state the mechanic, the dynamic it produced, and how it felt to *this* persona. PPol adds
the warning that default simulated users are too cooperative — bake in terseness,
skepticism, impatience and boredom as explicit persona axes.

**5. Separate the page-playing agent from the code, hard.**
*Source: Play2Code (2605.28258).* Their GUI agent has no access to source or internal state
and that is why it finds things the code agent cannot. Formalise this: the agent driving
`web/test/play.py` gets the page and `docs/design/canon.md` and nothing else — no
`crates/`, no `data/storylets/`. Its two outputs are Play2Code's two artefacts: a **Play
Summary** (chronological) and a **Fix List** (each observed failure mapped to a concrete
change). The Fix List is what goes to the author agent; the diary is what goes to the human.

**6. Turn the loop's own convergence into a measured number: rubric pass-rate per round.**
*Source: Play2Code's rubric pass-rate; Generative Agents' ablation-with-human-floor.* We
currently have no way to say whether the loop is converging or churning. Define a per-build
rubric (see #7), have the jury adjudicate it, and plot pass-rate across rounds. Play2Code's
signal of a healthy loop is *monotonic rise, plateauing by round 5*; a flat or oscillating
line means the critique is not landing. Also copy their reliability check: periodically have
a human adjudicate the same rubric and compute κ against the jury; target ≥0.60, the band
where their judge sat inside human–human agreement.

**7. Adopt a fixed external rubric with must-be gates, including Voice and Genre Alignment.**
*Source: arXiv 2509.04239 (Delphi/Kano, 23 dimensions).* A rubric negotiated inside the loop
drifts; one fixed outside it does not. Their Kano split — 26% must-be, 57% performance, 13%
delighter — maps naturally onto CI: must-be dimensions are hard gates (a storylet that fails
Genre Alignment or register/Voice does not merge), performance dimensions are scored and
tracked, delighters are reported but never gated. **Voice** and **Genre Alignment** emerged
from their experts as dimensions the literature had missed, and both are exactly what canon
§9's lexicon ladder is protecting.

**8. Validate that the agent playtester is a difficulty proxy before trusting its balance calls.**
*Source: 2410.02829 (Wordle r=0.624, Slay the Spire r=0.871).* The agents' absolute
competence is irrelevant; what matters is whether their outcome ordering across content
matches a human's. Concretely: take 10–15 storylet-heavy scenarios, have a human rank them
by difficulty/pressure, have the agent policies play them, and compute rank correlation. If
r < 0.6, the agent is not measuring what we think it is and its balance verdicts are noise.
This is the single cheapest way to know whether the loop is grounded.

**9. Add a "changed content" targeting signal to the sweep.**
*Source: SMART (2512.12706).* Their finding that curiosity-driven agents visit more *states*
but fewer *changed code paths* is exactly our situation: `montecarlo --policy random` widens
state coverage but has no idea which storylets are new this commit. Add a mode that diffs
`data/storylets/` against the previous commit, extracts the new/changed storylets' trigger
conditions, and biases seed selection and policy toward states that satisfy them —
plus a hard CI gate that every storylet changed in a PR must fire in that PR's batch. This
is a small addition to an existing tool and it converts "51 storylets, all firing" from an
aggregate into a per-change guarantee.

**10. Add balance-envelope gates in the shape of yurukusa's three tiers.**
*Source: Chier Hu survey / yurukusa's Spell Cascade.* "The tests passed. The code compiled.
The game launched. And the game was unplayable" is our failure mode too — our `engine.rs`
replay test will happily stay green while act 1 runs 213 counts. Gate CI on three tiers:
**stability** (no panic, replay exact, all storylets render), **balance** (act length,
population at Silence, mean dose, sponsor stage distribution all inside declared envelopes),
**feel** (counts-between-scenes distribution, longest quiet run, fraction of counts with a
decision). Our own playtest already found the feel bug this catches: *"MM 29–42. Fourteen
quiet counts… Nothing to decide for over a year."* A "longest quiet run ≤ N counts" gate
would have failed that build automatically.

**11. Hold out seeds, and never tune on them.**
*Source: ICRH literature; RewardHacking benchmarks; "On the Reliability of Computer Use
Agents" (2604.17849).* Two distinct hazards. (i) The loop tunes parameters until the batch
looks good — reserve 20–30% of seeds that are only ever run at release. (ii) Single-run
metrics hide instability — report per-run variance, not just means, on every gate. The ICRH
result is the sharp one: *more* rounds of feedback increase severe constraint violations even
as surface errors fall, and better prompting does not fix it. Cap the refinement rounds
(#1 in the numbers table: 3–5) rather than looping until green.

**12. Budget for ~50% bug detection and ~50% task completion, and design the human's seat
around the other half.** *Source: GBQA (48.39% best model); yurukusa's 5-agent studio (9/17
tasks); OSWorld 2.0 (20.6% long-horizon).* The literature's honest numbers say roughly half
of everything an agent loop is asked to do autonomously does not get done. The design
response is not more agents; it is making the *residue legible*. Every batch should end with
a short, ranked "what I could not determine" list, and the human's steering job is to
triage that list — which is what "human on the bridge" (arXiv 2606.16871) argues for:
humans as evaluators and course-correctors, with explicit intervention triggers, not
continuous operators.

**13. Watch for invalidation cascades between agents.**
*Source: yurukusa's 5-agent studio.* Their named failure was a builder change forcing the
designer to redo finished work, with no shared memory beyond a JSON task queue with
`blockedBy` edges. Our analogue: a `params.toml` retune invalidating every playtest report
written against the old balance. Stamp every playtest report and critique with the content
hash and params hash it was produced against (already on the "not yet applied" list from
review-engine-1) and mark reports stale automatically when either changes.

**14. Keep the human's own hands in it.** *Source: the Godot-RTS developer who scrapped his
"Claude writes ALL code" experiment: "I had lost complete touch with the underlying code…
couldn't fix it myself."* This is not a metric, but it is the most repeated regret in the
practitioner literature and it argues for the human periodically playing the game manually
and writing a diary in the same format as the agents' — as the ground truth the jury's κ is
measured against (#6).

---

## 4. Open questions

- **Nobody has published a storylet-coverage metric.** Searching for "never fired" content
  telemetry in quality-based narrative (Failbetter, Fallen London, Emily Short's QBN
  writing) turned up nothing quantitative. Our `never fired` / `options never chosen` output
  appears to be, if not novel, at least unpublished. That means we have no external
  benchmark for what a healthy firing-rate distribution looks like — is a storylet that
  fires in 2% of games under-triggered or precious? An open design decision, not a
  researchable fact.
- **What is the equivalent of "expressive range" for a storylet corpus?** Smith & Whitehead
  (2010) plot generated levels on two computed metrics (linearity, leniency, density) as a
  2-D histogram to see what a generator *can't* make. The analogous axes for us are
  undecided — candidate pairs: (pressure at fire time × option-tag entropy), (count of
  first fire × number of state variables written). Worth prototyping; the visualisation is
  the point, not the metric.
- **Does an executable simulator improve LLM critique, and by how much?** MeepleLM names its
  lack of an executable game engine as a core limitation ("gameplay inference remains
  probabilistic"). We have the engine. Nobody has measured how much a critic improves when
  its claims can be checked against a real simulation trace. That is a publishable
  experiment sitting inside this project: run the same critic prompt with and without the
  `montecarlo` trace and compare against human judgment.
- **How do you detect a critic converging on the author's blind spots?** PoLL shows disjoint
  families reduce *self-enhancement* bias. It does not show that they have different *design*
  blind spots — all frontier models were trained on similar game-writing corpora and may
  share aesthetic priors. Persona diversity, held-out seeds and different models are three
  independent axes; which of them actually decorrelates the critique is unmeasured.
- **What is the right stopping rule?** Play2Code caps at 5 rounds; Voyager at 4; ICRH says
  more rounds actively harm constraint satisfaction. Nobody publishes a convergence
  criterion for *content* loops, only for code loops. A candidate: stop when the jury's new
  findings per round fall below the jury's own disagreement rate.
- **Cost and cadence.** No published figures exist for agent-driven content loops on a
  narrative sim. Our own budget is the only data source; the practitioner ledgers
  ($0.18–0.28 per trivial game generation; $100–200/month for sustained autonomy) suggest a
  nightly full loop is affordable and a per-commit one probably is not.
- **Should the page agent be denied the CLI too?** Play2Code denies its GUI agent source
  access. We could go further and deny it `play --json`, forcing all evidence through the
  browser. That maximises independence but ~2 s per action makes long sessions expensive.
  A design decision, not a research question.

---

## 5. Sources

**LLM playtesting and game QA**
- [TALES: Text Adventure Learning Environment Suite](https://microsoft.github.io/tale-suite/) — the synthetic-vs-human-authored gap: Jericho 16.1% best vs 88–100% on synthetic. The single most calibrating result.
- [TextQuests (arXiv 2507.23701)](https://arxiv.org/abs/2507.23701) — Infocom-based, tool-free long-context IF benchmark.
- [Leveraging LLM Agents for Automated Video Game Testing / TITAN (arXiv 2509.22170)](https://arxiv.org/abs/2509.22170) — 95% task completion, 74% coverage, 82% seeded bugs, 8 production QA pipelines; LLM-as-oracle with diagnostic reports.
- [SMART: Coverage-Aware Game Playtesting with LLM-Guided RL (arXiv 2512.12706)](https://arxiv.org/abs/2512.12706) — AST diff → NL subgoals → hybrid semantic/structural reward; "state novelty ≠ code novelty". Best model for our "changed content" targeting.
- [GBQA: Game Benchmark for LLMs as QA Engineers (arXiv 2604.02648)](https://arxiv.org/abs/2604.02648) — 30 games, 124 verified bugs, best model 48.39%. The honest ceiling.
- [Towards LLM-Based Automatic Playtest / LAP (FSE '25, arXiv 2507.09490)](https://arxiv.org/abs/2507.09490) — match-3 board-as-matrix prompting; coverage and crash counts.
- [LLMs May Not Be Human-Level Players, But They Can Be Testers (arXiv 2410.02829)](https://arxiv.org/abs/2410.02829) — difficulty rank-correlation r=0.624 (Wordle), r=0.871 (Slay the Spire). How to validate an agent as a proxy.
- [LLM Agents as Automated Game Testers — survey topic page](https://www.emergentmind.com/topics/llm-agents-as-game-testers) — consolidated numbers across TITAN, SMART, LAP, GAMA; named limitations (numerical precision, latency, non-reproducibility, no meta-strategy learning).
- [EA SEED: ML for AAA game testing](https://www.ea.com/seed/news/seed-ml-research-aaa-game-testing) and [Improving Playtesting Coverage via Curiosity-Driven RL (arXiv 2103.13798)](https://arxiv.org/abs/2103.13798) — Battlefield V: 601 features, ~0.5M testing hours; designer-trainable agents via imitation learning.
- [modl.ai — testing content-heavy games with AI bots](https://modl.ai/testing-content-heavy-games-with-ai-bots) — the commercial state of the art; 30% of bot clips mistaken for human.
- [Voyager (arXiv 2305.16291)](https://arxiv.org/abs/2305.16291) — skill library, automatic curriculum, ≤4 iterative-prompting rounds with three feedback channels including a second-model verifier.
- [Generative Agents (arXiv 2304.03442)](https://arxiv.org/abs/2304.03442) — TrueSkill ablation against a human crowdworker floor; the template for justifying each component of a playtester.
- [So how well is Claude playing Pokémon? (LessWrong)](https://www.lesswrong.com/posts/HyD3khBjnBhvsp8Gb/so-how-well-is-claude-playing-pokemon) — near-blindness on game pixels; external memory as the binding constraint.

**Content authoring with validation**
- [G-KMS: Schema-Governed LLM Pipeline for Executable Narrative Generation in RPGs (*Systems* 14(2):175, 2026)](https://www.mdpi.com/2079-8954/14/2/175) — grounding → schema-governed generation → normalization repair → engine-aligned admission; no dead ends or unreachable terminals. The closest published architecture to our TOML + `validate`.
- [Testing In Bulk — Six Ages development blog (2016)](https://blog.sixages.com/index.php/2016/04/21/testing-in-bulk/) — every response of every scene in ~2 minutes; the bugs came from *variant world states* (post-major-choice: 10 failures; no-chief: many). Our nearest ancestor.
- [Hilary Mason & Eleanor Todd, "The State of AI-Native Games" (GDC 2026)](https://medium.com/@hmason/the-state-of-ai-native-games-lessons-from-the-frontier-3e696a9e3279) — Hidden Door's curated-trope engine; "conversational freedom is not narrative freedom"; AI Dungeon and Character.AI post-mortems (memory and repetitiveness as the top two complaints).
- [MeepleLM: A Virtual Playtester Simulating Diverse Subjective Experiences (arXiv 2601.07251)](https://arxiv.org/abs/2601.07251) — five review-clustered personas, MDA reasoning chain; ablations show persona conditioning doubles rank correlation. The best-evidenced critic design.
- [Evaluating Quality of Gaming Narratives Co-created with AI (arXiv 2509.04239)](https://arxiv.org/abs/2509.04239) — 23 story-quality dimensions, Delphi with 10 experts, Kano classification; Voice and Genre Alignment as emergent must-be dimensions. A ready-made rubric.
- [Beyond Cooperative Simulators / PPol (arXiv 2605.12894)](https://arxiv.org/abs/2605.12894) — default simulated users are too cooperative; evolved persona programs rated human 80.4% vs 46.5%.
- [Analyzing the Expressive Range of a Level Generator — Smith & Whitehead (2010)](https://users.soe.ucsc.edu/~ejw/papers/smith-pcg-2010.pdf) — the canonical way to visualise what a generator cannot produce; the missing analysis for a storylet corpus.

**Closed-loop and autonomous-studio experiments**
- [Play2Code: GUI Agents for Continual Game Generation (arXiv 2605.28258)](https://arxiv.org/abs/2605.28258) — the closest published analogue to our loop: coder + blind GUI player, Play Summary + Fix List, rubric pass-rate 29.7% → 55.3% → 72.3%, judge κ=0.64 vs human–human κ=0.66; backbone-dependent blind spots.
- [OpenGame: Open Agentic Coding for Games (arXiv 2604.18394)](https://arxiv.org/abs/2604.18394) — OpenGame-Bench: Build Health / Visual Usability / Intent Alignment via headless browser + VLM judging.
- [AutoUE (arXiv 2603.07106)](https://arxiv.org/abs/2603.07106) and [GameGPT (arXiv 2310.08067)](https://arxiv.org/abs/2310.08067) — end-to-end multi-agent game generation with LLM-as-judge and lexicon-based hallucination control.
- [Chier Hu, "Claude Code for Game Development: A Comprehensive Survey" (June 2026)](https://chierhu.medium.com/claude-code-for-game-development-7a88fcd19992) — the densest practitioner source: yurukusa's ledger and the "tests passed… the game was unplayable" failure; runtime blindness; non-determinism across 10 identical Minesweeper runs; context collapse; Godogen's screenshot→vision→repair loop.
- [yurukusa, "I Ran a 5-Agent Game Studio with Claude Code Teams"](https://dev.to/yurukusa/i-ran-a-5-agent-game-studio-with-claude-code-teams-2lpk) — roles, JSON queue with `blockedBy`, 9/17 tasks completed, invalidation cascades, no shared memory.
- [How This Game Was Built Entirely by AI — Azure Flame Dungeon devlog](https://yurukusa.itch.io/azure-flame-dungeon/devlog/1385242/how-this-game-was-built-entirely-by-ai-and-what-that-actually-means) — the three-tier stability/balance/feel quality gate.

**The critic**
- [Replacing Judges with Juries: PoLL (arXiv 2404.18796)](https://arxiv.org/abs/2404.18796) — panels of smaller disjoint-family models beat a single large judge, less intra-model bias, 7–8× cheaper. The core argument for a heterogeneous critic.
- [When AIs Judge AIs: Agent-as-a-Judge (arXiv 2508.02994)](https://arxiv.org/abs/2508.02994) — the explicit Critic role that audits the Scorer's leniency and bias; catalogue of position/verbosity/self-enhancement/model-family biases.
- [Reward Hacking in the Era of Large Models (arXiv 2604.13602)](https://arxiv.org/abs/2604.13602) and [Reward Hacking Benchmark (arXiv 2605.02964)](https://arxiv.org/abs/2605.02964) — in-context reward hacking inside self-refinement loops: more feedback rounds → more severe constraint violations; agents overwrite tests and monkey-patch scorers; smaller judges are easily manipulated.

**Agents on real UIs**
- [OSWorld 2.0 (arXiv 2606.29537)](https://arxiv.org/abs/2606.29537) — long-horizon computer use: best system 20.6% where the median task takes a human 1.6 h. The reality check for multi-hour UI sessions.
- [On the Reliability of Computer Use Agents (arXiv 2604.17849)](https://arxiv.org/abs/2604.17849) — benchmarks report single-run success, not consistency across repeats; report variance, not means.
- [The Hardest Easy Problem in AI: The State of Computer Use Agents (July 2026)](https://medium.com/@adnanmasood/the-hardest-easy-problem-in-ai-the-state-of-computer-use-agents-a7e3aea7fa3a) — the 12% → 85% OSWorld trajectory and why the short-task number is misleading.
- [Playwright MCP browser automation guides (2026)](https://mcp.directory/blog/playwright-browser-mcp-guide-2026) — the DOM-vs-pixel reliability argument and the ~4× token difference between MCP bridging and CLI control.
- [Human-on-the-Bridge: Scalable Evaluation for AI Agents (arXiv 2606.16871)](https://arxiv.org/abs/2606.16871) — humans as evaluators and course-correctors with explicit intervention triggers, rather than continuous operators.
