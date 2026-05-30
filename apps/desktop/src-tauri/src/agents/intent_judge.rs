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
            .generate_sync(&prompt)?;
        let category = category.trim().to_lowercase();
        Ok(format!("Intent classified: {}", category))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_returns_correct() {
        let agent = IntentJudge;
        assert_eq!(agent.name(), "Intent Judge");
    }

    #[test]
    fn description_not_empty() {
        let agent = IntentJudge;
        assert!(!agent.description().is_empty());
    }

    #[test]
    fn can_handle_matches_intent_keywords() {
        let agent = IntentJudge;
        assert!(agent.can_handle("niyet analizi yap"));
        assert!(agent.can_handle("intent classification"));
        assert!(agent.can_handle("ne yapmalıyım"));
        assert!(!agent.can_handle("role ac"));
        assert!(!agent.can_handle("borsa analiz"));
    }
}
