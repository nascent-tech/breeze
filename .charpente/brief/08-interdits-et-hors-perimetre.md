## 13. Ce qui est interdit

Chaque ligne est **un état qui doit être impossible**, pas un message d'erreur à afficher.

**Sur la contrainte :**

- Aucune commande n'écourte, ne retarde, ni ne modifie une pause en cours, hors le geste unique du
  Mode Hardcore (§8.5) — et ce geste ne change jamais son prix, seulement la façon d'y arriver.
- Une pause en cours qui se termine autrement que par l'écoulement complet de son décompte est
  toujours comptée interrompue ; il n'existe pas de troisième issue.
- Un levier refusé par le panneau n'est jamais accordé par un raccourci ou un menu contextuel.
- Une pause due n'est jamais coupée par la fermeture de la plage horaire ni par un changement d'heure
  système, qu'elle ait commencé ou qu'elle attende encore son signal (§9.3, §10.1).
- La sévérité, le rythme et le statut d'une application ne changent jamais, dans quelque sens que ce
  soit, pendant [PRÉAVIS], [PAUSE ACTIVE] ni [RETOUR] ; hors de ces états, leur affaiblissement attend
  de toute façon le cycle suivant — immédiatement s'ils renforcent pour la sévérité et le statut,
  toujours au cycle suivant pour le rythme (§10.3, §15 décision 9).
- Une suspension n'est jamais possible pendant [PRÉAVIS], [PAUSE ACTIVE] ni [RETOUR] (§10.1).
- **L'indice de profondeur (§9.3) ne retarde jamais rien d'autre que le début du préavis.** Il ne
  retarde jamais un préavis déjà commencé, ni une pause. C'est la seule exception nommée qui retarde
  une pause au bénéfice de l'utilisateur ; élargir ce qu'elle touche la transformerait en une
  deuxième.
- Aucune frappe clavier n'est lue pour son contenu, à quelque titre que ce soit. Seul l'**instant** de
  la dernière action clavier ou souris entre dans le calcul de l'inactivité et de la profondeur
  (§10.1, §9.3) — jamais ce qu'elle a produit.
- **Aucun hook clavier global bas niveau, sur aucun OS** — pas même pour neutraliser Alt-Tab, la
  touche Windows ou Super pendant une pause Hardcore. Un tel hook lit les frappes, ce que la ligne
  précédente interdit déjà, et il change le profil de sécurité du binaire aux yeux du système et des
  antivirus : Breeze préfère un Hardcore qui revient devant en moins d'une demi-seconde (§5.3, §12.2)
  à un Hardcore qui lit le clavier.
- **Aucune exécution privilégiée** : pas de service Windows, pas de binaire setuid, pas de daemon
  root, pas de demande d'élévation — pour « tenir mieux » ou pour quoi que ce soit d'autre. Breeze
  reste une application utilisateur ordinaire, sur les trois OS : celui qui subit la contrainte est
  celui qui l'a posée, et rien ne doit survivre à sa décision de quitter (§3, §5).

**Sur l'état :**

- Le cycle n'entre jamais dans un état sans sortie : ni jours tous inactifs, ni plage horaire
  dégénérée.
- Deux instances de Breeze ne posent jamais deux overlays.
- Aucun overlay n'est posé au démarrage de l'application.
- Aucun overlay ne survit à la fin de la pause qui l'a posé.

**Sur ce qui appartient à l'utilisateur :**

- Aucune application tierce n'est masquée, quittée ni suspendue.
- Aucune donnée ne quitte la machine, hors la vérification de mise à jour déclarée et désactivable.
- Aucun titre de fenêtre, aucun contenu d'écran, aucun contenu de frappe clavier n'est lu ni
  enregistré, à aucun moment, y compris dans le journal de diagnostic.
- Aucune permission n'est demandée pour une fonctionnalité qui ne s'en sert pas.
- **En Mode Simple**, l'utilisateur n'est jamais empêché d'atteindre les réglages du système ni le
  moniteur d'activité : les deux sont sur la liste de sécurité (§10.6), non éditable. Une application
  de santé qu'il a marquée `épargnée` — la seule façon dont Breeze la reconnaît (§10.6) — n'est pas non
  plus recouverte. **En Mode Hardcore, la liste de sécurité ne s'applique pas** (§10.6) : rien n'y est
  distingué, et la seule voie vers quoi que ce soit d'autre est le geste de sortie du §8.5 — c'est ce
  geste, et lui seul, qui porte la contrepartie en Hardcore.
- Le geste de sortie du Mode Hardcore n'est jamais caché, jamais rationné, jamais payant.
- Éteindre la machine, forcer à quitter, changer de session restent toujours possibles — sur les
  trois OS, Breeze ne touche à aucune des sorties que le système garantit (§5).
- Une capacité manquante ne lève jamais une contrainte : elle accélère le décompte ou réduit la
  couverture, et Breeze le dit à froid (§3, §10.5). Aucune session, aucun compositeur, aucune
  permission retirée ne produit une pause qu'on peut écourter.

---

## 14. Hors périmètre

Chaque ligne dit **pourquoi**, et si c'est une contrainte subie ou une décision produit — la seconde
se révise, la première non.

| Ce qu'on ne fait pas | Pourquoi, et à quelle condition ça se révise |
|---|---|
| **Distribution par le Mac App Store** | Contrainte subie (§5.5). Ne se révise pas tant que le bac à sable reste incompatible avec l'Accessibilité et les niveaux de fenêtre nécessaires |
| **Désinstallation complète de l'autorisation d'Accessibilité (macOS)** | Contrainte subie (§5.2). Breeze affiche la marche à suivre. Ne se révise pas |
| **Un Mode Hardcore qui bloque Alt-Tab, Windows, Super ou Ctrl-Alt-Suppr** | Décision produit, et un interdit du §13 : ça exige un hook clavier ou une exécution privilégiée. Ne se révise pas — un Hardcore qui revient devant est ce que Breeze promet sur Windows et Linux (§5.3, §17) |
| **Le Mode Simple par fenêtre sur Wayland** | Contrainte subie (§5.2) : aucun compositeur n'expose la géométrie des fenêtres d'autrui. Se révise le jour où un protocole public le fait ; jusque-là, voile plein écran par moniteur |
| **Le Mode Hardcore garanti et la détection du premier plan sur GNOME Wayland** | Contrainte subie (§5.1, §5.3). GNOME est couvert en mode déclaré dégradé, jamais exclu. Se révise avec les protocoles de GNOME, pas avec Breeze |
| **XWayland comme mode de fonctionnement** | Décision produit : une fenêtre X11 ne passe jamais devant les fenêtres Wayland natives, et Breeze y serait dégradé sans le savoir (§12.1). Ne se révise pas |
| **Tout bouton de délai, de snoozing ou d'exception activable par l'utilisateur** | Décision produit, et c'est l'identité même de cette version du brief : donner raison à celle qui décide à froid suppose de ne plus jamais laisser la parole à celle qui négocie à chaud (§2). Ne se révise pas sans redéfinir le produit — ce qui justifierait un nouveau brief, pas un ajustement de celui-ci |
| **Synchronisation entre appareils, compte, serveur** | Décision produit, contredit le refus de collecte (§10.6). Ne se révise qu'avec ce refus, donc pas au lancement |
| **Mode équipe, classement social, partage de statistiques** | Décision produit, même raison. Celui qui subit la contrainte reste celui qui l'a posée, jamais un tiers (§3) |
| **Blocage de sites, de domaines ou de contenus** | Décision produit. Demanderait de lire ce que l'utilisateur consulte, ce que le §13 interdit. Ne se révise pas |
| **Exercices guidés, contenu audio ou vidéo, conseils de santé** | Décision produit. Élargit le périmètre sans renforcer la promesse, et fait de Breeze un coach — ce qu'il n'est pas (§3). Ne se révise pas |
| **Profils de réglages multiples et nommés** | Décision produit. Un seul jeu de réglages couvre le besoin du premier public. Se révise si la bêta montre que les mêmes utilisateurs reconfigurent leur rythme plusieurs fois par semaine — la mesure vit au §12.3 |
| **Statistiques au-delà de trente jours** | Décision produit. Se révise si la dette de posture (§9.2) montre qu'une tendance longue aiderait à régler ses seuils — la mesure vit au §12.3 |
