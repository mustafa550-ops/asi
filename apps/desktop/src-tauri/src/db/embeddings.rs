use rusqlite::{params, Connection};
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub content: String,
    pub source: String,
    pub category: String,
    pub score: f32,
}

pub struct EmbeddingStore {
    conn: Arc<Mutex<Connection>>,
}

impl EmbeddingStore {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn with_conn<F, T>(&self, f: F) -> Result<T, rusqlite::Error>
    where
        F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }

    pub fn store(
        &self,
        content: &str,
        embedding: &[f32],
        source: &str,
        category: &str,
    ) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            let blob: Vec<u8> = embedding
                .iter()
                .flat_map(|f| f.to_le_bytes())
                .collect();
            let dim = embedding.len() as i32;
            conn.execute(
                "INSERT INTO embeddings (content, embedding, source, category, embedding_dim)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![content, blob, source, category, dim],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    pub fn search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<SearchResult>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, content, source, category, embedding
                 FROM embeddings WHERE embedding IS NOT NULL",
            )?;

            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let content: String = row.get(1)?;
                let source: String = row.get(2)?;
                let category: String = row.get(3)?;
                let blob: Vec<u8> = row.get(4)?;
                Ok((id, content, source, category, blob))
            })?;

            let mut results: Vec<SearchResult> = Vec::new();
            for row in rows {
                let (_id, content, source, category, blob) = row?;
                if blob.len() % 4 != 0 {
                    continue;
                }
                let stored: Vec<f32> = blob
                    .chunks_exact(4)
                    .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                    .collect();
                let score = cosine_similarity(query_embedding, &stored);
                if score > 0.0 {
                    results.push(SearchResult {
                        content,
                        source,
                        category,
                        score,
                    });
                }
            }

            results
                .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
            results.truncate(limit);
            Ok(results)
        })
    }

    pub fn keyword_search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT e.content, e.source, e.category, rank
                 FROM embeddings_fts f
                 JOIN embeddings e ON e.id = f.rowid
                 WHERE embeddings_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            )?;

            let rows = stmt.query_map(params![query, limit as i64], |row| {
                Ok(SearchResult {
                    content: row.get(0)?,
                    source: row.get(1)?,
                    category: row.get(2)?,
                    score: row.get::<_, f64>(3).unwrap_or(0.0) as f32,
                })
            })?;

            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn count_all(&self) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row("SELECT COUNT(*) FROM embeddings", [], |row| row.get(0))
        })
    }

    pub fn count_by_category(&self, category: &str) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row(
                "SELECT COUNT(*) FROM embeddings WHERE category = ?1",
                params![category],
                |row| row.get(0),
            )
        })
    }

    pub fn delete_older_than(&self, days: i64) -> Result<usize, rusqlite::Error> {
        let modifier = if days >= 0 {
            format!("-{} days", days)
        } else {
            format!("{} days", days)
        };
        self.with_conn(|conn| {
            conn.execute(
                "DELETE FROM embeddings WHERE timestamp < datetime('now', ?1)",
                params![modifier],
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> EmbeddingStore {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        crate::db::schema::run_migrations(&conn).unwrap();
        EmbeddingStore::new(Arc::new(Mutex::new(conn)))
    }

    fn dummy_embedding(value: f32, dim: usize) -> Vec<f32> {
        vec![value; dim]
    }

    #[test]
    fn store_and_count() {
        let store = setup();
        let id = store.store("test content", &dummy_embedding(0.5, 4), "test.md", "general").unwrap();
        assert!(id > 0);
        assert_eq!(store.count_by_category("general").unwrap(), 1);
    }

    #[test]
    fn search_returns_matching() {
        let store = setup();
        store.store("apple fruit", &dummy_embedding(0.9, 4), "a.md", "food").unwrap();
        store.store("car engine", &dummy_embedding(0.1, 4), "b.md", "tech").unwrap();
        let results = store.search(&dummy_embedding(0.8, 4), 5).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].score > 0.0);
    }

    #[test]
    fn search_empty_when_no_match() {
        let store = setup();
        store.store("hello", &dummy_embedding(1.0, 4), "x.md", "test").unwrap();
        let results = store.search(&vec![0.0; 4], 5).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn search_respects_limit() {
        let store = setup();
        for i in 0..5 {
            store.store(&format!("item {}", i), &dummy_embedding(0.5, 4), "x.md", "test").unwrap();
        }
        let results = store.search(&dummy_embedding(0.5, 4), 2).unwrap();
        assert!(results.len() <= 2);
    }

    #[test]
    fn keyword_search_via_fts() {
        let store = setup();
        store.store("bottom fishing signal detected", &dummy_embedding(0.5, 4), "report.md", "market").unwrap();
        store.store("weather is nice today", &dummy_embedding(0.3, 4), "weather.md", "general").unwrap();
        let results = store.keyword_search("fishing", 10).unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.content.contains("fishing")));
    }

    #[test]
    fn keyword_search_no_match() {
        let store = setup();
        store.store("test data", &dummy_embedding(0.5, 4), "t.md", "test").unwrap();
        let results = store.keyword_search("nonexistent", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn count_by_category_zero_when_empty() {
        let store = setup();
        assert_eq!(store.count_by_category("missing").unwrap(), 0);
    }

    #[test]
    fn count_by_category_multiple() {
        let store = setup();
        store.store("a", &dummy_embedding(0.5, 4), "src", "alpha").unwrap();
        store.store("b", &dummy_embedding(0.5, 4), "src", "beta").unwrap();
        store.store("c", &dummy_embedding(0.5, 4), "src", "alpha").unwrap();
        assert_eq!(store.count_by_category("alpha").unwrap(), 2);
        assert_eq!(store.count_by_category("beta").unwrap(), 1);
    }

    #[test]
    fn delete_older_than_removes_old_entries() {
        let store = setup();
        // Insert entry with an explicitly old timestamp via raw SQL
        let emb: Vec<u8> = dummy_embedding(0.5, 4).iter().flat_map(|f| f.to_le_bytes()).collect();
        store.with_conn(|conn| {
            conn.execute(
                "INSERT INTO embeddings (content, embedding, source, category, embedding_dim, timestamp) VALUES (?1, ?2, 'old.md', 'test', 4, datetime('now', '-30 days'))",
                params!["old data", emb],
            )
        }).unwrap();
        // Delete entries older than 1 day
        let deleted = store.delete_older_than(1).unwrap();
        assert_eq!(deleted, 1);
    }

    #[test]
    fn cosine_similarity_identical() {
        let v = vec![1.0, 2.0, 3.0];
        let s = cosine_similarity(&v, &v);
        assert!((s - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn cosine_similarity_empty_returns_zero() {
        assert_eq!(cosine_similarity(&[], &[]), 0.0);
    }

    #[test]
    fn cosine_similarity_different_lengths_returns_zero() {
        assert_eq!(cosine_similarity(&[1.0], &[1.0, 2.0]), 0.0);
    }

    #[test]
    fn stores_blob_with_correct_dim() {
        let store = setup();
        let emb = dummy_embedding(0.25, 8);
        store.store("dim test", &emb, "dim.md", "test").unwrap();
        let results = store.search(&emb, 1).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_sorts_by_score_descending() {
        let store = setup();
        store.store("close", &dummy_embedding(0.9, 4), "a.md", "test").unwrap();
        store.store("far", &dummy_embedding(0.1, 4), "b.md", "test").unwrap();
        let results = store.search(&dummy_embedding(0.9, 4), 5).unwrap();
        if results.len() >= 2 {
            assert!(results[0].score >= results[1].score);
        }
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a * norm_b)).max(0.0)
}
