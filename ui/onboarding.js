// Breeze — parcours de première utilisation (cinq écrans, brief §8.7).
// Enchaîne bienvenue → rythme → sévérité → applications → démarrage, mémorise les
// choix et les applique à l'hôte au démarrage. Aucun choix n'est obligatoire :
// fermer la fenêtre conserve ce qui a été sélectionné, le reste garde ses défauts.

const invoke = window.__TAURI__?.core?.invoke;

const steps = Array.from(document.querySelectorAll(".step"));
let current = 0;

const choice = {
  work: 50,
  pause: 10,
  severity: "Simple",
  accessibilityGranted: false,
  // Défaut du brief : tout bloqué. Chaque épargne est un bundle_id → true.
  spared: {},
};

function show(index) {
  current = Math.max(0, Math.min(index, steps.length - 1));
  steps.forEach((step, i) => step.classList.toggle("on", i === current));
  const active = steps[current];
  if (current === steps.length - 1) syncSummary();
  const heading = active.querySelector("h1");
  if (heading) {
    heading.setAttribute("tabindex", "-1");
    heading.focus();
  }
}

function syncSummary() {
  const work = document.getElementById("sum-work");
  const pause = document.getElementById("sum-pause");
  const sev = document.getElementById("sum-sev");
  if (work) work.innerHTML = `${choice.work}&nbsp;min de travail`;
  if (pause) pause.innerHTML = `${choice.pause}&nbsp;min de pause`;
  if (sev) sev.textContent = `Mode ${choice.severity}`;
}

// L'Accessibilité n'est demandée qu'en Mode Simple (brief §8.7).
function syncAccessAsk() {
  const ask = document.getElementById("access-ask");
  if (ask) ask.style.display = choice.severity === "Simple" ? "flex" : "none";
}

// Navigation.
document.querySelectorAll("[data-next]").forEach((el) => el.addEventListener("click", () => show(current + 1)));
document.querySelectorAll("[data-back]").forEach((el) => el.addEventListener("click", () => show(current - 1)));

// Sélection du rythme (role="radio").
document.querySelectorAll(".rhythm-opt").forEach((opt) =>
  opt.addEventListener("click", () => {
    document.querySelectorAll(".rhythm-opt").forEach((o) => o.setAttribute("aria-checked", "false"));
    opt.setAttribute("aria-checked", "true");
    choice.work = Number(opt.dataset.work);
    choice.pause = Number(opt.dataset.pause);
  }),
);

// Sélection de la sévérité (role="radio").
document.querySelectorAll(".sev-opt").forEach((opt) =>
  opt.addEventListener("click", () => {
    document.querySelectorAll(".sev-opt").forEach((o) => o.setAttribute("aria-checked", "false"));
    opt.setAttribute("aria-checked", "true");
    choice.severity = opt.dataset.sev;
    syncAccessAsk();
  }),
);

// Demande d'Accessibilité, portée par l'écran Sévérité.
const grant = document.querySelector("[data-grant-access]");
if (grant) {
  grant.addEventListener("click", async () => {
    if (invoke) {
      try {
        choice.accessibilityGranted = await invoke("request_accessibility");
      } catch (err) {
        console.error("onboarding: demande d'Accessibilité impossible", err);
      }
    }
    grant.textContent = "Demandée…";
    grant.classList.add("disabled");
  });
}

// Épargne d'applications : liste des VRAIES apps installées, chacune un commutateur.
// Noms via textContent, icônes via img.src=data: (jamais innerHTML).
function appRow(app) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "rowitem app-row";
  row.setAttribute("role", "switch");
  row.setAttribute("aria-checked", "false");

  const icon = document.createElement("span");
  icon.className = "appicon";
  icon.setAttribute("aria-hidden", "true");
  if (app.icon_data_url) {
    const img = document.createElement("img");
    img.src = app.icon_data_url;
    img.alt = "";
    img.width = 24;
    img.height = 24;
    img.style.cssText = "border-radius: 6px; display: block";
    icon.style.cssText = "background: transparent; padding: 0";
    icon.appendChild(img);
  } else {
    icon.textContent = (app.name.trim()[0] || "?").toUpperCase();
    icon.style.cssText = "background: linear-gradient(160deg, #9ba3b5, #6c7383)";
  }

  const name = document.createElement("span");
  name.className = "t-label";
  name.style.cssText = "flex: 1; text-align: left";
  name.textContent = app.name;

  const status = document.createElement("span");
  status.className = "app-status t-caption";
  status.style.color = "var(--ink-3)";
  status.textContent = "Bloquée";

  const toggle = document.createElement("span");
  toggle.className = "switch";
  toggle.setAttribute("aria-hidden", "true");

  row.append(icon, name, status, toggle);
  row.addEventListener("click", () => {
    const spared = row.getAttribute("aria-checked") !== "true";
    row.setAttribute("aria-checked", String(spared));
    toggle.classList.toggle("on", spared);
    choice.spared[app.bundle_id] = spared;
    status.textContent = spared ? "Épargnée" : "Bloquée";
    status.style.color = spared ? "var(--mint)" : "var(--ink-3)";
  });
  return row;
}

async function loadApps() {
  const list = document.getElementById("ob-app-list");
  const status = document.getElementById("ob-app-status");
  if (!invoke || !list) {
    if (status) status.textContent = "Aperçu : la liste réelle s’affiche dans l’application.";
    return;
  }
  try {
    const apps = await invoke("list_installed_apps");
    list.textContent = "";
    apps.forEach((app, index) => {
      if (index > 0) {
        const divider = document.createElement("div");
        divider.className = "divider";
        divider.style.cssText = "margin: 0 12px";
        list.appendChild(divider);
      }
      list.appendChild(appRow(app));
    });
  } catch (err) {
    if (status) status.textContent = "Impossible de lire les applications installées.";
    console.error("onboarding: énumération impossible", err);
  }
}

// Démarrage : applique rythme, sévérité et épargnes choisis, puis ferme la fenêtre.
document.querySelectorAll("[data-finish]").forEach((el) =>
  el.addEventListener("click", async () => {
    if (!invoke) return; // Aperçu navigateur : pas d'hôte à qui parler.
    const banner = document.getElementById("start-error");
    try {
      await invoke("set_rhythm", { workMinutes: choice.work, pauseMinutes: choice.pause });
      await invoke("set_severity", { severity: choice.severity });
      await invoke("set_spared_apps", { spared: choice.spared });
      await invoke("finish_onboarding");
    } catch (err) {
      console.error("onboarding: application des réglages impossible", err);
      if (banner) {
        banner.textContent = "Impossible d’enregistrer : Breeze démarre avec 50/10 en Mode Simple. Réessaie.";
        banner.style.display = "block";
      }
    }
  }),
);

show(0);
syncAccessAsk();
loadApps();
