# Macro Journal

Local-first fundamental research journal. The initial scaffold contains a Rust/WASM UI, a local Axum API, and an SQLite database.

## Current scope (v0.1.0)

This is an initial scaffold; the storage hardening iteration (XDG location, sentinel, snapshots) is described in the "Data location" section below.

- Dark Research interface.
- Two-panel Markdown editor with live preview.
- Markdown preview is sanitized before being inserted into the DOM.
- Create, list, open, and update research notes.
- Stable SQLite storage at `~/.local/share/macro-journal/macro_journal.db` (XDG), with an `.initialized` sentinel that prevents silent recreation of an empty database, and verified `VACUUM INTO` snapshots (`POST /api/backups`).
- External shortcuts to RateProbability and the CFTC.
- Navigation placeholders for future modules.

Not implemented yet: currency profiles, weekly outlook, event records, pair analysis, narrative versioning, attachments, full-text search, and export/backup UI.

## Stack

- Leptos 0.8.23 (CSR / WebAssembly)
- Axum 0.8.9
- SQLx 0.9.0 + SQLite
- Trunk for frontend development/build
- `pulldown-cmark` + `ammonia` for safe Markdown preview

## Prerequisites on Linux Mint

Install the Rust stable toolchain with rustup, then install the WASM target and Trunk:

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

The backend uses SQLite through SQLx's bundled SQLite feature. A C toolchain may be needed for native dependencies:

```bash
sudo apt update
sudo apt install build-essential pkg-config
```

## Development

Open two terminals from the project root.

Terminal 1 — API and SQLite:

```bash
cargo run -p macro-journal-server
```

Terminal 2 — Leptos frontend:

```bash
cd apps/web
trunk serve --address 127.0.0.1 --port 8080
```

Open:

```text
http://127.0.0.1:8080
```

The API is available at `http://127.0.0.1:3000/api/health`.

Use `127.0.0.1` consistently during development. The API CORS policy is intentionally restricted to the local Trunk origins above.

## Production-like local run

Build the frontend first:

```bash
cd apps/web
trunk build --release
cd ../..
cargo run -p macro-journal-server --release
```

Then open `http://127.0.0.1:3000`. Axum serves the compiled files from `apps/web/dist` after the API routes.

## Data location

Default database (independent of the current working directory):

```text
~/.local/share/macro-journal/macro_journal.db
```

`XDG_DATA_HOME` is respected when set (absolute paths only, per the XDG specification). Override the database explicitly with:

```bash
MACRO_JOURNAL_DB=/absolute/path/to/macro_journal.db cargo run -p macro-journal-server
```

On first use the data directory, database, and migrations are created, then an `.initialized` sentinel file is written next to the database — only after full success. On every start:

- Existing database: opened normally (`PRAGMA quick_check` runs first).
- Database missing while the sentinel exists: the server refuses to start and explains how to restore from a snapshot; it never recreates an empty database silently.
- Legacy database detected at the old relative location (`<workspace>/data/macro_journal.db`) while the XDG database does not exist: the server refuses to start and explains the migration (use `MACRO_JOURNAL_DB` to keep the old file, or copy it to the XDG path after backing it up). Nothing is moved automatically.

### Snapshots

```bash
curl -X POST http://127.0.0.1:3000/api/backups
```

creates `~/.local/share/macro-journal/backups/macro_journal-YYYYMMDD-HHMM.db` via `VACUUM INTO` (autonomous, compacted snapshot), then reopens it to run `PRAGMA integrity_check` and compares user tables and row counts with the live database. An invalid snapshot is reported as a failure.

The data directory is excluded from Git. The app is bound to localhost and is not configured for LAN or public access.

## Workspace layout

```text
macro-journal/
├── apps/
│   ├── server/       # Axum API, SQLite initialization, migrations
│   └── web/          # Leptos UI, Trunk config, CSS
├── crates/
│   └── shared/       # API data structures shared by browser/server
├── docs/
├── Cargo.toml
└── README.md
```

## Engineering rules

1. Facts, interpretations, and AI suggestions must remain distinguishable.
2. Research notes must retain their history when narrative versioning is added.
3. A source URL and observation date should be attached to important claims.
4. External services (RateProbability, CFTC, calendars, news) are referenced rather than reimplemented without a clear reason.
5. The database schema must remain exportable and documented.
