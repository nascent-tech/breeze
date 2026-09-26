---
type: decision
titre: Breeze — le voile fenêtre par fenêtre sans permission d'Accessibilité (macOS)
cree_le: 2026-09-26T12:05:00+0000
mis_a_jour_le: 2026-09-26T12:05:00+0000
branche: feat/window-veil
statut: applique
---

# Le voile fenêtre par fenêtre sans Accessibilité

## Contexte

Le brief (§5.2, §8.4, §8.7, §10.5) supposait que suivre le cadre des fenêtres d'autrui exigeait, sur
macOS, la permission Accessibilité : demande à l'onboarding, badge de réparation, voile plein écran
quand elle manque, marche à suivre à la désinstallation pour l'entrée qu'une app ne peut pas retirer.

## Décision

Breeze ne demande pas l'Accessibilité. Les cadres viennent de
`CGWindowListCopyWindowInfo(onScreenOnly | excludeDesktopElements)`, qui rend sans permission, pour
chaque fenêtre : bornes, PID propriétaire, couche et numéro — jamais le titre ni le contenu (le titre
exigerait l'Enregistrement de l'écran, que Breeze ne lit pas). Le PID devient une identité par
`NSRunningApplication` ; l'app au premier plan vient de `NSWorkspace.frontmostApplication`.

Chaque voile est une fenêtre de niveau normal, rangée juste au-dessus de sa fenêtre cible par
`NSWindow orderWindow:relativeTo:` avec ce numéro CGWindow : une app épargnée placée devant reste
devant et utilisable, et une app épargnée amenée par-dessus passe devant le voile, ce que le brief
assume (§8.4, « contournable »).

## Conséquences

- Le port `AccessibilityPermissionPort`, son adaptateur macOS, la dépendance
  `macos-accessibility-client` et les commandes `request_accessibility` / `accessibility_status` sont
  retirés ; l'onboarding ne demande plus rien.
- Le repli plein écran du §10.5 reste, pour une liste de fenêtres illisible (échec du système, ou
  plateforme sans adaptateur) et au-delà de 24 fenêtres bloquées.
- Plus d'entrée d'Accessibilité à laisser derrière soi à la désinstallation.
- Les passages du brief qui exigeaient l'Accessibilité (02 §5.2, 04 §8.1, §8.4, §8.6, §8.7,
  06 §10.5) sont mis à jour. Ceux de 07, 08, 09 et 10 (hypothèses, notarisation, Mac App Store)
  restent à relire.

## Vérification faite

- Le 2026-09-26, sur la machine de développement : la liste des fenêtres à l'écran rend, pour
  chaque fenêtre de couche 0, bornes, PID et numéro (4 fenêtres sur 4). Le processus de vérification
  était, lui, déjà autorisé en Accessibilité : la lecture sans permission s'appuie sur la
  documentation d'Apple (seul `kCGWindowName` est soumis à une permission, l'Enregistrement de
  l'écran) et sur le relevé du contrat du voile.
- `cargo test` couvre le suivi des voiles (pose, déplacement, retrait, repli) avec des doubles.
- Reste à valider à l'œil : un voile réel sous une app épargnée placée devant, et sur l'espace d'une
  app en plein écran natif.
