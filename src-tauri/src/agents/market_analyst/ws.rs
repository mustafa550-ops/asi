use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;

pub fn fetch_klines(symbol: &str, limit: usize) -> Result<Vec<f64>, String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("Tokio runtime olusturulamadi: {}", e))?;

    rt.block_on(async {
        let url_str = format!(
            "wss://stream.binance.com:9443/ws/{}@kline_1m",
            symbol.to_lowercase()
        );

        let (ws_stream, _) = connect_async(&url_str).await
            .map_err(|e| format!("Binance WS baglantisi basarisiz: {}", e))?;

        let (_write, read) = ws_stream.split();
        let mut prices = Vec::new();

        tokio::pin!(read);
        for _ in 0..limit.min(100) {
            if let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Some(price) = parse_kline_close(&text) {
                        prices.push(price);
                    }
                }
            }
            if prices.len() >= limit {
                break;
            }
        }

        if prices.is_empty() {
            Err("Kline verisi alinamadi".to_string())
        } else {
            Ok(prices)
        }
    })
}

fn parse_kline_close(raw: &str) -> Option<f64> {
    let parsed: serde_json::Value = serde_json::from_str(raw).ok()?;
    let kline = parsed.get("k")?;
    let close_str = kline.get("c")?.as_str()?;
    close_str.parse::<f64>().ok()
}

pub fn stream_prices(symbol: &str) -> Result<tokio::sync::mpsc::Receiver<f64>, String> {
    let (tx, rx) = tokio::sync::mpsc::channel::<f64>(256);
    let symbol = symbol.to_lowercase();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let url_str = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol);
            let (ws_stream, _) = connect_async(&url_str).await.unwrap();
            let (_write, read) = ws_stream.split();

            tokio::pin!(read);
            let mut count = 0u32;
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Some(price) = parse_trade_price(&text) {
                        if tx.blocking_send(price).is_err() {
                            break;
                        }
                        count += 1;
                        if count > 1000 { break; }
                    }
                }
            }
        });
    });

    Ok(rx)
}

fn parse_trade_price(raw: &str) -> Option<f64> {
    let parsed: serde_json::Value = serde_json::from_str(raw).ok()?;
    let price_str = parsed.get("p")?.as_str()?;
    price_str.parse::<f64>().ok()
}
