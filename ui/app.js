// Breeze panel — renders the CycleSnapshot the Tauri host projects, and sends
// the panel's levers back as commands. The view holds no product logic: every
// number comes from get_snapshot, every refusal is decided by the host.
// (Design preview without a real host lives in preview.html, which supplies a mock.)

const RING_CIRCUMFERENCE = 2 * Math.PI * 54;
const POLL_MS = 500;
const DAWN_HOUR = 6;
const TOAST_MS = 4000;
const SECONDS_PER_MINUTE = 60;
const MINUTES_PER_HOUR = 60;
const MS_PER_MINUTE = 60000;
const RHYTHM_PRESETS = [
  [25, 5],
  [50, 10],
  [90, 15],
];

const REFUSALS = {
  "break-due": "Une pause est due : ce réglage attend la fin de la pause.",
  "not-suspendable": "Breeze est en veille : rien à suspendre.",
  "not-suspended": "Breeze n’est pas suspendu.",
  "not-interruptible": "Impossible d’interrompre ici.",
  "unknown-severity": "Sévérité inconnue.",
  "invalid-rhythm": "Rythme hors bornes : la pause ne peut pas dépasser le travail.",
  "invalid-active-days": "Jours actifs invalides.",
  "invalid-schedule": "Plage horaire invalide.",
  "invalid-app-id": "Application inconnue.",
  "unknown-status": "Statut inconnu.",
  "persistence-failed": "Échec d’enregistrement : réessaie.",
  "autostart-failed": "Impossible de configurer le lancement au démarrage.",
  "enumeration-failed": "Échec de la liste des applications.",
  "enumeration-cancelled": "Liste des applications annulée.",
  "window-failed": "La fenêtre n’a pas pu s’ouvrir. Réessaie dans un instant.",
};

const PALETTE = {
  Working: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Travail en cours" },
  Notice: { stroke: "#1B9DF5", tint: "var(--primary-tint)", leaf: "#0A6FDB", title: "Pause imminente" },
  Break: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "Pause en cours" },
  Returning: { stroke: "#16B37E", tint: "var(--mint-tint)", leaf: "#0E7A57", title: "C’est fini" },
  Suspended: { stroke: "#868DA0", tint: "rgba(28,33,48,.06)", leaf: "#868DA0", title: "Suspendu" },
  Inactive: { stroke: "#868DA0", tint: "rgba(28,33,48,.06)", leaf: "#868DA0", title: "Inactif" },
};

const el = (id) => document.getElementById(id);

function pad(n) {
  return String(n).padStart(2, "0");
}

function formatClock(totalSeconds) {
  const s = Math.max(0, Math.round(totalSeconds));
  const minutes = Math.floor(s / SECONDS_PER_MINUTE);
  if (minutes < MINUTES_PER_HOUR) {
    return `${minutes}:${pad(s % SECONDS_PER_MINUTE)}`;
  }
  return `${Math.floor(minutes / MINUTES_PER_HOUR)}:${pad(minutes % MINUTES_PER_HOUR)}:${pad(s % SECONDS_PER_MINUTE)}`;
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
  return Math.max(1, Math.ceil((dawn - now) / MS_PER_MINUTE));
}

function matchesRhythmPreset(work, pause) {
  return RHYTHM_PRESETS.some(([w, p]) => w === work && p === pause);
}

const BREAK_DUE_PHASES = ["Notice", "Break", "Returning"];

function inactiveTitle(reason) {
  if (reason === "day") {
    return "Jour de repos";
  }
  if (reason === "schedule") {
    return "Hors plage horaire";
  }
  return "Inactif";
}

function workingCopy(snap) {
  if (snap.frozen) {
    return { title: "Travail en cours", sub: "Décompte gelé : tu es inactif", ringSub: "Gelé" };
  }
  const pauseAt = wallTimeIn(snap.break_in_secs);
  return { title: "Travail en cours", sub: "Cycle en cours", ringSub: `Pause à ${pauseAt}` };
}

function suspendedCopy(snap) {
  const resumeAt = wallTimeIn(snap.remaining_secs);
  return { title: "Suspendu", sub: `Reprise automatique à ${resumeAt}`, ringSub: "Avant reprise" };
}

function inactiveCopy(snap) {
  const sub = snap.next_start_label ? `Reprise ${snap.next_start_label}` : "En attente";
  return { title: inactiveTitle(snap.inactive_reason), sub, ringSub: "En veille" };
}

function phaseCopy(snap) {
  switch (snap.phase) {
    case "Working":
      return workingCopy(snap);
    case "Notice":
      return {
        title: "Pause imminente",
        sub: "Finis ta phrase, la pause arrive",
        ringSub: "Avant la pause",
      };
    case "Break":
      return { title: "Pause en cours", sub: "Profite-en", ringSub: "Temps restant" };
    case "Returning":
      return { title: "C’est fini", sub: "Retour au travail", ringSub: "Retour" };
    case "Suspended":
      return suspendedCopy(snap);
    case "Inactive":
      return inactiveCopy(snap);
    default:
      return { title: "Breeze", sub: "—", ringSub: "—" };
  }
}

function announceText(snap) {
  switch (snap.phase) {
    case "Working":
      return snap.frozen ? "Travail en cours, décompte gelé." : "Travail en cours.";
    case "Notice":
      return "Pause imminente.";
    case "Break":
      return "Pause en cours.";
    case "Returning":
      return "Pause terminée, retour au travail.";
    case "Suspended":
      return "Breeze suspendu.";
    case "Inactive":
      return `${inactiveTitle(snap.inactive_reason)}.`;
    default:
      return "";
  }
}

// ---------- painting ----------

function paintHeader(copy, look, frozen) {
  el("phase-title").textContent = copy.title;
  el("phase-sub").textContent = copy.sub;
  el("leaf-badge").style.background = look.tint;
  el("leaf-badge").style.opacity = frozen ? "0.55" : "1";
  el("leaf-path").setAttribute("fill", look.leaf);
}

function ringDash(snap) {
  const elapsed = snap.total_secs > 0 ? (snap.total_secs - snap.remaining_secs) / snap.total_secs : 0;
  return Math.max(0, Math.min(1, elapsed)) * RING_CIRCUMFERENCE;
}

function paintRing(snap, look, copy) {
  const wrap = el("ring-wrap");
  wrap.classList.toggle("frozen", snap.phase === "Working" && !!snap.frozen);
  wrap.classList.toggle("inactive", snap.phase === "Inactive");

  const arc = el("ring-arc");
  arc.setAttribute("stroke", look.stroke);

  const isInactive = snap.phase === "Inactive";
  const dash = isInactive ? 0 : ringDash(snap);
  arc.setAttribute("stroke-dasharray", `${dash.toFixed(1)} ${RING_CIRCUMFERENCE.toFixed(1)}`);
  arc.style.opacity = dash < 1 ? "0" : "1";
  const secs = snap.phase === "Notice" ? snap.break_in_secs : snap.remaining_secs;
  const clock = isInactive ? "—" : formatClock(secs);
  el("countdown").textContent = clock;
  el("countdown").classList.toggle("long", clock.length > 5);
  el("ring-sub").textContent = copy.ringSub;
}

function paintLevers(snap) {
  el("levers").hidden = snap.phase !== "Working";
  el("resume-box").hidden = snap.phase !== "Suspended";
}

// L'hôte refuse sévérité et rythme tant qu'une pause est due (préavis, pause, retour) :
// les leviers sont verrouillés sur ces trois phases, avec une seule note.
function isLocked(snap) {
  return BREAK_DUE_PHASES.includes(snap.phase);
}

function paintSeverity(snap) {
  const locked = isLocked(snap);
  for (const item of document.querySelectorAll("#severity-seg .seg-item")) {
    const on = item.dataset.sev === snap.chosen_severity;
    item.classList.toggle("on", on);
    item.setAttribute("aria-checked", String(on));
    item.tabIndex = on ? 0 : -1;
    item.disabled = locked;
  }
  const note = el("severity-note");
  if (!locked && snap.chosen_severity !== snap.severity) {
    note.textContent = `Retour à ${snap.chosen_severity} au cycle suivant.`;
    note.hidden = false;
  } else {
    note.hidden = true;
  }
}

function isPreset(item, snap) {
  return Number(item.dataset.work) === snap.work_minutes && Number(item.dataset.pause) === snap.pause_minutes;
}

function paintRhythm(snap) {
  const locked = isLocked(snap);
  const custom = !matchesRhythmPreset(snap.work_minutes, snap.pause_minutes);
  for (const item of document.querySelectorAll("#rhythm-seg .seg-item[data-work]")) {
    const on = !custom && isPreset(item, snap);
    item.classList.toggle("on", on);
    item.setAttribute("aria-checked", String(on));
    item.tabIndex = on ? 0 : -1;
    item.disabled = locked;
  }
  const customItem = el("rhythm-custom");
  customItem.hidden = !custom;
  customItem.classList.toggle("locked", locked);
  if (custom) {
    // L'option perso est inerte : le groupe garde un arrêt de tabulation sur le premier préréglage.
    document.querySelector("#rhythm-seg .seg-item[data-work]").tabIndex = 0;
    customItem.textContent = `${snap.work_minutes}/${snap.pause_minutes}`;
    const label = `Rythme personnalisé ${snap.work_minutes}/${snap.pause_minutes}, modifiable dans les réglages`;
    customItem.setAttribute("aria-label", label);
    customItem.classList.add("on");
    customItem.setAttribute("aria-checked", "true");
  }
  const note = el("rhythm-note");
  if (locked) {
    note.textContent = "Sévérité et rythme verrouillés jusqu’à la fin de la pause.";
    note.hidden = false;
  } else if (snap.rhythm_pending) {
    note.textContent = "Appliqué au cycle suivant.";
    note.hidden = false;
  } else {
    note.hidden = true;
  }
}

function paintToday(snap) {
  const served = snap.served_today || 0;
  const plural = served > 1;
  el("today-line").textContent = `${served} pause${plural ? "s" : ""} servie${plural ? "s" : ""}`;
  const debt = el("debt-line");
  if (snap.debt_minutes > 0) {
    debt.textContent = `Dette : ${snap.debt_minutes} min, rendue sur les prochaines pauses.`;
    debt.hidden = false;
  } else {
    debt.hidden = true;
  }
}

let lastAnnounceKey = "";

function announce(snap) {
  const key = `${snap.phase}|${snap.frozen}|${snap.inactive_reason}`;
  if (key === lastAnnounceKey) {
    return;
  }
  lastAnnounceKey = key;
  el("live-region").textContent = announceText(snap);
}

let currentPhase = null;

function render(snap) {
  currentPhase = snap.phase;
  if (!quitNeedsConfirm(snap.phase)) {
    hideQuitConfirm();
  }
  const look = PALETTE[snap.phase] || PALETTE.Inactive;
  const copy = phaseCopy(snap);
  paintHeader(copy, look, snap.frozen);
  paintRing(snap, look, copy);
  paintLevers(snap);
  paintSeverity(snap);
  paintRhythm(snap);
  paintToday(snap);
  announce(snap);
}

function renderNoHost() {
  currentPhase = null;
  el("phase-title").textContent = "Aperçu";
  el("phase-sub").textContent = "Ouvre ce panneau depuis Breeze.";
  el("leaf-badge").style.background = "var(--primary-tint)";
  el("leaf-path").setAttribute("fill", "#0A6FDB");
  el("countdown").textContent = "—";
  el("ring-sub").textContent = "—";
  el("ring-arc").setAttribute("stroke-dasharray", `0 ${RING_CIRCUMFERENCE.toFixed(1)}`);
  el("levers").hidden = true;
  el("resume-box").hidden = true;
  el("severity-note").hidden = true;
  el("rhythm-note").hidden = true;
  el("debt-line").hidden = true;
  el("today-line").textContent = "—";
}

// ---------- toast ----------

let toastTimer = null;

function showToast(message, isError) {
  const toast = el("toast");
  toast.textContent = message;
  toast.classList.toggle("error", !!isError);
  toast.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove("show"), TOAST_MS);
}

// ---------- host bridge ----------

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
    showToast(REFUSALS[String(code)] || "Action refusée.", true);
  } finally {
    poll();
  }
}

// ---------- quit confirmation ----------

// Quitter pendant le préavis ou la pause interrompt une pause due (§10.2).
function quitNeedsConfirm(phase) {
  return phase === "Notice" || phase === "Break";
}

function quitConfirmOpen() {
  return !el("quit-confirm").hidden;
}

function showQuitConfirm() {
  el("footer").hidden = true;
  el("quit-confirm").hidden = false;
  el("quit-cancel").focus();
}

function hideQuitConfirm() {
  if (!quitConfirmOpen()) {
    return;
  }
  const hadFocus = el("quit-confirm").contains(document.activeElement);
  el("quit-confirm").hidden = true;
  el("footer").hidden = false;
  if (hadFocus) {
    el("quit-btn").focus();
  }
}

function wireQuit() {
  el("quit-btn").addEventListener("click", () => {
    if (quitNeedsConfirm(currentPhase)) {
      showQuitConfirm();
    } else {
      send("quit");
    }
  });
  el("quit-cancel").addEventListener("click", hideQuitConfirm);
  el("quit-confirm-btn").addEventListener("click", () => {
    hideQuitConfirm();
    send("quit");
  });
}

// ---------- wiring ----------

function isPickable(item) {
  return !item.hidden && !item.disabled && ("sev" in item.dataset || "work" in item.dataset);
}

// Groupe radio au clavier : les flèches déplacent le focus et choisissent l'option.
function wireRadioArrows(selector) {
  const group = document.querySelector(selector);
  group.addEventListener("keydown", (event) => {
    const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key];
    if (!step) {
      return;
    }
    const items = Array.from(group.querySelectorAll(".seg-item")).filter(isPickable);
    const index = items.indexOf(document.activeElement);
    if (index < 0) {
      return;
    }
    event.preventDefault();
    const next = items[(index + step + items.length) % items.length];
    next.focus();
    next.click();
  });
}

function wireSegGroup(selector, onPick) {
  for (const item of document.querySelectorAll(selector)) {
    item.addEventListener("click", () => onPick(item));
  }
}

function wireControls() {
  wireSegGroup("#severity-seg .seg-item", (item) => send("set_severity", { severity: item.dataset.sev }));
  wireSegGroup("#rhythm-seg .seg-item[data-work]", (item) =>
    send("set_rhythm", { workMinutes: Number(item.dataset.work), pauseMinutes: Number(item.dataset.pause) })
  );
  wireSegGroup("[data-suspend]", (item) => {
    const raw = item.dataset.suspend;
    const minutes = raw === "dawn" ? minutesUntilDawn() : Number(raw);
    send("suspend", { minutes });
  });
  el("resume-btn").addEventListener("click", () => send("resume"));
  el("open-settings").addEventListener("click", () => send("open_settings"));
  el("adjust-rhythm").addEventListener("click", () => send("open_settings"));
  wireQuit();
  wireRadioArrows("#severity-seg");
  wireRadioArrows("#rhythm-seg");
  document.addEventListener("keydown", (event) => {
    if (event.key !== "Escape") {
      return;
    }
    if (quitConfirmOpen()) {
      hideQuitConfirm();
      return;
    }
    send("hide_panel");
  });
}

// ---------- polling ----------

let pollTimer = null;

async function poll() {
  const invoke = invoker();
  if (!invoke) {
    renderNoHost();
    return;
  }
  try {
    render(await invoke("get_snapshot"));
  } catch (_e) {
    // host busy; keep the last frame
  }
}

function startPolling() {
  if (pollTimer) {
    return;
  }
  poll();
  pollTimer = setInterval(poll, POLL_MS);
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer);
    pollTimer = null;
  }
}

document.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    stopPolling();
  } else {
    startPolling();
  }
});
window.addEventListener("focus", poll);

// ---------- window fit ----------

const PANEL_WIDTH = 360;
let fittedHeight = 0;

function fitWindowToPanel() {
  const tauri = window.__TAURI__;
  const win = tauri?.window?.getCurrentWindow?.();
  const LogicalSize = tauri?.dpi?.LogicalSize ?? tauri?.window?.LogicalSize;
  if (!win || !LogicalSize) {
    return;
  }
  const height = Math.ceil(el("panel").getBoundingClientRect().height);
  if (height === fittedHeight) {
    return;
  }
  fittedHeight = height;
  win.setSize(new LogicalSize(PANEL_WIDTH, height)).catch((error) => {
    fittedHeight = 0;
    console.error("panel: resize refused", error);
  });
}

new ResizeObserver(fitWindowToPanel).observe(el("panel"));

wireControls();
if (!document.hidden) {
  startPolling();
} else {
  renderNoHost();
}
