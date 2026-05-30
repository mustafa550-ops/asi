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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_level_debug_clone() {
        let levels = vec![
            ApprovalLevel::Observer,
            ApprovalLevel::SemiAutonomous,
            ApprovalLevel::Strategic,
        ];
        for level in &levels {
            let cloned = level.clone();
            assert_eq!(level, &cloned);
            let _debug = format!("{:?}", level);
        }
    }

    #[test]
    fn approval_level_partial_eq() {
        assert_eq!(ApprovalLevel::Observer, ApprovalLevel::Observer);
        assert_ne!(ApprovalLevel::Observer, ApprovalLevel::Strategic);
    }

    #[test]
    fn agent_trait_object_safe() {
        let _: &dyn Agent = &intent_judge::IntentJudge;
        let _: &dyn Agent = &diagnostic::DiagnosticAgent;
        let _: &dyn Agent = &system_manager::SystemManager;
        let _: &dyn Agent = &supervisor::SupervisorAgent;
    }
}
