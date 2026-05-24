use serde::{Deserialize, Serialize};

/// Ollama Client — Local LLM ile iletişim (§2, §9).
#[derive(Clone)]
pub struct OllamaClient {
    pub base_url: String,
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
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&GenerateRequest {
                model: model.to_string(),
                prompt: prompt.to_string(),
                stream: false,
            })
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;

        Ok(res.response)
    }

    pub async fn embedding(&self, model: &str, input: &str) -> Result<Vec<f32>, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&serde_json::json!({
                "model": model,
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
