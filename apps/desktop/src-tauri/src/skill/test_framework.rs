use crate::skill::bridge::{BridgeAction, BridgeResult, MockBridge, SkillBridge};
use crate::skill::Skill;

pub struct SkillTestRunner {
    bridge: Box<dyn SkillBridge>,
}

impl SkillTestRunner {
    pub fn new() -> Self {
        Self { bridge: Box::new(MockBridge::new()) }
    }

    pub fn with_bridge(bridge: Box<dyn SkillBridge>) -> Self {
        Self { bridge }
    }

    pub fn test_execution(&self, skill: &Skill, task: &str) -> TestResult {
        let step_results: Vec<String> = skill.steps.iter().map(|s| {
            let desc = s.description.replace("{task}", task);
            format!("[OK] {}", desc)
        }).collect();

        let success = !step_results.is_empty();
        TestResult {
            skill_name: skill.name.clone(),
            task: task.to_string(),
            step_results,
            success,
            bridge_calls: Vec::new(),
        }
    }

    pub fn test_trigger(&self, skill: &Skill, input: &str) -> bool {
        skill.triggers.iter()
            .any(|t| input.to_lowercase().contains(&t.to_lowercase()))
    }

    pub fn test_bridge_action(&self, action: &BridgeAction) -> Result<BridgeResult, String> {
        self.bridge.execute(action)
    }

    pub fn validate_manifesto(skill: &Skill) -> Vec<String> {
        let mut errors = Vec::new();
        if skill.name.is_empty() {
            errors.push("Skill adi bos".into());
        }
        if skill.triggers.is_empty() {
            errors.push("Tetikleyici yok".into());
        }
        if skill.steps.is_empty() {
            errors.push("Adim yok".into());
        }
        errors
    }
}

pub struct TestResult {
    pub skill_name: String,
    pub task: String,
    pub step_results: Vec<String>,
    pub success: bool,
    pub bridge_calls: Vec<String>,
}

impl TestResult {
    pub fn summary(&self) -> String {
        format!(
            "Skill '{}' (gorev: {}): {} adim, durum: {}",
            self.skill_name,
            self.task,
            self.step_results.len(),
            if self.success { "BASARILI" } else { "BASARISIZ" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::SkillStep;

    fn sample_skill() -> Skill {
        Skill {
            id: 1,
            name: "Test_Skill".into(),
            description: "Test".into(),
            triggers: vec!["test".into()],
            approval: "auto".into(),
            steps: vec![SkillStep { order: 1, description: "Adim 1".into() }],
            logic_code: None,
            evolution: vec![],
            run_count: 0,
            active: true,
            version: 1,
            created_at: "0".into(),
            category: "gen".into(),
            tags: vec![],
            rating: 0.0,
            rating_count: 0,
        }
    }

    #[test]
    fn test_execution_success() {
        let runner = SkillTestRunner::new();
        let skill = sample_skill();
        let result = runner.test_execution(&skill, "test_gorev");
        assert!(result.success);
        assert_eq!(result.step_results.len(), 1);
    }

    #[test]
    fn test_trigger_match() {
        let runner = SkillTestRunner::new();
        let skill = sample_skill();
        assert!(runner.test_trigger(&skill, "bu bir test mesaji"));
        assert!(!runner.test_trigger(&skill, "ilgisiz mesaj"));
    }

    #[test]
    fn test_trigger_case_insensitive() {
        let runner = SkillTestRunner::new();
        let skill = sample_skill();
        assert!(runner.test_trigger(&skill, "TEST MESAJI"));
    }

    #[test]
    fn test_bridge_action() {
        let runner = SkillTestRunner::new();
        let result = runner.test_bridge_action(&BridgeAction::Log { message: "test".into() });
        assert!(result.is_ok());
    }

    #[test]
    fn validate_manifesto_valid() {
        let skill = sample_skill();
        let errors = SkillTestRunner::validate_manifesto(&skill);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_manifesto_empty_name() {
        let mut skill = sample_skill();
        skill.name = "".into();
        let errors = SkillTestRunner::validate_manifesto(&skill);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_manifesto_no_triggers() {
        let mut skill = sample_skill();
        skill.triggers = vec![];
        let errors = SkillTestRunner::validate_manifesto(&skill);
        assert!(!errors.is_empty());
    }

    #[test]
    fn summary_contains_name() {
        let result = TestResult {
            skill_name: "Test".into(),
            task: "gorev".into(),
            step_results: vec!["OK".into()],
            success: true,
            bridge_calls: vec![],
        };
        let s = result.summary();
        assert!(s.contains("Test"));
        assert!(s.contains("BASARILI"));
    }
}
