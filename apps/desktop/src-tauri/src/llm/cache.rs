use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry {
    response: String,
    created: Instant,
}

pub struct LLMCache {
    cache: HashMap<String, CacheEntry>,
    max_entries: usize,
    ttl: Duration,
    hits: u64,
    misses: u64,
}

impl LLMCache {
    pub fn new(max_entries: usize, ttl_secs: u64) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            ttl: Duration::from_secs(ttl_secs),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, prompt: &str) -> Option<String> {
        let key = self.normalize(prompt);
        self.evict_expired();
        match self.cache.get_mut(&key) {
            Some(entry) => {
                self.hits += 1;
                entry.created = Instant::now();
                Some(entry.response.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    pub fn set(&mut self, prompt: &str, response: String) {
        let key = self.normalize(prompt);
        if self.cache.len() >= self.max_entries {
            let oldest_key = self.cache.iter()
                .min_by_key(|(_, e)| e.created)
                .map(|(k, _)| k.clone());
            if let Some(oldest) = oldest_key {
                self.cache.remove(&oldest);
            }
        }
        self.cache.insert(key, CacheEntry {
            response,
            created: Instant::now(),
        });
    }

    fn evict_expired(&mut self) {
        let now = Instant::now();
        self.cache.retain(|_, e| now.duration_since(e.created) < self.ttl);
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { return 0.0; }
        self.hits as f64 / total as f64
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    fn normalize(&self, prompt: &str) -> String {
        prompt.trim().to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_miss() {
        let mut cache = LLMCache::new(10, 60);
        assert_eq!(cache.get("test"), None);
    }

    #[test]
    fn test_cache_hit() {
        let mut cache = LLMCache::new(10, 60);
        cache.set("test", "response1".into());
        assert_eq!(cache.get("test"), Some("response1".into()));
    }

    #[test]
    fn test_cache_normalization() {
        let mut cache = LLMCache::new(10, 60);
        cache.set("Hello World", "resp".into());
        assert_eq!(cache.get("hello world"), Some("resp".into()));
        assert_eq!(cache.get("  Hello World  "), Some("resp".into()));
    }

    #[test]
    fn test_cache_max_entries() {
        let mut cache = LLMCache::new(3, 60);
        cache.set("a", "1".into());
        cache.set("b", "2".into());
        cache.set("c", "3".into());
        cache.set("d", "4".into());
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get("a"), None);
    }
}
