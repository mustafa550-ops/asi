use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareConfig {
    pub enabled: bool,
    pub gpio_pins: HashMap<String, u8>,
    pub sensor_types: Vec<String>,
    pub relay_pulse_ms: u64,
    pub safety_max_temp: f64,
    pub watchdog_timeout_s: u64,
}

impl Default for HardwareConfig {
    fn default() -> Self {
        let mut gpio_pins = HashMap::new();
        gpio_pins.insert("relay_1".into(), 17);
        gpio_pins.insert("relay_2".into(), 27);
        gpio_pins.insert("sensor_temp".into(), 22);
        Self {
            enabled: false,
            gpio_pins,
            sensor_types: vec!["ds18b20".into(), "cpu_temp".into()],
            relay_pulse_ms: 500,
            safety_max_temp: 85.0,
            watchdog_timeout_s: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub ollama_url: String,
    pub ollama_model: String,
    pub claude_api_key: Option<String>,
    pub claude_model: String,
    pub db_path: String,
    pub vosk_model_path: String,
    pub mcp_port: u16,
    pub approval_level: String,
    pub hardware: HardwareConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            ollama_model: "qwen2.5:1.5b".to_string(),
            claude_api_key: None,
            claude_model: "claude-sonnet-4-20250514".to_string(),
            db_path: "adler.db".to_string(),
            vosk_model_path: String::new(),
            mcp_port: 9876,
            approval_level: "SemiAutonomous".to_string(),
            hardware: HardwareConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let mut path = PathBuf::from(home);
        path.push(".config/adler/config.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => {
                    log::info!("Config loaded from {:?}", path);
                    config
                }
                Err(e) => {
                    log::warn!("Config parse error ({}), using defaults", e);
                    Self::default()
                }
            },
            Err(_) => {
                let config = Self::default();
                if let Err(e) = config.save() {
                    log::warn!("Could not save default config: {}", e);
                }
                config
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create config dir: {}", e))?;
        }
        let content = serde_json::to_string_pretty(self).map_err(|e| format!("Config serialize: {}", e))?;
        std::fs::write(&path, content).map_err(|e| format!("Config write: {}", e))?;
        log::info!("Config saved to {:?}", path);
        Ok(())
    }

    pub fn resolve_approval_level(&self) -> crate::agents::ApprovalLevel {
        match self.approval_level.to_lowercase().as_str() {
            "observer" => crate::agents::ApprovalLevel::Observer,
            "strategic" => crate::agents::ApprovalLevel::Strategic,
            _ => crate::agents::ApprovalLevel::SemiAutonomous,
        }
    }
}
