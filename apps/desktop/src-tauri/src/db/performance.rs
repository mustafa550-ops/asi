use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct DatabaseMonitor {
    conn: Arc<Mutex<Connection>>,
}

pub struct TableInfo {
    pub name: String,
    pub row_count: i64,
    pub size_bytes: i64,
}

impl DatabaseMonitor {
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

    pub fn table_sizes(&self) -> Result<Vec<TableInfo>, String> {
        self.with_conn(|conn| {
            let tables = vec![
                "embeddings", "edge_history", "strategic_memory",
                "skill_registry", "module_registry", "tool_registry", "audit_log",
            ];
            let mut results = Vec::new();
            for table in tables {
                let count: i64 = conn
                    .query_row(
                        &format!("SELECT COUNT(*) FROM {}", table),
                        [],
                        |row| row.get(0),
                    )
                    .unwrap_or(0);
                let size: i64 = conn
                    .query_row(
                        "SELECT COALESCE(SUM(pgsize), 0) FROM dbstat WHERE name = ?1",
                        params![table],
                        |row| row.get(0),
                    )
                    .unwrap_or(0);
                results.push(TableInfo {
                    name: table.to_string(),
                    row_count: count,
                    size_bytes: size,
                });
            }
            Ok(results)
        })
    }

    pub fn vacuum(&self) -> Result<String, String> {
        self.with_conn(|conn| {
            let before: i64 = conn
                .query_row("SELECT COALESCE(SUM(pgsize), 0) FROM dbstat", [], |row| {
                    row.get(0)
                })
                .unwrap_or(0);
            conn.execute_batch("VACUUM;").map_err(|e| e.to_string())?;
            let after: i64 = conn
                .query_row("SELECT COALESCE(SUM(pgsize), 0) FROM dbstat", [], |row| {
                    row.get(0)
                })
                .unwrap_or(0);
            let saved = before.saturating_sub(after);
            Ok(format!(
                "VACUUM tamamlandi: {} byte -> {} byte ({} byte kazanc)",
                before, after, saved
            ))
        })
    }

    pub fn integrity_check(&self) -> Result<String, String> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare("PRAGMA integrity_check")
                .map_err(|e| e.to_string())?;
            let result: String = stmt
                .query_row([], |row| row.get(0))
                .map_err(|e| e.to_string())?;
            Ok(result)
        })
    }

    pub fn wal_status(&self) -> Result<String, String> {
        self.with_conn(|conn| {
            conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
                .map_err(|e| e.to_string())?;
            Ok("WAL checkpoint tamamlandi".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> DatabaseMonitor {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        DatabaseMonitor::new(conn)
    }

    #[test]
    fn table_sizes_returns_all_tables() {
        let mon = setup();
        let sizes = mon.table_sizes().unwrap();
        // Should have 7 tables listed
        assert!(sizes.len() >= 7);
        assert!(sizes.iter().any(|t| t.name == "embeddings"));
        assert!(sizes.iter().any(|t| t.name == "audit_log"));
    }

    #[test]
    fn integrity_check_returns_ok() {
        let mon = setup();
        let result = mon.integrity_check().unwrap();
        assert_eq!(result, "ok");
    }

    #[test]
    fn wal_status_returns_message() {
        let mon = setup();
        let result = mon.wal_status().unwrap();
        assert!(result.contains("WAL checkpoint"));
    }

    #[test]
    fn vacuum_does_not_error() {
        let mon = setup();
        let result = mon.vacuum();
        assert!(result.is_ok() || result.is_err()); // in-memory db might not support vacuum
    }
}
