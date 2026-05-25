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
