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
