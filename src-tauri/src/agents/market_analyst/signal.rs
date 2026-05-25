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
