use std::time::Duration;

use anyhow::Result;
use axum::{Router, routing::get};
use tokio::{net::TcpListener, time::{Instant, sleep}};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    Layer, fmt::{self, format::FmtSpan}, layer::SubscriberExt, util::SubscriberInitExt
};
use tracing::debug;
use tracing::warn;
#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = tracing_appender::rolling::daily("/tmp/logs", "ecosystem2.log");
    let (non_blocking  ,_guard) = tracing_appender::non_blocking(file_appender);
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW |FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    let file = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW |FmtSpan::CLOSE)
        .pretty()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).with(file).init();
    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler));

    let listener = TcpListener::bind(addr).await?;
    info!("Starting server at {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index handler completed");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_secs(2)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration =elapsed, "long task completed");
    "Long task completed"
}