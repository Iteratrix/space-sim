# Driving the game as an agent

`space-sim play --json --seed N` runs one game and speaks newline-delimited JSON on
stdout. It reads choices from stdin, one per line.

Each count produces one object:

```json
{
  "status": "count  12 | pop 48 (14 res) | water 388 t | ...",
  "events": ["A convoy: 45 t landed, ..."],
  "firings": [
    {
      "id": "first_manifest",
      "title": "The First Manifest",
      "text": "...situation, names filled in...",
      "roles": {"hours": 37, "liaison": 1},
      "options": [
        {"index": 0, "id": "throughput", "label": "...", "text": "...", "tags": ["throughput", "sponsor"]},
        {"index": 1, "id": "balanced", "label": "...", "text": "...", "tags": ["capability", "throughput"]}
      ],
      "counsel": [
        {"seat": "Hulls", "holder": "Yara Iqbal", "favours": 2, "text": "...", "authored": true}
      ]
    }
  ],
  "ending": null
}
```

For every entry in `firings`, in order, write one line to stdin: either the 1-based
position in `options` or the option's `id`. Then, every count (including quiet ones),
the game reads commands until a line `end`, which advances the count. Before a choice you may also send
`assign <die> <project|hand>` (dice are `p:<person id>` or `r:<class>:<k>`; projects by
id) and `set <control> <value>` (`manifest throughput|balanced|capability|people`,
`throw ship|hold|stop`, `roster skill|strain|name`, `auto_deal on|off`); each answers
`{"ok":true}` or `{"error":"..."}`. With `auto_deal off` the engine leaves free dice
in the hand for you to place. The game's `hand` and `projects` are in the save and
in `view` (coming); the text mode prints a `hand:` line each count. After each choice the game prints
`{"chronicle": "..."}`. When `ending` is non-null the game prints `{"summary": "..."}`
and exits. Counts with no firings need no input.

`counsel[].favours` is an option `index` (not a position); `roles` maps role names to
person ids. `status` is the same line the text mode prints, for humans reading the log.

For batch evaluation use `space-sim montecarlo --games G --seed N --policy P` with a
policy in `random | first | ring | capability | throughput`; it reports endings, the
sponsor's final stage, act length, population, per-storylet firing rates, storylets
that never fired and options never chosen. `space-sim run --seed N --policy P --trace`
prints one game's chronicle with a status line every six counts on stderr.

Saves: `play --save FILE` writes the full state as JSON after every count;
`play --load FILE` resumes it. Replay is exact: the same seed and the same choices
produce the same chronicle.

## Playing the page itself

`web/test/play.py` drives the real browser page (Playwright + Chromium), one action per
invocation, with the save kept in a browser profile so state persists between calls:

```sh
python3 -m http.server -d web 8765 &            # serve the page once
P="uv run --with playwright python3 web/test/play.py --profile me"
$P new 5 tutorial          # start; prints the state as JSON
$P state                   # headline, scene (title, text, options with n/id), events, hand, projects, countdowns, ledger, pressures, sponsor, ring (counsel + favours), controls
$P choose 2                # or an option id
$P end                     # advance a count (refused while a scene is pending)
$P assign p:7 dig_keep     # die ids and project ids come from `state`
$P set auto_deal off       # manifest | throw | roster | auto_deal
$P chronicle
$P screenshot /tmp/shot.png
```

Each call takes ~2 s (a fresh Chromium with a persistent profile). Use a distinct
`--profile` per agent; the default profile is `web/test/.profile`.

## The feel gate

`space-sim feel --games G --scenario tutorial|act1` plays G games alternating the ring
and random policies and fails (exit 1) when the game is flat, silent, or partial: a
quiet streak (no scene and no engine event) longer than 5 counts in the tutorial or
9 in act 1; a storylet that never fires under either policy (a short allow-list
covers deliberate fallbacks); more than 60% of options never chosen; fewer than five
seats ever giving counsel; an act-1 game that never reaches an ending. CI runs it on
every push. The limits are the current measured feel, not the target — tighten them
as content fills the gaps.
