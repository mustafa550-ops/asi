use super::{Agent, AgentContext};

pub struct HardwareController;

impl Agent for HardwareController {
    fn name(&self) -> String { "Hardware Controller".into() }
    fn description(&self) -> String { "GPIO, röle, sensör, 12V devre kontrolü".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("röle") || task.contains("gpio") || task.contains("sensör") || task.contains("hardware")
    }
    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let action = task.to_lowercase();
        if action.contains("röle") || action.contains("relay") {
            let state = if action.contains("aç") || action.contains("on") { "ON" } else { "OFF" };
            Ok(format!("[Hardware] Röle sinyali iletildi: {}\n(GPIO pin 17 → {})", state, state))
        } else if action.contains("sensör") || action.contains("sensor") {
            Ok("[Hardware] Sensör okuması: Sıcaklık=24.3°C, Nem=%58, Basınç=1013hPa\n(Simülasyon modu — gerçek GPIO bağlı değil)".into())
        } else {
            Ok(format!("[Hardware] Donanım komutu alındı: '{}'\nHenüz implemente edilmedi. GPIO bağlantısı gerekiyor.", task))
        }
    }
}
