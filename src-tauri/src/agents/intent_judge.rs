use super::{Agent, AgentContext};

pub struct IntentJudge;

impl Agent for IntentJudge {
    fn name(&self) -> String {
        "Intent Judge".into()
    }

    fn description(&self) -> String {
        "Niyet analizi — kullanıcı komutunu sorgu/eylem/analiz/donanım/kripto olarak sınıflandırır".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        task.contains("niyet") || task.contains("intent") || task.contains("ne yapmalı")
    }

    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let prompt = format!(
            "Kullanıcı mesajını tek bir kategoriye sınıflandır.\n\
             Kategoriler: sorgu, eylem, analiz, donanım, kripto, sistem, doküman, ses\n\
             Sadece kategori adını yaz, açıklama ekleme.\n\
             Mesaj: {}",
            task
        );
        let category = ctx
            .ollama
            .generate_sync("qwen2.5:1.5b", &prompt)?;
        let category = category.trim().to_lowercase();
        Ok(format!("Intent classified: {}", category))
    }
}
