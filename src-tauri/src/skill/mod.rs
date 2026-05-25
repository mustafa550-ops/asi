pub mod parser;
pub mod registry;
pub mod executor;
pub mod evolution;

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
