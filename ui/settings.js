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
