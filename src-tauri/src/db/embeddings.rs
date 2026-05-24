use rusqlite::Connection;

/// Embedding operations — vektör depolama ve semantik arama (§5.1).
pub struct EmbeddingStore {
    conn: Connection,
}

impl EmbeddingStore {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn store(&self, content: &str, embedding: &[f32], source: &str, category: &str) -> Result<(), rusqlite::Error> {
        let blob: Vec<u8> = embedding
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        self.conn.execute(
            "INSERT INTO embeddings (content, embedding, source, category) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![content, blob, source, category],
        )?;
        Ok(())
    }

    pub fn search(&self, _query_embedding: &[f32], _limit: usize) -> Vec<(String, String, f32)> {
        Vec::new()
    }
}
