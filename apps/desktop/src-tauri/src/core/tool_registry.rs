use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct ToolEntry {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub approval_required: bool,
}

pub struct ToolRegistry {
    conn: Arc<Mutex<Connection>>,
}

impl ToolRegistry {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        if let Ok(c) = conn.lock() {
            c.execute_batch(
                "CREATE TABLE IF NOT EXISTS tool_registry (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL UNIQUE,
                    description TEXT NOT NULL DEFAULT '',
                    approval_required INTEGER NOT NULL DEFAULT 0,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );",
            )
            .ok();
        }
        Self { conn }
    }

    pub fn register(&self, name: &str, description: &str, approval_required: bool) -> Result<i64, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO tool_registry (name, description, approval_required) VALUES (?1, ?2, ?3)
             ON CONFLICT(name) DO UPDATE SET description = ?2, approval_required = ?3",
            params![name, description, approval_required as i32],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list(&self) -> Result<Vec<ToolEntry>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, approval_required FROM tool_registry ORDER BY name")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok(ToolEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    approval_required: row.get::<_, i32>(3)? != 0,
                })
            })
            .map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| e.to_string())?);
        }
        Ok(entries)
    }

    pub fn get(&self, name: &str) -> Result<Option<ToolEntry>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, approval_required FROM tool_registry WHERE name = ?1")
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map(params![name], |row| {
                Ok(ToolEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    approval_required: row.get::<_, i32>(3)? != 0,
                })
            })
            .map_err(|e| e.to_string())?;
        match rows.next() {
            Some(row) => Ok(Some(row.map_err(|e| e.to_string())?)),
            None => Ok(None),
        }
    }

    pub fn remove(&self, name: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM tool_registry WHERE name = ?1", params![name])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn requires_approval(&self, name: &str) -> Result<bool, String> {
        let entry = self.get(name)?;
        Ok(entry.map(|e| e.approval_required).unwrap_or(false))
    }
}
