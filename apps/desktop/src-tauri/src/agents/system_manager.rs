use super::{Agent, AgentContext};

pub struct SystemManager;

impl Agent for SystemManager {
    fn name(&self) -> String { "System Manager".into() }
    fn description(&self) -> String { "Sistem durumu izleme ve yönetim".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("sistem") || task.contains("ram") || task.contains("cpu") || task.contains("system")
    }
    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let cpu_count = sys.cpus().len();
        let cpu_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_count.max(1) as f32;
        let mem_used = sys.used_memory() / 1024 / 1024;
        let mem_total = sys.total_memory() / 1024 / 1024;
        let uptime = sysinfo::System::uptime() / 60;

        Ok(format!(
            "[System Manager]\n\
             İşlemci: {} çekirdek @ %{:.1} kullanım\n\
             Bellek: {}MB / {}MB kullanımda\n\
             Çalışma süresi: {} dakika\n\
             İstenen: {}",
            cpu_count, cpu_usage, mem_used, mem_total, uptime, task
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::ApprovalLevel;

    #[test]
    fn name_returns_correct() {
        let agent = SystemManager;
        assert_eq!(agent.name(), "System Manager");
    }

    #[test]
    fn can_handle_matches_system_keywords() {
        let agent = SystemManager;
        assert!(agent.can_handle("sistem durumu"));
        assert!(agent.can_handle("ram kullanimi"));
        assert!(agent.can_handle("cpu yuku"));
        assert!(agent.can_handle("system info"));
        assert!(!agent.can_handle("niyet analizi"));
    }

    #[test]
    fn execute_returns_system_info() {
        let agent = SystemManager;
        let ctx = AgentContext {
            ollama: &crate::llm::OllamaClient::new("http://localhost:11434".to_string(), "qwen2.5:1.5b".to_string()),
            claude: None,
            memory: None,
            event_bus: None,
            approval: ApprovalLevel::Observer,
            vosk_model_path: "",
        };
        let result = agent.execute("sistem raporu", &ctx).unwrap();
        assert!(result.starts_with("[System Manager]"));
        assert!(result.contains("İşlemci"));
        assert!(result.contains("Bellek"));
    }
}
