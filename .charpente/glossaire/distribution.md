---
type: glossaire
titre: Distribution
slug: distribution
cree_le: 2026-08-28T18:07:49+0000
mis_a_jour_le: 2026-08-28T18:22:45+0000
branche: develop
statut: brouillon
---

**Sorte** : support — décidé le 2026-08-28.

Indispensable, sans valeur propre : signature, notarisation, mise à jour. Le cadrage
`.charpente/cadrage/2026-08-28-poser-une-contrainte-inevitable-sur-macos.md` (§10) en fixe les
règles.

## Notarisation (Notarization)

Le verdict d'Apple sur un binaire signé, sans lequel macOS bloque l'application.

**Ce que ce n'est pas** : la signature — elle la suppose et s'y ajoute.
**À ne pas confondre avec** : la distribution par le Mac App Store, incompatible avec le produit
(`BRIEF.md` §5).
**En code** : `notarization`, chaîne de construction seulement — aucun concept d'exécution.

## Vérification de mise à jour (Update check)

La seule sortie réseau du produit : version installée et adresse réseau vers le dépôt de
publication, désactivable (`BRIEF.md` §10.6).

**Ce que ce n'est pas** : une télémétrie — rien d'autre ne sort, jamais.
**À ne pas confondre avec** : l'installation, qui n'a lieu qu'au prochain lancement.
**En code** : `UpdateCheck`.
