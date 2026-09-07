# Storylet format

One TOML file per storylet under `data/storylets/`. Files are bundled into the binary
at build time; `space-sim validate` parses them all and fails on the first error.
Every quality, seat, skill, and role a storylet touches is checked at load.

## Shape

```toml
id = "grace"                  # unique, snake_case
title = "Grace"
act = 1                       # 1, 2, or 3; only fires in that act
weight = 1.0                  # relative chance among eligible storylets (default 1)
once = true                   # fire at most once per game (default false)
cooldown = 12                 # counts before it may fire again (default 0)
priority = 0                  # >= 100 is a "must" scene: fires as soon as eligible
tags = ["machines", "sponsor"]
text = """
Situation text. {role} placeholders are replaced by cast names; {outpost},
{mind}, {turn} are globals. Write in the sponsor-era register; the lexicon
drifts the words later.
"""

# IMPORTANT: every scalar key above must come BEFORE the first [[when]] /
# [[cast]] / [[option]] block, or TOML will attach it to the last block.

[[when]]                      # all conditions must hold
q = "sponsor.attention"       # one comparison per condition: lt le gt ge eq
lt = 0.3

[[when]]
flag = "relay_failed"         # or not_flag = "..."

[[when]]
turn_min = 24                 # or turn_max, window = "open"/"closed",
                              # fired = "other_id", not_fired = "other_id",
                              # stage_min = 3, stage_max = 5,
                              # counter = "name" with a comparison

[[cast]]
role = "operator"             # used as {operator} in text
seat = "machines"             # the ring seat's holder, or
# skill = "engineering"       # the most skilled present person, or neither: anyone
min_skill = 1                 # optional
rotator = true                # optional: true = rotators only, false = residents only
optional = true               # storylet may fire with this role empty

[[option]]
id = "keep"
label = "Keep the shackles on"
text = "Longer description shown under the label."
tags = ["sponsor", "caution"]    # the ring scores options by tags (see below)
chronicle = "One line, past tense, in the polity's voice. Required."

[[option.requires]]           # optional gate, same shape as [[when]]
q = "stocks.spares"
ge = 1

[[option.effect]]
q = "sponsor.confidence"      # exactly one of add / set / mul
add = 0.1

[[option.effect]]
flag = "licence_kept"         # or unflag = "..."

[[option.effect]]
counter = "arguments_about_minds"   # add = 1 by default

[[option.effect]]
person = "operator"           # a cast role; then exactly one of:
strain = 0.2                  # strain / dose (Sv) / return_intent / leave = true /
                              # die = true / resident = true

[[option.effect]]
tie = ["operator", "liaison"] # directed, from -> to
hindrance = 0.3               # any of hindrance / work_positive / reliance

[[option.effect]]
lexicon = "unforgetting"      # fire a vocabulary trigger

[[option.effect]]
all_strain = 0.1              # everyone; or all_return_intent = -0.1

[[option.effect]]
name_mind = true              # the senior operator names the oldest unnamed mind

[[option.effect]]
end = "closed"                # or "extinct"; with reason = "..."
reason = "evacuated on the sponsor's last ship"

[[option.advice]]             # optional authored advice; overrides the generated line
seat = "hulls"
stance = "for"                # or "against"
text = "In the holder's voice. One or two sentences."
```

## Qualities

Readable everywhere; writable ones are marked `w`.

| key | meaning |
|---|---|
| `turn` | current count |
| `act` | 1-3 |
| `counts_since_convoy`, `missed_convoys` | the Silence clock |
| `earth_window_open` (0/1), `earth_window_cost` (km/s), `counts_to_window` | the Earth edge |
| `sponsor.runway` w, `sponsor.confidence` w (0-1), `sponsor.attention` w (0-1) | the sponsor's stocks |
| `sponsor.stage` | 0 enthusiasm, 1 anxiety, 2 updates stopped, 3 austerity, 4 skipped rotation, 5 sale, 6 no ship |
| `sponsor.counts_to_review` | |
| `stocks.water` w (t), `stocks.propellant` w (t), `stocks.nitrogen` w (kg), `stocks.spares` w, `stocks.boron` w (kg), `stocks.helium` w (kg), `stocks.medicine` w, `stocks.food_margin` w (counts) | |
| `closure` w (0-1) | mass closure |
| `power.capacity`, `power.demand` (kW), `power.reactor_life` w (counts) | |
| `relay.health` w (0-1) | |
| `throughput.shipped` w, `throughput.received` w, `throughput.phi` | tonnes; φ = shipped/received |
| `people.population`, `people.residents`, `people.rotators`, `people.children` | |
| `people.mean_strain`, `people.coherence`, `people.conflict_concentration`, `people.return_share`, `people.belt_born_share`, `people.mean_dose` | derived social indices |
| `labour.capacity`, `labour.demand` (h), `labour.deficit` (fraction; >0 is short) | |
| `robots.plant` w, `robots.haul` w, `robots.arm` w, `robots.dex` w, `robots.through_wall` w | fleet counts (fractional) |
| `minds.count`, `minds.units` w, `minds.licence` (0 compliant, 1 grace, 2 lapsed, 3 self-certified, 4 unlicensed), `minds.embodiment` | |
| `menace.suspicion` w, `menace.grievance` w, `menace.leak` w, `menace.reactor_wear` w | 0-10; threshold "must" scenes hang off these |
| `solar_phase` | 0 minimum .. 1 maximum |

## Seats

`hulls`, `hours`, `extraction`, `bodies`, `air`, `liaison` exist in act 1.
`machines` appears in act 2 or when the flag `machines_seat` is set; `children`
when children are present. Casting `seat = "liaison"` gives you the person who
speaks to the sponsor — the sponsor itself has no body.

## Tags the ring scores

`throughput`, `capability`, `spares`, `people`, `dose`, `risk`, `sponsor`,
`independence`, `machines`, `families`, `air`, `nitrogen`, `closure`, `water`,
`hours`, `cost`, `medicine`, `hulls`. Each seat has biases; a low-skill holder
gives wrong advice more often. Authored `[[option.advice]]` replaces the
generated line for that seat and option.

## Flags the engine sets

`conjunction` (Earth link blocked this count), `unforgetting` (first missed
reset), `someone_stayed` (a rotator chose residence at a convoy),
`stage:N` (sponsor reached stage N), `licence_jailbroken` (content may set this;
the engine then treats the licence as self-certified).

## Lexicon triggers the engine fires

`first_count`, `first_convoy`, `first_silence`, `reactor_dead`, `licence_grace`,
`unforgetting`, `mind_death`, `estates_named`. Content may fire these or its own
(`first_voiding`, `dose_ledger`) with `lexicon = "..."`.

## Writing rules

- The chronicle line is the storylet's memory. Past tense, one sentence or two,
  no option label restated.
- Every option should be defensible by some seat. If no seat would argue for it,
  cut it.
- Effects in sim units: a convoy is ~50 t; spares ~200 units at start and ~4
  consumed per count; nitrogen ~6,000 kg leaking ~45/count; strain and
  return_intent are 0-1; menaces 0-10 with "must" scenes at ~7.
- Use `once = true` for anything with a founding-trauma feel; use `cooldown`
  for texture scenes.
- Prefer `[[when]]` on qualities and stage over `turn_min`; the game's tempo is
  windows, not counts.
