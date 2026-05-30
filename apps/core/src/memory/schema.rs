use rusqlite::Connection;
use crate::error::Result;

pub fn apply_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS session_memory (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX IF NOT EXISTS idx_session_agent ON session_memory (agent_id);
        CREATE INDEX IF NOT EXISTS idx_session_time ON session_memory (timestamp DESC);

        CREATE TABLE IF NOT EXISTS semantic_memory (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            source_file TEXT,
            metadata TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "
    ).map_err(|e| crate::error::AdlerError::Db(format!("Failed to apply migrations: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_migrations_creates_tables() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(tables.contains(&"session_memory".to_string()));
        assert!(tables.contains(&"semantic_memory".to_string()));
    }

    #[test]
    fn apply_migrations_is_idempotent() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        apply_migrations(&conn).unwrap(); // second call should not error
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(tables.contains(&"session_memory".to_string()));
    }

    #[test]
    fn session_memory_has_correct_columns() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        let cols: Vec<String> = conn
            .prepare("PRAGMA table_info(session_memory)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(cols.contains(&"id".to_string()));
        assert!(cols.contains(&"agent_id".to_string()));
        assert!(cols.contains(&"user_id".to_string()));
        assert!(cols.contains(&"role".to_string()));
        assert!(cols.contains(&"content".to_string()));
        assert!(cols.contains(&"timestamp".to_string()));
    }
}
