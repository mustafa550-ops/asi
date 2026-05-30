use std::collections::HashMap;
use std::sync::Mutex;

pub trait GpioBackend: Send {
    fn read(&self, pin: u8) -> Result<String, String>;
    fn write(&self, pin: u8, value: bool) -> Result<(), String>;
}

pub trait SensorBackend: Send {
    fn read_all(&self) -> Result<String, String>;
}

pub trait RelayBackend: Send {
    fn set(&self, pin: u8, state: bool) -> Result<(), String>;
}

pub struct RealGpio;

impl GpioBackend for RealGpio {
    fn read(&self, pin: u8) -> Result<String, String> {
        set_direction(pin, "in")?;
        let value = std::fs::read_to_string(gpio_path(pin) + "value")
            .map_err(|e| format!("GPIO okuma hatasi pin {}: {}", pin, e))?;
        Ok(value.trim().to_string())
    }

    fn write(&self, pin: u8, value: bool) -> Result<(), String> {
        set_direction(pin, "out")?;
        let val = if value { "1" } else { "0" };
        std::fs::write(gpio_path(pin) + "value", val)
            .map_err(|e| format!("GPIO yazma hatasi pin {}: {}", pin, e))
    }
}

pub struct RealSensor;

impl SensorBackend for RealSensor {
    fn read_all(&self) -> Result<String, String> {
        let mut readings = Vec::new();

        match read_ds18b20() {
            Ok(temp) => readings.push(format!("- Sicaklik: {:.1} C", temp)),
            Err(_) => {}
        }
        match read_cpu_temp() {
            Ok(temp) => readings.push(format!("- CPU Sicaklik: {:.1} C", temp)),
            Err(_) => {}
        }

        if readings.is_empty() {
            Ok("Sensor bulunamadi. DS18B20 bagli mi kontrol edin.".into())
        } else {
            Ok(readings.join("\n"))
        }
    }
}

pub struct RealRelay;

impl RelayBackend for RealRelay {
    fn set(&self, pin: u8, state: bool) -> Result<(), String> {
        RealGpio.write(pin, state)
    }
}

#[derive(Clone)]
pub struct SimGpio {
    pins: std::sync::Arc<Mutex<HashMap<u8, bool>>>,
}

impl SimGpio {
    pub fn new() -> Self {
        SimGpio { pins: std::sync::Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn set_pin(&self, pin: u8, value: bool) {
        let mut map = self.pins.lock().unwrap();
        map.insert(pin, value);
    }

    pub fn get_pin(&self, pin: u8) -> Option<bool> {
        let map = self.pins.lock().unwrap();
        map.get(&pin).copied()
    }

    pub fn reset(&self) {
        let mut map = self.pins.lock().unwrap();
        map.clear();
    }
}

impl GpioBackend for SimGpio {
    fn read(&self, pin: u8) -> Result<String, String> {
        let map = self.pins.lock().map_err(|e| e.to_string())?;
        let value = map.get(&pin).copied().unwrap_or(false);
        Ok(if value { "1".to_string() } else { "0".to_string() })
    }

    fn write(&self, pin: u8, value: bool) -> Result<(), String> {
        let mut map = self.pins.lock().map_err(|e| e.to_string())?;
        map.insert(pin, value);
        Ok(())
    }
}

pub struct SimRelay {
    gpio: std::sync::Arc<SimGpio>,
}

impl SimRelay {
    pub fn new(gpio: std::sync::Arc<SimGpio>) -> Self {
        SimRelay { gpio }
    }
}

impl RelayBackend for SimRelay {
    fn set(&self, pin: u8, state: bool) -> Result<(), String> {
        self.gpio.write(pin, state)
    }
}

pub struct SimSensor {
    temperature: Mutex<f64>,
    cpu_temperature: Mutex<f64>,
    fail: Mutex<bool>,
}

impl SimSensor {
    pub fn new() -> Self {
        SimSensor {
            temperature: Mutex::new(22.5),
            cpu_temperature: Mutex::new(45.0),
            fail: Mutex::new(false),
        }
    }

    pub fn set_temperature(&self, temp: f64) {
        *self.temperature.lock().unwrap() = temp;
    }

    pub fn set_cpu_temperature(&self, temp: f64) {
        *self.cpu_temperature.lock().unwrap() = temp;
    }

    pub fn set_fail(&self, should_fail: bool) {
        *self.fail.lock().unwrap() = should_fail;
    }
}

impl SensorBackend for SimSensor {
    fn read_all(&self) -> Result<String, String> {
        if *self.fail.lock().unwrap() {
            return Err("Simulator sensor hatasi".into());
        }
        let temp = *self.temperature.lock().unwrap();
        let cpu = *self.cpu_temperature.lock().unwrap();
        Ok(format!("- Sicaklik: {:.1} C\n- CPU Sicaklik: {:.1} C", temp, cpu))
    }
}

const GPIO_BASE: &str = "/sys/class/gpio";

fn gpio_path(pin: u8) -> String {
    format!("{}/gpio{}/", GPIO_BASE, pin)
}

fn export_pin(pin: u8) -> Result<(), String> {
    let path = gpio_path(pin);
    if std::path::Path::new(&path).exists() {
        return Ok(());
    }
    std::fs::write(format!("{}/export", GPIO_BASE), pin.to_string())
        .map_err(|e| format!("GPIO export basarisiz pin {}: {}", pin, e))
}

fn set_direction(pin: u8, dir: &str) -> Result<(), String> {
    export_pin(pin)?;
    std::fs::write(gpio_path(pin) + "direction", dir)
        .map_err(|e| format!("GPIO yon hatasi pin {}: {}", pin, e))
}

fn read_ds18b20() -> Result<f64, String> {
    let dir = std::path::Path::new("/sys/bus/w1/devices");
    if !dir.exists() {
        return Err("DS18B20 bulunamadi".into());
    }
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("28-") {
            let slave_path = entry.path().join("w1_slave");
            let content = std::fs::read_to_string(&slave_path)
                .map_err(|e| format!("DS18B20 okuma hatasi: {}", e))?;
            for line in content.lines() {
                if let Some(pos) = line.find("t=") {
                    let temp_str = &line[pos + 2..].trim();
                    if let Ok(temp_milli) = temp_str.parse::<i64>() {
                        return Ok(temp_milli as f64 / 1000.0);
                    }
                }
            }
        }
    }
    Err("DS18B20 cihazi bulunamadi".into())
}

fn read_cpu_temp() -> Result<f64, String> {
    let path = std::path::Path::new("/sys/class/thermal/thermal_zone0/temp");
    if !path.exists() {
        return Err("CPU sensoru bulunamadi".into());
    }
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("CPU sicaklik okuma hatasi: {}", e))?;
    let millicelsius: f64 = raw.trim().parse()
        .map_err(|_| "CPU sicaklik parse hatasi".to_string())?;
    Ok(millicelsius / 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_gpio_read_write_roundtrip() {
        let gpio = SimGpio::new();
        gpio.write(17, true).unwrap();
        assert_eq!(gpio.read(17).unwrap(), "1");
        gpio.write(17, false).unwrap();
        assert_eq!(gpio.read(17).unwrap(), "0");
    }

    #[test]
    fn sim_gpio_default_is_low() {
        let gpio = SimGpio::new();
        assert_eq!(gpio.read(99).unwrap(), "0");
    }

    #[test]
    fn sim_gpio_reset_clears_state() {
        let gpio = SimGpio::new();
        gpio.write(17, true).unwrap();
        gpio.reset();
        assert_eq!(gpio.read(17).unwrap(), "0");
    }

    #[test]
    fn sim_sensor_returns_configured_values() {
        let sensor = SimSensor::new();
        sensor.set_temperature(30.0);
        sensor.set_cpu_temperature(50.0);
        let result = sensor.read_all().unwrap();
        assert!(result.contains("30.0"));
        assert!(result.contains("50.0"));
    }

    #[test]
    fn sim_sensor_fail_mode() {
        let sensor = SimSensor::new();
        sensor.set_fail(true);
        assert!(sensor.read_all().is_err());
    }

    #[test]
    fn sim_gpio_set_pin_direct() {
        let gpio = SimGpio::new();
        gpio.set_pin(27, true);
        assert_eq!(gpio.get_pin(27), Some(true));
        assert_eq!(gpio.get_pin(99), None);
    }
}
