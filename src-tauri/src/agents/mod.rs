pub mod intent_judge;
pub mod diagnostic;
pub mod hardware;
pub mod market_analyst;
pub mod system_manager;
pub mod document_analyst;
pub mod voice_handler;
pub mod supervisor;

use crate::bridge::event_bus::EventBus;
use crate::core::memory_manager::MemoryManager;
use crate::llm::claude::ClaudeClient;
use crate::llm::OllamaClient;

#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalLevel {
    Observer,
    SemiAutonomous,
    Strategic,
}

pub struct AgentContext<'a> {
    pub ollama: &'a OllamaClient,
    pub claude: Option<&'a ClaudeClient>,
    pub memory: Option<&'a MemoryManager>,
    pub event_bus: Option<&'a EventBus>,
    pub approval: ApprovalLevel,
    pub vosk_model_path: &'a str,
}

pub trait Agent {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn can_handle(&self, task: &str) -> bool;
    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String>;
}
