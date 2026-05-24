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
