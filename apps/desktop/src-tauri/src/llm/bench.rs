use std::time::Instant;
use crate::llm::OllamaClient;

pub struct BenchResult {
    pub model: String,
    pub latency_ms: u64,
    pub tokens_per_second: f64,
    pub response_length: usize,
}

pub struct LLMBenchmark {
    ollama: OllamaClient,
}

impl LLMBenchmark {
    pub fn new(ollama: OllamaClient) -> Self {
        Self { ollama }
    }

    pub fn run(&self, model: &str, prompt: &str) -> Result<BenchResult, String> {
        let custom = OllamaClient::new(self.ollama.base_url.clone(), model.to_string());
        let start = Instant::now();
        let response = custom.generate_sync(prompt)?;
        let elapsed = start.elapsed();
        let latency_ms = elapsed.as_millis() as u64;
        let response_len = response.len();
        let tokens_per_sec = if latency_ms > 0 {
            (response_len as f64 / latency_ms as f64) * 1000.0
        } else {
            0.0
        };
        Ok(BenchResult {
            model: model.to_string(),
            latency_ms,
            tokens_per_second: tokens_per_sec,
            response_length: response_len,
        })
    }

    pub fn compare_models(&self, models: &[&str], prompt: &str) -> Vec<Result<BenchResult, String>> {
        models.iter().map(|m| self.run(m, prompt)).collect()
    }
}
