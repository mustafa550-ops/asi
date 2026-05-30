use super::{Agent, AgentContext};

pub struct SupervisorAgent;

impl Agent for SupervisorAgent {
    fn name(&self) -> String { "Supervisor Agent".into() }
    fn description(&self) -> String { "Ajan hata düzeltme, retry ve optimizasyon".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("supervisor") || task.contains("optimize") || task.contains("retry")
    }
    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let target = task.replace("retry", "").replace("optimize", "").trim().to_string();
        let mut last_error = String::new();
        for attempt in 1..=3 {
            let prompt = format!(
                "Kullanıcı isteğini değerlendir ve nasıl çözüleceğini açıkla.\n\
                 İstek: {}\nDeneme: {}/3\nKısa ve net bir yanıt ver.", target, attempt
            );
            match ctx.ollama.generate_sync(&prompt) {
                Ok(response) => {
                    if attempt > 1 {
                        return Ok(format!("[Supervisor] {} numaralı denemede başarılı (önceki hata: {})\nYanıt: {}",
                            attempt, last_error, response.trim()));
                    }
                    return Ok(format!("[Supervisor] Optimizasyon önerisi:\n{}", response.trim()));
                }
                Err(e) => {
                    last_error = e.to_string();
                }
            }
        }
        Err(format!("[Supervisor] 3 deneme başarısız. Son hata: {}", last_error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::ApprovalLevel;

    #[test]
    fn name_returns_correct() {
        let agent = SupervisorAgent;
        assert_eq!(agent.name(), "Supervisor Agent");
    }

    #[test]
    fn can_handle_matches_supervisor_keywords() {
        let agent = SupervisorAgent;
        assert!(agent.can_handle("supervisor calistir"));
        assert!(agent.can_handle("optimize et"));
        assert!(agent.can_handle("retry yap"));
        assert!(!agent.can_handle("sistem durumu"));
    }

    #[test]
    fn execute_fails_after_3_attempts_when_ollama_down() {
        let agent = SupervisorAgent;
        let ctx = AgentContext {
            ollama: &crate::llm::OllamaClient::new("http://localhost:19999".to_string(), "qwen2.5:1.5b".to_string()),
            claude: None,
            memory: None,
            event_bus: None,
            approval: ApprovalLevel::Observer,
            vosk_model_path: "",
        };
        let result = agent.execute("retry test task", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("3 deneme başarısız"));
    }
}
