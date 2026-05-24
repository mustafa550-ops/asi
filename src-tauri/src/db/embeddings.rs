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
        self.with_conn(|conn| {
            conn.execute(
                "DELETE FROM embeddings WHERE timestamp < datetime('now', ?1)",
                params![format!("-{} days", days)],
            )
        })
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
