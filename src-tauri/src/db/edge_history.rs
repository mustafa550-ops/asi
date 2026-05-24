use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EdgeRecord {
    pub id: i64,
    pub parent_id: i64,
    pub child_id: i64,
    pub edge_type: String,
    pub diff: String,
    pub created_at: String,
}

pub struct EdgeHistory {
    conn: Arc<Mutex<Connection>>,
}

impl EdgeHistory {
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

    pub fn add_edge(
        &self,
        parent_id: i64,
        child_id: i64,
        edge_type: &str,
        diff: &str,
    ) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO edge_history (parent_id, child_id, type, diff)
                 VALUES (?1, ?2, ?3, ?4)",
                params![parent_id, child_id, edge_type, diff],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    pub fn get_children(&self, parent_id: i64) -> Result<Vec<EdgeRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, child_id, type, diff, created_at
                 FROM edge_history WHERE parent_id = ?1 ORDER BY created_at",
            )?;
            let rows = stmt.query_map(params![parent_id], Self::map_edge)?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn get_parents(&self, child_id: i64) -> Result<Vec<EdgeRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, child_id, type, diff, created_at
                 FROM edge_history WHERE child_id = ?1 ORDER BY created_at",
            )?;
            let rows = stmt.query_map(params![child_id], Self::map_edge)?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn traverse_ancestors(
        &self,
        node_id: i64,
        max_depth: usize,
    ) -> Result<Vec<EdgeRecord>, rusqlite::Error> {
        let mut visited = HashSet::new();
        let mut results = Vec::new();
        let mut current = node_id;

        for _ in 0..max_depth {
            if !visited.insert(current) {
                break;
            }
            let parents = self.get_parents(current)?;
            if parents.is_empty() {
                break;
            }
            let edge = parents[0].clone();
            current = edge.parent_id;
            results.push(edge);
        }

        Ok(results)
    }

    pub fn traverse_descendants(
        &self,
        node_id: i64,
        max_depth: usize,
    ) -> Result<Vec<EdgeRecord>, rusqlite::Error> {
        let mut visited = HashSet::new();
        let mut results = Vec::new();
        let mut queue = vec![(node_id, 0)];

        while let Some((current, depth)) = queue.pop() {
            if depth >= max_depth || !visited.insert(current) {
                continue;
            }
            let children = self.get_children(current)?;
            for edge in children {
                results.push(edge.clone());
                queue.push((edge.child_id, depth + 1));
            }
        }

        Ok(results)
    }

    pub fn get_by_type(
        &self,
        edge_type: &str,
        limit: usize,
    ) -> Result<Vec<EdgeRecord>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, child_id, type, diff, created_at
                 FROM edge_history WHERE type = ?1 ORDER BY created_at DESC LIMIT ?2",
            )?;
            let rows = stmt.query_map(params![edge_type, limit as i64], Self::map_edge)?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }

    pub fn count(&self) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row("SELECT COUNT(*) FROM edge_history", [], |row| row.get(0))
        })
    }

    fn map_edge(row: &rusqlite::Row) -> rusqlite::Result<EdgeRecord> {
        Ok(EdgeRecord {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            child_id: row.get(2)?,
            edge_type: row.get(3)?,
            diff: row.get(4)?,
            created_at: row.get(5)?,
        })
    }
}
