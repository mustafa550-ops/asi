use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub active_agents: usize,
    pub zero_mock_policy: bool,
}

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    // Acquire read lock to check the state safely
    let state_reader = state.read().await;
    
    // According to Zero-Mock policy, we report actual system facts
    let response = HealthResponse {
        status: "up".to_string(),
        active_agents: state_reader.active_agents,
        zero_mock_policy: true,
    };
    
    (StatusCode::OK, Json(response))
}
