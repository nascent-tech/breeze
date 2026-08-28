---
type: plan
titre: Reconnaissance macOS
slug: chore-reconnaissance-macos
cree_le: 2026-08-28T18:47:57+0000
mis_a_jour_le: 2026-08-28T20:47:46+0000
branche: develop
statut: en_cours
---

Depuis : `.charpente/cadrage/2026-08-28-poser-une-contrainte-inevitable-sur-macos.md`

Ce plan couvre **la phase 0 du cadrage §9, « Reconnaissance », et elle seule**. Sa sortie est celle
que le cadrage fixe : un relevé qui tranche **les six mesures non bloquantes** de `BRIEF.md` §12.3
sur chaque version de macOS couverte, **le verdict de notarisation**, qui est la septième et ne se
décline pas par version, un prototype qui pose un overlay inévitable trente secondes, et un binaire
soumis à la notarisation.

Les phases 1 à 5 du cadrage §9 ne sont pas planifiées ici, et ce n'est pas un oubli : six des sept
mesures conditionnent la conception des phases 2 et 4, et les planifier avant que les mesures soient
rendues serait planifier sur des suppositions. Chacune recevra son plan quand son inconnue
principale sera levée — c'est la règle que le cadrage §9 pose lui-même.

**Un écart déclaré avec l'amont** : le cadrage §8.1 demande « un relevé par version de macOS
couverte » pour les sept inconnues. Ce plan retire la septième de la matrice. Le verdict de
notarisation porte sur un binaire, pas sur une exécution : Apple juge la signature, les droits et le
runtime durci d'un paquet, et son verdict ne change pas selon la version de macOS qui l'exécutera.
Le décliner sur quatre versions produirait quatre soumissions du même paquet et quatre fois le même
verdict.

---

## 1. Le modèle

### Ce que la Reconnaissance ne modélise pas

Le cœur de métier — le contexte `cycle`, ses états, son budget de report, ses invariants — n'entre
pas dans cette phase. Le cadrage §9 le place en phase 1, et `.charpente/glossaire/cycle.md` en porte
déjà le vocabulaire arrêté. **Aucun agrégat `Cycle`, aucun objet-valeur `PostponeBudget`, aucun
événement `BreakBecameDue` n'est écrit ici.**

Deux contextes seulement sont touchés :

| Contexte | Sorte | Ce qu'il porte en phase 0 |
|---|---|---|
| `traduction-systeme` | `générique` | Les ports vers macOS, leurs doubles, le module natif qui les implémente |
| `distribution` | `support` | La chaîne de construction signée et la soumission à la notarisation |

### Le contexte `traduction-systeme` n'a pas de domaine

C'est la conclusion du test de `charpente:domain-modeling`, appliquée aux concepts que ce contexte
manipule, et elle décide de son arborescence.

**Première question — existe-t-il un invariant transactionnel entre ces objets ?** Non, et pour une
raison de nature : l'application au premier plan, les cadres de fenêtres d'un processus, les écrans
branchés et l'état de la permission d'Accessibilité sont des **observations d'un état que macOS
détient**. Rien ici ne se valide, ne se refuse ni ne se corrige : ce contexte ne décide de rien,
comme son glossaire l'énonce. Un concept sans invariant transactionnel n'ouvre pas d'agrégat.

**La question de survie** — le concept est-il cherché, nommé et supprimé pour lui-même ? Non plus :
personne ne charge un cadre de fenêtre par requête, personne ne le supprime. Il est lu, il est
remplacé en bloc au mouvement suivant, il est oublié.

Le test envoie donc chaque concept du côté des **objets-valeur**, et le contexte n'a **aucune racine
d'agrégat ni aucune entité interne**. Sa sorte le confirme : un contexte `générique` qui se
construirait un modèle riche serait exactement l'investissement mal placé que le classement du
cadrage §3 sert à interdire.

**Conséquence sur l'arbre** : `src/modules/system-bridge/` n'a **pas de dossier `domain/`**. Ses
objets-valeur vivent avec les ports qui les échangent, sous `application/model/`. Créer un `domain/`
vide, ou y ranger ces objets-valeur pour respecter une forme, donnerait à lire une frontière de
cohérence qui n'existe pas.

### Les objets-valeur du contexte

Tous immuables, gelés à la construction, valides par construction, égalité structurelle. Aucun ne
porte de comportement au-delà de sa validation.

| Concept | Nature | Ce qu'il porte | Ce qu'il ne porte pas |
|---|---|---|---|
| `AppIdentity` | objet-valeur | Identifiant de paquet et nom affiché de l'application au premier plan | Aucun titre de fenêtre, aucun contenu — `BRIEF.md` §13 l'interdit |
| `WindowFrame` | objet-valeur | Position et taille d'une fenêtre, et l'identifiant du processus qui la porte | Le contenu de la fenêtre, son titre |
| `Display` | objet-valeur | Identifiant d'un écran, ses dimensions, s'il porte la barre de menus | Sa luminosité, son profil couleur |
| `AccessibilityStatus` | objet-valeur | Accordée, refusée, ou jamais demandée | Le moment de l'octroi — l'observation est un niveau, pas un front |
| `WindowLevel` | objet-valeur | Le niveau d'empilement d'une fenêtre de Breeze | Le niveau d'une fenêtre tierce, que macOS n'expose pas |
| `Shortcut` | objet-valeur | La combinaison de touches d'un raccourci global, et l'identité de l'enregistrement | La frappe elle-même — Breeze n'intercepte aucun clavier hors ses propres raccourcis |
| `Instant` | objet-valeur | Un point de temps, sous ses deux formes — monotone et murale | Une durée : une durée est une différence d'échéances |

`Instant` est le seul concept de ce plan qui anticipe la phase 1, et il le fait parce que le cadrage
§8.3 en fait une contrainte de fondation : les minuteries de l'environnement dérivent et ne comptent
pas pendant la veille, la source de vérité est une échéance tenue sous deux formes. Le prototype de
la phase 9 est le premier objet qui a besoin d'une échéance ; il l'obtient par ce port et pas par un
décrément.

### Le contexte `distribution` n'a aucun concept d'exécution

Son glossaire le dit : `notarization` appartient à la chaîne de construction, pas au code qui tourne.
Il ne reçoit donc **ni module sous `src/`, ni port** — seulement des scripts sous `build/`, et un
verdict versé au cadrage. `UpdateCheck`, le second terme de son glossaire, appartient à la phase 5 du
cadrage §9 et n'est pas écrit ici.

---

## 2. Les ports

Neuf ports, tous **secondaires** : ils disent ce dont le cœur a besoin de macOS. Chacun a deux
implémentations dès sa naissance — l'adaptateur natif, et un double écrit à la main —, et c'est cette
seconde implémentation qui justifie son existence. Sans elle, aucune phase ultérieure ne serait
testable ailleurs que sur un Mac, avec les permissions accordées, à la main.

Ils vivent sous `src/modules/system-bridge/application/ports/`. Pas sous `domain/ports/`, que la
garde refuse et que l'absence de domaine rend sans objet.

| Port | Ce qu'il expose | Ce qu'il y a à remplacer | Adaptateur livré en phase |
|---|---|---|---|
| `ForegroundAppPort` | L'`AppIdentity` au premier plan, la liste des applications en cours, l'abonnement au changement de premier plan | Natif `NSWorkspace` ↔ double qui rejoue une suite de changements scriptée | 6 |
| `WindowFramesPort` | Les `WindowFrame` d'un processus, l'abonnement à leurs déplacements et redimensionnements, l'interrogation de secours | Natif API d'Accessibilité ↔ double qui rejoue une trajectoire de fenêtre | 8 |
| `AccessibilityPermissionPort` | L'`AccessibilityStatus` courant, la demande d'octroi, la revérification | Natif ↔ double qui bascule l'état sur commande, y compris la révocation silencieuse | 3 |
| `WindowLevelPort` | Pose le `WindowLevel` d'une fenêtre de Breeze, dont le niveau maximal | Natif ↔ double qui enregistre le niveau demandé | 3 |
| `PresentationOptionsPort` | Masque le Dock et la barre de menus, désactive le changement d'application, restaure | Natif ↔ double qui enregistre les options posées et vérifie leur restauration | 7 |
| `DisplaysPort` | La liste des `Display`, l'abonnement à l'ajout, au retrait et à la reconfiguration | Natif ↔ double qui simule un branchement et un débranchement à chaud | 7 |
| `ScreenSharingPort` | Le niveau de partage d'écran et de présentation plein écran, s'il existe sans permission d'enregistrement | Natif ↔ double qui pose le niveau sur commande | 7 |
| `GlobalShortcutsPort` | Enregistre un `Shortcut`, le désenregistre, énumère ceux qu'il détient, et purge ce qu'un processus mort a laissé | Natif ↔ double qui tient un registre inspectable et simule une sortie anormale | 9 |
| `ClockPort` | L'`Instant` monotone et l'`Instant` mural | Horloge système ↔ double pilotable, sans lequel aucun test d'échéance n'est déterministe | 2, **sans natif** |

`ClockPort` est le seul dont l'adaptateur n'est pas natif : macOS n'a rien à traduire ici, la
bibliothèque standard suffit. Il est livré en phase 2, avec les huit autres contrats.

`GlobalShortcutsPort` porte ce que le cadrage §7 exige le plus fermement : le désenregistrement
garanti par deux chemins indépendants, dont un qui survit à une sortie anormale. Sans port, cette
règle ne serait testable que sur un Mac, à la main, et c'est exactement la règle qu'on ne peut pas se
permettre de vérifier à la main — un raccourci global oublié pénalise tout le système.

**Aucun port de journalisation, aucun port de persistance, aucun port réseau dans ce plan.** Le
journal circulaire du cadrage §11 appartient à la phase 1, la persistance de l'état du cycle aussi,
et la seule sortie réseau du produit — la vérification de mise à jour — à la phase 5. Un port sans
rien à remplacer ne va pas dans le plan, et un port dont aucune phase de ce plan n'a besoin non plus.

---

## 3. Les phases

Dix phases, une pull request chacune. Le §4 donne leurs dépendances, ce qui part en parallèle, et la
surface que trois d'entre elles se partagent.

Trois règles valent partout et ne sont pas redites phase par phase :

- **Chaque phase se termine par un commit unique, les quatre portes vertes** — `pnpm run typecheck`,
  `pnpm run lint`, `pnpm run test`, `pnpm run build`.
- **La liste de tests d'une phase s'écrit avant son premier test**, jamais après son dernier.
- **Les sondes tournent sous Electron, pas sous Node.** Elles posent des fenêtres, des niveaux
  d'empilement et des options de présentation, qui n'existent que dans le processus principal
  d'Electron. Toute sonde se lance par `pnpm exec electron build/probe/<sonde>.mjs`, et la phase 1
  déclare le script `probe` qui porte ce préfixe.

Une réserve de méthode, dite une fois : les phases 1, 4, 5 et 10 ne commencent pas par un domaine,
parce qu'elles n'en ont pas — une chaîne de construction, une soumission et un relevé ne portent
aucun invariant métier. Les phases 2, 3, 6, 7, 8 et 9 respectent l'ordre d'écriture : le port et son
double d'abord, l'adaptateur natif ensuite, le harnais ou le prototype en dernier. Aucune ne commence
par une fenêtre Electron ni par un fichier d'emballage.

---

### Phase 1 — Les quatre portes et l'arborescence

**Statut : Fait**

Le dépôt n'a qu'une porte déclarée, `pnpm run test`, et elle est `exit 1` par construction. Tant
qu'elle l'est, aucune phase ne peut prétendre laisser les portes vertes. C'est donc la première, et
elle ne dépend de rien — ni d'un compte Apple, ni d'un Mac.

**Ce qu'elle livre**

- `package.json` : les scripts — `typecheck` (`tsc --checkJs --noEmit`), `lint` (`eslint .`),
  `test` (`node --test`), `build` (`electron-builder --dir`), et `probe`
  (`electron`), qui donne aux sondes des phases suivantes le seul binaire sous lequel elles peuvent
  tourner.
- `jsconfig.json` : `checkJs`, `strict`, cible ES2022, modules ESM.
- `eslint.config.js` : configuration à plat, règles de base.
- `src/modules/system-bridge/index.js` : le point d'entrée du contexte, vide de logique.
- `build/` : le dossier de la chaîne de construction, avec son fichier de configuration
  `electron-builder`.
- `main.js` : conservé à la racine comme racine de composition, et réduit à ce qu'il est — aucune
  fenêtre créée au démarrage, puisque `BRIEF.md` §13 interdit tout overlay posé au lancement.
- `test/modules/system-bridge/index.test.js` : le test qui prouve que le lanceur tourne.

**Critères de succès**

```bash
pnpm run typecheck && pnpm run lint && pnpm run test && pnpm run build && echo "portes=vertes"
```

La chaîne est en `&&` de bout en bout : une porte rouge interrompt et rend un code non nul. Un `;`
avant l'`echo` rendrait le critère toujours vert, et ne prouverait rien.

```bash
test -d "dist/mac-universal/Breeze.app" || test -d "dist/mac/Breeze.app"
```

**Liste de tests** — écrite avant le premier test de la phase.

- `node --test` échoue quand un test du dépôt échoue, et ne rend pas 0 sur une suite vide.
- `tsc --checkJs` refuse un appel à une fonction inexistante dans un fichier annoté en JSDoc.
- `eslint` refuse une variable inutilisée.
- Le paquet construit ne contient pas `node_modules` de développement.

**Ce que cette phase ne fait pas** : aucune signature, aucun module natif, aucune fenêtre.

---

### Phase 2 — Les ports et leurs doubles

**Statut : À faire**

JavaScript pur, aucun macOS requis. C'est le contrat que toutes les phases natives implémenteront, et
il se fusionne avant elles : un adaptateur écrit contre un contrat non publié se réécrit.

**Ce qu'elle livre**

- `src/modules/system-bridge/application/model/` : `app-identity.js`, `window-frame.js`,
  `display.js`, `accessibility-status.js`, `window-level.js`, `shortcut.js`, `instant.js` —
  objets-valeur gelés, valides par construction.
- `src/modules/system-bridge/application/ports/` : les neuf ports du §2, en classes de contrat dont
  chaque méthode lève tant qu'elle n'est pas implémentée.
- `src/modules/system-bridge/infrastructure/doubles/` : un double écrit à la main par port, pilotable
  depuis un test.
- `src/modules/system-bridge/infrastructure/system-clock.js` : le seul adaptateur réel de cette
  phase — l'horloge du système, sous ses deux formes. Il n'a rien de natif et n'attend aucune des
  phases suivantes.
- `test/modules/system-bridge/contract/` : le **harnais de contrat** — une suite par port, écrite une
  fois, que toute implémentation doit passer. C'est lui qui consomme les ports, et c'est ce qui rend
  cette phase autonome plutôt que spéculative.
- `test/modules/system-bridge/application/model/` : les tests des objets-valeur, en miroir.

**Critères de succès**

```bash
node --test test/modules/system-bridge/contract/ && echo "harnais=vert"
```

Le harnais de contrat tourne en entier contre les doubles, sans qu'aucun Mac ni aucune permission ne
soit en jeu. Le code de sortie de `node --test` est celui qui compte : passer sa sortie dans un
`grep` la perdrait, et compterait des lignes au lieu de trancher.

```bash
pnpm run test && pnpm run typecheck && pnpm run lint
```

**Liste de tests**

Les objets-valeur, refus d'abord :

- `AppIdentity` refuse un identifiant de paquet vide.
- `AppIdentity` refuse tout champ qui ressemble à un titre de fenêtre.
- `WindowFrame` refuse une largeur ou une hauteur négative.
- `WindowFrame` refuse une taille nulle.
- `Display` refuse deux écrans portant le même identifiant dans une même liste.
- `AccessibilityStatus` refuse une valeur hors des trois qu'il connaît.
- `WindowLevel` refuse un niveau au-delà du maximal que le port déclare.
- `Shortcut` refuse une combinaison vide.
- `Instant` refuse la comparaison d'un instant monotone avec un instant mural.
- `Instant` rend une durée comme différence de deux instants, jamais comme un décrément.
- Deux `AppIdentity` de mêmes champs sont égales ; deux `WindowFrame` de mêmes champs le sont aussi.
- Un objet-valeur construit est gelé : toute écriture sur lui échoue.

Le harnais de contrat, par port :

- `ForegroundAppPort` rend l'application au premier plan sans jamais rendre son titre de fenêtre.
- `ForegroundAppPort` notifie un changement de premier plan, et ne notifie rien quand l'application
  ne change pas.
- `ForegroundAppPort` se désabonne, et ne notifie plus après désabonnement.
- `WindowFramesPort` rend une liste vide pour un processus sans fenêtre, sans lever.
- `WindowFramesPort` notifie un déplacement et un redimensionnement séparément.
- `WindowFramesPort` bascule sur l'interrogation de secours après le silence que la conception
  chiffrera (§6), et cesse après le retour de notification que la même décision chiffrera.
- `AccessibilityPermissionPort` rend l'état courant à chaque appel, et non un état mémorisé — la
  révocation est silencieuse.
- `AccessibilityPermissionPort` refuse de demander l'octroi une seconde fois quand il est déjà
  accordé.
- `WindowLevelPort` refuse de poser un niveau sur une fenêtre qui n'existe plus.
- `PresentationOptionsPort` restaure les options posées, et une restauration sans pose ne fait rien.
- `DisplaysPort` rend au moins un écran, et notifie un branchement comme un débranchement.
- `ScreenSharingPort` rend un niveau, jamais un front.
- `GlobalShortcutsPort` refuse d'enregistrer deux fois le même `Shortcut`.
- `GlobalShortcutsPort` rend un registre vide après un désenregistrement de tout ce qu'il détient.
- `GlobalShortcutsPort` purge, au démarrage, ce qu'une sortie anormale simulée a laissé — c'est le
  second chemin indépendant que le cadrage §7 exige.
- `ClockPort` rend deux instants monotones croissants, et un instant mural que le test pilote.

**Ce que cette phase ne fait pas** : aucun appel système, aucun code natif, aucune fenêtre.

---

### Phase 3 — Le module natif compilable, et ses deux seules entrées

**Statut : À faire**

Cette phase existe pour une raison unique : **produire au plus tôt un binaire portant les deux signaux
que la notarisation juge** — un niveau d'empilement de fenêtre, et l'usage de l'Accessibilité. C'est
l'énoncé qui commande tout l'ordre de ce plan, et le §4 ne le redit pas.

Elle ne dépend d'aucun certificat : le code se compile et se charge non signé.

**Ce qu'elle livre**

- `native/binding.gyp` : la cible de compilation, universelle pour les deux architectures.
- `native/src/breeze_bridge.mm` : le point d'entrée N-API, qui déclare les fonctions exposées.
- `native/src/accessibility_permission.mm` : état, demande, revérification.
- `native/src/window_level.mm` : pose du niveau d'empilement.
- `package.json` : le script `build:native` (`node-gyp rebuild` sous les en-têtes d'Electron, par
  `electron-rebuild`), et `build` qui l'enchaîne avant `electron-builder`. Sans ce script, aucun
  `.node` n'existe et les critères des phases suivantes ne s'exécutent pas.
- `src/modules/system-bridge/infrastructure/native/accessibility-permission.native.js` et
  `window-level.native.js` : les adaptateurs minces, zéro logique.
- `test/modules/system-bridge/infrastructure/native/` : le harnais de contrat de la phase 2, rebranché
  sur ces deux implémentations réelles.

**Critères de succès**

```bash
pnpm run build:native && node -e "require('./native/build/Release/breeze_bridge.node')" && echo "chargé"
```

```bash
lipo -archs native/build/Release/breeze_bridge.node
```

Rend les deux architectures.

```bash
pnpm exec electron --test test/modules/system-bridge/infrastructure/native/ && echo "natif=vert"
```

Le harnais de contrat de `AccessibilityPermissionPort` et `WindowLevelPort` passe contre le natif, sur
un Mac réel.

**Liste de tests**

- Le module se charge sans permission d'Accessibilité accordée, et ne lève pas.
- `AccessibilityPermissionPort` rend « jamais demandée » sur une identité qui n'a jamais demandé.
- `AccessibilityPermissionPort` rend l'état réel après une révocation depuis les réglages du système,
  sans redémarrage du processus.
- `AccessibilityPermissionPort` rend encore « jamais demandée » après un appel de lecture — lire
  l'état ne le change pas, et c'est ce qui prouve qu'aucune boîte de dialogue n'a été déclenchée.
- `WindowLevelPort` refuse un identifiant de fenêtre inconnu.
- `WindowLevelPort` pose le niveau maximal et le relit à l'identique.

**Ce que cette phase ne fait pas** : aucune signature, aucune des sept autres capacités, aucun
overlay.

---

### Phase 4 — Le paquet signé

**Statut : À faire**

Le cadrage §8.2 fixe l'ordre et ne le laisse pas au choix : la chaîne signe **après** l'inclusion du
module natif, élément par élément puis le paquet entier. C'est pourquoi cette phase suit la 3 au lieu
de l'ouvrir — commencer par l'emballage signerait un paquet qui n'a pas sa forme finale, et le
travail serait à refaire.

**Bloquée par une dépendance humaine** : l'adhésion au programme développeur Apple et le certificat.
L'adhésion est à engager le jour où la phase 1 s'ouvre — la vérification d'identité prend plusieurs
jours, et les phases 1, 2 et 3 avancent sans elle.

**Ce qu'elle livre**

- `build/entitlements.mac.plist` : les droits demandés, et eux seuls — aucun droit pour une
  fonctionnalité qui ne s'en sert pas, `BRIEF.md` §13.
- `build/electron-builder.yml` : runtime durci, signature élément par élément puis paquet entier,
  construction universelle.
- `build/verify-signature.sh` : le script qui vérifie ce que la chaîne vient de produire.
- La documentation du piège du cadrage §8.2 : en développement, l'entrée de permission porte le nom
  de l'environnement d'exécution et non celui de Breeze.

**Critères de succès**

```bash
pnpm run build && bash build/verify-signature.sh && echo "signature=valide"
```

Le script enchaîne, et chaque commande rend 0 :

```bash
codesign --verify --deep --strict --verbose=2 "dist/mac-universal/Breeze.app"
```

```bash
codesign -d --entitlements :- "dist/mac-universal/Breeze.app"
```

```bash
spctl -a -vvv -t exec "dist/mac-universal/Breeze.app"
```

**Liste de tests** — assertions du script de vérification, refus d'abord.

- La vérification échoue quand le module natif n'est pas signé individuellement.
- La vérification échoue quand le runtime durci est absent.
- La vérification échoue quand les entitlements portent un droit d'enregistrement d'écran, de micro,
  de caméra ou de pilotage d'autres applications.
- La vérification échoue quand le paquet a été modifié après signature.
- La vérification échoue quand le paquet n'est pas universel.
- Une exécution du `.app` signé, permission d'Accessibilité accordée une fois, garde l'autorisation
  après une reconstruction et une réinstallation — c'est la preuve que l'identité signée est stable.

**Ce que cette phase ne fait pas** : aucune soumission, aucune capacité native nouvelle.

---

### Phase 5 — La soumission à la notarisation

**Statut : À faire**

C'est la mesure 7 de `BRIEF.md` §12.3 : **bloquante, sans repli**. Elle arrive en cinquième position
et non en dernière, pour la raison donnée en phase 3 : le binaire de la phase 4 porte déjà les deux
seuls signaux qu'Apple juge.

**Trois verdicts, et non deux.** Le plan les sépare parce que le premier est réparable et que les
confondre ferait arrêter le produit sur une erreur de chaîne de construction :

| Verdict | Ce qu'il est | Ce qu'on en fait |
|---|---|---|
| `Accepted` | Le paquet est notarié | Agrafage, puis la 7ᵉ ligne du relevé |
| `Invalid` **de chaîne** | Le motif d'Apple nomme un défaut de construction — signature absente, runtime durci manquant, architecture incomplète | Corrigé en phase 4, resoumis. Le nombre de resoumissions est borné (§6) |
| `Invalid` **de politique** | Le motif d'Apple nomme les niveaux de fenêtre ou l'usage de l'Accessibilité | **C'est le refus sans repli.** Le motif est rendu tel quel, la décision revient au fondateur (§6) |
| Aucun verdict | Le service ne répond pas, ou reste `In Progress` au-delà de l'attente maximale (§6) | Code de sortie distinct des deux précédents. Ce n'est ni une acceptation ni un refus, et rien ne s'en déduit |

**Ce qu'elle livre**

- `build/notarize.sh` : soumission, attente bornée, classement du verdict dans l'une des quatre
  lignes ci-dessus, agrafage sur `Accepted`.
- Le verdict, versé au relevé comme sa septième ligne — une ligne, pas quatre cellules : le verdict
  porte sur un binaire, non sur une version d'exécution.

**Critères de succès**

```bash
bash build/notarize.sh "dist/mac-universal/Breeze.app"; echo "verdict=$?"
```

C'est le seul critère de ce plan qui lit un code de sortie plutôt que d'exiger 0 : le script rend un
code par ligne du tableau, et c'est ce code qui est le verdict.

Sur `Accepted` seulement :

```bash
xcrun stapler validate "dist/mac-universal/Breeze.app" && echo "agrafé"
```

```bash
spctl -a -vvv -t exec "dist/mac-universal/Breeze.app" 2>&1 | grep -q "source=Notarized Developer ID" && echo "notarié"
```

Et la preuve qui compte vraiment, sur une machine vierge :

```bash
xattr -w com.apple.quarantine "0081;00000000;Safari;" "/Applications/Breeze.app" && open "/Applications/Breeze.app"
```

L'application s'ouvre sans avertissement de logiciel non identifié.

**Liste de tests**

- La soumission échoue quand le paquet n'est pas signé avec une identité Developer ID, et le script
  classe l'échec en `Invalid` de chaîne — pas en refus.
- La soumission échoue quand le runtime durci est absent, et le script la classe de même.
- Un motif nommant un niveau de fenêtre ou l'usage de l'Accessibilité est classé en `Invalid` de
  politique, jamais en défaut de chaîne.
- Le script rend son code « aucun verdict » quand l'attente maximale est atteinte, et n'écrit aucune
  ligne au relevé.
- L'agrafage échoue sur un paquet dont le verdict n'est pas `Accepted`.
- Le paquet agrafé s'ouvre hors ligne — l'agrafage sert exactement à cela.

**Ce que cette phase ne fait pas** : aucune publication, aucun dépôt de mise à jour — la phase 5 du
cadrage §9 s'en charge.

---

### Phase 6 — Le natif sans permission : l'application au premier plan

**Statut : À faire**

Première des trois phases natives, et la ligne de coupe n'est pas le nombre de fonctions : c'est **la
permission**. Ce qui se lit sans Accessibilité vit ici et en phase 7 ; ce qui l'exige vit en phase 8.
Mêler les deux rendrait la mesure 4 inobservable — on ne saurait plus quel appel a déclenché la
demande.

**Ce qu'elle livre**

- `native/src/foreground_app.mm` : `NSWorkspace`, l'application au premier plan, les applications en
  cours, l'observateur de changement.
- Les deux lignes correspondantes dans `native/binding.gyp` et `native/src/breeze_bridge.mm` — la
  surface que cette phase partage avec les phases 7 et 8 (§4).
- `src/modules/system-bridge/infrastructure/native/foreground-app.native.js`.
- Le harnais de contrat de `ForegroundAppPort`, rebranché sur le natif.
- `build/probe/foreground-app.mjs` et la cellule de la mesure 4, versée au relevé.

**Critères de succès**

```bash
pnpm exec electron --test test/modules/system-bridge/infrastructure/native/foreground-app.test.js && echo "vert"
```

La mesure 4, permission d'Accessibilité **refusée** :

```bash
pnpm exec electron build/probe/foreground-app.mjs --require-accessibility-denied --emit-cell
```

Rend l'identité de l'application au premier plan, ou dit explicitement que la version couverte exige
l'Accessibilité — et écrit sa cellule. Un code non nul quand la permission n'est pas dans l'état que
la mesure exige : une mesure prise dans le mauvais état n'est pas une mesure.

**Liste de tests**

- Le port ne rend jamais un titre de fenêtre, sur aucun chemin.
- Le port rend l'identité au premier plan alors que la permission d'Accessibilité est refusée — ou
  déclare que la version couverte l'exige, ce qui est le fait que la mesure 4 cherche.
- L'observateur ne notifie pas quand l'application au premier plan ne change pas.
- L'observateur se désabonne, et le désabonnement survit à une sortie anormale du processus.
- La liste des applications en cours ne contient pas les processus sans interface.
- `AccessibilityStatus` est inchangé après un aller-retour complet de cette phase — la preuve
  qu'aucun appel n'a déclenché de demande de permission.

**Ce que cette phase ne fait pas** : aucun cadre de fenêtre, aucun écran, aucun overlay.

---

### Phase 7 — Le natif sans permission : écrans, présentation, partage

**Statut : À faire**

**Ce qu'elle livre**

- `native/src/displays.mm` : énumération des écrans, observateurs d'ajout, de retrait et de
  reconfiguration.
- `native/src/presentation_options.mm` : masquage du Dock et de la barre de menus, désactivation du
  changement d'application, restauration.
- `native/src/screen_sharing.mm` : le signal de partage d'écran et celui de présentation plein écran,
  s'ils existent sans permission d'enregistrement.
- Les lignes correspondantes dans `native/binding.gyp` et `native/src/breeze_bridge.mm` (§4).
- Les trois adaptateurs sous `src/modules/system-bridge/infrastructure/native/`.
- Le harnais de contrat de `DisplaysPort`, `PresentationOptionsPort` et `ScreenSharingPort`, rebranché.
- `build/probe/screen-sharing.mjs`, `build/probe/menu-bar-hiding.mjs`, et les cellules des mesures 2,
  3 et 6.

**Critères de succès**

```bash
pnpm exec electron --test test/modules/system-bridge/infrastructure/native/displays.test.js test/modules/system-bridge/infrastructure/native/presentation-options.test.js test/modules/system-bridge/infrastructure/native/screen-sharing.test.js && echo "vert"
```

La mesure 2 :

```bash
pnpm exec electron build/probe/screen-sharing.mjs --require-no-screen-recording --emit-cell
```

Rend le signal, ou dit qu'aucun signal n'existe sans la permission d'enregistrement — auquel cas le
repli de `BRIEF.md` §12.3 s'applique et l'exception est retirée de l'interface.

Les mesures 3 et 6 sont **manuelles** : elles exigent une application tierce déjà en plein écran
natif, qu'aucun script ne peut poser sans piloter cette application — ce que le cadrage §5 interdit.
Leur mode opératoire est écrit dans `build/probe/README.md` et rappelé ici : ouvrir l'application
témoin nommée au §6, la passer en plein écran natif, puis lancer la sonde depuis un autre espace de
travail.

```bash
pnpm exec electron build/probe/screen-sharing.mjs --manual-fullscreen-witness --emit-cell
```

```bash
pnpm exec electron build/probe/menu-bar-hiding.mjs --manual-fullscreen-witness --emit-cell
```

Chacune rend un code non nul, et n'écrit aucune cellule, si aucune application tierce en plein écran
n'est détectée : une mesure prise sans son témoin n'est pas une mesure.

**Liste de tests**

- `ScreenSharingPort` ne déclenche jamais la demande de permission d'enregistrement d'écran, et
  `AccessibilityStatus` est inchangé après un aller-retour complet de la phase.
- `ScreenSharingPort` distingue une présentation plein écran d'un simple plein écran, **ou déclare
  qu'il ne le peut pas** — c'est la mesure 3, et son absence retire l'exception.
- `PresentationOptionsPort` restaure le Dock et la barre de menus après une sortie normale.
- `PresentationOptionsPort` restaure le Dock et la barre de menus après une sortie **anormale** — un
  Dock resté masqué pénalise la machine jusqu'au redémarrage.
- `DisplaysPort` notifie un débranchement d'écran, et le délai de cette notification est **relevé,
  non opposé** : les 500 ms de `BRIEF.md` §12.2 bornent la pose d'un overlay sur un écran *apparu*,
  et rien n'y borne la notification d'un retrait. Le seuil du retrait est au §6.
- `DisplaysPort` rend un écran unique sur une machine à un seul écran, sans lever.
- `DisplaysPort` identifie l'écran qui porte la barre de menus.
- Une sonde manuelle lancée sans témoin en plein écran rend un code non nul et n'écrit rien.

**Ce que cette phase ne fait pas** : aucun cadre de fenêtre tierce, aucun overlay posé.

---

### Phase 8 — Le natif sous Accessibilité : les cadres de fenêtres

**Statut : À faire**

La seule phase native qui exige la permission. C'est ce qui la sépare des deux précédentes, et c'est
ce qui rend la mesure 4 lisible.

**Ce qu'elle livre**

- `native/src/window_frames.mm` : cadres de fenêtres d'un processus donné, observateurs de
  déplacement et de redimensionnement, interrogation de secours à la cadence du cadrage §8.5 — 4 Hz,
  valeur de départ non arrêtée, que le §6 verse aux décisions ouvertes.
- Les lignes correspondantes dans `native/binding.gyp` et `native/src/breeze_bridge.mm` (§4).
- `src/modules/system-bridge/infrastructure/native/window-frames.native.js`.
- Le harnais de contrat de `WindowFramesPort`, rebranché.
- `build/probe/accessibility-hot-grant.mjs` et la cellule de la mesure 1.

**Critères de succès**

```bash
pnpm exec electron --test test/modules/system-bridge/infrastructure/native/window-frames.test.js && echo "vert"
```

La mesure 1 :

```bash
pnpm exec electron build/probe/accessibility-hot-grant.mjs --emit-cell
```

Dit si l'octroi se propage sans relancer l'application, par version de macOS. Le repli de
`BRIEF.md` §12.3 — une relance explicite proposée à l'onboarding — s'applique sur les versions où il
ne se propage pas.

**Liste de tests**

- Le port lève une erreur nommée, et non une erreur générique, quand la permission est refusée.
- Le port ne lit jamais le titre d'une fenêtre, ni son contenu — `BRIEF.md` §13.
- Le port rend une liste vide pour un processus sans fenêtre, sans lever.
- L'observateur notifie un déplacement et un redimensionnement comme deux faits distincts.
- L'interrogation de secours se déclenche après le seuil de silence chiffré au §6, et **cesse** après
  le retour de notification chiffré au même endroit — sinon elle coûte sa cadence en permanence,
  contre les cibles de `BRIEF.md` §12.2.
- Le port rend l'état réel après une révocation silencieuse de la permission, sans redémarrage.

**Ce que cette phase ne fait pas** : aucun overlay posé sur un cadre — la phase 4 du cadrage §9 s'en
charge, une fois les mesures rendues.

---

### Phase 9 — Le prototype d'overlay inévitable, trente secondes

**Statut : À faire**

C'est la deuxième des trois sorties que le cadrage §9 exige de la Reconnaissance, et elle porte la
mesure 5 : le comportement de l'overlay au niveau maximal face à une application tierce en plein
écran natif — la seule mesure qui touche à la promesse du Mode Hardcore, et celle dont la
documentation d'Apple déconseille explicitement la technique.

**Trois décisions que cette phase prend, et que la conception transcrira :**

1. **Les trente secondes courent sur l'`Instant` monotone**, et rien ne les gèle. Ni la veille, ni le
   verrouillage : le prototype n'est ni une phase de travail ni une pause, et `BRIEF.md` §13 ne
   tranche que pour celles-là. Un verrouillage de plus de trente secondes détruit donc l'overlay
   pendant le verrouillage, et c'est le comportement attendu.
2. **La pose multi-écran est tout ou rien.** Le cadrage §7 le commande : « Un écran non couvert est
   une porte de sortie complète. » Si la pose échoue sur un écran parmi N, le prototype démonte ce
   qu'il a posé, écrit l'échec au compte rendu, et ne tient pas trente secondes une couverture
   partielle.
3. **Une seule instance.** Une seconde instance lancée pendant qu'une première tourne ne pose aucun
   overlay, rend un code non nul et une erreur nommée. `BRIEF.md` §13 l'interdit et le cadrage §8.3
   le qualifie de pire défaut possible du produit.

**Ce qu'elle livre**

- `native/src/global_shortcuts.mm` et
  `src/modules/system-bridge/infrastructure/native/global-shortcuts.native.js` : l'implémentation de
  `GlobalShortcutsPort` — enregistrement, désenregistrement, et la purge au démarrage qui est le
  second chemin indépendant du cadrage §7.
- Les lignes correspondantes dans `native/binding.gyp` et `native/src/breeze_bridge.mm` (§4).
- Le verrou d'instance unique, dans `main.js`.
- `build/probe/hardcore-overlay.mjs` : le prototype — une fenêtre sans cadre, non déplaçable, non
  fermable, par écran physique, aux dimensions exactes de son écran, au niveau maximal, visible sur
  tous les espaces de travail, options de présentation posées, détruite au bout de trente secondes.
- `build/probe/assert-clean-system.mjs` : la sonde d'après-crash — aucun raccourci global résiduel,
  Dock et barre de menus revenus.
- Le harnais de contrat de `GlobalShortcutsPort`, rebranché sur le natif.
- La cellule de la mesure 5.

**Critères de succès**

```bash
pnpm exec electron build/probe/hardcore-overlay.mjs --seconds 30 --report && echo "posé"
```

Rend un compte rendu qui dit, par écran : l'overlay a été posé, à quel niveau, en combien de
millisecondes, et s'il a été recouvert pendant les trente secondes. **Le délai de pose est relevé, et
non opposé** : les 200 ms de `BRIEF.md` §12.2 sont la cible de l'overlay Hardcore du produit, que la
phase 2 du cadrage §9 livrera ; les opposer à un prototype ferait échouer la Reconnaissance sur une
cible qui n'est pas la sienne. Le chiffre relevé entre au relevé et sert à savoir si la cible est
atteignable.

La mesure 5, **manuelle** — elle exige le même témoin en plein écran natif que les mesures 3 et 6 :

```bash
pnpm exec electron build/probe/hardcore-overlay.mjs --seconds 30 --manual-fullscreen-witness --emit-cell
```

Rend `couvert` ou `non couvert`, et un code non nul si aucun témoin en plein écran n'est détecté.

L'après-crash :

```bash
pnpm exec electron build/probe/hardcore-overlay.mjs --crash-after 5; pnpm exec electron build/probe/assert-clean-system.mjs && echo "système=propre"
```

Le `;` est ici volontaire, comme celui de la phase 5 : la première commande **doit** rendre un code non
nul, puisqu'elle simule une sortie anormale. C'est la seconde qui tranche.

**Liste de tests**

- Aucun overlay n'est posé au démarrage du prototype — il l'est à l'échéance, jamais avant.
- Un écran branché pendant les trente secondes reçoit son overlay en moins de 500 ms, seuil de
  `BRIEF.md` §12.2.
- Un écran débranché pendant les trente secondes ne laisse pas d'overlay orphelin.
- Une pose qui échoue sur un écran parmi N démonte les overlays déjà posés et écrit l'échec.
- Un écran retiré entre la pose sur l'écran 1 et celle sur l'écran 2 produit le même démontage.
- Aucun overlay ne survit à la fin des trente secondes — `BRIEF.md` §13.
- Un verrouillage de session de moins de trente secondes laisse l'overlay revenir au premier plan au
  déverrouillage ; un verrouillage plus long le trouve détruit, échéance monotone écoulée.
- Une seconde instance lancée pendant la première ne pose aucun overlay et rend une erreur nommée.
- Aucun raccourci global n'est enregistré avant la pose de l'overlay.
- Aucun raccourci global ne survit à la fin des trente secondes.
- Aucun raccourci global ne survit à une sortie anormale du processus — le second chemin de
  désenregistrement, celui qui compte.
- Aucun raccourci enregistré n'intercepte une combinaison de mise hors tension ou de verrouillage —
  `BRIEF.md` §13 exige qu'éteindre le Mac reste possible, et c'est cette assertion qui le prouve sans
  détruire la session qui l'exécute.
- Le Dock et la barre de menus sont revenus après une sortie anormale du prototype.
- Le prototype ne masque, ne quitte ni ne suspend aucune application tierce.
- Le prototype ne charge aucune ressource distante — cadrage §11.

**Ce que cette phase ne fait pas** : aucun décompte de cycle, aucune sévérité, aucune sortie
d'urgence, aucun voile de Mode Simple. Le prototype prouve la faisabilité, il n'esquisse pas le
produit.

---

### Phase 10 — Le pilote de relevé et le relevé

**Statut : À faire**

La troisième sortie du cadrage §9. **Un document seul n'est pas une pull request** : ce qui se livre
ici est le **pilote** — le script qui exécute les mesures automatisables sur la machine courante et
consolide les cellules —, et le relevé en est le sous-produit.

**Où vivent les cellules, et ce qui les identifie.** Le cadrage §8.1 énumère sept mesures sans
structure par version : il ne peut pas accueillir la matrice. Les cellules vivent donc sous
`build/probe/releve/<version-macos>.json`, une par mesure, écrites au fil de l'eau par les phases 6
à 9. Le relevé consolidé est versé au cadrage §8.1 en fin de phase, comme un tableau nouveau.

- **La clé d'une cellule est le couple `(mesure, version de macOS)`**, et rien d'autre.
- Chaque cellule porte, outre son verdict : la version de macOS relevée, l'état de la permission
  d'Accessibilité au moment de la mesure, et si elle a été prise à la main ou par le pilote.
- **Une cellule déjà écrite n'est jamais écrasée en silence.** Un second relevé concordant est
  ignoré ; un second relevé **divergent** est refusé, et le pilote rend un code non nul en nommant la
  cellule et les deux verdicts. Deux verdicts opposés sur un même couple sont un fait à comprendre,
  pas une valeur à choisir.

**Ce qui est automatisable, et ce qui ne l'est pas.** Trois mesures — 3, 5 et 6 — exigent une
application tierce déjà en plein écran natif. Aucun script ne peut la poser : piloter une autre
application demande une permission que `BRIEF.md` §13 refuse. Elles sont **manuelles**, prises en
phases 7 et 9, et le pilote ne les exécute pas — il les compte comme attendues et les réclame si
elles manquent.

**Les versions couvertes sont 13, 14, 15 et 26** — `BRIEF.md` §12.1 : la numérotation saute de 15 à
26, et il n'existe aucune version 16 à 25. Six mesures non bloquantes sur quatre versions font
vingt-quatre cellules, dont douze manuelles. La septième mesure, la notarisation, est une ligne
unique rendue en phase 5.

**Ce qu'elle livre**

- `build/probe/run-all.mjs` : le pilote, qui exécute les trois mesures automatisables sur la machine
  courante, écrit leurs cellules, et applique la règle de non-écrasement.
- `build/probe/consolidate.mjs` : la consolidation des vingt-quatre cellules et de la ligne de
  notarisation en un relevé unique.
- Le relevé versé au cadrage §8.1, suivi de
  `bash "…/stamp.sh" toucher .charpente/cadrage/2026-08-28-poser-une-contrainte-inevitable-sur-macos.md`.
- Pour chaque mesure tranchée par la négative, **le repli de `BRIEF.md` §12.3 appliqué et nommé** —
  jamais une valeur inventée.

**Critères de succès**

```bash
pnpm exec electron build/probe/run-all.mjs && echo "cellules=écrites"
```

Rend les trois cellules automatisables de la machine courante, chacune tranchée par un fait, avec sa
version de macOS et l'état de permission relevés.

```bash
node build/probe/consolidate.mjs --assert-complete && echo "relevé=complet"
```

Rend un code non nul tant qu'une cellule des vingt-quatre, ou la ligne de notarisation, manque — en
la nommant.

```bash
node build/probe/consolidate.mjs --assert-consistent && echo "relevé=cohérent"
```

Rend un code non nul dès que deux verdicts divergent sur un même couple `(mesure, version)`.

**Liste de tests**

- Le pilote refuse d'écrire une cellule sans version de macOS relevée.
- Le pilote refuse d'écrire une cellule dont la mesure a levé — une erreur n'est pas un verdict.
- Le pilote refuse d'écrire une cellule prise dans un état de permission qui n'est pas celui que la
  mesure exige.
- Le pilote ignore un second relevé concordant, et refuse un second relevé divergent en nommant les
  deux verdicts.
- Le pilote n'exécute aucune des trois mesures manuelles, et les compte comme attendues.
- `--assert-complete` échoue sur un relevé partiel, et nomme les cellules manquantes.
- `--assert-consistent` échoue sur une cellule divergente, et passe sur un doublon concordant.
- Une mesure tranchée par la négative écrit son repli, jamais une valeur plausible.

**Ce que cette phase ne fait pas** : aucune décision produit. Le relevé constate ; le fondateur et le
brief décident.

---

## 4. Le découpage en pull requests

Dix pull requests. Une phase, une pull request, un commit.

| # | Pull request | Ce qu'elle livre | Dépend de | Écrite en parallèle de |
|---|---|---|---|---|
| 1 | `chore(build): les quatre portes et l'arborescence` | Portes, arborescence des contextes, racine de composition réduite | — | — |
| 2 | `feat(system-bridge): les ports et leurs doubles` | Neuf ports, objets-valeur, doubles, harnais de contrat | 1 | 3 |
| 3 | `feat(system-bridge): le module natif et ses deux entrées` | `binding.gyp`, N-API universel, `build:native`, permission d'Accessibilité, niveau de fenêtre | 1, 2 | — |
| 4 | `chore(distribution): le paquet signé` | Entitlements, runtime durci, signature en deux temps | 3 | — |
| 5 | `chore(distribution): la soumission à la notarisation` | Soumission, classement du verdict, agrafage | 4 | 6, 7, 8, 9 |
| 6 | `feat(system-bridge): l'application au premier plan` | `NSWorkspace`, mesure 4 | 2, 3 | 5, 7, 8 |
| 7 | `feat(system-bridge): écrans, présentation, partage` | Écrans, options de présentation, partage — mesures 2, 3, 6 | 2, 3 | 5, 6, 8 |
| 8 | `feat(system-bridge): les cadres de fenêtres` | API d'Accessibilité, mesure 1 | 2, 3 | 5, 6, 7, 9 |
| 9 | `feat(system-bridge): le prototype d'overlay inévitable` | Overlay trente secondes, raccourcis globaux, instance unique, mesure 5 | 2, 7 | 5, 8 |
| 10 | `chore(distribution): le pilote de relevé` | Pilote, consolidation, relevé | 5, 6, 7, 8, 9 | — |

La colonne « Écrite en parallèle de » porte sur **l'écriture**, jamais sur la fusion : les pull
requests se fusionnent dans l'ordre de leurs dépendances, une par une, sur un historique linéaire.

### Le chemin critique

**1 → 2 → 3 → 4 → 5.** C'est le chemin vers le verdict bloquant, et il est le plus court possible,
pour la raison donnée en phase 3.

Placer la soumission après les pull requests natives et le prototype, comme la lecture chronologique
du cadrage §9 y invite, coûterait quatre pull requests d'implémentation en cas de refus de politique
— soit l'essentiel de la charge de la Reconnaissance. Le cadrage §9 justifie exactement cette
compression : « Un refus de notarisation arrête le produit. »

### La surface que les pull requests 6, 7, 8 et 9 se partagent

Leurs fichiers Objective-C++ sont disjoints, leurs ports aussi. **Deux fichiers ne le sont pas** :
`native/binding.gyp`, qui liste les sources à compiler, et `native/src/breeze_bridge.mm`, qui déclare
les fonctions exposées. Les quatre y ajoutent des lignes, aux mêmes endroits.

Le conflit est donc certain, et il se règle par une règle et non par le hasard de la fusion :

- **Une pull request native n'ajoute jamais qu'une ligne par fichier partagé**, à la fin de sa liste,
  pour que le rebase se résolve mécaniquement.
- **L'ordre de fusion est celui du tableau** — 6, puis 7, puis 8, puis 9 —, et celle qui suit rebase
  sur la précédente avant de partir en revue. C'est l'ordre qui minimise l'attente : la 9 dépend de la
  7, la 8 ne bloque personne.

### Ce qui part en parallèle, et ce que ça vaut

- **2 et 3** : le contrat en JavaScript pur et le squelette natif ne se touchent pas. La 3 ne fusionne
  qu'après la 2 — un adaptateur écrit contre un contrat non publié se réécrit.
- **6, 7, 8 et 9**, dès que 3 est fusionnée, et **pendant l'attente du verdict de notarisation**.
  C'est le seul parallélisme qui a de la valeur ici : quatre familles d'API disjointes, quatre
  fichiers Objective-C++ disjoints, quatre ports disjoints — hors la surface partagée ci-dessus.

### Ce que le parallélisme coûte sur un refus de politique

Le verdict de la pull request 5 tombe alors que 6 à 9 sont écrites ou en cours. Ce plan ne décide pas
de leur sort — c'est une décision du fondateur, portée au §6 — mais il en pose la conséquence
technique, pour qu'elle ne se découvre pas au pire moment :

- Les six mesures non bloquantes **ne dépendent en rien d'Apple**. Leurs cellules sont déjà écrites
  sous `build/probe/releve/` et restent valides quel que soit le verdict.
- La pull request 10 dépend donc de 5 pour la **seule septième ligne** du relevé. Sur un refus de
  politique, elle se rend sans elle : un relevé qui porte six mesures tranchées et un refus nommé
  est exactement ce dont la décision du fondateur a besoin.

### La dépendance humaine

**L'adhésion au programme développeur Apple bloque les pull requests 4 et 5, et elles seules.** Les
pull requests 1, 2 et 3 avancent sans compte Apple ; le code Objective-C++ des pull requests 6 à 9
se compile et se teste non signé. Seule l'exécution avec une permission d'Accessibilité **stable**
exige l'identité signée, parce que l'autorisation suit l'identité et non le chemin (cadrage §8.2).

Elle est à engager le jour où la pull request 1 s'ouvre : la vérification d'identité prend plusieurs
jours, et elle est le seul délai de ce plan qu'aucun travail ne raccourcit.

### La frontière de relecture, dans chaque pull request native

La même partout, et c'est elle qui rend une pull request à deux langages relisable : **un fichier
Objective-C++ par famille d'API, un adaptateur JavaScript mince qui implémente un port déjà fusionné,
zéro logique dans l'adaptateur.** Le relecteur juge l'Objective-C++ contre la signature du port, et le
JavaScript contre un harnais de contrat déjà vert sur les doubles. Le cadrage §4 verrouille cette
frontière : « Toute logique produit reste hors du natif. »

---

## 5. Hors périmètre

- **Les phases 1 à 5 du cadrage §9** — squelette, Hardcore, applications, Simple, finition. Elles
  recevront leur plan quand les mesures seront rendues.
- **Le contexte `cycle` en entier** : états, échéances, budget de report, inhibition, suspension,
  sévérité, coupe-circuit. Rien de son glossaire n'est écrit ici.
- **Toute interface définitive** : barre de menus, panneau, réglages, onboarding, bannières. Le
  prototype de la phase 9 n'est pas un écran, et le cadrage §3 le dit — la Reconnaissance « ne produit
  aucun écran définitif ».
- **La persistance de l'état du cycle**, son écriture atomique et sa version de schéma — cadrage §8.3,
  phase 1.
- **Le journal local circulaire et borné** — cadrage §11, phase 1.
- **La vérification de mise à jour**, seule sortie réseau du produit — cadrage §10, phase 5.
- **Les tokens, contrastes et composants**, que `DESIGN.md` fige et que ce plan ne rouvre pas.
- **Les mesures produit** de `BRIEF.md` §12.3 — rétention, pauses menées à terme, sorties d'urgence.
  Elles relèvent du fondateur sur une bêta privée, pas de la Reconnaissance.

---

## 6. À décider

Les six mesures non bloquantes ne sont pas des décisions : elles se tranchent par mesure, leur repli
est déjà écrit dans `BRIEF.md` §12.3, et la phase 10 les rend. Ce qui suit attend une décision
humaine, et **aucune n'est remplacée par une valeur plausible en attendant**.

| Ce qui reste à décider | Qui tranche | En attendant |
|---|---|---|
| L'adhésion au programme développeur Apple, à engager | Le fondateur | Les pull requests 4 et 5 ne s'ouvrent pas. Les pull requests 1, 2, 3 et 6 à 9 avancent |
| Ce qu'un `Invalid` **de politique** déclenche, et le sort des pull requests 6 à 9 alors ouvertes | Le fondateur | `BRIEF.md` §12.3 dit qu'un refus arrête le produit. La phase 5 rend le motif d'Apple sans l'interpréter, et la phase 10 se rend sans la septième ligne |
| Le nombre de resoumissions autorisées après un `Invalid` **de chaîne** | Le fondateur | `build/notarize.sh` refuse de resoumettre sans une borne explicite passée en paramètre |
| L'attente maximale du verdict de notarisation, et la cadence d'interrogation | Le fondateur | `build/notarize.sh` exige les deux en paramètre et refuse de tourner sans elles |
| Le seuil de silence qui déclenche l'interrogation de secours des cadres, et le délai de retour qui l'arrête | La conception, mesure à l'appui | Aucune valeur n'est écrite. Deux tests de la phase 8 sont posés et non chiffrés, et ils le restent |
| La cadence de l'interrogation de secours — le cadrage §8.5 propose 4 Hz, valeur explicitement non sourcée et non reprise par `DESIGN.md` | La conception, mesure à l'appui | La phase 8 la traite comme provisoire, et aucun test n'oppose ce chiffre |
| Le délai maximal de notification d'un **retrait** d'écran — `BRIEF.md` §12.2 ne borne que la pose sur une *apparition* | La conception, mesure à l'appui | La phase 7 relève le délai et ne l'oppose à rien |
| L'application témoin en plein écran natif des mesures 3, 5 et 6 | Le fondateur | Les trois mesures sont déclarées manuelles, et leur mode opératoire vit dans `build/probe/README.md` |
| Le nom en code du contexte `traduction-systeme` — ce plan retient `system-bridge`, d'après le `NativeBridge` de son glossaire | La conception | `src/modules/system-bridge/`, à verser au glossaire dans la pull request qui le crée |
| L'accès à quatre machines ou machines virtuelles couvrant macOS 13, 14, 15 et 26 | Le fondateur | La phase 10 rend un relevé partiel et `--assert-complete` nomme les cellules manquantes |

Deux décisions du cadrage §14 — les deux rythmes proposés autour du défaut 50/10, et la fréquence de
vérification des mises à jour — restent ouvertes et n'affectent aucune phase de ce plan.
