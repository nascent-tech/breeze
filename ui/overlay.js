// Surface de pause, plein écran ou posée sur une fenêtre. Elle ne porte aucune logique
// produit : le décompte vient de get_snapshot, le genre de surface du paramètre d'URL
// posé par l'hôte. Le seul
// geste (§8.5) n'existe qu'en Mode Hardcore : maintenir Échap dix secondes remplit
// un anneau puis ouvre une confirmation ; le décompte ne s'arrête jamais.

const RING_CIRCUMFERENCE = 2 * Math.PI * 96;
const POLL_MS = 500;
const HOLD_MS = 10000;
const LEGEND = "Maintiens Échap pour sortir";
// macOS ne laisse aucune application prendre le clavier d'elle-même : tant que la surface ne
// l'a pas, le premier clic le lui donne.
const LEGEND_WITHOUT_KEYBOARD = "Clique sur l’écran, puis maintiens Échap pour sortir";

function restingLegend() {
  return document.hasFocus() ? LEGEND : LEGEND_WITHOUT_KEYBOARD;
}

const SUGGESTIONS = [
  "Regarde au loin pendant 20 secondes.",
  "Lève-toi et marche un peu.",
  "Bois un verre d’eau.",
  "Étire tes épaules.",
  "Respire lentement, quelques cycles.",
];

function pickSuggestion() {
  const buffer = new Uint32Array(1);
  crypto.getRandomValues(buffer);
  return SUGGESTIONS[buffer[0] % SUGGESTIONS.length];
}
const suggestion = pickSuggestion();

const el = (id) => document.getElementById(id);

function pad(n) {
  return String(n).padStart(2, "0");
}

function formatClock(totalSeconds) {
  const s = Math.max(0, Math.round(totalSeconds));
  return `${Math.floor(s / 60)}:${pad(s % 60)}`;
}

// Même écriture que le panneau et l'hôte : « 9:05 ».
function formatHour(date) {
  return `${date.getHours()}:${pad(date.getMinutes())}`;
}

// « veil » : voile plein écran d'un moniteur ; « window » : voile d'une seule fenêtre
// bloquée ; « hardcore » : overlay opaque. Toute autre valeur retombe sur le voile.
const KINDS = ["veil", "window", "hardcore"];

function readKind() {
  const kind = new URLSearchParams(location.search).get("kind");
  return KINDS.includes(kind) ? kind : "veil";
}

function paintRing(snap) {
  if (snap.phase === "Returning") {
    el("ring-arc").setAttribute("stroke-dasharray", `0 ${RING_CIRCUMFERENCE.toFixed(1)}`);
    return;
  }
  // Comme le minuteur d'Horloge : l'anneau montre ce qu'il reste, pas ce qui est passé.
  const left = snap.total_secs > 0 ? snap.remaining_secs / snap.total_secs : 0;
  const dash = Math.max(0, Math.min(1, left)) * RING_CIRCUMFERENCE;
  el("ring-arc").setAttribute("stroke-dasharray", `${dash.toFixed(1)} ${RING_CIRCUMFERENCE.toFixed(1)}`);
}

function paintPhaseCopy(snap) {
  const title = el("phase-title");
  const subtitle = el("phase-subtitle");
  if (snap.phase === "Returning") {
    title.textContent = "C’est fini";
    subtitle.textContent = "Bon retour";
    subtitle.style.display = "block";
  } else {
    title.textContent = "Pause";
    subtitle.style.display = "none";
  }
}

function paintResumeAt(snap) {
  const resumeAt = el("resume-at");
  if (snap.phase !== "Break" || snap.remaining_secs <= 0) {
    resumeAt.style.display = "none";
    return;
  }
  const at = new Date(Date.now() + snap.remaining_secs * 1000);
  resumeAt.textContent = `Reprise à ${formatHour(at)}`;
  resumeAt.style.display = "block";
}

function paintSuggestion(snap) {
  const box = el("suggestion");
  if (readKind() !== "veil" || snap.phase !== "Break") {
    box.style.display = "none";
    return;
  }
  box.textContent = suggestion;
  box.style.display = "block";
}

let lastAnnounced = null;

function announceRemaining(snap) {
  const region = el("sr-remaining");
  if (snap.phase === "Returning") {
    if (lastAnnounced !== "returning") {
      region.textContent = "C’est fini, bon retour.";
      lastAnnounced = "returning";
    }
    return;
  }
  const minutes = Math.ceil(snap.remaining_secs / 60);
  if (minutes === lastAnnounced) return;
  lastAnnounced = minutes;
  const plural = minutes > 1 ? "s" : "";
  region.textContent = minutes > 0 ? `${minutes} minute${plural} restante${plural}.` : "Moins d’une minute restante.";
}

let currentPhase = null;

function render(snap) {
  currentPhase = snap.phase;
  document.documentElement.dataset.phase = snap.phase;
  el("countdown").textContent = formatClock(snap.remaining_secs);
  paintRing(snap);
  paintPhaseCopy(snap);
  paintResumeAt(snap);
  paintSuggestion(snap);
  announceRemaining(snap);
  // Si l'échéance de la pause tombe pendant le maintien ou la confirmation, la pause
  // est servie et la confirmation se ferme sans effet (§8.5/§10.1).
  if (snap.phase !== "Break") {
    resetHold();
    closeConfirm();
  }
}

function invoker() {
  return window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
}

async function send(command) {
  const invoke = invoker();
  if (invoke) {
    try {
      await invoke(command);
    } catch (_e) {
      // l'hôte a refusé (p. ex. pause déjà servie) : rien à faire ici
    }
  }
}

let polling = false;

async function poll() {
  const invoke = invoker();
  if (!invoke) {
    el("phase-title").textContent = "En attente de l’hôte…";
    return;
  }
  if (polling) return; // évite d'empiler des appels si l'hôte répond lentement
  polling = true;
  try {
    render(await invoke("get_snapshot"));
  } catch (_e) {
    // host busy; keep the last frame
  } finally {
    polling = false;
  }
}

// L'échéance du maintien est une minuterie, pas l'animation : le geste aboutit même
// si le rendu est ralenti. L'intervalle ne sert qu'à peindre la jauge et la légende.
const HOLD_PAINT_MS = 50;
let holdStart = null;
let holdPaint = null;
let holdDone = null;

function paintHold() {
  if (holdStart === null) {
    return;
  }
  const held = performance.now() - holdStart;
  const remainingMs = Math.max(0, HOLD_MS - held);
  el("hold-fill").style.width = `${Math.min(100, (held / HOLD_MS) * 100)}%`;
  const remainingSeconds = Math.ceil(remainingMs / 1000);
  // La mention reste lisible pendant le maintien (§8.5) ; seul le restant s'y ajoute.
  el("legend").textContent = `${LEGEND} — encore ${remainingSeconds}\u00a0s`;
}

function startHold() {
  holdStart = performance.now();
  paintHold();
  holdPaint = setInterval(paintHold, HOLD_PAINT_MS);
  holdDone = setTimeout(() => {
    resetHold();
    openConfirm();
  }, HOLD_MS);
}

function resetHold() {
  holdStart = null;
  clearInterval(holdPaint);
  clearTimeout(holdDone);
  holdPaint = null;
  holdDone = null;
  const fill = el("hold-fill");
  if (fill) {
    fill.style.width = "0%";
  }
  paintRestingLegend();
}

function paintRestingLegend() {
  const legend = el("legend");
  if (legend && holdStart === null) {
    legend.textContent = restingLegend();
  }
}

function confirmIsOpen() {
  return document.documentElement.dataset.confirm === "open";
}

function openConfirm() {
  document.documentElement.dataset.confirm = "open";
  el("confirm-cancel").focus();
}

function closeConfirm() {
  delete document.documentElement.dataset.confirm;
}

const CONFIRM_FOCUSABLE_IDS = ["confirm-cancel", "confirm-ok"];

function trapConfirmFocus(event) {
  if (event.key !== "Tab" || !confirmIsOpen()) return;
  const focusables = CONFIRM_FOCUSABLE_IDS.map(el);
  const index = focusables.indexOf(document.activeElement);
  event.preventDefault();
  const step = event.shiftKey ? -1 : 1;
  const next = (index + step + focusables.length) % focusables.length;
  focusables[next].focus();
}

function wireGesture() {
  window.addEventListener("keydown", (event) => {
    if (event.key === "Tab") {
      trapConfirmFocus(event);
      return;
    }
    if (event.key !== "Escape" || event.repeat) {
      return;
    }
    if (confirmIsOpen()) {
      // Échap dans la boîte = son action par défaut, Annuler.
      closeConfirm();
      return;
    }
    // Le geste n'existe que pendant la pause : ni au retour, ni avant le premier instantané.
    if (holdStart !== null || currentPhase !== "Break") {
      return;
    }
    startHold();
  });
  window.addEventListener("keyup", (event) => {
    if (event.key === "Escape") {
      resetHold();
    }
  });
  // « Dix secondes consécutives » : si le focus part, la touche est relâchée sans
  // qu'on le sache — on abandonne le maintien plutôt que d'ouvrir la confirmation seul.
  window.addEventListener("blur", resetHold);
  window.addEventListener("focus", paintRestingLegend);
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) {
      resetHold();
    }
  });
  el("confirm-cancel").addEventListener("click", closeConfirm);
  el("confirm-ok").addEventListener("click", () => {
    closeConfirm();
    send("interrupt_break");
  });
}

document.documentElement.dataset.kind = readKind();
if (readKind() === "hardcore") {
  wireGesture();
  paintRestingLegend();
}
poll();
setInterval(poll, POLL_MS);
