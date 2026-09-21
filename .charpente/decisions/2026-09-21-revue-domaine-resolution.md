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

## Palier 3 (UI verre + enveloppe Tauri) — revue Fable, dette consignée

Verdict initial : 1 Critique + 5 Majeurs — **tous corrigés** :
- **C1** `withGlobalTauri: true` ajouté (sans quoi `window.__TAURI__` n'était jamais injecté et
  le binaire tournait sur le mock) ; le mock est **sorti de l'app** vers `ui/preview.html` seul.
- **M1** un **ticker de fond** (`spawn_ticker`, 250 ms) fait avancer le cycle ; `get_snapshot`
  devient une **lecture pure** (plus de `poll` mutant à la demande, donc plus de grand saut brut).
- **M2** `break_in_secs` calculé côté hôte (préavis compris) ; **M3** la sévérité réglée est
  toujours dans le DTO ; **M4** Instrument Sans **embarquée** (`ui/fonts/`, plus de Google Fonts —
  §10.6) ; **M5** panneau responsive (plus de rognage).
- Mineurs : CSP réelle posée, verrou `Mutex` tolérant au poison, leviers masqués hors `Working`,
  deux tests sur `to_dto`.

Dette nommée, à câbler à son palier :
- **Horloge macOS** : `std::time::Instant` ne compte pas la veille → à remplacer par `ClockPort`
  (adaptateur), avec le verdict d'absence §10.4. Le ticker de fond est provisoire.
- **Overlays réels** : encore `NullOverlay` — arrivent avec l'adaptateur macOS (fenêtres de niveau
  `CGShieldingWindowLevel`, `NSVisualEffectView` pour le vrai flou ; la transparence actuelle est du
  verre CSS). La notarisation reste le point bloquant à mesurer.
- **CSP** : durcir `style-src 'unsafe-inline'` en sortant les styles inline vers des classes.
- **Capability** : `core:default` est large ; les commandes locales n'en ont pas besoin — à réduire.
- **`ChangeSeverity`/`Suspend`** : le segmented et « Suspendre » sont affichés mais pas encore
  câblés à des commandes (le domaine ne porte pas encore la suspension).

## Palier 4 (suspension + commandes) — revue Fable, dette consignée

Verdict : corrections avant merge (0 Critique, 3 Majeurs). **Appliqués** :
- **M1** — la règle du sens §10.3 sur la sévérité (brief 06:213 « Hardcore→Simple : cycle suivant ;
  Simple→Hardcore : immédiat ») : `pending_severity` diffère l'affaiblissement au cycle suivant,
  appliqué dans `enter_next_work`. Test `weakening_the_severity_waits_for_the_next_cycle`.
- **M3 (partiel)** — renouvellement de suspension (§10.1 « une seconde suspension remplace
  l'échéance ») : `suspend` depuis `Suspended` remplace le terme. Test `suspending_again_replaces_the_term`.
- Mineurs — leviers offerts en [TRAVAIL] seul (plus en [INACTIF]), refus affiché à l'utilisateur
  (table française des codes, §D1), description de capability à jour.

Dette **décidée** (reportée à son palier porteur, tranché par moi) :
- **M2 — durées de suspension typées + « demain 6 h » côté hôte** : exige l'**horloge murale
  locale**, qui n'existe pas encore (`ClockPort` est déféré au palier macOS, et Fable note que
  `resume_at` monotone dérive déjà avec la veille — dette héritée du palier 3). Décision : la
  commande garde `minutes` **bornée côté hôte à ≤ 1 jour** (invariant de sécurité §13 « aucune
  suspension sans terme » tenu), et l'UI n'émet que 15 / 60 / aube. Le refactor en
  `SuspensionTerm { FifteenMinutes, OneHour, UntilDawn }` + calcul d'aube local arrive avec `ClockPort`.
- **M3 — verdict de dépassement / chaîne** : `InterruptionDoor::SuspensionOverrun` et le crédit
  d'une pause avalée par une suspension entrent avec le **palier absence/ledger** (`session.rs`,
  §10.4), avec le reste des sorts de pause.
- Nits reportés : refus en `enum Refusal: Serialize` plutôt que `String` ; `quit` via
  `QuitRequested` quand la persistance arrivera ; rôles/clavier ARIA sur les `seg-item`.

## Palier 6 (persistance SQLite) — revue Fable, corrections appliquées

Verdict : corrections avant merge (0 Critique, 4 Majeurs de perte de données). **Appliqués** :
- **M1** — `RunEvent::Exit` persiste l'état à la fermeture (Cmd-Q, menu, fermeture de fenêtre) ;
  plus de perte de ce qui n'a pas été écrit à une commande.
- **M2** — la **sévérité choisie** (`chosen_severity` = `pending_severity.unwrap_or(courante)`) est
  persistée : au relancement, l'affaiblissement accepté (Hardcore→Simple) prend effet, comme un
  nouveau cycle (§10.3).
- **M3** — le ticker persiste **au changement de `served_breaks`** ; combiné à M1, les statistiques
  survivent au relancement en usage nominal.
- **M4** — `PersistencePort` renvoie `Result<_, PersistenceError>` ; `load` distingue « pas de
  ligne » d'une base illisible ; `save`/lecture journalisées côté hôte ; `busy_timeout(5 s)`.
- **M5/m1/m3** — repli `in_memory()` + journal au lieu de `expect` sur le disque ; `user_version`
  posé pour les migrations à venir ; gets SQLite typés (`u16`/`u32` bornés, plus de `as`).

Dette **décidée**, reportée au palier ClockPort/échéance :
- **Échéance murale (autorité §10.4)** non persistée — exige l'horloge murale locale.
- **Plage horaire / jours actifs** non round-trippés (pas de `ChangeRhythm`, défauts `None`/tous
  actifs) — reviennent avec l'écran de réglages du rythme.
- Nit accepté : `lib.rs` de l'hôte à 235 lignes (> 200) — à découper en `commands.rs`/`host.rs` au
  prochain passage sur l'hôte.

## Palier 7 (gel d'inactivité §10.1) — revue Fable, corrections appliquées

Verdict : corrections avant merge (0 Critique, 1 Majeur). **Appliqués** :
- **M1** — `freeze_if_idle` ne gèle plus si l'échéance est déjà franchie (`now.has_reached(deadline)`) :
  une pause due n'est jamais transformée en phase gelée (§10.1 « ce gel ne joue que tant que le
  décompte n'a pas atteint zéro »). Tests `idleness_never_freezes_once_the_work_deadline_is_reached`,
  `only_a_running_work_phase_freezes_not_the_notice`.
- **m2** — le restant gelé est ancré sur `last_activity + IDLE_FREEZE` (l'instant où le seuil est
  franchi), pas sur un `now` de tick en retard.
- **m3** — dégel via `checked_plus`, cohérent avec `resume`.

Contrat **consigné** (m1), à honorer par l'appelant :
- **`freeze_if_idle` suppose une machine éveillée et déverrouillée.** Le domaine n'a pas ce signal ;
  le futur `SessionSignalsPort` ne fournira l'inactivité *que* session éveillée, et fournira
  `observe_activity(wake_at)` au réveil, faute de quoi une phase serait gelée par une veille — ce que
  §10.1 réserve au §10.4. Le **second seuil** d'inactivité et la veille/verrouillage restent au palier
  `session.rs`.
- **Non câblé, décidé** : `Scheduler` n'appelle ni `freeze_if_idle` ni `observe_activity` ; l'app est
  inchangée. Le câblage arrive avec `SessionSignalsPort` (adaptateurs OS). `CycleSnapshot` projettera
  alors `Frozen` avec son restant (dette m4).
## Palier 8 (coupe-circuit §10.6) — revue Fable, corrections appliquées

Verdict initial : pas mergeable (2 Majeurs). **Appliqués** :
- **M1** — `Breaker` horodate en **`WallClock`** (survit au relancement) et non plus en `Instant`
  monotone (qui repart à zéro à chaque processus, donc `len` ne dépassait jamais 1 et les 24 h ne
  traversaient pas un redémarrage). `restore(crashes, armed)` + `crashes()` rendent l'état
  **persistable**. `WallClock::saturating_duration_since` ajouté.
- **M2** — `record_crash` appelle `tick(at)` en tête : une chute isolée après 24 h désarme d'abord.
- Mineurs — désarmement mesuré depuis `max(crashes)` (l'ordre mural n'est pas garanti) ; élagage à la
  fenêtre de 5 min (borne mieux le `Vec`) ; `PartialEq/Eq` pour le round-trip ; tests de bord (fenêtre
  stricte, 4ᵉ chute qui repousse le désarmement, round-trip `restore`).
- **Seuil tranché (Fable)** : **≥ 3** (« la troisième chute arme », §12.2/§10.6) — inchangé.

Dette de **câblage** consignée (le breaker reste non câblé) : drapeau *running* persisté (posé au
démarrage, effacé à la fermeture propre → un démarrage retrouvant le drapeau = `record_crash`) ;
`tick` à chaque `poll` ; `is_armed()` lu **à l'entrée en pause seulement** ; swap vers `NullOverlay`
avec réinitialisation de la carte `covered` de l'exécuteur (dette palier 2) ; priorité sur la dette
de posture ; pause servie sans overlay comptée `Served` ; champ dans le DTO ; « Réarmer » → `reset()`.

## Palier 9 (réglage du rythme §10.3) — revue Fable, corrections appliquées

Verdict initial : sain, **1 Majeur** bloquant. **Appliqués** :
- **Majeur** — `change_rhythm` acceptait un changement pendant PRÉAVIS / PAUSE ACTIVE / RETOUR
  (brief 06:221-222 + décision 9 : « impossibles à changer », sans exception). Désormais il retourne
  `Result<(), CommandError>` avec la garde `break_is_due()` → `BreakDue`, miroir de `change_severity`.
  Propagé dans `Scheduler::change_rhythm` et la commande hôte `set_rhythm` (refus `break-due` affiché).
  Test `a_rhythm_change_is_refused_while_a_break_is_due` ajouté.
- **Mineur** — `set_rhythm` repart du `configured_rhythm()` courant pour `schedule`/`active_days` au
  lieu de forcer `None`/`everyday()` : pas de perte silencieuse le jour où la portée horaire entrera.
- **Mineur** — code de refus `invalid-rhythm` traduit côté UI.
- **Nit** — `change_rhythm` normalise (`(rhythm != self.rhythm).then_some(...)`), comme la sévérité.
- **Tests** — pause en cours **pinnée** sur l'ancien rythme (deadline vérifiée, pas juste la variante) ;
  `configured == rhythm == 25/5` après consommation du pending ; DTO avec `active != configured`
  (l'anneau suit l'actif, le réglage montre le configuré) — la raison d'être des deux paramètres de
  `to_dto`.

**Règle du sens — tranché (Fable, RAS)** : le rythme se change **sans notion de sens**, dans les deux
sens au cycle suivant (brief 06:215-219, 04:37-38 ; l'application de la règle du sens à la durée est
une option **écartée**, décision 7). Ne pas distinguer durcissement/affaiblissement.

**Sécurité — RAS** : bornes `Rhythm::new` (travail 5..=180, pause 1..=60, pause≤travail), entrée `u16`
(désérialisation Tauri rejette hors-type avant le code), boucle `tick` bornée. Pas de cycle de 0 min,
pas de DoS. La commande ne touche ni réseau ni chemin.

Dette **consignée** :
- **Persistance du configuré** (RAS aujourd'hui) : `save_state` persiste `configured_rhythm()` — juste
  tant qu'un redémarrage crée un cycle neuf (`Cycle::start(..., Instant::EPOCH)`), qui *est* un cycle
  suivant. Deviendra faux avec la **persistance d'échéance** (§10.1, décision 14) : il faudra persister
  actif **et** pending séparément, sinon un pending s'appliquerait en plein cycle repris.
- **`chosen_severity` vs `configured_rhythm`** : deux mots pour « choisi, appliqué au prochain cycle ».
  Uniformiser sur `configured_*` quand la sévérité sera retouchée (nit, non bloquant).
- **`machine.rs` > 200 lignes** (258 avec ce palier) : à découper (dette déjà notée).
