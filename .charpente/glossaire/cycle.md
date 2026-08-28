---
type: glossaire
titre: Cycle
slug: cycle
cree_le: 2026-08-28T18:07:10+0000
mis_a_jour_le: 2026-08-28T18:22:45+0000
branche: develop
statut: brouillon
---

**Sorte** : cœur de métier — décidé le 2026-08-28.

Le vocabulaire produit est fixé par `BRIEF.md` §4 ; ce glossaire ajoute ce que le brief ne porte
pas : le nom en code de chaque terme, décidé une fois. Les définitions ci-dessous renvoient au brief
et ne le réécrivent pas. Le cadrage
`.charpente/cadrage/2026-08-28-poser-une-contrainte-inevitable-sur-macos.md` (§3) présente ce
contexte et ses deux voisins.

## Cycle (Cycle)

Une phase de travail suivie d'une phase de pause — l'unité de temps de Breeze (`BRIEF.md` §10.1).

**Ce que ce n'est pas** : une session de l'application ; le cycle survit au processus.
**À ne pas confondre avec** : la « journée » des statistiques, qui va de minuit à minuit.
**En code** : `Cycle`, agrégat pressenti du contexte — la conception tranchera sa frontière.

## Pause (Break)

La phase contrainte du cycle (`BRIEF.md` §10.1).

**Ce que ce n'est pas** : une suspension de Breeze, ni le préavis.
**À ne pas confondre avec** : « pause » au sens anglais de mise en pause d'un processus.
**En code** : `Break` — jamais `Pause` : `suspend`/`resume` sont réservés à la suspension de Breeze.

## Sévérité (Severity)

Le niveau de contrainte de la pause : Simple ou Hardcore (`BRIEF.md` §8.5, §8.6).

**Ce que ce n'est pas** : un réglage d'apparence ; elle se fige dès qu'une pause est due.
**À ne pas confondre avec** : le statut d'une application, qui ne joue qu'en Mode Simple.
**En code** : `Severity`, valeurs `simple` et `hardcore`.

## Préavis (Notice)

La minute qui précède la pause (`BRIEF.md` §10.1).

**Ce que ce n'est pas** : la pause, ni une notification système — la bannière est propre à Breeze.
**À ne pas confondre avec** : « warning », réservé aux journaux techniques.
**En code** : `Notice` — état `notice`, fait `NoticeStarted` : le fait est l'entrée en préavis,
jamais l'affichage de la bannière, qui n'en est qu'une conséquence.

## Pause due (Due break)

Une pause dont l'échéance est atteinte et qui n'a pas encore été servie (`BRIEF.md` §10.2).

**Ce que ce n'est pas** : une pause en cours.
**À ne pas confondre avec** : l'état `[INHIBÉ]`, qui est une pause due qu'une condition repousse.
**En code** : fait `BreakBecameDue` — au passé, l'échéance a été atteinte ; « due » seul décrirait
une phase. Jamais un prédicat exposé pour décider hors de l'agrégat.

## Inhibition (Inhibition)

Une condition extérieure qui repousse une pause due — visio, partage d'écran, présentation, plage
d'exception (`BRIEF.md` §10.2).

**Ce que ce n'est pas** : un report — l'utilisateur ne l'a pas demandée.
**À ne pas confondre avec** : la suspension, geste explicite de l'utilisateur.
**En code** : `Inhibition`, état `inhibited`.

## Report (Postponement)

Un délai de cinq minutes demandé par l'utilisateur depuis le préavis (`BRIEF.md` §10.2).

**Ce que ce n'est pas** : une annulation, ni une inhibition.
**À ne pas confondre avec** : « snooze », le mot des réveils — refusé, il connote la notification
qu'on balaie.
**En code** : verbe `postpone`, fait `BreakPostponed`.

## Budget de report (Postpone budget)

Le total de minutes dont un cycle dispose pour repousser sa pause due, tous mécanismes confondus —
15 minutes, constante produit (`BRIEF.md` §10.2, §12.2).

**Ce que ce n'est pas** : un quota de reports — le quota borne la fréquence, le budget borne le
total.
**À ne pas confondre avec** : la dette de posture (`BRIEF.md` §9.2), hors lancement.
**En code** : `PostponeBudget`, objet-valeur pressenti.

## Statut d'application (App status)

Ce que la pause fait à une application, et ce que son usage fait au décompte (`BRIEF.md` §8.3).

**Ce que ce n'est pas** : l'appartenance à la liste des déclencheurs — deux listes distinctes.
**À ne pas confondre avec** : la liste de sécurité, en dur et non éditable.
**En code** : `AppStatus`, valeurs `blocked`, `spared`, `ignored`.

## Déclencheur (Trigger)

Une application dont la présence au premier plan fait avancer le décompte de travail
(`BRIEF.md` §8.4).

**Ce que ce n'est pas** : un statut — une application peut être déclencheuse et bloquée. Ce n'est
pas non plus l'interrupteur : le déclencheur est l'appartenance d'une application à la liste ;
l'interrupteur (`smartTrigger`) arme le mécanisme et porte sa propre garde — il ne s'arme pas sur
une liste vide, et la liste ne se vide pas tant qu'il est armé (`BRIEF.md` §8.4).
**À ne pas confondre avec** : la plage d'exception, qui repousse une pause au lieu de gouverner le
décompte.
**En code** : `Trigger` pour l'appartenance, `smartTrigger` pour l'interrupteur.

## Sortie d'urgence (Emergency exit)

Le maintien d'Échap pendant dix secondes, qui lève un overlay Hardcore (`BRIEF.md` §8.6).

**Ce que ce n'est pas** : un raccourci de confort, ni un levier rationné.
**À ne pas confondre avec** : « Terminer la pause », propre au Mode Simple et payé au budget.
**En code** : `EmergencyExit`, fait `EmergencyExitUsed`.

## Coupe-circuit (Circuit breaker)

La désactivation automatique des overlays après des chutes répétées du processus
(`BRIEF.md` §10.6).

**Ce que ce n'est pas** : une commande utilisateur.
**À ne pas confondre avec** : la sortie d'urgence, geste de l'utilisateur pendant une pause.
**En code** : `CircuitBreaker`.

## Absence (Absence)

Une période sans aucun signal d'activité, mesurée par le système (`BRIEF.md` §10.4).

**Ce que ce n'est pas** : l'application quittée — un processus mort ne mesure rien.
**À ne pas confondre avec** : la veille, qui est un état de la machine, pas de l'utilisateur.
**En code** : `Absence`.

## Suspension (Suspension)

L'arrêt volontaire et borné de Breeze — 15 minutes, 1 heure, ou jusqu'au lendemain 6 h
(`BRIEF.md` §10.1, §12.2).

**Ce que ce n'est pas** : un arrêt du cycle — la phase gelée reprend où elle en était.
**À ne pas confondre avec** : l'état `[INACTIF]`, produit par la plage horaire ou le jour inactif.
**En code** : `Suspension`, verbes `suspend` / `resume`.

## Plage horaire (Active hours)

La fenêtre quotidienne pendant laquelle le cycle vit, minuit franchissable (`BRIEF.md` §8.2).

**Ce que ce n'est pas** : la plage d'exception, qui repousse une pause en la débitant.
**À ne pas confondre avec** : les jours actifs, réglage distinct évalué en continu.
**En code** : `ActiveHours`.

## Jours actifs (Active days)

Les jours de la semaine où le cycle vit — l'une des deux causes de l'état `inactive`, évaluée en
continu comme la plage horaire (`BRIEF.md` §8.2, §10.1).

**Ce que ce n'est pas** : un calendrier — aucune date, seulement les sept jours de la semaine.
**À ne pas confondre avec** : la plage horaire, l'autre cause de `inactive` ; une plage ouverte un
jour actif reste ouverte au-delà de minuit.
**En code** : `ActiveDays`.

## Overlay (Overlay)

La fenêtre que Breeze pose devant ce qu'il contraint — voile par fenêtre en Mode Simple, fond
opaque par écran en Mode Hardcore. Il a son propre cycle de vie : posé dans le délai du
`BRIEF.md` §12.2, jamais posé au lancement, retiré à la fin de la pause qui l'a posé, désactivable
par le coupe-circuit (`BRIEF.md` §10.4, §10.6, §13).

**Ce que ce n'est pas** : le décompte ni la bannière de préavis — une pause peut courir sans aucun
overlay (permission manquante, coupe-circuit) et compte quand même.
**À ne pas confondre avec** : le voile, qui n'est que l'apparence de l'overlay du Mode Simple
(`DESIGN.md`, recette « Voile »).
**En code** : `Overlay`.

## Les états du cycle (Cycle state)

Les états de `BRIEF.md` §10.1, et leur nom en code : `[INACTIF]` → `inactive`, `[ARMÉ]` → `armed`,
`[TRAVAIL]` → `working`, `[PRÉAVIS]` → `notice`, `[INHIBÉ]` → `inhibited`, `[PAUSE ACTIVE]` →
`onBreak`, `[RETOUR]` → `return` — légal en JavaScript comme clé et comme valeur ; si un outillage
le refuse, le repli décidé est `returning`. Les trois états transversaux : `[SUSPENDU]` → `suspended`,
`[VEILLE]` → `asleep` ; le gel d'inactivité n'est pas un état mais un drapeau de la phase de
travail.

**Ce que ce n'est pas** : des écrans — plusieurs états partagent une même surface.
**À ne pas confondre avec** : les états d'interaction des composants (`DESIGN.md`).
**En code** : `CycleState`.
