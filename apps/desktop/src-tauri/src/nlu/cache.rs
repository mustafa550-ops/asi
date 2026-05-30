use std::collections::HashMap;
use crate::nlu::intent::Intent;

pub struct IntentCache {
    cache: HashMap<String, Intent>,
    max_entries: usize,
}

impl IntentCache {
    pub fn new(max_entries: usize) -> Self {
        Self { cache: HashMap::new(), max_entries }
    }

    pub fn get(&self, text: &str) -> Option<&Intent> {
        let key = text.to_lowercase().trim().to_string();
        self.cache.get(&key)
    }

    pub fn set(&mut self, text: &str, intent: Intent) {
        let key = text.to_lowercase().trim().to_string();
        if self.cache.len() >= self.max_entries {
            if let Some(oldest) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest);
            }
        }
        self.cache.insert(key, intent);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }
}
