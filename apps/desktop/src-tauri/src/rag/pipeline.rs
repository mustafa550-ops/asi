use crate::db::embeddings::EmbeddingStore;
use crate::db::edge_history::EdgeHistory;
use crate::db::strategic_memory::StrategicMemory;
use crate::db::fts::FullTextSearch;
use crate::llm::OllamaClient;

pub struct RagPipeline {
    embeddings: EmbeddingStore,
    fts: FullTextSearch,
    edge: EdgeHistory,
    strategic: StrategicMemory,
    llm: OllamaClient,
}

#[derive(serde::Serialize)]
pub struct RagResult {
    pub content: String,
    pub source: String,
    pub score: f32,
    pub method: String,
}

#[derive(serde::Serialize)]
pub struct GraphData {
    pub ancestors: Vec<crate::db::edge_history::EdgeRecord>,
    pub descendants: Vec<crate::db::edge_history::EdgeRecord>,
}

impl RagPipeline {
    pub fn new(
        embeddings: EmbeddingStore,
        fts: FullTextSearch,
        edge: EdgeHistory,
        strategic: StrategicMemory,
        llm: OllamaClient,
    ) -> Self {
        Self { embeddings, fts, edge, strategic, llm }
    }

    pub fn hybrid_search(&self, query: &str, limit: usize) -> Result<Vec<RagResult>, String> {
        let mut results = Vec::new();

        if let Ok(embedding) = self.llm.embedding_sync(query) {
            if let Ok(vec_results) = self.embeddings.search(&embedding, limit) {
                for r in vec_results {
                    results.push(RagResult {
                        content: r.content,
                        source: r.source,
                        score: r.score,
                        method: "vector".into(),
                    });
                }
            }
        }

        if let Ok(fts_results) = self.fts.search(query, limit) {
            for r in fts_results {
                results.push(RagResult {
                    content: r.content,
                    source: r.source,
                    score: r.rank as f32,
                    method: "fts".into(),
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }

    pub fn query_strategic(&self, context: &str) -> Result<Vec<String>, String> {
        let records = self.strategic.query_by_context(context, 5)
            .map_err(|e| e.to_string())?;
        Ok(records.into_iter().map(|r| r.decision).collect())
    }

    pub fn get_knowledge_graph(&self, node_id: i64) -> Result<GraphData, String> {
        let ancestors = self.edge.traverse_ancestors(node_id, 10)
            .map_err(|e| e.to_string())?;
        let descendants = self.edge.traverse_descendants(node_id, 10)
            .map_err(|e| e.to_string())?;
        Ok(GraphData { ancestors, descendants })
    }
}
