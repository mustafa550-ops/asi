use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SafetyThresholds {
    pub max_temp: f64,
    pub min_voltage: f64,
    pub max_relay_cycles_per_min: u32,
}

impl Default for SafetyThresholds {
    fn default() -> Self {
        SafetyThresholds {
            max_temp: 85.0,
            min_voltage: 11.0,
            max_relay_cycles_per_min: 30,
        }
    }
}

pub struct SafetyMonitor {
    last_heartbeat: Instant,
    thresholds: SafetyThresholds,
    relay_cycles: Vec<Instant>,
    failsafe_engaged: bool,
}

impl SafetyMonitor {
    pub fn new(thresholds: SafetyThresholds) -> Self {
        SafetyMonitor {
            last_heartbeat: Instant::now(),
            thresholds,
            relay_cycles: Vec::new(),
            failsafe_engaged: false,
        }
    }

    pub fn pet(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    pub fn check_heartbeat(&self, timeout: Duration) -> bool {
        self.last_heartbeat.elapsed() < timeout
    }

    pub fn record_relay_cycle(&mut self) -> Result<(), String> {
        let now = Instant::now();
        self.relay_cycles.retain(|t| now.duration_since(*t) < Duration::from_secs(60));
        if self.relay_cycles.len() >= self.thresholds.max_relay_cycles_per_min as usize {
            return Err(format!(
                "Role asiri kullanim: dakikada {} (limit: {})",
                self.relay_cycles.len() + 1,
                self.thresholds.max_relay_cycles_per_min
            ));
        }
        self.relay_cycles.push(now);
        Ok(())
    }

    pub fn check_temperature(&self, current_temp: f64) -> Result<(), String> {
        if current_temp > self.thresholds.max_temp {
            return Err(format!(
                "Yuksek sicaklik: {:.1}C (limit: {:.1}C). Failsafe devrede!",
                current_temp, self.thresholds.max_temp
            ));
        }
        Ok(())
    }

    pub fn engage_failsafe(&mut self) {
        self.failsafe_engaged = true;
    }

    pub fn disengage_failsafe(&mut self) {
        self.failsafe_engaged = false;
    }

    pub fn is_failsafe(&self) -> bool {
        self.failsafe_engaged
    }

    pub fn reset_relay_cycles(&mut self) {
        self.relay_cycles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_within_timeout() {
        let monitor = SafetyMonitor::new(SafetyThresholds::default());
        assert!(monitor.check_heartbeat(Duration::from_secs(10)));
    }

    #[test]
    fn heartbeat_expires() {
        let mut monitor = SafetyMonitor::new(SafetyThresholds::default());
        std::thread::sleep(Duration::from_millis(5));
        assert!(!monitor.check_heartbeat(Duration::from_millis(1)));
    }

    #[test]
    fn pet_refreshes_heartbeat() {
        let mut monitor = SafetyMonitor::new(SafetyThresholds::default());
        std::thread::sleep(Duration::from_millis(5));
        monitor.pet();
        assert!(monitor.check_heartbeat(Duration::from_millis(1)));
    }

    #[test]
    fn temperature_threshold_triggers() {
        let monitor = SafetyMonitor::new(SafetyThresholds::default());
        assert!(monitor.check_temperature(86.0).is_err());
        assert!(monitor.check_temperature(85.0).is_ok());
    }

    #[test]
    fn relay_cycle_limit() {
        let mut monitor = SafetyMonitor::new(SafetyThresholds::default());
        for _ in 0..30 {
            assert!(monitor.record_relay_cycle().is_ok());
        }
        assert!(monitor.record_relay_cycle().is_err());
    }

    #[test]
    fn failsafe_engage_disengage() {
        let mut monitor = SafetyMonitor::new(SafetyThresholds::default());
        assert!(!monitor.is_failsafe());
        monitor.engage_failsafe();
        assert!(monitor.is_failsafe());
        monitor.disengage_failsafe();
        assert!(!monitor.is_failsafe());
    }

    #[test]
    fn relay_cycles_reset() {
        let mut monitor = SafetyMonitor::new(SafetyThresholds::default());
        for _ in 0..30 {
            let _ = monitor.record_relay_cycle();
        }
        assert!(monitor.record_relay_cycle().is_err());
        monitor.reset_relay_cycles();
        assert!(monitor.record_relay_cycle().is_ok());
    }
}
