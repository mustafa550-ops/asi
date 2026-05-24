use super::Agent;

/// System Manager — RAM/CPU izleme, process yönetimi (§4.1).
pub struct SystemManager;

impl Agent for SystemManager {
    fn name(&self) -> String { "System Manager".into() }
    fn description(&self) -> String { "Sistem durumu izleme ve yönetim".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("sistem") || task.contains("ram") || task.contains("cpu")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Sistem durumu raporlandı".into())
    }
}
