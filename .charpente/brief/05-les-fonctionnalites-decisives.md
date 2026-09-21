## 9. Les fonctionnalités décisives

Trois fonctionnalités que personne n'a demandées, arbitrées avec l'humain avant d'entrer ici — et
retravaillées pour tenir sans aucune négociation, ce que deux d'entre elles supposaient dans la
version précédente de ce brief. **Aucune n'entre au lancement** (§16). **Rien de ce qui suit ne joue
au lancement** : la machine d'états du §10 ne porte ces trois mécanismes qu'à titre d'extension
future — §10.1 le dit explicitement.

### 9.1 Le bouclier de calendrier

**Ce qu'elle apporte.** Un overlay Hardcore qui tombe pendant une réunion client détruit la confiance
en une seule occurrence. Sans négociation pour rattraper le coup dans l'instant, la seule défense
possible est **en amont** : Breeze lit le calendrier local — les créneaux occupés et le statut de
disponibilité, **jamais les titres, jamais les participants** — et s'en sert pour **placer** les
pauses à venir dans les interstices de la journée plutôt qu'au milieu d'un bloc de travail.

**Le placement se calcule au démarrage de chaque cycle, et ne se recalcule sur aucun autre rythme que
ce déclencheur-ci** : un changement du calendrier, tant que la pause qu'il concerne n'est pas encore
due — une fois due, plus aucun recalcul ne la touche (§10.7). **Le décalage qu'il s'autorise est
plafonné à la durée de travail configurée** : Breeze ne repousse jamais un placement plus loin qu'un
second bloc de travail entier n'aurait duré de toute façon, pour ne pas transformer un bon placement
en attente sans fin. **Si aucun interstice n'existe dans cette fenêtre**, le repli est le placement
nominal, non décalé — la pause tombe à l'heure que le rythme aurait donnée sans le bouclier, et Breeze
ne prétend pas avoir évité ce qu'il n'a pas pu éviter. **Le bouclier ne fait jamais tomber une pause
plus tôt que son heure nominale : il la retient, jamais il ne l'avance.**

**Pourquoi c'est décisif.** Elle fait passer Breeze de « logiciel qu'on désactive les jours chargés »
à « logiciel qu'on garde justement les jours chargés ». Le Mode Hardcore devient tenable en
environnement professionnel, ce qu'il n'était pas sans elle.

**Ce qu'elle coûte.** Une permission de plus, Calendriers — la seule que Breeze demanderait au-delà du
strict nécessaire. Elle **interdit désormais** de lire quoi que ce soit d'autre que les créneaux, et
elle interdit tout autant de laisser croire qu'elle couvre les imprévus : un événement ajouté après
qu'une pause est devenue due, ou une journée sans interstice, ne sont pas rattrapés.

### 9.2 La dette de posture

**Ce qu'elle apporte.** Traiter chaque cycle comme indépendant fait du produit du bruit très vite :
une pause qu'on ignore aujourd'hui n'a aucune conséquence demain. Toute interruption d'une pause —
quitter par le raccourci de fermeture du système, par le menu de l'icône d'état (§8.6), par le geste du Mode Hardcore (§8.5), ou
une suspension qui aura englouti une pause due (§10.1) — crédite une **dette** en minutes, visible en
permanence sur l'icône.

**Ce qu'elle crédite : les minutes de la pause qui n'ont pas été servies.** Une pause de dix minutes
interrompue à la deuxième en crédite huit ; une pause due mais jamais commencée en crédite la durée
entière, puisque aucune minute n'en a été servie. C'est la même quantité, comptée de la même façon,
quelle que soit la porte par laquelle l'interruption arrive — quatre portes, un seul calcul.

**Elle se rembourse et elle durcit.** Au **début** de chaque pause — avant que son décompte ne
commence, jamais en cours de route —, Breeze calcule sa durée en ajoutant la dette courante à la
durée réglée, **sans jamais dépasser la plus basse des deux bornes suivantes** : la durée de travail
configurée, ou la borne haute de pause du §12.2 (60 minutes). La dette diminue alors de ce que cet
allongement a pu absorber ; ce qu'elle ne peut pas absorber en une fois reste due et s'ajoute à
l'allongement de la pause suivante, jusqu'à épuisement ou jusqu'à minuit. **Seule une pause dont le
décompte s'écoule réellement jusqu'à son terme rembourse ainsi ; une pause comptée servie par le
coupe-circuit (§10.2) ou validée par une absence (§10.4) n'a jamais été vécue sous contrainte et ne
rembourse rien, sans pour autant recréditer quoi que ce soit — la dette reste simplement gelée tant
que l'un ou l'autre joue.** Au-delà d'un second seuil
(§12.3, mesuré), la dette **durcit la contrainte** : Breeze impose le Mode Hardcore pour le cycle en
cours, avec un bandeau qui l'annonce à l'avance — jamais le mot « préavis », réservé à la minute qui
précède une pause (§4). **La dette s'efface à minuit, heure locale**, comme les statistiques du §12.

**L'escalade n'est pas un interrupteur qu'on arme puis qu'on relâche : c'est une question reposée à
chaque cycle.** Au démarrage de chaque cycle, Breeze compare la dette courante au seuil et en tire la
sévérité de ce seul cycle — Hardcore si elle le dépasse, la sévérité choisie sinon. Rien n'est
« consommé » ni « désactivé » : le cycle suivant repose la même question sur la dette telle qu'elle
est alors, remboursement et minuit compris.

**Deux précisions ferment ce que « durcir » veut dire :**

1. **Si l'utilisateur est déjà en Hardcore par choix**, la comparaison ne change rien d'observable —
   le cycle est déjà à la sévérité que l'escalade aurait imposée.
2. **Interrompre un cycle durci crédite de la dette comme n'importe quel autre**, ce qui peut
   maintenir la dette au-dessus du seuil et durcir le cycle suivant aussi. **C'est voulu, pas un
   défaut à corriger** : rien n'arrête l'escalade sauf rembourser sous le seuil en servant une pause,
   ou minuit. Un utilisateur qui continue d'interrompre continue de durcir ; c'est exactement le
   signal que la fonctionnalité existe pour porter.

**Pourquoi c'est décisive.** C'est la réponse directe au besoin que personne ne formule (§7) :
déléguer une décision à soi-même passé. Un seul geste — interrompre une pause — porte tout le signal,
compté de la même façon partout où il se produit.

**Ce qu'elle coûte.** Elle **interdit désormais** de traiter les cycles indépendamment : la dette
devient un état persistant à faire survivre aux veilles, aux plantages et aux mises à jour. Elle
déplace la charge sur l'utilisateur au moment où il vient déjà d'interrompre une pause. Le seuil de
durcissement ne se fixe pas au bureau : il se mesure en test utilisateur (§12.3).

### 9.3 La détection de session profonde

**Ce qu'elle apporte.** Interrompre quelqu'un en pleine concentration profonde est un dommage net,
et c'est une objection qu'on peut attendre de la cible : « ça va me couper en plein travail ». Breeze
calcule un
**indice de profondeur** à partir de signaux qu'il possède déjà, sans permission supplémentaire — la
fréquence des changements d'application, la durée de session continue sur une même application, et la
régularité de l'activité déduite du seul instant de la dernière action, jamais son contenu (§10.6). En
profondeur élevée, **ce qui s'étire, c'est l'instant où le décompte de travail épuisé fait démarrer le
préavis — jamais le préavis ni la pause une fois commencés** (§10.1) : Breeze retient ce début jusqu'au
premier changement d'application, quel qu'il soit, dans une limite maximale au-delà de laquelle le
préavis démarre de toute façon.

**C'est la seule exception nommée qui retarde une pause au bénéfice de l'utilisateur, en réponse à son
comportement — et le brief le déclare comme telle plutôt que de le laisser comme une incohérence
tacite.** Ce qui la distingue d'une négociation : elle ne se déclenche jamais sur un geste, un bouton
ou un argument de l'utilisateur. Personne ne la demande au moment où elle joue ; personne ne peut la
provoquer ; personne ne sait qu'elle est en train de jouer. Un levier que l'utilisateur ne peut pas
atteindre n'est pas une négociation — c'est une décision que Breeze prend seul. **Le coupe-circuit
(§10.2) n'est pas une seconde exception à cette même règle : il ne répond à aucun comportement de
l'utilisateur et ne l'avantage pas — il protège contre les propres défauts de Breeze, et il continue
de compter les pauses qu'il laisse passer comme servies plutôt que d'ouvrir une négociation.** **Un
utilisateur qui provoque ces chutes délibérément obtient le même armement qu'un vrai défaut, et c'est
assumé plutôt que caché** : Breeze ne peut pas distinguer les deux (§10.2), et fermer cette porte
demanderait exactement la lecture d'activité que le §10.6 interdit.

**Pourquoi c'est décisive.** Elle fait passer Breeze du rayon santé au rayon productivité, sans rien
retirer à la promesse santé.

**Ce qu'elle coûte.** Elle **interdit désormais** toute interception d'événements clavier, sur aucun
OS — un dépassement qui mettrait la notarisation en jeu sur macOS et changerait le profil de sécurité
du binaire partout ailleurs (§13), donc le produit entier. Elle interdit
aussi l'apprentissage automatique : des moyennes glissantes et deux seuils suffisent, et restent
explicables à l'utilisateur, ce qu'un modèle ne serait pas. Et elle coûte une exception à expliquer
dans l'onboarding, à l'endroit précis où le brief vient de promettre qu'il n'y en aurait aucune —
c'est un choix assumé, pas un angle mort (§15, décision 10).
