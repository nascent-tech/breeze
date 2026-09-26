<!--
type: architecture
titre: Breeze — architecture
cree_le: 2026-09-21T00:00:00+0000
mis_a_jour_le: 2026-09-26T00:00:00+0000
branche: main
statut: a_valider
source: .charpente/decisions/2026-09-21-fondations-D1-D4.md
-->

# Breeze — architecture

Ce document est le **cerveau technique** du produit. Il dit comment Breeze est construit, et pourquoi
il l'est ainsi. Il se lit sans être ingénieur : chaque terme technique est donné avec ce qu'il change
pour le produit. Ce qu'il faut construire et pourquoi vit dans le brief (`.charpente/brief/`) ; les
décisions fondatrices qui commandent ce document vivent dans le registre
(`.charpente/decisions/2026-09-21-fondations-D1-D4.md`). Ici, rien n'est tranché qui ne le soit
là-bas — ce document **transcrit** ces décisions en structure de code.

---

## 1. En une page

Breeze impose la pause. À l'échéance, il recouvre l'écran (ou les seules applications visées) d'un
overlay portant un décompte, et **aucun bouton ne l'écourte, ne le retarde ni ne l'annule**. Le
système d'exploitation reste l'arbitre : on peut toujours quitter Breeze ou éteindre la machine, et
Breeze ne prétend jamais à l'infaillibilité — il le **dit à froid**, avant que l'utilisateur le
découvre.

Le produit tient sur une idée d'architecture unique : **le cœur qui décide ne touche à aucun système
d'exploitation.** Toute la logique — compter le travail, annoncer la pause, la tenir, juger une
absence — vit dans un module de code pur, sans aucune dépendance à macOS, Windows ou Linux, et
testable sans machine réelle. Les systèmes d'exploitation n'entrent que par les **bords**, à travers
des contrats étroits (les *ports*), chacun implémenté une fois par plateforme (les *adaptateurs*).

Conséquence directe : **la promesse ne dépend d'aucune détection.** Le cycle tourne même si Breeze ne
voit pas quelle application est au premier plan, même s'il ne peut pas suivre le cadre d'une fenêtre.
Une capacité perdue ne fait jamais que deux choses — le décompte avance plus vite (côté sûr), ou la
couverture visuelle est moins complète — et jamais l'inverse. Elle ne lève jamais une contrainte.

---

## 2. La structure : cinq couches, une seule qui décide

Breeze est un **espace de travail Cargo** (Rust) qui héberge une application **Tauri v2** (un cœur
Rust + une interface web). Les couches, de la plus pure à la plus proche du système :

| Couche | Crate | Ce qu'elle sait | Ce qu'elle ne sait pas |
|---|---|---|---|
| **Domaine** | `breeze-domain` | Le cycle, ses états, ses règles, ses refus | Rien du monde : ni OS, ni fenêtre, ni horloge réelle, ni fichier |
| **Ports** | `breeze-ports` | Les contrats (traits) entre le domaine et le monde | Aucune implémentation |
| **Application** | `breeze-app` | L'orchestration : le planificateur, l'exécuteur d'overlays, les cas d'usage | Comment un OS pose une fenêtre |
| **Adaptateurs** | `breeze-bridge-*` | Comment **un** OS réalise chaque port | Les règles du produit |
| **Enveloppe** | `src-tauri` + `ui/` | Composer le tout, exposer les fenêtres et l'IPC, afficher | Décider quoi que ce soit du métier |

La règle de dépendance est **unidirectionnelle** : `ui` → `src-tauri` → `breeze-app` → `breeze-ports`
← `breeze-bridge-*`, et tout le monde → `breeze-domain`. **Le domaine n'importe rien.** Un adaptateur
n'importe jamais un autre adaptateur. L'interface web ne connaît que des **projections**
(`CycleSnapshot`, lecture seule) et des **commandes** (`Command`) : elle ne prend aucune décision.

Pourquoi cinq couches et pas moins : le cycle doit **survivre à une interface figée** et **tourner
sans aucune fenêtre** (au démarrage, en mode dégradé, sous coupe-circuit). Mettre la logique dans
l'interface, ou dans l'enveloppe Tauri, la lierait au webview et au fenêtrage — exactement ce qui ne
doit jamais pouvoir tomber en même temps que la décision.

---

## 3. Le domaine pur — le cœur qui décide

`breeze-domain` n'a **aucune dépendance système** (seulement `core`/`alloc`, et `serde` derrière une
option pour sérialiser les instantanés). **Le temps y est injecté** : chaque transition reçoit un
instant monotone et une horloge murale en paramètre, jamais lus au fil de l'eau. C'est ce qui rend le
cœur testable par des scénarios temporels déterministes (« à t0… avance de 50 min… attends
[PRÉAVIS] »).

### 3.1 La machine à états du cycle

```
Inactive                                  hors plage horaire ou jour inactif
Working { countdown }                      countdown = Running{deadline} | Frozen{remaining} | Due{since}
                                           gel = inactivité | app Ignorée au premier plan (FreezeReason)
Notice  { deadline }                       PRÉAVIS — 1 minute, constante
BreakActive { deadline, severity, mode }   mode = Nominal | Degraded(FramesUnobservable | TooManyWindows)
Returning { deadline }                     RETOUR — 3 secondes, constante
```

Trois états **transversaux** se superposent au cycle : `Suspension` (SUSPENDU, gèle la phase sans
l'effacer), et `SessionState = Awake | Asleep | Locked | OtherUserActive` (veille, verrouillage,
changement rapide d'utilisateur). Le modèle retenu (D1, **sans négociation**) n'a **ni [ARMÉ] ni
[INHIBÉ]** : il n'y a ni déclencheur intelligent à armer, ni report à inhiber.

Une extension est **déclarée mais inactive au lancement** : `Working { countdown: Due { held_until } }`
porte la retenue de la détection de session profonde (§9.3 du brief), qui n'entre pas au lancement.

### 3.2 Ce que le domaine porte, sans exception

- `Severity = Simple | Hardcore` · `AppStatus = Blocked | Spared | Ignored` · `SafetyList`.
- **Les statuts d'applications** (`AppStatuses`, tenus par le cycle) : les statuts **choisis** par
  l'utilisateur, les statuts **effectifs** du cycle en cours, et la **liste de sécurité** de l'OS
  (fournie par l'adaptateur), qui prime toujours — une app qui y figure vaut `Spared` et refuse toute
  modification (`CommandError::LockedApp`). Toute app inconnue est `Blocked`. Affaiblir un statut
  (Bloquée → Épargnée/Ignorée, Épargnée → Ignorée) n'agit qu'au travail suivant, au même instant que
  le rythme et la sévérité en attente ; renforcer agit tout de suite.
- **Le gel du décompte** a deux causes cumulables : l'inactivité (§10.1) et une application
  **Ignorée effective au premier plan** (§8.3), tant que le décompte n'est pas échu. Il ne repart que
  quand aucune des deux ne tient plus ; la raison est exposée (`idle | ignored_app`). Une identité au
  premier plan inconnue compte comme du travail (§10.5).
- `Rhythm { work: 5..=180 min, pause: 1..=60 min, pause ≤ work, schedule, active_days }` — les trois
  refus de saisie du brief sont des **erreurs typées** (`NoActiveDay`, `DegenerateRange`,
  `PauseLongerThanWork`).
- **La règle du sens** : un réglage qui **affaiblit** la contrainte attend le cycle suivant
  (`PendingSettings`) ; un réglage qui **renforce** s'applique tout de suite ; tout réglage — rythme,
  sévérité, statut d'une application — est refusé dès qu'une pause est due, préavis, pause et retour
  compris (`CommandError::BreakDue`, code `break-due`).
- **Le sort de chaque pause** : `BreakOutcome = Served | ValidatedByAbsence | Interrupted { unserved,
  door }`, avec `door = Quit | TrayMenu | HardcoreExitGesture | SuspensionOverrun | Crash`. Ce sort
  est **enregistré dès le lancement** (un registre `ledger`) pour que la dette de posture (§9.2)
  puisse entrer plus tard sans rien migrer — mais **rien n'escalade au lancement**.
- **Le verdict d'absence** : une absence plus longue que la pause (pendant le travail) valide le
  cycle ; plus longue que le restant (pendant la pause) la valide servie ; sinon l'échéance produit
  son effet, préavis sauté. **Le temps sans processus ne vaut jamais absence** — rien n'a été mesuré.
- **Le coupe-circuit** : `CrashCounter` (3 chutes en 5 min → `Breaker::Armed`) désarme les overlays
  pour le cycle suivant. La sortie doit exister **avant** le défaut qui la rend nécessaire.
- **L'horloge** : les échéances sont **murales** mais comparées au **temps monotone réellement
  écoulé** ; un recul d'horloge ne recule aucune échéance (§10.7 du brief).
- **Le mode d'une pause** se fixe **au premier instant de [PAUSE ACTIVE]**, sur la capacité « cadres
  observables » relevée à cet instant (y compris quand la pause démarre au réveil, §10.4) : en Mode
  Simple, cadres observables → `Nominal` (un voile par fenêtre bloquée), inobservables → `Degraded`
  (voile plein écran par moniteur). Il ne bouge plus jusqu'à [RETOUR], sauf une seule bascule
  possible, et une seule fois : au-delà de **24 fenêtres bloquées visibles** (`WINDOW_VEIL_CAP`), la
  pause passe en `Degraded(TooManyWindows)` pour rester réactive. Le Mode Hardcore n'en dépend pas.

Le domaine est testé **sans aucun double** : scénarios temporels + propriétés d'invariant (« aucune
commande n'écourte une pause », « une pause due n'est jamais coupée par l'horloge », « le
coupe-circuit prime sur toute escalade »).

---

## 4. Les ports — des ponts système étroits, une horloge

`breeze-ports` déclare des **traits** Rust (des contrats), un par besoin **du produit**, jamais un par
API d'OS. Le domaine n'en importe aucun ; c'est `breeze-app` qui les consomme.

| Port | Ce qu'il apporte au produit | Capacité rapportée |
|---|---|---|
| `ForegroundAppPort` | Quelle application est devant (identité seule, jamais le titre) ; `None` = inconnue | — |
| `WindowFramesPort` | Les fenêtres visibles des autres applications : numéro, identité de l'app, cadre en points globaux (origine en haut à gauche de l'écran principal) ; jamais le titre | `Ok` = observables, `Err(FramesUnobservable)` |
| `OverlaySurfacesPort` | Poser un overlay par écran (`cover_display`), un voile par fenêtre (`cover_window`), le faire suivre (`reframe`), le retirer (`dismiss`, `dismiss_all`) | `Layered \| BestEffort \| PlainFullscreen` |
| `PresentationLockPort` | Le verrou de présentation d'une pause Hardcore : `lock` à l'entrée (barre de menus et Dock masqués, changement d'application désactivé, Breeze au premier plan), `hold` à chaque tick (options maintenues et surfaces ramenées devant ; le premier plan ne se reprend pas seul depuis macOS 14), `release` une seule fois à la sortie | — |
| `SafetyListPort` | La liste de sécurité de l'OS courant (§10.6), fixe | — |
| `DisplayEnumerationPort` | Les écrans présents et leurs changements | — |
| `SessionSignalsPort` | Inactivité (instant, jamais contenu), veille, verrouillage | `Reliable \| Inferred` |
| `NotificationsPort` | Le préavis quand la bannière propre ne peut pas se poser | `Granted \| Denied \| NotApplicable` |
| `AutostartPort` | Le lancement au démarrage | — |
| `PersistencePort` | L'instantané, le journal borné, les statistiques 30 j, l'export | — |
| `UpdateCheckPort` | La seule sortie réseau, désactivable | — |
| `ClockPort` | Temps monotone + horloge murale (port du **domaine**). La détection de saut est *stateful* et vit chez celui qui *poll* (palier absence/§10.7), pas dans l'adaptateur sans état | — |
| `InstalledAppsPort` | Le catalogue des applications installées et leur **vraie** icône, pour que l'utilisateur choisisse ce qu'il épargne (les apps de la liste de sécurité installées y figurent, verrouillées) | — |

`InstalledAppsPort`, `SafetyListPort` et `WindowFramesPort` portent une décision du produit — quelle
app est épargnée, ce que le Mode Simple voile — donc ce sont des ports (l'identité d'une app est une
notion métier, `AppId`). L'icône voyage en PNG déjà rendu ; l'extraction native (macOS : `sips` +
`Info.plist`) vit dans l'adaptateur `breeze-bridge-macos`, jamais dans le port.

**Aucune permission d'Accessibilité n'est demandée.** Sur macOS, les cadres des fenêtres viennent de
`CGWindowListCopyWindowInfo(onScreenOnly | excludeDesktopElements)`, qui rend sans permission les
bornes, le PID, la couche et le numéro de chaque fenêtre — Breeze n'en lit jamais le titre. Le PID
devient une identité par `NSRunningApplication`, l'app au premier plan vient de
`NSWorkspace.frontmostApplication`.

`PresentationLockPort` est un port à part, et non trois méthodes de plus sur `OverlaySurfacesPort` :
une surface se pose par écran, le verrou vaut pour toute l'application ; ses doubles de test restent
séparés, et `breeze-bridge-null` le réalise en non-op hors macOS, où l'overlay revient devant sans rien
bloquer (§5.3). Le domaine n'en sait rien : c'est l'exécuteur qui le pose quand la couverture
Hardcore commence.

Deux choses ne sont **pas** des ports, car elles ne portent aucune décision du domaine : l'icône
d'état et l'instance unique. Ce sont des adaptateurs de l'enveloppe Tauri. La présence de l'icône
(`tray: Available | Missing`) remonte tout de même dans le relevé de capacités, parce que le produit
doit pouvoir **le dire à froid** (GNOME sans extension).

---

## 5. Les adaptateurs — un par plateforme

Chaque adaptateur implémente les ports pour **un** OS, et rapporte honnêtement ses capacités.

| Crate | Réalise | Avec (bibliothèques natives) |
|---|---|---|
| `breeze-bridge-macos` | catalogue d'apps, liste de sécurité, premier plan, cadres des fenêtres, inactivité | `objc2-app-kit` (`NSWorkspace`, `NSRunningApplication`), `objc2-core-graphics` (`CGWindowListCopyWindowInfo`, `CGEventSource`), `objc2-core-foundation` ; à venir : `SMAppService`, `UNUserNotificationCenter`. Le niveau `CGShieldingWindowLevel` et le verrou de présentation vivent dans l'enveloppe (`src-tauri/src/bridge`), parce qu'ils s'appliquent aux `NSWindow` et au `NSApplication` de Tauri |
| `breeze-bridge-windows` | les neuf ports | crate `windows` (Win32, DWM, WTS, Power — fenêtres `WS_EX_TOOLWINDOW` topmost par moniteur), notifications WinRT |
| `breeze-bridge-linux-x11` | foreground, frames, overlay, écrans, session | `x11rb` (EWMH, RandR, MIT-SCREEN-SAVER — fenêtres override-redirect), `zbus` (logind, notifications, icône SNI) |
| `breeze-bridge-linux-wayland` | idem, selon le compositeur | `wayland-client`, protocoles `wlr`/`plasma`, `gtk-layer-shell` (overlay), `ext-idle-notify` / Mutter / logind |
| `breeze-bridge-common` | persistance, MàJ, démarrage auto, horloge | `rusqlite` (WAL, écriture atomique), plugins Tauri officiels |
| `breeze-bridge-null` | les ports en **non-op déclaré** : premier plan inconnu, cadres inobservables, liste de sécurité vide, verrou de présentation sans effet | — (mode de production : coupe-circuit et sessions inconnues) |

`breeze-bridge-macos` refuse le code non sûr (`deny(unsafe_code)`) sauf dans un seul module,
`cf_containers`, qui affirme à objc2 que les conteneurs Core Foundation rendus par Core Graphics ne
contiennent que des objets CF (`CFType`) — l'invariant y est écrit ; chaque valeur lue est ensuite
vérifiée à l'exécution. Les clés du dictionnaire de fenêtre sont recréées par leur nom plutôt que lues
dans les constantes externes (accès non sûr).

Le choix de l'adaptateur Linux se fait **au lancement**, sur `XDG_SESSION_TYPE` et la présence
effective des protocoles Wayland — **jamais sur le nom du bureau**. `breeze-bridge-null` n'est pas
qu'un outil de test : c'est ce que le coupe-circuit installe, et ce sur quoi tombe une session
inconnue. Le produit y reste honnête — le cycle tourne, les pauses comptent servies, le panneau dit
pourquoi — mais il n'est plus contraignant.

---

## 6. L'application — une seule boucle, un seul écrivain de l'état

`breeze-app` porte les **cas d'usage** (commandes et requêtes séparées, CQRS léger sans bus) :
`StartCycle`, `Suspend(durée)`, `Resume`, `ChangeSeverity`, `ChangeRhythm`, `SetAppStatus`,
`QuitRequested`, `HardcoreExitGestureCompleted` ; requêtes `GetSnapshot`, `GetTodaySummary`,
`GetStats30d`.

Le **planificateur** (`Scheduler`) est la **seule** boucle : il réveille le domaine aux échéances
(timer monotone), à chaque événement de port, et à chaque commande. Après chaque transition, il
**persiste l'instantané avant** de poser ou retirer un overlay — c'est ce qui garantit que l'état
écrit est toujours en avance sur l'écran, et donc qu'aucun overlay irretirable ne peut survivre à un
plantage. L'**exécuteur** (`Enforcer`) traduit l'état du cycle en appels d'`OverlaySurfacesPort` et
de `PresentationLockPort`, en
respectant les délais de pose (≤ 200 ms pour un overlay Hardcore à l'échéance ; ≤ 500 ms pour un
écran ou une application apparus en cours de pause).

À chaque tick (250 ms), l'hôte relève **hors du verrou** l'instant de la dernière saisie,
l'application au premier plan et les fenêtres visibles (`Observation`), puis le planificateur les
passe au domaine et l'exécuteur réconcilie :

| Pause | Ce qui est posé |
|---|---|
| Hardcore | un overlay opaque plein écran par écran, au niveau `CGShieldingWindowLevel`, et le verrou de présentation |
| Simple, `Degraded` (cadres inobservables, ou plus de 24 fenêtres bloquées) | un voile plein écran par écran |
| Simple, `Nominal` | un voile par fenêtre **bloquée effective** (et par fenêtre sans identité, donc inconnue) : posé pour une fenêtre apparue, déplacé/redimensionné pour une fenêtre qui bouge, retiré pour une fenêtre fermée ou minimisée ; la surface existante est toujours réutilisée |

Les fenêtres de Breeze, celles d'une couche ≠ 0, les fenêtres transparentes et celles de moins de
40 × 40 points ne sont jamais voilées. Un relevé de cadres qui échoue en cours de pause laisse les
voiles tels quels. Côté Tauri, un voile de fenêtre est une `WebviewWindow` `overlay.html?kind=window`
sans décorations, transparente, **non focusable** (l'app épargnée garde le focus), au niveau de
fenêtre **normal** et rangée juste au-dessus de sa fenêtre cible
(`NSWindow orderWindow:relativeTo:` avec le numéro CGWindow de la cible, qui est le `windowNumber`
d'AppKit) : une application épargnée placée devant la fenêtre bloquée, ou amenée par-dessus, reste
devant le voile et utilisable (§8.3, §10.6). Ce rangement est refait à chaque tick, pour que le voile
repasse au-dessus d'une app bloquée que l'utilisateur ramène devant ; il n'active jamais Breeze. Un
voile de fenêtre appartient à l'espace de sa fenêtre (pas à tous les bureaux) ; il porte, comme les
surfaces plein écran (elles, sur tous les bureaux et au-dessus de tout), le comportement
`FullScreenAuxiliary`, qui le laisse paraître sur l'espace d'une app en plein écran natif. Les points
de `CGWindowList` et les positions **logiques** de Tauri partagent le même repère (origine en haut à
gauche de l'écran principal, écrans secondaires en coordonnées globales), le cadre se pose donc tel
quel ; au déplacement, la taille est posée **avant** la position (AppKit redimensionne en gardant le
coin bas-gauche, la position posée en dernier fixe le coin haut-gauche). Création, déplacement et
retrait sont **expédiés** au thread principal sous le verrou du planificateur (l'exécuteur les
demande depuis `poll`), sans attendre (`run_on_main_thread`) ; leur **exécution** a lieu plus tard,
sur la boucle d'événements, hors verrou. Le rangement de chaque tick est expédié juste après, une
fois le verrou rendu.

**Le verrou de présentation d'une pause Hardcore** (§8.5). L'exécuteur l'appelle **après** les
surfaces : `lock` au premier poll où la couverture Hardcore est demandée, `hold` à chaque poll qui
suit, `release` une seule fois dès qu'elle ne l'est plus — pause servie, geste Échap, sortie, ou pause
qui change de mode. Les appels étant expédiés dans l'ordre, le verrou s'exécute derrière la création
des surfaces et sa levée derrière leur retrait. Côté macOS (`bridge/tauri_presentation_lock.rs`,
`native_presentation.rs`, `presentation_options.rs`) :

- chaque surface Hardcore monte au niveau `CGShieldingWindowLevel()` (`setLevel` sur le `NSWindow`,
  thread principal), en gardant `FullScreenAuxiliary` et `CanJoinAllSpaces` ; le voile Simple reste
  « toujours devant » et le voile de fenêtre au niveau normal ;
- `lock` retient l'application au premier plan (lue sur le fil du ticker, avant que la création des
  surfaces n'active Breeze), pose `HideDock | HideMenuBar | DisableProcessSwitching |
  DisableHideApplication` — combinaison vérifiée contre les règles d'AppKit avant d'être appliquée, une
  combinaison refusée levant une exception —, active Breeze et rend clé la surface de l'écran
  principal, celle qui reçoit Échap ;
- `hold` ne fait rien tant que Breeze est actif avec une surface Hardcore pour fenêtre clé, ni quand
  `loginwindow` (Forcer à quitter, dialogue d'extinction) a la main ; sinon il réactive Breeze et
  ramène les surfaces devant, en moins d'un tick (250 ms) ;
- `release` rend `Default`, puis la main à l'application retenue si elle tourne encore.

**Ce que macOS permet réellement (vérifié sur macOS 27).** Depuis macOS 14, l'activation est
coopérative : ni `activateIgnoringOtherApps:` ni `NSRunningApplication.activate` ne donnent le premier
plan à une application qui le demande sur un minuteur, sans geste de l'utilisateur. Et tant que Breeze
n'est pas réellement actif, AppKit n'applique pas ses options de présentation. Au début d'une pause
Hardcore, la surface couvre donc bien chaque écran, barre de menus et Dock compris (niveau bouclier),
mais le clavier reste à l'application recouverte. **Le premier clic sur la surface** — un geste de
l'utilisateur — active Breeze : les options s'appliquent (⌘Tab désactivé), la surface reçoit Échap, et
`hold` les maintient. La surface l'écrit tant qu'elle n'a pas le clavier (« Clique sur l'écran, puis
maintiens Échap pour sortir »), et l'accueil comme les réglages le disent à froid. Un capteur clavier
non activant aurait pris le clavier sans ce clic ; il est écarté, le brief refusant tout mécanisme qui
capte les frappes (§8.5, §13). `activateIgnoringOtherApps:` reste tenté à chaque tick : il est sans
effet sur macOS 14+, utile sur macOS 13.

**Forcer à quitter et la fermeture de session ne sont jamais
désactivés.** ⌘Q vient du menu d'application que Tauri pose par défaut sur macOS (« Quitter » →
`terminate:`) : invisible pendant la pause, son raccourci marche dès que Breeze est actif ; il passe par
`RunEvent::Exit`, qui compte la pause interrompue (porte `Quit`), écrit le journal et rend la
présentation par défaut avant la fin du processus.

---

## 7. La stack, en clair

**Tauri v2** (et non Electron) : le cœur Rust pose lui-même, **dans le même processus**, les niveaux
de fenêtre et gestes natifs de chaque OS ; la mémoire au repos tient sous 120 Mo (Electron ouvrirait
un processus de rendu par fenêtre) ; le planificateur tourne hors du webview, donc une interface
figée ne fige pas le cycle. Le prix assumé : trois moteurs de rendu web (WebKit, WebView2, WebKitGTK)
à vérifier pour le verre — avec un repli opaque `#F4F6FB` quand un compositeur Linux n'offre pas la
transparence.

**Plugins Tauri retenus** : `single-instance` (deuxième instance interdite), `autostart`,
`notification`, `updater` (désactivable, jamais appliqué pendant une pause due). **Pas de plugin pour
la persistance** : `rusqlite` derrière le port, pour garder le schéma et l'écriture atomique sous
contrôle.

**Interface** : les 17 maquettes deviennent des vues ; le CSS des *Fondations* devient la feuille de
tokens (`ui/tokens.css`). Système visuel « **liquid glass × Material 3** » : encre `#1C2130`,
primaire `#0A6FDB`, menthe `#0E7A57` (pause), violet `#6D4FD4` (sévérité), fonds *mesh* pastel,
Instrument Sans, grille de 4 px, rayons 26/16/12/10.

---

## 8. Les données, et ce qui ne sort jamais de la machine

Un **seul fichier SQLite** par utilisateur (`rusqlite`, journal WAL, écriture atomique), rangé selon
l'OS : `~/Library/Application Support/Breeze/` (macOS), `%APPDATA%\Breeze\` (Windows),
`$XDG_DATA_HOME/breeze/` (Linux). Il porte : l'instantané persisté (JSON versionné dans une table),
le journal local (**circulaire, borné à 7 jours ou 5 Mo**, exportable par l'utilisateur seul), les
statistiques (30 jours glissants), les réglages.

**Ce que Breeze lit** : l'identité de l'application au premier plan, la position et la taille des
fenêtres visibles (et le PID qui les rattache à une application). **Ce qu'il ne lit jamais** : le titre d'une fenêtre, le contenu de l'écran,
les frappes clavier. Le journal ne contient jamais de titre ni de contenu applicatif. **Rien ne sort
de la machine**, hors la vérification de mise à jour — déclarée, visible, désactivable, et jamais
effectuée pendant une pause ou un préavis.

---

## 9. La sécurité, par construction

- **Aucun hook clavier global bas niveau, sur aucun OS.** Neutraliser Alt-Tab/Win/Super exigerait de
  lire les frappes : c'est interdit, et cela changerait le profil de sécurité du binaire. Breeze
  **déclare** donc que ces raccourcis restent, plutôt que de les capturer.
- **Aucune exécution privilégiée** : pas de service Windows, pas de `setuid`, pas de daemon root.
  Breeze est une application utilisateur ordinaire — celui qui subit la contrainte est celui qui l'a
  posée.
- **Aucune permission demandée pour une fonctionnalité qui ne s'en sert pas** : ni enregistrement
  d'écran, ni pilotage d'autres applications, ni micro, ni caméra, ni Accessibilité — le voile
  fenêtre par fenêtre s'appuie sur `CGWindowList`, qui ne demande rien.
- **Isolation Tauri** : pas de Node dans le rendu ; une **capacité IPC déclarée par fenêtre**
  (`src-tauri/capabilities/`) — le panneau ne peut pas ce que l'overlay peut.
- **Le Mode Hardcore ne lit aucune touche et ne demande aucune permission** : il masque la barre de
  menus et le Dock et désactive ⌘Tab par les options de présentation publiques d'AppKit ; seul
  `overlay.html` écoute Échap, dans sa propre fenêtre.
- **La notarisation d'un binaire Tauri** portant `CGShieldingWindowLevel` est un point **bloquant,
  sans repli** : elle se mesure avant toute écriture de fonctionnalité.

---

## 10. Développement, portes, et ordre de livraison

**Les quatre portes** qui définissent « terminé » (chacune verte avant toute PR) : `typecheck`
(`cargo check` + types TS), `lint` (`cargo clippy -D warnings` + lint front), `test` (`cargo test` +
tests d'application contre `breeze-bridge-null`), `build` (`cargo build` + `tauri build`).

**L'ordre de livraison naturel**, chaque étape laissant les portes vertes :

1. `breeze-domain` complet et prouvé — **aucun OS requis**.
2. `breeze-app` + `breeze-bridge-null` — le cycle tourne sans overlay, le panneau vit.
3. `breeze-bridge-macos` — premier OS ; **notarisation bloquante** mesurée d'abord.
4. `breeze-bridge-windows`.
5. `breeze-bridge-linux-x11`.
6. `breeze-bridge-linux-wayland` (KDE/wlroots), puis GNOME en **dégradé déclaré**.

Avant d'écrire une fonctionnalité d'adaptateur, le cadrage technique produit deux **relevés** (pas des
paris) : la **matrice de capacités mesurée** par OS/compositeur, et la **mémoire au repos par OS**
(icône seule, puis trois overlays). Une valeur inconnue bloque ou applique le repli le plus
conservateur — jamais une valeur inventée.

---

## 11. L'arbre du dépôt (cible)

```
breeze/
├── .charpente/{brief,decisions,cadrage,conception,plan}/   # artefacts en français
├── ARCHITECTURE.md · DESIGN.md · EXPERIENCE.md · README.md
├── Cargo.toml                       # [workspace] members = crates/*, src-tauri
├── crates/
│   ├── breeze-domain/               # PUR — aucune dépendance système
│   ├── breeze-ports/                # un trait par besoin, un fichier chacun
│   ├── breeze-app/                  # cas d'usage, Scheduler, Enforcer, projections
│   ├── breeze-bridge-common/        # persistance, update-check, autostart, clock
│   ├── breeze-bridge-macos/
│   ├── breeze-bridge-windows/
│   ├── breeze-bridge-linux-x11/
│   ├── breeze-bridge-linux-wayland/
│   └── breeze-bridge-null/          # capacités « inconnues » ; coupe-circuit et tests
├── src-tauri/                       # hôte : compose, expose l'IPC, les fenêtres, le tray
│   ├── tauri.conf.json · capabilities/ · src/{main,lib,shell/,ipc,platform_select}
├── ui/                              # TypeScript, sans logique métier
│   ├── tokens.css · screens/ · lib/{ipc,snapshot}.ts
├── design/                          # les 17 maquettes, retouchées (D1/D2)
└── tests-e2e/                       # par OS : pose < 200 ms, ré-assertion < 500 ms, mémoire au repos
```

---

## 12. Ce que cette architecture refuse

- **La machine à états côté TypeScript** — le cycle doit survivre à un webview gelé et tourner sans
  fenêtre.
- **Un bus d'événements ou une saga** — un seul processus, une seule boucle, un seul écrivain de l'état.
- **Un super-port `SystemBridge` à trente méthodes** — des ports étroits, remplaçables un à un.
- **Un ORM** — un fichier SQLite, un instantané JSON versionné, un journal borné.
- **Le scaffold Electron** — supprimé, pas conservé « au cas où » : deux hôtes de fenêtrage, deux fois les bugs.
- **Que le domaine connaisse les OS** — toute astuce d'empilement vit dans un adaptateur, derrière le
  délai de 500 ms ; le domaine ne l'apprend que par `SurfaceLost`.
