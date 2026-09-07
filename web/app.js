import init, * as wasm from "./pkg/space_sim_web.js";

const el = (id) => document.getElementById(id);
const parse = (s) => JSON.parse(s);
const FACES = ["·", "1", "2", "3", "4", "5", "6"];
const SAVE_KEY = "fortuna-save";
const PENDING_KEY = "fortuna-pending";

let handle = null;
let view = null;
let pending = [];
let lifted = null;
let counsel = [];

function ring(filled, segments, cls = "on", r = 18) {
  const n = Math.max(segments, 1);
  const c = 2 * Math.PI * r;
  const gap = n > 1 ? 2 : 0;
  const seg = c / n;
  let out = `<svg viewBox="0 0 44 44">`;
  for (let i = 0; i < n; i++) {
    const on = i < filled ? ` ${cls}` : "";
    out += `<circle class="seg${on}" cx="22" cy="22" r="${r}" stroke-dasharray="${seg - gap} ${c - seg + gap}" stroke-dashoffset="${-i * seg + c / 4}"/>`;
  }
  return out + `</svg>`;
}

function die(d) {
  const dull = !d.robot && d.strain > 0.5 ? " dull" : "";
  const upkeep = d.place === "upkeep" ? " upkeep" : "";
  const face = d.robot ? "R" : FACES[Math.min(d.face, 6)];
  const title = d.robot ? `${d.label} unit — ${d.face} pips` : `${d.label} — face ${d.face}${dull ? " (strained)" : ""}${upkeep ? " — eating and breathing" : ""}`;
  const name = d.robot ? "" : `<small>${d.label.split(" ")[0]}</small>`;
  const facesNote = d.faces && Object.keys(d.faces).length ? " — " + Object.entries(d.faces).map(([k, v]) => `${k} ${v}`).join(", ") : "";
  const draggable = d.robot || !upkeep;
  return `<span class="die${d.robot ? " robot" : ""}${dull}${upkeep && !d.robot ? " upkeep" : ""}" draggable="${draggable}" data-die="${d.id}" title="${title}${facesNote}">${face}${name}</span>`;
}

function headline(v) {
  const rsw = v.countdowns.find((c) => c.id === "rsw");
  if (v.ending) return "the Silence";
  if (rsw && rsw.remaining === 0) return "the window is open";
  if (rsw) return `the next RSW is in ${rsw.remaining}`;
  return "";
}

function render() {
  const v = view;
  el("headline").textContent = `count ${v.turn} · ${headline(v)}`;
  el("countdowns").querySelector(".clocks").innerHTML = v.countdowns
    .map((c) => `<div class="clock${c.urgent ? " urgent" : ""}" title="${c.why}">${ring(c.filled, c.segments)}<div><div class="label">${c.label}</div><div class="sub">${c.remaining == null ? "" : c.remaining + " left"}${c.id === "rsw" ? ` · manifest <select data-control="manifest"><option${v.controls.manifest === "throughput" ? " selected" : ""}>throughput</option><option${v.controls.manifest === "balanced" ? " selected" : ""}>balanced</option><option${v.controls.manifest === "capability" ? " selected" : ""}>capability</option><option${v.controls.manifest === "people" ? " selected" : ""}>people</option></select>` : ""}</div></div></div>`)
    .join("");
  el("projects").querySelector(".clocks").innerHTML = v.projects
    .map((p) => {
      const clock = p.standing ? ring(Math.round(Math.min(p.rate, 1) * 12), 12, "rate") : ring(p.filled, p.segments);
      const sub = p.standing ? `rate ${p.rate.toFixed(2)} · ${p.pips}/${Math.round(p.pips_needed)} pips` : `${p.filled}/${p.segments}${p.roll === "setback" ? " · setback" : p.roll === "bonus" ? " · bonus" : ""}`;
      const throwCtl = p.id === "throw" ? ` <select data-control="throw"><option value="ship"${v.controls.throw === "ship" ? " selected" : ""}>ship</option><option value="hold"${v.controls.throw === "hold" ? " selected" : ""}>hold at reserve</option><option value="stop"${v.controls.throw === "stop" ? " selected" : ""}>stop</option></select>` : "";
      return `<div class="clock" data-project="${p.id}" title="${p.description}">${clock}<div><div class="label">${p.title}</div><div class="sub">${sub}${throwCtl}</div><div class="dice">${p.dice.map(die).join("")}</div></div></div>`;
    })
    .join("");
  const h = v.hand;
  el("hand-count").textContent = `${h.free} of ${h.adults} free`;
  const handDice = h.dice.filter((d) => d.place === "hand" || d.place === "upkeep");
  el("hand-dice").innerHTML = handDice.map(die).join("");
  el("hand-note").textContent = h.free === 0 ? "Nobody is free this count. Everyone is keeping everyone alive." : `${h.eaten} eating and breathing · ${h.robots} robot units${h.shortfall ? ` · short ${h.shortfall}` : ""}`;
  el("roster").value = v.controls.roster;
  el("auto-deal").checked = v.controls.auto_deal;
  el("ledger").querySelector(".bars").innerHTML = v.ledger
    .map((b) => {
      const pct = Math.max(0, Math.min(100, (b.value / (b.full || 1)) * 100));
      const warn = ["below reserve", "dry", "critical", "zero", "failing", "farm short", "short", "brittle"].includes(b.word);
      const delta = Math.abs(b.delta) >= 0.05 ? ` ${b.delta > 0 ? "▲" : "▼"}${Math.abs(b.delta).toFixed(b.unit === "t/t" ? 2 : 0)}` : "";
      return `<div class="bar" title="${b.label}: ${b.value.toFixed(1)} ${b.unit} (${b.delta >= 0 ? "+" : ""}${b.delta.toFixed(1)} this count). ${b.why}"><span>${b.id}</span><div><div class="track"><div class="fill${warn ? " warn" : ""}" style="width:${pct}%"></div></div><span class="word">${b.word}${delta}</span></div></div>`;
    })
    .join("");
  el("pressures").querySelector(".rings").innerHTML = v.pressures
    .map((p) => `<div class="pressure" title="${p.label}: ${p.value.toFixed(1)} of 10">${ring(Math.round(p.value), 10, "p" + Math.max(p.band, 1))}${p.id}<br><em>${p.band_name}</em></div>`)
    .join("");
  const s = v.sponsor;
  el("sponsor-track").innerHTML = `sponsor: ${[0, 1, 2, 3, 4, 5, 6].map((i) => `<span class="dot${i <= s.stage ? " on" : ""}"></span>`).join("")} ${s.stage_name} · ${s.mood}<br>φ ${s.phi.toFixed(1)} against ${s.phi_expected.toFixed(1)} · review in ${s.counts_to_review}`;
  renderRing();
  bindDice();
}

function renderRing() {
  const v = view;
  el("seats").innerHTML = v.ring
    .map((seat) => {
      const c = counsel.find((x) => x.seat.toLowerCase() === seat.seat);
      const fav = c && pending[0] ? pending[0].options.find((o) => o.index === c.favours) : null;
      return `<div class="seat${seat.silent ? " silent" : ""}" title="${seat.title}${seat.holder ? ` · skill ${seat.skill}` : ""}"><div class="q">${seat.question}</div><div class="who">${seat.holder || "—"}</div><div class="mood">${seat.holder ? seat.mood : ""}</div>${c ? `<div class="say">“${c.text}”</div>` : ""}${fav ? `<div class="fav">favours: ${fav.label}</div>` : ""}</div>`;
    })
    .join("");
}

function bindDice() {
  document.querySelectorAll(".die[draggable=true]").forEach((d) => {
    d.addEventListener("dragstart", (e) => e.dataTransfer.setData("text/plain", d.dataset.die));
    d.addEventListener("click", () => {
      if (lifted === d.dataset.die) lifted = null;
      else lifted = d.dataset.die;
      document.querySelectorAll(".die").forEach((x) => x.classList.toggle("lifted", x.dataset.die === lifted));
    });
  });
  const targets = [...document.querySelectorAll(".clock[data-project]"), el("hand")];
  targets.forEach((t) => {
    const target = t.dataset.project || "hand";
    t.addEventListener("dragover", (e) => { e.preventDefault(); t.classList.add("drop"); });
    t.addEventListener("dragleave", () => t.classList.remove("drop"));
    t.addEventListener("drop", (e) => { e.preventDefault(); t.classList.remove("drop"); assign(e.dataTransfer.getData("text/plain"), target); });
    t.addEventListener("click", (e) => { if (lifted && !e.target.closest(".die")) { assign(lifted, target); lifted = null; } });
  });
  document.querySelectorAll("select[data-control]").forEach((s) => s.addEventListener("change", () => setControl(s.dataset.control, s.value)));
}

function apply(result) {
  const r = parse(result);
  if (r.error) { el("hand-note").textContent = r.error; return false; }
  view = r;
  render();
  persist();
  return true;
}
function assign(dieId, target) { apply(wasm.assign(handle, dieId, target)); }
function setControl(control, value) { apply(wasm.set(handle, control, value)); }

function showScene() {
  const scene = el("scene");
  if (pending.length === 0) {
    scene.innerHTML = "";
    counsel = [];
    renderRing();
    el("btn-end").disabled = !!view.ending;
    return;
  }
  const f = pending[0];
  counsel = f.counsel;
  const must = f.priority >= 100;
  scene.innerHTML = `<h2 class="${must ? "must" : ""}">${f.title}</h2><p>${f.text.trim()}</p><ul class="options">${f.options
    .map((o) => `<li data-option="${o.id}"${counsel.some((c) => c.favours === o.index) ? ' class="favoured"' : ""}>${o.label}<span class="desc">${o.text}</span></li>`)
    .join("")}</ul>`;
  scene.querySelectorAll("li[data-option]").forEach((li) => li.addEventListener("click", () => choose(f.id, li.dataset.option)));
  el("btn-end").disabled = true;
  renderRing();
}

function persist() {
  try {
    localStorage.setItem(SAVE_KEY, wasm.save(handle));
    localStorage.setItem(PENDING_KEY, JSON.stringify(pending));
  } catch (_) { /* private mode */ }
}

function choose(firingId, optionId) {
  const firing = pending.find((f) => f.id === firingId);
  if (!firing) return;
  const r = parse(wasm.resolve(handle, JSON.stringify(firing), optionId));
  if (r.error) { el("hand-note").textContent = r.error; return; }
  el("events").insertAdjacentHTML("beforeend", `<div class="chron-line">${r.chronicle}</div>`);
  pending.shift();
  apply(wasm.view(handle));
  persist();
  showScene();
}

function advance() {
  const r = parse(wasm.advance(handle));
  if (r.error) { el("hand-note").textContent = r.error; return; }
  view = r.view;
  pending = r.firings;
  el("events").innerHTML = r.events.map((e) => `<div>${e}</div>`).join("");
  render();
  showScene();
  persist();
  if (r.ending) {
    el("events").insertAdjacentHTML("beforeend", `<div class="chron-line">${parse(wasm.summary(handle)).replaceAll("\n", "<br>")}</div>`);
    el("btn-end").disabled = true;
  }
}

function start(seed, scenario) {
  const r = parse(wasm.new_game(BigInt(seed), scenario));
  if (r.error) { el("headline").textContent = r.error; return; }
  handle = r.handle;
  advance();
}

function resume() {
  try {
    const saved = localStorage.getItem(SAVE_KEY);
    if (!saved) return false;
    const r = parse(wasm.load(saved));
    if (r.error) return false;
    handle = r.handle;
    try { pending = JSON.parse(localStorage.getItem(PENDING_KEY) || "[]"); } catch (_) { pending = []; }
    apply(wasm.view(handle));
    el("events").innerHTML = "<div>Resumed from the last count.</div>";
    showScene();
    return true;
  } catch (_) { return false; }
}

el("btn-end").addEventListener("click", advance);
el("btn-chronicle").addEventListener("click", () => { el("chronicle").textContent = parse(wasm.chronicle(handle)).join("\n"); el("drawer").hidden = false; });
el("btn-close").addEventListener("click", () => { el("drawer").hidden = true; });
el("btn-new").addEventListener("click", () => { el("dialog").hidden = false; });
el("btn-cancel").addEventListener("click", () => { el("dialog").hidden = true; });
el("btn-start").addEventListener("click", () => { el("dialog").hidden = true; start(el("seed").value || 3, el("scenario").value); });
el("roster").addEventListener("change", () => setControl("roster", el("roster").value));
el("auto-deal").addEventListener("change", () => setControl("auto_deal", el("auto-deal").checked ? "on" : "off"));

function registerServiceWorker() {
  if (!("serviceWorker" in navigator)) return;
  if (location.hostname === "localhost" || location.hostname === "127.0.0.1") return;
  navigator.serviceWorker.register("./sw.js").catch(() => {});
  if (navigator.storage?.persist) navigator.storage.persist().catch(() => {});
}

await init();
registerServiceWorker();
if (!resume()) start(3, "tutorial");
