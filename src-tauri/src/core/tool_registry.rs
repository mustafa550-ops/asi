use std::collections::HashMap;

/// Tool Registry — Yetenek envanteri (§9.1).
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, ToolEntry>,
}

pub struct ToolEntry {
    pub name: String,
    pub description: String,
    pub approval_required: bool,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &str, description: &str, approval_required: bool) {
        self.tools.insert(name.into(), ToolEntry {
            name: name.into(),
            description: description.into(),
            approval_required,
        });
    }

    pub fn list(&self) -> Vec<&ToolEntry> {
        self.tools.values().collect()
    }
}
