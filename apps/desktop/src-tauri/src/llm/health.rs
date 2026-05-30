use crate::llm::OllamaClient;

pub struct LLMHealthCheck {
    ollama: OllamaClient,
    timeout_secs: u64,
}

pub struct HealthStatus {
    pub ollama_available: bool,
    pub ollama_latency_ms: u64,
    pub default_model_loaded: bool,
    pub details: String,
}

impl LLMHealthCheck {
    pub fn new(ollama: OllamaClient) -> Self {
        Self { ollama, timeout_secs: 5 }
    }

    pub fn check(&self) -> HealthStatus {
        let start = std::time::Instant::now();

        let (ollama_available, model_loaded) = match self.check_ollama() {
            Ok(true) => (true, true),
            Ok(false) => (true, false),
            Err(_) => (false, false),
        };

        let latency = start.elapsed().as_millis() as u64;

        let details = if ollama_available {
            if model_loaded {
                format!("Ollama {} surucusu calisiyor, model '{}' yuklu ({}ms)", self.ollama.base_url, self.ollama.model, latency)
            } else {
                format!("Ollama calisiyor ancak model '{}' yuklu degil", self.ollama.model)
            }
        } else {
            format!("Ollama'ya erisilemiyor: {}", self.ollama.base_url)
        };

        HealthStatus { ollama_available, ollama_latency_ms: latency, default_model_loaded: model_loaded, details }
    }

    fn check_ollama(&self) -> Result<bool, String> {
        let url = format!("{}/api/tags", self.ollama.base_url);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .build().map_err(|e| e.to_string())?;
        let res = client.get(&url).send().map_err(|e| e.to_string())?;
        if !res.status().is_success() {
            return Ok(false);
        }
        let body: serde_json::Value = res.json().map_err(|e| e.to_string())?;
        let models = body["models"].as_array();
        let model_loaded = models.map(|m| {
            m.iter().any(|v| v["name"].as_str().map_or(false, |n| n.contains(&self.ollama.model)))
        }).unwrap_or(false);
        Ok(model_loaded)
    }

    pub fn auto_start_ollama(&self) -> Result<bool, String> {
        if self.check_ollama().is_ok() {
            return Ok(true);
        }
        log::info!("Ollama baslatiliyor...");
        let output = std::process::Command::new("ollama")
            .arg("serve")
            .spawn();
        match output {
            Ok(_) => {
                std::thread::sleep(std::time::Duration::from_secs(3));
                Ok(true)
            }
            Err(e) => Err(format!("Ollama baslatilamadi: {}", e)),
        }
    }
}
