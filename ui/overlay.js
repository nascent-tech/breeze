// Surface de pause plein écran. Elle ne porte aucune logique produit : le décompte
// vient de get_snapshot, la sévérité du paramètre d'URL posé par l'hôte (l'overlay
// est un affichage, pas un décideur). Aucun bouton : rien ne raccourcit une pause.

const RING_CIRCUMFERENCE = 2 * Math.PI * 96;
const POLL_MS = 500;

const el = (id) => document.getElementById(id);

function pad(n) {
  return String(n).padStart(2, "0");
}

function formatClock(totalSeconds) {
  const s = Math.max(0, Math.round(totalSeconds));
  return `${Math.floor(s / 60)}:${pad(s % 60)}`;
}

function applyKind() {
  const kind = new URLSearchParams(location.search).get("kind");
  document.documentElement.dataset.kind = kind === "hardcore" ? "hardcore" : "veil";
}

function paintRing(snap) {
  const elapsed = snap.total_secs > 0 ? (snap.total_secs - snap.remaining_secs) / snap.total_secs : 0;
  const dash = Math.max(0, Math.min(1, elapsed)) * RING_CIRCUMFERENCE;
  el("ring-arc").setAttribute("stroke-dasharray", `${dash.toFixed(1)} ${RING_CIRCUMFERENCE.toFixed(1)}`);
}

function render(snap) {
  el("countdown").textContent = formatClock(snap.remaining_secs);
  paintRing(snap);
}

function invoker() {
  return window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
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

applyKind();
poll();
setInterval(poll, POLL_MS);
