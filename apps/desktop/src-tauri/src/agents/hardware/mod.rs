pub mod gpio;
pub mod relay;
pub mod sensor;

use super::{Agent, AgentContext};

pub struct HardwareController;

impl Agent for HardwareController {
    fn name(&self) -> String {
        "Hardware Controller".into()
    }

    fn description(&self) -> String {
        "GPIO, role, sensor, 12V devre kontrolu".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        let t = task.to_lowercase();
        t.contains("role") || t.contains("gpio") || t.contains("sensor") || t.contains("hardware") || t.contains("kap") || t.contains("zil") || t.contains("pin") || t.contains("sicaklik") || t.contains("basinc")
    }

    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let action = task.to_lowercase();

        if action.contains("role") || action.contains("relay") {
            let state = if action.contains("ac") || action.contains("on") { true } else { false };
            let pin = if action.contains("pin") {
                action.split_whitespace()
                    .filter_map(|w| w.parse::<u8>().ok())
                    .next()
                    .unwrap_or(17)
            } else { 17 };
            relay::set(pin, state)?;
            let label = if state { "ACIK" } else { "KAPALI" };
            Ok(format!("[Hardware] Role pin {} — {}", pin, label))
        } else if action.contains("sensor") || action.contains("sicaklik") || action.contains("basinc") {
            let reading = sensor::read_all()?;
            Ok(format!("[Hardware] Sensor okumalari:\n{}", reading))
        } else if action.contains("gpio") || action.contains("pin") {
            let pin = action.split_whitespace()
                .filter_map(|w| w.parse::<u8>().ok())
                .next()
                .unwrap_or(17);
            let mode = if action.contains("read") || action.contains("oku") { "read" }
                      else if action.contains("high") || action.contains("yuksek") || action.contains("1") { "high" }
                      else { "low" };
            match mode {
                "read" => {
                    let value = gpio::read(pin)?;
                    Ok(format!("[Hardware] GPIO pin {} = {}", pin, value))
                }
                _ => {
                    gpio::write(pin, mode == "high")?;
                    Ok(format!("[Hardware] GPIO pin {} -> {}", pin, mode))
                }
            }
        } else {
            Ok(format!("[Hardware] Bilinmeyen komut: '{}'\nKullanimi: 'role ac/kapa', 'sensor oku', 'gpio pin N oku/yaz'", task))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_returns_correct() {
        let agent = HardwareController;
        assert_eq!(agent.name(), "Hardware Controller");
    }

    #[test]
    fn can_handle_matches_hardware_keywords() {
        let agent = HardwareController;
        assert!(agent.can_handle("role ac"));
        assert!(agent.can_handle("gpio pin 17 oku"));
        assert!(agent.can_handle("sensor sicaklik"));
        assert!(agent.can_handle("kapı zili"));
        assert!(!agent.can_handle("borsa analizi"));
    }
}
