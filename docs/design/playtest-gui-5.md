# GUI playtest 5: the impatient player on onramp v2 (acceptance)

Seed 44, tutorial scenario, local page at build `Onramp v2: ten tutorial scenes…`
(2026-09-11). Persona: read nothing but the scene title and option labels; click
option 1; press End count every count; do only what the held button forces, and the
minimum of it. Driven with `web/test/play.py --profile e`; screenshots at MM 2 (before
and after the hold), MM 4, MM 5, MM 12, MM 18, MM 44.

## 1. Diary

| count | visible | what forced me | what I did | what changed on screen unprompted |
|---|---|---|---|---|
| 1 | scene + End count only | scene pending | chose "Survey the mass driver" | one log line under the scene |
| 2 | one clock "Shelter, first chamber 0/6"; hand of 14 dice (2 dimmed); no ring, no ledger | End count became the sentence *Place one crew die on Shelter, first chamber.* | tapped `4 Ines`, tapped the clock | clock read "0/6 · 4 pips/month" at once; the button went back to End count |
| 3 | same + "MM 3: Autonomous Units" (Haul / Plant) | **nothing** — End count was live after the choice | chose "Haul units first", ended | 11 square `R` dice appeared in the hand; no bake-out clock appeared; ring column appeared with one seat |
| 4 | water bar; ring: 3 seats (one with a recommendation) | scene pending | "Cut under the ridge" | shelter 1/6 |
| 5 | — | nothing | End | shelter 3/6 · bonus |
| 6 | RSW clock and a "Solar conjunction" ring appeared | nothing | End | shelter 5/6 |
| 7 | "First Chamber Pressurised"; shelter clock gone; people bar appeared | scene | option 1 | log: "Shelter, first chamber complete."; my die back in the hand |
| 8 | "Resupply Request, RSW-1"; Shelter (KEEP) 0/24 with 0 dice | scene | "Request throughput hardware" | manifest select appeared on the RSW clock next count reading *throughput*; 4th seat |
| 9–11 | KEEP 0/24, 0 dice; nothing else | nothing | End ×3 | "Relay unit failure" line at 9 |
| 12 | "RSW-1 Arrival"; spares bar; RSW "0 left" | after the choice, End count read *Place one crew die on Mass driver alignment (MDLS).* | tapped `4 Ines`, tapped the alignment clock | alignment 0/8 · 4 pips/month |
| 13 | alignment clock, **bake-out clock (rate 0.00)** and hours bar appeared; hand "14 of 22" | nothing | End | 5th seat |
| 14–19 | act-1 texture scenes every count (The Wrights, A Shift Request, Arms Through the Wall, The Questionnaire, Private Traffic) | scenes | option 1 each | alignment 2/8 → 7/8; at 18: "Bake-out plant (BOP) idle: no dice assigned. / Mass driver (MDLS) idle: no dice assigned." |
| 20 | "MDLS Alignment Complete"; throw clock, position select, throw bar appeared | End count read *Place one crew die on Mass driver (MDLS).* | tapped `4 Ines`, tapped the driver | driver "rate 0.63 · 5/8 pips" with my die and two `R` (the scene had placed them) |
| 21–42 | a texture scene most counts; KEEP 0/24 and BOP 0/10 idle throughout; hand 11–15 free | nothing | option 1, End | power bar at 39; contract and reactor clocks; "idle" line every six counts |
| 43 | "MM 43: RSW-3"; "RSW opened; no arrival." | scene | option 1 | — |
| 44 | full screen: 7 bars, 5 clocks, 3 pressure rings, sponsor dots, 6 seats; captions gone | scene | option 1 | ledger words: water *below reserve*, hours *tight*, throughput *short* |

## 2. Acceptance verdict

**Held at count 2: yes.** End count turned into "Place one crew die on Shelter, first
chamber." and stayed that way until a die was on the clock; `$P end` was refused with
`required first: …`. **Held at count 3: no.** After "Haul units first" End count was live,
no requirement text, no bake-out clock on screen. The robot lesson was silently skipped.
Root cause (engine, not content): the bake-out project is not open during the lesson
phase — `project::open_eligible` skips every condition-opened project except `dig_keep`
until `tutorial_open`, and `bake_out` is condition-opened (no `when`, standing) — so
`tutorial::required` treats the absent project as satisfied and clears the flag the
same tick the scene sets it. The clock first appeared at MM 13 with rate 0.00.

**Coherent game by count 12 without reading a caption: partly.** Yes: shelter chamber
done at MM 7 (one die, two bonus rolls), a manifest sent at MM 8 (the select showed
*throughput* afterwards), RSW-1 landed at MM 12 with 45 t and 8 crew, spares bar
appeared. **No: the bake-out never ran.** Rate 0.00 from MM 13 to the handoff; water
fell from 400 t to 101 t ("below reserve" at MM 46) and nothing but a six-monthly grey
"idle" line said so. φ reached 1.6 against 2.0 only because the throw scene placed two
arm units for me. The KEEP sat at 0/24 for 36 counts. The game is coherent (nobody
died, the sponsor is *satisfied*), but a player who does only what is forced ends the
tutorial with the station's main plant off and its shelter undug.

## 3. What the screen taught unprompted, and what it did not

Taught: End count and the log line (count 1); that a die on a clock makes a number
appear ("4 pips/month") and segments fill (counts 2–6); that a finished clock
disappears and gives the die back (MM 7); that the manifest select on the RSW clock
reflects the option I chose (MM 9); that a required action is a sentence on the
button, not a popup (MM 2, 12, 20); that a placed die shows a number on the clock
immediately (MM 20's "5/8 pips" the same count).

Not taught: what the `R` dice are or where they go (no hold, no fit-badge until you
lift one); that a standing clock at "rate 0.00 · 0/10 pips" means the plant is off —
grey text, same weight as everything else; that the KEEP at "0/24" is *mine* to staff
(the chamber's crew "minuted onto it" in the log, but the clock had no dice); why
"Solar conjunction" appeared at MM 6 with no clock lesson; what "hours" or "people"
bars measure (words never changed from *nominal* until MM 44); that the sponsor
review exists (the clock appeared at 13 with "14 left", never explained).

## 4. Bugs (command · state)

1. **Robot hold never engages.** `$P choose 1` at MM 3 → `required: null, can_end:
   true`; `$P state` projects `['shelter_first']` only; hand shows 11 robots at
   `upkeep: false`. Engine: `open_eligible` lesson gate excludes `bake_out`;
   `required()` clears an unopened target. Fix: open `bake_out` (and only it) at count
   3, or exempt standing projects from the lesson gate, or make an unopened target
   *hold* rather than satisfy.
2. **"Solar conjunction" ring at MM 6** — `countdowns: ['Resupply window (RSW)',
   'Solar conjunction']` — an unexplained clock in the count the RSW clock is being
   introduced; the grace/conjunction pass-through in `app.js` shows it whenever
   `ui:countdowns` is set.
3. **`through_wall_arm`/`repair_relay` are hidden now, but `dig_keep` opened at MM 8
   with no die and no hold**, and the MM 7 chronicle claims "the chamber crew minuted
   onto it". The directive is not enacted (the option's `assign` targets a project
   opened in the same option — the engine change for that exists; content did not use
   it, per the writer's note).
4. **Events pane empty on the count after a choice** — `$P end` at MM 3 → `events: []`
   though the chronicle has MM 2's line; the pane only carries engine lines, so the
   last thing I chose is not on screen when the next count opens.
5. `web/test/play.py`: the second `HELD` retry loop in my driver misidentified the MDLS
   project from the button text (script bug, not the game); the hold itself was
   correct and cleared on `assign p:0 throw`.

## 5. Layout

MM 2 (held): one clock, a hand of 14, a lone italic sentence where the button was —
clean; the sentence reads as an instruction. MM 5: the shelter clock filling under a
single die with "bonus" is the best moment; the ring's three empty-mouthed seats
("nominal", no counsel) are furniture until a scene gives them lines. MM 12: five
seats with counsel, four bars, one clock — the screen is now full and I have read
none of it. MM 18: the hand is four rows of dice plus the roster/auto-deal controls,
pushing the ledger to the fold; the 0/10 bake-out ring is visually identical to the
6/8 alignment ring except for fill. MM 44: everything at once, captions gone, three
grey pressure rings at the bottom edge.

## 6. Five changes first

1. **engine** — Make an unopened `require:*` target hold, not satisfy, and open
   `bake_out` at the robot lesson; the robot hold is the one lesson the impatient
   player cannot dodge, and it is the one that failed.
2. **page** — A standing clock at rate 0.00 with `ui:project:*` revealed should look
   *off*: dim ring, "idle" in the accent colour, and the six-monthly idle line moved
   from the log into the clock's own sub-line every count. The impatient player never
   reads the log.
3. **content** — The chamber-complete scene must `assign` its crew onto `dig_keep`
   (the engine now allows a same-option target) or add `require:place_person:dig_keep`;
   a 0/24 clock with no dice teaches nothing.
4. **page** — Hide the grace/conjunction pass-through until `ui:clock:rsw` has been
   revealed for at least one count, or caption it; "Solar conjunction" at MM 6 was
   the first unexplained thing on the screen.
5. **engine/page** — Carry the previous count's chronicle line into the next count's
   events pane (or keep the last scene's line under the new scene) so a skimmer sees
   what they just did when the next decision arrives.

Outside scope, noted: the act-1 texture density after `tutorial_open` (a scene nearly
every count from 14 to 46) is the opposite problem to the calm-month work and reads
as the tutorial never ending.
