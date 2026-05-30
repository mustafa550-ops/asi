use crate::skill::{Skill, SkillManifesto};

pub struct SkillExport;

impl SkillExport {
    pub fn to_json(skill: &Skill) -> Result<String, String> {
        serde_json::to_string_pretty(skill)
            .map_err(|e| format!("JSON serilestirme hatasi: {}", e))
    }

    pub fn from_json(json: &str) -> Result<Skill, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("JSON cozme hatasi: {}", e))
    }

    pub fn to_markdown(skill: &Skill) -> String {
        let triggers = skill.triggers.iter().map(|t| format!("  - \"{}\"", t)).collect::<Vec<_>>().join("\n");
        let steps = skill.steps.iter().map(|s| format!("{}. {}", s.order, s.description)).collect::<Vec<_>>().join("\n");
        let logic = skill.logic_code.as_ref().map(|l| format!("\n```\n{}\n```", l)).unwrap_or_default();

        format!(
            "# Skill: {}\n\n\
             ## Meta\n\
             - **Description:** {}\n\
             - **Triggers:**\n{}\n\
             - **Approval:** {}\n\
             - **Version:** v{}\n\
             - **Category:** {}\n\
             - **Run Count:** {}\n\n\
             ## Steps\n{}\n\
             ## Logic{}\n",
            skill.name, skill.description, triggers, skill.approval, skill.version,
            skill.category, skill.run_count, steps, logic
        )
    }

    pub fn export_string(skill: &Skill, format: &str) -> Result<String, String> {
        match format {
            "json" => Self::to_json(skill),
            "markdown" | "md" => Ok(Self::to_markdown(skill)),
            _ => Err(format!("Bilinmeyen format: '{}' (json/md)", format)),
        }
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
            description: "Test aciklamasi".into(),
            triggers: vec!["test".into()],
            approval: "required".into(),
            steps: vec![SkillStep { order: 1, description: "Adim 1".into() }],
            logic_code: Some("print('hello')".into()),
            evolution: vec![],
            run_count: 5,
            active: true,
            version: 1,
            created_at: "12345".into(),
            category: "general".into(),
            tags: vec!["test".into()],
            rating: 4.5,
            rating_count: 10,
        }
    }

    #[test]
    fn json_roundtrip() {
        let skill = sample_skill();
        let json = SkillExport::to_json(&skill).unwrap();
        let restored = SkillExport::from_json(&json).unwrap();
        assert_eq!(restored.name, skill.name);
        assert_eq!(restored.version, skill.version);
    }

    #[test]
    fn markdown_contains_name() {
        let skill = sample_skill();
        let md = SkillExport::to_markdown(&skill);
        assert!(md.contains("Test_Skill"));
        assert!(md.contains("print('hello')"));
    }

    #[test]
    fn export_format_json() {
        let skill = sample_skill();
        let out = SkillExport::export_string(&skill, "json").unwrap();
        assert!(out.contains("Test_Skill"));
    }

    #[test]
    fn export_format_markdown() {
        let skill = sample_skill();
        let out = SkillExport::export_string(&skill, "md").unwrap();
        assert!(out.contains("Skill: Test_Skill"));
    }

    #[test]
    fn export_invalid_format() {
        let skill = sample_skill();
        assert!(SkillExport::export_string(&skill, "xml").is_err());
    }

    #[test]
    fn from_json_invalid() {
        assert!(SkillExport::from_json("not json").is_err());
    }
}
