pub struct ContextResolver {
    history: Vec<String>,
}

impl ContextResolver {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn push(&mut self, message: &str) {
        self.history.push(message.to_string());
        if self.history.len() > 20 {
            self.history.remove(0);
        }
    }

    pub fn resolve_anaphora(&self, text: &str) -> String {
        let lower = text.to_lowercase();
        if lower.contains("bunu") || lower.contains("onu") || lower.contains("şunu") || lower.contains("su") {
            if let Some(last) = self.history.iter().rev().find(|m| {
                let m = m.to_lowercase();
                m.contains("btc") || m.contains("eth") || m.contains("sxt")
            }) {
                return text.replace("bunu", last)
                    .replace("onu", last)
                    .replace("şunu", last)
                    .replace("su", last);
            }
        }
        text.to_string()
    }

    pub fn has_context(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn get_last_topic(&self) -> Option<String> {
        for msg in self.history.iter().rev() {
            let lower = msg.to_lowercase();
            let keywords = ["btc", "eth", "sxt", "xrp", "ada", "sol", "doge"];
            if let Some(kw) = keywords.iter().find(|k| lower.contains(*k)) {
                return Some(kw.to_string().to_uppercase());
            }
        }
        None
    }
}
