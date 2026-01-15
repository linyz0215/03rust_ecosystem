
use anyhow::Result;
use axum::{Json, Router, extract::Path, routing::{get, post}};
use http::StatusCode;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let url = "postgres://postgres:postgres@localhost:5432/shortener";
    let state = AppState::try_new(url).await?;
    info!("Connected to database: {}", url);
    let addr = "0.0.0.0:9876";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}",addr);

    let app = Router::new().route("/", post(shorten)).route("/:id", get(redirect)).with_state(state);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn shorten(Json(data): Json<ShortenReq>) -> Result<Json<ShortenRes>, StatusCode> { 
    todo!()
}

async fn redirect(Path(id): Path<String>) -> &'static str {
    "Redirect endpoint"
}

impl AppState {
    async fn try_new(url: &str) -> Result<Self> {
        let pool = PgPool::connect(url).await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS urls (
                id CHAR(6) PRIMARY KEY,
                url TEXT NOT NULL UNIQUE
            )
            "#,
        ).execute(&pool).await?;
        Ok(Self { db: pool })
    }
}