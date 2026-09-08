use crate::state::AppState;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

/// Milestone 1 placeholder: real upload/list handlers land in Milestone 2
/// once the documents/pages/chunks schema exists.
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_documents))
}

async fn list_documents(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({ "documents": [] }))
}
