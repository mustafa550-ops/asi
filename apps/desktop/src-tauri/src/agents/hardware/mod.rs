pub mod backend;
pub mod detect;
pub mod events;
pub mod gpio;
pub mod relay;
pub mod safety;
pub mod sensor;

use super::{Agent, AgentContext};
use backend::{GpioBackend, RelayBackend, SensorBackend};

pub struct HardwareController {
    pub gpio: Box<dyn GpioBackend>,
    pub sensor: Box<dyn SensorBackend>,
    pub relay: Box<dyn RelayBackend>,
}

impl HardwareController {
    pub fn new(
        gpio: Box<dyn GpioBackend>,
        sensor: Box<dyn SensorBackend>,
        relay: Box<dyn RelayBackend>,
    ) -> Self {
        HardwareController { gpio, sensor, relay }
    }

    pub fn new_real() -> Self {
        Self::new(
            Box::new(backend::RealGpio),
            Box::new(backend::RealSensor),
            Box::new(backend::RealRelay),
        )
    }

    pub fn new_sim() -> Self {
        Self::new(
            Box::new(backend::SimGpio::new()),
            Box::new(backend::SimSensor::new()),
            Box::new(backend::RealRelay),
        )
    }
}

impl Agent for HardwareController {
    fn name(&self) -> String {
        "Hardware Controller".into()
    }

    fn description(&self) -> String {
        "GPIO, role, sensor, 12V devre kontrolu".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        let t = task.to_lowercase();
        t.contains("role") || t.contains("gpio") || t.contains("sensor")
            || t.contains("hardware") || t.contains("kap") || t.contains("zil")
            || t.contains("pin") || t.contains("sicaklik") || t.contains("basinc")
    }

    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let action = task.to_lowercase();

        if action.contains("role") || action.contains("relay") {
            let state = if action.contains("ac") || action.contains("on") { true } else { false };
            let pin = parse_pin(&action).unwrap_or(17);
            self.relay.set(pin, state)?;
            let label = if state { "ACIK" } else { "KAPALI" };
            Ok(format!("[Hardware] Role pin {} — {}", pin, label))
        } else if action.contains("sensor") || action.contains("sicaklik") || action.contains("basinc") {
            let reading = self.sensor.read_all()?;
            Ok(format!("[Hardware] Sensor okumalari:\n{}", reading))
        } else if action.contains("gpio") || action.contains("pin") {
            let pin = parse_pin(&action).unwrap_or(17);
            let mode = if action.contains("read") || action.contains("oku") { "read" }
                      else if action.contains("high") || action.contains("yuksek") || action.contains("1") { "high" }
                      else { "low" };
            match mode {
                "read" => {
                    let value = self.gpio.read(pin)?;
                    Ok(format!("[Hardware] GPIO pin {} = {}", pin, value))
                }
                _ => {
                    self.gpio.write(pin, mode == "high")?;
                    Ok(format!("[Hardware] GPIO pin {} -> {}", pin, mode))
                }
            }
        } else {
            Ok(format!("[Hardware] Bilinmeyen komut: '{}'\nKullanimi: 'role ac/kapa', 'sensor oku', 'gpio pin N oku/yaz'", task))
        }
    }
}

fn parse_pin(action: &str) -> Option<u8> {
    action.split_whitespace()
        .filter_map(|w| w.parse::<u8>().ok())
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use backend::{SimGpio, SimRelay};
    use std::sync::Arc;
    use crate::agents::ApprovalLevel;

    fn mock_context() -> AgentContext<'static> {
        AgentContext {
            ollama: Box::leak(Box::new(crate::llm::OllamaClient::new(
                "http://localhost:11434".into(),
                "test-model".into(),
            ))),
            claude: None,
            memory: None,
            event_bus: None,
            approval: ApprovalLevel::Observer,
            vosk_model_path: "",
        }
    }

    fn test_controller() -> HardwareController {
        let sim_gpio = SimGpio::new();
        let gpio_box: Box<dyn GpioBackend> = Box::new(sim_gpio.clone());
        HardwareController {
            gpio: gpio_box,
            sensor: Box::new(backend::SimSensor::new()),
            relay: Box::new(SimRelay::new(Arc::new(sim_gpio))),
        }
    }

    #[test]
    fn name_returns_correct() {
        let agent = test_controller();
        assert_eq!(agent.name(), "Hardware Controller");
    }

    #[test]
    fn can_handle_matches_hardware_keywords() {
        let agent = test_controller();
        assert!(agent.can_handle("role ac"));
        assert!(agent.can_handle("gpio pin 17 oku"));
        assert!(agent.can_handle("sensor sicaklik"));
        assert!(agent.can_handle("kapı zili"));
        assert!(!agent.can_handle("borsa analizi"));
    }

    #[test]
    fn execute_relay_on() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("role ac", &ctx).unwrap();
        assert!(result.contains("ACIK"));
    }

    #[test]
    fn execute_relay_off() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("role kapa", &ctx).unwrap();
        assert!(result.contains("KAPALI"));
    }

    #[test]
    fn execute_gpio_read() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("gpio pin 17 oku", &ctx).unwrap();
        assert!(result.contains("GPIO pin 17"));
    }

    #[test]
    fn execute_gpio_write_high() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("gpio pin 17 high", &ctx).unwrap();
        assert!(result.contains("high"));
    }

    #[test]
    fn execute_sensor_read() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("sensor sicaklik", &ctx).unwrap();
        assert!(result.contains("Sensor okumalari"));
    }

    #[test]
    fn execute_unknown_command() {
        let agent = test_controller();
        let ctx = mock_context();
        let result = agent.execute("xyzzy", &ctx).unwrap();
        assert!(result.contains("Bilinmeyen komut"));
    }

    #[test]
    fn sim_gpio_tracks_state_across_execute() {
        let sim_gpio = SimGpio::new();
        let gpio_box: Box<dyn GpioBackend> = Box::new(sim_gpio.clone());
        let agent = HardwareController::new(
            gpio_box,
            Box::new(backend::SimSensor::new()),
            Box::new(SimRelay::new(Arc::new(sim_gpio))),
        );
        let ctx = mock_context();
        agent.execute("gpio pin 17 high", &ctx).unwrap();
        let result = agent.execute("gpio pin 17 oku", &ctx).unwrap();
        assert!(result.contains("= 1"));
    }
}
