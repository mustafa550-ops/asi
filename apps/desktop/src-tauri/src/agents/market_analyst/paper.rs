use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: u64,
    pub symbol: String,
    pub side: TradeSide,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub quantity: f64,
    pub pnl: Option<f64>,
    pub pnl_pct: Option<f64>,
    pub opened_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradeSide {
    Long,
    Short,
}

#[derive(Debug, Clone)]
pub struct Holding {
    pub symbol: String,
    pub quantity: f64,
    pub avg_entry: f64,
}

pub struct PaperPortfolio {
    pub balance: f64,
    pub initial_balance: f64,
    pub holdings: HashMap<String, Holding>,
    pub trades: Vec<Trade>,
    next_trade_id: u64,
}

impl PaperPortfolio {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            initial_balance,
            holdings: HashMap::new(),
            trades: Vec::new(),
            next_trade_id: 1,
        }
    }

    pub fn total_equity(&self, prices: &HashMap<String, f64>) -> f64 {
        let holdings_value: f64 = self.holdings.values()
            .map(|h| {
                let price = prices.get(&h.symbol).unwrap_or(&h.avg_entry);
                h.quantity * price
            })
            .sum();
        self.balance + holdings_value
    }

    pub fn unrealized_pnl(&self, prices: &HashMap<String, f64>) -> f64 {
        self.holdings.values()
            .map(|h| {
                let price = prices.get(&h.symbol).unwrap_or(&h.avg_entry);
                (price - h.avg_entry) * h.quantity
            })
            .sum()
    }

    pub fn realized_pnl(&self) -> f64 {
        self.trades.iter().filter_map(|t| t.pnl).sum()
    }

    pub fn total_pnl(&self, prices: &HashMap<String, f64>) -> f64 {
        self.realized_pnl() + self.unrealized_pnl(prices)
    }

    pub fn buy(&mut self, symbol: &str, price: f64, quantity: f64) -> Result<Trade, String> {
        let cost = price * quantity;
        if cost > self.balance {
            return Err(format!(
                "Yetersiz bakiye: {:.2} gerekiyor, {:.2} mevcut",
                cost, self.balance
            ));
        }
        self.balance -= cost;
        let entry = self.holdings.entry(symbol.to_string()).or_insert(Holding {
            symbol: symbol.to_string(),
            quantity: 0.0,
            avg_entry: 0.0,
        });
        let total_qty = entry.quantity + quantity;
        entry.avg_entry = (entry.avg_entry * entry.quantity + price * quantity) / total_qty;
        entry.quantity = total_qty;

        let trade = Trade {
            id: self.next_trade_id,
            symbol: symbol.to_string(),
            side: TradeSide::Long,
            entry_price: price,
            exit_price: None,
            quantity,
            pnl: None,
            pnl_pct: None,
            opened_at: chrono_now(),
            closed_at: None,
        };
        self.next_trade_id += 1;
        self.trades.push(trade.clone());
        Ok(trade)
    }

    pub fn sell(&mut self, symbol: &str, price: f64, quantity: f64) -> Result<Trade, String> {
        let (avg_entry, remaining_qty) = {
            let holding = self.holdings.get_mut(symbol)
                .ok_or_else(|| format!("{} portfoyde bulunamadi", symbol))?;
            if quantity > holding.quantity {
                return Err(format!(
                    "Yetersiz miktar: {} portfoyde, {} satilmak isteniyor",
                    holding.quantity, quantity
                ));
            }
            let avg_entry = holding.avg_entry;
            holding.quantity -= quantity;
            (avg_entry, holding.quantity)
        };

        let proceeds = price * quantity;
        let cost_basis = avg_entry * quantity;
        let pnl = proceeds - cost_basis;
        let pnl_pct = (pnl / cost_basis) * 100.0;

        self.balance += proceeds;
        if remaining_qty.abs() < 1e-10 {
            self.holdings.remove(symbol);
        }

        let trade = Trade {
            id: self.next_trade_id,
            symbol: symbol.to_string(),
            side: TradeSide::Short,
            entry_price: avg_entry,
            exit_price: Some(price),
            quantity,
            pnl: Some(pnl),
            pnl_pct: Some(pnl_pct),
            opened_at: String::new(),
            closed_at: Some(chrono_now()),
        };
        self.next_trade_id += 1;
        self.trades.push(trade.clone());
        Ok(trade)
    }

    pub fn close_all(&mut self, prices: &HashMap<String, f64>) -> Vec<Trade> {
        let symbols: Vec<String> = self.holdings.keys().cloned().collect();
        let mut closed = Vec::new();
        for sym in symbols {
            if let Some(price) = prices.get(&sym) {
                if let Ok(trade) = self.sell(&sym, *price, self.holdings[&sym].quantity) {
                    closed.push(trade);
                }
            }
        }
        closed
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{}", dur.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn price_map() -> HashMap<String, f64> {
        let mut m = HashMap::new();
        m.insert("BTC".into(), 50000.0);
        m.insert("ETH".into(), 3000.0);
        m
    }

    #[test]
    fn new_portfolio_has_initial_balance() {
        let p = PaperPortfolio::new(10000.0);
        assert!((p.balance - 10000.0).abs() < 0.001);
        assert!(p.holdings.is_empty());
        assert!(p.trades.is_empty());
    }

    #[test]
    fn buy_reduces_balance() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        assert!((p.balance - 5000.0).abs() < 0.001);
    }

    #[test]
    fn buy_insufficient_balance() {
        let mut p = PaperPortfolio::new(100.0);
        let result = p.buy("BTC", 50000.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn sell_reduces_holding() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        p.sell("BTC", 51000.0, 0.05).unwrap();
        assert!((p.holdings["BTC"].quantity - 0.05).abs() < 0.001);
    }

    #[test]
    fn sell_full_closes_holding() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        p.sell("BTC", 51000.0, 0.1).unwrap();
        assert!(!p.holdings.contains_key("BTC"));
    }

    #[test]
    fn sell_profit_records_pnl() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        let trade = p.sell("BTC", 55000.0, 0.1).unwrap();
        let expected_pnl = (55000.0 - 50000.0) * 0.1;
        assert!((trade.pnl.unwrap() - expected_pnl).abs() < 0.001);
    }

    #[test]
    fn sell_loss_records_negative_pnl() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        let trade = p.sell("BTC", 45000.0, 0.1).unwrap();
        assert!(trade.pnl.unwrap() < 0.0);
    }

    #[test]
    fn total_equity_includes_holdings() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        let prices = price_map();
        let equity = p.total_equity(&prices);
        assert!((equity - (5000.0 + 0.1 * 50000.0)).abs() < 0.001);
    }

    #[test]
    fn unrealized_pnl_calculated() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        let prices = price_map();
        let upnl = p.unrealized_pnl(&prices);
        assert!(upnl.abs() < 0.001);
    }

    #[test]
    fn realized_pnl_accumulates() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        p.sell("BTC", 51000.0, 0.1).unwrap();
        let expected = (51000.0 - 50000.0) * 0.1;
        assert!((p.realized_pnl() - expected).abs() < 0.001);
    }

    #[test]
    fn close_all_sells_everything() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        p.buy("ETH", 3000.0, 1.0).unwrap();
        let prices = price_map();
        let closed = p.close_all(&prices);
        assert_eq!(closed.len(), 2);
        assert!(p.holdings.is_empty());
    }

    #[test]
    fn multiple_buys_average_price() {
        let mut p = PaperPortfolio::new(10000.0);
        p.buy("BTC", 50000.0, 0.1).unwrap();
        p.buy("BTC", 40000.0, 0.1).unwrap();
        let avg = (50000.0 * 0.1 + 40000.0 * 0.1) / 0.2;
        assert!((p.holdings["BTC"].avg_entry - avg).abs() < 0.001);
        assert!((p.holdings["BTC"].quantity - 0.2).abs() < 0.001);
    }
}
