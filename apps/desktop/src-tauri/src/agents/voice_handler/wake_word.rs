use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use super::stt::SttFallback;

static WAKE_TRIGGERED: AtomicBool = AtomicBool::new(false);
static STT_FALLBACK: OnceLock<SttFallback> = OnceLock::new();

const WAKE_WORDS: &[&str] = &["adler", "hey adler", "adler asistan", "adler asistana"];

pub fn init_wake_word(stt: SttFallback) -> Result<(), String> {
    if STT_FALLBACK.get().is_some() {
        log::info!("Wake word zaten baslatilmis — guncellenmiyor");
        return Ok(());
    }
    STT_FALLBACK.set(stt)
        .map_err(|_| "Wake word STT baslatilamadi".to_string())?;
    log::info!("Wake word basladi — kelimeler: {:?}", WAKE_WORDS);
    Ok(())
}

pub fn check_wake_word(text: &str) -> bool {
    let lower = text.to_lowercase();
    for &word in WAKE_WORDS {
        if lower.contains(word) {
            log::info!("Wake word detected: '{}' in '{}'", word, text);
            WAKE_TRIGGERED.store(true, Ordering::SeqCst);
            return true;
        }
    }
    false
}

pub fn feed_audio(samples: &[i16]) {
    let energy: f32 = samples.iter()
        .map(|&s| (s as f32 / 32767.0).abs())
        .sum::<f32>() / samples.len() as f32;

    if energy > 0.05 {
        if let Some(stt) = STT_FALLBACK.get() {
            if let Some(text) = stt.transcribe(samples) {
                check_wake_word(&text);
            }
        }
    }
}

pub fn was_triggered() -> bool {
    WAKE_TRIGGERED.swap(false, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::voice_handler::stt::SttFallback;

    #[test]
    fn check_wake_word_detects_adler() {
        assert!(check_wake_word("Adler merhaba"));
    }

    #[test]
    fn check_wake_word_detects_hey_adler() {
        assert!(check_wake_word("hey adler asistan"));
    }

    #[test]
    fn check_wake_word_case_insensitive() {
        assert!(check_wake_word("ADLER ASISTANA"));
    }

    #[test]
    fn check_wake_word_rejects_other_text() {
        assert!(!check_wake_word("merhaba dunya"));
    }

    #[test]
    fn check_wake_word_sets_trigger_flag() {
        WAKE_TRIGGERED.store(false, Ordering::SeqCst);
        check_wake_word("adler");
        assert!(was_triggered());
    }

    #[test]
    fn was_triggered_consumes_flag() {
        WAKE_TRIGGERED.store(true, Ordering::SeqCst);
        assert!(was_triggered());
        assert!(!was_triggered());
    }

    #[test]
    fn feed_audio_low_energy_does_not_trigger() {
        init_wake_word(SttFallback::new(vec![])).ok();
        WAKE_TRIGGERED.store(false, Ordering::SeqCst);
        let samples = vec![0i16; 100];
        feed_audio(&samples);
        assert!(!was_triggered());
    }

    #[test]
    fn init_wake_word_succeeds() {
        let result = init_wake_word(SttFallback::new(vec![]));
        assert!(result.is_ok());
    }
}
