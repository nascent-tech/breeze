---
type: conception
titre: Reconnaissance macOS
slug: chore-reconnaissance-macos
cree_le: 2026-08-28T19:30:57+0000
mis_a_jour_le: 2026-08-28T20:25:19+0000
branche: develop
statut: valide
---

Depuis : `.charpente/plans/2026-08-28-chore-reconnaissance-macos.md`

**En trois lignes.** Dix-neuf décisions figent un contexte `system-bridge` sans `domain/`, sans cas
d'usage et sans couche `interface/` : neuf objets-valeur, six refus, neuf ports à deux
implémentations, une chaîne signée qui se défend sur les droits **et** l'`Info.plist`, sept mesures.
*Elles écartent* quatre familles Objective-C++ qu'Electron 44 couvre déjà, les identifiants typés,
les défauts sur les seuils que le plan laisse à la mesure, et tout réexport par lequel `cycle`
importerait `Instant`. *Elles ne couvrent pas* le cœur de métier — le cycle, son budget de report,
sa persistance et son journal restent aux phases 1 à 5 du cadrage §9, et rien ici ne les préempte.

Cette conception fige les fichiers des **dix phases** de ce plan, et rien de plus. Le plan a arrêté
le modèle, les neuf ports, le découpage en dix pull requests et les critères de succès. Ce document
ne porte donc **aucune phase** — c'est le plan qui les porte, et `/charpente:build` les y lit. Le
§13 dit ce que la relecture a déplacé d'une phase à l'autre, fichier par fichier.

**Comment la lire.** Quatre familles de fichiers sont répétitives par construction : un double par
port, un adaptateur par port, une suite de contrat par port, un fichier Objective-C++ par famille
d'API. Leur surface se déduit du port qu'elles servent, et l'écrire neuf fois ferait un document que
personne ne relit ligne à ligne. Elles sont figées **en tableau**, une ligne par fichier. Les
objets-valeur, les erreurs et les ports — la seule surface que le reste déduit — sont figés **un par
un**.

---

## 1. La racine et les portes — phase 1

### `package.json`

**Surface** — les scripts, et eux seuls :

```
typecheck    tsc --checkJs --noEmit -p jsconfig.json
lint         eslint .
test         node --test test/
build        pnpm run build:native && electron-builder --dir --config build/electron-builder.yml
build:native electron-rebuild --module-dir native
probe        electron
```

`type` passe à `"module"` : tout le dépôt est en ESM, et l'extension `.js` cesse d'être ambiguë.

**Ce que ce fichier ne fait pas** : il ne signe pas et ne notarise pas. `build` construit un paquet
non signé (`--dir`) ; la signature est un réglage d'`electron-builder.yml`, la notarisation un
script appelé à la main. Une notarisation déclenchée par `pnpm run build` soumettrait à Apple chaque
construction locale.

### `jsconfig.json`

**Surface** : `checkJs`, `strict`, `target` ES2022, `module` ESM, `noEmit`, `include` sur `src/`,
`test/`, `build/` et `main.js`.

**Ce que ce fichier ne fait pas** : il n'introduit pas TypeScript. `tsc` n'y sert que de vérificateur
des annotations JSDoc, et aucun fichier `.ts` n'est écrit.

### `eslint.config.js`

**Surface** : configuration à plat, un seul export par défaut. Règles de base, plus les trois qui
tiennent des décisions de ce document : `no-unused-vars`, `no-restricted-syntax` sur `require(` hors
de `native-bridge-loader.js`, et `no-console` hors de `build/`.

**Ce que ce fichier ne fait pas** : il ne tient pas les frontières de couches. Les gardes de la
méthode et `dependency-cruiser` s'en chargent ; les recopier ici les ferait diverger.

### `main.js`

Racine de composition. Conservée à la racine, parce que `package.json` la désigne par `main`.

**Surface publique** — le fichier n'exporte rien, il s'exécute :

```
app.requestSingleInstanceLock()  → sortie immédiate, code non nul, erreur nommée
app.whenReady()                  → n'ouvre aucune fenêtre
app.on('will-quit')              → GlobalShortcutsPort.unregisterAll()
```

**Refus** — `AlreadyRunning`, déclarée **dans ce fichier** et nulle part ailleurs : une seconde
instance ne pose aucun overlay et n'enregistre aucun raccourci. Le verrou d'instance unique est une
primitive Electron portable, pas une traduction macOS ; elle n'a donc rien à faire dans les erreurs
de `system-bridge`, dont le glossaire dit qu'il rend accessible ce que **macOS** expose.

**Ce que ce fichier ne fait pas** : il ne crée aucune fenêtre au démarrage, n'enregistre aucun
raccourci global, ne lit aucun état de cycle. Ce qu'un lecteur pourrait croire qu'il porte — la
composition des cas d'usage — n'existe pas : la Reconnaissance n'a pas de cas d'usage, ses seuls
pilotes sont les sondes de `build/probe/`.

### `index.html` — supprimé

`main.js` ne charge plus aucune page ; le prototype de la phase 9 charge `build/probe/overlay.html`,
qui lui appartient. Le laisser à la racine donnerait à lire une fenêtre principale qui n'existe pas.

### `src/modules/system-bridge/index.js`

**Surface publique** : **vide de logique et sans réexport**. Le plan phase 1 le demande comme cible
de son test de lanceur, et c'est tout ce qu'il est.

**Ce que ce fichier ne fait pas** : il ne réexporte rien. Un baril qui rendrait publics les neuf
ports, les neuf objets-valeur et les sept erreurs serait une seconde source de vérité sur la surface
du contexte — elle avait déjà divergé dans une première version de ce document, qui comptait sept,
huit et neuf objets-valeur à trois endroits. Et c'est surtout la surface par laquelle un autre
contexte borné importerait `Instant` : voir le §12.

---

## 2. Les objets-valeur — `src/modules/system-bridge/application/model/`

Le plan l'a tranché : ce contexte n'a **pas de dossier `domain/`**. Ses objets-valeur vivent avec
les ports qui les échangent. Ils sont **neuf**.

**Quatre règles valent pour les neuf**, et ne sont pas redites fichier par fichier :

1. **Cinq ont une fabrique littérale** — `AppIdentity`, `Bounds`, `WindowFrame`, `Display`,
   `Shortcut` : `of(...)` prend **un seul objet littéral** et **refuse toute clé qu'elle ne connaît
   pas**. C'est cette règle, et non une inspection de contenu, qui tient le refus du plan
   « `AppIdentity` refuse tout champ qui ressemble à un titre de fenêtre » : un champ inconnu ne
   franchit pas la fabrique, quel qu'il soit. Elle borne la **forme** ; la règle 3 borne la taille.
2. **Quatre sont des valeurs fermées** — `AccessibilityStatus`, `WindowLevel`, `ScreenSharingLevel`,
   `Instant` : elles entrent par des fabriques nommées, et par `fromName` pour l'adaptateur qui
   reçoit une chaîne.
3. **Toute chaîne venue d'un processus tiers est bornée en longueur.** `displayName` vient de
   l'`Info.plist` d'une application qu'on n'a pas écrite, et finit dans le journal circulaire borné
   du cadrage §11. Une entrée non bornée qui entre dans un tampon borné est un déni de service.
4. **La validation vit dans le constructeur, pas dans la fabrique.** JavaScript n'a pas de
   constructeur privé : tant que le refus vit dans `of`, `new Bounds({ width: -1 })` reste un chemin
   ouvert vers un objet-valeur invalide. Les fabriques ne sont que des portes nommées qui appellent
   le constructeur.

L'instance est gelée par `Object.freeze` — qui est **superficiel** : tout champ composite est gelé
séparément à la construction et rendu en copie. L'état tient en champs `#`, l'égalité est
structurelle sauf mention contraire.

### `app-identity.value-object.js` — `AppIdentity`

```
static of({ bundleIdentifier, displayName }): AppIdentity
get bundleIdentifier(): string
get displayName(): string
equals(other: AppIdentity): boolean
```

**Refus** — `InvalidValue` :

| Refus | La règle qu'il protège |
|---|---|
| `bundleIdentifier` vide ou uniquement des espaces | Une identité sans identifiant ne désigne rien |
| `displayName` vide | Une identité que l'humain ne peut pas lire ne sert à rien |
| `bundleIdentifier` au-delà de 256 caractères, `displayName` au-delà de 256 | Les deux viennent d'une application tierce : c'est une entrée non fiable, et elle est bornée. 256 est une borne défensive arbitraire, pas une limite documentée d'Apple : elle borne un journal, elle ne modélise rien |
| Toute clé hors des deux | `BRIEF.md` §13 — aucun titre de fenêtre, aucun contenu, jamais |

**Ce que ce fichier ne fait pas** : il ne connaît ni le titre de la fenêtre, ni l'icône, ni le chemin
du binaire, ni l'identifiant de processus. Un lecteur pourrait croire qu'il porte « tout ce qu'on
sait de l'application » — il porte les deux champs que le brief autorise.

### `bounds.value-object.js` — `Bounds`

```
static of({ x, y, width, height }): Bounds
get x(): number ; get y(): number ; get width(): number ; get height(): number
equals(other: Bounds): boolean
```

**Refus** — `InvalidValue` : largeur ou hauteur **négative** ; toute coordonnée non finie ; toute
clé hors des quatre. Les coordonnées négatives sont **acceptées** : un écran secondaire à gauche du
principal en a.

**Une surface nulle est acceptée**, et c'est un refus qui a été retiré : macOS expose des fenêtres
de surface nulle — auxiliaires, en cours de destruction, éléments d'accessibilité dégénérés. Le
refuser transformerait un fait du système en erreur, contre la règle du §6 : un pont muet est un
fait, pas un zéro. Si une surface non nulle est requise, c'est une règle du prototype qui pose un
overlay, pas de l'observation.

**Ce que ce fichier ne fait pas** : aucune géométrie — ni intersection, ni contenance, ni conversion
entre repères. Le repère de macOS et celui d'Electron diffèrent, et cette traduction appartient à
l'adaptateur qui la connaît.

### `window-frame.value-object.js` — `WindowFrame`

```
static of({ windowId, processId, bounds }): WindowFrame
get windowId(): number ; get processId(): number ; get bounds(): Bounds
equals(other: WindowFrame): boolean
```

**Refus** — `InvalidValue` : `windowId` ou `processId` non entier positif ; `bounds` qui n'est pas un
`Bounds` ; toute clé hors des trois.

**Ce que ce fichier ne fait pas** : il ne porte ni titre, ni contenu, ni niveau d'empilement, ni état
de minimisation. Le cadre d'un overlay de Breeze n'est pas un `WindowFrame` — le glossaire le dit, et
le prototype de la phase 9 ne construit jamais celui-là pour décrire les siens.

### `display.value-object.js` — `Display`

```
static of({ displayId, bounds, carriesMenuBar }): Display
get displayId(): number ; get bounds(): Bounds ; get carriesMenuBar(): boolean
equals(other: Display): boolean
```

**Refus** — `InvalidValue` : `displayId` non entier positif ; `bounds` qui n'est pas un `Bounds` ;
`carriesMenuBar` non booléen ; toute clé hors des trois. **Un refus par champ, sans exception** : la
règle commune ne couvre que les clés inconnues, pas le type des clés connues.

**Ce que ce fichier ne fait pas** : ni luminosité, ni profil couleur, ni facteur d'échelle. Il ne
garantit pas non plus l'unicité des identifiants dans une liste — ce refus appartient à
`DisplaysPort`, seul à voir la liste entière.

### `accessibility-status.value-object.js` — `AccessibilityStatus`

```
static granted(): AccessibilityStatus
static notGranted(): AccessibilityStatus
static fromName(name: string): AccessibilityStatus
get name(): string
get isGranted(): boolean
equals(other: AccessibilityStatus): boolean
```

**Deux valeurs, pas trois.** Un troisième état `neverAsked` a été retiré : `AXIsProcessTrusted` et
`systemPreferences.isTrustedAccessibilityClient` rendent tous deux un **booléen**, et TCC n'expose
publiquement aucune distinction entre « refusé » et « jamais demandé ». Le troisième état serait soit
mort — accepté par `fromName`, produit par rien —, soit dérivé d'un souvenir de processus, ce que
D6 écarte. Inventer la distinction ferait décider de solliciter l'humain sur une valeur que rien ne
source, et rouvrirait une boîte de dialogue chez quelqu'un qui a déjà refusé. Si le produit en a
besoin, elle devient une **mesure** du relevé, pas une fabrique.

**Refus** — `InvalidValue` sur un nom hors des deux.

**Ce que ce fichier ne fait pas** : il ne porte pas le **moment** de l'octroi. L'observation est un
niveau, pas un front ; un `grantedAt` ferait croire qu'on détecte une révocation par comparaison,
alors qu'elle est silencieuse et se relit à chaque appel.

### `window-level.value-object.js` — `WindowLevel`

```
static maximum(): WindowLevel
static fromName(name: string): WindowLevel
get name(): string
equals(other: WindowLevel): boolean
```

**Le niveau est un nom, jamais un nombre** — c'est aussi la forme qu'Electron retient
(`setAlwaysOnTop(flag, level)` prend `'normal'`, `'floating'`, … `'screen-saver'`). Aucune constante
d'empilement n'est écrite en JavaScript ; l'adaptateur traduit le nom vers le nom d'Electron. Le
refus du plan — « refuse un niveau au-delà du maximal que le port déclare » — devient un refus de nom
inconnu, qui se vérifie sans Mac.

`normal()` a été retirée : le prototype pose le maximal, c'est tout, et `fromName('normal')` reste le
chemin de lecture.

**Refus** — `InvalidValue` sur un nom que la table de traduction ne connaît pas.

**Ce que ce fichier ne fait pas** : il ne compare pas deux niveaux et n'en tire aucun ordre. Rien
dans la Reconnaissance n'a besoin de savoir lequel est au-dessus.

### `shortcut.value-object.js` — `Shortcut`

```
static of({ identity, keyEquivalent, modifiers }): Shortcut
get identity(): string ; get keyEquivalent(): string ; get modifiers(): string[]
equals(other: Shortcut): boolean      // compare identity, et elle seule
```

**L'égalité porte sur `identity` seule**, et c'est une décision (D2 du §11). `identity` est le nom que
Breeze donne à l'enregistrement, et c'est par lui que `unregister` désigne ce qu'il retire — jamais
par la combinaison, qui peut changer. Une égalité structurelle déclarerait différents deux `Shortcut`
que le port traite comme le même enregistrement.

`modifiers` est un **ensemble** : la fabrique en normalise l'ordre avant de geler, faute de quoi
`['cmd','shift']` et `['shift','cmd']` — le même raccourci — seraient inégaux. Le tableau interne est
gelé et rendu en copie : sans cela, `shortcut.modifiers.push('hyper')` ferait entrer un modificateur
que la fabrique venait de refuser.

**Refus** — `InvalidValue` :

| Refus | La règle qu'il protège |
|---|---|
| `identity` vide, ou au-delà de 128 caractères | Un enregistrement qu'on ne sait pas nommer, on ne sait pas le retirer. 128 est une borne défensive arbitraire, au même titre que celle d'`AppIdentity` |
| `keyEquivalent` vide | Une combinaison vide n'est pas un raccourci |
| `modifiers` vide | Un raccourci global sans modificateur confisque une touche à tout le système |
| Un modificateur hors de la liste connue, ou répété | Un modificateur inventé s'enregistre sans effet et ne se retire jamais |

**Ce que ce fichier ne fait pas** : il n'interdit aucune combinaison. Le refus des combinaisons de
mise hors tension et de verrouillage — `BRIEF.md` §13 — appartient à `GlobalShortcutsPort`.

### `screen-sharing-level.value-object.js` — `ScreenSharingLevel`

```
static none(): ScreenSharingLevel
static sharing(): ScreenSharingLevel
static presenting(): ScreenSharingLevel
static indistinguishable(): ScreenSharingLevel
static fromName(name: string): ScreenSharingLevel
get name(): string
equals(other: ScreenSharingLevel): boolean
```

**Cet objet-valeur n'est pas dans la table du plan §1**, et c'est un ajout assumé : la mesure 3 peut
conclure que macOS ne distingue pas une présentation d'un simple plein écran sans permission
d'enregistrement. Sans une valeur qui porte ce constat, `ScreenSharingPort` rendrait une chaîne libre,
et l'absence de distinction se lirait comme une absence de partage — deux faits opposés confondus
dans la même cellule du relevé.

**`indistinguishable` n'est pas un niveau de partage : c'est l'absence de lecture possible.** Les
trois autres décrivent le monde, celui-ci décrit notre capacité à le lire sur une version de macOS
donnée. Tout appelant le traite comme un cas à part, et **jamais comme `none`**.

**Refus** — `InvalidValue` sur un nom hors des quatre.

**Ce que ce fichier ne fait pas** : il ne dit pas *qui* partage l'écran, ni vers où.

### `instant.value-object.js` — `Instant`

```
static monotonic(nanoseconds: bigint): Instant
static wallClock(epochMilliseconds: number): Instant
get isMonotonic(): boolean
millisecondsSince(other: Instant): number
```

**Refus** :

| Refus | La règle qu'il protège |
|---|---|
| `millisecondsSince` entre un instant monotone et un instant mural | Deux échelles sans origine commune ; leur différence n'a aucun sens |
| `monotonic` avec autre chose qu'un `bigint` | La source monotone du système compte en nanosecondes, et un `number` les perd |
| `wallClock` avec un nombre non fini | Une échéance non finie n'échoit jamais |

`equals` a été retirée : rien dans le plan ni ici ne compare deux instants pour l'égalité — on en fait
une différence, ou on compare une échéance à un réveil. La retirer ferme aussi la porte par laquelle
un `monotonic(1000n)` se serait déclaré égal à un `wallClock(1000)`.

**Ce que ce fichier ne fait pas** : il ne porte aucune durée et n'en construit aucune. Une durée est
une différence de deux instants, rendue en millisecondes — un nombre, pas un objet. Il ne
s'incrémente ni ne se décrémente : le cadrage §8.3 interdit les minuteries qui dérivent, et un
`Instant` qui saurait avancer les réintroduirait.

---

## 3. Les erreurs — `src/modules/system-bridge/application/errors/`

Sept fichiers, une classe chacun. Un contexte `générique` n'a pas de hiérarchie d'erreurs métier : il
a une base qui rend la famille rattrapable, et un refus nommé par règle protégée.

### `system-bridge.error.js` — `SystemBridgeError`

```
class SystemBridgeError extends Error
constructor(message: string)
```

Le constructeur pose `this.name = new.target.name` : sans lui, une erreur minifiée perd son
identité. Les messages sont en **anglais**, et ne révèlent ni chemin, ni identifiant de processus.

**Pas de champ `code`.** Il n'avait aucun lecteur — aucune des sondes du §10 ne le lit, et les seuls
codes de sortie de ce document sont ceux de `notarize.sh`, un script shell qui n'attrape aucune
erreur JavaScript. Il redoublait le nom de la classe et imposait une table nom↔code à tenir à la
main. Les appelants classent par `instanceof`, qui est le seul contrat vérifiable à l'exécution en
JavaScript.

**Pas de couches intermédiaires** non plus — ni `NotFound`, ni `Conflict`, ni `Forbidden`. Six
refus dans un contexte de sorte `générique` ne demandent pas quatre classes vides pour être
distingués ; le jour où la phase 1 du cadrage §9 écrira son traducteur d'erreurs, il les ajoutera
avec son besoin.

**Ce que ce fichier ne fait pas** : il ne porte aucun code de sortie, aucun statut. La traduction
d'une erreur en code de sortie appartient à la sonde qui la rattrape.

### Les six refus

| Fichier | Classe | La règle qu'elle protège | Levée par |
|---|---|---|---|
| `invalid-value.error.js` | `InvalidValue` | Un objet-valeur est valide par construction, ou n'existe pas | Les neuf objets-valeur, et eux seuls |
| `accessibility-denied.error.js` | `AccessibilityDenied` | Une lecture de cadre sans permission est un refus nommé, pas un plantage générique | `WindowFramesPort` |
| `unknown-window.error.js` | `UnknownWindow` | Poser un niveau sur une fenêtre détruite réussit en silence et ne se voit jamais | `WindowLevelPort` |
| `shortcut-already-registered.error.js` | `ShortcutAlreadyRegistered` | Le second enregistrement d'une même identité resterait orphelin au désenregistrement | `GlobalShortcutsPort` |
| `reserved-shortcut.error.js` | `ReservedShortcut` | `BRIEF.md` §13 : éteindre le Mac et verrouiller la session restent possibles, toujours | `GlobalShortcutsPort` |
| `duplicate-display.error.js` | `DuplicateDisplay` | Deux écrans de même identifiant feraient poser deux overlays sur le même, et zéro sur l'autre | `DisplaysPort` |

`ReservedShortcut` et `DuplicateDisplay` étaient des `InvalidValue` dans une première version. Une
erreur générique levée par des règles sans rapport entre elles n'est pas une erreur typée : un appelant qui la
rattrape doit lire une chaîne pour savoir s'il a demandé un raccourci que le brief interdit ou si le
système lui a rendu deux écrans identiques. `InvalidValue` est désormais **réservée aux fabriques
d'objets-valeur**.

Deux erreurs ont disparu :

- **`PermissionAlreadyGranted`** protégeait une règle d'écran — « fait croire à l'humain qu'il a mal
  cliqué » —, pas un refus du contrat, et rendait `requestGrant()` non idempotent alors que tout le
  contexte l'est. La non-répétition de la sollicitation appartient à l'onboarding de la phase 4 du
  cadrage §9, que le §12 exclut déjà.
- **`AlreadyRunning`** ne se rattachait à aucun port, et le verrou d'instance unique est une
  primitive Electron portable, pas une traduction macOS. Elle vit maintenant dans `main.js`.

**Ce que ces fichiers ne font pas** : aucune ne porte de donnée structurée au-delà de son message.
Ce que l'appelant fait d'un refus, c'est arrêter.

---

## 4. Les ports — `src/modules/system-bridge/application/ports/`

Neuf fichiers, une classe de contrat chacun, **toutes les méthodes lèvent**. Ils vivent sous
`application/ports/` et non sous `domain/ports/` : ce contexte n'a pas de domaine.

**Deux règles valent partout.** Un abonnement s'écrit `onX(listener)` et rend une fonction de
désabonnement idempotente — c'est la forme des API qu'on enveloppe, toutes en rappel, et celle que
les doubles câblent déjà. Et **un port ne revalide pas ce qu'un objet-valeur refuse déjà** : les
identifiants restent des entiers nus, leur refus vit dans `WindowFrame` et `Display`, une seule fois.

### `clock.port.js` — `ClockPort`

```
monotonicNow(): Instant
wallClockNow(): Instant
```

**Refus** : aucun. Lire l'heure ne refuse rien.

**Ce que ce fichier ne fait pas** : aucune minuterie, aucun `setTimeout`, aucun ordonnancement. Le
cadrage §8.3 fait de l'échéance la source de vérité ; un port qui saurait réveiller quelqu'un
ramènerait le décompte que ce choix écarte.

### `accessibility-permission.port.js` — `AccessibilityPermissionPort`

```
currentStatus(): AccessibilityStatus
requestGrant(): void
```

`currentStatus()` **relit à chaque appel** et ne mémorise rien : c'est ce qui couvre à la fois
« l'état courant » et « la revérification » du plan §2, et la seule forme qui voie une révocation
silencieuse. Une troisième méthode `recheck()` serait le même appel sous un autre nom.

**`requestGrant()` ne rend rien**, et c'est une correction de fond. `AXIsProcessTrustedWithOptions` —
comme `isTrustedAccessibilityClient(true)` — rend **immédiatement** la valeur courante, donc « non
accordé », puis rend la main pendant que l'humain part dans les Réglages Système. Un appelant qui
lirait ce retour comme le verdict de sa demande conclurait systématiquement au refus. C'est même
l'objet de la mesure 1 : savoir si l'octroi se propage sans relancer l'application. L'état se relit
par `currentStatus()`.

`requestGrant()` est **idempotent** : appelé sur un état déjà accordé, il n'ouvre rien et ne lève
pas.

**Refus** : aucun.

**Ce que ce fichier ne fait pas** : `currentStatus()` **n'ouvre aucune boîte de dialogue**. C'est la
distinction qui rend la mesure 4 lisible : lire l'état ne le change pas, et seul `requestGrant()`
sollicite l'humain. Il n'ouvre pas non plus les Réglages Système — la phase 4 du cadrage §9 s'en
chargera, avec son écran.

### `foreground-app.port.js` — `ForegroundAppPort`

```
currentApp(): AppIdentity
runningApps(): AppIdentity[]
onChange(listener: (app: AppIdentity) => void): () => void
```

**Refus** : aucun. Une machine a toujours une application au premier plan, et une liste vide n'est
pas une erreur.

**Ce que ce fichier ne fait pas** : il ne rend jamais un titre de fenêtre — le type de retour le
garantit, `AppIdentity` n'en a pas de place. Il ne masque, ne quitte ni ne suspend aucune
application : le cadrage §5 l'interdit, et aucune méthode d'écriture n'existe ici.

### `window-frames.port.js` — `WindowFramesPort`

```
framesOf(processId: number): WindowFrame[]
onMove(listener: (frame: WindowFrame) => void): () => void
onResize(listener: (frame: WindowFrame) => void): () => void
get pollingActive(): boolean
```

`pollingActive` est la trace observable du repli du cadrage §8.5. Sa justification n'est **pas** le
test : un repli d'interrogation qui ne cesse jamais coûte sa cadence en permanence, contre les cibles
de `BRIEF.md` §12.2, et c'est un défaut qu'il faut voir en exploitation. Qu'il rende aussi le repli
vérifiable est une conséquence, pas la raison.

**Refus** :

| Refus | Erreur | La règle qu'il protège |
|---|---|---|
| Toute méthode sans permission d'Accessibilité | `AccessibilityDenied` | Une erreur générique se confondrait avec un plantage du pont natif |

Un processus **sans fenêtre** rend une liste vide et ne lève pas : c'est un fait, pas un refus. Un
`processId` invalide n'est plus refusé ici — `WindowFrame` le refuse déjà, et l'écrire deux fois
faisait diverger deux règles.

**Ce que ce fichier ne fait pas** : il ne lit ni le titre ni le contenu d'une fenêtre
(`BRIEF.md` §13), ne les déplace pas, ne les redimensionne pas, ne les ferme pas. Il ne chiffre pas
non plus le seuil de silence, le délai de retour ni la cadence du repli : ce sont les trois
paramètres de l'adaptateur, et le plan §6 les laisse à la mesure.

### `window-level.port.js` — `WindowLevelPort`

```
apply(windowId: number, level: WindowLevel): void
levelOf(windowId: number): WindowLevel
```

**Refus** — `UnknownWindow` sur un `windowId` que le système ne connaît plus, pour les deux méthodes.

**Ce que ce fichier ne fait pas** : il ne pose de niveau que sur une fenêtre **de Breeze**. Le niveau
d'une fenêtre tierce n'est pas exposé par macOS, et le glossaire l'a déjà dit.

### `presentation-options.port.js` — `PresentationOptionsPort`

```
hideDockAndMenuBar(): void
disableApplicationSwitching(): void
restore(): void
```

`restore()` **sans pose préalable ne fait rien** et ne lève pas : c'est ce qui permet de l'appeler sur
tous les chemins de sortie, y compris celui d'un démarrage qui a échoué avant la pose. C'est aussi
pourquoi `applied` a été retiré : `restore()` étant idempotent, personne n'a besoin de poser la
question, et aucune fonction native ne pouvait la servir sans conserver l'état que le §6 interdit.

**Refus** : aucun. Toute erreur ici laisserait un Dock masqué.

**Ce que ce fichier ne fait pas** — et c'est le refus le plus important du port : **aucune méthode
n'expose `DisableForceQuit`, `DisableSessionTermination` ni `DisableHideApplication`**, et le port
n'accepte **aucun masque d'options**. Trois méthodes nommées, deux jeux d'options écrits en dur dans
le natif. `BRIEF.md` §13 exige que quitter, éteindre et atteindre les réglages du système restent
possibles à tout instant, et le cadrage §7 range « Forcer à quitter » en non interceptable, assumé.
Un port qui prendrait un masque rouvrirait ces trois options par la porte la plus directe qui soit.

Il ne mémorise pas non plus les options de l'utilisateur pour les restaurer une par une : macOS rend
les options **précédentes de l'application elle-même**, et prétendre sauvegarder celles du système
inventerait un état que personne ne détient.

### `displays.port.js` — `DisplaysPort`

```
list(): Display[]
onConnected(listener: (display: Display) => void): () => void
onDisconnected(listener: (displayId: number) => void): () => void
onReconfigured(listener: (display: Display) => void): () => void
```

Trois abonnements distincts plutôt qu'un `onChange` porteur d'une nature : la nature deviendrait un
dixième objet-valeur, et chaque abonné rouvrirait le même `switch`. `onDisconnected` rend un
**identifiant**, pas un `Display` — l'écran a disparu, ses dimensions ne sont plus lisibles.

**Refus** — `DuplicateDisplay` : `list()` refuse de rendre deux `Display` de même `displayId`. Le
refus vit ici parce que le port est le seul à voir la liste entière.

**Ce que ce fichier ne fait pas** : il ne borne aucun délai de notification. `BRIEF.md` §12.2 borne
la **pose** d'un overlay sur un écran apparu, pas la notification d'un retrait ; le plan §6 laisse ce
délai à la mesure, et la phase 7 le relève sans l'opposer.

### `screen-sharing.port.js` — `ScreenSharingPort`

```
currentLevel(): ScreenSharingLevel
```

Une seule méthode, et c'est une **lecture** : le plan l'exige — « rend un niveau, jamais un front ».
Aucun abonnement, donc aucune tentation de traiter un début de partage comme un événement.

**Refus** : aucun. `indistinguishable` est le verdict de la mesure 3 quand macOS ne distingue rien,
et c'est une valeur, pas une erreur.

**Ce que ce fichier ne fait pas** : il ne demande jamais la permission d'enregistrement d'écran, et
ne l'exige pas. Toute la mesure 2 tient dans ce refus.

### `global-shortcuts.port.js` — `GlobalShortcutsPort`

```
register(shortcut: Shortcut, listener: () => void): void
unregister(identity: string): void
unregisterAll(): number
registered(): Shortcut[]
```

`purgeStale()` est devenue `unregisterAll()`, et c'est une correction de fait, pas de nom. La version
précédente affirmait qu'elle « fonctionne après une sortie anormale précédente » : la conclusion ne
suivait pas. Un processus neuf ne détient rien ; appelée au démarrage, elle purgerait l'ensemble
vide. Il ne restait qu'un chemin réel, `will-quit`, qui ne se déclenche précisément pas sur `SIGKILL`
ni sur « Forcer à quitter » — alors que le cadrage §7 en exige **deux, dont un qui survit à une
sortie anormale**.

Le second chemin n'est donc pas dans ce port : c'est **le système lui-même**, qui relâche les
enregistrements avec le processus qui les détient. Ce document ne l'affirme pas — il en fait une
**mesure de plus au relevé**, prise par `build/probe/assert-clean-system.mjs` après une sortie
anormale simulée. Si des résidus apparaissent, le second chemin devra être figé, et ce sera un fait
mesuré qui le dira.

**Refus** :

| Refus | Erreur | La règle qu'il protège |
|---|---|---|
| `register` d'une `identity` déjà détenue | `ShortcutAlreadyRegistered` | Le second enregistrement resterait orphelin au désenregistrement |
| `register` d'une combinaison réservée par le système — mise hors tension, verrouillage | `ReservedShortcut` | `BRIEF.md` §13 : éteindre le Mac reste possible, toujours |
| `unregister` d'une `identity` inconnue | aucun | Un désenregistrement idempotent doit pouvoir tourner sur tous les chemins de sortie |

**La liste des combinaisons réservées a un domicile unique** : `application/model/reserved-shortcuts.js`,
une constante gelée que le port lit. Sans domicile, le double et l'adaptateur l'auraient chacun
réimplémentée, et un refus vert sur le double aurait pu rester silencieusement absent en vrai.

**Ce que ce fichier ne fait pas** : il n'intercepte aucune frappe hors des combinaisons qu'il
enregistre. Ce n'est pas un enregistreur de clavier ; aucune méthode n'ouvre un flux de touches.

---

## 5. Les doubles — `src/modules/system-bridge/infrastructure/doubles/`

Neuf fichiers, un par port. Chacun **étend son port**, l'implémente en mémoire, et ajoute les
méthodes de **pilotage** qu'un scénario appelle. Le pilotage est nommé à l'impératif — `set`, `emit`,
`simulate` — pour qu'aucune ligne ne confonde ce qui observe et ce qui provoque.

| Fichier | Classe | Ce qu'elle ajoute au port |
|---|---|---|
| `clock.double.js` | `ClockDouble` | `setWallClock(epochMilliseconds)`, `advance(milliseconds)` |
| `accessibility-permission.double.js` | `AccessibilityPermissionDouble` | `setStatus(status)`, `grantsOnRequest(boolean)` |
| `foreground-app.double.js` | `ForegroundAppDouble` | `setCurrentApp(identity)`, `setRunningApps(identities)`, `emitChange(identity)` |
| `window-frames.double.js` | `WindowFramesDouble` | `setFrames(processId, frames)`, `emitMove(frame)`, `emitResize(frame)`, `simulateSilence(ms)`, `denyAccessibility()`, `grantAccessibility()` |
| `window-level.double.js` | `WindowLevelDouble` | `openWindow(windowId)`, `destroyWindow(windowId)` |
| `presentation-options.double.js` | `PresentationOptionsDouble` | `appliedOptions()`, `simulateAbnormalExit()` |
| `displays.double.js` | `DisplaysDouble` | `setDisplays(displays)`, `emitConnected(display)`, `emitDisconnected(displayId)`, `emitReconfigured(display)` |
| `screen-sharing.double.js` | `ScreenSharingDouble` | `setLevel(level)` |
| `global-shortcuts.double.js` | `GlobalShortcutsDouble` | `trigger(identity)`, `simulateAbnormalExit()`, `leftovers()` |

`denyAccessibility()` / `grantAccessibility()` sur `WindowFramesDouble` sont un ajout de la relecture :
`AccessibilityDenied` est le refus le plus important du port — c'est lui que la mesure 4 observe — et
aucune méthode ne savait le provoquer. Les deux doubles ne se connaissent pas ; celui-ci tient son
propre drapeau.

`setMonotonic` a été retirée de `ClockDouble` : `advance` couvre tout ce que le plan demande, et
poser une origine absolue permettait de faire **reculer** une horloge monotone, état que le système
n'atteint jamais. `appliedLevels()` a été retirée de `WindowLevelDouble` — `levelOf` la rend déjà, et
elle est vérifiée contre le vrai adaptateur. `revokeSilently()` a été retirée de
`AccessibilityPermissionDouble` : le double n'émet aucune notification, donc une révocation y **est**
`setStatus(notGranted())`.

**Ce que ces fichiers ne font pas** : aucun ne charge le module natif, aucun ne touche au système,
aucun ne dort. `ClockDouble.advance` déplace un instant, il n'attend rien.

---

## 6. Les adaptateurs — `src/modules/system-bridge/infrastructure/`

**Quatre ports sur neuf n'ont rien à traduire.** Electron 44 les couvre déjà, vérifié dans
`node_modules/electron/electron.d.ts` du dépôt :

| Port | Ce qu'Electron expose |
|---|---|
| `AccessibilityPermissionPort` | `systemPreferences.isTrustedAccessibilityClient(prompt)` — la lecture (`false`) et la demande (`true`), exactement la séparation que D7 exige |
| `WindowLevelPort` | `BrowserWindow.setAlwaysOnTop(flag, level)` — des niveaux **nommés**, `'screen-saver'` au sommet, exactement D5 |
| `DisplaysPort` | `screen.getAllDisplays()`, et les événements `display-added`, `display-removed`, `display-metrics-changed` |
| `GlobalShortcutsPort` | `globalShortcut.register`, `.unregister`, `.unregisterAll()` |

Leur écrire un adaptateur Objective-C++ réimplémenterait un mécanisme que la cible fournit, avec sa
compilation universelle, sa signature élément par élément et son risque propre — pour un résultat
identique. Le §13 dit ce que ce retrait déplace dans les phases du plan.

### `infrastructure/electron/` — quatre adaptateurs

| Fichier | Classe | Port | Ce que sa traduction ajoute |
|---|---|---|---|
| `accessibility-permission.electron.js` | `ElectronAccessibilityPermission` | `AccessibilityPermissionPort` | Booléen → `granted()` / `notGranted()` |
| `window-level.electron.js` | `ElectronWindowLevel` | `WindowLevelPort` | Nom de `WindowLevel` → nom d'Electron ; `UnknownWindow` sur une fenêtre détruite |
| `displays.electron.js` | `ElectronDisplays` | `DisplaysPort` | `Display` d'Electron → `Bounds` et `Display`, `carriesMenuBar` déduit de l'écran principal |
| `global-shortcuts.electron.js` | `ElectronGlobalShortcuts` | `GlobalShortcutsPort` | Registre des `Shortcut` détenus, refus de doublon et de combinaison réservée |

### `infrastructure/native/` — quatre adaptateurs et un chargeur

`native-bridge-loader.js` — `NativeBridgeLoader` :

```
class NativeBridgeLoader
static load(): object     // les exports du .node
```

Le glossaire réserve `NativeBridge` au **composant compilé** ; ce fichier n'en est que la porte
d'entrée JavaScript, et il porte donc un autre nom. Un seul fichier charge le `.node` : ailleurs,
quatre adaptateurs le chargeraient chacun, et une compilation manquante produirait quatre messages
au lieu d'un. Ce n'est pas un cache — `require` mémoïse déjà — c'est **un seul site d'appel et un
seul message d'échec**.

| Fichier | Classe | Port | Ce que sa traduction ajoute |
|---|---|---|---|
| `foreground-app.native.js` | `NativeForegroundApp` | `ForegroundAppPort` | Deux champs bruts → `AppIdentity.of`, et rien d'autre |
| `presentation-options.native.js` | `NativePresentationOptions` | `PresentationOptionsPort` | Aucune : pose et restauration, sans état conservé |
| `screen-sharing.native.js` | `NativeScreenSharing` | `ScreenSharingPort` | Chaîne du `.node` → `ScreenSharingLevel.fromName` |
| `window-frames.native.js` | `NativeWindowFrames` | `WindowFramesPort` | Rectangle → `Bounds`, et le repli d'interrogation |

**`NativeWindowFrames` est le seul à prendre autre chose que le pont** :

```
constructor(bridge: object, { silenceMs: number, resumeMs: number, pollHz: number })
```

et il **refuse de se construire** — `InvalidValue` — si l'un des trois manque. C'est la forme que
prend la décision du plan §6 : le seuil de silence, le délai de retour **et la cadence du repli** se
mesurent, et **aucune valeur par défaut n'est écrite**. Les 4 Hz du cadrage §8.5 ne sont pas repris
ici : le cadrage les donne lui-même pour non sourcés, et `DESIGN.md` ne les reprend pas. Un défaut deviendrait un fait par inertie, et la mesure ne serait
jamais prise.

### `infrastructure/system-clock.js` — `SystemClock`

```
class SystemClock extends ClockPort
monotonicNow(): Instant      // process.hrtime.bigint()
wallClockNow(): Instant      // Date.now()
```

**Ce que ce fichier ne fait pas** : il ne corrige aucune dérive et ne synchronise rien. Le cadrage
§8.3 demande deux formes tenues côte à côte, pas une forme réconciliée.

**Ce que les neuf adaptateurs ne font pas** : aucun ne décide. Aucun ne met en cache un résultat —
une révocation de permission est silencieuse, et un cache la masquerait. Aucun ne rattrape une erreur
de sa source pour rendre une valeur de repli : un pont muet est un fait, pas un zéro.

---

## 7. Le harnais de contrat — `test/modules/system-bridge/contract/`

Neuf fichiers. Ce sont des fichiers de **code**, pas des suites : chacun exporte une fonction qui
déroule le contrat de son port. La phase 2 les branche sur les doubles, les phases 3 et 6 à 9 les
rebranchent sur les adaptateurs réels. Un adaptateur n'a ainsi **aucun test à écrire** : il a un
harnais à passer.

**La fabrique rend un sujet, pas un port :**

```
run<Port>Contract(makeSubject: () => { port, driver })
```

C'était la contradiction que la relecture a trouvée. Une première version passait `makePort` seul,
tout en interdisant au harnais de piloter le double — or presque tout ce que le plan §2 demande est
un **événement provoqué** : un changement de premier plan, un silence, un débranchement, une
révocation, une fenêtre détruite. Sans pilote, le harnais ne pouvait rien observer qu'il n'ait
provoqué, donc rien prouver.

Le `driver` porte les déclencheurs de son port, **et une seule méthode commune** :

```
supports(trigger: string): boolean
```

Le double supporte tout : `makeSubject()` y rend `{ port: d, driver: d }`. L'implémentation réelle
supporte ce qu'elle peut provoquer dans son propre processus, et **déclare le reste non supporté** —
un débranchement d'écran physique, une sortie anormale — auquel cas le harnais **saute le cas et
l'annonce**, plutôt que de le taire ou de le faire échouer. Ce qui est sauté côté réel est couvert
ailleurs : par le prototype et `assert-clean-system.mjs` pour la sortie anormale, par une mesure
manuelle pour le débranchement.

| Fichier | Fonction exportée | Déclencheurs attendus du `driver` |
|---|---|---|
| `clock.contract.js` | `runClockContract` | `setWallClock`, `advance` |
| `accessibility-permission.contract.js` | `runAccessibilityPermissionContract` | `setStatus`, `grantsOnRequest` |
| `foreground-app.contract.js` | `runForegroundAppContract` | `setCurrentApp`, `emitChange` |
| `window-frames.contract.js` | `runWindowFramesContract` | `setFrames`, `emitMove`, `emitResize`, `simulateSilence`, `denyAccessibility` |
| `window-level.contract.js` | `runWindowLevelContract` | `openWindow`, `destroyWindow` |
| `presentation-options.contract.js` | `runPresentationOptionsContract` | `appliedOptions`, `simulateAbnormalExit` |
| `displays.contract.js` | `runDisplaysContract` | `setDisplays`, `emitConnected`, `emitDisconnected` |
| `screen-sharing.contract.js` | `runScreenSharingContract` | `setLevel` |
| `global-shortcuts.contract.js` | `runGlobalShortcutsContract` | `trigger`, `simulateAbnormalExit`, `leftovers` |

`makeSubject` rend un sujet **neuf** à chaque appel, et le harnais l'appelle une fois par cas : un
état partagé entre deux cas ferait passer le second grâce au premier.

**Ce que ces fichiers ne font pas** : ils ne déclarent aucun `describe` racine et ne se lancent pas
seuls. Ce sont les fichiers de suite — `test/modules/system-bridge/infrastructure/doubles/*.test.js`,
`.../electron/*.test.js` et `.../native/*.test.js` — qui les appellent en fournissant leur
`makeSubject`, et eux seuls que `node --test` découvre.

---

## 8. La surface Objective-C++ — `native/`

**Quatre familles, et non huit** : les quatre autres sont passées à Electron (§6).

### `native/binding.gyp`

**Surface** : une cible `breeze_bridge`, `sources` listant `breeze_bridge.mm` puis un fichier par
famille, les cadres liés — `Cocoa`, `ApplicationServices`, `CoreGraphics` —, ARC activé, et `archs`
couvrant `x86_64` et `arm64`.

Aucun cadre de capture n'est lié : ni `ScreenCaptureKit`, ni `AVFoundation`, ni `CoreMediaIO`. Un
droit de caméra ou de micro n'aurait aucun consommateur dans cette liste, et c'est ce qui le rend
vérifiable plutôt que promis. `Carbon` a disparu avec `global_shortcuts.mm`.

**Ce que ce fichier ne fait pas** : il ne signe rien et ne construit aucun paquet. Il produit un
`.node`, qu'`electron-builder` signera comme un élément parmi d'autres.

### `native/src/breeze_bridge.mm`

**Surface** : les quatre déclarations `extern` en tête, `Init(Napi::Env, Napi::Object)` qui appelle
les quatre `Register*` — **un appel par ligne** — et le `NODE_API_MODULE`.

Un fichier d'en-tête `breeze_bridge.h` avait été figé pour tenir la règle du plan §4, *une ligne par
fichier partagé*. Le compte ne tenait pas : avec l'en-tête, une pull request native touche trois
points d'insertion — `binding.gyp`, la déclaration, l'appel ; sans lui, exactement trois aussi. Il ne
supprimait aucun site de conflit, il en déplaçait un dans un fichier de plus, et coûtait un écart
déclaré avec le plan. Il est retiré.

**Ce que ce fichier ne fait pas** : il n'implémente aucune fonction.

### Les quatre familles

Chacune expose une seule fonction publique, `RegisterX`, et garde tout le reste en statique.

| Fichier | Fonction exportée | API macOS | Phase |
|---|---|---|---|
| `foreground_app.mm` | `frontmostApp` | `NSWorkspace.frontmostApplication` | 3 |
| | `runningApps` | `NSWorkspace.runningApplications`, filtrées sur celles qui ont une interface | 3 |
| | `subscribeForegroundChange` | `NSWorkspaceDidActivateApplicationNotification` | 3 |
| `presentation_options.mm` | `applyPresentationOptions` | `NSApplication.presentationOptions`, **sans paramètre** | 7 |
| | `restorePresentationOptions` | la même, remise à `NSApplicationPresentationDefault` | 7 |
| `screen_sharing.mm` | `screenSharingLevel` | `CGSessionCopyCurrentDictionary`, `NSWorkspace` | 7 |
| `window_frames.mm` | `framesOfProcess` | `AXUIElementCopyAttributeValue`, position et taille seules | 8 |
| | `subscribeWindowFrames` | `AXObserver`, notifications de déplacement et de redimensionnement | 8 |

**`applyPresentationOptions` ne prend aucun paramètre**, et c'est ce qui ferme le refus du §4 : les
deux jeux d'options sont écrits en dur dans le `.mm`, et n'incluent jamais `DisableForceQuit`,
`DisableSessionTermination` ni `DisableHideApplication`. Un masque passé depuis JavaScript aurait
rendu le refus du port purement déclaratif.

**Ce que ces fichiers ne font pas** : aucun ne contient de logique produit — le cadrage §4 : « Toute
logique produit reste hors du natif. » Aucun ne lit un titre de fenêtre, un contenu, une frappe, ni
ne pilote une application tierce. `framesOfProcess` ne lit ni `kAXTitleAttribute` ni aucun attribut
de contenu, et le type de retour le rend de toute façon intransportable. Aucun ne conserve d'état
entre deux appels, hors les jetons d'abonnement qu'il doit relâcher.

---

## 9. La chaîne signée — `build/` — phases 4 et 5

Le contexte `distribution` ne reçoit **ni module sous `src/`, ni port** : son glossaire dit que
`notarization` appartient à la chaîne de construction et n'a aucun concept d'exécution.

**Deux chaînes, pas une.** L'entitlement et la clé d'usage sont deux mécanismes distincts, et
`BRIEF.md` §13 se tient sur les deux. Un droit s'inscrit dans les entitlements ; micro, caméra,
enregistrement d'écran et automatisation se déclarent, eux, dans l'`Info.plist`, par les clés
`NS*UsageDescription` — et ce sont **elles** qui produisent la boîte de dialogue que le brief
interdit. Une première version de ce document ne défendait que la première chaîne.

### `build/entitlements.mac.plist`

**Surface** : les droits demandés, et eux seuls.

```
com.apple.security.cs.allow-jit                          requis par le moteur JavaScript d'Electron
com.apple.security.cs.allow-unsigned-executable-memory   requis par le même moteur
```

**Ce que ce fichier ne fait pas** : aucun droit d'enregistrement, de micro, de caméra, de
localisation, de contacts, de réseau sortant en mode serveur, ni d'automatisation d'autres
applications. Aucun droit d'affaiblissement du runtime durci non plus —
`disable-library-validation`, `allow-dyld-environment-variables`, `cs.debugger`,
`disable-executable-page-protection`. La permission d'Accessibilité **n'est pas un droit** et ne s'y
déclare pas : elle s'accorde à l'exécution, par l'humain, dans les Réglages Système. Un lecteur qui
chercherait ici la ligne de l'Accessibilité ne la trouvera pas, et c'est correct.

### `build/electron-builder.yml`

**Surface** : `appId`, `productName`, cibles `dmg` et `zip` en `universal`, `hardenedRuntime: true`,
`gatekeeperAssess: false`, `entitlements` et `entitlementsInherit` pointant le fichier ci-dessus,
`extraFiles` embarquant le `.node`, `afterSign` **non renseigné**, et un bloc `extendInfo` qui
**retire les clés `NS*UsageDescription`** qu'`electron-builder` injecte par défaut — caméra et micro
comprises. Sans ce bloc, le paquet livré déclarerait à macOS des usages que le produit se promet de
ne jamais avoir, et passerait la vérification, la notarisation et les vingt-quatre cellules sans
qu'aucune ne le voie.

L'identité de signature vient du trousseau ou de `CSC_LINK` / `CSC_KEY_PASSWORD`, **jamais du
fichier**.

`afterSign` reste vide **par décision** : c'est le crochet par lequel `electron-builder` notarise
automatiquement. Le laisser vide fait de la notarisation un appel explicite — la phase 5 — et empêche
chaque construction locale de partir chez Apple.

**Ce que ce fichier ne fait pas** : il ne publie rien. Aucun bloc `publish`, aucun dépôt de mise à
jour — le cadrage §10 les place en phase 5 du cadrage §9, hors de ce plan.

### `build/verify-signature.sh`

**Surface** : `verify-signature.sh <chemin du .app>`. Code de sortie 0 si tout passe, non nul au
premier échec, en nommant l'assertion qui a échoué.

**Refus** — chacun est une assertion, et chacun protège une règle :

| Refus | La règle qu'il protège |
|---|---|
| L'ensemble des droits extraits du **binaire signé** — `codesign -d --entitlements :-` — n'est pas **exactement** `{allow-jit, allow-unsigned-executable-memory}` | `BRIEF.md` §13. Une **liste blanche** : une liste noire de quatre noms laissait passer tout ce qu'elle n'énumérait pas. Étendu aux binaires d'aide, qui portent `entitlementsInherit` |
| Le paquet porte une clé `NS*UsageDescription`, quelle qu'elle soit | La seconde chaîne : c'est elle qui produirait la boîte de dialogue que le brief interdit |
| Le `.node` n'est pas signé individuellement | La signature élément par élément est l'ordre que le cadrage §8.2 fixe |
| Le runtime durci est absent | Sans lui, la notarisation est refusée d'office |
| La signature n'est pas une identité **Developer ID Application** | Un paquet signé ad hoc satisfait tout le reste, et le cadrage §8.2 rend le certificat non négociable — `BRIEF.md` §18 fait dépendre l'autorisation d'Accessibilité de l'identité signée |
| L'horodatage sécurisé est absent | Sans lui la notarisation échoue, et le code 1 la classerait en défaut de chaîne sans dire lequel |
| Le paquet a été modifié après signature | Une signature qui ne couvre pas ce qui tourne ne prouve rien |
| Le paquet n'est pas universel | Une architecture manquante rend le produit inutilisable sur la moitié du parc |

**Ce que ce fichier ne fait pas** : il ne construit pas et ne corrige pas. Il constate ce
qu'`electron-builder` vient de produire ; un script qui re-signerait masquerait le défaut de
configuration qui l'a rendu nécessaire.

### `build/notarize.sh`

**Surface** :

```
notarize.sh <chemin du .app> --max-wait <secondes> --poll-every <secondes> --max-resubmissions <n>
```

Les trois options sont **obligatoires et sans défaut** : le plan §6 les laisse au fondateur, et le
script refuse de tourner sans elles. Une valeur par défaut choisie ici deviendrait la décision.

**Les codes de sortie** — c'est la surface publique qui compte, puisque le plan en fait le critère de
succès de la phase :

| Code | Verdict | Ce que le script fait |
|---|---|---|
| `0` | `Accepted` | Agrafe, **vérifie l'agrafage**, puis écrit la septième ligne du relevé |
| `1` | `Invalid` **de chaîne** | Rend le motif d'Apple, n'agrafe pas, n'écrit aucune ligne |
| `2` | `Invalid` **de politique** | Rend le motif d'Apple **tel quel**, n'écrit aucune ligne |
| `3` | `Rejected` | Le quatrième statut de `notarytool`, distinct d'`Invalid`. N'écrit aucune ligne |
| `4` | Soumission partie, aucun verdict dans `--max-wait` | Apple travaille encore. N'écrit aucune ligne |
| `5` | Soumission **jamais partie** — authentification, téléversement, réseau | On n'a pas demandé. N'écrit aucune ligne |
| `6` | Statut inconnu de cette liste | Sort en erreur, **jamais en 0** |

Les codes 4 et 5 étaient confondus. Sur la seule mesure sans repli, faire passer un échec
d'authentification local pour un délai d'Apple envoie chercher au mauvais endroit. Et un statut
inconnu qui sortirait en 0 par défaut inscrirait `Accepted` au relevé sur un refus — la pire issue
possible de ce script.

**Refus** :

| Refus | La règle qu'il protège |
|---|---|
| Une option manquante | Le plan §6 : aucune valeur plausible ne remplace une décision du fondateur |
| Un identifiant Apple ou un mot de passe passé **en argument** | Ils seraient lisibles par `ps` de tout utilisateur de la machine, et enregistrés dans l'historique du shell. Le script lit exclusivement un profil de trousseau ou les variables d'environnement de clé API, et refuse de démarrer sans. Aucune de ces valeurs n'est journalisée |
| Resoumettre au-delà de `--max-resubmissions`, ou resoumettre sur autre chose qu'un `Invalid` **de chaîne** | Resoumettre le même artefact après un refus de politique sera toujours refusé, et consomme le quota que ce refus protège |
| Agrafer sur un verdict autre qu'`Accepted` | Un agrafage sur un refus produit un paquet qui ment sur son état |
| Classer en politique un motif qui ne nomme ni les niveaux de fenêtre ni l'Accessibilité | Confondre les deux arrêterait le produit sur un défaut de construction |
| Écrire la septième ligne sans que `stapler validate` et l'évaluation Gatekeeper aient passé sur le paquet agrafé | Sans cela la ligne s'écrit sur une croyance, et l'échec silencieux de l'agrafage se découvre chez un utilisateur hors ligne |

**Ce que ce fichier ne fait pas** : il n'interprète pas le motif d'un refus de politique et ne propose
aucune correction. Le plan §6 laisse cette décision au fondateur.

---

## 10. Les sondes, le prototype et le pilote — `build/probe/` — phases 3, 7, 8, 9 et 10

### `build/probe/macos-version.mjs` et `build/probe/require-accessibility.mjs`

Deux gardes de sonde, une fonction chacune :

```
currentMacOSVersion(): string
requireAccessibility(port: AccessibilityPermissionPort, expected: AccessibilityStatus): void
```

Elles vivaient sur `Measurement`, qui portait alors trois responsabilités — une valeur, une garde qui
interroge un port, une lecture d'environnement. Une valeur qui sait interroger un port n'est plus une
valeur, et son test exigeait un double que rien ne justifiait.

`requireAccessibility` lève et fait sortir la sonde en code non nul : une mesure prise dans le mauvais
état n'est pas une mesure.

### `build/probe/measurement.mjs` — `Measurement`

```
static take({ measure, macOSVersion, verdict, accessibility, taken }): Measurement
static fromJSON(payload: object): Measurement
get measure(): number
get key(): string      // « <mesure>@<version de macOS> »
toJSON(): object
```

`taken` vaut `'pilot'` ou `'manual'`.

**`macOSVersion` est un paramètre, et `fromJSON` reprend exactement ce que `toJSON` émet.** C'était
une perte de données : `toJSON` émettait une version que `take` n'acceptait pas, et `consolidate.mjs`
lit les **quatre** fichiers de version depuis une seule machine — il aurait réattribué à chaque
cellule la version de la machine qui consolide, écrasant en silence la clé qui fonde tout le relevé,
`--assert-consistent` compris.

**Refus** :

| Refus | La règle qu'il protège |
|---|---|
| Une `macOSVersion` absente ou illisible | Une cellule sans version n'a pas de clé et ne se consolide pas |
| Un `verdict` absent | Une mesure qui a levé n'a pas de verdict, et une erreur n'en est pas un |
| Un `payload` dont les champs ne sont pas exactement ceux de `toJSON` | Une reconstitution partielle produit une cellule qui ment sur ce qu'elle a mesuré |

**Ce que ce fichier ne fait pas** : il n'écrit rien sur disque, et n'interroge aucun port.

### `build/probe/cell-store.mjs` — `CellStore`

Le seul écrivain de `build/probe/releve/<version-macos>.json`.

```
static forVersion(macOSVersion: string): CellStore
write(measurement: Measurement): void
```

**Refus** :

| Refus | La règle qu'il protège |
|---|---|
| Un second relevé **divergent** sur la même clé | Deux verdicts opposés sont un fait à comprendre, pas une valeur à choisir |
| Une écriture sans `Measurement` valide | Une cellule incomplète passerait `--assert-complete` sans rien prouver |

Un second relevé **concordant** est ignoré en silence : relancer une sonde doit être sans conséquence.
`read()` a été retirée — `consolidate.mjs` lit les quatre fichiers de version, pas celui de la machine
courante, et la règle de non-écrasement se sert d'une lecture interne à `write()`.

**Ce que ce fichier ne fait pas** : il ne consolide pas et ne juge pas la complétude.

### Les six sondes

Toutes se lancent sous Electron. Toutes rendent un code non nul, et n'écrivent **aucune cellule**,
quand l'état que la mesure exige n'est pas réuni.

| Fichier | Mesure | Options | Ce que la cellule porte |
|---|---|---|---|
| `foreground-app.mjs` | 4 | `--require-accessibility-not-granted`, `--emit-cell` | Un **booléen** : l'Accessibilité est-elle exigée pour lire l'application au premier plan. L'identité observée reste sur la sortie standard |
| `screen-sharing.mjs` | 2, 3 | `--require-no-screen-recording`, `--manual-fullscreen-witness`, `--emit-cell` | Un `ScreenSharingLevel`, `indistinguishable` compris |
| `menu-bar-hiding.mjs` | 6 | `--manual-fullscreen-witness`, `--emit-cell` | Si la barre de menus reste masquée face à un plein écran tiers |
| `accessibility-hot-grant.mjs` | 1 | `--witness-present`, `--emit-cell` | Si l'octroi se propage sans relancer l'application |
| `opaque-overlay-coverage.mjs` | 5 | `--report`, `--manual-fullscreen-witness`, `--emit-cell`, `--crash-after` | `couvert` ou `non couvert` |
| `assert-clean-system.mjs` | 7 | aucune | Aucun raccourci résiduel, Dock et barre de menus revenus, après une sortie anormale |

**La cellule de la mesure 4 porte un booléen**, jamais l'`AppIdentity` observée : le relevé est
versionné, et il enregistrerait sinon quelle application était au premier plan sur la machine de qui
a lancé la sonde. C'est la seule trace d'environnement qui entrait dans un fichier committé.

**La mesure 1 est manuelle**, et son option `--witness-present` est obligatoire. Elle ne se prend
qu'en appelant `requestGrant()` — le seul chemin qui sollicite l'humain — puis en attendant qu'il
aille cocher une case dans les Réglages Système. Un pilote qui l'exécuterait sans surveillance
ouvrirait une demande de permission à l'insu de qui est devant le Mac.

`assert-clean-system.mjs` **est devenue une mesure**, la septième non bloquante. Le §4 l'explique :
le cadrage §7 exige deux chemins de désenregistrement dont un qui survit à une sortie anormale, et
ce document refuse d'affirmer que le système relâche les enregistrements avec le processus. Il le
mesure.

**Ce que ces fichiers ne font pas** : aucune ne pilote une application tierce — le cadrage §5
l'interdit, et c'est pourquoi les mesures 1, 3, 5 et 6 sont manuelles. Aucune ne charge de ressource
distante. Aucune ne décide.

### `build/probe/opaque-overlay-coverage.mjs` et `build/probe/overlay.html`

Le prototype. Le seul fichier de ce document qui crée des fenêtres. Il s'appelait
`hardcore-overlay.mjs` : `hardcore` est une valeur de `Severity`, du contexte `cycle`, que le plan
interdit d'écrire ici — et que ce même §10 déclare plus bas ne pas simuler. Le nom dit
maintenant ce qui est mesuré.

**Quatre règles que ce fichier porte seul :**

| Règle | Ce qu'elle donne dans le code |
|---|---|
| L'échéance court sur l'`Instant` monotone | `SystemClock.monotonicNow()` à la pose, comparaison à chaque réveil. Aucun décompte, rien ne gèle. Trente secondes, constante nommée, sourcée sur le cadrage §9 |
| La pose multi-écran est **tout ou rien** | Une pose ratée sur un écran parmi N démonte tous les autres, écrit l'échec au compte rendu, et ne tient pas trente secondes une couverture partielle |
| Une seule instance | Le verrou de `main.js`. Une seconde instance ne pose rien et rend `AlreadyRunning` |
| Le rendu est durci — cadrage §11 | Chaque `BrowserWindow` : `nodeIntegration: false`, `contextIsolation: true`, `sandbox: true`, aucun préchargement, `webSecurity` jamais touché. `overlay.html` porte une CSP `default-src 'self'` |

La quatrième manquait. Un prototype est exactement l'endroit où `nodeIntegration: true` s'écrit
« juste pour la mesure », puis se recopie — et c'est le binaire qui part chez Apple en phase 5. La CSP
rend l'absence de ressource distante **vérifiable**, là où le document se contentait de la déclarer.

L'option `--seconds` a été retirée : les trois invocations du plan passent la même valeur, et une
option à valeur unique est un comportement que personne n'a testé et que le prochain croira supporté.

Le compte rendu de `--report` porte, **par écran** : la pose, le `WindowLevel` posé, le délai de pose
en millisecondes, et si l'overlay a été recouvert. Le délai est **relevé, jamais opposé** : les 200 ms
de `BRIEF.md` §12.2 visent l'overlay du produit, pas ce prototype.

**Ce que ces fichiers ne font pas** : aucun décompte de cycle, aucune sévérité, aucune sortie
d'urgence, aucun voile de Mode Simple. Le prototype prouve la faisabilité, il n'esquisse pas le
produit — et rien de ce qu'il pose ne survit aux trente secondes.

### `build/probe/run-all.mjs` — le pilote

**Surface** : `run-all.mjs`, sans option. Exécute les **deux mesures automatisables** — 2 et 4 — sur
la machine courante, écrit leurs cellules par `CellStore`, et rend un code non nul si l'une échoue.

**Refus** : il **n'exécute pas** les mesures 1, 3, 5, 6 et 7. Il les compte comme attendues et les
réclame si elles manquent. La mesure 1 a rejoint les manuelles : elle ouvre une boîte de dialogue.

### `build/probe/consolidate.mjs`

**Surface** :

```
consolidate.mjs                      rend le relevé consolidé sur la sortie standard
consolidate.mjs --assert-complete    code non nul tant qu'une cellule ou la ligne de notarisation manque
consolidate.mjs --assert-consistent  code non nul dès que deux verdicts divergent sur une même clé
```

**Sept mesures non bloquantes** sur macOS 13, 14, 15 et 26 — la septième étant celle
qu'`assert-clean-system.mjs` a fait naître. La notarisation reste une **ligne unique**, et ses champs
sont figés : date, version d'outil, verdict. L'identifiant de soumission, l'identifiant d'équipe et
le journal brut d'Apple — qui énumère l'arborescence du paquet — n'y entrent pas ; ils vont sur la
sortie standard de l'opérateur, jamais dans un fichier versionné.

**Ce que ce fichier ne fait pas** : il ne prend aucune mesure et n'applique aucun repli. Le repli de
`BRIEF.md` §12.3 s'écrit dans la cellule par la sonde qui a tranché par la négative.

### `build/probe/README.md`

Le mode opératoire des quatre mesures manuelles : ouvrir l'application témoin, la passer en plein
écran natif, lancer la sonde depuis un autre espace de travail — et, pour la mesure 1, rester devant
la machine pour accorder la permission. Et le piège du cadrage §8.2 : en développement, l'entrée de
permission d'Accessibilité porte le nom de l'environnement d'exécution, pas celui de Breeze.

**Ce que ce fichier ne fait pas** : il ne nomme pas l'application témoin — le plan §6 laisse ce choix
au fondateur.

---

## 11. Les décisions prises, et ce qu'elles écartent

**D1 — Le contexte `traduction-systeme` s'écrit `system-bridge`.** Le plan §6 laissait ce nom à la
conception.
*Écarte* : `native-bridge`, qui ferait porter au contexte le nom de l'un de ses composants ; `macos`,
qui nommerait un contexte d'après une technologie. À verser au glossaire dans la pull request 1.

**D2 — L'égalité de `Shortcut` porte sur `identity` seule ; les huit autres objets-valeur gardent
l'égalité structurelle.** Le document dit lui-même qu'on désigne un raccourci par son identité et que
sa combinaison peut changer.
*Écarte* : une égalité structurelle, qui déclarerait différents deux `Shortcut` que le port traite
comme le même enregistrement ; et une entité, qui contredirait la thèse du plan §1 — ce contexte n'en
a aucune, et le registre est détenu par macOS, pas par nous.

**D3 — Les quatre ports que la plateforme couvre sont implémentés en JavaScript au-dessus
d'Electron.** `AccessibilityPermissionPort`, `WindowLevelPort`, `DisplaysPort`,
`GlobalShortcutsPort`. Vérifié dans `node_modules/electron/electron.d.ts`, version 44.0.0.
*Écarte* : quatre fichiers Objective-C++ qui réimplémenteraient ce que la cible fournit, leur
compilation universelle, leur signature élément par élément et leur risque propre — pour un résultat
identique. Le §13 dit ce que ce retrait déplace dans les phases du plan.

**D4 — Les fabriques d'objets-valeur littérales prennent un objet unique et refusent toute clé
inconnue ; la validation vit dans le constructeur.** La première moitié tient le refus du plan sans
inspecter aucun contenu ; la seconde ferme `new Bounds({ width: -1 })`, que JavaScript laisse ouvert
tant que le refus vit dans la fabrique.
*Écarte* : des paramètres positionnels, qui laisseraient passer un champ surnuméraire sans rien dire.

**D5 — Un `WindowLevel` est un nom, pas un nombre.** C'est aussi la forme qu'Electron retient.
*Écarte* : un entier en JavaScript — une valeur non sourcée, comparable, arithmétique, et fausse au
premier changement d'API.

**D6 — `AccessibilityStatus` a deux valeurs, pas trois.** macOS rend un booléen, et TCC n'expose pas
la distinction entre « refusé » et « jamais demandé ».
*Écarte* : un `neverAsked` soit mort, soit dérivé d'un souvenir de processus — qui rouvrirait une
boîte de dialogue chez quelqu'un ayant déjà refusé. Si le produit en a besoin, c'est une mesure du
relevé, pas une fabrique.

**D7 — `requestGrant()` ne rend rien et ne refuse rien.** L'API rend l'état courant puis rend la main
pendant que l'humain va dans les Réglages.
*Écarte* : un type de retour qui promettrait un verdict inexistant, et `PermissionAlreadyGranted`,
qui protégeait une règle d'écran et rendait l'appel non idempotent.

**D8 — Aucun seuil laissé à la mesure par le plan §6 n'a de valeur par défaut.**
`NativeWindowFrames` refuse de se construire sans `silenceMs`, `resumeMs` ni `pollHz` ; `notarize.sh` refuse de
tourner sans ses trois options.
*Écarte* : un défaut « raisonnable », qui deviendrait un fait par inertie et ferait que la mesure ne
serait jamais prise.

**D9 — Un port ne revalide pas ce qu'un objet-valeur refuse déjà.** Les identifiants restent des
entiers nus ; leur refus vit dans `WindowFrame` et `Display`, une seule fois.
*Écarte* : quatre types marqués — `WindowId`, `ProcessId`, `DisplayId`, `ShortcutIdentity` — dans un
contexte `générique` que sa sorte invite à ne pas enrichir. Ce qu'on accepte en retour : deux entiers
restent interchangeables pour le lecteur comme pour le vérificateur.

**D10 — `SystemBridgeError` plus six refus, sans champ `code` et sans couches intermédiaires.**
*Écarte* : un `code` qu'aucun appelant figé ne lit et qui redoublait le nom de la classe ; et quatre
classes vides — `NotFound`, `Conflict`, `Forbidden`, `InvalidInput` — que six refus ne réclament pas
encore. Les appelants classent par `instanceof`.

**D11 — `InvalidValue` est réservée aux fabriques d'objets-valeur.** `ReservedShortcut` et
`DuplicateDisplay` sont nées de ce resserrage.
*Écarte* : une erreur générique levée par des règles sans rapport entre elles, que l'appelant
devrait départager en lisant une chaîne.

**D12 — `DisplaysPort` expose trois abonnements nommés plutôt qu'un `onChange` porteur d'une
nature.**
*Écarte* : un dixième objet-valeur `DisplayChangeKind`, et le `switch` que chaque abonné rouvrirait.

**D13 — `Bounds` est extrait de `WindowFrame` et de `Display`.** Les deux portent la même géométrie
et la même validation.
*Écarte* : quatre champs plats répétés dans deux objets-valeur, et deux validations à maintenir.
La justification précédente — « la limite de trois paramètres l'impose » — était fausse : D4 établit
déjà que la fabrique prend un objet littéral unique.

**D14 — Le harnais de contrat reçoit un sujet, `{ port, driver }`, et le pilote déclare ce qu'il ne
sait pas provoquer.**
*Écarte* : un `makePort` seul, qui rendait le harnais incapable d'observer quoi que ce soit qu'il
n'avait pas provoqué ; et des tests recopiés entre le double et le réel, qui divergeraient au premier
ajout.

**D15 — `NativeBridgeLoader` charge le `.node`, et `NativeBridge` reste le composant compilé.**
*Écarte* : la divergence avec le glossaire, qu'une première version constatait par une phrase au lieu
de la fermer.

**D16 — La règle de non-écrasement des cellules vit dans `CellStore`, un seul écrivain ; la clé, la
version et la garde de permission vivent ailleurs.**
*Écarte* : la même règle réimplémentée dans six sondes ; et un `Measurement` à trois responsabilités
dont une interrogeait un port.

**D17 — La chaîne signée se défend sur les deux mécanismes : entitlements **et** `Info.plist`.** Le
contrôle des droits est une liste blanche à égalité exacte, et aucune `NS*UsageDescription` ne survit
dans le paquet.
*Écarte* : une liste noire de quatre noms, plus courte que l'interdit qu'elle servait, et une défense
qui regardait le mauvais fichier — micro et caméra ne sont pas des droits.

**D18 — `afterSign` reste vide, et les secrets ne passent jamais en argument.**
*Écarte* : une soumission chez Apple à chaque construction locale ; et un mot de passe lisible par
`ps` de tout utilisateur de la machine.

**D19 — `index.html` est supprimé, `main.js` ne crée aucune fenêtre, et `index.js` ne réexporte
rien.**
*Écarte* : une fenêtre principale que `BRIEF.md` §13 interdit au lancement ; et un baril qui serait
une seconde source de vérité sur la surface du contexte — et la porte par laquelle un autre contexte
importerait `Instant`.

---

## 12. Ce que je choisis de ne pas créer

- **Aucun dossier `domain/` dans `system-bridge`.** Le plan §1 le démontre : aucun invariant
  transactionnel, aucun concept qui survive pour lui-même.
- **Aucune couche `interface/`, nulle part.** Pas de contrôleur, pas d'écran. Les seuls pilotes de ce
  périmètre sont les sondes, et elles vivent dans `build/`.
- **Aucun cas d'usage, aucune commande, aucun handler.** Rien ici n'orchestre une décision métier :
  une sonde appelle un port et constate.
- **Aucun objet-valeur de `system-bridge` n'est importé par un autre contexte borné.** `cycle`
  définira son propre concept d'échéance et le traduira, à sa couche anticorruption, depuis les
  primitifs que `ClockPort` rend — un `bigint` de nanosecondes, un `number` d'époque. Partager
  `Instant` ferait dépendre le cœur de métier d'un contexte `générique`, et une évolution motivée par
  une contrainte macOS irait frapper les invariants du cycle. C'est `index.js` qui l'aurait autorisé ;
  il ne réexporte rien.
- **Aucun module `src/` pour `distribution`.** Des scripts, et rien d'autre.
- **Aucun port de journalisation, de persistance ou de réseau.** Le journal circulaire, l'état du
  cycle et la vérification de mise à jour appartiennent aux phases 1 et 5 du cadrage §9.
- **Aucun registre de raccourcis sur disque.** Le second chemin de désenregistrement se mesure au
  lieu de s'affirmer ; un registre persistant ouvrirait une persistance que le plan §2 exclut, et
  mentirait après un redémarrage de la machine.
- **Aucun objet-valeur `Duration`.** Une durée est une différence d'instants, rendue en nombre.
- **Aucun identifiant typé.** D9 : la validation vit dans l'objet-valeur, et les ports ne la refont
  pas.
- **Aucune abstraction d'événement partagée** — pas d'`EventEmitter` maison, pas de bus, pas
  d'`Observable`. Trois ports exposent des abonnements ; une base commune n'unifierait qu'une
  signature, pas un comportement.
- **Aucun mapper, aucun DTO.** Rien ne persiste et rien ne sort sur le réseau.
- **Aucune fabrique ni conteneur d'injection.** `main.js` et les sondes construisent ce dont elles ont
  besoin, à la main, en deux lignes.
- **Aucun test de performance opposant un seuil.** Le plan §5 le dit : les délais de pose et de
  notification sont relevés, pas opposés.
- **Aucun fichier d'en-tête `breeze_bridge.h`.** Il ne supprimait aucun site de conflit.

---

## 13. Les écarts déclarés avec le plan

Quatre, et ils se disent ici plutôt que de se découvrir au build.

**É1 — Neuf objets-valeur, là où le plan §1 en annonce sept.** `Bounds` (D13) et `ScreenSharingLevel`
portent chacun une règle que les sept autres n'ont pas où loger. À verser au glossaire de
`traduction-systeme` dans la pull request qui les crée, avec `Instant` une fois la question de sa
forme close — ce qui est fait.

**É2 — Sept mesures non bloquantes, là où le plan §10 en compte six.** La septième est celle
qu'`assert-clean-system.mjs` fait naître : le cadrage §7 exige un chemin de désenregistrement qui
survive à une sortie anormale, et ce document refuse d'affirmer que le système le fournit. Vingt-huit
cellules au lieu de vingt-quatre, plus la ligne de notarisation.

**É3 — La mesure 1 est manuelle, là où le plan §10 la range parmi les trois automatisables.** Elle
ouvre une boîte de dialogue et attend un clic humain dans les Réglages Système. Le pilote en exécute
**deux**, 2 et 4.

**É4 — Quatre familles natives au lieu de huit (D3), et le contenu de quatre phases se déplace.** Le
découpage en dix pull requests, leur ordre de fusion et le chemin critique 1→2→3→4→5 sont
**inchangés** ; c'est ce que chaque phase livre qui bouge :

| Ce qui se déplace | Phase au plan | Phase après D3 |
|---|---|---|
| `binding.gyp`, `breeze_bridge.mm` — le squelette compilable | 3 | 3 |
| `foreground_app.mm`, son adaptateur, son harnais, la mesure 4 | 6 | **3** |
| `accessibility-permission.electron.js`, `window-level.electron.js` | 3 | **6** |
| `displays.electron.js` | 7 | **6** |
| `global-shortcuts.electron.js` | 9 | **6** |
| `presentation_options.mm`, `screen_sharing.mm`, mesures 2, 3 et 6 | 7 | 7 |
| `window_frames.mm`, mesure 1 | 8 | 8 |
| Le prototype, le verrou d'instance unique, la mesure 5 | 9 | 9 |
| Le pilote, la consolidation, le relevé | 10 | 10 |

La pull request 3 reste celle qui produit **au plus tôt un binaire portant les deux signaux
qu'Apple est censée juger**. L'argument qui commande tout l'ordre du plan repose sur une attente, et
elle se dit comme telle : on **attend** que le niveau d'empilement et l'usage de l'Accessibilité
soient dans le paquet dès qu'Electron y est, accompagnés du `.node` de la phase 3. Aucune source ne
l'établit — ni D3, qui ne cite `electron.d.ts` que pour l'existence des API, ni la documentation
d'Apple sur ce que la notarisation inspecte. C'est le verdict de la phase 5 qui tranchera, et c'est
sa raison d'être : si l'attente est fausse, c'est l'ordre des pull requests qu'il faudra reprendre,
pas le verdict. La pull request 6 ne dépend plus que de la 2 : n'étant plus native, elle part en
parallèle de la 3 au lieu d'attendre sa fusion.

**Ce que ces écarts ne changent pas** : le modèle du plan §1, les neuf ports du plan §2, le hors
périmètre du plan §5, et les décisions que le plan §6 laisse au fondateur.

---

## 14. La relecture

Sept agents ont relu ce document contre les chemins qu'il fige : `hexagonal`, `domain-purity`,
`security`, `ubiquitous-language`, `design-patterns`, `test-quality`, `simplicity`.
`clean-code-review-agent` et `idioms-review-agent` n'ont pas été lancés — ils jugent du code écrit,
et il n'y en a pas.

**Trente-trois constats, tous traités.** Un constat vu par deux agents indépendants est fusionné.
Les corrections sont dans les sections ci-dessus ; ce tableau dit ce qui a changé et pourquoi, pour
qu'une décision ne se rediscute pas au build.

### Deux contradictions internes

| Constat | Où c'est corrigé |
|---|---|
| §4 justifiait `pollingActive` par le besoin du harnais d'observer, §7 interdisait au harnais de piloter le double. Le harnais ne pouvait provoquer **aucun** événement : une douzaine d'assertions du plan §2 devenaient inobservables | §7, D14 — `makeSubject()` rend `{ port, driver }`. §4 — `pollingActive` est justifié par l'exploitation, pas par le test |
| `Object.freeze` est superficiel : `shortcut.modifiers.push('hyper')` faisait entrer un modificateur que la fabrique venait de refuser | §2, règle commune — tout champ composite est gelé séparément et rendu en copie |

### Quatre faits que le système ne peut pas tenir — vérifiés, pas supposés

| Constat | Où c'est corrigé |
|---|---|
| `requestGrant(): AccessibilityStatus` promettait un verdict que macOS ne rend pas au moment de l'appel (`domain-purity` et `security`, indépendamment) | §4, D7 — `requestGrant(): void` |
| `AccessibilityStatus.neverAsked()` n'avait aucune source : les deux API rendent un booléen | §2, D6 — deux valeurs |
| `purgeStale()` ne pouvait pas « fonctionner après une sortie anormale précédente » : un processus neuf ne détient rien | §4 — `unregisterAll()`, et le second chemin devient une mesure (É2) |
| `Bounds` refusait la surface nulle, que macOS expose bel et bien | §2 — refus retiré |

### Sept trous de sécurité

| Constat | Où c'est corrigé |
|---|---|
| Le contrôle des droits était une **liste noire** de quatre noms, plus courte que l'interdit qu'elle servait | §9 — égalité exacte sur le binaire signé, D17 |
| Micro et caméra ne sont pas des droits : ils se déclarent dans l'`Info.plist`, qu'`electron-builder` remplit par défaut et que rien ne vérifiait | §9 — `extendInfo` et un refus sur toute `NS*UsageDescription`, D17 |
| Aucun des deux scripts ne disait d'où venaient ses secrets ; `notarytool` accepte un mot de passe en argument, lisible par `ps` | §9, D18 |
| Le tableau des verdicts ignorait `Rejected`, et un statut inconnu serait sorti en 0 — inscrivant `Accepted` sur un refus | §9 — sept codes, dont un pour l'inconnu |
| L'agrafage était fait, jamais vérifié, et aucune assertion ne portait sur l'identité du signataire | §9 — `stapler validate` et Gatekeeper conditionnent le code 0 |
| Le durcissement du rendu exigé par le cadrage §11 n'était figé nulle part, dans le fichier même qui part chez Apple | §10 — quatrième règle du prototype, et une CSP dans `overlay.html` |
| `PresentationOptionsPort` déclarait « Refus : aucun » alors que `NSApplicationPresentationOptions` porte `DisableForceQuit` | §4 et §8 — aucun masque, options en dur, refus écrit |

### Trois fuites vers un fichier versionné

| Constat | Où c'est corrigé |
|---|---|
| `displayName` vient d'une application tierce, n'était borné en longueur par rien, et finit dans un journal borné | §2, règle commune 3 |
| Le pilote exécutait la mesure 1, seule mesure qui ouvre une boîte de dialogue — sans surveillance | §10, É3 |
| La cellule de la mesure 4 pouvait emporter l'application au premier plan de la machine du fondateur ; la septième ligne n'avait aucune forme figée | §10 — un booléen, et trois champs |

### Cinq incohérences de nommage et de frontière

| Constat | Où c'est corrigé |
|---|---|
| `NativeBridge` désignait le composant compilé dans le glossaire et son chargeur ici | §6, D15 — `NativeBridgeLoader` |
| `hardcore-overlay.mjs` empruntait `hardcore`, valeur de `Severity` du contexte `cycle` | §10 — `opaque-overlay-coverage.mjs` |
| `AlreadyRunning` ne se rattachait à aucun port, et le verrou d'instance est une primitive Electron | §1, §3 — déclarée dans `main.js` |
| `InvalidValue` était levée par des règles sans rapport entre elles : un appelant devait lire une chaîne | §3, D11 — `ReservedShortcut`, `DuplicateDisplay` |
| Les combinaisons réservées n'avaient pas de domicile : double et adaptateur les auraient réimplémentées | §4 — `reserved-shortcuts.js`, une constante gelée |

### Douze allègements et corrections de forme

Retirés : `breeze_bridge.h` (n'achetait aucun site de conflit), les réexports d'`index.js` (aucun
appelant, et la porte par laquelle `cycle` aurait importé `Instant`), `SystemBridgeError.code`,
`PermissionAlreadyGranted`, `PresentationOptionsPort.applied`, `CellStore.read`,
`WindowLevelDouble.appliedLevels`, `AccessibilityPermissionDouble.revokeSilently`,
`ClockDouble.setMonotonic`, `Instant.equals`, `WindowLevel.normal`, l'option `--seconds`, le script
`start`, et la publicité de `Measurement.macOSVersion`.

Corrigés : le compte des objets-valeur, écrit trois fois avec trois valeurs différentes sur une
surface que le build lit telle quelle ; la règle commune du §2, qui ne valait que pour cinq des neuf ;
le refus manquant sur `Display.bounds` ; l'ordre des `modifiers`, qui rendait inégaux deux raccourcis
identiques ; `SystemBridgeError.name` ; la symétrie `toJSON`/`fromJSON` de `Measurement`, dont
l'asymétrie aurait fait écraser la clé qui fonde tout le relevé ; les trois responsabilités de
`Measurement` ; la phrase manquante sur `indistinguishable` ; le tableau des familles natives, qui ne
liait pas chaque API à sa fonction ; et la justification de D13, qui était fausse.

### Ce que la relecture a confirmé

- L'absence de `domain/`, de cas d'usage et de couche `interface/` — la démonstration du plan §1
  tient, et le refus de créer un `domain/` vide est le bon geste.
- Neuf ports à deux implémentations chacun : le remplacement est leur raison d'être, pas une
  abstraction sortie avant son problème.
- La règle « toute clé inconnue est refusée » tient `BRIEF.md` §13 par le type, pas par la
  discipline — et vaut mieux qu'une liste noire.
- Aucune signature figée ne permet de lire un titre, un contenu ou une frappe : les types de retour
  ferment la frontière, et `binding.gyp` ne lie aucun cadre de capture.
- Le déterminisme : aucune lecture d'horloge dans les objets-valeur.
- D8 — aucun défaut sur un seuil que le plan §6 laisse à la mesure.

### Ce qui restait ouvert, et qui l'est resté quelques heures

Quatre points ont été posés au fondateur, chacun avec sa conséquence sur le code. Ses réponses sont
D3 (les quatre ports passent à Electron, le découpage du plan est conservé), D2 (`Shortcut` reste un
objet-valeur, égalité sur `identity`), D9 (entiers nus, refus dans l'objet-valeur seul) et D10 (base
plus six refus, sans `code`).

Une cinquième est apparue à la relecture de validation, et a été tranchée de même : **la cadence de
l'interrogation de secours**, que le plan §6 renvoyait à la conception et que celle-ci avait omise.
Elle devient `pollHz`, troisième paramètre exigé de `NativeWindowFrames`, sans valeur par défaut —
D8 s'étend à elle. *Écarte* : les 4 Hz du cadrage §8.5 écrits en dur, chiffre non sourcé qui serait
devenu le comportement de Breeze sans mesure ; et une huitième mesure du relevé, qui aurait allongé
une phase de Reconnaissance déjà à dix pull requests.

**Aucune question ouverte ne subsiste.**
