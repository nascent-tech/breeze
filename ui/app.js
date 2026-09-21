// Breeze panel — renders the CycleSnapshot the Tauri host projects, and sends
// the panel's levers back as commands. The view holds no product logic: every
// number comes from get_snapshot, every refusal is decided by the host.
// (Design preview without a host lives in preview.html, which supplies a mock.)

const RING_CIRCUMFERENCE = 2 * Math.PI * 54;
const POLL_MS = 500;
const MINUTES_PER_DAY = 24 * 60;
const DAWN_HOUR = 6;

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

function minutesUntilDawn() {
  const now = new Date();
  const dawn = new Date(now);
  dawn.setHours(DAWN_HOUR, 0, 0, 0);
  if (dawn <= now) {
    dawn.setDate(dawn.getDate() + 1);
  }
  return Math.max(1, Math.round((dawn - now) / 60000));
}

const PALETTE = {
  Working: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Travail en cours" },
  Notice: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Pause imminente" },
  Break: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "Pause en cours" },
  Returning: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "C’est fini" },
  Suspended: { stroke: "#868DA0", tint: "rgba(28,33,48,.06)", leaf: "#868DA0", title: "Suspendu" },
  Inactive: { stroke: "#868DA0", tint: "rgba(28,33,48,.06)", leaf: "#868DA0", title: "En veille" },
};

function ringSub(snap) {
  if (snap.phase === "Working" || snap.phase === "Notice") {
    return `PAUSE À ${wallTimeIn(snap.break_in_secs)}`;
  }
  if (snap.phase === "Break") {
    return "TIENS BON";
  }
  if (snap.phase === "Suspended") {
    return `REPRISE À ${wallTimeIn(snap.remaining_secs)}`;
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

function paintLevers(phase) {
  el("levers").style.display = phase === "Working" ? "flex" : "none";
  el("resume-box").style.display = phase === "Suspended" ? "flex" : "none";
  const coldChoice = phase === "Working" || phase === "Inactive";
  el("severity-row").style.opacity = coldChoice ? "1" : "0.4";
  el("rhythm-row").style.opacity = coldChoice ? "1" : "0.4";
}

function paintRhythm(snap) {
  for (const item of document.querySelectorAll("#rhythm-seg .seg-item")) {
    const on = Number(item.dataset.work) === snap.work_minutes && Number(item.dataset.pause) === snap.pause_minutes;
    item.classList.toggle("on", on);
  }
}

const REFUSALS = {
  "break-due": "Une pause est due — réglable au cycle suivant.",
  "not-suspendable": "Rien à suspendre ici.",
  "not-suspended": "Breeze n’est pas suspendu.",
  "unknown-severity": "Sévérité inconnue.",
  "invalid-rhythm": "Rythme hors bornes — la pause ne peut pas dépasser le travail.",
};

let hintTimer = null;

function showHint(code) {
  const sub = el("phase-sub");
  if (!sub) {
    return;
  }
  sub.textContent = REFUSALS[code] || "Action refusée.";
  clearTimeout(hintTimer);
  hintTimer = setTimeout(() => {
    sub.textContent = "Cycle de la journée";
  }, 4000);
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
  paintRhythm(snap);
  paintLevers(snap.phase);

  const served = snap.served_breaks || 0;
  el("today-line").textContent = `${served} pause${served === 1 ? "" : "s"} servie${served === 1 ? "" : "s"}`;
  el("menubar-clock").textContent = formatClock(snap.remaining_secs);
}

function invoker() {
  return window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
}

async function send(command, args) {
  const invoke = invoker();
  if (!invoke) {
    return;
  }
  try {
    await invoke(command, args);
  } catch (code) {
    showHint(String(code));
  }
}

function wireControls() {
  for (const item of document.querySelectorAll("#severity-seg .seg-item")) {
    item.addEventListener("click", () => send("set_severity", { severity: item.dataset.sev }));
  }
  for (const item of document.querySelectorAll("#rhythm-seg .seg-item")) {
    item.addEventListener("click", () =>
      send("set_rhythm", { workMinutes: Number(item.dataset.work), pauseMinutes: Number(item.dataset.pause) })
    );
  }
  for (const item of document.querySelectorAll("[data-suspend]")) {
    item.addEventListener("click", () => {
      const raw = item.dataset.suspend;
      const minutes = raw === "dawn" ? minutesUntilDawn() : Number(raw);
      send("suspend", { minutes });
    });
  }
  const resume = el("resume-btn");
  if (resume) {
    resume.addEventListener("click", () => send("resume"));
  }
  const quit = el("quit");
  if (quit) {
    quit.addEventListener("click", () => send("quit"));
  }
}

async function poll() {
  const invoke = invoker();
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

wireControls();
poll();
setInterval(poll, POLL_MS);
