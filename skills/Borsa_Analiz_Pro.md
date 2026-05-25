# Skill: Borsa_Analiz_Pro

## Meta
- **name:** Borsa_Analiz_Pro
- **description:** Binance API ile alım satım sinyalleri üretir.
- **tool:** Local_Python / Anthropic_API
- **bridge:** Tauri_FS_Command (Rust)
- **triggers:** ["SXT'yi kontrol et", "bottom fishing", "piyasa analizi"]
- **approval:** required

## Steps
1. Veri çek (Binance WebSocket)
2. Trend analizi (Ollama/Claude)
3. Risk yönetimi (Strategic Memory sorgula)
4. Kullanıcı onayı

## Logic
```python
def analyze(symbol: str) -> Signal:
    data = fetch_binance(symbol)
    trend = analyze_trend(data)
    risk = check_risk(trend)
    return Signal(symbol, trend, risk)
```

## Evolution
- v1.0: Temel RSI analizi
- v1.1: Hacim profili eklendi
