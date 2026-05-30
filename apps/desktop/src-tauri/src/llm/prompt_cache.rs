use std::collections::HashMap;

pub struct PromptCache {
    cache: HashMap<String, String>,
    max_entries: usize,
}

impl PromptCache {
    pub fn new(max_entries: usize) -> Self {
        Self { cache: HashMap::new(), max_entries }
    }

    pub fn get_or_create(&mut self, key: &str, creator: impl FnOnce() -> String) -> String {
        if let Some(cached) = self.cache.get(key) {
            return cached.clone();
        }
        let value = creator();
        if self.cache.len() >= self.max_entries {
            if let Some(oldest) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest);
            }
        }
        self.cache.insert(key.to_string(), value.clone());
        value
    }

    /// Anthropic prompt caching prefix for repeated system prompts
    pub fn cache_control_header() -> &'static str {
        "{\"type\": \"ephemeral\"}"
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
