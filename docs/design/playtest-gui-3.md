# GUI playtest 3: the careful reader (v0.1.4, seed 21, tutorial)

Persona: a first-time player who reads every report, every caption and every
recommendation before acting, and does what the mind tells them to do. Driven through
`web/test/play.py --profile c` against the local page; screenshots at MM 1, 4, 8, 12,
18 (alignment complete) and 43 (handoff), all read. Played to the handoff at MM 43 and
two counts beyond. Auto-deal off throughout (the tutorial's default).

## 1. Diary

Understood? = could I tell, from the screen and the report alone and before reading the
caption, what the thing was and what to do with it. Caption? = did the caption add
anything I needed.

| count | revealed | what I did | understood? | caption? |
|---|---|---|---|---|
| 1 | scene column, End count | read; chose *Survey the MDLS* | yes — "This screen is the station log… End count advances the month" is exactly enough | none shown |
| 2 | RSW countdown (after the choice) | *Enter the window on the board* | yes; the ring with "10 left" and "opens at MM 12" agree | no — the caption restates the report |
| 3 | the hand, people only (after the choice) | *Post the free count at the meal* | mostly: dice, faces, dimmed upkeep were clear; "a die is not worth the same everywhere" I only half-believed until MM 13 | no |
| 4 | projects region, KEEP clock | *Open the clock*; dragged Eli Novak (engineering 4) onto the KEEP as told | the drag: yes. The directive-placed die: **no** — Ceri did not appear on the clock until I placed a second die (see bugs) | no |
| 5 | — | *Log it as return inventory* | the roll/carry paragraph landed; "1/24" after 9 pips (two dice) contradicted "one month of one die" | — |
| 6 | robot dice, BOP standing clock (after the choice) | *Task units to the bake-out*; placed 5 units by hand to reach 10 pips | the standing-clock ring as a rate: yes. "a unit taken off upkeep returns a crew die to it": read it, did not grasp the scale until MM 7 | the BOP caption helped: "a rate, not a progress" |
| 7 | ledger, Hours bar | *Post the CED ledger by name* | the hand went 12 → 5 free with no line saying why. Understood only because I had read MM 6 twice | the ledger caption is generic; the Hours "nominal" word was wrong from here on |
| 8 | the ring (four seats) | *Put the KEEP ahead*; placed two more engineers | yes, the cards explain themselves; the ring's own recommendations start appearing only from MM 9 | no |
| 9 | Water bar | *Run a seal survey* | yes | no |
| 10 | manifest select on the RSW clock | *Request a balanced manifest* | yes; "the reply answers the pre-launch request" was clear | no |
| 11 | — | *Read it and table it* | story scene, fine | — |
| 12 | Spares bar; alignment clock announced | *Reconcile the manifest*; tried to staff alignment — **failed**, the clock did not exist until MM 13 | spares: yes. "Project opened: MDLS alignment" — no, it was not on screen | no |
| 13 | alignment clock | act-1 texture (The First Midwinter) fired; placed two dice on alignment | **no** — my two free dice were "·" (engineering 0) on the clock and I had no way to know before dropping them except the hover | the alignment caption ("robot units do not fit") was useful |
| 14 | review countdown, sponsor track | *Send the figure as recorded* | the review paragraph is the best in the tutorial; but my alignment dice were zeros and Ceri had been evicted to upkeep by my over-assignment with no line saying so | no |
| 15 | People bar; KEEP complete | *Keep a surface watch of two*; moved the freed engineers onto alignment | yes | no |
| 16 | Power bar, reactor clock | *Enter the power budget* (titled "MM 15", fired at 16) | yes | reactor caption fine |
| 17 | Margin bar | *Enter the margin* | "patched" as a word: no idea what it meant; the hover explains, the report did not | no |
| 18 | throw clock, throw select, throughput bar | *Set the throw position to Ship* | the position control: yes. **The throw clock had no dice and the mind said "the first pod went down the rail"** — the ring read rate 0.00 for the next twelve counts | no |
| 19 | pressures (one ring) | *Post all three* | one ring shown, three named; fine | the leak caption explained bands, good |
| 20 | contract countdown | *Enter the contract countdown* | yes | no |
| 22 | roster order, auto-deal | *Set the roster order to skill* | **no** — I could not see what the order changed; upkeep had already eaten my three operations-rated crew under "skill" | the auto-deal caption helped more than the scene |
| 25 | — | throw position reprise | yes | — |
| 26, 40 | — | act-1 *The Manifest* scenes | fine, but they duplicate the MM 10 control lesson | — |
| 27 | — | *Record Amal Halloran as a resident* | yes | — |
| 28–29 | — | RSW-2 did not arrive; two missed-window scenes in a row (tutorial's and act 1's) | the first is good; the second reads as the same event again | — |
| 30–42 | — | four act-1 texture scenes, eight quiet counts | quiet counts are legible now that the rail is full | — |
| 43 | everything; the language turns | *Minute it as the count from now on* | yes; the three pressure rings appeared with drifted names, which was the best reveal in the run | no |

## 2. Lessons that landed, and the ones that did not

Landed, from the screen and the report alone: the log and End count; the window as a
countdown; the hand with faces and dimmed upkeep; the project clock and the drag; the
standing ring as a rate; the ring's cards; the manifest control; the review and the
one figure; the KEEP's completion returning dice; the handoff.

Did not land:

- **Upkeep's scale.** MM 6 says "a unit taken off upkeep returns a crew die to it." I
  moved five units and lost seven crew dice (12 → 5 free). No line at MM 7 said "five
  units left upkeep; seven crew now carry it." The Hours bar read *nominal* at 5 of 14
  free and *nominal* at 4 of 21. The bar's word never moved in 45 counts until "thin"
  at 2 of 27.
- **Faces per domain.** MM 3: "Each task reads a different rating, so a die is not
  worth the same everywhere." True and unteachable as written: the hand shows one
  number (the best face), the clock shows another, and the only way to see the
  alignment face before dropping is the hover. Two of my first three alignment
  placements were zeros. The report at MM 4 should say which rating the KEEP reads and
  which dice have it — or the hand should show the face for the clock you are holding.
- **Over-assignment evicts silently.** With 5 free and 6 dice placed, the engine
  returned Ceri Umeh (engineering 5, on the KEEP) to upkeep and left my two zero dice
  on alignment. Nothing said so. A player who does not diff the clocks each count will
  never know.
- **The throw needs dice, and an operations rating.** MM 18: "The first pod went down
  the rail this month… Its dice set tonnes thrown per month." The clock opened empty;
  the mind did not tell me to staff it; my engineers on it read "·" because the throw
  reads *operations*, which none of my free crew had — all three operations-rated crew
  were on upkeep, where dice cannot be taken from. The ratio stayed 0.0 to MM 30,
  the review at 32 was off-nominal, and RSW-2 did not come. The tutorial's central
  economic lesson (the sponsor reads one number) was taught by a failure the tutorial
  caused and never explained.
- **The roster order.** MM 22's options are three words with no visible consequence.
  Under *skill* the engine keeps the highest *any* skill free, which is why the
  operators were eaten; the lesson does not say what "skill" means for a hand with
  several tasks.
- **"patched".** The Margin bar's word appeared with no definition in the report; the
  hover has it.

## 3. Contradictions between report and state

- MM 1: "Sol Hoshino answers for the hours." From MM 8 the hours seat is Vik Chandra;
  Sol Hoshino holds the sponsor seat from MM 12. (The writer's exact-top note; `top =
  true` is not used in the arrival scene.)
- MM 2, 3, 4, 6, 7, 8, 12, 14, 16, 17, 18: every "X enabled/added/opened" report
  describes a region that is not on screen until the option is chosen. The careful
  reader looks for the board the report names and does not find it.
- MM 4/5: "The KEEP clock has taken one month of one die" — two dice were on it (one by
  directive, one by me); the clock read 1/24 from 9 pips.
- MM 12: "Project opened: MDLS alignment." The clock appeared at MM 13; an attempt to
  staff it at MM 12 fails.
- MM 16 scene is titled "MM 15: Power Budget."
- MM 18: "The first pod went down the rail this month" while the MDLS clock showed rate
  0.00 with no dice, and "Ledger: throughput ratio added… the one figure the sponsor
  reads" showed 0.0 for twelve months.
- MM 12 counsel (Cato Oyelaran): "The plant has run under rated since MM 1" — the BOP
  had read rate 1.00 since MM 7.
- MM 4 onward: a "Solar conjunction" countdown appears, unexplained, before any clock
  lesson mentions it; it vanishes by MM 8.
- After the handoff the reveal captions persist on every section ("MDLS position:
  ship, hold at reserve, stop." at MM 43 and after).
- People bar word "steady" (old register) beside "nominal" everywhere else.

## 4. Bugs

- **Directive-placed dice do not render until the next count.** MM 4: after "That die
  is placed on the KEEP clock with this entry", the clock showed `0/24 []`; Ceri
  appeared only when I assigned a second die (`assign p:0 dig_keep`) — the `assign`
  effect updates assignments but not the hand view. Command sequence: `choose 1`,
  `state` → `dig_keep 0/24 []`; `assign p:0 dig_keep` → `['p:0', 'p:9']`.
- **Returning a die to the hand from a project does not work.** `assign p:3 hand` and
  `assign p:6 hand` at MM 14, 15 and 30 left the dice on `align_driver` / `throw` every
  time; `assign <die> <project>` works. Same for `assign p:0 hand` at MM 30 (p:4 stayed
  on the throw, p:0 moved only because I reassigned it elsewhere). Tap-tap onto the
  hand section fails; the engine call is fine.
- **Over-assignment evicts the wrong die silently** (MM 13→14): eviction is by die id
  order, so the best engineer left the KEEP while two zero-face dice stayed on the
  alignment clock.
- **A project opened by a scene is not droppable that count** (`open_project:*` takes
  effect next count): MM 12 `assign p:2 align_driver` → "no such die or project."
- **The Hours bar word is stuck at "nominal"** from 12/14 through 5/14 and 4/21;
  "thin" only at 2/27.
- Two scenes for one missed window (MM 28 `tut_rsw_missed`, MM 29 act-1
  `sponsor_missed_window`).
- The sponsor review at MM 16 left no line in the events (the clock reset 1 → 16 with
  nothing said); the MM 32 review did ("Review: off-nominal").
- Robot units in the hand read "R" with a hover only; the hand caption counts them
  ("11 robot units") three counts before they are revealed at MM 6.

## 5. Layout, from the screenshots

- MM 1 (only the scene): right. The empty rail reads as intentional, not broken.
- MM 4: the rail holds two clocks, two projects and the hand; the KEEP clock shows no
  dice (the bug above), so the first lesson's evidence is missing from the picture.
- MM 8, 12: good — the column of reveal captions in italic gold is the tutorial's
  spine; by MM 12 the rail is the act-1 rail minus four bars. The manifest select
  sits inside the RSW clock's subtitle and is easy to miss.
- MM 18: the throw clock with three empty pips is the most important thing on the
  screen and nothing points at it.
- MM 43: the hand is "0 of 21 free" with every die dimmed and the captions still
  present; the pressures section is below the fold again once the hand has four rows.
  The three drifted pressure names are worth the wait.
- Throughout: "Resumed from the last count." sits in the centre column on every
  count (driver artefact, but a human who reloads sees it too).

## 6. The ten changes I would make first

1. **content** — MM 18: tell the Commander to staff the MDLS, name the rating it reads
   (operations), and name who has it. As written the tutorial's economy lesson fails
   silently.
2. **engine** — when the deal evicts an assigned die, write a line ("Upkeep took Ceri
   Umeh off KEEP excavation.") and evict the lowest face on its clock, not the highest
   die id.
3. **page** — show the face for the clock being hovered or held: when a die is lifted,
   every clock's dice row shows what that die would read there.
4. **engine** — `assign` effects and `open_project:*` take effect in the same count
   (call `refresh_hand` after the effect; open the project before the deal), so
   "placed with this entry" and "project opened" are true on screen.
5. **page** — fix tap-tap onto the hand section (returning a die), and make the
   hand's caption count only revealed dice.
6. **engine/view** — Hours word from the free fraction ("nominal" ≥ 40%, "thin"
   < 25%, "none" at 0), and write a line when the free count falls by more than two.
7. **content** — every "X enabled" report should fire on the count the region appears
   (set the `ui:*` flag in the scene's `when`-time effects, or reveal on the count
   before and let the report describe what is already visible).
8. **content** — MM 22: give the roster order a visible consequence in the same count
   (which names dim) and say what "skill" ranks.
9. **content** — dedupe the missed-window scenes (`tut_rsw_missed` should suppress
   `sponsor_missed_window` with a flag) and fix the MM 15/16 title and the MM 1 hours
   holder (`top = true`).
10. **page** — clear the reveal captions at `tutorial_done`, and explain "Solar
    conjunction" or hide it until `ui:clock:sun`.
