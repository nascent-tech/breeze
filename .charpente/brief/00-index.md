---
type: brief
titre: Breeze
cree_le: 2026-08-31T17:47:54+0000
mis_a_jour_le: 2026-09-21T16:56:39+0000
branche: main
statut: a_valider
---

# Breeze — brief

Ce document énonce **ce qu'il faut construire et pourquoi**. Il ne dit ni comment : ni architecture,
ni découpage, ni fichier, ni technologie. Le fondateur, la designer et la développeuse le lisent tous
et doivent le comprendre pareil. Les contraintes techniques de chaque système d'exploitation — macOS,
Windows, Linux en session X11 ou Wayland — y figurent uniquement par **ce qu'elles imposent au
produit** et par **ce qui est déclaré dégradé** (§5) ; la manière de les affronter vit dans le cadrage
technique, pas ici.

**Tout y est tranché.** Aucune section ne renvoie une question à plus tard. Là où une valeur n'est pas
connue — un seuil, un signal qu'un système ou un compositeur expose ou non —, ce n'est pas une décision
qui manque : c'est une **mesure**, et le §12.3 dit qui la porte, sous quelle forme, et ce qui se passe
tant qu'elle manque.

**Premier public : le travailleur du savoir seul devant sa machine — macOS, Windows ou Linux —
personnelle ou professionnelle, sans administrateur au-dessus de lui.** Ce choix fixe tout le reste :
pas de compte, pas de serveur, pas de déploiement en flotte, aucune donnée qui quitte la machine hors
une vérification de mise à jour déclarée et désactivable, et une contrainte que l'utilisateur s'impose
à lui-même plutôt qu'une politique qu'on lui applique.

Cette version reprend la génération sans négociation du 2026-08-31 et y ajoute la couverture des
trois systèmes, selon le registre de décisions du 2026-09-21 (D1 à D4). Le principe qui rend cet
ajout possible sans changer le produit est posé au §3 : **la promesse ne dépend d'aucune détection —
une capacité perdue accélère le décompte ou réduit la couverture, jamais elle ne lève une
contrainte.**

---

## Sommaire

Dix parties, chacune couvrant une ou plusieurs sections numérotées — la numérotation des sections
traverse les parties.

| Partie | Sections |
|---|---|
| [01 — Le produit et son public](01-le-produit-et-son-public.md) | 1. Le projet en une page · 2. À qui ça s'adresse, et sur quel terrain · 3. Ce que Breeze fait, ce qu'il n'est pas, et où il tourne |
| [02 — Vocabulaire, cadre et acteurs](02-vocabulaire-cadre-et-acteurs.md) | 4. Les mots de Breeze · 5. Ce que ta machine impose — par mécanisme, puis par plateforme · 6. Les acteurs, et ce que chacun a le droit de faire |
| [03 — Les besoins](03-les-besoins.md) | 7. Les besoins, séparés des solutions |
| [04 — Les fonctionnalités](04-les-fonctionnalites.md) | 8. Le produit, fonctionnalité par fonctionnalité |
| [05 — Les fonctionnalités décisives](05-les-fonctionnalites-decisives.md) | 9. Les fonctionnalités décisives |
| [06 — Les règles transverses](06-les-regles-transverses.md) | 10. Les règles transverses — dont 10.5, ce que Breeze fait quand une capacité manque |
| [07 — Le paiement et les limites](07-paiement-et-limites.md) | 11. Comment le produit se paie · 12. Les limites, et où vivent les chiffres — versions par OS, mesures par OS |
| [08 — Interdits et hors périmètre](08-interdits-et-hors-perimetre.md) | 13. Ce qui est interdit · 14. Hors périmètre |
| [09 — Les décisions](09-les-decisions.md) | 15. Les décisions, et ce que chacune écarte |
| [10 — Après le lancement, hypothèses, sources](10-suite-hypotheses-sources.md) | 16. Ce qui vient après le lancement · 17. Les hypothèses risquées · 18. Sources |

**Ce qui est déjà tranché**, avant la première partie :

- **Public et terrain** (§2) : un individu seul, sans administrateur au-dessus de lui.
- **Plateformes** (§3, §5, §12.1) : macOS 13+, Windows 10 (1809)+/11, Linux X11, Linux Wayland sur
  KDE Plasma ≥ 5.20 et wlroots (Sway, Hyprland, river) ; GNOME Wayland en mode déclaré dégradé. Ce
  que chaque machine ne garantit pas est dit **à froid**, en une phrase par session (§5.7).
- **Mécanisme de contrainte** (§8) : Breeze recouvre l'écran ou les applications visées — pas de
  verrouillage clavier/souris, pas de hook clavier, pas d'exécution privilégiée, pas de simple rappel.
- **Négociation** (§10) : aucune. Une pause due tombe sans bouton pour la retarder ni l'écourter. La
  seule issue est celle que le système d'exploitation garantit déjà — quitter Breeze (§5) —, et le
  Mode Hardcore lui ouvre une seconde porte vers cette même issue, au même prix, jamais moins cher
  (§8.5) : les deux gestes sont comptés, jamais gratuits.
- **Vie privée** (§10, §11) : aucune donnée ne quitte la machine, hors une vérification de mise à
  jour déclarée et désactivable.
