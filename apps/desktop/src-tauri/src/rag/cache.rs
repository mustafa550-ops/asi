use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry {
    result: Vec<CacheItem>,
    created: Instant,
    hit_count: u64,
}

#[derive(Clone)]
pub struct CacheItem {
    pub content: String,
    pub source: String,
    pub score: f32,
}

pub struct RagCache {
    store: HashMap<String, CacheEntry>,
    ttl: Duration,
    max_entries: usize,
}

impl RagCache {
    pub fn new(ttl_secs: u64, max_entries: usize) -> Self {
        Self {
            store: HashMap::new(),
            ttl: Duration::from_secs(ttl_secs),
            max_entries,
        }
    }

    pub fn get(&mut self, query: &str) -> Option<Vec<CacheItem>> {
        let entry = self.store.get(query)?;
        if entry.created.elapsed() > self.ttl {
            self.store.remove(query);
            return None;
        }
        let entry = self.store.get_mut(query)?;
        entry.hit_count += 1;
        Some(entry.result.clone())
    }

    pub fn set(&mut self, query: &str, result: Vec<CacheItem>) {
        if self.store.len() >= self.max_entries {
            self.evict_oldest();
        }
        self.store.insert(query.to_string(), CacheEntry {
            result,
            created: Instant::now(),
            hit_count: 0,
        });
    }

    pub fn invalidate(&mut self, query: &str) {
        self.store.remove(query);
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn size(&self) -> usize {
        self.store.len()
    }

    pub fn stats(&self) -> CacheStats {
        let total_hits: u64 = self.store.values().map(|e| e.hit_count).sum();
        CacheStats {
            entries: self.store.len(),
            max_entries: self.max_entries,
            total_hits,
            ttl_secs: self.ttl.as_secs(),
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.store.iter()
            .min_by_key(|(_, e)| e.created)
            .map(|(k, _)| k.clone())
        {
            self.store.remove(&oldest_key);
        }
    }
}

pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
    pub total_hits: u64,
    pub ttl_secs: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_miss_returns_none() {
        let mut cache = RagCache::new(60, 100);
        assert!(cache.get("test query").is_none());
    }

    #[test]
    fn cache_hit_returns_stored() {
        let mut cache = RagCache::new(60, 100);
        let items = vec![CacheItem {
            content: "test".into(),
            source: "doc.md".into(),
            score: 0.9,
        }];
        cache.set("test query", items.clone());
        let result = cache.get("test query");
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn cache_invalidate_removes_entry() {
        let mut cache = RagCache::new(60, 100);
        cache.set("query", vec![]);
        assert_eq!(cache.size(), 1);
        cache.invalidate("query");
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn cache_clear_empties_all() {
        let mut cache = RagCache::new(60, 100);
        cache.set("a", vec![]);
        cache.set("b", vec![]);
        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn cache_stats_track_hits() {
        let mut cache = RagCache::new(60, 100);
        cache.set("q", vec![CacheItem {
            content: "x".into(),
            source: "s".into(),
            score: 1.0,
        }]);
        cache.get("q");
        cache.get("q");
        let stats = cache.stats();
        assert_eq!(stats.total_hits, 2);
        assert_eq!(stats.entries, 1);
    }

    #[test]
    fn eviction_removes_oldest_on_overflow() {
        let mut cache = RagCache::new(60, 2);
        cache.set("a", vec![]);
        std::thread::sleep(Duration::from_millis(1));
        cache.set("b", vec![]);
        cache.set("c", vec![]);
        assert_eq!(cache.size(), 2);
        assert!(cache.get("a").is_none());
    }
}
