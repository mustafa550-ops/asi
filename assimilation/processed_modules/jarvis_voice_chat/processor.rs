use sysinfo::System;
use serde_json::json;

pub struct VoiceProcessor;

impl VoiceProcessor {
    /// Otonom Rust versiyonu - MOCK KULLANIMI YASAKTIR
    pub async fn process_command(transcript: &str) -> Result<String, String> {
        let command = transcript.to_lowercase();
        
        if command.contains("sistem") || command.contains("system") || command.contains("ram") {
            let mut sys = System::new_all();
            sys.refresh_all();
            let total_ram = sys.total_memory() / 1024 / 1024;
            let used_ram = sys.used_memory() / 1024 / 1024;
            
            return Ok(format!("Sistem durumu gerçek verisi: Toplam RAM {} MB, Kullanılan RAM {} MB.", total_ram, used_ram));
        } else if command.contains("hava") || command.contains("weather") {
            // Gerçek API olmadığı için mock string dönmek yerine hata (Truth Enforcer uyumu)
            return Err("Hava durumu API entegrasyonu henüz bağlanmadı.".to_string());
        } else if command.contains("kamera") || command.contains("camera") || command.contains("gör") {
            // Gerçek kamera bağlantısı olmadığı için hata
            return Err("Kamera modülü henüz donanıma bağlanmadı.".to_string());
        } else {
            // Gerçek LLM API bağlantısı (Ollama)
            let client = reqwest::Client::new();
            let request_body = json!({
                "model": "llama3", // ADLER default modeli
                "prompt": format!("Sen ADLER ASI'sin. Şu komuta kısa ve öz Türkçe cevap ver: {}", transcript),
                "stream": false
            });

            match client.post("http://localhost:11434/api/generate").json(&request_body).send().await {
                Ok(response) => {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if let Some(reply) = json.get("response").and_then(|v| v.as_str()) {
                            return Ok(reply.to_string());
                        }
                    }
                    Err("Ollama API'den beklenen JSON formatında cevap alınamadı.".to_string())
                },
                Err(e) => {
                    Err(format!("Ollama API'ye bağlanılamadı. Servisin açık olduğundan emin olun. Hata: {}", e))
                }
            }
        }
    }
}
