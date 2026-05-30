use rusqlite::{params, Connection};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub message_count: i64,
    pub last_message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionMessage {
    pub id: i64,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

pub struct SessionsStore {
    conn: Arc<Mutex<Connection>>,
}

impl SessionsStore {
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

    pub fn create_session(&self, id: &str, title: &str) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT OR IGNORE INTO sessions (id, title) VALUES (?1, ?2)",
                params![id, title],
            )?;
            Ok(())
        })
    }

    pub fn add_message(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<i64, rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO session_messages (session_id, role, content) VALUES (?1, ?2, ?3)",
                params![session_id, role, content],
            )?;
            conn.execute(
                "UPDATE sessions SET message_count = message_count + 1, last_message = ?1 WHERE id = ?2",
                params![content, session_id],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    pub fn list_sessions(&self, limit: usize) -> Result<Vec<ChatSession>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, message_count, last_message, created_at
                 FROM sessions ORDER BY created_at DESC LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok(ChatSession {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    message_count: row.get(2)?,
                    last_message: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            let mut sessions = Vec::new();
            for row in rows {
                sessions.push(row?);
            }
            Ok(sessions)
        })
    }

    pub fn get_messages(&self, session_id: &str) -> Result<Vec<SessionMessage>, rusqlite::Error> {
        self.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, session_id, role, content, created_at
                 FROM session_messages WHERE session_id = ?1
                 ORDER BY id ASC",
            )?;
            let rows = stmt.query_map(params![session_id], |row| {
                Ok(SessionMessage {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?;
            let mut msgs = Vec::new();
            for row in rows {
                msgs.push(row?);
            }
            Ok(msgs)
        })
    }

    pub fn delete_session(&self, session_id: &str) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute("DELETE FROM session_messages WHERE session_id = ?1", params![session_id])?;
            conn.execute("DELETE FROM sessions WHERE id = ?1", params![session_id])?;
            Ok(())
        })
    }

    pub fn rename_session(&self, session_id: &str, new_title: &str) -> Result<(), rusqlite::Error> {
        self.with_conn(|conn| {
            conn.execute(
                "UPDATE sessions SET title = ?1 WHERE id = ?2",
                params![new_title, session_id],
            )?;
            Ok(())
        })
    }
}
