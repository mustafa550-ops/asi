# ADLER ASI — Yapılacaklar Listesi

> **Son güncelleme:** 2026-05-30
> **Mevcut versiyon:** 0.2.1 (Alpha - Tauri Bridge + UI Wiring)
> **Durum:** Alpha — RAG Pipeline + UI Entegrasyonu Tamamlandı

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
- [x] 227 component testi

### Phase 7 — Chat & Sesli Arayüz (G106-G120)
- [x] Chat arayüzü, Message bubble, Markdown render
- [x] Typing indicator, dosya ekleme
- [x] ChatHistory (backend session), ContextWindow (RAG)
- [x] Slash commands, VoiceAssistant full-screen (G117)
- [x] Approval loop UI, proactive alert
- [ ] CodeRunner (G116) — chat'te kod çalıştırma
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

## 3. Devam Eden ve Sıradaki Fazlar

### Phase 9 — Skill Registry & Manifesto Sistemi (G151-G165)
- [ ] Skill manifesto parser, şema validasyonu
- [ ] Skill tetikleyici motoru, runtime (Node.js)
- [ ] Skill sandbox (Wasm), API bridge
- [ ] Skill versiyonlama, gelişim takibi
- [ ] Skill market, template generator, dependency resolver
- [ ] Skill Registry UI

### Phase 10 — Intent Judge & NLP (G166-G180)
- [ ] Intent classification (LLM tabanlı)
- [ ] NER, bağlam anlama, duygu analizi
- [ ] Çok dilli destek, confidence threshold
- [ ] Slot filling, custom intent, A/B test

### Phase 11 — Ollama & Cloud LLM (G181-G205)
- [ ] Ollama client, model yöneticisi, context window
- [ ] Prompt template engine, stream parser
- [ ] Model benchmark, fallback, quantizasyon
- [ ] Prompt injection koruması, function calling
- [ ] Claude API client, hibrit mod, maliyet optimizasyonu

### Phase 12 — Sesli Asistan (G206-G220)
- [ ] Wake word algılama, ses kaydı
- [ ] STT (Vosk + Whisper), TTS (Piper)
- [ ] Ses kuyruğu, sesli diyalog yöneticisi
- [ ] Ses profilleri, gürültü engelleme
- [ ] Çoklu dil sesli destek

---

## 4. Known Issues (Bilinen Sorunlar)

| # | Sorun | Etki | Öncelik |
|---|-------|------|---------|
| 1 | `voice` feature libvosk gerektiriyor | `--no-default-features` ile test | DÜŞÜK |
| 2 | Wasm sandbox henüz skill'lerde aktif değil | Skill'ler native çalışır | ORTA |
| 3 | LLM pipeline canlı Ollama gerektiriyor | Testler mock LLM ile geçer | DÜŞÜK |
| 4 | HW Controller gerçek GPIO gerektiriyor | Simulator mod ile test | DÜŞÜK |

---

## 5. Kod Kalitesi Metrikleri

| Metrik | Değer | Hedef |
|--------|-------|-------|
| Rust test sayısı | 402 (285 unit + 117 integration) | 500+ |
| React test sayısı | 227 (35 dosya) | 300+ |
| Coverage (stmts) | %69 | %65 |
| E2E test | 5 Playwright | 10+ |
| `cargo check` | 0 error | 0 |
| Monorepo | apps/desktop, packages/* | Modüler |

---

## 6. Dağıtım Planı

- [x] **v0.1.0**: İlk iskelet
- [x] **v0.2.0**: Headless Core + Tauri Bridge
- [x] **v0.2.1**: RAG Pipeline + UI Wiring (şu an)
- [ ] **v0.3.0**: Skill Registry + NLP
- [ ] **v0.4.0**: Voice Assistant + LLM
- [ ] **v1.0.0**: Tam Otonom Yapay Zeka Operatörü
