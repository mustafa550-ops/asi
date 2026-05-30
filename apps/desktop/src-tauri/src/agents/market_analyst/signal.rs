pub enum Signal {
    Buy { reason: String, confidence: f64 },
    Sell { reason: String, confidence: f64 },
    Hold { reason: String },
}

pub fn check_bottom_fishing(
    prices: &[f64],
    rsi_values: &[f64],
    macd_line: &[f64],
    signal_line: &[f64],
) -> String {
    if prices.len() < 20 {
        return "yetersiz veri".into();
    }

    let current_price = prices.last().unwrap_or(&0.0);
    let lowest_50 = prices.iter().cloned().fold(f64::INFINITY, f64::min);
    let highest_50 = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let drop_pct = (highest_50 - lowest_50) / highest_50;
    let price_from_low = (current_price - lowest_50) / lowest_50;

    let rsi_current = rsi_values.last().unwrap_or(&50.0);
    let rsi_oversold = *rsi_current < 30.0;

    let macd_current = macd_line.last().unwrap_or(&0.0);
    let signal_current = signal_line.last().unwrap_or(&0.0);
    let macd_cross_up = macd_current > signal_current
        && macd_line.len() >= 2
        && macd_line[macd_line.len() - 2] <= signal_line[signal_line.len().max(2) - 2];

    let mut signals = Vec::new();

    if drop_pct > 0.2 && price_from_low < 0.05 {
        signals.push(format!("fiyat diplerde (dip: {:.4}, suan: {:.4})", lowest_50, current_price));
    }

    if rsi_oversold {
        signals.push(format!("RSI asiri-satim bolgesinde ({:.1})", rsi_current));
    }

    if macd_cross_up {
        signals.push("MACD alim sinyali verdi (sinyal cizgisini yukari kesti)".into());
    }

    if signals.is_empty() {
        "bekleme — net sinyal yok".into()
    } else {
        format!("BOTTOM FISHING SINYALI! Sebep: {}", signals.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bottom_fishing_insufficient_data() {
        let prices = vec![1.0; 10];
        let rsi = vec![50.0; 10];
        let macd = vec![0.0; 10];
        let signal = vec![0.0; 10];
        let result = check_bottom_fishing(&prices, &rsi, &macd, &signal);
        assert_eq!(result, "yetersiz veri");
    }

    #[test]
    fn check_bottom_fishing_no_signal() {
        let prices = vec![100.0; 50];
        let rsi = vec![50.0; 50];
        let macd = vec![0.0; 50];
        let signal = vec![0.0; 50];
        let result = check_bottom_fishing(&prices, &rsi, &macd, &signal);
        assert_eq!(result, "bekleme — net sinyal yok");
    }

    #[test]
    fn check_bottom_fishing_rsi_oversold() {
        let prices = vec![100.0; 50];
        let mut rsi = vec![50.0; 49];
        rsi.push(25.0);
        let macd = vec![0.0; 50];
        let signal = vec![0.0; 50];
        let result = check_bottom_fishing(&prices, &rsi, &macd, &signal);
        assert!(result.contains("BOTTOM FISHING"));
        assert!(result.contains("RSI"));
    }

    #[test]
    fn check_bottom_fishing_price_drop() {
        let mut prices = vec![100.0; 25];
        prices.extend(vec![79.0; 25]);
        let rsi = vec![35.0; 50];
        let macd = vec![0.0; 50];
        let signal = vec![0.0; 50];
        let result = check_bottom_fishing(&prices, &rsi, &macd, &signal);
        assert!(result.contains("BOTTOM FISHING"));
        assert!(result.contains("fiyat"));
    }

    #[test]
    fn evaluate_buy_signal() {
        let s = Signal::Buy { reason: "RSI oversold".into(), confidence: 0.85 };
        let result = evaluate_signal(&s);
        assert!(result.contains("AL"));
        assert!(result.contains("85%"));
    }

    #[test]
    fn evaluate_sell_signal() {
        let s = Signal::Sell { reason: "MACD bearish".into(), confidence: 0.75 };
        let result = evaluate_signal(&s);
        assert!(result.contains("SAT"));
        assert!(result.contains("75%"));
    }

    #[test]
    fn evaluate_hold_signal() {
        let s = Signal::Hold { reason: "No clear direction".into() };
        let result = evaluate_signal(&s);
        assert!(result.contains("BEKLE"));
    }
}

pub fn evaluate_signal(signal: &Signal) -> String {
    match signal {
        Signal::Buy { reason, confidence } => {
            format!("AL {:.0}% guven: {}", confidence * 100.0, reason)
        }
        Signal::Sell { reason, confidence } => {
            format!("SAT {:.0}% guven: {}", confidence * 100.0, reason)
        }
        Signal::Hold { reason } => {
            format!("BEKLE: {}", reason)
        }
    }
}
