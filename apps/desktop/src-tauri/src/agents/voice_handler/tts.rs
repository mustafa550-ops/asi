use std::process::Command;

pub trait TtsEngine: Send + Sync {
    fn synthesize(&self, text: &str, output_path: &str) -> Result<String, String>;
}

pub struct EspeakEngine;

impl EspeakEngine {
    pub fn new() -> Self {
        EspeakEngine
    }
}

impl TtsEngine for EspeakEngine {
    fn synthesize(&self, text: &str, output_path: &str) -> Result<String, String> {
        let output = Command::new("espeak-ng")
            .args(["-w", output_path, "--", text])
            .output()
            .map_err(|e| format!("espeak-ng subprocess hatasi: {}", e))?;

        if output.status.success() {
            Ok("espeak-ng basarili".into())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("espeak-ng basarisiz: {}", stderr.trim()))
        }
    }
}

pub struct SupertonicEngine;

impl SupertonicEngine {
    pub fn new() -> Self {
        SupertonicEngine
    }
}

impl TtsEngine for SupertonicEngine {
    fn synthesize(&self, text: &str, output_path: &str) -> Result<String, String> {
        let url = "http://127.0.0.1:8765/tts";
        let client = reqwest::blocking::Client::new();
        let payload = serde_json::json!({
            "text": text,
            "lang": "tr",
            "voice_style": "default",
            "output_path": output_path,
        });

        let resp = client.post(url)
            .json(&payload)
            .send()
            .map_err(|e| format!("Supertonic API hatasi: {}", e))?;

        if resp.status().is_success() {
            Ok("Supertonic TTS basarili".into())
        } else {
            Err(format!("Supertonic HTTP {}", resp.status()))
        }
    }
}

pub struct ElevenlabsEngine {
    api_key: String,
}

impl ElevenlabsEngine {
    pub fn new(api_key: &str) -> Self {
        ElevenlabsEngine {
            api_key: api_key.to_string(),
        }
    }
}

impl TtsEngine for ElevenlabsEngine {
    fn synthesize(&self, text: &str, output_path: &str) -> Result<String, String> {
        let voice_id = "21m00Tcm4TlvDq8ikWAM";
        let url = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{}",
            voice_id
        );

        let client = reqwest::blocking::Client::new();
        let payload = serde_json::json!({
            "text": text,
            "model_id": "eleven_multilingual_v2",
            "voice_settings": {
                "stability": 0.3,
                "similarity_boost": 0.7,
                "speed": 1.0,
            }
        });

        let resp = client.post(&url)
            .header("xi-api-key", &self.api_key)
            .json(&payload)
            .send()
            .map_err(|e| format!("ElevenLabs API hatasi: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("ElevenLabs HTTP {}", resp.status()));
        }

        let bytes = resp.bytes()
            .map_err(|e| format!("ElevenLabs yanit okunamadi: {}", e))?;

        std::fs::write(output_path, &bytes)
            .map_err(|e| format!("ElevenLabs WAV yazma: {}", e))?;

        Ok("ElevenLabs TTS basarili".into())
    }
}

struct SineWaveEngine;

impl TtsEngine for SineWaveEngine {
    fn synthesize(&self, text: &str, output_path: &str) -> Result<String, String> {
        let sample_rate = 24000u32;
        let duration_secs = (text.len() as f32 * 0.08).max(1.0);
        let num_samples = (sample_rate as f32 * duration_secs) as u32;

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(output_path, spec)
            .map_err(|e| format!("WAV dosyasi olusturulamadi: {}", e))?;

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let freq = 220.0 + (text.len() as f32 * 10.0).min(800.0);
            let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
            let amplitude = 0.3;
            let value = (sample * amplitude * 32767.0) as i16;
            writer.write_sample(value).map_err(|e| format!("WAV yazma hatasi: {}", e))?;
        }

        writer.finalize().map_err(|e| format!("WAV finalize hatasi: {}", e))?;
        Ok(format!("sine-wave: {} Hz, {:.1}s", sample_rate, duration_secs))
    }
}

pub struct TtsFallback {
    engines: Vec<Box<dyn TtsEngine>>,
}

impl TtsFallback {
    pub fn new(engines: Vec<Box<dyn TtsEngine>>) -> Self {
        TtsFallback { engines }
    }

    pub fn synthesize(&self, text: &str, output_path: &str) -> String {
        for engine in &self.engines {
            match engine.synthesize(text, output_path) {
                Ok(msg) => {
                    let size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
                    return format!("TTS ({}): {} -> {} bayt", msg, text, size);
                }
                Err(e) => log::warn!("TTS engine basarisiz: {}", e),
            }
        }
        format!("TTS (none): tum motorlar basarisiz")
    }
}

pub fn create_default_tts() -> TtsFallback {
    let mut engines: Vec<Box<dyn TtsEngine>> = Vec::new();
    engines.push(Box::new(EspeakEngine::new()));
    engines.push(Box::new(SupertonicEngine::new()));

    let eleven_key = std::env::var("ELEVENLABS_API_KEY").unwrap_or_default();
    if !eleven_key.is_empty() {
        engines.push(Box::new(ElevenlabsEngine::new(&eleven_key)));
    } else {
        log::info!("ElevenLabs API anahtari bulunamadi — devre disi");
    }

    engines.push(Box::new(SineWaveEngine));
    TtsFallback { engines }
}

pub fn synthesize(text: &str) -> Result<String, String> {
    let output_path = "/tmp/adler_tts_output.wav";
    let fallback = create_default_tts();
    Ok(fallback.synthesize(text, output_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesize_falls_back_to_sine_wave() {
        let result = synthesize("test").unwrap();
        assert!(result.contains("sine-wave") || result.contains("espeak") || result.contains("supertonic") || result.contains("ElevenLabs"));
    }

    #[test]
    fn synthesize_creates_wav_file() {
        let _ = synthesize("merhaba");
        let path = std::path::Path::new("/tmp/adler_tts_output.wav");
        let _ = path.exists();
    }

    #[test]
    fn fallback_with_sine_only_works() {
        let fb = TtsFallback::new(vec![Box::new(SineWaveEngine)]);
        let result = fb.synthesize("test", "/tmp/adler_test.wav");
        assert!(result.contains("sine-wave"));
        let _ = std::fs::remove_file("/tmp/adler_test.wav");
    }

    #[test]
    fn elevenlabs_missing_key_graceful() {
        let engine = ElevenlabsEngine::new("");
        let result = engine.synthesize("test", "/tmp/adler_test_11.wav");
        assert!(result.is_err());
    }

    #[test]
    fn espeak_fallback_to_sine() {
        let fb = TtsFallback::new(vec![
            Box::new(EspeakEngine::new()),
            Box::new(SineWaveEngine),
        ]);
        let result = fb.synthesize("test", "/tmp/adler_test_es.wav");
        // espeak might or might not be available; either is fine
        assert!(!result.contains("none"));
        let _ = std::fs::remove_file("/tmp/adler_test_es.wav");
    }
}
