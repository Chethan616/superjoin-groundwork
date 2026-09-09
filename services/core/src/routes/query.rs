use crate::error::AppError;
use crate::intelligence::AnswerPlanRequest;
use crate::state::AppState;
use crate::{db, routes::facts::to_view};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(run_query))
}

#[derive(Deserialize)]
struct QueryRequest {
    question: String,
}

/// Hybrid retrieval (vector similarity over chunks + facts) followed by a
/// Groq-generated, schema-validated "answer plan" the frontend renders via
/// its typed generative-UI component registry — never raw model output.
async fn run_query(State(state): State<AppState>, Json(req): Json<QueryRequest>) -> Result<Json<serde_json::Value>, AppError> {
    if req.question.trim().is_empty() {
        return Err(AppError::BadRequest("question must not be empty".into()));
    }

    let embed_resp = state.intelligence.embed(vec![req.question.clone()]).await.map_err(AppError::from)?;
    let Some(embedding) = embed_resp.embeddings.into_iter().next() else {
        return Err(AppError::Internal(anyhow::anyhow!("embedding service returned no vector")));
    };

    let chunk_matches = db::search_chunks_by_embedding(&state.db, embedding.clone(), 8).await?;
    let fact_matches = db::search_facts_by_embedding(&state.db, embedding, 12).await?;

    let mut retrieved_facts = Vec::with_capacity(fact_matches.len());
    for f in fact_matches {
        let view = to_view(&state, f).await?;
        let relationships = db::relationships_for_fact(&state.db, view.id).await?;
        retrieved_facts.push(json!({ "fact": view, "relationships": relationships }));
    }
    let retrieved_chunks: Vec<serde_json::Value> = chunk_matches.into_iter().map(|c| json!(c)).collect();

    let plan = state.intelligence.answer_plan(&AnswerPlanRequest {
        question: req.question,
        retrieved_facts,
        retrieved_chunks,
    }).await.map_err(AppError::from)?;

    Ok(Json(plan.plan))
}
