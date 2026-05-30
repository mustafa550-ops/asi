use crate::skill::{SkillManifesto, SkillStep};

pub struct ManifestoParser;

impl ManifestoParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(content: &str, source_path: &str) -> Result<SkillManifesto, String> {
        let name = Self::extract_title(content)
            .unwrap_or_else(|| "Unnamed".to_string());
        let description = Self::extract_meta_field(content, "Description")
            .or_else(|| Self::extract_meta_field(content, "description"))
            .or_else(|| {
                if content.contains("---") {
                    content.splitn(3, "---").nth(1)
                        .and_then(|fm| Self::extract_frontmatter(fm, "Description")
                            .or_else(|| Self::extract_frontmatter(fm, "description")))
                } else { None }
            })
            .unwrap_or_default();
        let tool;
        let bridge;
        let triggers;
        let approval;
        let steps;
        let logic;
        let evolution;

        if content.contains("---") {
            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() >= 3 {
                let frontmatter = parts[1];
                let body = parts[2];
                tool = Self::extract_frontmatter(frontmatter, "Tool").or_else(|| Self::extract_frontmatter(frontmatter, "tool"));
                bridge = Self::extract_frontmatter(frontmatter, "Bridge").or_else(|| Self::extract_frontmatter(frontmatter, "bridge"));
                triggers = Self::parse_triggers(&Self::extract_frontmatter(frontmatter, "Triggers")
                    .or_else(|| Self::extract_frontmatter(frontmatter, "triggers"))
                    .unwrap_or_default());
                approval = Self::extract_frontmatter(frontmatter, "Approval")
                    .or_else(|| Self::extract_frontmatter(frontmatter, "approval"))
                    .unwrap_or_else(|| "required".to_string());
                steps = Self::extract_section(body, "Steps");
                logic = Self::extract_code_block(body, "Logic");
                evolution = Self::extract_list(body, "Evolution");
            } else {
                return Err(format!(".md format hatası: {}", source_path));
            }
        } else {
            tool = Self::extract_meta_field(content, "Tool")
                .or_else(|| Self::extract_meta_field(content, "tool"));
            bridge = Self::extract_meta_field(content, "Bridge")
                .or_else(|| Self::extract_meta_field(content, "bridge"));
            triggers = Self::parse_triggers(&Self::extract_meta_field(content, "Triggers")
                .or_else(|| Self::extract_meta_field(content, "triggers"))
                .unwrap_or_default());
            approval = Self::extract_meta_field(content, "Approval")
                .or_else(|| Self::extract_meta_field(content, "approval"))
                .unwrap_or_else(|| "required".to_string());
            steps = Self::extract_section(content, "Steps");
            logic = Self::extract_code_block(content, "Logic");
            evolution = Self::extract_list(content, "Evolution");
        }

        Ok(SkillManifesto {
            name,
            description,
            tool,
            bridge,
            triggers,
            approval,
            steps,
            logic,
            evolution,
        })
    }

    fn extract_frontmatter(frontmatter: &str, key: &str) -> Option<String> {
        for line in frontmatter.lines() {
            let trimmed = line.trim();
            if let Some(val) = trimmed.strip_prefix(&format!("- **{}:", key))
                .or_else(|| trimmed.strip_prefix(&format!("{}:", key)))
            {
                let val = val.trim().trim_matches('"').trim_matches(|c| c == '[' || c == ']').trim();
                if val.is_empty() { continue; }
                return Some(val.to_string());
            }
            let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
            if parts.len() == 2 && parts[0].trim().eq_ignore_ascii_case(key) {
                let val = parts[1].trim().trim_matches('"');
                if !val.is_empty() { return Some(val.to_string()); }
            }
        }
        None
    }

    fn extract_meta_field(content: &str, key: &str) -> Option<String> {
        let mut in_meta = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("## meta") {
                in_meta = true;
                continue;
            }
            if in_meta {
                if trimmed.starts_with("## ") {
                    break;
                }
                if let Some(val) = trimmed.strip_prefix(&format!("- **{}:", key))
                    .or_else(|| trimmed.strip_prefix(&format!("- **{}:**", key)))
                {
                    let val = val.trim().trim_matches('"').trim_matches(|c| c == '[' || c == ']').trim();
                    if val.is_empty() { continue; }
                    return Some(val.to_string());
                }
                if let Some(val) = trimmed.strip_prefix(&format!("- {}:", key))
                    .or_else(|| trimmed.strip_prefix(&format!("- {}:", key.to_lowercase())))
                {
                    let val = val.trim().trim_matches('"').trim_matches(|c| c == '[' || c == ']').trim();
                    if val.is_empty() { continue; }
                    return Some(val.to_string());
                }
                if let Some(val) = trimmed.strip_prefix(&format!("**{}:**", key)) {
                    let val = val.trim().trim_matches('"').trim_matches(|c| c == '[' || c == ']').trim();
                    if val.is_empty() { continue; }
                    return Some(val.to_string());
                }
            }
        }
        None
    }

    fn extract_title(body: &str) -> Option<String> {
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                let name = trimmed[2..].trim();
                if let Some(skill_name) = name.strip_prefix("Skill: ") {
                    return Some(skill_name.trim().to_string());
                }
                return Some(name.to_string());
            }
        }
        None
    }

    fn parse_triggers(raw: &str) -> Vec<String> {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn extract_section(body: &str, section_name: &str) -> Vec<String> {
        let mut in_section = false;
        let mut items = Vec::new();

        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") && trimmed[3..].trim().eq_ignore_ascii_case(section_name) {
                in_section = true;
                continue;
            }
            if in_section {
                if trimmed.starts_with("## ") { break; }
                if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    items.push(trimmed[2..].trim().to_string());
                } else if let Some(content) = trimmed.splitn(2, ". ").nth(1) {
                    if trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        items.push(content.trim().to_string());
                    }
                }
            }
        }
        items
    }

    fn extract_code_block(body: &str, section_name: &str) -> Option<String> {
        let mut in_section = false;
        let mut in_code = false;
        let mut code_lines = Vec::new();

        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") && trimmed[3..].trim().eq_ignore_ascii_case(section_name) {
                in_section = true;
                continue;
            }
            if in_section {
                if trimmed.starts_with("## ") { break; }
                if trimmed.starts_with("```") {
                    in_code = !in_code;
                    continue;
                }
                if in_code { code_lines.push(line.to_string()); }
            }
        }
        if code_lines.is_empty() { None } else { Some(code_lines.join("\n")) }
    }

    fn extract_list(body: &str, section_name: &str) -> Vec<String> {
        Self::extract_section(body, section_name)
    }

    pub fn manifesto_to_steps(manifesto: &SkillManifesto) -> Vec<SkillStep> {
        manifesto.steps.iter().enumerate()
            .map(|(i, desc)| SkillStep { order: (i + 1) as i32, description: desc.clone() })
            .collect()
    }
}
