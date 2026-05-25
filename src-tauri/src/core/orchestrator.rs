use crate::agents::supervisor::SupervisorAgent;
use crate::agents::{Agent, AgentContext, ApprovalLevel};
use crate::bridge::event_bus::EventBus;
use crate::core::memory_manager::MemoryManager;
use crate::llm::claude::ClaudeClient;
use crate::llm::OllamaClient;
use crate::skill::executor::SkillExecutor;
use crate::skill::registry::SkillRegistry;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStep {
    Intent,
    Delegate,
    Plan,
    Execute,
    Confirm,
    Report,
}

pub struct Orchestrator {
    agents: Vec<Box<dyn Agent + Send>>,
    pub approval_level: ApprovalLevel,
    supervisor: SupervisorAgent,
    pending_approvals: Mutex<HashMap<String, String>>,
    skill_registry: Option<SkillRegistry>,
    skill_executor: SkillExecutor,
    claude: Option<ClaudeClient>,
}

impl Orchestrator {
    pub fn new(approval_level: ApprovalLevel) -> Self {
        Self {
            agents: Vec::new(),
            approval_level,
            supervisor: SupervisorAgent,
            pending_approvals: Mutex::new(HashMap::new()),
            skill_registry: None,
            skill_executor: SkillExecutor::new(),
            claude: None,
        }
    }

    pub fn with_claude(mut self, claude: ClaudeClient) -> Self {
        self.claude = Some(claude);
        self
    }

    pub fn with_skill_registry(mut self, registry: SkillRegistry) -> Self {
        self.skill_registry = Some(registry);
        self
    }

    pub fn register_agent(&mut self, agent: Box<dyn Agent + Send>) {
        self.agents.push(agent);
    }

    pub fn skill_registry(&self) -> Option<&SkillRegistry> {
        self.skill_registry.as_ref()
    }

    fn llm_generate(&self, prompt: &str, ollama: &OllamaClient) -> Result<String, String> {
        match ollama.generate_sync(prompt) {
            Ok(r) => Ok(r),
            Err(ollama_err) => {
                if let Some(ref claude) = self.claude {
                    log::warn!("Ollama hatasi, Claude fallback: {}", ollama_err);
                    claude.generate_sync(prompt, 1024)
                        .map_err(|_| format!("Ollama ve Claude basarisiz: {}", ollama_err))
                } else {
                    Err(ollama_err)
                }
            }
        }
    }

    pub fn run_pipeline(
        &self,
        task: &str,
        ollama: &OllamaClient,
        memory: Option<&MemoryManager>,
        event_bus: Option<&EventBus>,
    ) -> Result<String, String> {
        let ctx = AgentContext {
            ollama,
            claude: self.claude.as_ref(),
            memory,
            event_bus,
            approval: self.approval_level.clone(),
            vosk_model_path: "",
        };

        // Phase 0: Skill Trigger Check
        if let Some(ref registry) = self.skill_registry {
            let matched_skills = registry.find_by_trigger(task, Some(ollama)).unwrap_or_default();
            if let Some(skill) = matched_skills.first() {
                let log = format!("[Pipeline] SKILL TRIGGERED: {} ({} trigger ile eşleşti)", skill.name, skill.triggers.len());
                if let Some(bus) = event_bus {
                    bus.emit("pipeline-step", &log);
                }

                let result = self.skill_executor.execute(skill, task, ollama, memory)?;
                registry.increment_run_count(&skill.name).ok();

                let report = format!(
                    "=== ADLER Skill Rapor ===\nSkill: {}\n{}\n{}",
                    skill.name, result.summary, result.step_results.join("\n")
                );

                if let Some(mem) = memory {
                    mem.store_long_term(&report, &skill.name, "skill_execution").ok();
                }

                return Ok(report);
            }
        }

        // 1. Intent Analysis
        let intent = self.step_intent(task, ollama)?;
        let log = format!("[Pipeline] STEP 1/6 — Intent: {}", intent);
        if let Some(bus) = event_bus {
            bus.emit("pipeline-step", &log);
        }

        // 2. Agent Delegation
        let matched: Vec<&Box<dyn Agent + Send>> = self.agents.iter()
            .filter(|a| a.can_handle(task) || intent.to_lowercase().contains(&a.name().to_lowercase()))
            .collect();
        if matched.is_empty() {
            return Err(format!("Hiçbir ajan '{}' görevini işleyemiyor", task));
        }
        let log = format!("[Pipeline] STEP 2/6 — Delegated to {} agent(s): {}",
            matched.len(), matched.iter().map(|a| a.name()).collect::<Vec<_>>().join(", "));
        if let Some(bus) = event_bus {
            bus.emit("pipeline-step", &log);
        }

        // 3. Plan
        let plan = self.step_plan(task, matched[0].as_ref(), ollama)?;
        let log = format!("[Pipeline] STEP 3/6 — Plan: {}", plan);
        if let Some(bus) = event_bus {
            bus.emit("pipeline-step", &log);
        }

        // 4. Execute
        let mut results = Vec::new();
        for agent in &matched {
            match agent.execute(task, &ctx) {
                Ok(r) => {
                    results.push(format!("[{}] Başarılı: {}", agent.name(), r));
                    let log = format!("[Pipeline] STEP 4/6 — {} completed", agent.name());
                    if let Some(bus) = event_bus {
                        bus.emit("pipeline-step", &log);
                    }
                }
                Err(e) => {
                    let log = format!("[Pipeline] STEP 4/6 — {} failed: {}", agent.name(), e);
                    if let Some(bus) = event_bus {
                        bus.emit("pipeline-error", &log);
                    }
                    match self.supervisor.execute(&format!("retry {}", task), &ctx) {
                        Ok(fix) => results.push(format!("[Supervisor] {}", fix)),
                        Err(super_err) => results.push(format!("[{}] Hata: {}\n[Supervisor] Kurtarılamadı: {}", agent.name(), e, super_err)),
                    }
                }
            }
        }

        // 5. Confirmation
        if self.approval_level == ApprovalLevel::Observer && matched.len() > 1 {
            if let Some(bus) = event_bus {
                let confirm_id = format!("confirm-{}", std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
                let summary = results.join("\n");
                {
                    let mut pending = self.pending_approvals.lock().unwrap();
                    pending.insert(confirm_id.clone(), summary.clone());
                }
                bus.emit("approval-required", &format!("{{\"id\":\"{}\",\"task\":\"{}\",\"summary\":\"{}\"}}",
                    confirm_id, task, summary.replace('"', "'")));
                results.push(format!("[Confirmation] Onay bekleniyor — ID: {}", confirm_id));
            }
        }
        let log = format!("[Pipeline] STEP 5/6 — Confirmation level: {:?}", self.approval_level);
        if let Some(bus) = event_bus {
            bus.emit("pipeline-step", &log);
        }

        // 6. Report
        let report = self.step_report(task, &results, memory)?;
        let log = format!("[Pipeline] STEP 6/6 — Report generated");
        if let Some(bus) = event_bus {
            bus.emit("pipeline-complete", &log);
        }
        Ok(report)
    }

    fn step_intent(&self, task: &str, ollama: &OllamaClient) -> Result<String, String> {
        let prompt = format!(
            "Kullanıcı mesajını analiz et. En uygun kategoriyi seç: sorgu, eylem, analiz, donanım, kripto, sistem, doküman, ses.\n\
             Sadece kategori adını yaz.\nMesaj: {}", task);
        self.llm_generate(&prompt, ollama).map(|r| r.trim().to_string())
    }

    fn step_plan(&self, _task: &str, agent: &dyn Agent, ollama: &OllamaClient) -> Result<String, String> {
        let prompt = format!(
            "Bu gorev icin kisa bir aksiyon plani olustur. Agent: {}. Adimlari sirala.",
            agent.name()
        );
        match self.llm_generate(&prompt, ollama) {
            Ok(r) => Ok(format!("Plan: {}", r.trim())),
            Err(_) => Ok(format!("Plan: {} tarafından işlenecek. Adımlar: Girdiyi doğrula → İşle → Sonuçları raporla → Hafızaya kaydet", agent.name()))
        }
    }

    fn step_report(&self, task: &str, results: &[String], memory: Option<&MemoryManager>) -> Result<String, String> {
        let mut report = format!("=== ADLER Rapor ===\nGörev: {}\n\n", task);
        for r in results {
            report.push_str(r);
            report.push('\n');
        }
        report.push_str("\n---\nSistem stabil.");
        if let Some(mem) = memory {
            mem.store_long_term(&report, "Orchestrator", "log").ok();
        }
        Ok(report)
    }

    pub fn approve(&self, id: &str) -> Result<String, String> {
        let mut pending = self.pending_approvals.lock().unwrap();
        pending.remove(id).ok_or_else(|| format!("Onay ID '{}' bulunamadı veya süresi doldu", id))
    }

    pub fn reject(&self, id: &str) -> Result<(), String> {
        let mut pending = self.pending_approvals.lock().unwrap();
        pending.remove(id).ok_or_else(|| format!("Onay ID '{}' bulunamadı", id)).map(|_| ())
    }

    pub fn pending_count(&self) -> usize {
        self.pending_approvals.lock().unwrap().len()
    }

    pub fn run_skill_direct(&self, name: &str, ollama: &OllamaClient, memory: Option<&MemoryManager>) -> Result<String, String> {
        let registry = self.skill_registry.as_ref()
            .ok_or_else(|| "Skill registry aktif degil".to_string())?;
        let skill = registry.get_by_name(name)?
            .ok_or_else(|| format!("Skill '{}' bulunamadi", name))?;

        if !skill.active {
            return Err(format!("Skill '{}' pasif — once aktif edin", name));
        }

        let result = self.skill_executor.execute(&skill, &skill.name, ollama, memory)?;
        registry.increment_run_count(&skill.name).ok();

        let report = format!(
            "=== ADLER Skill Rapor ===\nSkill: {}\n{}\n{}",
            skill.name, result.summary, result.step_results.join("\n")
        );

        if let Some(mem) = memory {
            mem.store_long_term(&report, &skill.name, "skill_execution").ok();
        }

        Ok(report)
    }
}
