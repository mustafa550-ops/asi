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
