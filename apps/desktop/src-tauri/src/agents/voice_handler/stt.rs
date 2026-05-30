pub trait SttEngine: Send + Sync {
    fn transcribe(&self, samples: &[i16]) -> Option<String>;
}

#[cfg(feature = "voice")]
mod vosk {
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use vosk::{DecodingState, Recognizer, CompleteResult};
    use super::SttEngine;

    static VOSK_RECOGNIZER: OnceLock<Mutex<Recognizer>> = OnceLock::new();

    pub struct VoskEngine;

    impl VoskEngine {
        pub fn new(model_path: &str) -> Result<Self, String> {
            if VOSK_RECOGNIZER.get().is_some() {
                return Ok(VoskEngine);
            }
            let model = vosk::Model::new(model_path)
                .ok_or_else(|| format!("Vosk model yuklenemedi: {}", model_path))?;
            let recognizer = Recognizer::new(&model, 16000.0)
                .ok_or_else(|| "Vosk recognizer olusturulamadi".to_string())?;
            VOSK_RECOGNIZER.set(Mutex::new(recognizer))
                .map_err(|_| "Vosk zaten baslatilmis".to_string())?;
            log::info!("Vosk STT basladi — model: {}", model_path);
            Ok(VoskEngine)
        }
    }

    impl SttEngine for VoskEngine {
        fn transcribe(&self, samples: &[i16]) -> Option<String> {
            let lock = VOSK_RECOGNIZER.get()?;
            let mut recognizer = lock.lock().ok()?;
            match recognizer.accept_waveform(samples) {
                Ok(DecodingState::Finalized) => {
                    let result = recognizer.result();
                    let text: String = match result {
                        CompleteResult::Single(s) => s.text.to_string(),
                        CompleteResult::Multiple(m) => {
                            m.alternatives.first().map(|a| a.text.to_string()).unwrap_or_default()
                        }
                    };
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
                Ok(DecodingState::Running) => {
                    let partial = recognizer.partial_result();
                    let text = partial.partial.to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
                _ => {}
            }
            None
        }
    }
}

mod whisper {
    use std::process::Command;
    use super::SttEngine;

    pub struct WhisperEngine {
        model_path: String,
        available: bool,
    }

    impl WhisperEngine {
        pub fn new(model_path: &str) -> Self {
            let available = Command::new("whisper")
                .arg("--help")
                .output()
                .is_ok();
            if available {
                log::info!("Whisper CLI bulundu — model: {}", model_path);
            } else {
                log::warn!("Whisper CLI bulunamadi");
            }
            WhisperEngine {
                model_path: model_path.to_string(),
                available,
            }
        }

        pub fn is_available(&self) -> bool {
            self.available
        }
    }

    impl SttEngine for WhisperEngine {
        fn transcribe(&self, samples: &[i16]) -> Option<String> {
            if !self.available {
                return None;
            }

            let temp_path = format!("/tmp/adler_whisper_{}.wav", std::process::id());
            if let Err(e) = write_wav(&temp_path, samples) {
                log::error!("Whisper WAV yazma hatasi: {}", e);
                return None;
            }

            let output = Command::new("whisper")
                .arg(&temp_path)
                .arg("--model")
                .arg(&self.model_path)
                .arg("--output-format")
                .arg("txt")
                .arg("--language")
                .arg("auto")
                .output()
                .ok()?;

            let _ = std::fs::remove_file(&temp_path);

            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !text.is_empty() {
                    return Some(text);
                }
            }

            let txt_path = temp_path.replace(".wav", ".txt");
            let text = std::fs::read_to_string(&txt_path).ok();
            let _ = std::fs::remove_file(&txt_path);
            text.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
        }
    }

    fn write_wav(path: &str, samples: &[i16]) -> Result<(), String> {
        use hound::{WavSpec, WavWriter};
        let spec = WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = WavWriter::create(path, spec)
            .map_err(|e| format!("WAV yazma: {}", e))?;
        for &s in samples {
            writer.write_sample(s).map_err(|e| format!("Sample yazma: {}", e))?;
        }
        writer.finalize().map_err(|e| format!("WAV finalize: {}", e))
    }
}

pub struct SttFallback {
    engines: Vec<Box<dyn SttEngine>>,
}

impl SttFallback {
    pub fn new(engines: Vec<Box<dyn SttEngine>>) -> Self {
        SttFallback { engines }
    }

    pub fn transcribe(&self, samples: &[i16]) -> Option<String> {
        for engine in &self.engines {
            if let Some(text) = engine.transcribe(samples) {
                return Some(text);
            }
        }
        None
    }
}

pub fn create_default_stt(vosk_model_path: &str) -> SttFallback {
    let mut engines: Vec<Box<dyn SttEngine>> = Vec::new();

    #[cfg(feature = "voice")]
    if !vosk_model_path.is_empty() {
        match vosk::VoskEngine::new(vosk_model_path) {
            Ok(engine) => engines.push(Box::new(engine)),
            Err(e) => log::warn!("Vosk baslatilamadi: {}", e),
        }
    }

    #[cfg(not(feature = "voice"))]
    let _ = vosk_model_path;

    let whisper_engine = whisper::WhisperEngine::new("base");
    if whisper_engine.is_available() {
        engines.push(Box::new(whisper_engine));
    }

    SttFallback { engines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_with_no_engines_returns_none() {
        let fb = SttFallback::new(vec![]);
        assert!(fb.transcribe(&[0i16; 160]).is_none());
    }

    #[test]
    fn whisper_detects_cli_availability() {
        let engine = whisper::WhisperEngine::new("base");
        let _ = engine.is_available();
    }

    #[test]
    fn create_default_does_not_panic() {
        let fb = create_default_stt("");
        assert!(fb.transcribe(&[0i16; 160]).is_none());
    }
}
