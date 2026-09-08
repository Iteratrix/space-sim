# GUI playtest 4 — the impatient player (v0.1.4, seed 44, tutorial)

Played through the real page with `web/test/play.py --profile d`, option 1 every time
unless breaking something looked more fun, captions never read, every control changed
the moment it appeared. Reached the handoff at MM 43 and stopped at MM 45. Four
screenshots read (MM 4, MM 7, MM 15, MM 30).

## 1. Diary, condensed

| MM | what I did | what the screen did |
|---|---|---|
| 1 | Clicked option 1 ("Survey the MDLS") without reading. Ended. | Empty screen: title, one report, three buttons, End count. Fine. |
| 2 | One option only ("Enter the window on the board"). Clicked it. | A clock appeared. Nothing told me why the click was needed. |
| 3 | Option 1. | Fourteen dice appeared with faces and names. Two greyed. |
| 4 | Option 1 ("Open the clock"). Then shoved all fourteen dice at the KEEP clock. | Twelve landed. The two greyed ones did nothing — no message, no shake, nothing. The log says "One engineering die placed by directive" but the clock showed **0** dice before I started. |
| 5 | Option 1. | KEEP 4/24 with a "bonus" tag. A "Through-wall manipulator fabrication" clock had appeared at MM 4 that no scene mentioned. A "Solar conjunction" ring came and went at MM 4–7 unexplained. |
| 6 | Option 1 ("Task units to the bake-out"). Then put three robots on the KEEP too; tried a haul unit on the through-wall arm (refused, with a sentence); a dex unit on it (accepted). | Log: "autonomous units to the BOP". BOP clock: **0 dice, rate 0.00**. The directive I clicked did not happen. |
| 7 | Option 1. Ended. | Next count the KEEP had 11 dice, not 15; the hand caption went from "1 eating" to "6 eating". Four of my people were pulled off my clock and nothing said so. |
| 8–11 | Option 1 each; the ring appeared (4, then 5, 6 seats); at MM 10 I asked for crew and set the manifest to "people". | Water bar appeared at MM 9 reading **nominal** beside a bake-out at rate 0.00. Manifest select showed "people" immediately — good. |
| 12 | Option 1 at RSW-1. | "KEEP excavation complete." with everyone on it. Seven crew arrived. Relay failed (engine line) and a "Relay repair" clock appeared, unexplained. |
| 13–16 | Option 1; four dice onto MDLS alignment. MM 15 and MM 16 were one-option scenes ("Enter the power budget", "Enter the margin"). | Bars and clocks appeared one at a time. The one-option scenes are a click tax. |
| 17–29 | Option 1 each; set the throw to **stop** the moment its select appeared (MM 19). | At MM 25 a scene I clicked blind put it back to **ship**; no line said so. Hand fell 13→11 at MM 24, no line said why. "leak · tight" ring appeared at MM 21. |
| 28–30 | RSW-2: no arrival. | The clock just restarted at 16. "The Empty Window" scene at MM 30 was the first thing that spoke to it — two counts late. |
| 31–43 | Option 1 each: midwinter, sharp review, the Wrights, a surgery, the manifest, the formulary, RSW-3, handoff. | Act-1 texture arrived in the report register; readable at speed. Handoff line reads well. |
| 45 | Stopped. | BOP still 0.00 after forty-five counts. Throw stopped since MM 19. φ 0.0. Water 335 t, "nominal". |

## 2. Where the game stopped me, and whether the stop was clear

- **Robots on unstructured work**: refused with a clear sentence in the hand note
  ("Through-wall manipulator fabrication does not take haul units"). Good stop.
- **Greyed (upkeep) dice**: not draggable, not tappable, no message. I tried twice and
  concluded the page was broken. Unclear stop.
- **End count while a scene is pending**: the button greys. Clear enough, but there is
  no hint that the scene is the reason.
- **Nothing else stopped me.** Twelve people and three robots on one clock, a plant
  unstaffed for a year and a half, the throw stopped for two years, a crew request
  instead of hardware: every one of these was accepted silently and the bars stayed
  green.

## 3. Could I tell what my actions did without reading?

Yes for: the KEEP ring filling under my pile of dice; the "bonus" tag; the refusal
sentence for robots; the manifest select changing at once; the Hours caption
("6 eating and breathing") — though not *why* it changed.

No for: the four dice evicted from my clock at MM 8 (no line, no animation — the hand
just had different names in it); the throw select reverting at MM 25; the plant at
rate 0.00 for 45 counts with **water: nominal** beside it (the bar reports the tank,
not the plant); the two dice that left the hand at MM 24 (a rotation? nobody said);
the "Relay repair" and "Through-wall manipulator" clocks appearing from nowhere; the
"Solar conjunction" ring flickering in and out. The screen shows *state*; it almost
never shows *change*, and the impatient player only notices change.

## 4. What I broke (commands and state)

1. **The station's die is not placed.** MM 4 option 1 → log "One engineering die
   placed by directive" → `state`: `dig_keep` dice = 0. Either the `assign` effect
   applied before the deal or the page rendered before the refresh; the lesson's
   only demonstration is invisible.
2. **"Task units to the bake-out" tasks nothing.** MM 6 option 1 → log "autonomous
   units to the BOP" → `bake_out` dice = 0, rate 0.00, for the rest of the game.
3. **Upkeep eviction without a line.** `assign r:haul:0 dig_keep` ×3 at MM 7 → next
   count `dig_keep` 15 → 11 dice, hand note "1 eating" → "6 eating". No chronicle
   line, no caption. (Engine behaviour is right; the page is mute.)
4. **Upkeep dice are dead to the pointer.** `assign p:12 dig_keep` → no error, no
   move; on screen they are greyed and inert.
5. **Scene overrides the control silently.** `set throw stop` at MM 19 → MM 25
   option 1 → `controls.throw = ship`. The chronicle says "set to Ship" if you read
   it; the select just changes.
6. **Manifest for crew is honoured but the arrivals wreck the hand.** MM 10 "Request
   crew" → MM 12 "0 out, 7 in" → hand 8 → 14 of 21 free at MM 13, then the eaten
   count climbs every count after. Coherent, but the "people" word on the bar is
   `steady` — the new mood words (nominal/fatigued/degraded/critical) are on the seat
   cards but **not on the People bar**; it still says steady/tired.
7. **Single-option scenes** at MM 2, MM 4, MM 15, MM 16: a forced click each. The
   impatient player reads these as the game lecturing.
8. **Unannounced clocks.** `through_wall_arm` (from MM 4) and `repair_relay` (from
   MM 13) appear because their own conditions opened them; no lesson names them.
9. **The driver artefact.** Every reload prints "Resumed from the last count." into
   the events; harmless for a person, noise for an agent.

## 5. Was the game coherent at the handoff?

Coherent, and it let me get away with everything. Count 45: 27 crew (8 residents),
KEEP dug at MM 12 (the race was trivial with twelve dice on it), water 335 t and
still "nominal" after 45 counts of a dead plant, spares 224, φ **0.0** (12 t shipped
against 340 received) with the throw stopped since MM 19, sponsor "milestone review ·
impatient", one off-nominal review at MM 32, all three pressures quiet/tight. The
world did not lie; it simply never raised its voice. A player who never staffed the
bake-out or the driver reaches the handoff with a green ledger and a sponsor who is
merely "impatient".

## 6. Ten changes, in priority order

1. **page** — Show change, not state: a one-line event per count for anything the
   deal moved ("Upkeep took 4 dice off KEEP excavation"), for a control a scene
   changed ("Throw position → Ship, by SMB directive"), for dice that left the hand.
2. **engine** — A standing clock at rate 0.00 for more than two counts writes a line
   ("BOP idle: no dice assigned. Water not being extracted.") and the Water bar's
   word should be the *flow* word (idle / extracting / shipping), not the stock.
3. **content** — MM 6 and MM 4 must enact their directives with `assign` (or say the
   Commander must do it and show nothing done). A directive the log records and the
   clock contradicts is the fastest way to teach "the text is decoration".
4. **page** — Greyed dice: on click, say why ("Efe is on upkeep this month") instead
   of nothing; or let the click show the roster order control.
5. **engine** — Cap dice per one-time project (three or four) so piling twelve on the
   KEEP is refused with a sentence; the race the plan wants cannot exist otherwise.
6. **content** — Merge the four single-option scenes into their neighbours or give
   them a second option ("Enter it" / "Enter it and flag the number").
7. **page** — Announce every clock that opens by its own condition with the mind's
   one line (the caption table has entries only for lesson projects).
8. **engine** — φ 0.0 for two years with the throw stopped should move the sponsor
   faster than "impatient"; the review at MM 32 was the only consequence of
   stopping the driver.
9. **page** — The People bar still uses steady/tired; switch it to the report
   mood words like the seat cards.
10. **content** — RSW-2 no-arrival needs a same-count line ("RSW-2: no arrival") in
    the scene column, not just the ring restarting; the scene two counts later is
    too late for someone who skims.
