//! Binaire du serveur Macro Journal : bootstrap uniquement.
//!
//! La logique de stockage vit dans la bibliothèque `macro_journal_server`
//! (modules `config`, `storage`, `backup`), testable unitairement ; ce
//! fichier se limite à l'assemblage : logging, persistance, routes, écoute.

use std::{env, path::PathBuf};

use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use macro_journal_server::{
    backup::create_backup,
    config::{workspace_root, StorageLayout},
    storage::{open_or_initialize, StorageError},
};
use macro_journal_shared::{Entry, EntryDraft, HealthResponse};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{error, info};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    /// Répertoire des snapshots, résolu au démarrage à côté de la base :
    /// jamais dérivé du répertoire courant au moment d'une requête.
    backups_dir: PathBuf,
}

#[derive(Debug, FromRow)]
struct DbEntry {
    id: i64,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

impl From<DbEntry> for Entry {
    fn from(row: DbEntry) -> Self {
        Self {
            id: row.id,
            title: row.title,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

#[derive(Debug, Serialize)]
struct BackupResponse {
    path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "macro_journal_server=info,tower_http=info".into()),
        )
        .with_target(false)
        .init();

    // Résolution des chemins : MACRO_JOURNAL_DB > XDG_DATA_HOME > $HOME/.local/share.
    // Toujours absolu par défaut : indépendant du répertoire courant.
    let layout = StorageLayout::resolve()?;

    // Ouverture ou première installation sécurisée (sentinelle `.initialized`,
    // contrôle d'intégrité, détection d'une éventuelle ancienne base relative).
    let pool = match open_or_initialize(&layout, &sqlx::migrate!("./migrations")).await {
        Ok(pool) => pool,
        Err(storage_error) => {
            report_startup_storage_failure(&storage_error);
            return Err(storage_error.into());
        }
    };
    info!("SQLite database: {}", layout.db_path.display());

    let state = AppState {
        pool,
        backups_dir: layout.backups_dir.clone(),
    };
    // Clone bon marché (pool Arc) : le pool reste accessible pour la fermeture
    // ordonnée après l'arrêt du serveur.
    let shutdown_pool = state.pool.clone();
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://127.0.0.1:8080"),
            HeaderValue::from_static("http://localhost:8080"),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE]);

    // En développement, Trunk sert l'UI sur :8080. Après `trunk build --release`,
    // le même serveur peut servir le frontend compilé depuis apps/web/dist,
    // résolu depuis la racine du workspace (indépendant du répertoire courant).
    let dist_dir = workspace_root().join("apps/web/dist");
    let static_files =
        ServeDir::new(&dist_dir).not_found_service(ServeFile::new(dist_dir.join("index.html")));

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/entries", get(list_entries).post(create_entry))
        .route(
            "/api/entries/{id}",
            get(get_entry).put(update_entry).delete(delete_entry),
        )
        .route("/api/backups", post(create_backup_handler))
        .fallback_service(static_files)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;
    info!("Macro Journal API listening on http://{address}");
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());
    server.await?;

    // Fermeture ordonnée du pool : checkpoint WAL avant l'arrêt du processus.
    shutdown_pool.close().await;
    Ok(())
}

/// Journalise lisiblement les échecs de stockage au démarrage : ce sont des
/// situations opérationnelles (base perdue, ancienne base détectée…) qui
/// méritent un message explicite, pas une trace brute.
fn report_startup_storage_failure(error: &StorageError) {
    error!("Macro Journal cannot start: {error}");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn list_entries(State(state): State<AppState>) -> Result<Json<Vec<Entry>>, ApiFailure> {
    let rows = sqlx::query_as::<_, DbEntry>(
        "SELECT id, title, content, created_at, updated_at FROM entries ORDER BY updated_at DESC, id DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(ApiFailure::database)?;

    Ok(Json(rows.into_iter().map(Entry::from).collect()))
}

async fn get_entry(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Entry>, ApiFailure> {
    let row = sqlx::query_as::<_, DbEntry>(
        "SELECT id, title, content, created_at, updated_at FROM entries WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiFailure::database)?
    .ok_or(ApiFailure::not_found())?;

    Ok(Json(Entry::from(row)))
}

async fn create_entry(
    State(state): State<AppState>,
    Json(draft): Json<EntryDraft>,
) -> Result<impl IntoResponse, ApiFailure> {
    let title = draft.title.trim();
    if title.is_empty() {
        return Err(ApiFailure::bad_request("The title cannot be empty."));
    }

    let row = sqlx::query_as::<_, DbEntry>(
        "INSERT INTO entries (title, content) VALUES (?, ?) RETURNING id, title, content, created_at, updated_at",
    )
    .bind(title)
    .bind(draft.content)
    .fetch_one(&state.pool)
    .await
    .map_err(ApiFailure::database)?;

    Ok((StatusCode::CREATED, Json(Entry::from(row))))
}

async fn update_entry(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(draft): Json<EntryDraft>,
) -> Result<Json<Entry>, ApiFailure> {
    let title = draft.title.trim();
    if title.is_empty() {
        return Err(ApiFailure::bad_request("The title cannot be empty."));
    }

    let row = sqlx::query_as::<_, DbEntry>(
        "UPDATE entries SET title = ?, content = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') WHERE id = ? RETURNING id, title, content, created_at, updated_at",
    )
    .bind(title)
    .bind(draft.content)
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(ApiFailure::database)?
    .ok_or(ApiFailure::not_found())?;

    Ok(Json(Entry::from(row)))
}

async fn delete_entry(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiFailure> {
    let result = sqlx::query("DELETE FROM entries WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(ApiFailure::database)?;

    if result.rows_affected() == 0 {
        return Err(ApiFailure::not_found());
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Crée un snapshot SQLite vérifié (`VACUUM INTO` + `integrity_check` +
/// comparaison tables/compteurs) dans le répertoire des sauvegardes
/// mémorisé au démarrage.
async fn create_backup_handler(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<BackupResponse>), ApiFailure> {
    match create_backup(&state.pool, &state.backups_dir).await {
        Ok(path) => Ok((
            StatusCode::CREATED,
            Json(BackupResponse {
                path: path.display().to_string(),
            }),
        )),
        Err(backup_error) => {
            error!("Backup failed: {backup_error}");
            Err(ApiFailure::internal(
                "The database backup could not be created and verified.",
            ))
        }
    }
}

struct ApiFailure {
    status: StatusCode,
    message: String,
}

impl ApiFailure {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: "Entry not found.".into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    fn database(error: sqlx::Error) -> Self {
        error!(?error, "Database request failed");
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Database request failed.".into(),
        }
    }
}

impl IntoResponse for ApiFailure {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(ApiError {
                error: self.message,
            }),
        )
            .into_response()
    }
}

/// Tests intégration du routeur complet sur une base SQLite isolée en
/// répertoire temporaire (jamais la base personnelle).
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use macro_journal_server::config::StorageLayout;
    use tower::ServiceExt;

    async fn test_app(dir: &std::path::Path) -> Router {
        let layout =
            StorageLayout::with_legacy_candidates(dir.join("macro_journal.db"), Vec::new());
        let pool = open_or_initialize(&layout, &sqlx::migrate!("./migrations"))
            .await
            .unwrap();
        Router::new()
            .route("/api/health", get(health))
            .route("/api/entries", get(list_entries).post(create_entry))
            .route(
                "/api/entries/{id}",
                get(get_entry).put(update_entry).delete(delete_entry),
            )
            .route("/api/backups", post(create_backup_handler))
            .with_state(AppState {
                pool,
                backups_dir: layout.backups_dir,
            })
    }

    fn health_request() -> Request<Body> {
        Request::get("/api/health").body(Body::empty()).unwrap()
    }

    fn create_entry_request(title: &str, content: &str) -> Request<Body> {
        let body = serde_json::json!({ "title": title, "content": content });
        Request::post("/api/entries")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    async fn body_string(response: axum::response::Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn health_route_still_works() {
        let dir = tempfile::tempdir().unwrap();
        let app = test_app(dir.path()).await;
        let response = app.oneshot(health_request()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_string(response).await;
        assert!(body.contains("\"status\":\"ok\""));
    }

    #[tokio::test]
    async fn entries_crud_still_works() {
        let dir = tempfile::tempdir().unwrap();
        let app = test_app(dir.path()).await;

        let response = app
            .clone()
            .oneshot(create_entry_request("Titre", "Contenu"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .clone()
            .oneshot(Request::get("/api/entries").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_string(response).await;
        assert!(body.contains("Titre"));

        let response = app
            .clone()
            .oneshot(
                Request::put("/api/entries/1")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::json!({ "title": "Titre 2", "content": "Contenu 2" })
                            .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::delete("/api/entries/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let response = app
            .oneshot(Request::get("/api/entries/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn backup_route_creates_verified_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let app = test_app(dir.path()).await;

        let response = app
            .clone()
            .oneshot(create_entry_request("Avec sauvegarde", "x"))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = app
            .oneshot(Request::post("/api/backups").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = body_string(response).await;
        assert!(body.contains("macro_journal-"), "réponse : {body}");

        // Le snapshot annoncé existe réellement.
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        let path = parsed["path"].as_str().unwrap();
        assert!(std::path::Path::new(path).is_file());
    }
}
