use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, serde::Serialize)]
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

    pub fn delete_older_than(&self, days: i64) -> Result<usize, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "DELETE FROM edge_history WHERE created_at < datetime('now', ?1)",
                params![format!("-{} days", days)],
            )
        })
    }

    pub fn count_all(&self) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.query_row("SELECT COUNT(*) FROM edge_history", [], |row| row.get(0))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::create_tables;

    fn setup() -> EdgeHistory {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        EdgeHistory::new(Arc::new(Mutex::new(conn)))
    }

    #[test]
    fn add_edge_returns_id() {
        let store = setup();
        let id = store.add_edge(1, 2, "skill_evolution", "diff content").unwrap();
        assert!(id > 0);
    }

    #[test]
    fn get_children_returns_edges() {
        let store = setup();
        store.add_edge(10, 20, "bug_fix", "patch").unwrap();
        store.add_edge(10, 30, "bug_fix", "patch2").unwrap();
        let children = store.get_children(10).unwrap();
        assert_eq!(children.len(), 2);
        assert!(children.iter().all(|e| e.parent_id == 10));
    }

    #[test]
    fn get_parents_returns_edges() {
        let store = setup();
        store.add_edge(100, 200, "refinement", "evolved").unwrap();
        let parents = store.get_parents(200).unwrap();
        assert_eq!(parents.len(), 1);
        assert_eq!(parents[0].child_id, 200);
    }

    #[test]
    fn traverse_ancestors() {
        let store = setup();
        store.add_edge(1, 2, "link", "").unwrap();
        store.add_edge(2, 3, "link", "").unwrap();
        let ancestors = store.traverse_ancestors(3, 5).unwrap();
        assert_eq!(ancestors.len(), 2);
    }

    #[test]
    fn traverse_ancestors_respects_max_depth() {
        let store = setup();
        store.add_edge(1, 2, "link", "").unwrap();
        store.add_edge(2, 3, "link", "").unwrap();
        store.add_edge(3, 4, "link", "").unwrap();
        let ancestors = store.traverse_ancestors(4, 2).unwrap();
        assert_eq!(ancestors.len(), 2);
    }

    #[test]
    fn traverse_descendants() {
        let store = setup();
        store.add_edge(1, 2, "link", "").unwrap();
        store.add_edge(1, 3, "link", "").unwrap();
        store.add_edge(2, 4, "link", "").unwrap();
        let descendants = store.traverse_descendants(1, 5).unwrap();
        assert_eq!(descendants.len(), 3);
    }

    #[test]
    fn get_by_type_filters() {
        let store = setup();
        store.add_edge(1, 2, "bug_fix", "fix1").unwrap();
        store.add_edge(3, 4, "feature", "feat1").unwrap();
        store.add_edge(5, 6, "bug_fix", "fix2").unwrap();
        let bugs = store.get_by_type("bug_fix", 10).unwrap();
        assert_eq!(bugs.len(), 2);
    }

    #[test]
    fn get_by_type_respects_limit() {
        let store = setup();
        for i in 0..5 {
            store.add_edge(i, i + 10, "test_type", "").unwrap();
        }
        let results = store.get_by_type("test_type", 2).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn count_returns_total() {
        let store = setup();
        assert_eq!(store.count().unwrap(), 0);
        store.add_edge(1, 2, "t", "").unwrap();
        store.add_edge(2, 3, "t", "").unwrap();
        assert_eq!(store.count().unwrap(), 2);
    }

    #[test]
    fn edge_record_fields_match() {
        let store = setup();
        let id = store.add_edge(7, 8, "custom_type", "diff_data").unwrap();
        let children = store.get_children(7).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].child_id, 8);
        assert_eq!(children[0].edge_type, "custom_type");
        assert_eq!(children[0].diff, "diff_data");
    }

    #[test]
    fn empty_graph_returns_empty() {
        let store = setup();
        assert!(store.get_children(1).unwrap().is_empty());
        assert!(store.get_parents(1).unwrap().is_empty());
        assert!(store.traverse_ancestors(1, 10).unwrap().is_empty());
        assert!(store.traverse_descendants(1, 10).unwrap().is_empty());
    }
}
