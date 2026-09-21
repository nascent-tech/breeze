## 4. Les mots de Breeze

**Ce numéro ne porte pas l'arithmétique du modèle que le gabarit y attend d'ordinaire.** Breeze est
gratuit, sans transaction ni réseau d'aucune sorte (§11) : il n'y a pas de modèle économique à
justifier par un calcul. La logique qui en tient lieu — donner raison à celle qui décide à froid
contre celle qui négocie à chaud — est déjà posée au §2, et ce §4 reprend plutôt le numéro pour le
glossaire du produit, qu'aucune section fixe du gabarit ne porte ailleurs.

**Ce glossaire recense tout le vocabulaire de Breeze, mais ne le définit pas toujours lui-même.** Un
mot qui traverse plusieurs sections sans section propre se définit ici, une fois, et la section qui
porte sa règle est nommée — aucune autre section ne redonne cette définition, elle y renvoie. Un mot
propre à une seule fonctionnalité décisive ou à une seule règle transverse se définit à sa première
apparition, dans la section qui porte déjà sa règle : le répéter ici l'éloignerait du contexte qui le
rend compréhensible. Ce glossaire s'y limite alors à le renvoyer, jamais à le redéfinir — c'est la
troisième colonne, dans les deux cas, qui fait office d'index unique.

| Le mot | Ce qu'il désigne | Où vit sa règle |
|---|---|---|
| **Cycle** | Une phase de travail suivie d'une phase de pause. L'unité de temps de Breeze | §10.1 |
| **Sévérité** | L'étendue de ce qu'une pause recouvre : Simple (les applications bloquées) ou Hardcore (l'écran entier). Fixée à froid, jamais négociable une fois une pause due — mais elle n'égalise pas ce que les deux modes portent : seul le Hardcore porte un geste de sortie dédié (§8.5), au même prix que quitter | §8 |
| **Préavis** | La minute qui précède la pause, annoncée par une bannière. Purement informative : rien ne s'y clique, et rien ne le prolonge une fois commencé — seul son début peut être retenu, avant qu'il commence (§9.3) | §10.1 |
| **Application bloquée** | Recouverte par un overlay pendant une pause en Mode Simple. Statut par défaut | §8 |
| **Application épargnée** | Utilisable pendant une pause en Mode Simple. Son usage compte comme du travail | §8 |
| **Application ignorée** | Utilisable pendant une pause, **et** son usage ne fait pas avancer le décompte de travail. L'un des trois réglages protégés par le §10.3 — avec la sévérité et le rythme, chacun selon son propre mécanisme | §8 |
| **Coupe-circuit** | La désactivation automatique des overlays après des chutes répétées du processus. Ce n'est pas une commande utilisateur, c'est une protection contre les défauts de Breeze lui-même — et les pauses qu'il laisse passer comptent quand même comme servies | §10.2 |
| **Absence** | Une période sans signal d'activité observable pendant une session éveillée et déverrouillée — la veille et le verrouillage sont un fait distinct, que le §10.4 traite avec les mêmes règles | §10.4 |
| **Bouclier de calendrier** | La fonctionnalité décisive qui place les pauses à venir dans les interstices du calendrier. Définie à sa section, pas ici | §9.1 |
| **Dette de posture** | Les minutes non servies d'une pause interrompue, créditées puis remboursées par les pauses suivantes. Définie à sa section, pas ici | §9.2 |
| **Indice de profondeur** | La mesure de concentration qui peut retenir le début d'un préavis. Définie à sa section, pas ici | §9.3 |
| **Liste de sécurité** | Les applications système toujours épargnées, non éditables — une liste par système d'exploitation. Définie à sa section, pas ici | §10.6 |
| **Capacité** | Ce qu'un système d'exploitation laisse Breeze faire sur une session donnée : voir quelle application est devant, suivre le cadre des fenêtres, poser une couche devant les fenêtres des autres, loger une icône d'état. Une capacité est fiable, partielle, non garantie ou impossible (§5) ; quand elle manque, le §10.5 dit ce que Breeze fait à la place | §5, §10.5 |
| **Mode déclaré dégradé** | Une session où au moins une capacité manque, et où Breeze le dit à froid — à l'onboarding et dans les réglages — plutôt que de le laisser découvrir en pause. GNOME Wayland en est le cas nominal (§5) | §5 |

**Trois mots de l'ancien vocabulaire de Breeze disparaissent avec la négociation, et c'est
délibéré** : *report*, *budget de report* et *inhibition* n'ont plus d'objet, puisque rien ne
retarde ni n'écourte plus une pause due (§10, §15 décision 4). Le vocabulaire d'un produit qui
ne négocie pas ne devait pas hériter du vocabulaire de la négociation.

---

## 5. Ce que ta machine impose

Breeze tourne sur trois systèmes d'exploitation, et sur Linux sous deux types de session — X11 et
Wayland — dont le second se stratifie encore par compositeur. Ce ne sont pas trois produits : c'est
un seul produit dont **chaque mécanisme** trouve, sur chaque machine, une capacité fiable, partielle,
non garantie ou impossible. Cette section est donc rangée **par mécanisme, puis par plateforme**.
Chaque contrainte est donnée avec **ce qu'elle impose au produit** et **ce qui est déclaré dégradé**.
La manière de la traiter n'est pas du ressort de ce document ; les sources sont au §18.

**Le principe qui tient toute la section**, posé au §3 : la promesse de Breeze ne dépend d'aucune
détection. Une capacité manquante accélère le décompte ou réduit la couverture ; jamais elle ne lève
une contrainte. Les trois OS ont en commun ce qui compte le plus : **aucun ne laisse Breeze
empêcher de forcer à quitter, de changer de session, d'éteindre ou de débrancher** — ce sont des
sorties qui **doivent** rester (§2), et c'est précisément celle-là que Breeze retient comme unique
issue pendant une pause plutôt que d'en construire une à lui (§10.2).

### 5.1 Voir quelle application est devant

Breeze lit l'**identité** de l'application au premier plan — jamais le titre de sa fenêtre (§10.6).
C'est ce qui fait avancer ou geler le décompte selon le statut de l'application (§8.3).

| Plateforme | Ce que la machine impose | Ce qui est déclaré dégradé |
|---|---|---|
| macOS | L'identité de l'application active est publique, sans permission. Le titre d'une fenêtre, lui, exigerait l'Enregistrement d'écran : Breeze ne le demande jamais | Rien |
| Windows | L'identité du processus au premier plan est publique, sans permission. Une application lancée avec des droits élevés refuse de dire qui elle est | Une application élevée est vue comme « inconnue » : son usage compte comme du travail |
| Linux X11 | L'identité est publique sur tout gestionnaire de fenêtres conforme aux conventions courantes | Rien sur les gestionnaires courants |
| Linux Wayland — KDE, wlroots | Le compositeur expose l'application active par un protocole qui donne aussi le titre : Breeze ne s'y abonne pas | Rien là où le protocole est exposé |
| Linux Wayland — GNOME | Aucun protocole public. GNOME réserve cette information à des applications qu'il autorise lui-même | **Impossible.** Tout usage compte comme du travail ; le statut `ignorée` n'a aucun effet, et Réglages › Applications le dit : « Sur cette session, Breeze ne voit pas quelle application est devant : tout compte comme du travail » |

Le repli est toujours le même, et il est le côté sûr : **identité inconnue = tout compte comme du
travail** (§10.5). Le décompte avance plus vite, il ne gèle jamais à tort.

### 5.2 Suivre le cadre des fenêtres — le Mode Simple

Le Mode Simple pose un voile sur chaque fenêtre d'une application bloquée (§8.4). Il lui faut la
position et la taille des fenêtres d'autrui.

| Plateforme | Ce que la machine impose | Ce qui est déclaré dégradé |
|---|---|---|
| macOS | Aucun moyen public de se placer au-dessus d'une **fenêtre précise** d'autrui : Breeze ne peut que suivre le cadre et se poser dessus. Suivre ce cadre exige la permission **Accessibilité**, que l'utilisateur accorde ou retire quand il veut, et qui suit l'**identité signée** du binaire — toute mise à jour doit garder la même identité, sans quoi l'utilisateur ré-autorise à chaque version. macOS ne laisse aucune application retirer sa propre entrée d'Accessibilité | Sans la permission : voile plein écran (§10.5). La désinstallation propre s'arrête là où macOS l'arrête : Breeze **affiche la marche à suivre** pour l'entrée qu'il ne peut pas retirer (§8.6) |
| Windows | Les cadres sont publics, sans permission. Une application lancée « en tant qu'administrateur » n'est pas énumérable | Une application élevée n'est pas voilée. Dit dans Réglages › Applications |
| Linux X11 | Les cadres sont publics sur tout gestionnaire de fenêtres conforme. Une fenêtre sur un autre bureau virtuel n'est pas visible, donc pas voilée | Rien de plus que ce que le bureau virtuel cache déjà |
| Linux Wayland — tous compositeurs | **Aucun protocole public n'expose la géométrie des fenêtres d'autrui.** Le compositeur donne l'état d'une fenêtre, jamais sa position | **Impossible.** Le Mode Simple pose un **voile unique plein écran par moniteur** avec le décompte — c'est le repli du §10.5, et sur Wayland c'est le comportement nominal, dit dès l'onboarding : « Sur Wayland, le Mode Simple voile tout l'écran plutôt que des fenêtres précises » |

Sur toutes les plateformes, le Mode Simple reste contournable en déplaçant une fenêtre épargnée
par-dessus, et le brief l'assume plutôt que de promettre l'inverse (§8.4).

### 5.3 Couvrir chaque écran, devant ce que le système laisse recouvrir — le Mode Hardcore

Le Mode Hardcore couvre chaque écran d'un fond opaque et tient devant tout ce que le système laisse
recouvrir (§8.5). C'est le mécanisme où les plateformes divergent le plus, et où Breeze doit le plus
dire à froid.

| Plateforme | Ce que la machine impose | Ce qui est déclaré dégradé |
|---|---|---|
| macOS | Une fenêtre de niveau maximal passe au-dessus du Dock et de la barre de menus ; masquer les deux et désactiver le changement d'application est une API publique. Mais Apple **ne garantit rien face à une application en plein écran natif** et déconseille explicitement la technique pour ce cas. Aucune permission | Le plein écran natif est une **mesure** (§12.3) ; le repli est de le dire à froid. Cmd-Q de Breeze reste possible (§8.5) |
| Windows | Une fenêtre « toujours devant » par moniteur est publique, sans permission. **Alt-Tab, la touche Windows et Ctrl-Alt-Suppr ne sont pas neutralisés** : les bloquer exigerait un hook clavier bas niveau qui lit les touches, ce que §10.6 et §13 interdisent. Une application en **plein écran exclusif** (un jeu) et le **bureau sécurisé** (élévation, Win+L) passent au-dessus de tout | Le menu Démarrer ou une autre fenêtre peuvent passer devant ; **l'overlay revient devant en moins de 500 ms** (§12.2), il ne bloque pas. Le plein écran exclusif est une mesure (§12.3) |
| Linux X11 | Une fenêtre hors du contrôle du gestionnaire de fenêtres, une par sortie, est publique. Le gestionnaire garde le dernier mot sur l'empilement ; un menu contextuel ou une notification peuvent passer devant. Alt-Tab **non neutralisé**, même raison que Windows | L'overlay revient devant en moins de 500 ms. Dit à froid : « ton gestionnaire de fenêtres garde le dernier mot sur l'empilement » |
| Linux Wayland — KDE, wlroots | Le compositeur expose une **couche overlay** dédiée, une surface par sortie, qui tient devant les fenêtres ordinaires. La capture exclusive du clavier par cette couche est refusée ou ignorée par certains compositeurs | **Les raccourcis du compositeur ne sont pas garantis** : dit à froid, « ton compositeur peut laisser passer un raccourci ». La capture exclusive est une mesure par compositeur (§12.3) |
| Linux Wayland — GNOME | **Pas de couche overlay.** Breeze ne peut poser qu'une fenêtre plein écran ordinaire par sortie ; GNOME peut placer une autre fenêtre devant, l'utilisateur peut Super ou Alt-Tab, le shell reste accessible | **Non garanti.** Dit à froid : « Sur GNOME Wayland, le Mode Hardcore n'est pas un mode Hardcore. Il couvre chaque écran mais ne tient pas devant le système. » Le choix Hardcore reste offert, avec cette légende sous le bouton — jamais grisé sans raison |

**Ce qui est écarté, sur toutes les plateformes** : toute promesse « au-dessus de tout » dans la
présentation du produit ; tout hook clavier global pour neutraliser Alt-Tab, Windows ou Super ; toute
exécution privilégiée pour « tenir mieux » (§13). Breeze reste une application utilisateur ordinaire :
celui qui subit la contrainte est celui qui l'a posée.

### 5.4 Savoir que la machine dort, se verrouille ou attend

Le §10.4 ne connaît que trois faits : une activité observée (son instant, jamais son contenu), un
état de session (éveillée, endormie, verrouillée, autre utilisateur actif), et un saut d'horloge.
**Le verdict d'absence est le même sur les trois OS — c'est la durée cumulée qui compte, jamais sa
cause.**

| Signal | Ce que la machine impose | Ce qui est déclaré dégradé |
|---|---|---|
| Inactivité | L'instant de la dernière action clavier ou souris est public sans permission sur macOS, Windows, X11, et sur Wayland par le compositeur ou par le bureau (GNOME compris) | Rien : **fiable partout** |
| Veille et réveil | Notifiés par le système sur macOS et Windows ; sur Linux, par le gestionnaire de session (logind) | Sans logind — systèmes sans systemd — la veille est **déduite d'un saut de l'horloge monotone** et traitée comme une absence de même durée (§10.4). La présence de logind est une mesure (§12.3) |
| Verrouillage et déverrouillage | Notifié par le système sur Windows, par logind sur Linux (GNOME ajoute son propre signal). Sur macOS, la notification est **stable depuis dix ans mais non documentée** | Sur macOS, c'est une mesure par version (§12.3) ; en son absence, un verrouillage non vu compte comme une inactivité, ce qui ne fait qu'accélérer le décompte |
| Changement rapide d'utilisateur | Notifié sur les trois OS | Rien |

### 5.5 Loger Breeze, le lancer au démarrage, le prévenir, le garder, le mettre à jour

| Besoin | Ce que la machine impose | Ce qui est déclaré dégradé |
|---|---|---|
| L'icône d'état (§8.6) | Barre de menus sur macOS, sans icône dans le Dock ; zone de notification sur Windows ; indicateur d'état standard sur X11, KDE et wlroots | **GNOME n'a pas de zone d'état sans l'extension AppIndicator.** Dit à froid : « Sur GNOME, installe l'extension AppIndicator, sinon Breeze s'ouvre par sa fenêtre » — le panneau reste ouvrable en relançant l'application |
| Le préavis quand la bannière propre ne peut pas se poser (§8.1) | Notification système : demande une permission sur macOS seulement, aucune sur Windows ni Linux | Rien : la permission macOS n'est demandée que dans ce cas |
| Le démarrage automatique | Mécanisme standard de chaque OS, sans droits élevés | Rien |
| La persistance (échéances murales, sort des pauses, réglages, journal, statistiques — §10.6, §12.2) | Le dossier de données utilisateur de chaque OS, un seul fichier, écrit de façon atomique | Rien |
| La mise à jour (§10.6) | macOS : signée et notariée, **même identité de signature** que la version précédente. Windows : installeur signé. Linux : paquets ou image autonome ; quand un gestionnaire de paquets est là, c'est lui qui applique | Rien de plus que ce que le §10.6 fixe déjà : jamais pendant une pause due ou en cours |
| La distribution | Le bac à sable du Mac App Store est incompatible avec ce que Breeze fait : distribution directe, signée et notariée, faute de quoi macOS bloque. Aucun magasin n'est exigé sur Windows ni Linux | Rien |
| L'horloge | Chaque OS fournit une horloge monotone et une horloge murale : le §10.7 en a besoin pour ne jamais reculer une échéance | Rien |

### 5.6 Ce qui est déclaré dégradé, plateforme par plateforme

Le récapitulatif que les réglages et l'onboarding reprennent. Une case « fiable » est une promesse ;
tout le reste est dit à froid.

| Plateforme | Détection premier plan | Mode Simple par fenêtre | Mode Hardcore devant tout | Icône d'état |
|---|---|---|---|---|
| macOS 13+ | Fiable | Fiable (Accessibilité) | Fiable, **sauf plein écran natif (mesure)** | Fiable |
| Windows 10/11 | Fiable | Fiable | Partiel : **Alt-Tab/Win non neutralisés, retour < 500 ms ; plein écran exclusif et bureau sécurisé passent devant** | Fiable |
| Linux X11 | Fiable | Fiable | Partiel : **empilement au gestionnaire de fenêtres, retour < 500 ms ; Alt-Tab non neutralisé** | Fiable ; GNOME : extension |
| Linux Wayland KDE / wlroots | Fiable | **Impossible → voile plein écran** | Fiable via la couche overlay ; **raccourcis du compositeur non garantis** | Fiable |
| Linux Wayland GNOME | **Impossible → tout compte** | **Impossible → voile plein écran** | **Non garanti → plein écran ordinaire** | **Extension requise** |

### 5.7 La phrase à froid, par session

L'onboarding affiche **une seule phrase** sous le choix de sévérité (§8.7), propre à la session
détectée au lancement — jamais au nom du bureau, toujours à ce que la session expose réellement. La
même phrase est reprise dans Réglages › Sévérité. Elle ne change pas pendant une pause : si la
session change (X11 vers Wayland, permission retirée), la phrase change au prochain [TRAVAIL], et la
pause en cours garde le mode fixé à son premier instant (§10.5).

| Session | Phrase à froid |
|---|---|
| macOS | « Aucun mode n'est infaillible — macOS te laisse toujours forcer à quitter ou éteindre. » Si la mesure du plein écran natif (§12.3) est défavorable : « Une application en plein écran natif peut passer devant le Mode Hardcore. » |
| Windows | « Aucun mode n'est infaillible — Windows te laisse toujours Alt-Tab, le menu Démarrer, Ctrl-Alt-Suppr. Le Mode Hardcore revient devant en moins d'une demi-seconde, il ne te bloque pas. Un jeu en plein écran exclusif passe devant. » |
| Linux X11 | « Aucun mode n'est infaillible — ton gestionnaire de fenêtres garde le dernier mot sur l'empilement. Le Mode Hardcore revient devant en moins d'une demi-seconde. » |
| Linux Wayland (KDE, wlroots) | « Le Mode Simple voile tout l'écran plutôt que des fenêtres précises : Wayland ne montre pas où sont les fenêtres des autres. Le Mode Hardcore tient devant tout ce que ton compositeur laisse recouvrir. » |
| Linux Wayland (GNOME) | « Sur GNOME Wayland, Breeze ne voit pas quelle application est devant, ne peut pas voiler des fenêtres précises, et le Mode Hardcore ne tient pas devant le système. Il couvre tes écrans, et c'est tout. » |

**Une contrainte qui ne dépend d'aucune plateforme, et que la négociation retirée a rendue
marginale** : détecter un appel visio sans lire le micro, la caméra ni l'écran n'est possible que par
l'identité de l'application au premier plan, et une visio tenue dans un onglet de navigateur n'est
donc pas détectable de façon fiable — sur aucun OS. Sans exception à activer sur ce signal, cette
limite ne concerne plus que le placement proactif des pauses (§9).

---

## 6. Les acteurs, et ce que chacun a le droit de faire

Il n'y a **qu'une seule personne** dans ce produit, et — contrairement à l'ancien modèle — elle n'a
plus qu'un seul régime de droits : ce qu'elle règle à froid **est** ce qui s'applique, sans qu'aucune
version « à chaud » d'elle-même ne puisse plus rien changer une fois une pause en cours.

| Acteur | Ce qu'il est | Ce qu'il peut | Ce qu'il ne peut pas |
|---|---|---|---|
| **L'utilisateur, hors pause due** | La personne, tant que le décompte de travail n'a pas atteint zéro : onboarding, réglages, phase de travail | Choisir la sévérité, le rythme, le statut de chaque application. Quitter Breeze à tout instant. Suspendre Breeze à tout instant de [TRAVAIL] (§10.1), y compris juste avant qu'une pause devienne due | Régler la durée du préavis : c'est une constante du produit, pas un réglage (§12) |
| **L'utilisateur, pendant l'attente d'une pause due (§9.3) ou pendant [PRÉAVIS]** | La même personne, depuis l'instant où le décompte de travail a atteint zéro | Attendre. Quitter Breeze — ce que son système garantit de toute façon | Changer la sévérité, le rythme ou le statut d'une application pour cette pause (§10.3). L'interdiction de suspendre ne vaut, elle, que depuis l'instant où [PRÉAVIS] commence — l'attente qui le précède reste ouverte à la suspension, comme le reste de [TRAVAIL] |
| **L'utilisateur, pendant [PAUSE ACTIVE]** | La même personne, dès que la pause a commencé | Regarder le décompte. Quitter Breeze, ou — en Mode Hardcore seulement — utiliser le geste dédié (§8.5), qui mène à la même issue que quitter, au même prix | Écourter la pause par un autre moyen, ou changer sa sévérité. Aucune de ces commandes n'existe dans l'interface : ce n'est pas qu'elles sont refusées, c'est qu'il n'y a rien à actionner |
| **Le système d'exploitation** | L'arbitre, et le seul détenteur d'un pouvoir réel sur la machine — macOS, Windows, ou sur Linux le gestionnaire de fenêtres ou le compositeur de la session | Accorder et retirer les permissions à tout instant, garder l'ordre d'empilement des fenêtres, exposer ou non une capacité (§5), laisser l'utilisateur éteindre ou forcer à quitter | Rien ne le contraint. Breeze ne lui impose jamais rien et traite chaque refus — permission, protocole absent, couche refusée — comme un cas normal, pas comme une erreur (§10.5) |
| **L'application tierce** | Une application installée sur la machine, bloquée, épargnée ou ignorée | Continuer de fonctionner normalement en toutes circonstances | Elle ne sait pas que Breeze existe. Elle n'est **jamais** masquée, quittée, ni suspendue : Breeze se pose devant elle et ne la touche pas (§10.6) |

**Trois états n'apparaissent dans aucune ligne du tableau, parce qu'aucun n'y change un droit** :
[INACTIF] (rien n'est dû, rien ne se règle que ce que §8.7 couvre déjà) ; [SUSPENDU], où seule la
commande « Reprendre » s'ajoute à celles de la ligne « hors pause due » (§10.1) ; [RETOUR], où rien
ne se suspend ni ne se raccourcit (§10.1).
