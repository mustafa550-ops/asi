use std::sync::atomic::{AtomicBool, Ordering};

static WAKE_TRIGGERED: AtomicBool = AtomicBool::new(false);

const WAKE_WORDS: &[&str] = &["adler", "hey adler", "adler asistan", "adler asistana"];

pub fn init_wake_word() -> Result<(), String> {
    log::info!("Wake word baslati — kelimeler: {:?}", WAKE_WORDS);
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
        if let Some(text) = super::stt::transcribe(samples) {
            check_wake_word(&text);
        }
    }
}

pub fn was_triggered() -> bool {
    WAKE_TRIGGERED.swap(false, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        WAKE_TRIGGERED.store(false, Ordering::SeqCst);
        let samples = vec![0i16; 100];
        feed_audio(&samples);
        assert!(!was_triggered());
    }

    #[test]
    fn init_wake_word_succeeds() {
        let result = init_wake_word();
        assert!(result.is_ok());
    }
}
