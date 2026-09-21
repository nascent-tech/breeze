// Breeze — parcours de première utilisation.
// Enchaîne les six étapes, mémorise les choix de rythme et de sévérité, et les
// applique à l'hôte au démarrage. Aucun choix n'est obligatoire : fermer la
// fenêtre conserve ce qui a été sélectionné, le reste garde ses valeurs par défaut.

const invoke = window.__TAURI__?.core?.invoke;

const steps = Array.from(document.querySelectorAll(".step"));
let current = 0;

const choice = {
  work: 50,
  pause: 10,
  severity: "Simple",
};

function show(index) {
  current = Math.max(0, Math.min(index, steps.length - 1));
  steps.forEach((step, i) => step.classList.toggle("on", i === current));
  if (current === steps.length - 1) syncSummary();
}

function syncSummary() {
  const work = document.getElementById("sum-work");
  const pause = document.getElementById("sum-pause");
  const sev = document.getElementById("sum-sev");
  if (work) work.innerHTML = `${choice.work}&nbsp;min de travail`;
  if (pause) pause.innerHTML = `${choice.pause}&nbsp;min de pause`;
  if (sev) sev.textContent = `Mode ${choice.severity}`;
}

// Navigation : « Continuer », « Passer » avancent d'une étape.
document.querySelectorAll("[data-next]").forEach((el) =>
  el.addEventListener("click", () => show(current + 1)),
);

// Sélection du rythme.
document.querySelectorAll(".rhythm-opt").forEach((opt) =>
  opt.addEventListener("click", () => {
    document.querySelectorAll(".rhythm-opt").forEach((o) => o.classList.remove("on"));
    opt.classList.add("on");
    choice.work = Number(opt.dataset.work);
    choice.pause = Number(opt.dataset.pause);
  }),
);

// Sélection de la sévérité.
document.querySelectorAll(".sev-opt").forEach((opt) =>
  opt.addEventListener("click", () => {
    document.querySelectorAll(".sev-opt").forEach((o) => o.classList.remove("on"));
    opt.classList.add("on");
    choice.severity = opt.dataset.sev;
  }),
);

// Épargne d'applications : bascule le commutateur et l'étiquette d'état.
document.querySelectorAll(".app-row").forEach((row) => {
  const sw = row.querySelector(".switch");
  if (!sw || sw.classList.contains("locked")) return;
  row.addEventListener("click", () => {
    const spared = sw.classList.toggle("on");
    const status = row.querySelector(".app-status");
    if (status) {
      status.textContent = spared ? "Épargnée" : "Bloquée";
      status.style.color = spared ? "var(--mint)" : "var(--ink-3)";
    }
  });
});

// Démarrage : applique le rythme et la sévérité choisis, puis ferme la fenêtre.
document.querySelectorAll("[data-finish]").forEach((el) =>
  el.addEventListener("click", async () => {
    if (!invoke) return; // Aperçu navigateur : pas d'hôte à qui parler.
    try {
      await invoke("set_rhythm", { workMinutes: choice.work, breakMinutes: choice.pause });
      await invoke("set_severity", { severity: choice.severity });
      await invoke("finish_onboarding");
    } catch (err) {
      console.error("onboarding: application des réglages impossible", err);
    }
  }),
);

show(0);
