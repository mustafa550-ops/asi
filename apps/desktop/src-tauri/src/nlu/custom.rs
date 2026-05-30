use std::collections::HashMap;
use crate::nlu::intent::Intent;

pub struct CustomIntent {
    pub name: String,
    pub keywords: Vec<String>,
    pub intent: Intent,
}

pub struct CustomIntentRegistry {
    intents: HashMap<String, CustomIntent>,
}

impl CustomIntentRegistry {
    pub fn new() -> Self {
        Self { intents: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, keywords: Vec<String>, intent: Intent) -> Result<(), String> {
        if self.intents.contains_key(name) {
            return Err(format!("Custom intent '{}' zaten kayitli", name));
        }
        self.intents.insert(name.to_string(), CustomIntent {
            name: name.to_string(),
            keywords,
            intent,
        });
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), String> {
        let _ = self.intents.remove(name);
        Ok(())
    }

    pub fn match_custom(&self, text: &str) -> Option<(String, &Intent)> {
        let lower = text.to_lowercase();
        for (name, custom) in &self.intents {
            if custom.keywords.iter().any(|k| lower.contains(k)) {
                return Some((name.clone(), &custom.intent));
            }
        }
        None
    }

    pub fn list(&self) -> Vec<&CustomIntent> {
        self.intents.values().collect()
    }
}
