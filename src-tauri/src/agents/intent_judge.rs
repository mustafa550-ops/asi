use super::Agent;

/// Intent Judge — Niyet analizi, intent classification (§4.1).
pub struct IntentJudge;

impl Agent for IntentJudge {
    fn name(&self) -> String { "Intent Judge".into() }
    fn description(&self) -> String { "Niyet analizi ve intent classification".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("niyet") || task.contains("intent") || task.contains("ne yapmalı")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Sorgu/Eylem/Analiz".into())
    }
}
