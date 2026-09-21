## 10. Les règles transverses

Ce qui vaut partout, et qu'aucune fonctionnalité ne redit.

### 10.1 Le cycle et ses états

```
   ┌──────────────── fin de plage horaire, jour inactif ────────────────┐
   ▼                                                                    │
[INACTIF] ──plage horaire et jour actifs──▶ [TRAVAIL] ───────────────────┤
                                                │    ▲                   │
                                                │    └── [RETOUR] ◀──┐   │
                              décompte épuisé,  │      (3 s, murale, │   │
                        profondeur comprise     │       gelée en     │   │
                        (§9.3) — la pause        │       veille)      │   │
                        devient due              │                    │   │
                                                ▼                     │   │
                                          [PRÉAVIS] (1 minute, fixe) │   │
                                                │                     │   │
                                           échéance,                 │   │
                                     ou sautée si l'absence l'a      │   │
                                     déjà consommée (§10.4)          │   │
                                                ▼                     │   │
                                        [PAUSE ACTIVE] ──servie───────┘   │
                                                │                         │
                                          interrompue (§10.2) ────────────┤
                                        (repart directement en [TRAVAIL]) │
```

**Ce diagramme est celui du produit livré au lancement.** Sans les trois fonctionnalités décisives du
§9 — aucune n'y entre (§9, §16) —, une pause due entre en [PRÉAVIS] sans délai, aucune dette de
posture ne se crédite ni ne durcit un cycle, et aucun placement n'écarte une pause d'une réunion : le
§9.3 seul retient le début du [PRÉAVIS] ; retirer les trois, le diagramme ci-dessus reste exact tel
quel. Le décalage que s'autorise le bouclier de calendrier (§9.1), quand il joue, ne crée ni état ni
attente supplémentaire : il retient seulement, en amont, l'instant où [TRAVAIL] atteint zéro pour ce
cycle, exactement comme s'il s'agissait d'un rythme différent.

**Ce que le diagramme ne montre pas, et qui le ferme :**

- Il n'existe plus d'état d'attente d'un déclencheur : tout usage compte comme du travail sauf si
  l'application au premier plan est `ignorée` (§8.3), auquel cas **[TRAVAIL] gèle sur place** — le
  décompte cesse d'avancer, et aucune autre transition n'en découle tant que l'inactivité ou le gel ne
  cessent pas, hors le verdict d'absence du §10.4 : au-delà d'un second seuil, il valide le cycle
  entier et le fait repartir en [TRAVAIL] plein — la seule transition que ce gel autorise. La même
  règle vaut pour une inactivité mesurée au-delà du seuil du §12 (machine éveillée et session
  déverrouillée). **Ce gel ne joue que tant que le décompte n'a pas atteint zéro** : la fermeture de
  la plage horaire, une suspension ou l'entrée en veille agissent normalement par-dessus un [TRAVAIL]
  gelé, exactement comme sur un [TRAVAIL] qui avance.
- **Une pause devient due dès que le décompte de travail atteint zéro**, immédiatement ou après la
  retenue du §9.3. Breeze reste formellement en [TRAVAIL] pendant cette attente — le décompte est à
  zéro, immobile, et Breeze attend le signal du §9.3 (un changement d'application, ou son plafond,
  lui-même **mural** : il continue de courir pendant une veille, exactement comme l'échéance de
  travail dont il découle). **Une fois due, le gel d'inactivité ne s'applique plus** : il n'y a plus
  de décompte à protéger, seulement un signal à attendre, et seul le plafond du §9.3 le fait
  aboutir. Un changement d'application vers **n'importe quelle** application — y compris une
  application `ignorée` — met fin à l'attente et démarre [PRÉAVIS] : le statut de la nouvelle
  application ne rejoue aucun rôle ici, il ne gouverne que l'avancée du décompte avant que la pause
  soit due.
- **[PRÉAVIS] dure une minute, sans exception, une fois entré.** Rien ne s'y clique, rien ne l'écourte
  ni ne le prolonge. Seul son *début* peut être retenu par le §9.3, avant qu'il commence.
- **[PAUSE ACTIVE]** porte le décompte de la pause. En Mode Hardcore, un geste unique peut y mettre fin
  (§8.5) ; en Mode Simple, aucun. Dans les deux cas, la pause se termine soit par l'écoulement complet
  de son décompte (**servie** → [RETOUR]), soit par une interruption (§10.2) qui repart directement en
  [TRAVAIL], décompte plein, sans passer par [RETOUR] — il n'y a rien à dire « c'est fini » à quelqu'un
  qui vient de choisir de partir. **Le geste de sortie du Mode Hardcore suit ce second chemin : il ne
  quitte pas Breeze**, il interrompt la pause exactement comme le ferait le raccourci de fermeture du système, et Breeze continue
  de tourner.
- **[RETOUR]** dure trois secondes, suit la même règle temporelle qu'une pause active — murale pendant
  que la machine tourne, gelée pendant la veille, reprise pour ce qu'il en restait au réveil — et rien
  d'autre ne l'écourte ni ne le prolonge. Aucune commande ni aucun événement ne raccourcit [RETOUR], et
  aucun ne suspend pendant [RETOUR] (§10.3). Une fermeture de plage horaire pendant [RETOUR] attend ces
  trois secondes avant de faire passer Breeze en [INACTIF].
- **[INACTIF]** a exactement deux causes : la fermeture de la plage horaire et le passage à un jour
  inactif. Pendant [TRAVAIL], la fermeture de la plage fait perdre le décompte en cours **sauf si une
  pause y est déjà due** (§9.3) : une pause due va à son terme — [PRÉAVIS] puis [PAUSE ACTIVE] — avant
  que Breeze ne passe en [INACTIF], exactement comme depuis [PRÉAVIS] ou [PAUSE ACTIVE]. **Une pause
  due n'est donc jamais coupée par l'horloge, qu'elle ait commencé ou qu'elle attende encore son
  signal.** **Le jour actif s'évalue en continu, avec une seule exception, propre aux plages qui
  traversent minuit** : une plage ouverte un jour actif reste ouverte jusqu'à sa fin, même si minuit
  la fait passer à un jour inactif — sans elle, un rythme 22 h – 2 h se couperait chaque nuit à
  minuit. Les compteurs calés sur minuit (la dette de posture du §9.2, les statistiques du §12.2)
  suivent, eux, le jour civil sans exception : un cycle à cheval sur minuit alimente les deux
  journées qu'il touche.

**Deux états transversaux se superposent au cycle :**

- **[SUSPENDU]** — l'utilisateur a suspendu Breeze pour l'une des trois durées du §12 (15 minutes,
  1 heure, jusqu'au prochain 6 h, heure locale). **[SUSPENDU] porte, gelés, ce que [TRAVAIL] portait à l'instant de
  la commande : le décompte de travail, et une pause due le cas échéant.** Ce n'est ni [ARMÉ] — qui
  n'existe plus —, ni un état vide : c'est [TRAVAIL] mis en pause, rien de plus.
  **La référence qui compte est celle du tout premier « Suspendre » d'une chaîne, jamais celle d'une
  commande qui la remplace.** Suspendre à nouveau depuis [SUSPENDU] remplace l'échéance en cours (plus
  bas) sans rouvrir cette référence : c'est le temps total écoulé depuis le premier geste (un temps
  réellement écoulé, immunisé contre un changement de l'heure système — voir plus bas) qui se compare
  au travail qu'il restait à cet instant-là, pas le temps depuis le dernier renouvellement — sans quoi
  renouveler une suspension avant son terme la rendrait indéfiniment gratuite. **Une chaîne se referme
  dès que Breeze repart réellement en [TRAVAIL]** — au terme normal, par une reprise anticipée, ou par
  le verdict de dépassement ci-dessous. La commande « Suspendre » suivante, si elle part de ce
  [TRAVAIL]-là, ouvre une chaîne neuve avec une référence neuve ; seule une commande reçue **depuis
  [SUSPENDU] lui-même** appartient à la chaîne en cours.
  **Au plus une pause peut jamais être en jeu.** Le cycle est gelé pendant toute la suspension : rien
  n'y fait avancer un second décompte de travail ni devenir due une seconde pause pendant que la
  première reste en attente. À la reprise :
  - si le temps total écoulé depuis le premier « Suspendre » **n'a pas dépassé** le travail qu'il
    restait à cet instant, le décompte reprend là où il s'était arrêté (ou l'attente d'une pause déjà
    due continue, si c'est de là qu'on est parti) ;
  - s'il **l'a dépassé**, la pause qui serait devenue due — une seule, jamais davantage — est comptée
    interrompue (§10.2), et le cycle repart en [TRAVAIL], décompte plein.
  L'échéance de la suspension elle-même est **murale** : elle court pendant que la machine dort, et ne
  s'arrête jamais avant son terme — « aucune suspension sans terme » (§12) le garantit.
  **Reprendre Breeze avant le terme est une commande à part**, offerte à tout instant depuis
  [SUSPENDU] ; elle referme l'échéance immédiatement et applique la même règle de dépassement
  ci-dessus.
  **Suspendre est offert à tout instant de [TRAVAIL], y compris pendant l'attente d'une pause due —
  où le travail qu'il restait vaut alors zéro, donc toute reprise plus tardive dépasse ce zéro et
  compte la pause interrompue** —, mais jamais depuis [INACTIF], [PRÉAVIS], [PAUSE ACTIVE] ni
  [RETOUR]. **À son terme — atteint ou par la commande « Reprendre » —, Breeze repart en [TRAVAIL] si
  la plage horaire et le jour actif l'autorisent à cet instant, en [INACTIF] sinon** ; si cet instant
  tombe pendant la veille, le [TRAVAIL] qui en résulte suit alors les règles normales de la veille
  (plus bas), sans rien de spécial à la suspension qui l'a précédé.
- **[VEILLE]** — la machine dort ou la session est verrouillée par un verrouillage manuel ou automatique.
  La veille gèle l'affichage. Pour une pause en cours ou pour [RETOUR], elle gèle aussi le décompte
  **et repousse d'autant l'échéance** — personne ne
  regarde un écran éteint, donc rien n'y est imposé pendant ce temps. Pour une phase de travail
  (décompte en cours ou pause déjà due en attente), un préavis ou une suspension, l'échéance reste
  **murale** et continue de courir pendant la veille : c'est le §10.4 qui dit ce que produit ce délai
  au réveil, jamais un gel. **La veille et une suspension traitent la même durée différemment, et
  c'est voulu** : la veille est un fait qui arrive à l'utilisateur, la suspension est un choix qu'il
  fait. Le premier valide une pause qu'on n'a pas pu vivre (§10.4) ; le second, parce qu'il est un
  geste délibéré de mise en attente et non une absence constatée, se règle par la dette plutôt que
  par une validation gratuite.

**Un changement rapide d'utilisateur est un fait distinct, pas une veille.** La session d'origine est
verrouillée par le système comme pour tout verrouillage, et Breeze y gèle l'affichage de la même façon —
mais une autre session reste active sur la même machine : quelqu'un peut continuer à y travailler
pendant que la première paraît absente. Le temps passé ainsi ne suit donc jamais la validation par
absence du §10.4 : une pause due ou en cours qu'il traverse est comptée interrompue et créditée à la
dette de posture (§9.2), au même titre qu'une suspension qui aurait englouti une pause due (décision
14) — jamais validée comme prise gratuitement. Revenir à la session d'origine avant qu'aucune pause
n'ait été en jeu retrouve le cycle exactement où il en était, gelé comme sous n'importe quelle veille.

### 10.2 Ce qui interrompt une pause, et ce que ça coûte

**Chaque pause a un sort, et il en existe exactement trois : servie, validée par une absence
(§10.4), ou interrompue.** Ce sort est **toujours** enregistré — au bilan du jour (§8.6) et dans les
statistiques (§12) —, que la dette de posture (§9.2) soit ou non déjà livrée : une interruption n'est
jamais silencieusement comptée comme une pause prise. **Interrompue** regroupe deux origines, traitées
différemment :

- **Une terminaison propre** — le raccourci de fermeture du système (Cmd-Q sur macOS), le menu de
  l'icône d'état (§8.6), ou le geste du Mode Hardcore (§8.5) — pendant [PRÉAVIS]
  ou [PAUSE ACTIVE], ou une pause due que le processus n'a jamais vue commencer, constatée au
  relancement (plus bas). Breeze reçoit le signal d'arrêt et peut agir avant de mourir : ces trois
  portes sont individuellement créditées à la dette de posture, une fois qu'elle existe (§9.2).
- **Une chute** — un plantage ou un `forcer à quitter` (§5), que rien ne distingue l'un de l'autre à
  l'instant où ça arrive : les deux tuent le processus sans lui laisser la main. Breeze ne peut pas
  savoir lequel des deux s'est produit, et ne prétend pas le savoir : **une chute n'est jamais
  créditée individuellement**, quelle que soit sa cause. C'est le compteur de chutes rapprochées, pas
  la dette, qui y répond (plus bas) — `forcer à quitter` reste la sortie que le système garantit
  (§5), et la friction d'y accéder est déjà son prix, comme pour tout ce que le système autorise en
  dehors de Breeze. **Sur Linux, une surface d'overlay que le compositeur tue compte comme une chute
  du processus** : le coupe-circuit protège aussi contre les propres adaptateurs de Breeze.

**Aucune des trois portes de terminaison propre n'ouvre de négociation, et aucune n'est moins chère
qu'une autre.** Une fois la dette de posture livrée, les trois créditent la même quantité, selon la
même formule (§9.2). Ce que le geste du Mode Hardcore change n'est jamais le prix, seulement la façon
d'y arriver.

**Des chutes rapprochées arment le coupe-circuit plutôt que de peser sur la dette.** Au-delà d'un
seuil rapproché (§12), le coupe-circuit démarre **tous les cycles suivants sans aucun overlay** — la
pause de chacun d'eux va à son terme sans contrainte et **compte comme servie**, puisque son échéance
a été atteinte sans que rien n'ait empêché de la vivre. Breeze le dit dans le panneau plutôt que de
cesser de compter. Le coupe-circuit n'y renonce, et ne repose donc d'overlay, qu'après un délai sans
nouvelle chute ou un désarmement manuel depuis les réglages (§12), et il ne réduit ni n'annule la
dette déjà créditée par des interruptions propres antérieures. **« Les cycles suivants », ici, inclut
un cycle repris après la chute qui arme le coupe-circuit** — la troisième chute survient
nécessairement pendant un cycle, jamais entre deux — y compris si ce cycle était durci par la dette de
posture (§9.2) avant l'armement : l'overlay qu'il aurait porté est celui que le coupe-circuit retire.
**Trois arrêts forcés valent, à l'instant où ils arrivent, exactement trois chutes** : Breeze ne peut
pas les distinguer (ci-dessus), et un utilisateur qui les provoque délibérément arme donc le
coupe-circuit comme s'il avait vraiment planté trois fois — vingt-quatre heures sans overlay, sans que
la dette n'en profite ni n'en pâtisse. C'est un contournement possible, au même titre que déplacer une
fenêtre épargnée en Mode Simple (§8.4) : il exige une manipulation technique délibérée en dehors de
Breeze, à répéter chaque jour, et Breeze ne prétend pas le fermer (§2, §15 décision 3). **Le
coupe-circuit prime toujours sur
l'escalade de la dette de posture (§9.2) : un cycle sans overlay le reste même si la dette y aurait
sinon imposé le Mode Hardcore** — il protège contre un défaut de Breeze, ce que rien d'autre ne doit
pouvoir surclasser ; l'escalade se réévalue au cycle suivant, une fois le coupe-circuit désarmé,
exactement comme tous les autres cycles (§9.2).

**Aucun overlay n'est jamais posé au lancement de l'application.** L'échéance d'une phase de travail
reste **murale** même pendant que le processus est mort : un relancement qui la trouve dépassée
traite la phase comme une pause due mais jamais commencée. Une telle pause — commencée ou non — et pas
déjà validée par une absence (§10.4) **avant que le processus ne meure** — jamais pendant le temps où
il était mort, que la décision 18 exclut —, est comptée interrompue à ce relancement, dans
l'état d'origine propre correspondant (une chute reste une chute, sans dette individuelle ; un
« Quitter » resté sans suite reste une terminaison propre, créditée). Le cycle suivant repart alors en [TRAVAIL]
si la plage horaire et le jour actif l'autorisent à l'instant du relancement, en [INACTIF] sinon.
C'est ce qui empêche un défaut de reprise de poser un overlay irretirable, et c'est ce qui ferme la
sortie gratuite que quitter pendant [PRÉAVIS] offrirait sinon : qu'on quitte pendant le préavis ou
pendant la pause, le prochain lancement constate la même chose.

### 10.3 Ce qui affaiblit une contrainte n'agit jamais tout de suite

**Règle générale, valable pour le statut d'une application et pour la sévérité : un changement qui
affaiblit ce que Breeze impose n'entre en vigueur qu'au cycle suivant ; l'inverse — un changement qui
renforce — s'applique immédiatement.** Sans elle, ces réglages deviendraient la négociation que le
reste du produit vient de fermer.

| Le réglage | Quand il affaiblit | Quand il renforce |
|---|---|---|
| Statut d'une application (§8.3) | Ignorer, ou débloquer ce qui était bloqué : cycle suivant | Bloquer, ou cesser d'ignorer : immédiat |
| Sévérité | Passer de Hardcore à Simple : cycle suivant | Passer de Simple à Hardcore : immédiat |

**La durée de travail ou de pause suit une règle plus simple, sans notion de sens : elle ne s'applique
jamais à une phase déjà en cours, quel que soit le changement (§8.2).** Rallonger ou raccourcir prend
effet au cycle suivant dans les deux cas — contrairement au statut et à la sévérité, une durée n'a pas
de version « immédiate » qui garde un sens pendant un décompte déjà entamé : la recalculer en cours de
phase changerait une échéance déjà annoncée, ce qu'aucun réglage n'a le droit de faire.

**Les trois sont de toute façon impossibles à changer pendant [PRÉAVIS], [PAUSE ACTIVE] et
[RETOUR]** (§10.1, §8.2) : la règle du cycle suivant ne joue donc que pendant [TRAVAIL], y compris
pendant l'attente d'une pause due. Une pause en cours se déroule toujours sous les réglages qu'elle
avait à son premier instant.

**La suspension n'est pas de ces réglages : elle suit sa propre règle, au §10.1.** Elle reste
utilisable à tout instant de [TRAVAIL], et son coût se règle à la reprise plutôt que par un délai
d'entrée en vigueur.

### 10.4 L'absence, la veille et la reprise

**L'échéance fait autorité, jamais le décompte affiché.** Le temps restant se lit comme une
différence d'échéances. Les échéances d'une phase de travail (décompte en cours ou pause due en
attente), d'un préavis ou d'une suspension sont **murales** et continuent de courir pendant la veille
et le verrouillage ; l'échéance d'une pause en cours ou de [RETOUR] ne l'est pas — elle se repousse du
temps passé en veille (§10.1), puisque rien ne s'impose à personne sur un écran éteint.

**« Absence » (§4) désigne la mesure faite pendant une session éveillée et déverrouillée : c'est la
seule façon de détecter *l'inactivité* — l'absence de tout signal alors que rien n'empêchait d'en
produire un.** La veille et le verrouillage sont un fait différent, connu directement par le système
plutôt que déduit d'un silence, mais ils produisent le même besoin : dire ce qui se passe pour une
échéance que personne n'a pu voir arriver. Les règles qui suivent s'appliquent donc **identiquement**
qu'une phase ait été traversée par de l'inactivité mesurée, par une veille, par un verrouillage, ou
par une combinaison des trois — la durée cumulée est ce qui compte, jamais sa cause. Elles ne
concernent jamais [SUSPENDU], qui suit sa propre règle de dépassement (§10.1) plutôt qu'un verdict
d'absence : suspendre est un choix, pas une absence constatée.

**Cette mesure ne sait pas dire pourquoi les périphériques se sont tus** — personne devant l'écran, ou
quelqu'un d'immobile qui lit, regarde, ou participe sans les toucher. Breeze ne cherche pas à le
savoir : lire l'écran ou la caméra pour le deviner est exactement ce que le §10.6 interdit, et le seuil
s'applique donc pareil dans les deux cas, aussi souvent qu'il se présente — la même limite, assumée de
la même façon, qu'une visio tenue dans un onglet de navigateur (§5). **Un changement rapide
d'utilisateur n'entre jamais dans les règles qui suivent** : il suit la règle propre que le §10.1 lui
donne, puisqu'une autre session peut y rester active — ce n'est jamais une absence, même s'il en prend
l'apparence à l'écran.

- **Plus longue que la durée de pause du cycle, pendant [TRAVAIL] ou [PRÉAVIS]** : le cycle est
  validé comme pause prise et repart à zéro en [TRAVAIL]. Rappeler une pause à quelqu'un déjà parti
  depuis plus longtemps qu'elle ne dure n'a pas de sens.
- **Plus longue que le temps de pause restant, pendant [PAUSE ACTIVE] ou [RETOUR]** : la pause est
  validée servie, et Breeze enchaîne sur [RETOUR] (ou termine les trois secondes qu'il en restait) au
  réveil.
- **Plus courte que ces seuils, mais qui dépasse l'échéance de la phase qu'elle traverse** :
  l'échéance produit ce qu'elle aurait produit si l'utilisateur était resté — le préavis est **sauté**
  s'il aurait dû jouer pendant l'absence (il n'y a personne à qui l'annoncer), et la pause démarre au
  réveil. C'est la seule façon dont [TRAVAIL] rejoint [PAUSE ACTIVE] sans passer par un [PRÉAVIS]
  vécu : la durée du préavis (une minute) est alors simplement absorbée dans l'absence qui l'a
  couverte, elle n'est ni rejouée ni raccourcie ailleurs.
- **Le temps passé sans processus ne vaut jamais absence.** Aucun signal n'a pu être mesuré, aucune
  veille n'a pu être constatée, pendant que Breeze était mort ; un verdict ne se rend pas sur une
  observation que personne n'a faite. C'est le §10.2, et lui seul, qui tranche ce qui se passe au
  relancement.

**Les autres états ne produisent aucun verdict d'absence.** [INACTIF] et [RETOUR] — hors la validation
par une absence qui l'achève, ci-dessus — n'ont ni décompte de travail ni pause due ; rien n'y est
validé, quelle que soit la durée écoulée.

### 10.5 Ce que Breeze fait quand une capacité manque

**C'est la seule table qui décrit un mode dégradé.** Une capacité (§4) manque pour l'une de deux
raisons, et la table ne les distingue pas : une permission que l'utilisateur n'a pas accordée ou a
retirée, ou une session qui ne l'expose pas du tout (§5). Le comportement est le même ; seule change
la réparation possible — un badge et un clic dans le premier cas, rien à réparer et une phrase à
froid dans le second.

| Capacité manquante | Ce qui continue de fonctionner | Ce qui change |
|---|---|---|
| **Notifications** seule | Tout | Le préavis passe par la bannière propre à Breeze |
| **Cadres des fenêtres** (Accessibilité refusée sur macOS), sévérité Hardcore | Tout | Aucune différence : le Mode Hardcore n'en a pas besoin (§8.5) |
| **Cadres des fenêtres** (Accessibilité refusée sur macOS), sévérité Simple | Le cycle, le préavis, le décompte | Breeze ne peut pas suivre les cadres de fenêtres. Il pose un voile unique couvrant l'écran entier, avec le décompte |
| **Identité au premier plan inconnue** (GNOME Wayland, application élevée sur Windows, session non reconnue) | Le cycle, le préavis, la pause, les deux modes | **Tout usage compte comme du travail.** Le statut `ignorée` n'a aucun effet et Réglages › Applications le dit. Le gel d'inactivité (§10.1) continue de jouer, puisqu'il ne dépend que de l'instant de la dernière action |
| **Cadres inobservables** (toute session Wayland), sévérité Simple | Le cycle, le préavis, le décompte | Même voile plein écran que la ligne Accessibilité, **un par moniteur**. C'est le comportement nominal sur Wayland, dit dès l'onboarding — pas un incident |
| **Couche overlay refusée par le compositeur** (GNOME Wayland, ou capture clavier exclusive refusée sur KDE/wlroots), sévérité Hardcore | Le cycle, le préavis, la pause, le décompte, le geste de sortie | L'overlay couvre chaque écran mais **les raccourcis du compositeur ne sont pas garantis** : une autre fenêtre peut passer devant, Super ou Alt-Tab peuvent répondre. Dit à froid (§5.7). Une pause servie sans couverture tenue compte comme les autres (décision 15) |

**Le voile dégradé reste un overlay de Mode Simple, pas une promotion vers le Hardcore.** Il ne
masque ni le Dock, ni la barre de menus, ni la barre des tâches, ne neutralise aucun raccourci, et ne
porte aucun geste de sortie dédié — le raccourci de fermeture du système ou le menu de l'icône d'état
(§8.6) restent la seule sortie, exactement comme n'importe quelle pause Simple (§8.4). Rien ne le
ferme depuis l'intérieur : la seule réparation est d'accorder la permission quand une permission est
en cause, ce que le badge de l'icône rappelle en continu ; quand c'est la session qui ne l'expose pas,
il n'y a rien à réparer, et Breeze ne fait pas semblant du contraire.

**Le mode d'une pause — nominal ou dégradé — se fixe une seule fois, à l'instant où [PAUSE ACTIVE]
commence — juste avant que l'overlay ne soit posé —, sur le relevé des capacités à cet instant
précis**, et ne bouge plus jusqu'à [RETOUR], quoi qu'il arrive aux capacités entre-temps : c'est la
même règle que pour la sévérité (§10.1, §10.3), ancrée sur le même instant qu'elle. Une capacité
perdue ou retrouvée avant cet instant — pendant [TRAVAIL], y compris pendant l'attente d'une pause due,
ou pendant tout [PRÉAVIS] vécu — vaut donc pour la pause à venir, sans qu'aucune notion de cycle
suivant n'ait à s'en mêler ; et sur le chemin où le préavis est sauté (§10.4), c'est ce même instant —
le début de [PAUSE ACTIVE] — qui sert d'évaluation, puisqu'aucun autre n'existe sur ce chemin. Il n'y
a qu'une seule évaluation, faite au bon moment, quel que soit le chemin qui y mène. **La règle ne
distingue pas la raison du manque** : une Accessibilité retirée à la 49ᵉ minute et un compositeur qui
refuse la couche produisent la même chose — une pause entière dans le mode fixé à son premier
instant.

**Une capacité peut disparaître ou apparaître à tout instant pendant que Breeze tourne** — permission
retirée ou accordée, session passée de X11 à Wayland, surface tuée par le compositeur. Le relevé est
refait au lancement et à chaque changement, pose ou retire le badge quand une permission est en cause,
propose la réparation en un clic quand elle existe, et met à jour la phrase à froid (§5.7) ; l'effet
sur le mode d'une pause suit la règle ci-dessus. **Une capacité qui apparaît en cours de pause ne
renforce pas cette pause, et une capacité qui disparaît en cours de pause ne l'affaiblit pas** : le
voile ou l'overlay posés restent tels quels jusqu'à [RETOUR].

### 10.6 Les garde-fous et ce qui ne sort jamais de la machine

**La liste de sécurité, non modifiable, une par système d'exploitation.** Chaque application y est
nommée par son **identité d'exécutable**, jamais par une catégorie qu'aucun OS ne rend lisible sur
une application tierce. Seule la liste de l'OS courant s'applique ; les autres n'existent pas sur la
machine.

| OS | Les applications toujours épargnées |
|---|---|
| macOS | Sept : les Réglages du système, le Moniteur d'activité, le Terminal, le Finder, le Trousseau d'accès, la fenêtre d'ouverture de session, et VoiceOver — le lecteur d'écran fourni par macOS, seul utilitaire d'accessibilité système qui s'ouvre en fenêtre susceptible d'être recouverte ; les autres (Zoom, Contrôle de synthèse) se pilotent sans fenêtre dédiée |
| Windows | Six : les Paramètres, le Gestionnaire des tâches, l'Explorateur, le Terminal et PowerShell (une seule ligne, deux identités), le Narrateur — le lecteur d'écran fourni par Windows —, et l'écran de connexion |
| Linux | Quatre : l'émulateur de terminal par défaut de la session, le centre de contrôle du bureau, le moniteur système, et le lecteur d'écran Orca. Les identités concrètes dépendent du bureau installé et sont relevées au lancement ; quand un bureau n'expose pas l'une d'elles, la ligne est simplement absente, jamais remplacée par une devinette |

**Leur statut n'est pas éditable et vaut toujours `épargnée`** : elles ne peuvent pas être mises
`ignorée`, et leur usage compte comme du travail, au même titre que n'importe quelle application
épargnée. En Mode Hardcore, la liste ne s'applique pas — l'overlay couvre l'écran entier — et la
contrepartie est portée par l'accessibilité de l'overlay lui-même et par son geste de sortie (§8.5).
Sur une session où l'identité au premier plan est inconnue (§10.5), la liste n'a rien sur quoi
s'appliquer : c'est le voile plein écran, qui ne recouvre aucune application précise, qui la rend
sans objet.

**Une application de santé ou d'urgence que l'utilisateur tient à garder accessible se marque
`épargnée`, comme n'importe quelle autre** : Breeze ne peut pas la reconnaître de lui-même, et
l'onboarding le dit à l'écran des applications plutôt que de prétendre couvrir ce qu'aucun signal ne
lui rend visible.

**Rien ne sort de la machine.** Aucune télémétrie, aucun compte, aucune remontée d'usage, même
consentie. **Seule exception, déclarée et visible : la vérification de mise à jour**, qui transmet la
version installée et l'adresse réseau au serveur de publication, et que l'utilisateur peut couper dans
les réglages. Une mise à jour trouvée n'est **jamais installée tant qu'une pause est due ou en
cours** (§10.3) : elle attend un moment où rien n'est en cours ou en attente, pendant [TRAVAIL] ou
[INACTIF] — et puisque Breeze tourne en continu sans être quitté en usage normal (§8.6), c'est Breeze
lui-même qui se ferme et se rouvre à ce moment-là pour l'appliquer, sans intervention de
l'utilisateur.

**Ce que Breeze lit, et ce qu'il ne lit jamais — sur les trois OS.** Il lit l'identité de
l'application au premier plan, la position et la taille des fenêtres pour le Mode Simple là où la
session les expose, et l'**instant** de la dernière action clavier ou souris — jamais son contenu.
C'est cette seule mesure, le temps écoulé depuis cet instant, qui alimente le gel d'inactivité (§10.1)
et l'indice de profondeur (§9.3) : aucune des deux ne lit ni ne compte de frappe, ni n'en distingue
une autre. **Jamais le titre d'une fenêtre, jamais le contenu de l'écran, jamais ce qu'une touche a
produit** — y compris quand un protocole Wayland offre le titre à côté de l'identité : Breeze ne s'y
abonne pas (§5.1) ; y compris quand neutraliser Alt-Tab l'exigerait : Breeze ne pose aucun hook
clavier (§13). Le journal local — circulaire,
borné à sept jours ou cinq mégaoctets, exportable par l'utilisateur seul — contient les transitions
d'état, les identités d'applications vues, les permissions perdues et les erreurs ; jamais un titre de
fenêtre ni un contenu applicatif. Les statistiques des trente derniers jours (§12) sont un compteur
séparé, plus long que le journal, et ne portent que des totaux — jamais un motif ni un contenu.

### 10.7 Ce qui se passe quand — cas limites tranchés

| Situation | Comportement |
|---|---|
| Le processus meurt (chute ou `forcer à quitter`, indistincts) pendant [PRÉAVIS] | Au relancement, si la plage horaire et le jour actif l'autorisent : comptée comme une pause due interrompue, sans dette individuelle (§10.2), cycle suivant en [TRAVAIL], décompte plein. Sinon, [INACTIF] |
| Le processus meurt pendant [PAUSE ACTIVE] | Même traitement qu'un préavis interrompu par une chute (ci-dessus) |
| Le processus tombe trois fois en cinq minutes | Le coupe-circuit s'arme : tous les cycles suivants tournent sans overlay et comptent comme servis, jusqu'au désarmement (§10.2, §12) |
| Le processus meurt pendant [TRAVAIL], avant qu'une pause soit due | Au relancement, la phase reprend là où elle en était si l'échéance de travail n'est pas dépassée ; sinon, elle est traitée comme une pause due jamais commencée, ci-dessus |
| Le processus meurt pendant [SUSPENDU] | L'échéance de la suspension, murale, a continué de courir. Au relancement, si elle n'est pas dépassée, la suspension reste active pour ce qu'il en restait. Si elle est dépassée, la suspension est terminée : la règle de dépassement du §10.1 s'applique — une pause éventuellement engloutie est comptée interrompue et créditée à la dette (décision 14), et non traitée comme une chute sans dette individuelle (§10.2) : c'est le calcul de la suspension qui a scellé son sort, la chute du processus lui est étrangère. Le cycle repart en [TRAVAIL] ou [INACTIF] selon la plage horaire à cet instant |
| Le processus meurt pendant [RETOUR] | La pause était déjà servie : le cycle suivant repart en [TRAVAIL], décompte plein, sans rien à compter de plus |
| Une deuxième instance de Breeze est lancée dans la même session utilisateur | Elle détecte la première déjà active, ramène son panneau au premier plan, et se termine aussitôt sans toucher au cycle en cours |
| Une instance de Breeze tourne dans un autre compte de la même machine | Elle ne voit pas celle du premier compte : chaque compte porte son propre Breeze, sa propre dette, ses propres statistiques, comme s'il s'agissait de deux machines distinctes. Le premier public de Breeze (§2) est une personne seule sur sa machine ; plusieurs comptes actifs n'y sont pas couverts |
| L'utilisateur change rapidement de session | La session d'origine est gelée comme sous n'importe quelle veille, mais le temps qui s'écoule ainsi ne valide jamais une pause par absence : il suit la règle propre au changement de session (§10.1) — une pause due ou en cours qu'il traverse est interrompue et créditée à la dette, jamais prise gratuitement |
| L'utilisateur ouvre une session d'un autre type sur la même machine — X11 un jour, Wayland le lendemain | Le cycle est le même : les échéances murales, le sort des pauses, les réglages et les statistiques sont partagés, puisqu'ils vivent dans le même dossier de données. Seul le relevé des capacités change, au lancement, et avec lui la phrase à froid (§5.7) et le mode des pauses à venir (§10.5). Une pause en cours à l'instant de la fermeture de session est traitée comme n'importe quelle terminaison propre (§10.2) |
| Le compositeur tue une surface d'overlay pendant [PAUSE ACTIVE] | Breeze la repose dans le délai du §12.2 ; si elle est tuée de nouveau, la perte compte comme une chute du processus au sens du coupe-circuit (§10.2). La pause continue de courir ; son sort ne change pas |
| Une application élevée (Windows) ou non énumérable prend le premier plan pendant une pause Simple | Elle n'est pas voilée — Breeze ne la voit pas (§5.2). La pause continue ; Réglages › Applications le dit. Ce n'est pas une porte de sortie : rien n'est écourté |
| Une mise à jour est disponible | Téléchargée en tâche de fond, jamais installée tant qu'une pause est due ou en cours ; appliquée par un redémarrage que Breeze s'impose lui-même dès que rien n'est en cours ou en attente |
| La désinstallation, ou la réouverture de l'onboarding, sont demandées alors qu'une pause est due ou en cours | Ni l'une ni l'autre n'est offerte — aucune commande n'affaiblit une pause en cours ou due (§10.3), et toutes deux en sont |
| L'heure système avance pendant un cycle | Les échéances murales (travail y compris pause due en attente, préavis, suspension) suivent l'heure système : une échéance rapprochée par l'avance tombe plus tôt, ce qui ne fait jamais qu'accélérer une contrainte, jamais la retarder. Une pause en cours ou [RETOUR] suivent leur propre décompte, jamais l'horloge |
| L'heure système recule pendant un cycle | Les échéances murales ne reculent jamais avec elle : Breeze mesure le temps réellement écoulé, immunisé contre un recul manuel de l'horloge — sans quoi reculer l'heure depuis les réglages du système, toujours accessibles sur les trois OS (§10.6), repousserait gratuitement n'importe quelle échéance. Une pause en cours ou [RETOUR] suivent leur propre décompte, jamais l'horloge |
| La plage horaire se ferme pendant que la profondeur retient le début du préavis (§9.3) | La pause est déjà due : elle va à son terme comme depuis [PRÉAVIS] ou [PAUSE ACTIVE] (§10.1), l'horloge ne l'efface pas |
| Le calendrier change pendant [PRÉAVIS] ou [PAUSE ACTIVE] | Aucun effet : le bouclier de calendrier (§9.1) ne replace jamais une pause déjà due |
| La veille ou le verrouillage survient pendant que la profondeur retient le début du préavis (§9.3) | Le plafond de retenue est mural (§10.1) : il continue de courir pendant la veille, comme l'échéance de travail dont il découle |
