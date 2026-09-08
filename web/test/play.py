"""Drive the game's page from the shell, one action per invocation, state kept in the browser profile.

Usage:
  play.py [--url URL] [--profile NAME] state                 # JSON: headline, scene, options, counsel, hand, projects, ledger, pressures, sponsor, events
  play.py choose <n|option-id>              # pick an option in the current scene
  play.py end                               # end the count (only when no scene is pending)
  play.py assign <die-id> <project|hand>    # move a die; ids from `state`
  play.py set <manifest|throw|roster|auto_deal> <value>
  play.py new <seed> <tutorial|act1>        # start a fresh game
  play.py chronicle                         # the whole chronicle
  play.py screenshot [path]                 # PNG of the current screen (default web/test/shot.png)

Needs Playwright with Chromium (uv tool install playwright; playwright install chromium).
Run with: uv run --with playwright python3 web/test/play.py ...
The browser profile lives in web/test/.profile so localStorage (the save) persists between calls.
"""

import json
import os
import sys
import time

from playwright.sync_api import sync_playwright

HERE = os.path.dirname(os.path.abspath(__file__))
PROFILE = os.path.join(HERE, ".profile")
url = "http://127.0.0.1:8765/"
args = sys.argv[1:]
while args and args[0] in ("--url", "--profile"):
    if args[0] == "--url":
        url = args[1]
    else:
        PROFILE = os.path.join(HERE, ".profile-" + args[1])
    args = args[2:]
cmd = args[0] if args else "state"

JS_STATE = """
() => {
  const vis = (e) => !!e && e.offsetParent !== null;
  const t = (sel) => { const e = document.querySelector(sel); return e && vis(e) ? e.textContent.trim() : null; };
  const opts = [...document.querySelectorAll('.options li')].map((li, i) => ({ n: i + 1, id: li.dataset.option, label: li.firstChild.textContent.trim(), desc: li.querySelector('.desc')?.textContent.trim(), favoured: li.classList.contains('favoured') }));
  const seats = [...document.querySelectorAll('.seat')].filter(vis).map((s) => ({ question: s.querySelector('.q')?.textContent.trim(), holder: s.querySelector('.who')?.textContent.trim(), mood: s.querySelector('.mood')?.textContent.trim(), says: s.querySelector('.say')?.textContent.trim(), favours: s.querySelector('.fav')?.textContent.trim(), silent: s.classList.contains('silent') }));
  const dice = (root) => [...root.querySelectorAll('.die')].filter(vis).map((d) => ({ id: d.dataset.die, face: d.textContent.trim(), title: d.title, robot: d.classList.contains('robot'), upkeep: d.classList.contains('upkeep') }));
  const projects = [...document.querySelectorAll('.clock[data-project]')].filter(vis).map((c) => ({ id: c.dataset.project, title: c.querySelector('.label')?.textContent.trim(), sub: c.querySelector('.sub')?.textContent.trim().replace(/\\s+/g, ' '), dice: dice(c) }));
  const countdowns = [...document.querySelectorAll('#countdowns .clock')].filter(vis).map((c) => ({ label: c.querySelector('.label')?.textContent.trim(), sub: c.querySelector('.sub')?.textContent.trim().replace(/\\s+/g, ' '), urgent: c.classList.contains('urgent'), why: c.title }));
  const ledger = [...document.querySelectorAll('.bar')].filter(vis).map((b) => ({ id: b.querySelector('span')?.textContent.trim(), word: b.querySelector('.word')?.textContent.trim(), why: b.title }));
  const pressures = [...document.querySelectorAll('.pressure')].filter(vis).map((p) => ({ text: p.textContent.trim().replace(/\\s+/g, ' '), why: p.title }));
  return {
    headline: t('#headline'),
    scene: { title: t('#scene h2'), must: !!document.querySelector('#scene h2.must'), text: t('#scene p'), options: opts },
    events: [...document.querySelectorAll('#events div')].map((d) => d.textContent.trim()),
    can_end: !document.getElementById('btn-end').disabled,
    hand: vis(document.getElementById('hand')) ? { count: t('#hand-count'), note: t('#hand-note'), dice: dice(document.getElementById('hand-dice')), roster: document.getElementById('roster').value, auto_deal: document.getElementById('auto-deal').checked } : null,
    countdowns, projects, ledger, pressures,
    sponsor: t('#sponsor-track'),
    ring: seats,
    controls: { manifest: vis(document.querySelector("select[data-control='manifest']")) ? document.querySelector("select[data-control='manifest']").value : null, throw: vis(document.querySelector("select[data-control='throw']")) ? document.querySelector("select[data-control='throw']").value : null },
    captions: [...document.querySelectorAll('.reveal')].filter(vis).map((e) => e.textContent.trim()),
  };
}
"""

with sync_playwright() as p:
    ctx = p.chromium.launch_persistent_context(PROFILE, viewport={"width": 1400, "height": 900})
    page = ctx.pages[0] if ctx.pages else ctx.new_page()
    errors = []
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.goto(url)
    page.wait_for_function("document.getElementById('headline').textContent.startsWith('count')", timeout=30000)
    time.sleep(0.2)
    out = None
    if cmd == "new":
        page.click("#btn-new")
        page.fill("#seed", args[1] if len(args) > 1 else "3")
        page.select_option("#scenario", args[2] if len(args) > 2 else "tutorial")
        page.click("#btn-start")
        time.sleep(0.3)
        out = page.evaluate(JS_STATE)
    elif cmd == "choose":
        key = args[1]
        li = page.query_selector(f".options li[data-option='{key}']") or (page.query_selector_all(".options li")[int(key) - 1] if key.isdigit() and page.query_selector_all(".options li") else None)
        if li is None:
            out = {"error": "no such option; no scene pending?"}
        else:
            li.click()
            time.sleep(0.2)
            out = page.evaluate(JS_STATE)
    elif cmd == "end":
        if page.get_attribute("#btn-end", "disabled") is not None:
            out = {"error": "cannot end: a scene is pending"}
        else:
            page.click("#btn-end")
            time.sleep(0.3)
            out = page.evaluate(JS_STATE)
    elif cmd == "assign":
        die, target = args[1], args[2]
        d = page.query_selector(f".die[data-die='{die}']")
        tgt = page.query_selector("#hand > h3") if target == "hand" else page.query_selector(f".clock[data-project='{target}']")
        if d is None or tgt is None:
            out = {"error": "no such die or project"}
        else:
            d.click()
            tgt.click()
            time.sleep(0.2)
            out = page.evaluate(JS_STATE)
    elif cmd == "set":
        ctl, val = args[1], args[2]
        if ctl == "roster":
            page.select_option("#roster", val)
        elif ctl == "auto_deal":
            box = page.query_selector("#auto-deal")
            if box.is_checked() != (val == "on"):
                box.click()
        else:
            page.select_option(f"select[data-control='{ctl}']", val)
        time.sleep(0.2)
        out = page.evaluate(JS_STATE)
    elif cmd == "chronicle":
        page.click("#btn-chronicle")
        out = page.text_content("#chronicle")
        page.click("#btn-close")
    elif cmd == "screenshot":
        path = args[1] if len(args) > 1 else os.path.join(HERE, "shot.png")
        page.screenshot(path=path)
        out = {"screenshot": path}
    else:
        out = page.evaluate(JS_STATE)
    ctx.close()
    if errors:
        print(json.dumps({"page_errors": errors}), file=sys.stderr)
    print(out if isinstance(out, str) else json.dumps(out, indent=1))
