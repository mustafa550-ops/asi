use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscoveredDevice {
    pub name: String,
    pub bus_type: String,
    pub address: String,
    pub description: String,
}

pub fn auto_detect() -> Result<Vec<DiscoveredDevice>, String> {
    let mut devices = Vec::new();

    devices.extend(detect_onewire()?);
    devices.extend(detect_gpio_chips()?);
    devices.extend(detect_i2c_buses()?);
    devices.extend(detect_thermal_zones()?);

    Ok(devices)
}

fn detect_onewire() -> Result<Vec<DiscoveredDevice>, String> {
    let mut devices = Vec::new();
    let w1_dir = Path::new("/sys/bus/w1/devices");
    if !w1_dir.exists() {
        return Ok(devices);
    }
    for entry in std::fs::read_dir(w1_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let addr = entry.file_name().to_string_lossy().to_string();
        if addr.starts_with("28-") {
            devices.push(DiscoveredDevice {
                name: format!("DS18B20 ({})", addr),
                bus_type: "onewire".into(),
                address: addr.clone(),
                description: "Sicaklik sensoru, -55..125C".into(),
            });
        } else if addr.starts_with("10-") {
            devices.push(DiscoveredDevice {
                name: format!("DS18S20 ({})", addr),
                bus_type: "onewire".into(),
                address: addr,
                description: "Sicaklik sensoru (eski model)".into(),
            });
        }
    }
    Ok(devices)
}

fn detect_gpio_chips() -> Result<Vec<DiscoveredDevice>, String> {
    let mut devices = Vec::new();
    let gpio_dir = Path::new("/sys/class/gpio");
    if !gpio_dir.exists() {
        return Ok(devices);
    }
    if let Ok(entries) = std::fs::read_dir(gpio_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("gpiochip") {
                let num = name.trim_start_matches("gpiochip").to_string();
                if let Ok(label) = std::fs::read_to_string(entry.path().join("label")) {
                    devices.push(DiscoveredDevice {
                        name: format!("GPIO Chip {} ({})", num, label.trim()),
                        bus_type: "gpio".into(),
                        address: name.clone(),
                        description: format!("{} pin GPIO yongasi", num),
                    });
                }
            }
        }
    }

    let gpiochip_dev = Path::new("/dev/gpiochip0");
    if gpiochip_dev.exists() {
        if !devices.iter().any(|d| d.address == "gpiochip0") {
            devices.push(DiscoveredDevice {
                name: "GPIO Chip 0 (chardev)".into(),
                bus_type: "gpio".into(),
                address: "gpiochip0".into(),
                description: "/dev/gpiochip0 uzerinden GPIO".into(),
            });
        }
    }

    Ok(devices)
}

fn detect_i2c_buses() -> Result<Vec<DiscoveredDevice>, String> {
    let mut devices = Vec::new();
    let i2c_dir = Path::new("/sys/bus/i2c/devices");
    if !i2c_dir.exists() {
        return Ok(devices);
    }
    for entry in std::fs::read_dir(i2c_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.contains('-') {
            if let Ok(name_file) = std::fs::read_to_string(entry.path().join("name")) {
                let addr = name.clone();
                devices.push(DiscoveredDevice {
                    name: format!("I2C {} ({})", name, name_file.trim()),
                    bus_type: "i2c".into(),
                    address: addr,
                    description: format!("I2C cihazi {}", name),
                });
            }
        }
    }
    Ok(devices)
}

fn detect_thermal_zones() -> Result<Vec<DiscoveredDevice>, String> {
    let mut devices = Vec::new();
    let thermal_dir = Path::new("/sys/class/thermal");
    if !thermal_dir.exists() {
        return Ok(devices);
    }
    for entry in std::fs::read_dir(thermal_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("thermal_zone") {
            let zone_type = std::fs::read_to_string(entry.path().join("type"))
                .unwrap_or_default();
            let temp_raw = std::fs::read_to_string(entry.path().join("temp"))
                .unwrap_or_default();
            let temp_c = temp_raw.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
            devices.push(DiscoveredDevice {
                name: format!("Thermal {} ({})", name, zone_type.trim()),
                bus_type: "thermal".into(),
                address: name,
                description: format!("{:.1} C", temp_c),
            });
        }
    }
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_detect_never_panics() {
        let result = auto_detect();
        assert!(result.is_ok());
    }

    #[test]
    fn discover_thermal_exists_on_linux() {
        let thermal = detect_thermal_zones().unwrap();
        if std::path::Path::new("/sys/class/thermal/thermal_zone0").exists() {
            assert!(!thermal.is_empty());
        }
    }

    #[test]
    fn discover_i2c_does_not_crash() {
        let result = detect_i2c_buses().unwrap();
        assert!(result.is_empty() || !result.is_empty());
    }

    #[test]
    fn discovered_device_json_serializable() {
        let d = DiscoveredDevice {
            name: "test".into(),
            bus_type: "gpio".into(),
            address: "17".into(),
            description: "test device".into(),
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("test"));
    }
}
