use crate::db::strategic_memory::StrategicMemory;
use crate::llm::OllamaClient;
use crate::skill::registry::SkillRegistry;

const PROMOTION_THRESHOLD: i64 = 10;

pub struct SkillEvolution;

impl SkillEvolution {
    pub fn new() -> Self {
        Self
    }

    pub fn check_and_evolve(
        &self,
        skill_name: &str,
        registry: &SkillRegistry,
        strategic: &StrategicMemory,
        ollama: &OllamaClient,
    ) -> Result<Option<String>, String> {
        let run_count = registry.get_run_count(skill_name)?;

        if run_count < PROMOTION_THRESHOLD {
            return Ok(None);
        }

        let skill = registry.get_by_name(skill_name)?
            .ok_or_else(|| format!("Skill '{}' not found", skill_name))?;

        let decisions = strategic.query_by_context(skill_name, 20)
            .map_err(|e| format!("Strategic memory query failed: {}", e))?;
        if decisions.len() < 5 {
            return Ok(None);
        }

        let success_rate = decisions.iter()
            .filter(|d| d.outcome == "success")
            .count() as f64 / decisions.len() as f64;

        let tree = self.generate_behavior_tree(&skill_name, success_rate, &decisions, ollama)?;

        let mut evolution = skill.evolution.clone();
        let version = evolution.len() + 1;
        let entry = format!("v{}: behavior_tree_generated (success_rate={:.1}%)",
            version, success_rate * 100.0);
        evolution.push(entry);
        evolution.push(format!("v{}: tree={}", version, tree));

        let steps = skill.steps.clone();
        let triggers = skill.triggers.clone();
        let logic = skill.logic_code.as_deref();

        registry.register(
            &skill.name, &skill.description, &triggers, &skill.approval,
            &steps, logic, &evolution,
        )?;

        Ok(Some(format!(
            "Skill '{}' evolve edildi (v{}, {}/{} başarılı, çalışma={})",
            skill_name, version, decisions.iter().filter(|d| d.outcome == "success").count(),
            decisions.len(), run_count
        )))
    }

    fn generate_behavior_tree(
        &self,
        skill_name: &str,
        success_rate: f64,
        decisions: &[crate::db::strategic_memory::StrategicRecord],
        ollama: &OllamaClient,
    ) -> Result<String, String> {
        let decisions_summary: Vec<String> = decisions.iter()
            .map(|d| format!("[{}] context={}", d.outcome, d.context.chars().take(60).collect::<String>()))
            .collect();

        let prompt = format!(
            "Skill '{}' için {} karardan bir davranış modeli (behavior tree) oluştur.\n\
             Başarı oranı: {:.1}%\n\
             Kararlar:\n{}\n\n\
             Hangi koşullarda başarılı/başarısız olunduğunu belirten bir ağaç yapısı (indent-based) çıkar.",
            skill_name, decisions.len(), success_rate * 100.0,
            decisions_summary.join("\n")
        );

        ollama.generate_sync(&prompt)
    }

    pub fn resolve_approval(&self, skill: &crate::skill::Skill) -> crate::agents::ApprovalLevel {
        match skill.approval.to_lowercase().as_str() {
            "observer" | "required" => crate::agents::ApprovalLevel::Observer,
            "strategic" => crate::agents::ApprovalLevel::Strategic,
            _ => crate::agents::ApprovalLevel::SemiAutonomous,
        }
    }
}

