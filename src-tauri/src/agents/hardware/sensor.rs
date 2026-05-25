use std::fs;
use std::path::Path;

const DS18B20_BASE: &str = "/sys/bus/w1/devices";

pub struct SensorReading {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

pub fn read_all() -> Result<String, String> {
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

fn read_ds18b20() -> Result<f64, String> {
    let dir = Path::new(DS18B20_BASE);
    if !dir.exists() {
        return Err("DS18B20 bulunamadi".into());
    }

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("28-") {
            let slave_path = entry.path().join("w1_slave");
            let content = fs::read_to_string(&slave_path)
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
    let path = Path::new("/sys/class/thermal/thermal_zone0/temp");
    if !path.exists() {
        return Err("CPU sensoru bulunamadi".into());
    }
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("CPU sicaklik okuma hatasi: {}", e))?;
    let millicelsius: f64 = raw.trim().parse()
        .map_err(|_| "CPU sicaklik parse hatasi".to_string())?;
    Ok(millicelsius / 1000.0)
}
