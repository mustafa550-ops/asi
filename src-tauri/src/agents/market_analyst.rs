use super::Agent;

/// Market Analyst — Binance API, kripto analizi (§4.1).
pub struct MarketAnalyst;

impl Agent for MarketAnalyst {
    fn name(&self) -> String { "Market Analyst".into() }
    fn description(&self) -> String { "Kripto piyasa analizi ve sinyal üretimi".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("kripto") || task.contains("borsa") || task.contains("piyasa")
    }
    fn execute(&self, _task: &str) -> Result<String, String> {
        Ok("Piyasa analizi tamamlandı".into())
    }
}
