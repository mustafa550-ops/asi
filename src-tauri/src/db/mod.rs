pub mod schema;
pub mod embeddings;
pub mod edge_history;
pub mod strategic_memory;

use rusqlite::Connection;

pub fn init(path: &std::path::Path) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;

    // Enable WAL mode
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Initialize tables
    schema::create_tables(&conn)?;

    log::info!("Database initialized at {:?}", path);
    Ok(conn)
}
