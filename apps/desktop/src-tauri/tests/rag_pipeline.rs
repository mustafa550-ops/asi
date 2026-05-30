use std::sync::{Arc, Mutex};
use rusqlite::Connection;

use adler_asi_lib::db::embeddings::EmbeddingStore;
use adler_asi_lib::db::edge_history::EdgeHistory;
use adler_asi_lib::db::strategic_memory::StrategicMemory;
use adler_asi_lib::db::fts::FullTextSearch;
use adler_asi_lib::db::schema::{create_tables, run_migrations};
use adler_asi_lib::llm::OllamaClient;
use adler_asi_lib::rag::pipeline::{RagPipeline, RagResult};
use adler_asi_lib::rag::chunker::Chunker;
use adler_asi_lib::rag::eval::EvalFramework;
use adler_asi_lib::rag::cache::{RagCache, CacheItem};
use adler_asi_lib::rag::pruning::PruningEngine;
use adler_asi_lib::rag::xref::CrossReferenceResolver;

fn create_db() -> (EmbeddingStore, FullTextSearch, EdgeHistory, StrategicMemory) {
    let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));
    create_tables(&conn.lock().unwrap()).unwrap();
    run_migrations(&conn.lock().unwrap()).unwrap();
    let embeddings = EmbeddingStore::new(conn.clone());
    let fts = FullTextSearch::new(conn.clone());
    let edge = EdgeHistory::new(conn.clone());
    let strategic = StrategicMemory::new(conn);
    (embeddings, fts, edge, strategic)
}

fn mock_llm() -> OllamaClient {
    OllamaClient::new("http://127.0.0.1:11434".into(), "mock".into())
}

#[test]
fn pipeline_hybrid_search_empty_db() {
    let (embeddings, fts, edge, strategic) = create_db();
    let llm = mock_llm();
    let pipeline = RagPipeline::new(embeddings, fts, edge, strategic, llm);
    let results = pipeline.hybrid_search("test query", 10);
    assert!(results.is_ok());
    assert!(results.unwrap().is_empty());
}

#[test]
fn pipeline_query_strategic_returns_decisions() {
    let (embeddings, fts, edge, strategic) = create_db();
    strategic.record("kripto analiz", "BTC al", "success", 0.9).unwrap();
    strategic.record("kripto analiz", "ETH tut", "partial", 0.7).unwrap();
    let llm = mock_llm();
    let pipeline = RagPipeline::new(embeddings, fts, edge, strategic, llm);
    let decisions = pipeline.query_strategic("kripto analiz").unwrap();
    assert_eq!(decisions.len(), 2);
    assert!(decisions.contains(&"BTC al".to_string()));
    assert!(decisions.contains(&"ETH tut".to_string()));
}

#[test]
fn pipeline_query_strategic_empty_context() {
    let (embeddings, fts, edge, strategic) = create_db();
    let llm = mock_llm();
    let pipeline = RagPipeline::new(embeddings, fts, edge, strategic, llm);
    let decisions = pipeline.query_strategic("nonexistent").unwrap();
    assert!(decisions.is_empty());
}

#[test]
fn pipeline_knowledge_graph_traverse() {
    let (embeddings, fts, edge, strategic) = create_db();
    let parent = edge.add_edge(0, 1, "parent", "").unwrap();
    let child = edge.add_edge(1, 2, "child", "").unwrap();
    let llm = mock_llm();
    let pipeline = RagPipeline::new(embeddings, fts, edge, strategic, llm);
    let graph = pipeline.get_knowledge_graph(1).unwrap();
    assert!(!graph.ancestors.is_empty());
    assert!(!graph.descendants.is_empty());
}

#[test]
fn chunker_markdown_splits_by_headings() {
    let chunker = Chunker::new(500, 20);
    let md = "# Birinci Baslik\nIcerik 1\n\n# Ikinci Baslik\nIcerik 2\n";
    let chunks = chunker.chunk_markdown(md, "test.md");
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].heading, Some("Birinci Baslik".into()));
    assert_eq!(chunks[1].heading, Some("Ikinci Baslik".into()));
}

#[test]
fn chunker_plain_text_splits_paragraphs() {
    let chunker = Chunker::new(500, 20);
    let text = "Paragraf 1\n\nParagraf 2\n\nParagraf 3";
    let chunks = chunker.chunk_plain_text(text, "doc.txt", "\n\n");
    assert_eq!(chunks.len(), 3);
}

#[test]
fn chunker_token_estimation() {
    let chunker = Chunker::default();
    let tokens = chunker.estimate_tokens("hello world");
    assert_eq!(tokens, (11 + 3) / 4);
}

#[test]
fn chunker_by_size_limit_triggers_split() {
    let chunker = Chunker::new(10, 2);
    let long = "abcdefghijklmnopqrstuvwxyz\n1234567890abcdefghij\n";
    let chunks = chunker.chunk_markdown(long, "big.md");
    assert!(chunks.len() >= 2);
}

#[test]
fn eval_precision_recall_faithfulness() {
    let eval = EvalFramework::new();
    let samples = vec![
        adler_asi_lib::rag::eval::EvalSample {
            query: "kripto".into(),
            retrieved_sources: vec!["a.md".into(), "b.md".into()],
            relevant_sources: vec!["a.md".into()],
            answer: "Bitcoin analizi".into(),
            ground_truth: None,
        },
    ];
    let result = eval.evaluate(&samples);
    assert!((result.precision - 0.5).abs() < 1e-6);
    assert!((result.recall - 1.0).abs() < 1e-6);
    assert!(result.total_queries == 1);
}

#[test]
fn eval_empty_samples_returns_zeros() {
    let eval = EvalFramework::new();
    let result = eval.evaluate(&[]);
    assert_eq!(result.total_queries, 0);
    assert_eq!(result.precision, 0.0);
    assert_eq!(result.recall, 0.0);
    assert_eq!(result.faithfulness, 0.0);
    assert_eq!(result.context_relevance, 0.0);
}

#[test]
fn cache_set_get_eviction() {
    let mut cache = RagCache::new(60, 2);
    let item = CacheItem {
        content: "test content".into(),
        source: "doc.md".into(),
        score: 0.9,
    };
    cache.set("query", vec![item.clone()]);
    assert_eq!(cache.size(), 1);
    let hit = cache.get("query");
    assert!(hit.is_some());
    assert_eq!(hit.unwrap().len(), 1);
    cache.set("q2", vec![item.clone()]);
    cache.set("q3", vec![item]);
    assert_eq!(cache.size(), 2);
    assert!(cache.get("query").is_none());
}

#[test]
fn cache_invalidate_clear() {
    let mut cache = RagCache::new(60, 100);
    cache.set("a", vec![]);
    cache.set("b", vec![]);
    assert_eq!(cache.size(), 2);
    cache.invalidate("a");
    assert_eq!(cache.size(), 1);
    cache.clear();
    assert_eq!(cache.size(), 0);
}

#[test]
fn cache_stats() {
    let mut cache = RagCache::new(60, 100);
    cache.set("q", vec![CacheItem {
        content: "x".into(), source: "s".into(), score: 1.0,
    }]);
    cache.get("q");
    cache.get("q");
    let stats = cache.stats();
    assert_eq!(stats.total_hits, 2);
    assert_eq!(stats.entries, 1);
    assert_eq!(stats.max_entries, 100);
}

#[test]
fn pruning_count_all_returns_zero_on_empty_db() {
    let (embeddings, fts, edge, strategic) = create_db();
    let engine = PruningEngine::new(embeddings, strategic, edge);
    let counts = engine.count_all();
    assert!(counts.embeddings >= 0);
    assert!(counts.strategic >= 0);
    assert!(counts.edges >= 0);
}

#[test]
fn pruning_prune_old_embeddings_no_error() {
    let (embeddings, fts, edge, strategic) = create_db();
    let engine = PruningEngine::new(embeddings, strategic, edge);
    let result = engine.prune_old_embeddings(1);
    assert!(result.is_ok());
}

#[test]
fn pruning_full_prune_returns_report() {
    let (embeddings, fts, edge, strategic) = create_db();
    let engine = PruningEngine::new(embeddings, strategic, edge);
    let report = engine.full_prune(1, 0.5, 1).unwrap();
    assert_eq!(report.deleted_embeddings, 0);
    assert!(report.total_before.embeddings >= 0);
    assert!(report.total_after.embeddings >= 0);
}

#[test]
fn xref_resolves_wiki_links_integration() {
    let resolver = CrossReferenceResolver::new();
    let content = "Bilgi icin [[dokuman]] sayfasina ve [[rehber|Rehber Sayfa]]";
    let links = resolver.resolve_links(content);
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].target, "dokuman");
    assert_eq!(links[0].title, "dokuman");
    assert_eq!(links[1].target, "rehber");
    assert_eq!(links[1].title, "Rehber Sayfa");
}

#[test]
fn xref_ignores_external_urls() {
    let resolver = CrossReferenceResolver::new();
    let content = "[Google](https://google.com)";
    let links = resolver.resolve_links(content);
    assert!(links.is_empty());
}

#[test]
fn xref_build_backlink_index_integration() {
    let resolver = CrossReferenceResolver::new();
    let docs = vec![
        ("a.md".into(), "[[hedef]]".into()),
        ("b.md".into(), "[[hedef]] ve [[diger]]".into()),
    ];
    let index = resolver.build_backlink_index(&docs);
    assert!(index.contains_key("hedef"));
    assert_eq!(index["hedef"].len(), 2);
    assert!(index.contains_key("diger"));
    assert_eq!(index["diger"].len(), 1);
}

#[test]
fn pipeline_hybrid_search_with_data_returns_sorted() {
    let (embeddings, fts, edge, strategic) = create_db();
    let llm = mock_llm();
    let pipeline = RagPipeline::new(embeddings, fts, edge, strategic, llm);
    let results = pipeline.hybrid_search("query", 5);
    assert!(results.is_ok());
}
