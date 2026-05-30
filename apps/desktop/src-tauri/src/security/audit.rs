use crate::config::AppConfig;

pub struct SecurityAuditor;

impl SecurityAuditor {
    pub fn new() -> Self {
        Self
    }

    pub fn audit_config(config: &AppConfig) -> Vec<String> {
        let mut findings = Vec::new();

        if config.ollama_url.contains("://localhost") || config.ollama_url.contains("://127.0.0.1") {
            findings.push("OK: Ollama local adres kullaniyor".into());
        } else {
            findings.push("UYARI: Ollama remote adres — TLS kullanildigindan emin olun".into());
        }

        if config.ollama_model.is_empty() {
            findings.push("UYARI: Ollama model adi bos".into());
        }

        if config.approval_level == "full" {
            findings.push("UYARI: Tam yetki modu — tum aksiyonlar otonom".into());
        }

        if config.db_path.contains("/tmp/") {
            findings.push("UYARI: Veritabani /tmp/ altinda — guvenli degil".into());
        }

        if config.vosk_model_path.is_empty() {
            findings.push("BILGI: Vosk model yolu ayarlanmamis — STT pasif".into());
        }

        findings
    }

    pub fn check_env_secrets() -> Vec<String> {
        let mut findings = Vec::new();
        for (key, val) in std::env::vars() {
            let upper = key.to_uppercase();
            if (upper.contains("API") || upper.contains("TOKEN") || upper.contains("SECRET")
                || upper.contains("KEY") || upper.contains("PASSWORD"))
                && upper != "RUST_LOG"
            {
                let masked = if val.len() > 8 {
                    format!("{}...{}", &val[..4], &val[val.len()-4..])
                } else {
                    "***".into()
                };
                findings.push(format!("ORTAM: {}={}", key, masked));
            }
        }
        findings
    }
}
