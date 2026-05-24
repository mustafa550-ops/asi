use crate::core::memory_manager::MemoryManager;

pub struct ContextManager {
    buffer: Vec<ContextEntry>,
    max_tokens: usize,
    token_count: usize,
}

#[derive(Debug, Clone)]
pub struct ContextEntry {
    pub role: String,
    pub content: String,
    pub tokens: usize,
    pub source: Option<String>,
}

impl ContextManager {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_tokens,
            token_count: 0,
        }
    }

    pub fn push(&mut self, role: &str, content: &str) {
        let tokens = estimate_tokens(content);
        let entry = ContextEntry {
            role: role.to_string(),
            content: content.to_string(),
            tokens,
            source: None,
        };
        self.token_count += tokens;
        self.buffer.push(entry);
        self.evict_to_fit();
    }

    pub fn push_with_source(&mut self, role: &str, content: &str, source: &str) {
        let tokens = estimate_tokens(content);
        let entry = ContextEntry {
            role: role.to_string(),
            content: content.to_string(),
            tokens,
            source: Some(source.to_string()),
        };
        self.token_count += tokens;
        self.buffer.push(entry);
        self.evict_to_fit();
    }

    pub fn build_prompt(&self) -> String {
        self.buffer
            .iter()
            .map(|e| {
                let mut line = format!("[{}]: {}", e.role, e.content);
                if let Some(ref src) = e.source {
                    line.push_str(&format!("\n   (source: {})", src));
                }
                line
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    pub fn enrich_from_memory(
        &mut self,
        memory: &MemoryManager,
        query: &str,
        limit: usize,
    ) -> Result<(), String> {
        let results = memory.semantic_search(query, limit)?;
        for r in &results {
            let label = format!("[memory: {} | {:.2}]", r.category, r.score);
            self.push_with_source("memory", &r.content, &label);
        }
        Ok(())
    }

    pub fn last_n(&self, n: usize) -> Vec<&ContextEntry> {
        self.buffer.iter().rev().take(n).collect()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.token_count = 0;
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn token_usage(&self) -> (usize, usize) {
        (self.token_count, self.max_tokens)
    }

    fn evict_to_fit(&mut self) {
        while self.token_count > self.max_tokens && self.buffer.len() > 1 {
            if let Some(removed) = self.buffer.first() {
                self.token_count = self.token_count.saturating_sub(removed.tokens);
            }
            self.buffer.remove(0);
        }
    }
}

fn estimate_tokens(text: &str) -> usize {
    (text.len() / 4).max(1)
}
