use serde::Deserialize;
use std::path::Path;
use crate::error::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct OllamaConfig {
    pub host: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClaudeConfig {
    pub model: String,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LlmConfig {
    pub default_provider: String,
    pub ollama: OllamaConfig,
    pub claude: ClaudeConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceConfig {
    pub wake_word: String,
    pub stt_provider: String,
    pub tts_provider: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HardwareConfig {
    pub enabled: bool,
    // Add specific pins later
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemoryConfig {
    pub db_path: String,
    pub vector_dim: usize,
    pub encryption_enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub llm: LlmConfig,
    pub voice: VoiceConfig,
    pub hardware: HardwareConfig,
    pub memory: MemoryConfig,
}

impl AppConfig {
    pub fn load(config_path: impl AsRef<Path>) -> Result<Self> {
        // Load .env variables (ignoring errors if .env is missing)
        let _ = dotenvy::dotenv();

        let settings = config::Config::builder()
            .add_source(config::File::with_name(config_path.as_ref().to_str().unwrap()))
            // Override with environment variables using `ADLER_` prefix if needed
            .add_source(config::Environment::with_prefix("ADLER").separator("__"))
            .build()?;

        let app_config: AppConfig = settings.try_deserialize()?;
        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn deserialize_minimal_config() {
        let yaml = r#"
llm:
  default_provider: ollama
  ollama:
    host: http://localhost:11434
    model: qwen2.5:1.5b
  claude:
    model: claude-sonnet-4-20250514
    max_tokens: 4096
voice:
  wake_word: hey adler
  stt_provider: vosk
  tts_provider: espeak
  language: tr
hardware:
  enabled: false
memory:
  db_path: /tmp/adler.db
  vector_dim: 384
  encryption_enabled: true
"#;
        let config: AppConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.llm.default_provider, "ollama");
        assert_eq!(config.llm.ollama.host, "http://localhost:11434");
        assert_eq!(config.llm.ollama.model, "qwen2.5:1.5b");
        assert_eq!(config.llm.claude.max_tokens, 4096);
        assert_eq!(config.voice.wake_word, "hey adler");
        assert!(!config.hardware.enabled);
        assert!(config.memory.encryption_enabled);
    }

    #[test]
    fn deserialize_yaml_from_reader() {
        let yaml = r#"
llm:
  default_provider: claude
  ollama:
    host: http://ollama:11434
    model: llama3
  claude:
    model: claude-opus-4-20250514
    max_tokens: 8192
voice:
  wake_word: adler
  stt_provider: whisper
  tts_provider: elevenlabs
  language: en
hardware:
  enabled: true
memory:
  db_path: /data/adler.db
  vector_dim: 768
  encryption_enabled: false
"#;
        let config: AppConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.llm.default_provider, "claude");
        assert_eq!(config.llm.ollama.host, "http://ollama:11434");
        assert_eq!(config.voice.stt_provider, "whisper");
        assert!(config.hardware.enabled);
    }

    #[test]
    fn load_from_yaml_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file_path = dir.path().join("config.yaml");
        std::fs::write(&file_path, r#"
llm:
  default_provider: ollama
  ollama:
    host: http://localhost:11434
    model: test-model
  claude:
    model: claude-test
    max_tokens: 1024
voice:
  wake_word: test
  stt_provider: test
  tts_provider: test
  language: tr
hardware:
  enabled: false
memory:
  db_path: /tmp/test.db
  vector_dim: 384
  encryption_enabled: false
"#).unwrap();
        let config = AppConfig::load(&file_path).unwrap();
        assert_eq!(config.llm.ollama.model, "test-model");
    }
}
