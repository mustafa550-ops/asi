use crate::db::strategic_memory::{StrategicMemory, StrategicRecord};

pub struct StrategicQuery {
    memory: StrategicMemory,
}

impl StrategicQuery {
    pub fn new(memory: StrategicMemory) -> Self {
        StrategicQuery { memory }
    }

    pub fn historical_signal_performance(&self, signal_type: &str) -> Result<SignalStats, String> {
        let decisions = self.memory.query_by_context(signal_type, 1000)
            .map_err(|e| e.to_string())?;
        if decisions.is_empty() {
            return Ok(SignalStats::default());
        }

        let total = decisions.len() as f64;
        let successes = decisions.iter().filter(|d| d.outcome == "success").count() as f64;
        let failures = decisions.iter().filter(|d| d.outcome == "failure").count() as f64;

        let avg_confidence: f64 = decisions.iter().map(|d| d.confidence).sum::<f64>() / total;

        let recent = decisions.iter()
            .filter(|d| d.outcome == "success")
            .take(5)
            .map(|d| d.decision.clone())
            .collect();

        Ok(SignalStats {
            total_signals: total as i64,
            success_rate: if total > 0.0 { (successes / total) * 100.0 } else { 0.0 },
            failure_rate: if total > 0.0 { (failures / total) * 100.0 } else { 0.0 },
            avg_confidence,
            recent_successes: recent,
        })
    }

    pub fn similar_conditions(&self, context: &str, min_confidence: f64) -> Result<Vec<StrategicRecord>, String> {
        let all = self.memory.query_by_context(context, 100)
            .map_err(|e| e.to_string())?;
        Ok(all.into_iter()
            .filter(|d| d.confidence >= min_confidence)
            .collect())
    }

    pub fn best_strategies(&self, limit: usize) -> Result<Vec<StrategicRecord>, String> {
        let high_conf = self.memory.get_high_confidence(0.8, limit)
            .map_err(|e| e.to_string())?;
        Ok(high_conf)
    }
}

#[derive(Debug, Clone)]
pub struct SignalStats {
    pub total_signals: i64,
    pub success_rate: f64,
    pub failure_rate: f64,
    pub avg_confidence: f64,
    pub recent_successes: Vec<String>,
}

impl Default for SignalStats {
    fn default() -> Self {
        SignalStats {
            total_signals: 0,
            success_rate: 0.0,
            failure_rate: 0.0,
            avg_confidence: 0.0,
            recent_successes: vec![],
        }
    }
}

impl std::fmt::Display for SignalStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Toplam sinyal: {} | Basari: {:.1}% | Basarisizlik: {:.1}% | Ort. guven: {:.2}",
            self.total_signals, self.success_rate, self.failure_rate, self.avg_confidence
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn test_memory() -> StrategicMemory {
        let conn = db::open(std::path::Path::new(":memory:")).unwrap();
        StrategicMemory::new(conn)
    }

    #[test]
    fn empty_db_returns_default_stats() {
        let mem = test_memory();
        let query = StrategicQuery::new(mem);
        let stats = query.historical_signal_performance("BTC_buy").unwrap();
        assert_eq!(stats.total_signals, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[test]
    fn records_are_counted_correctly() {
        let mem = test_memory();
        mem.record("BTC_buy", "buy at 50000", "success", 0.9).unwrap();
        mem.record("BTC_buy", "buy at 48000", "failure", 0.7).unwrap();
        mem.record("BTC_buy", "buy at 45000", "success", 0.8).unwrap();

        let query = StrategicQuery::new(mem);
        let stats = query.historical_signal_performance("BTC_buy").unwrap();
        assert_eq!(stats.total_signals, 3);
        assert!((stats.success_rate - 66.67).abs() < 0.1);
        assert!((stats.failure_rate - 33.33).abs() < 0.1);
    }

    #[test]
    fn similar_conditions_filters_by_confidence() {
        let mem = test_memory();
        mem.record("SXT_analysis", "oversold bounce", "success", 0.9).unwrap();
        mem.record("SXT_analysis", "weak volume", "failure", 0.3).unwrap();

        let query = StrategicQuery::new(mem);
        let results = query.similar_conditions("SXT_analysis", 0.5).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].decision, "oversold bounce");
    }

    #[test]
    fn best_strategies_returns_top() {
        let mem = test_memory();
        mem.record("trend", "signal_a", "success", 0.95).unwrap();
        mem.record("trend", "signal_b", "success", 0.85).unwrap();
        mem.record("trend", "signal_c", "failure", 0.9).unwrap();

        let query = StrategicQuery::new(mem);
        let best = query.best_strategies(2).unwrap();
        assert!(!best.is_empty());
    }
}
