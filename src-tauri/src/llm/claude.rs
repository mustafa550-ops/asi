/// Claude API Client — Cloud fallback LLM (§2).
pub struct ClaudeClient {
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn query(&self, _prompt: &str) -> Result<String, String> {
        // Anthropic Claude API çağrısı (onaylı)
        Ok("Claude yanıtı".into())
    }
}
