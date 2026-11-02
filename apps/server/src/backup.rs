//! Snapshots SQLite vérifiés via `VACUUM INTO`.
//!
//! Contrairement à une copie de fichier à chaud (incohérente en mode WAL si
//! les fichiers `-wal`/`-shm` sont ignorés), `VACUUM INTO` produit depuis une
//! lecture transactionnelle un fichier **autonome et compacté**. Chaque
//! snapshot est ensuite rouvert en lecture seule pour subir :
//!
//! 1. `PRAGMA integrity_check` (exigence minimale),
//! 2. une comparaison des tables utilisateur et de leurs compteurs de lignes
//!    avec la base source.
//!
//! Une sauvegarde invalide est signalée comme un échec : jamais silencieuse.

use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    AssertSqlSafe, Row, SqlitePool,
};

/// Erreurs de sauvegarde.
#[derive(Debug)]
pub enum BackupError {
    /// Échec d'entrée/sortie (création du répertoire des snapshots…).
    Io {
        context: &'static str,
        source: io::Error,
    },
    /// L'horodatage du nom de fichier n'a pas pu être obtenu.
    Timestamp { source: sqlx::Error },
    /// `VACUUM INTO` a échoué.
    Vacuum {
        target: PathBuf,
        source: sqlx::Error,
    },
    /// La sauvegarde n'a pas pu être rouverte ou interrogée pour vérification.
    Verify {
        target: PathBuf,
        source: sqlx::Error,
    },
    /// `PRAGMA integrity_check` négatif sur la sauvegarde.
    IntegrityCheckFailed {
        target: PathBuf,
        findings: Vec<String>,
    },
    /// Le contenu de la sauvegarde ne correspond pas à la base source.
    ContentMismatch { target: PathBuf, reason: String },
}

impl fmt::Display for BackupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { context, source } => write!(formatter, "{context}: {source}"),
            Self::Timestamp { source } => {
                write!(
                    formatter,
                    "cannot obtain a timestamp for the backup name: {source}"
                )
            }
            Self::Vacuum { target, source } => write!(
                formatter,
                "VACUUM INTO failed for {}: {source}",
                target.display()
            ),
            Self::Verify { target, source } => write!(
                formatter,
                "cannot verify the backup at {}: {source}",
                target.display()
            ),
            Self::IntegrityCheckFailed { target, findings } => write!(
                formatter,
                "integrity check failed for the backup at {}: {}",
                target.display(),
                findings.join("; ")
            ),
            Self::ContentMismatch { target, reason } => write!(
                formatter,
                "the backup at {} does not match the source database: {reason}",
                target.display()
            ),
        }
    }
}

impl std::error::Error for BackupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Timestamp { source } | Self::Vacuum { source, .. } => Some(source),
            Self::Verify { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Crée un snapshot vérifié dans `backups_dir` et retourne son chemin.
///
/// Le nom est `macro_journal-YYYYMMDD-HHMM.db` (UTC, via SQLite) ; en cas de
/// collision dans la même minute, un suffixe `-02`, `-03`… est ajouté car
/// `VACUUM INTO` exige une cible inexistante.
pub async fn create_backup(pool: &SqlitePool, backups_dir: &Path) -> Result<PathBuf, BackupError> {
    fs::create_dir_all(backups_dir).map_err(|source| BackupError::Io {
        context: "cannot create the backups directory",
        source,
    })?;
    let stamp: String = sqlx::query_scalar("SELECT strftime('%Y%m%d-%H%M', 'now')")
        .fetch_one(pool)
        .await
        .map_err(|source| BackupError::Timestamp { source })?;
    let target = backup_target(backups_dir, &stamp);
    let filename = target.to_string_lossy().into_owned();
    sqlx::query("VACUUM INTO ?")
        .bind(&filename)
        .execute(pool)
        .await
        .map_err(|source| BackupError::Vacuum {
            target: target.clone(),
            source,
        })?;
    verify_backup(pool, &target).await?;
    tracing::info!(backup = %target.display(), "Verified snapshot created");
    Ok(target)
}

fn backup_target(backups_dir: &Path, stamp: &str) -> PathBuf {
    let mut candidate = backups_dir.join(format!("macro_journal-{stamp}.db"));
    let mut suffix = 1u32;
    while candidate.exists() {
        suffix += 1;
        candidate = backups_dir.join(format!("macro_journal-{stamp}-{suffix:02}.db"));
    }
    candidate
}

/// Vérifie un fichier de sauvegarde : intégrité, puis mêmes tables et mêmes
/// compteurs de lignes que la base source.
pub async fn verify_backup(source: &SqlitePool, target: &Path) -> Result<(), BackupError> {
    let options = SqliteConnectOptions::new().filename(target).read_only(true);
    let backup = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .map_err(|source| BackupError::Verify {
            target: target.to_owned(),
            source,
        })?;
    let result = compare_with_source(source, &backup, target).await;
    backup.close().await;
    result
}

async fn compare_with_source(
    source: &SqlitePool,
    backup: &SqlitePool,
    target: &Path,
) -> Result<(), BackupError> {
    // 1. Intégrité du fichier de sauvegarde.
    let findings = integrity_check(backup)
        .await
        .map_err(|source| BackupError::Verify {
            target: target.to_owned(),
            source,
        })?;
    if findings.iter().any(|finding| finding != "ok") {
        return Err(BackupError::IntegrityCheckFailed {
            target: target.to_owned(),
            findings,
        });
    }

    // 2. Périmètre : mêmes tables utilisateur.
    let source_tables = user_tables(source)
        .await
        .map_err(|source| BackupError::Verify {
            target: target.to_owned(),
            source,
        })?;
    let backup_tables = user_tables(backup)
        .await
        .map_err(|source| BackupError::Verify {
            target: target.to_owned(),
            source,
        })?;
    if source_tables != backup_tables {
        return Err(BackupError::ContentMismatch {
            target: target.to_owned(),
            reason: format!(
                "user tables differ: source has {source_tables:?}, backup has {backup_tables:?}"
            ),
        });
    }

    // 3. Volumétrie : mêmes compteurs de lignes, table par table.
    for table in &source_tables {
        let source_count =
            row_count(source, table)
                .await
                .map_err(|source| BackupError::Verify {
                    target: target.to_owned(),
                    source,
                })?;
        let backup_count =
            row_count(backup, table)
                .await
                .map_err(|source| BackupError::Verify {
                    target: target.to_owned(),
                    source,
                })?;
        if source_count != backup_count {
            return Err(BackupError::ContentMismatch {
                target: target.to_owned(),
                reason: format!(
                    "table {table:?}: {source_count} rows in source but {backup_count} in backup"
                ),
            });
        }
    }
    Ok(())
}

async fn integrity_check(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>("PRAGMA integrity_check")
        .fetch_all(pool)
        .await
}

async fn user_tables(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        "SELECT name FROM sqlite_master \
         WHERE type = 'table' AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\' \
         ORDER BY name",
    )
    .fetch_all(pool)
    .await
}

async fn row_count(pool: &SqlitePool, table: &str) -> Result<i64, sqlx::Error> {
    // Nom de table issu de `sqlite_master`, cité et échappé : SQL dynamique
    // audité, d'où `AssertSqlSafe` (sqlx 0.9 réserve `query*` aux littéraux).
    let quoted = format!("\"{}\"", table.replace('"', "\"\""));
    let sql = format!("SELECT COUNT(*) FROM {quoted}");
    let rows = sqlx::raw_sql(AssertSqlSafe(sql)).fetch_all(pool).await?;
    rows.first()
        .map(|row| row.try_get::<i64, _>(0))
        .transpose()
        .map(|count| count.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::StorageLayout, storage};
    use tempfile::TempDir;

    async fn initialized_layout_and_pool(dir: &TempDir) -> (StorageLayout, SqlitePool) {
        let layout =
            StorageLayout::with_legacy_candidates(dir.path().join("macro_journal.db"), Vec::new());
        let pool = storage::open_or_initialize(&layout, &sqlx::migrate!("./migrations"))
            .await
            .unwrap();
        (layout, pool)
    }

    async fn add_entry(pool: &SqlitePool, title: &str) {
        sqlx::query("INSERT INTO entries (title, content) VALUES (?, 'contenu')")
            .bind(title)
            .execute(pool)
            .await
            .unwrap();
    }

    // Scénario 8 — création d'un snapshot SQLite valide.
    #[tokio::test]
    async fn snapshot_is_valid_and_verified() {
        let dir = TempDir::new().unwrap();
        let (layout, pool) = initialized_layout_and_pool(&dir).await;
        add_entry(&pool, "note a").await;
        add_entry(&pool, "note b").await;

        let backup_path = create_backup(&pool, &layout.backups_dir)
            .await
            .expect("la sauvegarde doit réussir et être vérifiée");
        assert!(backup_path.is_file());
        assert!(backup_path.starts_with(&layout.backups_dir));
        let name = backup_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        assert!(
            name.starts_with("macro_journal-") && name.ends_with(".db"),
            "nom de snapshot inattendu : {name}"
        );

        // Re-vérification explicite (create_backup l'a déjà faite).
        verify_backup(&pool, &backup_path).await.unwrap();
        pool.close().await;
    }

    // Scénario 9 — réouverture et vérification du snapshot.
    #[tokio::test]
    async fn snapshot_reopens_with_same_data() {
        let dir = TempDir::new().unwrap();
        let (layout, pool) = initialized_layout_and_pool(&dir).await;
        add_entry(&pool, "unique-42").await;
        let backup_path = create_backup(&pool, &layout.backups_dir).await.unwrap();
        pool.close().await;

        // Réouverture autonome du snapshot, sans la base source.
        let options = SqliteConnectOptions::new()
            .filename(&backup_path)
            .read_only(true);
        let snapshot = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .unwrap();
        let findings: Vec<String> = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_all(&snapshot)
            .await
            .unwrap();
        assert_eq!(findings, vec!["ok".to_string()]);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM entries")
            .fetch_one(&snapshot)
            .await
            .unwrap();
        assert_eq!(count, 1);
        let title: String = sqlx::query_scalar("SELECT title FROM entries")
            .fetch_one(&snapshot)
            .await
            .unwrap();
        assert_eq!(title, "unique-42");
        snapshot.close().await;
    }

    // Sauvegardes successives : fichiers distincts et valides (collision de
    // nom dans la même minute).
    #[tokio::test]
    async fn successive_backups_get_distinct_files() {
        let dir = TempDir::new().unwrap();
        let (layout, pool) = initialized_layout_and_pool(&dir).await;
        let first = create_backup(&pool, &layout.backups_dir).await.unwrap();
        let second = create_backup(&pool, &layout.backups_dir).await.unwrap();
        assert_ne!(first, second);
        assert!(first.is_file() && second.is_file());
        verify_backup(&pool, &first).await.unwrap();
        verify_backup(&pool, &second).await.unwrap();
        pool.close().await;
    }

    // Une sauvegarde invalide doit être signalée comme un échec.
    #[tokio::test]
    async fn tampered_backup_is_reported_invalid() {
        let dir = TempDir::new().unwrap();
        let (layout, pool) = initialized_layout_and_pool(&dir).await;
        add_entry(&pool, "donnée").await;
        let backup_path = create_backup(&pool, &layout.backups_dir).await.unwrap();
        pool.close().await;

        // Corruption volontaire du snapshot (troncature de la fin du fichier).
        let mut bytes = fs::read(&backup_path).unwrap();
        let length = bytes.len();
        bytes.truncate(length.saturating_sub(200));
        fs::write(&backup_path, &bytes).unwrap();

        let pool = storage::open_or_initialize(
            &StorageLayout::with_legacy_candidates(layout.db_path.clone(), Vec::new()),
            &sqlx::migrate!("./migrations"),
        )
        .await
        .unwrap();
        let error = verify_backup(&pool, &backup_path).await.unwrap_err();
        assert!(
            matches!(
                error,
                BackupError::Verify { .. } | BackupError::IntegrityCheckFailed { .. }
            ),
            "erreur inattendue : {error:?}"
        );
        pool.close().await;
    }
}
