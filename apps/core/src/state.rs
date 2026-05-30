use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AppConfig;
use crate::events::EventBus;
use crate::memory::MemoryManager;

pub struct AppStateInner {
    pub config: AppConfig,
    pub event_bus: EventBus,
    pub active_agents: usize,
    pub memory: Arc<MemoryManager>,
}

pub type AppState = Arc<RwLock<AppStateInner>>;

impl AppStateInner {
    pub fn new(config: AppConfig, memory: MemoryManager) -> Self {
        Self {
            config,
            event_bus: EventBus::new(1024), // Capacity
            active_agents: 0,
            memory: Arc::new(memory),
        }
    }
}

pub fn create_shared_state(config: AppConfig, memory: MemoryManager) -> AppState {
    Arc::new(RwLock::new(AppStateInner::new(config, memory)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, LlmConfig, OllamaConfig, ClaudeConfig, VoiceConfig, HardwareConfig, MemoryConfig};
    use crate::memory::MemoryManager;

    fn test_config() -> AppConfig {
        AppConfig {
            llm: LlmConfig {
                default_provider: "ollama".into(),
                ollama: OllamaConfig {
                    host: "http://localhost:11434".into(),
                    model: "qwen2.5:1.5b".into(),
                },
                claude: ClaudeConfig {
                    model: "claude-sonnet-4-20250514".into(),
                    max_tokens: 4096,
                },
            },
            voice: VoiceConfig {
                wake_word: "hey adler".into(),
                stt_provider: "vosk".into(),
                tts_provider: "espeak".into(),
                language: "tr".into(),
            },
            hardware: HardwareConfig { enabled: false },
            memory: MemoryConfig {
                db_path: ":memory:".into(),
                vector_dim: 384,
                encryption_enabled: false,
            },
        }
    }

    #[test]
    fn state_inner_initializes_with_config() {
        let config = test_config();
        let manager = MemoryManager::new_in_memory().unwrap();
        let state = AppStateInner::new(config.clone(), manager);
        assert_eq!(state.config.llm.default_provider, "ollama");
        assert_eq!(state.active_agents, 0);
    }

    #[test]
    fn create_shared_state_returns_arc() {
        let config = test_config();
        let manager = MemoryManager::new_in_memory().unwrap();
        let state = create_shared_state(config, manager);
        let inner = state.blocking_read();
        assert!(inner.event_bus.subscribe().try_recv().is_err()); // nothing published yet
    }
}
