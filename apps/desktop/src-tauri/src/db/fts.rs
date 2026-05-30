use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct FullTextSearch {
    conn: Arc<Mutex<Connection>>,
}

pub struct FtsResult {
    pub id: i64,
    pub content: String,
    pub source: String,
    pub category: String,
    pub rank: f64,
}

pub struct FtsHighlight {
    pub id: i64,
    pub content: String,
    pub source: String,
    pub category: String,
    pub highlighted: String,
}

impl FullTextSearch {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn with_conn<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&Connection) -> Result<T, String>,
    {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        f(&conn)
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<FtsResult>, String> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT e.id, e.content, e.source, e.category, rank
                 FROM embeddings_fts f
                 JOIN embeddings e ON e.id = f.rowid
                 WHERE embeddings_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            ).map_err(|e| e.to_string())?;
            let rows = stmt.query_map(params![query, limit as i64], |row| {
                Ok(FtsResult {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    source: row.get(2)?,
                    category: row.get(3)?,
                    rank: row.get::<_, f64>(4).unwrap_or(0.0),
                })
            }).map_err(|e| e.to_string())?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row.map_err(|e| e.to_string())?);
            }
            Ok(results)
        })
    }

    pub fn search_highlighted(&self, query: &str, limit: usize) -> Result<Vec<FtsHighlight>, String> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT e.id, e.content, e.source, e.category,
                        highlight(embeddings_fts, 0, '<mark>', '</mark>')
                 FROM embeddings_fts f
                 JOIN embeddings e ON e.id = f.rowid
                 WHERE embeddings_fts MATCH ?1
                 ORDER BY rank
                 LIMIT ?2",
            ).map_err(|e| e.to_string())?;
            let rows = stmt.query_map(params![query, limit as i64], |row| {
                Ok(FtsHighlight {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    source: row.get(2)?,
                    category: row.get(3)?,
                    highlighted: row.get(4)?,
                })
            }).map_err(|e| e.to_string())?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row.map_err(|e| e.to_string())?);
            }
            Ok(results)
        })
    }

    pub fn rebuild_index(&self) -> Result<(), String> {
        self.with_conn(|conn| {
            conn.execute_batch("INSERT INTO embeddings_fts(embeddings_fts) VALUES('rebuild');")
                .map_err(|e| e.to_string())
        })
    }

    pub fn count_entries(&self) -> Result<i64, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row("SELECT COUNT(*) FROM embeddings_fts", [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> (FullTextSearch, Arc<Mutex<Connection>>) {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        // Insert embedding data so FTS triggers populate
        {
            let c = conn.lock().unwrap();
            c.execute("INSERT INTO embeddings (content, embedding, source, category) VALUES ('test apple content', x'00000000', 'src/a.md', 'food'), ('banana fruit', x'00000000', 'src/b.md', 'food')", []).unwrap();
        }
        (FullTextSearch::new(conn.clone()), conn)
    }

    #[test]
    fn search_returns_matching() {
        let (fts, _) = setup();
        let results = fts.search("apple", 10).unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.content.contains("apple")));
    }

    #[test]
    fn search_no_match_returns_empty() {
        let (fts, _) = setup();
        let results = fts.search("nonexistent", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn search_respects_limit() {
        let (fts, _) = setup();
        let results = fts.search("content", 1).unwrap();
        assert!(results.len() <= 1);
    }

    #[test]
    fn search_highlighted_returns_markup() {
        let (fts, _) = setup();
        let results = fts.search_highlighted("apple", 10).unwrap();
        assert!(!results.is_empty());
        assert!(results[0].highlighted.contains("<mark>"));
    }

    #[test]
    fn rebuild_index_does_not_error() {
        let (fts, _) = setup();
        assert!(fts.rebuild_index().is_ok());
    }

    #[test]
    fn count_entries_returns_number() {
        let (fts, _) = setup();
        let count = fts.count_entries().unwrap();
        assert!(count >= 0);
    }
}
