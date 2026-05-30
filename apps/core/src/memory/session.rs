use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rusqlite::params;

use crate::error::{AdlerError, Result};
use crate::memory::MemoryManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    pub id: String,
    pub agent_id: String,
    pub user_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

pub struct SessionMemory<'a> {
    manager: &'a MemoryManager,
}

impl<'a> SessionMemory<'a> {
    pub fn new(manager: &'a MemoryManager) -> Self {
        Self { manager }
    }

    pub fn store(&self, agent_id: &str, user_id: &str, role: &str, content: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        
        self.manager.with_connection(|conn| {
            conn.execute(
                "INSERT INTO session_memory (id, agent_id, user_id, role, content) 
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, agent_id, user_id, role, content],
            ).map_err(|e| AdlerError::Db(format!("Failed to store session message: {}", e)))?;
            Ok(())
        })
    }

    pub fn get_recent(&self, agent_id: &str, limit: usize) -> Result<Vec<SessionMessage>> {
        self.manager.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, agent_id, user_id, role, content, timestamp 
                 FROM session_memory 
                 WHERE agent_id = ?1 
                 ORDER BY rowid DESC 
                 LIMIT ?2"
            ).map_err(|e| AdlerError::Db(format!("Prepare error: {}", e)))?;

            let rows = stmt.query_map(params![agent_id, limit as i64], |row| {
                Ok(SessionMessage {
                    id: row.get(0)?,
                    agent_id: row.get(1)?,
                    user_id: row.get(2)?,
                    role: row.get(3)?,
                    content: row.get(4)?,
                    timestamp: row.get(5)?,
                })
            }).map_err(|e| AdlerError::Db(format!("Query error: {}", e)))?;

            let mut messages = Vec::new();
            for msg in rows {
                messages.push(msg.map_err(|e| AdlerError::Db(format!("Row error: {}", e)))?);
            }
            
            // Reverse to get chronological order
            messages.reverse();
            Ok(messages)
        })
    }
}
