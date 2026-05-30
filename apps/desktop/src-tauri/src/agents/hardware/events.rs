use super::backend::SensorBackend;
use crate::bridge::event_bus::EventBus;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub struct HardwareEventMonitor {
    event_bus: EventBus,
    sensor: Box<dyn SensorBackend>,
    running: Arc<AtomicBool>,
    max_temp: f64,
    poll_interval: Duration,
}

impl HardwareEventMonitor {
    pub fn new(
        event_bus: EventBus,
        sensor: Box<dyn SensorBackend>,
        max_temp: f64,
    ) -> Self {
        HardwareEventMonitor {
            event_bus,
            sensor,
            running: Arc::new(AtomicBool::new(false)),
            max_temp,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn with_interval(mut self, secs: u64) -> Self {
        self.poll_interval = Duration::from_secs(secs);
        self
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let event_bus = self.event_bus.clone();
        let max_temp = self.max_temp;
        let interval = self.poll_interval;

        std::thread::spawn(move || {
            let mut last_high_temp = false;
            while running.load(Ordering::SeqCst) {
                match crate::agents::hardware::backend::RealSensor.read_all() {
                    Ok(data) => {
                        let is_high = data.contains("Sicaklik") && {
                            data.split(|c| c == ' ' || c == '\n')
                                .filter_map(|w| w.trim().parse::<f64>().ok())
                                .any(|t| t > max_temp)
                        };
                        if is_high && !last_high_temp {
                            event_bus.emit("hardware-warning", &format!("Yuksek sicaklik tespit edildi: {}", data));
                        }
                        last_high_temp = is_high;
                    }
                    Err(e) => {
                        event_bus.emit("hardware-error", &format!("Sensor okuma hatasi: {}", e));
                    }
                }
                std::thread::sleep(interval);
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
