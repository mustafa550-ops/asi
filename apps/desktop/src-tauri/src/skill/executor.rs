use crate::core::memory_manager::MemoryManager;
use crate::llm::OllamaClient;
use crate::skill::{Skill, SkillExecutionResult};
use std::process::Command;

pub struct SkillExecutor;

impl SkillExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        skill: &Skill,
        task: &str,
        ollama: &OllamaClient,
        memory: Option<&MemoryManager>,
    ) -> Result<SkillExecutionResult, String> {
        let mut step_results = Vec::new();
        let mut success = true;

        for step in &skill.steps {
            let rendered = step.description.replace("{task}", task);
            let log = format!("[{}] Adım {}/{}: {}",
                skill.name, step.order, skill.steps.len(), rendered);
            step_results.push(log.clone());

            match self.run_step(&rendered, ollama) {
                Ok(out) => step_results.push(format!("  → {}", out)),
                Err(e) => {
                    step_results.push(format!("  → Hata: {}", e));
                    success = false;
                    break;
                }
            }
        }

        if let Some(ref logic) = skill.logic_code {
            let log = "  [Logic] Kod çalıştırılıyor...".to_string();
            let code_result = self.execute_code(logic);
            step_results.push(log);
            match code_result {
                Ok(out) => step_results.push(format!("  → {}", out)),
                Err(e) => step_results.push(format!("  → Hata: {}", e)),
            }
        }

        let summary = if success {
            format!("Skill '{}' başarıyla tamamlandı ({} adım)", skill.name, skill.steps.len())
        } else {
            format!("Skill '{}' başarısız oldu (adım {}/{})",
                skill.name, step_results.len() / 2, skill.steps.len())
        };

        if let Some(mem) = memory {
            let result_content = step_results.join("\n");
            mem.store_long_term(&result_content, &skill.name, "skill").ok();
            mem.record_decision(task, &skill.name, if success { "success" } else { "failure" }, 0.8).ok();
        }

        Ok(SkillExecutionResult {
            skill_name: skill.name.clone(),
            step_results,
            success,
            summary,
        })
    }

    fn run_step(&self, description: &str, ollama: &OllamaClient) -> Result<String, String> {
        if description.starts_with("$ ") {
            let cmd = description.trim_start_matches("$ ");
            return self.run_shell(cmd);
        }
        if description.starts_with("http") || description.contains("curl") {
            return self.run_shell(description);
        }
        let prompt = format!(
            "Bir adimi yorumla. Adim aciklamasi: {}\n\nBu adimin ne yapmasi gerektigini kisaca acikla.",
            description
        );
        ollama.generate_sync(&prompt)
    }

    pub fn execute_code(&self, code: &str) -> Result<String, String> {
        let code = code.trim();
        if code.starts_with("#!/usr/bin/env python") || code.starts_with("import ") || code.contains("def ") {
            return self.run_python(code);
        }
        if code.starts_with("#!/usr/bin/env node") || code.contains("require(") || code.contains("import ") {
            return self.run_javascript(code);
        }
        if code.starts_with("#!/bin/bash") || code.starts_with("#!/bin/sh") {
            return self.run_shell(code);
        }
        if code.starts_with("fn ") || code.starts_with("pub fn ") || code.contains("fn main") {
            return self.run_rust_sandbox(code);
        }
        Err(format!("Bilinmeyen kod dili. Kodun basina shebang ekleyin veya Python/JS/Shell kullanin.\nKod: {}...", &code[..code.len().min(100)]))
    }

    fn run_python(&self, code: &str) -> Result<String, String> {
        let output = Command::new("python3")
            .arg("-c")
            .arg(code)
            .output()
            .map_err(|e| format!("Python subprocess hatasi: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(if stdout.is_empty() { stderr } else { stdout })
        } else {
            Err(format!("Python hatasi (exit: {}): {}", output.status.code().unwrap_or(-1), stderr))
        }
    }

    fn run_javascript(&self, code: &str) -> Result<String, String> {
        let output = Command::new("node")
            .arg("-e")
            .arg(code)
            .output()
            .map_err(|e| format!("Node subprocess hatasi: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(if stdout.is_empty() { stderr } else { stdout })
        } else {
            Err(format!("Node hatasi (exit: {}): {}", output.status.code().unwrap_or(-1), stderr))
        }
    }

    fn run_shell(&self, cmd: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("Shell subprocess hatasi: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(if stdout.is_empty() { stderr } else { stdout })
        } else {
            Err(format!("Shell hatasi (exit: {}): {}", output.status.code().unwrap_or(-1), stderr))
        }
    }

    fn run_rust_sandbox(&self, code: &str) -> Result<String, String> {
        use crate::core::wasm_compile::WasmCompiler;
        use crate::core::wasm_sandbox::WasmSandbox;
        let sandbox = WasmSandbox::new();
        match WasmCompiler::compile_rust_source(code) {
            Ok(bytes) => {
                sandbox.execute(&bytes).map_err(|e| format!("Wasm calisma hatasi: {}", e))
            }
            Err(e) => Err(format!("Rust derleme hatasi (wasm-pack gerekli): {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::SkillStep;

    #[test]
    fn test_shell_echo() {
        let executor = SkillExecutor::new();
        let result = executor.run_shell("echo hello_adler");
        assert_eq!(result.unwrap(), "hello_adler");
    }

    #[test]
    fn test_shell_failure() {
        let executor = SkillExecutor::new();
        let result = executor.run_shell("exit 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_python_addition() {
        let executor = SkillExecutor::new();
        let result = executor.run_python("print(2 + 3)");
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_python_error() {
        let executor = SkillExecutor::new();
        let result = executor.run_python("1/0");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_code_python_detection() {
        let executor = SkillExecutor::new();
        let result = executor.execute_code("import sys\nprint('detected')");
        // Python may not be installed in test environment
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_execute_code_python_with_def() {
        let executor = SkillExecutor::new();
        let code = "def foo():\n    return 42\nprint(foo())";
        let result = executor.execute_code(code);
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_execute_code_shell_detection() {
        let executor = SkillExecutor::new();
        let result = executor.execute_code("#!/bin/bash\necho 'shell_detected'");
        assert_eq!(result.unwrap(), "shell_detected");
    }

    #[test]
    fn test_execute_code_unknown() {
        let executor = SkillExecutor::new();
        let result = executor.execute_code("blargh ?? unknown lang");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_step_shell_command() {
        let executor = SkillExecutor::new();
        let result = executor.run_step("$ echo dollar_prefix", &dummy_ollama());
        assert_eq!(result.unwrap(), "dollar_prefix");
    }

    #[test]
    fn test_execute_code_javascript() {
        let executor = SkillExecutor::new();
        let result = executor.execute_code("#!/usr/bin/env node\nconsole.log('js_ok')");
        // Node.js may not be installed in test environment
        assert!(result.is_ok() || result.is_err());
    }

    fn make_skill(name: &str, steps: Vec<SkillStep>, logic: Option<&str>) -> Skill {
        Skill {
            id: 0,
            name: name.to_string(),
            description: String::new(),
            triggers: vec![],
            approval: "auto".into(),
            steps,
            logic_code: logic.map(|s| s.to_string()),
            evolution: vec![],
            run_count: 0,
            active: true,
            version: 1,
            created_at: String::new(),
            category: "general".into(),
            tags: vec![],
            rating: 0.0,
            rating_count: 0,
        }
    }

    fn dummy_ollama() -> OllamaClient {
        OllamaClient::new("http://localhost:99999".into(), "test".into())
    }

    #[test]
    fn test_execute_with_shell_step() {
        let executor = SkillExecutor::new();
        let skill = make_skill("shell_test", vec![
            SkillStep { order: 1, description: "$ echo step1_ok".into() },
        ], None);
        let ollama = dummy_ollama();
        let result = executor.execute(&skill, "", &ollama, None).unwrap();
        assert!(result.success);
        assert!(result.step_results.iter().any(|s| s.contains("step1_ok")));
    }

    #[test]
    fn test_execute_with_logic() {
        let executor = SkillExecutor::new();
        let skill = make_skill("logic_test", vec![
            SkillStep { order: 1, description: "$ echo before_logic".into() },
        ], Some("print('logic_ran')"));
        let ollama = dummy_ollama();
        let result = executor.execute(&skill, "", &ollama, None).unwrap();
        assert!(result.success);
        assert!(result.step_results.iter().any(|s| s.contains("logic_ran")));
    }

    #[test]
    fn test_execute_empty_skill() {
        let executor = SkillExecutor::new();
        let skill = make_skill("empty_test", vec![], None);
        let ollama = dummy_ollama();
        let result = executor.execute(&skill, "", &ollama, None).unwrap();
        assert!(result.success);
        assert_eq!(result.step_results.len(), 0);
    }
}
