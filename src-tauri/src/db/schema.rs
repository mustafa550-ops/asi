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
        "
    )?;
    Ok(())
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
