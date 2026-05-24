use rusqlite::Connection;

/// Edge History — Bilgi grafiği (§5.1).
pub struct EdgeHistory {
    conn: Connection,
}

impl EdgeHistory {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn add_edge(&self, parent_id: i64, child_id: i64, edge_type: &str, diff: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO edge_history (parent_id, child_id, type, diff) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![parent_id, child_id, edge_type, diff],
        )?;
        Ok(())
    }
}
