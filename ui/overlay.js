// Surface de pause plein écran. Elle ne porte aucune logique produit : le décompte
// vient de get_snapshot, la sévérité du paramètre d'URL posé par l'hôte. Le seul
// geste (§8.5) n'existe qu'en Mode Hardcore : maintenir Échap dix secondes remplit
// un anneau puis ouvre une confirmation ; le décompte ne s'arrête jamais.

const RING_CIRCUMFERENCE = 2 * Math.PI * 96;
const POLL_MS = 500;
const HOLD_MS = 10000;

const el = (id) => document.getElementById(id);

function pad(n) {
  return String(n).padStart(2, "0");
}

function formatClock(totalSeconds) {
  const s = Math.max(0, Math.round(totalSeconds));
  return `${Math.floor(s / 60)}:${pad(s % 60)}`;
}

function readKind() {
  const kind = new URLSearchParams(location.search).get("kind");
  return kind === "hardcore" ? "hardcore" : "veil";
}

function paintRing(snap) {
  const elapsed = snap.total_secs > 0 ? (snap.total_secs - snap.remaining_secs) / snap.total_secs : 0;
  const dash = Math.max(0, Math.min(1, elapsed)) * RING_CIRCUMFERENCE;
  el("ring-arc").setAttribute("stroke-dasharray", `${dash.toFixed(1)} ${RING_CIRCUMFERENCE.toFixed(1)}`);
}

function render(snap) {
  el("countdown").textContent = formatClock(snap.remaining_secs);
  paintRing(snap);
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

let holdStart = null;
let holdRaf = null;

function tickHold() {
  if (holdStart === null) {
    return;
  }
  const held = performance.now() - holdStart;
  el("hold-fill").style.width = `${Math.min(100, (held / HOLD_MS) * 100)}%`;
  if (held >= HOLD_MS) {
    resetHold();
    openConfirm();
    return;
  }
  holdRaf = requestAnimationFrame(tickHold);
}

function resetHold() {
  holdStart = null;
  if (holdRaf !== null) {
    cancelAnimationFrame(holdRaf);
    holdRaf = null;
  }
  const fill = el("hold-fill");
  if (fill) {
    fill.style.width = "0%";
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

function wireGesture() {
  window.addEventListener("keydown", (event) => {
    if (event.key !== "Escape" || event.repeat) {
      return;
    }
    if (confirmIsOpen()) {
      // Échap dans la boîte = son action par défaut, Annuler.
      closeConfirm();
      return;
    }
    if (holdStart !== null) {
      return;
    }
    holdStart = performance.now();
    holdRaf = requestAnimationFrame(tickHold);
  });
  window.addEventListener("keyup", (event) => {
    if (event.key === "Escape") {
      resetHold();
    }
  });
  // « Dix secondes consécutives » : si le focus part, la touche est relâchée sans
  // qu'on le sache — on abandonne le maintien plutôt que d'ouvrir la confirmation seul.
  window.addEventListener("blur", resetHold);
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
}
poll();
setInterval(poll, POLL_MS);
