use axum::{
    extract::{State, Path as AxumPath, Multipart},
    routing::{get, post},
    Json, Router,
    http::Method,
};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub mod kofferwechsel;
use crate::kofferwechsel::{KofferManagement, KofferwechselAuftrag, AuftragsStatus, Auftraggeber, Koffer, Fahrgestell};

type AppState = SqlitePool;

#[tokio::main]
async fn main() {
    fs::create_dir_all("data/images").expect("Konnte Bilderverzeichnis nicht erstellen");

    let db_url = "sqlite:data/kofferwechsel.db";
    let connection_options = SqliteConnectOptions::from_str(db_url)
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Kann Datenbank nicht verbinden");

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS koffer_state (
                id INTEGER PRIMARY KEY NOT NULL,
                state_json TEXT NOT NULL
            );
            "#,
    )
    .execute(&pool)
    .await
    .expect("Tabelle konnte nicht erstellt werden");

    let mut initial_management = KofferManagement::new();
    let initial_state_json = serde_json::to_string(&initial_management).unwrap();
    sqlx::query("INSERT OR IGNORE INTO koffer_state (id, state_json) VALUES (1, ?);")
        .bind(initial_state_json)
        .execute(&pool)
        .await
        .expect("Initialer State konnte nicht eingefügt werden");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![axum::http::header::CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/state", get(get_state).post(update_state))
        .route("/api/upload/{nr}", post(upload_image))
        .route("/api/delete-image/{nr}/{filename}", post(delete_image))
        .nest_service("/api/images", ServeDir::new("data/images"))
        .with_state(pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Kofferwechsel-Backend lauscht auf http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(State(pool): State<AppState>) -> Json<KofferManagement> {
    let result: (String,) = sqlx::query_as("SELECT state_json FROM koffer_state WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap_or_else(|_| ("{\"auftraege\":[], \"kunden\":[]}".to_string(),));
    Json(serde_json::from_str(&result.0).unwrap())
}

async fn update_state(State(pool): State<AppState>, Json(new_state): Json<KofferManagement>) {
    let state_json = serde_json::to_string(&new_state).unwrap();
    sqlx::query("UPDATE koffer_state SET state_json = ? WHERE id = 1")
        .bind(state_json)
        .execute(&pool)
        .await
        .unwrap();
}

async fn upload_image(
    State(pool): State<AppState>,
    AxumPath(nr): AxumPath<String>,
    mut multipart: Multipart,
) -> Result<String, String> {
    if let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let name = field.file_name().unwrap_or("image.jpg").to_string();
        let data = field.bytes().await.map_err(|e| e.to_string())?;
        let filename = format!("{}_{}", nr, name);
        let path = Path::new("data/images").join(&filename);
        let mut file = tokio::fs::File::create(&path).await.map_err(|e| e.to_string())?;
        file.write_all(&data).await.map_err(|e| e.to_string())?;

        let result: (String,) = sqlx::query_as("SELECT state_json FROM koffer_state WHERE id = 1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
        let mut management: KofferManagement = serde_json::from_str(&result.0).unwrap();
        if let Some(a) = management.auftraege.iter_mut().find(|a| a.auftrags_nummer == nr) {
            a.bilder.push(filename.clone());
            let state_json = serde_json::to_string(&management).unwrap();
            sqlx::query("UPDATE koffer_state SET state_json = ? WHERE id = 1").bind(state_json).execute(&pool).await.map_err(|e| e.to_string())?;
        }
        return Ok(filename);
    }
    Err("Kein Feld gefunden".to_string())
}

async fn delete_image(
    State(pool): State<AppState>,
    AxumPath((nr, filename)): AxumPath<(String, String)>,
) -> Result<String, String> {
    let path = Path::new("data/images").join(&filename);
    if path.exists() { let _ = fs::remove_file(path); }
    let result: (String,) = sqlx::query_as("SELECT state_json FROM koffer_state WHERE id = 1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let mut management: KofferManagement = serde_json::from_str(&result.0).unwrap();
    if let Some(a) = management.auftraege.iter_mut().find(|a| a.auftrags_nummer == nr) {
        a.bilder.retain(|b| b != &filename);
        let state_json = serde_json::to_string(&management).unwrap();
        sqlx::query("UPDATE koffer_state SET state_json = ? WHERE id = 1").bind(state_json).execute(&pool).await.map_err(|e| e.to_string())?;
    }
    Ok("Gelöscht".to_string())
}
