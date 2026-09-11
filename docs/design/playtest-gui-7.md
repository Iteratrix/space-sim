# GUI playtest 7: the impatient player on onramp v2, re-run after playtests 5 and 6

Seed 44, tutorial scenario, local page at build `From playtest 6: requirements demand a
rated die…` (2026-09-11). Same persona and rules as playtest 5: nothing read beyond the
scene title and option labels; option 1 every time; End count pressed the moment a
scene is resolved; when the button holds, the minimum it demands — the first die in
the hand, then the next if the hold does not clear. Driven with `web/test/play.py
--profile g` by a script that did exactly that; screenshots at MM 3 (held), MM 7, MM 13,
MM 44.

## 1. Diary

| count | visible | forced | did | changed on screen without reading |
|---|---|---|---|---|
| 1 | scene + End count | scene | "Survey the mass driver" | one log line |
| 2 | one clock *Shelter, first chamber 0/6*; hand of 14 (2 dimmed) | End count read *Place one crew die with a rating on Shelter, first chamber.* | tapped the first die, tapped the clock | "0/6 · 4 pips/month" at once; button back to End count |
| 3 | *Bake-out plant (BOP)* clock, dim ring, "idle — no dice" in the accent colour; 11 square `R` dice in the hand | **held: *Place one robot unit on Bake-out plant (BOP).*** | tapped the first `R`, tapped the plant | ring brightened to "rate 0.20 · 2/10 pips"; water bar appeared with "▲4"; one seat appeared |
| 4 | ring: 3 seats, two with recommendations | scene | "Cut under the ridge" | shelter 1/6 |
| 5 | RSW clock appeared, "5 left" | nothing | End | shelter 3/6 · bonus |
| 6 | people bar appeared | nothing | End | shelter 5/6 → clock gone; my die back in the hand |
| 7 | "First Chamber Pressurised"; events: *Shelter, first chamber complete.* | scene | "Direct the chamber crew onto the KEEP" | **Shelter (KEEP) 0/24 with one die on it, "4 pips/month"** |
| 8 | "Resupply Request, RSW-1"; 4th seat | scene | "Request throughput hardware" | KEEP 1/24 |
| 9–11 | KEEP filling 3 → 4/24; spares bar at 11 | nothing | End ×3 | — |
| 12 | "RSW-1 Arrival"; events: relay failure, *RSW arrival: 45 t … 8 in* | held: *Place one crew die with a rating on Mass driver alignment (MDLS).* | first die → still held; second die → clear | alignment "0/8 · 3 pips/month" with two dice on it (one reads `·`) |
| 13 | hours bar; roster/auto-deal controls; 5th seat; *Relay repair 0/4* clock | nothing | End | hand "13 of 22" |
| 14–19 | The Wrights at 14, else quiet | one scene | option 1 | KEEP 7 → 13/24; alignment 1 → 4/8; review clock at 15 |
| 20–24 | act-1 scenes most counts (Nobody Is Coming, Shift Request, Questionnaire, Phasing, Private Traffic) | scenes | option 1 | people bar *fatigued* at 21; alignment "2 pips/month"; events at 24: *MDLS at 2 pips a month: 3 months to the next segment* |
| 25–27 | The Manifest at 26; contract clock | scene | "Manifest for throughput" | two setback lines on the alignment; 7/8 → 6/8 |
| 28 | "Someone Wants to Stay"; events: *RSW arrival: 41 t … 7 in* | scene | "Grant residence" | KEEP 23/24; hand "13 of 29" |
| 29–30 | KEEP gone at 29 (*Shelter (KEEP) complete.* in events at 30); throw clock, position select and throw bar appeared at 30 | nothing | End | throw "rate 0.00 · 0/8 pips", *short* |
| 31 | "MDLS Alignment Complete"; events: *alignment complete* | held: *Place one crew die with a rating on Mass driver (MDLS).* | first die → clear | throw "rate 0.63 · 5/8 pips", three dice (two `R` placed by the scene); 6th seat; hours *tight* |
| 32–42 | act-1 texture most counts; Solar conjunction ring 36–41 | scenes | option 1 | throw 0.63 → 0.50 at 42 (a die pulled); hand 11 → 9 free |
| 43 | "MM 43: RSW-3" (handoff) | scene | "Minute it as the count from now on" | — |
| 44 | full screen: 7 bars, 5 clocks, 3 pressure rings, 6 seats; captions gone | nothing | End | *margin patched*, *hours tight*, *throw short*; hand "11 of 35" |

## 2. Verdict

**Held at count 2 and at count 3: yes, both.** Count 2 as before. Count 3: the bake-out
clock was on screen from the start of the count (dim ring, "idle — no dice" beside the
title), End count read *Place one robot unit on Bake-out plant (BOP).*, `$P end` was
refused with `required first: …`, and the first `R` tapped onto the plant cleared it
(`assign r:plant:0 bake_out` → `rate 0.20 · 2/10 pips`). Screenshot `/tmp/g-3-held.png`.

**Coherent game by count 12 without reading a caption: yes.** Chamber done at MM 6
(clock gone; "First Chamber Pressurised" at 7); bake-out running from MM 4 (rate 0.20,
one plant unit — the minimum, and it stayed there all game); manifest sent at MM 8
(select read *throughput* at 13); RSW-1 landed at 12 (45 t, 8 crew); the KEEP staffed by
the chamber scene's own directive (0/24 with a die at 7) and completed at 29–30; the
alignment staffed by the hold at 12 and completed at 31; the throw staffed by the hold
at 31 and running at 0.63. Water read *nominal* at 44 (it was *below reserve* in
playtest 5). Nobody died. I read no caption and no report body.

Against playtest 5's failures:

| playtest 5 | now |
|---|---|
| robot hold never engaged (bake-out not open) | **fixed** — held at 3, plant visible and marked idle |
| "Solar conjunction" ring at MM 6, unexplained | **fixed for the lessons** — first seen at 36, after `tutorial_open`; still no caption there |
| KEEP opened at MM 8 with no die despite "minuted onto it" | **fixed** — a die on it the same count, 4 pips/month |
| events pane empty on the count after a choice | **not fixed** — see bug 1 |
| bake-out rate 0.00 all game, water to 101 t | **fixed by the hold** — but at the minimum it runs at 0.20 for 44 counts and nothing says that is a fifth of the plant |
| new | a zero-face die sits on the alignment clock beside the rated one for 19 counts; the hold text did not say why the first die did not count |
| new | act-1 texture from 14 on is calmer (13 scenes in 30 counts, four quiet stretches of 2–4) — the "tutorial never ends" feel is gone |

## 3. What the new signals did for a skimmer

- **Idle marking**: decisive. At MM 3 the plant's dim ring and "idle — no dice" made the
  hold's sentence obviously about *that* clock, before I had read the sentence. At MM 30
  the throw clock appeared already dim and marked idle one count before the scene that
  held me for it; a skimmer sees the thing is off before being told.
- **Crawl lines**: seen once (MM 24, "MDLS at 2 pips a month: 3 months to the next
  segment"), in the events pane where a skimmer does glance. It did not make me act — a
  persona rule — but it is the first time the game said a staffed clock was too slow.
  The 0.20 bake-out never got one, because the crawl line is for one-time clocks only;
  "rate 0.20" in grey never changed weight.
- **Carried-forward lines**: **did not appear** (bug 1). Through the driver the events
  pane on the count after a choice was empty every time; the only lines a skimmer saw
  were engine events (completions, arrivals, setbacks, the crawl).

## 4. Bugs (command · state)

1. **The previous count's chronicle line is not carried into the next count.** Fresh
   profile: `$P new 5 tutorial` → `$P choose 1` → `events: ['MM 1. Site handover…']`;
   `$P end` → `events: []`; `$P state` → `events: []`. Same at every count of the run
   (MM 8, 13, 21… all `events: []` after a scene the count before). Either the
   `.chron-line` carry in `advance()` runs before the restored pane exists, or the
   restore from `fortuna-events` does not happen on the reload each driver call makes.
2. **A zero-face die counts as placed but not as "rated".** MM 12: `assign p:1
   align_driver` → hold persists with the same text; `assign p:3 align_driver` → clears.
   Both dice stay on the clock; the `·` one adds nothing for 19 counts. The button
   should say the placed die has no rating here, or refuse the zero die.
3. **Standing clock at a fraction of rate is never called out.** Bake-out at "rate
   0.20 · 2/10 pips" from MM 4 to 44 with one plant unit; no line, no colour. Water
   stayed *nominal* only because nothing shipped until MM 31.
4. **Solar conjunction clock, still uncaptioned** (MM 36–41), now after the lessons, so
   minor.
5. Cosmetic: the page's throw select renders fine; the driver's `sub` concatenates the
   option texts ("shiphold at reservestop") — script, not game.

## 5. Three changes first

1. **page** — Make the carry-forward actually appear: on the count after a choice the
   last chronicle line should be the first thing in the events pane (bug 1). For the
   skimmer it is the only place the consequence of the last click is visible.
2. **engine** — Refuse (or name) a zero-face die on a held clock: the hold text should
   change to "That die has no engineering rating. Place one that does." and the die
   should bounce back to the hand, rather than sitting there as a `·`.
3. **page/engine** — Treat "rate below 0.5 with dice on it" on a standing clock the
   way idle is treated: a half-dim ring and "running at a fifth" in the accent colour,
   plus a six-monthly line like the crawl line for one-time clocks. The impatient
   player will run the plant at the minimum forever unless the screen says so.
