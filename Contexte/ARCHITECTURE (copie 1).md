# ARCHITECTURE — Architecture cible de Macro Journal

> Ce document **étend** les décisions d'architecture initiales (v0.1.0) sans les invalider.
> Les sections 1–3 reprennent et précisent l'existant ; les sections suivantes décrivent la
> cible. Il est volontairement pragmatique : pas de couche générique, pas de trait
> superflu, lisible pour un apprenant Rust (chapitres 1–8 du Rust Book).

---

## 1. Principes non négociables

1. **Local-first** : écoute sur `127.0.0.1` uniquement, base SQLite locale, aucun compte,
   aucun service distant obligatoire.
2. **Outil de raisonnement, pas terminal de marché** : RateProbability, calendriers et
   research providers restent **externes** ; l'application stocke des références, des
   observations et le raisonnement — jamais une réplique des données. **Précision
   [ADDENDUM, nov. 2026]** : pour le COT, trois acteurs sont distingués — le CFTC
   (fournisseur externe des rapports bruts), notre moteur Python local (téléchargement,
   normalisation et calcul des mesures pour les 8 marchés suivis) et l'application Rust
   (stockage structuré des observations normalisées, consultation, liaison aux analyses).
   Stocker ce sous-ensemble dérivé, documenté et daté n'est pas une « réplique » au sens de
   ce principe ; répliquer l'intégralité de la base CFTC ou bâtir une plateforme COT
   généraliste le serait (voir §4.5 et `COT_ENGINE.md`).
3. **Faits et interprétations distingués** au niveau du schéma et de l'interface.
4. **L'histoire est conservée** : révisions des notes, versions des narratifs.
5. **Simplicité maintenue** : chaque ajout doit pouvoir être expliqué en une phrase ;
   toute abstraction doit attendre son deuxième usage concret.

## 2. Répartition des crates (inchangée, précisée)

```text
macro-journal/                 (workspace, resolver 2, edition 2021)
├── apps/
│   ├── server/                # macro-journal-server — Axum 0.8 + SQLx/SQLite
│   │   ├── src/main.rs        # aujourd'hui monolithique → cible : modules ci-dessous
│   │   └── migrations/        # SQL versionné (sqlx::migrate!)
│   └── web/                   # macro-journal-web — Leptos 0.8 CSR (Trunk)
│       ├── src/main.rs        # aujourd'hui monolithique → cible : modules ci-dessous
│       ├── assets/styles.css  # Dark Research
│       └── index.html
└── crates/
    └── shared/                # macro-journal-shared — types sérialisables communs
```

- `crates/shared` : **types de l'API uniquement** (DTO). Pas de logique métier, pas de
  dépendance à SQLx — il doit rester compilable côté wasm.
- `apps/server` : détient la base, la logique métier et la validation.
- `apps/web` : rendu et interactions ; aucune logique métier ; appel HTTP via `gloo-net`.

## 3. Décisions existantes conservées

- Leptos 0.8 CSR compilé par Trunk ; Axum 0.8 ; SQLx 0.9 (SQLite, WAL, foreign_keys) ;
  pulldown-cmark + ammonia pour l'aperçu Markdown assaini ; CORS restreint aux origines
  Trunk locales ; en mode « production locale », Axum sert `apps/web/dist`.
- Le README v0.1.0 documente honnêtement que la génération d'origine n'avait pas pu compiler
  (environnement sans toolchain) — voir §8 sur l'état de compilation réel.

## 4. Architecture logicielle cible

### 4.1 Backend (`apps/server/src/`)

Découpage par responsabilité, introduit **progressivement** lors du MVP (pas de grand soir) :

```text
src/
├── main.rs          # bootstrap : config, logging, pool, migrations, router, écoute
├── config.rs        # résolution des chemins (DB, dist) : env > XDG > défauts
├── error.rs         # ApiFailure (statut + message) — existe déjà, à déplacer ici
├── state.rs         # AppState { pool }
├── routes/
│   ├── mod.rs       # assemblage du Router
│   ├── health.rs
│   ├── entries.rs   # handlers notes (thin) → délèguent à services::
│   ├── sources.rs
│   ├── events.rs    # (v0.3)
│   ├── narratives.rs# (v0.4)
│   └── export.rs    # dump JSON
└── services/
    ├── mod.rs
    ├── entries.rs   # SQL + validation métier (titres, genres, biais, liens)
    ├── sources.rs   # déduplication par URL
    └── search.rs    # filtres/recherche (LIKE, puis FTS5 si besoin)
```

Règles : les **handlers** ne contiennent que l'extraction de la requête et la réponse ; le
**SQL vit dans les services** avec des requêtes lisibles et nommées ; les erreurs remontent
en `ApiFailure` typée (400 validation, 404 introuvable, 500 base).

### 4.2 Frontend (`apps/web/src/`)

```text
src/
├── main.rs          # montage
├── app.rs           # coquille : sidebar + topbar + routage de section
├── api.rs           # client HTTP (gloo-net) : une fonction par endpoint, erreurs en String
├── markdown.rs      # render_markdown (pulldown-cmark + ammonia) — existe déjà
├── pages/
│   ├── journal.rs   # liste + éditeur deux panneaux
│   ├── overview.rs
│   ├── currencies.rs / currency_detail.rs   (v0.3)
│   ├── weekly.rs    (v0.4)
│   ├── events.rs    (v0.3)
│   ├── pairs.rs     (v0.5)
│   ├── reviews.rs   (v0.5)
│   └── settings.rs  (v0.2 : emplacement DB, export, raccourcis)
└── components/
    ├── markdown_editor.rs   # éditeur + aperçu (réutilisé partout)
    ├── tag_input.rs
    ├── source_picker.rs
    ├── currency_picker.rs
    └── ...
```

- La navigation actuelle (signal de section) est conservée au MVP ; **`leptos_router`** est
  envisagé en v0.3 (URLs profondes par note/devise) quand le nombre de pages le justifie —
  décision explicite, pas un réflexe.
- Préférences visuelles (barre latérale repliée) persistées en `localStorage`.

### 4.3 Surface de l'API (cible MVP)

```text
GET    /api/health
GET    /api/currencies                      # référentiel 8 + CNY(externe)
GET    /api/entries?kind=&currency=&tag=&q=&after=&before=&status=
POST   /api/entries                         # + liens devises/tags/sources en une requête
GET    /api/entries/{id}                    # + relations
PUT    /api/entries/{id}                    # snapshot de l'ancienne version dans note_revisions
DELETE /api/entries/{id}
GET    /api/entries/{id}/revisions          # (consultation : v0.5)
GET    /api/tags
GET    /api/sources?q=
POST   /api/sources
PUT    /api/sources/{id}
DELETE /api/sources/{id}
GET    /api/export                          # dump JSON complet (sauvegarde vérifiable)
```

(v0.3 : `GET/POST/PUT/DELETE /api/events[...]` ; v0.4 : `/api/narratives`,
`/api/narratives/{id}/versions`.)

(v0.6, module H : `GET /api/cot/markets`, `GET /api/cot/observations?currency=&from=&to=`,
`GET /api/cot/summary` — dernière observation + métriques + fraîcheur par devise —,
`POST /api/cot/imports` (upload **manuel** du JSON produit par le moteur Python),
`GET /api/cot/imports` (historique des imports et anomalies). Spécification :
`COT_ENGINE.md`.)

### 4.4 Parcours utilisateurs servis

1. **Quotidien (le matin)** : ouvrir le journal → note du jour à partir du gabarit
   « avant session » → rédiger → lier devises/sources → enregistrer ; en fin de session,
   compléter la section « après session » (la révision d'aujourd'hui reste consultable).
2. **Week-end** : Weekly Outlook de la semaine à venir → consulter les narratifs actifs →
   noter événements attendus et informations manquantes.
3. **À froid (recherche)** : rechercher par devise/mot-clé → retrouver la note, ses sources,
   l'événement déclencheur, le narratif et ses versions → comprendre pourquoi l'analyse a
   changé.
4. **Review (mensuel)** : rouvrir les hebdos passées → confronter aux événements survenus →
   rédiger le bilan en distinguant raisonnement / données / interprétation / réaction du
   marché / résultat.

### 4.5 Moteur COT Python — composant indépendant (v0.6) [ADDENDUM]

Le traitement COT existant en Python (`cot_reports`, Legacy Futures) est **conservé** : il
n'est pas réécrit en Rust et ne doit pas dépendre de l'application WebAssembly.

- **Rôles** : CFTC = fournisseur externe des rapports bruts ; Python = téléchargement,
  extraction des 8 marchés suivis, calcul (Net Non-Commercial, % OI, percentiles) ;
  Rust = stockage SQLite, consultation, liaison aux dossiers devise et aux analyses.
- **Contrat** : le moteur produit un fichier **JSON au format d'échange versionné**
  (spécification et exemple : `COT_ENGINE.md` §6) ; le backend l'importe via un endpoint
  dédié. Aucun appel de Python par Rust, aucune orchestration du moteur par le backend.
- **Exécution manuelle, à la demande** : jamais de téléchargement automatique à l'ouverture
  de l'application (cohérent avec T13 : pas de traitement en arrière-plan).
- **Emplacement proposé [PROPOSITION]** : `tools/cot/` dans le dépôt, hors workspace Cargo
  (le code Python en est naturellement exclu), avec son propre `requirements.txt` épinglé
  — un seul lieu de sauvegarde, zéro couplage de compilation. Alternative : dépôt séparé.
- **Audit** : les deux scripts existants ont été examinés intégralement **avant** toute
  proposition de remplacement — défauts confirmés et corrections prévues dans
  `COT_ENGINE.md` (dont les percentiles réellement glissants « as-of »).

## 5. Stockage, chemins et configuration

- **Base** : `~/.local/share/macro-journal/macro_journal.db` (convention XDG), créée au
  premier lancement avec un fichier sentinelle `.initialized` dans le même répertoire.
  Variable d'environnement `MACRO_JOURNAL_DB` pour surcharger (chemin absolu).
- **Comportement anti-perte** : si le chemin attendu ne contient pas de base **et** que la
  sentinelle existe → erreur explicite au démarrage (« base attendue introuvable ») au lieu
  de recréer silencieusement une base vide. Premier lancement (pas de sentinelle) → création
  avec message de log clair.
- **Fichiers statiques** : `apps/web/dist` résolu depuis la racine du workspace (calculé, pas
  dépendant du répertoire courant).
- **Sauvegarde** : bouton Settings → `GET /api/export` (JSON daté de toutes les tables) ;
  recommandation documentée de copier le fichier `.db` (SQLite en WAL : copier aussi les
  `-wal`/`-shm`, ou utiliser `VACUUM INTO` — retenu : endpoint serveur `VACUUM INTO` un
  fichier horodaté dans `~/.local/share/macro-journal/backups/`, plus sûr qu'une copie de
  fichiers chauds).

## 6. Performance et ressources (cible matérielle : i5-6200U, 8 Go, SSD)

- Pool SQLx limité (5 connexions) ; requêtes indexées ; recherche LIKE acceptable à l'échelle
  personnelle (quelques milliers de notes), FTS5 seulement si la lenteur est constatée.
- Aucun polling automatique ; rechargement à la demande.
- Rendu Markdown à la frappe (déjà en place) ; un debounce pourra être ajouté si ressenti.
- WASM CSR : bundle raisonnable, pas de police/lourage externe (CSS local, système de fonts).

## 7. Stratégie de tests

- **Unitaires** (serveur) : validation des drafts, normalisation des tags, déduplication des
  sources, calcul des filtres.
- **Intégration** (serveur) : SQLite en mémoire (`sqlite::memory:`) + `sqlx::migrate!` →
  cycle complet note (création/mise à jour/révisions/suppression), événements, liens.
- **Frontend** : la compilation wasm reste le garde-fou principal au MVP ; des tests de
  rendu Markdown (pulldown + ammonia) peuvent vivre dans `crates/shared` ou un test unitaire
  côté web.
- Chaque version documente **ce qui a réellement été testé** (commandes exécutées).

## 8. État réel du prototype (audit v0.1.0 + patch sidebar)

**Ce qui fonctionne / est en place** (vérifié par lecture du code, non par exécution — voir
§9) :
- Workspace 3 crates, versions épinglées (Leptos 0.8.23, Axum 0.8.9, SQLx 0.9.0) ;
- CRUD notes complet côté API (list/get/create/update/delete) avec erreurs typées ;
- migration 0001 (`entries`) + index sur `updated_at` ;
- éditeur deux panneaux, aperçu assaini, états de sauvegarde, indicateur API ;
- barre latérale repliable (patch) avec `aria-expanded`, `prefers-reduced-motion` ;
- CORS local, `run-dev.sh` fonctionnel, toolchain pinné (+ wasm target).

**Écarts et dettes identifiés** :
1. `main.rs` serveur (≈ 250 lignes) et `main.rs` web (≈ 390 lignes) monolithiques →
   découpage prévu au §4, introduit en même temps que les fonctionnalités MVP.
2. Chemins relatifs (`data/macro_journal.db`, `apps/web/dist`) dépendants du répertoire
   courant → corrigé par `config.rs` (§5).
3. Aucune persistance de l'état de la barre latérale (retour à « affichée » au rechargement,
   documenté dans le PATCH_README) → `localStorage` au MVP.
4. Navigation sans « Settings » (prévue par le cahier des charges) → MVP.
5. `update_entry` écrase sans historique → `note_revisions` au MVP.
6. Aucun test, pas de `Cargo.lock` dans l'archive → MVP.
7. Chaînes résiduelles en anglais (placeholder de l'éditeur, « characters », « LIVE
   APERÇU ») → harmonisation française au MVP.
8. Petites faiblesses UI : pas de confirmation de suppression (l'API DELETE n'a pas de
   bouton), pas de recherche, pas de filtre.

**Composant annexe audité (nov. 2026 [ADDENDUM])** : le moteur COT Python personnel de
l'utilisateur — deux scripts (`cot_reports`, Legacy Futures : dashboard complet + variante
de debug) — ne fait pas partie du prototype Rust. Son audit intégral et son intégration
planifiée (v0.6, sans impact sur le MVP v0.2) sont documentés dans `COT_ENGINE.md`.

## 9. Véracité de l'état de compilation

- L'archive `macro-journal-v0.1.0.zip` contient `on:click=load_entries` (bouton
  « Actualiser ») avec une closure **sans paramètre d'événement** ; dans Leptos 0.8, un
  handler `on:` doit accepter l'événement — cette ligne telle quelle ne devrait pas compiler.
- Le patch sidebar contient la forme corrigée `on:click=move |_| load_entries()` et son
  README précise qu'il « conserve le correctif du gestionnaire Actualiser » : l'état qui a
  compilé sur la machine de l'utilisateur est donc **v0.1.0 + correctif**, et l'arborescence
  de travail de ce projet (v0.1.0 + patch appliqué) est l'état de référence.
- **Dans l'environnement d'audit (sandbox sans toolchain Rust), `cargo check` n'a pas pu être
  exécuté** : aucune affirmation de compilation n'est faite ici au-delà de la lecture du
  code. Première action de la Phase 3 : `cargo fmt && cargo clippy && cargo check` (natif +
  `--target wasm32-unknown-unknown`) sur la machine cible, puis versionnement du
  `Cargo.lock`.

## 10. Feuille de route d'implémentation

Voir `ROADMAP.md`. Règle de découpage : une version = un jeu de fonctionnalités livrables +
sa validation ; aucun module démarré avant que le précédent ne soit vérifié.
