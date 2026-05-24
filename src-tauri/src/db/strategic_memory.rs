use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct StrategicRecord {
    pub id: i64,
    pub context: String,
    pub decision: String,
    pub outcome: String,
    pub confidence: f64,
    pub updated_at: String,
}

pub struct StrategicMemory {
    conn: Arc<Mutex<Connection>>,
}

impl StrategicMemory {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn with_conn<F, T>(&self, f: F) -> Result<T, rusqlite::Error>
    where
        F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }

    pub fn record(
        &self,
        context: &str,
        decision: &str,
        outcome: &str,
        confidence: f64,
    ) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO strategic_memory (context, decision, outcome, confidence)
                 VALUES (?1, ?2, ?3, ?4)",
                params![context, decision, outcome, confidence.clamp(0.0, 1.0)],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    pub fn update_outcome(
        &self,
        id: i64,
        outcome: &str,
        confidence: f64,
    ) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE strategic_memory
                 SET outcome = ?1, confidence = ?2, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?3",
                params![outcome, confidence.clamp(0.0, 1.0), id],
            )?;
            Ok(())
        })
    }

    pub fn boost_confidence(&self, id: i64, delta: f64) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE strategic_memory
                 SET confidence = MIN(1.0, confidence + ?1), updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                params![delta.abs(), id],
            )?;
            Ok(())
        })
    }

    pub fn decay_confidence(&self, id: i64, delta: f64) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE strategic_memory
                 SET confidence = MAX(0.0, confidence - ?1), updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?2",
                params![delta.abs(), id],
            )?;
            Ok(())
        })
    }

    pub fn query_by_context(
        &self,
        context: &str,
        limit: usize,
    ) -> Result<Vec<StrategicRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, context, decision, outcome, confidence, updated_at
                 FROM strategic_memory
                 WHERE context LIKE ?1
                 ORDER BY confidence DESC, updated_at DESC
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map(
                params![format!("%{}%", context), limit as i64],
                Self::map_record,
            )?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn get_high_confidence(
        &self,
        min_confidence: f64,
        limit: usize,
    ) -> Result<Vec<StrategicRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, context, decision, outcome, confidence, updated_at
                 FROM strategic_memory
                 WHERE confidence >= ?1 AND outcome = 'success'
                 ORDER BY confidence DESC
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![min_confidence, limit as i64], Self::map_record)?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn get_decisions_by_outcome(
        &self,
        outcome: &str,
        limit: usize,
    ) -> Result<Vec<StrategicRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, context, decision, outcome, confidence, updated_at
                 FROM strategic_memory
                 WHERE outcome = ?1
                 ORDER BY confidence DESC
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![outcome, limit as i64], Self::map_record)?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn apply_time_decay(&self, half_life_days: i64) -> Result<usize, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE strategic_memory
                 SET confidence = MAX(0.0, confidence * 0.5),
                     updated_at = CURRENT_TIMESTAMP
                 WHERE updated_at < datetime('now', ?1)
                   AND confidence > 0.01",
                params![format!("-{} days", half_life_days)],
            )
        })
    }

    pub fn count(&self) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row("SELECT COUNT(*) FROM strategic_memory", [], |row| {
                row.get(0)
            })
        })
    }

    fn map_record(row: &rusqlite::Row) -> rusqlite::Result<StrategicRecord> {
        Ok(StrategicRecord {
            id: row.get(0)?,
            context: row.get(1)?,
            decision: row.get(2)?,
            outcome: row.get(3)?,
            confidence: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}
