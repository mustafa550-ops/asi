use std::collections::HashMap;
use crate::nlu::intent::Intent;

pub struct IntentAnalytics {
    total: u64,
    correct: u64,
    by_intent: HashMap<String, IntentStats>,
}

#[derive(Default, Clone)]
pub struct IntentStats {
    pub count: u64,
    pub correct: u64,
}

impl IntentAnalytics {
    pub fn new() -> Self {
        Self {
            total: 0,
            correct: 0,
            by_intent: HashMap::new(),
        }
    }

    pub fn record(&mut self, intent: &Intent, was_correct: bool) {
        self.total += 1;
        if was_correct {
            self.correct += 1;
        }
        let key = intent.as_str().to_string();
        let stats = self.by_intent.entry(key).or_default();
        stats.count += 1;
        if was_correct {
            stats.correct += 1;
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total == 0 { return 0.0; }
        self.correct as f64 / self.total as f64
    }

    pub fn intent_accuracy(&self, intent: &Intent) -> f64 {
        let key = intent.as_str();
        self.by_intent.get(key)
            .map(|s| if s.count == 0 { 0.0 } else { s.correct as f64 / s.count as f64 })
            .unwrap_or(0.0)
    }

    pub fn report(&self) -> String {
        let mut lines = vec![
            format!("Intent Dogruluk Raporu"),
            format!("  Toplam: {}, Dogru: {}, Oran: {:.1}%", self.total, self.correct, self.accuracy() * 100.0),
            format!("  Intent Bazinda:"),
        ];
        let mut sorted: Vec<_> = self.by_intent.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        for (intent, stats) in sorted {
            let rate = if stats.count == 0 { 0.0 } else { stats.correct as f64 / stats.count as f64 };
            lines.push(format!("    {}: {}/{} ({:.1}%)", intent, stats.correct, stats.count, rate * 100.0));
        }
        lines.join("\n")
    }
}
