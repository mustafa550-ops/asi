use rusqlite::Connection;

/// Strategic Memory — Deneyim hafızası (§5.1).
pub struct StrategicMemory {
    conn: Connection,
}

impl StrategicMemory {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn record(&self, context: &str, decision: &str, outcome: &str, confidence: f64) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO strategic_memory (context, decision, outcome, confidence) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![context, decision, outcome, confidence],
        )?;
        Ok(())
    }
}
