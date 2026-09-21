// Breeze panel — renders the CycleSnapshot the Tauri host projects.
// The view holds no product logic: every number comes from get_snapshot.
// (Design preview without a host lives in preview.html, which supplies a mock.)

const RING_CIRCUMFERENCE = 2 * Math.PI * 54;
const POLL_MS = 500;

const el = (id) => document.getElementById(id);

function pad(n) {
  return String(n).padStart(2, "0");
}

function formatClock(totalSeconds) {
  const s = Math.max(0, Math.round(totalSeconds));
  return `${Math.floor(s / 60)}:${pad(s % 60)}`;
}

function wallTimeIn(seconds) {
  const t = new Date(Date.now() + seconds * 1000);
  return `${t.getHours()}:${pad(t.getMinutes())}`;
}

const PALETTE = {
  Working: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Travail en cours" },
  Notice: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Pause imminente" },
  Break: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "Pause en cours" },
  Returning: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "C’est fini" },
  Inactive: { stroke: "#868DA0", tint: "rgba(28,33,48,.06)", leaf: "#868DA0", title: "En veille" },
};

function ringSub(snap) {
  if (snap.phase === "Working" || snap.phase === "Notice") {
    return `PAUSE À ${wallTimeIn(snap.break_in_secs)}`;
  }
  if (snap.phase === "Break") {
    return "TIENS BON";
  }
  return "—";
}

function paintRing(snap) {
  const elapsed = snap.total_secs > 0 ? (snap.total_secs - snap.remaining_secs) / snap.total_secs : 0;
  const dash = Math.max(0, Math.min(1, elapsed)) * RING_CIRCUMFERENCE;
  el("ring-arc").setAttribute("stroke-dasharray", `${dash.toFixed(1)} ${RING_CIRCUMFERENCE.toFixed(1)}`);
}

function paintSeverity(severity) {
  el("severity-chip").textContent = severity;
  el("severity-chip").className = severity === "Hardcore" ? "chip chip-violet" : "chip chip-neutral";
  for (const item of document.querySelectorAll("#severity-seg .seg-item")) {
    item.classList.toggle("on", item.dataset.sev === severity);
  }
}

function render(snap) {
  const look = PALETTE[snap.phase] || PALETTE.Inactive;
  el("phase-title").textContent = look.title;
  el("leaf-badge").style.background = look.tint;
  el("leaf-path").setAttribute("fill", look.leaf);
  el("ring-arc").setAttribute("stroke", look.stroke);

  paintRing(snap);
  el("countdown").textContent = formatClock(snap.remaining_secs);
  el("ring-sub").textContent = ringSub(snap);
  paintSeverity(snap.severity);

  // No lever weakens a due break: hide them from the notice onward.
  const held = snap.phase !== "Working" && snap.phase !== "Inactive";
  el("levers").style.display = held ? "none" : "flex";
  el("severity-row").style.opacity = held ? "0.4" : "1";

  const served = snap.served_breaks || 0;
  el("today-line").textContent = `${served} pause${served === 1 ? "" : "s"} servie${served === 1 ? "" : "s"}`;
  el("menubar-clock").textContent = formatClock(snap.remaining_secs);
}

function tauriInvoke() {
  return window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
}

async function poll() {
  const invoke = tauriInvoke();
  if (!invoke) {
    el("phase-title").textContent = "En attente de l’hôte…";
    return;
  }
  try {
    render(await invoke("get_snapshot"));
  } catch (_e) {
    // host busy; keep the last frame
  }
}

poll();
setInterval(poll, POLL_MS);
