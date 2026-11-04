# DOCUMENTATION_AUDIT_REPORT — Rapport de validation de la spécification Macro Journal

> **Date** : 2026-11-03. **Objet** : révision des livrables documentaires à partir des
> observations de l'audit transversal (contrat JSON COT, modèle SQLite, règles de calcul
> Python, architecture Rust), puis vérification que l'ensemble constitue une spécification
> cohérente et exploitable. Documents concernés : `COURSE_METHOD.md`, `OLD_JOURNAL_AUDIT.md`,
> `REQUIREMENTS.md`, `ARCHITECTURE.md`, `DATA_MODEL.md`, `ROADMAP.md`, `COT_ENGINE.md`.
> Décisions définitives : `SPECIFICATION_DECISIONS.md` (D1–D14).

---

## 1. Méthode

1. Relecture **intégrale** des sept documents (versions du 2026-11-02).
2. Relecture **intégrale** des deux scripts Python de référence
   (`work/cot-engine-reference/script_A_dashboard.py`, 180 lignes ;
   `script_B_debug_variant.py`, 83 lignes) — **audit par lecture de code, sans exécution**
   (cf. §6 pour les limites).
3. Confrontation avec le code réel du prototype v0.1.0 : `migrations/0001_create_entries.sql`
   et `apps/server/src/main.rs` (routes `/api/entries`, types `Entry`/`EntryDraft`,
   index `idx_entries_updated_at`) — pour vérifier l'exécutabilité de la migration 0002 et
   la terminologie réelle.
4. Pour chaque observation de l'audit : vérification de l'existence réelle de
   l'incohérence dans les documents, analyse des conséquences, correction si justifiée.
5. Aucune modification de code, aucune implémentation COT — révision documentaire
   uniquement, conformément aux contraintes.

## 2. Incohérences initiales constatées (vérifiées, pas supposées)

| # | Composant | Incohérence constatée | Gravité |
|---|---|---|---|
| 1 | COT / SQL vs JSON | `cot_observations.net_pct_oi REAL NOT NULL` contredit le contrat (et le garde-fou moteur) qui produisent `null` quand OI ≤ 0 | bloquante à l'implémentation |
| 2 | COT / contrat | Exemple JSON : `cross_sectional_rank = 88.0` — **impossible** avec la formule donnée pour 8 marchés (valeurs possibles : k/7 × 100 ∈ {0 ; 14,3 ; 28,6 ; 42,9 ; 57,1 ; 71,4 ; 85,7 ; 100}) | erreur factuelle |
| 3 | COT / méthodologie | Le rang transversal était étiqueté « enseigné » ; la formation enseigne le **concept** (zones 80–100), la **formule** était une formalisation non vérifiée | méthodologique |
| 4 | COT / contrat | `window_weeks: 52` alors que la fenêtre est comptée en **observations** (§5.1 le documentait lui-même) | dénomination mensongère |
| 5 | COT / SQL | Aucune relation traçable `cot_imports` ↔ `cot_observations` | traçabilité |
| 6 | COT / SQL | Anomalies **individuelles** du JSON perdues à l'import (seules les anomalies globales étaient persistées) | perte d'information |
| 7 | COT / SQL | `generated_at` et `retrieved_at` non conservées dans `cot_imports` (seul `imported_at` existait) | perte d'information |
| 8 | COT / règles d'import | §7.4 : « un marché inconnu de la base est inséré » — contradiction directe avec « ne jamais créer silencieusement de nouveaux marchés » ; par ailleurs `cot_markets` créé **vide** (aucune initialisation) | bloquante (intégrité référentielle) |
| 9 | COT / règles d'import | Comportements non définis : fichier partiellement invalide, ré-import, atomicité, doublons internes au fichier, marché totalement absent vs semaine manquante | spécification incomplète |
| 10 | COT / méthodes | Méthode de quantile non épinglée (interpolation) ; traitement des égalités et des marchés manquants dans le rang non défini ; traitement du DXY dans le classement non défini | spécification incomplète |
| 11 | COT / versionnement | Aucun versionnement séparé du format JSON, du moteur et des méthodes statistiques | traçabilité |
| 12 | COT / contrat | « la base stocke la valeur exportée, pas un recalcul » — contredit l'exigence « ne pas faire confiance aveuglément aux métriques dérivées envoyées par Python » | contradiction frontale |
| 13 | Diagramme / migrations | Diagramme ER : `note_revisions` sans `kind`/`status`/`bias`/`period_start`, `notes` sans `deleted_at`, tables COT (`cot_imports` notamment) absentes ou désynchronisées des migrations | incohérence documentaire |
| 14 | Architecture / API | Routes `/api/entries` + fichiers `entries.rs` vs table `notes`, UI et documentation « notes » (vérifié dans le code v0.1.0) | incohérence de terminologie |
| 15 | DATA_MODEL §2.6 | « unicité après trim garantie par la contrainte UNIQUE (url) » — le SQL ne trim pas ; la garantie vient du trim applicatif à l'écriture | imprécision |
| 16 | ROADMAP v0.6 | Le « préalable documentation (ACCOMPLI) » ne mentionnait aucune **porte de validation utilisateur** avant l'implémentation COT | processus |

**Observations de l'audit vérifiées comme déjà conformes (aucune correction nécessaire)** :
séparation Python → JSON → Rust → SQLite → Leptos (ARCHITECTURE §4.5, conforme) ; absence
d'exécution du moteur depuis le frontend ou le backend (conforme, H1/H10/T13) ; moteur COT
reconnu comme fonctionnalité existante à intégrer (REQUIREMENTS §10.2, module H, ROADMAP
v0.6 — conforms) ; RateProbability externe (§10.2, T16 — conforme) ; intégration COT ne
bloque pas le MVP (v0.2 « hors périmètre » explicite — conforme) ; v0.6 à sa place
(conforme).

## 3. Corrections réalisées

### `COT_ENGINE.md` (révision majeure)
- **§5.1** : méthode de quantile **épinglée** (interpolation linéaire, formule in extenso),
  traitement des valeurs nulles, arrondi, `window_observations` (D7/D8).
- **§5.2** : référentiel des 8 marchés avec **identifiants stables**, table d'alias,
  marché attendu introuvable = bloquant, marché inconnu = jamais ajouté (D5) ; NZD marqué
  À VALIDER (Q1).
- **§5.5** : nouveau défaut documenté (i) — suppression silencieuse des lignes incomplètes
  par `dropna` ; correction prévue (journalisation `incomplete_row`).
- **§5.6** (nouveau) : distinction marché totalement absent (`missing_market`, bloquant) vs
  semaine manquante (`missing_market_week`, informatif).
- **§6** (réécrit) : schéma JSON **complet et versionné** — chaque champ avec type, statut
  (requis / optionnel / requis-nullable), unité, règle de validation ; exemple corrigé
  (rang retiré, `window_observations`, `method_version`, `stats_methods`, cas `published_at`
  null + anomalie, `net_pct_oi` null si OI ≤ 0) ; **identités arithmétiques** formalisées
  (Net = Long − Short ; % OI ; 0 ≤ Long/Short ≤ OI ; doublons) ; **versionnement à trois
  niveaux** (§6.6, D9).
- **§7** (réécrit) : réception + **SHA-256** (D11) ; **validation intégrale avant
  persistance** et **transaction unique** (D1) ; référentiels sans création silencieuse
  (D5) ; upsert avec `first/last_import_id` (D4) ; métriques remplacées + **vérification
  par échantillonnage** (D10) ; journalisation (compteurs, trois dates, D2).
- **§8** (réécrit) : **trois** mesures distinguées (percentile historique / position
  actuelle dans l'histoire propre / classement transversal), provenances étiquetées
  (concept [FORMATION] vs formule [AMÉLIORATION]) ; formule du rang avec égalités, marchés
  manquants (décompte « x/7 ») et **exclusion du DXY** (D6).
- **§9** : le moteur ne calcule pas le classement transversal.

### `DATA_MODEL.md` (révision ciblée)
- **§1** : diagramme ER **aligné exactement sur les migrations** — `note_revisions`
  complet, `deleted_at`, entités `cot_imports` et `cot_market_aliases`, champs COT
  corrigés (`net_pct_oi` nullable, anomalies, FK d'import, `window_observations`,
  `method_version`).
- **§2.6** : formulation corrigée (trim applicatif ↔ unicité SQL des valeurs stockées).
- **§2.8** : mis à jour (référentiel seedé + alias, traçabilité, trois dates, dérivés
  recalculés, classement non stocké, `net_pct_oi` nullable avec justification).
- **§3, migration 0006** (réécrite, exécutable) : seed des 8 marchés (ids 1–8,
  `currency_id` par sous-requête), `cot_market_aliases`, `cot_imports` complété (3 dates,
  SHA-256, compteurs), `cot_observations` (nullable, anomalies, `first/last_import_id`,
  CHECKs ≥ 0), `cot_metrics` (CHECK fermé sur les deux percentiles,
  `window_observations`, `method_version`, index).
- **§3, migration 0002** : note d'implémentation complétée — compatibilité **vérifiée**
  contre la 0001 réelle.
- **§4** : règles d'import réécrites (validation intégrale, transaction unique, recalcul
  des dérivés, SHA-256) ; URL non vide après trim.
- **§5** : export JSON incluant `cot_market_aliases`.
- **§6** : deux requêtes type ajoutées (classement transversal à la lecture ; traçabilité
  d'une valeur via `last_import_id`).

### `ARCHITECTURE.md` (corrections ciblées)
- **§4.1 / §4.3 / §8** : renommage cohérent `entries` → `notes` planifié v0.2 (routes
  `/api/notes`, types `Note`/`NoteDraft`, fichiers `routes/notes.rs`/`services/notes.rs`),
  avec mention explicite de l'état v0.1.0 (D12) ; dette n° 9 ajoutée.
- **§4.3** : routes v0.6 précisées (`summary` avec classement calculé à la lecture ;
  `POST /api/cot/imports` avec validation intégrale puis transaction unique).
- **§4.5** : **chaîne complète explicite** CFTC → Python (manuel) → JSON → import manuel →
  validation → transaction → Leptos ; rôles précisés (backend : recalcul des dérivés
  simples, classement à la lecture).

### `REQUIREMENTS.md` (corrections ciblées)
- Étiquette **[RÉVISION]** définie dans la légende des origines.
- Module H : mention explicite « moteur Python EXISTANT [OUTIL PERSO], intégré tel quel ».
- **H6** précisée (trois mesures ; fenêtre 52 observations ; formule = formalisation
  Macro Journal, jamais attribuée au formateur).
- **H12** (dérivés non crus aveuglément) et **H13** (import atomique et intégral, pas de
  création silencieuse de marchés) **ajoutées** [RÉVISION].
- §11 : ligne de risque « import de données incohérentes » ajoutée ; parade DXY complétée
  (exclusion du classement, décompte affiché).

### `ROADMAP.md` (corrections ciblées)
- En-tête : note de révision 2026-11-03 (place de la v0.6 et priorités **inchangées**).
- v0.2 : item technique « renommage entries → notes » (D12).
- v0.6 : **étape 0 « porte de validation »** (validation du registre et des points À
  VALIDER ; relecture conjointe contrat + modèle **avant** toute implémentation) ;
  contenu fonctionnel aligné sur les spécifications révisées ; liste de validation
  complétée (fichier corrompu → rejet intégral ; test `metric_mismatch`).

### `COURSE_METHOD.md` (retouches minimales — cohérence méthodologique uniquement)
- §7 : paragraphe provenance complété — classement transversal : **concept enseigné**,
  formule = formalisation Macro Journal calculée par l'application.
- §14 : ligne « Positionnement COT chiffré » reformulée (provenances séparées).
- §15.3 : la formalisation du rang et son calcul applicatif ajoutés aux améliorations
  [AMÉLIORATION]. **Aucun autre passage modifié** — la structure de la formation, les
  durées corrigées (8 h 58) et les contenus enseignés restent intacts.

### `OLD_JOURNAL_AUDIT.md` — **inchangé**
Aucune erreur factuelle démontrée n'a été trouvée : la note complémentaire COT y est
exacte (l'outil Python existait hors du carnet, le carnet lui-même reste sans
positionnement/sources/historique). Réécrire ce document pour « uniformiser » aurait
contrairement à la consigne — il décrit des **faits observés** dans les PDF, pas une
spécification.

## 4. Observations contestées ou nuancées

Aucune observation de l'audit n'a été écartée : **toutes celles vérifiées se sont
avérées fondées** (§2). Deux points ont été traités **plus largement** que la lettre de
l'observation, avec justification :

1. **« Corriger l'exemple JSON dont le rang 88.0 est incompatible avec la formule »** —
   l'incompatibilité est confirmée (erreur factuelle). Plutôt que de corriger la valeur
   seule, le rang a été **retiré du contrat JSON** et recalculé à la lecture par le
   backend (D6). Justification : le rang est une fonction pure des faits stockés ; le
   transporter duplique la formule en deux langages (risque de divergence permanent) et
   contredit l'exigence de l'audit « ne pas faire confiance aveuglément aux métriques
   dérivées envoyées par Python » ; surtout, **aucun des deux scripts ne calcule de rang
   aujourd'hui** — il ne s'agit donc pas de déplacer une fonctionnalité existante du
   moteur, mais de choisir où une métrique nouvelle vivra. La fonctionnalité UI (H5/H6)
   et la priorité v0.6 sont conservées.
2. **« Évaluer l'intérêt d'un SHA-256 »** — évaluation faite (D11) : adopté pour la
   détection de ré-import (réponse `409` + confirmation explicite exigée), et non comme
   un rejet définitif, l'idempotence rendant le ré-import inoffensif pour les données.

Par ailleurs, plusieurs exigences de l'audit ont été vérifiées **déjà conformes** dans les
documents (§2, liste finale) — elles sont listées comme telles plutôt que « corrigées »,
pour ne pas laisser croire à des changements qui n'ont pas eu lieu.

## 5. Décisions encore à valider

Reprises de `SPECIFICATION_DECISIONS.md` (§ Questions ouvertes) :

| # | Décision à valider | Par qui | Bloque quoi |
|---|---|---|---|
| Q1 | Nom CFTC exact du marché NZD (2019–2026) — vérification sur fichier réel | Utilisateur (données réelles) | Export moteur (l'alias la rend non bloquante côté base) |
| Q2 | Emplacement `tools/cot/` vs dépôt séparé | Utilisateur | Organisation du dépôt uniquement |
| Q3 | Plage d'années 2019 → courant | Utilisateur (confirmation) | Profondeur d'historique |
| Q4 | Renommage `/api/entries` → `/api/notes` en v0.2 (D12) | Utilisateur | Vocabulaire API ; si refusé, l'écart est documenté comme assumé |
| Q5 | Provenance du commentaire « Style ForexFondamentale » | Utilisateur (mémoire) | Attribution méthodologique uniquement |
| Q6 | Tolérances de vérification (±0,005 / ±0,01) | À l'usage | Faux positifs d'anomalies |

## 6. Dépendances envers les scripts Python — ce qui a pu être vérifié, et ce qui ne peut pas l'être

**Vérifié par lecture intégrale des deux scripts** (les deux fichiers sont présents dans
`work/cot-engine-reference/` — l'audit a bien porté sur le code réel) :
- noms de colonnes utilisés (deux conventions : officielle CFTC dans A, soulignée dans B) ;
- filtres de marchés (égalité exacte dans A ; `str.contains` + regex dans B, avec les
  capturations parasites documentées) ;
- calculs (Net % OI seulement ; le net n'est jamais materialisé) ;
- dates (référence seule ; publication et récupération absentes) ;
- percentiles (`tail(52).quantile()` → lignes droites : **look-ahead confirmé**) ;
- différences entre versions (USD présent seulement dans A ; seuils 10 vs 5 ; imports
  morts dans B ; sortie PNG vs debug console).

**Non vérifiable sans exécution** (et donc **non affirmé** dans les documents) :
- la convention de colonnes réellement produite par la version actuelle du paquet
  `cot_reports` (la résolution double-convention est spécifiée, §5.3, mais non testée) ;
- le nom exact du marché NZD dans les fichiers réels 2019–2026 (Q1) ;
- la qualité réelle des fichiers CFTC anciens (lignes incomplètes, OI nuls) — les
  garde-fous sont spécifiés mais leur fréquence réelle est inconnue ;
- le comportement précis de `cot.cot_year` (erreurs réseau, années partielles).

Le durcissement du moteur (v0.6) inclut un **échantillon doré** et des tests pour couvrir
ces points au moment de l'implémentation.

## 7. Cohérence finale

| Composant | Contrat attendu | Implémentation concernée | État de validation |
|---|---|---|---|
| Format d'échange JSON v1 | `COT_ENGINE.md` §6 (schéma complet : champs, types, statuts, unités, identités arithmétiques, versionnement 3 niveaux) | Moteur Python (export) + backend Rust (import) | **Spécifié et cohérent** ; à implémenter en v0.6 après porte de validation |
| Moteur Python COT | `COT_ENGINE.md` §2–§5 (défauts documentés, corrections spécifiées : as-of, référentiel/alias, dates, robustesse) | `script_A_dashboard.py` / `script_B_debug_variant.py` (référence auditée par lecture) + moteur durci | **Audit fait** ; durcissement à faire (v0.6.1) ; Q1/Q2/Q3 ouvertes |
| Import Rust | `COT_ENGINE.md` §7 (validation intégrale, transaction unique, référentiels, upsert, SHA-256, journalisation) | `apps/server` (v0.6) — n'existe pas encore | **Spécifié** ; à implémenter (v0.6.2) |
| Stockage SQLite COT | `DATA_MODEL.md` §3-0006 (migration exécutable : seed 8 marchés, alias, 3 dates, traçabilité, nullable, anomalies) | `apps/server/migrations/0006_*.sql` — n'existe pas encore | **Spécifié et vérifié exécutable sur le papier** ; à tester sur copie de base réelle (H11) |
| Modèle notes/révisions/sources | `DATA_MODEL.md` §1–§4 (diagramme = migrations, exactement) | `apps/server` (v0.2) + migration 0002 | **Spécifié** ; migration 0002 vérifiée contre 0001 réelle |
| API REST | `ARCHITECTURE.md` §4.3 (routes `/api/notes` en v0.2, v0.6 COT) | `apps/server/src/main.rs` (v0.1.0 : `/api/entries`) | **Spécifié** ; renommage planifié v0.2 (Q4) |
| Frontend | `ARCHITECTURE.md` §4.2 + ROADMAP (blocs COT, étiquette proxy, décompte x/7) | `apps/web` (v0.6) | **Spécifié** ; à implémenter (v0.6.3) |
| Roadmap / priorités | `ROADMAP.md` (MVP v0.2 non impacté ; v0.6 inchangée dans l'ordre ; porte de validation étape 0) | — | **Cohérente** avec REQUIREMENTS (priorités inchangées) |
| Méthodologie | `COURSE_METHOD.md` §7/§14/§15 (enseigné / [OUTIL PERSO] / [AMÉLIORATION] distingués) | `COT_ENGINE.md` §8 (provenances alignées) | **Cohérente** |
| Audit historique | `OLD_JOURNAL_AUDIT.md` (inchangé — aucun fait nouveau ni erreur) | — | **Cohérent** tel quel |

Vérifications croisées finales effectuées : plus aucune occurrence de `window_weeks`,
`cross_sectional_rank` (champ JSON/SQL), `net_pct_oi ... NOT NULL`, « rang 88 » ni
« marché inconnu ... inséré » dans le jeu documentaire ; les références croisées
(§, D1–D14, H1–H13, Q1–Q6) ont été reliées.

## 8. Verdict

**Les spécifications sont prêtes pour passer au développement du MVP v0.2** : il ne
dépend d'aucune décision COT ni d'aucune question ouverte (Q1–Q6), et sa documentation
(migrations 0002, API, modèle notes) est cohérente et vérifiée contre le code existant —
sous réserve de la confirmation Q4 (renommage `/api/notes`), qui est un choix de
vocabulaire sans impact fonctionnel.

**Le module COT (v0.6) est prêt pour l'implémentation sous conditions** : la porte de
validation (ROADMAP v0.6, étape 0) exige d'abord la validation par l'utilisateur du
registre des décisions et des points ouverts Q1 (nom NZD — dépend de données réelles,
résoluble au durcissement du moteur via la table d'alias), Q2 (emplacement du moteur),
Q3 (plage d'années) et Q4 (renommage API). Q5 et Q6 sont non bloquantes.

Aucune incohérence connue ne subsiste entre le contrat JSON, le modèle SQLite, les règles
Python et l'architecture Rust à l'issue de cette révision.
