pub struct SkillTemplate;

impl SkillTemplate {
    pub fn generate(name: &str, description: &str, triggers: &[&str]) -> String {
        let triggers_str = triggers.iter()
            .map(|t| format!("  - \"{}\"", t))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "# Skill: {}\n\n\
             ## Meta\n\
             - **Description:** {}\n\
             - **Tool:** local_python\n\
             - **Bridge:** rust_core\n\
             - **Triggers:**\n{}\n\
             - **Approval:** required\n\n\
             ## Steps\n\
             1. Gorevi analiz et\n\
             2. Gerekli islemi yap\n\
             3. Sonucu raporla\n\n\
             ## Logic\n\
             ```python\n\
             def run(task: str) -> str:\n\
                 # Gorev mantigi buraya\n\
                 return f\"{} islendi: {{task}}\"\n\
             ```\n\n\
             ## Evolution\n\
             - **v1.0:** Ilk surum\n",
            name, description, triggers_str, name
        )
    }

    pub fn generate_python(name: &str, description: &str, triggers: &[&str]) -> String {
        Self::generate(name, description, triggers)
    }

    pub fn generate_shell(name: &str, description: &str, triggers: &[&str]) -> String {
        let triggers_str = triggers.iter()
            .map(|t| format!("  - \"{}\"", t))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "# Skill: {}\n\n\
             ## Meta\n\
             - **Description:** {}\n\
             - **Tool:** shell\n\
             - **Bridge:** tauri_fs_command\n\
             - **Triggers:**\n{}\n\
             - **Approval:** required\n\n\
             ## Steps\n\
             1. Shell komutunu calistir\n\
             2. Ciktiyi kontrol et\n\n\
             ## Logic\n\
             ```sh\n\
             #!/bin/bash\n\
             # {} islemi\n\
             echo \"{} basladi\"\n\
             ```\n\n\
             ## Evolution\n\
             - **v1.0:** Ilk surum\n",
            name, description, triggers_str, name, name
        )
    }

    pub fn generate_rust(name: &str, description: &str, triggers: &[&str]) -> String {
        let triggers_str = triggers.iter()
            .map(|t| format!("  - \"{}\"", t))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "# Skill: {}\n\n\
             ## Meta\n\
             - **Description:** {}\n\
             - **Tool:** rust\n\
             - **Bridge:** rust_core\n\
             - **Triggers:**\n{}\n\
             - **Approval:** required\n\n\
             ## Steps\n\
             1. Rust kodunu derle\n\
             2. WASM sandbox'ta calistir\n\n\
             ## Logic\n\
             ```rust\n\
             pub fn process(task: &str) -> String {{\n\
                 format!(\"{} isleniyor: {{}}\", task)\n\
             }}\n\
             ```\n\n\
             ## Evolution\n\
             - **v1.0:** Ilk surum\n",
            name, description, triggers_str, name
        )
    }

    pub fn list_templates() -> Vec<&'static str> {
        vec!["python", "shell", "rust"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_python_template() {
        let md = SkillTemplate::generate("Test_Skill", "Test aciklamasi", &["test", "demo"]);
        assert!(md.contains("Test_Skill"));
        assert!(md.contains("local_python"));
        assert!(md.contains("test"));
        assert!(md.contains("demo"));
    }

    #[test]
    fn generate_shell_template() {
        let md = SkillTemplate::generate_shell("Backup", "Dosya yedekleme", &["backup", "yedek"]);
        assert!(md.contains("Backup"));
        assert!(md.contains("shell"));
        assert!(md.contains("#!/bin/bash"));
    }

    #[test]
    fn generate_rust_template() {
        let md = SkillTemplate::generate_rust("Analyzer", "Veri analizi", &["analiz", "analyze"]);
        assert!(md.contains("Analyzer"));
        assert!(md.contains("rust"));
        assert!(md.contains("WASM"));
    }

    #[test]
    fn list_templates_returns_three() {
        let list = SkillTemplate::list_templates();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&"python"));
        assert!(list.contains(&"shell"));
        assert!(list.contains(&"rust"));
    }

    #[test]
    fn template_contains_evolution_section() {
        let md = SkillTemplate::generate("Test", "desc", &["t"]);
        assert!(md.contains("Evolution"));
        assert!(md.contains("v1.0"));
    }

    #[test]
    fn template_contains_steps_section() {
        let md = SkillTemplate::generate("Test", "desc", &["t"]);
        assert!(md.contains("## Steps"));
        assert!(md.contains("## Logic"));
    }
}
