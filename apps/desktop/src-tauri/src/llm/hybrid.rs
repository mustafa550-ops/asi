use crate::llm::OllamaClient;
use crate::llm::claude::ClaudeClient;

#[derive(Debug)]
pub enum ModelTier {
    Local,
    Cloud,
    Hybrid,
}

pub struct HybridRouter {
    ollama: OllamaClient,
    claude: Option<ClaudeClient>,
    tier: ModelTier,
    local_token_budget: u32,
}

impl HybridRouter {
    pub fn new(ollama: OllamaClient, claude: Option<ClaudeClient>) -> Self {
        Self { ollama, claude, tier: ModelTier::Hybrid, local_token_budget: 2000 }
    }

    pub fn set_tier(&mut self, tier: ModelTier) {
        self.tier = tier;
    }

    pub fn generate(&self, prompt: &str) -> Result<String, String> {
        match self.tier {
            ModelTier::Local => self.ollama.generate_sync(prompt),
            ModelTier::Cloud => match &self.claude {
                Some(c) => c.generate_sync(prompt, 4096),
                None => self.ollama.generate_sync(prompt),
            },
            ModelTier::Hybrid => {
                let token_count = (prompt.len() / 4) as u32;
                if token_count > self.local_token_budget {
                    match &self.claude {
                        Some(c) => c.generate_sync(prompt, 4096),
                        None => self.ollama.generate_sync(prompt),
                    }
                } else {
                    self.ollama.generate_sync(prompt)
                }
            }
        }
    }

    pub fn report(&self) -> String {
        format!("Hybrid Router: {:?}, Local: {}, Cloud: {}",
            self.tier,
            self.ollama.model,
            self.claude.as_ref().map_or("yok".into(), |c| c.model.clone()))
    }
}
