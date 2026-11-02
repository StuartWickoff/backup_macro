//! Résolution des chemins de stockage : `MACRO_JOURNAL_DB` > `XDG_DATA_HOME` > `$HOME/.local/share`.
//!
//! Le chemin par défaut est toujours **absolu** : il ne dépend jamais du
//! répertoire courant du processus, contrairement à l'ancien défaut
//! `data/macro_journal.db` (cause directe de la perte de données du projet :
//! la base vivait dans une arborescence jetable).

use std::{env, path::PathBuf};

use crate::storage::StorageError;

/// Sous-répertoire XDG consacré à l'application.
pub const APP_DIR_NAME: &str = "macro-journal";
/// Nom du fichier de base de données.
pub const DB_FILE_NAME: &str = "macro_journal.db";
/// Nom du fichier sentinelle anti-recréation (voir [`crate::storage`]).
pub const SENTINEL_FILE_NAME: &str = ".initialized";
/// Sous-répertoire des snapshots, à côté de la base.
pub const BACKUPS_DIR_NAME: &str = "backups";

/// Racine du workspace, calculée à la compilation.
///
/// Permet de localiser `apps/web/dist` et l'ancien emplacement `data/` quel
/// que soit le répertoire courant au lancement du binaire.
pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|parent| parent.parent())
        .map(PathBuf::from)
        .unwrap_or(manifest_dir)
}

/// Parent utilisable d'un chemin : jamais `""`, `.` à défaut.
pub(crate) fn parent_or_current(path: &std::path::Path) -> PathBuf {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Photographie de l'environnement utile à la résolution des chemins.
///
/// Les tests unitaires construisent des valeurs synthétiques (aucune mutation
/// de l'environnement du processus, donc aucun risque de course entre tests
/// parallèles) ; seuls les tests sérialisés d'intégration lisent le vrai
/// environnement.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageEnv {
    /// Surcharge explicite du chemin de base (`MACRO_JOURNAL_DB`).
    pub macro_journal_db: Option<PathBuf>,
    /// Racine des données XDG (`XDG_DATA_HOME`).
    pub xdg_data_home: Option<PathBuf>,
    /// Répertoire personnel (`HOME`).
    pub home: Option<PathBuf>,
}

impl StorageEnv {
    /// Lit l'environnement réel du processus.
    pub fn from_process() -> Self {
        Self {
            macro_journal_db: non_empty_var("MACRO_JOURNAL_DB"),
            xdg_data_home: non_empty_var("XDG_DATA_HOME"),
            home: non_empty_var("HOME"),
        }
    }
}

fn non_empty_var(key: &str) -> Option<PathBuf> {
    env::var_os(key)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

/// Ensemble des chemins dérivés d'un emplacement de base.
#[derive(Debug, Clone)]
pub struct StorageLayout {
    /// Fichier de base SQLite.
    pub db_path: PathBuf,
    /// Sentinelle `.initialized`, à côté de la base.
    pub sentinel_path: PathBuf,
    /// Répertoire des snapshots `VACUUM INTO`.
    pub backups_dir: PathBuf,
    /// Emplacements où l'ancienne version (défaut relatif `data/`) pouvait
    /// avoir laissé une base. Vérifiés avant toute première installation
    /// XDG pour ne jamais créer une base vide par-dessus un historique.
    pub legacy_candidates: Vec<PathBuf>,
}

impl StorageLayout {
    /// Résout la layout depuis l'environnement réel (chemin de production).
    pub fn resolve() -> Result<Self, StorageError> {
        Self::resolve_with(&StorageEnv::from_process())
    }

    /// Résout la layout depuis un environnement explicite (tests).
    pub fn resolve_with(env: &StorageEnv) -> Result<Self, StorageError> {
        let db_path = resolve_db_path(env)?;
        Ok(Self::from_db_path(db_path))
    }

    /// Construit la layout pour un chemin de base explicite, avec détection
    /// des anciennes bases aux emplacements historiques.
    pub fn from_db_path(db_path: PathBuf) -> Self {
        let legacy_candidates = default_legacy_candidates();
        Self::with_legacy_candidates(db_path, legacy_candidates)
    }

    /// Construit la layout en contrôlant explicitement les candidats
    /// « anciennes bases » (les tests passent une liste vide pour être
    /// déterministes, indépendamment de la machine qui les exécute).
    pub fn with_legacy_candidates(db_path: PathBuf, legacy_candidates: Vec<PathBuf>) -> Self {
        let parent = parent_or_current(&db_path);
        Self {
            sentinel_path: parent.join(SENTINEL_FILE_NAME),
            backups_dir: parent.join(BACKUPS_DIR_NAME),
            db_path,
            legacy_candidates,
        }
    }
}

/// `MACRO_JOURNAL_DB` > `XDG_DATA_HOME` (absolu uniquement) > `$HOME/.local/share`.
fn resolve_db_path(env: &StorageEnv) -> Result<PathBuf, StorageError> {
    if let Some(path) = &env.macro_journal_db {
        if path.is_relative() {
            // Accepté pour ne pas casser un réglage existant, mais découragé :
            // un chemin relatif retombe dans le piège cwd-dépendant.
            tracing::warn!(
                path = %path.display(),
                "MACRO_JOURNAL_DB est relatif : un chemin absolu est fortement recommandé"
            );
        }
        return Ok(path.clone());
    }
    let data_home = match &env.xdg_data_home {
        // La spécification XDG ignore les valeurs relatives.
        Some(path) if path.is_absolute() => path.clone(),
        _ => home_data_home(env)?,
    };
    Ok(data_home.join(APP_DIR_NAME).join(DB_FILE_NAME))
}

fn home_data_home(env: &StorageEnv) -> Result<PathBuf, StorageError> {
    match &env.home {
        Some(home) => Ok(home.join(".local").join("share")),
        None => Err(StorageError::HomeNotFound),
    }
}

/// Emplacements historiques de l'ancienne base `data/macro_journal.db` :
/// racine du workspace (cas `run-dev.sh` / `cargo run` depuis la racine),
/// puis répertoire courant (lancement depuis un autre dossier).
fn default_legacy_candidates() -> Vec<PathBuf> {
    let mut candidates = vec![workspace_root().join("data").join(DB_FILE_NAME)];
    if let Ok(cwd) = env::current_dir() {
        candidates.push(cwd.join("data").join(DB_FILE_NAME));
    }
    candidates.sort();
    candidates.dedup();
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(db: Option<&str>, xdg: Option<&str>, home: Option<&str>) -> StorageEnv {
        StorageEnv {
            macro_journal_db: db.map(PathBuf::from),
            xdg_data_home: xdg.map(PathBuf::from),
            home: home.map(PathBuf::from),
        }
    }

    #[test]
    fn macro_journal_db_overrides_everything() {
        let layout = StorageLayout::resolve_with(&env(
            Some("/custom/path/db.sqlite"),
            Some("/xdg"),
            Some("/home/u"),
        ))
        .unwrap();
        assert_eq!(layout.db_path, PathBuf::from("/custom/path/db.sqlite"));
    }

    #[test]
    fn xdg_data_home_is_used_when_absolute() {
        let layout =
            StorageLayout::resolve_with(&env(None, Some("/xdg-data"), Some("/home/u"))).unwrap();
        assert_eq!(
            layout.db_path,
            PathBuf::from("/xdg-data/macro-journal/macro_journal.db")
        );
        assert_eq!(
            layout.sentinel_path,
            PathBuf::from("/xdg-data/macro-journal/.initialized")
        );
        assert_eq!(
            layout.backups_dir,
            PathBuf::from("/xdg-data/macro-journal/backups")
        );
    }

    #[test]
    fn relative_xdg_data_home_is_ignored() {
        // La spécification XDG : une valeur relative vaut « non définie ».
        let layout =
            StorageLayout::resolve_with(&env(None, Some("relative/xdg"), Some("/home/u"))).unwrap();
        assert_eq!(
            layout.db_path,
            PathBuf::from("/home/u/.local/share/macro-journal/macro_journal.db")
        );
    }

    #[test]
    fn default_is_home_local_share_and_is_absolute() {
        let layout = StorageLayout::resolve_with(&env(None, None, Some("/home/u"))).unwrap();
        assert!(layout.db_path.is_absolute());
        assert_eq!(
            layout.db_path,
            PathBuf::from("/home/u/.local/share/macro-journal/macro_journal.db")
        );
    }

    #[test]
    fn missing_home_is_an_explicit_error() {
        let error = StorageLayout::resolve_with(&env(None, None, None)).unwrap_err();
        assert!(matches!(error, StorageError::HomeNotFound));
    }

    #[test]
    fn workspace_root_contains_the_workspace_manifest() {
        let root = workspace_root();
        assert!(root.join("Cargo.toml").is_file());
        assert!(root.join("apps/server/Cargo.toml").is_file());
    }
}
