# Breeze — Brief produit

**Version** : 1.0 · **Date** : 2026-08-27 · **Statut** : définitif, prêt pour design et développement
**Plateforme** : macOS 13 Ventura et ultérieur (Apple Silicon + Intel) · **Stack** : Electron (main + renderer) + modules natifs Objective-C/Swift

---

## 1. Vision

Breeze est une application de barre de menus macOS qui force la pause. Là où les concurrents envoient une notification qu'on balaie sans y penser, Breeze intervient sur le poste de travail : il fige les applications qui retiennent l'utilisateur, ou verrouille l'écran entier, le temps d'un compte à rebours qu'on ne peut pas écourter d'un clic.

**Promesse** : « Tu ne peux pas ignorer Breeze. C'est le but. »

**Utilisateur cible** : travailleur du savoir sur Mac, 6 à 10 heures d'écran par jour, qui a déjà essayé des rappels passifs et les a désactivés au bout d'une semaine.

**Problème résolu** : les rappels de pause échouent parce qu'ils sont négociables. Breeze rend la pause non négociable, à un niveau de contrainte que l'utilisateur choisit lui-même à froid, avant d'être en pleine session de travail.

---

## 2. Périmètre

### Dans le MVP

| # | Fonctionnalité | Description |
|---|---|---|
| F1 | Rappel périodique | Compte à rebours de travail configurable, suivi d'une pause de durée configurable |
| F2 | Personnalisation | Durée de travail, durée de pause, plage horaire active, jours actifs |
| F3 | Ciblage d'applications | Liste d'applications à figer pendant la pause, liste d'applications toujours épargnées |
| F4 | Déclencheur intelligent | Le cycle ne démarre que si une application « déclencheuse » est au premier plan |
| F5 | Mode Simple | Overlay par fenêtre au-dessus des seules applications ciblées |
| F6 | Mode Hardcore | Overlay plein écran sur tous les écrans, toutes les entrées capturées, sortie impossible |
| F7 | Barre de menus | Icône d'état, compte à rebours, menu de contrôle, accès aux réglages |
| F8 | Onboarding | Parcours de première ouverture, demande des permissions, configuration minimale |

### Hors MVP (documenté pour ne pas être redécouvert)

- Synchronisation entre appareils, compte utilisateur, backend.
- Statistiques historiques au-delà de 30 jours locaux.
- Version Windows ou Linux.
- Intégration calendrier (déplacé en v1.1 — voir §7 Killer Feature 1).
- Exercices guidés vidéo, contenu audio.
- Mode équipe, classement social.

### Refus explicites

- **Breeze ne collecte aucune donnée hors de la machine.** Aucun télémétrie, aucun compte. Ce n'est pas une option, c'est une contrainte d'architecture.
- **Breeze n'enregistre jamais le contenu des applications.** On lit le nom et le bundle identifier de l'application au premier plan, jamais le titre de fenêtre, jamais l'écran.
- **Le Mode Hardcore n'empêche pas l'arrêt du Mac.** Couper l'alimentation, forcer l'extinction, ou tuer le processus depuis un autre compte reste possible. On assume et on le dit à l'utilisateur.

---

## 3. Analyse fonctionnelle et parcours utilisateur

### 3.1 Modèle conceptuel

Trois objets suffisent à décrire tout le produit.

**Le Cycle** — l'unité de temps de Breeze. Un cycle est composé d'une *phase de travail* (par défaut 50 min) et d'une *phase de pause* (par défaut 5 min). Un cycle se répète tant que Breeze est actif.

**Le Profil** — un jeu de réglages nommé : durée travail, durée pause, sévérité, applications ciblées, applications déclencheuses, plage horaire. L'utilisateur en a un par défaut (« Travail »). Le MVP autorise plusieurs profils mais un seul est actif à la fois.

**La Politique d'application** — chaque application connue du système reçoit un statut parmi trois :

| Statut | Effet en pause | Effet sur le déclenchement |
|---|---|---|
| `blocked` | Figée par un overlay | Compte comme activité de travail |
| `allowed` | Laissée libre | Compte comme activité de travail |
| `ignored` | Laissée libre | Ne compte pas comme activité de travail |

Par défaut toute application inconnue est `blocked` si la sévérité est Simple et que le déclencheur intelligent est désactivé ; sinon `allowed`. Une liste système d'applications critiques est `allowed` en dur et non modifiable (§4.7).

### 3.2 Machine à états

```
                   ┌──────────────────────────────────────────────┐
                   ▼                                              │
  [INACTIF] ──démarrer──▶ [ARMÉ] ──déclencheur détecté──▶ [TRAVAIL]
      ▲                     │ ▲                              │  │
      │                     │ └──plus d'app déclencheuse──────┘  │
      │              déclencheur absent                   fin du décompte
      │                     │                                    ▼
      └────arrêter──────────┴──────────────────────────── [PRÉAVIS] (60 s)
                                                                 │
                                          ┌──reporter (snooze)───┤
                                          │                      ▼
                                    [TRAVAIL]              [PAUSE ACTIVE]
                                                                 │
                                                          fin du décompte
                                                                 │
                                                                 ▼
                                                           [RETOUR] (3 s)
                                                                 │
                                                                 ▼
                                                            [TRAVAIL]
```

États complémentaires, transversaux :

- **[SUSPENDU]** — l'utilisateur a mis Breeze en pause pour 15 min / 1 h / jusqu'à demain. Le décompte est gelé.
- **[VEILLE]** — le Mac s'est endormi ou l'écran s'est verrouillé. Le décompte est gelé et repris au réveil (§4.6).
- **[INHIBÉ]** — une condition externe empêche la pause : partage d'écran actif, appel visio, application en plein écran exclusif déclarée exempte. Le décompte continue mais la pause est repoussée jusqu'à la levée de l'inhibition, avec un plafond de report de 15 min au-delà duquel on force.

### 3.3 Onboarding — première ouverture

Objectif : l'utilisateur a un cycle qui tourne en moins de 90 secondes, et il a compris ce que Breeze va faire à son écran **avant** que ça arrive.

Fenêtre d'onboarding classique (pas un popover) : 720 × 520, non redimensionnable, centrée, sans barre de menus applicative visible.

**Écran 1 — Bienvenue.** Une phrase, l'icône, un bouton « Commencer ». Aucun champ.

**Écran 2 — Ton rythme.** Trois presets en grandes cartes cliquables, plus un lien « Personnaliser » :
- *Pomodoro* — 25 min travail / 5 min pause
- *Classique* — 50 min travail / 10 min pause **(pré-sélectionné)**
- *Longue haleine* — 90 min travail / 15 min pause

Le champ personnalisé accepte 5–180 min de travail et 1–60 min de pause.

**Écran 3 — Quel niveau de fermeté.** Deux cartes, avec une **animation de prévisualisation** dans chaque carte (une maquette de bureau miniature qui montre l'overlay se poser). C'est le point de décision produit le plus important : l'utilisateur doit voir Hardcore avant de le choisir.
- *Simple* — « On grise les applications qui te retiennent. Le reste du Mac fonctionne. » **(pré-sélectionné)**
- *Hardcore* — « On prend l'écran entier. Tu ne peux rien faire jusqu'à la fin. » + mention en petit : « Tu peux toujours éteindre ton Mac. On ne bloque pas ça. »

**Écran 4 — Permissions.** Une carte par permission, avec l'état en direct (`⚠️ requise` → `✅ accordée`) et un bouton « Autoriser » qui ouvre le volet Réglages Système correspondant. La fenêtre repolle l'état toutes les secondes et coche automatiquement quand la permission est accordée, sans redémarrage visible (§4.2).
- **Accessibilité** — « pour savoir quelle application est devant toi, et pour poser l'écran de pause ». Requise pour F4, F5, F6.
- **Notifications** — « pour te prévenir une minute avant la pause ». Optionnelle, dégradation propre si refusée (le préavis passe alors par une bannière Breeze propriétaire).
- Le bouton « Continuer » reste actif même sans permissions : Breeze fonctionne en mode dégradé (rappels par notification seule) et affiche un bandeau persistant dans le menu tant que l'Accessibilité manque.

**Écran 5 — Applications.** Liste des applications installées (scan de `/Applications`, `~/Applications`, `/System/Applications`), avec icône, nom, et un segmented control à trois positions (`Bloquer` / `Laisser` / `Ignorer`). Un champ de recherche. Un bouton « Sélectionner mes 5 apps les plus utilisées » qui pré-coche depuis les applications actuellement lancées. **Cet écran est passable** — « Je verrai plus tard » applique le défaut.

**Écran 6 — C'est parti.** Résumé en trois lignes, mention « Breeze vit dans ta barre de menus, en haut à droite » avec une flèche animée pointant vers l'icône réellement posée, case « Lancer Breeze au démarrage » (cochée par défaut), bouton « Démarrer mon premier cycle ».

À la fermeture, la fenêtre se réduit visuellement vers l'icône de la barre de menus. L'onboarding n'est jamais rejoué, mais est réaccessible depuis Réglages › Aide › « Revoir l'introduction ».

### 3.4 Interaction depuis la barre de menus

**L'icône.** Un gabarit monochrome (template image) qui respecte automatiquement le thème clair/sombre et le mode Réduire la transparence. Quatre états visuels :

| État | Icône |
|---|---|
| Inactif / suspendu | Contour de feuille, ajouré |
| Travail | Feuille pleine + anneau de progression qui se remplit |
| Préavis (dernière minute) | Anneau complet, pulsation lente 1 Hz |
| Pause en cours | Feuille pleine inversée |

**Le titre à côté de l'icône.** Optionnel, réglable en trois modes : `Rien` / `Minutes restantes` (`23`) / `Compte à rebours complet` (`23:04`). Défaut : `Minutes restantes`. Le texte est rendu en police monospace tabulaire pour ne pas faire sautiller la barre de menus à chaque tick.

**Clic gauche → popover** (largeur 320, hauteur adaptative, flèche ancrée sous l'icône) :

```
┌────────────────────────────────────┐
│  ◍  23:04                          │   ← anneau + temps, gros
│     avant ta pause de 10 min       │
├────────────────────────────────────┤
│  ⏸  Suspendre           15 min ▾   │
│  ⏭  Faire une pause maintenant     │
│  🔁 Redémarrer le cycle            │
├────────────────────────────────────┤
│  Profil : Travail             ▾    │
│  Sévérité : Simple            ▾    │
├────────────────────────────────────┤
│  Aujourd'hui : 4 pauses · 38 min   │
├────────────────────────────────────┤
│  ⚙  Réglages…              ⌘,      │
│  ⏻  Quitter Breeze                 │
└────────────────────────────────────┘
```

Le popover se ferme au clic extérieur, à Échap, et au changement d'espace de travail. Il est navigable entièrement au clavier (Tab / flèches / Entrée) et lisible par VoiceOver, chaque contrôle portant un label explicite.

**Clic droit → menu natif court** : Suspendre 1 h · Pause maintenant · Réglages · Quitter. Pour l'utilisateur pressé et pour le cas où le renderer du popover n'a pas encore chargé.

**Raccourcis globaux** (configurables, désactivés par défaut sauf le premier) :
- `⌥⌘B` — pause immédiate
- `⌥⌘S` — suspendre 15 min
- Aucun raccourci ne peut annuler une pause en cours. C'est délibéré.

### 3.5 Configuration des applications

Fenêtre Réglages, onglet **Applications**. Trois zones.

**Zone 1 — Déclencheurs.** « Ne compter le temps de travail que si j'utilise… » avec un interrupteur maître. Actif, il révèle une liste d'applications à cocher. Inactif (défaut), tout usage du Mac compte comme du travail.

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

Les applications sont triées : d'abord celles actuellement lancées, puis celles déjà vues par Breeze, puis le reste par ordre alphabétique. Une application inconnue apparaît automatiquement dans la liste la première fois qu'elle passe au premier plan.

**Zone 3 — Exceptions automatiques.** Interrupteurs pour les inhibitions (§3.2) :
- « Ne pas interrompre pendant un appel visio » (défaut : activé)
- « Ne pas interrompre pendant un partage d'écran » (défaut : activé)
- « Ne pas interrompre pendant une présentation plein écran » (défaut : activé)
- « Ne pas interrompre entre 12 h et 14 h » — plage libre (défaut : désactivé)

En Mode Hardcore, un bandeau prévient : « Ces exceptions repoussent la pause de 15 minutes au maximum. Ensuite, Breeze passe outre. »

### 3.6 Comportement des overlays

#### Préavis (T-60 s, avant toute sévérité)

Une bannière discrète, 380 × 96, coin supérieur droit sous la barre de menus, `alwaysOnTop` niveau `status`, sans focus volé, sans bordure, coins arrondis 14 px, matériau `hud` vibrancy.

> **Pause dans 60 secondes.** Termine ta phrase.
> `[ Reporter 5 min ]`  `[ Commencer maintenant ]`

Elle disparaît d'elle-même à T-0. Le report est limité (§3.7). En Mode Hardcore, le bouton « Reporter » est présent au premier report, puis absent aux suivants si le quota est épuisé.

#### Mode Simple — overlay par application

Pour chaque application `blocked` ayant au moins une fenêtre visible, Breeze pose une fenêtre sans cadre, transparente aux couleurs mais **opaque aux clics**, positionnée exactement sur le cadre de l'application cible, et maintenue au-dessus d'elle dans l'ordre Z.

Rendu : voile sombre à 55 % d'opacité + flou d'arrière-plan léger (le contenu reste devine sans être lisible), et au centre :

```
        ◍  04:12

     Pause en cours
   Figma est en pause

  ↓ Lève-toi, regarde au loin ↓
```

Comportement :
- Le reste du Mac reste utilisable. L'utilisateur peut ouvrir Notes, Musique, son navigateur si ceux-ci ne sont pas `blocked`.
- Si l'application cible est masquée, minimisée ou fermée, l'overlay la suit ou disparaît avec elle.
- Si l'utilisateur lance une nouvelle application `blocked` pendant la pause, un overlay est posé dessus en moins de 500 ms.
- Un clic sur l'overlay ne fait rien, mais déclenche une micro-animation de « rebond » qui rappelle le temps restant. Pas de fermeture accidentelle possible.
- Le son du système n'est pas coupé. Une musique en cours continue.

#### Mode Hardcore — overlay total

Une fenêtre par écran physique, chacune en `kiosk` + `alwaysOnTop` niveau `screen-saver`, couvrant l'intégralité du `bounds` de son écran, `visibleOnAllWorkspaces` avec `visibleOnFullScreen: true`, sans ombre, sans cadre, non déplaçable, non fermable.

Rendu : fond opaque, dégradé sombre animé très lentement (pas de mouvement agressif), et au centre :

```
              ◍
            04:12

         Debout, Breeze

    Regarde par la fenêtre. Étire tes épaules.
    On se retrouve dans quatre minutes.


                                  ⌫ ⌫ ⌫  (indice de sortie d'urgence, très discret)
```

Comportement :
- La barre de menus et le Dock sont masqués pendant toute la durée (§4.3).
- Mission Control, le changement d'espace, `⌘Tab`, `⌘Q`, `⌘W`, `⌃↑` sont neutralisés dans la mesure de ce que macOS autorise (§4.3 pour les limites exactes).
- Le curseur est masqué après 3 secondes d'immobilité.
- Les notifications système sont supprimées pendant la pause via l'activation du mode Concentration « Breeze » (§4.5).
- Aucun bouton de fermeture. Aucune combinaison documentée dans l'interface pour sortir, hormis la sortie d'urgence.
- **Sortie d'urgence** — maintenir `Échap` pendant 10 secondes consécutives fait apparaître un anneau de progression puis un dialogue : « Sortir de la pause ? Ce cycle sera compté comme interrompu. » Deux boutons, celui d'annulation en position par défaut. Cette sortie est **loguée localement** et affichée dans les statistiques hebdomadaires (« 2 pauses interrompues cette semaine »). C'est un garde-fou de sécurité, pas un raccourci de confort — la friction de 10 secondes est calibrée pour être insupportable quand on veut juste tricher, et acceptable quand on a réellement besoin de son Mac.

#### Retour (T+0, 3 s)

L'overlay se dissout en 400 ms. Une bannière de 3 secondes : « Bien joué. Prochaine pause dans 50 minutes. » Puis le cycle repart.

### 3.7 Règles de négociation

Le produit tient parce que les échappatoires sont chiffrées et non modifiables au moment où on en a envie.

| Levier | Mode Simple | Mode Hardcore |
|---|---|---|
| Reporter le préavis | 3 fois max, 5 min chacun | 1 fois, 5 min |
| Terminer la pause en avance | Après 60 s, bouton visible | Impossible, hors sortie d'urgence |
| Changer de sévérité | Immédiat | Prend effet au **prochain** cycle, jamais pendant |
| Suspendre Breeze | Immédiat | Immédiat, mais impossible pendant une pause active |
| Quitter l'application | Immédiat | Confirmation + rappel du compteur d'abandons |

**Règle d'or** : aucun réglage qui affaiblit la contrainte ne prend effet pendant une pause en cours. On négocie à froid, jamais à chaud.

### 3.8 Réglages — arborescence

- **Général** — profil actif, lancement au démarrage, affichage du titre dans la barre de menus, langue, thème.
- **Rythme** — durée travail, durée pause, plage horaire active, jours actifs, presets.
- **Sévérité** — Simple / Hardcore, quotas de report, message affiché pendant la pause (personnalisable).
- **Applications** — §3.5.
- **Statistiques** — 30 derniers jours, locaux.
- **Permissions** — état en direct, boutons de réparation.
- **Aide** — revoir l'introduction, journal de diagnostic, désinstaller proprement.

---

## 4. Défis techniques — Electron sur macOS

Cette section est la partie du brief qui coûte le plus cher si elle est ignorée. Chaque point liste la contrainte, l'approche retenue, et le risque résiduel.

### 4.1 Ce qu'Electron ne sait pas faire seul

Electron n'expose ni l'application au premier plan, ni le contrôle de fenêtres tierces, ni les niveaux de fenêtre au-dessus du Dock, ni le masquage du Dock, ni les modes Concentration. **Un module natif est obligatoire dès le MVP.**

**Décision** : un unique addon N-API en Objective-C++ (`breeze-native`), compilé pour `arm64` et `x64`, exposant une surface minimale :

| Fonction | Rôle | Framework macOS |
|---|---|---|
| `getFrontmostApp()` | Bundle id + nom de l'app active | `NSWorkspace` |
| `onFrontmostAppChanged(cb)` | Notification de changement | `NSWorkspaceDidActivateApplicationNotification` |
| `getRunningApps()` | Applications lancées avec fenêtres | `NSWorkspace.runningApplications` |
| `getWindowFramesFor(pid)` | Cadres des fenêtres d'un processus | `AXUIElement` (Accessibilité) |
| `hasAccessibilityPermission()` | État de la permission | `AXIsProcessTrusted` |
| `requestAccessibilityPermission()` | Ouvre le prompt système | `AXIsProcessTrustedWithOptions` |
| `setWindowLevel(handle, level)` | Niveau Z au-dessus du Dock | `NSWindow.level` |
| `setPresentationOptions(mask)` | Masquer barre de menus + Dock | `NSApplication.presentationOptions` |
| `pushFocusMode(id)` / `popFocusMode()` | Silence des notifications | `UNUserNotificationCenter` / `Intents` |
| `getDisplays()` | Écrans, changements à chaud | `NSScreen` + notifications |

Toute logique produit reste en JavaScript dans le processus principal. Le natif ne fait que traduire des appels système.

**Alternative rejetée** : `active-win` et `node-mac-permissions` en dépendances tierces. Elles couvrent 60 % du besoin, ajoutent deux surfaces de maintenance, et ne donnent pas les niveaux de fenêtre ni les cadres AX. Un addon maison de ~600 lignes est plus sûr sur la durée.

### 4.2 Permissions macOS

| Permission | Nécessaire pour | Clé Info.plist | Quand demander |
|---|---|---|---|
| **Accessibilité** (`kAXTrustedCheckOptionPrompt`) | Lire les cadres de fenêtres, poser les overlays de Mode Simple sur les bonnes coordonnées, fiabiliser la détection d'app active | *(pas de clé — TCC gère)* | Onboarding écran 4 |
| **Notifications** | Préavis à T-60 s | *(demande à l'exécution)* | Onboarding écran 4 |
| **Automation / Apple Events** | *(seulement si on choisit l'approche AppleScript, non retenue)* | `NSAppleEventsUsageDescription` | — |
| **Enregistrement d'écran** | **Non requis** — on ne capture rien. À ne surtout pas demander : ce serait un signal de défiance majeur. | — | Jamais |

**Piège critique n° 1 — l'Accessibilité et la signature.** L'autorisation TCC est liée à la **signature de code** du binaire, pas au chemin. Chaque build non signé recrée une identité différente : l'utilisateur doit ré-autoriser à chaque itération de développement, et les entrées mortes s'accumulent dans le volet Réglages. Conséquences opérationnelles :
- Signer avec un certificat Developer ID **dès le premier jour de développement**, même en local. Non négociable pour l'équipe.
- En développement, l'entrée TCC porte le nom d'Electron, pas de Breeze. Documenter ce point dans le README pour éviter les faux bugs.
- Toute modification du bundle après signature invalide l'autorisation. Le pipeline de build doit signer *après* l'inclusion des addons natifs, en profondeur (`--deep` est déprécié : signer chaque `.node`, chaque framework, puis le bundle).

**Piège critique n° 2 — la révocation silencieuse.** L'utilisateur peut retirer l'Accessibilité pendant que Breeze tourne. `AXIsProcessTrusted()` doit être re-vérifié à chaque début de cycle et à chaque réveil, pas seulement au lancement. En cas de perte : basculer en mode dégradé, poser un badge d'avertissement sur l'icône de barre de menus, et proposer la réparation en un clic dans le popover.

**Piège critique n° 3 — le redémarrage.** Historiquement, l'octroi de l'Accessibilité exigeait un relancement de l'application pour être pris en compte. Sur les versions récentes de macOS, le changement est propagé sans relancer, mais le comportement n'est pas garanti. **Vérification requise en début de développement** sur les versions cibles (13, 14, 15, 26) : implémenter un polling de `AXIsProcessTrusted()` à 1 Hz pendant l'onboarding, et prévoir un chemin de relance propre (`app.relaunch()` + `app.exit(0)`) si la propagation à chaud s'avère non fiable sur une version supportée.

### 4.3 Le Mode Hardcore — jusqu'où on peut aller

C'est le point technique le plus dur, et le plus mal compris. macOS protège délibérément l'utilisateur contre les applications qui prennent l'écran en otage.

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

- Le niveau `'screen-saver'` (`CGShieldingWindowLevel`) passe au-dessus du Dock, de la barre de menus, et des applications en plein écran natif.
- `presentationOptions` avec `NSApplicationPresentationHideDock | NSApplicationPresentationHideMenuBar | NSApplicationPresentationDisableProcessSwitching` masque le Dock et la barre de menus et **désactive `⌘Tab`**. C'est l'API que les applications de kiosque utilisent, elle est publique et supportée.
- `globalShortcut.register` sur `⌘Q`, `⌘W`, `⌘H`, `⌘M` pendant la pause, dés-enregistrés à la fin. Attention : ces enregistrements sont globaux ; les oublier après un crash pénalise tout le système jusqu'au redémarrage de Breeze. **Un handler `will-quit` et un `process.on('exit')` doivent systématiquement dés-enregistrer.**
- Multi-écran : une fenêtre par `screen.getAllDisplays()`, plus un écouteur sur `display-added` / `display-removed` / `display-metrics-changed` pour couvrir un écran branché **pendant** la pause. Un écran non couvert est une porte de sortie complète.

**Ce qui ne fonctionne pas, ou pas de façon garantie :**

| Contournement utilisateur | Bloquable ? | Traitement |
|---|---|---|
| `⌘Tab` | ✅ via `DisableProcessSwitching` | Bloqué |
| Clic sur le Dock | ✅ Dock masqué + niveau shielding | Bloqué |
| Mission Control (`⌃↑`), Exposé | ⚠️ Partiellement — le geste trackpad reste souvent capté par le WindowServer | L'overlay reste au-dessus au retour ; on ne peut pas empêcher l'animation |
| Changement d'espace (`⌃→`) | ⚠️ `visibleOnAllWorkspaces` suit l'utilisateur | Fonctionnel en pratique |
| `⌘⌥Échap` (Forcer à quitter) | ❌ **Impossible.** Niveau système, non interceptable | Assumé et documenté |
| Verrouillage puis déverrouillage (`⌃⌘Q`) | ⚠️ L'overlay doit être re-poussé au premier plan sur `unlock-screen` | Handler explicite requis |
| Basculement d'utilisateur rapide | ❌ Impossible | Assumé |
| Terminal depuis SSH, `killall Breeze` | ❌ Impossible | Assumé |
| Débrancher l'écran / éteindre le Mac | ❌ Impossible, et **ne doit pas** l'être | Assumé |

**Position produit à tenir** : Breeze n'est pas un logiciel de contrôle parental ni un MDM. Il rend la triche coûteuse et consciente, pas impossible. La documentation utilisateur et la page marketing doivent le dire, franchement. Promettre l'infaillibilité génère des remboursements et des avis à une étoile.

**Ne pas utiliser** `kiosk: true` seul : sur macOS il déclenche le plein écran natif, ce qui crée un espace dédié, joue une animation de 500 ms, et laisse `⌃←` ramener l'utilisateur au bureau. La combinaison `bounds` + `screen-saver` + `presentationOptions` est strictement supérieure pour cet usage.

### 4.4 Le Mode Simple — poser un overlay sur une fenêtre tierce

macOS n'a **aucune API publique pour placer une fenêtre au-dessus d'une fenêtre spécifique d'une autre application**. L'ordre Z inter-processus est géré par le WindowServer et n'est pas adressable.

Trois approches, évaluées :

| Approche | Principe | Verdict |
|---|---|---|
| **A. Suivi par cadre AX** | Lire `AXPosition` / `AXSize` via l'API Accessibilité, poser une fenêtre Electron `alwaysOnTop` niveau `floating` sur ces coordonnées, re-synchroniser sur mouvement | ✅ **Retenue.** Publique, signable, notarisable |
| **B. Minimiser / masquer l'app** | `NSRunningApplication.hide()` sur les apps ciblées | ❌ Rejetée — perte de contexte, casse les rendus en cours, l'utilisateur récupère l'app d'un clic |
| **C. Suspendre le processus** | `SIGSTOP` sur le pid cible | ❌ **Rejetée fermement** — corruption de données, sockets rompus, refus de notarisation probable, et un `SIGSTOP` sur une app de visio ou une base de données est une catastrophe utilisateur |

**Détail de l'approche A et ses limites honnêtes :**

- La position est suivie par observation AX (`AXObserver` sur `kAXWindowMovedNotification` et `kAXWindowResizedNotification`), pas par polling. Un polling de secours à 4 Hz couvre les applications qui n'émettent pas ces notifications (certaines applications Electron tierces, certains jeux).
- **L'overlay est au-dessus de tout, pas seulement de sa cible.** Conséquence : si l'utilisateur amène une application `allowed` par-dessus la zone couverte, l'overlay la masque aussi. Mitigation : quand l'application au premier plan est `allowed`, on masque temporairement les overlays des applications qui l'intersectent, et on les repose quand l'utilisateur revient. Cette gymnastique est la vraie complexité du Mode Simple — la budgéter, pas la découvrir.
- Les applications en plein écran natif occupent leur propre espace : un overlay niveau `floating` ne les couvre pas. Pour ces cas, basculer l'overlay concerné en niveau `screen-saver` avec `visibleOnFullScreen: true`.
- Sans permission Accessibilité, l'approche A est impossible. Dégradation : le Mode Simple pose alors **un seul overlay plein écran désactivable d'un clic** — visuellement contraignant, réellement contournable — et le popover affiche clairement pourquoi.

**Alerte de charge** : le Mode Simple est plus difficile à réaliser proprement que le Mode Hardcore. Si le planning se tend, c'est le Mode Hardcore qu'il faut livrer en premier, malgré l'intuition inverse.

### 4.5 Détection d'usage et déclencheur intelligent

- **Application au premier plan** : notification `NSWorkspaceDidActivateApplicationNotification`, événementiel, coût nul. Pas de polling.
- **Inactivité utilisateur** : `powerMonitor.getSystemIdleTime()` d'Electron. Au-delà de 3 minutes d'inactivité, la phase de travail est gelée — inutile de rappeler de faire une pause à quelqu'un qui est déjà parti. Au retour, si l'absence dépasse la durée de pause configurée, le cycle est validé comme pause prise.
- **Détection de visio** : ne pas essayer de lire le micro ou la caméra (permission requise, et ce serait indéfendable pour une app qui promet de ne rien capter). À la place, une heuristique sur le bundle id de l'application au premier plan croisée avec une liste connue (`us.zoom.xos`, `com.microsoft.teams2`, `com.google.Chrome` en mode Meet détecté par le titre — non, le titre est exclu, donc Chrome n'est pas détectable et on l'assume). L'utilisateur peut compléter la liste manuellement.
- **Détection de partage d'écran** : le natif expose `CGDisplayStream` actif / `CGSessionCopyCurrentDictionary`. Approche à valider techniquement ; à défaut, s'appuyer sur la présence au premier plan d'une application de visio, ce qui couvre la majorité des cas réels.
- **Modes Concentration** : lecture de l'état Focus depuis Electron n'est pas exposée. Le natif peut interroger `NSUserNotificationCenter` / le fichier de configuration Focus — approche fragile entre versions. **À trancher par recherche technique en phase 0**, avec repli : Breeze n'observe pas les modes Concentration mais en active un pendant les pauses.

### 4.6 Cycle de vie, veille, persistance

- **Les timers JavaScript sont faux.** `setInterval` dérive, et surtout ne compte pas pendant la veille du Mac. La source de vérité doit être un **timestamp de fin de phase** (`phaseEndsAt`, en millisecondes époque monotone corrigée), et le tick de 1 s ne sert qu'à rafraîchir l'affichage. Toute logique qui calcule le temps restant part de `phaseEndsAt - now()`, jamais d'un décrément.
- **`powerMonitor`** : sur `suspend` et `lock-screen`, geler la phase et mémoriser le reste. Sur `resume` et `unlock-screen`, recalculer. Règle produit : une veille supérieure à la durée de pause configurée compte comme une pause prise et redémarre le cycle à zéro.
- **Persistance** : `electron-store` (JSON dans `~/Library/Application Support/Breeze/`), écriture atomique, versionné avec un champ `schemaVersion` et des migrations. L'état du cycle est persisté à chaque transition, pas à chaque tick, pour survivre à un crash sans marteler le disque.
- **Instance unique** : `app.requestSingleInstanceLock()` obligatoire. Deux instances qui posent chacune un overlay Hardcore est le pire bug possible du produit.
- **Pas d'icône dans le Dock** : `app.dock.hide()` au démarrage, et `LSUIElement: true` dans l'Info.plist. Attention : avec `LSUIElement`, les fenêtres (onboarding, réglages) ne prennent pas le focus automatiquement — appeler `app.dock.show()` avant d'ouvrir une vraie fenêtre puis `hide()` à sa fermeture, ou forcer `app.focus({ steal: true })`.
- **Lancement au démarrage** : `app.setLoginItemSettings({ openAtLogin: true, openAsHidden: true })`.

### 4.7 Sécurité et garde-fous

- **Liste blanche système non modifiable** — jamais d'overlay, jamais de blocage, quel que soit le mode : `com.apple.systempreferences`, `com.apple.ActivityMonitor`, `com.apple.Terminal`, `com.apple.finder`, `com.apple.keychainaccess`, `com.apple.loginwindow`, applications d'accessibilité (VoiceOver, Switch Control, contrôle vocal), et toute application déclarant une catégorie médicale ou d'accessibilité. Cette liste est en dur, non éditable depuis l'interface.
- **Coupe-circuit** — si Breeze détecte plus de 3 crashs du processus principal en 5 minutes, il démarre au cycle suivant en mode dégradé sans overlay et affiche un avertissement. Un bug qui pose un overlay Hardcore impossible à retirer est un incident critique ; il faut une sortie prévue.
- **Sécurité Electron** : `contextIsolation: true`, `nodeIntegration: false`, `sandbox: true` sur tous les renderers, préchargement minimal, IPC typé et validé côté main, `webSecurity` jamais désactivé. Les renderers d'overlay ne chargent aucune ressource distante — tout est local.
- **Notarisation** : indispensable. Une application non notarisée qui demande l'Accessibilité et prend l'écran entier est un profil de logiciel malveillant classique ; Gatekeeper la bloquera et les utilisateurs auront raison de se méfier.
- **Confidentialité** : aucune sortie réseau dans le MVP, hors vérification de mise à jour. Le documenter en clair dans l'application, pas seulement dans une politique de confidentialité.

### 4.8 Performance

Une application de barre de menus est jugée sur ce qu'elle ne coûte pas.

| Métrique | Cible |
|---|---|
| Mémoire au repos | < 120 Mo (main + un renderer de popover) |
| CPU au repos | < 0,3 % moyenne sur 10 min |
| Renderers vivants au repos | 1 (popover), overlays créés à la demande |
| Latence de pose de l'overlay Hardcore | < 200 ms après T-0 |
| Impact batterie | Non listé dans le top 5 de « Consommation d'énergie importante » |

Moyens : détruire les fenêtres d'overlay après usage plutôt que les masquer, geler le tick d'affichage quand le popover est fermé (le calcul reste sur `phaseEndsAt`), ne pas animer le titre de la barre de menus, éviter tout polling remplaçable par une notification système.

### 4.9 Distribution

- **Hors Mac App Store.** Le bac à sable de l'App Store interdit l'API Accessibilité pour ce type d'usage et les niveaux de fenêtre de kiosque. La distribution est directe, en Developer ID signé et notarisé, avec mises à jour via `electron-updater` sur un dépôt de publication.
- Builds universels (`arm64` + `x64`), DMG avec fond d'installation, et une désinstallation propre documentée (le retrait des entrées TCC doit être expliqué, il n'est pas automatisable).

---

## 5. Exigences non fonctionnelles

| Domaine | Exigence |
|---|---|
| Versions macOS | 13, 14, 15, 26 — matrice de test explicite, l'Accessibilité et les niveaux de fenêtre étant les points de variation |
| Accessibilité | Overlays lisibles par VoiceOver (annonce du temps restant), contraste AA minimum, respect de « Réduire les animations » et « Réduire la transparence », navigation clavier complète hors overlay Hardcore |
| Langues | Français et anglais au lancement, chaînes externalisées dès le premier écran |
| Thème | Clair / sombre / système, icône de barre de menus en template image |
| Résilience | Aucune perte d'état sur crash ; reprise de cycle correcte après veille de plusieurs heures |
| Journalisation | Journal local circulaire (7 jours, 5 Mo), exportable depuis Réglages › Aide, sans nom de fenêtre ni contenu |

---

## 6. Découpage de livraison

| Phase | Contenu | Sortie vérifiable |
|---|---|---|
| **0 — Reconnaissance** | Addon natif minimal, validation empirique de : propagation à chaud de l'Accessibilité, comportement de `screen-saver` + `presentationOptions` sur les 4 versions cibles, faisabilité de la détection de partage d'écran et de l'état Focus | Un rapport tranchant chaque inconnu, et un prototype qui pose un overlay inévitable pendant 30 s |
| **1 — Squelette** | Barre de menus, machine à états, persistance, timers par timestamp, réglages Rythme | Un cycle complet tourne, sans blocage |
| **2 — Hardcore** | Overlay total multi-écran, neutralisation des sorties, sortie d'urgence, coupe-circuit | Une pause Hardcore de 5 min tient face à une tentative active de contournement, hors sorties documentées comme impossibles |
| **3 — Applications** | Scan, politiques, déclencheur intelligent, inhibitions | Le cycle ne démarre qu'avec l'app déclencheuse au premier plan |
| **4 — Simple** | Overlays par fenêtre, suivi AX, gestion de l'intersection avec les apps autorisées | Figma est figée, Notes reste utilisable, et déplacer Figma déplace l'overlay |
| **5 — Finition** | Onboarding complet, statistiques, accessibilité, i18n, signature, notarisation, mise à jour | Build notarisé installé sur une machine vierge, onboarding jusqu'au premier cycle sans intervention |
| **6 — Killer features** | §7 | — |

---

## 7. Killer features

Trois propositions, choisies pour renforcer la promesse sans en élargir le périmètre.

### KF1 — Bouclier de calendrier

**Le problème réel.** Un overlay Hardcore qui tombe pendant une réunion client détruit la confiance en une seule occurrence. Aucune heuristique d'application ne rattrape ça : Zoom peut être en arrière-plan, la réunion peut être au téléphone, la présentation peut se faire sur un second écran.

**La fonctionnalité.** Breeze lit le calendrier local via EventKit — **titres et participants jamais lus, seulement les créneaux occupés et le statut de disponibilité**. Il en déduit deux comportements :

1. **Décalage** — une pause qui tomberait dans un créneau occupé est avancée ou repoussée pour s'insérer dans le premier trou disponible, dans une fenêtre de ±20 minutes. Breeze préfère décaler que sauter.
2. **Placement opportuniste** — Breeze détecte les interstices naturels (le trou de 12 minutes entre deux réunions) et y place la pause plutôt que de l'imposer au milieu d'un bloc de travail profond.

Le popover annonce le décalage à l'avance : « Réunion à 15 h — ta pause est avancée à 14 h 40. »

**Pourquoi c'est décisif.** C'est ce qui fait passer Breeze de « logiciel qu'on désactive les jours chargés » à « logiciel qu'on garde justement les jours chargés ». Le mode Hardcore devient acceptable en environnement professionnel, ce qui débloque le segment le plus disposé à payer.

**Coût technique.** Permission Calendriers (`NSCalendarsUsageDescription`), lecture EventKit via l'addon natif, et une logique de placement de créneau. Modéré. La discipline de ne lire que les créneaux, jamais les titres, doit être visible dans le code et affirmée dans l'interface.

### KF2 — Dette de posture

**Le problème réel.** Toutes les applications de pause traitent chaque cycle comme indépendant. On ignore une pause, elle disparaît. Aucune conséquence, aucun apprentissage, et le produit devient du bruit en deux semaines.

**La fonctionnalité.** Chaque report ou pause écourtée crédite une **dette**, exprimée en minutes. La dette a trois effets :

1. **Visible en permanence** — un anneau secondaire autour de l'icône de barre de menus se remplit à mesure que la dette monte. On la voit sans l'ouvrir.
2. **Elle se rembourse** — la pause suivante est allongée à hauteur de la dette, plafonnée à +50 % de sa durée nominale. Reporter deux fois ne fait pas disparaître le besoin de bouger, ça le déplace.
3. **Elle durcit la contrainte** — au-delà d'un seuil configurable (défaut : 20 minutes de dette), Breeze passe automatiquement en Mode Hardcore pour le prochain cycle, avec un préavis explicite : « Tu as reporté trois fois. La prochaine pause sera ferme. » L'escalade est annoncée, jamais subie.

La dette s'efface chaque nuit. On ne culpabilise pas sur la semaine.

**Pourquoi c'est décisif.** C'est le mécanisme qui empêche la dérive progressive vers l'inefficacité — la raison principale d'abandon de cette catégorie de produits. Et c'est une contrainte que l'utilisateur a lui-même armée à froid, ce qui la rend légitime.

**Coût technique.** Faible. Purement de la logique d'état et un rendu d'icône. Le vrai travail est dans le réglage des seuils, à faire en test utilisateur.

### KF3 — Détection de session profonde

**Le problème réel.** Interrompre quelqu'un à la minute 47 d'un état de concentration profonde est un dommage net, même pour sa santé. Un rappel purement horaire ne distingue pas le travail profond du parcours distrait de six onglets.

**La fonctionnalité.** Breeze mesure un **indice de profondeur** à partir de signaux qu'il possède déjà et qui ne coûtent aucune permission supplémentaire : nombre de changements d'application par minute, durée de la session continue sur une même application, régularité de l'activité clavier/souris agrégée (compte d'événements, jamais le contenu — pas de journalisation de frappe, jamais).

Deux comportements :

1. **Report intelligent** — en profondeur élevée, le préavis se transforme en fenêtre de tolérance de 10 minutes : Breeze attend le premier changement d'application, qui signale la fin naturelle du bloc, et déclenche à ce moment-là. Passé les 10 minutes, il déclenche quand même.
2. **Rapport hebdomadaire** — une carte simple : « Tes blocs profonds durent 43 minutes en moyenne. Ton réglage est à 50. » avec une proposition de réglage en un clic. Le produit apprend le rythme de l'utilisateur au lieu de le lui imposer.

**Pourquoi c'est décisif.** Ça déplace Breeze du rayon santé vers le rayon productivité, où le consentement à payer est plus élevé, sans rien retirer à la promesse santé. Et ça répond à l'objection n° 1 de la cible : « ça va me couper en plein travail ».

**Coût technique.** Modéré. Les signaux sont déjà collectés pour le déclencheur intelligent (§4.5) ; il faut y ajouter une agrégation de compteurs d'événements d'entrée (via `CGEventSource.secondsSinceLastEventType`, sans tap d'événements — un event tap serait un dépassement de permission et un risque de notarisation) et un modèle statistique simple. **Aucun apprentissage automatique dans le MVP+1** : des moyennes glissantes et deux seuils suffisent et restent explicables à l'utilisateur.

---

## 8. Indicateurs de succès

| Indicateur | Cible à 30 jours | Mesure |
|---|---|---|
| Rétention | > 40 % des installations ont un cycle actif à J+30 | Locale, agrégée, remontée uniquement si l'utilisateur y consent explicitement |
| Taux de pause respectée | > 70 % des pauses vont à leur terme | Locale |
| Adoption du Hardcore | > 25 % des utilisateurs actifs l'ont activé au moins une semaine | Locale |
| Sorties d'urgence | < 1 par utilisateur et par semaine | Locale — au-delà, le seuil de friction est mal calibré |
| Désinstallation dans les 48 h | < 15 % | Signal d'échec de l'onboarding ou de brutalité mal annoncée |

---

## 9. Risques et décisions à trancher

| # | Risque | Impact | Traitement |
|---|---|---|---|
| R1 | La propagation à chaud de l'Accessibilité n'est pas fiable sur toutes les versions cibles | Onboarding cassé | Phase 0 le mesure ; repli par relance explicite |
| R2 | Le Mode Simple s'avère instable sur les applications qui n'émettent pas les notifications AX | Fonctionnalité dégradée | Polling de secours + liste connue d'applications problématiques + repli sur un overlay unique |
| R3 | La notarisation est refusée à cause des niveaux de fenêtre ou de l'usage AX | Distribution bloquée | Soumettre un build de phase 0 à la notarisation **avant** d'écrire le produit |
| R4 | Un bug pose un overlay Hardcore irrémovible | Incident critique, avis catastrophiques | Sortie d'urgence + coupe-circuit + montre de surveillance dans le processus principal |
| R5 | Le Mode Hardcore est perçu comme du logiciel malveillant | Confiance, désinstallations | Prévisualisation à l'onboarding, honnêteté sur les limites, notarisation, aucune sortie réseau |
| R6 | La détection de l'état des modes Concentration est fragile entre versions | KF/UX dégradée | Ne pas en dépendre ; Breeze active un mode Focus, il n'en lit pas |

**Décisions ouvertes, à trancher en phase 0** — chacune par mesure, jamais par supposition :
1. Propagation à chaud de l'Accessibilité, par version de macOS.
2. Faisabilité et coût de la détection de partage d'écran sans permission d'enregistrement.
3. Comportement exact de `presentationOptions` conjugué à une application tierce déjà en plein écran natif.
4. Verdict de notarisation sur un binaire portant les niveaux de fenêtre de kiosque.

---

## 10. Glossaire

| Terme | Définition |
|---|---|
| **Cycle** | Une phase de travail suivie d'une phase de pause |
| **Profil** | Un jeu nommé de réglages, un seul actif à la fois |
| **Sévérité** | Simple ou Hardcore — le niveau de contrainte de l'overlay |
| **Application ciblée** (`blocked`) | Figée par un overlay pendant la pause |
| **Application épargnée** (`allowed`) | Utilisable pendant la pause |
| **Application ignorée** (`ignored`) | Son usage ne compte pas comme du temps de travail |
| **Déclencheur** | Application dont la présence au premier plan arme le cycle |
| **Préavis** | Bannière de 60 s annonçant la pause |
| **Inhibition** | Condition externe qui repousse temporairement une pause |
| **Dette** | Minutes de pause reportées, à rembourser (KF2) |
| **Sortie d'urgence** | Maintien d'Échap 10 s pour lever un overlay Hardcore |
| **Coupe-circuit** | Désactivation automatique des overlays après crashs répétés |
