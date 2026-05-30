use super::{Agent, AgentContext};

pub struct DiagnosticAgent;

impl Agent for DiagnosticAgent {
    fn name(&self) -> String { "Diagnostic Agent".into() }
    fn description(&self) -> String { "Hata teşhisi ve self-healing önerisi".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("hata") || task.contains("diagnostic") || task.contains("arıza") || task.contains("error")
    }
    fn execute(&self, task: &str, _ctx: &AgentContext) -> Result<String, String> {
        let log_dir = std::path::Path::new("/var/log");
        let mut findings = Vec::new();
        if log_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(log_dir) {
                for entry in entries.flatten().take(5) {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "log" {
                            findings.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        let log_summary = if findings.is_empty() {
            "No system logs found".to_string()
        } else {
            format!("Found {} log files. First: {}", findings.len(), findings[0])
        };
        Ok(format!("[Diagnostic] Analiz: '{}'\n{}\nÖneri: Logları incelemek için 'detayları anlat' komutunu kullan.", task, log_summary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::ApprovalLevel;

    #[test]
    fn name_returns_correct() {
        let agent = DiagnosticAgent;
        assert_eq!(agent.name(), "Diagnostic Agent");
    }

    #[test]
    fn description_not_empty() {
        let agent = DiagnosticAgent;
        assert!(!agent.description().is_empty());
    }

    #[test]
    fn can_handle_matches_error_keywords() {
        let agent = DiagnosticAgent;
        assert!(agent.can_handle("hata var"));
        assert!(agent.can_handle("diagnostic calistir"));
        assert!(agent.can_handle("arıza tespiti"));
        assert!(agent.can_handle("error log"));
        assert!(!agent.can_handle("sistem durumu"));
    }

    #[test]
    fn execute_returns_diagnostic_format() {
        let agent = DiagnosticAgent;
        let ctx = AgentContext {
            ollama: &crate::llm::OllamaClient::new("http://localhost:11434".to_string(), "qwen2.5:1.5b".to_string()),
            claude: None,
            memory: None,
            event_bus: None,
            approval: ApprovalLevel::Observer,
            vosk_model_path: "",
        };
        let result = agent.execute("hatali modul", &ctx).unwrap();
        assert!(result.starts_with("[Diagnostic]"));
        assert!(result.contains("hatali modul"));
    }
}
