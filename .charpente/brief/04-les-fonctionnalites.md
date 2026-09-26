## 8. Le produit, fonctionnalité par fonctionnalité

**Sept fonctionnalités, et rien d'autre au lancement.** Chacune est décrite par ce qu'elle fait, ce
qu'elle exige, et ce qu'elle refuse. C'est une de moins que dans la version précédente de ce brief :
l'ancien « déclencheur intelligent » n'existe plus comme réglage séparé — il est absorbé par le statut
`ignorée` (§8.3) —, et « exceptions automatiques » a disparu avec la négociation qu'il servait.

### 8.1 Le cycle de travail et de pause

**Ce qu'elle fait.** Compte une phase de travail, puis impose une phase de pause, et recommence tant
que Breeze est actif.

**Ce qu'elle exige.** Rien. C'est la seule fonctionnalité qui tourne sans permission ni
configuration : un utilisateur qui ferme l'onboarding à la première fenêtre a un cycle qui tourne. Le
préavis passe par une bannière propre à Breeze plutôt que par une notification système ; c'est
uniquement quand cette bannière ne peut pas se poser — le compositeur refuse la couche (§5.3) — que
Breeze porte le préavis par une notification système,
et ne demande la permission correspondante que sur l'OS qui l'exige (§5.5).

**Ce qu'elle refuse.** Aucune commande d'arrêt du cycle. Qui veut arrêter Breeze le suspend, ou le
quitte — et quitter pendant le préavis ou pendant une pause est compté de la même façon (§10.2).

### 8.2 Le réglage du rythme

**Ce qu'elle fait.** Fixe la durée de travail, la durée de pause, la plage horaire active et les
jours actifs. Trois rythmes prêts à l'emploi sont proposés à l'onboarding, le réglage fin reste
accessible.

**Ce qu'elle exige.** Des valeurs dans les bornes du §12, une pause qui ne dépasse jamais la durée de
travail.

**Ce qu'elle refuse.** Trois combinaisons, refusées à la saisie — l'écran de réglages ne les
enregistre pas et dit pourquoi : aucun jour actif, aucune plage horaire dont le début égale la fin, et
aucune pause qui dépasserait la durée de travail. Les trois enfermeraient le cycle dans un état sans
sortie ou videraient le rythme de son sens. **Une plage dont la fin précède le début traverse
minuit, et c'est admis** — 22 h à 2 h est un rythme de travail réel, pas une erreur de saisie. Un
changement de rythme ne s'applique jamais à une phase déjà en cours, qu'il l'allonge ou la
raccourcisse : il attend toujours le cycle suivant, sans notion de sens (§10.3).

### 8.3 Le statut des applications

**Ce qu'elle fait.** Donne à chaque application connue de Breeze l'un de trois statuts, qui décide de
deux choses : ce qu'un overlay du Mode Simple lui fait, et si son usage fait avancer le décompte de
travail.

| Statut | Pendant une pause en Mode Simple | Effet sur le décompte du travail |
|---|---|---|
| **Bloquée** | Recouverte par un overlay | Son usage compte comme du travail |
| **Épargnée** | Utilisable librement | Son usage compte comme du travail |
| **Ignorée** | Utilisable librement | Son usage **ne compte pas** comme du travail |

**Ce qu'elle exige.** Rien de la part de l'utilisateur : toute application inconnue est **bloquée**
dès sa première apparition au premier plan. Un défaut permissif produirait une première pause sans
effet, ce qui casse la promesse avant qu'elle ait été éprouvée.

**Ce qu'elle refuse.**

- Aucun effet visuel du statut en Mode Hardcore : l'overlay y couvre chaque écran entier, quel que
  soit le statut.
- **Un changement de statut qui affaiblit la contrainte — épargner ou ignorer une application, ou
  débloquer ce qui était bloqué — ne prend effet qu'au cycle suivant ; un changement qui la renforce
  s'applique immédiatement (§10.3).** Sans cette règle, `ignorée` deviendrait la négociation que le
  reste du produit vient de fermer : marquer l'application au premier plan comme `ignorée` à la
  quarante-neuvième minute d'une phase de cinquante gèlerait la pause à venir sans qu'aucune règle ne
  s'y oppose. Le statut d'une application est l'un des trois réglages protégés par le §10.3, avec la
  sévérité et le rythme — chacun selon son propre mécanisme.
- Les applications de la liste de sécurité (§10.6) valent toujours `épargnée`, sans possibilité de
  les marquer `ignorée` : leur usage compte comme du travail, sans exception.

### 8.4 Mode Simple

**Ce qu'elle fait.** Pendant la pause, recouvre chaque fenêtre visible de chaque application bloquée
d'un voile qui laisse deviner le contenu sans le rendre lisible, et porte le décompte. Le reste de la
machine reste utilisable.

**Ce qu'elle exige.** De pouvoir observer le cadre des fenêtres d'autrui : sur macOS, Windows et X11,
aucune permission — macOS expose les cadres sans permission (liste des fenêtres du système : bornes,
propriétaire, couche, numéro, jamais le titre ni le contenu), et Breeze n'y demande pas
l'Accessibilité (§5.2). Sur Wayland, aucun compositeur ne l'expose : le Mode
Simple y est **toujours** le voile plein écran par moniteur du §10.5, et l'onboarding le dit. Quand
la capacité manque, quelle qu'en soit la raison, le mode dégradé du §10.5 fait autorité, et lui seul.

**Ce qu'elle refuse.** Aucune application n'est masquée, quittée ni suspendue — Breeze se pose devant,
il ne touche à rien. La pause reste contournable en amenant une application épargnée par-dessus la
zone couverte, et Breeze ne prétend pas le contraire (§5) : l'utilisateur qui veut une pause qui tient
a le Mode Hardcore. Aucun bouton n'écourte la pause, à aucun moment — c'est le Mode Hardcore, pas le
Simple, qui porte l'unique geste de sortie du §8.5.

### 8.5 Mode Hardcore

**Ce qu'elle fait.** Pendant la pause, couvre l'intégralité de chaque écran physique d'un fond opaque
portant le décompte, identique sur tous les écrans, et tient devant tout ce que le système laisse
recouvrir (§5.3). Sur macOS, la barre de menus et le Dock sont masqués et le changement d'application
est désactivé, dans la limite de ce que macOS autorise. Sur Windows et Linux X11, l'overlay est
« toujours devant » et **revient devant en moins de 500 ms** quand une fenêtre, le menu Démarrer ou
un menu contextuel passent dessus (§12.2) — il ne bloque pas Alt-Tab, ni la touche Windows ou Super,
ni Ctrl-Alt-Suppr, parce que les bloquer exigerait de lire les touches (§13). Sur Wayland KDE et
wlroots, l'overlay occupe la couche que le compositeur réserve à ce qui passe devant tout ; sur GNOME
Wayland, c'est une fenêtre plein écran ordinaire qui ne tient pas devant le système, et le choix
Hardcore y reste offert avec cette légende sous le bouton (§5.3). **Le raccourci de fermeture de
Breeze lui-même — Cmd-Q sur macOS, la commande « Quitter » du menu de l'icône partout — n'est jamais
neutralisé** : quitter Breeze reste possible pendant une pause Hardcore comme pendant n'importe quel
autre état (§6, §15 décision 5). Ce que Breeze neutralise là où il le peut, ce sont les raccourcis
d'une fenêtre ou d'une application recouverte, jamais le sien.

**Ce qu'elle exige.** Aucune permission, sur aucun OS.

**Ce qu'elle refuse.** Aucun bouton, aucun délai à demander, aucun changement de sévérité pendant la pause.
Aucune notification émise par Breeze pendant la pause. Aucune prétention à l'infaillibilité : forcer à
quitter, changer d'utilisateur, débrancher l'écran ou éteindre la machine restent possibles, et c'est
déclaré à l'onboarding, avant le choix du mode, par la phrase propre à la session (§5.7). **Aucun
hook clavier, aucune exécution privilégiée pour « tenir mieux »** (§13) : un Hardcore qui revient
devant en une demi-seconde est ce que Breeze promet sur Windows et Linux, et c'est l'hypothèse
risquée que le §17 nomme.

**Le seul geste que porte l'overlay.** Maintenir la touche d'échappement dix secondes consécutives
remplit un anneau, puis ouvre une confirmation dont le bouton d'annulation est en position par défaut.
La mention « Maintiens Échap pour sortir » reste affichée en permanence. Ce geste **n'est pas une
négociation** : il ne raccourcit rien à prix réduit, il met fin à la pause en cours sans quitter
Breeze — le cycle repart directement en travail, décompte plein, sans passer par le retour de trois
secondes qui suit une pause servie (§10.1). Pour ce que ça coûte, c'est la même issue qu'un abandon
par le raccourci de fermeture ou par le menu de l'icône : comptée interrompue, créditée à la dette de posture une fois qu'elle existe
(§9.2, §10.2) — la différence n'est que le chemin pour y arriver, jamais le prix, et ce chemin-ci ne
détruit pas le processus. Il n'est pas rationné — un garde-fou qu'on épuise n'en est plus un — et son
coût est sa seule friction :
dix secondes de maintien et une confirmation, à chaque fois, sans jamais diminuer avec la répétition.
Le décompte de pause ne s'arrête jamais pendant les dix secondes ni pendant que la confirmation est
ouverte ; si l'échéance de la pause tombe pendant ce délai, la pause est comptée prise et la
confirmation se ferme sans effet (§10.1).

### 8.6 L'icône d'état et les réglages

**Ce qu'elle fait.** Loge Breeze dans la **barre de menus (macOS) ou la zone de notification
(Windows/Linux)**, sans icône dans le Dock ni dans la barre des tâches. L'icône porte l'état du cycle,
et un panneau donne accès au décompte, à la sévérité, au bilan du jour, aux réglages, et permet de
quitter Breeze — c'est ce menu, et lui seul avec le raccourci de fermeture du système (Cmd-Q sur
macOS), que le §10.2 nomme comme porte de terminaison propre.

**Ce qu'elle exige.** Rien sur macOS, Windows, X11, KDE et wlroots. **Sur GNOME, l'icône d'état exige
l'extension AppIndicator** ; sans elle, Breeze n'a pas de zone où se loger et **s'ouvre par sa
fenêtre** : relancer l'application ramène le panneau, et l'onboarding le dit sur cette session (§5.5).
Rien du cycle n'en dépend — l'icône est une façon d'atteindre le panneau, pas une capacité de la
contrainte.

**Ce qu'elle refuse.** Pendant une pause en Mode Hardcore, l'icône est masquée là où le système le
permet (macOS) ou recouverte par l'overlay (Windows, Linux), et le panneau est donc inatteignable —
c'est voulu, et c'est cohérent avec l'absence de tout levier pendant une pause. **La commande de
désinstallation n'est pas offerte tant qu'une pause est due ou en cours** — aucune commande n'affaiblit
une pause en cours (§10.3), et la désinstallation en est une. La désinstallation propre s'arrête là où
le système l'arrête : Breeze supprime ses données et son élément de démarrage, puis se quitte.
Breeze ne demandant aucune permission d'Accessibilité, rien ne reste à retirer à la main, sur aucun
système (§5.2).

### 8.7 La première ouverture

**Ce qu'elle fait.** Amène l'utilisateur à un cycle qui tourne en moins de quatre-vingt-dix secondes,
et lui fait **comprendre ce que Breeze va faire à son écran, et qu'il n'y aura rien à négocier une
fois que ça a commencé**, avant que ça arrive. Cinq écrans : bienvenue, rythme, sévérité, applications,
démarrage.

**Ce qu'elle exige.** L'écran de la sévérité porte la prévisualisation animée des deux modes — c'est
le point de décision produit le plus important du parcours, puisque plus rien ne se rattrape après
coup — **et c'est cet écran qui porte la phrase à froid propre à la session** (§5.7), sous le choix,
avant qu'il soit fait. Aucune permission n'est demandée à l'onboarding, sur aucun système : sur
macOS, les cadres des fenêtres que le Mode Simple voile se lisent sans permission (§5.2, §8.4).

**Ce qu'elle refuse.** Aucune permission n'est obligatoire pour continuer — Breeze fonctionne alors en
mode dégradé (§10.5), avec un badge de réparation tant qu'elle manque ; sur une session où la capacité
n'existe pas du tout (Wayland pour les cadres, GNOME pour le premier plan), il n'y a rien à réparer,
donc pas de badge — seulement la phrase à froid. Fermer la fenêtre avant la
fin ne laisse jamais Breeze inerte : ce qui a été choisi est conservé, le reste prend les valeurs par
défaut du §12, et le premier cycle démarre en [TRAVAIL] si la plage horaire l'autorise, en [INACTIF]
sinon — la plage étant désactivée par défaut, c'est [TRAVAIL] dans le cas courant. **L'onboarding ne
se rejoue jamais automatiquement** ; il reste accessible depuis l'aide, et le rouvrir le reprend
depuis le premier écran plutôt que là où il avait été quitté.
