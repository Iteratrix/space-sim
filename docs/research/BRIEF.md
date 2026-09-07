# Shared research brief

You are researching for a realistic asteroid-belt settlement simulation game.
Read `/home/ve/Projects/space-sim/NOTES.md` first for the design so far.

Key framing: settled-society assumptions break in the belt. The better analogies
are nomadic (steppe) and maritime (Polynesian, Bajau) cultures. The game is
King of Dragon Pass-style: turn-based, a real simulation underneath, an advisor
ring on top, early choices are seeds for later acts. Three acts: sponsored
outpost -> cut off and going Voidborn -> confederating the belt.

## Output

Write your report to the file named in your task, in Markdown, using exactly
these sections in this order:

1. **Findings** — the substance. Be concrete, cite numbers, prefer primary and
   technical sources. Disagree with the design brief where the evidence says so.
2. **Numbers the sim needs** — a table or list of parameters with values or
   ranges and units, ready to be turned into constants or data files.
3. **Implications for mechanics** — specific, actionable design suggestions.
   Each one: what the mechanic is, what evidence motivates it, which act it
   belongs to. This section is what the next phase of agents will read most.
4. **Open questions** — what you couldn't resolve, what needs a design decision.
5. **Sources** — links or citations, with a one-line note on what each is good for.

## Working method

- Write the file EARLY with a skeleton and your first findings, then update it
  as you go. If you are interrupted, what is on disk is what survives.
- Use WebSearch and WebFetch. If a page blocks you (403, bot check), use
  `uv run ~/.claude/scripts/webfetch.py URL` instead. Do not use a browser.
- Depth over breadth. Twenty solid sources beat sixty skimmed ones.
- Target length: 2,000-5,000 words. Longer is fine if it is dense.
- Do not pad. Do not summarize the brief back. Do not write the game design;
  research it and hand over implications.
