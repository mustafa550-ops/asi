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

    pub fn delete_low_confidence(&self, min_confidence: f64) -> Result<usize, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "DELETE FROM strategic_memory WHERE confidence < ?1",
                params![min_confidence],
            )
        })
    }

    pub fn count_all(&self) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row("SELECT COUNT(*) FROM strategic_memory", [], |row| row.get(0))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> StrategicMemory {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        StrategicMemory::new(Arc::new(Mutex::new(conn)))
}

    #[test]
    fn record_clamps_confidence() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "partial", 1.5).unwrap();
        let records = mem.query_by_context("ctx", 1).unwrap();
        assert!(records[0].confidence <= 1.0);
    }

    #[test]
    fn update_outcome_changes_record() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "partial", 0.5).unwrap();
        mem.update_outcome(id, "success", 0.9).unwrap();
        let records = mem.query_by_context("ctx", 1).unwrap();
        assert_eq!(records[0].outcome, "success");
        assert!((records[0].confidence - 0.9).abs() < 0.01);
    }

    #[test]
    fn boost_confidence_increases() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.5).unwrap();
        mem.boost_confidence(id, 0.3).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.8).abs() < 0.01);
    }

    #[test]
    fn boost_confidence_caps_at_one() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.8).unwrap();
        mem.boost_confidence(id, 0.5).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 1.0).abs() < 0.01);
    }

    #[test]
    fn decay_confidence_decreases() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.9).unwrap();
        mem.decay_confidence(id, 0.4).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.5).abs() < 0.01);
    }

    #[test]
    fn decay_confidence_floors_at_zero() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.2).unwrap();
        mem.decay_confidence(id, 0.5).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.0).abs() < 0.01);
    }

    #[test]
    fn query_by_context_uses_like() {
        let mem = setup();
        mem.record("buy signal for SXT", "buy", "success", 0.8).unwrap();
        mem.record("weather report", "none", "success", 0.5).unwrap();
        let results = mem.query_by_context("SXT", 10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_high_confidence_filters_by_outcome() {
        let mem = setup();
        mem.record("ctx1", "dec1", "success", 0.9).unwrap();
        mem.record("ctx2", "dec2", "failure", 0.9).unwrap();
        mem.record("ctx3", "dec3", "success", 0.7).unwrap();
        let results = mem.get_high_confidence(0.8, 10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_decisions_by_outcome_filters() {
        let mem = setup();
        mem.record("a", "d1", "success", 0.5).unwrap();
        mem.record("b", "d2", "failure", 0.5).unwrap();
        mem.record("c", "d3", "success", 0.5).unwrap();
        let successes = mem.get_decisions_by_outcome("success", 10).unwrap();
        assert_eq!(successes.len(), 2);
        let failures = mem.get_decisions_by_outcome("failure", 10).unwrap();
        assert_eq!(failures.len(), 1);
    }

    #[test]
    fn apply_time_decay_reduces_confidence() {
        let mem = setup();
        let id = mem.record("old ctx", "old dec", "success", 0.8).unwrap();
        {
            let store = StrategicMemory::new(mem.conn.clone());
            store.with_conn(|c| {
                c.execute("UPDATE strategic_memory SET updated_at = '2020-01-01' WHERE id = ?1", params![id])
            }).unwrap();
        }
        let affected = mem.apply_time_decay(1).unwrap();
        assert!(affected > 0);
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!(r[0].confidence < 0.8);
    }
}

#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> StrategicMemory {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        StrategicMemory::new(Arc::new(Mutex::new(conn)))
    }

    #[test]
    fn record_inserts_row() {
        let mem = setup();
        let id = mem.record("test context", "buy signal", "success", 0.8).unwrap();
        assert!(id > 0);
        assert_eq!(mem.count().unwrap(), 1);
    }

    #[test]
    fn record_clamps_confidence() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "partial", 1.5).unwrap();
        let records = mem.query_by_context("ctx", 1).unwrap();
        assert!(records[0].confidence <= 1.0);
    }

    #[test]
    fn update_outcome_changes_record() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "partial", 0.5).unwrap();
        mem.update_outcome(id, "success", 0.9).unwrap();
        let records = mem.query_by_context("ctx", 1).unwrap();
        assert_eq!(records[0].outcome, "success");
        assert!((records[0].confidence - 0.9).abs() < 0.01);
    }

    #[test]
    fn boost_confidence_increases() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.5).unwrap();
        mem.boost_confidence(id, 0.3).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.8).abs() < 0.01);
    }

    #[test]
    fn boost_confidence_caps_at_one() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.8).unwrap();
        mem.boost_confidence(id, 0.5).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 1.0).abs() < 0.01);
    }

    #[test]
    fn decay_confidence_decreases() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.9).unwrap();
        mem.decay_confidence(id, 0.4).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.5).abs() < 0.01);
    }

    #[test]
    fn decay_confidence_floors_at_zero() {
        let mem = setup();
        let id = mem.record("ctx", "dec", "success", 0.2).unwrap();
        mem.decay_confidence(id, 0.5).unwrap();
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!((r[0].confidence - 0.0).abs() < 0.01);
    }

    #[test]
    fn query_by_context_uses_like() {
        let mem = setup();
        mem.record("buy signal for SXT", "buy", "success", 0.8).unwrap();
        mem.record("weather report", "none", "success", 0.5).unwrap();
        let results = mem.query_by_context("SXT", 10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_high_confidence_filters_by_outcome() {
        let mem = setup();
        mem.record("ctx1", "dec1", "success", 0.9).unwrap();
        mem.record("ctx2", "dec2", "failure", 0.9).unwrap();
        mem.record("ctx3", "dec3", "success", 0.7).unwrap();
        let results = mem.get_high_confidence(0.8, 10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn get_decisions_by_outcome_filters() {
        let mem = setup();
        mem.record("a", "d1", "success", 0.5).unwrap();
        mem.record("b", "d2", "failure", 0.5).unwrap();
        mem.record("c", "d3", "success", 0.5).unwrap();
        let successes = mem.get_decisions_by_outcome("success", 10).unwrap();
        assert_eq!(successes.len(), 2);
        let failures = mem.get_decisions_by_outcome("failure", 10).unwrap();
        assert_eq!(failures.len(), 1);
    }

    #[test]
    fn apply_time_decay_reduces_confidence() {
        let mem = setup();
        let id = mem.record("old ctx", "old dec", "success", 0.8).unwrap();
        // Force updated_at to be far in the past
        {
            let store = StrategicMemory::new(mem.conn.clone());
            store.with_conn(|c| {
                c.execute("UPDATE strategic_memory SET updated_at = '2020-01-01' WHERE id = ?1", params![id])
            }).unwrap();
        }
        let affected = mem.apply_time_decay(1).unwrap();
        assert!(affected > 0);
        let r = mem.get_decisions_by_outcome("success", 1).unwrap();
        assert!(r[0].confidence < 0.8);
    }
}
