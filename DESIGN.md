---
version: alpha
name: Breeze
description: Système de design de Breeze — liquid glass sur fondations Material 3, pour une application de barre de menus macOS qui force la pause.

colors:
  background: "#F4F6FB"
  surface: "#FBFCFE"
  surface-dim: "#EDEFF4"
  surface-hover: "#E2E5EE"
  surface-pressed: "#D8DCE8"
  surface-skeleton: "#E9ECF3"
  on-surface: "#1C2130"
  on-surface-muted: "#474F63"
  on-surface-faint: "#868DA0"
  on-surface-disabled: "#9AA1B2"
  primary: "#084F9E"
  primary-hover: "#073F7E"
  primary-pressed: "#062F60"
  on-primary: "#FBFCFE"
  primary-vivid: "#0A6FDB"
  primary-container: "#E3F1FD"
  on-primary-container: "#075093"
  success: "#095A40"
  success-container: "#DEF4E9"
  on-success-container: "#095A40"
  accent: "#4F35A8"
  accent-container: "#ECE7FB"
  on-accent-container: "#4F35A8"
  warning: "#6E4200"
  warning-container: "#FDF0DC"
  on-warning-container: "#6E4200"
  danger: "#9E2036"
  danger-container: "#FCE8EC"
  on-danger-container: "#7E1A2C"
  border: "#E6E8EF"
  border-control: "#767E92"
  mesh-1: "#CBE5FF"
  mesh-2: "#E5DDFF"
  mesh-3: "#D2F2E3"
  mesh-4: "#FCE7F0"
  night-1: "#1B2340"
  night-2: "#0E1322"
  on-night: "#EFF3FB"
  on-night-muted: "#A9B4CD"

colors-dark:
  background: "#12151F"
  surface: "#1B1F2C"
  surface-dim: "#232838"
  surface-hover: "#2A3042"
  surface-pressed: "#323950"
  surface-skeleton: "#262C3C"
  on-surface: "#E8EBF4"
  on-surface-muted: "#ADB4C6"
  on-surface-faint: "#6A7186"
  on-surface-disabled: "#5C6377"
  primary: "#7FBFF8"
  primary-hover: "#92C8F9"
  primary-pressed: "#A9D2FA"
  on-primary: "#0A2540"
  primary-vivid: "#4AA8F4"
  primary-container: "#14344F"
  on-primary-container: "#A8D4FA"
  success: "#7FD8B8"
  success-container: "#0E3D2F"
  on-success-container: "#7FD8B8"
  accent: "#C6B5F5"
  accent-container: "#2A2350"
  on-accent-container: "#C6B5F5"
  warning: "#F2C280"
  warning-container: "#4A2E07"
  on-warning-container: "#F2C280"
  danger: "#F49DAD"
  danger-container: "#3A121C"
  on-danger-container: "#F49DAD"
  border: "#2E3444"
  border-control: "#8891A6"
  mesh-1: "#16283E"
  mesh-2: "#241E3E"
  mesh-3: "#12302A"
  mesh-4: "#33202B"
  night-1: "#1B2340"
  night-2: "#0E1322"
  on-night: "#EFF3FB"
  on-night-muted: "#A9B4CD"

typography:
  display:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 32px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: -0.02em
  h1:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 24px
    fontWeight: 700
    lineHeight: 1.25
    letterSpacing: -0.015em
  h2:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 16px
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: -0.008em
  h3:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 600
    lineHeight: 1.35
  body:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 400
    lineHeight: 1.45
  caption:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 12px
    fontWeight: 500
    lineHeight: 1.35
  overline:
    fontFamily: "Instrument Sans, -apple-system, SF Pro Text, Helvetica Neue, system-ui, sans-serif"
    fontSize: 11px
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: 0.02em

rounded:
  sm: 8px
  md: 16px
  lg: 24px
  full: 999px

spacing:
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 32px
  2xl: 48px
  3xl: 64px

stroke:
  hairline: 1px
  ring: 2px

focus:
  ringColor: "{colors.primary-vivid}"
  ringWidth: "{stroke.ring}"
  ringOffset: "{stroke.ring}"

motion:
  duration-fast: 150ms
  duration-base: 200ms
  duration-slow: 300ms
  easing-standard: "cubic-bezier(0.2, 0, 0, 1)"

components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
    textColor: "{colors.on-primary}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-primary-pressed:
    backgroundColor: "{colors.primary-pressed}"
    textColor: "{colors.on-primary}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-primary-disabled:
    backgroundColor: "{colors.surface-dim}"
    textColor: "{colors.on-surface-disabled}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-secondary:
    backgroundColor: "{colors.surface-dim}"
    textColor: "{colors.on-surface}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-secondary-hover:
    backgroundColor: "{colors.surface-hover}"
    textColor: "{colors.on-surface}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-secondary-pressed:
    backgroundColor: "{colors.surface-pressed}"
    textColor: "{colors.on-surface}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-secondary-disabled:
    backgroundColor: "{colors.surface-dim}"
    textColor: "{colors.on-surface-disabled}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  button-ghost:
    backgroundColor: "transparent"
    textColor: "{colors.primary}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "8px 24px"
    height: 40px
  chip:
    backgroundColor: "{colors.primary-container}"
    textColor: "{colors.on-primary-container}"
    typography: "{typography.caption}"
    rounded: "{rounded.full}"
    padding: "4px 8px"
    height: 24px
  list-row:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "4px 8px"
    height: 48px
  list-row-hover:
    backgroundColor: "{colors.surface-dim}"
    textColor: "{colors.on-surface}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "4px 8px"
    height: 48px
  list-row-disabled:
    backgroundColor: "transparent"
    textColor: "{colors.on-surface-disabled}"
    typography: "{typography.h3}"
    rounded: "{rounded.sm}"
    padding: "4px 8px"
    height: 48px
  card:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "{spacing.md}"
  panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.body}"
    rounded: "{rounded.lg}"
    padding: "{spacing.md}"
  switch-off:
    backgroundColor: "{colors.border-control}"
    rounded: "{rounded.full}"
    padding: "{spacing.xs}"
    width: 48px
    height: 24px
  switch-on:
    backgroundColor: "{colors.primary-vivid}"
    rounded: "{rounded.full}"
    padding: "{spacing.xs}"
    width: 48px
    height: 24px
  switch-knob:
    backgroundColor: "{colors.surface}"
    rounded: "{rounded.full}"
    size: 16px
  switch-knob-locked:
    backgroundColor: "{colors.surface-dim}"
    rounded: "{rounded.full}"
    size: 16px
  countdown:
    textColor: "{colors.on-surface}"
    typography: "{typography.display}"
---

# Breeze — système de design

Le `.md` fait foi ; `DESIGN.html` en est le miroir généré. Ils se modifient ensemble, jamais l'un
sans l'autre. Les écrans consomment ce fichier — ils ne redécident rien. Tout ce qui est normatif
vit dans le bloc de tokens ; la prose dit le pourquoi.

## Les ratios, à côté des paires

Chaque ratio est calculé (WCAG 2.x, sRGB), jamais estimé. Barres : 7:1 pour le texte courant,
4,5:1 pour le texte large, 3:1 pour ce qui n'est pas du texte. Les deux tables couvrent les mêmes
paires, ligne pour ligne.

### Thème clair

| La paire | Ratio | Barre | Verdict |
|---|---|---|---|
| `on-surface` sur `surface` | 15,6:1 | 7:1 | tient |
| `on-surface` sur `background` | 14,8:1 | 7:1 | tient |
| `on-surface` sur `surface-dim` | 13,9:1 | 7:1 | tient |
| `on-surface` sur `surface-hover` | 12,7:1 | 7:1 | tient |
| `on-surface` sur `surface-pressed` | 11,7:1 | 7:1 | tient |
| `on-surface-muted` sur `surface` | 8,0:1 | 7:1 | tient |
| `on-surface-muted` sur `background` | 7,6:1 | 7:1 | tient |
| `on-surface-muted` sur `surface-dim` | 7,1:1 | 7:1 | tient |
| `primary` sur `surface` (liens) | 7,8:1 | 7:1 | tient |
| `primary` sur `background` (liens) | 7,4:1 | 7:1 | tient |
| `on-primary` sur `primary` | 7,8:1 | 7:1 | tient |
| `on-primary` sur `primary-hover` | 10,1:1 | 7:1 | tient |
| `on-primary` sur `primary-pressed` | 12,9:1 | 7:1 | tient |
| `on-primary-container` sur `primary-container` | 7,1:1 | 7:1 | tient |
| `on-success-container` sur `success-container` | 7,2:1 | 7:1 | tient |
| `success` sur `surface` | 8,0:1 | 7:1 | tient |
| `on-accent-container` sur `accent-container` | 7,2:1 | 7:1 | tient |
| `on-warning-container` sur `warning-container` | 7,7:1 | 7:1 | tient |
| `danger` sur `surface` | 7,5:1 | 7:1 | tient |
| `on-danger-container` sur `danger-container` | 8,6:1 | 7:1 | tient |
| `on-night` sur `night-1` | 13,9:1 | 7:1 | tient |
| `on-night-muted` sur `night-1` | 7,4:1 | 7:1 | tient |
| `primary-vivid` sur `surface` | 4,8:1 | 3:1 (jamais du texte) | tient |
| `primary-vivid` sur `background` | 4,5:1 | 3:1 (jamais du texte) | tient |
| `border-control` sur `surface` | 4,0:1 | 3:1 | tient |
| `switch-knob-locked` sur `primary-vivid` | 4,2:1 | 3:1 | tient |
| `switch-knob-locked` sur `border-control` | 3,5:1 | 3:1 | tient |
| `on-surface-faint` sur `surface` | 3,2:1 | 3:1 (jamais du texte) | tient |

### Thème sombre

| La paire | Ratio | Barre | Verdict |
|---|---|---|---|
| `on-surface` sur `surface` | 13,8:1 | 7:1 | tient |
| `on-surface` sur `background` | 15,3:1 | 7:1 | tient |
| `on-surface` sur `surface-dim` | 12,3:1 | 7:1 | tient |
| `on-surface` sur `surface-hover` | 11,0:1 | 7:1 | tient |
| `on-surface` sur `surface-pressed` | 9,6:1 | 7:1 | tient |
| `on-surface-muted` sur `surface` | 7,9:1 | 7:1 | tient |
| `on-surface-muted` sur `background` | 8,8:1 | 7:1 | tient |
| `on-surface-muted` sur `surface-dim` | 7,1:1 | 7:1 | tient |
| `primary` sur `surface` (liens) | 8,4:1 | 7:1 | tient |
| `primary` sur `background` (liens) | 9,3:1 | 7:1 | tient |
| `on-primary` sur `primary` | 7,9:1 | 7:1 | tient |
| `on-primary` sur `primary-hover` | 8,8:1 | 7:1 | tient |
| `on-primary` sur `primary-pressed` | 9,8:1 | 7:1 | tient |
| `on-primary-container` sur `primary-container` | 8,2:1 | 7:1 | tient |
| `on-success-container` sur `success-container` | 7,2:1 | 7:1 | tient |
| `success` sur `surface` | 9,7:1 | 7:1 | tient |
| `on-accent-container` sur `accent-container` | 7,8:1 | 7:1 | tient |
| `on-warning-container` sur `warning-container` | 7,6:1 | 7:1 | tient |
| `danger` sur `surface` | 8,0:1 | 7:1 | tient |
| `on-danger-container` sur `danger-container` | 8,0:1 | 7:1 | tient |
| `on-night` sur `night-1` | 13,9:1 | 7:1 | tient |
| `on-night-muted` sur `night-1` | 7,4:1 | 7:1 | tient |
| `primary-vivid` sur `surface` | 6,4:1 | 3:1 (jamais du texte) | tient |
| `primary-vivid` sur `background` | 7,1:1 | 3:1 (jamais du texte) | tient |
| `border-control` sur `surface` | 5,2:1 | 3:1 | tient |
| `switch-knob-locked` sur `primary-vivid` | 5,7:1 | 3:1 | tient |
| `switch-knob-locked` sur `border-control` | 4,6:1 | 3:1 | tient |
| `on-surface-faint` sur `surface` | 3,4:1 | 3:1 (jamais du texte) | tient |

Quatre notes que les tables ne portent pas. Les tokens `night-*` et `on-night*` sont identiques
dans les deux thèmes : l'overlay Hardcore ne suit pas le thème, il est la nuit dans les deux.
`on-surface-disabled` et le fond d'un contrôle inactif sont exempts de barre (WCAG les exempte) —
c'est pourquoi aucun texte porteur d'information ne les emploie jamais, la raison d'un levier grisé
en premier. Les `mesh-*` sont décoratifs : aucun texte ne se pose jamais dessus directement, donc
aucune paire ne les implique. Deux autres emplois sont décoratifs par contrat et donc exemptés en
le disant : les filets en `border` (séparateurs, contours de cartes — l'information ne repose
jamais sur eux, `border-control` prend le relais quand elle doit s'y trouver) et
`surface-skeleton`, qui n'est jamais le seul signal de chargement — il accompagne toujours une
Caption « Chargement… » en `on-surface-muted`.

## Vue d'ensemble

Breeze force la pause sur le poste de travail d'un travailleur du savoir qui vit devant son Mac six
à dix heures par jour. La réponse émotionnelle visée : le calme d'une contrainte qu'on s'est choisie
— de l'air, du verre, des pastels — jamais la punition. L'interface dit toujours la vérité, y
compris quand elle refuse : un levier grisé explique pourquoi, une limite s'annonce à froid. Ce
design ne doit jamais devenir une vitrine séduisante qui cache la contrainte, ni un utilitaire gris
qui la rend punitive.

## Couleurs

La palette part du ciel : un bleu d'action posé et sérieux (`primary`), doublé d'un bleu vif
(`primary-vivid`) pour les éléments non textuels porteurs d'état — anneaux de progression, jauges,
anneau de focus, piste d'un switch allumé — et rien d'autre : il attire l'œil mais ne porte jamais
de texte. `success` (menthe) appartient à la pause servie, `accent` (violet) à la sévérité,
`warning` aux badges de réparation, `danger` à la seule sortie d'urgence — jamais décoratif. Les
fonds sont des blancs et des nuits teintés de bleu-gris ; le noir pur et le blanc pur n'existent
pas. Le clair et le sombre sont deux jeux complets et symétriques : le produit suit le thème de
macOS. Les quatre `mesh-*` (bleu, lavande, menthe, rose pâle) composent le décor d'arrière-plan,
jamais un fond de texte direct ; leurs arrêts transparents sont la même teinte à alpha zéro, jamais
le mot-clé `transparent`, qui est un noir pur. `on-danger-container` est le seul `on-*-container`
qui ne soit pas l'alias de sa teinte : `danger` sur son container tenait 6,7:1, l'assombrir à
`#7E1A2C` porte la paire à 8,6:1 sans toucher au rouge des textes sur surface. La filiation Material 3 est un alias, posé une fois :
`surface-dim` joue surface-container, `danger` joue error, `border` joue outline-variant,
`border-control` joue outline, les `*-container`/`on-*-container` sont les paires tonales de M3 —
et une divergence est assumée : l'élévation vient du verre et de l'ombre, pas de la teinte de
surface.

## Typographie

Instrument Sans partout, en une seule famille, avec sa pile de repli déclarée dans chaque niveau —
SF Pro d'abord, dont les métriques sont proches. Six niveaux se lisent contre M3 : H1 →
headline-small, H2 → title-medium, H3 → label-large, Body → body-medium, Caption → body-small,
Overline → label-small ; Display n'a pas d'équivalent M3 — c'est le style propre des décomptes, et
il ne sert à rien d'autre. Display n'existe que pour les décomptes
et les grands chiffres ; H1 ouvre un écran, H2 titre une carte, H3 porte les libellés des leviers
et des boutons. Body raconte, Caption porte les raisons — celles des leviers grisés notamment — et
Overline étiquette en capitales les axes et les états. Tout chiffre qui compte du temps s'écrit en
chiffres tabulaires (`font-variant-numeric: tabular-nums`). Le tracking se resserre à mesure que la
taille monte, jamais l'inverse ; la hauteur de ligne se desserre à mesure qu'elle descend.

## Mise en page

L'échelle d'espacement est la suite 4, 8, 16, 24, 32, 48, 64 — le 4 réservé aux micro-ajustements,
rien entre deux échelons. Les traits ont leur propre échelle (`stroke` : 1 filet, 2 anneau) et ne
comptent pas comme des espacements. La densité est confortable : Breeze s'ouvre trente secondes à
la fois depuis la barre de menus, il doit se lire d'un regard. La taille dessinée n'est pas la
cible : tout contrôle dessiné sous 48 px — bouton de 40, switch de 24 — étend sa zone cliquable à
48 px au minimum ; les rangées font 48 px dessinés. L'espace entre deux composants appartient à
leur parent (`gap` d'un conteneur flex ou grid), jamais au composant : un composant ne porte pas de
marge externe. Une fenêtre respire par un padding de 16 minimum ; l'onboarding monte à 32.

## Élévation et profondeur

La profondeur vient du verre, pas des bordures : un plan flotte parce qu'on devine le fond à
travers lui. Trois verres et une ombre de carte, déclarés par thème, jamais improvisés.

| Recette | Thème clair | Thème sombre |
|---|---|---|
| Panneau — fond | `rgba(251, 252, 254, 0.62)` | `rgba(27, 31, 44, 0.62)` |
| Panneau — flou / saturation | 28 px / 1,5 | 28 px / 1,5 |
| Panneau — liseré 1 px | `rgba(251, 252, 254, 0.78)` | `rgba(232, 235, 244, 0.12)` |
| Panneau — ombre | `0 24px 48px -20px rgba(28, 33, 48, 0.28)` | `0 24px 48px -20px rgba(14, 19, 34, 0.6)` |
| Bannière — fond | `rgba(251, 252, 254, 0.55)` | `rgba(27, 31, 44, 0.55)` |
| Bannière — flou / saturation | 18 px / 1,4 | 18 px / 1,4 |
| Bannière — liseré 1 px | `rgba(251, 252, 254, 0.78)` | `rgba(232, 235, 244, 0.12)` |
| Bannière — ombre | `0 8px 24px -12px rgba(28, 33, 48, 0.18)` | `0 8px 24px -12px rgba(14, 19, 34, 0.45)` |
| Nuit (Hardcore) — fond | `rgba(27, 35, 64, 0.55)` sur `night-1`→`night-2` | identique |
| Nuit — flou / saturation | 30 px / 1,3 | identique |
| Nuit — liseré 1 px | `rgba(239, 243, 251, 0.14)` | identique |
| Voile (Mode Simple) — fond | `rgba(251, 252, 254, 0.44)` | `rgba(27, 31, 44, 0.44)` |
| Voile — flou / saturation | 16 px / 1,2 — devinable, jamais lisible | 16 px / 1,2 |
| Carte — ombre seule | `0 2px 8px rgba(28, 33, 48, 0.06)` | `0 2px 8px rgba(14, 19, 34, 0.35)` |

Chaque verre porte un reflet interne en haut (`inset 0 1px 0` de son liseré). Les rgba dérivent
des tokens : les fonds clairs de `surface`, les fonds sombres de `surface` sombre, la nuit de
`night-1` et `on-night`, les ombres de l'encre et de `night-2`. En thème sombre, les plans se
distinguent par `border` et par l'ombre, pas par l'écart de luminance des surfaces — trop faible,
et c'est assumé. Trois réglages macOS ont leur repli déclaré : sous « Réduire la transparence »,
panneau et bannière deviennent `surface` opaque avec `border`, la nuit devient `night-1` opaque —
la hiérarchie doit tenir sans le flou ; sous « Contraste augmenté », `border-control` remplace
`border` sur les contours de plans ; sous « Réduire les animations », voir Composants.

## Formes

Trois rayons et un cercle : 8 pour les contrôles, 16 pour les cartes, 24 pour les fenêtres et
panneaux, la pilule pour les chips et bannières flottantes. Les rayons sont concentriques : le
rayon intérieur vaut l'extérieur moins l'écart, et le rayon se mesure sur la boîte de padding — le
filet de 1 px n'entre pas dans l'écart, sans quoi aucune valeur de l'échelle ne serait
concentrique. Un panneau de rayon 24 avec un padding de 8 donne 16 à son enfant, qui donne 8 au
sien. L'arrondi généreux signale la
douceur de la contrainte choisie ; rien n'est tranchant dans Breeze, pas même le refus.

## Composants

Un bouton n'a qu'une taille de libellé (H3). Ses cinq états sont tous des tokens : repos, survol et
appui sont des entrées du bloc `components` ; le focus est le même pour tout composant interactif —
l'anneau du bloc `focus`, en `primary-vivid`, 2 px décalés de 2 px — sans lui le clavier est
aveugle, et Breeze se pilote au clavier ; le désactivé troque ses couleurs pour `surface-dim` et
`on-surface-disabled`, sans opacité. **La raison d'un levier grisé n'est jamais grisée avec lui** :
elle reste en Caption `on-surface-muted`, pleine opacité — c'est une règle de produit. Les chips
existent en quatre teintes, chacune sur sa paire `*-container` / `on-*-container`. Une rangée de
liste porte le levier à gauche, sa valeur ou sa raison à droite ; survol en `surface-dim`, jamais
de changement de taille. Le switch est une piste (`switch-off` / `switch-on`) dont le padding de 4
loge la pastille (`switch-knob`, 16) ; la course est dérivée — largeur moins retraits — jamais un
échelon de la suite. Verrouillé, la piste **garde** la couleur de sa position (l'information ne se
perd pas) et c'est la pastille qui passe en `switch-knob-locked`, avec toujours la mention qui
explique le verrou. La zone cliquable étendue à 48 se pose par un pseudo-élément invisible qui
déborde le dessin — c'est le geste canonique, montré dans le miroir. Le `countdown` est tabulaire,
en `on-surface` sur les surfaces, `on-night` sur l'overlay. Le mouvement a ses tokens (`motion`) :
150 ms pour un survol, 200 ms pour une pastille ou un panneau, 300 ms pour un overlay, une seule
courbe ; sous « Réduire les animations », tout changement d'état est instantané et le décompte
change sans glisser. Le chargement et le vide sont deux états distincts : ce qui charge montre
`surface-skeleton` accompagné d'une Caption « Chargement… » — le squelette seul ne porte jamais
l'information ; les trois vides — première utilisation, recherche sans résultat, liste vidée — se
disent chacun par une phrase Body qui explique quoi faire ensuite, jamais par un écran nu.

## Ce qu'on fait, et ce qu'on ne fait jamais

Ce qu'on fait :

1. Toute couleur, toute distance, tout rayon, toute durée vient de ce fichier — une valeur
   littérale dans un composant est un défaut de revue.
2. Tout levier désactivé affiche la raison de son grisement, en Caption `on-surface-muted` pleine
   opacité, à côté de lui.
3. Tout décompte s'écrit en chiffres tabulaires.
4. Tout verre suit l'une des recettes d'Élévation, valeurs et repli opaque compris.
5. Toute icône est un SVG au trait sur grille 16, 20 ou 24, du même style de tracé.
6. Toute paire texte-fond nouvelle se calcule et s'ajoute aux deux tables avant d'être employée.

Ce qu'on ne fait jamais :

1. Jamais de noir pur ni de blanc pur, nulle part — surfaces, textes, ombres comprises.
2. Jamais de texte porteur d'information en `primary-vivid`, `on-surface-faint` ou
   `on-surface-disabled`.
3. Jamais de texte posé directement sur le mesh ou sur une image : toujours une surface ou un verre
   mesuré entre les deux.
4. Jamais d'emoji ni de glyphe de police en guise d'icône.
5. Jamais une promesse d'infaillibilité dans un texte d'interface — le brief l'interdit, le design
   la rendrait crédible.
6. Jamais un espacement hors de la suite, un quatrième rayon, ou un huitième niveau typographique.
