use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct ServerState {
    pub app_handle: AppHandle,
}

#[derive(Deserialize)]
pub struct IpcRequest {
    pub cmd: String,
    pub args: Option<Value>,
}

#[derive(Serialize)]
pub struct IpcResponse {
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
}

pub async fn start_server(app_handle: AppHandle) {
    let state = ServerState { app_handle };

    let app = Router::new()
        .route("/ipc", post(handle_ipc))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1421")
        .await
        .unwrap();

    log::info!("IPC HTTP Fallback server running on 127.0.0.1:1421");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_ipc(
    State(_state): State<ServerState>,
    Json(payload): Json<IpcRequest>,
) -> Json<IpcResponse> {
    // In a real implementation, you would route the IPC request to the Tauri AppState
    // and call the corresponding rust command.
    // For this fallback, we'll return a mock response indicating it was processed via HTTP.
    
    log::info!("Fallback IPC received command: {}", payload.cmd);

    let response_data = match payload.cmd.as_str() {
        "send_command" => Value::String("HTTP üzerinden aldım. Tauri arayüzü tam çalışmıyor gibi görünüyor, web tarayıcısından test ediyorsunuz.".to_string()),
        "start_listening" => Value::String("Listening started (mock via HTTP)".to_string()),
        "stop_listening" => Value::String("Sesli mesaj (mock via HTTP)".to_string()),
        "list_skills" => serde_json::json!([
            {
                "name": "system-monitor",
                "description": "Monitors system resources like CPU and RAM.",
                "active": true,
                "version": "1.0",
                "approval_level": "auto",
                "triggers": ["cpu", "ram", "sistem"]
            },
            {
                "name": "crypto-analyst",
                "description": "Analyzes crypto prices via API.",
                "active": false,
                "version": "1.2",
                "approval_level": "strategic",
                "triggers": ["kripto", "bitcoin", "fiyat"]
            }
        ]),
        "toggle_skill" | "delete_skill" | "add_skill_md" => Value::String(format!("[mock via HTTP] {} completed", payload.cmd)),
        _ => Value::String(format!("[mock via HTTP] {} completed", payload.cmd)),
    };

    Json(IpcResponse {
        success: true,
        data: Some(response_data),
        error: None,
    })
}
