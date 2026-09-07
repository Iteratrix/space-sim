# Playtest 2: the page, dealing by hand

Played the tutorial on the real page through Playwright (`web/test/play.py` logic, one
action per browser session), seed 3 on a fresh profile (my driver copy resolved the
profile under `/tmp`, so the page auto-started seed 3 rather than the seed 11 I asked
for; everything below is seed 3), auto-deal off from count 1, every die placed by hand.
Reached the handoff at count 43 and one count of act 1.

## 1. Diary

| count | hand (free/adults, upkeep) | what I did | projects after the count | notes |
|---|---|---|---|---|
| 1 | 12/14, 2 | MM 1: option 2 (BOP first). auto-deal off. Moved p:9(4), p:12(4) off the alignment onto the KEEP; tried to put the four haul units and two arms on the KEEP | KEEP 3/24 (bonus), alignment 0/8 | Robots never moved: dice sitting on upkeep are rendered `draggable=false`, so they cannot be lifted at all (§4 bug 1). Faces on the KEEP: p:0 3, p:9 4, p:12 4 |
| 2 | 11/14, 3 | MM 2 "Crew Allocation": option 2 (split). Tried robots again | KEEP 5/24, alignment 0/8 · setback | The scene says "put one [robot] on a task and the upkeep it was carrying comes back to a human" — the one lesson the page cannot let you act on |
| 3 | 11/14 | MM 3: splice the dex unit (option 1). p:7 → KEEP | KEEP 7/24 | p:7 shows face 1 on the KEEP |
| 4 | 12/14, 2 | MM 4 manifest: option 3 (capability). p:12 and p:3 → alignment | KEEP 8/24, alignment 2/8 (bonus), BOP 1.0 | p:3 shows `·` on alignment: engineering 0. I had no way to know before moving him |
| 5 | 12/14 | MM 5: read the log (1). p:3 back to BOP; p:8, p:13 → alignment | KEEP 11/24 (bonus), alignment 4/8 (bonus) | p:8 also `·`. Three of my first five moves were blind |
| 6 | 12/14 | quiet. p:8 back to throw | alignment 5/8, KEEP 13/24 (bonus) | throw rate 0.63 while p:8 was away — visible and honest |
| 7 | 12/14 | MM 7 CED ledger: option 1 (KEEP priority). p:2, p:5 (BOP 5 and 3) → KEEP | KEEP 14/24, BOP 0.4 | both `·` on the KEEP. Water still "nominal"; I had cut the plant to 40% and the ledger did not blink |
| 8–9 | 12/14 | p:2, p:5 back to BOP | KEEP 16/24, alignment 6/8 | quiet counts |
| 10 | 12/14 | "MDLS Alignment Complete": option 2 (Hold at reserve). p:12, p:13 freed | KEEP 18/24 | the freed dice are not shown anywhere this count (§4 bug 2). Control still reads `throw: ship` after choosing Hold (§4 bug 4) |
| 11 | 12/14 | p:12, p:13 → KEEP (took effect next count) | KEEP 19/24 | |
| 12 | 12/14 | RSW-1: option 2 (unpack). 7 crew in, 43 t, 30 t capability | KEEP 22/24 (bonus) | option 3's text says "The MDLS is not aligned" two counts after it was aligned; the arrival is 70% capability hardware although the scene says RSW-1's manifest was fixed before we launched |
| 13 | 12/21, 9 | KEEP complete; everyone down (only option) | BOP 1.2, throw 1.0 | mean CED 206 mSv at count 13. All seven arrivals are eaten by upkeep; the five KEEP dice vanish from the screen for a count |
| 14–16 | 12/21 | quiet; review at 16: honest number (φ 0.7 vs 1.0, sponsor "impatient") | | assignments in these counts were lost — see §4 bug 3 (clicks on hand dice land on the name label) |
| 17 | 12/21 | after switching the driver to JS clicks: p:9, p:12 → through-wall arm; p:7, p:13 → BOP | arm 2/16 (bonus), BOP 1.3 | p:7 is face 1 on the BOP and was face 5 in the hand; p:13 is `·` on the BOP and 4 in the hand |
| 18–26 | 12→10/21, 9→11 | quiet ×9. The arm from 5/16 to 14/16 with two 4s | φ 1.1 → 1.8 | one robot died silently (12 → 11 units); the hand shrank by two; "hours: nominal" throughout |
| 27 | 10/21 | MM 27 contract: option 1 (Amal stays) | arm 15/16 | four of six seats "tired" |
| 28 | 10/28, 18 | arm complete; RSW-2: option 1 (units, not crew). 8 in, 1 out | | the arm's completion put nothing visible in the hand or the fleet |
| 29–42 | 10→9/28, 18→19 | quiet ×14, nothing to do but End count | φ 1.9 → 3.2, "satisfied" | fourteen counts, no scene, no pressure moved, no clock urgent |
| 43 | 9/28 | RSW-3 handoff: option 1 (minute it as the count). 15 in, 7 out | BOP **0.10** with one die | the rotation took my whole plant crew home and nobody re-dealt; nine 4-face dice sit idle in the hand |
| 44 | 12/36, 24 | act 1's "Arms Through the Wall" fires at once | | "there are no more coming" — sixteen counts after we built one |

## 2. Is dealing by hand a real decision?

Half of it is, and it is the good half.

**What came through.** Faces set pace in a way you can count: three 1s on the KEEP was
3 pips a month and hopeless; swapping in two 4s made it 11 pips and the KEEP finished at
13, one count after the first RSW. The race the tutorial promises — dig before the
window — is real and I won it by a count. Pulling p:8 off the throw dropped the rate to
0.63 that same count; putting the plant's two best on the KEEP dropped the BOP to 0.4.
The one roll per project is legible: `· bonus` and `· setback` sit on the clock, and the
bonus fired often enough with 4-face dice (five bonuses in the first twelve counts) that I
started wanting the 4s on the long clocks for the extra segment, not just the pips. "12 of
14 free · 2 eating and breathing" is the right caption, and watching it go to "10 of 28 ·
18 eating" as the sponsor sent people is the labour research made visible: the crew
tripled and the hand shrank.

**What did not.** Robots never entered the game: they are the square dice the design is
about and the page will not let you touch one (§4). Strain dulling never happened in 44
counts — no die was ever half-shaded — so I cannot say whether it reads. Upkeep eating
people came through only as a count; *who* is eaten is decided by the roster order and
the page does not say so, and the eaten dice look identical to robots on upkeep. And the
faces lie across domains: a die in the hand shows its engineering face (p:7 "5"), on the
plant it shows extraction (p:7 "1"), and there is no way to see a person's faces before
you move them. Three of my first five moves placed a `·` die on a project. The decision
the design wants — "Okonkwo is a 4 in engineering and strained, and six counts on the
bearing with Varga builds a tie" — is not on the screen; the screen shows one number per
die that changes meaning depending on where it sits.

**Could I tell what my assignments changed?** Yes for projects and rates, next count. No
for anything downstream: the plant at 40% for a count showed as "water: nominal"; the plant
at 10% after the RSW-3 rotation showed as "water: nominal" with a why-sentence about the
closure gap that did not mention production. The bars report stocks, not flows, so the
consequence of a bad deal arrives as a stock crossing a word three or four counts later.

## 3. Where the game lied or confused me

- "MDLS to Hold: ship only what the plant can spare" — I chose it at count 10; the throw
  control kept reading `ship`, the chronicle says "MDLS position: Hold at reserve". One of
  them is wrong (the control: the scene sets a flag the engine does not read).
- RSW-1 (count 12): the scene says the manifest "was fixed before this crew launched"; the
  arrival was 30 t of 43 capability hardware, i.e. exactly my count-4 request. And option
  3 reads "The MDLS is not aligned" two counts after the alignment scene.
- "Arms Through the Wall" at count 44: "the two through-wall manipulators … are wearing and
  there are no more coming" — the tutorial had me build one at count 28 and its completion
  is in the chronicle above it.
- "hours: nominal" while the hand went from 12 of 14 to 9 of 28 and the plant crew went
  home. The word never changed in 44 counts; neither did "margin: patched" or any pressure.
- Faces in the hand are engineering faces; nothing says so. p:7 is "5" in the hand and "1"
  on the plant.
- The countdown label under the RSW clock reads "11 left · manifest throughputba…" because
  the select's option text is inlined into the caption (visible in `state`, not on screen).
- The events column after a reload says "Resumed from the last count." above the real
  chronicle line, every count, forever.
- Solar cycle: the ring is empty with no caption; "Quiet Sun: galactic dose at its highest"
  is a hover I only found through the driver.

## 4. Bugs

1. **Robots on upkeep cannot be assigned** (page). `state` at count 1: every `r:*` die has
   `upkeep: true`, and the die template sets `draggable="false"` for upkeep, and
   `bindDice` binds click and dragstart only to `.die[draggable=true]`. So no robot can be
   lifted or dragged, ever, and `$P assign r:haul:0 dig_keep` at counts 1, 2, 4 changed
   nothing. The engine accepts the move (`assign` on the wasm side) — it is purely the
   page's gating. Human dice eaten by upkeep are correctly refused by the engine ("that
   person is eaten by upkeep this count"); robots are not eaten, they are covering.
2. **Dice returned by a completed project disappear for a count** (page/engine).
   Count 10, after "MDLS Alignment Complete": p:12 and p:13 are in neither the hand nor any
   project in `state`; count 13, after the KEEP completed, five dice (p:0, p:7, p:9, p:12,
   p:13) are gone from the rail. The view's `hand.dice` carries the *deal's* placement
   (`place: "align_driver"`) and the page renders only `hand`/`upkeep` dice in the hand and
   project dice under existing clocks. They reappear next count. During that count they
   cannot be assigned (`assign p:12 dig_keep` → "no such die or project").
3. **Real clicks on hand dice do not lift them** (page). A Playwright `page.click` on
   `.die[data-die='p:7']` reports `elementFromPoint` = the inner `<small>` name label, the
   click is delivered (a capture listener counts it) but the die's handler does not set
   `lifted`; `element.click()` from JS does. Tap-tap from the hand therefore fails for a
   human at a pointer; drag_and_drop of a hand die onto a clock also did nothing. Dice
   already on a project (no name label rendered) tap-tap fine. Everything I assigned from
   the hand in counts 1–16 was lost this way; I switched my driver to JS clicks at 17.
4. **Scene choices do not set the standing controls** (engine/content). "MDLS to Hold"
   (count 10) and "Free allocation to the BOP; robots stay on upkeep" (count 2) are written
   as if they move dice or the throw position; neither does. Only the manifest flags are
   read by the engine.
5. **`assign` does not re-deal** (engine). After `assign`, the returned view's dice still
   show the old placement; the move shows at the next count. With the page it means the
   die you dragged snaps back to where it was until you press End count.
6. **`assign`/`set` are not persisted** (page). Only `advance` and `resolve` write the save.
   Reload after dealing and the deal is gone; `set auto_deal off` followed by a reload is
   back to on. (This is what makes the driver's one-session-per-action model lose work;
   it will also bite a phone user whose tab is discarded.)
7. **The rotation strips projects without a re-deal** (engine). Count 43: seven people out,
   among them the whole BOP crew; the plant fell to 0.10 with one die, nine free 4-face
   dice sat in the hand, no event said so. Auto-deal off means the player must notice.
8. **Through-wall completion adds no unit** (engine). `robots.through_wall` stayed at 0
   after "Through-wall manipulator fabrication complete" (robot units 11 → 10 → 13 only via
   attrition and RSW-3).
9. Fourteen scene-free counts (29–42) inside the tutorial (content): the `tutorial` flag
   suppresses everything below priority 100 and nothing is scripted between RSW-2 and the
   handoff.
10. Cosmetic: the RSW countdown caption concatenates the select's options; "Resumed from
    the last count." persists in the events column; the solar-cycle clock has no caption.

## 5. Layout

Three screenshots (counts 3, 13, 44). The rail reads well: segmented rings, the dice row
under each project, the hand as a grid with names — at 36 people the hand is six rows and
still fits above the ledger. The scene column is quiet and the serif is right; the ring's
counsel in italics with "favours:" under it is exactly the design. Specific problems:
- The hand's dice are visually two kinds (bright = free, faded = upkeep) but the caption
  "12 of 36 free" counts dice that are on projects, so the hand often shows *no* bright die
  while claiming twelve free. Say "12 free, 9 placed, 3 in hand" or draw placed dice as
  ghosts in the hand.
- Robots and eaten people look the same (faded "R" and faded "2 Zev"); the design wants
  robots as squares that carry upkeep, distinct from people upkeep has eaten.
- Empty rings (Solar cycle, Reactor core life at 149/150) look broken rather than "not yet".
- The ledger sits below the fold at 1400×900 once the hand is six rows; the pressures and
  the sponsor's dots are never in view without scrolling — the two things the design says
  should always be visible.
- "MM 3: Non-Responsive Units" renders as a plain h2 though it is a must scene; every
  tutorial scene is priority ≥ 100, so the "must" styling is meaningless in the tutorial.
- Generated counsel ("Leans toward "…"") is visually identical to authored counsel, as
  designed, but in count 44 four of six seats said exactly the same generated sentence.

## 6. Ten changes, in order

1. **page** — Make robots on upkeep liftable and draggable; render them as squares in their
   own row with "carrying upkeep" as the hover, and refuse only what the engine refuses.
2. **engine** — `assign` re-runs the deal (or the view derives placement from
   `assignments`), so a move shows at once; and dice from a project that completed this
   count are placed in the hand in `hand.dice`.
3. **page** — Persist after `assign` and `set`, not only after `advance`/`resolve`.
4. **page** — Show every face a person has: on hover, a five-cell strip (E L X M A…) with the
   project-domain face highlighted; in the hand, show the face for the *selected* project
   (the roster-order control's "for the selected project" mode from the GUI study).
5. **engine/content** — Let scenes move dice and set controls: an `assign` effect
   (`assign = { role = "hulls", project = "dig_keep" }`) and a `control` effect
   (`throw = "hold"`), and use them in MM 2, "MDLS Alignment Complete", and the RSW scenes.
6. **engine** — A "crew changed" re-deal event: when a rotation, death or grounding empties
   a project's dice, write a chronicle line ("BOP crew rotated out; plant at 10%") and, if
   auto-deal is off, mark the project clock as needing hands.
7. **page** — Bars report flows, not stocks: water's why-sentence should say "making 1.4 t,
   losing 2.5 t" and its word should turn when net is negative, not when the tank crosses
   a line three counts later. Same for hours: "short" when the hand shrinks.
8. **content** — Fix the three lies: the Hold option, RSW-1's "fixed before launch" against
   a manifest that honoured my request (either make RSW-1 ignore the split, or drop the
   sentence), and "no more coming" when the outpost has just built one. Add two or three
   quiet-count scenes between RSW-2 and the handoff (a dose-ledger reading, a plant fault,
   the first strained die), and gate the act-1 "Arms Through the Wall" on the arm not having
   been built.
9. **page** — Move the pressures and the sponsor's dots to the top of the rail above the
   hand, or into the header strip; they were never on screen.
10. **engine** — The through-wall project's completion should add a unit
    (`robots.through_wall += 1`) and the project should reopen with more segments, so the
    outpost's first robot is a thing you can see in the hand.

Outside scope, noted: the tutorial's scene texts refer to dragging "a line onto a task"
while the page calls them dice; pick one word.
