"""Headless smoke test of the page: load, play a few counts, report console errors.

Run: python3 web/test/smoke.py [http://127.0.0.1:8765/]
Needs Playwright with Chromium installed (uv tool install playwright; playwright install chromium).
"""

import sys
import time

from playwright.sync_api import sync_playwright

url = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8765/"
errors: list[str] = []
with sync_playwright() as p:
    browser = p.chromium.launch()
    page = browser.new_page(viewport={"width": 1400, "height": 900})
    page.on("console", lambda m: errors.append(m.text) if m.type == "error" else None)
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.goto(url)
    page.wait_for_function("document.getElementById('headline').textContent.startsWith('count')", timeout=30000)
    print("headline:", page.text_content("#headline"))
    for step in range(12):
        opt = page.query_selector(".options li")
        if opt:
            print(f"step {step}: scene '{page.text_content('#scene h2').strip()}' -> choosing option 1")
            opt.click()
            time.sleep(0.1)
            continue
        end = page.query_selector("#btn-end")
        if end and end.is_enabled():
            end.click()
            time.sleep(0.1)
            print(f"step {step}: end count -> {page.text_content('#headline')}")
        else:
            print(f"step {step}: nothing to do")
            break
    print("hand:", page.text_content("#hand-count"), "|", page.text_content("#hand-note"))
    print("projects:", [e.text_content().strip()[:40] for e in page.query_selector_all("#projects .label")])
    print("ring:", [e.text_content().strip() for e in page.query_selector_all("#seats .who")][:6])
    dice = page.query_selector_all("#hand-dice .die[draggable=true]")
    proj = page.query_selector(".clock[data-project='dig_keep']") or page.query_selector(".clock[data-project]")
    if dice and proj:
        before = len(proj.query_selector_all(".die"))
        dice[0].click()
        proj.click()
        time.sleep(0.1)
        proj = page.query_selector(f".clock[data-project='{proj.get_attribute('data-project')}']")
        after = len(proj.query_selector_all(".die"))
        print(f"tap-tap assign: project dice {before} -> {after}")
    page.select_option("select[data-control='manifest']", "capability")
    time.sleep(0.1)
    print("manifest control:", page.eval_on_selector("select[data-control='manifest']", "e => e.value"))
    page.click("#btn-chronicle")
    print("chronicle lines:", len(page.text_content("#chronicle").strip().splitlines()))
    page.screenshot(path="web/test/smoke.png")
    browser.close()
print("console errors:", errors if errors else "none")
sys.exit(1 if errors else 0)
