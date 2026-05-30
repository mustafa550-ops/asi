#[cfg(feature = "voice")]
mod real {
    use std::sync::Mutex;
    use std::sync::OnceLock;
    use vosk::{DecodingState, Recognizer, CompleteResult};

    static VOSK_RECOGNIZER: OnceLock<Mutex<Recognizer>> = OnceLock::new();

    pub fn init_stt(model_path: &str) -> Result<(), String> {
        if VOSK_RECOGNIZER.get().is_some() {
            return Ok(());
        }

        let model = vosk::Model::new(model_path)
            .ok_or_else(|| format!("Vosk model yuklenemedi: {}", model_path))?;
        let recognizer = Recognizer::new(&model, 16000.0)
            .ok_or_else(|| "Vosk recognizer olusturulamadi".to_string())?;

        VOSK_RECOGNIZER.set(Mutex::new(recognizer))
            .map_err(|_| "Vosk zaten baslatilmis".to_string())?;

        log::info!("Vosk STT baslati — model: {}", model_path);
        Ok(())
    }

    pub fn transcribe(samples: &[i16]) -> Option<String> {
        let lock = VOSK_RECOGNIZER.get()?;
        let mut recognizer = lock.lock().ok()?;

        match recognizer.accept_waveform(samples) {
            Ok(DecodingState::Finalized) => {
                let result = recognizer.result();
                let text = match result {
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

#[cfg(not(feature = "voice"))]
mod dummy {
    pub fn init_stt(_model_path: &str) -> Result<(), String> {
        log::info!("Vosk STT devre disi — derle: --features voice");
        Ok(())
    }

    pub fn transcribe(_samples: &[i16]) -> Option<String> {
        None
    }
}

#[cfg(feature = "voice")]
pub use real::*;

#[cfg(not(feature = "voice"))]
pub use dummy::*;
