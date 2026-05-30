use std::sync::atomic::{AtomicU64, Ordering};

pub struct TokenUsage {
    prompt_tokens: AtomicU64,
    completion_tokens: AtomicU64,
    total_requests: AtomicU64,
    failed_requests: AtomicU64,
}

impl TokenUsage {
    pub fn new() -> Self {
        Self {
            prompt_tokens: AtomicU64::new(0),
            completion_tokens: AtomicU64::new(0),
            total_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
        }
    }

    pub fn record(&self, prompt_tokens: u64, completion_tokens: u64, success: bool) {
        self.prompt_tokens.fetch_add(prompt_tokens, Ordering::Relaxed);
        self.completion_tokens.fetch_add(completion_tokens, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        if !success {
            self.failed_requests.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn stats(&self) -> UsageStats {
        UsageStats {
            prompt_tokens: self.prompt_tokens.load(Ordering::Relaxed),
            completion_tokens: self.completion_tokens.load(Ordering::Relaxed),
            total_tokens: self.prompt_tokens.load(Ordering::Relaxed) + self.completion_tokens.load(Ordering::Relaxed),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
        }
    }

    pub fn report(&self) -> String {
        let s = self.stats();
        format!(
            "LLM Kullanim:\n  Toplam istek: {}\n  Prompt token: {}\n  Yanit token: {}\n  Toplam token: {}\n  Basarisiz: {} ({:.1}%)",
            s.total_requests,
            s.prompt_tokens,
            s.completion_tokens,
            s.total_tokens,
            s.failed_requests,
            if s.total_requests > 0 { s.failed_requests as f64 / s.total_requests as f64 * 100.0 } else { 0.0 },
        )
    }

    pub fn estimate_cost(&self, cost_per_million_tokens: f64) -> f64 {
        let s = self.stats();
        s.total_tokens as f64 / 1_000_000.0 * cost_per_million_tokens
    }
}

pub struct UsageStats {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    pub total_requests: u64,
    pub failed_requests: u64,
}
