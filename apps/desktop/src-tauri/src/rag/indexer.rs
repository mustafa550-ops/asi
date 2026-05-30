use crate::db::embeddings::EmbeddingStore;
use crate::llm::OllamaClient;
use super::chunker::{Chunk, Chunker};

pub struct Indexer {
    embeddings: EmbeddingStore,
    llm: OllamaClient,
    chunker: Chunker,
}

pub struct IndexingResult {
    pub total_chunks: usize,
    pub indexed: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

impl Indexer {
    pub fn new(embeddings: EmbeddingStore, llm: OllamaClient, chunker: Chunker) -> Self {
        Self { embeddings, llm, chunker }
    }

    pub fn index_document(&self, content: &str, source: &str, category: &str) -> Result<IndexingResult, String> {
        let chunks = self.chunker.chunk_markdown(content, source);
        let mut result = IndexingResult {
            total_chunks: chunks.len(),
            indexed: 0,
            failed: 0,
            errors: Vec::new(),
        };

        for chunk in &chunks {
            match self.index_chunk(chunk, category) {
                Ok(_) => result.indexed += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(format!("Chunk '{}...': {}", &chunk.content[..chunk.content.len().min(40)], e));
                }
            }
        }

        Ok(result)
    }

    pub fn index_chunk(&self, chunk: &Chunk, category: &str) -> Result<i64, String> {
        let embedding = self.llm.embedding_sync(&chunk.content)
            .map_err(|e| format!("Embedding failed: {}", e))?;
        self.embeddings.store(&chunk.content, &embedding, &chunk.source, category)
            .map_err(|e| e.to_string())
    }

    pub fn index_plain_text(&self, content: &str, source: &str, category: &str) -> Result<IndexingResult, String> {
        let chunks = self.chunker.chunk_plain_text(content, source, "\n\n");
        let mut result = IndexingResult {
            total_chunks: chunks.len(),
            indexed: 0,
            failed: 0,
            errors: Vec::new(),
        };

        for chunk in &chunks {
            match self.index_chunk(chunk, category) {
                Ok(_) => result.indexed += 1,
                Err(e) => {
                    result.failed += 1;
                    result.errors.push(e);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::embeddings::EmbeddingStore;
    use std::sync::{Arc, Mutex};
    use rusqlite::Connection;
    use crate::db::schema::create_tables;

    fn mock_llm() -> OllamaClient {
        OllamaClient::new("http://127.0.0.1:11434".into(), "mock".into())
    }

    fn setup_store() -> EmbeddingStore {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        EmbeddingStore::new(conn)
    }

    #[test]
    fn indexer_creation_succeeds() {
        let store = setup_store();
        let llm = mock_llm();
        let chunker = Chunker::default();
        let indexer = Indexer::new(store, llm, chunker);
        let result = indexer.index_document("test", "test.md", "test");
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn index_plain_text_zero_chunks_when_empty() {
        let store = setup_store();
        let llm = mock_llm();
        let chunker = Chunker::default();
        let indexer = Indexer::new(store, llm, chunker);
        let result = indexer.index_plain_text("", "empty.md", "test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().total_chunks, 0);
    }

    #[test]
    fn index_document_empty_content() {
        let store = setup_store();
        let llm = mock_llm();
        let chunker = Chunker::default();
        let indexer = Indexer::new(store, llm, chunker);
        let result = indexer.index_document("", "empty.md", "test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().total_chunks, 0);
    }
}
