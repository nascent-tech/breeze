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
- Mode Simple : pendant la pause, chaque fenêtre d'une application bloquée porte son propre voile, qui la suit quand elle bouge ou change de taille et disparaît quand elle se ferme ; les applications épargnées restent utilisables, y compris placées devant une fenêtre voilée, et le voile paraît aussi sur une application en plein écran. Si les fenêtres ne peuvent pas être suivies, ou au-delà de 24 fenêtres bloquées, la pause se tient en voile plein écran.
- Une application Ignorée au premier plan gèle le décompte de travail, qui reprend dès qu'une autre application passe devant.
- Les applications de la liste de sécurité (Réglages Système, Moniteur d'activité, Terminal, Finder, Trousseau d'accès, fenêtre d'ouverture de session, VoiceOver) apparaissent dans la liste, toujours épargnées et verrouillées.
- Mode Hardcore sur macOS : pendant la pause, chaque écran est couvert au plus haut niveau de fenêtre qu'une application ordinaire puisse prendre, barre de menus et Dock compris. Au premier clic sur l'écran de pause, Breeze prend le clavier et désactive ⌘Tab et « Masquer Breeze » — macOS ne permet à aucune application de prendre le clavier d'elle-même, et l'écran de pause le dit tant que ce clic n'a pas eu lieu. La fenêtre « Forcer à quitter » reste utilisable et ⌘Q quitte toujours Breeze. À la fin de la pause, la présentation revient par défaut. Une application en plein écran natif peut encore passer devant — macOS ne garantit rien dans ce cas —, et l'accueil comme les réglages le disent.

### Modifié

- L'écran de pause prend l'apparence de macOS : typographie système, voile clair ou sombre selon l'apparence du Mac en Mode Simple, fond toujours sombre en Mode Hardcore. L'anneau montre le temps qui reste, comme le minuteur de l'app Horloge.
- Relâcher le statut d'une application (épargner, ignorer, débloquer) ne vaut qu'au cycle suivant ; bloquer ou cesser d'ignorer vaut tout de suite. Pendant le préavis, la pause et le retour, les statuts ne changent pas.

### Retiré

- Breeze ne demande plus la permission Accessibilité : le suivi des fenêtres s'en passe.

### Corrigé

- La plage horaire et les jours actifs sont réellement appliqués, y compris après une mise en veille ; une suspension jusqu'au lendemain reprend à l'heure prévue.
- Le compteur « Aujourd'hui » compte les pauses du jour, et non depuis le lancement.
- Un décompte gelé n'affiche plus 0:00.
- Le rythme choisi à l'accueil s'applique dès le premier cycle ; rouvrir l'accueil n'écrase plus les réglages.
- Les fenêtres Réglages et Accueil sont natives et se déplacent ; tous leurs contrôles répondent.
- La sévérité se règle depuis les Réglages ; les heures de la plage sont conservées quand on la désactive.
- Un échec d'enregistrement est signalé au lieu d'être passé sous silence ; la réinitialisation et les migrations de la base sont atomiques.
- Les écrans ne promettent plus ce qui n'existe pas (vérification des mises à jour, notifications).
- Breeze ne démarre plus deux fois : une seconde ouverture montre le panneau de l'instance déjà lancée au lieu de faire tourner un second cycle.
