# SPECIFICATION_DECISIONS — Registre des décisions techniques définitives

> **Statut** : établi le 2026-11-03 à l'issue de l'audit transversal de la documentation
> (voir `DOCUMENTATION_AUDIT_REPORT.md`). Chaque décision consigne : le problème initial,
> les options envisagées, la décision, la justification et les documents affectés.
> Les décisions marquées **À VALIDER PAR L'UTILISATEUR** dans le texte ne sont pas
> définitives tant que la validation n'a pas eu lieu (§ « Questions ouvertes »).
> **Règle** : aucune de ces décisions n'est implémentée avant la porte de validation v0.6
> (ROADMAP, étape 0) ; le MVP v0.2 ne dépend d'aucune décision COT.

---

## D1 — Modèle d'import COT : validation intégrale puis transaction unique

**Problème initial.** `COT_ENGINE.md` §7 (version du 2026-11-02) décrivait un upsert
observation par observation sans dire si un fichier partiellement invalide pouvait être
importé par moitié, ni si l'écriture était atomique. L'audit exige une validation intégrale
avant persistance et l'atomicité SQLite.

**Options.**
- (a) Import en flux : valider et écrire observation par observation (reprise sur erreur
  possible) ;
- (b) Validation intégrale en mémoire, puis écritures dans une transaction unique ;
- (c) Validation intégrale + écriture en plusieurs transactions (journal / données).

**Décision.** Option (b) : le fichier est intégralement parsé et validé **avant toute
écriture** (structure, référentiels, identités arithmétiques, doublons internes,
complétude des 8 marchés — rejet = `422` avec la liste complète des problèmes, **zéro
écriture**) ; toutes les écritures (ligne `cot_imports`, observations, métriques,
compteurs) sont exécutées dans **une transaction SQLite unique** (`BEGIN IMMEDIATE`).
Les tentatives rejetées ne créent pas de ligne d'import ; chaque import **abouti** est
journalisé avec ses compteurs (`inserted`/`updated`/`unchanged`).

**Justification.** Un import par moitié produirait des séries incohérentes silencieuses —
exactement ce que la méthode interdit (H3, H4 : jamais de changement silencieux).
SQLite rend la transaction unique triviale et gratuite à cette échelle (quelques milliers
de lignes). La reprise sur erreur (option a) n'a aucun intérêt pour un outil local à
l'exécution manuelle : corriger le fichier et le ré-importer est plus sûr.

**Documents affectés.** `COT_ENGINE.md` §7 (réécrit), `DATA_MODEL.md` §3-0006 et §4,
`REQUIREMENTS.md` H13 (ajouté).

## D2 — Horodatages : trois dates indépendantes, conservées de bout en bout

**Problème initial.** `cot_imports` ne conservait que `imported_at` ; les dates
`generated_at` (exécution du moteur) et `retrieved_at` (téléchargement CFTC) existaient
dans le JSON mais disparaissaient à l'import. L'audit exige leur conservation
indépendante.

**Options.**
- (a) Ne conserver que `imported_at` (les autres dates restent dans le fichier source) ;
- (b) Conserver les trois dans `cot_imports` ;
- (c) Conserver les trois + un `fetched_at` par observation.

**Décision.** Option (b) : `cot_imports.generated_at`, `retrieved_at` (venues du fichier)
et `imported_at` (horloge de la base), toutes trois `NOT NULL`. Pas de date de
récupération par observation (une même récupération couvre tout le fichier).

**Justification.** Les trois dates répondent à des questions distinctes : « quand ces
chiffres ont-ils été calculés », « à quelle date le CFTC a été téléchargé », « quand sont-ils
entrés dans **mon** journal ». La fraîcheur (H5) et l'audit méthodologique en dépendent.
L'option (c) multiplierait des colonnes identiques à quelques milliers d'exemplaires.

**Documents affectés.** `DATA_MODEL.md` §3-0006, `COT_ENGINE.md` §5.4 et §7.7.

## D3 — Gestion des anomalies : individuelles et globales, toutes conservées

**Problème initial.** Le contrat JSON distinguait `observations[].anomalies` (par ligne) et
`import_anomalies` (par fichier), mais le schéma SQL ne persistait que les anomalies
globales : les anomalies de ligne étaient perdues à l'import.

**Options.**
- (a) Ne persister que les anomalies globales (perte d'information ligne) ;
- (b) Persister les deux : colonne `anomalies` (JSON) sur `cot_observations` **et** sur
  `cot_imports` ;
- (c) Table relationnelle d'anomalies (une ligne par anomalie).

**Décision.** Option (b). Codification en deux niveaux : anomalies **de ligne**
(`oi_zero_or_negative`, `missing_publication_date`, `ref_date_not_tuesday`,
`publication_date_out_of_range`, `long_exceeds_oi` / `short_exceeds_oi`,
`derived_mismatch`) — non bloquantes, l'observation est stockée et signalée ; anomalies
**de fichier / d'import** (`missing_market_week`, `year_unavailable`, `incomplete_row`,
`duplicate_observation_collapsed` côté moteur ; `value_changed`, `metric_mismatch` côté
backend) — journalisées sur `cot_imports.anomalies`. Les erreurs **bloquantes**
(`unknown_market`, `currency_mismatch`, `missing_market`, `invalid_position`,
`duplicate_conflict`, format inconnu) rejetent le fichier entier (D1) et ne sont donc
jamais persistées.

**Justification.** Une anomalie de ligne (ex. OI nul) décrit **cette** observation et doit
la suivre dans le temps ; une anomalie de fichier décrit **ce** téléchargement. L'option
(c) est une sur-ingénierie pour quelques dizaines d'anomalies par import ; le JSON en
colonne suffit à l'affichage et à l'export.

**Documents affectés.** `DATA_MODEL.md` §3-0006 et §2.8, `COT_ENGINE.md` §6 et §7.

## D4 — Relations imports ↔ observations : `first_import_id` / `last_import_id`

**Problème initial.** Aucune relation traçable entre `cot_imports` et
`cot_observations` : impossible de savoir quel import a produit une valeur, ou ce qu'un
import donné avait fait.

**Options.**
- (a) Aucune relation (statu quo) ;
- (b) Deux FK sur chaque observation : `first_import_id` (créateur) et `last_import_id`
  (dernier import ayant **écrit** les valeurs courantes) ;
- (c) Table de liaison exhaustive `cot_import_observations(import_id, observation_id,
  action)`.

**Décision.** Option (b). `first_import_id` est posé à l'INSERT et ne bouge plus ;
`last_import_id` est mis à jour uniquement quand un import **modifie** les valeurs (un
import qui ne change rien ne le touche pas — il « confirme », il n'« écrit » pas). Les
modifications de valeurs sont par ailleurs détaillées dans `cot_imports.anomalies`
(`value_changed` : marché, date, champ, ancien, nouveau).

**Justification.** Les deux questions réelles — « qui a créé cette ligne » et « quel import
a écrit la valeur que je regarde » — sont répondues par deux colonnes. L'option (c) donne
l'historique complet des actions (y compris confirmations) au prix d'une table croissante
(~2 400 lignes par import) et d'aucune question supplémentaire résolue pour un usage local
mono-utilisateur ; elle resterait une évolution possible si un besoin réel apparaissait
(principe « pas d'abstraction avant le deuxième usage »).

**Documents affectés.** `DATA_MODEL.md` §3-0006, §2.8 et §6 (nouvelle requête type),
`COT_ENGINE.md` §7.5.

## D5 — Référentiel des marchés : initialisation par migration, identifiants stables, alias, aucune création silencieuse

**Problème initial.** La migration 0006 créait `cot_markets` **vide**, et `COT_ENGINE.md`
§7.4 disposait qu'« un marché inconnu de la base est inséré » — contradiction directe avec
l'exigence « ne jamais créer silencieusement de nouveaux marchés ». Aucun mécanisme
d'alias n'existait pour absorber les renommages CFTC (le nom NZD a varié selon les
époques).

**Options.**
- (a) Marchés créés au premier import (statu quo, silencieux) ;
- (b) Référentiel sérialisé en migration (comme `currencies`) + table d'alias + rejet des
  marchés inconnus ;
- (c) Référentiel géré uniquement dans l'UI (Settings).

**Décision.** Option (b) : les **8 marchés sont initialisés par la migration 0006** avec
des identifiants internes 1–8 **stables** (`currency_id` résolu par code, pas par rowid) ;
`cot_market_aliases` recueille les variantes historiques de noms CFTC ; l'import résout un
marché par `cot_markets.market_name` **ou** alias, et **rejette** tout marché inconnu
(`unknown_market`) ; les 8 marchés doivent être présents dans le fichier
(`missing_market` sinon). Ajouter un marché ou un alias est une action **explicite et
versionnée** (migration), jamais un effet de bord d'import. Le nom principal NZD est la
forme du script A opérationnel ; **À VALIDER PAR L'UTILISATEUR** sur le fichier CFTC réel
(voir Questions ouvertes, Q1).

**Justification.** Le référentiel est une donnée de référence, pas une donnée utilisateur :
le sérialiser en migration garantit que deux bases fraîches sont identiques et que les ids
ne dépendent pas de l'ordre d'import. La création silencieuse de marchés ferait dériver le
référentiel au gré des fichiers (un fichier B mal filtré créerait des contrats croisés —
défaut §4.2). Les alias absorbent les renommages CFTC **sans** casser l'identité interne
ni l'historique.

**Documents affectés.** `DATA_MODEL.md` §3-0006 (seed + alias), §2.8, `COT_ENGINE.md`
§5.2 et §7.4, `REQUIREMENTS.md` H13.

## D6 — Classement transversal et traitement du DXY

**Problème initial.** (1) Le rang transversal était étiqueté « enseigné » alors que la
formation enseigne un **concept** (classement des devises, zones 80–100) sans formule
vérifiée — la formule du document était notre invention ; (2) l'exemple JSON donnait un
rang `88.0`, **impossible** avec la formule fournie pour 8 marchés (valeurs possibles :
k/7 × 100) ; (3) le traitement des égalités, des marchés manquants et du DXY dans ce
classement n'était pas défini ; (4) le rang était calculé par le moteur et transporté dans
le JSON, alors que l'audit exige de ne pas faire confiance aveuglément aux métriques
dérivées envoyées par Python.

**Options.**
- (a) Corriger seulement la valeur d'exemple et garder le rang dans le JSON (calcul
  moteur, validation backend) ;
- (b) Retirer le rang du contrat JSON et le **calculer à la lecture** par le backend à
  partir des observations stockées ;
- (c) Supprimer la mesure.

**Décision.** Option (b). Le champ `cross_sectional_rank` disparaît du contrat d'échange
v1 et de `cot_metrics` ; le backend le calcule à la lecture : `rang = (nombre de marchés
non-proxy présents à la même ref_date avec net_pct_oi strictement inférieur) / (nombre de
marchés présents − 1) × 100`. Égalités : les ex æquo partagent le même rang (comptage
strictement inférieur). Marchés manquants : rang calculé parmi les présents, affiché avec
son décompte (« 5/7 ») ; moins de 2 marchés → `null`. **DXY exclu du classement** : la
ligne USD reste affichée avec ses percentiles et son étiquette « proxy DXY pondéré », mais
ne participe pas au rang. La formule est explicitement étiquetée **formalisation Macro
Journal [AMÉLIORATION]**, jamais attribuée au formateur. En complément, une troisième
mesure est distinguée et documentée : la **position actuelle** du marché dans sa propre
histoire (fenêtre 52 observations), calculée elle aussi à la lecture.

**Justification.** Le rang est une fonction pure des faits déjà stockés (les
`net_pct_oi` à une même date) : l'envoyer dans le fichier dupliquerait la formule en deux
langages avec un risque de divergence — précisément la famille de risque que l'audit
demande d'éliminer — et exigerait une revalidation après chaque correction/ré-import.
Calculé à la lecture, il est **toujours** cohérent avec les données stockées, et le moteur
reste concentré sur les statistiques temporelles (percentiles) qui, elles, exigent la série
complète. Ce n'est pas un déplacement de fonctionnalité existante : **aucun des deux
scripts ne calcule de rang aujourd'hui** (vérifié : ils produisent `Net_Pct_OI`, `Pct_20`,
`Pct_80` uniquement). L'exclusion du DXY évite de doubler l'influence de l'EUR (~57,6 % du
panier) et de comparer un instrument dérivé pondéré à des agrégats de devises ;
l'agrégat USD non pondéré enseigné n'étant pas disponible, aucun substitut ne doit
mascarer cette absence — d'où le décompte affiché. L'affichage UI (H5/H6) et la priorité
v0.6 sont inchangés.

**Documents affectés.** `COT_ENGINE.md` §4.4, §6 (exemple et schéma), §8, §9 ;
`DATA_MODEL.md` §2.8, §3-0006 (CHECK de `cot_metrics`), §6 ; `REQUIREMENTS.md` H6, §11 ;
`COURSE_METHOD.md` §7, §14, §15.3 ; `ROADMAP.md` v0.6 ; `ARCHITECTURE.md` §4.3.

## D7 — Fenêtre des percentiles : 52 **observations** (renommage `window_weeks` → `window_observations`)

**Problème initial.** Le champ s'appelait `window_weeks: 52` alors que la fenêtre est
comptée en **observations** (défaut e des scripts : les trous de jours fériés allongent la
fenêtre calendaire réelle). La dénomination mentait sur la sémantique.

**Options.**
- (a) Conserver `window_weeks` et documenter l'écart ;
- (b) Renommer en `window_observations` (contrat JSON + colonne SQL) ;
- (c) Rendre la fenêtre calendaire (52 semaines exactes).

**Décision.** Option (b) : `window_observations: 52` dans le contrat et
`cot_metrics.window_observations` en base. Le comportement (fenêtre en observations) est
inchangé — seul le nom devient honnête. L'option (c) est refusée : elle changerait la
méthode elle-même et l'historique des métriques.

**Justification.** Un contrat technique dont les noms contredisent la sémantique est une
source permanente d'erreurs d'interprétation (le lecteur croirait « 52 semaines
calendaires »). Le renommage est gratuit tant que rien n'est implémenté.

**Documents affectés.** `COT_ENGINE.md` §5.1, §6 ; `DATA_MODEL.md` §3-0006, §1 (diagramme).

## D8 — Percentiles historiques : définition « as-of » et méthode de quantile épinglée

**Problème initial.** Les scripts calculent les percentiles sur les **52 dernières
observations au moment du téléchargement** puis les appliquent à tout l'historique
(look-ahead : un point de 2021 est comparé à des bornes calculées avec des données de
2026). De plus, la méthode de quantile (interpolation) n'était pas spécifiée — deux
implémentations pourraient diverger.

**Options.**
- (a) Statu quo (lignes droites) ;
- (b) Percentile glissant « as-of » : fenêtre des 52 observations se terminant à
  l'observation courante ; méthode d'interpolation épinglée ;
- (c) Fenêtre calendaire glissante de 52 semaines.

**Décision.** Option (b), avec les précisions suivantes : aucune métrique avant la 52ᵉ
observation (fenêtre expansive puis glissante) ; **interpolation linéaire** sur les ordres
statistiques (défaut pandas/numpy, formule écrite in extenso dans `COT_ENGINE.md` §5.1
pour permettre une réimplémentation exacte) ; les valeurs `null` (OI ≤ 0) restent dans le
décompte des 52 lignes mais sont exclues du calcul ; arrondi à 2 décimales ; le PNG
« lignes droites » reste disponible sur demande explicite (`--snapshot`) pour compatibilité
visuelle. Option (c) refusée : changement de méthode sans bénéfice démontré (voir D7).

**Justification.** Le look-ahead contredit la vocation du journal (« retrouver ce que je
savais à la date t ») et l'exigence d'audit méthodologique. L'épinglage de la méthode
évite la divergence silencieuse entre implémentations futures (test Rust de
vérification par échantillonnage, D10).

**Documents affectés.** `COT_ENGINE.md` §5.1, §6 ; `ROADMAP.md` v0.6 ;
`REQUIREMENTS.md` §11.

## D9 — Versionnement à trois niveaux : format, moteur, méthodes statistiques

**Problème initial.** Seuls `format_version` (contrat) et `generator.version` (moteur)
existaient ; une évolution de la **méthode statistique** (fenêtre, interpolation) n'était
traçable nulle part.

**Options.**
- (a) Un seul numéro de version global ;
- (b) Trois niveaux indépendants : `format_version`, `generator.version`,
  `method_version` par métrique (déclarées dans `stats_methods` du fichier).

**Décision.** Option (b) : le JSON racine porte `format_version` + `generator` (nom,
version, source) + `stats_methods` (déclaration des méthodes : id, définition, paramètres,
version) ; chaque métrique référence sa méthode par `method_version` ; en base,
`cot_metrics.method_version` conserve la trace. Règles : un changement de **format** est
incompatible (backend refuse les versions inconnues) ; un changement de **méthode** est
compatible mais tracé par métrique ; un changement de **moteur** (bug fix, durcissement)
ne touche ni l'un ni l'autre tant que les mêmes méthodes sont appliquées.

**Justification.** Les trois évoluent pour des raisons indépendantes. Un numéro unique
forcerait des bumps de format pour des corrections de bugs moteur (cassant les imports
pour rien) ou masquerait des changements de méthode (silence interdit par H4).

**Documents affectés.** `COT_ENGINE.md` §6 (schéma + §6.6), `DATA_MODEL.md` §3-0006.

## D10 — Métriques dérivées : le backend ne fait pas confiance aveuglément au moteur

**Problème initial.** `COT_ENGINE.md` §6 précisait « la base stocke la valeur exportée,
pas un recalcul » — l'inverse exact de l'exigence d'audit sur les métriques dérivées
envoyées par Python.

**Options.**
- (a) Stocker les valeurs moteur telles quelles (statu quo) ;
- (b) Recalculer **toutes** les métriques côté backend (y compris les percentiles
  complets) ;
- (c) Politique proportionnée : recalcul exact des dérivés arithmétiques, vérification par
  échantillonnage des statistiques, calcul local du rang (D6).

**Décision.** Option (c). `net_noncomm` et `net_pct_oi` sont **recalculés** par le backend
à partir des faits (Long/Short/OI) et les valeurs recalculées sont stockées ; tout écart
avec le fichier (tolérance 0,005 sur le % après règle d'arrondi) est journalisé
`derived_mismatch`. Les percentiles ne sont pas recalculés intégralement (ce serait
réimplémenter le moteur en Rust) : pour chaque marché, le backend **recalcule** P20/P80 de
la dernière observation dotée de métriques à partir des valeurs stockées (fenêtre 52,
méthode épinglée D8, tolérance ±0,01) et journalise `metric_mismatch` en cas d'écart — la
valeur moteur reste alors stockée (autorité statistique), l'anomalie est visible. Le
classement transversal n'est pas transporté (D6).

**Justification.** Les dérivés arithmétiques sont exacts et gratuits à recalculer : il
serait absurde de stocker une valeur tierce contredite par les faits de la même ligne. Les
percentiles dépendent d'une définition statistique épinglée mais subtile : une
réimplémentation intégrale dupliquerait la méthode (risque de faux positifs), tandis que
l'échantillonnage détecte engine bogué ou fichier trafiqué à coût quasi nul. L'ensemble
répond à « ne pas faire confiance aveuglément » sans construire un moteur concurrent —
interdit par ailleurs (anti-exigence #2, « pas d'architecture parallèle »).

**Documents affectés.** `COT_ENGINE.md` §6.5, §7.5-§7.6 ; `DATA_MODEL.md` §2.8, §4 ;
`REQUIREMENTS.md` H12 (ajouté).

## D11 — SHA-256 du fichier importé et ré-import identique

**Problème initial.** Rien ne permettait de reconnaître un fichier déjà importé ; le
`source_file` (nom) est un identifiant fragile.

**Options.**
- (a) Ne rien faire (l'idempotence rend le ré-import inoffensif de toute façon) ;
- (b) Conserver le SHA-256 et **refuser** tout ré-import identique ;
- (c) Conserver le SHA-256, détecter le ré-import et exiger une **confirmation
  explicite** pour le rejouer.

**Décision.** Option (c) : `cot_imports.source_sha256` ; si le hash correspond à un import
antérieur réussi → `409` avec l'identifiant et la date de l'import précédent ; le
ré-import d'un fichier strictement identique exige `force=true` (et reste un no-op
idempotent : `unchanged`).

**Justification.** L'idempotence protège les données, pas l'expérience utilisateur :
importer deux fois le même fichier par erreur doit être visible et explicite, pas
silencieusement absorbé. Le refus définitif (b) est inutilement rigide (le no-op est
inoffensif). Le hash sert aussi de contrôle d'intégrité des fichiers archivés. Écarté :
comparer les contenus sans hash (coûteux et imprécis).

**Documents affectés.** `COT_ENGINE.md` §7.1, `DATA_MODEL.md` §3-0006.

## D12 — Terminologie unifiée `entries` → `notes` (table, API, types partagés)

**Problème initial.** La migration 0002 renomme la table en `notes`, toute la
documentation et l'UI parlent de notes… mais la surface API cible restait `/api/entries`,
les fichiers cibles `routes/entries.rs` / `services/entries.rs`, et le prototype v0.1.0
expose `Entry`/`EntryDraft` + `/api/entries` (vérifié dans le code). L'audit demande la
cohérence des routes et de la terminologie.

**Options.**
- (a) Conserver `/api/entries` (la table seule devient `notes`) ;
- (b) Renommage complet en v0.2 : routes `/api/notes`, types `Note`/`NoteDraft`,
  fichiers `routes/notes.rs`/`services/notes.rs`, frontend mis à jour dans la même
  version ;
- (c) Double routes (alias `/api/entries` → `/api/notes`).

**Décision.** Option (b), réalisée **atomiquement en v0.2** avec la migration 0002. Le
prototype v0.1.0 (documenté comme tel dans `ARCHITECTURE.md` §8) garde son nommage
jusqu'à la v0.2 — aucune modification du code existant n'est faite dans le cadre de cette
révision documentaire. **À VALIDER PAR L'UTILISATEUR** (voir Questions ouvertes, Q4).

**Justification.** L'option (a) institutionnaliserait un vocabulaire scindé pour la vie
entière du projet ; l'option (c) est de la dette gratuite. Le coût du renommage est
minimal (aucun consommateur externe : application locale, frontend propre), et v0.2 est la
dernière fenêtre gratuite — après le MVP, renommer l'API deviendrait une rupture. La
migration `ALTER TABLE entries RENAME TO notes` a été vérifiée contre la migration 0001
réelle (table et index présents, aucune autre table ne référence `entries`).

**Documents affectés.** `ARCHITECTURE.md` §4.1, §4.3, §8 ; `ROADMAP.md` v0.2 ;
`DATA_MODEL.md` §3-0002 (note d'implémentation).

## D13 — Déduplication : observations COT (dans le fichier et entre imports) et sources

**Problème initial.** Les règles de déduplication n'étaient pas formalisées : que faire
de deux observations identiques dans un même fichier, de deux différentes, d'un ré-import
partiellement convergent ; et la formulation « unicité d'URL exacte après trim » prêtait
le trim à la contrainte SQL.

**Décisions.**
- **Dans un fichier** : deux observations de même `(market_name, ref_date)` **identiques**
  → fusion en une + anomalie `duplicate_observation_collapsed` ; **différentes** → rejet
  du fichier (`duplicate_conflict`, D1).
- **Entre imports** : upsert sur `(market_id, report_type, ref_date)` — identique →
  `unchanged` (les FK d'import ne bougent pas, D4) ; différent → `updated` + anomalie
  `value_changed` (H4). Les observations absentes du fichier ne sont jamais supprimées.
- **Sources (hors COT)** : le service applique `trim()` à l'écriture et exige une URL non
  vide ; la contrainte `UNIQUE (url)` garantit l'unicité **des valeurs stockées** —
  formulation corrigée (le SQL ne trim pas). La normalisation fine (casse, fragment,
  slash final) reste reportée (ROADMAP).

**Justification.** Le doublon interne « différent » signale un moteur ou un fichier
corrompu : importer l'un des deux au hasard serait arbitraire (défaut §4.2 du script B) ;
le rejet est le seul comportement honnête. Le doublon identique est inoffensif mais doit
être visible. Pour les sources, distinguer ce que fait le **service** (trim) de ce que
garantit le **SQL** (unicité des valeurs stockées) évite de croire à une protection qui
n'existe pas.

**Documents affectés.** `COT_ENGINE.md` §6.5, §7.5 ; `DATA_MODEL.md` §2.6, §4.

## D14 — Historisation et suppression des notes (décisions addendum reprises au registre)

**Problème initial.** Périmètre de `note_revisions` non défini (quels champs ?),
sémantique de `saved_at` ambiguë, comportement de suppression non tranché.

**Décisions** (posées par l'addendum, vérifiées cohérentes par l'audit du 2026-11-03,
reprises ici pour le registre).
- **Champs historisés** : `title`, `content`, `kind`, `status`, `bias`, `period_start` +
  `saved_at` (date de sauvegarde **de la version archivée**, copiée de
  `notes.updated_at`). Non couverts : les liens (devises/tags/sources) — conséquence
  documentée (« à quelles devises était liée cette note en mars » n'est pas
  reconstituable).
- **Mécanique** : INSERT de l'ancienne version **avant** l'UPDATE, dans la même
  transaction.
- **Suppression** : logique (`deleted_at`), l'historique est conservé ; corbeille dans
  Settings avec restauration ou purge physique (cascade sur liens et révisions, avec
  avertissement et proposition d'export).

**Justification.** « Ne pas détruire involontairement l'historique » prime sur la
simplicité absolue ; le coût (une colonne nullable + un prédicat de filtre) est faible.
L'audit du 2026-11-03 a corrigé le **diagramme** (qui omettait `kind`/`status`/`bias`/
`period_start` et `deleted_at`) pour qu'il corresponde **exactement** à la migration — le
contenu des décisions lui-même était sain.

**Documents affectés.** `DATA_MODEL.md` §1 (diagramme corrigé), §2.3, §3-0002, §4.

---

## Questions ouvertes (non résolues — tracées explicitement)

| # | Question | Statut | Impact si non résolue |
|---|---|---|---|
| Q1 | **Nom CFTC exact du marché NZD** sur la plage 2019–2026 (`NEW ZEALAND DOLLAR - ...` et/ou `NZ DOLLAR - ...`) : à vérifier sur le fichier réel au durcissement du moteur, puis alias ajouté si nécessaire | **À VALIDER PAR L'UTILISATEUR** (données réelles requises) | Blocante pour l'export moteur (un marché introuvable bloque le JSON) ; l'alias la rend non bloquante côté base |
| Q2 | **Emplacement du moteur** : `tools/cot/` dans le dépôt (proposition) vs dépôt séparé | **À VALIDER PAR L'UTILISATEUR** | Organisation du dépôt uniquement ; aucun impact contrat/modèle |
| Q3 | **Plage d'années 2019 → courant** (paramétrable) : sufficient pour disposer de ≥ 52 observations avant 2021 (début d'analyse des scripts actuels) — confirmation souhaitée | À confirmer | Historique des percentiles plus ou moins profond ; aucun impact structurel |
| Q4 | **Renommage `/api/entries` → `/api/notes` en v0.2** (D12) : touche au code du prototype (routes, types, frontend) | **À VALIDER PAR L'UTILISATEUR** | Si refusé : conserver `/api/entries` et documenter l'écart de vocabulaire comme décision assumée |
| Q5 | **Provenance du commentaire « Style ForexFondamentale : lignes droites »** (script B) : la présentation « lignes droites » imite-t-elle un style vu dans la formation ou ailleurs ? | Ouvert (hérité de `COURSE_METHOD.md` §15) | Attribution méthodologique uniquement ; aucun impact technique |
| Q6 | **Tolérances de vérification** (±0,005 sur `net_pct_oi` recalculé ; ±0,01 sur l'échantillonnage des percentiles) | Détail d'implémentation, confirmable à l'usage | Faux positifs d'anomalies si trop serrées ; ajustables sans changement de format |

**Les questions méthodologiques non résolues sont les seules Q1 (dépendante de données
réelles) et Q5 (dépendante de la mémoire de l'utilisateur). Aucune ne bloque le MVP v0.2.**
