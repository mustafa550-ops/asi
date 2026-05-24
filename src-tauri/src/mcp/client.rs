use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;

pub struct McpClient;

impl McpClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn call(&self, url: &str, method: &str, params: serde_json::Value) -> Result<serde_json::Value, String> {
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| format!("Connection failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1u64,
        });

        write
            .send(tokio_tungstenite::tungstenite::Message::Text(request.to_string().into()))
            .await
            .map_err(|e| format!("Send failed: {}", e))?;

        if let Some(Ok(msg)) = read.next().await {
            let text = msg.to_text().unwrap_or("{}");
            let value: serde_json::Value =
                serde_json::from_str(text).map_err(|e| format!("Parse failed: {}", e))?;
            Ok(value["result"].clone())
        } else {
            Err("No response from server".into())
        }
    }

    pub async fn ping(&self, url: &str) -> Result<bool, String> {
        self.call(url, "ping", json!({})).await.map(|_| true)
    }

    pub async fn list_tools(&self, url: &str) -> Result<Vec<serde_json::Value>, String> {
        let result = self.call(url, "tools/list", json!({})).await?;
        Ok(result["tools"].as_array().cloned().unwrap_or_default())
    }
}
