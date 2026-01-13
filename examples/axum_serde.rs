use std::sync::Arc;

use anyhow::Result;

use axum::routing::patch;
use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{debug, info, instrument, level_filters::LevelFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{
    Layer as _,
    fmt::{self, format::FmtSpan},
    util::SubscriberInitExt,
};
#[derive(Debug, PartialEq, Serialize, Clone)]
struct User {
    name: String,
    age: u32,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    age: Option<u32>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Axum".to_string(), "Serde".to_string()],
    };

    let user = Arc::new(Mutex::new(user));
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    let app = Router::new()
        .route("/", get(user_handler))
        .route("/", patch(update_handler))
        .with_state(user);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    debug!("index handler started");
    let user = user.lock().await;
    let user_clone = user.clone();
    info!(http.status = 200, "index handler completed");
    Json(user_clone)
}


#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(user_update): Json<UserUpdate>,
) -> Json<User> {
    debug!("update handler started");
    let mut user = user.lock().await;

    if let Some(age) = user_update.age {
        user.age = age;
    }
    if let Some(skills) = user_update.skills {
        user.skills = skills;
    }

    let updated_user = user.clone();
    info!(http.status = 200, "update handler completed");
    Json(updated_user)
}
