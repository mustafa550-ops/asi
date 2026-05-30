use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct ModuleEntry {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub dependencies: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub id: i64,
    pub name: String,
    pub run_count: i64,
    pub created_at: String,
}

pub struct ModuleRegistry {
    conn: Arc<Mutex<Connection>>,
}

impl ModuleRegistry {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        if let Ok(c) = conn.lock() {
            c.execute_batch(
                "CREATE TABLE IF NOT EXISTS module_registry (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL UNIQUE,
                    path TEXT NOT NULL,
                    dependencies TEXT DEFAULT '',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );",
            )
            .ok();
        }
        Self { conn }
    }

    pub fn register(&self, name: &str, path: &str, dependencies: &[String]) -> Result<i64, String> {
        let deps = dependencies.join(",");
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO module_registry (name, path, dependencies) VALUES (?1, ?2, ?3)
             ON CONFLICT(name) DO UPDATE SET path = ?2, dependencies = ?3",
            params![name, path, deps],
        )
        .map_err(|e| e.to_string())?;
        Ok(conn.last_insert_rowid())
    }

    pub fn remove(&self, name: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM module_registry WHERE name = ?1", params![name])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_all(&self) -> Vec<ModuleEntry> {
        self.list().unwrap_or_default()
    }

    pub fn list_skills(&self) -> Vec<SkillEntry> {
        self.skill_list().unwrap_or_default()
    }

    fn list(&self) -> Result<Vec<ModuleEntry>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, path, dependencies, created_at FROM module_registry ORDER BY name")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok(ModuleEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    dependencies: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| e.to_string())?);
        }
        Ok(entries)
    }

    pub fn get(&self, name: &str) -> Result<Option<ModuleEntry>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, path, dependencies, created_at FROM module_registry WHERE name = ?1")
            .map_err(|e| e.to_string())?;
        let mut rows = stmt
            .query_map(params![name], |row| {
                Ok(ModuleEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    dependencies: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;
        match rows.next() {
            Some(row) => Ok(Some(row.map_err(|e| e.to_string())?)),
            None => Ok(None),
        }
    }

    pub fn count(&self) -> Result<i64, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row("SELECT COUNT(*) FROM module_registry", [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }

    fn skill_list(&self) -> Result<Vec<SkillEntry>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, name, run_count, created_at FROM skill_registry ORDER BY name")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok(SkillEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    run_count: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| e.to_string())?);
        }
        Ok(entries)
    }
}
