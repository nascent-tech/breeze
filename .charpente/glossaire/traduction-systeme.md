---
type: glossaire
titre: Traduction des appels système
slug: traduction-systeme
cree_le: 2026-08-28T18:07:49+0000
mis_a_jour_le: 2026-08-28T18:22:45+0000
branche: develop
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
