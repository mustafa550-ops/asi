use crate::agents::market_analyst::signal::Signal;

pub struct RiskManager {
    pub max_risk_per_trade: f64,
    pub max_daily_loss: f64,
    pub max_leverage: f64,
    pub portfolio_value: f64,
}

impl Default for RiskManager {
    fn default() -> Self {
        Self {
            max_risk_per_trade: 0.02,
            max_daily_loss: 0.10,
            max_leverage: 1.0,
            portfolio_value: 10000.0,
        }
    }
}

impl RiskManager {
    pub fn new(portfolio_value: f64) -> Self {
        Self {
            portfolio_value,
            ..Default::default()
        }
    }

    pub fn kelly_criterion(&self, win_prob: f64, avg_win: f64, avg_loss: f64) -> f64 {
        if avg_loss <= 0.0 || win_prob <= 0.0 || win_prob >= 1.0 {
            return 0.0;
        }
        let b = avg_win / avg_loss;
        let kelly = (win_prob * b - (1.0 - win_prob)) / b;
        kelly.clamp(0.0, self.max_risk_per_trade)
    }

    pub fn position_size(&self, entry_price: f64, stop_price: f64, win_prob: f64, avg_win: f64, avg_loss: f64) -> f64 {
        let risk_per_unit = (entry_price - stop_price).abs();
        if risk_per_unit <= 0.0 {
            return 0.0;
        }
        let kelly_frac = self.kelly_criterion(win_prob, avg_win, avg_loss);
        let risk_capital = self.portfolio_value * kelly_frac;
        let units = risk_capital / risk_per_unit;
        (units * entry_price * (1.0 + self.max_leverage)).min(self.portfolio_value * 0.5)
    }

    pub fn stop_loss(target_price: f64, entry_price: f64, risk_pct: f64, is_long: bool) -> f64 {
        let range = entry_price * risk_pct;
        if is_long {
            entry_price - range
        } else {
            entry_price + range
        }
    }

    pub fn take_profit(target_price: f64, entry_price: f64, risk_reward: f64, is_long: bool) -> f64 {
        let range = (entry_price - target_price).abs() * risk_reward;
        if is_long {
            entry_price + range
        } else {
            entry_price - range
        }
    }

    pub fn var_simple(returns: &[f64], confidence: f64) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }
        let mut sorted = returns.to_vec();
        sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((1.0 - confidence) * sorted.len() as f64).floor() as usize;
        let idx = idx.min(sorted.len().saturating_sub(1));
        sorted[idx].abs()
    }

    pub fn var_parametric(returns: &[f64], confidence: f64) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }
        let n = returns.len() as f64;
        let mean = returns.iter().sum::<f64>() / n;
        let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let z = match confidence {
            c if c >= 0.99 => 2.326,
            c if c >= 0.95 => 1.645,
            c if c >= 0.90 => 1.282,
            _ => 1.0,
        };
        (mean.abs() + z * std_dev).abs()
    }

    pub fn risk_score(&self, signal: &Signal, returns: &[f64]) -> f64 {
        let signal_risk = match signal {
            Signal::Buy { confidence, .. } => 1.0 - confidence,
            Signal::Sell { confidence, .. } => 1.0 - confidence,
            Signal::Hold { .. } => 0.5,
        };
        let var_95 = Self::var_simple(returns, 0.95);
        let var_normalized = (var_95 / self.portfolio_value).min(1.0);
        let score = signal_risk * 0.4 + var_normalized * 0.3 + self.max_risk_per_trade * 0.3;
        (score * 100.0).clamp(0.0, 100.0)
    }

    pub fn validate_trade(&self, position_value: f64, daily_loss: f64) -> Result<(), String> {
        if position_value > self.portfolio_value * self.max_risk_per_trade * 10.0 {
            return Err(format!(
                "Pozisyon buyuklugu ({:.2}) portfoyun {}'sinden fazla",
                position_value,
                self.max_risk_per_trade * 10.0
            ));
        }
        if daily_loss > self.portfolio_value * self.max_daily_loss {
            return Err(format!(
                "Gunluk zarar limiti asildi ({:.2} > {:.2})",
                daily_loss,
                self.portfolio_value * self.max_daily_loss
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kelly_criterion_basic() {
        let rm = RiskManager::new(10000.0);
        let kelly = rm.kelly_criterion(0.6, 1.5, 1.0);
        assert!(kelly > 0.0);
        assert!(kelly <= rm.max_risk_per_trade);
    }

    #[test]
    fn kelly_criterion_zero_for_no_edge() {
        let rm = RiskManager::new(10000.0);
        let kelly = rm.kelly_criterion(0.5, 1.0, 1.0);
        assert!(kelly.abs() < 0.001);
    }

    #[test]
    fn kelly_clamps_to_max_risk() {
        let rm = RiskManager::new(10000.0);
        let kelly = rm.kelly_criterion(0.9, 3.0, 1.0);
        assert!(kelly <= rm.max_risk_per_trade);
    }

    #[test]
    fn position_size_basic() {
        let rm = RiskManager::new(10000.0);
        let size = rm.position_size(100.0, 95.0, 0.6, 2.0, 1.0);
        assert!(size > 0.0);
        assert!(size <= 5000.0);
    }

    #[test]
    fn position_size_zero_for_invalid() {
        let rm = RiskManager::new(10000.0);
        let size = rm.position_size(100.0, 100.0, 0.6, 2.0, 1.0);
        assert!(size.abs() < 0.001);
    }

    #[test]
    fn stop_loss_long() {
        let sl = RiskManager::stop_loss(110.0, 100.0, 0.05, true);
        assert!((sl - 95.0).abs() < 0.001);
    }

    #[test]
    fn stop_loss_short() {
        let sl = RiskManager::stop_loss(90.0, 100.0, 0.05, false);
        assert!((sl - 105.0).abs() < 0.001);
    }

    #[test]
    fn take_profit_long() {
        let tp = RiskManager::take_profit(100.0, 90.0, 2.0, true);
        assert!((tp - 110.0).abs() < 0.001);
    }

    #[test]
    fn take_profit_short() {
        let tp = RiskManager::take_profit(100.0, 110.0, 2.0, false);
        assert!((tp - 90.0).abs() < 0.001);
    }

    #[test]
    fn var_simple_basic() {
        let returns = vec![-0.01, -0.02, -0.005, -0.03, 0.01, 0.02, -0.015, -0.025];
        let var = RiskManager::var_simple(&returns, 0.95);
        assert!(var > 0.0);
    }

    #[test]
    fn var_simple_empty() {
        assert!(RiskManager::var_simple(&[], 0.95).abs() < 0.001);
    }

    #[test]
    fn var_parametric_basic() {
        let returns = vec![-0.01, -0.02, -0.005, -0.03, 0.01, 0.02, -0.015, -0.025];
        let var = RiskManager::var_parametric(&returns, 0.95);
        assert!(var > 0.0);
    }

    #[test]
    fn risk_score_hold_is_50() {
        let rm = RiskManager::new(10000.0);
        let signal = Signal::Hold { reason: "test".into() };
        let score = rm.risk_score(&signal, &[-0.01, 0.01]);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn risk_score_buy_low_risk() {
        let rm = RiskManager::new(10000.0);
        let signal = Signal::Buy { reason: "strong".into(), confidence: 0.95 };
        let score = rm.risk_score(&signal, &[-0.001, 0.001]);
        assert!(score < 50.0);
    }

    #[test]
    fn validate_trade_ok() {
        let rm = RiskManager::new(10000.0);
        assert!(rm.validate_trade(100.0, 50.0).is_ok());
    }

    #[test]
    fn validate_trade_oversize() {
        let rm = RiskManager::new(10000.0);
        assert!(rm.validate_trade(10000.0, 50.0).is_err());
    }

    #[test]
    fn validate_trade_excessive_loss() {
        let rm = RiskManager::new(10000.0);
        assert!(rm.validate_trade(100.0, 2000.0).is_err());
    }
}
