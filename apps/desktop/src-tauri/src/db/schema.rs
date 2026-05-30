use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS embeddings (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            embedding BLOB,
            source TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            category TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_embeddings_category ON embeddings(category);
        CREATE INDEX IF NOT EXISTS idx_embeddings_timestamp ON embeddings(timestamp);

        CREATE VIRTUAL TABLE IF NOT EXISTS embeddings_fts USING fts5(
            content, source, category,
            content=embeddings, content_rowid=id
        );

        CREATE TRIGGER IF NOT EXISTS embeddings_ai AFTER INSERT ON embeddings BEGIN
            INSERT INTO embeddings_fts(rowid, content, source, category)
            VALUES (new.id, new.content, new.source, new.category);
        END;

        CREATE TRIGGER IF NOT EXISTS embeddings_ad AFTER DELETE ON embeddings BEGIN
            INSERT INTO embeddings_fts(embeddings_fts, rowid, content, source, category)
            VALUES ('delete', old.id, old.content, old.source, old.category);
        END;

        CREATE TRIGGER IF NOT EXISTS embeddings_au AFTER UPDATE ON embeddings BEGIN
            INSERT INTO embeddings_fts(embeddings_fts, rowid, content, source, category)
            VALUES ('delete', old.id, old.content, old.source, old.category);
            INSERT INTO embeddings_fts(rowid, content, source, category)
            VALUES (new.id, new.content, new.source, new.category);
        END;

        CREATE TABLE IF NOT EXISTS edge_history (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            child_id INTEGER,
            type TEXT NOT NULL,
            diff TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_edge_parent ON edge_history(parent_id);
        CREATE INDEX IF NOT EXISTS idx_edge_child ON edge_history(child_id);
        CREATE INDEX IF NOT EXISTS idx_edge_type ON edge_history(type);

        CREATE TABLE IF NOT EXISTS strategic_memory (
            id INTEGER PRIMARY KEY,
            context TEXT NOT NULL,
            decision TEXT NOT NULL,
            outcome TEXT NOT NULL CHECK(outcome IN ('success', 'failure', 'partial')),
            confidence REAL NOT NULL DEFAULT 0.0 CHECK(confidence >= 0.0 AND confidence <= 1.0),
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_strategic_outcome ON strategic_memory(outcome);
        CREATE INDEX IF NOT EXISTS idx_strategic_confidence ON strategic_memory(confidence);
        CREATE INDEX IF NOT EXISTS idx_strategic_updated ON strategic_memory(updated_at);

        CREATE TABLE IF NOT EXISTS skill_registry (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT DEFAULT '',
            triggers TEXT DEFAULT '[]',
            approval TEXT DEFAULT 'required',
            steps TEXT DEFAULT '[]',
            logic_code TEXT DEFAULT '',
            evolution TEXT DEFAULT '[]',
            run_count INTEGER DEFAULT 0,
            active INTEGER DEFAULT 1,
            version INTEGER DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS module_registry (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            path TEXT NOT NULL,
            dependencies TEXT DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS tool_registry (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT DEFAULT '',
            approval_required INTEGER DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL DEFAULT 'Yeni Sohbet',
            message_count INTEGER DEFAULT 0,
            last_message TEXT DEFAULT '',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS session_messages (
            id INTEGER PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_sm_session ON session_messages(session_id);

        CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY,
            event_type TEXT NOT NULL,
            actor TEXT NOT NULL,
            details TEXT,
            hash TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_audit_type ON audit_log(event_type);
        CREATE INDEX IF NOT EXISTS idx_audit_created ON audit_log(created_at);
        "
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
        conn
    }

    #[test]
    fn create_tables_creates_all_tables() {
        let conn = create_in_memory();
        create_tables(&conn).unwrap();
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(tables.contains(&"embeddings".to_string()));
        assert!(tables.contains(&"edge_history".to_string()));
        assert!(tables.contains(&"strategic_memory".to_string()));
        assert!(tables.contains(&"skill_registry".to_string()));
        assert!(tables.contains(&"module_registry".to_string()));
        assert!(tables.contains(&"tool_registry".to_string()));
        assert!(tables.contains(&"audit_log".to_string()));
        assert!(tables.contains(&"sessions".to_string()));
        assert!(tables.contains(&"session_messages".to_string()));
    }

    #[test]
    fn create_tables_is_idempotent() {
        let conn = create_in_memory();
        create_tables(&conn).unwrap();
        create_tables(&conn).unwrap(); // second call should not error
    }

    #[test]
    fn embeddings_fts_virtual_table_exists() {
        let conn = create_in_memory();
        create_tables(&conn).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='embeddings_fts'",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn run_migrations_adds_dim_column() {
        let conn = create_in_memory();
        create_tables(&conn).unwrap();
        // Before migration, embedding_dim shouldn't exist
        // After migration, it should
        run_migrations(&conn).unwrap();
        let cols: Vec<String> = conn
            .prepare("PRAGMA table_info(embeddings)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(cols.contains(&"embedding_dim".to_string()));
    }
}

    pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    let version: i32 = conn.pragma_query_value(None, "user_version", |r| r.get(0)).unwrap_or(0);
    if version < 1 {
        conn.execute_batch(
            "ALTER TABLE embeddings ADD COLUMN embedding_dim INTEGER DEFAULT 0;"
        )?;
        conn.pragma_update(None, "user_version", 1)?;
    }
    Ok(())
}
