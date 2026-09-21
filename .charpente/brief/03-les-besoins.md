## 7. Les besoins, séparés des solutions

Le tableau le plus important du document. La colonne de gauche porte **la formulation qui circule**
dans cette catégorie de produit — telle qu'elle arrive, déjà transformée en solution. Aucune ligne
n'est attribuée à un utilisateur nommé.

### Ce que demande celui qui veut s'arrêter

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « un rappel de pause toutes les heures » | être **empêché** de continuer, pas informé qu'il faudrait s'arrêter | une notification est un rappel, un overlay est une pause. Le produit intervient sur le poste de travail, jamais dans le centre de notifications |
| « qu'il me laisse choisir à quel point c'est strict » | décider **à froid** l'étendue de la contrainte, sans qu'aucune version future de lui-même puisse la rouvrir | la sévérité se choisit hors pause et se fige dès qu'une pause commence : elle se déroule sous la sévérité qu'elle avait à son premier instant (§10.3) |
| « une prévisualisation avant de choisir » | voir ce que le Mode Hardcore fait à son écran **avant** de le subir | l'onboarding montre les deux modes en animation. Découvrir le Hardcore en le subissant produit une désinstallation, pas un utilisateur convaincu (§8) |

### Ce que demande celui qui est en train de travailler

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « un bouton pour reporter quand je suis occupé » | obtenir un **délai** plutôt que d'être interrompu au pire moment possible | Breeze ne le sert pas au moment où la pause tombe — aucun bouton n'existe pour ça, et ce document ne fait semblant du contraire nulle part. Il le sert **avant**, en évitant d'y placer une pause quand c'est possible de le savoir à l'avance (§9). Une fois la pause due, la demande n'est plus servie : c'est le choix central du produit (§2) |
| « qu'il ne se déclenche pas pendant mes réunions » | ne pas subir une pause en pleine visio | même réponse : ça se traite en amont, dans le placement de la pause, jamais en la retardant après qu'elle est tombée (§9) |
| « pouvoir couper le truc si j'ai vraiment besoin de ma machine » | ne jamais être **piégé** par un logiciel | quitter Breeze — un geste que le système garantit de toute façon, sur les trois OS (§5) — reste possible à tout instant, y compris pendant une pause. Ce n'est pas une fonctionnalité de Breeze, c'est un fait du système que Breeze n'essaie pas de masquer. Le geste est **compté**, pas gratuit (§10.2) |
| « qu'il ne compte pas quand je ne suis pas devant » | ne pas devoir une pause qu'on a déjà prise en s'absentant | une absence — ou une veille, traitée pareil — assez longue vaut la pause (§10.4) |

### Ce que demande celui qui installe un logiciel qui prend son écran

| Formulé en solution | Le besoin dessous | Ce que ça change |
|---|---|---|
| « bloquer Slack et Figma, mais pas Spotify » | distinguer ce qui **retient** de ce qui **accompagne** | trois statuts, dont un — `ignoré` — qui arrête aussi le décompte : un aveu de l'utilisateur sur son propre usage, pas une exemption d'overlay (§8) |
| « je ne veux pas qu'une appli me surveille » | la **preuve**, et non la promesse, que rien ne sort de la machine | aucune sortie réseau hors la vérification de mise à jour, déclarée et désactivable. Jamais de titre de fenêtre, jamais de contenu d'écran, jamais de contenu de frappe clavier (§10.6) |
| « ça doit marcher tout seul » | ne pas avoir à reconfigurer après une veille, une mise à jour ou un plantage | l'échéance persistée fait autorité, jamais un décompte tenu en mémoire (§10.4) ; aucun overlay n'est jamais posé au lancement (§10.2) |
| « je suis sur Windows / sur Linux, est-ce que ça marche pareil ? » | savoir **à froid** ce qui est garanti chez soi, avant la première pause — pas le découvrir quand Alt-Tab passe devant l'overlay | une seule phrase, propre à la session détectée, sous le choix de sévérité (§5.7), et un tableau de ce qui est déclaré dégradé (§5.6). Breeze ne promet jamais « au-dessus de tout » : il dit ce que ta machine laisse recouvrir, et le Hardcore qui revient devant en moins d'une demi-seconde est présenté comme ce qu'il est, pas comme un verrou (§8.5) |
| « je passe de X11 à Wayland selon les jours » | que Breeze **s'adapte sans mentir** : ni prétendre suivre des fenêtres qu'il ne voit plus, ni lever une contrainte parce qu'une capacité a disparu | les capacités sont relevées à chaque lancement et à chaque changement de session, jamais au nom du bureau (§5.7) ; le mode d'une pause est fixé à son premier instant et ne bouge plus (§10.5) ; une capacité perdue accélère le décompte ou réduit la couverture, jamais elle ne lève une contrainte (§3) |

### Le besoin que personne ne formule

**Aucune de ces demandes ne dit ce qui rend le produit viable : pouvoir déléguer, une fois pour
toutes et à froid, le pouvoir de dire non — et ne plus jamais pouvoir le reprendre, quelle que soit
la qualité de l'argument du moment.** Ce n'était déjà pas formulé dans l'ancien modèle ; ça l'est
encore moins maintenant qu'aucune négociation n'existe pour laisser croire au contraire. C'est la
seule raison pour laquelle Breeze existe plutôt qu'un rappel de plus, et c'est de ce besoin unique que
sort la fonctionnalité décisive du §9.
