pub mod audio;
pub mod wake_word;
pub mod stt;
pub mod tts;

use super::{Agent, AgentContext};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct VoiceHandler {
    active: AtomicBool,
}

impl VoiceHandler {
    pub fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
        }
    }

    pub fn start_listener(&self, model_path: &str) -> Result<(), String> {
        if self.active.load(Ordering::Relaxed) {
            return Ok(());
        }

        wake_word::init_wake_word()?;
        if !model_path.is_empty() {
            stt::init_stt(model_path)?;
        } else {
            log::warn!("Vosk model yolu ayarlanmamis — STT ses tanima pasif");
        }
        self.active.store(true, Ordering::Relaxed);
        let running = Arc::new(AtomicBool::new(true));
        let r = Arc::clone(&running);

        std::thread::spawn(move || {
            match audio::record_loop() {
                Ok(stream) => {
                    while r.load(Ordering::Relaxed) {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        if wake_word::was_triggered() {
                            log::info!("Wake word tetiklendi — pipeline baslatiliyor");
                        }
                    }
                    std::mem::drop(stream);
                }
                Err(e) => {
                    log::error!("Ses kaydi basarisiz: {}", e);
                }
            }
        });

        Ok(())
    }

    pub fn stop_listener(&self) {
        self.active.store(false, Ordering::Relaxed);
        audio::stop_recording();
    }
}

impl Agent for VoiceHandler {
    fn name(&self) -> String {
        "Voice Handler".into()
    }

    fn description(&self) -> String {
        "Ses tanima, wake word, STT ve TTS".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        task.contains("ses") || task.contains("voice") || task.contains("konus") || task.contains("speak") || task.contains("dinle")
    }

    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        if task.contains("dinle") || task.contains("baslat") {
            self.start_listener(ctx.vosk_model_path)?;
            return Ok("[Voice Handler] Dinleme basladi — wake word bekleniyor...".into());
        }

        if task.contains("durdur") || task.contains("stop") {
            self.stop_listener();
            return Ok("[Voice Handler] Dinleme durduruldu.".into());
        }

        let text_to_speak = task.trim();
        let prompt = format!(
            "Asagidaki metni konusma icin kisa ve net bir Turkce cumleye donustur.\n\
             Sadece donusturulmus metni yaz, aciklama ekleme.\n\
             Metin: {}", text_to_speak
        );
        let spoken = ctx.ollama.generate_sync(&prompt)?;
        let spoken = spoken.trim();
        let tts_result = tts::synthesize(spoken).unwrap_or_else(|e| {
            format!("TTS sentez hatasi: {}", e)
        });

        if let Some(bus) = ctx.event_bus {
            bus.emit("voice-output", &tts_result);
        }

        Ok(format!("[Voice Handler] Soylenen: {} | TTS: {}", spoken, tts_result))
    }
}
