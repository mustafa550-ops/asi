use crate::agents::market_analyst::paper::{PaperPortfolio, Trade};
use crate::agents::market_analyst::risk::RiskManager;
use crate::agents::market_analyst::signal::Signal;
use std::collections::HashMap;

pub struct MarketReport {
    pub symbol: String,
    pub current_price: f64,
    pub rsi: f64,
    pub macd: f64,
    pub macd_signal: f64,
    pub signal: String,
    pub risk_score: f64,
    pub portfolio: Option<PortfolioSummary>,
    pub analysis_text: String,
}

pub struct PortfolioSummary {
    pub balance: f64,
    pub equity: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub total_pnl: f64,
    pub open_positions: usize,
    pub trade_count: usize,
}

impl MarketReport {
    pub fn to_markdown(&self) -> String {
        let header = format!("# Piyasa Raporu: {}\n", self.symbol);
        let prices = format!(
            "## Fiyat\n- Guncel: **${:.4}**\n- RSI(14): **{:.1}**\n- MACD: **{:.4}** (sinyal: {:.4})\n",
            self.current_price, self.rsi, self.macd, self.macd_signal
        );
        let signal_line = format!("## Sinyal\n{}\n", self.signal);
        let risk_line = format!("## Risk\n- Risk Skoru: **{:.1}/100**\n", self.risk_score);
        let analysis = format!("## Analiz\n{}\n", self.analysis_text);
        let portfolio = if let Some(p) = &self.portfolio {
            format!(
                "\n## Portfoy\n- Bakiye: **${:.2}**\n- Sermaye: **${:.2}**\n- Gerceklesmemis K/Z: **${:.2}**\n- Gerceklesmis K/Z: **${:.2}**\n- Toplam K/Z: **${:.2}**\n- Acik Pozisyon: **{}**\n- Toplam Islem: **{}**\n",
                p.balance, p.equity, p.unrealized_pnl, p.realized_pnl, p.total_pnl, p.open_positions, p.trade_count
            )
        } else {
            String::new()
        };

        format!(
            "---\n{}\n{}{}{}{}{}",
            header, prices, signal_line, risk_line, analysis, portfolio
        )
    }

    pub fn to_json(&self) -> String {
        let mut fields = vec![
            format!("\"symbol\": \"{}\"", self.symbol),
            format!("\"current_price\": {:.4}", self.current_price),
            format!("\"rsi\": {:.1}", self.rsi),
            format!("\"macd\": {:.4}", self.macd),
            format!("\"macd_signal\": {:.4}", self.macd_signal),
            format!("\"signal\": \"{}\"", self.signal.replace('\"', "\\\"")),
            format!("\"risk_score\": {:.1}", self.risk_score),
            format!("\"analysis\": \"{}\"", self.analysis_text.replace('\"', "\\\"")),
        ];
        if let Some(p) = &self.portfolio {
            fields.push(format!("\"balance\": {:.2}", p.balance));
            fields.push(format!("\"equity\": {:.2}", p.equity));
            fields.push(format!("\"unrealized_pnl\": {:.2}", p.unrealized_pnl));
            fields.push(format!("\"realized_pnl\": {:.2}", p.realized_pnl));
            fields.push(format!("\"total_pnl\": {:.2}", p.total_pnl));
            fields.push(format!("\"open_positions\": {}", p.open_positions));
            fields.push(format!("\"trade_count\": {}", p.trade_count));
        }
        format!("{{\n  {}\n}}", fields.join(",\n  "))
    }
}

pub fn build_report(
    symbol: &str,
    prices: &[f64],
    rsi_values: &[f64],
    macd_line: &[f64],
    signal_line: &[f64],
    signal_str: &str,
    risk_manager: &RiskManager,
    portfolio: Option<&PaperPortfolio>,
    market_prices: &HashMap<String, f64>,
) -> MarketReport {
    let current_price = *prices.last().unwrap_or(&0.0);
    let rsi = *rsi_values.last().unwrap_or(&50.0);
    let macd = *macd_line.last().unwrap_or(&0.0);
    let macd_sig = *signal_line.last().unwrap_or(&0.0);

    let dummy = Signal::Hold { reason: "report".into() };
    let risk_score = risk_manager.risk_score(&dummy, prices);

    let direction = if rsi > 70.0 {
        "Asiri-alim bolgesi, dusus beklenebilir"
    } else if rsi < 30.0 {
        "Asiri-satim bolgesi, yukselis beklenebilir"
    } else {
        "Yatay seyir"
    };
    let analysis_text = format!(
        "{} icin RSI {:.1} ile {}. Fiyat ${:.4}, MACD {:.4}. {}",
        symbol, rsi, direction, current_price, macd, signal_str
    );

    let portfolio_summary = portfolio.map(|p| PortfolioSummary {
        balance: p.balance,
        equity: p.total_equity(market_prices),
        unrealized_pnl: p.unrealized_pnl(market_prices),
        realized_pnl: p.realized_pnl(),
        total_pnl: p.total_pnl(market_prices),
        open_positions: p.holdings.len(),
        trade_count: p.trades.len(),
    });

    MarketReport {
        symbol: symbol.to_string(),
        current_price,
        rsi,
        macd,
        macd_signal: macd_sig,
        signal: signal_str.to_string(),
        risk_score,
        portfolio: portfolio_summary,
        analysis_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::market_analyst::paper::PaperPortfolio;

    #[test]
    fn report_to_markdown_contains_symbol() {
        let report = MarketReport {
            symbol: "BTC".into(),
            current_price: 50000.0,
            rsi: 45.0,
            macd: 0.001,
            macd_signal: 0.002,
            signal: "bekleme".into(),
            risk_score: 30.0,
            portfolio: None,
            analysis_text: "Test analiz".into(),
        };
        let md = report.to_markdown();
        assert!(md.contains("BTC"));
        assert!(md.contains("50000.0"));
        assert!(md.contains("30.0"));
    }

    #[test]
    fn report_to_markdown_with_portfolio() {
        let mut pf = PaperPortfolio::new(10000.0);
        let mut prices = HashMap::new();
        prices.insert("BTC".into(), 50000.0);
        pf.buy("BTC", 50000.0, 0.1).unwrap();

        let report = MarketReport {
            symbol: "BTC".into(),
            current_price: 50000.0,
            rsi: 50.0,
            macd: 0.0,
            macd_signal: 0.0,
            signal: "bekleme".into(),
            risk_score: 20.0,
            portfolio: Some(PortfolioSummary {
                balance: pf.balance,
                equity: pf.total_equity(&prices),
                unrealized_pnl: pf.unrealized_pnl(&prices),
                realized_pnl: pf.realized_pnl(),
                total_pnl: pf.total_pnl(&prices),
                open_positions: pf.holdings.len(),
                trade_count: pf.trades.len(),
            }),
            analysis_text: "Test".into(),
        };
        let md = report.to_markdown();
        assert!(md.contains("Portfoy"));
        assert!(md.contains("10000.0"));
    }

    #[test]
    fn report_to_json_contains_symbol() {
        let report = MarketReport {
            symbol: "ETH".into(),
            current_price: 3000.0,
            rsi: 55.0,
            macd: 0.01,
            macd_signal: 0.015,
            signal: "AL".into(),
            risk_score: 40.0,
            portfolio: None,
            analysis_text: "ETH guclu".into(),
        };
        let json = report.to_json();
        assert!(json.contains("\"symbol\": \"ETH\""));
        assert!(json.contains("\"risk_score\": 40.0"));
    }

    #[test]
    fn build_report_creates_valid_report() {
        let prices = vec![100.0; 50];
        let rsi = vec![50.0; 50];
        let macd = vec![0.0; 50];
        let signal = vec![0.0; 50];
        let rm = RiskManager::new(10000.0);
        let market_prices = HashMap::new();

        let report = build_report("TEST", &prices, &rsi, &macd, &signal, "bekleme", &rm, None, &market_prices);
        assert_eq!(report.symbol, "TEST");
        assert!((report.current_price - 100.0).abs() < 0.001);
        assert!((report.rsi - 50.0).abs() < 0.001);
    }

    #[test]
    fn build_report_with_portfolio() {
        let prices = vec![100.0; 50];
        let rsi = vec![50.0; 50];
        let macd = vec![0.0; 50];
        let signal = vec![0.0; 50];
        let rm = RiskManager::new(10000.0);
        let mut pf = PaperPortfolio::new(10000.0);
        let mut market_prices = HashMap::new();
        market_prices.insert("TEST".into(), 100.0);
        pf.buy("TEST", 100.0, 10.0).unwrap();

        let report = build_report("TEST", &prices, &rsi, &macd, &signal, "bekleme", &rm, Some(&pf), &market_prices);
        assert!(report.portfolio.is_some());
        let p = report.portfolio.unwrap();
        assert!((p.balance - 9000.0).abs() < 0.001);
    }
}
