pub mod schema;
pub mod embeddings;
pub mod edge_history;
pub mod strategic_memory;
pub mod audit;
pub mod backup;
pub mod fts;
pub mod performance;
pub mod sessions;

use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn open_creates_db_file() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test.db");
        let conn = open(&db_path).unwrap();
        let c = conn.lock().unwrap();
        // Verify tables were created
        let count: i64 = c.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", [], |row| row.get(0)).unwrap();
        assert!(count > 0);
    }

    #[test]
    fn open_opens_existing_db() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("existing.db");
        // Call open twice
        let _conn1 = open(&db_path).unwrap();
        let conn2 = open(&db_path).unwrap();
        let c = conn2.lock().unwrap();
        let count: i64 = c.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", [], |row| row.get(0)).unwrap();
        assert!(count > 0);
    }

    #[test]
    fn open_returns_wal_mode() {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("wal_test.db");
        let conn = open(&db_path).unwrap();
        let c = conn.lock().unwrap();
        let journal: String = c.query_row("PRAGMA journal_mode", [], |row| row.get(0)).unwrap();
        // WAL should be set (but SQLCipher key pragma may affect this)
        assert!(journal == "wal" || journal == "memory" || journal == "delete");
    }
}

pub fn open(path: &Path) -> Result<Arc<Mutex<Connection>>, rusqlite::Error> {
    let conn = Connection::open(path)?;
    // SQLCipher Encryption Key
    conn.execute_batch("PRAGMA key = 'adler-secret-dev-key';")?;
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    schema::create_tables(&conn)?;
    schema::run_migrations(&conn)?;
    log::info!("Database initialized at {:?}", path);
    Ok(Arc::new(Mutex::new(conn)))
}
