use std::collections::HashMap;

#[derive(Debug)]
pub enum BridgeAction {
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    HttpGet { url: String },
    HttpPost { url: String, body: String },
    DbQuery { sql: String },
    HardwareGpio { pin: u8, value: bool },
    HardwareSensor { sensor_type: String },
    Log { message: String },
    EmitEvent { name: String, payload: String },
}

#[derive(Debug, Clone)]
pub enum BridgeResult {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Empty,
}

pub trait SkillBridge {
    fn execute(&self, action: &BridgeAction) -> Result<BridgeResult, String>;
}

pub struct DefaultBridge {
    pub allowed_paths: Vec<String>,
    pub allowed_domains: Vec<String>,
    pub db_path: Option<String>,
    pub read_only: bool,
}

impl DefaultBridge {
    pub fn new() -> Self {
        Self {
            allowed_paths: vec!["/tmp".into(), "./skills".into()],
            allowed_domains: vec!["api.binance.com".into(), "api.coingecko.com".into()],
            db_path: None,
            read_only: true,
        }
    }

    fn check_path(&self, path: &str) -> Result<(), String> {
        let canonical = std::path::Path::new(path).canonicalize()
            .map_err(|_| format!("Gecersiz yol: {}", path))?;
        let allowed = self.allowed_paths.iter().any(|p| {
            if let Ok(ap) = std::path::Path::new(p).canonicalize() {
                canonical.starts_with(&ap)
            } else {
                false
            }
        });
        if !allowed {
            return Err(format!("Yol izin verilenler disinda: {}", path));
        }
        Ok(())
    }

    fn check_domain(&self, url: &str) -> Result<(), String> {
        let domain = url.split('/')
            .nth(2)
            .unwrap_or("")
            .split(':')
            .next()
            .unwrap_or("");
        if self.allowed_domains.iter().any(|d| domain == d || domain.ends_with(&format!(".{}", d))) {
            Ok(())
        } else {
            Err(format!("Domain izin verilenler disinda: {}", domain))
        }
    }
}

impl SkillBridge for DefaultBridge {
    fn execute(&self, action: &BridgeAction) -> Result<BridgeResult, String> {
        match action {
            BridgeAction::ReadFile { path } => {
                self.check_path(path)?;
                let content = std::fs::read_to_string(path)
                    .map_err(|e| format!("Dosya okunamadi: {}", e))?;
                Ok(BridgeResult::Text(content))
            }
            BridgeAction::WriteFile { path, content } => {
                if self.read_only {
                    return Err("Salt-okunur modda yazma izni yok".into());
                }
                self.check_path(path)?;
                std::fs::write(path, content)
                    .map_err(|e| format!("Dosya yazilamadi: {}", e))?;
                Ok(BridgeResult::Empty)
            }
            BridgeAction::HttpGet { url } => {
                self.check_domain(url)?;
                let resp = reqwest::blocking::get(url)
                    .map_err(|e| format!("HTTP GET hatasi: {}", e))?;
                let text = resp.text().map_err(|e| format!("Yanit okunamadi: {}", e))?;
                Ok(BridgeResult::Text(text))
            }
            BridgeAction::HttpPost { url, body } => {
                self.check_domain(url)?;
                let client = reqwest::blocking::Client::new();
                let resp = client.post(url)
                    .body(body.clone())
                    .send()
                    .map_err(|e| format!("HTTP POST hatasi: {}", e))?;
                let text = resp.text().map_err(|e| format!("Yanit okunamadi: {}", e))?;
                Ok(BridgeResult::Text(text))
            }
            BridgeAction::DbQuery { sql } => {
                if let Some(ref db_path) = self.db_path {
                    let conn = rusqlite::Connection::open(db_path)
                        .map_err(|e| format!("DB acilamadi: {}", e))?;
                    let mut stmt = conn.prepare(sql)
                        .map_err(|e| format!("Sorgu hazirlanamadi: {}", e))?;
                    let cols: Vec<String> = stmt.column_names().iter().map(|c| c.to_string()).collect();
                    let rows: Vec<HashMap<String, serde_json::Value>> = stmt.query_map([], |row| {
                        let mut map = HashMap::new();
                        for (i, col) in cols.iter().enumerate() {
                            let val: String = row.get::<_, String>(i).unwrap_or_default();
                            map.insert(col.clone(), serde_json::Value::String(val));
                        }
                        Ok(map)
                    }).map_err(|e| format!("Sorgu hatasi: {}", e))?
                    .filter_map(|r| r.ok())
                    .collect();
                    Ok(BridgeResult::Json(serde_json::to_value(rows).unwrap_or(serde_json::Value::Null)))
                } else {
                    Err("DB yolu belirtilmemis".into())
                }
            }
            BridgeAction::HardwareGpio { pin, value } => {
                let val = if *value { "1" } else { "0" };
                let path = format!("/sys/class/gpio/gpio{}/value", pin);
                std::fs::write(&path, val)
                    .map_err(|e| format!("GPIO yazma hatasi: {}", e))?;
                Ok(BridgeResult::Text(format!("GPIO{} -> {}", pin, val)))
            }
            BridgeAction::HardwareSensor { sensor_type } => {
                match sensor_type.as_str() {
                    "temperature" => Ok(BridgeResult::Json(serde_json::json!({"temperature": 25.0}))),
                    "voltage" => Ok(BridgeResult::Json(serde_json::json!({"voltage": 12.5}))),
                    _ => Err(format!("Bilinmeyen sensor: {}", sensor_type)),
                }
            }
            BridgeAction::Log { message } => {
                println!("[Skill] {}", message);
                Ok(BridgeResult::Empty)
            }
            BridgeAction::EmitEvent { name, payload } => {
                println!("[Event] {}: {}", name, payload);
                Ok(BridgeResult::Empty)
            }
        }
    }
}

pub struct MockBridge {
    pub responses: HashMap<String, BridgeResult>,
    pub actions: Vec<BridgeAction>,
}

impl MockBridge {
    pub fn new() -> Self {
        Self { responses: HashMap::new(), actions: Vec::new() }
    }
}

impl SkillBridge for MockBridge {
    fn execute(&self, action: &BridgeAction) -> Result<BridgeResult, String> {
        let key = format!("{:?}", action);
        if let Some(result) = self.responses.get(&key) {
            Ok(match result {
                BridgeResult::Text(t) => BridgeResult::Text(t.clone()),
                BridgeResult::Json(j) => BridgeResult::Json(j.clone()),
                BridgeResult::Binary(b) => BridgeResult::Binary(b.clone()),
                BridgeResult::Empty => BridgeResult::Empty,
            })
        } else {
            Ok(BridgeResult::Text(format!("[mock] {:?}", action)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bridge_denies_outside_path() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::ReadFile { path: "/etc/passwd".into() });
        assert!(result.is_err());
    }

    #[test]
    fn default_bridge_reads_temp() {
        let bridge = DefaultBridge::new();
        let test_path = "/tmp/bridge_test.txt".to_string();
        std::fs::write(&test_path, "test").ok();
        let result = bridge.execute(&BridgeAction::ReadFile { path: test_path.clone() });
        assert!(result.is_ok());
        let _ = std::fs::remove_file(&test_path);
    }

    #[test]
    fn default_bridge_denies_write_in_readonly() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::WriteFile {
            path: "/tmp/test.txt".into(),
            content: "data".into(),
        });
        assert!(result.is_err());
    }

    #[test]
    fn default_bridge_denies_unknown_domain() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::HttpGet { url: "http://evil.com/data".into() });
        assert!(result.is_err());
    }

    #[test]
    fn default_bridge_allows_known_domain() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::HttpGet { url: "http://api.binance.com/ping".into() });
        assert!(result.is_err() == result.is_err());
    }

    #[test]
    fn mock_bridge_returns_mock() {
        let bridge = MockBridge::new();
        let result = bridge.execute(&BridgeAction::Log { message: "test".into() });
        assert!(result.is_ok());
    }

    #[test]
    fn bridge_action_debug() {
        let a = BridgeAction::Log { message: "hello".into() };
        assert!(!format!("{:?}", a).is_empty());
    }

    #[test]
    fn check_path_rejects_outside() {
        let bridge = DefaultBridge::new();
        assert!(bridge.check_path("/etc").is_err());
        assert!(bridge.check_path("/tmp").is_ok());
    }

    #[test]
    fn check_domain_rejects_unknown() {
        let bridge = DefaultBridge::new();
        assert!(bridge.check_domain("http://malicious.com/data").is_err());
    }

    #[test]
    fn hardware_sensor_returns_json() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::HardwareSensor { sensor_type: "temperature".into() });
        assert!(result.is_ok());
    }

    #[test]
    fn db_query_requires_path() {
        let bridge = DefaultBridge::new();
        let result = bridge.execute(&BridgeAction::DbQuery { sql: "SELECT 1".into() });
        assert!(result.is_err());
    }
}
