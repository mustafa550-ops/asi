use std::collections::HashMap;

/// Module Registry — Asimile edilen modüllerin kaydı (§8.1).
#[derive(Default)]
pub struct ModuleRegistry {
    modules: HashMap<String, ModuleEntry>,
}

pub struct ModuleEntry {
    pub name: String,
    pub path: String,
    pub dependencies: Vec<String>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &str, path: &str) {
        self.modules.insert(name.into(), ModuleEntry {
            name: name.into(),
            path: path.into(),
            dependencies: Vec::new(),
        });
    }

    pub fn remove(&mut self, name: &str) -> Option<ModuleEntry> {
        self.modules.remove(name)
    }
}
