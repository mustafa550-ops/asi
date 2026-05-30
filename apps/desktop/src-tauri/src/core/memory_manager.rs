use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;

use crate::db::embeddings::{EmbeddingStore, SearchResult};
use crate::db::edge_history::{EdgeHistory, EdgeRecord};
use crate::db::strategic_memory::{StrategicMemory, StrategicRecord};
use crate::llm::OllamaClient;

pub struct MemoryManager {
    pub embeddings: EmbeddingStore,
    pub edge_history: EdgeHistory,
    pub strategic: StrategicMemory,
    ollama: OllamaClient,
    short_term: Vec<String>,
    max_short_term: usize,
}

impl MemoryManager {
    pub fn new(
        conn: Arc<Mutex<Connection>>,
        ollama: OllamaClient,
    ) -> Self {
        let embeddings = EmbeddingStore::new(Arc::clone(&conn));
        let edge_history = EdgeHistory::new(Arc::clone(&conn));
        let strategic = StrategicMemory::new(Arc::clone(&conn));

        Self {
            embeddings,
            edge_history,
            strategic,
            ollama,
            short_term: Vec::new(),
            max_short_term: 50,
        }
    }

    pub fn push_short_term(&mut self, entry: String) {
        self.short_term.push(entry);
        if self.short_term.len() > self.max_short_term {
            self.short_term.remove(0);
        }
    }

    pub fn get_short_term_context(&self) -> String {
        self.short_term.join("\n")
    }

    pub fn clear_short_term(&mut self) {
        self.short_term.clear();
    }

    pub fn store_long_term(
        &self,
        content: &str,
        source: &str,
        category: &str,
    ) -> Result<i64, String> {
        let embedding = self.ollama.embedding_sync(content)?;
        self.embeddings
            .store(content, &embedding, source, category)
            .map_err(|e| format!("DB store failed: {}", e))
    }

    pub fn semantic_search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
        let embedding = self.ollama.embedding_sync(query)?;
        self.embeddings
            .search(&embedding, limit)
            .map_err(|e| format!("Search failed: {}", e))
    }

    pub fn keyword_search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
        self.embeddings
            .keyword_search(query, limit)
            .map_err(|e| format!("Keyword search failed: {}", e))
    }

    pub fn hybrid_search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, String> {
        let semantic = self.semantic_search(query, limit)?;
        if semantic.len() >= limit {
            return Ok(semantic);
        }
        let keyword = self.keyword_search(query, limit)?;

        let mut combined: Vec<SearchResult> = Vec::with_capacity(limit);
        let seen: std::collections::HashSet<String> =
            semantic.iter().map(|r| r.content.clone()).collect();

        for r in semantic {
            combined.push(r);
        }
        for r in keyword {
            if !seen.contains(&r.content) && combined.len() < limit {
                combined.push(r);
            }
        }
        Ok(combined)
    }

    pub fn record_decision(
        &self,
        context: &str,
        decision: &str,
        outcome: &str,
        confidence: f64,
    ) -> Result<i64, String> {
        self.strategic
            .record(context, decision, outcome, confidence)
            .map_err(|e| format!("Record failed: {}", e))
    }

    pub fn get_similar_decisions(
        &self,
        context: &str,
        limit: usize,
    ) -> Result<Vec<StrategicRecord>, String> {
        self.strategic
            .query_by_context(context, limit)
            .map_err(|e| format!("Query failed: {}", e))
    }

    pub fn get_high_confidence_decisions(
        &self,
        min_confidence: f64,
        limit: usize,
    ) -> Result<Vec<StrategicRecord>, String> {
        self.strategic
            .get_high_confidence(min_confidence, limit)
            .map_err(|e| format!("Query failed: {}", e))
    }

    pub fn add_edge(
        &self,
        parent_id: i64,
        child_id: i64,
        edge_type: &str,
        diff: &str,
    ) -> Result<i64, String> {
        self.edge_history
            .add_edge(parent_id, child_id, edge_type, diff)
            .map_err(|e| format!("Edge add failed: {}", e))
    }

    pub fn get_ancestors(&self, node_id: i64, depth: usize) -> Result<Vec<EdgeRecord>, String> {
        self.edge_history
            .traverse_ancestors(node_id, depth)
            .map_err(|e| format!("Traverse failed: {}", e))
    }

    pub fn get_descendants(&self, node_id: i64, depth: usize) -> Result<Vec<EdgeRecord>, String> {
        self.edge_history
            .traverse_descendants(node_id, depth)
            .map_err(|e| format!("Traverse failed: {}", e))
    }

    pub fn apply_time_decay(&self, half_life_days: i64) -> Result<usize, String> {
        self.strategic
            .apply_time_decay(half_life_days)
            .map_err(|e| format!("Decay failed: {}", e))
    }

    pub fn index_content(&self, content: &str, source: &str, category: &str) -> Result<i64, String> {
        self.store_long_term(content, source, category)
    }

    pub fn consolidate(&self) -> Result<(), String> {
        let count = self.apply_time_decay(30)?;
        log::info!("Memory consolidation: {} records decayed", count);
        Ok(())
    }
}
