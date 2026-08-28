# Breeze — brief

Ce document énonce **ce qu'il faut construire et pourquoi**. Il ne dit ni comment : ni architecture,
ni découpage, ni fichier, ni technologie. Le fondateur, la designer et la développeuse le lisent tous
et doivent le comprendre pareil. Les contraintes techniques de macOS y figurent uniquement par **ce
qu'elles imposent au produit** (§5) ; la manière de les affronter vit dans le cadrage technique, et
l'ordre de livraison avec elle.

**Tout y est tranché.** Aucune section ne renvoie une question à plus tard. Là où une valeur n'est
pas connue — un seuil de succès, un signal que macOS expose ou non —, ce n'est pas une décision qui
manque : c'est une **mesure**, et le §12.3 dit qui la porte, sous quelle forme, et ce qui se passe
tant qu'elle manque.

**Premier public : le travailleur du savoir seul devant son Mac personnel ou professionnel, sans
administrateur au-dessus de lui.** Ce choix fixe tout le reste : pas de compte, pas de serveur, pas
de déploiement en flotte, aucune donnée qui quitte la machine, et une contrainte que l'utilisateur
s'impose à lui-même plutôt qu'une politique qu'on lui applique.

---

## 1. Le projet en une page

Breeze est une application de barre de menus macOS qui **force la pause**. Là où les concurrents
envoient une notification qu'on balaie sans y penser, Breeze intervient sur le poste de travail : il
recouvre les applications qui retiennent l'utilisateur, ou l'écran entier, le temps d'un compte à
rebours qu'on ne peut pas écourter d'un clic.

**Ce que Breeze ne détient pas.** Il ne détient ni le système d'exploitation, ni les applications
qu'il recouvre, ni aucun droit d'administration sur la machine. macOS reste l'arbitre : il accorde ou
retire les permissions, garde l'ordre d'empilement des fenêtres, et laisse toujours à l'utilisateur
le moyen d'éteindre son Mac. Breeze n'a aucun pouvoir que macOS ne lui prête, et il ne prétend jamais
en avoir un.

**Ce que Breeze détient, c'est la friction** : le coût, en secondes et en minutes, de ne pas prendre
la pause qu'on s'était promise.

Ce que Breeze promet, en une phrase : **tu ne peux pas m'ignorer, et c'est le but.**

---

## 2. À qui ça s'adresse, et sur quel terrain

**Qui.** Un travailleur du savoir sur Mac — développeuse, designer, analyste, rédacteur — devant son
écran six à dix heures par jour. Il a déjà essayé des rappels de pause, et il les a désactivés au
bout d'une semaine.

**Ce qu'il vit aujourd'hui.** Il sait qu'il devrait s'arrêter. Il l'a même configuré. La notification
tombe au milieu d'une phrase, il la balaie d'un geste devenu automatique, et deux heures plus tard il
n'a pas bougé. Le rappel n'a pas échoué parce qu'il était mal réglé : il a échoué **parce qu'il était
négociable**, et que la négociation se fait à l'instant précis où l'on est le moins capable de la
gagner.

**La même personne, deux états, et le produit repose entièrement sur leur écart.** À froid — le
matin, dans les réglages —, elle veut s'arrêter toutes les heures et se dit prête à y être forcée. À
chaud — à la minute 47 d'un bloc de travail —, elle veut exactement l'inverse, et elle a toujours une
bonne raison. **Breeze donne raison à celle qui décide à froid, contre celle qui négocie à chaud.**
C'est la seule chose qu'il fait, et tout le reste du document en découle.

**Ce que le terrain impose.** macOS protège délibérément son utilisateur contre les applications qui
prennent l'écran en otage, et il a raison de le faire. Aucune contrainte que Breeze pose n'est
absolue : couper l'alimentation, forcer à quitter depuis un autre compte, débrancher l'écran restent
possibles et **doivent** le rester. Breeze rend donc la triche **coûteuse et consciente, jamais
impossible** — et il le dit à l'utilisateur avant que celui-ci l'apprenne tout seul, parce qu'une
promesse d'infaillibilité se paie en désinstallations le jour où elle est démentie.

---

## 3. Ce que Breeze fait, et ce qu'il n'est pas

**Ce qu'il fait.** Il compte le temps de travail, annonce la pause une minute avant, puis rend
l'écran — ou les seules applications désignées — inutilisable pendant la durée de la pause. Il offre
un nombre fini de reports, chiffré à l'avance, et qui se paie sur un budget commun.

**Ce que ça donne concrètement.** À 14 h 50, une bannière annonce la pause. L'utilisateur clique
« Reporter 5 min », une fois — le bouton disparaît. À 14 h 55, Figma et Slack se voilent, le décompte
apparaît, et rien ne les ramène avant 15 h 05. Notes et Spotify, épargnés à froid le premier jour,
continuent de fonctionner.

**Pourquoi ce n'est pas copiable en un trimestre.** Ce qui coûte cher dans Breeze n'est pas l'idée de
recouvrir un écran : c'est **le tissu de règles qui empêche l'utilisateur de reprendre d'une main ce
qu'il a concédé de l'autre**. Un budget de report unique, des états d'origine déclarés pour chaque
levier, une pause avortée qui débite les minutes qu'elle n'a pas servies, un réglage qui ne s'applique
jamais à une contrainte déjà due. Un concurrent qui copie l'overlay obtient un logiciel qu'on
désactive en trois clics ; les échappatoires ne se voient pas sur une capture d'écran, et chacune
d'elles suffit à ramener le produit au rang de rappel passif.

**Ce que Breeze n'est pas.**

- **Ce n'est pas un logiciel de contrôle parental, ni un MDM.** Personne ne l'installe sur la machine
  de quelqu'un d'autre. Il n'y a ni mot de passe administrateur, ni politique imposée à distance, ni
  compte qui verrouillerait les réglages : celui qui subit la contrainte est exactement celui qui l'a
  posée.
- **Ce n'est pas un bloqueur de sites ni de distractions.** Breeze ne lit aucune URL, ne connaît aucun
  domaine, et n'a pas d'avis sur ce qui mérite d'être fait. Il compte du temps et il l'interrompt.
- **Ce n'est pas un logiciel de suivi de productivité.** Rien n'est mesuré pour être rapporté à
  quiconque, et rien ne sort de la machine (§10.6).
- **Ce n'est pas un coach.** Aucun exercice guidé, aucun contenu audio ou vidéo, aucun conseil de
  santé. Breeze dit de se lever et regarde ailleurs.
- **Ce n'est pas une application multiplateforme.** Ce qu'il fait repose sur des mécanismes propres à
  macOS et n'a pas d'équivalent ailleurs (§14).

---

## 4. Les mots de Breeze

Chaque terme est défini une fois ici, et la section qui porte sa règle est nommée. Aucune autre
section ne redonne une définition : elle y renvoie.

| Le mot | Ce qu'il désigne | Où vit sa règle |
|---|---|---|
| **Cycle** | Une phase de travail suivie d'une phase de pause. L'unité de temps de Breeze | §10.1 |
| **Sévérité** | Le niveau de contrainte de la pause : Simple ou Hardcore. Ce n'est pas un réglage d'apparence | §8.5, §8.6 |
| **Préavis** | La minute qui précède la pause, annoncée par une bannière. Ce n'est pas la pause | §10.1 |
| **Pause due** | Une pause dont l'échéance est atteinte et qui n'a pas encore été servie. Ce n'est pas une pause en cours | §10.2 |
| **Inhibition** | Une condition extérieure qui repousse une pause due — visio, partage d'écran, présentation, plage d'exception. Ce n'est pas un report : l'utilisateur ne l'a pas demandée | §10.2 |
| **Report** | Un délai de cinq minutes demandé par l'utilisateur depuis le préavis. Ce n'est pas une annulation | §10.2 |
| **Budget de report** | Le total de minutes dont un cycle dispose pour repousser sa pause due, tous mécanismes confondus | §10.2 |
| **Application bloquée** | Recouverte par un overlay pendant une pause en Mode Simple. Statut par défaut | §8.3 |
| **Application épargnée** | Utilisable pendant une pause en Mode Simple. Son usage compte quand même comme du travail | §8.3 |
| **Application ignorée** | Utilisable pendant une pause, **et** son usage ne fait pas avancer le décompte de travail. C'est le seul statut qui touche au décompte | §8.3 |
| **Déclencheur** | Une application dont la présence au premier plan fait avancer le décompte de travail. À ne pas confondre avec le statut : ce sont deux listes distinctes | §8.4 |
| **Sortie d'urgence** | Le maintien d'une touche pendant dix secondes, qui lève un overlay Hardcore. Ce n'est pas un raccourci de confort | §8.6 |
| **Coupe-circuit** | La désactivation automatique des overlays après des chutes répétées du processus. Ce n'est pas une commande utilisateur | §10.6 |
| **Absence** | Une période sans aucun signal d'activité, mesurée par le système. Ce n'est pas l'application quittée : un processus mort ne mesure rien | §10.4 |

---

## 5. Le cadre imposé par macOS

Chaque contrainte est donnée avec **ce qu'elle impose au produit**. La manière de la traiter n'est pas
du ressort de ce document.

| La contrainte | Ce qu'elle impose au produit |
|---|---|
| macOS n'expose aucun moyen public de placer une fenêtre au-dessus d'une **fenêtre précise** d'une autre application | Le Mode Simple ne peut que suivre le cadre des fenêtres cibles et se poser dessus. Il est donc contournable en déplaçant une fenêtre épargnée par-dessus, et le brief l'assume plutôt que de promettre l'inverse (§8.5) |
| Suivre le cadre des fenêtres d'une autre application exige la permission **Accessibilité**, que l'utilisateur accorde ou retire quand il veut | Le Mode Simple a un mode dégradé obligatoire, décrit une seule fois (§10.5), et la permission est revérifiée en continu plutôt que supposée acquise |
| L'autorisation d'Accessibilité suit l'**identité signée** du binaire, pas son emplacement | Toute mise à jour doit conserver la même identité, sans quoi l'utilisateur ré-autorise à chaque version — ce qui détruirait la confiance plus sûrement que n'importe quel défaut |
| macOS ne permet à aucune application de retirer sa propre entrée d'Accessibilité | La désinstallation propre ne peut pas être complète : Breeze retire ce qu'il peut et **affiche la marche à suivre** pour le reste (§8.7) |
| Le bac à sable du Mac App Store est incompatible avec ce que Breeze fait | Distribution directe, hors magasin. Conséquence produit : l'application doit être signée et notariée, faute de quoi macOS la bloque — et une application non notariée qui demande l'Accessibilité et prend l'écran entier a le profil exact d'un logiciel malveillant |
| macOS place bien une fenêtre de niveau maximal au-dessus du Dock et de la barre de menus, mais **ne garantit rien face à une application en plein écran natif** — Apple déconseille explicitement la technique pour ce cas | Le Mode Hardcore peut ne pas couvrir un écran occupé par une application en plein écran natif. Le §12.3 en fait une mesure, et le repli est de **le dire à froid** plutôt que de le découvrir en pause |
| macOS laisse toujours forcer à quitter, changer d'utilisateur, ou éteindre | Aucune promesse d'infaillibilité, nulle part : ni dans l'interface, ni dans l'onboarding, ni dans la page de présentation (§2) |
| macOS n'expose l'état des modes Concentration qu'aux applications de messagerie autorisées, sous la forme d'un booléen sans nom de mode, et n'expose **aucun** moyen d'en activer un | Breeze ne lit ni ne pilote les modes Concentration : il n'entre dans aucune des catégories d'applications qui y ont droit. Il se contente de **n'émettre aucune notification pendant une pause**, et ne promet rien sur celles des autres applications (§8.6) |
| Détecter un appel visio sans lire le micro, la caméra ni l'écran n'est possible que par l'identité de l'application au premier plan | Une visio tenue dans un onglet de navigateur n'est pas détectable. Breeze le dit à l'endroit du réglage plutôt que de laisser croire à une couverture complète (§8.4) |

---

## 6. Les acteurs, et ce que chacun a le droit de faire

Il n'y a **qu'une seule personne** dans ce produit. Mais elle intervient dans deux états, et leur
séparation est la règle centrale de Breeze : c'est elle qu'on lit à chaque fois qu'un levier est
refusé quelque part.

| Acteur | Ce qu'il est | Ce qu'il peut | Ce qu'il ne peut pas |
|---|---|---|---|
| **L'utilisateur à froid** | La personne, hors de toute pause due : onboarding, réglages, début d'une phase de travail | Choisir la sévérité, le rythme, la plage horaire et les jours actifs, le statut de chaque application, les déclencheurs, les exceptions. Suspendre sans rien débiter, déclencher une pause, et redémarrer le cycle tant que le budget couvre les minutes de travail à effacer (§10.2) | Régler les quotas de report, le budget, la durée du préavis ou celle de la sortie d'urgence : ce sont des constantes du produit, pas des réglages (§12.2) |
| **L'utilisateur à chaud** | La même personne, dès qu'une pause est due ou en cours | Reporter une fois par préavis, dans la limite de son quota et du budget. Écourter une pause Simple après une minute. Sortir en urgence d'une pause Hardcore | Effacer une pause due, changer de sévérité pendant une pause, ou faire valoir sur la pause en cours un réglage qui l'affaiblit (§10.2) |
| **macOS** | L'arbitre, et le seul détenteur d'un pouvoir réel sur la machine | Accorder et retirer les permissions à tout instant, garder l'ordre d'empilement des fenêtres, laisser l'utilisateur éteindre ou forcer à quitter | Rien ne le contraint. Breeze ne lui impose jamais rien et traite chaque refus comme un cas normal, pas comme une erreur |
| **L'application tierce** | Une application installée sur la machine, bloquée, épargnée ou ignorée | Continuer de fonctionner normalement en toutes circonstances | Elle ne sait pas que Breeze existe. Elle n'est **jamais** masquée, quittée, ni suspendue : Breeze se pose devant elle et ne la touche pas (§13) |

---

## 7. Les besoins, séparés des solutions

Le tableau le plus important du document. La colonne de gauche porte **la formulation qui circule**
dans cette catégorie de produit — telle qu'elle arrive, déjà transformée en solution. Aucune ligne
n'est attribuée à un utilisateur nommé : ce sont les demandes auxquelles les refus et les arbitrages
de ce brief répondent déjà.

### Ce que demande celui qui veut s'arrêter

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « un rappel de pause toutes les heures » | être **empêché** de continuer, pas informé qu'il faudrait s'arrêter | une notification est un rappel, un overlay est une pause. Le produit intervient sur le poste de travail, jamais dans le centre de notifications |
| « qu'il me laisse choisir à quel point c'est strict » | décider **à froid** ce qu'il ne pourra plus négocier à chaud | la sévérité est un choix, mais elle se fige dès qu'une pause commence : une pause se déroule sous la sévérité avec laquelle elle a démarré (§10.2) |
| « une prévisualisation avant de choisir » | voir ce que le Mode Hardcore fait à son écran **avant** de le subir | l'onboarding montre les deux modes en animation. Découvrir le Hardcore en le subissant produit une désinstallation, pas un utilisateur convaincu (§8.8) |

### Ce que demande celui qui est en train de travailler

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « un bouton pour reporter quand je suis occupé » | obtenir un **délai**, pas une annulation | le report existe, il dure cinq minutes, il est comptable, et il se paie sur le même budget que tout ce qui repousse une pause. Un report gratuit et illimité est un rappel passif déguisé (§10.2) |
| « qu'il ne se déclenche pas pendant mes réunions » | que le produit reconnaisse ce qui, dans une journée, n'est **réellement** pas négociable | l'inhibition existe, et elle est plafonnée par le même budget. Sans plafond, « je suis en réunion » devient la phrase qui annule toutes les pauses |
| « pouvoir couper le truc si j'ai vraiment besoin de mon Mac » | ne jamais être **piégé** par un logiciel | la sortie d'urgence est visible en permanence, n'est jamais refusée, et coûte au cycle suivant les minutes de pause non faites — puis, budget vide, sa friction de dix secondes lui tient lieu de prix. Un garde-fou qu'on épuise n'en est plus un ; un garde-fou sans friction n'en est pas un non plus (§8.6, §10.2) |
| « qu'il ne compte pas quand je ne suis pas devant » | ne pas devoir une pause qu'on a déjà prise en s'absentant | une absence plus longue que la pause due **vaut** la pause. Le gel de la phase de travail n'est pas une fin, c'est le premier palier (§10.4) |

### Ce que demande celui qui installe un logiciel qui prend son écran

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « bloquer Slack et Figma, mais pas Spotify » | distinguer ce qui **retient** de ce qui **accompagne** | trois statuts et non deux. Le troisième, `ignoré`, dit « ceci n'est pas du travail » et arrête le décompte : c'est un aveu de l'utilisateur sur son propre usage, pas une exemption d'overlay (§8.3) |
| « je ne veux pas qu'une appli me surveille » | la **preuve**, et non la promesse, que rien ne sort de la machine | aucune sortie réseau hors la vérification de mise à jour, déclarée et désactivable. Jamais de titre de fenêtre, jamais de contenu d'écran, jamais de frappe clavier (§10.6) |
| « ça doit marcher tout seul » | ne pas avoir à reconfigurer après une veille, une mise à jour ou un plantage | l'échéance persistée fait autorité, jamais un décompte tenu en mémoire ; et **aucun overlay n'est jamais posé au lancement** (§10.4) |

### Le besoin que personne ne formule

**Aucune de ces demandes ne dit ce qui rend le produit viable : pouvoir déléguer une décision à
soi-même passé, et ne plus pouvoir la reprendre.** C'est la seule raison pour laquelle Breeze existe
plutôt qu'un rappel de plus. Toutes les règles de négociation du §10.2 servent ce besoin unique, et
c'est de lui que sort la première fonctionnalité décisive du §9.

---

## 8. Le produit, fonctionnalité par fonctionnalité

**Huit fonctionnalités, et rien d'autre au lancement.** Chacune est décrite par ce qu'elle fait, ce
qu'elle exige, et ce qu'elle refuse.

### 8.1 Le cycle de travail et de pause

**Ce qu'elle fait.** Compte une phase de travail, puis impose une phase de pause, et recommence tant
que Breeze est actif. La mécanique complète des états est en §10.1.

**Ce qu'elle exige.** Rien. C'est la seule fonctionnalité qui tourne sans permission, sans réglage et
sans configuration : un utilisateur qui ferme l'onboarding à la première fenêtre a un cycle qui
tourne.

**Ce qu'elle refuse.** Aucune commande d'arrêt du cycle. Qui veut arrêter Breeze le suspend, ou le
quitte. Un bouton « arrêter » serait un troisième levier de négociation, gratuit et illimité.

### 8.2 Le réglage du rythme

**Ce qu'elle fait.** Fixe la durée de travail, la durée de pause, la plage horaire active et les
jours actifs. Trois rythmes prêts à l'emploi sont proposés à l'onboarding, et le réglage fin reste
accessible.

**Ce qu'elle exige.** Des valeurs dans les bornes du §12.2, et une pause qui ne dépasse jamais la
durée de travail.

**Ce qu'elle refuse.** Aucun jour actif, et aucune plage horaire dont le début égale la fin : l'une
comme l'autre enfermerait le cycle dans un état sans aucune sortie.

**Une plage dont la fin précède le début traverse minuit, et elle est admise** — 22 h à 2 h est un
rythme de travail réel, pas une erreur de saisie.

**Le jour actif s'évalue en continu**, comme la plage horaire, et son passage à inactif reste l'une
des deux causes de [INACTIF] (§10.1). **Une seule exception, et elle est propre aux plages qui
traversent minuit** : une plage ouverte un jour actif reste ouverte jusqu'à sa fin, même si minuit la
fait passer à un jour inactif. Sans elle, un rythme 22 h – 2 h se couperait chaque nuit à minuit,
et l'utilisateur n'aurait aucun moyen de le dire autrement.

La journée des statistiques, elle, reste celle du §12.2 — de minuit à minuit — et un cycle à cheval
alimente les deux journées qu'il touche. Aucun changement ne s'applique à
une phase déjà en cours, et **aucun changement qui affaiblit la contrainte ne s'applique à une pause
déjà due** — §10.2 porte cette règle pour tous les réglages du produit.

### 8.3 Le statut des applications

**Ce qu'elle fait.** Donne à chaque application connue de Breeze l'un de trois statuts, qui décide de
deux choses distinctes : ce que la pause lui fait, et ce que son usage fait au décompte.

| Statut | Pendant une pause en Mode Simple | Effet sur le décompte du travail |
|---|---|---|
| **Bloquée** | Recouverte par un overlay | Son usage compte comme du travail |
| **Épargnée** | Utilisable librement | Son usage compte comme du travail |
| **Ignorée** | Utilisable librement | Son usage **ne compte pas** comme du travail |

**Ce qu'elle exige.** Rien de la part de l'utilisateur : toute application inconnue est **bloquée**
dès sa première apparition au premier plan. Un défaut permissif produirait une première pause sans
effet, ce qui casse la promesse avant que l'utilisateur l'ait éprouvée. L'onboarding sert donc à
**épargner** ce dont on a besoin pendant une pause, jamais à choisir ce qu'on bloque.

**Ce qu'elle refuse.**

- Le statut n'a **aucun effet visuel en Mode Hardcore** : l'overlay y couvre chaque écran entier, et
  le statut ne sert plus qu'au décompte.
- Les applications de la liste de sécurité (§10.6) sont épargnées en dur, sans contrôle de statut,
  avec la mention « Toujours autorisée ».
- **Un changement de statut ne retire jamais un overlay déjà posé, et ne gèle jamais un décompte en
  cours.** Épargner ou ignorer une application vaut au **cycle suivant** — pour l'overlay comme pour
  le décompte ; bloquer ou épargner une application ignorée vaut immédiatement. C'est la règle d'or
  du §10.2 : ce qui renforce s'applique tout de suite, ce qui affaiblit attend.
  Sans cette symétrie, passer en `ignorée` l'application au premier plan à la quarante-neuvième
  minute d'une phase de cinquante gèlerait la pause à venir sans rien coûter, et la repasser en
  `bloquée` plus tard rendrait la phase intacte.

**Un statut « ignorée » gèle la phase de travail, et c'est voulu.** Y rester indéfiniment fait que la
pause n'est jamais due, et rien n'est débité. Ce n'est pas une échappatoire à fermer : `ignorée` veut
dire « ceci n'est pas du travail », et qui reconfigure son outil à froid a fait un choix conscient,
exactement comme il aurait suspendu Breeze. Le produit se défend contre celui qui négocie **au moment
où la pause arrive**, pas contre celui qui décide de ce qui compte.

**C'est aussi pourquoi le basculement ne vaut qu'au cycle suivant.** L'arbitrage couvre celui qui
décide à froid ce que son travail est ; il ne couvre pas celui qui le redéfinit à la quarante-neuvième
minute pour échapper à la pause qui vient. La temporalité est ce qui sépare les deux, et c'est la
seconde moitié de la règle d'or (§10.2) qui la porte.

### 8.4 Le déclencheur et les exceptions

**Ce qu'elle fait.** Deux réglages qui décident **quand** le décompte avance et **quand** une pause
due peut attendre.

- **Le déclencheur intelligent**, désactivé par défaut, restreint l'avancée du décompte aux seules
  applications d'une liste que l'utilisateur coche. Activé, il rend le statut `ignorée` sans effet
  sur le décompte, puisque seule l'appartenance à la liste le fait avancer. Une application peut être
  à la fois déclencheuse et bloquée — c'est même le cas courant.
- **Les exceptions automatiques** — appel visio, partage d'écran, présentation plein écran, et une
  plage d'exception horaire libre — repoussent une pause due dans la limite du budget de report
  (§10.2).

**Ce qu'elle exige.** Au moins une application cochée pour que le déclencheur intelligent s'active.

**Ce qu'elle refuse.**

- **L'interrupteur ne s'arme pas sur une liste vide, et la liste ne se vide pas tant qu'il est
  armé.** Un cycle armé sans aucun déclencheur n'entrerait plus jamais en phase de travail, et rien à
  l'écran ne l'annoncerait. Décocher la dernière application désactive l'interrupteur.
- Ces deux refus portent sur des **commandes**. Quand la liste se vide sans commande — la dernière
  application cochée est désinstallée —, Breeze ne touche pas au réglage de l'utilisateur :
  l'interrupteur reste armé, le décompte retombe sur « tout usage compte comme du travail » tant que
  la liste effective est vide, et un bandeau le dit.
- **Une visio tenue dans un onglet de navigateur n'est pas détectable**, puisque cela demanderait de
  lire le titre de fenêtre — ce que le §13 interdit. Breeze l'assume, et l'interface le dit à
  l'endroit du réglage.
- **Activer une exception vaut au cycle suivant ; la désactiver vaut immédiatement.** C'est la règle
  d'or du §10.2 appliquée aux exceptions. Sans elle, activer « ne pas interrompre entre 12 h et
  14 h » sur l'heure courante, pendant un préavis, repousserait la pause due sans rien débiter.
- **Le déclencheur intelligent obéit à la même règle, et c'est le levier le plus direct qu'elle
  couvre.** Armer l'interrupteur, ou retirer une application de la liste, **vaut au cycle suivant** :
  les deux rétrécissent ce qui compte comme du travail, donc ralentissent le décompte. Désarmer, ou
  ajouter une application, vaut **immédiatement**. Sans cette règle, armer l'interrupteur sur une
  application qu'on n'ouvre jamais, à la quarante-neuvième minute d'une phase de cinquante, gèlerait
  la phase de travail pour toujours et sans rien débiter — et le §13 ne le rattraperait pas, puisque
  aucune pause ne deviendrait jamais due.

### 8.5 Le Mode Simple

**Ce qu'elle fait.** Pendant la pause, recouvre chaque fenêtre visible de chaque application bloquée
d'un voile qui laisse deviner le contenu sans le rendre lisible, et porte le décompte. Le reste du
Mac reste utilisable : les applications épargnées et ignorées fonctionnent normalement, le son
continue, la musique en cours ne s'arrête pas.

**Ce qu'elle exige.** La permission Accessibilité. Sans elle, le comportement dégradé du §10.5 fait
autorité, et lui seul.

**Ce qu'elle refuse.**

- **Aucune application n'est masquée, quittée ni suspendue.** Breeze se pose devant, il ne touche à
  rien. Suspendre le processus d'une application de visio ou d'une base de données est une catastrophe
  utilisateur, pas une astuce (§13).
- Un clic sur l'overlay ne le ferme pas ; il rappelle le temps restant.
- **La pause reste contournable, et Breeze ne prétend pas le contraire** : amener une application
  épargnée par-dessus la zone couverte fonctionne. C'est la conséquence directe de la première
  contrainte du §5, et l'utilisateur qui veut une pause qui tient a le Mode Hardcore.
- **Terminer en avance est possible, mais pas tout de suite, et pas à tout prix.** Le bouton apparaît
  après une minute de pause écoulée, **et seulement si le budget couvre les minutes de pause non
  faites** (§10.2) ; il disparaît dès qu'il a été actionné. La pause est alors comptée écourtée et
  débitée. Budget insuffisant, le bouton n'apparaît pas — et il réapparaît en fin de pause, quand ce
  qu'il resterait à effacer redevient payable.

**Quand aucune application bloquée n'a de fenêtre visible**, Breeze pose la bannière de pause seule,
avec le décompte. Une pause sans aucun signal visible serait indistinguable d'une pause manquée.

### 8.6 Le Mode Hardcore

**Ce qu'elle fait.** Pendant la pause, couvre l'intégralité de chaque écran physique d'un fond opaque
portant le décompte, identique sur tous les écrans — détourner le regard vers un autre écran ne doit
pas faire disparaître le compte à rebours. La barre de menus et le Dock sont masqués. Le changement
d'application, le changement d'espace et les raccourcis de fermeture sont neutralisés dans la limite
de ce que macOS autorise.

**Ce qu'elle exige.** Aucune permission. C'est le mode qui tient sa promesse le plus complètement, et
c'est aussi celui qui n'a rien à demander.

**Ce qu'elle refuse.**

- Aucun bouton de fermeture, aucun report, aucune suspension, aucun changement de sévérité pendant la
  pause.
- Aucune notification émise par Breeze pendant la pause, et aucune promesse sur celles des autres
  applications (§5).
- **Aucune prétention à l'infaillibilité.** Forcer à quitter, changer d'utilisateur, tuer le processus
  depuis un autre compte, débrancher l'écran ou éteindre le Mac restent possibles. C'est déclaré à
  l'onboarding, avant le choix du mode.
- **Aucune promesse sur une application en plein écran natif tant que la mesure du §12.3 n'a pas
  tranché.** macOS ne garantit pas qu'une fenêtre, si haute soit-elle, se pose au-dessus d'un tel
  écran (§5). Si la mesure est défavorable, la limite est déclarée à froid, au même endroit que les
  précédentes — jamais découverte pendant une pause.

**La sortie d'urgence.** Maintenir la touche d'échappement pendant dix secondes consécutives remplit
un anneau, puis ouvre une confirmation dont le bouton d'annulation est en position par défaut. La
mention « Maintiens Échap pour sortir » reste affichée en permanence : **cacher une sortie de secours
la transforme en piège.** Elle n'est pas rationnée — un garde-fou qu'on épuise n'en est plus un — mais
elle coûte : la pause est comptée interrompue, et le budget du cycle suivant est diminué des minutes
de pause non faites (§10.2). Sortir à la deuxième minute d'une pause de dix en coûte huit.

**Le décompte de pause ne s'arrête jamais** — ni pendant les dix secondes de maintien, ni pendant que
la confirmation est ouverte. Annuler laisse la pause exactement où elle en est, et la confirmation se
rouvre autant de fois qu'on veut : un garde-fou ne se rationne pas davantage à l'échelle du geste
qu'à celle du cycle. Si l'échéance tombe pendant que la confirmation est ouverte, l'échéance l'emporte
(§10.2) : la pause est comptée **prise**, et la confirmation se ferme sans effet.

### 8.7 La barre de menus et les réglages

**Ce qu'elle fait.** Loge Breeze dans la barre de menus, sans icône dans le Dock. L'icône porte l'état
du cycle, et un panneau donne accès au décompte, aux leviers autorisés à cet instant, à la sévérité,
au bilan du jour et aux réglages.

| État du cycle | Ce que porte l'icône |
|---|---|
| Inactif, ou en attente d'un déclencheur | Contour de feuille, ajouré |
| Suspendu | Contour de feuille barré |
| Travail en cours | Feuille pleine, anneau de progression qui se remplit |
| Préavis | Anneau complet, en pulsation |
| Pause due, en attente d'une inhibition | Anneau complet, sans pulsation |
| Pause en cours | Feuille pleine inversée |
| Retour, les trois secondes qui suivent la pause | Feuille pleine, anneau vidé |

Un badge d'avertissement se superpose à l'icône quand une permission requise manque, ou quand le
coupe-circuit a désactivé les overlays. Le panneau dit lequel des deux, et propose la réparation.

**Ce qu'elle exige.** Rien.

**Ce qu'elle refuse.**

- **Un levier grisé porte toujours la raison de son grisement.** Un contrôle désactivé sans
  explication se lit comme un défaut.
- **Aucune entrée n'est plus permissive parce qu'elle passe par un raccourci.** Le panneau, le menu
  contextuel et les raccourcis globaux obéissent exactement aux mêmes règles (§10.3).
- Aucun raccourci n'annule une pause en cours.
- Pendant une pause en Mode Hardcore, la barre de menus est masquée et le panneau est donc
  inatteignable. C'est voulu.
- **La désinstallation propre s'arrête où macOS l'arrête** : Breeze supprime ses données et son
  élément de démarrage, se quitte, et laisse affichée la marche à suivre pour l'entrée d'Accessibilité
  qu'il ne peut pas retirer lui-même (§5).

**Les réglages** couvrent le général, le rythme, la sévérité et son message, les applications, les
statistiques des trente derniers jours, l'état des permissions et l'aide. **Les quotas et le budget
n'y figurent pas** : ce sont des constantes du produit (§12.2).

### 8.8 La première ouverture

**Ce qu'elle fait.** Amène l'utilisateur à un cycle qui tourne en moins de quatre-vingt-dix secondes,
et lui fait **comprendre ce que Breeze va faire à son écran avant que ça arrive**. Six écrans :
bienvenue, rythme, sévérité, permissions, applications, démarrage.

**Ce qu'elle exige.** L'écran de la sévérité porte une **prévisualisation animée des deux modes** —
une maquette de bureau où l'overlay vient se poser. C'est le point de décision produit le plus
important du parcours : personne ne doit découvrir le Mode Hardcore en le subissant.

**Ce qu'elle refuse.**

- **Aucune permission n'est obligatoire pour continuer.** Breeze fonctionne alors en mode dégradé
  (§10.5) et pose un bandeau de réparation tant qu'une permission requise par la sévérité choisie
  manque.
- **Fermer la fenêtre avant la fin ne laisse jamais Breeze inerte** : ce qui a été choisi est
  conservé, le reste prend les valeurs par défaut du §12.2, et le premier cycle démarre.
- L'écran des applications est passable, et le passer conserve le défaut — donc tout est bloqué.
- L'onboarding n'est jamais rejoué automatiquement. Il reste accessible depuis l'aide.

---

## 9. Les fonctionnalités décisives

Trois fonctionnalités que personne n'a demandées, et qui changent la raison de choisir Breeze plutôt
qu'un autre. **Aucune n'entre au lancement** : le §16 dit ce que chacune attend. Elles figurent ici,
et non au §8, pour que nul ne les confonde avec ce qui était demandé.

### 9.1 Le bouclier de calendrier

**Ce qu'elle apporte.** Un overlay Hardcore qui tombe pendant une réunion client détruit la confiance
**en une seule occurrence**, et aucune heuristique d'application ne rattrape ça : la visio peut être
en arrière-plan, la réunion peut se tenir au téléphone, la présentation peut passer par un second
écran. Breeze lit le calendrier local — **les créneaux occupés et le statut de disponibilité,
jamais les titres, jamais les participants** — et en tire deux comportements : il **décale** une pause
qui tomberait dans un créneau occupé vers le premier trou disponible, et il **place** les pauses dans
les interstices naturels de la journée plutôt qu'au milieu d'un bloc de travail.

**Pourquoi c'est décisif.** Elle fait passer Breeze de « logiciel qu'on désactive les jours chargés »
à « logiciel qu'on garde justement les jours chargés ». Le Mode Hardcore devient tenable en
environnement professionnel, ce qui ouvre le segment le plus disposé à payer.

**Ce qu'elle coûte.** Une permission de plus, Calendriers — donc une demande de confiance
supplémentaire, à un produit qui bâtit tout sur le fait de n'en demander aucune. Elle **interdit
désormais** de lire quoi que ce soit d'autre que les créneaux : la discipline doit être affirmée dans
l'interface, faute de quoi le refus de collecte du §10.6 devient invérifiable par l'utilisateur.
Avancer une pause ne consomme rien ; la repousser puise dans le budget de report comme tout autre
mécanisme (§10.2).

### 9.2 La dette de posture

**Ce qu'elle apporte.** Les applications de pause traitent chaque cycle comme indépendant : une pause
ignorée disparaît sans conséquence, et le produit devient du bruit en deux semaines. Chaque report et
chaque pause écourtée crédite une **dette** en minutes, visible en permanence sur l'icône. Elle se
rembourse — la pause suivante est allongée à hauteur de la dette, dans une limite fixée — et au-delà
d'un seuil, elle **durcit la contrainte** : Breeze passe en Mode Hardcore pour le cycle suivant, avec
un préavis explicite. La dette s'efface chaque nuit.

**Pourquoi c'est décisive.** C'est la réponse directe au besoin que personne ne formule (§7) :
déléguer une décision à soi-même passé. L'escalade est **annoncée, jamais subie**, et c'est
l'utilisateur qui l'a armée à froid — ce qui la rend légitime là où la même contrainte imposée par un
tiers serait insupportable.

**Ce qu'elle coûte.** Elle **interdit désormais** de traiter les cycles indépendamment : tout compteur
du produit devient un état persistant à faire survivre aux veilles, aux plantages et aux mises à
jour. Et elle déplace la charge sur l'utilisateur au pire moment — celui où il a déjà reporté trois
fois. Le réglage des seuils ne se fait pas au bureau : il se mesure en test utilisateur (§12.3).

### 9.3 La détection de session profonde

**Ce qu'elle apporte.** Interrompre quelqu'un à la minute 47 d'une concentration profonde est un
dommage net, même pour sa santé — et c'est l'objection numéro un de la cible : « ça va me couper en
plein travail ». Breeze calcule un **indice de profondeur** à partir de signaux qu'il possède déjà et
sans permission supplémentaire — la fréquence des changements d'application, la durée de session
continue sur une même application, et la régularité de l'activité déduite du **seul temps écoulé
depuis la dernière action**, exactement la mesure qui sert déjà à geler la phase de travail (§12.2).
Aucun de ces trois signaux ne lit une frappe, et aucun ne demande une permission que Breeze n'a
pas déjà. En profondeur élevée, le préavis devient une fenêtre de
tolérance qui attend le premier changement d'application, signal de la fin naturelle du bloc ; et un
bilan hebdomadaire propose d'ajuster le rythme à la durée réelle des blocs.

**Pourquoi c'est décisive.** Breeze passe du rayon santé au rayon productivité, où le consentement à
payer est plus élevé, sans rien retirer à la promesse santé.

**Ce qu'elle coûte.** Elle **interdit désormais** toute interception d'événements clavier : un tel
mécanisme serait un dépassement de permission et mettrait la notarisation en jeu — donc le produit
entier. Elle **interdit aussi l'apprentissage automatique** : des moyennes glissantes et deux seuils
suffisent, et restent explicables à l'utilisateur, ce qu'un modèle ne serait pas. La tolérance qu'elle
accorde puise dans le budget de report et ne s'y ajoute pas.

---

## 10. Les règles transverses

Ce qui vaut partout, et qu'aucune fonctionnalité ne redit.

### 10.1 Le cycle et ses états

```
   ┌──────────────────────── fin de plage horaire, jour inactif ──────────────┐
   │                                                                          │
   ▼                                                                          │
[INACTIF] ──démarrer──▶ [ARMÉ] ──déclencheur au premier plan──▶ [TRAVAIL] ────┤
                           ▲                                        │         │
                           └──perte du déclencheur──────────────────┤         │
                                  (l'inactivité gèle sur place)     │         │
                                                       fin du décompte        │
                                                                    ▼         │
                          [TRAVAIL] ◀──report accordé (§10.2)──[PRÉAVIS] ─────┤
                                                                    │         │
                                             budget épuisé ou échéance atteinte
                                                                    ▼         │
                                                          [PAUSE ACTIVE] ─────┤
                                                                    │         │
                                                       fin du décompte        │
                                                                    ▼         │
                                                             [RETOUR] ────────┤
                                                                    │         │
                                                                    ▼         │
                                                              [ARMÉ] ─────────┘
```

**Ce que le diagramme ne montre pas, et qui le ferme :**

- Quand le déclencheur intelligent est désactivé — le défaut —, **[ARMÉ]** est traversé sans attente.
- **[RETOUR]** revient toujours en **[ARMÉ]**, jamais directement en **[TRAVAIL]** : le cycle suivant
  respecte la condition de déclenchement et la plage horaire comme le premier.
- Un cycle qui commence part d'un décompte de travail **plein**. Une phase de travail interrompue par
  la perte du déclencheur ou par l'inactivité est **gelée**, pas remise à zéro.
- **L'inactivité gèle la phase de travail sur place, dans [TRAVAIL] ; elle ne fait transiter vers
  aucun autre état**, et le retour de l'activité la dégèle. **Elle exige une machine éveillée et une
  session déverrouillée** : le compteur d'inactivité ne court ni pendant la veille ni pendant un
  verrouillage, que le §10.4 traite seul. Sans cette réserve, fermer le capot huit minutes gèlerait
  la phase au bout de trois et repousserait l'échéance gratuitement, ce que le §10.1 refuse par
  ailleurs à la veille. Seule la perte du déclencheur ramène en
  **[ARMÉ]**. Sans cette distinction, un utilisateur qui ne règle rien — donc sans déclencheur
  intelligent, où [ARMÉ] est traversé sans attente — n'aurait jamais de gel du tout.
- **Ni le préavis ni l'inhibition ne dépendent du déclencheur.** Une fois le décompte de travail
  terminé, le déclencheur n'a plus d'objet : le préavis va à son terme et le débit d'inhibition
  continue, quelle que soit l'application au premier plan. Le déclencheur ne gouverne que l'avancée
  du décompte de travail, et rien d'autre.
- **[RETOUR] dure trois secondes** (§12.2), et rien ne l'en fait sortir que leur écoulement. Aucun
  levier n'y vaut, et aucun événement ne l'y retient.
- **[INACTIF]** a exactement deux causes : la fermeture de la plage horaire et le passage à un jour
  inactif. Il n'existe aucune commande d'arrêt du cycle (§8.1). Il a aussi deux sorties :
  l'ouverture de la plage horaire ou le passage à un jour actif — sans quoi Breeze resterait inerte
  du soir au lendemain matin — et le démarrage de l'application.

**Trois états transversaux se superposent au cycle :**

- **[SUSPENDU]** — l'utilisateur a suspendu Breeze pour l'une des durées du §12.2. Une seconde
  commande de suspension **remplace** l'échéance de la première, elle ne s'y ajoute pas.
  **Une suspension est révocable** : reprendre plus tôt que prévu renforce la contrainte, et la règle
  d'or ne s'y oppose donc pas.
  **Une suspension gèle la phase de travail, elle ne la remet pas à zéro** : à la reprise, le
  décompte repart là où il s'était arrêté. Sans cette règle, suspendre une minute puis reprendre
  effacerait une phase de travail presque finie, gratuitement et à volonté — et ce serait le chemin
  le moins cher du modèle.
  **Une phase gelée n'expire pas** : une suspension jusqu'au lendemain matin rend le décompte tel
  qu'il était, à la minute près. La fermeture de la plage horaire, elle, **perd** le décompte en
  cours (plus bas). La différence est voulue : suspendre est un geste de quelqu'un qui compte
  revenir, la fermeture de la plage est la fin de la journée. Aucun des deux n'affaiblit la
  contrainte — l'un conserve le travail fait, l'autre le sacrifie — et aucun n'est commandable au
  moment où la pause approche.
  **L'échéance d'une suspension est une échéance murale** : elle court pendant que la machine dort,
  sinon fermer le capot prolongerait la suspension sans limite. À son terme, Breeze repart en
  **[ARMÉ]** si la plage horaire **et** le jour actif l'autorisent, en **[INACTIF]** sinon.
- **[VEILLE]** — le Mac dort ou la session est verrouillée. **La veille ne gèle jamais une
  échéance** : elle gèle l'affichage, le décompte d'une pause en cours, et le **budget de report**,
  qu'une inhibition ne consomme donc pas pendant que la machine dort. Ce qui se passe au réveil est
  tranché par le §10.4, et lui seul. Geler l'échéance d'une phase de travail ferait de la mise en
  veille un report gratuit et répétable.
- **[INHIBÉ]** — une pause est due, mais une condition extérieure la repousse. L'attente consomme le
  budget de report ; budget épuisé, la pause démarre malgré la condition.

**Quatre précisions ferment [INHIBÉ] :**

1. **L'inhibition s'évalue en niveau, à l'instant où la pause devient due, jamais sur son front.**
   Une visio ouverte vingt minutes avant la fin de la phase inhibe donc bien la pause — c'est même le
   cas dominant, et lire un front laisserait l'overlay tomber en pleine réunion.
2. **L'ensemble des causes reste ouvert.** L'état tient tant qu'au moins une condition tient, y
   compris une condition apparue **après** l'échéance. Le budget est débité **une seule fois** pour un
   intervalle recouvert : deux causes concurrentes ne le vident pas deux fois plus vite.
3. **Quand l'échéance est franchie sans que personne ne puisse l'observer** — processus mort, écran
   verrouillé —, les causes sont constatées **à la reprise**, et le budget débite à partir de là.
   Poser l'overlay au réveil sans regarder ferait tomber une pause Hardcore en pleine visio.
4. **Quand la dernière condition cesse, la pause démarre**, sans préavis rejoué et sans restitution du
   budget consommé. Rejouer le préavis repousserait encore la pause sans rien débiter, et ré-offrirait
   un report que l'inhibition venait d'interdire.

**[PRÉAVIS] et [INHIBÉ] ne se composent pas.** Une condition qui commence pendant le préavis ne
produit rien avant l'échéance : le report reste offert jusque-là, et il n'existe aucun report pendant
l'inhibition.

**Fin de plage horaire et jour inactif suivent la même règle.** Pendant **[TRAVAIL]** ou **[ARMÉ]**,
Breeze passe en **[INACTIF]** immédiatement, et le décompte en cours est perdu. Pendant
**[PRÉAVIS]**, **[INHIBÉ]**, **[PAUSE ACTIVE]** ou **[RETOUR]**, la pause va à son terme, puis
Breeze passe en **[INACTIF]**. **Une pause commencée n'est jamais coupée par l'horloge.**

**Quitter l'application reste possible à tout instant**, et n'est refusé nulle part : c'est la
conséquence directe du §15, décision 3 — la triche est rendue coûteuse et consciente, jamais
impossible. Mais quitter détruit le **processus**, jamais ce qu'il a écrit : la phase de travail
entamée est persistée et reprend au relancement (§10.4). Une pause en cours, elle, est comptée
interrompue et débitée comme telle, puisqu'aucun overlay n'est jamais reposé au lancement.

### 10.2 Ce qui repousse une pause, et ce que ça coûte

Le produit tient parce que les échappatoires sont **chiffrées et non modifiables au moment où on en a
envie**.

**Le budget de report.** Chaque cycle dispose d'un budget unique, remis à plein **au démarrage du
cycle**, jamais à l'entrée en phase de travail — un report ramène en **[TRAVAIL]**, et recharger là
rechargerait le budget que le report vient de débiter. Tout ce qui repousse une **pause due** y puise,
au premier arrivé. Budget épuisé, la pause démarre, quelle que soit la raison invoquée. Un seul
plafond, donc aucun cumul possible entre mécanismes.

**Le budget se plancher à zéro, et aucun déficit ne se reporte.** Un débit supérieur au restant vide
le budget du cycle qu'il ouvre, sans jamais le rendre négatif ni contaminer les cycles d'après —
Breeze ne tient aucune dette au lancement, et celle qui existerait est une fonctionnalité à part
entière (§9.2).

**Un levier est refusé dès que le budget restant ne couvre pas son prix**, et non seulement quand ce
budget est nul. C'est le refus qui donne son sens au chiffrage, et sans lui rien ne le porterait : un
report demandé avec quatre minutes restantes en obtiendrait cinq, et l'affirmation du §12.2 —
« trois reports épuisent **exactement** le budget » — serait fausse.

Le prix se lit levier par levier, à l'instant où il est actionné :

| Le levier | Son prix | Ce qui se passe quand le budget ne le couvre pas |
|---|---|---|
| Reporter | 5 minutes | Le bouton n'est pas offert |
| Repousser par inhibition | Le temps réellement écoulé sous elle | L'inhibition cesse de repousser, et la pause démarre |
| Terminer la pause | Les minutes de pause non faites | Le bouton n'est pas offert. Il réapparaît quand il reste assez peu de pause pour que le budget la couvre |
| Suspendre ou redémarrer pendant une pause | Idem | La commande n'est pas offerte |
| Redémarrer le cycle hors pause | Les minutes de travail effacées | La commande n'est pas offerte |

**Une seule chose n'est jamais refusée : la sortie d'urgence.** Elle prend au budget ce qu'il peut
donner et pas davantage, et quand il ne peut plus rien donner **son coût devient sa friction seule**
— dix secondes de maintien et une confirmation, à chaque fois, sans jamais diminuer avec la
répétition, et chaque usage compté. C'est un arbitrage assumé : un garde-fou dont le prix monterait
finirait par se refuser, et un garde-fou qui se refuse est un piège (§8.6).

**Le budget borne le recul d'une pause due, et le prix d'une phase de travail effacée.** Suspendre
depuis **[TRAVAIL]** ne coûte rien : une suspension **gèle** la phase, elle ne l'efface pas (§10.1).
Redémarrer le cycle, en revanche, l'efface, et **son prix est les minutes de travail effacées**.

Il en découle une borne simple : **on ne redémarre un cycle que tant que le budget peut payer ce
qu'on efface.** Avec un budget plein, cela laisse un quart d'heure au début d'une phase. Passé ce
point, la commande n'est plus offerte : la pause approche, et l'effacer n'est plus un redémarrage,
c'est une négociation.

**Et un cycle ouvert par un redémarrage n'ouvre jamais un budget plein.** Il reçoit le budget restant
du cycle effacé, diminué du prix payé — la règle même de la pause avortée, plus bas. Sans cette
seconde borne, la première serait sans effet : redémarrer à la septième minute d'une phase coûterait
sept minutes à un budget rechargé à quinze, et la boucle « travailler sept minutes, cliquer,
recommencer » tournerait indéfiniment sans qu'aucune pause n'arrive jamais. Avec elle, elle s'arrête
au troisième tour.

**La règle qui unifie les deux : seule une pause servie rouvre un budget plein.** Ni un redémarrage,
ni une pause avortée, ni une pause débitée au relancement — une pause menée jusqu'au bout de son
décompte, ou validée par une absence (§10.4), et rien d'autre.

**Le prix se compte sur la phase, gelée comprise.** Une phase gelée par la perte du déclencheur garde
sa valeur : redémarrer depuis **[ARMÉ]** avec quarante-neuf minutes gelées coûte quarante-neuf
minutes, donc est refusé. Sans cela, passer au premier plan une application non déclencheuse puis
redémarrer serait un effaceur gratuit, et ce serait la dernière porte ouverte du modèle.

**Redémarrer n'est pas le geste qui absorbe une interruption.** Une interruption se traite par la
suspension, qui **gèle** — on revient de réunion et le décompte reprend où il en était, pause comprise
si elle est due. Redémarrer sert à repartir d'une phase pleine quand on vient de commencer et qu'on
s'est trompé de moment, et la borne du quart d'heure correspond exactement à cet usage.

Tarifer sans refuser n'aurait rien fermé. Un prix plafonné par un budget vide s'annule au deuxième
usage : le premier redémarrage créerait l'état dans lequel tous les suivants seraient gratuits, et
« redémarrer à la minute 49 » resterait un clic par cycle, indéfiniment, sans qu'aucune pause
n'arrive jamais. **Un prix qui s'annule à la répétition n'est pas un prix.**

Les deux leviers deviennent en outre **impossibles dès qu'une pause est due**, précisément pour que
la distinction ne se transforme pas en porte de sortie.

**Les deux mécanismes ne débitent pas de la même façon.** Un **report** débite cinq minutes au moment
du clic, et rien ne lui est rendu si l'utilisateur commence sa pause plus tôt. Une **inhibition**
débite le temps réellement écoulé sous elle, à partir de l'instant où la pause devient due, jamais
avant : une visio pendant la phase de travail ne coûte rien, puisqu'elle ne repousse rien.

**Une pause avortée débite le temps de pause qu'elle n'a pas servi.** Seule une pause menée jusqu'au
bout de son décompte, ou validée par une absence (§10.4), ouvre le cycle suivant avec un budget plein.
Toute autre fin — écourtée en Mode Simple, levée par la sortie d'urgence, close par une suspension ou
un redémarrage acceptés — **reporte le budget restant, diminué des minutes de pause non faites**.

C'est le seul chiffrage qui ferme le chemin le plus court : sans lui, déclencher une pause depuis le
préavis puis la terminer après une minute annulerait dix minutes de pause pour le prix d'une,
indéfiniment. « Reconduire le budget entamé » n'y changerait rien, puisque rien n'aurait été débité —
il faut une quantité que le modèle possède déjà, et ce sont les minutes non faites. Avec cette règle,
deux pauses avortées épuisent le budget, et la troisième pause du jour n'est plus négociable.

**Un report repousse la pause de cinq minutes au total, préavis rejoué compris.** Le nouveau préavis
est joué pendant la dernière de ces cinq minutes, et il ne débite rien de plus : les cinq minutes ont
déjà été payées au clic. Sans cette règle, chaque report coûterait cinq minutes et en rendrait six,
et trois reports en Mode Simple n'épuiseraient plus exactement le budget (§12.2).

**Une phase de travail rouverte par un report ne gèle plus.** Les cinq minutes accordées s'écoulent en
temps réel, quoi que fasse l'utilisateur : sans cette règle, passer sur une application ignorée après
avoir reporté suspendrait la pause due indéfiniment, pour cinq minutes de budget — la plus large
échappatoire du modèle. La veille et le verrouillage restent la seule exception, et ils ne créent pas
de faille : une absence qui dépasse ce délai est traitée par la règle d'absence du §10.4, exactement
comme toute absence survenue avant que la pause ait commencé.

**Le quota de reports se lit contre la sévérité du moment.** Changer de sévérité pendant
**[TRAVAIL]** — le seul endroit où c'est encore possible — vaut pour le reste du cycle : passer de
Hardcore à Simple y autorise davantage de reports, dans la limite du budget. **Le quota borne la
fréquence, le budget borne le total.** En Mode Simple, le quota n'est atteignable que si aucune inhibition n'a rien
débité ; dans tous les autres cas, c'est le budget qui s'épuise en premier.

**Chaque levier, et ce qu'il vaut selon la sévérité :**

| Levier | Mode Simple | Mode Hardcore |
|---|---|---|
| Reporter depuis le préavis | Plusieurs fois, **une seule par préavis** | Une fois |
| Repousser par inhibition | Jusqu'à épuisement du budget | Jusqu'à épuisement du budget |
| Terminer la pause en avance | Après une minute de pause écoulée, et si le budget couvre les minutes non faites | Impossible, hors sortie d'urgence |
| Déclencher une pause immédiatement | Immédiat | Immédiat |
| Redémarrer le cycle | Tant qu'aucune pause n'est due **et que le budget couvre les minutes de travail à effacer** ; pendant une pause, après une minute et au prix des **minutes de pause non faites** | Idem, mais impossible pendant une pause |
| Changer de sévérité | Tant qu'aucune pause n'est due. **Jamais dès qu'elle l'est** — préavis, inhibition et pause comprises | Idem |
| Suspendre Breeze | Tant qu'aucune pause n'est due, sans rien débiter ; pendant une pause, après une minute | Idem, mais impossible pendant une pause |
| Suspendre ou redémarrer **quand une pause est due** | **Impossible** | **Impossible** |
| Quitter l'application | Immédiat | Confirmation, avec rappel du compteur de pauses interrompues |

**La règle d'or, et elle a deux moitiés.**

1. **Aucun réglage qui affaiblit la contrainte ne prend effet sur une contrainte déjà due ou en
   cours**, et son symétrique : ce qui la renforce s'applique immédiatement.
2. **Aucun réglage n'efface une phase de travail déjà entamée.** C'est la moitié qui manquait, et
   c'est elle qui outille la frontière du §6 entre l'utilisateur à froid et l'utilisateur à chaud :
   sans elle, chaque réglage qui gèle ou remet le décompte à zéro devient un moyen d'empêcher la
   pause de **devenir** due, et le §13 ne rattrape rien puisqu'aucune pause n'est jamais due.

Tout réglage du produit est un cas particulier de ces deux moitiés, et **voici les huit** :

| Le réglage | Quand il affaiblit | Quand il renforce |
|---|---|---|
| Durée de travail | Cycle suivant | Cycle suivant |
| Durée de pause | Cycle suivant | Dès le démarrage de la pause suivante, préavis déjà annoncé compris |
| Plage horaire, jours actifs | Cycle suivant | Cycle suivant |
| Statut d'une application — overlay **et** décompte | Cycle suivant | Immédiat |
| Exception automatique | Cycle suivant | Immédiat |
| Interrupteur du déclencheur intelligent | **Armer** : cycle suivant | **Désarmer** : immédiat |
| Liste des déclencheurs | **Retirer une application** : cycle suivant | **Ajouter une application** : immédiat |
| Sévérité | Jamais dès qu'une pause est due | Jamais dès qu'une pause est due |

**« Hors pause » veut dire « tant qu'aucune pause n'est due »**, jamais « hors [PAUSE ACTIVE] ». La
sévérité est donc figée dès le préavis et pendant toute inhibition : sans cela, passer de Hardcore à
Simple pendant qu'une visio repousse la pause livrerait une pause contournable et écourtable, ce que
le §13 déclare impossible.

La sévérité est aussi la seule qui ne change **dans aucun sens** une fois la pause due : une pause se
déroule toujours sous la sévérité avec laquelle elle a été annoncée, sans quoi les compteurs
mesureraient un mode que la pause n'a pas eu.

**Quand une échéance et une commande tombent au même instant, l'échéance l'emporte.** Une pause dont
le décompte est arrivé à zéro est une **pause prise** ; le clic sur « Terminer la pause » parti trois
cents millisecondes trop tard n'en fait pas une pause écourtée. Sans cet arbitrage, les compteurs
mesureraient le hasard d'ordonnancement.

**Le délai d'une minute avant de pouvoir écourter se compte en temps de pause écoulé, gels déduits.**
Un verrouillage de session de cinq minutes à la trentième seconde d'une pause ne rend pas le bouton
disponible au déverrouillage : il reste trente secondes à faire.

### 10.3 Ce que chaque levier peut, et depuis où

Chaque levier a ses états d'origine, et il est **sans effet ailleurs**. La liste est exhaustive, et
elle est la même quelle que soit l'entrée employée — panneau, menu contextuel ou raccourci global.

| Levier | Ses seuls états d'origine |
|---|---|
| Reporter | [PRÉAVIS], une fois par préavis |
| Faire une pause maintenant | [ARMÉ], [TRAVAIL], [PRÉAVIS], [INHIBÉ] |
| Suspendre Breeze | [ARMÉ], [TRAVAIL] **avant tout report** · [PAUSE ACTIVE] en Mode Simple après une minute · [SUSPENDU], où la commande remplace l'échéance |
| Redémarrer le cycle | [ARMÉ], [TRAVAIL] **avant tout report** · [PAUSE ACTIVE] en Mode Simple après une minute — et dans tous les cas seulement si le budget couvre son prix (§10.2) |
| Reprendre Breeze | [SUSPENDU] |
| Terminer la pause | [PAUSE ACTIVE] en Mode Simple, après une minute, tant que l'échéance n'est pas atteinte |
| Sortie d'urgence | [PAUSE ACTIVE] en Mode Hardcore |
| Changer de sévérité | Tous les états **tant qu'aucune pause n'est due** — donc jamais depuis [PRÉAVIS], [INHIBÉ] ni [PAUSE ACTIVE]. C'est le seul levier que la règle d'or bloque **dans les deux sens** (§10.2) |
| Quitter l'application | Tous les états, sans exception. C'est le seul levier qui n'en a aucun : le refuser quelque part ferait de Breeze un logiciel dont on ne peut pas sortir (§15, décision 3). En Mode Hardcore pendant une pause, une confirmation s'interpose |

**Une phase de travail rouverte par un report n'est pas un état d'origine** pour la suspension ni pour
le redémarrage. La pause est déjà due ; le report n'a fait que la déplacer. Sans cette réserve,
reporter puis redémarrer effacerait la pause **et** rendrait les cinq minutes débitées.

**Ce qui suit ne vaut que pour les cinq leviers qui négocient le cycle** — reporter, faire une pause
maintenant, suspendre, redémarrer, terminer. **Aucun d'eux ne vaut depuis [INACTIF] ni [VEILLE]**, où
il contournerait la plage horaire ou s'adresserait à une machine que personne ne regarde. Depuis
**[SUSPENDU]**, seules « Suspendre » et « Reprendre » valent. Depuis **[RETOUR]**, aucun : il n'y a
plus rien à négocier. Et « Faire une pause maintenant » ne vaut pas pendant **[PAUSE ACTIVE]** : une
pause ne se redémarre pas.

**Changer de sévérité et quitter l'application ne sont pas de ces leviers**, et cette restriction ne
les touche pas. La sévérité se règle partout tant qu'aucune pause n'est due — y compris le soir, hors
plage horaire, ce qui est précisément le moment de l'utilisateur à froid (§6). Quitter vaut partout,
sans exception.

### 10.4 L'absence, la veille et la reprise

**L'échéance fait autorité, jamais le décompte affiché.** Le temps restant se lit toujours comme une
différence d'échéances, et l'échéance survit à la veille, au verrouillage et à l'arrêt de
l'application.

**Trois règles d'absence, selon la phase interrompue :**

1. **Absence avant que la pause ait commencé** — pendant [TRAVAIL], [PRÉAVIS] ou [INHIBÉ], par veille,
   verrouillage ou inactivité — **plus longue que la durée de pause du cycle** : le cycle est validé comme pause
   prise et repart à zéro. Le seuil ne dépend pas de l'état où l'absence a débuté : trente secondes
   plus tôt ou plus tard, une absence de trois heures reste une pause. Il est **figé au démarrage du
   cycle**, et **une durée de pause allongée en cours de cycle le relève d'autant** : sans le figeage,
   ramener la pause à une minute ferait passer deux minutes d'inactivité pour une pause prise ; sans
   le relèvement, allonger la pause laisserait une absence trop courte la valider quand même.
2. **Absence pendant [PAUSE ACTIVE]** plus longue que le temps de pause restant : la pause est
   validée, et Breeze enchaîne sur [RETOUR] au réveil.
3. **Absence plus courte que ce seuil, mais qui dépasse l'échéance de phase** : l'échéance produit ce
   qu'elle aurait produit si l'utilisateur était resté. Le préavis est **sauté** — il annoncerait un
   fait déjà arrivé. La pause démarre au réveil, **sauf si une cause d'inhibition tient à cet
   instant** : les causes sont constatées à la reprise (§10.1), et cette vérification passe en
   premier.

**Les autres états ne produisent aucun verdict.** Une absence, si longue soit-elle, ne vaut jamais
pause depuis **[ARMÉ]**, **[INACTIF]**, **[RETOUR]** ou **[SUSPENDU]** : aucune pause n'y est due,
aucun décompte de travail n'y court, et créditer une pause pour une nuit passée en [ARMÉ] rendrait le
premier cycle du matin gratuit. Seule l'échéance d'une suspension continue de courir, parce qu'elle
est murale (§10.1).

**Le verdict est rendu au retour de l'utilisateur**, jamais à l'instant où le seuil est franchi :
c'est le seul moment où il y a quelqu'un à qui le rendre.

**Le temps passé sans processus ne vaut jamais absence.** Aucun signal d'inactivité n'a pu être mesuré
pendant que Breeze était mort, et un verdict ne se rend pas sur une observation que personne n'a
faite. Quitter Breeze onze minutes en continuant de travailler ne produit donc pas une pause prise.

**Une pause due mais jamais commencée est, au relancement, comptée interrompue et débitée de sa durée
entière.** Elle n'est ni servie — aucun overlay n'est posé au lancement — ni effacée : elle coûte au
cycle qui s'ouvre autant qu'elle aurait duré. Quitter pendant un préavis coûte donc la pause entière,
et deux fois de suite laisse un budget vide. C'est le troisième sort possible d'une pause due, à côté
de la pause servie et de la pause validée par une absence, et il n'y en a pas d'autre.

**Aucun overlay n'est jamais posé au lancement.** C'est ce qui empêche un défaut de reprise de poser
un overlay irretirable. Une pause est donc **close, jamais reprise** : échéance passée, elle est
simplement finie ; échéance à venir, elle est comptée interrompue et le cycle repart en [ARMÉ], budget
reconduit et diminué des minutes de pause non faites. Reprendre une pause sans son overlay rendrait
contraignant un état qui ne l'est plus.

**Un arrêt volontaire persiste l'état comme n'importe quelle transition.** Quitter Breeze détruit le
processus, jamais ce qu'il avait écrit : au relancement, la phase de travail entamée reprend là où
elle en était, et l'échéance persistée décide comme après n'importe quelle interruption. Sans cette
règle, quitter puis relancer serait le dernier effaceur gratuit du modèle — deux gestes pour annuler
quarante-neuf minutes de travail et rouvrir un budget plein, indéfiniment.

**Seule une première ouverture n'a pas d'état.** Breeze y démarre en **[ARMÉ]** si la plage horaire
l'autorise, en **[INACTIF]** sinon.

### 10.5 Ce que Breeze fait quand une permission manque

**C'est la seule table qui décrit un mode dégradé.** Aucune autre section n'en décrit.

| Permission manquante | Ce qui continue de fonctionner | Ce qui change |
|---|---|---|
| **Notifications** seule | Tout | Le préavis passe par la bannière propre à Breeze. Aucune autre différence, dans les deux sévérités |
| **Accessibilité** seule, sévérité Hardcore | Tout | Aucune différence : le Mode Hardcore n'en a pas besoin (§8.6) |
| **Accessibilité** seule, sévérité Simple | Le cycle, le préavis, le décompte | Breeze ne peut pas suivre les cadres de fenêtres. Il pose un **overlay unique couvrant l'écran, fermable après une minute et tant que le budget couvre les minutes de pause non faites** — le fermer clôt la pause, qui est comptée écourtée et débite les minutes non faites, exactement comme le bouton « Terminer la pause » (§10.2). Le panneau affiche « Sans l'Accessibilité, ta pause est contournable. Autorise Breeze, ou passe en Hardcore », et un badge reste sur l'icône jusqu'à la réparation |
| **Les deux** | Le cycle en Hardcore | Cumul des deux lignes ci-dessus |

**L'overlay dégradé n'est pas une cinquième façon de clore une pause.** Il obéit au même délai d'une
minute et au même débit que la fin anticipée du Mode Simple. Sans cela, refuser une permission
deviendrait le moyen le plus rapide d'annuler toutes ses pauses.

**Budget insuffisant, il cesse d'être fermable**, exactement comme le bouton « Terminer la pause »
cesse d'être offert (§10.2). Il n'y a alors plus de sortie d'urgence — elle n'existe qu'en Mode
Hardcore —, et il en reste deux : les applications de la liste de sécurité, qui restent atteignables,
et quitter Breeze, qui n'est refusé nulle part (§15, décision 3). C'est le seul endroit du produit où
le Mode Simple devient aussi ferme que le Hardcore, et c'est le prix de deux pauses déjà annulées.

**Il s'efface devant la liste de sécurité, comme tout overlay du Mode Simple.** C'est du Mode Simple,
et le §10.6 y vaut sans exception : les réglages du système, le moniteur d'activité et les
applications de santé restent atteignables, y compris pendant la minute où l'overlay n'est pas
fermable. Ce serait absurde autrement — c'est précisément ce mode qui invite l'utilisateur à aller
accorder la permission qui lui manque.

**Une permission peut être retirée pendant que Breeze tourne.** Son état est donc revérifié en
continu, et sa perte applique la ligne correspondante de cette table, pose le badge, et propose la
réparation en un clic.

### 10.6 Les garde-fous et ce qui ne sort jamais de la machine

**La liste de sécurité, non modifiable.** Certaines applications ne reçoivent **jamais** d'overlay en
Mode Simple : les réglages du système, le moniteur d'activité, le terminal, le Finder, le trousseau
d'accès, la fenêtre d'ouverture de session, les applications d'accessibilité de macOS, et toute
application déclarant relever de la santé ou du médical. La liste est en dur et n'est pas éditable :
un utilisateur ne doit jamais avoir à négocier avec Breeze pour atteindre ses réglages système ou une
application de santé. En Mode Hardcore, elle ne s'applique pas — l'overlay couvre l'écran entier — et
la contrepartie est portée par l'accessibilité de l'overlay lui-même, lisible et annonçant le temps
restant.

**Le coupe-circuit.** Breeze compte les chutes de son propre processus. Au-delà d'un seuil rapproché
(§12.2), il démarre le cycle suivant **sans aucun overlay**, l'affiche dans le panneau, et n'y renonce
qu'après un délai sans nouvelle chute, ou quand l'utilisateur réarme les overlays depuis les réglages.
Un défaut qui pose un overlay Hardcore irretirable est un incident critique : **la sortie doit exister
avant lui.** Un coupe-circuit qui s'arme pendant une pause en cours ne l'interrompt pas ; il vaut pour
le cycle suivant.

**Une pause servie sans contrainte compte quand même.** Que l'overlay manque parce que l'Accessibilité
n'est pas accordée ou parce que le coupe-circuit l'a désactivé, la pause s'écoule et alimente les
compteurs comme les autres. Breeze **dit ce qu'il ne peut pas faire** — badge sur l'icône, bandeau
dans le panneau — plutôt que de cesser de compter.

**Rien ne sort de la machine.** Aucune télémétrie, aucun compte, aucune remontée d'usage, **même
consentie**. C'est une contrainte de conception, pas une option de réglage. **Seule exception,
déclarée et visible : la vérification de mise à jour**, qui transmet la version installée et l'adresse
réseau au serveur de publication. L'utilisateur peut la couper dans les réglages ; Breeze fonctionne
alors sans aucune sortie réseau.

**Ce que Breeze lit, et ce qu'il ne lit jamais.** Il lit le nom et l'identité de l'application au
premier plan, ainsi que la position et la taille de ses fenêtres. **Jamais le titre d'une fenêtre,
jamais le contenu de l'écran, jamais les frappes clavier.** Le journal local — circulaire, borné, et
exportable par l'utilisateur seul — contient les transitions d'état, les identités d'applications
vues, les permissions perdues et les erreurs. Il ne contient jamais de titre de fenêtre ni de contenu
applicatif.

### 10.7 Ce qui se passe quand — cas limites tranchés

| Situation | Comportement |
|---|---|
| Un écran est branché pendant une pause Hardcore | Il reçoit son overlay dans le délai de pose du §12.2 |
| Un écran est débranché pendant une pause | L'overlay correspondant disparaît, la pause continue |
| La session est verrouillée pendant une pause | La pause est gelée ; au déverrouillage, l'overlay est reposé au premier plan et le décompte reprend |
| Le Mac dort pendant une pause | §10.4, règle 2 |
| L'utilisateur s'absente pendant le travail, le préavis ou une inhibition, plus longtemps que la durée de pause du cycle | §10.4, règle 1 : le cycle est validé comme pause prise et repart à zéro |
| L'utilisateur s'absente moins longtemps que ce seuil, mais l'échéance est dépassée | §10.4, règle 3 : le préavis est sauté, la pause démarre au réveil sauf inhibition |
| Une application bloquée est lancée pendant une pause Simple | Elle reçoit son overlay dans le délai de pose du §12.2 |
| Une application épargnée recouvre la zone d'un overlay Simple | L'overlay qui l'intersecte s'efface tant qu'elle est au premier plan, puis revient |
| Breeze est quitté pendant une pause Hardcore | Confirmation, puis tout est détruit. Au relancement, la pause est comptée interrompue (§10.4) |
| La plage horaire se ferme pendant une pause ou un préavis | La pause va à son terme, puis [INACTIF] (§10.1) |
| Le processus tombe pendant une pause | Les overlays meurent avec lui. Au-delà du seuil du §12.2, le coupe-circuit s'arme (§10.6) |
| Deux instances de Breeze sont lancées | La seconde se termine immédiatement (§13) |
| Breeze est relancé alors qu'une phase était en cours | §10.4 : l'échéance persistée décide, et aucun overlay n'est posé. Une phase de travail reprend là où elle en était ; une pause due mais jamais commencée est débitée de sa durée entière ; une pause en cours est comptée interrompue. Jamais une absence : rien n'a pu être mesuré pendant que le processus était mort |
| Une application connue est désinstallée | Sa ligne disparaît. Son statut et son appartenance à la liste des déclencheurs sont conservés, et réappliqués si elle revient (§8.4) |
| L'heure système change pendant un cycle | Les échéances en cours ne bougent pas, et **la plage horaire ne se réévalue qu'au cycle suivant** — comme son réglage (§10.2). Sans cela, avancer l'horloge d'une heure à 17 h 59 fermerait la plage, effacerait une phase de travail presque finie, et la bascule saisonnière le produirait deux fois par an |
| L'utilisateur enchaîne les sorties d'urgence | Aucun rationnement, aucune escalade au lancement. Le compteur monte, le budget se vide, et c'est tout (§8.6) |
| Une mise à jour est disponible pendant un cycle | Elle est téléchargée, jamais installée avant le prochain lancement, et jamais vérifiée pendant une pause ou un préavis |

---

## 11. Comment le produit se paie

**Il ne se paie pas au lancement.** Breeze est gratuit, sans licence, sans activation, sans compte.

**Ce n'est pas une décision commerciale, c'est une conséquence du refus de collecte.** Toute
monétisation par licence en ligne rouvre la contrainte « rien ne sort de la machine » (§10.6) :
vérifier une licence, c'est établir une connexion, transmettre un identifiant, et tenir un registre
côté serveur de qui utilise le produit. Le refus de collecte est ce qui rend crédible une application
qui demande l'Accessibilité et prend l'écran entier ; le contredire pour encaisser au lancement
reviendrait à vendre la raison pour laquelle on installe le produit.

**Ce qui n'est jamais facturé, quelle que soit la monétisation retenue plus tard :** la pause
elle-même, la sortie d'urgence, et la désinstallation. Aucun garde-fou ne se paie — un garde-fou
payant est un piège.

**À quelle condition ce choix se révise.** Il se révise avec un modèle qui n'exige aucune vérification
en ligne — un achat unique vérifié hors ligne, ou une version payante distribuée séparément. Il ne se
révise pas par un abonnement à validation périodique, qui est précisément ce que le §10.6 interdit.

---

## 12. Les limites, et où vivent les chiffres

### 12.1 Ce que macOS fixe

Les versions couvertes sont macOS 13 Ventura et ultérieures, sur les deux architectures de processeur.
**La numérotation saute : Apple est passé de macOS 15 Sequoia à macOS 26 Tahoe**, et il n'existe
aucune version 16 à 25 (§18). Ce n'est pas une erreur de saisie, et une matrice de test qui les
chercherait perdrait son temps.

Tout ce que macOS impose au-delà est au §5, avec la conséquence produit de chaque contrainte.

### 12.2 Ce que le produit décide

Ces valeurs sont des **constantes**, pas des réglages : elles ne figurent nulle part dans l'interface
des réglages, et §6 dit pourquoi l'utilisateur à froid lui-même ne peut pas y toucher.

| Ce que le produit fixe | La valeur | Pourquoi elle est constante |
|---|---|---|
| Durée du préavis | 1 minute | Assez pour finir une phrase, trop court pour changer d'avis |
| Durée de [RETOUR] | 3 secondes | Le temps de dire que c'est fini. Rien ne l'écourte ni ne le prolonge |
| Durées de suspension offertes | 15 minutes, 1 heure, jusqu'au lendemain 6 h | Trois durées, et **aucune suspension sans terme** : une suspension qu'on oublie de lever est une désinstallation qui ne dit pas son nom |
| Borne du journal local | 7 jours, ou 5 Mo — la première atteinte l'emporte | Il porte les mesures du §12.3 : sept jours couvrent l'export hebdomadaire d'un participant de la bêta |
| Budget de report par cycle | 15 minutes | Tout ce qui repousse une pause due y puise. Un seul plafond, aucun cumul |
| Durée d'un report | 5 minutes | Trois reports en Mode Simple épuisent exactement le budget |
| Quota de reports par cycle | 3 en Mode Simple, 1 en Mode Hardcore | Le quota borne la fréquence, le budget borne le total (§10.2) |
| Délai avant de pouvoir écourter une pause | 1 minute de pause écoulée, gels déduits | En dessous, écourter deviendrait un réflexe et non une décision |
| Maintien de la touche pour la sortie d'urgence | 10 secondes | Insupportable pour tricher, acceptable en cas de besoin réel |
| Inactivité au-delà de laquelle la phase de travail gèle | 3 minutes | Rappeler une pause à quelqu'un déjà parti n'a pas de sens |
| Seuil d'armement du coupe-circuit | 3 chutes du processus en 5 minutes | Un défaut répété, pas un accident isolé |
| Désarmement automatique du coupe-circuit | 24 heures sans nouvelle chute | Ou un réarmement explicite depuis les réglages |
| Profondeur des statistiques | 30 derniers jours glissants, journée de minuit à minuit en heure locale | Au-delà, il faudrait un écran d'exploration que la promesse ne justifie pas |
| Bornes de saisie du rythme | 5 à 180 minutes de travail, 1 à 60 minutes de pause, la pause ne dépassant jamais le travail | Hors de ces bornes, le cycle cesse d'être un cycle |
| Délai de pose d'un overlay sur un écran ou une application apparus en cours de pause | 500 millisecondes | Au-delà, la fenêtre d'échappement devient exploitable |
| Délai de pose de l'overlay Hardcore à l'échéance | 200 millisecondes | La pause doit tomber, pas s'installer |

**Ce que le produit s'impose en plus**, parce qu'une application de barre de menus est jugée sur ce
qu'elle ne coûte pas : moins de 120 Mo de mémoire au repos, moins de 0,3 % de processeur en moyenne
sur dix minutes, et l'absence du palmarès système des applications à forte consommation d'énergie.

**Les valeurs par défaut du produit** — celles qu'un utilisateur qui ne règle rien obtient : 50
minutes de travail, 10 minutes de pause, sévérité Simple, plage horaire désactivée, les sept jours
actifs, déclencheur intelligent désactivé, statut `bloquée` pour toute application inconnue, les trois
exceptions de visio, de partage d'écran et de présentation activées, l'exception horaire désactivée,
minutes restantes affichées à côté de l'icône, lancement au démarrage activé, langue du système avec
l'anglais à défaut, thème du système, vérification des mises à jour activée. **Cette liste fait
autorité sur tout défaut du produit** ; aucune autre section n'en donne.

### 12.3 Ce qu'on ne connaît pas encore

**Une valeur qu'aucun fait ne fixe n'est pas une décision qui manque : c'est une mesure.** Chacune a
un responsable, une forme, et un **comportement en son absence**. **L'absence d'une valeur ne permet
jamais rien** : elle bloque, ou elle applique le repli le plus conservateur — la borne du côté sûr,
jamais une valeur inventée.

**Les mesures techniques.** Porteur : l'équipe technique, pendant la reconnaissance qui précède toute
écriture de fonctionnalité. Forme : un relevé par version de macOS couverte, versé au cadrage
technique.

| Ce qui se mesure | Régime en son absence |
|---|---|
| L'octroi de l'Accessibilité se propage-t-il sans relancer l'application, sur chaque version couverte ? | **Repli** : sur les versions concernées, l'onboarding propose une relance explicite au lieu d'attendre |
| Existe-t-il un signal de partage d'écran qui n'exige pas la permission d'enregistrement ? | **Repli** : l'exception « partage d'écran » est retirée de l'interface. Une exception qui n'existe pas ne repousse aucune pause |
| Existe-t-il un signal de présentation plein écran distinguable d'un simple plein écran ? | **Repli** : l'exception « présentation » est retirée de l'interface, comme la précédente |
| Lire l'application au premier plan exige-t-il l'Accessibilité sur une version couverte ? | **Repli** : sur cette version, le déclencheur intelligent est désactivé et grisé, et tout usage compte comme du travail — le côté sûr, puisque le décompte avance plus vite |
| L'overlay Hardcore se pose-t-il au-dessus d'une application tierce en **plein écran natif**, sur chaque version couverte ? Apple déconseille la technique et n'en garantit pas le résultat (§5, §18) | **Repli** : le Mode Hardcore déclare la limite **à froid** — à l'onboarding et au réglage de sévérité —, et une pause servie sans couverture compte comme les autres (§10.6). Le repli du côté sûr est de dire ce qu'on ne couvre pas, jamais de laisser croire qu'on couvre tout |
| Le masquage de la barre de menus tient-il face à une application tierce déjà en plein écran ? | **Repli** : le masquage est abandonné sur ce seul cas, sans autre conséquence |
| La notarisation accepte-t-elle un binaire portant ces niveaux de fenêtre et cet usage de l'Accessibilité ? | **Bloquant.** Aucun repli n'existe : un refus arrête le produit. C'est pourquoi la soumission a lieu avant toute écriture de fonctionnalité |

**Les mesures produit.** Porteur : le fondateur, sur une bêta privée. Forme : l'export manuel du
journal local, que chaque participant envoie lui-même — le refus de collecte du §10.6 interdit toute
remontée, et aucun consentement ne le rouvre. Hors bêta, ces quantités restent visibles du seul
utilisateur, dans ses statistiques.

| Ce qui se mesure | Régime en son absence |
|---|---|
| La part des installations qui ont encore un cycle actif après trente jours | **Repli** : aucun seuil n'est opposé au produit. Tant que la bêta n'a pas rendu ses chiffres, le brief fait foi, et aucune décision d'arrêt ou de pivot ne se prend sur une intuition |
| La part des pauses menées jusqu'à leur terme | **Repli** : idem |
| La part des utilisateurs qui ont choisi le Mode Hardcore au moins une fois pendant leur première semaine | **Repli** : idem. C'est aussi la mesure qui tranche la deuxième hypothèse du §17 |
| Le nombre de sorties d'urgence par utilisateur et par semaine, avec leur motif déclaré | **Repli** : idem. Les dix secondes du §12.2 restent la constante tant que rien ne les conteste |
| La part des désinstallations dans les deux jours suivant l'installation | **Repli** : idem |
| Les seuils de la dette de posture (§9.2) | **Bloquant** pour cette fonctionnalité : elle n'entre pas au lancement, et elle n'entrera pas tant que ses seuils n'auront pas été mesurés en test utilisateur. Une escalade automatique mal calibrée est pire que pas d'escalade du tout |

**Aucun chiffre de succès n'est fixé dans ce brief**, et c'est délibéré : une cible de rétention posée
avant la première mesure n'est pas un objectif, c'est un nombre auquel on finit par ajuster la
lecture des faits.

---

## 13. Ce qui est interdit

Chaque ligne est **un état qui doit être impossible**, pas un message d'erreur à afficher.

**Sur la contrainte :**

- Une pause due n'est jamais effacée. Elle connaît exactement trois sorts : elle est servie, elle est
  validée par une absence, ou elle est débitée comme interrompue quand le processus est mort avant
  qu'elle ait commencé (§10.4).
- Seule une pause servie rouvre un budget plein.
- Aucun chemin ne repousse une pause due sans débiter le budget du cycle.
- Aucun chemin ne rend au budget des minutes déjà débitées.
- Aucun réglage modifié pendant une pause due ou en cours ne l'affaiblit.
- Un levier refusé par le panneau n'est jamais accordé par un raccourci ou un menu contextuel.
- Une pause n'est jamais coupée par la fermeture de la plage horaire ni par un changement d'heure.
- Deux reports ne partent jamais d'un même préavis.
- Le budget de report n'est jamais rechargé ailleurs qu'au démarrage d'un cycle.
- **Aucune commande et aucun réglage n'efface une phase de travail entamée sans la débiter** — y
  compris quitter l'application, qui la persiste au lieu de la détruire (§10.4) —, et aucun ne la
  gèle sans attendre le cycle suivant.
- Aucun levier n'est offert quand le budget restant ne couvre pas son prix, hors la sortie d'urgence.
- Le budget ne devient jamais négatif, et aucun déficit ne franchit le cycle qu'il ouvre.
- La sortie d'urgence n'est jamais refusée, quel que soit l'état du budget.
- L'échéance d'une phase de travail, d'un préavis ou d'une suspension n'est jamais gelée par la
  veille ni par le verrouillage. Le décompte d'une pause **en cours**, lui, l'est (§10.1).
- La sévérité ne change jamais, dans aucun sens, dès qu'une pause est due.

**Sur l'état :**

- Le cycle n'entre jamais dans un état sans sortie : ni jours tous inactifs, ni plage horaire
  dégénérée, ni déclencheur armé sur une liste vide.
- Deux instances de Breeze ne posent jamais deux overlays.
- Aucun overlay n'est posé au démarrage de l'application.
- Aucun overlay ne survit à la fin de la pause qui l'a posé.
- Un raccourci global enregistré par Breeze ne survit jamais à la pause qui l'a fait enregistrer, ni à
  l'arrêt de l'application.

**Sur ce qui appartient à l'utilisateur :**

- Aucune application tierce n'est masquée, quittée ni suspendue.
- Aucune donnée ne quitte la machine, hors la vérification de mise à jour déclarée et désactivable.
- Aucun titre de fenêtre, aucun contenu d'écran, aucune frappe clavier n'est lu ni enregistré, à
  aucun moment, y compris dans le journal de diagnostic.
- Aucune permission n'est demandée pour une fonctionnalité qui ne s'en sert pas — ni l'enregistrement
  d'écran, ni le pilotage d'autres applications, ni le micro, ni la caméra.
- L'utilisateur n'est jamais empêché d'atteindre les réglages du système, le moniteur d'activité, ni
  une application de santé en Mode Simple.
- La sortie d'urgence n'est jamais cachée, jamais rationnée, jamais payante.
- Éteindre le Mac reste toujours possible.

---

## 14. Hors périmètre

Chaque ligne dit **pourquoi**, et si c'est une contrainte subie ou une décision produit — la seconde
se révise, la première non.

| Ce qu'on ne fait pas | Pourquoi, et à quelle condition ça se révise |
|---|---|
| **Windows et Linux** | Contrainte subie. Ce que Breeze fait repose sur des mécanismes propres à macOS sans équivalent. Ne se révise pas |
| **Distribution par le Mac App Store** | Contrainte subie (§5). Ne se révise pas tant que le bac à sable reste incompatible avec l'usage de l'Accessibilité et les niveaux de fenêtre nécessaires |
| **Désinstallation complète de l'autorisation d'Accessibilité** | Contrainte subie (§5). Breeze affiche la marche à suivre. Ne se révise pas |
| **Synchronisation entre appareils, compte, serveur** | Décision produit, et elle contredit le refus de collecte du §10.6. Ne se révise qu'avec ce refus, c'est-à-dire pas au lancement |
| **Mode équipe, classement social, partage de statistiques** | Décision produit, même raison. Et §3 : celui qui subit la contrainte est celui qui l'a posée, jamais un tiers |
| **Profils de réglages multiples et nommés** | Décision produit. Un seul jeu de réglages couvre le besoin du premier public ; la gestion de profils demande un écran entier pour un gain que rien ne mesure encore. Se révise si la bêta montre que les mêmes utilisateurs reconfigurent leur rythme plusieurs fois par semaine |
| **Statistiques au-delà de trente jours** | Décision produit. Un historique plus long demande un écran d'exploration que la promesse ne justifie pas. Se révise si la dette de posture (§9.2) entre, puisqu'elle donne un sens à la tendance longue |
| **Exercices guidés, contenu audio ou vidéo, conseils de santé** | Décision produit. Élargit le périmètre sans renforcer la promesse, et fait de Breeze un coach — ce qu'il n'est pas (§3). Ne se révise pas |
| **Blocage de sites, de domaines ou de contenus** | Décision produit. Demanderait de lire ce que l'utilisateur consulte, ce que le §13 interdit. Ne se révise pas |
| **Escalade automatique de la contrainte** | Décision produit, et c'est la dette de posture (§9.2). Attend ses seuils mesurés (§12.3) |
| **Rationnement de la sortie d'urgence** | Décision produit, définitive. Un garde-fou qu'on épuise n'en est plus un (§8.6). Ne se révise pas |

---

## 15. Les décisions, et ce que chacune écarte

Numérotées pour être citées depuis le reste du document. **Une décision sans sa contrepartie n'est pas
une décision, c'est une préférence.**

| # | La décision | Ce qu'elle écarte | Pourquoi |
|---|---|---|---|
| 1 | Aucune donnée ne quitte la machine, même avec consentement | Toute mesure d'usage à distance, et toute monétisation par licence en ligne (§11) | C'est ce qui rend crédible une application qui demande l'Accessibilité et prend l'écran entier |
| 2 | La vérification de mise à jour est la seule exception, déclarée et désactivable | Le refus absolu, qui laisserait des versions défectueuses en circulation sans moyen de les corriger | Une exception nommée vaut mieux qu'un refus qu'on contourne en silence |
| 3 | Breeze rend la triche coûteuse et consciente, jamais impossible | Toute promesse d'infaillibilité, dans l'interface comme dans la présentation du produit | Une promesse démentie une fois se paie en désinstallations et en avis d'une étoile |
| 4 | Un budget de report unique par cycle, partagé par tous les mécanismes | Les plafonds séparés par mécanisme, qui se cumulent sans qu'aucun ne soit dépassé | Deux plafonds sont un plafond qu'on peut additionner |
| 5 | Le budget se remet à plein au démarrage du cycle, jamais à l'entrée en phase de travail | Le rechargement à chaque report, qui rendrait le report illimité | Un report ramène en phase de travail : recharger là annule ce que le report vient de coûter |
| 6 | Une pause avortée débite les minutes de pause non faites | La simple reconduction du budget entamé, qui ne coûte rien quand rien n'a été débité | C'est le seul chiffrage qui ferme le chemin « déclencher puis écourter » (§10.2) |
| 7 | Une phase rouverte par un report court en temps réel et ne gèle plus | Le gel par application ignorée après un report, qui repousserait la pause due indéfiniment | La plus large échappatoire du modèle, pour cinq minutes de budget |
| 8 | Une phase rouverte par un report n'est un état d'origine ni pour la suspension ni pour le redémarrage | Le chemin « reporter puis redémarrer », qui effaçait la pause **et** rendait les minutes débitées | La pause est déjà due ; le report ne l'a que déplacée |
| 9 | Suspendre et redémarrer sont impossibles dès qu'une pause est due | Deux leviers gratuits qui effaçaient une pause sans débiter une minute, de façon répétable | La distinction « pause due ou non » est ce qui tient tout le §10.2 |
| 10 | L'inhibition s'évalue en niveau, à l'échéance, jamais sur son front | La lecture par front, qui laisserait l'overlay tomber en pleine réunion déjà commencée | C'est le cas dominant, pas le cas limite |
| 11 | L'ensemble des causes d'inhibition reste ouvert, et le budget débite une fois par intervalle recouvert | La liste arrêtée à l'échéance, et le double débit par causes concurrentes | Une visio qui commence pendant la plage du déjeuner ne doit ni être ignorée ni coûter double |
| 12 | Le statut par défaut de toute application inconnue est `bloquée` | Le défaut permissif, qui produirait une première pause sans effet | La promesse doit être éprouvée à la première pause, pas à la dixième |
| 13 | Le statut `ignorée` gèle le décompte, et ce n'est pas une échappatoire | La fermeture de ce chemin, qui reviendrait à refuser à l'utilisateur de dire ce qui n'est pas du travail | Le produit se défend contre celui qui négocie à chaud, pas contre celui qui décide à froid (§6) |
| 14 | La sévérité par défaut est Simple, et la prévisualisation du Hardcore est montrée avant tout choix | Le Hardcore par défaut, qui produirait une désinstallation à la première pause | Contrepartie assumée : la promesse est d'abord éprouvée sur le mode qui la tient le moins (§17) |
| 15 | La sévérité ne change dans aucun sens pendant une pause | Le durcissement en cours de pause, pourtant conforme à la règle d'or | Les compteurs mesureraient un mode que la pause n'a pas eu |
| 16 | La sortie d'urgence n'est jamais rationnée, et elle est affichée en permanence | Le quota de sorties, et la sortie cachée | Un garde-fou qu'on épuise n'en est plus un ; un garde-fou caché est un piège |
| 17 | Aucun overlay n'est posé au lancement de l'application | La reprise fidèle d'une pause interrompue par un plantage | C'est ce qui empêche un défaut de reprise de poser un overlay irretirable |
| 18 | Le temps passé sans processus ne vaut jamais absence | Le crédit d'une pause pour du temps que personne n'a mesuré | Un verdict ne se rend pas sur une observation que personne n'a faite |
| 19 | Le seuil d'absence est figé au démarrage du cycle, et relevé si la pause s'allonge | Sa relecture au réveil, dans les deux sens | Sans le figeage, deux minutes d'inactivité passeraient pour une pause ; sans le relèvement, une absence trop courte validerait une pause allongée |
| 20 | Quand une échéance et une commande tombent au même instant, l'échéance l'emporte | L'arbitrage par ordre d'arrivée | Sinon les compteurs mesurent le hasard d'ordonnancement |
| 21 | L'overlay dégradé obéit au même délai et au même débit que la fin anticipée | La fermeture immédiate et gratuite | Sans cela, refuser une permission serait le moyen le plus rapide d'annuler toutes ses pauses |
| 22 | Une pause servie sans overlay compte comme les autres | L'arrêt du comptage quand la contrainte n'a pas pu être posée | Breeze dit ce qu'il ne peut pas faire plutôt que de cesser de compter |
| 23 | Le coupe-circuit désarme les overlays après des chutes répétées | La confiance dans un processus qui vient de tomber trois fois | La sortie doit exister avant le défaut qui la rendra nécessaire |
| 24 | La liste de sécurité ne s'applique qu'au Mode Simple | Son extension au Hardcore, qui y ouvrirait une porte de sortie complète | La contrepartie est portée par l'accessibilité de l'overlay Hardcore lui-même |
| 25 | Aucune application tierce n'est masquée, quittée ni suspendue | Les approches qui suspendent ou masquent le processus cible | Perte de contexte, données corrompues, et un profil de logiciel malveillant |
| 26 | Le produit est gratuit au lancement, sans licence ni compte | L'abonnement à validation périodique, qui rouvrirait la sortie réseau | Vendre la raison pour laquelle on installe le produit (§11) |
| 27 | Aucun seuil de succès n'est fixé avant la première mesure | Les cibles de rétention posées à l'avance | Un nombre posé d'avance devient un nombre auquel on ajuste la lecture des faits (§12.3) |
| 28 | La notarisation est un point bloquant, mesuré avant toute écriture de fonctionnalité | Le pari qu'elle passera, et la découverte du refus après le développement | C'est la seule mesure du brief qui n'a aucun repli |
| 29 | Une suspension **gèle** la phase de travail au lieu de la remettre à zéro | La suspension-éclair suivie d'une reprise, qui effaçait une phase presque finie pour rien | C'est aussi ce que l'utilisateur veut dire quand il suspend : mettre Breeze en attente, pas recommencer sa journée |
| 30 | Redémarrer le cycle coûte les minutes de travail effacées, et **n'est offert que si le budget les couvre** | Le redémarrage gratuit juste avant le préavis — et le redémarrage seulement tarifé, dont le prix s'annule au deuxième usage puisque le premier vide le budget | Un prix qui s'annule à la répétition n'est pas un prix. Le refus, lui, ne s'use pas |
| 31 | Le budget se plancher à zéro, et **un levier est refusé dès que le budget restant ne couvre pas son prix** | Le budget négatif, la dette reportée de cycle en cycle, et le refus posé sur la nullité — qui laissait obtenir cinq minutes de report pour quatre de budget | « Trois reports épuisent exactement le budget » n'est vrai que si le quatrième est refusé, et le troisième aussi quand une inhibition est passée avant |
| 32 | La veille ne gèle jamais une échéance, seulement l'affichage, la pause en cours et le budget | Le gel total, qui faisait de la fermeture du capot un report gratuit et répétable | Deux règles s'excluaient : si l'échéance gèle, aucune absence ne peut la dépasser, et la règle 3 du §10.4 devient inatteignable |
| 33 | La règle d'or couvre aussi le déclencheur intelligent, et le décompte autant que l'overlay | Les deux seuls réglages capables d'empêcher une pause de **devenir** due, que rien ne bornait | Le §13 ne rattrape rien tant qu'aucune pause n'est due : la borne devait être en amont |
| 34 | « Hors pause » veut dire « tant qu'aucune pause n'est due » | L'affaiblissement de la sévérité pendant le préavis ou l'inhibition, qui livrait une pause contournable | Une pause due sans avoir commencé restait un angle mort entre deux formulations |
| 35 | Un report repousse la pause de cinq minutes **au total**, préavis rejoué compris | Le préavis rejoué facturé en plus, qui rendait six minutes pour cinq débitées | Sans quoi trois reports n'épuisent plus exactement le budget, et l'arithmétique du §12.2 est fausse |
| 36 | Un saut d'heure système ne referme jamais une plage horaire sur le cycle en cours | L'effacement gratuit d'une phase de travail par un réglage d'horloge, deux fois par an sans le vouloir | La plage suit déjà la règle d'or par son réglage ; elle devait la suivre par l'horloge aussi |
| 37 | Un arrêt volontaire **persiste** la phase de travail entamée | La destruction de l'état à la sortie, qui faisait de « quitter puis relancer » le dernier effaceur gratuit du modèle | Quitter détruit le processus, jamais ce qu'il a écrit — et c'est ce qui rend vrai l'invariant du §13 |
| 38 | La veille et le verrouillage ne comptent pas comme de l'inactivité | Le gel de la phase de travail au bout de trois minutes de sommeil, qui rendait à la mise en veille le report gratuit qu'on venait de lui retirer | Deux corrections se seraient annulées : le §10.4 traite seul ce qui se passe pendant qu'on dort |
| 39 | Le jour actif s'évalue en continu, sauf pour une plage déjà ouverte qui traverse minuit | L'évaluation unique au démarrage du cycle, qui supprimait l'une des deux causes de [INACTIF] | Un rythme de nuit doit pouvoir se dire ; il ne doit pas pour autant désarmer le calendrier |
| 40 | Une phase de travail gelée par une suspension n'expire pas ; une phase perdue par la fermeture de la plage horaire ne revient pas | Le traitement uniforme des deux, qui aurait rendu l'un des deux exploitable | Suspendre est le geste de quelqu'un qui revient ; la fermeture de la plage est la fin de la journée. Ni l'un ni l'autre ne se commande quand la pause approche |
| 41 | Un cycle ouvert par un redémarrage n'ouvre jamais un budget plein : seule une pause servie le fait | Le rechargement à chaque redémarrage, qui rendait la boucle « sept minutes, un clic, recommencer » stable et infinie | Un refus qui ne mord que sur un budget rechargé par le geste qu'il devait refuser ne refuse rien |
| 42 | Le prix d'un redémarrage compte la phase gelée comme du travail effacé | Le redémarrage gratuit depuis [ARMÉ] après avoir fait perdre le déclencheur | C'était la dernière porte ouverte du modèle |
| 43 | Une pause due mais jamais commencée est, au relancement, débitée de sa durée entière | Les deux seuls autres sorts — la servir, ce qui poserait un overlay au lancement ; l'effacer, ce que le §13 interdit | Quitter pendant un préavis devenait sinon le successeur direct de l'effaceur que la décision 37 vient de fermer |

---

## 16. Ce qui vient après le lancement

Les trois fonctionnalités décisives du §9 sont repoussées **sciemment**. Chacune doit franchir quatre
conditions, dans cet ordre :

1. **Elle est impossible sans ce que Breeze tient déjà** — sinon un concurrent la copie en un
   trimestre.
2. **Elle renforce la promesse du §1** plutôt que d'élargir le périmètre.
3. **Elle tient dans ce qui existe**, sans second produit à construire.
4. **Elle ne coûte aucune permission que le §13 interdit.**

| La fonctionnalité | Ce qu'elle apporte | Ce qu'elle attend |
|---|---|---|
| **Le bouclier de calendrier** (§9.1) | Le Mode Hardcore devient tenable en environnement professionnel | Que la permission Calendriers soit jugée acceptable par la bêta — c'est la seule permission que Breeze demanderait au-delà du strict nécessaire, et elle contredit en apparence le §10.6 |
| **La dette de posture** (§9.2) | Empêche la dérive vers l'inefficacité, première cause d'abandon dans cette catégorie | Ses seuils, mesurés en test utilisateur (§12.3). Bloquant : une escalade mal calibrée est pire que pas d'escalade |
| **La détection de session profonde** (§9.3) | Répond à l'objection numéro un de la cible, et ouvre le segment productivité | Que la durée réelle des blocs de travail soit mesurée sur des utilisateurs réels : les seuils ne se posent pas au bureau |
| **Les profils de réglages multiples** (§14) | Un rythme par contexte de travail | Que la bêta montre des reconfigurations répétées du rythme. Sans ce signal, c'est un écran de plus pour rien |

---

## 17. Les hypothèses risquées

Ce que ce brief tient pour vrai sans l'avoir vérifié, et qui, faux, casse le produit. Une hypothèse
n'est ni une mesure ni une question ouverte : c'est une règle **déjà tranchée dans ce document** dont
rien ne prouve encore qu'elle est bonne.

| Ce qu'on tient pour vrai | Ce qui casse si c'est faux | Ce qui le trancherait |
|---|---|---|
| **Celui qui a désactivé les rappels passifs veut être contraint, pas mieux rappelé** (§2) | Le produit entier. Breeze n'est pas un rappel de plus mieux réglé : s'il n'existe pas de demande pour la contrainte elle-même, il n'y a pas de marché, et aucun ajustement de rythme n'y changera rien | La part des participants de la bêta qui, après une semaine, ont **durci** leur réglage plutôt que de l'assouplir ou de désinstaller |
| **Le Mode Simple par défaut convainc assez pour faire passer au Hardcore** (§15, décision 14) | La promesse est éprouvée sur le mode qui la tient le moins : l'utilisateur constate qu'il contourne sa pause en déplaçant une fenêtre, en conclut que Breeze ne marche pas, et désinstalle sans avoir jamais essayé le mode qui marche | La part des utilisateurs passés en Hardcore pendant leur première semaine (§12.3) |
| **Dix secondes de maintien est le bon seuil de friction** (§12.2) | Trop court, le Mode Hardcore devient contournable par réflexe et cesse d'être ferme. Trop long, il devient un piège, et c'est exactement le reproche qui transforme le produit en logiciel malveillant aux yeux de l'utilisateur | Le nombre de sorties d'urgence par semaine **et leur motif déclaré** : la valeur seule ne dit pas de quel côté le seuil est faux (§12.3) |

---

## 18. Sources

Ce qui appuie les contraintes du §5, avec ce qu'on en a tiré. Les sources ont été lues le 2026-08-28 ;
chacune porte le niveau de confiance qu'elle mérite, et une affirmation confirmée par une source non
normative est signalée comme telle plutôt que présentée comme acquise.

| Ce qui est affirmé | Ce qu'on en a tiré, et où |
|---|---|
| Une fenêtre au niveau maximal passe au-dessus du Dock et de la barre de menus | Hiérarchie des `CGWindowLevelKey`, où le niveau de l'économiseur d'écran est au-dessus de ceux du Dock et du menu principal — `developer.apple.com/documentation/coregraphics/cgwindowlevelkey`. **Acquis.** |
| Cette même fenêtre n'est **pas** garantie au-dessus d'une application en plein écran natif | Apple écrit de la technique qu'elle *« n'est pas recommandée »*, à cause des interactions imprévisibles avec les contextes plein écran — `developer.apple.com/documentation/coregraphics/cgshieldingwindowlevel()`. **Non acquis, et c'est une mesure du §12.3.** Le comportement dépend des espaces de travail, pas du niveau de fenêtre |
| Masquer le Dock et la barre de menus, et désactiver le changement d'application, est une API publique | Les trois options de présentation d'AppKit, documentées depuis macOS 10.6 et décrites par Apple comme destinées *« aux applications plein écran telles que les jeux ou les bornes »* — `developer.apple.com/documentation/appkit/nsapplication/presentationoptions-swift.struct`. **Acquis.** |
| L'autorisation d'Accessibilité suit l'identité signée du binaire, pas son chemin | macOS suit l'identité du code par sa *designated requirement* ; un code signé ad hoc n'en porte pas de stable, et le système ne peut alors pas reconnaître la version N+1 comme le même logiciel que la version N — réponse d'un ingénieur Apple sur le forum développeur officiel, renvoyant à la note technique TN3127 sur les *requirements* de signature. **Confirmé par une source Apple, mais non normative** : aucune page de documentation formelle ne l'énonce en ces termes |
| Aucune API publique ne place une fenêtre au-dessus d'une **fenêtre précise** d'une autre application | Les niveaux documentés sont catégoriels, et les méthodes d'ordonnancement n'opèrent qu'entre fenêtres d'une même application — `developer.apple.com/library/archive/documentation/Cocoa/Conceptual/WinPanel/Concepts/WindowLevel.html`. **Confirmé par absence** : c'est une conclusion tirée de la surface d'API publique, pas une affirmation d'Apple |
| Le Mac App Store est fermé à ce produit | Les règles de revue d'Apple exigent le bac à sable pour toute application macOS (règle 2.4.5) et rejettent celles *« qui créent des environnements de bureau alternatifs »* (règle 2.5.8) — `developer.apple.com/app-store/review/guidelines/`. Un ingénieur Apple confirme par ailleurs sur le forum officiel que l'API Accessibilité ne fonctionne pas depuis une application en bac à sable, **même si l'utilisateur accorde la permission à la main**. **Acquis.** |
| Les modes Concentration ne sont ni lisibles ni pilotables par Breeze | Une API publique de lecture existe depuis macOS 12, mais elle rend un simple booléen sans nom de mode et n'est ouverte qu'aux applications de messagerie disposant de l'habilitation correspondante — Breeze n'en est pas une. Aucune API publique ne permet d'**activer** un mode. **Acquis pour Breeze**, et l'affirmation générale « aucune API n'existe » est fausse : c'est une restriction d'éligibilité, pas une absence |
| Apple est passé de macOS 15 à macOS 26 | Communiqué officiel annonçant macOS Tahoe 26 comme successeur de macOS 15 Sequoia — `apple.com/newsroom`. **Acquis.** |

**Ce qui n'a pas pu être vérifié**, et qui reste à la charge de la reconnaissance technique : le
comportement réel du niveau de fenêtre maximal face au plein écran natif, **version par version** de
macOS 13 à 26. Les rapports publics disponibles s'étalent sur plusieurs versions sans permettre de les
isoler.

**Aucune source de marché n'appuie ce document, et il n'en cite aucune.** C'est délibéré : le §12.3
n'accueille que des mesures à faire sur les utilisateurs réels de Breeze, jamais des repères empruntés
à un secteur voisin. Un taux de rétention observé ailleurs projette ; il n'établit rien ici.
