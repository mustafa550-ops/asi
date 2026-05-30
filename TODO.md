# ADLER ASI — Yapılacaklar Listesi

> **Son güncelleme:** 2026-05-30
> **Mevcut versiyon:** 0.2.4 (Alpha - Milestone 3: CodeRunner + Strategic Query + Skill Market/Versioning)
> **Durum:** Alpha — ~%89 komple (266/300 görev tamam)

---

## 1. Bloker (Acil)

Şu an bilinen acil bir bloker bulunmamaktadır.

---

## 2. Tamamlanan Fazlar

### Phase 1 — Monorepo Yapısı (G001-G015)
- [x] Monorepo yapısı (`apps/`, `packages/`, Cargo workspace)
- [x] pnpm workspace + cargo workspace
- [x] Git hooks (Husky), CI/CD (GitHub Actions)
- [x] Kök konfigürasyon (`config/adler.yaml`), .env.example
- [x] README, CONTRIBUTING, LICENSE, SECURITY
- [x] Justfile, Dockerfile.core, VS Code settings
- [x] Changesets sürüm yönetimi

### Phase 2 — Rust Çekirdek Altyapısı (G016-G035)
- [x] Kernel giriş noktası, tokio runtime, graceful shutdown
- [x] Konfigürasyon yöneticisi, error.rs (AdlerError)
- [x] AppState (global durum), Event Bus (tokio broadcast)
- [x] JSON-RPC router, scheduler, log rotasyonu
- [x] Metrik toplayıcı, Wasm sandbox (wasmtime)
- [x] IPC, health check, feature flags, sysinfo
- [x] İş kuyruğu, rate limiter, core unit testleri

### Phase 3 — Ajan Sistemi (G036-G055)
- [x] Agent trait, yaşam döngüsü, Orchestrator
- [x] Intent Judge, Diagnostic, Hardware Controller
- [x] Market Analyst (Binance), System Manager
- [x] Document Analyst (RAG), Voice Handler, Supervisor
- [x] Ajan iletişim protokolü (AMP), kuyruk, sandbox
- [x] FSM, yetki matrisi, performans monitörü
- [x] Hot-reload, retry/backoff, test framework

### Phase 4 — Bellek & DB (G056-G070, G121-G135)
- [x] Memory Manager (short/long term), session memory
- [x] Bağlam sıkıştırma, strategic memory
- [x] Bellek kategorizasyonu, önceliklendirme, consolidation
- [x] Episodic, semantic, procedural memory
- [x] Bellek export/import, şifreleme (SQLCipher)
- [x] SQLite bağlantı yöneticisi, migrasyon (refinery)
- [x] sqlite-vss vektör arama, embedding üretim
- [x] Edge history, strategic memory, skill registry tabloları
- [x] Audit log, backup/restore, FTS5, şifreleme

### Phase 5 — Tauri Köprüsü (G071-G085)
- [x] Tauri v2 proje iskeleti, komut router
- [x] Broadcast event bus, FS API (güvenli dosya erişimi)
- [x] Notification API, window yönetimi, global kısayol
- [x] Autostart, updater (taslak), clipboard
- [x] Bridge performans monitörü, güvenlik politikası

### Phase 6 — React UI — Temel Bileşenler (G086-G105)
- [x] React 19 + TypeScript + Vite iskeleti
- [x] Tasarım sistemi token'ları, CSS değişkenleri (`:root`)
- [x] UI Kit: Button, Input, Card, Badge, Tooltip, Modal, Toast
- [x] Layout: Sidebar, Header, Main Content, Status Bar
- [x] Dashboard ana ekranı, AgentCard, EventStream
- [x] SystemMetrics live data, NotificationCenter
- [x] ApprovalPanel (onResult callback), Settings (6 sekme)
- [x] ErrorBoundary, LoadingStates, klavye navigasyonu
- [x] a11y (aria-label, role, tabindex)
- [ ] PWA / Web manifest (düşük öncelik)
- [x] 232 component testi

### Phase 7 — Chat & Sesli Arayüz (G106-G120)
- [x] Chat arayüzü, Message bubble, Markdown render
- [x] Typing indicator, dosya ekleme
- [x] ChatHistory (backend session), ContextWindow (RAG)
- [x] Slash commands, VoiceAssistant full-screen (G117)
- [x] Approval loop UI, proactive alert
- [x] CodeRunner (G116) — chat'te kod çalıştırma (CodeRunner.tsx + run_code Tauri cmd)
- [ ] Global klavye kısayolları

### Phase 8 — RAG Pipeline (G136-G150)
- [x] RAG pipeline yöneticisi, chunker, source attribution
- [x] Dinamik indeksleme, semantik arama (hibrit)
- [x] Context builder, bilgi tutarlılık kontrolü
- [x] Bellek ağacı görselleştirmesi, eval framework
- [x] RAG cache, feedback loop, pruning
- [x] Cross-reference resolver
- [x] 20 RAG integration test

---

## 3. Devam Eden Fazlar (Kısmi Eksikler)

> 🔍 **Not:** FAZ 9-12 kodda büyük oranda implemente edilmiş durumda. Aşağıda sadece kalan eksikler işaretlenmiştir. Tamamlanan maddeler üstte [x] ile gösterilir.

### Phase 9 — Skill Registry & Manifesto Sistemi (G151-G165)
- [x] Skill manifesto parser (YAML frontmatter + markdown body)
- [x] Skill şema validasyonu (trigger/approval/steps/logic_code)
- [x] Skill registry CRUD (SQLite-backed, substring + semantic trigger matching)
- [x] Skill executor (Python/JS/Shell/Wasm adım adım icra, step pipeline)
- [x] Skill evolution (10 çalıştırmada davranış modeli türetme, strategic memory sorgu)
- [x] Skill Registry UI (3 panelli: list/detail/add)
- [ ] Skill Wasm sandbox izolasyonu (wasmtime var, skill'e bağlı değil)
- [ ] Skill API Bridge (FS/DB/HTTP erişimi)
- [x] Skill versioning & rollback (G158: skill_version_history tablosu, record_version, rollback_to_version)
- [ ] Skill export/import (.adler-skill formatı)
- [x] Skill market (G161: skill_market_reviews, rate_skill, search_by_category, top_rated)
- [ ] Skill template generator, dependency resolver

### Phase 10 — Intent Judge & NLP (G166-G180)
- [x] Intent classification (10 variant: Sorgu/Eylem/Analiz/Chat/Donanim/Kripto/Sistem/Dokuman/Ses/Bilinmiyor)
- [x] NER (coin symbol, price, GPIO pin extraction)
- [x] Bağlam anlama (anaphora resolution: bunu/onu/şunu)
- [x] Duygu analizi (Urgent/Neutral/Casual)
- [x] Çok dilli destek (Türkçe/İngilizce intent detection)
- [x] Confidence threshold (%60 altı → Chat fallback)
- [x] Intent fallback & repair (anlaşılmayan komutlarda öneri)
- [x] Slot filling (eksik parametre sorma)
- [x] Custom intent registry (kullanıcı tanımlı intent'ler)
- [x] Prompt templates (intent/entity/sentiment/context için few-shot)
- [x] Intent cache (LRU)
- [ ] Intent A/B test (iki prompt versiyonunu karşılaştırma)
- [ ] Weekly intent accuracy report

### Phase 11 — Ollama & Cloud LLM (G181-G205)
- [x] Ollama HTTP client (/api/generate, /api/chat, stream)
- [x] Model yöneticisi (listeleme, indirme, silme)
- [x] Context window yöneticisi (system + history + RAG, token sayma)
- [x] Prompt template engine (Handlebars-style değişken enjeksiyonu)
- [x] Stream parser (SSE token işleme)
- [x] Model benchmark (latency, token/s, accuracy)
- [x] Model fallback zinciri (llama3→mistral→gemma)
- [x] Quantization seçimi (q4/q8/fp16 donanıma göre)
- [x] Ollama health check & otomatik başlatma
- [x] Prompt injection koruması (sistem prompt izolasyonu)
- [x] LLM response cache
- [x] Function calling (JSON mode tool call)
- [x] Claude API client (/v1/messages, stream)
- [x] API anahtar yönetimi (keyring)
- [x] Cloud fallback (yerel yetersiz kalınca otomatik buluta geçiş)
- [x] Hibrit mod (basit intent yerelde, karmaşık analiz bulutta)
- [x] Maliyet optimizasyonu (input token, cache, model seçimi)
- [x] Prompt cache (Anthropic feature)
- [x] Cloud rate limit (429 yönetimi, retry, backoff)
- [x] Cloud güvenlik audit (PII tarama, log)
- [ ] LLM fine-tuning pipeline (LoRA)

### Phase 12 — Sesli Asistan (G206-G220)
- [x] Wake word algılama ("adler", "hey adler")
- [x] Ses kaydı (cpal ile PCM capture, noise gate)
- [x] STT Vosk (offline speech-to-text, Türkçe/İngilizce)
- [x] TTS 4-kademeli fallback (espeak-ng → supertonic → ElevenLabs → sine-wave)
- [x] Voice Handler ajanı (Agent trait, start/stop listener)
- [x] Voice Assistant UI (full-screen overlay, waveform, FAB)
- [x] Whisper STT entegrasyonu (whisper.cpp subprocess, SttEngine trait)
- [x] STT fallback (Vosk → Whisper, SttFallback chain)
- [x] ElevenLabs TTS (reqwest API, xi-api-key env)
- [x] Ses kuyruğu yöneticisi (Priority High/Normal/Low, interrupt)
- [x] Gerçek zamanlı ses çıkışı (rodio/cpal playback modülü)
- [x] Diyalog yöneticisi (turn-taking FSM, barge-in)
- [x] Ses profilleri (Hızlı/Sakin/Teknik/Varsayılan, TtsParams)
- [ ] Gürültü engelleme (RNNoise/speexdsp)
- [x] Çoklu dil sesli destek (TR/EN detect, aynı dilde TTS)

### Phase 13 — Wasm Sandbox & Güvenlik (G236-G250)
- [x] Wasmtime runtime kurulumu
- [x] Wasm bellek limiti (128MB/256MB)
- [x] Wasm CPU limiti (fuel-based, 10M instruction)
- [x] Wasm FS izolasyonu (WASI, izinli dizinler)
- [x] Wasm ağ izolasyonu (izinli domain)
- [x] Wasm host fonksiyonları (log, emit_event, read_config)
- [x] Wasm modül imzalama (Ed25519)
- [x] Wasm modül karantina (24 saat, sınırlı yetki)
- [x] Güvenlik audit log (append-only)
- [x] PII tarama (giden isteklerde TC kimlik/telefon/adres)
- [x] Şifreleme yardımcıları (AES-256-GCM, argon2)
- [x] Sistem anahtar zinciri (keyring crate)
- [x] CSP & content security (Tauri)
- [ ] Security test suite (sandbox kaçış, yetki aşımı)
- [ ] Security dokümantasyonu (threat model, STRIDE)

### Phase 14 — Asimilasyon Motoru (G221-G235)
- [x] Repo klonlayıcı (git2, shallow clone)
- [x] Repo analizör (dil tespiti, bağımlılık okuma)
- [x] Bağımlılık haritası (Cargo/NPM/Python/Go/Ruby/C#)
- [x] Kod parçalayıcı (core/interface/config ayrımı)
- [x] Adaptasyon motoru (Python/JS/Go→Rust transpiler)
- [x] Static analysis (cargo check, clippy, fmt)
- [x] Wasm sandbox testi
- [x] Entegrasyon planlayıcı (risk assessment)
- [x] Bridge kod üretici (Tauri command wrapper)
- [x] Asimilasyon onay akışı (pipeline sonunda özet)
- [x] Geri alma (rollback: temp/registry/dizin temizleme)
- [x] Modül registry kaydı (SQLite)
- [x] Asimilasyon CLI (--assimilate)
- [ ] Asimilasyon log & markdown rapor

### Phase 15 — Donanım Kontrolörü (G251-G260)
- [x] GPIO abstraction (sysfs, pin export/unexport)
- [x] Röle sürücüsü (gpio.write wrapper)
- [x] Sensör okuyucu (DS18B20, CPU temp)
- [ ] Donanım keşif (I2C/GPIO auto-detect)
- [ ] Donanım simülatörü (test için mock)
- [ ] Donanım event handler (voltaj→event)
- [ ] Donanım güvenliği (failsafe, watchdog)
- [ ] Donanım konfigürasyonu (YAML pin mapping)
- [ ] Donanım test suite
- [ ] Donanım UI paneli (React)

### Phase 16 — Market Analisti (G261-G270)
- [x] Binance REST client (kline, ticker, orderbook)
- [x] Binance WebSocket (canlı fiyat akışı)
- [x] Veri normalizasyonu (standart OHLCV Candle)
- [x] Teknik indikatörler (RSI, MACD, EMA, hacim profili)
- [x] Sinyal üretici (al/sat/bekle, bottom fishing)
- [ ] Risk yöneticisi (pozisyon büyüklüğü, stop-loss)
- [x] Strategic memory sorgu (G267: strategic_query.rs, SignalStats, best_strategies)
- [ ] Piyasa rapor üretici (markdown/grafik)
- [ ] Paper trading (sanal bakiye simülasyonu)

### Phase 17 — MCP & CLI (G271-G280)
- [x] MCP server (WebSocket, JSON-RPC 2.0)
- [x] MCP client (harici server'a bağlanma)
- [x] Tool registry (MCPToolRegistry, 5 built-in tool)
- [x] CLI (clap, 9 subcommand: assimilate, skill-add/list/activate/deactivate/run/remove, diagnostic, status, security-audit)
- [ ] CLI chat modu (REPL)
- [ ] CLI log görüntüleyici (follow/filter)
- [ ] CLI rapor üretici (PDF)
- [ ] Self-train (sandbox'ta yeni yöntem dene, öner)

### Phase 18 — Self-Healing & Git (G281-G290)
- [x] Hata tespit motoru (log pattern matching)
- [x] Log analizör (hata zinciri çıkarma)
- [x] Dry-run derleyici (sandbox'ta derleme)
- [x] Otomatik patch üretici (syntax fix)
- [x] Self-healing onay akışı
- [x] Git entegrasyonu (libgit2, auto commit `[ADLER-SELFHEAL]`)
- [x] Otomatik commit mesajı
- [ ] Feature branch yöneticisi (adler/auto-heal)
- [ ] Git diff etki analizi
- [ ] Self-healing test suite (5 hata senaryosu)

### Phase 19 — Test & Kalite (G291-G300)
- [x] Rust unit test coverage (%69 stmts)
- [x] Rust integration testleri (19 dosya, ~62 test)
- [x] React component testleri (227 test, 35 dosya)
- [x] React E2E testler (5 Playwright)
- [ ] Load test (100 eşzamanlı ajan, 1000 RPS)
- [ ] Fuzz test (CLI + API input)
- [ ] Haftalık güvenlik audit (cargo audit)
- [ ] CPU/memory profilleme (flamegraph)
- [ ] Dokümantasyon testleri (cargo test --doc)
- [ ] Release checklist

### Phase 20 — Dağıtım & v1.0
- [ ] `Result<T, String>` → custom `AdlerError` enum (G018)
- [ ] Brute-force cosine → HNSW ANN index (>10k embedding)
- [ ] Husky hook fix (şu an non-blocking)
- [ ] Light theme CSS
- [x] CodeRunner (G116) — CodeRunner.tsx + run_code Tauri command
- [ ] Zero-Mock: SystemMetrics mock fallback kaldırma
- [ ] StatusBar string parsing → structured invoke
- [ ] Genel coverage %69 → %75+

---

## 4. Known Issues (Bilinen Sorunlar)

| # | Sorun | Etki | Öncelik |
|----|-------|------|---------|
| 1 | `voice` feature libvosk gerektiriyor | `--no-default-features` ile test | DÜŞÜK |
| 2 | Wasm sandbox henüz skill'lerde aktif değil | Skill'ler native çalışır | ORTA |
| 3 | LLM pipeline canlı Ollama gerektiriyor | Testler mock LLM ile geçer | DÜŞÜK |
| 4 | HW Controller gerçek GPIO gerektiriyor | Simulator mod ile test | DÜŞÜK |
| 5 | SystemMetrics mock fallback (Zero-Mock ihlali) | Backend yokken sahte veri gösterir | ORTA |
| 6 | StatusBar string parsing (`raw.includes("8 ajan")`) | Ajan sayısı değişince kırılır | ORTA |
| 7 | Husky pre-commit non-blocking modda | Commit öncesi check bypass | DÜŞÜK |
| 8 | `Result<T, String>` custom error enum yok | Hata zinciri zayıf | ORTA |

---

## 5. Kod Kalitesi Metrikleri

| Metrik | Değer | Hedef |
|--------|-------|-------|
| Rust test sayısı | 441 (324 unit + 117 integration) | 500+ |
| React test sayısı | 232 (36 dosya) | 300+ |
| Coverage (stmts) | %69 | %75+ |
| E2E test | 5 Playwright | 10+ |
| `cargo check` | 0 error | 0 |
| `cargo check --no-default-features` | 0 error | 0 |
| Monorepo | apps/desktop, apps/core, packages/* | Modüler |

---

## 6. Dağıtım Planı

- [x] **v0.1.0**: İlk iskelet
- [x] **v0.2.0**: Headless Core + Tauri Bridge
- [x] **v0.2.1**: RAG Pipeline + UI Wiring
- [x] **v0.2.2**: Milestone 1 rötuşlar
- [x] **v0.2.3**: Milestone 2: Voice Assistant (Whisper STT, ElevenLabs TTS, queue, dialog, profiles, i18n)
- [x] **v0.2.4**: Strategic Query fix + CodeRunner test + Skill Market/Versioning
- [ ] **v0.3.0**: HW Panel + Market Risk Manager + CLI Chat REPL
- [ ] **v0.4.0**: Security Test Suite + Load Test + E2E 10+
- [ ] **v1.0.0**: Tam Otonom Yapay Zeka Operatörü (test coverage %75+, load test, güvenlik audit)
