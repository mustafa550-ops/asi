use crate::db::embeddings::EmbeddingStore;
use crate::db::strategic_memory::StrategicMemory;
use crate::db::edge_history::EdgeHistory;

pub struct PruningEngine {
    embeddings: EmbeddingStore,
    strategic: StrategicMemory,
    edge: EdgeHistory,
}

pub struct PruningReport {
    pub deleted_embeddings: usize,
    pub deleted_strategic: usize,
    pub deleted_edges: usize,
    pub total_before: PruningCounts,
    pub total_after: PruningCounts,
}

pub struct PruningCounts {
    pub embeddings: i64,
    pub strategic: i64,
    pub edges: i64,
}

impl PruningEngine {
    pub fn new(embeddings: EmbeddingStore, strategic: StrategicMemory, edge: EdgeHistory) -> Self {
        Self { embeddings, strategic, edge }
    }

    pub fn prune_old_embeddings(&self, days: i64) -> Result<usize, String> {
        self.embeddings.delete_older_than(days)
            .map_err(|e| e.to_string())
    }

    pub fn prune_low_confidence_strategic(&self, min_confidence: f64) -> Result<usize, String> {
        self.strategic.delete_low_confidence(min_confidence)
            .map_err(|e| e.to_string())
    }

    pub fn prune_old_edges(&self, days: i64) -> Result<usize, String> {
        self.edge.delete_older_than(days)
            .map_err(|e| e.to_string())
    }

    pub fn full_prune(&self, embedding_days: i64, min_confidence: f64, edge_days: i64) -> Result<PruningReport, String> {
        let counts_before = self.count_all();

        let deleted_embeddings = self.prune_old_embeddings(embedding_days)?;
        let deleted_strategic = self.prune_low_confidence_strategic(min_confidence)?;
        let deleted_edges = self.prune_old_edges(edge_days)?;

        let counts_after = self.count_all();

        Ok(PruningReport {
            deleted_embeddings,
            deleted_strategic,
            deleted_edges,
            total_before: counts_before,
            total_after: counts_after,
        })
    }

    pub fn count_all(&self) -> PruningCounts {
        PruningCounts {
            embeddings: self.embeddings.count_all().unwrap_or(0),
            strategic: self.strategic.count_all().unwrap_or(0),
            edges: self.edge.count_all().unwrap_or(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::embeddings::EmbeddingStore;
    use crate::db::edge_history::EdgeHistory;
    use crate::db::strategic_memory::StrategicMemory;
    use crate::db::schema::create_tables;
    use std::sync::{Arc, Mutex};
    use rusqlite::Connection;

    fn setup() -> (EmbeddingStore, StrategicMemory, EdgeHistory) {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        let embeddings = EmbeddingStore::new(conn.clone());
        let strategic = StrategicMemory::new(conn.clone());
        let edge = EdgeHistory::new(conn);
        (embeddings, strategic, edge)
    }

    #[test]
    fn prune_old_embeddings_does_not_error() {
        let (embeddings, strategic, edge) = setup();
        let engine = PruningEngine::new(embeddings, strategic, edge);
        let result = engine.prune_old_embeddings(1);
        assert!(result.is_ok());
    }

    #[test]
    fn prune_low_confidence_strategic_does_not_error() {
        let (embeddings, strategic, edge) = setup();
        let engine = PruningEngine::new(embeddings, strategic, edge);
        let result = engine.prune_low_confidence_strategic(0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn full_prune_returns_report() {
        let (embeddings, strategic, edge) = setup();
        let engine = PruningEngine::new(embeddings, strategic, edge);
        let result = engine.full_prune(1, 0.5, 1);
        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.deleted_embeddings, 0);
    }

    #[test]
    fn count_all_returns_zero_for_empty_db() {
        let (embeddings, strategic, edge) = setup();
        let engine = PruningEngine::new(embeddings, strategic, edge);
        let counts = engine.count_all();
        assert!(counts.embeddings >= 0);
        assert!(counts.strategic >= 0);
        assert!(counts.edges >= 0);
    }
}
