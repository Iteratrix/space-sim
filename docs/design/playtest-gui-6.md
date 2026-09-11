# Playtest 6 — onramp v2, the careful reader (acceptance)

Seed 21, tutorial scenario, the live page at 127.0.0.1:8765 driven through
`web/test/play.py --profile f`. I read every report, caption and recommendation
before acting, did what the mind recommended, and placed dice by hand from count 2
(auto-deal off). Played to count 46; the handoff fired at count 43. Screenshots at
MM 1, MM 2 (after the board opened), MM 4, MM 8, MM 12 and count 46.

## 1. Diary

| count | revealed (new on screen) | understood without the caption? | caption needed? |
|---|---|---|---|
| 1 | Scene, two options, End count (disabled until chosen). Nothing else. | Yes. Choose, the log line appears, End count enables. | No caption shown; none needed. |
| 2 | On choosing: the hand (12 dice with numerals and first names, two dimmed) and one clock, "Shelter, first chamber 0/6". End count replaced by *Place one crew die on Shelter, first chamber.* | Yes. The button told me what to do; the report named who. Dragging Ceri showed "0/6 · 5 pips/month" instantly; Eli made it 9. | The two captions ("one-time clock… six segments", "the face is skill in the task's domain") added the word *segments*; the report had already said it. Not needed. |
| 3 | Robot dice (11 "R" squares) in the hand. **No bake-out clock.** The report said "Place one unit on the plant"; the button never held; `assign r:haul:0 bake_out` → "no such die or project". | I understood what a unit was. I could not do the thing. | The robot caption appeared; three others (ledger, SMB, shelter) appeared this count with nothing to attach to. |
| 4 | Water bar ("nominal ▲4"); the ring with one seat (water and metal) giving counsel; the chamber at 4/6 with a *bonus* mark. | Yes: a card, a name, a sentence, "favours: …". The bonus mark I inferred from the pips jumping. | Ring caption restated the report. Ledger caption ("bars show stock") was fine but early: one bar, one word. |
| 5 | Chamber complete (event line). Scene: dose halved, "Shelter (KEEP) 0/24" opens; the People bar. Hulls and hours seats appear (set at count 4). | Yes. **But** the option "Direct the chamber crew onto the KEEP" and its log line "chamber crew minuted onto it" left the KEEP at `0/24 []` with Ceri and Eli back in the hand. I placed them myself. | Caption "Shelter (KEEP): the full excavation. Twenty-four segments." — fine, short. |
| 6 | RSW countdown ("6 left") and a **"Solar conjunction"** clock with no explanation. | RSW yes (caption + label). Conjunction: no idea what it was until count 39's scene. | RSW caption useful. Conjunction has none. |
| 7 | — | | |
| 8 | Manifest scene; the manifest select on the RSW clock; the liaison seat. Text: "New seat: liaison, held by Vik Chandra." Ring: **Sol Hoshino**. | The control: yes (a select with four words). The seat: confused by the wrong name. | Manifest caption restated the report. |
| 9–11 | Nothing. Quiet counts; End count only. | — | — |
| 12 | RSW-1 lands (event). Spares bar ("nominal ▲6"); "Mass driver alignment (MDLS) 0/8" opens droppable; bodies seat; button held: *Place one crew die on Mass driver alignment (MDLS).* Eight arrivals. | I placed the highest number in the hand (Ada, 5). On the clock it read **3** — the hand shows the best face in any domain, the clock reads engineering. The report said "engineering" once; I still picked by the big number. | Alignment caption ("only dex units fit") contradicts nothing now. |
| 13 | Bake-out clock finally appears (rate 0.00, empty); Hours bar; roster order and auto-deal controls; "BOP" caption; act-1 scene *Private Traffic*. | The plant: yes, I staffed it with 3 haul + 2 plant and it read 1.00. The roster/auto-deal controls: two selects with no consequence I could see. | BOP caption was the count-3 lesson arriving ten counts late. |
| 14–19 | Sponsor review clock at 16 (no caption); act-1 scenes every count (Wrights, Phasing, Arms, Shift, Questionnaire). The alignment at 1 pip/month (I had Wale, engineering 1, on it). | Review clock: label enough. | None shown for it. |
| 20 | KEEP complete; its dice return. | Yes — event line and the clock disappearing. | — |
| 21–25 | Calm scenes; "Next contract end" clock at 26 (no caption). | Yes from the label. | — |
| 26–42 | RSW-2 lands at 27 (2 out, 9 in); review off-nominal at 31; conjunction at 37–42; **alignment stuck at 5/8 with no dice from 26 to 46** after my restaff silently failed; "Mass driver (MDLS) idle: no dice assigned" logged at 18, 24, 29, 35, 41 for a clock I could not see. | — | — |
| 43 | Handoff. Full screen: 7 bars (margin *patched*, throw *short*), 5 clocks, 4 projects incl. an invisible-until-now "Mass driver (driver)" at rate 0.00 and "Relay repair", 6 seats, the sponsor track, three pressures. Captions cleared. | The reveal itself was legible. | — |

## 2. Acceptance verdict

**Words read before the game opened (end of count 3):** MM 1 ≈ 100 (58 report + 41
options), MM 2 ≈ 135 (55 + 48 + two captions 32), MM 3 ≈ 105 (58 + 36 + one caption
9). **≈ 340 words**, of which ≈ 40 were captions and ≈ 125 were option descriptions.

**Before RSW-1 (end of count 11):** + MM 4 ≈ 200 (60 + 60 + one counsel 55 + two
captions 24), MM 5 ≈ 185 (57 + 40 + three counsel 80 + caption 8), MM 6 caption 13,
MM 8 ≈ 235 (75 + 60 + three counsel 90 + caption 10). **≈ 975 words**, ≈ 100 of them
captions.

Playtest 3 (v1, same reader) needed twenty briefings of 60–140 words each plus
captions to reach the same point — on the order of 2,500–3,000 words by RSW-1 and
five required "click to continue" scenes. v2 is roughly a third of the reading and
every count that has a scene has a decision. **Same game, fewer words: yes.**

**But the acceptance criterion "held at counts 2 and 3" is half met.** Count 2 held
and taught. Count 3 could not hold because the bake-out clock did not exist
(`open_eligible` skips condition-opened projects before `tutorial_open`, and the
bake-out is one); the requirement `require:place_robot:bake_out` counted as satisfied
because the project was not open. The robot lesson is void, and the plant sits idle
until count 13. A player who follows the mind at count 3 is told to do something the
screen cannot do.

## 3. Reveals: right time, early, late, never

| element | expected | actual | verdict |
|---|---|---|---|
| chronicle button | after the first log line | count 2 | right |
| hand, first clock | count 2 | count 2 | right |
| robots | count 3 | count 3 | right (but no clock to use them on) |
| bake-out clock | count 3 | count 13 | **ten counts late** |
| ring | first counsel (count 4) | count 4 (extraction seat only; hulls/hours at 5) | right |
| water bar | first move | count 4, "▲4" with the plant idle (the delta was consumption noise) | slightly early |
| people bar | shelter chamber | count 5 | right |
| RSW clock | window ≤ 6 | count 6 | right |
| **solar conjunction clock** | never in the tutorial | counts 6 and 37–42, no caption | **early and unexplained** |
| manifest control | manifest scene | count 8 | right |
| spares bar | first convoy | count 12 | right |
| alignment clock | RSW-1 | count 12, droppable the same count | right |
| hours bar, roster, auto-deal | crew ≥ 20 | count 13 | right by rule; nothing to do with them |
| review clock | first review | count 16, no caption, **no sponsor track** (gated on the same flag; not visible until 43) | half |
| contract clock | contract ≤ 4 | count 26, no caption | right |
| throw clock, position, φ bar | alignment | **never** (alignment never completed) | never |
| margin, power, reactor, Sun bars/clocks | when low/tight/near | none triggered; all appeared at 43 via `ui:all` | fine |
| pressures | first band > 0 | none moved; all three at 43 | fine |

## 4. Lessons that did not land

- **The throw.** The alignment requirement was satisfied by any crew die, and mine
  was a face-3 (then a face-1) engineer. Nothing after count 12 says "this clock is
  slow"; the log says "Mass driver (MDLS) idle: no dice assigned" — about the *other*
  clock, which I could not see. The alignment never completed, the throw lesson
  never fired, φ read *short* at the handoff. The central economic mechanic was not
  taught.
- **Faces per domain.** MM 2: "the face is their rating in the clock's domain" is
  true of a die *on* a clock, but the hand shows the best face in any domain (Ada
  "5" is life support 5, engineering 3). The lifted-die badge would have told me
  (the page shows "→ 3" beside each clock when a die is lifted) — but the report
  never mentions the badge, so the careful reader has no reason to lift-and-look.
- **The bake-out** (see §2).
- **Roster order / auto-deal**: two controls appeared at count 13 with captions
  and no scene; I never touched them and nothing asked me to.

## 5. Contradictions and bugs

1. **Bake-out clock absent at count 3.** `$P assign r:haul:0 bake_out` →
   `{"error": "no such die or project"}`; `state` shows projects `['shelter_first']`
   only; `required` was already `None`. Engine: `project::open_eligible` skips every
   non-`dig_keep` condition project while `tutorial` is set and `tutorial_open` is
   not; `bake_out` and `throw` are condition projects.
2. **"Chamber crew minuted onto it" with an empty clock** (count 5). Log:
   *"Excavation of the KEEP opened and the chamber crew minuted onto it."*; state:
   `dig_keep 0/24 []`, Ceri and Eli in the hand. Content: the option has no
   `assign` (the writer removed it when same-count assign did not work; the engine
   now allows it).
3. **Liaison holder named wrong** (count 8). Text: "New seat: liaison, held by Vik
   Chandra." Ring: "who answers to the sponsor — Sol Hoshino". Vik holds hours.
4. **Solar conjunction clock** shown at counts 6 and 37–42 with no caption, before
   any scene explains it; gated on `ui:countdowns`, not on its own reveal.
5. **Restaffing silently failed** (count 25). `assign p:7 hand` then
   `assign p:9 align_driver`, `assign p:12 align_driver` returned no error and the
   clock read `5/8 []`. Both are probably upkeep dice after that count's deal (the
   hand read "8 eating and breathing"); the page's `assign` returns the view without
   an error when the engine refuses an upkeep die — or the driver dropped the rapid
   successive calls (known, `play.py`). Either way the reader gets no refusal.
6. **"Mass driver (MDLS) idle: no dice assigned"** logged at 18, 24, 29, 35, 41 for a
   clock hidden until 43.
7. **Lexicon mangles the plain labels after the handoff.** "Mass driver alignment
   (MDLS)" became "Mass driver alignment (driver)" and "Mass driver (MDLS)" became
   "Mass driver (driver)" — `MDLS → driver` applied inside the parenthesis.
8. **Sponsor track not shown with the review clock** (count 16–42): `ui:sponsor` and
   `ui:clock:review` are set together, but `#sponsor-track` stayed hidden until 43.
9. **Review and contract clocks reveal without a caption** although `CAPTIONS` has
   entries for both.
10. Hand caption stale on an arrival count: count 12 read "12 of 14 free" with 22
    dice on screen.
11. Every act-1 scene from count 13 lists two or three options as *favoured* (the
    ring split), so the "favoured" mark stops meaning anything; at count 8 three of
    four options were favoured.
12. Screenshots at MM 8 and MM 12 show an empty centre column with only End count
    once the scene is resolved — the count's events (the convoy line, the log line)
    are not shown there after the choice, only in the drawer.

## 6. Layout (screenshots)

- MM 1: one paragraph and two options on an empty page. Good; exactly the intent.
- MM 2 (after the option): the held button, in serif italic, reads as the mind
  speaking. The hand's numerals with first names are legible; the two dimmed dice
  ("4 Yara", "4 Sol") were obviously "not available" without explanation.
- MM 4: three columns are in use with one card on the right and one bar on the
  left; the water bar with "nominal ▲4" beside an idle plant reads as a lie.
- MM 12: the alignment clock shows the placed die as **3** (its engineering face)
  under a hand that shows the same person as **5** — the fact is on screen, but the
  two numbers are not next to each other.
- Count 46: with 29 crew the hand is five rows and the pressures and sponsor track
  are below the fold; the ring column scrolls. "Mass driver (driver)" is visible in
  the projects section.

## 7. Five changes first

1. **Engine** — open the standing projects (`bake_out`, `throw`) at tutorial start
   regardless of `tutorial_open`; only suppress *condition* one-time projects
   (`through_wall_arm`, `repair_relay`). Then the count-3 hold works and the plant
   runs from count 3.
2. **Engine + content** — the alignment requirement should demand a *useful* die:
   `require:place_person:align_driver:min_face:3` (or the scene names the two best
   engineers and the requirement is "one of these"). And when a required clock is
   left under-staffed for three counts, one engine line: "Mass driver alignment at 1
   pip a month: eight months to complete." Same line would have shown me the 1-face
   die.
3. **Content** — MM 2 should say the one sentence that fixes the faces problem:
   "Lift a die: each clock shows the face it would give." And MM 5's option must
   `assign` the chamber crew to the KEEP (the engine allows it now) or the log must
   stop saying they were.
4. **Page** — never drift text inside a label's parentheses (render project titles
   and clock labels without the lexicon, or exempt parenthesised acronyms); reveal
   the sponsor track with the review clock; add the conjunction and grace clocks to
   the caption table and gate them on their own reveal; keep the count's event lines
   in the centre column after the scene resolves.
5. **Content/engine** — retire the *favoured* mark when more than one option carries
   it, or mark the ring's majority only; and give the count-13 controls (roster,
   auto-deal) either a scene that asks for them or a later reveal (act 1, at the
   first upkeep eviction).
