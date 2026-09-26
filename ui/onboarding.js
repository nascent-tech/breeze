// Breeze — parcours de première utilisation (cinq écrans, brief §8.7).
// Enchaîne bienvenue → rythme → sévérité → applications → démarrage. Il part des
// réglages réels de l'hôte (l'accueil peut être rouvert depuis les Réglages) et, au
// démarrage comme à la fermeture, n'écrit QUE ce que l'utilisateur a changé ici.

const invoke = window.__TAURI__?.core?.invoke;
const NBSP = " ";
const ACCESS_POLL_MS = 1500;
const SEVERITY_STEP = 2;

const el = (id) => document.getElementById(id);
const steps = Array.from(document.querySelectorAll(".step"));
let current = 0;

// Valeurs de l'hôte au chargement (défauts du premier lancement tant qu'elles ne sont pas lues).
const initial = { work: 50, pause: 10, severity: "Simple" };
const choice = { work: 50, pause: 10, severity: "Simple", accessibilityGranted: false };
// Statut épargné affiché, et seulement les bascules faites pendant l'accueil.
const spared = new Map();
const touched = new Map();
let allApps = [];

function show(index) {
  current = Math.max(0, Math.min(index, steps.length - 1));
  steps.forEach((step, i) => step.classList.toggle("on", i === current));
  if (current === steps.length - 1) syncSummary();
  if (current === SEVERITY_STEP) syncAccessAsk();
  else stopAccessPolling();
  const heading = steps[current].querySelector("h1");
  if (heading) {
    heading.setAttribute("tabindex", "-1");
    heading.focus();
  }
}

function pluralizeApp(count) {
  return count > 1 ? `${count} applications épargnées` : `${count} application épargnée`;
}

function sparedCount() {
  return Array.from(spared.values()).filter(Boolean).length;
}

function syncSummary() {
  el("sum-work").textContent = `${choice.work}${NBSP}min de travail`;
  el("sum-pause").textContent = `${choice.pause}${NBSP}min de pause`;
  el("sum-sev").textContent = `Mode ${choice.severity}`;
  el("sum-spared").textContent = pluralizeApp(sparedCount());
  el("plan-pause-desc").textContent =
    choice.severity === "Hardcore"
      ? "Chaque écran est couvert jusqu’à la fin, puis un nouveau cycle démarre."
      : "Un voile couvre l’écran jusqu’à la fin, puis un nouveau cycle démarre.";
}

document.querySelectorAll("[data-next]").forEach((btn) => btn.addEventListener("click", () => show(current + 1)));
document.querySelectorAll("[data-back]").forEach((btn) => btn.addEventListener("click", () => show(current - 1)));

// Entrée = Continuer, sauf si le focus est sur un contrôle qui gère déjà sa
// propre touche Entrée (bouton natif, champ texte).
const OWNS_ENTER = ["BUTTON", "INPUT", "TEXTAREA"];
document.addEventListener("keydown", (event) => {
  if (event.key !== "Enter") return;
  const focused = document.activeElement;
  if (focused && OWNS_ENTER.includes(focused.tagName)) return;
  const btn = steps[current].querySelector("[data-next], [data-finish]");
  if (btn) {
    event.preventDefault();
    btn.click();
  }
});

// Groupe radio : tabindex itinérant, flèches, options masquées ignorées.
function radioItems(group) {
  return Array.from(group.querySelectorAll('[role="radio"]')).filter((item) => !item.hidden);
}

function checkRadio(group, target) {
  radioItems(group).forEach((item) => {
    const checked = item === target;
    item.setAttribute("aria-checked", String(checked));
    item.tabIndex = checked ? 0 : -1;
  });
}

function wireRadioGroup(group, onSelect) {
  group.addEventListener("click", (event) => {
    const item = event.target.closest('[role="radio"]');
    if (!item) return;
    checkRadio(group, item);
    onSelect(item);
  });
  group.addEventListener("keydown", (event) => {
    const step = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 }[event.key];
    const items = radioItems(group);
    const index = items.indexOf(event.target.closest('[role="radio"]'));
    if (!step || index < 0) return;
    event.preventDefault();
    const next = items[(index + step + items.length) % items.length];
    checkRadio(group, next);
    next.focus();
    onSelect(next);
  });
}

wireRadioGroup(el("rhythm-group"), (opt) => {
  choice.work = Number(opt.dataset.work);
  choice.pause = Number(opt.dataset.pause);
});

wireRadioGroup(el("sev-group"), (opt) => {
  choice.severity = opt.dataset.sev;
  syncAccessAsk();
});

function selectRhythm(work, pause) {
  const group = el("rhythm-group");
  let match = radioItems(group).find((item) => Number(item.dataset.work) === work && Number(item.dataset.pause) === pause);
  if (!match) {
    match = el("rhythm-current");
    match.dataset.work = String(work);
    match.dataset.pause = String(pause);
    el("rhythm-current-desc").textContent = `${work}${NBSP}min de travail · ${pause}${NBSP}min de pause`;
    match.hidden = false;
  }
  checkRadio(group, match);
}

function selectSeverity(severity) {
  const group = el("sev-group");
  const match = radioItems(group).find((item) => item.dataset.sev === severity);
  if (match) checkRadio(group, match);
}

// Réglages réels : un accueil rouvert ne doit rien écraser. Sans lecture possible,
// les défauts du premier lancement restent la référence.
async function loadSettings() {
  if (!invoke) return;
  try {
    const settings = await invoke("get_settings");
    initial.work = choice.work = settings.work_minutes;
    initial.pause = choice.pause = settings.pause_minutes;
    initial.severity = choice.severity = settings.chosen_severity || settings.severity;
    selectRhythm(choice.work, choice.pause);
    selectSeverity(choice.severity);
    syncAccessAsk();
  } catch (_error) {
    // Premier lancement sans lecture des réglages : les défauts affichés sont ceux de l'hôte.
  }
}

// L'Accessibilité n'est demandée qu'en Mode Simple (brief §8.7), et seulement
// sondée pendant que l'étape Sévérité est affichée.
let accessRequested = false;
let accessPollTimer = null;

function syncAccessAsk() {
  const showAsk = choice.severity === "Simple";
  el("access-ask").hidden = !showAsk;
  if (showAsk && current === SEVERITY_STEP && !choice.accessibilityGranted) startAccessPolling();
  else stopAccessPolling();
}

function renderAccessState(status) {
  const granted = status === "Granted";
  choice.accessibilityGranted = granted;
  el("access-granted-chip").hidden = !granted;
  el("access-grant").hidden = granted || accessRequested;
  el("access-open-settings").hidden = granted || !accessRequested;
  if (granted) stopAccessPolling();
}

async function pollAccessStatus() {
  if (!invoke) return;
  try {
    renderAccessState(await invoke("accessibility_status"));
  } catch (err) {
    stopAccessPolling();
    el("access-desc").textContent = "État de l’Accessibilité illisible pour l’instant. Tu pourras l’accorder depuis les Réglages.";
    console.error("onboarding: accessibility_status", err);
  }
}

function startAccessPolling() {
  if (accessPollTimer !== null) return;
  pollAccessStatus();
  accessPollTimer = setInterval(pollAccessStatus, ACCESS_POLL_MS);
}

function stopAccessPolling() {
  if (accessPollTimer === null) return;
  clearInterval(accessPollTimer);
  accessPollTimer = null;
}

async function requestAccess() {
  if (!invoke) return;
  accessRequested = true;
  try {
    if (await invoke("request_accessibility")) {
      renderAccessState("Granted");
      return;
    }
  } catch (err) {
    el("access-desc").textContent = "macOS n’a pas pu ouvrir la demande. Réessaie, ou accorde-la plus tard depuis les Réglages.";
    console.error("onboarding: request_accessibility", err);
  }
  await pollAccessStatus();
  if (!choice.accessibilityGranted) startAccessPolling();
}

el("access-grant").addEventListener("click", requestAccess);
el("access-open-settings").addEventListener("click", requestAccess);

// Épargne d'applications : liste des VRAIES apps installées, chacune un commutateur.
// Noms via textContent, icônes via img.src=data: (jamais innerHTML).
function normalize(text) {
  return text.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase().trim();
}

function statusLabel(app, isSpared) {
  if (isSpared) return "Épargnée";
  return app.status === "Ignored" && !touched.has(app.bundle_id) ? "Ignorée" : "Bloquée";
}

function appIcon(app) {
  const icon = document.createElement("span");
  icon.className = "appicon";
  icon.setAttribute("aria-hidden", "true");
  if (app.icon_data_url && app.icon_data_url.startsWith("data:image/")) {
    const img = document.createElement("img");
    img.src = app.icon_data_url;
    img.alt = "";
    icon.classList.add("has-img");
    icon.appendChild(img);
  } else {
    icon.classList.add("fallback");
    icon.textContent = (app.name.trim()[0] || "?").toUpperCase();
  }
  return icon;
}

function paintAppRow(row, app) {
  const isSpared = Boolean(spared.get(app.bundle_id));
  row.setAttribute("aria-checked", String(isSpared));
  row.querySelector(".switch").classList.toggle("on", isSpared);
  const status = row.querySelector(".app-status");
  status.textContent = statusLabel(app, isSpared);
  status.classList.toggle("is-spared", isSpared);
}

function appRow(app) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "rowitem app-row";
  row.setAttribute("role", "switch");

  const name = document.createElement("span");
  name.className = "t-label app-name";
  name.textContent = app.name;
  name.title = app.name;

  const status = document.createElement("span");
  status.className = "app-status t-caption";

  const toggle = document.createElement("span");
  toggle.className = "switch";
  toggle.setAttribute("aria-hidden", "true");

  row.append(appIcon(app), name, status, toggle);
  paintAppRow(row, app);
  row.addEventListener("click", () => {
    const next = !spared.get(app.bundle_id);
    spared.set(app.bundle_id, next);
    // Revenir à l'état d'origine annule la bascule : une app Ignorée le reste.
    if (next === (app.status === "Spared")) touched.delete(app.bundle_id);
    else touched.set(app.bundle_id, next);
    paintAppRow(row, app);
    updateAppCounter();
  });
  return row;
}

function updateAppCounter() {
  el("app-counter").textContent = pluralizeApp(sparedCount());
}

function listMessage(className, text) {
  const box = document.createElement("div");
  box.className = `${className} t-caption`;
  box.textContent = text;
  return box;
}

function renderAppList(filterText) {
  const list = el("ob-app-list");
  const query = normalize(filterText);
  const visible = query ? allApps.filter((app) => normalize(app.name).includes(query)) : allApps;
  if (visible.length === 0) {
    const text = query ? `Aucune application ne correspond à «${NBSP}${filterText.trim()}${NBSP}».` : "Aucune application installée trouvée.";
    list.replaceChildren(listMessage("app-empty", text));
    return;
  }
  const rows = [];
  visible.forEach((app, index) => {
    if (index > 0) {
      const divider = document.createElement("div");
      divider.className = "divider app-divider";
      rows.push(divider);
    }
    rows.push(appRow(app));
  });
  list.replaceChildren(...rows);
}

function renderAppSkeletons() {
  const rows = [0, 1, 2, 3].map((i) => {
    const row = document.createElement("div");
    row.className = "app-skeleton-row";
    row.setAttribute("aria-hidden", "true");
    const icon = document.createElement("span");
    icon.className = "skl skl-icon";
    const bar = document.createElement("span");
    bar.className = "skl";
    bar.style.width = `${120 + ((i * 37) % 90)}px`;
    row.append(icon, bar);
    return row;
  });
  el("ob-app-list").replaceChildren(...rows);
}

function renderAppError() {
  const wrap = listMessage("app-error", "");
  const message = document.createElement("span");
  message.textContent = "Impossible de lire les applications installées.";
  const retry = document.createElement("button");
  retry.type = "button";
  retry.className = "btn btn-secondary btn-sm";
  retry.textContent = "Réessayer";
  retry.addEventListener("click", loadApps);
  wrap.replaceChildren(message, retry);
  el("ob-app-list").replaceChildren(wrap);
}

async function loadApps() {
  if (!invoke) {
    el("ob-app-list").replaceChildren(listMessage("app-empty", `Aperçu${NBSP}: la liste réelle s’affiche dans l’application.`));
    return;
  }
  renderAppSkeletons();
  try {
    allApps = await invoke("list_installed_apps");
    allApps.forEach((app) => {
      if (!touched.has(app.bundle_id)) spared.set(app.bundle_id, app.status === "Spared");
    });
    renderAppList(el("app-search").value);
    updateAppCounter();
  } catch (err) {
    console.error("onboarding: list_installed_apps", err);
    renderAppError();
  }
}

el("app-search").addEventListener("input", (event) => renderAppList(event.target.value));

// N'écrit que ce qui a changé pendant l'accueil ; chaque commande est indépendante.
// Renvoie true si au moins une commande a échoué.
function pendingWrites() {
  const writes = [];
  if (choice.work !== initial.work || choice.pause !== initial.pause) {
    writes.push(["set_rhythm", { workMinutes: choice.work, pauseMinutes: choice.pause }]);
  }
  if (choice.severity !== initial.severity) {
    writes.push(["set_severity", { severity: choice.severity }]);
  }
  if (touched.size > 0) {
    writes.push(["set_spared_apps", { spared: Object.fromEntries(touched) }]);
  }
  return writes;
}

async function applyChoices() {
  let failed = false;
  for (const [name, args] of pendingWrites()) {
    try {
      await invoke(name, args);
    } catch (err) {
      failed = true;
      console.error(`onboarding: ${name}`, err);
    }
  }
  return failed;
}

// Démarrage ou fermeture de la fenêtre : même promesse (« ce que tu as choisi
// est conservé »). finish_onboarding est TOUJOURS appelé pour ne jamais
// bloquer l'utilisateur ; seul son propre échec affiche un message + réessai.
let finishing = false;

function setFinishBusy(busy, label) {
  const finishBtn = document.querySelector("[data-finish]");
  finishBtn.disabled = busy;
  finishBtn.toggleAttribute("aria-busy", busy);
  finishBtn.textContent = label;
}

function showStartError(text) {
  const banner = el("start-error");
  banner.textContent = text;
  banner.hidden = !text;
}

async function finishOnboarding(options = {}) {
  if (!invoke || finishing) return;
  finishing = true;
  showStartError("");
  if (options.busyButton) setFinishBusy(true, "Démarrage…");
  const applyFailed = await applyChoices();
  try {
    await invoke("finish_onboarding");
  } catch (err) {
    console.error("onboarding: finish_onboarding", err);
    finishing = false;
    setFinishBusy(false, "Réessayer");
    if (current !== steps.length - 1) show(steps.length - 1);
    showStartError("Impossible de terminer l’accueil. Réessaie.");
    return;
  }
  if (applyFailed) {
    showStartError(`Certains choix n’ont pas pu être enregistrés${NBSP}: ajuste-les dans les Réglages.`);
  }
}

document.querySelector("[data-finish]").addEventListener("click", () => finishOnboarding({ busyButton: true }));

// Fermer la fenêtre (feu rouge natif ou ⌘W) tient la même promesse que « Démarrer ».
async function wireWindowClose() {
  const win = window.__TAURI__?.window?.getCurrentWindow?.();
  if (!win) return;
  try {
    await win.onCloseRequested(async (event) => {
      event.preventDefault();
      await finishOnboarding();
    });
  } catch (err) {
    console.error("onboarding: onCloseRequested", err);
  }
}

show(0);
loadSettings();
loadApps();
wireWindowClose();
