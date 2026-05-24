pub mod schema;
pub mod embeddings;
pub mod edge_history;
pub mod strategic_memory;

use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub fn open(path: &Path) -> Result<Arc<Mutex<Connection>>, rusqlite::Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    schema::create_tables(&conn)?;
    schema::run_migrations(&conn)?;
    log::info!("Database initialized at {:?}", path);
    Ok(Arc::new(Mutex::new(conn)))
}
