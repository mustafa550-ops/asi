use std::collections::HashMap;

/// Keyring — API anahtarları güvenli depolama (§11).
#[derive(Default)]
pub struct Keyring {
    keys: HashMap<String, String>,
}

impl Keyring {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.keys.insert(name.into(), value.into());
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.keys.get(name)
    }
}
