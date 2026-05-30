use std::collections::HashMap;
use crate::nlu::intent::Intent;

pub struct IntentAnalytics {
    total: u64,
    correct: u64,
    by_intent: HashMap<String, IntentStats>,
    confusion: HashMap<String, HashMap<String, u64>>,
}

#[derive(Default, Clone)]
pub struct IntentStats {
    pub count: u64,
    pub correct: u64,
}

pub struct PerClassMetrics {
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub support: u64,
}

impl IntentAnalytics {
    pub fn new() -> Self {
        Self {
            total: 0,
            correct: 0,
            by_intent: HashMap::new(),
            confusion: HashMap::new(),
        }
    }

    pub fn record_with_feedback(&mut self, predicted: &Intent, actual: &Intent, confidence: f32) {
        let pkey = predicted.as_str().to_string();
        let akey = actual.as_str().to_string();
        let was_correct = predicted == actual;

        self.total += 1;
        if was_correct {
            self.correct += 1;
        }

        let stats = self.by_intent.entry(pkey.clone()).or_default();
        stats.count += 1;
        if was_correct {
            stats.correct += 1;
        }

        let row = self.confusion.entry(pkey).or_default();
        *row.entry(akey).or_insert(0) += 1;
    }

    pub fn record(&mut self, intent: &Intent, confidence: f32) {
        let was_correct = confidence > 0.6;
        self.total += 1;
        if was_correct {
            self.correct += 1;
        }
        let key = intent.as_str().to_string();
        let stats = self.by_intent.entry(key.clone()).or_default();
        stats.count += 1;
        if was_correct {
            stats.correct += 1;
        }
        let row = self.confusion.entry(key).or_default();
        let actual_key = intent.as_str().to_string();
        let counter = row.entry(actual_key).or_insert(0);
        *counter += 1;
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

    pub fn per_class_metrics(&self, intent_label: &str) -> PerClassMetrics {
        let support = self.by_intent.get(intent_label).map(|s| s.count).unwrap_or(0);
        let tp = self.by_intent.get(intent_label).map(|s| s.correct).unwrap_or(0);

        let mut fp: u64 = 0;
        for (pred, row) in &self.confusion {
            if pred != intent_label {
                if let Some(count) = row.get(intent_label) {
                    fp += count;
                }
            }
        }

        let mut fn_count: u64 = 0;
        if let Some(row) = self.confusion.get(intent_label) {
            for (actual, count) in row {
                if actual != intent_label {
                    fn_count += count;
                }
            }
        }

        let precision = if tp + fp == 0 { 0.0 } else { tp as f64 / (tp + fp) as f64 };
        let recall = if tp + fn_count == 0 { 0.0 } else { tp as f64 / (tp + fn_count) as f64 };
        let f1 = if precision + recall == 0.0 { 0.0 } else { 2.0 * precision * recall / (precision + recall) };

        PerClassMetrics { precision, recall, f1, support }
    }

    pub fn macro_f1(&self) -> f64 {
        let keys: Vec<String> = self.by_intent.keys().cloned().collect();
        if keys.is_empty() { return 0.0; }
        let total: f64 = keys.iter().map(|k| self.per_class_metrics(k).f1).sum();
        total / keys.len() as f64
    }

    pub fn report(&self) -> String {
        let mut lines = vec![
            format!("Intent Dogruluk Raporu"),
            format!("  Toplam: {}, Dogru: {}, Oran: {:.1}%", self.total, self.correct, self.accuracy() * 100.0),
            format!("  Macro F1: {:.3}", self.macro_f1()),
            format!("  Intent Bazinda:"),
        ];
        let mut sorted: Vec<_> = self.by_intent.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        for (intent, stats) in sorted {
            let rate = if stats.count == 0 { 0.0 } else { stats.correct as f64 / stats.count as f64 };
            let m = self.per_class_metrics(intent);
            lines.push(format!(
                "    {}: {}/{} ({:.1}%) p={:.2} r={:.2} f1={:.3}",
                intent, stats.correct, stats.count, rate * 100.0,
                m.precision, m.recall, m.f1
            ));
        }
        lines.join("\n")
    }

    pub fn confusion_matrix(&self) -> String {
        let mut lines = vec!["Karmasiklik Matrisi:".to_string()];
        let mut keys: Vec<&String> = self.confusion.keys().collect();
        keys.sort();
        lines.push(format!("  {:12}", "") + &keys.iter().map(|k| format!("{:12}", k)).collect::<Vec<_>>().join(""));
        for pred in &keys {
            let row = self.confusion.get(*pred).unwrap();
            let mut row_str = format!("  {:12}", *pred);
            for actual in &keys {
                let count = row.get(*actual).copied().unwrap_or(0);
                row_str += &format!("{:12}", count);
            }
            lines.push(row_str);
        }
        lines.join("\n")
    }
}
