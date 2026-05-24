use super::Agent;

/// Hardware Controller — GPIO, röle, sensör, 12V devre kontrolü (§4.1).
pub struct HardwareController;

impl Agent for HardwareController {
    fn name(&self) -> String { "Hardware Controller".into() }
    fn description(&self) -> String { "GPIO, röle, sensör kontrolü".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("röle") || task.contains("gpio") || task.contains("sensör")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Donanım komutu iletildi".into())
    }
}
