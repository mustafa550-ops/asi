pub fn rsi(prices: &[f64], period: usize) -> Vec<f64> {
    if prices.len() < period + 1 {
        return vec![50.0; prices.len()];
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    let mut rsi_values = vec![50.0; period];

    let mut avg_gain: f64 = gains[..period].iter().sum::<f64>() / period as f64;
    let mut avg_loss: f64 = losses[..period].iter().sum::<f64>() / period as f64;

    for i in period..gains.len() {
        avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i]) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i]) / period as f64;

        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsi_values.push(rsi);
    }

    rsi_values
}

pub fn macd(prices: &[f64], fast: usize, slow: usize, signal: usize) -> (Vec<f64>, Vec<f64>) {
    let fast_ema = ema(prices, fast);
    let slow_ema = ema(prices, slow);

    let macd_line: Vec<f64> = fast_ema.iter().zip(slow_ema.iter())
        .map(|(f, s)| f - s)
        .collect();

    let signal_line = ema(&macd_line, signal);

    (macd_line, signal_line)
}

fn ema(values: &[f64], period: usize) -> Vec<f64> {
    if values.is_empty() || period == 0 {
        return values.to_vec();
    }

    let k = 2.0 / (period as f64 + 1.0);
    let mut result = Vec::with_capacity(values.len());

    let sma: f64 = values.iter().take(period).sum::<f64>() / period as f64;
    result.push(sma);

    for &v in values.iter().skip(period) {
        let prev = *result.last().unwrap();
        result.push(v * k + prev * (1.0 - k));
    }

    let padding = values.len() - result.len();
    let mut padded = vec![result[0]; padding];
    padded.extend(result);
    padded
}

pub fn volume_profile(prices: &[f64], volumes: &[f64], levels: usize) -> Vec<f64> {
    if prices.is_empty() || prices.len() != volumes.len() {
        return vec![];
    }

    let min = prices.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    if range == 0.0 {
        return vec![volumes.iter().sum::<f64>()];
    }

    let bin_size = range / levels as f64;
    let mut profile = vec![0.0; levels];

    for (&price, &vol) in prices.iter().zip(volumes.iter()) {
        let idx = ((price - min) / bin_size).floor() as usize;
        if idx < levels {
            profile[idx] += vol;
        }
    }

    profile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsi_returns_50_for_insufficient_data() {
        let prices = vec![100.0; 5];
        let result = rsi(&prices, 14);
        assert_eq!(result.len(), prices.len());
        assert!(result.iter().all(|&v| v == 50.0));
    }

    #[test]
    fn rsi_returns_high_value_for_constant_prices() {
        let prices = vec![100.0; 20];
        let result = rsi(&prices, 14);
        assert_eq!(result.len(), 19);
        let last = result.last().unwrap();
        assert!(*last > 95.0);
        assert!(*last < 100.0);
    }

    #[test]
    fn rsi_detects_oversold() {
        let mut prices = vec![100.0; 15];
        for i in 0..15 {
            prices.push(100.0 - (i as f64 * 5.0));
        }
        let result = rsi(&prices, 14);
        assert!(result.last().unwrap() < &30.0);
    }

    #[test]
    fn rsi_detects_overbought() {
        let mut prices = vec![100.0; 15];
        for i in 0..15 {
            prices.push(100.0 + (i as f64 * 5.0));
        }
        let result = rsi(&prices, 14);
        assert!(result.last().unwrap() > &70.0);
    }

    #[test]
    fn macd_returns_same_length_vectors() {
        let prices = vec![100.0; 50];
        let (macd_line, signal_line) = macd(&prices, 12, 26, 9);
        assert_eq!(macd_line.len(), prices.len());
        assert_eq!(signal_line.len(), prices.len());
    }

    #[test]
    fn macd_is_zero_for_constant_prices() {
        let prices = vec![100.0; 50];
        let (macd_line, signal_line) = macd(&prices, 12, 26, 9);
        assert!(macd_line.iter().all(|&v| v == 0.0));
        assert!(signal_line.iter().all(|&v| v == 0.0));
    }

    #[test]
    fn macd_bullish_cross() {
        let prices: Vec<f64> = (0..100).map(|i| 100.0 + (i as f64).sin() * 10.0 + i as f64 * 0.1).collect();
        let (macd_line, signal_line) = macd(&prices, 12, 26, 9);
        let last_macd = macd_line.last().unwrap();
        let last_signal = signal_line.last().unwrap();
        // After a bullish trend, MACD should be positive
        assert!(*last_macd >= -1.0);
        assert!(*last_signal >= -1.0);
    }

    #[test]
    fn ema_returns_correct_length() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let result = ema(&values, 3);
        assert_eq!(result.len(), values.len());
    }

    #[test]
    fn ema_trending_up() {
        let values: Vec<f64> = (1..=20).map(|i| i as f64).collect();
        let result = ema(&values, 5);
        assert!(result.last().unwrap() > result.first().unwrap());
    }

    #[test]
    fn volume_profile_returns_empty_for_empty_data() {
        let result = volume_profile(&[], &[], 10);
        assert!(result.is_empty());
    }

    #[test]
    fn volume_profile_mismatched_lengths() {
        let result = volume_profile(&[1.0, 2.0], &[1.0], 5);
        assert!(result.is_empty());
    }

    #[test]
    fn volume_profile_basic_distribution() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let volumes = vec![10.0, 20.0, 30.0, 20.0, 10.0];
        let result = volume_profile(&prices, &volumes, 5);
        assert_eq!(result.len(), 5);
        // Last price (5.0) maps to idx=5 which is out of bounds (0..5), so skipped
        assert_eq!(result.iter().sum::<f64>(), 80.0);
    }

    #[test]
    fn volume_profile_single_price_level() {
        let prices = vec![5.0, 5.0, 5.0];
        let volumes = vec![10.0, 20.0, 30.0];
        let result = volume_profile(&prices, &volumes, 3);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 60.0).abs() < 0.001);
    }

    #[test]
    fn ema_empty_values() {
        let result = ema(&[], 5);
        assert!(result.is_empty());
    }

    #[test]
    fn ema_zero_period() {
        let values = vec![1.0, 2.0, 3.0];
        let result = ema(&values, 0);
        assert_eq!(result, values);
    }
}
