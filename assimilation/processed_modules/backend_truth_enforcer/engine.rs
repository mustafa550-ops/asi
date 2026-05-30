use super::hooks::RuntimeHooks;
use super::validators::{
    backend_validator::BackendValidator,
    contract_validator::ContractValidator,
    frontend_validator::FrontendValidator,
};
use super::self_healing::SelfHealingLoop;

pub struct GovernorEngine {
    pub allow_mock: bool,
    pub enforce_backend: bool,
    pub max_retries: u8,
}

impl GovernorEngine {
    pub fn new() -> Self {
        Self {
            allow_mock: false,
            enforce_backend: true,
            max_retries: 3,
        }
    }

    /// Evaluates generated source code before writing to disk
    pub fn evaluate_generation(&self, file_path: &str, content: &str) -> Result<(), String> {
        // 1. AST/Regex check for frontend
        if file_path.ends_with(".tsx") || file_path.ends_with(".ts") {
            FrontendValidator::detect_mocks(content)?;
        }

        // 2. Validate backend route existence if it is a frontend IPC call
        BackendValidator::verify_routes(content)?;

        // 3. Contract matching
        ContractValidator::verify_contracts(content)?;

        Ok(())
    }

    /// Entry point for orchestrator to pass generated content
    pub fn intercept(&self, file_path: &str, content: &str) -> Result<(), String> {
        RuntimeHooks::pre_generation();
        
        match self.evaluate_generation(file_path, content) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("[Governor] Verification failed: {}", e);
                // Trigger Self-Healing
                let repaired_content = SelfHealingLoop::attempt_repair(content, &e, self.max_retries)?;
                println!("[Governor] Self-healing successful");
                // Recurse with repaired content or just accept if valid
                RuntimeHooks::post_generation_repair();
                Ok(())
            }
        }
    }
}
