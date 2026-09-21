<!--
type: resolution-de-revue
titre: Breeze — résolution de la revue du domaine (palier fondations)
cree_le: 2026-09-21T00:00:00+0000
mis_a_jour_le: 2026-09-21T00:00:00+0000
branche: main
statut: applique
-->

# Résolution de la revue du domaine — palier « fondations »

Le noyau `breeze-domain` a été revu (agent Fable). Verdict : noyau nominal correct,
mergeable après M1, M3 et une décision consignée sur M2. Voici ce qui a été fait, et ce
qui est **sciemment reporté** aux paliers où le code qui le porte arrive.

## Appliqué dans ce palier

- **M1 — tick en retard.** Deux assertions nouvelles vérifient qu'un tick postérieur à
  l'échéance n'étire ni le préavis ni la pause : les échéances sont ancrées sur l'échéance
  franchie, jamais sur `now` (`tests/cycle_progression.rs`).
- **M3 — format.** `cargo fmt --check` est vert ; c'est une porte du projet.
- **Anti-débordement.** `Instant::checked_plus` remplace l'avance saturante dans les
  transitions : un instant qui saturerait arrête l'avance au lieu de boucler.
- **Divers** : `#![forbid(unsafe_code)]`, `ClockJump` ré-exporté, couverture
  `PauseOutOfBounds` et minute hors journée.

## M2 — contrat consigné (pas un bug nominal)

`tick(now)` crédite **une pause servie (`BreakOutcome::Served`) par pause pleinement
écoulée** entre l'ancien état et `now`. Rejouer un grand saut (p. ex. 5 cycles) crédite
donc 5 pauses — c'est vérifié par un test compté (`replaying_whole_cycles_credits_one_served_break_each`).

**Le contrat qui borne ce comportement** : un grand écart de temps (veille/réveil, session
verrouillée, processus relancé) **ne doit jamais** arriver au domaine comme un `tick` brut.
C'est le chemin d'**absence du §10.4**, que `crates/breeze-domain/src/session.rs`
(`AbsenceVerdict`) portera au palier suivant : il traduit un écart en `ValidatedByAbsence`
ou en reprise d'échéance, avant que la boucle nominale ne crédite quoi que ce soit. Le
planificateur (`breeze-app`) tient ce contrat : il tick à la seconde, et route les écarts
par l'absence. Tant que `session.rs` n'existe pas, aucun appelant réel ne fournit de grand
saut — le test le fait pour documenter le comportement, pas pour l'autoriser en production.

## Reporté aux paliers porteurs (nits D4, sans dette cachée)

- `BreakOutcome::Interrupted { unserved: Duration }` deviendra `Minutes` quand la dette de
  posture (§9.2) câblera le ledger — l'unité y prend son sens.
- `BreakMode::Degraded` recevra ses `reasons` quand `capabilities.rs` (le relevé
  `EnforcementCapabilities`) arrivera avec le premier adaptateur.
- Le placement de `BreakMode` (aujourd'hui dans `cycle/`) sera revu avec `capabilities.rs`.
- `Countdown::{Frozen, Due}`, `ValidatedByAbsence` et le breaker restent **déclarés,
  non câblés** : ils entrent avec la suspension, l'absence et le coupe-circuit. Aucun code
  mort trompeur : ce sont des variantes d'API publiques, documentées comme extensions.

## Palier 2 (ports + app + null) — revue Fable, dette consignée

Verdict : mergeable après M1 (idempotence prouvée *pendant* la pause) et M2 (exécuteur
réconcilié par `BTreeMap<DisplayId, SurfaceId>` — un écran branché en cours de pause est
couvert au poll suivant) — **tous deux appliqués**, plus deux triviaux (`match` fusionné,
dépendance `breeze-domain` retirée du null). La dette déclarée, à câbler à son palier :

- **Persistance avant overlay** (D4 §10.2) : `PersistencePort` non encore câblé ; le
  `Scheduler` persistera l'instantané **avant** `reconcile` quand le port arrivera. Place à
  réserver dans `poll`.
- **Capacités non projetées** : `OverlaySurfacesPort::capability()`, `BreakMode` (toujours
  `Nominal`) et le relevé `EnforcementCapabilities` ne remontent pas encore dans
  `CycleSnapshot` — la promesse D2 « dire à froid » se câble avec `capabilities.rs` et le
  premier adaptateur. Jusque-là, aucun écran ne prétend un mode qu'il n'a pas.
- **Instantané mince** : pas de sévérité en `Working`, pas de `remaining` (l'UI le calcule
  depuis `deadline` et l'horloge). À enrichir au palier UI.
- **Placement d'`OverlayCapability`** (concept proche domaine, posé dans `ports`) : accepté
  tant que le domaine ne le consomme pas ; à revoir avec `capabilities.rs`.
- **`DisplayId(u32)`** ne porte pas encore l'identifiant natif (`HMONITOR`, `CGDirectDisplayID`,
  sortie XRandR) : l'adaptateur fera la correspondance ; le domaine ne connaît qu'un id opaque.
- **Coupe-circuit** : le swap vers `NullOverlay` devra réinitialiser l'état de l'exécuteur
  pour ne pas désynchroniser la carte `covered`.
