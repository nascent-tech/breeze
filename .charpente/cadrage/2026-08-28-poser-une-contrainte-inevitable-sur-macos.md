---
type: cadrage
titre: Poser une contrainte inévitable sur macOS
slug: poser-une-contrainte-inevitable-sur-macos
cree_le: 2026-08-28T15:34:11+0000
mis_a_jour_le: 2026-08-28T15:56:51+0000
branche: develop
statut: brouillon
---

Depuis : `BRIEF.md`

Ce document recueille ce que `BRIEF.md` a cessé de porter : la manière d'affronter les contraintes de
macOS, l'ordre de livraison qu'elles imposent, et les valeurs d'interface que la conception reprendra.
Le brief dit ce qu'il faut construire ; celui-ci dit avec quoi, et ce qu'on refuse d'employer.

**Tout ce qui suit est subordonné au brief.** Là où les deux divergent, le brief fait autorité, et
c'est ce document qui est faux.

---

## 1. Acteur et déclencheur

**L'acteur** : l'équipe technique, avant toute écriture de fonctionnalité.

**Le déclencheur** : le brief est arrêté, et sept de ses décisions dépendent d'un comportement de macOS
que personne n'a encore mesuré (`BRIEF.md`, §12.3). L'une d'elles — la notarisation — est bloquante et
n'a aucun repli.

---

## 2. Le besoin, séparé de la solution

**Le besoin** : rendre une pause inévitable sur une machine dont on ne détient rien, sans jamais
toucher aux applications de l'utilisateur, sans lire son écran, et sans que macOS traite Breeze comme
un logiciel malveillant.

**La solution qui arrive avec la demande** : « prendre l'écran ». C'est la formulation, pas le besoin.
Le besoin est que **le coût de continuer dépasse le coût de s'arrêter** — ce que le brief obtient par
ses règles de négociation bien plus que par ses pixels.

---

## 3. Le vocabulaire

Les termes du produit sont fixés une fois pour toutes en `BRIEF.md`, §4, et ne se redéfinissent pas
ici. Trois termes appartiennent en propre à ce document :

| Terme | Ce qu'il désigne | Ce que ce n'est pas |
|---|---|---|
| **Module natif** | Le composant qui traduit les appels système que l'environnement d'exécution n'expose pas | Pas un lieu de logique produit : il traduit, il ne décide de rien |
| **Cadre de fenêtre** | La position et la taille d'une fenêtre d'une autre application | Pas son contenu, ni son titre — le brief l'interdit (§13) |
| **Reconnaissance** | La phase qui mesure les sept inconnues de `BRIEF.md` §12.3 et soumet un binaire à la notarisation | Pas un prototype d'interface : elle ne produit aucun écran définitif |

### Le contexte borné

**Contexte** : `cycle` — le cycle de travail et de pause, ses états, et les règles qui décident de ce
qui peut le repousser.

**Sorte** : `cœur de métier`, décidée le 2026-08-28.

C'est la seule partie du produit qu'un concurrent ne copie pas en regardant une capture d'écran
(`BRIEF.md`, §3). Recouvrir un écran est une commodité ; le tissu de règles qui empêche l'utilisateur
de reprendre d'une main ce qu'il a concédé de l'autre est ce qui distingue Breeze d'un rappel passif.
Il porte donc l'investissement, et rien n'y est délégué à une bibliothèque tierce.

Deux contextes voisins se distinguent de lui et ne sont pas de sa sorte : la **traduction des appels
système** (§4), qui est du `générique` — elle ne fait que rendre accessible ce que macOS expose déjà —
et la **distribution** (§10), qui est du `support` : indispensable, sans aucune valeur propre.

---

## 4. Ce que l'environnement d'exécution ne sait pas faire seul

L'environnement retenu — Electron — n'expose ni l'application au premier plan, ni les cadres de
fenêtres tierces, ni le masquage du Dock et de la barre de menus. **Un module natif est obligatoire
dès la première version.**

Un unique module natif, compilé pour les deux architectures, expose une surface minimale : identité et
nom de l'application au premier plan, notification de son changement, applications en cours
d'exécution, cadres de fenêtres d'un processus donné, observation de leurs déplacements et
redimensionnements, état et demande de la permission d'Accessibilité, niveau d'empilement d'une
fenêtre, options de présentation, détection du partage d'écran, et énumération des écrans avec leurs
changements à chaud.

**Toute logique produit reste hors du natif.** Le module traduit des appels système ; il ne décide
jamais d'un état du cycle.

**Alternative rejetée** : les dépendances tierces de capture de fenêtre active et de gestion de
permissions. Elles couvrent une partie du besoin, ajoutent deux surfaces de maintenance, et ne donnent
ni les niveaux d'empilement ni les cadres de fenêtres.

---

## 5. Les refus

Ces approches sont écartées, et chacune l'est pour une raison qui ne se rediscute pas au moment où le
planning se tend.

| Approche | Pourquoi elle est refusée |
|---|---|
| **Masquer les applications cibles** au lieu de les recouvrir | Perte de contexte, rendus en cours cassés, et l'utilisateur les récupère d'un clic. Le brief l'interdit (§13) |
| **Suspendre le processus cible** | Corruption de données, connexions rompues, refus de notarisation probable. Une suspension d'une application de visio ou d'une base de données est une catastrophe utilisateur. Le brief l'interdit (§13) |
| **Le mode kiosque de l'environnement, seul** | Sur macOS il déclenche le plein écran natif : un espace de travail dédié, une animation, et un raccourci qui ramène l'utilisateur au bureau. La combinaison des dimensions d'écran, du niveau d'empilement maximal et des options de présentation est strictement supérieure |
| **Piloter les autres applications par événements Apple** | Demande une permission que le brief refuse de demander (§13), pour un résultat qu'aucune fonctionnalité n'exige |
| **Demander la permission d'enregistrement d'écran** | Breeze ne capture rien. La demander serait un signal de défiance majeur, et contredirait la promesse qui fait installer le produit |
| **Intercepter les événements clavier** | Dépassement de permission et risque de notarisation, donc risque sur le produit entier. Vaut aussi pour la détection de session profonde (`BRIEF.md`, §9.3) |
| **Lire le micro ou la caméra pour détecter une visio** | Indéfendable pour une application qui promet de ne rien capter. La détection passe par l'identité de l'application au premier plan, croisée avec une liste que l'utilisateur complète |

---

## 6. Poser un overlay au-dessus d'une fenêtre tierce

macOS n'a **aucune API publique pour placer une fenêtre au-dessus d'une fenêtre précise d'une autre
application** (`BRIEF.md`, §18). L'ordre d'empilement inter-processus appartient au serveur de fenêtres
et n'est pas adressable.

| Approche | Verdict |
|---|---|
| **Suivre le cadre par l'API d'Accessibilité** : lire position et taille, poser une fenêtre sur ces coordonnées, resynchroniser au mouvement | **Retenue.** Publique, signable, notarisable |
| Masquer l'application cible | Rejetée (§5) |
| Suspendre le processus cible | Rejetée (§5) |

**Les limites de l'approche retenue, à budgéter et non à découvrir :**

- La position se suit par observation des notifications de déplacement et de redimensionnement, pas
  par interrogation périodique. Une interrogation de secours couvre les applications qui n'émettent
  pas ces notifications.
- **L'overlay est au-dessus de tout, pas seulement de sa cible.** Quand l'application au premier plan
  est épargnée ou ignorée, les overlays qui l'intersectent doivent s'effacer et revenir ensuite.
  **C'est la vraie complexité du Mode Simple.**
- Une application en plein écran natif occupe son propre espace de travail : l'overlay concerné doit
  basculer au niveau maximal et être déclaré visible en plein écran — sous la réserve de la mesure
  ci-dessous.
- Sans la permission d'Accessibilité, l'approche est impossible. Le comportement de repli est en
  `BRIEF.md` §10.5, et lui seul fait autorité.

**Le Mode Simple est plus difficile à réaliser proprement que le Mode Hardcore.** Si le planning se
tend, c'est le Mode Hardcore qu'il faut livrer en premier, malgré l'intuition inverse. L'ordre du §9
applique déjà cette contrainte.

---

## 7. Prendre l'écran entier

**Ce qui fonctionne de façon fiable :** une fenêtre sans cadre, non déplaçable, non fermable, par
écran physique, posée aux dimensions exactes de son écran, au niveau d'empilement maximal, déclarée
visible sur tous les espaces de travail ; les options de présentation qui masquent le Dock et la barre
de menus et désactivent le changement d'application ; l'enregistrement des raccourcis de fermeture
pendant la pause seulement ; et des écouteurs sur l'ajout, le retrait et la reconfiguration des
écrans. **Un écran non couvert est une porte de sortie complète.**

**Les raccourcis enregistrés sont globaux.** Les oublier après une chute du processus pénalise tout le
système jusqu'au redémarrage de Breeze. Leur désenregistrement doit être garanti par deux chemins
indépendants, dont un qui survit à une sortie anormale — c'est ce que `BRIEF.md` §13 exige quand il
interdit qu'un raccourci survive à la pause qui l'a fait enregistrer.

**Ce qui ne fonctionne pas, ou pas de façon garantie :**

| Contournement | Traitement |
|---|---|
| Changement d'application | Bloqué par les options de présentation |
| Clic sur le Dock | Bloqué : Dock masqué et niveau maximal |
| Mission Control, Exposé | Partiel — le geste reste capté par le serveur de fenêtres. L'overlay revient au-dessus ; l'animation n'est pas évitable |
| Changement d'espace de travail | Bloqué en pratique : l'overlay suit l'utilisateur |
| **Application tierce en plein écran natif** | **Non garanti** — mesure ci-dessous |
| Forcer à quitter | Non interceptable. Assumé et documenté (`BRIEF.md`, §5) |
| Verrouillage puis déverrouillage | Partiel — l'overlay doit être repoussé au premier plan au déverrouillage |
| Basculement d'utilisateur rapide | Non. Assumé |
| Tuer le processus depuis un autre compte | Non. Assumé |
| Débrancher l'écran, éteindre le Mac | Non, et **ne doit pas** l'être |

---

## 8. Les limites et les seuils

### 8.1 Les mesures à rendre

Les sept inconnues de `BRIEF.md` §12.3, dont le repli est déjà décidé. **Chacune se tranche par mesure,
jamais par supposition.** À rendre : un relevé par version de macOS couverte, de 13 à 26.

1. La propagation à chaud de l'octroi d'Accessibilité.
2. Un signal de partage d'écran qui n'exige pas la permission d'enregistrement.
3. Un signal de présentation plein écran distinguable d'un simple plein écran.
4. L'exigence, ou non, de l'Accessibilité pour lire l'application au premier plan.
5. Le comportement de l'overlay au niveau maximal face à une application en plein écran natif — **la
   documentation d'Apple déconseille explicitement la technique pour ce cas** (`BRIEF.md`, §18), et
   c'est la seule mesure qui touche à la promesse du Mode Hardcore.
6. Le comportement du masquage de la barre de menus face à une application tierce déjà en plein
   écran.
7. Le verdict de notarisation sur un binaire portant ces niveaux de fenêtre et cet usage de
   l'Accessibilité. **Bloquant, sans repli.** La soumission a lieu pendant la reconnaissance, avant
   toute écriture de fonctionnalité.

### 8.2 Les pièges de la permission d'Accessibilité

**L'autorisation suit l'identité signée, pas le chemin** (`BRIEF.md`, §18). Chaque construction non
signée crée une identité différente : l'utilisateur ré-autorise à chaque itération, et les entrées
mortes s'accumulent dans les réglages du système. Trois conséquences opérationnelles :

- Signer avec un certificat de développeur **dès le premier jour**, y compris en local. Non
  négociable.
- En développement, l'entrée de permission porte le nom de l'environnement d'exécution, pas celui de
  Breeze. Le documenter évite des faux défauts.
- Toute modification du paquet après signature invalide l'autorisation. La chaîne de construction
  signe **après** l'inclusion du module natif, élément par élément puis le paquet entier.

**La révocation est silencieuse.** L'état de la permission se revérifie au lancement, au début de
chaque cycle et à chaque réveil. `BRIEF.md` §10.5 dit ce que le produit fait en cas de perte.

### 8.3 Le temps

**Les minuteries de l'environnement d'exécution dérivent et ne comptent pas pendant la veille.** La
source de vérité est une **échéance de phase**, tenue sous deux formes : une horloge monotone pour le
décompte pendant que le processus vit, une horloge murale pour la persistance et le recalcul après
veille. Le rafraîchissement d'affichage ne sert qu'à l'affichage ; tout calcul de temps restant est
une différence d'échéances, jamais un décrément.

L'état du cycle s'écrit à chaque transition, jamais à chaque battement d'horloge, en écriture atomique
et avec une version de schéma qui permet les migrations.

**L'instance unique est obligatoire.** Deux instances posant chacune un overlay Hardcore est le pire
défaut possible du produit, et `BRIEF.md` §13 l'interdit.

### 8.4 La performance

Une application de barre de menus est jugée sur ce qu'elle ne coûte pas. Les cibles sont fixées par
`BRIEF.md` §12.2 et ne se renégocient pas ici. Les moyens : détruire les fenêtres d'overlay après
usage plutôt que les masquer, aligner le rafraîchissement sur la cadence du titre affiché dans la
barre de menus, ne pas animer ce titre, et remplacer toute interrogation périodique par une
notification système partout où elle existe.

### 8.5 Les valeurs que la conception d'interface reprendra

Ces valeurs viennent de la première rédaction du brief. Elles ne sont ni arrêtées ni sourcées : elles
sont **le point de départ de `DESIGN.md`**, qui les tranchera avec ses contrastes calculés.

| Élément | Valeur de départ |
|---|---|
| Fenêtre d'onboarding | 720 × 520, non redimensionnable, centrée |
| Panneau de la barre de menus | 320 de large, hauteur adaptative |
| Bannière de préavis et de pause | 380 × 96, coin supérieur droit, sur l'écran qui porte la barre de menus, jamais dupliquée |
| Voile du Mode Simple | 55 % d'opacité, flou d'arrière-plan léger — le contenu reste devinable, jamais lisible |
| Fond du Mode Hardcore | Opaque, dégradé sombre animé très lentement |
| Dissolution de l'overlay en fin de pause | 400 ms |
| Bannière de retour | 3 secondes |
| Masquage du curseur en Mode Hardcore | Après 3 secondes d'immobilité |
| Interrogation de l'état des permissions à l'onboarding | 1 Hz |
| Interrogation de secours des cadres de fenêtres | 4 Hz |
| Titre à côté de l'icône | Police monospace tabulaire, pour que la barre ne sautille pas |
| Cadence de rafraîchissement du titre | Chaque seconde en compte à rebours complet, chaque minute en minutes restantes, aucune si le titre est masqué |

Les trois rythmes proposés à l'onboarding — 25/5, 50/10 en pré-sélection, et 90/15 — relèvent du
produit et sont repris de `BRIEF.md` §12.2.

---

## 9. L'ordre imposé par les contraintes

Cet ordre fixe une **succession et une sortie vérifiable**, pas des durées : une estimation se pose à
l'entrée d'une phase, quand son inconnue principale est levée, jamais avant. Le découpage en lots
livrables relève de `MVP.md`, qui le consommera.

| # | Contenu | Sortie vérifiable |
|---|---|---|
| 0 | **Reconnaissance** — module natif minimal, les sept mesures du §8.1 | Un relevé qui tranche chaque mesure, un prototype qui pose un overlay inévitable trente secondes, et un binaire soumis à la notarisation |
| 1 | **Squelette** — barre de menus, machine à états, persistance, échéances, réglage du rythme | Un cycle complet tourne, sans aucun overlay |
| 2 | **Hardcore** — overlay total multi-écran, neutralisation des sorties, sortie d'urgence, coupe-circuit | Une pause Hardcore tient face à une tentative active de contournement, hors les sorties déclarées impossibles au §7 |
| 3 | **Applications** — découverte, statuts, déclencheur, exceptions, budget de report | Le décompte n'avance qu'avec une application déclencheuse au premier plan, et rien ne repousse une pause au-delà du budget |
| 4 | **Simple** — overlays par fenêtre, suivi des cadres, effacement à l'intersection d'une application épargnée | Une application bloquée est figée, une application épargnée reste utilisable, et déplacer la première déplace son overlay |
| 5 | **Finition** — onboarding complet, statistiques, accessibilité, deux langues, signature, notarisation, mise à jour | Un binaire notarié installé sur une machine vierge, onboarding mené jusqu'au premier cycle sans intervention |

**La phase 0 précède tout.** Un refus de notarisation arrête le produit, et le découvrir après la
phase 4 coûterait tout le développement.

---

## 10. La distribution

**Hors Mac App Store**, et ce n'est pas un choix (`BRIEF.md`, §5, §18). Distribution directe, signée
et notariée, avec mise à jour depuis un dépôt de publication.

- **Vérification au lancement puis toutes les 24 heures, jamais pendant une pause ni un préavis.**
  Téléchargement en arrière-plan, installation au prochain lancement. Une mise à jour ne quitte jamais
  l'application d'elle-même.
- Constructions universelles pour les deux architectures.
- **Le retrait de l'entrée de permission n'est pas automatisable** : il est expliqué à l'utilisateur
  (`BRIEF.md`, §8.7).
- **La notarisation est indispensable.** Une application non notariée qui demande l'Accessibilité et
  prend l'écran entier a le profil exact d'un logiciel malveillant, et les utilisateurs ont raison de
  s'en méfier.

---

## 11. Les garde-fous de l'exécution

- Isolation du contexte, aucune intégration de l'environnement d'exécution, bac à sable actif sur tous
  les rendus, préchargement minimal, communication inter-processus typée et validée du côté du
  processus principal, sécurité web jamais désactivée.
- **Les rendus d'overlay ne chargent aucune ressource distante.** Le brief interdit toute sortie
  réseau hors la vérification de mise à jour (§13).
- Le journal local est circulaire et borné. Il contient les transitions d'état, les identités
  d'applications vues, les permissions perdues et les erreurs — **jamais un titre de fenêtre, jamais
  un contenu applicatif** (`BRIEF.md`, §10.6).

---

## 12. Hors périmètre de ce cadrage

- **Tout ce que le brief tranche.** Les règles de négociation, les états, les compteurs, les valeurs
  par défaut et les constantes ne se rediscutent pas ici.
- **Le découpage en pull requests**, qui relève du plan.
- **Les tokens de design, les contrastes et les composants**, qui relèvent de `DESIGN.md`. Le §8.5 ne
  donne qu'un point de départ.
- **Le modèle économique**, tranché par `BRIEF.md` §11.

---

## 13. À décider

Rien n'est en attente d'un arbitrage humain. Les sept inconnues du §8.1 se tranchent par mesure, et
leur repli est déjà décidé dans le brief — **aucune ne peut bloquer la conception ni le
développement**, sauf la notarisation, dont le refus arrête le produit et qui est traitée en premier
pour cette raison.
