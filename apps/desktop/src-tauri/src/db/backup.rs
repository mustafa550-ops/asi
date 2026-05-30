use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use chrono::Local;

pub struct DatabaseBackup {
    conn: Arc<Mutex<Connection>>,
    backup_dir: PathBuf,
}

impl DatabaseBackup {
    pub fn new(conn: Arc<Mutex<Connection>>, backup_dir: PathBuf) -> Self {
        Self { conn, backup_dir }
    }

    pub fn create_snapshot(&self) -> Result<PathBuf, String> {
        std::fs::create_dir_all(&self.backup_dir).map_err(|e| e.to_string())?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S_%3f");
        let backup_path = self.backup_dir.join(format!("adler_backup_{}.db", timestamp));

        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .map_err(|e| e.to_string())?;

        let mut dst = Connection::open(&backup_path).map_err(|e| e.to_string())?;
        let b = rusqlite::backup::Backup::new(&conn, &mut dst)
            .map_err(|e| e.to_string())?;
        b.run_to_completion(5, std::time::Duration::from_millis(250), None)
            .map_err(|e| e.to_string())?;

        log::info!("Database snapshot created: {:?}", backup_path);
        Ok(backup_path)
    }

    pub fn restore(&self, backup_path: &Path) -> Result<(), String> {
        if !backup_path.exists() {
            return Err(format!("Backup dosyasi bulunamadi: {:?}", backup_path));
        }

        let src = Connection::open(backup_path).map_err(|e| e.to_string())?;
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut dst = Connection::open_in_memory().map_err(|e| e.to_string())?;
        let b = rusqlite::backup::Backup::new(&src, &mut dst)
            .map_err(|e| e.to_string())?;
        b.run_to_completion(5, std::time::Duration::from_millis(250), None)
            .map_err(|e| e.to_string())?;

        log::info!("Database restored from: {:?}", backup_path);
        Ok(())
    }

    pub fn list_snapshots(&self) -> Result<Vec<PathBuf>, String> {
        let mut snapshots = Vec::new();
        if self.backup_dir.exists() {
            let mut entries: Vec<_> = std::fs::read_dir(&self.backup_dir)
                .map_err(|e| e.to_string())?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "db"))
                .collect();
            entries.sort_by_key(|e| e.path().metadata().and_then(|m| m.modified()).ok());
            entries.reverse();
            snapshots = entries.into_iter().map(|e| e.path()).collect();
        }
        Ok(snapshots)
    }

    pub fn cleanup_old_snapshots(&self, keep_count: usize) -> Result<usize, String> {
        let snapshots = self.list_snapshots()?;
        let mut deleted = 0;
        if snapshots.len() > keep_count {
            for snap in snapshots.iter().skip(keep_count) {
                std::fs::remove_file(snap).map_err(|e| e.to_string())?;
                deleted += 1;
            }
        }
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_with_db() -> (DatabaseBackup, Arc<Mutex<Connection>>, TempDir) {
        let tmp = TempDir::new().unwrap();
        let db_path = tmp.path().join("test.db");
        let conn = Arc::new(Mutex::new(Connection::open(&db_path).unwrap()));
        {
            let c = conn.lock().unwrap();
            c.execute_batch("CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, val TEXT);").unwrap();
            c.execute("INSERT INTO test_table (val) VALUES ('hello')", []).unwrap();
        }
        let backup_dir = tmp.path().join("backups");
        let backup = DatabaseBackup::new(conn.clone(), backup_dir);
        (backup, conn, tmp)
    }

    #[test]
    fn create_snapshot_creates_file() {
        let (backup, _, _tmp) = setup_with_db();
        let path = backup.create_snapshot().unwrap();
        assert!(path.exists());
        assert!(path.extension().unwrap() == "db");
    }

    #[test]
    fn list_snapshots_returns_snapshots() {
        let (backup, _, _tmp) = setup_with_db();
        backup.create_snapshot().unwrap();
        let snaps = backup.list_snapshots().unwrap();
        assert!(!snaps.is_empty());
    }

    #[test]
    fn list_snapshots_empty_when_no_backups() {
        let (backup, _, _tmp) = setup_with_db();
        let snaps = backup.list_snapshots().unwrap();
        assert!(snaps.is_empty());
    }

    #[test]
    fn restore_returns_ok() {
        let (backup, _, _tmp) = setup_with_db();
        let snap = backup.create_snapshot().unwrap();
        let result = backup.restore(&snap);
        assert!(result.is_ok());
    }

    #[test]
    fn restore_nonexistent_returns_err() {
        let (backup, _, _tmp) = setup_with_db();
        let result = backup.restore(&Path::new("/nonexistent/backup.db"));
        assert!(result.is_err());
    }

    #[test]
    fn cleanup_old_snapshots_removes_excess() {
        let (backup, _, _tmp) = setup_with_db();
        backup.create_snapshot().unwrap();
        backup.create_snapshot().unwrap();
        backup.create_snapshot().unwrap();
        let deleted = backup.cleanup_old_snapshots(1).unwrap();
        assert!(deleted > 0);
        let remaining = backup.list_snapshots().unwrap();
        assert!(remaining.len() <= 1);
    }

    #[test]
    fn cleanup_does_nothing_when_under_limit() {
        let (backup, _, _tmp) = setup_with_db();
        backup.create_snapshot().unwrap();
        let deleted = backup.cleanup_old_snapshots(5).unwrap();
        assert_eq!(deleted, 0);
    }
}
