# DATA_MODEL — Modèle de données et schéma SQLite

> **Principes directeurs** (dérivés du cahier des charges, de la formation et de l'audit des
> carnets) :
> 1. **Relier plutôt que copier** : un événement concerne plusieurs devises et alimente
>    plusieurs analyses ; rien n'est dupliqué.
> 2. **Faits et interprétations séparés** : champs distincts, jamais mélangés.
> 3. **L'incertitude est légitime** : tout champ d'opinion est facultatif ; « indéterminé »
>    est une valeur de biais.
> 4. **L'histoire est conservée** : révisions des notes, versions des narratifs.
> 5. **Grouper quand ça simplifie réellement** : hebdo / paire / review / synthèse devise =
>    des **notes** avec un `kind` ; seuls les narratifs reçoivent une entité dédiée (leur
>    versionnement explicite est une fonctionnalité centrale).
> 6. Types SQL simples, dates ISO 8601 TEXT (cohérent avec l'existant `strftime`), clés
>    étrangères activées.

---

## 1. Diagramme des relations

```mermaid
erDiagram
    currencies ||--o{ note_currencies : "associe"
    notes       ||--o{ note_currencies : ""
    notes       ||--o{ note_tags : ""
    tags        ||--o{ note_tags : ""
    notes       ||--o{ note_sources : ""
    sources     ||--o{ note_sources : ""
    notes       ||--o{ note_revisions : "archive chaque version"
    events      }o--||| sources : "optionnellement"
    currencies  ||--o{ event_currencies : ""
    events      ||--o{ event_currencies : ""
    currencies  ||--o{ narratives : "1 narratif = 1 devise"
    narratives  ||--o{ narrative_versions : "v1..vn datées"
    events      ||--o{ event_narratives : "déclenche/éclaire"
    currencies  ||--o{ cot_markets : "moteur COT (v0.6)"
    cot_markets ||--o{ cot_observations : "1 par marché et semaine"
    cot_observations ||--o{ cot_metrics : "percentiles, rangs"

    currencies {
        INTEGER id PK
        TEXT code UK "USD..NZD, CNY"
        TEXT name
        TEXT central_bank_name
        TEXT role "major | external"
        INTEGER sort_order
    }
    notes {
        INTEGER id PK
        TEXT kind "journal|weekly|pair|review|currency"
        TEXT title
        TEXT content "Markdown libre"
        TEXT status "draft|final"
        TEXT bias "bullish|bearish|neutral|undetermined|NULL"
        DATE period_start "hebdo : lundi ISO"
        TEXT created_at
        TEXT updated_at
    }
    note_currencies {
        INTEGER note_id FK
        INTEGER currency_id FK
        TEXT role "base|quote (notes pair) ou NULL"
    }
    tags {
        INTEGER id PK
        TEXT name UK
    }
    sources {
        INTEGER id PK
        TEXT title
        TEXT url
        TEXT publisher
        TEXT kind "article|official|data|video|other"
        DATE published_at "publication"
        TEXT accessed_at "consultation/enregistrement"
        TEXT notes
    }
    note_revisions {
        INTEGER id PK
        INTEGER note_id FK
        TEXT title
        TEXT content
        TEXT saved_at
    }
    events {
        INTEGER id PK
        TEXT title
        TEXT event_ts "date+heure de l'événement"
        TEXT category "taux|inflation|emploi|croissance|pmi|discours|geopolitique|positionnement|autre"
        INTEGER source_id FK "NULL"
        TEXT actual "facultatif"
        TEXT consensus "facultatif"
        TEXT previous "facultatif"
        TEXT previous_revised "facultatif"
        TEXT unit "ex: % YoY, k emplois"
        TEXT factual_summary "resume factuel"
        TEXT interpretation "champ d'opinion"
        TEXT macro_implication "champ d'opinion"
        TEXT policy_implication "champ d'opinion"
        TEXT market_reaction "champ d'opinion"
        TEXT created_at
        TEXT updated_at
    }
    narratives {
        INTEGER id PK
        INTEGER currency_id FK
        TEXT title
        TEXT status "building|active|challenged|superseded|retired"
        TEXT created_at
        TEXT updated_at
    }
    narrative_versions {
        INTEGER id PK
        INTEGER narrative_id FK
        INTEGER version_no
        TEXT content "arguments, donnees, consequence monetaire, ce qui change"
        TEXT change_reason "motif explicite"
        TEXT created_at
    }
    cot_markets {
        INTEGER id PK
        TEXT market_name UK "nom officiel CFTC exact"
        INTEGER currency_id FK
        INTEGER is_usd_proxy "1 = contrat ICE DXY (proxy pondere)"
    }
    cot_observations {
        INTEGER id PK
        INTEGER market_id FK
        TEXT report_type "legacy_fut"
        TEXT ref_date "mardi : date des positions"
        TEXT published_at "vendredi : publication CFTC"
        INTEGER long_noncomm
        INTEGER short_noncomm
        INTEGER open_interest
        INTEGER net_noncomm "derive stocke"
        REAL net_pct_oi "derive stocke"
    }
    cot_metrics {
        INTEGER id PK
        INTEGER observation_id FK
        TEXT metric "pctl_20_52w | pctl_80_52w | cross_rank"
        INTEGER window_weeks "NULL pour le rang transversal"
        REAL value
        TEXT computed_at "date de calcul cote moteur"
    }
```

## 2. Choix de modélisation et justifications

### 2.1 Une table `notes` polymorphe par `kind` (et non 4 tables)

`journal` (défaut), `weekly`, `pair`, `review`, `currency` (synthèse d'une devise).
- **Pour** : même contenu (titre + Markdown + dates + liens devises/tags/sources), même API,
  même éditeur ; évite 4 CRUD quasi identiques ; l'utilisateur regroupe lui-même ses vues.
- **Différenciation minimale** : `period_start` (hebdo), `role base/quote` dans
  `note_currencies` (paires), `bias` (facultatif, partout où un biais est exprimé).
- **Contre-exemple assumé** : `narratives` **n'est pas** un kind de note, car l'identité
  longue durée + le versionnement explicite + le statut + les liens événements dépassent le
  modèle « note » ; en faire des notes obligerait à détourner les révisions.

### 2.2 Chine : `currencies.role = 'external'`

L'ancien carnet réservait une 9ᵉ ligne à la Chine (facteur externe AUD/NZD). Plutôt qu'une
table « facteurs externes », une simple valeur de rôle : même mécanique de liens, affichage
séparé (« À surveiller »), aucune table supplémentaire. Le référentiel est **sérialisé en
migration** (données de référence, pas de saisie utilisateur).

### 2.3 `note_revisions` : historique passif, périmètre défini

À chaque `UPDATE` d'une note, le service **archive d'abord** l'ancienne version, dans la
même transaction. **Champs couverts** (décision [ADDENDUM, nov. 2026] — l'ambiguïté est
levée) : `title`, `content`, `kind`, `status`, `bias`, `period_start` — soit tous les champs
scalaires éditables qui portent le raisonnement — plus `saved_at`, horodatage de sauvegarde
**de la version archivée** (copié de `notes.updated_at` au moment de l'archivage ; ce n'est
pas la date de l'archivage lui-même). **Champs non couverts** : les liens (devises, tags,
sources) — ils relèvent du classement, pas du raisonnement, et leur historisation
exigerait un modèle complet de versionnement des liens, explicitement exclu
(sur-ingénierie). Conséquence documentée : « à quelles devises/sources cette note
était-elle liée en mars » n'est pas reconstituable ; « que disais-je, avec quel statut et
quel biais » l'est. Consultation simple par note, chronologique. Pas de diff, pas de
branches, pas de restauration au MVP (une future action « restaurer » est triviale à
ajouter si le besoin apparaît). Ça suffit à « retrouver ce que je pensais à une date
donnée ».

### 2.4 `narratives` + `narrative_versions` : historique actif

Là où les révisions sont passives (on modifie une note), le narratif exige une **intention** :
chaque évolution significative ajoute une version avec un `change_reason` (nouvelle
publication, discours, élément contradictoire…). Le statut du narratif (`building`, `active`,
`challenged`, `superseded`, `retired`) traduit l'incertitude autorisée par le cahier des
charges. La dernière version est l'affichée ; les précédentes sont en lecture seule.

### 2.5 `events` : la fiche la plus structurée, volontairement

C'est le seul endroit où la structure est riche — parce que l'audit des carnets a montré que
la saisie libre y produisait des erreurs (typos, chiffres tronqués, dates mal formées) et que
le triplet réel/consensus/précédent est le cœur de la méthode d'interprétation.
- Champs chiffrés **TEXT facultatifs** (`actual`, `consensus`, `previous`,
  `previous_revised`, `unit`) : on préserve la saisie telle qu'elle est lue (« 3,3 % »,
  « 143k ») sans imposer un parseur numérique ni mentir sur une précision illusoire. `unit`
  explicite (« % YoY », « k, 3-mth avg ») corrige l'ambiguïté relevée dans les carnets.
- Les cinq champs narratifs sont **séparés** des faits : `factual_summary` décrit ce qui s'est
  passé ; `interpretation`, `macro_implication`, `policy_implication`, `market_reaction`
  sont des opinions — vidables et ré-ouvrables sans toucher aux faits.

### 2.6 `sources` : déduplication par URL, dates doubles

`published_at` (quand l'info est parue) ≠ `accessed_at` (quand je l'ai enregistrée) — exigence
explicite du cahier des charges. **Règle de déduplication alignée sur le SQL retenu
[ADDENDUM, nov. 2026]** : au MVP, unicité **exacte après `trim()`**, garantie par la
contrainte `UNIQUE (url)` ; le service ne crée pas de doublon et rattache la source
existante en cas d'URL identique. La déduplication fine (normalisation : casse du
schéma/hôte, fragment, slash final) reste une évolution reportée — voir `ROADMAP.md`
« Points ouvertement reportés ». Les devises concernées par une source se **déduisent** des
notes/événements qui l'utilisent (pas de table source↔devise : évite la redondance).

### 2.7 Ce qui est volontairement absent

- `note_events` (lier une note à un événement) : reporté en v0.5 avec le module Review, quand
  le parcours l'exige — pas de table spéculative.
- `note_cot` (lier une note à une observation COT) : les dossiers devise et les panneaux de
  paire affichent les observations via `currency_id` ; une table de liaison ne sera ajoutée
  que si un besoin réel se manifeste.
- Scores, notes numériques, KPI : exclus par principe.
- Utilisateur/compte : application mono-utilisateur locale.
- Pièces jointes/fichiers : hors périmètre actuel ; les sources portent les URLs.

### 2.8 Observations COT : données structurées, jamais des notes Markdown [ADDENDUM]

Le pilier 3 (positionnement) exige des séries numériques requêtables — dernières valeurs,
fraîcheur, historique, comparaison entre devises — ce que le Markdown ne permet pas. Les
observations issues du moteur Python vivent donc dans des tables dédiées (migration 0006) :

- **Clé d'unicité** : `(market_id, report_type, ref_date)` — un marché, un type de rapport,
  une date de référence CFTC (le mardi). Ré-importer le même fichier est idempotent.
- **Immutabilité attendue, corrections journalisées** : les données CFTC publiées ne
  changent normalement pas ; si un ré-import modifie une valeur existante (correction
  CFTC ou évolution de méthode du moteur), la modification est appliquée **et**
  journalisée comme anomalie dans `cot_imports.anomalies` — jamais silencieuse.
- **Faits bruts et mesures dérivées séparés** : `cot_observations` porte les valeurs
  publiées (Long/Short/OI) et deux dérivés immédiats stockés pour l'affichage
  (`net_noncomm`, `net_pct_oi`) ; tout le reste (percentiles glissants, rangs) vit dans
  `cot_metrics` avec sa fenêtre et sa date de calcul — la méthode peut évoluer sans
  corrompre les faits.
- **Mesures distinctes, jamais fusionnées** : percentile historique glissant (P20/P80 sur
  52 semaines — notre implémentation) ≠ rang transversal enseigné (classement 80–100 des
  devises entre elles) ≠ niveau institutionnel CME (enseigné, externe). Voir
  `COT_ENGINE.md` §8.
- **Proxy USD documenté** : la ligne USD provient du contrat ICE U.S. Dollar Index (DXY,
  pondéré) — `cot_markets.is_usd_proxy = 1`, étiquette visible dans l'UI ; ce n'est pas
  l'agrégat USD non pondéré enseigné par la formation.
- **Pas de `fetched_at` par observation** : la date de récupération appartient à l'import
  (`cot_imports.imported_at` + le champ `retrieved_at` du fichier d'échange) ; une même
  récupération couvre toutes les observations d'un fichier.

## 3. Schéma SQL (migrations cibles)

### Migration 0002 (MVP v0.2) — devise, liens, sources, révisions

```sql
-- Référentiel des devises (8 majeures + Chine en rôle externe)
CREATE TABLE IF NOT EXISTS currencies (
    id          INTEGER PRIMARY KEY,
    code        TEXT NOT NULL UNIQUE,          -- 'USD' ... 'NZD', 'CNY'
    name        TEXT NOT NULL,                 -- 'Dollar américain' ...
    central_bank_name TEXT NOT NULL,           -- 'Federal Reserve' ...
    role        TEXT NOT NULL DEFAULT 'major'
                CHECK (role IN ('major', 'external')),
    sort_order  INTEGER NOT NULL DEFAULT 0
);

INSERT INTO currencies (code, name, central_bank_name, role, sort_order) VALUES
    ('USD', 'Dollar américain',  'Federal Reserve',            'major', 1),
    ('EUR', 'Euro',              'European Central Bank',      'major', 2),
    ('GBP', 'Livre sterling',    'Bank of England',            'major', 3),
    ('JPY', 'Yen japonais',      'Bank of Japan',              'major', 4),
    ('CHF', 'Franc suisse',      'Swiss National Bank',        'major', 5),
    ('CAD', 'Dollar canadien',   'Bank of Canada',             'major', 6),
    ('AUD', 'Dollar australien', 'Reserve Bank of Australia',  'major', 7),
    ('NZD', 'Dollar néo-zélandais', 'Reserve Bank of New Zealand', 'major', 8),
    ('CNY', 'Yuan renminbi',     "People's Bank of China",     'external', 9);

-- Métadonnées des notes existantes — ambiguïté entries/notes LEVÉE [ADDENDUM, nov. 2026].
-- Décision : renommage explicite en `notes` (nom fidèle au domaine, utilisé par toute la
-- documentation et l'UI). Non destructif, versionné ; aucune table ne référence encore
-- `entries`, le coût de migration est nul. L'index attaché suit le renommage.
ALTER TABLE entries RENAME TO notes;

-- Colonnes nouvelles : ADD COLUMN simples (pas de CHECK posable en ADD COLUMN sous
-- SQLite — les valeurs autorisées sont validées par le service, cf. §4).
ALTER TABLE notes ADD COLUMN kind         TEXT NOT NULL DEFAULT 'journal';
ALTER TABLE notes ADD COLUMN status       TEXT NOT NULL DEFAULT 'draft';
ALTER TABLE notes ADD COLUMN bias         TEXT;     -- bullish|bearish|neutral|undetermined
ALTER TABLE notes ADD COLUMN period_start TEXT;     -- hebdo : lundi ISO
ALTER TABLE notes ADD COLUMN deleted_at   TEXT;     -- suppression logique (corbeille)

-- Index renommé pour la cohérence (l'ancien, hérité du renommage, est retiré).
CREATE INDEX IF NOT EXISTS idx_notes_updated ON notes (updated_at DESC);
DROP INDEX IF EXISTS idx_entries_updated_at;
```

> **Note d'implémentation [ADDENDUM, nov. 2026 — placeholder supprimé]** : l'ancien
> pseudo-SQL `notes_extra ...` de ce document était un placeholder non exécutable ; il a
> été remplacé par les `ALTER TABLE` réels ci-dessus. Le renommage `entries → notes` est
> la décision retenue ; si un coût imprévu apparaissait à l'implémentation, la décision se
> reprendrait explicitement (pas de repli silencieux vers `entries`).

```sql
CREATE TABLE IF NOT EXISTS note_currencies (
    note_id      INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    currency_id  INTEGER NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    role         TEXT CHECK (role IN ('base', 'quote') OR role IS NULL),
    PRIMARY KEY (note_id, currency_id)
);

CREATE TABLE IF NOT EXISTS tags (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS note_tags (
    note_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

CREATE TABLE IF NOT EXISTS sources (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    title        TEXT NOT NULL,
    url          TEXT NOT NULL UNIQUE,
    publisher    TEXT,
    kind         TEXT NOT NULL DEFAULT 'other'
                 CHECK (kind IN ('article', 'official', 'data', 'video', 'other')),
    published_at TEXT,                          -- date de publication (ISO)
    accessed_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    notes        TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS note_sources (
    note_id   INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    source_id INTEGER NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, source_id)
);

CREATE TABLE IF NOT EXISTS note_revisions (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    note_id      INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    title        TEXT NOT NULL,
    content      TEXT NOT NULL,
    kind         TEXT NOT NULL,           -- champs couverts : voir §2.3
    status       TEXT NOT NULL,
    bias         TEXT,
    period_start TEXT,
    saved_at     TEXT NOT NULL            -- sauvegarde de CETTE version (copie de updated_at)
);
CREATE INDEX IF NOT EXISTS idx_note_revisions_note ON note_revisions (note_id, saved_at);

-- index de recherche/filtrage
CREATE INDEX IF NOT EXISTS idx_notes_kind      ON notes (kind);
-- (idx_notes_updated est déjà créé par le bloc de renommage ci-dessus)
CREATE INDEX IF NOT EXISTS idx_note_currencies_currency ON note_currencies (currency_id);
```

### Migration 0003 (v0.3) — événements

```sql
CREATE TABLE IF NOT EXISTS events (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    title            TEXT NOT NULL,
    event_ts         TEXT NOT NULL,             -- '2025-02-12T13:30:00Z'
    category         TEXT NOT NULL DEFAULT 'autre'
                     CHECK (category IN ('taux','inflation','emploi','croissance',
                                         'pmi','discours','geopolitique',
                                         'positionnement','autre')),
    source_id        INTEGER REFERENCES sources(id) ON DELETE SET NULL,
    actual           TEXT, consensus TEXT, previous TEXT, previous_revised TEXT,
    unit             TEXT,
    factual_summary  TEXT NOT NULL DEFAULT '',
    interpretation   TEXT, macro_implication TEXT,
    policy_implication TEXT, market_reaction TEXT,
    created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_events_ts        ON events (event_ts DESC);
CREATE INDEX IF NOT EXISTS idx_events_category  ON events (category);

CREATE TABLE IF NOT EXISTS event_currencies (
    event_id    INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    currency_id INTEGER NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    PRIMARY KEY (event_id, currency_id)
);
```

### Migration 0004 (v0.4) — narratifs

```sql
CREATE TABLE IF NOT EXISTS narratives (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    currency_id  INTEGER NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    title        TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'building'
                 CHECK (status IN ('building','active','challenged',
                                   'superseded','retired')),
    created_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_narratives_currency ON narratives (currency_id, status);

CREATE TABLE IF NOT EXISTS narrative_versions (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    narrative_id  INTEGER NOT NULL REFERENCES narratives(id) ON DELETE CASCADE,
    version_no    INTEGER NOT NULL,
    content       TEXT NOT NULL,
    change_reason TEXT,
    created_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (narrative_id, version_no)
);

CREATE TABLE IF NOT EXISTS event_narratives (
    event_id      INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    narrative_id  INTEGER NOT NULL REFERENCES narratives(id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, narrative_id)
);
```

### Migration 0005 (v0.5) — liens notes ↔ événements (Review)

```sql
CREATE TABLE IF NOT EXISTS note_events (
    note_id  INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    PRIMARY KEY (note_id, event_id)
);
```

### Migration 0006 (v0.6) — observations COT [ADDENDUM]

```sql
-- Référentiel des marchés suivis par le moteur Python (noms CFTC exacts).
-- Sérialisé en migration (données de référence) ; le nom exact du marché NZD
-- ('NEW ZEALAND DOLLAR - ...' vs 'NZ DOLLAR - ...') a varié selon les époques du
-- fichier CFTC : la valeur retenue sera vérifiée sur le fichier réel au durcissement
-- du moteur (voir COT_ENGINE.md §5.2).
CREATE TABLE IF NOT EXISTS cot_markets (
    id           INTEGER PRIMARY KEY,
    market_name  TEXT NOT NULL UNIQUE,   -- ex. 'EURO FX - CHICAGO MERCANTILE EXCHANGE'
    currency_id  INTEGER NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    is_usd_proxy INTEGER NOT NULL DEFAULT 0,  -- 1 = ICE U.S. Dollar Index (DXY pondéré)
    proxy_note   TEXT
);

-- Journal des imports : traçabilité, fraîcheur, anomalies.
CREATE TABLE IF NOT EXISTS cot_imports (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    imported_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    format_version TEXT NOT NULL,
    generator      TEXT NOT NULL,             -- ex. 'cot-engine 0.2.0'
    source_file    TEXT,
    observations   INTEGER NOT NULL,          -- nombre d'observations importées
    anomalies      TEXT NOT NULL DEFAULT '[]' -- JSON : corrections/écarts détectés
);

-- Observations hebdomadaires : faits publiés + deux dérivés immédiats.
CREATE TABLE IF NOT EXISTS cot_observations (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    market_id     INTEGER NOT NULL REFERENCES cot_markets(id) ON DELETE CASCADE,
    report_type   TEXT NOT NULL DEFAULT 'legacy_fut',
    ref_date      TEXT NOT NULL,              -- mardi : date de référence des positions
    published_at  TEXT,                       -- vendredi : publication CFTC (si dispo)
    long_noncomm  INTEGER NOT NULL,
    short_noncomm INTEGER NOT NULL,
    open_interest INTEGER NOT NULL,
    net_noncomm   INTEGER NOT NULL,           -- long - short (dérivé stocké)
    net_pct_oi    REAL NOT NULL,              -- net / OI × 100 (dérivé stocké)
    UNIQUE (market_id, report_type, ref_date)
);
CREATE INDEX IF NOT EXISTS idx_cot_obs_ref_date ON cot_observations (ref_date DESC);

-- Mesures dérivées (percentiles glissants, rangs) — méthode versionnée par métrique.
CREATE TABLE IF NOT EXISTS cot_metrics (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    observation_id INTEGER NOT NULL REFERENCES cot_observations(id) ON DELETE CASCADE,
    metric         TEXT NOT NULL,   -- 'net_pct_oi_pctl_20' | 'net_pct_oi_pctl_80'
                                   -- | 'cross_sectional_rank'
    window_weeks   INTEGER,         -- 52 pour les percentiles ; NULL pour le rang
    value          REAL NOT NULL,
    computed_at    TEXT NOT NULL,   -- date de calcul côté moteur Python
    UNIQUE (observation_id, metric, window_weeks)
);
```

Règles associées : upsert par `(market_id, report_type, ref_date)` ; toute valeur modifiée
par un ré-import est journalisée dans `cot_imports.anomalies` (H4) ; les métriques sont
recalculées par le moteur puis remplacées (DELETE + INSERT par observation importée) — les
faits, eux, ne sont jamais modifiés silencieusement.

## 4. Règles métier côté service (pas de triggers)

- Note : titre non vide (trim) ; **suppression logique** [ADDENDUM, nov. 2026 — décision
  explicite] : « Supprimer » (après confirmation UI) pose `deleted_at` ; la note disparaît
  des listes et recherches mais **son historique (`note_revisions`) est conservé**. La
  corbeille (Settings) permet de restaurer (`deleted_at = NULL`) ou de purger (DELETE
  physique avec cascade sur liens et révisions, précédé d'un avertissement et d'une
  proposition d'export de la note). Raison : « ne pas détruire involontairement
  l'historique » prime sur la simplicité absolue ; le coût est une colonne nullable + un
  prédicat de filtre par requête.
- Mise à jour de note : INSERT de l'ancienne version dans `note_revisions` **avant**
  l'UPDATE, dans la même transaction (champs couverts : §2.3 ; `saved_at` copié depuis
  `notes.updated_at`).
- Paire : exactement deux `note_currencies` avec rôles `base`/`quote` distincts (validé en
  service, pas en SQL).
- Événement : `event_ts` parseable ISO ; champs chiffrés libres (TEXT) ; au moins une devise
  liée recommandée (avertissement, pas blocage).
- Narratif : `version_no` = max+1 calculé en transaction ; passer un narratif à
  `superseded` n'efface rien ; un narratif `active` peut coexister avec un `challenged` sur
  la même devise (arguments contradictoires autorisés).
- Tags : trim + minuscules + unicité ; sources : **unicité d'URL exacte après `trim()`**
  (contrainte `UNIQUE (url)` — voir §2.6 ; normalisation reportée).
- Import COT (v0.6) : fichier au format d'échange versionné refusé si `format_version`
  inconnu ; upsert idempotent ; anomalies journalisées, jamais silencieuses (H3, H4).

## 5. Persistance, sauvegarde, export

- **Emplacement** : `~/.local/share/macro-journal/macro_journal.db` (XDG), WAL activé,
  `foreign_keys = ON` (déjà en place), surcharge par `MACRO_JOURNAL_DB`.
- **Anti-perte** : sentinel `.initialized` à côté de la base (détails en
  `ARCHITECTURE.md` §5).
- **Export JSON** (`GET /api/export`) : `{meta: {version, generated_at}, currencies, notes,
  note_revisions, tags, sources, events, narratives, narrative_versions, cot_markets,
  cot_imports, cot_observations, cot_metrics (+ tables de liaison)}` — toutes les tables,
  format documenté et stable, testé par un test d'intégration qui re-charge le dump.
- **Snapshot SQLite** : endpoint `POST /api/backups` → `VACUUM INTO
  ~/.local/share/macro-journal/backups/macro_journal-YYYYMMDD-HHMM.db` ; liste et télé-
  chargement depuis Settings. Une sauvegarde locale simple et vérifiable suffit au stade du
  prototype (cahier des charges §9).
- **Import** : hors périmètre avant v1.0 (l'export JSON documenté en garantit la
  faisabilité future).

## 6. Requêtes type attendues (vérification d'adéquation du schéma)

- « Tout ce qui concerne l'USD depuis 2 semaines » : `notes JOIN note_currencies` +
  `events JOIN event_currencies`, tri par date. ✔
- « Narratifs actifs de l'EUR » : `narratives WHERE currency_id=? AND status IN
  ('building','active','challenged')` + dernière version. ✔
- « Paires où figure le JPY » : `notes kind='pair' JOIN note_currencies role base/quote`. ✔
- « Événements d'inflation USD du trimestre, avec surprise » : filtres catégorie+devise,
  comparaison `actual`/`consensus` faite à l'affichage (TEXT, pas en SQL). ✔
- « Ce que je pensais de l'GBP en février » : révisions des notes liées GBP sur la période +
  versions de narratifs GBP datées de la période. ✔
- « Positionnement COT actuel de l'USD et fraîcheur » : dernière observation par marché
  (`ref_date` max) + métriques associées + `cot_imports.imported_at`. ✔
- « % OI de l'EUR à chaque publication de janvier, avec P20/P80 contemporains » :
  `cot_observations JOIN cot_metrics` filtré sur la période (percentiles « as-of », sans
  donnée future). ✔
