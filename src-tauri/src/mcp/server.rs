use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct McpServer {
    port: u16,
    tools: Arc<Mutex<HashMap<String, String>>>,
}
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    method: String,
    #[allow(dead_code)]
    params: serde_json::Value,
    id: u64,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: serde_json::Value,
    id: u64,
}

impl McpServer {
    pub fn new(port: u16) -> Self {
        let mut tools = HashMap::new();
        tools.insert("ping".into(), "Health check".into());
        tools.insert("tools/list".into(), "List all available tools".into());
        Self {
            port,
            tools: Arc::new(Mutex::new(tools)),
        }
    }

    pub fn register_tool(&self, name: &str, description: &str) {
        let mut tools = self.tools.lock().unwrap();
        tools.insert(name.to_string(), description.to_string());
    }

    pub async fn start(&self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| format!("MCP server bind error: {}", e))?;
        log::info!("MCP Server listening on ws://{}", addr);

        let tools = Arc::clone(&self.tools);
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let tools = Arc::clone(&tools);
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, tools).await {
                        log::error!("MCP connection error: {}", e);
                    }
                });
            }
        });

        Ok(())
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    tools: Arc<Mutex<HashMap<String, String>>>,
) -> Result<(), String> {
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("WebSocket accept error: {}", e))?;
    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_text() || msg.is_binary() {
            let text = msg.to_text().unwrap_or("");
            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(text) {
                let response = match req.method.as_str() {
                    "ping" => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        result: serde_json::json!({"status": "ok"}),
                        id: req.id,
                    },
                    "tools/list" => {
                        let t = tools.lock().unwrap();
                        let items: Vec<serde_json::Value> = t
                            .iter()
                            .map(|(n, d)| serde_json::json!({"name": n, "description": d}))
                            .collect();
                        JsonRpcResponse {
                            jsonrpc: "2.0".into(),
                            result: serde_json::json!({"tools": items}),
                            id: req.id,
                        }
                    }
                    _ => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        result: serde_json::json!({"error": "method not found"}),
                        id: req.id,
                    },
                };
                let json = serde_json::to_string(&response)
                    .map_err(|e| format!("JSON error: {}", e))?;
                write
                    .send(tokio_tungstenite::tungstenite::Message::Text(json.into()))
                    .await
                    .map_err(|e| format!("Send error: {}", e))?;
            }
        }
    }
    Ok(())
}
