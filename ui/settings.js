// Breeze — fenêtre de réglages.
// Bascule entre les sept panneaux depuis la barre latérale. Chaque panneau
// reflète l'état réel de l'hôte ; l'écriture des réglages reste portée par les
// commandes set_rhythm / set_severity, câblées écran par écran.

const navitems = Array.from(document.querySelectorAll(".navitem"));
const panels = Array.from(document.querySelectorAll(".panel"));

function select(key) {
  navitems.forEach((n) => n.classList.toggle("on", n.dataset.nav === key));
  panels.forEach((p) => p.classList.toggle("on", p.dataset.panel === key));
}

navitems.forEach((n) => n.addEventListener("click", () => select(n.dataset.nav)));
