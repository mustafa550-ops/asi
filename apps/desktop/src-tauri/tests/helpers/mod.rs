use std::sync::{Arc, Mutex};

/// Seed a temporary SQLite database with the schema and return the path.
/// The database file is destroyed when the returned `TempDir` is dropped.
pub fn create_test_db() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let db_path = dir.path().join("test.db");
    let db_path_str = db_path.to_str().unwrap().to_string();
    let conn = rusqlite::Connection::open(&db_path)
        .expect("failed to open test database");
    adler_asi_lib::db::schema::create_tables(&conn)
        .expect("failed to run schema creation");
    adler_asi_lib::db::schema::run_migrations(&conn)
        .expect("failed to run migrations");
    (dir, db_path_str)
}

/// Create an in-memory DB connection for testing (with schema + migrations).
pub fn create_test_db_in_memory() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory()
        .expect("failed to open in-memory database");
    adler_asi_lib::db::schema::create_tables(&conn)
        .expect("failed to run schema creation");
    adler_asi_lib::db::schema::run_migrations(&conn)
        .expect("failed to run migrations");
    conn
}

/// Create an in-memory DB with a SchemaRegistry-compatible Arc<Mutex<Connection>>.
pub fn create_test_db_arc() -> Arc<Mutex<rusqlite::Connection>> {
    Arc::new(Mutex::new(create_test_db_in_memory()))
}
