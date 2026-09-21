---
type: registre-de-decisions
titre: Breeze — décisions fondatrices (D1 à D4)
cree_le: 2026-09-21T00:00:00+0000
statut: a_valider
auteur: agent de décision (Fable)
sources:
  - /Users/houssenedao/Code/NascentTechProjects/breeze/BRIEF.md (génération AVEC négociation, versionnée)
  - scratchpad/stale-charpente-brief/.charpente/brief/00..10 (génération SANS négociation, 2026-08-31)
  - scratchpad/design/*.dc.html (17 artboards, dessinés contre la génération AVEC)
---

# Breeze — registre de décisions

Quatre décisions, chacune sous la forme Contexte / Décision / Conséquences / Ce qui est déclaré
dégradé. Elles sont prises pour être **citées** par la réécriture du brief, l'architecture, le
cadrage et l'implémentation. Une décision sans contrepartie n'est pas une décision : chacune nomme
ce qu'elle écarte et ce qu'elle ne pourra pas garantir.

Ce que ce registre ne fait pas : il n'invente aucune capacité système. Là où une capacité dépend
d'une version d'OS ou d'un compositeur, elle est classée **mesure** (à relever avant d'écrire la
fonctionnalité) ou **déclarée dégradée** (Breeze le dit à froid).

---

## D1 — Philosophie produit : SANS négociation

### Contexte

Deux générations du brief coexistent :

- **AVEC négociation** (`BRIEF.md`, versionné, 1 271 lignes) : budget de report de 15 min par
  cycle, report de 5 min, inhibition (visio, partage d'écran, présentation, plage d'exception),
  « Terminer la pause » après une minute, redémarrage du cycle tarifé, déclencheur intelligent,
  état [ARMÉ] et [INHIBÉ], 43 décisions numérotées dont 20 servent uniquement à boucher les fuites
  du budget (décisions 4-11, 30-31, 33-35, 41-43).
- **SANS négociation** (brief charpente du 2026-08-31, non versionné) : report, budget, inhibition,
  déclencheur intelligent et exceptions **supprimés** ; sept fonctionnalités au lieu de huit ;
  onboarding en cinq écrans ; [ARMÉ] et [INHIBÉ] disparaissent ; la seule issue d'une pause est
  quitter Breeze ou, en Hardcore, le maintien d'Échap — **au même prix** ; dette de posture,
  bouclier de calendrier et session profonde restent après lancement, retravaillés pour tenir sans
  négociation ; 22 décisions numérotées.

Le design (17 artboards) illustre la génération AVEC : « Reporter 5 min », « Budget de report
15 / 15 min », « Terminer la pause — débitera les 9 min non faites », « Déclencheur intelligent »,
« Exceptions automatiques », icône « PAUSE DUE, INHIBÉE », stat « 11 Reports — 55 min débitées ».

### Décision

**La cible du produit livré est la génération SANS négociation.** Le brief à réécrire part du brief
charpente du 2026-08-31, pas de `BRIEF.md`.

Quatre raisons, par ordre de poids :

1. **Cohérence de la promesse.** « La pause n'est pas une proposition, c'est une échéance » n'est
   vraie qu'avec zéro bouton. La version AVEC promet « tu ne peux pas m'ignorer » puis offre un
   bouton « Reporter » dans le préavis : c'est un rappel mieux réglé, et c'est exactement ce que
   le §2 des deux briefs dit avoir échoué. Le budget de 15 min *chiffre* la négociation ; il ne la
   supprime pas.
2. **Coût d'implémentation et surface de fuite.** La version AVEC porte 20 décisions sur 43 qui ne
   font que fermer des chemins ouverts par le budget (« reporter puis redémarrer », « report puis
   application ignorée », « redémarrer à la 7ᵉ minute en boucle »…). Chaque chemin est un test de
   propriété à écrire et un bug latent. La version SANS supprime la classe entière : pas de budget,
   pas de prix, pas de refus par insuffisance de budget, pas de quota par sévérité. Sa machine à
   états a **5 états de cycle + 2 transversaux** au lieu de 7 + 3. Estimation grossière : le domaine
   SANS coûte 40 % de moins à écrire et à prouver, et sa suite de tests d'invariants est de l'ordre
   de 25 propriétés au lieu de 45.
3. **Intention la plus récente du fondateur.** Le brief SANS est postérieur (31 août, statut
   `a_valider`), et son §14 nomme la version AVEC comme « la version précédente ». C'est la
   direction prise, pas une variante explorée.
4. **Cross-platform (D2).** La négociation AVEC repose sur des signaux que seul macOS expose un peu
   (partage d'écran, présentation, identité fiable de l'app au premier plan pour inhiber). Sur
   Wayland, ces signaux n'existent pas ; un modèle à inhibition y serait *silencieusement* faux
   (l'overlay tombe en pleine visio parce que le compositeur ne dit pas qui est devant). Le modèle
   SANS ne dépend d'aucun de ces signaux pour tenir sa promesse au lancement : la perte de
   détection ne fait qu'accélérer le décompte (côté sûr), jamais lever une contrainte.

Ce qui est écarté : la version AVEC en entier, y compris comme « mode optionnel ». Un mode qui
rouvre la négociation rouvre le produit ; le §14 du brief SANS dit qu'un tel retour « justifierait
un nouveau brief ».

### Conséquences

- **Le brief réécrit** part du brief SANS et y ajoute la partie cross-platform (D2). `BRIEF.md`
  versionné devient un document historique ; il n'est plus la source.
- **Le design est à retoucher, pas à refaire.** Le système visuel (Fondations, verre, tokens,
  typographie, cinq états de bouton) est indépendant de la négociation. Modifications par artboard,
  toutes de légende ou de suppression :

| Artboard | Ce qui change |
|---|---|
| `Main.dc.html` (panneau travail) | Supprimer « Reporter 5 min », « Redémarrer le cycle » et sa raison budget, la ligne « Budget de report 15 / 15 min ». Garder « Faire une pause maintenant » ? **Non** : le brief SANS n'offre aucune commande de pause immédiate (§6, §8.1) — supprimer. Garder « Suspendre » (libre en [TRAVAIL]), « Sévérité », bilan du jour, « Réglages… », « Quitter Breeze ». Réserver un emplacement « Dette : 0 min » masqué tant que §9.2 n'est pas livré |
| `PanneauPause.dc.html` | Supprimer budget, « Terminer la pause », « Suspendre » et leurs raisons. Ne reste que le décompte, « Sévérité — figée dès qu'une pause est due », « Réglages… » grisé avec raison, « Quitter Breeze » avec la mention « comptée interrompue » |
| `Preavis.dc.html` | Supprimer « Reporter 5 min » et sa légende. La bannière ne porte que « Pause dans 0:47 · Mode Simple · 10 min ». Rien ne s'y clique |
| `PauseSimple.dc.html` | Supprimer le bouton « Terminer la pause » et sa légende budget |
| `SortieUrgence.dc.html` | Légende : « Il reste 6 minutes. La pause sera comptée interrompue. » Au lancement, aucune mention de budget ni de dette ; quand §9.2 entre : « … et créditées à ta dette de posture » |
| `ReglagesApplications.dc.html` | Supprimer les blocs « Déclencheur intelligent » et « Exceptions automatiques » et la colonne DÉCLENCHEUR. La note sur la visio en onglet disparaît (elle ne servait qu'à l'inhibition) |
| `ReglagesStatistiques.dc.html` | « 11 Reports — 55 min débitées » → « 11 Interrompues — 55 min non servies » ; « 1 Sortie d'urgence » reste (compté à part comme le brief SANS §12.3 le demande) |
| `Fondations.dc.html` | Icône : retirer « INACTIF / ARMÉ » → « INACTIF » ; retirer « PAUSE DUE, INHIBÉE » ; ajouter « PAUSE DUE, EN ATTENTE » (retenue §9.3, post-lancement). Exemple de raison de grisement : « Le budget ne couvre pas… » → « Une pause est due — réglable au cycle suivant » |
| `Permissions.dc.html` | L'écran **disparaît** de l'onboarding (cinq écrans). La demande d'Accessibilité migre sur `Severite.dc.html` au choix « Simple ». Le contenu de l'écran survit dans Réglages › Permissions |
| `Bienvenue`, `Severite`, `Demarrage` | Retouches cross-platform de D2 (« ton Mac » → « ta machine », « macOS te laisse toujours forcer à quitter » → « ton système te laisse toujours… », « barre de menus » → « barre de menus ou zone de notification ») |

- **Ce que le domaine doit quand même porter dès le lancement** pour que §9.2 (dette) entre sans
  refonte : le **sort de chaque pause** (`Served`, `ValidatedByAbsence`, `Interrupted { unserved_minutes }`)
  et la porte d'interruption (`Quit`, `TrayMenu`, `HardcoreExitGesture`, `SuspensionOverrun`,
  `Crash`). C'est un enregistrement, pas un comportement : rien n'escalade au lancement.

### Ce qui est déclaré dégradé

Rien côté produit. L'hypothèse risquée reste celle du §17 du brief SANS : *zéro négociation
produit-il plus de désinstallations qu'un budget borné ?* Elle se mesure en bêta (part de Hardcore
en première semaine, installations actives à 30 jours). Aucune soupape n'est réintroduite « au cas
où » : si la mesure est défavorable, c'est un nouveau brief, pas un bouton.

---

## D2 — Réconciliation cross-platform : macOS, Windows, Linux (X11 et Wayland)

### Contexte

Le brief SANS écrit « Ce n'est pas une application multiplateforme » (§3) et classe Windows/Linux
en « contrainte subie, ne se révise pas » (§14). Le fondateur impose désormais les trois OS. Il faut
donc remplacer le §5 « Le cadre imposé par macOS » par un cadre **par mécanisme et par plateforme**,
sans que l'âme du produit (rendre la triche coûteuse et consciente, jamais impossible ; dire ce
qu'on ne garantit pas *avant* que l'utilisateur le découvre) ne change.

Un principe d'abord, qui rend le reste possible : **la promesse de Breeze ne dépend d'aucune
détection.** Le cycle tourne sans lire quoi que ce soit. Chaque capacité perdue ne fait qu'une
chose : soit le décompte avance plus vite (côté sûr), soit la couverture visuelle est moins
complète — et dans ce second cas, Breeze le dit à froid.

### Décision

**Breeze est livré sur macOS 13+, Windows 10 (1809)+/11, Linux X11, et Linux Wayland sur les
compositeurs qui exposent les protocoles nécessaires (KDE Plasma ≥ 5.20, wlroots : Sway, Hyprland,
river…). Sur GNOME Wayland, Breeze tourne en mode déclaré dégradé.** Chaque adaptateur d'OS
rapporte au domaine un **relevé de capacités** (`EnforcementCapabilities`) au démarrage et à chaque
changement (permission retirée, session passée de X11 à Wayland), et le domaine choisit le mode
d'une pause à son premier instant — la règle d'ancrage du §10.5 est conservée telle quelle, elle
est simplement généralisée de « l'Accessibilité manque » à « une capacité manque ».

#### Tableau des capacités par mécanisme

Légende : **Fiable** = API publique, stable, sans permission ou avec permission déclarée ·
**Partiel** = fonctionne avec une limite connue et déclarée · **Non garanti** = dépend du
compositeur/de l'appli tierce, déclaré dégradé · **Impossible** = pas d'API, repli obligatoire.

**A. Détection de l'application au premier plan (identité seulement, jamais le titre)**

| OS | Moyen | Verdict | Ce qu'on ne lit jamais |
|---|---|---|---|
| macOS | `NSWorkspace.shared.frontmostApplication` + notification `didActivateApplication` (bundle id, pid). Aucune permission | **Fiable** | Titre (`kCGWindowName`, exigerait Enregistrement d'écran) |
| Windows | `GetForegroundWindow` → `GetWindowThreadProcessId` → `QueryFullProcessImageNameW` ; suivi par `SetWinEventHook(EVENT_SYSTEM_FOREGROUND)`. Aucune permission | **Fiable** (limite : processus élevés/UAC renvoient accès refusé → « inconnu ») | `GetWindowText` n'est jamais appelé |
| Linux X11 | `_NET_ACTIVE_WINDOW` → `_NET_WM_PID` → `/proc/<pid>/exe` ; `PropertyNotify` sur la racine | **Fiable** sur un WM conforme EWMH (tous les courants) | `_NET_WM_NAME` n'est jamais lu |
| Linux Wayland — wlroots | `wlr-foreign-toplevel-management-unstable-v1` (app_id, état `activated`) | **Fiable** là où le protocole est exposé | Le protocole expose aussi `title` : l'adaptateur ne s'y abonne pas |
| Linux Wayland — KDE | `org_kde_plasma_window_management` (app_id, actif) | **Fiable** | idem |
| Linux Wayland — GNOME | Aucun protocole public ; `org.gnome.Shell.Introspect` est restreint aux applis autorisées | **Impossible** | — |

**Repli déclaré** (déjà dans le brief SANS §12.3, ligne « lire l'application au premier plan ») :
quand l'identité est inconnue, **tout usage compte comme du travail**. Le statut `ignorée` n'a
alors aucun effet et Réglages › Applications l'affiche : « Sur cette session, Breeze ne voit pas
quelle application est devant : tout compte comme du travail. » Le côté sûr est celui où le
décompte avance plus vite.

**B. Overlay Mode Simple (suivre le cadre des fenêtres d'applications bloquées)**

| OS | Moyen | Verdict |
|---|---|---|
| macOS | `AXUIElement` (position, taille, `AXWindows`) avec permission **Accessibilité** ; polling ≤ 100 ms + `AXObserver` (`kAXMovedNotification`, `kAXResizedNotification`). Une fenêtre Breeze `NSPanel` non activante par fenêtre cible, niveau `.floating`, `ignoresMouseEvents=false` | **Fiable avec permission**, contournable en déplaçant une fenêtre épargnée par-dessus (assumé dans les deux briefs) |
| Windows | `EnumWindows` + `GetWindowThreadProcessId` + `DwmGetWindowAttribute(DWMWA_EXTENDED_FRAME_BOUNDS)` + `IsWindowVisible`/`DWMWA_CLOAKED` ; suivi par `SetWinEventHook(EVENT_OBJECT_LOCATIONCHANGE, EVENT_SYSTEM_MINIMIZE*)`. Une fenêtre `WS_EX_LAYERED|WS_EX_TOOLWINDOW|WS_EX_TOPMOST` par cible. **Aucune permission** | **Fiable**, même contournement. Limite déclarée : une application lancée « en tant qu'administrateur » n'est pas énumérable → non voilée |
| Linux X11 | `_NET_CLIENT_LIST_STACKING`, `XGetWindowAttributes`/`XTranslateCoordinates`, `_NET_FRAME_EXTENTS` ; fenêtres override-redirect ou `_NET_WM_STATE_ABOVE`. Aucune permission | **Fiable** sur WM EWMH ; **Partiel** avec compositeurs qui recomposent (fenêtres sur plusieurs bureaux virtuels : suivre `_NET_CURRENT_DESKTOP`) |
| Linux Wayland (tous compositeurs) | Aucun protocole public n'expose la géométrie des fenêtres d'autrui ; `wlr-foreign-toplevel` donne l'état, pas la position | **Impossible** |

**Repli déclaré** (règle §10.5 du brief SANS, généralisée) : quand les cadres ne sont pas
observables, le Mode Simple pose **un voile unique plein écran par moniteur**, avec le décompte,
sans masquer ni barre système ni raccourcis, sans geste de sortie — exactement l'overlay dégradé
existant. Sur Wayland, ce repli est **le comportement nominal**, affiché à froid dès l'onboarding :
« Sur Wayland, le Mode Simple voile tout l'écran plutôt que des fenêtres précises. »

**C. Overlay Mode Hardcore (couvrir tous les écrans, au-dessus de tout)**

| OS | Moyen | Verdict | Limite déclarée |
|---|---|---|---|
| macOS | Une `NSWindow` par `NSScreen`, `level = CGShieldingWindowLevel()`, `collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary, .stationary]`, `NSApp.presentationOptions = [.hideDock, .hideMenuBar, .disableProcessSwitching]` — **jamais** `.disableForceQuit` ni `.disableSessionTermination`. Aucune permission | **Fiable** hors plein écran natif | Appli tierce en plein écran natif : **mesure §12.3**, repli « dit à froid ». Cmd-Q de Breeze reste possible (décision 5 SANS) |
| Windows | Une fenêtre `WS_POPUP` par moniteur (`EnumDisplayMonitors`), `WS_EX_TOPMOST|WS_EX_TOOLWINDOW`, taille = moniteur, `SetForegroundWindow` + ré-assertion `HWND_TOPMOST` sur chaque `EVENT_SYSTEM_FOREGROUND` (≤ 500 ms, constante §12.2). Aucune permission | **Fiable** pour tout ce qui est fenêtré | (1) Alt-Tab, Win, Ctrl-Alt-Suppr **ne sont pas neutralisés** : les bloquer exige un hook clavier bas niveau (`WH_KEYBOARD_LL`) qui *lit les touches*, ce que §10.6/§13 interdisent ; le menu Démarrer peut s'ouvrir par-dessus, l'overlay revient dessous dans les 500 ms. (2) Une appli en **plein écran exclusif** (jeux DirectX) ou le **bureau sécurisé** (UAC, Win+L) passent au-dessus. (3) Une autre fenêtre topmost lancée après la nôtre peut passer devant jusqu'à la ré-assertion |
| Linux X11 | Une fenêtre **override-redirect** par sortie XRandR, `XRaiseWindow` + ré-assertion sur `ConfigureNotify`/`MapNotify` de la racine ; `XGrabKeyboard` **non** utilisé (ce serait une capture de frappe) | **Fiable** sur WM EWMH | Un WM peut re-empiler ; un autre override-redirect (menu contextuel, notification) peut passer devant. Alt-Tab **non neutralisé** (même raison que Windows) |
| Linux Wayland — wlroots, KDE | `zwlr_layer_shell_v1`, couche `overlay`, une surface par `wl_output`, ancrée aux quatre bords, `exclusive_zone = -1`, `keyboard_interactivity = exclusive` (wlroots) / `on_demand` (KDE) | **Fiable** sur ces compositeurs | `exclusive` est refusé ou ignoré par certains compositeurs → Alt-Tab possible. Déclaré : « ton compositeur peut laisser passer un raccourci » |
| Linux Wayland — GNOME | Pas de layer-shell. Repli : une `xdg_toplevel` **plein écran** par sortie, avec `set_fullscreen(output)` | **Non garanti** : GNOME peut placer une autre fenêtre devant, l'utilisateur peut Super/Alt-Tab, le shell reste accessible | Déclaré à froid : « Sur GNOME Wayland, le Mode Hardcore n'est pas un mode Hardcore. Il couvre chaque écran mais ne tient pas devant le système. » Le choix « Hardcore » y reste offert, avec cette légende sous le bouton — jamais grisé sans raison |
| Tous | Aucun OS ne laisse Breeze empêcher : forcer à quitter, changer de session, éteindre, débrancher | Rappel §5 : ce sont des sorties qui **doivent** rester |

**D. Signaux veille / verrouillage / inactivité**

| Signal | macOS | Windows | Linux X11 | Linux Wayland |
|---|---|---|---|---|
| Inactivité (instant de la dernière action, jamais son contenu) | `CGEventSourceSecondsSinceLastEventType(.combinedSessionState, .null)` — sans permission | `GetLastInputInfo` — sans permission | `XScreenSaverQueryInfo` (extension MIT-SCREEN-SAVER) | `ext-idle-notify-v1` (wlroots, KDE ≥ 5.27) ; GNOME : `org.gnome.Mutter.IdleMonitor.GetIdletime` (D-Bus) ; repli `org.freedesktop.ScreenSaver.GetSessionIdleTime` — **Fiable partout** |
| Veille / réveil | `NSWorkspace.willSleepNotification` / `didWakeNotification` | `WM_POWERBROADCAST` (`PBT_APMSUSPEND` / `PBT_APMRESUMEAUTOMATIC`) | `org.freedesktop.login1.Manager.PrepareForSleep` (D-Bus) | idem logind — **Fiable** ; sans logind (systèmes sans systemd) : **Partiel**, la veille est déduite d'un saut d'horloge monotone (traitement identique à une absence, §10.4) |
| Verrouillage / déverrouillage | Notification distribuée `com.apple.screenIsLocked` / `screenIsUnlocked` — **non documentée mais stable depuis 10.x** ; déclarée mesure | `WTSRegisterSessionNotification` → `WM_WTSSESSION_CHANGE` (`WTS_SESSION_LOCK/UNLOCK`) | `org.freedesktop.login1.Session` `Lock`/`Unlock` + `LockedHint` | idem ; GNOME : aussi `org.gnome.ScreenSaver.ActiveChanged` |
| Changement rapide d'utilisateur | `NSWorkspace.sessionDidResignActiveNotification` | `WTS_CONSOLE_DISCONNECT` / `WTS_REMOTE_*` | `org.freedesktop.login1.Session.Active` | idem |

Le domaine ne reçoit que trois événements normalisés : `ActivityObserved(at)`, `SessionState(Awake
| Asleep | Locked | Inactive)`, et un `ClockJump`. Le verdict d'absence (§10.4) ne change pas d'un
OS à l'autre : **c'est la durée cumulée qui compte, jamais sa cause**.

**E. Notifications, démarrage automatique, barre de menus, persistance, mise à jour**

| Besoin | macOS | Windows | Linux |
|---|---|---|---|
| Préavis quand la bannière propre ne peut pas se poser | `UNUserNotificationCenter` (permission demandée seulement dans ce cas, §8.1 SANS) | Toast WinRT (`Windows.UI.Notifications`), sans permission | `org.freedesktop.Notifications` (D-Bus), sans permission |
| Démarrage auto | `SMAppService.mainApp` (13+) | Clé `HKCU\…\Run` | `~/.config/autostart/breeze.desktop` (XDG) |
| Icône d'état | Barre de menus (`NSStatusItem`), sans icône Dock (`LSUIElement`) | Zone de notification (`Shell_NotifyIcon`) | X11 et KDE/wlroots : `StatusNotifierItem` (D-Bus). **GNOME : pas de zone d'état sans extension AppIndicator** — déclaré : « Sur GNOME, installe l'extension AppIndicator, sinon Breeze s'ouvre par sa fenêtre » ; une fenêtre de panneau reste ouvrable par raccourci de lancement de l'appli |
| Persistance (échéances murales, sort des pauses, réglages, journal 7 j / 5 Mo, stats 30 j) | `~/Library/Application Support/Breeze/` | `%APPDATA%\Breeze\` | `$XDG_DATA_HOME/breeze/` — fichier SQLite unique, écriture atomique (journal WAL) |
| Vérification de mise à jour (seule sortie réseau, désactivable) | Mise à jour signée + notariée, **même identité de signature** (§5) | Installeur signé (Authenticode), MSI ou NSIS | AppImage + `.deb`/`.rpm` ; la vérification est la même, l'application se fait par le gestionnaire de paquets quand il y en a un |
| Horloge | `mach_absolute_time` (monotone) + horloge murale | `QueryPerformanceCounter` + `GetSystemTimePreciseAsFileTime` | `CLOCK_MONOTONIC` / `CLOCK_BOOTTIME` + `CLOCK_REALTIME` — nécessaire au §10.7 (recul d'horloge) |

#### Le repli déclaré, plateforme par plateforme (ce que Breeze dit à froid)

Le brief réécrit remplace le §5 par un tableau « Ce que ta machine impose » et l'onboarding affiche
**une seule phrase** sous le choix de sévérité, propre à la session détectée :

| Session | Phrase à froid |
|---|---|
| macOS | « Aucun mode n'est infaillible — macOS te laisse toujours forcer à quitter ou éteindre. Une application en plein écran natif peut passer devant le Mode Hardcore. » (inchangé, plus la seconde phrase si la mesure §12.3 est défavorable) |
| Windows | « Aucun mode n'est infaillible — Windows te laisse toujours Alt-Tab, le menu Démarrer, Ctrl-Alt-Suppr. Le Mode Hardcore revient devant en moins d'une demi-seconde, il ne te bloque pas. Un jeu en plein écran exclusif passe devant. » |
| Linux X11 | « Aucun mode n'est infaillible — ton gestionnaire de fenêtres garde le dernier mot sur l'empilement. Le Mode Hardcore revient devant en moins d'une demi-seconde. » |
| Linux Wayland (KDE, wlroots) | « Le Mode Simple voile tout l'écran plutôt que des fenêtres précises : Wayland ne montre pas où sont les fenêtres des autres. Le Mode Hardcore tient devant tout ce que ton compositeur laisse recouvrir. » |
| Linux Wayland (GNOME) | « Sur GNOME Wayland, Breeze ne voit pas quelle application est devant, ne peut pas voiler des fenêtres précises, et le Mode Hardcore ne tient pas devant le système. Il couvre tes écrans, et c'est tout. » |

Ce qui est écarté : (1) toute promesse « au-dessus de tout » dans la présentation du produit, sur
n'importe quel OS ; (2) tout hook clavier bas niveau pour neutraliser Alt-Tab/Win/Super — il lit
les touches, il est interdit par §10.6/§13 et il change le profil de sécurité du binaire ;
(3) toute exécution privilégiée (service Windows, `setuid`, daemon root) pour « tenir mieux » —
Breeze reste une application utilisateur ordinaire, celui qui subit la contrainte est celui qui
l'a posée ; (4) XWayland comme mode de fonctionnement sur Wayland : une fenêtre X11 ne passe
jamais devant les fenêtres Wayland natives, et Breeze y serait dégradé *sans le savoir*.

### Conséquences

- Le §3 (« pas multiplateforme ») et la ligne §14 « Windows et Linux » du brief sont **retirés** ;
  le §5 devient « Ce que ta machine impose », par mécanisme, avec le tableau ci-dessus condensé.
- Le §12.1 liste les versions couvertes par OS. Le §12.3 (mesures techniques) reçoit six lignes
  nouvelles : plein écran exclusif Windows ; `keyboard_interactivity=exclusive` par compositeur ;
  notification `screenIsLocked` par version macOS ; comportement de GNOME face à deux `xdg_toplevel`
  plein écran ; ré-assertion topmost < 500 ms sous charge ; présence de logind.
- Le §10.5 devient « Ce que Breeze fait quand une capacité manque » : la table gagne trois lignes
  (identité inconnue, cadres inobservables, couche overlay refusée) et garde sa règle d'ancrage au
  premier instant de la pause.
- Le §10.6 (liste de sécurité) devient **par OS** : macOS (les sept du brief SANS) ; Windows
  (Paramètres, Gestionnaire des tâches, Explorateur, Terminal/PowerShell, Narrateur, écran de
  connexion) ; Linux (l'émulateur de terminal par défaut, le centre de contrôle du bureau, le
  moniteur système, le lecteur d'écran Orca). Toujours par identité d'exécutable, jamais par
  catégorie.
- Le coupe-circuit (3 chutes en 5 min) prend un sens supplémentaire sur Linux : un compositeur qui
  tue une surface layer-shell mal formée compte comme une chute. Il protège aussi contre nos
  adaptateurs.
- Le budget de mémoire (< 120 Mo au repos, §12.2) est conservé sur les trois OS ; il devient un
  critère de D3.

### Ce qui est déclaré dégradé

Récapitulé pour être copié dans le brief :

| Plateforme | Détection premier plan | Mode Simple par fenêtre | Mode Hardcore au-dessus de tout | Icône d'état |
|---|---|---|---|---|
| macOS 13+ | Fiable | Fiable (Accessibilité) | Fiable, **sauf plein écran natif (mesure)** | Fiable |
| Windows 10/11 | Fiable | Fiable | Partiel : **Alt-Tab/Win non neutralisés, retour < 500 ms ; plein écran exclusif et bureau sécurisé passent devant** | Fiable |
| Linux X11 | Fiable (EWMH) | Fiable (EWMH) | Partiel : **empilement au WM, retour < 500 ms ; Alt-Tab non neutralisé** | Fiable (SNI) ; GNOME : extension |
| Linux Wayland KDE / wlroots | Fiable | **Impossible → voile plein écran** | Fiable via layer-shell ; **raccourcis du compositeur non garantis** | Fiable (SNI) |
| Linux Wayland GNOME | **Impossible → tout compte** | **Impossible → voile plein écran** | **Non garanti → plein écran ordinaire** | **Extension requise** |

---

## D3 — Stack technique : Tauri v2 (cœur Rust, webview, plugins officiels)

### Contexte

Le dépôt a commencé un scaffold Electron. Le design est en HTML/CSS (liquid glass : `backdrop-filter`,
dégradés, verre teinté), ce qui rend les deux options crédibles. Les critères imposés : fiabilité de
l'always-on-top/kiosk sur les trois OS, overlay tri-plateforme, poids/mémoire (< 120 Mo au repos,
§12.2), accès natif (premier plan, énumération d'écrans, layer-shell), maturité, et ce que D2 vient
d'imposer : **pas de XWayland**, pas de hook clavier, une identité de signature stable.

### Décision

**Tauri v2.** Cœur Rust dans un espace de travail Cargo, interface en TypeScript sans framework
lourd (le design existe déjà en HTML/CSS ; il devient des composants web natifs ou un Svelte
minimal — le choix du micro-framework est du ressort du cadrage d'interface, pas de ce registre).

Critère par critère :

| Critère | Tauri v2 | Electron | Verdict |
|---|---|---|---|
| Always-on-top / kiosk | `always_on_top`, `visible_on_all_workspaces`, `fullscreen`, `decorations=false`, `skip_taskbar` intégrés ; **le niveau `CGShieldingWindowLevel`, `presentationOptions`, `WS_EX_TOOLWINDOW`, override-redirect et layer-shell se posent dans le même processus Rust** via `objc2-app-kit`, `windows`, `x11rb`, `gtk-layer-shell` (Tauri v2 Linux repose sur GTK3 + WebKitGTK ; `gtk-layer-shell` s'applique à sa `GtkWindow` avant `map`) | `setAlwaysOnTop(true, 'screen-saver')`, `kiosk`, `setVisibleOnAllWorkspaces` mûrs et éprouvés sur macOS/Windows ; **Linux : Electron tourne sous XWayland par défaut** (`--ozone-platform=wayland` existe, mais aucun accès layer-shell sans addon natif) ; tout ce qui dépasse l'API Electron exige un addon N-API C++/Rust dans un processus séparé de la logique | **Tauri** — égalité sur macOS/Windows, avantage net sur Wayland, et une seule frontière (Rust ↔ webview) au lieu de deux (JS ↔ N-API ↔ natif) |
| Overlay tri-plateforme | Une `WebviewWindow` par écran, transparente, `shadow=false`, `focus`, `set_ignore_cursor_events` ; sur Linux, transparence conditionnée au compositeur (comme Electron) | Une `BrowserWindow` par écran ; **chaque fenêtre = un processus de rendu** (30–60 Mo chacun) | **Tauri** — trois écrans Hardcore + panneau + bannière = 5 fenêtres ; Electron dépasse 120 Mo avant même le cycle |
| Poids / mémoire | Binaire 8–20 Mo ; au repos (icône + aucune fenêtre) de l'ordre de 20–60 Mo selon OS, à **mesurer** (WebKitGTK est le plus lourd des trois) | Bundle 150–250 Mo ; au repos ≥ 80–120 Mo (main + GPU + un renderer), **à la limite ou au-dessus du budget §12.2** avant la première fenêtre | **Tauri** |
| Accès natif : premier plan, écrans, idle, lock, layer-shell | Directement en Rust dans le processus hôte, testable par crate | Via addons N-API à compiler par OS et par version d'ABI Electron, ou `ffi-napi` (abandonné) | **Tauri** |
| Maturité | v2 stable depuis 2024 ; plugins officiels `autostart`, `notification`, `single-instance`, `updater`, `store`/`sql`, tray intégré ; communauté plus petite, bugs de fenêtrage encore ouverts sur Linux | Très mûr, 10 ans de kiosques ; comportement de fenêtre connu et documenté ; équipe importante | **Electron** — c'est le seul critère qu'il gagne |
| Design HTML/CSS | WebKit (macOS), WebView2/Chromium (Windows), WebKitGTK (Linux) : `backdrop-filter` supporté partout ; **trois moteurs, donc trois rendus du verre à vérifier** | Chromium partout, un seul rendu | **Electron** sur l'homogénéité, **égalité** sur la faisabilité |
| Signature / notarisation | Un exécutable, quelques dylibs ; identité stable ; pas de helpers | Cinq helpers `.app` à signer avec la même identité ; notarisation éprouvée | égalité, Tauri plus simple |
| Sécurité / profil « logiciel malveillant » | Pas de Node dans le rendu, capacités explicitement déclarées par fenêtre (`capabilities/*.json`), isolation par défaut | `contextIsolation` + `sandbox` bien réglés = équivalent, mais tout est à activer | **Tauri** |
| Temps réel des échéances | Le planificateur tourne en Rust (`tokio` timers + horloge monotone), indépendant du webview — une fenêtre gelée ne gèle pas le cycle | Le main process Node tient les timers ; correct, mais un renderer bloqué et le GC partagent le même hôte JS | **Tauri** |

Ce qui est écarté : le scaffold Electron du dépôt (supprimé, pas conservé « au cas où » — deux
hôtes de fenêtrage, c'est deux fois les bugs) ; Flutter desktop et Qt (aucun avantage sur l'overlay,
le design HTML serait à refaire) ; une application native par OS (trois bases de code pour un
fondateur seul).

### Conséquences

- **Espace de travail Cargo** avec un crate de domaine sans dépendance (`breeze-domain`), un crate
  d'application (`breeze-app`), les ports (`breeze-ports`), un adaptateur par OS, et le hôte Tauri
  (`src-tauri`) qui ne fait que composer. L'arbre est en D4.
- **Interface** : les 17 artboards deviennent des vues ; le CSS de `Fondations.dc.html` devient la
  feuille de tokens. Le verre est vérifié sur les trois moteurs ; le repli sans `backdrop-filter`
  (compositeur Linux sans transparence) est un fond opaque `#F4F6FB` — déjà le « fond de pause »
  des Fondations.
- **Plugins Tauri retenus** : `tauri-plugin-single-instance` (§10.7 « deuxième instance »),
  `tauri-plugin-autostart`, `tauri-plugin-notification`, `tauri-plugin-updater` (vérification
  désactivable, jamais appliquée pendant une pause due — c'est l'application qui décide de l'instant,
  pas le plugin), tray intégré. **Aucun plugin pour la persistance** : `rusqlite` derrière le port,
  pour garder le schéma et l'écriture atomique sous contrôle.
- **Ce qui est écrit à la main en Rust** (les endroits où Tauri ne suffit pas) : niveau de fenêtre
  et `presentationOptions` (macOS), `WS_EX_TOOLWINDOW` + ré-assertion topmost (Windows),
  override-redirect (X11), `gtk-layer-shell` (Wayland), énumération des cadres (A/B de D2),
  signaux de session (D de D2). Chacun vit dans son crate d'adaptateur et ne remonte au domaine que
  par un port.
- **Mesures à faire avant d'écrire une fonctionnalité** (§12.3, nouvelles lignes) : mémoire au
  repos par OS avec icône seule et avec trois overlays ; rendu du verre sur WebKitGTK ; délai de
  pose < 200 ms d'un overlay Hardcore sur trois écrans ; notarisation d'un binaire Tauri portant
  `CGShieldingWindowLevel` et l'Accessibilité (**bloquant**, comme dans le brief).

### Ce qui est déclaré dégradé (les risques du choix, et du perdant)

- **Risque principal Tauri** : les bugs de fenêtrage Linux (focus, transparence, multi-moniteur
  sous GTK3) sont réels et la communauté est plus petite ; l'absence d'un comportement kiosque
  « clé en main » oblige à posséder ce code. Mitigation : les adaptateurs sont des crates isolés,
  testés à la main sur une matrice de compositeurs, derrière le coupe-circuit.
- **Risque secondaire** : trois moteurs de rendu. Mitigation : aucune dépendance CSS hors la liste
  supportée par WebKitGTK 2.40+, WebView2 et WebKit 16+ ; le verre a un repli opaque.
- **Ce qu'on perd avec Electron** : la certitude d'un `setAlwaysOnTop('screen-saver')` qui marche
  le premier jour sur macOS et Windows, et un rendu unique. Si en cadrage la mesure de mémoire
  Tauri sur Linux dépassait 120 Mo au repos, la réponse serait de revoir le budget pour Linux, pas
  de revenir à Electron — qui le dépasserait aussi.

---

## D4 — Architecture : domaine pur, neuf ports de pont système, un adaptateur par OS

### Contexte

Charpente impose un cœur pur (aucune dépendance système), des ports nommés, des adaptateurs aux
bords, le domaine et les données en anglais, les artefacts et la doc en français. L'historique du
dépôt déclarait neuf ports de « system-bridge » : foreground-app, window-frames, overlay-surfaces,
display-enumeration, idle/sleep/lock, notifications, autostart, persistence, update-check. D1 fixe
la machine à états (brief SANS §10) ; D2 ajoute le relevé de capacités ; D3 fixe Rust.

### Décision

**Un espace de travail Cargo à cinq couches : `breeze-domain` (pur), `breeze-ports` (traits),
`breeze-app` (cas d'usage et planificateur), un crate d'adaptateur par plateforme, et `src-tauri`
qui compose.** L'interface web ne connaît que des projections (`CycleSnapshot`) et des commandes
(`Command`) ; elle ne prend aucune décision métier.

#### Le domaine pur (`breeze-domain`)

Aucune dépendance hors `core`/`alloc` (et `serde` derrière une feature pour les instantanés).
Le temps est **injecté** : le domaine reçoit `Instant` (monotone) et `WallClock` (mural) en
paramètre de chaque transition, jamais lus.

```
CycleState
  Inactive                              // hors plage horaire ou jour inactif
  Working { countdown: Countdown }      // Countdown = Running{deadline} | Frozen{remaining} | Due{since}
  Notice  { deadline }                  // PRÉAVIS, 1 minute, fixe
  BreakActive { deadline, severity, mode: BreakMode }   // mode = Nominal | Degraded(reasons)
  Returning { deadline }                // RETOUR, 3 s

Overlay states (se superposent) :
  Suspension { chain_started_at, remaining_work_at_start, until }   // SUSPENDU
  SessionState = Awake | Asleep | Locked | OtherUserActive            // VEILLE et changement de session

Extension déclarée, inactive au lancement :
  Working { countdown: Due { since, held_until: Option<Instant> } } // retenue §9.3
```

Valeurs et règles portées par le domaine, sans exception :

- `Severity = Simple | Hardcore`, `AppStatus = Blocked | Spared | Ignored`, `SafetyListed`.
- `Rhythm { work: Minutes(5..=180), pause: Minutes(1..=60), pause <= work, schedule: Option<TimeRange>, active_days: NonEmptySet<Weekday> }` — les trois refus de saisie §8.2 sont des erreurs typées (`RhythmError::NoActiveDay`, `DegenerateRange`, `PauseLongerThanWork`).
- **Règle du sens** (§10.3) : `PendingSettings` porte les changements qui affaiblissent jusqu'au cycle suivant ; ceux qui renforcent s'appliquent immédiatement ; tous sont refusés pendant `Notice`, `BreakActive`, `Returning` (`SettingsError::BreakDue`).
- **Sort de la pause** (§10.2) : `BreakOutcome = Served | ValidatedByAbsence | Interrupted { unserved: Minutes, door: InterruptionDoor }` ; `InterruptionDoor = Quit | TrayMenu | HardcoreExitGesture | SuspensionOverrun | Crash`. Une chute n'est jamais créditée individuellement ; elle alimente `CrashCounter` (3 en 5 min → `Breaker::Armed { until }`).
- **Verdict d'absence** (§10.4) : `AbsenceVerdict::judge(state, absence: Duration, now) -> Transition` — plus longue que la pause pendant `Working`/`Notice` → cycle validé ; plus longue que le restant pendant `BreakActive`/`Returning` → pause servie ; sinon l'échéance produit son effet, préavis sauté. Le temps sans processus n'entre jamais dans `absence`.
- **Suspension** (§10.1) : chaîne, référence au premier « Suspendre », dépassement → une seule pause `Interrupted { door: SuspensionOverrun }`.
- **Horloge** (§10.7) : échéances murales comparées au **temps réellement écoulé** (monotone) ; un `ClockJump::Backward` ne recule aucune échéance ; un `Forward` les rapproche.
- **Capacités** (D2) : `EnforcementCapabilities { foreground: Reliable|Unknown, frames: Observable|Unobservable, overlay: Layered|BestEffort|PlainFullscreen, tray: Available|Missing }` — évalué **au premier instant de `BreakActive`** avec l'état de la permission, pour fixer `BreakMode` une fois pour toutes (règle §10.5 conservée, généralisée).
- **Dette de posture** (§9.2, post-lancement) : `PostureDebt` est un module déclaré mais **non câblé** ; le ledger des `BreakOutcome` existe dès le lancement pour qu'il n'y ait rien à migrer.

Le domaine est testé **sans aucun double** : des scénarios temporels (`at(t0) … advance(50min) … expect(Notice)`) et des propriétés (« aucune commande n'écourte une pause », « une pause due n'est jamais coupée par l'horloge », « le breaker prime sur l'escalade »).

#### Les ports (`breeze-ports`) — neuf ports de pont système, un port d'horloge

Traits Rust, nommés en anglais, un fichier chacun. Le domaine n'en importe **aucun** ; c'est
`breeze-app` qui les consomme.

| Port | Sorties (vers le domaine) | Entrées (depuis l'application) | Capacité rapportée |
|---|---|---|---|
| `ForegroundAppPort` | `ForegroundChanged { app: AppIdentity, at }` | `current() -> Option<AppIdentity>` | `Reliable \| Unknown` |
| `WindowFramesPort` | `FramesChanged { app, frames: Vec<Rect> }` | `frames_of(app) -> Vec<Rect>` | `Observable \| Unobservable` |
| `OverlaySurfacesPort` | `SurfaceLost { id }` (surface tuée par le compositeur) | `cover_display(display, kind: Hardcore\|Veil) -> SurfaceId`, `cover_rect(rect, kind)`, `move_surface`, `dismiss_all()` | `Layered \| BestEffort \| PlainFullscreen` |
| `DisplayEnumerationPort` | `DisplaysChanged { displays: Vec<Display> }` | `displays() -> Vec<Display>` | — |
| `SessionSignalsPort` (idle / sleep / lock) | `Activity { last_input_at }`, `SessionChanged(SessionState)` | `seconds_since_last_input() -> Duration` | `Reliable \| Inferred` (sans logind) |
| `NotificationsPort` | `PermissionChanged(bool)` | `notify(Notice)`, `request_permission()` | `Granted \| Denied \| NotApplicable` |
| `AutostartPort` | — | `enable()`, `disable()`, `is_enabled()` | — |
| `PersistencePort` | — | `load() -> Option<Snapshot>`, `save(Snapshot)` (atomique), `append_journal(Event)`, `stats_window(30d)`, `export()` | — |
| `UpdateCheckPort` | `UpdateAvailable { version }` | `check()`, `apply_now()`, `set_enabled(bool)` | — |
| `ClockPort` (port du domaine, pas du pont système) | — | `monotonic() -> Instant`, `wall() -> WallClock`, `subscribe_jumps()` | — |

Deux choses ne sont **pas** des ports : l'icône d'état et l'instance unique. Elles ne portent
aucune décision du domaine ; ce sont des adaptateurs de l'enveloppe Tauri (`shell/`), qui
consomment `CycleSnapshot` et émettent des `Command`. Le relevé `tray: Available|Missing` remonte
quand même dans `EnforcementCapabilities` parce que le brief doit pouvoir le **dire** à froid.

Le port d'Accessibilité macOS n'est pas un dixième port : c'est la capacité `frames` de
`WindowFramesPort`, et `request_permission()` sur ce même port. Un port par **besoin du produit**,
pas par API d'OS.

#### Les adaptateurs par OS

| Crate | Implémente | Avec |
|---|---|---|
| `breeze-bridge-macos` | les neuf ports | `objc2-app-kit`, `objc2-foundation`, `core-graphics`, `accessibility-sys` (AX), `SMAppService`, `UNUserNotificationCenter` |
| `breeze-bridge-windows` | les neuf ports | crate `windows` (Win32 UI/WindowsAndMessaging, Dwm, Wts, Power), WinRT notifications |
| `breeze-bridge-linux-x11` | foreground, frames, overlay, displays, session (X11 + logind) | `x11rb` (EWMH, RandR, MIT-SCREEN-SAVER), `zbus` (logind, Notifications, SNI) |
| `breeze-bridge-linux-wayland` | foreground (`wlr-foreign-toplevel` / `plasma-window-management` / **none**), overlay (`gtk-layer-shell` / plein écran), session (`ext-idle-notify` / Mutter / logind) | `wayland-client`, `wayland-protocols-wlr`, `wayland-protocols-plasma`, `gtk-layer-shell`, `zbus` |
| `breeze-bridge-common` | `PersistencePort` (rusqlite, WAL, écriture atomique), `UpdateCheckPort` (sur `tauri-plugin-updater`), `AutostartPort` (sur `tauri-plugin-autostart`), `ClockPort` (`std::time::Instant` + `SystemTime`, détection de saut) | partagé par les quatre |
| `breeze-bridge-null` | les neuf ports en **non-op déclaré** (`Unknown`/`Unobservable`/`PlainFullscreen`) | pour les tests d'application et pour le coupe-circuit (`Breaker::Armed` = swap vers `OverlaySurfacesPort` nul) |

Le choix de l'adaptateur Linux se fait **au lancement** sur `XDG_SESSION_TYPE` et sur la présence
effective des globals Wayland (`zwlr_layer_shell_v1`, `zwlr_foreign_toplevel_manager_v1`,
`org_kde_plasma_window_management`) — jamais sur le nom du bureau.

#### L'application (`breeze-app`)

Cas d'usage en commandes et requêtes séparées (CQRS léger, pas de bus) : `StartCycle`,
`Suspend(Duration)`, `Resume`, `ChangeSeverity`, `ChangeRhythm`, `SetAppStatus`, `QuitRequested`,
`HardcoreExitGestureCompleted` ; requêtes : `GetSnapshot`, `GetTodaySummary`, `GetStats30d`. Le
**planificateur** (`Scheduler`) est la seule boucle : il réveille le domaine aux échéances (timer
monotone), sur chaque événement de port, et sur chaque commande ; après chaque transition il
persiste l'instantané **avant** de poser ou retirer un overlay (§10.2 : aucun overlay au lancement
ne tient que si l'état écrit est toujours en avance sur l'écran). `Enforcer` traduit `CycleState`
en appels d'`OverlaySurfacesPort`, avec le délai de pose ≤ 200 ms (Hardcore à l'échéance) et
≤ 500 ms (écran ou application apparus en cours de pause).

#### Arbre de dossiers cible

```
breeze/
├── .charpente/                         # artefacts en français (brief, cadrage, conception, plan)
│   └── brief/                          # réécrit à partir du brief SANS + D2
├── ARCHITECTURE.md · DESIGN.md · EXPERIENCE.md · README.md
├── Cargo.toml                          # [workspace] members = crates/*, src-tauri
├── crates/
│   ├── breeze-domain/                  # PUR — aucune dépendance système, aucun port
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── cycle/                  # CycleState, transitions, Countdown, Notice, Returning
│   │   │   ├── suspension.rs           # chaîne, référence, dépassement
│   │   │   ├── session.rs              # SessionState, AbsenceVerdict
│   │   │   ├── settings/               # Rhythm, Severity, AppStatus, PendingSettings (règle du sens)
│   │   │   ├── outcome.rs              # BreakOutcome, InterruptionDoor, ledger
│   │   │   ├── breaker.rs              # CrashCounter, Breaker
│   │   │   ├── capabilities.rs         # EnforcementCapabilities, BreakMode
│   │   │   ├── clock.rs                # Instant, WallClock, ClockJump (types seulement)
│   │   │   ├── constants.rs            # §12.2 : NOTICE=1min, RETURN=3s, IDLE_FREEZE=3min, ESC_HOLD=10s…
│   │   │   ├── snapshot.rs             # Snapshot (persisté) et CycleSnapshot (projeté vers l'UI)
│   │   │   └── extensions/             # posture_debt.rs, deep_session.rs, calendar_shield.rs — déclarés, non câblés
│   │   └── tests/                      # scénarios temporels + propriétés, zéro double
│   ├── breeze-ports/                   # 10 traits, un fichier chacun, + types d'événements
│   │   └── src/{foreground_app,window_frames,overlay_surfaces,display_enumeration,
│   │            session_signals,notifications,autostart,persistence,update_check,clock}.rs
│   ├── breeze-app/                     # cas d'usage, Scheduler, Enforcer, projections
│   │   ├── src/{commands/,queries/,scheduler.rs,enforcer.rs,composition.rs}
│   │   └── tests/                      # contre breeze-bridge-null et une horloge pilotée
│   ├── breeze-bridge-common/           # persistence (rusqlite), update-check, autostart, clock
│   ├── breeze-bridge-macos/
│   ├── breeze-bridge-windows/
│   ├── breeze-bridge-linux-x11/
│   ├── breeze-bridge-linux-wayland/
│   └── breeze-bridge-null/             # capacités « inconnues » partout ; sert au coupe-circuit
├── src-tauri/                          # hôte : compose, expose les commandes IPC, fenêtres
│   ├── tauri.conf.json
│   ├── capabilities/                   # une capacité IPC par fenêtre (panel, notice, overlay, settings, onboarding)
│   └── src/{main.rs,lib.rs,shell/{tray.rs,single_instance.rs,windows.rs},ipc.rs,platform_select.rs}
├── ui/                                 # TypeScript, sans logique métier
│   ├── tokens.css                      # issu de Fondations.dc.html
│   ├── screens/{onboarding/,panel/,notice/,overlay-simple/,overlay-hardcore/,exit-gesture/,settings/}
│   └── lib/{ipc.ts,snapshot.ts}        # types miroir de CycleSnapshot et Command
├── design/                             # les 17 artboards, retouchés selon D1/D2
└── tests-e2e/                          # par OS : pose < 200 ms, ré-assertion < 500 ms, mémoire au repos
```

Ce qui est écarté : (1) mettre la machine à états côté TypeScript — le cycle doit survivre à un
webview gelé et tourner sans fenêtre ; (2) un bus d'événements ou une saga — un seul processus, une
seule boucle, un seul écrivain de l'état ; (3) un « super-port » `SystemBridge` à trente méthodes —
neuf ports étroits, un par besoin, pour que `breeze-bridge-null` et chaque adaptateur soient
remplaçables un à un ; (4) un ORM — un fichier SQLite, un instantané JSON versionné dans une table,
un journal borné.

### Conséquences

- Le brief réécrit peut renvoyer chaque règle à un module : §10.1 → `cycle/`, §10.2 → `outcome.rs`
  et `breaker.rs`, §10.3 → `settings/pending.rs`, §10.4 → `session.rs`, §10.5 → `capabilities.rs`,
  §10.7 → `clock.rs` + `scheduler.rs`.
- Le cadrage technique doit produire, avant toute fonctionnalité, la **matrice de capacités
  mesurée** (D2) et la **mesure de mémoire par OS** (D3) ; les deux sont des relevés, pas des
  paris.
- L'ordre de livraison naturel : `breeze-domain` complet et prouvé (aucun OS requis) →
  `breeze-app` + `breeze-bridge-null` (le cycle tourne sans overlay, le panneau vit) →
  `breeze-bridge-macos` (premier OS, notarisation bloquante) → Windows → Linux X11 → Linux Wayland
  KDE/wlroots → GNOME en dégradé déclaré.

### Ce qui est déclaré dégradé

- **`breeze-bridge-null` est un mode de production, pas seulement de test** : c'est ce que le
  coupe-circuit installe, et ce sur quoi tombe une session inconnue. Le produit y reste honnête
  (le cycle tourne, les pauses comptent servies, le panneau dit pourquoi) ; il n'est plus
  contraignant.
- **Le domaine ne sait rien des OS, et c'est voulu** : il ne pourra donc jamais « mieux tenir » sur
  un OS en connaissant ses ruses. Toute astuce d'empilement vit dans un adaptateur et derrière le
  délai de 500 ms ; si elle échoue, le domaine ne le sait que par `SurfaceLost`.

---

## Ce que ces quatre décisions changent au brief, en une liste

1. Source : brief SANS du 2026-08-31, pas `BRIEF.md`.
2. §3 et §14 : « pas multiplateforme » retiré ; trois OS, Wayland stratifié par compositeur.
3. §5 → « Ce que ta machine impose », par mécanisme et par plateforme (tableau récapitulatif D2).
4. §8.6 : « barre de menus » → « barre de menus ou zone de notification » ; GNOME : extension.
5. §10.5 → « quand une capacité manque », trois lignes de plus, même règle d'ancrage.
6. §10.6 : liste de sécurité par OS, toujours par identité.
7. §12.1 : versions par OS. §12.3 : six mesures techniques nouvelles + mémoire par OS + notarisation Tauri.
8. §13 : deux interdits nouveaux — « aucun hook clavier global, sur aucun OS » et « aucune exécution privilégiée ».
9. §17 : une hypothèse nouvelle — « un utilisateur Windows/Linux accepte un Hardcore qui revient devant en 500 ms plutôt qu'un Hardcore qui bloque ».
10. Design : retouches de légendes et suppressions listées en D1 ; aucun nouvel écran sauf la phrase de capacité par session sur `Severite`.
