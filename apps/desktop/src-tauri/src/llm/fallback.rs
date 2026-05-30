use crate::llm::claude::ClaudeClient;
use crate::llm::OllamaClient;

pub struct FallbackChain {
    ollama: OllamaClient,
    claude: Option<ClaudeClient>,
    models: Vec<String>,
}

impl FallbackChain {
    pub fn new(ollama: OllamaClient, claude: Option<ClaudeClient>) -> Self {
        Self { ollama, claude, models: Vec::new() }
    }

    pub fn with_model_priority(mut self, models: Vec<String>) -> Self {
        self.models = models;
        self
    }

    pub fn generate(&self, prompt: &str) -> Result<String, String> {
        match self.ollama.generate_sync(prompt) {
            Ok(response) => Ok(response),
            Err(e) => {
                log::warn!("Ollama hatasi (fallback): {}", e);
                match &self.claude {
                    Some(claude) => {
                        log::info!("Claude fallback devrede");
                        claude.generate_sync(prompt, 4096)
                    }
                    None => Err(format!("Ollama ve Claude kullanilamiyor: {}", e)),
                }
            }
        }
    }

    pub fn generate_with_priority(&self, prompt: &str) -> Result<String, String> {
        if self.models.is_empty() {
            return self.generate(prompt);
        }
        for model in &self.models {
            let custom = OllamaClient::new(self.ollama.base_url.clone(), model.clone());
            match custom.generate_sync(prompt) {
                Ok(response) => return Ok(response),
                Err(e) => log::warn!("Model '{}' hatasi: {}", model, e),
            }
        }
        self.generate(prompt)
    }
}
