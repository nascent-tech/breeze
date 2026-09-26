// Breeze — fenêtre Réglages. Chaque contrôle écrit par une commande de l'hôte : état
// optimiste, retour arrière et message en cas de refus. Les données de l'hôte (noms
// d'applications) n'entrent que par textContent. Hors application (navigateur), un
// hôte d'aperçu en mémoire garde la page présentable.

const hostInvoke = window.__TAURI__?.core?.invoke;
const PREVIEW = typeof hostInvoke !== "function";
const TAB_KEY = "breeze.settings.tab";
const PAUSE_MAX = 60;
const AX_POLL_MS = 1500;
const AX_POLL_TICKS = 8;
const REPORT_URL = "https://github.com/nascent-tech/breeze/issues/new";
const KEY_NAMES = { "⌥": "Option", "⌘": "Commande", "⇧": "Maj", "⌃": "Contrôle" };

const ERROR_MESSAGES = {
  "break-due": "Une pause est due : réessaie après elle.",
  "invalid-rhythm": "Rythme refusé : travail de 5 à 180 min, pause de 1 à 60 min, pas plus longue que le travail.",
  "invalid-active-days": "Garde au moins un jour actif.",
  "invalid-schedule": "Plage refusée : le début et la fin doivent être différents.",
  "invalid-app-id": "Cette application n’a pas pu être identifiée.",
  "unknown-status": "Statut d’application inconnu.",
  "unknown-severity": "Sévérité inconnue.",
  "persistence-failed": "Réglage non enregistré : Breeze n’a pas pu écrire sur le disque. Réessaie.",
  "autostart-failed": "macOS a refusé de changer l’ouverture à la session. Réessaie dans un instant.",
  "enumeration-failed": "Impossible de lire les applications installées.",
  "enumeration-cancelled": "La lecture des applications a été interrompue.",
  "invalid-url": "Adresse refusée : seul un lien https peut s’ouvrir.",
  "open-failed": "Le navigateur n’a pas pu s’ouvrir. Réessaie dans un instant.",
  "window-failed": "La fenêtre n’a pas pu s’ouvrir. Réessaie dans un instant.",
  preview: "Aperçu navigateur : cette action n’existe que dans l’application.",
};
const SEVERITY_ERRORS = { "break-due": "Une pause est due : change la sévérité après elle." };
const AX_LOOK = {
  Granted: { text: "Accordée", tone: "chip-mint" },
  Denied: { text: "Refusée", tone: "chip-warn" },
  Unknown: { text: "Non vérifiée", tone: "chip-neutral" },
};
const DATE_FORMAT = new Intl.DateTimeFormat("fr-FR", { weekday: "short", day: "numeric", month: "short" });
const SHORT_DATE_FORMAT = new Intl.DateTimeFormat("fr-FR", { day: "numeric", month: "short" });

const $ = (id) => document.getElementById(id);
const state = { settings: null, snapshot: null, apps: [], axStatus: null, axTimer: null, toastTimer: null };

const preview = {
  settings: {
    work_minutes: 50, pause_minutes: 10, severity: "Simple", chosen_severity: "Simple", severity_pending: false,
    active_days: 0b0011111, schedule_enabled: true, schedule_start: 540, schedule_end: 1110,
    update_check: true, launch_at_login: false, sounds: true, menubar_text: true, shortcut: "⌥⌘B",
  },
  snapshot: { phase: "Working", rhythm_pending: false, inactive_reason: null, next_start_label: null },
  apps: [
    ["com.apple.Safari", "Safari", "Blocked"], ["com.apple.mail", "Mail", "Blocked"],
    ["com.apple.Music", "Musique", "Spared"], ["com.apple.Notes", "Notes", "Blocked"],
    ["com.tinyspeck.slackmacgap", "Slack", "Ignored"], ["com.microsoft.VSCode", "Visual Studio Code", "Blocked"],
  ].map(([bundle_id, name, status]) => ({ bundle_id, name, icon_data_url: null, status })),
};
const PREVIEW_MINUTES = [36, 40, 0, 0, 30, 28, 44, 32, 36, 0, 0, 22, 26, 40, 36,
  30, 0, 0, 20, 34, 42, 28, 36, 24, 0, 0, 38, 30, 40, 18];

function isoDaysAgo(offset) {
  const day = new Date();
  day.setDate(day.getDate() - offset);
  const pad = (n) => String(n).padStart(2, "0");
  return `${day.getFullYear()}-${pad(day.getMonth() + 1)}-${pad(day.getDate())}`;
}

function previewStats() {
  const days = PREVIEW_MINUTES.map((minutes, i) => ({
    date: isoDaysAgo(PREVIEW_MINUTES.length - 1 - i),
    served: Math.round(minutes / 10), interrupted: i === 20 ? 1 : 0, validated: 0, served_minutes: minutes,
  }));
  const sum = (key) => days.reduce((total, day) => total + day[key], 0);
  const served = sum("served");
  const interrupted = sum("interrupted");
  return {
    days, served, interrupted, validated: 0, due: served + interrupted, served_minutes: sum("served_minutes"),
    emergency_exits: 1, emergency_minutes: 8, debt_minutes: 4,
  };
}

const PREVIEW_HOST = {
  get_settings: () => ({ ...preview.settings }),
  get_snapshot: () => ({ ...preview.snapshot }),
  get_stats: previewStats,
  get_app_info: () => ({ name: "Breeze", version: "(aperçu)" }),
  list_installed_apps: () => preview.apps.map((app) => ({ ...app })),
  accessibility_status: () => "Unknown",
  set_launch_at_login: ({ enabled }) => Object.assign(preview.settings, { launch_at_login: enabled }),
  set_sounds: ({ enabled }) => Object.assign(preview.settings, { sounds: enabled }),
  set_update_check: ({ enabled }) => Object.assign(preview.settings, { update_check: enabled }),
  set_menubar_mode: ({ text }) => Object.assign(preview.settings, { menubar_text: text }),
  set_rhythm: ({ workMinutes, pauseMinutes }) =>
    Object.assign(preview.settings, { work_minutes: workMinutes, pause_minutes: pauseMinutes }),
  set_schedule: ({ enabled, start, end }) =>
    Object.assign(preview.settings, { schedule_enabled: enabled, schedule_start: start, schedule_end: end }),
  set_active_days: ({ mask }) => Object.assign(preview.settings, { active_days: mask }),
  set_severity: ({ severity }) => Object.assign(preview.settings, { severity, chosen_severity: severity }),
  set_app_status: ({ bundleId, status }) => {
    preview.apps.find((app) => app.bundle_id === bundleId).status = status;
  },
  reset_settings: () => {
    Object.assign(preview.settings, {
      work_minutes: 50, pause_minutes: 10, severity: "Simple", chosen_severity: "Simple", active_days: 0b1111111,
      schedule_enabled: false, sounds: true, menubar_text: true,
    });
    preview.apps.forEach((app) => { app.status = "Blocked"; });
  },
};

function call(command, args) {
  if (!PREVIEW) return hostInvoke(command, args);
  const handler = PREVIEW_HOST[command];
  return handler ? Promise.resolve(handler(args || {})) : Promise.reject("preview");
}

function errorMessage(error, overrides = {}) {
  const code = typeof error === "string" ? error : error?.message;
  return overrides[code] || ERROR_MESSAGES[code] || "Action impossible pour l’instant. Réessaie dans un instant.";
}

function showToast(text, { error = false } = {}) {
  const toast = $("toast");
  toast.textContent = text;
  toast.classList.toggle("error", error);
  toast.classList.add("show");
  clearTimeout(state.toastTimer);
  state.toastTimer = setTimeout(() => toast.classList.remove("show"), error ? 5000 : 3000);
}

// Un refus connu de l'hôte est un résultat attendu, déjà expliqué par le message :
// seules les erreurs inattendues vont à la console.
function toastError(error, overrides) {
  const code = typeof error === "string" ? error : error?.message;
  if (!PREVIEW && !(code in ERROR_MESSAGES)) console.error("settings:", error);
  showToast(errorMessage(error, overrides), { error: true });
}

function plural(count, one, many) {
  return `${count} ${count > 1 ? many : one}`;
}

function isChecked(control) {
  return control.getAttribute("aria-checked") === "true";
}

function setChecked(control, on) {
  control.setAttribute("aria-checked", String(on));
}

function readStoredTab() {
  try {
    return sessionStorage.getItem(TAB_KEY);
  } catch {
    return null;
  }
}

function storeTab(key) {
  try {
    sessionStorage.setItem(TAB_KEY, key);
  } catch {
    // Stockage indisponible : l'onglet ne sera simplement pas mémorisé.
  }
}

function tabs() {
  return Array.from(document.querySelectorAll('[role="tab"]'));
}

function selectTab(key, { focus = false } = {}) {
  tabs().forEach((tab) => {
    const on = tab.dataset.tab === key;
    tab.classList.toggle("on", on);
    tab.setAttribute("aria-selected", String(on));
    tab.tabIndex = on ? 0 : -1;
    if (on) $("pane-title").textContent = tab.querySelector(".tab-label").textContent;
    if (on && focus) tab.focus();
  });
  document.querySelectorAll('[role="tabpanel"]').forEach((panel) => {
    panel.hidden = panel.dataset.panel !== key;
  });
  $("panels").scrollTop = 0;
  storeTab(key);
}

function onTabKeydown(event) {
  const list = tabs();
  const index = list.indexOf(event.target);
  const moves = { ArrowDown: index + 1, ArrowRight: index + 1, ArrowUp: index - 1, ArrowLeft: index - 1,
    Home: 0, End: list.length - 1 };
  if (!(event.key in moves)) return;
  event.preventDefault();
  const next = list[(moves[event.key] + list.length) % list.length];
  selectTab(next.dataset.tab, { focus: true });
}

function wireTabs() {
  tabs().forEach((tab) => {
    tab.addEventListener("click", () => selectTab(tab.dataset.tab));
    tab.addEventListener("keydown", onTabKeydown);
  });
  const panels = $("panels");
  panels.addEventListener("scroll", () => $("pane-head").classList.toggle("scrolled", panels.scrollTop > 0));
  const stored = readStoredTab();
  selectTab(tabs().some((tab) => tab.dataset.tab === stored) ? stored : "general");
}

function markRadio(group, value) {
  group.querySelectorAll('[role="radio"]').forEach((radio) => {
    const on = radio.dataset.value === value;
    setChecked(radio, on);
    radio.classList.toggle("on", on);
    radio.tabIndex = on ? 0 : -1;
  });
}

function wireRadioGroups(root, onChoose) {
  root.addEventListener("click", (event) => {
    const radio = event.target.closest('[role="radio"]');
    if (radio && !isChecked(radio)) onChoose(radio.closest('[role="radiogroup"]'), radio.dataset.value);
  });
  root.addEventListener("keydown", (event) => {
    const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key];
    const radio = event.target.closest('[role="radio"]');
    if (!step || !radio) return;
    event.preventDefault();
    const group = radio.closest('[role="radiogroup"]');
    const list = Array.from(group.querySelectorAll('[role="radio"]'));
    const next = list[(list.indexOf(radio) + step + list.length) % list.length];
    next.focus();
    onChoose(group, next.dataset.value);
  });
}

function wireSwitch(id, command) {
  const control = $(id);
  control.addEventListener("click", async () => {
    if (control.dataset.busy) return;
    const enabled = !isChecked(control);
    setChecked(control, enabled);
    control.dataset.busy = "1";
    try {
      await call(command, { enabled });
    } catch (error) {
      setChecked(control, !enabled);
      toastError(error);
    } finally {
      delete control.dataset.busy;
    }
  });
}

async function chooseMenubarMode(group, value) {
  const previous = state.settings?.menubar_text ? "text" : "icon";
  markRadio(group, value);
  try {
    await call("set_menubar_mode", { text: value === "text" });
    if (state.settings) state.settings.menubar_text = value === "text";
  } catch (error) {
    markRadio(group, previous);
    toastError(error);
  }
}

function renderShortcut(shortcut) {
  const keys = Array.from(shortcut || "⌥⌘B");
  document.querySelectorAll(".js-shortcut").forEach((holder) => {
    holder.replaceChildren(...keys.map((key) => {
      const kbd = document.createElement("span");
      kbd.className = "kbd";
      kbd.textContent = key;
      return kbd;
    }));
    holder.setAttribute("aria-label", keys.map((key) => KEY_NAMES[key] || key).join(" "));
  });
}

function applySettings(settings) {
  state.settings = settings;
  setChecked($("launch-switch"), settings.launch_at_login);
  setChecked($("sounds-switch"), settings.sounds);
  setChecked($("update-switch"), settings.update_check);
  markRadio($("menubar-seg"), settings.menubar_text ? "text" : "icon");
  renderShortcut(settings.shortcut);
  applyRhythm(settings.work_minutes, settings.pause_minutes);
  applySchedule(settings);
  applyDays(settings.active_days);
  applySeverity(settings);
}

async function loadSettings() {
  try {
    applySettings(await call("get_settings"));
  } catch (error) {
    toastError(error);
  }
}

function inactiveText(snapshot) {
  if (snapshot.phase !== "Inactive" || !snapshot.inactive_reason) return "";
  const why = snapshot.inactive_reason === "day"
    ? "Aujourd’hui n’est pas un jour actif : Breeze est en veille."
    : "Hors plage horaire : Breeze est en veille.";
  return snapshot.next_start_label ? `${why} Reprise ${snapshot.next_start_label}.` : why;
}

function applySnapshot(snapshot) {
  state.snapshot = snapshot;
  $("rhythm-pending").hidden = !snapshot.rhythm_pending;
  const note = $("inactive-note");
  note.textContent = inactiveText(snapshot);
  note.hidden = !note.textContent;
}

async function refreshSnapshot() {
  try {
    applySnapshot(await call("get_snapshot"));
  } catch (error) {
    toastError(error);
  }
}

function paintRange(input, output) {
  const min = Number(input.min);
  const max = Number(input.max);
  const value = Number(input.value);
  const percent = max > min ? ((value - min) / (max - min)) * 100 : 100;
  input.style.setProperty("--fill", `${percent}%`);
  input.setAttribute("aria-valuetext", `${value} minutes`);
  output.textContent = `${value} min`;
}

function paintRhythm() {
  $("pause-range").max = String(Math.min(PAUSE_MAX, Number($("work-range").value)));
  paintRange($("work-range"), $("work-value"));
  paintRange($("pause-range"), $("pause-value"));
}

function applyRhythm(work, pause) {
  $("work-range").value = String(work);
  $("pause-range").max = String(Math.min(PAUSE_MAX, work));
  $("pause-range").value = String(pause);
  paintRhythm();
}

async function saveRhythm() {
  const workMinutes = Number($("work-range").value);
  const pauseMinutes = Number($("pause-range").value);
  const saved = state.settings;
  if (saved && saved.work_minutes === workMinutes && saved.pause_minutes === pauseMinutes) return;
  try {
    await call("set_rhythm", { workMinutes, pauseMinutes });
    if (saved) Object.assign(saved, { work_minutes: workMinutes, pause_minutes: pauseMinutes });
    await refreshSnapshot();
  } catch (error) {
    if (saved) applyRhythm(saved.work_minutes, saved.pause_minutes);
    toastError(error);
  }
}

function wireRhythm() {
  ["work-range", "pause-range"].forEach((id) => {
    $(id).addEventListener("input", paintRhythm);
    $(id).addEventListener("change", saveRhythm);
  });
}

function toTimeValue(minutes) {
  const pad = (n) => String(n).padStart(2, "0");
  return `${pad(Math.floor(minutes / 60))}:${pad(minutes % 60)}`;
}

function fromTimeValue(text) {
  const match = /^(\d{2}):(\d{2})/.exec(text);
  return match ? Number(match[1]) * 60 + Number(match[2]) : null;
}

function applySchedule(settings) {
  const enabled = settings.schedule_enabled;
  setChecked($("schedule-switch"), enabled);
  [["schedule-start", settings.schedule_start], ["schedule-end", settings.schedule_end]].forEach(([id, minutes]) => {
    const input = $(id);
    if (input !== document.activeElement) input.value = toTimeValue(minutes);
    input.disabled = !enabled;
  });
}

function showScheduleMessage(text) {
  $("schedule-msg").textContent = text;
  $("schedule-msg").hidden = !text;
}

function scheduleRefusal(enabled, start, end) {
  if (!enabled) return "";
  if (start === null || end === null) return "Indique une heure complète, par exemple 09:00.";
  if (start === end) return "Le début et la fin doivent être différents : une plage de zéro minute n’existe pas.";
  return "";
}

async function saveSchedule() {
  const saved = state.settings;
  const enabled = isChecked($("schedule-switch"));
  const start = fromTimeValue($("schedule-start").value);
  const end = fromTimeValue($("schedule-end").value);
  const refusal = scheduleRefusal(enabled, start, end);
  showScheduleMessage(refusal);
  if (refusal) {
    document.activeElement.blur();
    if (saved) applySchedule(saved);
    return;
  }
  try {
    await call("set_schedule", { enabled, start, end });
    if (saved) Object.assign(saved, { schedule_enabled: enabled, schedule_start: start, schedule_end: end });
    applySchedule(saved || { schedule_enabled: enabled, schedule_start: start, schedule_end: end });
    await refreshSnapshot();
  } catch (error) {
    if (saved) applySchedule(saved);
    toastError(error);
  }
}

function wireSchedule() {
  $("schedule-switch").addEventListener("click", () => {
    const enabled = !isChecked($("schedule-switch"));
    setChecked($("schedule-switch"), enabled);
    $("schedule-start").disabled = !enabled;
    $("schedule-end").disabled = !enabled;
    saveSchedule();
  });
  $("schedule-start").addEventListener("change", saveSchedule);
  $("schedule-end").addEventListener("change", saveSchedule);
}

function dayButtons() {
  return Array.from(document.querySelectorAll(".day-toggle"));
}

function applyDays(mask) {
  dayButtons().forEach((button, i) => {
    const on = (mask & (1 << i)) !== 0;
    button.setAttribute("aria-pressed", String(on));
    button.firstElementChild.classList.toggle("chip-tint", on);
    button.firstElementChild.classList.toggle("chip-neutral", !on);
  });
}

function currentDaysMask() {
  const pressed = (button) => button.getAttribute("aria-pressed") === "true";
  return dayButtons().reduce((mask, button, i) => (pressed(button) ? mask | (1 << i) : mask), 0);
}

function showDaysMessage(warn) {
  const message = $("days-msg");
  message.textContent = warn
    ? "Garde au moins un jour actif : sans lui, Breeze ne reprendrait jamais le cycle."
    : "Au moins un jour reste actif : tout désactiver enfermerait le cycle sans sortie.";
  message.classList.toggle("msg-warn", warn);
  message.classList.toggle("hint", !warn);
}

async function toggleDay(index) {
  const previous = currentDaysMask();
  const mask = previous ^ (1 << index);
  if (mask === 0) {
    showDaysMessage(true);
    return;
  }
  showDaysMessage(false);
  applyDays(mask);
  try {
    await call("set_active_days", { mask });
    if (state.settings) state.settings.active_days = mask;
    await refreshSnapshot();
  } catch (error) {
    applyDays(previous);
    toastError(error);
  }
}

// Seul un retour Hardcore → Simple peut être en attente : le cycle en cours reste Hardcore.
function activeSeverity(settings, chosen) {
  return settings.severity_pending ? "Hardcore" : chosen;
}

function applySeverity(settings) {
  const chosen = settings.chosen_severity || settings.severity;
  const active = activeSeverity(settings, chosen);
  markRadio($("severity-group"), chosen);
  document.querySelectorAll(".sev-current").forEach((chip) => {
    chip.hidden = chip.dataset.for !== active;
  });
  $("severity-pending").hidden = !settings.severity_pending;
}

async function chooseSeverity(group, value) {
  markRadio(group, value);
  try {
    await call("set_severity", { severity: value });
    await loadSettings();
  } catch (error) {
    if (state.settings) applySeverity(state.settings);
    toastError(error, SEVERITY_ERRORS);
  }
}

function normalize(text) {
  return text.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase().trim();
}

function appIcon(app) {
  const icon = document.createElement("span");
  icon.className = "appicon sm";
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

function statusSegment(app) {
  const group = document.createElement("div");
  group.className = "seg status-seg status-col";
  group.setAttribute("role", "radiogroup");
  group.setAttribute("aria-label", `Statut de ${app.name} pendant la pause`);
  group.dataset.bundleId = app.bundle_id;
  [["Blocked", "Bloquée"], ["Spared", "Épargnée"], ["Ignored", "Ignorée"]].forEach(([value, label]) => {
    const radio = document.createElement("button");
    radio.type = "button";
    radio.className = "seg-item";
    radio.setAttribute("role", "radio");
    radio.dataset.value = value;
    radio.textContent = label;
    group.appendChild(radio);
  });
  markRadio(group, app.status);
  return group;
}

function appRow(app) {
  const row = document.createElement("li");
  row.className = "app-row";
  row.dataset.search = normalize(app.name);
  const name = document.createElement("span");
  name.className = "t-label app-name";
  name.textContent = app.name;
  name.title = app.name;
  row.append(appIcon(app), name, statusSegment(app));
  return row;
}

function skeletonRow() {
  const row = document.createElement("li");
  row.className = "app-row skeleton";
  row.setAttribute("aria-hidden", "true");
  ["skl-icon", "skl skl-name", "skl skl-seg"].forEach((className) => {
    const part = document.createElement("span");
    part.className = className;
    row.appendChild(part);
  });
  return row;
}

function showAppsState(text, { retry = false } = {}) {
  $("app-state-text").textContent = text;
  $("app-retry").hidden = !retry;
  $("app-state").hidden = !text;
}

function filterApps() {
  const query = normalize($("app-search").value);
  let visible = 0;
  Array.from($("app-list").children).forEach((row) => {
    row.hidden = Boolean(query) && !row.dataset.search.includes(query);
    if (!row.hidden) visible += 1;
  });
  const total = state.apps.length;
  $("app-count").textContent = query ? `${visible} sur ${total}` : plural(total, "application", "applications");
  if (!total) return;
  showAppsState(visible ? "" : `Aucune application ne correspond à « ${$("app-search").value.trim()} ».`);
}

async function loadApps() {
  const list = $("app-list");
  showAppsState("");
  $("app-count").textContent = "";
  list.setAttribute("aria-busy", "true");
  list.replaceChildren(...Array.from({ length: 6 }, skeletonRow));
  try {
    state.apps = await call("list_installed_apps");
    list.replaceChildren(...state.apps.map(appRow));
    if (!state.apps.length) showAppsState("Aucune application installée n’a été trouvée.");
    filterApps();
  } catch (error) {
    state.apps = [];
    list.replaceChildren();
    showAppsState(errorMessage(error, { preview: ERROR_MESSAGES["enumeration-failed"] }), { retry: true });
  } finally {
    list.removeAttribute("aria-busy");
  }
}

async function chooseAppStatus(group, value) {
  const app = state.apps.find((candidate) => candidate.bundle_id === group.dataset.bundleId);
  if (!app) return;
  const previous = app.status;
  app.status = value;
  markRadio(group, value);
  try {
    await call("set_app_status", { bundleId: app.bundle_id, status: value });
  } catch (error) {
    app.status = previous;
    markRadio(group, previous);
    toastError(error);
  }
}

function parseDay(iso) {
  const [year, month, day] = iso.split("-").map(Number);
  return new Date(year, month - 1, day);
}

function niceTop(max) {
  const raw = Math.max(1, max / 2);
  const magnitude = 10 ** Math.floor(Math.log10(raw));
  const step = [1, 2, 3, 5, 10].map((factor) => factor * magnitude).find((candidate) => candidate >= raw);
  return Math.max(2, Math.round(step) * 2);
}

function dayValueText(day) {
  const parts = [`${day.served_minutes} min`, plural(day.served, "pause servie", "pauses servies")];
  if (day.interrupted) parts.push(plural(day.interrupted, "interrompue", "interrompues"));
  return parts.join(" · ");
}

function dayDateText(day, isToday) {
  const label = DATE_FORMAT.format(parseDay(day.date));
  return isToday ? `aujourd’hui, ${label}` : label;
}

function barColumn(day, top, isToday) {
  const column = document.createElement("div");
  column.className = "bar-col";
  column.classList.toggle("today", isToday);
  column.classList.toggle("zero", day.served_minutes === 0);
  column.setAttribute("role", "img");
  column.setAttribute("aria-label", `${dayDateText(day, isToday)} : ${dayValueText(day)}`);
  column.tabIndex = isToday ? 0 : -1;
  const fill = document.createElement("div");
  fill.className = "bar-fill";
  if (day.served_minutes) fill.style.height = `${Math.max(2, (day.served_minutes / top) * 100)}%`;
  column.appendChild(fill);
  column.addEventListener("mouseenter", () => showChartTip(column, day, isToday));
  column.addEventListener("focus", () => showChartTip(column, day, isToday));
  column.addEventListener("mouseleave", hideChartTip);
  column.addEventListener("blur", hideChartTip);
  return column;
}

function showChartTip(column, day, isToday) {
  const tip = $("chart-tip");
  const plot = $("chart-plot");
  $("chart-tip-date").textContent = dayDateText(day, isToday);
  $("chart-tip-value").textContent = dayValueText(day);
  tip.hidden = false;
  const half = tip.offsetWidth / 2;
  const center = column.offsetLeft + column.offsetWidth / 2;
  tip.style.left = `${Math.min(Math.max(center, half), plot.clientWidth - half)}px`;
  const above = column.firstElementChild.offsetHeight + 8;
  tip.style.bottom = `${Math.min(above, plot.clientHeight - tip.offsetHeight + 16)}px`;
}

function hideChartTip() {
  $("chart-tip").hidden = true;
}

function onChartKeydown(event) {
  const columns = Array.from($("chart-plot").querySelectorAll(".bar-col"));
  const index = columns.indexOf(event.target);
  const moves = { ArrowRight: index + 1, ArrowLeft: index - 1, Home: 0, End: columns.length - 1 };
  if (index < 0 || !(event.key in moves)) return;
  event.preventDefault();
  const next = columns[Math.min(Math.max(moves[event.key], 0), columns.length - 1)];
  columns.forEach((column) => { column.tabIndex = column === next ? 0 : -1; });
  next.focus();
}

function renderChart(stats) {
  const days = stats.days;
  const top = niceTop(Math.max(0, ...days.map((day) => day.served_minutes)));
  $("tick-top").textContent = String(top);
  $("tick-mid").textContent = String(top / 2);
  const plot = $("chart-plot");
  plot.querySelectorAll(".bar-col").forEach((column) => column.remove());
  days.forEach((day, i) => plot.appendChild(barColumn(day, top, i === days.length - 1)));
  hideChartTip();
  $("chart-first").textContent = SHORT_DATE_FORMAT.format(parseDay(days[0].date));
  $("chart-last").textContent = `aujourd’hui, ${SHORT_DATE_FORMAT.format(parseDay(days[days.length - 1].date))}`;
  $("chart-total").textContent = `${stats.served_minutes} min de pause servies en 30 jours.`;
}

function renderKpis(stats) {
  const activeDays = stats.days.filter((day) => day.served + day.interrupted + day.validated > 0).length;
  $("kpi-served").textContent = `${Math.floor((stats.served * 100) / stats.due)} %`;
  $("kpi-served-sub").textContent = `${stats.served} sur ${stats.due} dues`;
  $("kpi-due").textContent = String(stats.due);
  $("kpi-due-sub").textContent = `sur ${plural(activeDays, "jour", "jours")}`;
  $("kpi-interrupted").textContent = String(stats.interrupted);
  $("kpi-interrupted-sub").textContent = stats.debt_minutes ? `${stats.debt_minutes} min de dette` : "Aucune dette";
  $("kpi-emergency").textContent = String(stats.emergency_exits);
  $("kpi-emergency-sub").textContent = stats.emergency_exits ? `${stats.emergency_minutes} min non servies` : "Aucune";
}

function showStatsView(view) {
  $("stats-body").hidden = view !== "body";
  $("stats-empty").hidden = view !== "empty";
  $("stats-error").hidden = view !== "error";
}

async function loadStats() {
  try {
    const stats = await call("get_stats");
    if (!stats.due || !stats.days.length) {
      showStatsView("empty");
      return;
    }
    renderKpis(stats);
    renderChart(stats);
    showStatsView("body");
  } catch (error) {
    if (!PREVIEW && !(String(error) in ERROR_MESSAGES)) console.error("settings: get_stats", error);
    $("stats-error-text").textContent = "Impossible de lire le journal des pauses. Réessaie dans un instant.";
    showStatsView("error");
  }
}

function applyAccessibility(status) {
  const look = AX_LOOK[status] || AX_LOOK.Unknown;
  const chip = $("ax-chip");
  chip.textContent = look.text;
  chip.className = `chip chip-sm ${look.tone}`;
  const granted = status === "Granted";
  $("ax-request").hidden = granted;
  if (granted) {
    $("ax-wait").hidden = true;
    stopAccessibilityPolling();
    if (state.axStatus && state.axStatus !== "Granted") showToast("Accessibilité accordée.");
  }
  state.axStatus = status;
}

async function refreshAccessibility() {
  try {
    applyAccessibility(await call("accessibility_status"));
  } catch (error) {
    stopAccessibilityPolling();
    applyAccessibility("Unknown");
    toastError(error);
  }
}

function stopAccessibilityPolling() {
  clearInterval(state.axTimer);
  state.axTimer = null;
}

function startAccessibilityPolling() {
  stopAccessibilityPolling();
  let ticks = 0;
  state.axTimer = setInterval(() => {
    ticks += 1;
    if (ticks >= AX_POLL_TICKS) stopAccessibilityPolling();
    refreshAccessibility();
  }, AX_POLL_MS);
}

async function requestAccessibility() {
  try {
    if (await call("request_accessibility")) {
      applyAccessibility("Granted");
      return;
    }
    $("ax-wait").hidden = false;
    startAccessibilityPolling();
  } catch (error) {
    toastError(error);
  }
}

function toggleFaq(button) {
  const expanded = button.getAttribute("aria-expanded") !== "true";
  button.setAttribute("aria-expanded", String(expanded));
  $(button.getAttribute("aria-controls")).hidden = !expanded;
}

async function runHostAction(command, args) {
  try {
    await call(command, args);
  } catch (error) {
    toastError(error);
  }
}

async function loadAppInfo() {
  try {
    const info = await call("get_app_info");
    $("app-version").textContent = `${info.name} ${info.version}`;
  } catch (error) {
    toastError(error);
  }
}

function trapDialogFocus(event) {
  if (event.key !== "Tab") return;
  const focusables = Array.from($("reset-dialog").querySelectorAll("button:not(:disabled)"));
  const first = focusables[0];
  const last = focusables[focusables.length - 1];
  if (event.shiftKey && document.activeElement === first) {
    event.preventDefault();
    last.focus();
  } else if (!event.shiftKey && document.activeElement === last) {
    event.preventDefault();
    first.focus();
  }
}

function setResetBusy(busy) {
  const dialog = $("reset-dialog");
  dialog.dataset.busy = busy ? "1" : "";
  $("reset-confirm").disabled = busy;
  $("reset-cancel").disabled = busy;
  $("reset-confirm").textContent = busy ? "Réinitialisation…" : "Réinitialiser";
}

async function confirmReset() {
  setResetBusy(true);
  try {
    await call("reset_settings");
    $("reset-dialog").close();
    showToast("Réglages réinitialisés.");
    await Promise.all([loadSettings(), refreshSnapshot(), loadApps()]);
  } catch (error) {
    $("reset-dialog").close();
    toastError(error);
  } finally {
    setResetBusy(false);
  }
}

function wireResetDialog() {
  const dialog = $("reset-dialog");
  $("reset-open").addEventListener("click", () => {
    dialog.showModal();
    $("reset-cancel").focus();
  });
  $("reset-cancel").addEventListener("click", () => dialog.close());
  $("reset-confirm").addEventListener("click", confirmReset);
  dialog.addEventListener("keydown", trapDialogFocus);
  dialog.addEventListener("cancel", (event) => {
    if (dialog.dataset.busy) event.preventDefault();
  });
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog && !dialog.dataset.busy) dialog.close();
  });
  dialog.addEventListener("close", () => $("reset-open").focus());
}

function wireControls() {
  wireSwitch("launch-switch", "set_launch_at_login");
  wireSwitch("sounds-switch", "set_sounds");
  wireSwitch("update-switch", "set_update_check");
  wireRadioGroups($("menubar-seg"), chooseMenubarMode);
  wireRadioGroups($("severity-group"), chooseSeverity);
  wireRadioGroups($("app-list"), chooseAppStatus);
  wireRhythm();
  wireSchedule();
  dayButtons().forEach((button, i) => button.addEventListener("click", () => toggleDay(i)));
  $("app-search").addEventListener("input", filterApps);
  $("app-retry").addEventListener("click", loadApps);
  $("stats-retry").addEventListener("click", loadStats);
  $("chart-plot").addEventListener("keydown", onChartKeydown);
  $("ax-request").addEventListener("click", requestAccessibility);
  document.querySelectorAll(".faq-btn").forEach((button) => button.addEventListener("click", () => toggleFaq(button)));
  $("reopen-onboarding").addEventListener("click", () => runHostAction("reopen_onboarding"));
  $("report-issue").addEventListener("click", () => runHostAction("open_url", { url: REPORT_URL }));
  wireResetDialog();
}

function reloadOnFocus() {
  loadSettings();
  refreshSnapshot();
  refreshAccessibility();
  loadStats();
}

function init() {
  $("preview-chip").hidden = !PREVIEW;
  wireTabs();
  wireControls();
  window.addEventListener("focus", reloadOnFocus);
  loadSettings();
  refreshSnapshot();
  loadApps();
  loadStats();
  refreshAccessibility();
  loadAppInfo();
}

init();
