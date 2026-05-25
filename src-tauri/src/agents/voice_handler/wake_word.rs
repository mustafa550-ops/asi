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
