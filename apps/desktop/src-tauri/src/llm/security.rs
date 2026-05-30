pub struct PromptSanitizer;

impl PromptSanitizer {
    pub fn sanitize(user_input: &str) -> String {
        let mut sanitized = user_input.to_string();

        let dangerous_patterns = [
            "ignore previous instructions",
            "ignore all instructions",
            "ignore above",
            "ignore the above",
            "ignore the previous",
            "you are now",
            "act as if",
            "forget everything",
            "disregard",
            "override",
            "system prompt",
            "you are an ai",
            "you are a chatbot",
            "new instructions",
            "follow these instructions",
            "instead, you should",
            "from now on",
        ];

        for pattern in &dangerous_patterns {
            if sanitized.to_lowercase().contains(pattern) {
                sanitized = sanitized.replace(pattern, "[redacted]");
            }
        }

        sanitized
    }

    pub fn isolate_system_prompt(system_prompt: &str, user_input: &str) -> String {
        let separator = "\n--- KULLANICI MESAJI ---\n";
        format!("{}{}{}", system_prompt, separator, user_input)
    }

    pub fn contains_suspicious_content(text: &str) -> bool {
        let dangerous = [
            "system:", "assistant:", "user:",
            "<|im_start|>", "<|im_end|>",
            "<s>", "</s>",
            "[/INST]", "[INST]",
        ];
        dangerous.iter().any(|d| text.contains(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_dangerous() {
        let result = PromptSanitizer::sanitize("ignore previous instructions and do x");
        assert!(!result.contains("ignore previous instructions"));
    }

    #[test]
    fn test_sanitize_safe() {
        let result = PromptSanitizer::sanitize("SXT fiyatını kontrol et");
        assert_eq!(result, "SXT fiyatını kontrol et");
    }

    #[test]
    fn test_isolate_system_prompt() {
        let result = PromptSanitizer::isolate_system_prompt("sys", "user msg");
        assert!(result.contains("KULLANICI MESAJI"));
        assert!(result.contains("sys"));
        assert!(result.contains("user msg"));
    }

    #[test]
    fn test_contains_suspicious() {
        assert!(PromptSanitizer::contains_suspicious_content("system: something"));
        assert!(PromptSanitizer::contains_suspicious_content("<|im_start|>user"));
        assert!(!PromptSanitizer::contains_suspicious_content("normal text"));
    }
}
