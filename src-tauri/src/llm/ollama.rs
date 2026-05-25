use serde::{Deserialize, Serialize};

/// Ollama Client — Local LLM ile iletişim (§2, §9).
#[derive(Clone)]
pub struct OllamaClient {
    pub base_url: String,
    pub model: String,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self { base_url, model }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&GenerateRequest {
                model: self.model.clone(),
                prompt: prompt.to_string(),
                stream: false,
            })
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;

        Ok(res.response)
    }

    pub fn generate_sync(&self, prompt: &str) -> Result<String, String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(self.generate(prompt))
            .map_err(|e| e.to_string())
    }

    pub fn embedding_sync(&self, input: &str) -> Result<Vec<f32>, String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(self.embedding(input))
            .map_err(|e| e.to_string())
    }

    pub async fn embedding(&self, input: &str) -> Result<Vec<f32>, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": input
            }))
            .send()
            .await?;

        let body = res.json::<serde_json::Value>().await?;
        let embedding = body["embedding"]
            .as_array()
            .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect())
            .unwrap_or_default();

        Ok(embedding)
    }
}
