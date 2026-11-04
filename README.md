# Macro Journal

Journal de recherche fondamentale **local-first** (les données restent sur votre machine). Le projet contient une interface Rust/WASM, une API locale Axum et une base SQLite.

## Périmètre actuel (v0.2)

Socle initial + itération « sécurisation du stockage » (emplacement XDG, sentinel anti-perte, snapshots vérifiés) — détails dans la section [Emplacement des données](#emplacement-des-données) ci-dessous.

- Interface « Dark Research ».
- Dossiers réellement cloisonnés pour USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD et CNY (facteur externe) ; chaque note est stockée et modifiable dans son dossier uniquement.
- Les notes antérieures au classement par devise restent accessibles dans « Notes à classer » ; aucune devise ne leur est attribuée automatiquement.
- Drapeaux repris des carnets fondamentaux de février et mars.
- Tableau de bord de recherche avec recherche dans les notes, historique récent, repères méthodologiques et liens vers RateProbability et la CFTC.
- Lecture des notes dans une mise en page éditoriale ; édition et aperçu Markdown assaini accessibles séparément.
- Aperçu Markdown assaini avant insertion dans le DOM (`ammonia`).
- Création, liste, ouverture et mise à jour de notes de recherche.
- Stockage SQLite **stable** : `~/.local/share/macro-journal/macro_journal.db` (convention XDG), indépendant du répertoire courant.
- Sentinel `.initialized` empêchant la recréation silencieuse d'une base vide après une perte.
- Snapshots SQLite vérifiés via `VACUUM INTO` (`POST /api/backups`).
- Placeholders de navigation pour les futurs modules.

Pas encore implémenté : agrégation et synthèse des dossiers de devises, profils macro des autres devises, outlook hebdomadaire, relevés d'événements, analyse de paires, versionnage des narratifs, pièces jointes, recherche plein texte, et interface d'export/sauvegarde.

## Stack technique

- Leptos 0.8.23 (CSR / WebAssembly)
- Axum 0.8.9
- SQLx 0.9.0 + SQLite
- Trunk pour le développement/build du frontend
- `pulldown-cmark` + `ammonia` pour un aperçu Markdown sûr

## Prérequis sur Linux Mint

Installer la toolchain Rust stable via rustup, puis la cible WASM et Trunk :

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Le backend utilise SQLite via la fonctionnalité *bundled* de SQLx. Une toolchain C peut être nécessaire pour les dépendances natives :

```bash
sudo apt update
sudo apt install build-essential pkg-config
```

## Développement

Préparer l'environnement COT une fois, puis lancer l'application :

```bash
python3 -m venv .venv
.venv/bin/python -m pip install -r cot/requirements.txt
./run-dev.sh
```

`run-dev.sh` actualise les rapports COT et publie les graphiques avant de lancer
l'API et le frontend. L'année CFTC en cours est retéléchargée à chaque lancement;
les années précédentes sont réutilisées depuis le cache. Si l'actualisation échoue,
un avertissement est affiché et le dashboard démarre avec les derniers graphiques publiés.

Les données économiques USD sont conservées dans [economic_data/usd/history.csv](./economic_data/usd/history.csv).
Elles ne sont pas téléchargées au démarrage : les nouvelles observations se saisissent
depuis la fiche de chaque indicateur. Voir [economic_data/README.md](./economic_data/README.md).

Le dashboard est disponible à l'adresse :

```text
http://127.0.0.1:8080
```

L'API est disponible sur `http://127.0.0.1:3000/api/health`. Utilisez `./run-dev.sh` pour lancer les deux d'un seul geste.

Utiliser systématiquement `127.0.0.1` pendant le développement : la politique CORS de l'API est volontairement restreinte aux origines Trunk locales ci-dessus.

## Lancement local « production »

Compiler d'abord le frontend :

```bash
./cot/update.sh
cd apps/web
trunk build --release
cd ../..
cargo run -p macro-journal-server --release
```

Puis ouvrir `http://127.0.0.1:3000`. Axum sert les fichiers compilés depuis `apps/web/dist`, après les routes de l'API.

## Emplacement des données

Base par défaut (indépendante du répertoire courant) :

```text
~/.local/share/macro-journal/macro_journal.db
```

`XDG_DATA_HOME` est respecté lorsqu'il est défini (chemins absolus uniquement, conformément à la spécification XDG). Surcharger explicitement la base avec :

```bash
MACRO_JOURNAL_DB=/chemin/absolu/vers/macro_journal.db cargo run -p macro-journal-server
```

À la première utilisation, le répertoire de données, la base et les migrations sont créés, puis un fichier sentinel `.initialized` est écrit à côté de la base — **uniquement après un succès complet**. À chaque démarrage :

- Base existante : ouverture normale (un `PRAGMA quick_check` est exécuté au préalable).
- Base absente alors que le sentinel existe : le serveur **refuse de démarrer** et explique comment restaurer depuis un snapshot ; aucune base vide n'est jamais recréée silencieusement.
- Ancienne base détectée à l'emplacement relatif historique (`<workspace>/data/macro_journal.db`) alors que la base XDG n'existe pas : le serveur refuse de démarrer et explique la procédure de migration (relancer avec `MACRO_JOURNAL_DB` pour continuer à utiliser l'ancien fichier, ou le copier vers l'emplacement XDG après sauvegarde). **Rien n'est jamais déplacé automatiquement.**

### Snapshots

```bash
curl -X POST http://127.0.0.1:3000/api/backups
```

crée `~/.local/share/macro-journal/backups/macro_journal-YYYYMMDD-HHMM.db` via `VACUUM INTO` (snapshot autonome et compacté — pas une copie de fichiers à chaud), puis le rouvre pour exécuter `PRAGMA integrity_check` et comparer les tables utilisateur et leurs compteurs de lignes avec la base en cours. Un snapshot invalide est signalé comme un échec.

Le répertoire de données est exclu de Git. L'application est liée à localhost et n'est pas configurée pour un accès LAN ou public.

## Arborescence du workspace

```text
macro-journal/
├── apps/
│   ├── server/       # API Axum, initialisation SQLite, migrations
│   └── web/          # UI Leptos, config Trunk, CSS
├── crates/
│   └── shared/       # structures de données partagées navigateur/serveur
├── docs/             # spécifications et décisions d'architecture
├── Cargo.toml
└── README.md
```

## Règles d'ingénierie

1. Les faits, les interprétations et les suggestions d'IA doivent rester distinguables.
2. Les notes de recherche doivent conserver leur historique lorsque le versionnage des narratifs sera ajouté.
3. Une URL source et une date d'observation doivent être attachées aux affirmations importantes.
4. Les services externes (RateProbability, CFTC, calendriers, actualités) sont référencés plutôt que réimplémentés, sauf raison claire.
5. Le schéma de base de données doit rester exportable et documenté.

## Validation

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo check -p macro-journal-web --target wasm32-unknown-unknown
cargo clippy --workspace --all-targets
cargo test --workspace
```

21 tests couvrent les scénarios de stockage (première initialisation, réouverture, redémarrage, base perdue + sentinel, migration en échec, surcharges d'environnement, indépendance du répertoire courant) et les snapshots (création, réouverture, détection de corruption).
