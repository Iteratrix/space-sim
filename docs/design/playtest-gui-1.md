# Playtest 1: the tutorial in the browser (seed 7, auto-deal on)

Played through `web/test/play.py` against the local page, 43 counts, from the
arrival to the handoff scene. Choices made on the merits and on the ring's counsel.
Screenshots at counts 1, 4, 12 and 31. Everything quoted is the game's text.

## 1. Diary

- **MM 1, Site Handover.** Six seats already sit on the ring while the scene says
  "Idris Chandra and Nils Villanueva constitute the SMB as of this entry. The other
  seats fill as their questions come up." Chose the MDLS survey (the Chief Engineer
  wanted to see the rails; the Air seat agreed). Hand read "12 of 14 free" but every
  free die was already on a project and three projects had progress (1/8, 1/24)
  before I had decided anything.
- **MM 2, Crew Allocation.** The scene teaches the allocation board — "Drag a line
  onto a task and it stays there until the SMB moves it" — and offers three
  allocations. The hand had dropped to 6 of 14 free between counts with no action of
  mine. Chose the split (Hours, Bodies, Air). The chronicle then said "MDLS alignment
  unstaffed" while the driver clock showed a `5` on it and went 1/8 → 3/8.
- **MM 3, Non-Responsive Units.** Good scene; the counsel is the best of the arc
  ("A dexterous unit is three hundred hours a month. A splice is four hours and a
  signature. Sign it."). Chose the splice. Zara Saito became a name I remembered.
- **MM 4, Resupply Request.** Chose capability hardware (Hulls, Air). The manifest
  control on the RSW clock stayed on "balanced" until the convoy landed eight counts
  later, when it flipped to "capability" by itself.
- **MM 5, the mind's log.** Chose to read it and table it. Strong scene; the
  liaison's "Records are read" is the right chill.
- **MM 6, MDLS Alignment Complete.** Fired the count after MM 5 with no allocation
  on my part (auto-deal). Chose Hold at reserve. The throw control on the project
  stayed on "ship"; the chronicle says "MDLS position: Hold at reserve".
- **MM 7, CED Ledger.** Chose the KEEP directive. The chronicle says "BOP and MDLS
  allocations reduced"; no die moved.
- **MM 8–11.** Quiet. The KEEP clock filled steadily (9, 12, 14, 17 of 24) with one
  "bonus". This is the game working: I watched a clock I cared about fill.
- **MM 12, RSW-1.** "RSW arrival: 43 t landed, 30 t capability hardware" — my MM 4
  request, which the scene insists "was for RSW-2", had arrived on RSW-1. Option 3
  said "The MDLS is not aligned"; it had been aligned since MM 6. Chose the inventory.
  Seven arrived; the hand's caption stayed "4 of 14" for a count.
- **MM 14, KEEP complete.** Fired two counts after 20/24. Chose everyone down. The
  best chronicle line of the run: "Keep pressurised count 14. All crew relocated to
  drum. Surface habitat reclassified as storage and STE shelter."
- **MM 16, First Review.** The review clock already read "16 left" — the review had
  happened this count before the scene about preparing the packet. Chose the true
  number.
- **MM 17–26.** Ten quiet counts. Through-wall arm completed at 23 as a single event
  line. φ climbed 1.2 → 2.2 past an expectation of 1.4; "throw" went "on target".
- **MM 27, First Contract Expiry.** Zara Saito — the Wright — wants to stay. Chose to
  write her down as staying. Exactly the right person for the scene.
- **MM 28.** "RSW opened; no arrival." Nothing else. No scene, no counsel, no
  explanation. The MM 27 scene had just promised "home on RSW-2" and "the next hull
  brings the replacement". The sponsor's mood word turned "impatient" while φ was 2.4
  against 1.4.
- **MM 29–42.** Fourteen quiet counts. The hand fell to 2 of 21; "hours" went
  "thin". The "Next contract end" clock hit "0 left" (urgent, gold) at count 31 with
  no scene and no consequence I could see. Nothing to decide for over a year.
- **MM 43, RSW-3.** A convoy landed ("9 out, 17 in"), three seats changed holders
  silently (Hours, Air, Liaison), and the handoff scene fired. Chose to minute the
  count. "This is the log now, and it is ours." Good ending to the arc.

## 2. Where the game lied to me or lost me

1. **The ring is full on count 1** while the text says two seats exist and "the
   other seats fill as their questions come up." The most-promised beat of the
   tutorial (seats appearing) never happens; the column never changes shape.
2. **Allocation scenes describe dice moving that do not move.** MM 2 ("Free
   allocation split: BOP and KEEP"), MM 7 ("BOP and MDLS allocations reduced") — the
   projects' dice were identical before and after. With auto-deal on, the tutorial's
   central lesson is narrated but never enacted, and the hand shrinks from 12 to 6 to
   3 to 4 free across the first four counts with no visible cause.
3. **The two standing controls lag the choice.** Choosing "Capability" at MM 4 left
   the manifest select on "balanced" for eight counts; choosing "Hold at reserve" at
   MM 6 left the throw select on "ship" for the rest of the run. Either the scene sets
   the control and the select should show it, or the scene should not claim to.
4. **RSW-1 delivered the RSW-2 request.** The scene's whole point ("every request is
   answered one window late") is contradicted by the event line on the same screen
   ("30 t capability hardware").
5. **Stale text after completions.** "The MDLS is not aligned" (MM 12) six counts
   after "MDLS Alignment Complete"; "The plant has been at a third for a year" while
   the bake-out clock reads rate 1.00.
6. **A missed window with no scene.** MM 28 is the single most consequential event
   of the run and it is one grey line. The tutorial has a fallback for RSW-1 missing
   but nothing for RSW-2, and the MM 28 crew-augmentation scene simply never fires.
7. **The review scene fires after the review.** At MM 16 the packet is being
   "prepared" while the review countdown has already reset to 16.
8. **Seats change hands with no word.** At MM 43 three of six holders were new
   names. A KoDP player would expect the ring to say who left.
9. **The sponsor's mood contradicts the ratio.** "impatient" at MM 28–31 with φ
   2.4–2.8 against 1.4; back to "satisfied" at MM 32 with nothing I did. The mood
   tracks attention, which the screen never shows.
10. **Solar conjunction sat on the rail for MM 4–7** while the liaison said "the
    board will read the request in nine minutes and reply in forty."
11. **Numbers I wanted and could not find:** water in tonnes (only "nominal"), the
    dose rate now versus the ledger, how many people arrived and what they are good
    at, why the hand shrank, what "patched" means for margin. Hovers exist in the
    page but the state JSON exposed only the `why` strings, which are good sentences
    — they should be visible without hovering at least once.
12. **Nothing to do for 25 of 43 counts.** The quiet counts are right in principle
    (§3 of the GUI study) but here they are empty: no event line, no clock crossing a
    band, no sentence. The centre column shows only "Resumed from the last count."
    after any reload.

## 3. Bugs

- `choose` at MM 5: the chronicle line appeared in the chronicle but the events
  column showed "Resumed from the last count." above it (cosmetic; the resume line
  is printed on every reload, which the driver does every call — a human reloading
  sees the same).
- The throw select does not reflect the storylet's "Hold at reserve" (`CONTROLS:
  throw ship` after the MM 6 chronicle line said Hold). Either the effect is a flag
  the engine does not read into `controls.throw`, or the page does not re-render the
  select after resolve.
- The manifest select flips to the requested value only when the convoy consumes
  the flag (MM 12), not when the option is chosen (MM 4).
- Hand dice for new arrivals all show `·` (face 0) — seven people arrived at MM 12
  with skills 2–5 in something, but the hand renders their face for the engineering
  domain (the default when unassigned), so the whole cohort looks useless.
- The hand caption ("4 of 14 free") did not update the count of adults on the count
  the seven arrived; it read 14 at MM 12 and 21 at MM 13.
- "Next contract end: 0 left" stays on the rail, gold and urgent, for several counts
  with no scene and no visible consequence (MM 31, screenshot 4).
- The throw project's subtitle in the DOM concatenates the select's option text
  ("8/8 pips shiphold at reservestop") — a driver/page markup issue, invisible in the
  rendered screen but it means the select text is inside the `.sub` span.
- The project titles in the rail drift with the lexicon while the scene text does
  not: at MM 12 the rail reads "Keep excavation" and "driver operations (mass
  driver)" beside a scene that says "KEEP" and "MDLS". Intended for the chronicle;
  odd on the chrome mid-tutorial. "driver operations (mass driver)" is a bad phrase
  in any register.

## 4. The layout, from the screenshots

Good: the three columns read at once; the segmented clocks are legible and the
gold-filled arc against grey is the right amount of information; project clocks
with their dice rows underneath are the best thing on the screen — at MM 8–11 I was
watching a clock fill, which is the game. The serif scene text and the boxed options
with a gold left edge for "favoured" work. The ring cards with the question as the
label and the holder's sentence in italics are exactly the KoDP feeling, and the
authored lines ("Hands are not free hours; they are hours with a mouth") carry it.

Bad: at 1400×900 the ledger runs off the bottom of the rail — pressures and the
sponsor's dots are below the fold in every screenshot, so the three pressures I was
told to watch were never visible. The centre column is mostly empty on quiet counts
(a single grey line and a button), which makes 25 of 43 counts look like nothing.
Generated counsel ("Leans toward "Full inventory before anything is unpacked".")
is typographically identical to authored counsel but reads as filler; four of six
seats saying it makes the ring feel like a poll. The hand renders as a row of
identical boxes of `·` and `R` with no names — without hovering, the dice are
anonymous. "Solar conjunction" as a full gold ring with no number is alarming and
meaningless. The `New` and `Chronicle` buttons are fine; there is no way to see the
polity summary or a person.

## 5. Ten changes, in priority order

1. **engine** — The MM 28 miss must produce a scene; more generally, any missed RSW
   after the first should fire a "no arrival" storylet with the liaison's line and a
   manifest consequence. The tutorial's RSW-2 scene should have a fallback like RSW-1's.
2. **engine/page** — Make storylet effects on the standing controls real and visible:
   `manifest_*` flags set `controls.manifest` at resolve time (not at the convoy) and
   a `throw_*` flag/effect sets `controls.throw`; the page re-renders the selects
   after `resolve`.
3. **content** — Tutorial allocation scenes (MM 2, MM 7) should either move dice (an
   `assign` effect, or open/close projects so auto-deal re-deals) or stop claiming
   to. With auto-deal on, MM 2 should say so: "the board is dealing the free lines
   itself; open the board to deal by hand."
4. **content** — Seats should appear as promised: start the tutorial with two seats
   (Hulls, Hours) and have MM 3 seat Extraction, MM 5 seat Operations, MM 7 seat
   Bodies, KEEP completion seat Air, MM 12 seat the Liaison. This needs an engine
   flag per seat (`seat_open:hulls`) that `Seat::exists` honours in the tutorial.
5. **content** — Fix the stale claims: RSW-1 text vs the capability event (make the
   MM 4 request genuinely arrive at RSW-2 by having the engine keep a queued
   manifest one window deep); "The MDLS is not aligned" gated on `not_flag
   driver_aligned`; "at a third for a year" gated on the bake-out rate.
6. **page** — Show the pressures and the sponsor's dots above the fold: move them
   under the hand or make the ledger a second column of the rail. Nothing the player
   is told to watch should need scrolling.
7. **page** — Dice need a name: initial under the die, full name on hover, and the
   face shown for the person's *best* skill when unassigned (with the domain letter).
   Cohorts of `·` read as useless people.
8. **engine** — A seat changing hands writes a chronicle line and the ring card
   marks the new holder for one count. The "Next contract end: 0" clock should fire a
   scene or vanish; a clock at zero with nothing happening is a lie.
9. **page/engine** — Quiet counts should show something: the count's engine events
   (there were none for 25 counts), the pressure band and the clocks nearest to
   turning, or one generated sentence ("The KEEP is four segments from done. The
   window is in three."). And drop "Resumed from the last count." as the centre's
   only content after a reload — show the last chronicle line instead.
10. **content** — Cut generated "Leans toward" lines when three or more seats would
    say only that: show the favoured option on the card with no quote, so authored
    counsel stands out and the ring stops reading as a poll. Add authored lines for
    Extraction and Air in MM 1, MM 5, MM 12 and MM 14, where they only lean.

Outside my directive, noted: the review firing after its own clock (MM 16) and the
KEEP completion landing two counts after 20/24 suggest the completion scenes fire on
the count after the flag, which is fine, but the countdown for the review should
show the *upcoming* review on the count its scene fires.
