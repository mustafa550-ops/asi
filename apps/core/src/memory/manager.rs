use rusqlite::Connection;
use std::sync::Mutex;
use std::path::Path;

use crate::error::{AdlerError, Result};
use crate::memory::schema;

/// Thread-safe SQLite Connection Manager
pub struct MemoryManager {
    conn: Mutex<Connection>,
}

impl MemoryManager {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let path = db_path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| AdlerError::Io(e))?;
            }
        }

        let conn = Connection::open(path)
            .map_err(|e| AdlerError::Db(format!("Failed to open DB: {}", e)))?;
            
        // Apply WAL mode for better concurrency
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| AdlerError::Db(format!("Failed to set WAL: {}", e)))?;

        // Apply DB Schema Migrations
        schema::apply_migrations(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AdlerError::Db(format!("Failed to open in-memory DB: {}", e)))?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| AdlerError::Db(format!("Failed to set WAL: {}", e)))?;
        schema::apply_migrations(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn with_connection<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.conn.lock().map_err(|_| AdlerError::System("DB lock poisoned".into()))?;
        f(&conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::memory::session::SessionMemory;
    use crate::memory::semantic::SemanticMemory;

    #[test]
    fn new_creates_db_file() {
        let dir = TempDir::new().unwrap();
        let db_path = dir.path().join("test.db");
        let manager = MemoryManager::new(&db_path).unwrap();
        assert!(db_path.exists());
        // Cleanup via TempDir drop
        drop(manager);
    }

    #[test]
    fn new_in_memory_works() {
        let manager = MemoryManager::new_in_memory().unwrap();
        assert!(manager.with_connection(|conn| {
            let count: i64 = conn.query_row("SELECT COUNT(*) FROM session_memory", [], |r| r.get(0)).unwrap();
            Ok(count)
        }).is_ok());
    }

    #[test]
    fn with_connection_executes_query() {
        let manager = MemoryManager::new_in_memory().unwrap();
        let result = manager.with_connection(|conn| {
            let val: i32 = conn.query_row("SELECT 1", [], |r| r.get(0))
                .map_err(|e| AdlerError::Db(e.to_string()))?;
            Ok(val)
        });
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn session_memory_store_and_retrieve() {
        let manager = MemoryManager::new_in_memory().unwrap();
        let session = SessionMemory::new(&manager);
        session.store("agent_01", "user_01", "user", "Merhaba").unwrap();
        session.store("agent_01", "user_01", "assistant", "Merhaba!").unwrap();
        let recent = session.get_recent("agent_01", 10).unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].role, "user");
        assert_eq!(recent[1].role, "assistant");
    }

    #[test]
    fn session_memory_limit() {
        let manager = MemoryManager::new_in_memory().unwrap();
        let session = SessionMemory::new(&manager);
        for i in 0..5 {
            session.store("agent_01", "user_01", "user", &format!("msg {}", i)).unwrap();
        }
        let recent = session.get_recent("agent_01", 3).unwrap();
        assert_eq!(recent.len(), 3);
        assert!(recent.last().unwrap().content.contains("msg 4"));
    }

    #[test]
    fn semantic_memory_store_and_retrieve() {
        let manager = MemoryManager::new_in_memory().unwrap();
        let semantic = SemanticMemory::new(&manager);
        let id1 = semantic.store("RSI nedir?", Some("docs/trading.md"), None).unwrap();
        let id2 = semantic.store("MACD hesaplama", None, Some("{\"type\":\"indicator\"}")).unwrap();
        assert_ne!(id1, id2);
        let results = semantic.mock_search("", 10).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn semantic_store_with_all_fields() {
        let manager = MemoryManager::new_in_memory().unwrap();
        let semantic = SemanticMemory::new(&manager);
        let id = semantic.store("test", Some("source.md"), Some("{\"key\":\"val\"}")).unwrap();
        let results = semantic.mock_search("", 10).unwrap();
        assert_eq!(results[0].source_file.as_deref(), Some("source.md"));
        assert_eq!(results[0].metadata.as_deref(), Some("{\"key\":\"val\"}"));
    }
}
