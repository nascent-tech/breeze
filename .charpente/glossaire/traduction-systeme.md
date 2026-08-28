---
type: glossaire
titre: Traduction des appels système
slug: traduction-systeme
cree_le: 2026-08-28T18:07:49+0000
mis_a_jour_le: 2026-08-28T22:00:45+0000
branche: feat/system-bridge-ports-and-doubles
statut: brouillon
---

**Sorte** : générique — décidé le 2026-08-28.

Ce contexte rend accessible ce que macOS expose déjà ; il ne décide de rien. Le cadrage
`.charpente/cadrage/2026-08-28-poser-une-contrainte-inevitable-sur-macos.md` (§4) en fixe la
surface.

## Module natif (Native bridge)

Le composant compilé qui traduit les appels système que l'environnement d'exécution n'expose pas —
application au premier plan, cadres de fenêtres, permission d'Accessibilité, niveaux d'empilement,
écrans.

**Ce que ce n'est pas** : un lieu de logique produit — il traduit, il ne décide d'aucun état du
cycle.
**À ne pas confondre avec** : le préchargement de l'environnement d'exécution, qui n'expose que la
communication inter-processus.
**En code** : `NativeBridge`.

## Cadre de fenêtre (Window frame)

La position et la taille d'une fenêtre d'une autre application.

**Ce que ce n'est pas** : son contenu ni son titre — le brief l'interdit (`BRIEF.md` §13).
**À ne pas confondre avec** : le cadre d'un overlay de Breeze, qui lui appartient.
**En code** : `WindowFrame`.

## Nom de code du contexte (System bridge)

Le nom que le code donne à ce contexte borné, d'après le module natif qui en porte la traduction.

**Ce que ce n'est pas** : un second contexte — c'est le même, sous le nom que le code emploie.
**À ne pas confondre avec** : le module natif lui-même, qui n'en est qu'un adaptateur.
**En code** : `src/modules/system-bridge/`.

## Rectangle (Bounds)

L'origine et la taille d'une surface rectangulaire, sur l'écran ou sur une fenêtre.

**Ce que ce n'est pas** : une géométrie — il ne coupe, ne contient et ne convertit aucun repère.
**À ne pas confondre avec** : le cadre de fenêtre, qui ajoute la fenêtre et le processus qui la
portent.
**En code** : `Bounds`.

## Niveau de partage d'écran (Screen sharing level)

Ce que la session laisse voir à un tiers : rien, un partage, une présentation — ou le constat que
macOS ne les distingue pas sans permission d'enregistrement.

**Ce que ce n'est pas** : l'identité de qui partage, ni la destination du flux.
**À ne pas confondre avec** : l'absence de partage — un niveau illisible n'est pas un niveau nul.
**En code** : `ScreenSharingLevel`.

## Point de temps (Instant)

Un point de temps, tenu sous ses deux formes : la forme monotone, qui ne recule pas, et la forme
murale, que l'humain lit.

**Ce que ce n'est pas** : une durée — une durée est la différence de deux points de temps. Ce n'est
pas non plus un point qui échoit : il n'avance ni ne recule de lui-même.
**À ne pas confondre avec** : ce que le contexte `cycle` définit pour lui-même à partir des
primitifs que ce contexte rend.
**En code** : `Instant`.
