use serde_json::json;

pub struct ClaudeClient {
    api_key: String,
    pub model: String,
    base_url: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-sonnet-4-20250514".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    pub fn new_with_config(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    pub async fn generate(&self, prompt: &str, max_tokens: u32) -> Result<String, String> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": self.model,
                "max_tokens": max_tokens,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await
            .map_err(|e| format!("Claude API request failed: {}", e))?;

        let status = res.status();
        let body = res.json::<serde_json::Value>().await
            .map_err(|e| format!("Claude parse error (status {}): {}", status, e))?;

        if !status.is_success() {
            let error_msg = body["error"]["message"].as_str().unwrap_or("unknown error");
            return Err(format!("Claude API error ({}): {}", status, error_msg));
        }

        let content = body["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["text"].as_str())
            .unwrap_or("Claude: yanıt alınamadı")
            .to_string();

        Ok(content)
    }

    pub fn generate_sync(&self, prompt: &str, max_tokens: u32) -> Result<String, String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(self.generate(prompt, max_tokens))
    }
}
