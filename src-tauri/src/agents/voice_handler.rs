use super::{Agent, AgentContext};

pub struct VoiceHandler;

impl Agent for VoiceHandler {
    fn name(&self) -> String { "Voice Handler".into() }
    fn description(&self) -> String { "Ses tanıma ve sentez".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("ses") || task.contains("voice") || task.contains("konuş") || task.contains("speak")
    }
    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let text_to_speak = task.trim();
        let prompt = format!(
            "Aşağıdaki metni konuşma için kısa ve net bir Türkçe cümleye dönüştür.\n\
             Sadece dönüştürülmüş metni yaz, açıklama ekleme.\n\
             Metin: {}", text_to_speak
        );
        let spoken = ctx.ollama.generate_sync("qwen2.5:1.5b", &prompt)?;
        Ok(format!("[Voice Handler] Ses sentezlendi:\n\"{}\"\n---\nTTS motoru: Piper (offline) — kurulum gerektirir.\nMevcut mod: metin ön işleme tamamlandı.", spoken.trim()))
    }
}
