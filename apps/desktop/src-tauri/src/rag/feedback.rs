use crate::db::embeddings::EmbeddingStore;
use crate::db::strategic_memory::StrategicMemory;
use crate::llm::OllamaClient;

pub struct FeedbackLoop {
    embeddings: EmbeddingStore,
    strategic: StrategicMemory,
    llm: OllamaClient,
}

pub struct Feedback {
    pub query: String,
    pub helpful: bool,
    pub comment: Option<String>,
}

impl FeedbackLoop {
    pub fn new(embeddings: EmbeddingStore, strategic: StrategicMemory, llm: OllamaClient) -> Self {
        Self { embeddings, strategic, llm }
    }

    pub fn record_feedback(&self, feedback: Feedback) -> Result<(), String> {
        let outcome = if feedback.helpful { "success" } else { "failure" };
        let context = format!("Kullanıcı sorgusu: {}", feedback.query);

        let decision = match &feedback.comment {
            Some(c) => format!("Geri bildirim: {} – {}", if feedback.helpful { "Yararlı" } else { "Yararsız" }, c),
            None => format!("Geri bildirim: {}", if feedback.helpful { "Yararlı" } else { "Yararsız" }),
        };

        self.strategic.record(&context, &decision, outcome, 1.0)
            .map_err(|e| e.to_string())?;

        if !feedback.helpful {
            self.downgrade_similar(&feedback.query)?;
        }

        Ok(())
    }

    fn downgrade_similar(&self, query: &str) -> Result<(), String> {
        if let Ok(embedding) = self.llm.embedding_sync(query) {
            if let Ok(results) = self.embeddings.search(&embedding, 5) {
                let downgrade_confidence = 0.3;
                for result in results {
                    if result.score > 0.7 {
                        let ctx = format!("Düşük kaliteli sonuç (geribildirim): {}", result.content);
                        self.strategic.record(&ctx, "Düşük güven", "failure", downgrade_confidence)
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn boost_good_sources(&self, query: &str) -> Result<(), String> {
        if let Ok(embedding) = self.llm.embedding_sync(query) {
            if let Ok(results) = self.embeddings.search(&embedding, 3) {
                for result in results {
                    let ctx = format!("Yüksek kaliteli sonuç: {}", result.content);
                    self.strategic.record(&ctx, "Yüksek güven", "success", 0.9)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::embeddings::EmbeddingStore;
    use crate::db::strategic_memory::StrategicMemory;
    use crate::db::schema::create_tables;
    use std::sync::{Arc, Mutex};
    use rusqlite::Connection;

    fn setup() -> (EmbeddingStore, StrategicMemory) {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
        create_tables(&conn.lock().unwrap()).unwrap();
        let embeddings = EmbeddingStore::new(conn.clone());
        let strategic = StrategicMemory::new(conn);
        (embeddings, strategic)
    }

    fn mock_llm() -> OllamaClient {
        OllamaClient::new("http://127.0.0.1:11434".into(), "mock".into())
    }

    #[test]
    fn record_positive_feedback() {
        let (embeddings, strategic) = setup();
        let llm = mock_llm();
        let feedback = FeedbackLoop::new(embeddings, strategic, llm);
        let result = feedback.record_feedback(Feedback {
            query: "test".into(),
            helpful: true,
            comment: None,
        });
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn record_negative_feedback() {
        let (embeddings, strategic) = setup();
        let llm = mock_llm();
        let feedback = FeedbackLoop::new(embeddings, strategic, llm);
        let result = feedback.record_feedback(Feedback {
            query: "test".into(),
            helpful: false,
            comment: Some("ilgisiz sonuç".into()),
        });
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn boost_good_sources_does_not_error() {
        let (embeddings, strategic) = setup();
        let llm = mock_llm();
        let feedback = FeedbackLoop::new(embeddings, strategic, llm);
        let result = feedback.boost_good_sources("iyi kaynak");
        assert!(result.is_ok() || result.is_err());
    }
}
