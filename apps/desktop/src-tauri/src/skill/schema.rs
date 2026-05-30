use crate::skill::SkillManifesto;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        Self { field: field.into(), message: message.into() }
    }
}

pub struct ManifestoSchema;

impl ManifestoSchema {
    pub fn validate(manifesto: &SkillManifesto) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        if manifesto.name.is_empty() {
            errors.push(ValidationError::new("name", "Skill adi bos olamaz"));
        } else if manifesto.name.len() > 100 {
            errors.push(ValidationError::new("name", "Skill adi en fazla 100 karakter olabilir"));
        } else if !manifesto.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ') {
            errors.push(ValidationError::new("name", "Skill adi yalnizca harf, rakam, bosluk, '_' ve '-' icerebilir"));
        }

        if manifesto.description.len() > 500 {
            errors.push(ValidationError::new("description", "Aciklama en fazla 500 karakter olabilir"));
        }

        if manifesto.triggers.is_empty() {
            errors.push(ValidationError::new("triggers", "En az bir tetikleyici gereklidir"));
        }
        for (i, t) in manifesto.triggers.iter().enumerate() {
            if t.len() > 100 {
                errors.push(ValidationError::new(&format!("triggers[{}]", i), "Tetikleyici en fazla 100 karakter olabilir"));
            }
        }

        match manifesto.approval.as_str() {
            "required" | "auto" | "strategic" | "observer" => {}
            _ => errors.push(ValidationError::new("approval", "Gecersiz onay seviyesi (required/auto/strategic/observer)")),
        }

        if manifesto.steps.is_empty() {
            errors.push(ValidationError::new("steps", "En az bir adim gereklidir"));
        }

        if let Some(ref tool) = manifesto.tool {
            match tool.as_str() {
                "local_python" | "anthropic_api" | "shell" | "javascript" | "rust" => {}
                _ => errors.push(ValidationError::new("tool", "Gecersiz araç (local_python/anthropic_api/shell/javascript/rust)")),
            }
        }

        if let Some(ref bridge) = manifesto.bridge {
            match bridge.as_str() {
                "tauri_fs_command" | "rust_core" | "http" => {}
                _ => errors.push(ValidationError::new("bridge", "Gecersiz kopru (tauri_fs_command/rust_core/http)")),
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() { return Err("Skill adi bos olamaz".into()); }
        if name.len() > 100 { return Err("Skill adi en fazla 100 karakter".into()); }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Skill adi yalnizca harf, rakam, '_' ve '-' icerebilir".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_manifesto() -> SkillManifesto {
        SkillManifesto {
            name: "Test_Skill".into(),
            description: "Test aciklamasi".into(),
            tool: Some("local_python".into()),
            bridge: Some("rust_core".into()),
            triggers: vec!["test".into(), "demo".into()],
            approval: "required".into(),
            steps: vec!["Adim 1".into(), "Adim 2".into()],
            logic: Some("print('hello')".into()),
            evolution: vec!["v1.0: Ilk surum".into()],
        }
    }

    #[test]
    fn valid_manifesto_passes() {
        let m = valid_manifesto();
        assert!(ManifestoSchema::validate(&m).is_ok());
    }

    #[test]
    fn empty_name_fails() {
        let mut m = valid_manifesto();
        m.name = "".into();
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn long_name_fails() {
        let mut m = valid_manifesto();
        m.name = "a".repeat(101);
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn invalid_chars_in_name_fails() {
        let mut m = valid_manifesto();
        m.name = "test@skill!".into();
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn name_with_underscore_and_dash_passes() {
        let mut m = valid_manifesto();
        m.name = "my_cool-skill".into();
        assert!(ManifestoSchema::validate(&m).is_ok());
    }

    #[test]
    fn empty_triggers_fails() {
        let mut m = valid_manifesto();
        m.triggers = vec![];
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "triggers"));
    }

    #[test]
    fn invalid_approval_fails() {
        let mut m = valid_manifesto();
        m.approval = "invalid".into();
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "approval"));
    }

    #[test]
    fn empty_steps_fails() {
        let mut m = valid_manifesto();
        m.steps = vec![];
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "steps"));
    }

    #[test]
    fn invalid_tool_fails() {
        let mut m = valid_manifesto();
        m.tool = Some("invalid_tool".into());
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "tool"));
    }

    #[test]
    fn invalid_bridge_fails() {
        let mut m = valid_manifesto();
        m.bridge = Some("invalid_bridge".into());
        let err = ManifestoSchema::validate(&m).unwrap_err();
        assert!(err.iter().any(|e| e.field == "bridge"));
    }

    #[test]
    fn validate_name_ok() {
        assert!(ManifestoSchema::validate_name("Test_Skill").is_ok());
    }

    #[test]
    fn validate_name_empty_fails() {
        assert!(ManifestoSchema::validate_name("").is_err());
    }

    #[test]
    fn validate_name_invalid_chars_fails() {
        assert!(ManifestoSchema::validate_name("test skill!").is_err());
    }

    #[test]
    fn multiple_errors_returned() {
        let m = SkillManifesto {
            name: "".into(),
            description: "".into(),
            tool: None,
            bridge: None,
            triggers: vec![],
            approval: "wrong".into(),
            steps: vec![],
            logic: None,
            evolution: vec![],
        };
        let errs = ManifestoSchema::validate(&m).unwrap_err();
        assert!(errs.len() >= 3);
    }
}
