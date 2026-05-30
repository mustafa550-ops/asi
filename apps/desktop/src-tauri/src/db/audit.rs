use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use ring::digest::{Context, SHA256};

pub struct AuditLog {
    conn: Arc<Mutex<Connection>>,
}

pub struct AuditEntry {
    pub id: i64,
    pub event_type: String,
    pub actor: String,
    pub details: String,
    pub hash: String,
    pub created_at: String,
}

impl AuditLog {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn with_conn<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&Connection) -> Result<T, String>,
    {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        f(&conn)
    }

    fn compute_hash(event_type: &str, actor: &str, details: &str) -> String {
        let mut ctx = Context::new(&SHA256);
        ctx.update(event_type.as_bytes());
        ctx.update(actor.as_bytes());
        ctx.update(details.as_bytes());
        hex::encode(ctx.finish().as_ref())
    }

    pub fn log(&self, event_type: &str, actor: &str, details: &str) -> Result<i64, String> {
        self.with_conn(|conn| {
            let hash = Self::compute_hash(event_type, actor, details);
            conn.execute(
                "INSERT INTO audit_log (event_type, actor, details, hash)
                 VALUES (?1, ?2, ?3, ?4)",
                params![event_type, actor, details, hash],
            ).map_err(|e| e.to_string())?;
            Ok(conn.last_insert_rowid())
        })
    }

    pub fn get_recent(&self, limit: usize) -> Result<Vec<AuditEntry>, String> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, event_type, actor, details, hash, created_at
                 FROM audit_log ORDER BY id DESC LIMIT ?1",
            ).map_err(|e| e.to_string())?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok(AuditEntry {
                    id: row.get(0)?,
                    event_type: row.get(1)?,
                    actor: row.get(2)?,
                    details: row.get(3)?,
                    hash: row.get(4)?,
                    created_at: row.get(5)?,
                })
            }).map_err(|e| e.to_string())?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row.map_err(|e| e.to_string())?);
            }
            Ok(results)
        })
    }

    pub fn verify_integrity(&self, id: i64) -> Result<bool, String> {
        self.with_conn(|conn| {
            let r = conn.query_row(
                "SELECT event_type, actor, details, hash FROM audit_log WHERE id = ?1",
                params![id],
                |row| {
                    let et: String = row.get(0)?;
                    let a: String = row.get(1)?;
                    let d: String = row.get(2)?;
                    let h: String = row.get(3)?;
                    Ok((et, a, d, h))
                },
            );
            match r {
                Ok((event_type, actor, details, stored_hash)) => {
                    let computed = Self::compute_hash(&event_type, &actor, &details);
                    Ok(computed == stored_hash)
                }
                Err(_) => Ok(false),
            }
        })
    }

    pub fn count(&self) -> Result<i64, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> AuditLog {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        AuditLog::new(conn)
    }

    #[test]
    fn log_inserts_entry() {
        let log = setup();
        let id = log.log("system_start", "adler", "System booted").unwrap();
        assert!(id > 0);
        assert_eq!(log.count().unwrap(), 1);
    }

    #[test]
    fn get_recent_returns_entries() {
        let log = setup();
        log.log("test", "user", "entry1").unwrap();
        log.log("test", "user", "entry2").unwrap();
        let entries = log.get_recent(10).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn get_recent_orders_by_id_desc() {
        let log = setup();
        log.log("t", "u", "first").unwrap();
        log.log("t", "u", "second").unwrap();
        let entries = log.get_recent(10).unwrap();
        assert!(entries[0].id > entries[1].id);
    }

    #[test]
    fn get_recent_respects_limit() {
        let log = setup();
        log.log("t", "u", "a").unwrap();
        log.log("t", "u", "b").unwrap();
        log.log("t", "u", "c").unwrap();
        let entries = log.get_recent(2).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn verify_integrity_returns_true_for_unchanged() {
        let log = setup();
        let id = log.log("test", "adler", "valid entry").unwrap();
        assert!(log.verify_integrity(id).unwrap());
    }

    #[test]
    fn verify_integrity_returns_false_for_nonexistent() {
        let log = setup();
        assert!(!log.verify_integrity(999).unwrap());
    }

    #[test]
    fn compute_hash_is_deterministic() {
        let h1 = AuditLog::compute_hash("a", "b", "c");
        let h2 = AuditLog::compute_hash("a", "b", "c");
        assert_eq!(h1, h2);
    }

    #[test]
    fn compute_hash_different_inputs_differ() {
        let h1 = AuditLog::compute_hash("a", "b", "c");
        let h2 = AuditLog::compute_hash("a", "b", "d");
        assert_ne!(h1, h2);
    }

    #[test]
    fn count_returns_zero_when_empty() {
        let log = setup();
        assert_eq!(log.count().unwrap(), 0);
    }
}
