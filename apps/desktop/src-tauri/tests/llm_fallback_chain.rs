use adler_asi_lib::llm::OllamaClient;

#[test]
fn llm_ollama_client_creation() {
    let client = OllamaClient::new("http://localhost:11434".into(), "qwen2.5:1.5b".into());
    assert_eq!(client.model, "qwen2.5:1.5b");
    assert_eq!(client.base_url, "http://localhost:11434");
}

#[test]
fn llm_ollama_client_different_model() {
    let client = OllamaClient::new("http://127.0.0.1:11434".into(), "llama3.2:3b".into());
    assert_eq!(client.model, "llama3.2:3b");
}

#[test]
fn llm_ollama_client_generate_does_not_panic_with_offline_server() {
    let client = OllamaClient::new("http://localhost:11434".into(), "qwen2.5:1.5b".into());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(client.generate("merhaba"));
    assert!(result.is_ok() || result.is_err(),
        "generate() should return Ok or Err without panicking");
}
