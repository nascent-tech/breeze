## 1. Le projet en une page

Breeze est une application de bureau — logée dans la barre de menus sur macOS, dans la zone de
notification sur Windows et Linux — qui **impose la pause**. Là où les concurrents
envoient une notification qu'on balaie sans y penser, ou proposent un report qu'on finit toujours par
prendre, Breeze ne demande rien : à l'heure due, il recouvre les applications qui retiennent
l'utilisateur, ou l'écran entier, et rien dans l'interface ne permet de l'écourter, de la retarder ou
de l'annuler.

**Ce que Breeze ne détient pas.** Il ne détient ni le système d'exploitation, ni les applications
qu'il recouvre, ni aucun droit d'administration sur la machine. Le système d'exploitation — macOS,
Windows ou Linux — reste l'arbitre : il accorde ou retire les permissions, garde l'ordre d'empilement
des fenêtres, et laisse toujours à l'utilisateur le moyen de quitter Breeze ou d'éteindre sa machine.
Breeze n'a aucun pouvoir que son système ne lui prête, et il ne prétend jamais en avoir un.

**Ce que Breeze détient, c'est l'absence de bouton.** Pas de délai à demander, pas de « encore cinq minutes »,
pas d'exception programmée pour une réunion. La seule issue est celle que le système garantit déjà —
quitter Breeze (§5) —, et le Mode Hardcore lui ouvre une seconde porte au même prix, jamais moins
chère (§8.5) ; aucune des deux n'est un choix confortable que Breeze offrirait, et aucune n'est
gratuite (§10.2).

Ce que Breeze promet, en une phrase : **la pause n'est pas une proposition, c'est une échéance.**

---

## 2. À qui ça s'adresse, et sur quel terrain

**Qui.** Un travailleur du savoir — développeuse, designer, analyste, rédacteur — devant son écran
plusieurs heures par jour, seul devant sa machine, qu'elle tourne sous macOS, Windows ou Linux :
personne au-dessus de lui n'installe ni ne configure Breeze à sa place.

**Ce qu'il vit aujourd'hui.** Il sait qu'il devrait s'arrêter. Il a déjà réglé des rappels de pause,
et il a fini par les désactiver. Le rappel n'a pas échoué parce qu'il était mal réglé :
il a échoué **parce qu'il restait négociable**, et la négociation se gagne toujours à l'instant précis
où l'on est le moins capable de la refuser à soi-même.

**La même personne, deux états, et le produit repose entièrement sur leur écart.** À froid — le
matin, dans les réglages —, elle choisit un rythme et se dit prête à s'y tenir. À chaud — à la
minute 47 d'un bloc de travail —, elle veut continuer, et elle a toujours une bonne raison. **Breeze
ne laisse plus aucune prise à celle qui négocie à chaud** : aucune part du produit n'écoute ses
arguments, aussi bons soient-ils, une fois la pause due. Le choix se fait entièrement à froid, ou pas
du tout — c'est un cran plus loin que « donner raison à celle qui décide à froid » : c'est ne plus
donner la parole à celle qui négocie à chaud.

**Ce que le terrain impose.** Chaque système d'exploitation protège délibérément son utilisateur
contre les applications qui prennent l'écran en otage, et il a raison de le faire — macOS par ses
permissions et ses niveaux de fenêtre, Windows par son bureau sécurisé et ses raccourcis système,
Linux par un gestionnaire de fenêtres ou un compositeur qui garde le dernier mot sur l'empilement.
Aucune contrainte que Breeze pose n'est absolue : couper l'alimentation, forcer à quitter depuis un
autre compte, débrancher l'écran restent possibles et **doivent** le rester (§5). Breeze ne prétend
donc jamais à l'infaillibilité, et le dit avant que l'utilisateur l'apprenne tout seul — une promesse
démentie une fois se paie plus cher que l'aveu qui la précède. Ce que chaque système laisse passer
n'est pas le même d'une machine à l'autre ; le §5 le dit plateforme par plateforme, et l'onboarding
le dit en une phrase, propre à la session détectée.

---

## 3. Ce que Breeze fait, et ce qu'il n'est pas

**Ce qu'il fait.** Il compte le temps de travail, annonce la pause une minute avant, puis rend l'écran
— ou les seules applications désignées — inutilisable pendant la durée de la pause. Aucun bouton ne
l'écourte, ne la retarde, ni ne l'annule.

**Ce que ça donne concrètement.** À 14 h 50, une bannière annonce la pause dans une minute. Il n'y a
rien à cliquer. À 14 h 51, Figma et Slack se voilent, le décompte apparaît, et rien ne les ramène
avant 15 h 01. Notes et Spotify, épargnés à froid, continuent de fonctionner.

**Pourquoi ce n'est pas copiable en un trimestre.** L'overlay se copie en un week-end. Ce qui ne se
copie pas, c'est de **refuser durablement** chaque demande de bouton « juste cette fois » — un report,
une exception, un mode « là je suis vraiment occupé ». Un concurrent qui cède une seule fois à cette
demande, dans une seule version, redevient un rappel qu'on négocie, et perd la seule chose qui
distingue Breeze : que la pause ne dépend de la conversation de personne, pas même de celle qu'on
aurait avec son éditeur produit.

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
- **Ce n'est pas un produit qui promet la même chose partout.** Breeze tourne sur trois systèmes,
  mais il ne prétend jamais qu'une machine tient la pause aussi bien qu'une autre : ce qui est
  garanti chez chacun est dit à froid (§5), jamais découvert en pause.

**Où Breeze tourne.** Sur macOS 13 et ultérieurs ; sur Windows 10 (1809) et ultérieurs, et
Windows 11 ; sur Linux en session X11 ; sur Linux en session Wayland avec les compositeurs qui
exposent les protocoles nécessaires — KDE Plasma 5.20 et ultérieurs, et les compositeurs wlroots
tels que Sway, Hyprland et river. Sur GNOME Wayland, Breeze tourne en **mode déclaré dégradé** : le
cycle et la pause y sont entiers, la couverture visuelle et la détection y sont réduites, et Breeze
le dit dès l'onboarding (§5). Les versions couvertes vivent au §12.1.

**Le principe qui rend les trois plateformes possibles sans changer le produit : la promesse de
Breeze ne dépend d'aucune détection.** Le cycle tourne sans rien lire de la machine. Chaque capacité
que le système refuse ou n'expose pas ne fait qu'une chose parmi deux — soit le décompte de travail
avance plus vite (tout compte comme du travail, le côté sûr), soit la couverture visuelle de la pause
est moins complète, et Breeze le dit à froid. **Une capacité perdue accélère le décompte ou réduit la
couverture ; jamais elle ne lève une contrainte.** C'est ce principe, et lui seul, qui autorise le §5
à lister ce qui manque sur chaque machine sans qu'aucune de ces lignes n'ouvre une négociation.
