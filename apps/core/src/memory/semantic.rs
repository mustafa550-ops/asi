use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rusqlite::params;

use crate::error::{AdlerError, Result};
use crate::memory::MemoryManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDocument {
    pub id: String,
    pub content: String,
    pub source_file: Option<String>,
    pub metadata: Option<String>,
    pub timestamp: String,
}

pub struct SemanticMemory<'a> {
    manager: &'a MemoryManager,
}

impl<'a> SemanticMemory<'a> {
    pub fn new(manager: &'a MemoryManager) -> Self {
        Self { manager }
    }

    pub fn store(&self, content: &str, source_file: Option<&str>, metadata: Option<&str>) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        
        self.manager.with_connection(|conn| {
            conn.execute(
                "INSERT INTO semantic_memory (id, content, source_file, metadata) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![id, content, source_file, metadata],
            ).map_err(|e| AdlerError::Db(format!("Failed to store semantic document: {}", e)))?;
            Ok(id.clone())
        })
    }

    // A placeholder for actual vector search (will be implemented later with sqlite-vss)
    pub fn mock_search(&self, _query: &str, limit: usize) -> Result<Vec<SemanticDocument>> {
        self.manager.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, content, source_file, metadata, timestamp 
                 FROM semantic_memory 
                 ORDER BY timestamp DESC 
                 LIMIT ?1"
            ).map_err(|e| AdlerError::Db(format!("Prepare error: {}", e)))?;

            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok(SemanticDocument {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    source_file: row.get(2)?,
                    metadata: row.get(3)?,
                    timestamp: row.get(4)?,
                })
            }).map_err(|e| AdlerError::Db(format!("Query error: {}", e)))?;

            let mut docs = Vec::new();
            for doc in rows {
                docs.push(doc.map_err(|e| AdlerError::Db(format!("Row error: {}", e)))?);
            }
            
            Ok(docs)
        })
    }
}
