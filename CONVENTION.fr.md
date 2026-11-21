# Vibecoding Convention

Version 0.1, brouillon. Traduction de [CONVENTION.md](CONVENTION.md) modifié le 2026-11-21 ; le texte anglais fait foi.

Un logiciel écrit entièrement ou partiellement par des assistants IA peut être aussi fiable que tout autre logiciel, à condition d'être cadré, revu et audité. Cette page définit ces trois mots en termes vérifiables. Personne n'accorde la convention ; un projet déclare un niveau et conserve les preuves dans son propre dépôt, où n'importe qui peut les vérifier.

Un mainteneur l'utilise pour montrer la qualité avec des preuves au lieu de promesses. Un utilisateur l'utilise pour vérifier un projet avant de s'y fier. Un contributeur l'utilise pour qu'une pull request assistée par IA soit prise au sérieux.

Chaque ligne ci-dessous est une phrase. L'énoncé complet, la preuve et la vérification de chaque exigence sont dans [docs/requirements.md](docs/requirements.md) (en anglais) ; les raisons de la forme de la convention sont dans [docs/rationale.md](docs/rationale.md) (en anglais) ; les critiques auxquelles elle répond, avec sources, sont dans [docs/criticisms.md](docs/criticisms.md) (en anglais). Les outils lisent la même liste à partir de [convention.yaml](convention.yaml) (en anglais).

## Principes

- **Transparence plutôt que pureté.** Dire ce que l'IA a écrit ; personne n'est tenu d'éviter l'IA.
- **Des preuves plutôt que des affirmations.** Chaque exigence nomme un artefact ou une commande que n'importe qui peut exécuter.
- **Proportionnalité.** Un projet choisit le niveau qui convient et le prouve.
- **Les humains décident.** L'IA propose ; un humain nommé valide, décide et est responsable.
- **Des résultats, pas des formalités.** La convention dit ce qui doit être visible, jamais quel outil utiliser ni comment travailler.

## Partie A : un projet

La partie A s'applique à un dépôt et aux personnes qui le maintiennent. Chaque niveau inclut ceux ci-dessous.

### Niveaux

| Niveau | Vous faites | Cela prouve | Coût | Convient à |
|---|---|---|---|---|
| 1 **Déclaré** | Dire que l'IA a écrit une partie du projet, avec quels outils et modèles, qui décide, comment vous acceptez les contributions et votre position sur la licence. Marquer les commits écrits par l'IA. Un humain révise chaque changement. | Rien n'est caché : qui a écrit quoi, qui est responsable. | Un après-midi. Texte seulement, aucun outil à installer. | scripts, prototypes, tout ce qui est public |
| 2 **Testé** | Niveau 1, plus tests, couverture, lint, détection de secrets, analyse statique et vérifications des dépendances en CI, leurs résultats enregistrés dans le dépôt comme preuves derrière un badge daté. | Le code se comporte comme affirmé et est vérifié à chaque changement, pas une seule fois. | Un ou deux jours de configuration CI, puis il tourne tout seul. | tout ce sur quoi d'autres dépendent |
| 3 **Audité** | Niveau 2, plus audits versionnés dans le dépôt : revue d'architecture, audit de sécurité, test d'utilisabilité en aveugle, documents d'architecture validés, chaque constat avec une décision. | Quelqu'un d'autre que l'auteur a regardé, et ce qui a été trouvé est consigné. | Quelques heures par version, à chaque fois. | les données, l'argent ou la sécurité d'autrui |

### Exigences

Les identifiants en gras dans le tableau ci-dessous sont le cœur d'un niveau : les exigences qui le constituent. Les autres le consolident. Toutes sont obligatoires : un projet qui ne peut pas en respecter une le dit, et pourquoi, sous `Exceptions` dans son `VIBECODING.md`. La convention n'est pas à la carte, mais elle ne demande jamais le silence.

Vérification : **M** mécanique (un outil la vérifie sans jugement), **J** jugement (un relecteur, humain ou agent, tranche), **M+J** les deux.

| ID | Niveau | Exigence | Preuve | Vérification |
|---|---|---|---|---|
| D1 | 1 | Le README divulgue l'assistance IA et pointe vers `VIBECODING.md` | README | M |
| **D2** | 1 | `VIBECODING.md` déclare l'implication IA, le niveau de convention, les outils et modèles, les mainteneurs, la politique de revue, l'état, la provenance, le commit d'adoption et les seuils | le fichier | M |
| **D3** | 1 | Les commits écrits par IA portent un trailer nommant l'outil et le modèle, et un modèle ne signe jamais un certificat d'origine | historique git | M |
| **D4** | 1 | Aucun changement écrit par IA n'atteint la branche par défaut sans une revue humaine, et le générateur n'est jamais le seul réviseur | politique, historique de pull request | M+J |
| D7 | 1 | La déclaration indique la licence, la position sur le droit d'auteur des parties écrites par IA, qu'aucune reproduction connue incompatible avec la licence n'a été incluse, et les garde-fous utilisés | la déclaration | M |
| D8 | 1 | `CONTRIBUTING.md` indique la politique pour les contributions IA entrantes, nomme le trailer du projet et demande C1 à C3 dans les pull request | le fichier, modèle de pull request | M |
| D9 | 1 | La déclaration indique l'état du projet et le délai de réponse en matière de sécurité, correspondant à `SECURITY.md` | les fichiers | M |
| **T1** | 2 | Les tests s'exécutent en CI à chaque changement depuis un checkout propre | config CI | M |
| **T2** | 2 | La couverture, mesurée par un outil déclaré, est au moins égale au seuil déclaré, et CI écrit le rapport dans le fichier de preuve | fichier de preuve de couverture | M |
| T3 | 2 | Les tests affirment le comportement, aucun test n'est sauté sans raison, et chaque changement de comportement arrive avec son test | suite de tests, historique | J |
| T4 | 2 | Le logiciel destiné à l'utilisateur a des tests de bout en bout de ses parcours principaux | suite e2e | J |
| T5 | 2 | Lint et formatage s'exécutent en CI et échouent sur les avertissements, avec les exceptions listées avec une raison | config CI | M |
| **T6** | 2 | Le fichier de verrouillage (lockfile) est versionné, les vérifications de vulnérabilités et de licence s'exécutent en CI, et chaque nouvelle dépendance est vérifiée contre le registre avant fusion | fichier de verrouillage (lockfile), CI, fichier de preuve, historique | M |
| **T7** | 2 | Un analyseur de secrets et un analyseur statique s'exécutent en CI à chaque changement, et les constats sont corrigés ou acceptés avec une raison | config CI | M |
| T8 | 2 | La duplication, la complexité, la longueur des fonctions et des fichiers et les marqueurs résiduels sont mesurés avec un outil déclaré et des seuils, et enregistrés dans le fichier d'état | fichier de preuve, fichier d'état | M |
| T9 | 2 | Les dépendances sont à jour, maintenues à jour par un mécanisme déclaré, aucune n'est abandonnée, et les exceptions sont déclarées avec une raison | config de mise à jour, fichier de preuve obsolète | M |
| D5 | 2 | `audit/status.json` contient la date, le commit et le verdict de la dernière vérification, est écrit par la vérification, alimente le badge et se marque obsolète | fichier d'état, badge | M |
| D6 | 2 | Un changement qui supprime, saute ou affaiblit un test indique pourquoi, et un humain approuve | historique git | M+J |
| D10 | 2 | Les fichiers d'instructions d'agent nomment le niveau, pointent vers `VIBECODING.md`, donnent les commandes à passer, les règles auxquelles les humains sont tenus, et ce que l'agent ne fait jamais seul | les fichiers | M |
| A1 | 3 | Une revue d'architecture juge le code par rapport à des principes de conception nommés, y compris le signal plutôt que le bruit, et donne un verdict, renouvelé à chaque changement mineur ou structurel | `audit/*-architecture.md` | J |
| **A2** | 3 | `SECURITY.md` nomme un contact, un délai de réponse et un modèle de menace, chaque affirmation de sécurité est tracée jusqu'à un test réussi, et les chemins de données utilisateur ont des tests d'autorisation | `audit/*-security.md` | M+J |
| **A3** | 3 | Lorsque le logiciel a une documentation utilisateur, un testeur sans accès au code source l'installe et l'utilise à partir de la documentation seule | `audit/*-blind-*.md` | J |
| **A4** | 3 | `audit/README.md` indexe chaque rapport avec date, commit, type, outil, lecteur et verdict, et les rapports ne sont jamais réécrits, seulement ajoutés | l'index | M |
| A5 | 3 | Aucune publication ne sort avec un `BLOCKER` ou un `DOC-WRONG` ouvert, et chaque publication nomme l'humain qui l'a approuvée | index à l'étiquette, notes de publication | M |
| A6 | 3 | Une vue d'ensemble de l'architecture et des enregistrements de décision existent, chacun validé par un humain nommé avec une date, revalidé en cas de changement structurel, et correspondent au code | les documents | M+J |
| A7 | 3 | Le score de mutation est mesuré sur les modules principaux et est au moins égal au seuil déclaré (recommandé) | fichier de preuve de mutation | M |
| A8 | 3 | Tout ce qui s'exécute avec un accès au dépôt est versionné, revu, épinglé, exempt de secrets, et a un modèle de permissions déclaré | les fichiers | M+J |

## Partie B : contribuer à d'autres projets

La partie B s'applique à une personne ou à un agent envoyant une contribution à un projet qu'il ne maintient pas, quelle que soit la position propre de ce projet. La preuve est dans la contribution elle-même, et le mainteneur destinataire est celui qui la vérifie. Un projet à tout niveau de la partie A fait de C1 à C3 les cases à cocher de son modèle de pull request (D8).

| ID | Exigence | Preuve |
|---|---|---|
| C1 | La contribution divulgue l'assistance IA, avec outil et modèle, dans son texte et comme le trailer attendu par le projet cible | description, trailers |
| C2 | Le contributeur connaît chaque changement dans le diff et peut dire ce qu'il fait, pourquoi il est là et ce qu'il affecte | description, réponses de revue |
| C3 | La contribution est complète : un changement apporte ses tests et sa documentation et dit ce qui a été exécuté, un rapport de bug ou de vulnérabilité apporte une reproduction | le diff, la description |
| C4 | Le guide de contribution et la politique IA du projet cible sont respectés, un changement par pull request, pas de changements générés non sollicités dans de nombreux dépôts | description, historique du contributeur |
| C5 | Le contributeur répond à la revue lui-même, dans le délai attendu par le projet, ou la contribution peut être fermée sans discussion | le fil de revue |

## Constats et décisions

Les sévérités sont jugées du point de vue de la personne affectée, un utilisateur, un mainteneur ou un opérateur : `BLOCKER` (une tâche documentée ne peut pas être accomplie, ou une affirmation de sécurité est fausse), `DOC-WRONG` (la documentation dit quelque chose que le logiciel ne fait pas), `PAINFUL` (possible, à un coût que cette personne détesterait à chaque fois), `MINOR` (tout le reste qui mérite d'être écrit). Chaque constat reçoit une décision d'un mainteneur humain : `fixed` (corrigé), `documented` (documenté), `won't fix` (non corrigé) avec une raison, ou `open` (ouvert). Les constats ne sont jamais supprimés ni rétrogradés. Aucune publication ne sort avec un `BLOCKER` ou un `DOC-WRONG` ouvert.

## Non-objectifs

La convention ne dit pas s'il faut utiliser l'IA, ni si elle est plus rapide ou moins chère. Ce n'est pas une certification. Elle n'exige jamais qu'un humain ait écrit quelque chose, seulement qu'un humain l'ait validé. Lorsqu'une exigence n'a pas de sens pour un projet, la déclaration le dit et pourquoi.

## Outils

La convention peut être respectée à la main. Un validateur, des agents, des compétences et des modèles qui l'appliquent depuis des outils de codage IA sont décrits dans [docs/tooling.md](docs/tooling.md) (en anglais).
