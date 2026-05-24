use super::Agent;

/// Voice Handler — STT/TTS, wake word, sesli diyalog (§4.1).
pub struct VoiceHandler;

impl Agent for VoiceHandler {
    fn name(&self) -> String { "Voice Handler".into() }
    fn description(&self) -> String { "Ses tanıma ve sentez".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("ses") || task.contains("voice") || task.contains("konuş")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Sesli yanıt hazırlandı".into())
    }
}
