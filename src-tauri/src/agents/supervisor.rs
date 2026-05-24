use super::Agent;

/// Supervisor Agent — Diğer ajanların hatalarını düzeltir (§4.1).
pub struct SupervisorAgent;

impl Agent for SupervisorAgent {
    fn name(&self) -> String { "Supervisor Agent".into() }
    fn description(&self) -> String { "Ajan hata düzeltme ve optimizasyon".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("supervisor") || task.contains("optimize")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Süpervizör analizi tamamlandı".into())
    }
}
