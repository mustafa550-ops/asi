pub mod intent_judge;
pub mod diagnostic;
pub mod hardware;
pub mod market_analyst;
pub mod system_manager;
pub mod document_analyst;
pub mod voice_handler;
pub mod supervisor;

/// Agent trait — Tüm ajanlar bu interface'i implement eder (§4.1).
pub trait Agent {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn can_handle(&self, task: &str) -> bool;
    fn execute(&self, task: &str) -> Result<String, String>;
}
