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
position in `options` or the option's `id`. After each choice the game prints
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
