pub mod parser;
pub mod registry;
pub mod executor;
pub mod evolution;
pub mod schema;
pub mod bridge;
pub mod version;
pub mod export;
pub mod market;
pub mod templates;
pub mod deps;
pub mod test_framework;
pub mod wasm_runtime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub triggers: Vec<String>,
    pub approval: String,
    pub steps: Vec<SkillStep>,
    pub logic_code: Option<String>,
    pub evolution: Vec<String>,
    pub run_count: i64,
    pub active: bool,
    pub version: i32,
    pub created_at: String,
    pub category: String,
    pub tags: Vec<String>,
    pub rating: f64,
    pub rating_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub order: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifesto {
    pub name: String,
    pub description: String,
    pub tool: Option<String>,
    pub bridge: Option<String>,
    pub triggers: Vec<String>,
    pub approval: String,
    pub steps: Vec<String>,
    pub logic: Option<String>,
    pub evolution: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionResult {
    pub skill_name: String,
    pub step_results: Vec<String>,
    pub success: bool,
    pub summary: String,
}
