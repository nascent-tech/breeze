# Journal des modifications

Toutes les modifications notables de Breeze sont consignées ici. Le format suit
[Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et le projet suit le
[versionnage sémantique](https://semver.org/lang/fr/).

## [Non publié]

### Ajouté

- Statistiques réelles sur 30 jours (pauses servies, interrompues, sorties d'urgence, minutes de pause par jour), tirées d'un journal des pauses persisté.
- Gel du décompte après 3 minutes sans saisie sur macOS.
- Panneau en popover sous l'icône de la barre de menus, sans icône dans le Dock ; ⌥⌘B l'ouvre et le ferme.
- Réglages : recherche d'applications, confirmation avant réinitialisation, aide dépliable, version affichée, « Revoir l'accueil » et « Signaler un problème ».

### Corrigé

- La plage horaire et les jours actifs sont réellement appliqués, y compris après une mise en veille ; une suspension jusqu'au lendemain reprend à l'heure prévue.
- Le compteur « Aujourd'hui » compte les pauses du jour, et non depuis le lancement.
- Un décompte gelé n'affiche plus 0:00.
- Le rythme choisi à l'accueil s'applique dès le premier cycle ; rouvrir l'accueil n'écrase plus les réglages.
- Les fenêtres Réglages et Accueil sont natives et se déplacent ; tous leurs contrôles répondent.
- La sévérité se règle depuis les Réglages ; les heures de la plage sont conservées quand on la désactive.
- Un échec d'enregistrement est signalé au lieu d'être passé sous silence ; la réinitialisation et les migrations de la base sont atomiques.
- Les écrans ne promettent plus ce qui n'existe pas (vérification des mises à jour, notifications, voile fenêtre par fenêtre).
