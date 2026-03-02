use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;

use super::templates;
use crate::storage::Database;

#[derive(Clone)]
struct AppState {
    db: Database,
}

pub async fn start(db: Database, port: u16) -> Result<()> {
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/status", get(api_status))
        .route("/api/snapshot", get(api_snapshot))
        .route("/api/activities", get(api_activities))
        .route("/api/insights", get(api_insights))
        .route("/api/actions", get(api_actions))
        .route("/api/logs", get(api_logs))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("  Web dashboard running at http://localhost:{port}");
    println!("  Press Ctrl+C to stop.\n");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<String> {
    Html(templates::index_html())
}

async fn api_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let snapshot = state.db.get_latest_snapshot().ok().flatten();
    let activity_count = state.db.activity_count().unwrap_or(0);
    let insight_count = state.db.insight_count().unwrap_or(0);

    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "has_snapshot": snapshot.is_some(),
        "activity_count": activity_count,
        "insight_count": insight_count,
        "hostname": snapshot.as_ref().map(|s| s.hostname.clone()).unwrap_or_default(),
        "os": snapshot.as_ref().map(|s| format!("{} {}", s.os_name, s.os_version)).unwrap_or_default(),
    }))
}

async fn api_snapshot(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_latest_snapshot() {
        Ok(Some(snap)) => Json(serde_json::to_value(snap).unwrap()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "No snapshot available").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_activities(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_recent_activities(100) {
        Ok(activities) => Json(serde_json::to_value(activities).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_insights(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_all_insights() {
        Ok(insights) => Json(serde_json::to_value(insights).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_actions(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_all_actions() {
        Ok(actions) => Json(serde_json::to_value(actions).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn api_logs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_action_logs() {
        Ok(logs) => Json(serde_json::to_value(logs).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
