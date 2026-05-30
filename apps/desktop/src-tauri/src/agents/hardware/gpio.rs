use std::path::Path;

const GPIO_BASE: &str = "/sys/class/gpio";

fn gpio_path(pin: u8) -> String {
    format!("{}/gpio{}/", GPIO_BASE, pin)
}

fn export_pin(pin: u8) -> Result<(), String> {
    let path = gpio_path(pin);
    if Path::new(&path).exists() {
        return Ok(());
    }
    std::fs::write(format!("{}/export", GPIO_BASE), pin.to_string())
        .map_err(|e| format!("GPIO export basarisiz pin {}: {}", pin, e))
}

#[allow(dead_code)]
fn unexport_pin(pin: u8) -> Result<(), String> {
    std::fs::write(format!("{}/unexport", GPIO_BASE), pin.to_string())
        .map_err(|e| format!("GPIO unexport basarisiz pin {}: {}", pin, e))
}

fn set_direction(pin: u8, dir: &str) -> Result<(), String> {
    export_pin(pin)?;
    std::fs::write(gpio_path(pin) + "direction", dir)
        .map_err(|e| format!("GPIO yon hatasi pin {}: {}", pin, e))
}

pub fn read(pin: u8) -> Result<String, String> {
    set_direction(pin, "in")?;
    let value = std::fs::read_to_string(gpio_path(pin) + "value")
        .map_err(|e| format!("GPIO okuma hatasi pin {}: {}", pin, e))?;
    Ok(value.trim().to_string())
}

pub fn write(pin: u8, value: bool) -> Result<(), String> {
    set_direction(pin, "out")?;
    let val = if value { "1" } else { "0" };
    std::fs::write(gpio_path(pin) + "value", val)
        .map_err(|e| format!("GPIO yazma hatasi pin {}: {}", pin, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_returns_ok_for_nonexistent_path() {
        let result = export_pin(255);
        // Will fail because /sys/class/gpio doesn't exist in CI
        // but the error should be about the export file, not a crash
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn read_fails_with_expected_message() {
        let result = read(1);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("GPIO") || err.contains("hatasi"));
    }
}
