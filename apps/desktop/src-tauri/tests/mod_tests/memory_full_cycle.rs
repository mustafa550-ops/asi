use adler_asi_lib::db::embeddings::EmbeddingStore;
use adler_asi_lib::db::strategic_memory::StrategicMemory;
use adler_asi_lib::db::edge_history::EdgeHistory;
use super::helpers::create_test_db_arc;

#[test]
fn memory_embedding_store_and_search() {
    let conn = create_test_db_arc();
    let store = EmbeddingStore::new(conn);
    store.store("test content", &[0.1, 0.2, 0.3], "unit_test", "test").unwrap();
    let results = store.search(&[0.1, 0.2, 0.3], 5).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn memory_embedding_empty_store_search() {
    let conn = create_test_db_arc();
    let store = EmbeddingStore::new(conn);
    let results = store.search(&[0.5, 0.5, 0.5], 5).unwrap();
    assert!(results.is_empty());
}

#[test]
fn memory_strategic_record_and_query() {
    let conn = create_test_db_arc();
    let strategic = StrategicMemory::new(conn);
    strategic.record("test context", "buy decision", "success", 0.8).unwrap();
    let records = strategic.query_by_context("test", 10).unwrap();
    assert!(!records.is_empty());
}

#[test]
fn memory_strategic_update_outcome() {
    let conn = create_test_db_arc();
    let strategic = StrategicMemory::new(conn);
    let id = strategic.record("ctx", "dec", "success", 0.5).unwrap();
    strategic.update_outcome(id, "success", 0.9).unwrap();
    let records = strategic.query_by_context("ctx", 10).unwrap();
    assert!(!records.is_empty());
}

#[test]
fn memory_strategic_boost_confidence() {
    let conn = create_test_db_arc();
    let strategic = StrategicMemory::new(conn);
    let id = strategic.record("ctx2", "dec2", "success", 0.5).unwrap();
    strategic.boost_confidence(id, 0.3).unwrap();
    let records = strategic.get_high_confidence(0.7, 10).unwrap();
    assert!(!records.is_empty());
}

#[test]
fn memory_edge_add_and_get_children() {
    let conn = create_test_db_arc();
    let edges = EdgeHistory::new(conn);
    edges.add_edge(1, 2, "skill_evolution", "added feature").unwrap();
    let children = edges.get_children(1).unwrap();
    assert!(!children.is_empty());
    assert_eq!(children[0].child_id, 2);
}

#[test]
fn memory_edge_no_children_for_nonexistent_parent() {
    let conn = create_test_db_arc();
    let edges = EdgeHistory::new(conn);
    let children = edges.get_children(999).unwrap();
    assert!(children.is_empty());
}
