## 16. Ce qui vient après le lancement

**Les trois fonctionnalités décisives du §9 sont repoussées sciemment.** Chacune doit franchir quatre
conditions, dans cet ordre : elle est impossible sans ce que Breeze tient déjà · elle renforce la
promesse du §1 plutôt que d'élargir le périmètre · elle tient dans ce qui existe, sans second produit
à construire · elle ne coûte aucune permission que le §13 interdit.

| La fonctionnalité décisive | Ce qu'elle apporte | Ce qu'elle attend |
|---|---|---|
| **Le bouclier de calendrier** (§9.1) | Le Mode Hardcore devient tenable en environnement professionnel | Que la permission Calendriers soit jugée acceptable par la bêta (§12.3) — la seule que Breeze demanderait au-delà du strict nécessaire |
| **La dette de posture** (§9.2) | Empêche la dérive vers l'inefficacité qu'ignorer une pause, cycle après cycle sans aucune conséquence, produit inévitablement | Son seuil de durcissement, mesuré en test utilisateur (§12.3). Bloquant : une escalade mal calibrée est pire que pas d'escalade |
| **La détection de session profonde** (§9.3) | Répond à une objection qu'on peut attendre de la cible, sans réouvrir la négociation ailleurs | Ses seuils et le plafond de retenue du préavis, mesurés sur des utilisateurs réels (§12.3). Bloquant, même raison |

**Une quatrième chose est repoussée, mais elle ne vient pas du §9 et ne répond pas au même test** :
les profils de réglages multiples (§14) sont une décision de hors périmètre, pas une fonctionnalité
décisive — ils se révisent sur un seul signal, distinct des quatre conditions ci-dessus.

| Ce qui est hors périmètre et pourrait rentrer | Ce qu'il apporterait | Ce qu'il attend |
|---|---|---|
| **Les profils de réglages multiples** (§14) | Un rythme par contexte de travail | Que la bêta montre des reconfigurations répétées du rythme. Sans ce signal, c'est un écran de plus pour rien |

---

## 17. Les hypothèses risquées

Ce que ce brief tient pour vrai sans l'avoir vérifié, et qui, faux, casse le produit. **Quatre
hypothèses, là où le gabarit en admet trois** : les trois premières sont celles du produit sans
négociation, inchangées ; la quatrième est née de la couverture des trois systèmes (décision 23) et
ne se fusionne avec aucune des autres — elle porte sur un fait de plateforme, pas sur le
comportement de l'utilisateur face à la contrainte. La lever du plafond est un choix de cette
version, dit ici plutôt que laissé au compte.

| Ce qu'on tient pour vrai | Ce qui casse si c'est faux | Ce qui le trancherait |
|---|---|---|
| **Celui qui a désactivé les rappels passifs veut être contraint sans aucune marge de négociation — pas juste mieux rappelé, et pas juste moins négociable qu'avant** (§2) | Le pari central de cette version du brief. S'il faut malgré tout une soupape, même petite, zéro négociation produit plus de désinstallations qu'un modèle à budget borné, sans rien gagner en discipline | La part des utilisateurs de la bêta qui ont choisi le Mode Hardcore pendant leur première semaine, et la part des installations encore actives après trente jours (§12.3) |
| **Le geste de sortie du Mode Hardcore, tarifé exactement comme quitter Breeze, n'est pas utilisé comme un raccourci pratique pour interrompre une pause à volonté** (§8.5, §9.2) | Si le coût réel — la dette de posture, l'escalade qu'elle finit par déclencher — n'est pas ressenti au moment du geste, il devient un bouton de sortie ordinaire sous un autre nom, et « aucune négociation » n'est plus vrai que sur le papier | La fréquence du geste par utilisateur et par semaine, comparée à la fréquence du raccourci de fermeture du système ou du menu de l'icône d'état pendant une pause (§12.3) |
| **Retarder seulement le début du préavis, jamais le préavis ou la pause eux-mêmes, suffit à désamorcer « ça va me couper en plein travail »** (§9.3) | Si le plafond de retenue est trop court, l'objection reste entière et la fonctionnalité ne change rien ; s'il est trop long, elle devient un deuxième budget de négociation sous un autre nom, exactement ce que ce brief vient de fermer | La part des pauses servies plutôt qu'interrompues, mesurée séparément pour les utilisateurs à profondeur habituellement élevée (§12.3) |
| **Un utilisateur Windows ou Linux accepte un Mode Hardcore qui revient devant en moins de 500 ms plutôt qu'un Mode Hardcore qui bloque le système** (§5.3, §8.5, §13) | Si un Hardcore qu'Alt-Tab traverse une demi-seconde est vécu comme « ça ne marche pas » plutôt que comme « ça revient toujours », le Mode Hardcore n'est crédible que sur macOS, et sur deux OS sur trois Breeze n'est plus qu'un Mode Simple avec un fond opaque. La réponse ne serait pas un hook clavier ni un service — les deux sont interdits (§13, décisions 24 et 25) : ce serait de dire à froid que le Hardcore n'existe pas sur cette machine, et de le mesurer en désinstallations | La part des utilisateurs Windows et Linux qui gardent le Mode Hardcore après leur première pause Hardcore, comparée à celle des utilisateurs macOS, et le nombre de fois où une fenêtre est passée devant l'overlay pendant une pause (§12.3) — en bêta, sur les trois OS |

---

## 18. Sources

Ce qui appuie les contraintes du §5. Les sources macOS ont été lues le 2026-08-28 pour la version
précédente de ce brief et réaffirmées le 2026-08-31 : l'intervalle est trop court pour que la
documentation Apple citée ait changé, et aucune des contraintes ci-dessous n'est liée au mécanisme de
négociation que cette version retire — seules les lignes qui l'étaient ont été retirées avec lui (le
signal de partage d'écran et celui de présentation plein écran, qui ne servaient qu'à l'ancienne
inhibition). Les sources Windows et Linux ont été posées le 2026-09-21 par le registre de décisions
D2 ; **leur accès n'a pas été revérifié par ce brief**, et c'est la reconnaissance technique du §12.3
qui les confirme ou les infirme, compositeur par compositeur.

**macOS**

| Ce qui est affirmé | Ce qu'on en a tiré, et où |
|---|---|
| Une fenêtre au niveau maximal passe au-dessus du Dock et de la barre de menus | Hiérarchie des `CGWindowLevelKey`, où le niveau de l'économiseur d'écran est au-dessus de ceux du Dock et du menu principal — `developer.apple.com/documentation/coregraphics/cgwindowlevelkey`. **Acquis.** |
| Cette même fenêtre n'est **pas** garantie au-dessus d'une application en plein écran natif | Apple écrit de la technique qu'elle *« n'est pas recommandée »* — `developer.apple.com/documentation/coregraphics/cgshieldingwindowlevel()`. **Non acquis, et c'est une mesure du §12.3** |
| Masquer le Dock et la barre de menus, et désactiver le changement d'application, est une API publique | Les trois options de présentation d'AppKit, documentées depuis macOS 10.6 — `developer.apple.com/documentation/appkit/nsapplication/presentationoptions-swift.struct`. **Acquis.** |
| L'autorisation d'Accessibilité suit l'identité signée du binaire, pas son chemin | macOS suit l'identité du code par sa *designated requirement* — réponse d'un ingénieur Apple sur le forum développeur officiel, renvoyant à la note technique TN3127. **Confirmé par une source Apple, mais non normative** |
| Aucune API publique ne place une fenêtre au-dessus d'une **fenêtre précise** d'une autre application | Les niveaux documentés sont catégoriels — `developer.apple.com/library/archive/documentation/Cocoa/Conceptual/WinPanel/Concepts/WindowLevel.html`. **Confirmé par absence** |
| Le Mac App Store est fermé à ce produit | Règles de revue d'Apple (2.4.5, 2.5.8) — `developer.apple.com/app-store/review/guidelines/` — et confirmation qu'un binaire en bac à sable ne peut pas utiliser l'API Accessibilité même avec la permission accordée. **Acquis.** |
| Apple est passé de macOS 15 à macOS 26, sans versions 16 à 25 | Communiqué officiel — `apple.com/newsroom`. **Acquis.** Une matrice de test qui chercherait ces versions perdrait son temps |

**Windows**

| Ce qui est affirmé | Ce qu'on en a tiré, et où |
|---|---|
| L'identité du processus au premier plan et les cadres des fenêtres visibles sont publics, sans permission ; une application élevée refuse l'accès à son nom | API Win32 documentées sur `learn.microsoft.com` (`GetForegroundWindow`, `QueryFullProcessImageNameW`, `DwmGetWindowAttribute`). **Acquis par la documentation, accès non revérifié.** §5.1, §5.2 |
| Une fenêtre « toujours devant » peut être passée par une autre fenêtre « toujours devant » créée après elle, et par le plein écran exclusif ou le bureau sécurisé | Documentation de `SetWindowPos` et du bureau sécurisé (UAC) sur `learn.microsoft.com`. **Acquis par la documentation.** C'est la ligne Windows du §5.3 et la mesure du plein écran exclusif au §12.3 |
| Neutraliser Alt-Tab, la touche Windows ou Ctrl-Alt-Suppr exige un hook clavier bas niveau (`WH_KEYBOARD_LL`) qui reçoit chaque frappe, ou une stratégie de kiosque réservée à l'administrateur | Documentation de `SetWindowsHookExW` et du mode kiosque sur `learn.microsoft.com`. **Acquis par la documentation.** C'est ce qui fonde les deux interdits du §13 |

**Linux**

| Ce qui est affirmé | Ce qu'on en a tiré, et où |
|---|---|
| Sur X11, l'identité, les cadres et l'empilement sont publics sur tout gestionnaire conforme à EWMH ; le gestionnaire garde le dernier mot sur l'empilement | Spécification EWMH (`_NET_ACTIVE_WINDOW`, `_NET_CLIENT_LIST_STACKING`, `_NET_WM_PID`) sur `specifications.freedesktop.org`. **Acquis par la spécification.** §5.1 à §5.3 |
| Sur Wayland, aucun protocole public ne donne la géométrie des fenêtres d'autrui ; les protocoles d'identité de fenêtre (wlroots, KDE) donnent l'état et l'identité, pas la position | Protocoles `wlr-foreign-toplevel-management-unstable-v1` et `org_kde_plasma_window_management` sur `gitlab.freedesktop.org` et `invent.kde.org`. **Confirmé par absence ; à revérifier à chaque version de protocole.** §5.2 |
| La couche overlay (`wlr-layer-shell-unstable-v1`) est exposée par wlroots et par KDE Plasma depuis 5.20, pas par GNOME ; la capture clavier exclusive y est optionnelle | Protocole `wlr-layer-shell` et notes de version de Plasma 5.20 ; absence de layer-shell dans Mutter documentée par les tickets ouverts de GNOME. **Acquis pour wlroots et KDE ; GNOME confirmé par absence.** §5.3, mesure de la capture exclusive au §12.3 |
| L'inactivité est lisible sans permission sur toutes les sessions Linux ; la veille et le verrouillage passent par logind quand il est présent | `ext-idle-notify-v1`, `org.gnome.Mutter.IdleMonitor`, `org.freedesktop.login1` sur `freedesktop.org`. **Acquis par la documentation ; la présence de logind est une mesure du §12.3.** §5.4 |
| GNOME n'a pas de zone d'état sans l'extension AppIndicator | Documentation de GNOME Shell et de l'extension `appindicator-support`. **Acquis.** §5.5, §8.6 |

**Ce qui n'a pas pu être vérifié**, et qui reste à la charge de la reconnaissance technique : le
comportement réel du niveau de fenêtre maximal face au plein écran natif, version par version de
macOS 13 à 26 ; le délai de retour devant sous charge sur Windows et X11 ; le comportement de GNOME
face à deux fenêtres plein écran ; et la capture clavier exclusive, compositeur par compositeur. Les
quatre sont des lignes du §12.3.

**Aucune source de marché n'appuie ce document, et il n'en cite aucune.** C'est délibéré : le §12.3
n'accueille que des mesures à faire sur les utilisateurs réels de Breeze.
