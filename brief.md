# Breeze — Brief produit

**Version** : 2.0 · **Date** : 2026-08-27 · **Statut** : définitif, prêt pour design et développement
**Plateforme** : macOS 13 Ventura et ultérieur (Apple Silicon + Intel) · **Stack** : Electron (main + renderer) + module natif Objective-C++

Ce document est la référence unique du MVP. Toute règle y figure à un seul endroit ; les autres
sections y renvoient. Les inconnues techniques restantes sont listées en §9, chacune avec le
comportement de repli déjà décidé : aucune ne peut bloquer le design ni le développement.

---

## 1. Vision

Breeze est une application de barre de menus macOS qui force la pause. Là où les concurrents envoient
une notification qu'on balaie sans y penser, Breeze intervient sur le poste de travail : il fige les
applications qui retiennent l'utilisateur, ou verrouille l'écran entier, le temps d'un compte à
rebours qu'on ne peut pas écourter d'un clic.

**Promesse** : « Tu ne peux pas ignorer Breeze. C'est le but. »

**Utilisateur cible** : travailleur du savoir sur Mac, 6 à 10 heures d'écran par jour, qui a déjà
essayé des rappels passifs et les a désactivés au bout d'une semaine.

**Problème résolu** : les rappels de pause échouent parce qu'ils sont négociables. Breeze rend la
pause non négociable, à un niveau de contrainte que l'utilisateur choisit lui-même à froid, avant
d'être en pleine session de travail.

---

## 2. Périmètre

### Dans le MVP

| # | Fonctionnalité | Description |
|---|---|---|
| F1 | Rappel périodique | Compte à rebours de travail configurable, suivi d'une pause de durée configurable |
| F2 | Personnalisation | Durée de travail, durée de pause, plage horaire active, jours actifs |
| F3 | Ciblage d'applications | Statut par application : bloquée, épargnée, ignorée (§3.1) |
| F4 | Déclencheur intelligent | Le décompte de travail n'avance que si une application déclencheuse est au premier plan |
| F5 | Mode Simple | Overlay par fenêtre au-dessus des seules applications bloquées |
| F6 | Mode Hardcore | Overlay plein écran sur tous les écrans, sorties neutralisées |
| F7 | Barre de menus | Icône d'état, compte à rebours, popover de contrôle, accès aux réglages |
| F8 | Onboarding | Parcours de première ouverture, demande des permissions, configuration minimale |

### Hors MVP

Chaque ligne est un refus assumé, avec sa version visée, pas un oubli.

| Sujet | Version visée | Raison |
|---|---|---|
| Profils multiples (plusieurs jeux de réglages nommés) | v1.1 | Un seul jeu de réglages couvre le besoin du MVP ; la gestion de profils demande un écran entier |
| Bouclier de calendrier (KF1, §7) | v1.1 | Demande la permission Calendriers et une logique de placement de créneau |
| Dette de posture (KF2, §7) | v1.1 | Ses seuils se règlent en test utilisateur, pas au lancement |
| Détection de session profonde (KF3, §7) | v1.2 | Repose sur des mesures que seul l'usage réel calibre |
| Synchronisation entre appareils, compte, backend | Aucune | Contredit le refus n° 1 ci-dessous |
| Statistiques au-delà de 30 jours | Aucune | Un historique plus long demande un écran d'exploration que la promesse ne justifie pas |
| Windows, Linux | Aucune | Le produit repose sur des API macOS sans équivalent |
| Exercices guidés, contenu audio ou vidéo | Aucune | Élargit le périmètre sans renforcer la promesse |
| Mode équipe, classement social | Aucune | Contredit le refus n° 1 |

### Non couvert par ce brief

**Le modèle économique.** Le MVP est gratuit, sans licence, sans activation, sans compte. Ce choix
n'est pas commercial : toute monétisation par licence en ligne rouvre la contrainte « aucune sortie
réseau » (§4.7) et doit donc être décidée avec elle, en v1.1 au plus tôt.

### Refus explicites

- **Breeze ne collecte aucune donnée hors de la machine.** Aucune télémétrie, aucun compte, aucune
  remontée d'usage, même consentie, dans le MVP. C'est une contrainte d'architecture, pas une option
  de réglage. Seule exception, déclarée et visible : la vérification de mise à jour, qui transmet la
  version installée et l'adresse IP au serveur de publication (§4.9). L'utilisateur peut la couper
  dans Réglages › Général ; Breeze fonctionne alors sans aucune sortie réseau.
- **Breeze n'enregistre jamais le contenu des applications.** Breeze lit le nom et le bundle
  identifier de l'application au premier plan, ainsi que la position et la taille de ses fenêtres.
  Jamais le titre de fenêtre, jamais le contenu de l'écran, jamais les frappes clavier.
- **Le Mode Hardcore n'empêche pas l'arrêt du Mac.** Couper l'alimentation, forcer l'extinction, ou
  tuer le processus depuis un autre compte reste possible. Breeze l'assume et le dit à l'utilisateur
  dès l'onboarding.

---

## 3. Analyse fonctionnelle et parcours utilisateur

### 3.1 Modèle conceptuel

Trois objets suffisent à décrire tout le produit.

**Le Cycle** — l'unité de temps de Breeze : une *phase de travail* suivie d'une *phase de pause*. Le
cycle se répète tant que Breeze est actif. Ses durées par défaut sont celles du preset *Classique*
de l'onboarding (§3.3), et figurent dans la table ci-dessous.

**Les Réglages** — un jeu unique de valeurs. Le MVP n'en a qu'un seul ; les profils nommés sont hors
périmètre (§2). Cette table fait autorité sur toutes les valeurs par défaut du produit :

| Réglage | Défaut |
|---|---|
| Durée de travail | 50 min |
| Durée de pause | 10 min |
| Sévérité | Simple |
| Plage horaire active | Désactivée — Breeze tourne à toute heure |
| Jours actifs | Les sept jours |
| Déclencheur intelligent | Désactivé |
| Statut d'une application inconnue | `blocked` |
| Inhibitions visio, partage d'écran, présentation | Activées |
| Inhibition « entre 12 h et 14 h » | Désactivée |
| Titre à côté de l'icône | `Minutes restantes` |
| Lancement au démarrage | Activé |
| Langue | Celle du système ; anglais si elle n'est ni le français ni l'anglais |
| Thème | Système |
| Vérification des mises à jour | Activée |
| Raccourci `⌥⌘B` | Actif |
| Raccourci `⌥⌘S` | Inactif |

Un changement de durée de travail prend effet au cycle suivant. Un changement de durée de pause
prend effet à la pause suivante. Aucun des deux ne modifie une phase déjà en cours.

**Le Statut d'application** — chaque application connue de Breeze porte un statut parmi trois :

| Statut | Effet pendant la pause | Effet sur le décompte du travail |
|---|---|---|
| `blocked` | Recouverte par un overlay en Mode Simple | Son usage compte comme du travail |
| `allowed` | Utilisable librement en Mode Simple | Son usage compte comme du travail |
| `ignored` | Utilisable librement en Mode Simple | Son usage ne compte pas comme du travail |

**Statut par défaut : `blocked`.** Une application inconnue est bloquée dès sa première apparition au
premier plan. Un défaut permissif produirait une première pause sans effet, ce qui casse la promesse
avant que l'utilisateur l'ait éprouvée. L'onboarding sert donc à *épargner* ce dont on a besoin
pendant une pause, pas à choisir ce qu'on bloque.

**Exception non modifiable** : les applications de la liste blanche système (§4.7) sont `allowed` en
dur. L'interface les affiche avec la mention « Toujours autorisée » et sans contrôle de statut.

**Le statut ne s'applique qu'au Mode Simple.** En Mode Hardcore, l'overlay couvre chaque écran
entier : aucune application n'est utilisable, liste blanche système comprise. En Hardcore, le statut
ne sert plus qu'au décompte du travail — colonne de droite du tableau ci-dessus.

**Statut et déclencheur sont deux mécanismes distincts, et ils se combinent ainsi :**

- Déclencheur intelligent **désactivé** (défaut) : tout usage du Mac fait avancer le décompte, sauf
  celui d'une application `ignored`.
- Déclencheur intelligent **activé** : le décompte n'avance que si l'application au premier plan est
  dans la liste des déclencheurs. Le statut `ignored` devient sans effet sur le décompte, puisque
  seule l'appartenance à la liste des déclencheurs le fait avancer. Une application peut être à la
  fois déclencheuse et `blocked` : c'est même le cas courant (Figma déclenche le travail et est
  figée pendant la pause).

### 3.2 Machine à états

```
   ┌──────────────────────────── arrêter, fin de plage horaire, jour inactif ──┐
   │                                                                           │
   ▼                                                                           │
[INACTIF] ──démarrer──▶ [ARMÉ] ──déclencheur au premier plan──▶ [TRAVAIL] ─────┤
                           ▲                                        │          │
                           └──perte du déclencheur──────────────────┤          │
                              ou inactivité de plus de 3 min        │          │
                                                       fin du décompte         │
                                                                    ▼          │
                          [TRAVAIL] ◀──report accordé (§3.7)──[PRÉAVIS] (60 s) │
                                                                    │          │
                                             budget épuisé ou T-0 atteint      │
                                                                    ▼          │
                                                          [PAUSE ACTIVE] ──────┤
                                                                    │          │
                                                       fin du décompte         │
                                                                    ▼          │
                                                          [RETOUR] (3 s) ──────┤
                                                                    │          │
                                                                    ▼          │
                                                              [ARMÉ] ──────────┘
```

**Précisions qui ferment le diagramme :**

- Quand le déclencheur intelligent est désactivé (défaut), l'état **[ARMÉ]** est traversé sans
  attente : Breeze passe immédiatement en **[TRAVAIL]**.
- **[RETOUR]** revient toujours en **[ARMÉ]**, jamais directement en **[TRAVAIL]**. Le cycle suivant
  respecte donc la condition de déclenchement et la plage horaire comme le premier.
- Un cycle qui commence part d'un décompte de travail plein. Une phase de travail interrompue par la
  perte du déclencheur ou par l'inactivité est **gelée**, pas remise à zéro : au retour du
  déclencheur ou de l'utilisateur, le décompte reprend là où il s'était arrêté.
- La sortie vers **[INACTIF]** est possible depuis n'importe quel état. Les conditions et le sort
  d'une pause en cours sont donnés plus bas.

**États transversaux :**

- **[SUSPENDU]** — l'utilisateur a suspendu Breeze pour 15 min, 1 h, ou jusqu'au lendemain 6 h. Tout
  décompte est gelé. Pendant **[PAUSE ACTIVE]**, la suspension suit la même règle que la fin
  anticipée de pause (§3.7) : possible après 60 s en Mode Simple, impossible en Mode Hardcore. À
  l'expiration, Breeze repart en **[ARMÉ]** si la plage horaire l'autorise, et en **[INACTIF]**
  sinon. Le décompte de travail repart d'une phase pleine.
- **[VEILLE]** — le Mac dort ou la session est verrouillée. Tout décompte est gelé et recalculé au
  réveil (§4.6).
- **[INHIBÉ]** — une condition externe repousse la pause : appel visio, partage d'écran,
  présentation plein écran, plage d'exception horaire (§3.5). Le décompte de travail est terminé,
  mais la pause attend. L'attente consomme le budget de report du cycle (§3.7) ; budget épuisé, la
  pause démarre malgré l'inhibition. L'icône porte alors l'anneau complet du préavis, sans
  pulsation, et le popover affiche « Pause en attente » avec les minutes de budget restantes.

**Fin de plage horaire pendant un cycle :**

- Pendant **[TRAVAIL]** ou **[ARMÉ]** : Breeze passe en **[INACTIF]** à la fin de la plage. Le
  décompte en cours est perdu, pas mémorisé.
- Pendant **[PRÉAVIS]**, **[PAUSE ACTIVE]** ou **[RETOUR]** : la pause va à son terme, puis Breeze
  passe en **[INACTIF]**. Une pause commencée n'est jamais coupée par l'horloge.

### 3.3 Onboarding — première ouverture

Objectif : l'utilisateur a un cycle qui tourne en moins de 90 secondes, et il a compris ce que Breeze
va faire à son écran **avant** que ça arrive.

Fenêtre classique, pas un popover : 720 × 520, non redimensionnable, centrée.

**Écran 1 — Bienvenue.** Une phrase, l'icône, un bouton « Commencer ». Aucun champ.

**Écran 2 — Ton rythme.** Trois presets en grandes cartes cliquables, plus un lien « Personnaliser » :

- *Pomodoro* — 25 min de travail / 5 min de pause
- *Classique* — 50 min de travail / 10 min de pause **(pré-sélectionné)**
- *Longue haleine* — 90 min de travail / 15 min de pause

Le lien « Personnaliser » remplace les trois cartes par deux champs numériques. Il accepte 5 à
180 minutes de travail et 1 à 60 minutes de pause, la pause ne pouvant dépasser la durée de travail.
Ces bornes valent aussi pour Réglages › Rythme.

**Écran 3 — Quel niveau de fermeté.** Deux cartes, chacune portant une **animation de
prévisualisation** — une maquette de bureau miniature où l'overlay vient se poser. C'est le point de
décision produit le plus important : l'utilisateur doit voir le Mode Hardcore avant de le choisir.

- *Simple* — « On grise les applications qui te retiennent. Le reste du Mac fonctionne. »
  **(pré-sélectionné)**
- *Hardcore* — « On prend l'écran entier. Tu ne peux rien faire jusqu'à la fin. » avec la mention
  « Tu peux toujours éteindre ton Mac. On ne bloque pas ça. »

**Écran 4 — Permissions.** Une carte par permission, avec son état en direct (`⚠️ requise` →
`✅ accordée`) et un bouton « Autoriser ». Pour l'Accessibilité, ce bouton déclenche le prompt
système puis ouvre
`x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility`. L'écran interroge
l'état à 1 Hz et coche dès que la permission est accordée (§4.2).

- **Accessibilité** — « pour poser l'écran de pause exactement sur tes fenêtres ». Requise pour le
  Mode Simple (F5) seulement.
- **Notifications** — « pour te prévenir une minute avant la pause ». Optionnelle.

Le bouton « Continuer » reste actif sans aucune permission. Breeze fonctionne alors en mode dégradé,
décrit exhaustivement en §3.9, et pose un bandeau de réparation dans le popover tant qu'une
permission requise par la sévérité choisie manque.

**Écran 5 — Applications.** Les applications installées, découvertes dans `/Applications`,
`~/Applications` et `/System/Applications` : icône, nom, et un segmented control à trois positions
(`Bloquer` / `Laisser` / `Ignorer`). Toutes les applications sont sur `Bloquer` à l'ouverture,
conformément au statut par défaut (§3.1) ; l'écran sert à épargner. Un champ de recherche et deux
actions rapides, « Tout bloquer » et « Tout laisser ». **Cet écran est passable** — « Je verrai plus
tard » conserve le défaut, donc tout est bloqué.

**Écran 6 — C'est parti.** Résumé en trois lignes, mention « Breeze vit dans ta barre de menus, en
haut à droite » avec une flèche animée pointant vers l'icône réellement posée, case « Lancer Breeze
au démarrage » cochée par défaut, bouton « Démarrer mon premier cycle ».

Fermer la fenêtre avant l'écran 6 conserve ce qui a été choisi jusque-là, applique les valeurs par
défaut de §3.1 pour le reste, et démarre le premier cycle : Breeze ne reste jamais inerte parce que
l'utilisateur a fermé une fenêtre.

À la fermeture, la fenêtre se réduit visuellement vers l'icône de la barre de menus. L'onboarding
n'est jamais rejoué automatiquement ; il reste accessible depuis Réglages › Aide › « Revoir
l'introduction ».

### 3.4 Interaction depuis la barre de menus

**L'icône.** Un gabarit monochrome (*template image*) qui suit automatiquement le thème clair ou
sombre et le réglage « Réduire la transparence ». Quatre états :

| État | Icône |
|---|---|
| Inactif, armé ou suspendu | Contour de feuille, ajouré |
| Travail | Feuille pleine, anneau de progression qui se remplit |
| Préavis | Anneau complet, pulsation lente à 1 Hz |
| Pause en attente, inhibée | Anneau complet, sans pulsation |
| Pause en cours | Feuille pleine inversée |

Un badge d'avertissement se superpose à l'icône quand une permission requise manque (§3.9) ou quand
le coupe-circuit a désactivé les overlays (§4.7). Le popover dit lequel des deux, et propose la
réparation.

**Le titre à côté de l'icône.** Réglable en trois modes : `Rien`, `Minutes restantes` (`23`),
`Compte à rebours complet` (`23:04`). Défaut : `Minutes restantes`. Le texte est rendu en police
monospace tabulaire pour que la barre de menus ne sautille pas à chaque tick. La cadence de
rafraîchissement suit le mode choisi : chaque seconde pour `Compte à rebours complet`, chaque minute
pour `Minutes restantes`, aucune pour `Rien`. C'est la seule chose qui tourne quand le popover est
fermé (§4.8).

**Clic gauche → popover**, largeur 320, hauteur adaptative, flèche ancrée sous l'icône :

```
┌────────────────────────────────────┐
│  ◍  23:04                          │   ← anneau + temps, gros
│     avant ta pause de 10 min       │
├────────────────────────────────────┤
│  ⏸  Suspendre           15 min ▾   │
│  ⏭  Faire une pause maintenant     │
│  🔁 Redémarrer le cycle            │   ← repart d'une phase de travail pleine
├────────────────────────────────────┤
│  Sévérité : Simple            ▾    │
├────────────────────────────────────┤
│  Aujourd'hui : 4 pauses · 38 min   │
├────────────────────────────────────┤
│  ⚙  Réglages…              ⌘,      │
│  ⏻  Quitter Breeze                 │
└────────────────────────────────────┘
```

Pendant une pause active, le sélecteur de sévérité est présent mais désactivé, avec l'explication
« La sévérité ne change pas pendant une pause » (§3.7). Les trois actions du deuxième bloc suivent
le tableau de §3.7 : « Faire une pause maintenant » reste toujours disponible ; « Suspendre » et
« Redémarrer le cycle » sont désactivés pendant une pause, sauf en Mode Simple après 60 secondes.

Pendant une pause en Mode Simple, le popover reste ouvrable et affiche le décompte de la pause à la
place de celui du travail. Pendant une pause en Mode Hardcore, la barre de menus est masquée : le
popover est inatteignable, et c'est voulu.

Le popover se ferme au clic extérieur, à Échap, et au changement d'espace de travail. Il est
navigable entièrement au clavier et lisible par VoiceOver, chaque contrôle portant un label
explicite.

**Clic droit → menu natif court** : Suspendre 1 h · Pause maintenant · Réglages · Quitter. Pour
l'utilisateur pressé, et pour le cas où le renderer du popover n'a pas encore chargé. Ces entrées
obéissent aux mêmes règles de négociation que celles du popover.

**Raccourcis globaux**, configurables :

- `⌥⌘B` — pause immédiate. Actif par défaut.
- `⌥⌘S` — suspendre 15 min. Inactif par défaut.
- Aucun raccourci ne peut annuler une pause en cours. C'est délibéré.

### 3.5 Configuration des applications

Fenêtre Réglages, onglet **Applications**. Trois zones.

**Zone 1 — Déclencheurs.** « Ne compter le temps de travail que si j'utilise… » avec un interrupteur
maître, inactif par défaut. Actif, il révèle la liste des applications à cocher.

**Zone 2 — Comportement en pause.** Le tableau principal, une ligne par application :

```
🔍 [ Rechercher une application…              ]     [ Tout bloquer ] [ Tout laisser ]

  Icône  Nom                    Comportement en pause
  ────────────────────────────────────────────────────────────
  🅵     Figma                  [ Bloquer │ Laisser │ Ignorer ]
  💬     Slack                  [ Bloquer │ Laisser │ Ignorer ]
  🎵     Spotify                [ Bloquer │ Laisser │ Ignorer ]
  🔒     Trousseau d'accès      Toujours autorisée  ⓘ
```

Le tableau est trié : les applications lancées d'abord, puis celles déjà vues par Breeze, puis le
reste par ordre alphabétique. Une application inconnue rejoint la liste, au statut `blocked`, la
première fois qu'elle passe au premier plan. Un bandeau rappelle que ces statuts n'ont d'effet
visuel qu'en Mode Simple (§3.1).

**Zone 3 — Exceptions automatiques.** Les inhibitions de §3.2 :

- « Ne pas interrompre pendant un appel visio » — activé par défaut
- « Ne pas interrompre pendant un partage d'écran » — activé par défaut
- « Ne pas interrompre pendant une présentation plein écran » — activé par défaut
- « Ne pas interrompre entre 12 h et 14 h », plage libre — désactivé par défaut

Un bandeau permanent, dans les deux sévérités : « Ces exceptions repoussent la pause de 15 minutes
au maximum, report manuel compris. Ensuite, Breeze passe outre. »

### 3.6 Comportement des overlays

#### Préavis — T-60 s, dans les deux sévérités

Une bannière de 380 × 96, coin supérieur droit sous la barre de menus, `alwaysOnTop` au niveau
`status`, sans vol de focus, coins arrondis de 14 px, matériau `hud`. Elle est posée sur l'écran qui
porte la barre de menus, jamais dupliquée sur les autres.

> **Pause dans 60 secondes.** Termine ta phrase.
> `[ Reporter 5 min ]`  `[ Commencer maintenant ]`

Elle disparaît d'elle-même à T-0. Le bouton « Reporter 5 min » est présent tant que le budget de
report du cycle le permet, et absent sinon (§3.7). Le préavis est rejoué à chaque report, 60
secondes avant la nouvelle échéance.

Sans permission Notifications, le préavis passe par cette même bannière : Breeze ne dépend pas des
notifications système pour prévenir.

#### Mode Simple — un overlay par fenêtre bloquée

Pour **chaque fenêtre visible de chaque application `blocked`** — fenêtres secondaires, palettes et
inspecteurs compris — Breeze pose une fenêtre sans cadre, `transparent: true`, **opaque aux clics**,
positionnée exactement sur le cadre de la fenêtre cible et maintenue au-dessus d'elle. Une seule
d'entre elles, la plus grande, porte le décompte ; les autres ne portent que le voile. Niveau
`floating`, sauf pour une cible en plein écran natif où le niveau passe à `screen-saver` avec
`visibleOnFullScreen: true` (§4.4).

Rendu : voile sombre à 55 % d'opacité et flou d'arrière-plan léger — le contenu reste devinable sans
être lisible — avec au centre :

```
        ◍  04:12

     Pause en cours
   Figma est en pause

  ↓ Lève-toi, regarde au loin ↓
```

Comportement :

- Le reste du Mac reste utilisable. Les applications `allowed` et `ignored` fonctionnent normalement.
- Si l'application cible est masquée, minimisée ou fermée, l'overlay la suit ou disparaît avec elle.
- Une application `blocked` lancée pendant la pause reçoit son overlay en moins de 500 ms.
- Un clic sur l'overlay ne le ferme pas ; il déclenche une micro-animation de rebond qui rappelle le
  temps restant.
- Le son du système n'est pas coupé. Une musique en cours continue.
- **Aucune application bloquée n'a de fenêtre visible au début de la pause** : Breeze pose alors la
  bannière de pause seule, au même emplacement que le préavis, avec le décompte. Une pause sans
  aucun signal visible serait indistinguable d'une pause manquée.
- **Terminer en avance** : après 60 secondes de pause, un bouton « Terminer la pause » apparaît sous
  le décompte — sur l'overlay qui le porte, ou sur la bannière quand aucun overlay n'est posé. Il
  termine la pause et enchaîne sur [RETOUR]. Ce bouton n'existe qu'en Mode Simple.

#### Mode Hardcore — overlay total

Une fenêtre par écran physique, couvrant l'intégralité du `bounds` de son écran, au niveau
`screen-saver`, `visibleOnAllWorkspaces` avec `visibleOnFullScreen: true`, sans ombre, sans cadre,
non déplaçable, non fermable. Chaque écran affiche le même contenu : détourner le regard vers un
autre écran ne doit pas faire disparaître le décompte.

Rendu : fond opaque, dégradé sombre animé très lentement, avec au centre :

```
              ◍
            04:12

         Debout, Breeze

    Regarde par la fenêtre. Étire tes épaules.
    On se retrouve dans quatre minutes.


         Maintiens Échap pour sortir
```

Comportement :

- La barre de menus et le Dock sont masqués pendant toute la durée (§4.3).
- `⌘Tab`, `⌘Q`, `⌘W`, `⌘H`, `⌘M` et le changement d'espace sont neutralisés dans la limite de ce que
  macOS autorise. Le tableau de §4.3 dit exactement ce qui reste possible.
- Le curseur est masqué après 3 secondes d'immobilité.
- Breeze n'émet aucune notification pendant la pause. Il ne prétend pas faire taire celles des autres
  applications (§4.5).
- Aucun bouton de fermeture, aucune autre sortie que celle décrite ci-dessous.
- **Sortie d'urgence** — maintenir `Échap` pendant 10 secondes consécutives remplit un anneau de
  progression, puis ouvre un dialogue : « Sortir de la pause ? Ce cycle sera compté comme
  interrompu. » Le bouton d'annulation est en position par défaut. C'est un garde-fou de sécurité,
  pas un raccourci de confort : la friction de 10 secondes est calibrée pour être insupportable
  quand on veut tricher, et acceptable quand on a réellement besoin de son Mac. L'indice
  « Maintiens Échap pour sortir » est affiché en permanence, en petit et en bas de l'écran ; cacher
  une sortie de secours la transforme en piège. Elle n'est pas rationnée : un garde-fou qu'on épuise
  n'en est plus un. Chaque usage est enregistré dans le journal local et compté dans les
  statistiques (§3.8), et c'est la seule conséquence.
- **Après une sortie d'urgence**, Breeze détruit tous les overlays et passe en [ARMÉ]. La pause est
  comptée comme interrompue. Le cycle suivant repart avec un budget de report plein.

#### Retour — T+0, 3 s

L'overlay se dissout en 400 ms. Une bannière de 3 secondes : « Bien joué. Prochaine pause dans
50 minutes. » Puis Breeze passe en [ARMÉ].

### 3.7 Règles de négociation

Le produit tient parce que les échappatoires sont chiffrées et non modifiables au moment où on en a
envie.

**Le budget de report.** Chaque cycle dispose d'un budget unique de **15 minutes** de report, remis à
plein au début de chaque phase de travail. Tout ce qui repousse une pause y puise, au premier arrivé :
un report manuel depuis le préavis, une inhibition (§3.5). Budget épuisé, la pause démarre, quelle
que soit la raison invoquée. Un seul plafond, donc aucun cumul possible entre mécanismes.

| Levier | Mode Simple | Mode Hardcore |
|---|---|---|
| Reporter depuis le préavis | 3 fois, 5 min chacune | 1 fois, 5 min |
| Repousser par inhibition | Jusqu'à épuisement du budget | Jusqu'à épuisement du budget |
| Terminer la pause en avance | Bouton visible après 60 s | Impossible, hors sortie d'urgence |
| Déclencher une pause immédiatement | Immédiat | Immédiat |
| Redémarrer le cycle | Immédiat hors pause ; pendant une pause, seulement après 60 s | Immédiat hors pause, impossible pendant |
| Changer de sévérité | Immédiat hors pause, impossible pendant | Immédiat hors pause, impossible pendant |
| Changer la durée de travail ou de pause | Au cycle ou à la pause suivante | Au cycle ou à la pause suivante |
| Suspendre Breeze | Immédiat hors pause ; pendant une pause, seulement après 60 s | Immédiat hors pause, impossible pendant |
| Quitter l'application | Immédiat | Confirmation, avec rappel du compteur de pauses interrompues |

**Règle d'or** : aucun réglage qui affaiblit la contrainte ne prend effet pendant une pause en
cours. La sévérité, elle, ne change dans aucun sens pendant une pause : une pause se déroule toujours
sous la sévérité avec laquelle elle a commencé. Changer de sévérité hors pause est immédiat et
s'applique au cycle en cours.

### 3.8 Réglages — arborescence

- **Général** — lancement au démarrage, affichage du titre dans la barre de menus, langue, thème,
  vérification des mises à jour. Les valeurs par défaut sont en §3.1.
- **Rythme** — durée de travail, durée de pause, plage horaire active, jours actifs, presets. Les
  bornes de saisie sont celles de §3.3. La plage horaire suit l'heure locale du système ; un
  changement d'heure ne coupe jamais une pause en cours.
- **Sévérité** — Simple ou Hardcore, et le message affiché pendant la pause, libre dans la limite de
  120 caractères sur une ligne. Les quotas de report ne sont pas réglables : ce sont des constantes
  du produit (§3.7).
- **Applications** — §3.5.
- **Statistiques** — 30 derniers jours glissants, en local. Quatre compteurs et un cumul, définis
  une fois pour toutes ici :
  - **Pause prise** — allée jusqu'au bout de son décompte, ou validée par une absence (§4.6).
  - **Pause écourtée** — terminée par le bouton « Terminer la pause » du Mode Simple.
  - **Pause interrompue** — levée par la sortie d'urgence du Mode Hardcore.
  - **Report** — chaque report accordé depuis le préavis. Les minutes prises par une inhibition ne
    comptent pas comme un report : l'utilisateur ne les a pas demandées.
  - **Minutes de pause** — le temps réellement passé en pause, quelle qu'ait été sa fin.

  La ligne « Aujourd'hui » du popover (§3.4) affiche les pauses prises et les minutes de pause
  cumulées, elles seules.
- **Permissions** — état en direct, boutons de réparation, et le bouton qui réarme les overlays
  quand le coupe-circuit les a désactivés (§4.7).
- **Aide** — revoir l'introduction, exporter le journal de diagnostic, et « Désinstaller
  proprement » : Breeze supprime son dossier de données et son élément de démarrage, se quitte, et
  laisse affichée la marche à suivre pour retirer son entrée d'Accessibilité, que lui seul ne peut
  pas retirer.

### 3.9 Fonctionnement selon les permissions accordées

C'est la seule table qui dit ce que Breeze fait quand une permission manque. Aucune autre section ne
décrit de mode dégradé.

| Permission manquante | Ce qui continue de fonctionner | Ce qui change |
|---|---|---|
| **Notifications** seule | Tout | Le préavis passe par la bannière propriétaire de Breeze (§3.6). Aucune autre différence, dans les deux sévérités. |
| **Accessibilité** seule, sévérité Hardcore | Tout | Aucune différence. Le Mode Hardcore n'utilise pas l'API Accessibilité (§4.2). |
| **Accessibilité** seule, sévérité Simple | Le cycle, le préavis, le décompte | Breeze ne peut pas suivre les cadres de fenêtres. Il pose un **overlay unique plein écran, fermable d'un clic**, et affiche dans le popover : « Sans l'Accessibilité, ta pause est contournable. Autorise Breeze, ou passe en Hardcore. » Un badge d'avertissement reste sur l'icône jusqu'à la réparation. |
| **Les deux** | Le cycle en Hardcore | Cumul des deux lignes ci-dessus. |

Le déclencheur intelligent (F4) repose sur `NSWorkspace` et ne demande aucune permission. Si la
phase 0 démontre le contraire sur une version cible, le repli est décidé : sur cette version, le
déclencheur intelligent est désactivé et grisé dans les réglages, et tout usage du Mac compte comme
du travail (§9).

### 3.10 Ce qui se passe quand — cas limites tranchés

| Situation | Comportement |
|---|---|
| Un écran est branché pendant une pause Hardcore | Un overlay est posé dessus en moins de 500 ms, sur `display-added` |
| Un écran est débranché pendant une pause | L'overlay correspondant est détruit, la pause continue |
| La session est verrouillée pendant une pause | La pause est gelée ; au déverrouillage, l'overlay est reposé au premier plan et le décompte reprend |
| Le Mac dort pendant une pause | §4.6 : une absence plus longue que le temps de pause restant valide la pause |
| L'utilisateur est inactif plus de 3 min pendant le travail | La phase de travail est gelée (§4.5) |
| L'utilisateur est absent, pendant le travail, plus longtemps que la durée de pause configurée | §4.6 : le cycle est validé comme pause prise et repart à zéro |
| Une application `blocked` est lancée pendant une pause Simple | Elle reçoit son overlay en moins de 500 ms |
| Une application `allowed` recouvre la zone d'un overlay Simple | L'overlay qui l'intersecte est masqué tant qu'elle est au premier plan, puis reposé (§4.4) |
| Breeze est quitté pendant une pause Hardcore | Confirmation, puis tous les overlays sont détruits et les raccourcis globaux dés-enregistrés (§4.3) |
| Le processus principal tombe pendant une pause | Les fenêtres d'overlay meurent avec lui. Au-delà de 3 chutes en 5 minutes, le coupe-circuit s'arme (§4.7) |
| Deux instances de Breeze sont lancées | La seconde se termine immédiatement (§4.6) |
| Breeze est relancé alors qu'une phase était en cours | §4.6 : l'échéance persistée décide — encore à venir, la phase reprend ; déjà passée, elle est traitée comme une absence |
| Une application connue de Breeze est désinstallée | Sa ligne disparaît du tableau. Son statut est conservé, et réappliqué si l'application revient |
| L'heure système change pendant un cycle | Les échéances en cours sont conservées telles quelles. Seule la plage horaire suit la nouvelle heure locale |
| L'utilisateur enchaîne les sorties d'urgence | Aucun rationnement, aucune escalade dans le MVP. Le compteur monte, et c'est tout (§3.6) |
| Une mise à jour est disponible pendant un cycle | Elle est téléchargée, jamais installée avant le prochain lancement (§4.9) |

---

## 4. Défis techniques — Electron sur macOS

C'est la partie du brief qui coûte le plus cher si elle est ignorée. Chaque point donne la
contrainte, l'approche retenue, et le risque résiduel.

### 4.1 Ce qu'Electron ne sait pas faire seul

Electron n'expose ni l'application au premier plan, ni les cadres de fenêtres tierces, ni le masquage
du Dock et de la barre de menus. **Un module natif est obligatoire dès le MVP.**

**Décision** : un unique addon N-API en Objective-C++ (`breeze-native`), compilé pour `arm64` et
`x64`, exposant une surface minimale :

| Fonction | Rôle | Framework macOS |
|---|---|---|
| `getFrontmostApp()` | Bundle id et nom de l'application active | `NSWorkspace` |
| `onFrontmostAppChanged(cb)` | Notification de changement | `NSWorkspaceDidActivateApplicationNotification` |
| `getRunningApps()` | Applications lancées avec fenêtres | `NSWorkspace.runningApplications` |
| `getWindowFramesFor(pid)` | Cadres des fenêtres d'un processus | `AXUIElement` |
| `observeWindowGeometry(pid, cb)` | Déplacement et redimensionnement | `AXObserver` |
| `hasAccessibilityPermission()` | État de la permission | `AXIsProcessTrusted` |
| `requestAccessibilityPermission()` | Ouvre le prompt système | `AXIsProcessTrustedWithOptions` |
| `setWindowLevel(handle, level)` | Niveau Z au-dessus du Dock | `NSWindow.level` |
| `setPresentationOptions(mask)` | Masquer la barre de menus et le Dock | `NSApplication.presentationOptions` |
| `isScreenBeingShared()` | Détection de partage d'écran | À trancher en phase 0 (§9) |
| `getDisplays()` | Écrans et changements à chaud | `NSScreen` et ses notifications |

Toute logique produit reste en JavaScript dans le processus principal. Le natif ne fait que traduire
des appels système.

**Alternative rejetée** : les dépendances tierces `active-win` et `node-mac-permissions`. Elles
couvrent une partie du besoin, ajoutent deux surfaces de maintenance, et ne donnent ni les niveaux de
fenêtre ni les cadres AX. Un addon maison d'environ 600 lignes est plus sûr sur la durée.

### 4.2 Permissions macOS

| Permission | Nécessaire pour | Clé Info.plist | Quand la demander |
|---|---|---|---|
| **Accessibilité** | Lire les cadres de fenêtres, donc le Mode Simple (F5) uniquement | *(aucune — TCC la gère)* | Onboarding, écran 4 |
| **Notifications** | Confort du préavis | *(demande à l'exécution)* | Onboarding, écran 4 |
| **Enregistrement d'écran** | **Jamais demandée** — Breeze ne capture rien. La demander serait un signal de défiance majeur | — | Jamais |
| **Apple Events** | **Jamais demandée** — l'approche AppleScript est rejetée | — | Jamais |

Ni le Mode Hardcore, ni la détection de l'application au premier plan, ni `globalShortcut`
n'utilisent l'API Accessibilité. Cette affirmation est vérifiée en phase 0 sur chaque version cible,
avec le repli déjà décidé en §3.9 et §9.

**Piège critique n° 1 — l'Accessibilité et la signature.** L'autorisation TCC est liée à la
**signature de code** du binaire, pas à son chemin. Chaque build non signé crée une identité
différente : l'utilisateur doit ré-autoriser à chaque itération, et les entrées mortes s'accumulent
dans le volet Réglages Système. Conséquences opérationnelles :

- Signer avec un certificat Developer ID **dès le premier jour de développement**, y compris en
  local. Non négociable pour l'équipe.
- En développement, l'entrée TCC porte le nom d'Electron, pas de Breeze. Documenter ce point dans le
  README pour éviter les faux bugs.
- Toute modification du bundle après signature invalide l'autorisation. Le pipeline signe *après*
  l'inclusion des addons natifs, en profondeur : chaque `.node`, chaque framework, puis le bundle.
  L'option `--deep` est dépréciée et ne remplace pas cette signature élément par élément.

**Piège critique n° 2 — la révocation silencieuse.** L'utilisateur peut retirer l'Accessibilité
pendant que Breeze tourne. `AXIsProcessTrusted()` est donc revérifié au lancement, au début de chaque
cycle, et à chaque réveil. En cas de perte : appliquer la ligne correspondante de §3.9, poser le
badge d'avertissement sur l'icône, et proposer la réparation en un clic dans le popover.

**Piège critique n° 3 — la propagation à chaud.** L'octroi de l'Accessibilité a historiquement exigé
un relancement de l'application. Les versions récentes propagent le changement à chaud, sans
garantie. L'onboarding interroge `AXIsProcessTrusted()` à 1 Hz ; si la phase 0 montre que la
propagation n'est pas fiable sur une version cible, Breeze y propose une relance explicite
(`app.relaunch()` puis `app.exit(0)`) au lieu d'attendre.

### 4.3 Le Mode Hardcore — jusqu'où on peut aller

macOS protège délibérément l'utilisateur contre les applications qui prennent l'écran en otage.

**Ce qui fonctionne de façon fiable :**

```js
const overlay = new BrowserWindow({
  frame: false, transparent: false, hasShadow: false,
  resizable: false, movable: false, minimizable: false,
  closable: false, fullscreenable: false,
  skipTaskbar: true, focusable: true,
  webPreferences: { contextIsolation: true, nodeIntegration: false, sandbox: true }
})
overlay.setBounds(display.bounds)
overlay.setAlwaysOnTop(true, 'screen-saver', 1)
overlay.setVisibleOnAllWorkspaces(true, { visibleOnFullScreen: true })
overlay.setFullScreenable(false)
overlay.setIgnoreMouseEvents(false)
```

- Le niveau `screen-saver` (`CGShieldingWindowLevel`) passe au-dessus du Dock, de la barre de menus
  et des applications en plein écran natif.
- `presentationOptions` combinant `NSApplicationPresentationHideDock`,
  `NSApplicationPresentationHideMenuBar` et `NSApplicationPresentationDisableProcessSwitching` masque
  le Dock et la barre de menus, et désactive `⌘Tab`. C'est l'API publique des applications de
  kiosque.
- `globalShortcut.register` sur `⌘Q`, `⌘W`, `⌘H` et `⌘M` pendant la pause, dés-enregistrés à la fin.
  Ces enregistrements sont globaux : les oublier après une chute du processus pénalise tout le
  système jusqu'au redémarrage de Breeze. Un handler `will-quit` **et** un `process.on('exit')`
  dés-enregistrent systématiquement.
- Multi-écran : une fenêtre par `screen.getAllDisplays()`, plus des écouteurs sur `display-added`,
  `display-removed` et `display-metrics-changed`. Un écran non couvert est une porte de sortie
  complète.

**Ce qui ne fonctionne pas, ou pas de façon garantie :**

| Contournement | Bloquable ? | Traitement |
|---|---|---|
| `⌘Tab` | Oui, via `DisableProcessSwitching` | Bloqué |
| Clic sur le Dock | Oui, Dock masqué et niveau shielding | Bloqué |
| Mission Control (`⌃↑`), Exposé | Partiellement — le geste trackpad reste capté par le WindowServer | L'overlay revient au-dessus ; l'animation n'est pas évitable |
| Changement d'espace (`⌃→`) | Oui, `visibleOnAllWorkspaces` suit l'utilisateur | Bloqué en pratique |
| `⌘⌥Échap`, Forcer à quitter | Non — niveau système, non interceptable | Assumé et documenté |
| Verrouillage puis déverrouillage | Partiellement — l'overlay doit être re-poussé sur `unlock-screen` | Handler explicite requis |
| Basculement d'utilisateur rapide | Non | Assumé |
| `killall Breeze` depuis SSH ou un autre compte | Non | Assumé |
| Débrancher l'écran, éteindre le Mac | Non, et **ne doit pas** l'être | Assumé |

**Position produit à tenir** : Breeze n'est ni un logiciel de contrôle parental, ni un MDM. Il rend
la triche coûteuse et consciente, pas impossible. La documentation utilisateur et la page marketing
le disent franchement. Promettre l'infaillibilité génère des remboursements et des avis à une étoile.

**Ne pas utiliser `kiosk: true` seul** : sur macOS il déclenche le plein écran natif, ce qui crée un
espace dédié, joue une animation de 500 ms, et laisse `⌃←` ramener l'utilisateur au bureau. La
combinaison `bounds` + `screen-saver` + `presentationOptions` est strictement supérieure.

### 4.4 Le Mode Simple — poser un overlay sur une fenêtre tierce

macOS n'a **aucune API publique pour placer une fenêtre au-dessus d'une fenêtre précise d'une autre
application**. L'ordre Z inter-processus appartient au WindowServer et n'est pas adressable.

| Approche | Principe | Verdict |
|---|---|---|
| **A. Suivi par cadre AX** | Lire `AXPosition` et `AXSize` via l'API Accessibilité, poser une fenêtre Electron `alwaysOnTop` au niveau `floating` sur ces coordonnées, resynchroniser au mouvement | **Retenue.** Publique, signable, notarisable |
| **B. Masquer l'application** | `NSRunningApplication.hide()` sur les cibles | Rejetée — perte de contexte, rendus en cours cassés, l'utilisateur récupère l'application d'un clic |
| **C. Suspendre le processus** | `SIGSTOP` sur le pid cible | Rejetée fermement — corruption de données, sockets rompus, refus de notarisation probable. Un `SIGSTOP` sur une application de visio ou une base de données est une catastrophe utilisateur |

**Limites honnêtes de l'approche A :**

- La position est suivie par `AXObserver` sur `kAXWindowMovedNotification` et
  `kAXWindowResizedNotification`, pas par polling. Un polling de secours à 4 Hz couvre les
  applications qui n'émettent pas ces notifications.
- **L'overlay est au-dessus de tout, pas seulement de sa cible.** Si l'utilisateur amène une
  application `allowed` par-dessus la zone couverte, l'overlay la masque aussi. Mitigation : quand
  l'application au premier plan est `allowed` ou `ignored`, Breeze masque les overlays qui
  l'intersectent et les repose au retour. Cette gymnastique est la vraie complexité du Mode Simple —
  la budgéter, pas la découvrir.
- Une application en plein écran natif occupe son propre espace : un overlay `floating` ne la couvre
  pas. L'overlay concerné bascule alors au niveau `screen-saver` avec `visibleOnFullScreen: true`.
- Sans permission Accessibilité, l'approche A est impossible. Le comportement de repli est en §3.9,
  et lui seul fait autorité.

**Alerte de charge** : le Mode Simple est plus difficile à réaliser proprement que le Mode Hardcore.
Si le planning se tend, c'est le Mode Hardcore qu'il faut livrer en premier, malgré l'intuition
inverse. Le découpage de §6 applique déjà cet ordre.

### 4.5 Détection d'usage, déclencheur et inhibitions

- **Application au premier plan** : `NSWorkspaceDidActivateApplicationNotification`, événementiel,
  coût nul. Aucun polling.
- **Inactivité utilisateur** : `powerMonitor.getSystemIdleTime()`. Au-delà de 3 minutes
  d'inactivité, la phase de travail est gelée — rappeler une pause à quelqu'un déjà parti n'a pas de
  sens. Les conséquences d'une absence longue sont en §3.10 et §4.6.
- **Détection de visio** : Breeze ne lit ni le micro ni la caméra — la permission serait indéfendable
  pour une application qui promet de ne rien capter. La détection repose sur le bundle id de
  l'application au premier plan, croisé avec une liste connue (`us.zoom.xos`,
  `com.microsoft.teams2`, `com.cisco.webexmeetingsapp`, entre autres), que l'utilisateur peut
  compléter. **Une visio dans un onglet de navigateur n'est pas détectable**, puisque cela
  demanderait de lire le titre de fenêtre, ce que le refus n° 2 de §2 interdit. Breeze l'assume, et
  l'interface le dit à l'endroit du réglage.
- **Détection de partage d'écran** : approche à trancher en phase 0 (§9). Repli déjà décidé : à
  défaut de signal fiable, l'inhibition « partage d'écran » est retirée de l'interface, et la
  présence d'une application de visio au premier plan couvre la majorité des cas réels.
- **Modes Concentration** : Breeze n'en lit pas l'état et n'en active pas. Lire l'état des modes
  Concentration n'a pas d'API publique, et l'activation programmatique d'un Focus par une
  application tierce non plus. Breeze se contente donc de **ne pas émettre de notification pendant
  une pause**. Aucune promesse n'est faite sur les notifications des autres applications, ni dans
  l'interface, ni dans le marketing.

### 4.6 Cycle de vie, veille, persistance

- **Les timers JavaScript sont faux.** `setInterval` dérive et ne compte pas pendant la veille. La
  source de vérité est une **échéance de phase**, mémorisée sous deux formes : une valeur d'horloge
  monotone (`performance.now()`) pour le décompte pendant que le processus vit, et un timestamp
  d'horloge murale (`Date.now()`) pour la persistance et le recalcul après veille. Le tick d'une
  seconde ne sert qu'à rafraîchir l'affichage ; tout calcul de temps restant part d'une différence
  d'échéances, jamais d'un décrément.
- **`powerMonitor`** : sur `suspend` et `lock-screen`, geler la phase et mémoriser le reste. Sur
  `resume` et `unlock-screen`, recalculer depuis l'horloge murale. Deux règles produit, selon la
  phase interrompue :
  - Absence pendant **[TRAVAIL]** — veille, verrouillage ou inactivité — plus longue que la durée de
    pause configurée : le cycle est validé comme pause prise et repart à zéro.
  - Absence pendant **[PAUSE ACTIVE]** plus longue que le temps de pause restant : la pause est
    validée, et Breeze enchaîne sur [RETOUR] au réveil.
- **Persistance** : `electron-store`, en JSON dans `~/Library/Application Support/Breeze/`, écriture
  atomique, champ `schemaVersion` et migrations. L'état du cycle est écrit à chaque transition
  d'état, jamais à chaque tick.
- **Reprise au lancement** : Breeze relit l'état persisté et compare l'échéance de phase à l'heure
  murale. Échéance à venir, la phase reprend là où elle s'était arrêtée. Échéance dépassée, le temps
  écoulé est traité comme une absence, avec les deux règles ci-dessus. Un overlay n'est jamais posé
  au lancement : une pause dont l'échéance est passée est close, pas rejouée. Sans état persisté —
  première ouverture, ou arrêt volontaire — Breeze démarre en **[ARMÉ]** si la plage horaire
  l'autorise, et en **[INACTIF]** sinon.
- **Instance unique** : `app.requestSingleInstanceLock()`, obligatoire. Deux instances posant chacune
  un overlay Hardcore est le pire bug possible du produit.
- **Pas d'icône dans le Dock** : `LSUIElement: true` dans l'Info.plist et `app.dock.hide()` au
  démarrage. Conséquence : les fenêtres d'onboarding et de réglages ne prennent pas le focus
  automatiquement. Breeze appelle `app.dock.show()` avant d'ouvrir une de ces fenêtres, et
  `app.dock.hide()` à sa fermeture.
- **Lancement au démarrage** :
  `app.setLoginItemSettings({ openAtLogin: true, openAsHidden: true })`.

### 4.7 Sécurité et garde-fous

- **Liste blanche système, non modifiable** — ces applications ne reçoivent jamais d'overlay en Mode
  Simple : `com.apple.systempreferences`, `com.apple.ActivityMonitor`, `com.apple.Terminal`,
  `com.apple.finder`, `com.apple.keychainaccess`, `com.apple.loginwindow`, les applications
  d'accessibilité de macOS (VoiceOver, Contrôle vocal, Switch Control, Loupe), et toute application
  déclarant `public.app-category.medical` ou `public.app-category.healthcare-fitness` dans son
  Info.plist. La liste est en dur, non éditable depuis l'interface. En
  Mode Hardcore, elle ne s'applique pas : l'overlay couvre l'écran entier (§3.1). La contrepartie
  d'accessibilité est portée par §5 — l'overlay Hardcore est lisible par VoiceOver et annonce le
  temps restant.
- **Coupe-circuit** — Breeze compte les chutes du processus principal dans un fichier persistant.
  Au-delà de 3 chutes en 5 minutes, il démarre le cycle suivant sans aucun overlay, affiche un
  avertissement dans le popover, et n'y renonce qu'après 24 heures sans nouvelle chute, ou quand
  l'utilisateur réarme les overlays depuis Réglages › Permissions (§3.8). Un bug qui pose un overlay
  Hardcore irretirable est un incident critique ; la sortie doit être prévue avant lui.
- **Sécurité Electron** : `contextIsolation: true`, `nodeIntegration: false`, `sandbox: true` sur
  tous les renderers, préchargement minimal, IPC typé et validé côté main, `webSecurity` jamais
  désactivé. Les renderers d'overlay ne chargent aucune ressource distante.
- **Notarisation** : indispensable. Une application non notarisée qui demande l'Accessibilité et
  prend l'écran entier a le profil exact d'un logiciel malveillant ; Gatekeeper la bloque, et les
  utilisateurs ont raison de s'en méfier.
- **Confidentialité** : la seule sortie réseau du MVP est la vérification de mise à jour, que
  l'utilisateur peut couper depuis Réglages › Général (§2). L'application le dit en clair dans son
  interface, pas seulement dans une politique de confidentialité.

### 4.8 Performance

Une application de barre de menus est jugée sur ce qu'elle ne coûte pas.

| Métrique | Cible |
|---|---|
| Mémoire au repos | Moins de 120 Mo, processus principal et renderer de popover compris |
| CPU au repos | Moins de 0,3 % en moyenne sur 10 minutes |
| Renderers vivants au repos | 1, le popover. Les overlays sont créés à la demande |
| Latence de pose de l'overlay Hardcore | Moins de 200 ms après T-0 |
| Latence de pose d'un overlay Simple sur une application lancée en cours de pause | Moins de 500 ms |
| Impact batterie | Absent du top 5 de « Consommation d'énergie importante » |

Moyens : détruire les fenêtres d'overlay après usage plutôt que les masquer, ramener le
rafraîchissement à la cadence du titre de barre de menus quand le popover est fermé (§3.4), ne pas
animer ce titre, remplacer tout polling par une notification système quand elle existe.

### 4.9 Distribution

- **Hors Mac App Store.** Le bac à sable de l'App Store interdit l'usage de l'API Accessibilité pour
  ce type de besoin et les niveaux de fenêtre de kiosque. La distribution est directe, en Developer
  ID signé et notarisé, avec mise à jour par `electron-updater` sur un dépôt de publication.
- **Politique de mise à jour** : vérification au lancement puis toutes les 24 heures, jamais pendant
  une pause ni un préavis. Le téléchargement se fait en arrière-plan ; l'installation attend le
  prochain lancement de Breeze. Une mise à jour ne quitte jamais l'application d'elle-même.
- Builds universels `arm64` et `x64`, DMG avec fond d'installation, et une désinstallation propre
  documentée. Le retrait des entrées TCC n'est pas automatisable : il est expliqué à l'utilisateur.

---

## 5. Exigences non fonctionnelles

| Domaine | Exigence |
|---|---|
| Versions macOS | 13, 14, 15 et 26 — matrice de test explicite. Il n'existe pas de macOS 16 à 25 : Apple est passé de 15 à 26 |
| Accessibilité | Overlays lisibles par VoiceOver, avec annonce du temps restant ; contraste AA minimum ; respect de « Réduire les animations » et « Réduire la transparence » ; navigation clavier complète hors overlay Hardcore, où seule la sortie d'urgence répond au clavier |
| Langues | Français et anglais au lancement, chaînes externalisées dès le premier écran. Langue choisie d'après celle du système, anglais à défaut |
| Thème | Clair, sombre, système. Icône de barre de menus en template image |
| Résilience | Aucune perte d'état sur chute du processus ; reprise de cycle correcte après une veille de plusieurs heures |
| Journalisation | Journal local circulaire, 7 jours et 5 Mo, exportable depuis Réglages › Aide. Il contient les transitions d'état, les bundle identifiers vus, les permissions perdues et les erreurs. Jamais de titre de fenêtre, jamais de contenu applicatif |

---

## 6. Découpage de livraison

Ce découpage fixe un ordre et une sortie vérifiable par phase, pas des durées : une estimation se
pose à l'entrée d'une phase, quand son inconnue principale est levée, jamais avant.

| Phase | Contenu | Sortie vérifiable |
|---|---|---|
| **0 — Reconnaissance** | Addon natif minimal et mesure des cinq inconnues de §9 | Un rapport tranchant chaque inconnue, un prototype qui pose un overlay inévitable pendant 30 s, et un build soumis à la notarisation |
| **1 — Squelette** | Barre de menus, machine à états de §3.2, persistance, échéances de phase, réglages Rythme | Un cycle complet tourne, sans overlay |
| **2 — Hardcore** | Overlay total multi-écran, neutralisation des sorties, sortie d'urgence, coupe-circuit | Une pause Hardcore de 5 min tient face à une tentative active de contournement, hors sorties déclarées impossibles en §4.3 |
| **3 — Applications** | Scan, statuts, déclencheur intelligent, inhibitions, budget de report | Le décompte n'avance qu'avec une application déclencheuse au premier plan, et rien ne repousse une pause au-delà de 15 minutes |
| **4 — Simple** | Overlays par fenêtre, suivi AX, gestion de l'intersection avec les applications épargnées | Figma est figée, Notes reste utilisable, et déplacer Figma déplace l'overlay |
| **5 — Finition** | Onboarding complet, statistiques, accessibilité, internationalisation, signature, notarisation, mise à jour | Build notarisé installé sur une machine vierge, onboarding mené jusqu'au premier cycle sans intervention |
| **6 — v1.1** | KF1 et KF2 (§7) | — |
| **7 — v1.2** | KF3 (§7) | — |

---

## 7. Killer features — après le MVP

Trois fonctionnalités, choisies pour renforcer la promesse sans élargir le périmètre. Aucune n'entre
dans le MVP ; §2 en donne la version visée.

### KF1 — Bouclier de calendrier — v1.1

**Le problème réel.** Un overlay Hardcore qui tombe pendant une réunion client détruit la confiance
en une seule occurrence. Aucune heuristique d'application ne rattrape ça : Zoom peut être en
arrière-plan, la réunion peut se tenir au téléphone, la présentation peut passer par un second écran.

**La fonctionnalité.** Breeze lit le calendrier local via EventKit — **jamais les titres, jamais les
participants ; seulement les créneaux occupés et le statut de disponibilité**. Il en tire deux
comportements :

1. **Décalage** — une pause qui tomberait dans un créneau occupé est avancée ou repoussée pour
   s'insérer dans le premier trou disponible : jusqu'à 20 minutes en avance, et jusqu'à 15 minutes en
   retard, ce second plafond étant celui du budget de report. Breeze préfère décaler que sauter.
2. **Placement opportuniste** — Breeze repère les interstices naturels, comme le trou de 12 minutes
   entre deux réunions, et y place la pause plutôt que de l'imposer au milieu d'un bloc de travail.

Avancer une pause ne consomme rien. La repousser puise dans le budget de report du cycle (§3.7),
comme tout autre mécanisme qui retarde une pause. Le popover annonce le décalage à l'avance :
« Réunion à 15 h — ta pause est avancée à 14 h 40. »

**Pourquoi c'est décisif.** C'est ce qui fait passer Breeze de « logiciel qu'on désactive les jours
chargés » à « logiciel qu'on garde justement les jours chargés ». Le Mode Hardcore devient acceptable
en environnement professionnel, ce qui débloque le segment le plus disposé à payer.

**Coût technique.** Permission Calendriers (`NSCalendarsUsageDescription`), lecture EventKit dans
l'addon natif, logique de placement de créneau. Modéré. La discipline de ne lire que les créneaux
doit être visible dans le code et affirmée dans l'interface.

### KF2 — Dette de posture — v1.1

**Le problème réel.** Les applications de pause traitent chaque cycle comme indépendant. Une pause
ignorée disparaît sans conséquence, et le produit devient du bruit en deux semaines.

**La fonctionnalité.** Chaque report ou pause écourtée crédite une **dette**, en minutes, avec trois
effets :

1. **Visible en permanence** — un anneau secondaire autour de l'icône de barre de menus se remplit à
   mesure que la dette monte.
2. **Elle se rembourse** — la pause suivante est allongée à hauteur de la dette, plafonnée à +50 % de
   sa durée nominale, et sans jamais dépasser la borne de 60 minutes de §3.3. Reporter deux fois ne
   fait pas disparaître le besoin de bouger : ça le déplace.
3. **Elle durcit la contrainte** — au-delà d'un seuil configurable, 20 minutes par défaut, Breeze
   passe en Mode Hardcore pour le cycle suivant, avec un préavis explicite : « Tu as reporté trois
   fois. La prochaine pause sera ferme. » L'escalade est annoncée, jamais subie. Elle prend effet au
   cycle suivant, conformément à la règle d'or de §3.7.

La dette s'efface chaque nuit.

**Pourquoi c'est décisif.** C'est le mécanisme qui empêche la dérive vers l'inefficacité, première
cause d'abandon dans cette catégorie. Et c'est une contrainte que l'utilisateur a armée lui-même à
froid, ce qui la rend légitime.

**Coût technique.** Faible : de la logique d'état et un rendu d'icône. Le vrai travail est le réglage
des seuils, en test utilisateur.

### KF3 — Détection de session profonde — v1.2

**Le problème réel.** Interrompre quelqu'un à la minute 47 d'une concentration profonde est un
dommage net, même pour sa santé. Un rappel purement horaire ne distingue pas le travail profond du
parcours distrait de six onglets.

**La fonctionnalité.** Breeze calcule un **indice de profondeur** à partir de signaux qu'il possède
déjà, sans permission supplémentaire : nombre de changements d'application par minute, durée de
session continue sur une même application, régularité de l'activité d'entrée agrégée — un compte
d'événements, jamais leur contenu.

1. **Report intelligent** — en profondeur élevée, le préavis devient une fenêtre de tolérance de
   10 minutes : Breeze attend le premier changement d'application, qui signale la fin naturelle du
   bloc. Cette tolérance puise dans le budget de report du cycle (§3.7) et ne s'y ajoute pas.
2. **Rapport hebdomadaire** — une carte : « Tes blocs profonds durent 43 minutes en moyenne. Ton
   réglage est à 50. », avec un ajustement en un clic.

**Pourquoi c'est décisif.** Breeze passe du rayon santé au rayon productivité, où le consentement à
payer est plus élevé, sans rien retirer à la promesse santé. Et ça répond à l'objection n° 1 de la
cible : « ça va me couper en plein travail ».

**Coût technique.** Modéré. Les signaux existent déjà pour le déclencheur intelligent (§4.5) ; il
faut y ajouter une agrégation via `CGEventSource.secondsSinceLastEventType`, **sans event tap** — un
tap serait un dépassement de permission et un risque de notarisation. **Aucun apprentissage
automatique** : des moyennes glissantes et deux seuils suffisent, et restent explicables à
l'utilisateur.

---

## 8. Indicateurs de succès

Ces indicateurs ne sont pas remontés : le refus n° 1 de §2 l'interdit, et aucun consentement ne le
rouvre dans le MVP. Ils se mesurent en **bêta privée**, sur un panel recruté, par export manuel du
journal local que le participant envoie lui-même. Hors bêta, ils restent visibles du seul
utilisateur, dans Réglages › Statistiques.

| Indicateur | Cible à 30 jours | Ce qu'il signale s'il est manqué |
|---|---|---|
| Rétention | Plus de 40 % des installations ont un cycle actif à J+30 | Le produit n'a pas trouvé sa place |
| Pauses menées à leur terme | Plus de 70 % | La contrainte est mal calibrée ou mal acceptée |
| Adoption du Hardcore | Plus de 25 % des utilisateurs actifs pendant au moins une semaine | La prévisualisation de l'onboarding ne convainc pas |
| Sorties d'urgence | Moins de 1 par utilisateur et par semaine | Le seuil de friction est mal calibré |
| Désinstallation dans les 48 h | Moins de 15 % | Échec de l'onboarding, ou brutalité mal annoncée |

---

## 9. Risques et inconnues

| # | Risque | Impact | Traitement |
|---|---|---|---|
| R1 | La propagation à chaud de l'Accessibilité n'est pas fiable sur toutes les versions cibles | Onboarding cassé | Mesure en phase 0 ; repli par relance explicite (§4.2) |
| R2 | Le Mode Simple est instable sur les applications qui n'émettent pas les notifications AX | Fonctionnalité dégradée | Polling de secours à 4 Hz, liste des applications problématiques, repli sur l'overlay unique de §3.9 |
| R3 | La notarisation est refusée à cause des niveaux de fenêtre ou de l'usage AX | Distribution bloquée | Soumettre un build de phase 0 à la notarisation **avant** d'écrire le produit |
| R4 | Un bug pose un overlay Hardcore irretirable | Incident critique | Sortie d'urgence, coupe-circuit, montre de surveillance dans le processus principal (§4.7) |
| R5 | Le Mode Hardcore est perçu comme un logiciel malveillant | Confiance, désinstallations | Prévisualisation à l'onboarding, honnêteté sur les limites, notarisation, sortie réseau désactivable |
| R6 | Le partage d'écran n'est pas détectable sans permission d'enregistrement | Une inhibition en moins | Repli déjà décidé : retirer l'inhibition de l'interface (§4.5) |

**Les cinq inconnues de la phase 0.** Chacune se tranche par mesure, jamais par supposition. Le
comportement de repli est déjà décidé pour chacune : aucune ne peut bloquer le design ni le
développement.

| # | Inconnue | Repli si la mesure est défavorable |
|---|---|---|
| 1 | Propagation à chaud de l'Accessibilité, par version de macOS | Relance explicite proposée dans l'onboarding sur les versions concernées (§4.2) |
| 2 | Détection du partage d'écran sans permission d'enregistrement | L'inhibition « partage d'écran » est retirée de l'interface (§4.5) |
| 3 | Comportement de `presentationOptions` face à une application tierce déjà en plein écran natif | L'overlay Hardcore reste posé au niveau `screen-saver` ; le masquage de la barre de menus est abandonné sur ce cas, sans autre conséquence |
| 4 | Verdict de notarisation sur un binaire portant les niveaux de fenêtre de kiosque | Aucun repli technique n'existe. Un refus arrête le produit : c'est pourquoi la soumission a lieu en phase 0, avant toute écriture de fonctionnalité |
| 5 | `NSWorkspace` demande-t-il l'Accessibilité sur une version cible ? | Sur cette version, le déclencheur intelligent est désactivé et grisé, et tout usage compte comme du travail (§3.9) |

---

## 10. Glossaire

| Terme | Définition |
|---|---|
| **Cycle** | Une phase de travail suivie d'une phase de pause |
| **Sévérité** | Simple ou Hardcore — le niveau de contrainte de l'overlay |
| **Application bloquée** (`blocked`) | Recouverte par un overlay en Mode Simple. Statut par défaut |
| **Application épargnée** (`allowed`) | Utilisable pendant une pause en Mode Simple |
| **Application ignorée** (`ignored`) | Utilisable pendant une pause, et son usage ne fait pas avancer le décompte de travail |
| **Déclencheur** | Application dont la présence au premier plan fait avancer le décompte de travail |
| **Préavis** | Bannière de 60 secondes annonçant la pause |
| **Inhibition** | Condition externe qui repousse une pause, dans la limite du budget de report |
| **Budget de report** | 15 minutes par cycle, partagées entre reports manuels et inhibitions |
| **Sortie d'urgence** | Maintien d'Échap pendant 10 secondes pour lever un overlay Hardcore |
| **Coupe-circuit** | Désactivation automatique des overlays après trois chutes du processus en cinq minutes |
| **Dette** | Minutes de pause reportées, à rembourser — KF2, hors MVP |

---

## 11. Journal des arbitrages — de la version 1.0 à la version 2.0

Cette version ne change pas l'ambition du produit. Elle ferme les points que la version 1.0 laissait
ouverts, contradictoires, ou décrits à deux endroits différents.

| # | Ce qui posait problème en 1.0 | Ce qui est décidé en 2.0 |
|---|---|---|
| 1 | La durée de pause par défaut valait 5 min en §3.1 et 10 min partout ailleurs | 50 min de travail, 10 min de pause, partout |
| 2 | Le statut par défaut d'une application inconnue dépendait de la sévérité et du déclencheur, par une règle à deux conditions | `blocked` toujours. L'onboarding sert à épargner |
| 3 | Le rôle des statuts en Mode Hardcore n'était pas dit | En Hardcore, le statut ne joue plus que sur le décompte du travail. L'overlay couvre tout |
| 4 | Statut `ignored` et liste des déclencheurs se recouvraient sans règle de combinaison | §3.1 donne la combinaison exacte des deux mécanismes |
| 5 | La sortie d'urgence était « maintenir Échap 10 s » dans le texte et « ⌫ ⌫ ⌫ » dans la maquette | Maintien d'Échap 10 s, indiqué en clair sur l'overlay |
| 6 | Le tableau de négociation annonçait un bouton « terminer en avance » en Mode Simple que la description de l'overlay ignorait | Le bouton apparaît après 60 s au centre de chaque overlay Simple (§3.6) |
| 7 | Le report manuel et l'inhibition avaient chacun leur plafond, sans règle de cumul | Un budget unique de 15 minutes par cycle, partagé par tous les mécanismes qui repoussent une pause |
| 8 | Les profils multiples étaient annoncés sans aucun écran pour les gérer | Un seul jeu de réglages dans le MVP. Les profils passent en v1.1 |
| 9 | L'Accessibilité était déclarée requise pour F4, F5 et F6 | Requise pour F5 seulement. §3.9 donne le fonctionnement pour chaque permission manquante, et §9 le repli si la phase 0 contredit ce point |
| 10 | Le mode dégradé était décrit différemment à l'onboarding et dans la section Mode Simple | Une seule table, §3.9, fait autorité |
| 11 | Breeze promettait de faire taire les notifications système via un mode Concentration, tout en jugeant l'approche fragile | Breeze n'émet pas de notification pendant une pause, et ne promet rien sur celles des autres applications |
| 12 | La section sur la détection d'usage contenait un raisonnement contradictoire sur Google Meet | La visio en onglet de navigateur est déclarée non détectable, avec sa raison |
| 13 | L'échéance de phase était décrite comme un « timestamp d'époque monotone », ce qui n'existe pas | Horloge monotone pour le décompte, horloge murale pour la persistance et la reprise après veille |
| 14 | Les indicateurs de succès prévoyaient une remontée consentie, que le refus n° 1 interdit | Aucune remontée. Mesure en bêta privée par export manuel du journal |
| 15 | Le refus « aucune donnée hors de la machine » cohabitait avec une vérification de mise à jour non déclarée | L'exception est déclarée, décrite, et désactivable dans les réglages |
| 16 | La machine à états ignorait la plage horaire, l'état ARMÉ quand le déclencheur est désactivé, et le sort d'une phase de travail interrompue | §3.2 traite les trois cas |
| 17 | Le sort d'une pause en cours à la fin de la plage horaire n'était pas défini | Une pause commencée va toujours à son terme |
| 18 | Les cas limites (écran branché en pause, verrouillage, seconde instance, chute du processus) étaient dispersés ou absents | §3.10 les rassemble et les tranche |
| 19 | La liste blanche système promettait « jamais d'overlay, quel que soit le mode », ce que le Mode Hardcore contredit | La liste vaut pour le Mode Simple ; l'accessibilité en Hardcore est portée par la lisibilité VoiceOver de l'overlay |
| 20 | Le coupe-circuit ne disait pas comment il se désarme | Réactivation explicite par l'utilisateur, ou 24 heures sans nouvelle chute |
| 21 | Le modèle économique n'était nulle part, alors que les killer features et les indicateurs de succès s'y référaient | MVP gratuit, sans licence. Toute monétisation rouvre la contrainte réseau, en v1.1 au plus tôt |
| 22 | La version de livraison des killer features était implicite | §2 donne la version visée de chacune |
| 23 | Les décisions ouvertes de phase 0 n'avaient pas de comportement de repli | Chaque inconnue de §9 en a un, sauf la notarisation, dont le refus arrête le produit — et c'est dit |
| 24 | Le saut de macOS 15 à macOS 26 pouvait passer pour une erreur de saisie | §5 le signale explicitement |
| 25 | Le tableau de négociation ignorait « Redémarrer le cycle » et « Faire une pause maintenant », que le popover propose | Les deux ont leur ligne en §3.7, et le popover renvoie à ce tableau |
| 26 | Rien ne disait ce que voit l'utilisateur si aucune application bloquée n'a de fenêtre au début d'une pause Simple | Breeze pose la bannière de pause seule (§3.6) |
| 27 | Les quotas de report étaient réglables selon §3.8 et « non modifiables » selon §3.7 | Ce sont des constantes du produit. §3.8 le dit |
| 28 | Une seule règle de veille couvrait le travail et la pause, deux situations différentes | §4.6 donne les deux règles séparément, et §3.10 y renvoie |
| 29 | « Geler le tick d'affichage quand le popover est fermé » contredisait le compte à rebours affiché dans la barre de menus | §3.4 fixe la cadence par mode d'affichage, et §4.8 s'y aligne |
| 30 | KF1 annonçait un décalage de ±20 minutes, au-delà du budget de report de 15 minutes | 20 minutes en avance, 15 en retard. Avancer une pause ne consomme pas de budget |
| 31 | F4 était décrit comme conditionnant le démarrage du cycle, alors qu'il conditionne l'avancée du décompte | §2 et §6 disent l'avancée du décompte |
| 32 | La liste blanche renvoyait à une « catégorie médicale ou d'accessibilité » sans la nommer | §4.7 nomme les catégories Info.plist et les applications concernées |
| 33 | Le découpage de livraison s'arrêtait à la v1.1 alors que KF3 vise la v1.2 | Une phase 7 porte KF3 |
| 34 | Les valeurs par défaut étaient éparpillées, et la plage horaire comme les jours actifs n'en avaient aucune | §3.1 porte une table qui fait autorité sur tous les défauts du produit |
| 35 | Rien ne disait ce qu'un changement de durée fait à une phase en cours | La durée de travail s'applique au cycle suivant, la durée de pause à la pause suivante (§3.1, §3.7) |
| 36 | Le nombre d'usages de la sortie d'urgence et ce qui suit une sortie n'étaient pas définis | Aucun rationnement — un garde-fou qu'on épuise n'en est plus un. Après une sortie, Breeze passe en [ARMÉ] et le cycle suivant a un budget plein (§3.6) |
| 37 | Les compteurs de statistiques étaient nommés sans être définis | §3.8 définit pause prise, écourtée, interrompue et report, et dit ce qu'affiche la ligne « Aujourd'hui » |
| 38 | Le coupe-circuit se réarmait « explicitement », sans dire où | Bouton dans Réglages › Permissions, ou 24 heures sans chute (§4.7) |
| 39 | Le comportement au relancement, phase en cours, n'était pas décidé | §4.6 : l'échéance persistée décide, et aucun overlay n'est posé au lancement |
| 40 | La mise à jour n'avait ni cadence ni règle d'installation | Vérification au lancement puis toutes les 24 h, jamais pendant une pause ; installation au prochain lancement (§4.9) |
| 41 | Le journal disait ce qu'il ne contient pas, jamais ce qu'il contient | §5 énumère les deux |
| 42 | « Un overlay par application » ou « par fenêtre » restait ambigu en Mode Simple | Un overlay par fenêtre visible ; la plus grande porte le décompte (§3.6) |
| 43 | Sur plusieurs écrans, ni le préavis ni le contenu de l'overlay Hardcore n'avaient de place définie | Préavis sur l'écran de la barre de menus ; contenu répété à l'identique sur chaque écran (§3.6) |
| 44 | Le popover n'avait pas de comportement défini pendant une pause | Ouvrable et à jour en Mode Simple, inatteignable en Hardcore puisque la barre de menus est masquée (§3.4) |
| 45 | La fin d'une suspension ne disait pas dans quel état Breeze revient | [ARMÉ] si la plage horaire l'autorise, [INACTIF] sinon, avec une phase de travail pleine (§3.2) |
| 46 | L'état [INHIBÉ] n'avait aucune traduction visible | Anneau complet sans pulsation, et « Pause en attente » avec le budget restant dans le popover (§3.2) |
| 47 | Fermer l'onboarding avant la fin laissait Breeze dans un état indéterminé | Les choix faits sont conservés, le reste prend les défauts, et le premier cycle démarre (§3.3) |
| 48 | « Désinstaller proprement » ne disait pas ce qu'il supprime | Dossier de données et élément de démarrage, puis la marche à suivre pour l'entrée TCC, que Breeze ne peut pas retirer (§3.8) |
| 49 | Le changement d'heure système face à la plage horaire n'était pas traité | Les échéances en cours ne bougent pas ; seule la plage horaire suit la nouvelle heure locale (§3.8, §3.10) |
| 50 | Le sort du statut d'une application désinstallée était indéterminé | Sa ligne disparaît, son statut est conservé et réappliqué si elle revient (§3.10) |
| 51 | L'état de départ à froid, sans état persisté, n'était pas dit | [ARMÉ] si la plage horaire l'autorise, [INACTIF] sinon (§4.6) |
| 52 | L'absence de durées dans le découpage pouvait se lire comme un oubli | §6 dit que les estimations se posent à l'entrée de chaque phase, et pourquoi |
