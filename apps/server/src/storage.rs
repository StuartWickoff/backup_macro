//! Ouverture et initialisation sécurisées de la base SQLite.
//!
//! Garanties (itération v0.2 « stockage stable ») :
//!
//! - **Jamais de recréation silencieuse** : la sentinelle `.initialized`,
//!   créée à côté de la base uniquement après une initialisation complète
//!   (répertoire + base + migrations), matérialise « une installation a
//!   réussi ici ».
//! - Base absente + sentinelle présente → [`StorageError::MissingDatabase`],
//!   rien n'est créé.
//! - Ancienne base relative (`data/macro_journal.db`) détectée alors que la
//!   nouvelle base XDG est absente → [`StorageError::LegacyDatabaseFound`] :
//!   la procédure de migration est expliquée, **rien n'est déplacé
//!   automatiquement**.
//! - Migration en échec → arrêt propre, sentinelle **non** créée.
//! - Base existante → `PRAGMA quick_check` avant les migrations, puis
//!   sentinelle garantie (adoption des bases créées avant ce mécanisme ou
//!   copiées manuellement vers le nouvel emplacement).

use std::{
    fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};

use crate::config::StorageLayout;

/// Erreurs de stockage. Toutes sont explicites et actionnables : aucune
/// dégradation silencieuse n'est tolérée pour la persistance.
#[derive(Debug)]
pub enum StorageError {
    /// Ni `HOME`, ni `XDG_DATA_HOME`, ni `MACRO_JOURNAL_DB` : impossible de
    /// résoudre un emplacement stable.
    HomeNotFound,
    /// La base est absente mais la sentinelle existe : suppression ou
    /// déplacement accidentel probable.
    MissingDatabase {
        db_path: PathBuf,
        sentinel_path: PathBuf,
    },
    /// Une ancienne base (chemin relatif historique) a été trouvée alors que
    /// la cible XDG n'existe pas : la première installation est refusée
    /// jusqu'à migration explicite.
    LegacyDatabaseFound {
        legacy_path: PathBuf,
        db_path: PathBuf,
    },
    /// Le chemin de la base existe mais n'est pas un fichier régulier.
    NotAFile(PathBuf),
    /// Échec d'entrée/sortie (création de répertoire, etc.).
    Io {
        context: &'static str,
        source: io::Error,
    },
    /// Connexion ou requête SQLite impossible.
    SqliteRequest {
        db_path: PathBuf,
        source: sqlx::Error,
    },
    /// `PRAGMA quick_check` a signalé une base endommagée.
    IntegrityFailed {
        db_path: PathBuf,
        findings: Vec<String>,
    },
    /// Une migration a échoué (la sentinelle n'est pas créée).
    Migrate { source: sqlx::migrate::MigrateError },
    /// Impossible d'écrire la sentinelle après une initialisation réussie.
    SentinelWrite { path: PathBuf, source: io::Error },
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HomeNotFound => write!(
                formatter,
                "cannot determine a stable storage location: set HOME, XDG_DATA_HOME or MACRO_JOURNAL_DB"
            ),
            Self::MissingDatabase {
                db_path,
                sentinel_path,
            } => write!(
                formatter,
                "database file {} is missing but the sentinel {} exists: the database was \
                 probably deleted or moved by mistake. No new empty database was created. \
                 Restore the file (e.g. from a snapshot in the backups directory); only if \
                 the loss is intentional should you also remove the sentinel file",
                db_path.display(),
                sentinel_path.display()
            ),
            Self::LegacyDatabaseFound {
                legacy_path,
                db_path,
            } => write!(
                formatter,
                "a legacy database was found at {}, but the expected database {} does not \
                 exist yet: refusing to silently create a new empty database. Either restart \
                 with MACRO_JOURNAL_DB={} to keep using the legacy file, or (after stopping \
                 the server and backing it up) copy it to {} and restart. Nothing was moved \
                 automatically",
                legacy_path.display(),
                db_path.display(),
                legacy_path.display(),
                db_path.display()
            ),
            Self::NotAFile(path) => {
                write!(formatter, "{} exists but is not a regular file", path.display())
            }
            Self::Io { context, source } => write!(formatter, "{context}: {source}"),
            Self::SqliteRequest { db_path, source } => write!(
                formatter,
                "SQLite request failed on {}: {source}",
                db_path.display()
            ),
            Self::IntegrityFailed { db_path, findings } => write!(
                formatter,
                "integrity check failed for {}: {}",
                db_path.display(),
                findings.join("; ")
            ),
            Self::Migrate { source } => write!(
                formatter,
                "database migration failed: {source}; the installation is NOT marked as \
                 initialized and the partial database is kept so a corrected migration can \
                 resume"
            ),
            Self::SentinelWrite { path, source } => write!(
                formatter,
                "cannot write the initialization sentinel at {}: {source}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } | Self::SentinelWrite { source, .. } => Some(source),
            Self::SqliteRequest { source, .. } => Some(source),
            Self::Migrate { source } => Some(source),
            _ => None,
        }
    }
}

/// Ouvre la base existante ou procède à la première installation complète.
///
/// Point d'entrée unique du démarrage de la persistance ; `migrator` est
/// fourni par l'appelant (`sqlx::migrate!` en production, migrations
/// volontairement cassées dans les tests d'échec).
pub async fn open_or_initialize(
    layout: &StorageLayout,
    migrator: &Migrator,
) -> Result<SqlitePool, StorageError> {
    match Installation::classify(layout)? {
        Installation::Existing => open_existing(layout, migrator).await,
        Installation::Fresh => initialize(layout, migrator).await,
    }
}

enum Installation {
    Existing,
    Fresh,
}

impl Installation {
    /// Classe la situation avant toute écriture sur disque.
    fn classify(layout: &StorageLayout) -> Result<Self, StorageError> {
        if layout.db_path.exists() && !layout.db_path.is_file() {
            return Err(StorageError::NotAFile(layout.db_path.clone()));
        }
        if layout.db_path.is_file() {
            return Ok(Self::Existing);
        }
        if layout.sentinel_path.exists() {
            return Err(StorageError::MissingDatabase {
                db_path: layout.db_path.clone(),
                sentinel_path: layout.sentinel_path.clone(),
            });
        }
        if let Some(legacy_path) = layout
            .legacy_candidates
            .iter()
            .find(|candidate| candidate.is_file() && *candidate != &layout.db_path)
        {
            return Err(StorageError::LegacyDatabaseFound {
                legacy_path: legacy_path.clone(),
                db_path: layout.db_path.clone(),
            });
        }
        Ok(Self::Fresh)
    }
}

/// Base existante : ouverture normale, intégrité, migrations en attente,
/// puis sentinelle garantie.
async fn open_existing(
    layout: &StorageLayout,
    migrator: &Migrator,
) -> Result<SqlitePool, StorageError> {
    tracing::info!(db = %layout.db_path.display(), "Opening existing database");
    let pool = connect(&layout.db_path, false).await?;
    if let Err(error) = run_quick_check(&pool, &layout.db_path).await {
        pool.close().await;
        return Err(error);
    }
    if let Err(error) = run_migrations(&pool, migrator).await {
        pool.close().await;
        return Err(error);
    }
    // Base saine et à jour : garantir la sentinelle (bases créées avant ce
    // mécanisme, ou copiées manuellement vers le nouvel emplacement).
    if let Err(error) = write_sentinel(layout, &pool).await {
        pool.close().await;
        return Err(error);
    }
    Ok(pool)
}

/// Première installation : répertoire, base, migrations, puis sentinelle —
/// dans cet ordre, la sentinelle n'étant écrite qu'après un succès complet.
async fn initialize(
    layout: &StorageLayout,
    migrator: &Migrator,
) -> Result<SqlitePool, StorageError> {
    tracing::info!(db = %layout.db_path.display(), "First use: creating a new database");
    let parent = crate::config::parent_or_current(&layout.db_path);
    fs::create_dir_all(&parent).map_err(|source| StorageError::Io {
        context: "cannot create the data directory",
        source,
    })?;
    let pool = connect(&layout.db_path, true).await?;
    if let Err(error) = run_migrations(&pool, migrator).await {
        // Arrêt propre : la base partielle est conservée (les migrations sqlx
        // sont transactionnelles, une exécution corrigée pourra reprendre),
        // mais la sentinelle n'est PAS créée.
        pool.close().await;
        return Err(error);
    }
    write_sentinel(layout, &pool).await?;
    tracing::info!(
        sentinel = %layout.sentinel_path.display(),
        "Installation complete"
    );
    Ok(pool)
}

fn connect_options(db_path: &Path, create_if_missing: bool) -> SqliteConnectOptions {
    SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(create_if_missing)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
}

async fn connect(db_path: &Path, create_if_missing: bool) -> Result<SqlitePool, StorageError> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options(db_path, create_if_missing))
        .await
        .map_err(|source| StorageError::SqliteRequest {
            db_path: db_path.to_owned(),
            source,
        })
}

/// Contrôle d'intégrité rapide au démarrage : une base endommagée doit être
/// signalée, jamais « réparée » par une recréation vide.
async fn run_quick_check(pool: &SqlitePool, db_path: &Path) -> Result<(), StorageError> {
    let findings = sqlx::query_scalar::<_, String>("PRAGMA quick_check")
        .fetch_all(pool)
        .await
        .map_err(|source| StorageError::SqliteRequest {
            db_path: db_path.to_owned(),
            source,
        })?;
    if findings.iter().any(|finding| finding != "ok") {
        return Err(StorageError::IntegrityFailed {
            db_path: db_path.to_owned(),
            findings,
        });
    }
    Ok(())
}

async fn run_migrations(pool: &SqlitePool, migrator: &Migrator) -> Result<(), StorageError> {
    migrator
        .run(pool)
        .await
        .map_err(|source| StorageError::Migrate { source })
}

/// Crée la sentinelle si elle n'existe pas encore (idempotent : une base
/// saine existante n'écrase jamais une sentinelle déjà présente).
async fn write_sentinel(layout: &StorageLayout, pool: &SqlitePool) -> Result<(), StorageError> {
    if layout.sentinel_path.exists() {
        return Ok(());
    }
    // Horodatage fourni par SQLite (UTC) : aucune dépendance de date.
    let created_at: String = sqlx::query_scalar("SELECT strftime('%Y-%m-%dT%H:%M:%SZ', 'now')")
        .fetch_one(pool)
        .await
        .map_err(|source| StorageError::SqliteRequest {
            db_path: layout.db_path.clone(),
            source,
        })?;
    let db_name = layout
        .db_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let content = format!(
        "macro-journal initialized\ndb: {db_name}\ncreated_at: {created_at}\n\
         Do not delete the database file without deleting this marker as well.\n"
    );
    let path = &layout.sentinel_path;
    let mut file = fs::File::create(path).map_err(|source| StorageError::SentinelWrite {
        path: path.clone(),
        source,
    })?;
    file.write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|source| StorageError::SentinelWrite {
            path: path.clone(),
            source,
        })?;
    // Best effort : synchroniser aussi l'entrée de répertoire (Linux).
    if let Some(parent) = path.parent() {
        if let Ok(directory) = fs::File::open(parent) {
            let _ = directory.sync_all();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StorageLayout;
    use sqlx::migrate::Migrator;
    use tempfile::TempDir;

    /// Layout déterministe en répertoire temporaire : aucune détection
    /// d'anciennes bases (les tests ne dépendent pas de la machine).
    fn layout_in(dir: &TempDir) -> StorageLayout {
        StorageLayout::with_legacy_candidates(dir.path().join("macro_journal.db"), Vec::new())
    }

    fn real_migrator() -> Migrator {
        sqlx::migrate!("./migrations")
    }

    async fn insert_entry(pool: &SqlitePool, title: &str) {
        sqlx::query("INSERT INTO entries (title, content) VALUES (?, 'corps de note')")
            .bind(title)
            .execute(pool)
            .await
            .unwrap();
    }

    async fn count_entries(pool: &SqlitePool) -> i64 {
        sqlx::query_scalar("SELECT COUNT(*) FROM entries")
            .fetch_one(pool)
            .await
            .unwrap()
    }

    // Scénario 1 — première initialisation.
    #[tokio::test]
    async fn first_initialization_creates_database_and_sentinel() {
        let dir = TempDir::new().unwrap();
        let layout = layout_in(&dir);
        let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();

        assert!(layout.db_path.is_file(), "la base doit exister");
        assert!(
            layout.sentinel_path.is_file(),
            "la sentinelle doit être créée après le succès complet"
        );
        insert_entry(&pool, "première note").await;
        assert_eq!(count_entries(&pool).await, 1);
        pool.close().await;
    }

    // Scénarios 2 et 3 — réouverture d'une base existante, puis
    // « redémarrage » de l'application avec conservation des données.
    #[tokio::test]
    async fn existing_database_reopens_and_keeps_data_across_restarts() {
        let dir = TempDir::new().unwrap();
        let layout = layout_in(&dir);

        // Premier lancement.
        {
            let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();
            insert_entry(&pool, "note conservée").await;
            pool.close().await;
        }
        assert!(layout.db_path.is_file());

        // Redémarrage : mêmes données, sentinelle toujours là.
        let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();
        assert_eq!(count_entries(&pool).await, 1);
        let title: String = sqlx::query_scalar("SELECT title FROM entries")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(title, "note conservée");
        assert!(layout.sentinel_path.is_file());
        pool.close().await;
    }

    // Scénario 4 — base supprimée alors que la sentinelle existe.
    #[tokio::test]
    async fn missing_database_with_sentinel_errors_and_never_recreates() {
        let dir = TempDir::new().unwrap();
        let layout = layout_in(&dir);
        let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();
        pool.close().await;

        // « Perte » de la base : seul le fichier de base disparaît.
        fs::remove_file(&layout.db_path).unwrap();
        assert!(layout.sentinel_path.exists());

        let error = open_or_initialize(&layout, &real_migrator())
            .await
            .unwrap_err();
        assert!(
            matches!(error, StorageError::MissingDatabase { .. }),
            "erreur inattendue : {error:?}"
        );
        assert!(
            !layout.db_path.exists(),
            "aucune base vide ne doit être recréée silencieusement"
        );
    }

    // Scénario 5 — migration en échec : pas de sentinelle.
    #[tokio::test]
    async fn failed_migration_leaves_no_sentinel() {
        let dir = TempDir::new().unwrap();
        let migrations = dir.path().join("broken_migrations");
        fs::create_dir(&migrations).unwrap();
        fs::write(
            migrations.join("0001_fine.sql"),
            "CREATE TABLE fine (id INTEGER PRIMARY KEY);",
        )
        .unwrap();
        fs::write(migrations.join("0002_broken.sql"), "CREATE TABLE oops (;").unwrap();
        let migrator = Migrator::new(migrations).await.unwrap();

        let data = TempDir::new().unwrap();
        let layout = layout_in(&data);
        let error = open_or_initialize(&layout, &migrator).await.unwrap_err();
        assert!(
            matches!(error, StorageError::Migrate { .. }),
            "erreur inattendue : {error:?}"
        );
        assert!(
            !layout.sentinel_path.exists(),
            "l'échec de migration ne doit pas marquer l'installation comme initialisée"
        );
    }

    // Exigence §3 — ancienne base relative détectée, nouvelle base absente.
    #[tokio::test]
    async fn legacy_database_blocks_silent_fresh_install() {
        let dir = TempDir::new().unwrap();
        let legacy = dir.path().join("old-project/data/macro_journal.db");
        fs::create_dir_all(legacy.parent().unwrap()).unwrap();
        fs::write(&legacy, b"ancienne base (contenu factice)").unwrap();

        let layout = StorageLayout::with_legacy_candidates(
            dir.path().join("xdg/macro-journal/macro_journal.db"),
            vec![legacy.clone()],
        );
        let error = open_or_initialize(&layout, &real_migrator())
            .await
            .unwrap_err();
        match &error {
            StorageError::LegacyDatabaseFound { legacy_path, .. } => {
                assert_eq!(legacy_path, &legacy);
            }
            other => panic!("erreur inattendue : {other:?}"),
        }
        assert!(
            !layout.db_path.exists(),
            "aucune base vide ne doit être créée à côté d'une ancienne base détectée"
        );
    }

    // Base endommagée : signalée, jamais reconstruite silencieusement.
    #[tokio::test]
    async fn corrupt_database_is_reported_not_silently_rebuilt() {
        let dir = TempDir::new().unwrap();
        let layout = layout_in(&dir);
        fs::write(&layout.db_path, b"ceci n'est pas une base sqlite").unwrap();

        let error = open_or_initialize(&layout, &real_migrator())
            .await
            .unwrap_err();
        assert!(
            matches!(
                error,
                StorageError::SqliteRequest { .. } | StorageError::IntegrityFailed { .. }
            ),
            "erreur inattendue : {error:?}"
        );
        assert!(!layout.sentinel_path.exists());
    }

    // ---- Tests sérialisés sur l'environnement réel du processus ----
    // Ils touchent à des variables globales : un mutex asynchrone commun les
    // rend exclusifs (un Mutex tokio peut être tenu à travers un await,
    // contrairement à un Mutex std), et un garde restaure l'état initial
    // quoi qu'il arrive.

    static ENV_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    struct EnvGuard {
        saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
        cwd: PathBuf,
    }

    impl EnvGuard {
        fn new(keys: &[&'static str]) -> Self {
            Self {
                saved: keys
                    .iter()
                    .map(|key| (*key, std::env::var_os(key)))
                    .collect(),
                cwd: std::env::current_dir().unwrap(),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, value) in self.saved.drain(..) {
                match value {
                    Some(value) => std::env::set_var(key, value),
                    None => std::env::remove_var(key),
                }
            }
            let _ = std::env::set_current_dir(&self.cwd);
        }
    }

    async fn lock_env() -> tokio::sync::MutexGuard<'static, ()> {
        ENV_MUTEX.lock().await
    }

    // Scénario 6 — MACRO_JOURNAL_DB fonctionnel (environnement réel).
    #[tokio::test]
    async fn macro_journal_db_env_var_selects_the_database() {
        let _guard = lock_env().await;
        let dir = TempDir::new().unwrap();
        let db_path = dir.path().join("perso.db");
        let _env = EnvGuard::new(&["MACRO_JOURNAL_DB", "XDG_DATA_HOME", "HOME"]);
        std::env::set_var("MACRO_JOURNAL_DB", &db_path);
        std::env::remove_var("XDG_DATA_HOME");
        std::env::remove_var("HOME");

        let resolved = StorageLayout::resolve().unwrap();
        assert_eq!(resolved.db_path, db_path);

        // Ouverture réelle avec la base explicite (déterministe : pas de
        // candidats legacy, la machine d'exécution ne doit pas influer).
        let layout = StorageLayout::with_legacy_candidates(db_path.clone(), Vec::new());
        let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();
        assert!(db_path.is_file());
        assert!(
            layout.sentinel_path.is_file(),
            "la sentinelle doit être créée à côté de la base explicite"
        );
        insert_entry(&pool, "via MACRO_JOURNAL_DB").await;
        pool.close().await;
    }

    // Scénario 7 — répertoire courant différent : même base utilisée.
    #[tokio::test]
    async fn different_cwd_uses_the_same_database() {
        let _guard = lock_env().await;
        let home = TempDir::new().unwrap();
        let first_cwd = TempDir::new().unwrap();
        let second_cwd = TempDir::new().unwrap();
        let _env = EnvGuard::new(&["MACRO_JOURNAL_DB", "XDG_DATA_HOME", "HOME"]);
        std::env::remove_var("MACRO_JOURNAL_DB");
        std::env::remove_var("XDG_DATA_HOME");
        std::env::set_var("HOME", home.path());

        let expected = home
            .path()
            .join(".local/share/macro-journal/macro_journal.db");

        // Premier lancement, depuis un répertoire quelconque.
        std::env::set_current_dir(first_cwd.path()).unwrap();
        let layout = StorageLayout::resolve().unwrap();
        assert_eq!(layout.db_path, expected);
        {
            let layout = StorageLayout::with_legacy_candidates(expected.clone(), Vec::new());
            let pool = open_or_initialize(&layout, &real_migrator()).await.unwrap();
            insert_entry(&pool, "insensible au cwd").await;
            pool.close().await;
        }

        // « Redémarrage » depuis un autre répertoire : même base, données
        // conservées.
        std::env::set_current_dir(second_cwd.path()).unwrap();
        let layout_bis = StorageLayout::resolve().unwrap();
        assert_eq!(
            layout_bis.db_path, expected,
            "le chemin résolu ne doit pas dépendre du répertoire courant"
        );
        let layout_bis = StorageLayout::with_legacy_candidates(expected, Vec::new());
        let pool = open_or_initialize(&layout_bis, &real_migrator())
            .await
            .unwrap();
        assert_eq!(count_entries(&pool).await, 1);
        pool.close().await;
    }
}
