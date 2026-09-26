# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

Interface HTML/CSS/JS vanilla rendue dans les webviews de Tauri v2 (WKWebView sur macOS, WebView2 sur Windows, WebKitGTK sur Linux). Application de bureau logée dans la barre de menus (macOS) ou la zone de notification (Windows, Linux).

## Users

Un travailleur du savoir — développeuse, designer, analyste, rédacteur — seul devant sa machine plusieurs heures par jour, sans administrateur au-dessus de lui. Il a déjà essayé des rappels de pause et les a désactivés, parce qu'ils restaient négociables.

La même personne existe en deux états : **à froid** (le matin, dans les réglages) elle choisit un rythme ; **à chaud** (minute 47 d'un bloc de travail) elle veut continuer. Le produit donne toute la parole à la première et aucune à la seconde.

## Product Purpose

Breeze **impose la pause** : il compte le temps de travail, annonce la pause une minute avant, puis rend l'écran — ou les seules applications désignées — inutilisable pendant la durée de la pause. Aucun bouton ne l'écourte, ne la retarde ni ne l'annule.

Promesse : **la pause n'est pas une proposition, c'est une échéance.**

## Positioning

L'overlay se copie en un week-end ; ce qui ne se copie pas, c'est le refus durable de tout bouton « juste cette fois » (report, exception, mode occupé). Breeze détient l'absence de bouton. Il ne prétend jamais à l'infaillibilité : le système d'exploitation reste l'arbitre, et ce qui manque sur chaque machine est dit à froid, jamais découvert en pause.

## Operating Context

- **Panneau de barre de menus** : consulté en un coup d'œil pendant le travail (décompte, suspension, sévérité, rythme).
- **Overlay de pause** : plein écran, multi-écran. Mode **Simple** = voile fenêtre par fenêtre sur les apps bloquées ; Mode **Hardcore** = tout l'écran, barre de menus et Dock couverts, seule sortie = maintien d'Échap 10 s, au prix d'une dette de posture.
- **Réglages** : décidés à froid — rythme, jours, plage horaire, sévérité, apps épargnées, statistiques, confidentialité.
- **Onboarding** : premier lancement, 5 écrans ; dit en une phrase ce que la machine détectée laisse passer.

## Capabilities and Constraints

- Aucune sortie réseau (hors vérification de mise à jour déclarée et désactivable) : polices système, aucune ressource distante.
- CSP par nonce dans les webviews : pas de `style=""` inline hors `style-src-attr` autorisé.
- Tri-plateforme : macOS 13+, Windows 10 (1809)+/11, Linux X11 et Wayland (KDE ≥ 5.20, wlroots) ; GNOME Wayland déclaré dégradé.
- Vocabulaire produit : cycle, pause, sévérité (Simple / Hardcore), rythme (travail/pause), suspension, dette de posture, apps épargnées/bloquées, retour.
- Source de vérité produit : `.charpente/brief/` et le domaine implémenté (`crates/breeze-domain`). L'ancien artefact de design dépeint des mécanismes retirés (report, budget, exceptions) et ne fait plus autorité.

## Brand Commitments

- Nom : Breeze. Langue de l'interface : français, tutoiement.
- Ton : direct, calme, honnête sur ses limites ; pas de coach, pas de conseil santé, pas d'exercice guidé.
- **Direction visuelle choisie (2026-09-26) : utilitaire natif macOS**, exécuté au niveau de finition des apps Apple (Réglages Système, Horloge, Centre de contrôle). Même look macOS sur les trois OS ; police système de chaque OS (SF Pro n'est pas redistribuable hors Apple).
- À éviter : le générique (préférences système sans soin, template SaaS) et le décoratif (l'effet qui prend le pas sur la lecture du décompte et des réglages).

## Evidence on Hand

- Brief complet : `.charpente/brief/`.
- Aucune donnée utilisateur, aucun témoignage : rien à fabriquer.

## Product Principles

1. **Aucune négociation à chaud** : nulle part l'interface n'offre d'écourter, reporter ou annuler une pause due.
2. **Tout se décide à froid** : les réglages sont l'endroit où l'utilisateur a le pouvoir.
3. **L'aveu avant la promesse** : ce que la machine ne tient pas est dit à l'avance.
4. **Rien ne quitte la machine.**
5. **Breeze dit de se lever et regarde ailleurs** : il interrompt, il ne commente pas.

## Accessibility & Inclusion

Contrastes WCAG AA calculés ; respect de `prefers-reduced-motion` ; navigation clavier complète dans le panneau et les réglages ; l'overlay reste lisible sur tout écran, y compris à distance (l'utilisateur est invité à se lever).
