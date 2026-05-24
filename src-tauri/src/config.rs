use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub ollama_url: String,
    pub ollama_model: String,
    pub claude_api_key: Option<String>,
    pub db_path: String,
    pub approval_level: ApprovalLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalLevel {
    Observer,
    SemiAutonomous,
    Strategic,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            ollama_model: "qwen2.5:1.5b".to_string(),
            claude_api_key: None,
            db_path: "adler.db".to_string(),
            approval_level: ApprovalLevel::SemiAutonomous,
        }
    }
}
