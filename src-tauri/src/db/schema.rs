use rusqlite::Connection;

/// Database schema — Ayna görüntüsü CLAUDE.md §5.1
pub fn create_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS embeddings (
            id INTEGER PRIMARY KEY,
            content TEXT,
            embedding BLOB,
            source TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            category TEXT
        );

        CREATE TABLE IF NOT EXISTS edge_history (
            id INTEGER PRIMARY KEY,
            parent_id INTEGER,
            child_id INTEGER,
            type TEXT,
            diff TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS strategic_memory (
            id INTEGER PRIMARY KEY,
            context TEXT,
            decision TEXT,
            outcome TEXT,
            confidence REAL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "
    )?;
    Ok(())
}
