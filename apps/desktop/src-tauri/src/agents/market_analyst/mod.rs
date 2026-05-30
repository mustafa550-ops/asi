pub mod ws;
pub mod indicators;
pub mod signal;
pub mod strategic_query;
pub mod risk;
pub mod paper;
pub mod report;

use super::{Agent, AgentContext};
use std::sync::Mutex;

pub struct MarketAnalyst {
    last_prices: Mutex<Vec<f64>>,
}

impl MarketAnalyst {
    pub fn new() -> Self {
        Self {
            last_prices: Mutex::new(Vec::new()),
        }
    }

    pub fn get_latest_prices(&self, symbol: &str) -> Vec<f64> {
        if let Ok(prices) = ws::fetch_klines(symbol, 100) {
            if let Ok(mut last) = self.last_prices.lock() {
                *last = prices.clone();
            }
            return prices;
        }
        self.last_prices.lock().map(|l| l.clone()).unwrap_or_default()
    }
}

impl Agent for MarketAnalyst {
    fn name(&self) -> String {
        "Market Analyst".into()
    }

    fn description(&self) -> String {
        "Kripto piyasa analizi, Binance WS baglantisi, RSI/MACD gostergeleri ve bottom fishing sinyali".into()
    }

    fn can_handle(&self, task: &str) -> bool {
        task.contains("kripto") || task.contains("borsa") || task.contains("piyasa") || task.contains("analiz") || task.contains("sinyal")
    }

    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let symbol = task.split_whitespace()
            .find(|w| w.len() <= 10 && w.chars().all(|c| c.is_alphanumeric() && c.is_uppercase()))
            .unwrap_or("BTC")
            .to_uppercase();

        let prices = self.get_latest_prices(&symbol);
        if prices.is_empty() {
            return Err(format!("{} icin fiyat verisi alinamadi", symbol));
        }

        let rsi = indicators::rsi(&prices, 14);
        let (macd_line, signal_line) = indicators::macd(&prices, 12, 26, 9);

        let analysis = if let Some(rsi_val) = rsi.last() {
            let direction = if *rsi_val > 70.0 { "asiri-alim (dusus beklenebilir)" }
                else if *rsi_val < 30.0 { "asiri-satim (yukselis beklenebilir)" }
                else { "yatay" };
            format!("RSI(14): {:.1} — {}", rsi_val, direction)
        } else {
            "RSI: yetersiz veri".into()
        };

        let signal_text = signal::check_bottom_fishing(&prices, &rsi, &macd_line, &signal_line);

        let report = format!(
            "[Market Analyst] {} analizi:\n\
             Son fiyat: ${:.4}\n\
             {}\n\
             MACD: {:.4}, Sinyal: {:.4}\n\
             Sinyal: {}",
            symbol,
            prices.last().unwrap_or(&0.0),
            analysis,
            macd_line.last().unwrap_or(&0.0),
            signal_line.last().unwrap_or(&0.0),
            signal_text
        );

        if let Some(memory) = ctx.memory {
            let _ = memory.store_long_term(&report, "MarketAnalyst", "market");
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_returns_correct() {
        let agent = MarketAnalyst::new();
        assert_eq!(agent.name(), "Market Analyst");
    }

    #[test]
    fn can_handle_matches_market_keywords() {
        let agent = MarketAnalyst::new();
        assert!(agent.can_handle("kripto para analiz"));
        assert!(agent.can_handle("borsa durumu"));
        assert!(agent.can_handle("piyasa sinyali"));
        assert!(agent.can_handle("analiz yap"));
        assert!(agent.can_handle("sinyal ver"));
        assert!(!agent.can_handle("sistem durumu"));
    }

    #[test]
    fn new_initializes_empty_prices() {
        let agent = MarketAnalyst::new();
        let prices = agent.last_prices.lock().unwrap();
        assert!(prices.is_empty());
    }
}
