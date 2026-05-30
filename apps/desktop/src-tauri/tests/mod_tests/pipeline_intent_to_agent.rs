use adler_asi_lib::db::embeddings::EmbeddingStore;
use super::helpers::create_test_db_arc;

#[test]
fn db_connection_opens_in_memory() {
    let conn = create_test_db_arc();
    let store = EmbeddingStore::new(conn);
    let result = store.store("test", &[0.1, 0.2, 0.3], "test", "test");
    assert!(result.is_ok(), "Should store in embeddings after schema creation");
}

#[test]
fn db_schema_reentrant_safe() {
    let conn = create_test_db_arc();
    let store = EmbeddingStore::new(conn);
    let r1 = store.store("test", &[0.1, 0.2, 0.3], "test", "test");
    let r2 = store.store("test2", &[0.3, 0.2, 0.1], "test", "test");
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}

#[test]
fn db_keyword_search_after_store() {
    let conn = create_test_db_arc();
    let store = EmbeddingStore::new(conn);
    store.store("rust syntax hatasi", &[0.1, 0.2, 0.3], "test", "test").unwrap();
    let results = store.keyword_search("rust", 5).unwrap();
    assert!(!results.is_empty(), "Keyword search should find 'rust' related content");
}
