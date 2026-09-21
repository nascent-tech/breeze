## 11. Comment le produit se paie

**Il ne se paie pas au lancement.** Breeze est gratuit, sans licence, sans activation, sans compte.

**Ce n'est pas une décision commerciale, c'est une conséquence du refus de collecte (§10.6).** Vérifier
une licence en ligne établit une connexion, transmet un identifiant, tient un registre côté serveur de
qui utilise le produit — exactement ce que Breeze refuse. Le contredire pour encaisser au lancement
reviendrait à vendre la raison pour laquelle on installe un produit qui demande l'Accessibilité et
prend l'écran entier.

**Ce qui n'est jamais facturé, quelle que soit la monétisation retenue plus tard :** la pause
elle-même, le geste de sortie du Mode Hardcore, et la désinstallation. Aucun garde-fou ne se paie.

**À quelle condition ce choix se révise.** Avec un modèle qui n'exige aucune vérification en ligne —
un achat unique vérifié hors ligne, ou une version payante distribuée séparément. Il ne se révise pas
par un abonnement à validation périodique, qui est précisément ce que le §10.6 interdit.

---

## 12. Les limites, et où vivent les chiffres

### 12.1 Ce que chaque système fixe

Les versions couvertes, par OS. Tout ce que chaque système impose au-delà est au §5, avec la
conséquence produit de chaque contrainte.

| OS | Versions couvertes | Ce qui n'est pas couvert, et pourquoi |
|---|---|---|
| macOS | 13 Ventura et ultérieures, sur les deux architectures de processeur | Les versions antérieures : le démarrage automatique et les notifications y suivent des mécanismes différents, et la notarisation y est moins prévisible |
| Windows | 10 à partir de la version 1809, et 11 | Les versions antérieures de Windows 10 : les notifications système y sont incomplètes |
| Linux X11 | Tout gestionnaire de fenêtres conforme aux conventions courantes d'empilement et d'identité (EWMH) | Un gestionnaire non conforme : Breeze y tourne, mais le premier plan et les cadres y sont déclarés inconnus (§10.5) |
| Linux Wayland | KDE Plasma 5.20 et ultérieurs ; les compositeurs wlroots (Sway, Hyprland, river) | **GNOME Wayland est couvert en mode déclaré dégradé** (§5.6) — pas exclu, pas garanti. XWayland n'est jamais un mode de fonctionnement : une fenêtre X11 ne passe pas devant les fenêtres Wayland natives, et Breeze y serait dégradé sans le savoir |

Le choix de la session se fait au lancement sur ce qu'elle expose réellement, jamais sur le nom du
bureau (§5.7).

### 12.2 Ce que le produit décide

Ces valeurs sont des **constantes**, pas des réglages : elles ne figurent nulle part dans l'interface
des réglages.

| Ce que le produit fixe | La valeur | Pourquoi elle est constante |
|---|---|---|
| Durée du préavis | 1 minute, fixe une fois commencé | Assez pour finir une phrase ; rien ne le rallonge après coup, sans quoi ce ne serait plus une échéance mais une nouvelle forme de négociation |
| Durée de [RETOUR] | 3 secondes | Le temps de dire que c'est fini |
| Durées de suspension offertes | 15 minutes, 1 heure, jusqu'au prochain 6 h (heure locale) | Trois durées, et aucune suspension sans terme : une suspension qu'on oublie de lever est une désinstallation qui ne dit pas son nom |
| Rythmes prêts à l'emploi, proposés à l'onboarding (§8.7) | Court (25 min de travail, 5 de pause), Standard (50 min, 10 — c'est aussi le défaut du produit), Long (90 min, 20 min) | Trois points d'entrée réels plutôt qu'un écran de saisie vierge en première ouverture ; le réglage fin reste accessible ensuite |
| Borne du journal local | 7 jours, ou 5 Mo — la première atteinte l'emporte | Couvre le diagnostic d'un incident sur une semaine, sans avoir à le garder plus longtemps |
| Maintien de la touche pour le geste du Mode Hardcore | 10 secondes | Insupportable pour tricher, acceptable en cas de besoin réel |
| Inactivité au-delà de laquelle la phase de travail gèle | 3 minutes | Rappeler une pause à quelqu'un déjà parti n'a pas de sens |
| Seuil d'armement du coupe-circuit | 3 chutes du processus en 5 minutes | Un défaut répété, pas un accident isolé |
| Désarmement automatique du coupe-circuit | 24 heures sans nouvelle chute | Ou un désarmement explicite depuis les réglages |
| Profondeur des statistiques | 30 derniers jours glissants, stockées séparément du journal circulaire de 7 jours | Au-delà, il faudrait un écran d'exploration que la promesse ne justifie pas ; c'est cette réserve, plus longue et exportable, qui porte les mesures produit du §12.3, jamais le journal |
| Heure d'effacement de la dette de posture (§9.2) | Minuit, heure locale | Même convention que les statistiques ; la sévérité qu'une dette encore au-dessus du seuil aurait imposée au cycle suivant s'efface avec elle |
| Bornes de saisie du rythme | 5 à 180 minutes de travail, 1 à 60 minutes de pause, la pause ne dépassant jamais le travail | Hors de ces bornes, le cycle cesse d'être un cycle |
| Délai de pose d'un overlay sur un écran ou une application apparus en cours de pause | 500 millisecondes | Au-delà, la fenêtre d'échappement devient exploitable |
| Délai de retour devant de l'overlay Hardcore quand une fenêtre passe dessus (Windows, Linux X11, §5.3) | 500 millisecondes | Le même chiffre, pour la même raison : c'est la promesse dite à froid — « revient devant en moins d'une demi-seconde » — et l'hypothèse du §17 |
| Délai de pose de l'overlay Hardcore à l'échéance | 200 millisecondes | La pause doit tomber, pas s'installer |

**Deux chiffres de ce tableau, et deux hors de lui, ne sont pas des chiffres de succès au sens du
§12.3 et de la décision 20** : les quatre-vingt-dix secondes de la première ouverture (§8.7) et les
trois budgets ci-dessous sont des engagements d'ingénierie — ce que Breeze s'impose à lui-même —
jamais une mesure de ce que l'utilisateur en fait. Ils font autorité au même titre que le reste de
cette liste.

**Ce que le produit s'impose en plus**, parce qu'une application d'icône d'état est jugée sur ce
qu'elle ne coûte pas : moins de 120 Mo de mémoire au repos **sur chacun des trois OS**, moins de
0,3 % de processeur en moyenne sur dix minutes, et l'absence du palmarès système des applications à
forte consommation d'énergie là où le système en tient un. La mémoire au repos se mesure par OS
(§12.3) ; si un OS dépassait le budget, la réponse serait de revoir le budget pour cet OS, jamais de
retirer une capacité pour y tenir.

**Les valeurs par défaut du produit** — celles qu'un utilisateur qui ne règle rien obtient : 50
minutes de travail, 10 minutes de pause, sévérité Simple, plage horaire désactivée, les sept jours
actifs, statut `bloquée` pour toute application inconnue, minutes restantes affichées à côté de
l'icône, lancement au démarrage activé, langue du système avec l'anglais à défaut, thème du système,
vérification des mises à jour activée. **Cette liste fait autorité sur tout défaut du produit** ;
aucune autre section n'en donne.

### 12.3 Ce qu'on ne connaît pas encore

**Une valeur qu'aucun fait ne fixe n'est pas une décision qui manque : c'est une mesure.** Chacune a un
responsable, une forme, et un comportement en son absence. **L'absence d'une valeur ne permet jamais
rien** : elle bloque, ou elle applique le repli le plus conservateur.

**Les mesures techniques.** Porteur : l'équipe technique, pendant la reconnaissance qui précède toute
écriture de fonctionnalité. Forme : un relevé par version d'OS couverte et, sur Linux, par
compositeur de la matrice du §12.1, versé au cadrage technique — la **matrice de capacités mesurée**
qui donne au §5.6 ses cases réelles.

| Ce qui se mesure | Régime en son absence |
|---|---|
| **macOS** — L'octroi de l'Accessibilité se propage-t-il sans relancer l'application, sur chaque version couverte ? | **Repli** : sur les versions concernées, l'onboarding propose une relance explicite au lieu d'attendre |
| **macOS** — Lire l'application au premier plan exige-t-il l'Accessibilité sur une version couverte ? | **Repli** : sur cette version, tout usage compte comme du travail sans distinction possible — le côté sûr, puisque le décompte avance plus vite plutôt que de geler à tort |
| **macOS** — L'overlay Hardcore se pose-t-il au-dessus d'une application tierce en **plein écran natif**, sur chaque version couverte ? Apple déconseille la technique et n'en garantit pas le résultat (§5.3, §18) | **Repli** : le Mode Hardcore déclare la limite **à froid**, à l'onboarding et au réglage de sévérité (§5.7), et une pause servie sans couverture compte comme les autres |
| **macOS** — Le masquage de la barre de menus tient-il face à une application tierce déjà en plein écran ? | **Repli** : le masquage est abandonné sur ce seul cas, sans autre conséquence |
| **macOS** — La notification de verrouillage d'écran, stable mais non documentée, est-elle émise sur chaque version couverte (§5.4) ? | **Repli** : sur une version où elle manque, un verrouillage compte comme une inactivité de même durée — le décompte avance, rien n'est levé |
| **macOS** — La notarisation accepte-t-elle un binaire portant ce niveau de fenêtre maximal et cet usage de l'Accessibilité, tels que la voie technique retenue les produit ? | **Bloquant.** Aucun repli : un refus arrête le produit. La soumission a lieu avant toute écriture de fonctionnalité |
| **Windows** — Une application en **plein écran exclusif** passe-t-elle devant l'overlay Hardcore, et l'overlay revient-il devant à la sortie du plein écran ? | **Repli** : la phrase à froid Windows (§5.7) garde sa dernière phrase, et une pause servie sans couverture compte comme les autres |
| **Windows, Linux X11** — L'overlay Hardcore revient-il devant en **moins de 500 ms sous charge** — processeur saturé, trois écrans, une fenêtre « toujours devant » lancée après lui ? | **Repli** : tant que la mesure manque, la phrase à froid ne promet aucun délai chiffré — « revient devant » sans « en moins d'une demi-seconde » ; le chiffre du §12.2 reste l'objectif d'ingénierie |
| **Linux Wayland** — La capture clavier exclusive par la couche overlay est-elle honorée, compositeur par compositeur (KDE, Sway, Hyprland, river) ? | **Repli** : sur un compositeur où elle est refusée ou ignorée, les raccourcis sont déclarés non garantis (§5.7), et rien de plus n'est promis |
| **Linux Wayland GNOME** — Face à deux fenêtres plein écran de Breeze, une par sortie, GNOME les garde-t-il toutes les deux devant, ou n'en garde-t-il qu'une ? | **Repli** : le Mode Hardcore y est déjà déclaré non garanti (§5.3) ; si une sortie reste découverte, la phrase à froid GNOME le dit, et la pause compte comme les autres |
| **Linux** — Le gestionnaire de session logind est-il présent sur la machine ? Relevé au lancement, pas seulement en reconnaissance | **Repli** : sans logind, la veille est déduite d'un saut de l'horloge monotone et traitée comme une absence (§5.4, §10.4) ; le verrouillage n'est pas vu et compte comme une inactivité |
| **Trois OS** — La mémoire au repos — icône seule, aucune fenêtre — et avec trois overlays Hardcore posés, tient-elle sous les 120 Mo du §12.2, OS par OS ? | **Repli** : le budget de l'OS en dépassement est revu et déclaré, jamais une capacité retirée pour y tenir (§12.2) |

**Les mesures produit.** Porteur : le fondateur, sur une bêta privée. Forme : l'export manuel des
statistiques locales (§12.2), que chaque participant envoie lui-même — le refus de collecte du §10.6
interdit toute remontée automatique. Le journal circulaire de 7 jours n'entre pas dans ces mesures :
il porte le diagnostic, pas la tendance à trente jours.

| Ce qui se mesure | Régime en son absence |
|---|---|
| La part des participants de la bêta à qui le bouclier de calendrier (§9.1) est proposé et qui accordent la permission Calendriers | **Bloquant** pour cette fonctionnalité : c'est la mesure que le §16 nomme sans lui donner sa forme — elle n'entre pas au lancement tant que cette part n'a pas été observée |
| La part des installations qui ont encore un cycle actif après trente jours | **Repli** : aucun seuil n'est opposé au produit tant que la bêta n'a pas rendu ses chiffres |
| La part des pauses servies plutôt qu'interrompues | **Repli** : idem |
| La part des utilisateurs qui ont choisi le Mode Hardcore au moins une fois pendant leur première semaine | **Repli** : idem — c'est l'une des mesures qui tranchent la première hypothèse du §17 |
| La part des utilisateurs Windows et Linux qui gardent le Mode Hardcore après leur première pause Hardcore, comparée à celle des utilisateurs macOS ; et, dans l'export, le nombre de fois où une fenêtre est passée devant l'overlay pendant une pause | **Repli** : aucun seuil n'est opposé au produit tant que la bêta n'a pas rendu ses chiffres — c'est la mesure qui tranche la quatrième hypothèse du §17 |
| Le nombre de gestes de sortie du Mode Hardcore par utilisateur et par semaine | **Repli** : idem. Les dix secondes du §12.2 restent la constante tant que rien ne les conteste |
| La fréquence du raccourci de fermeture du système ou du menu de l'icône d'état pendant une pause active, comparée à celle du geste de sortie du Mode Hardcore | **Repli** : aucun seuil n'est opposé au produit tant que la bêta n'a pas rendu ses chiffres — c'est la mesure que le §17 nomme sans lui donner sa forme |
| La part des pauses servies plutôt qu'interrompues, mesurée séparément pour les utilisateurs à profondeur habituellement élevée (§9.3) | **Repli** : idem — c'est la mesure que le §17 nomme sans lui donner sa forme |
| Le nombre de reconfigurations du rythme par utilisateur et par mois | **Repli** : aucun signal n'est présumé ; les profils de réglages multiples restent hors périmètre tant que ce signal ne paraît pas (§14) |
| L'utilité déclarée d'une tendance longue, une fois la dette de posture en service | **Repli** : idem ; les statistiques restent bornées à trente jours tant que ce signal ne paraît pas (§14) |
| Le seuil de durcissement de la dette de posture — au-delà de combien de minutes le Mode Hardcore s'impose (§9.2) | **Bloquant** pour cette fonctionnalité : elle n'entre pas au lancement tant que ce seuil n'a pas été mesuré en test utilisateur. Ce qu'elle crédite, en revanche, est déjà fixé (§9.2) : ce n'est plus une mesure |
| Les seuils de l'indice de profondeur et le plafond de retenue du préavis (§9.3) | **Bloquant** pour cette fonctionnalité, même raison — un plafond mal calibré rend l'exception plus large que la règle qu'elle est censée rester |

**Aucun chiffre de succès n'est fixé dans ce brief**, et c'est délibéré : une cible de rétention posée
avant la première mesure n'est pas un objectif, c'est un nombre auquel on finit par ajuster la lecture
des faits.
