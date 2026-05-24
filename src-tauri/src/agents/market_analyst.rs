use super::{Agent, AgentContext};

pub struct MarketAnalyst;

impl Agent for MarketAnalyst {
    fn name(&self) -> String { "Market Analyst".into() }
    fn description(&self) -> String { "Kripto piyasa analizi ve sinyal üretimi".into() }
    fn can_handle(&self, task: &str) -> bool {
        task.contains("kripto") || task.contains("borsa") || task.contains("piyasa")
    }
    fn execute(&self, task: &str, ctx: &AgentContext) -> Result<String, String> {
        let symbol = task.split_whitespace().find(|w| w.len() <= 10 && w.chars().all(|c| c.is_alphanumeric() && c.is_uppercase()))
            .unwrap_or("BTC");
        let prompt = format!(
            "Kripto para {} için kısa bir piyasa analizi yap.\n\
             Trend: yükseliş/düşüş/yatay\nDestek/Direnç seviyeleri\nRisk seviyesi: düşük/orta/yüksek\n\
             Maximum 3 cümle.", symbol
        );
        let analysis = ctx.ollama.generate_sync("qwen2.5:1.5b", &prompt)?;
        if let Some(memory) = ctx.memory {
            let _ = memory.store_long_term(&format!("Market analysis for {}: {}", symbol, analysis), "MarketAnalyst", "market");
        }
        Ok(format!("[Market Analyst] {} analizi:\n{}", symbol, analysis.trim()))
    }
}
