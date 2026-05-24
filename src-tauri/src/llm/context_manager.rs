/// Context Manager — Kısa süreli bellek ve bağlam yönetimi (§5).
pub struct ContextManager {
    short_term: Vec<String>,
}

impl ContextManager {
    pub fn new() -> Self {
        Self { short_term: Vec::new() }
    }

    pub fn push(&mut self, entry: String) {
        self.short_term.push(entry);
        if self.short_term.len() > 50 {
            self.short_term.remove(0);
        }
    }

    pub fn context(&self) -> String {
        self.short_term.join("\n")
    }
}
