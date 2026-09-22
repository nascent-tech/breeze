// Breeze — fenêtre de réglages.
// Bascule entre les sept panneaux depuis la barre latérale, tenue comme un jeu
// d'onglets accessible (rôles ARIA, flèches, sélection annoncée). L'écriture des
// réglages reste portée par les commandes de l'hôte, câblées écran par écran.

const navitems = Array.from(document.querySelectorAll(".navitem"));
const panels = Array.from(document.querySelectorAll(".panel"));

// Toutes les icônes sont décoratives : le libellé textuel adjacent porte le sens.
document.querySelectorAll("svg").forEach((svg) => svg.setAttribute("aria-hidden", "true"));

// Titre de chaque panneau promu en en-tête pour le lecteur d'écran.
panels.forEach((panel) => {
  const title = panel.querySelector(".t-headline");
  if (title) {
    title.setAttribute("role", "heading");
    title.setAttribute("aria-level", "1");
  }
});

// Barre latérale = tablist ; panneaux = tabpanels.
const sidebar = document.querySelector(".sidebar");
if (sidebar) {
  sidebar.setAttribute("role", "tablist");
  sidebar.setAttribute("aria-orientation", "vertical");
  sidebar.setAttribute("aria-label", "Sections des réglages");
}
navitems.forEach((nav) => {
  const key = nav.dataset.nav;
  nav.setAttribute("role", "tab");
  nav.setAttribute("id", `tab-${key}`);
  nav.setAttribute("aria-controls", `panel-${key}`);
  const active = nav.classList.contains("on");
  nav.setAttribute("aria-selected", String(active));
  nav.setAttribute("tabindex", active ? "0" : "-1");
});
panels.forEach((panel) => {
  const key = panel.dataset.panel;
  panel.setAttribute("role", "tabpanel");
  panel.setAttribute("id", `panel-${key}`);
  panel.setAttribute("aria-labelledby", `tab-${key}`);
  panel.setAttribute("tabindex", "0");
});

function select(key, focusTab) {
  navitems.forEach((n) => {
    const on = n.dataset.nav === key;
    n.classList.toggle("on", on);
    n.setAttribute("aria-selected", String(on));
    n.setAttribute("tabindex", on ? "0" : "-1");
    if (on && focusTab) n.focus();
  });
  panels.forEach((p) => p.classList.toggle("on", p.dataset.panel === key));
}

navitems.forEach((nav, i) => {
  nav.addEventListener("click", () => select(nav.dataset.nav));
  nav.addEventListener("keydown", (event) => {
    let next = null;
    const last = navitems.length - 1;
    if (event.key === "ArrowDown" || event.key === "ArrowRight") next = navitems[(i + 1) % navitems.length];
    else if (event.key === "ArrowUp" || event.key === "ArrowLeft") next = navitems[i === 0 ? last : i - 1];
    else if (event.key === "Home") next = navitems[0];
    else if (event.key === "End") next = navitems[navitems.length - 1];
    if (next) {
      event.preventDefault();
      select(next.dataset.nav, true);
    }
  });
});

// Jours actifs : bascule l'état, en gardant au moins un jour actif (brief §8.2 —
// tout désactiver enfermerait le cycle sans sortie).
const days = Array.from(document.querySelectorAll(".day-toggle"));
days.forEach((day) =>
  day.addEventListener("click", () => {
    const pressed = day.getAttribute("aria-pressed") === "true";
    const activeCount = days.filter((d) => d.getAttribute("aria-pressed") === "true").length;
    if (pressed && activeCount === 1) return; // Refuse de retirer le dernier jour actif.
    const chip = day.querySelector(".chip");
    day.setAttribute("aria-pressed", String(!pressed));
    chip.classList.toggle("chip-tint", !pressed);
    chip.classList.toggle("chip-neutral", pressed);
  }),
);

// ---- Applications : vraies icônes des apps installées, statut par app ----
// Toute donnée venant de l'hôte (nom d'app tiers, icône) entre par textContent ou
// img.src=data:, jamais innerHTML : un nom hostile ne peut pas injecter de balise.
const invoke = window.__TAURI__?.core?.invoke;

const STATUSES = [
  { key: "Blocked", label: "Bloquée" },
  { key: "Spared", label: "Épargnée" },
  { key: "Ignored", label: "Ignorée" },
];

async function setStatus(bundleId, status, seg, chosen) {
  if (!invoke) return;
  try {
    await invoke("set_app_status", { bundleId, status });
    seg.querySelectorAll(".seg-item").forEach((item) => item.classList.remove("on"));
    chosen.classList.add("on");
  } catch (err) {
    console.error("settings: statut d'application non enregistré", err);
  }
}

function appIcon(app) {
  const icon = document.createElement("span");
  icon.className = "appicon sm";
  if (app.icon_data_url) {
    const img = document.createElement("img");
    img.src = app.icon_data_url; // data URL PNG uniquement
    img.alt = "";
    img.width = 22;
    img.height = 22;
    img.style.cssText = "border-radius: 5px; display: block";
    icon.style.cssText = "background: transparent; padding: 0";
    icon.appendChild(img);
  } else {
    icon.textContent = (app.name.trim()[0] || "?").toUpperCase();
    icon.style.cssText = "background: linear-gradient(160deg, #9ba3b5, #6c7383)";
  }
  return icon;
}

function appSegment(app) {
  const seg = document.createElement("span");
  seg.className = "seg";
  seg.style.cssText = "width: 236px; display: flex";
  seg.setAttribute("role", "group");
  seg.setAttribute("aria-label", `Statut de ${app.name} pendant la pause`);
  STATUSES.forEach((status) => {
    const item = document.createElement("span");
    item.className = status.key === app.status ? "seg-item on" : "seg-item";
    item.style.cssText = "flex: 1; text-align: center; padding: 5px 0";
    item.textContent = status.label;
    item.setAttribute("role", "button");
    item.addEventListener("click", () => setStatus(app.bundle_id, status.key, seg, item));
    seg.appendChild(item);
  });
  return seg;
}

function appRow(app) {
  const row = document.createElement("div");
  row.className = "rowitem";
  row.setAttribute("role", "listitem");
  row.style.cssText = "min-height: 40px; padding: 4px 12px";
  const name = document.createElement("span");
  name.className = "t-label";
  name.style.cssText = "flex: 1";
  name.textContent = app.name;
  row.append(appIcon(app), name, appSegment(app));
  return row;
}

function fillAppList(apps) {
  const list = document.getElementById("app-list");
  if (!list) return;
  list.textContent = "";
  if (!apps.length) {
    const empty = document.createElement("div");
    empty.className = "t-caption";
    empty.style.cssText = "padding: 12px; color: var(--ink-3)";
    empty.textContent = "Aucune application détectée sur cette machine.";
    list.appendChild(empty);
    return;
  }
  apps.forEach((app, index) => {
    if (index > 0) {
      const divider = document.createElement("div");
      divider.className = "divider";
      divider.style.cssText = "margin: 0 12px";
      list.appendChild(divider);
    }
    list.appendChild(appRow(app));
  });
}

async function loadApps() {
  const status = document.getElementById("app-list-status");
  if (!invoke) {
    if (status) status.textContent = "Aperçu navigateur : la liste réelle s’affiche dans l’application.";
    return;
  }
  try {
    fillAppList(await invoke("list_installed_apps"));
  } catch (err) {
    if (status) status.textContent = "Impossible de lire les applications installées.";
    console.error("settings: énumération des applications impossible", err);
  }
}

// ---- Permissions : Accessibilité réelle ----
const AX_LOOK = {
  Granted: { text: "Accordée", cls: "chip chip-mint" },
  Denied: { text: "Refusée", cls: "chip chip-warn" },
  Unknown: { text: "Inconnue", cls: "chip chip-neutral" },
};

async function refreshAccessibility() {
  const chip = document.getElementById("ax-chip");
  if (!chip || !invoke) return;
  try {
    const look = AX_LOOK[await invoke("accessibility_status")] || AX_LOOK.Unknown;
    chip.textContent = look.text;
    chip.className = look.cls;
    chip.style.cssText = "height: 24px; font-size: 12px";
  } catch (err) {
    console.error("settings: état d'Accessibilité illisible", err);
  }
}

const axButton = document.getElementById("ax-request");
if (axButton) {
  const request = async () => {
    if (invoke) {
      try {
        await invoke("request_accessibility");
      } catch (err) {
        console.error("settings: demande d'Accessibilité impossible", err);
      }
    }
    refreshAccessibility();
  };
  axButton.addEventListener("click", request);
  axButton.addEventListener("keydown", (event) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      request();
    }
  });
}

loadApps();
refreshAccessibility();

// ---- Réglages persistés : rythme (affiché), jours actifs, plage, MàJ, reset ----
const WORK_MIN = 5;
const WORK_MAX = 180;
const PAUSE_MIN = 1;
const PAUSE_MAX = 60;
const DAY_MINUTES = 1440;
const TIME_STEP = 30;
let lastSettings = null;

function pad2(n) {
  return String(n).padStart(2, "0");
}

function formatHM(minute) {
  return `${pad2(Math.floor(minute / 60))}:${pad2(minute % 60)}`;
}

function parseHM(text) {
  const [h, m] = text.trim().split(":").map(Number);
  return (h || 0) * 60 + (m || 0);
}

function setSwitch(el, on) {
  if (!el) return;
  el.classList.toggle("on", on);
  el.setAttribute("aria-checked", String(on));
}

function daysMask() {
  return days.reduce(
    (mask, day, i) => (day.getAttribute("aria-pressed") === "true" ? mask | (1 << i) : mask),
    0,
  );
}

function applySettings(s) {
  lastSettings = s;
  const work = document.getElementById("rhythm-work-value");
  const pause = document.getElementById("rhythm-pause-value");
  const workFill = document.getElementById("rhythm-work-fill");
  const workKnob = document.getElementById("rhythm-work-knob");
  const pauseFill = document.getElementById("rhythm-pause-fill");
  const pauseKnob = document.getElementById("rhythm-pause-knob");
  if (work) work.textContent = `${s.work_minutes} min`;
  if (pause) pause.textContent = `${s.pause_minutes} min`;
  const workPct = Math.round(((s.work_minutes - WORK_MIN) / (WORK_MAX - WORK_MIN)) * 100);
  const pausePct = Math.round(((s.pause_minutes - PAUSE_MIN) / (PAUSE_MAX - PAUSE_MIN)) * 100);
  if (workFill) workFill.style.width = `${workPct}%`;
  if (workKnob) workKnob.style.left = `calc(${workPct}% - 10px)`;
  if (pauseFill) pauseFill.style.width = `${pausePct}%`;
  if (pauseKnob) pauseKnob.style.left = `calc(${pausePct}% - 10px)`;

  days.forEach((day, i) => {
    const on = (s.active_days & (1 << i)) !== 0;
    day.setAttribute("aria-pressed", String(on));
    const chip = day.querySelector(".chip");
    chip.classList.toggle("chip-tint", on);
    chip.classList.toggle("chip-neutral", !on);
  });

  setSwitch(document.getElementById("schedule-switch"), s.schedule_enabled);
  const start = document.getElementById("schedule-start");
  const end = document.getElementById("schedule-end");
  if (start) start.textContent = formatHM(s.schedule_start);
  if (end) end.textContent = formatHM(s.schedule_end);
  setSwitch(document.getElementById("update-check-switch"), s.update_check);
  setSwitch(document.getElementById("launch-switch"), s.launch_at_login);
  setSwitch(document.getElementById("sounds-switch"), s.sounds);
  document.querySelectorAll("#menubar-seg .seg-item").forEach((item) => {
    const on = (item.dataset.mode === "text") === s.menubar_text;
    item.classList.toggle("on", on);
  });
}

async function loadSettings() {
  if (!invoke) return;
  try {
    applySettings(await invoke("get_settings"));
  } catch (err) {
    console.error("settings: lecture des réglages impossible", err);
  }
}

async function pushActiveDays() {
  if (!invoke) return;
  try {
    await invoke("set_active_days", { mask: daysMask() });
  } catch (err) {
    console.error("settings: jours actifs non enregistrés", err);
  }
}
days.forEach((day) => day.addEventListener("click", pushActiveDays));

const scheduleSwitch = document.getElementById("schedule-switch");
if (scheduleSwitch) {
  scheduleSwitch.addEventListener("click", async () => {
    const enabled = scheduleSwitch.getAttribute("aria-checked") !== "true";
    setSwitch(scheduleSwitch, enabled);
    if (!invoke) return;
    const start = parseHM(document.getElementById("schedule-start").textContent);
    const end = parseHM(document.getElementById("schedule-end").textContent);
    try {
      await invoke("set_schedule", { enabled, start, end });
    } catch (err) {
      setSwitch(scheduleSwitch, !enabled);
      console.error("settings: plage horaire non enregistrée", err);
    }
  });
}

const updateSwitch = document.getElementById("update-check-switch");
if (updateSwitch) {
  updateSwitch.addEventListener("click", async () => {
    const enabled = updateSwitch.getAttribute("aria-checked") !== "true";
    setSwitch(updateSwitch, enabled);
    if (!invoke) return;
    try {
      await invoke("set_update_check", { enabled });
    } catch (err) {
      setSwitch(updateSwitch, !enabled);
      console.error("settings: préférence de mise à jour non enregistrée", err);
    }
  });
}

const resetButton = document.getElementById("reset-settings");
if (resetButton) {
  resetButton.addEventListener("click", async () => {
    if (!invoke) return;
    try {
      await invoke("reset_settings");
      loadSettings();
      loadApps();
    } catch (err) {
      console.error("settings: réinitialisation impossible", err);
    }
  });
}

// Entrée/Espace activent les contrôles à rôle bouton/switch (parité clavier).
["schedule-switch", "update-check-switch", "reset-settings"].forEach((id) => {
  const control = document.getElementById(id);
  if (!control) return;
  control.addEventListener("keydown", (event) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      control.click();
    }
  });
});

loadSettings();

// ---- Général : lancement à l'ouverture de session, sons, mode barre de menus ----
function wireToggleCommand(id, command) {
  const el = document.getElementById(id);
  if (!el) return;
  const toggle = async () => {
    const enabled = el.getAttribute("aria-checked") !== "true";
    setSwitch(el, enabled);
    if (!invoke) return;
    try {
      await invoke(command, { enabled });
    } catch (err) {
      setSwitch(el, !enabled);
      console.error(`settings: ${command} impossible`, err);
    }
  };
  el.addEventListener("click", toggle);
  el.addEventListener("keydown", (event) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      toggle();
    }
  });
}
wireToggleCommand("launch-switch", "set_launch_at_login");
wireToggleCommand("sounds-switch", "set_sounds");

document.querySelectorAll("#menubar-seg .seg-item").forEach((item) => {
  item.addEventListener("click", async () => {
    const text = item.dataset.mode === "text";
    document
      .querySelectorAll("#menubar-seg .seg-item")
      .forEach((other) => other.classList.toggle("on", other === item));
    if (!invoke) return;
    try {
      await invoke("set_menubar_mode", { text });
    } catch (err) {
      console.error("settings: mode barre de menus impossible", err);
    }
  });
});

// ---- Rythme : clic sur la piste = valeur proportionnelle → set_rhythm ----
async function pushRhythm(work, pause) {
  if (!invoke) return;
  try {
    await invoke("set_rhythm", { workMinutes: work, pauseMinutes: pause });
    loadSettings();
  } catch (err) {
    console.error("settings: rythme non enregistré", err);
  }
}

function wireSliderTrack(fillId, min, max, isWork) {
  const fill = document.getElementById(fillId);
  const track = fill && fill.parentElement;
  if (!track) return;
  track.style.cursor = "pointer";
  track.addEventListener("click", (event) => {
    if (!lastSettings) return;
    const rect = track.getBoundingClientRect();
    const ratio = Math.min(1, Math.max(0, (event.clientX - rect.left) / rect.width));
    const value = Math.round(min + ratio * (max - min));
    const work = isWork ? value : lastSettings.work_minutes;
    let pause = isWork ? lastSettings.pause_minutes : value;
    if (pause > work) pause = work; // Le domaine refuse pause > travail : borné côté UI.
    pushRhythm(work, pause);
  });
}
wireSliderTrack("rhythm-work-fill", WORK_MIN, WORK_MAX, true);
wireSliderTrack("rhythm-pause-fill", PAUSE_MIN, PAUSE_MAX, false);

// ---- Plage : clic sur une borne = +30 min (boucle sur 24 h) → set_schedule ----
async function stepScheduleBound(which) {
  if (!invoke || !lastSettings || !lastSettings.schedule_enabled) return;
  let start = lastSettings.schedule_start;
  let end = lastSettings.schedule_end;
  if (which === "start") start = (start + TIME_STEP) % DAY_MINUTES;
  else end = (end + TIME_STEP) % DAY_MINUTES;
  if (start === end) return; // Le domaine refuse une plage dégénérée.
  try {
    await invoke("set_schedule", { enabled: true, start, end });
    loadSettings();
  } catch (err) {
    console.error("settings: plage non enregistrée", err);
  }
}
["start", "end"].forEach((which) => {
  const chip = document.getElementById(`schedule-${which}`);
  if (!chip) return;
  chip.style.cursor = "pointer";
  chip.setAttribute("title", "Cliquer pour avancer de 30 min");
  chip.addEventListener("click", () => stepScheduleBound(which));
});
