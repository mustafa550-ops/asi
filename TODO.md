# ADLER ASI — Yapılacaklar Listesi

> **Son güncelleme:** 2026-05-25  
> **Mevcut versiyon:** 0.1.0  
> **Durum:** Alpha — 69 Rust dosyası, 0 warning (cargo check)

---

## 1. Bloker (Acil)

### `-lvosk` link hatası

**Sorun:** `cargo build` ve `cargo test` linker aşamasında `-lvosk` kütüphanesini bulamıyor. `cargo check` sorunsuz çalışır.

**Etki:** Binary üretilemiyor, testler çalıştırılamıyor.

**Çözüm önerileri:**
- [ ] Vosk kütüphanesini sisteme kur (`apt install vosk` veya source'dan derle)
- [ ] Veya vosk bağımlılığını `cfg(feature = "voice")` ile opsiyonel yap
- [ ] Veya CI/CD ortamında vosk'u önceden yükle

---

## 2. Tamamlanan Fazlar

### Phase 1 — Temel Yapı
- [x] Rust çekirdek + Tauri köprüsü
- [x] React 19 arayüz (Chat, Dashboard, Approval)
- [x] SQLite veritabanı + şema
- [x] Ollama entegrasyonu
- [x] EventBus (Tauri Events)

### Phase 2 — Ajanlar ve Yetenekler
- [x] 8 ajan implementasyonu (IntentJudge, Diagnostic, Hardware, MarketAnalyst, SystemManager, DocumentAnalyst, VoiceHandler, Supervisor)
- [x] Rust→Rust adaptasyonu (assimilation pipeline)
- [x] Hardware Controller (GPIO/relay/sensor)
- [x] Document Analyst (reader/RAG)
- [x] CLI (assimilate, skill-add, diagnostic, status)
- [x] MCP client (generic WebSocket)
- [x] Self-healing (strategic memory lookup, optimization suggestions)
- [x] CLI genişletme (skill-list, activate/deactivate/run/remove, security-audit)

### Phase 3 — Stabilizasyon (M3.1–M3.7)
- [x] Kritik hata fix: duplicate schema, hardcoded model string, dead code, todo!()
- [x] Voice pipeline: Vosk STT init, espeak-ng TTS fallback, wake word fix
- [x] Skill executor: real subprocess execution (Python/JS/Shell/Wasm)
- [x] Semantic trigger matching (Ollama fallback)
- [x] Skill lifecycle (active/version, auto-bump)
- [x] Wasm sandbox (fuel limits, `_start` fallback, WAT compile)
- [x] Unit tests (20+ — registry, executor, sandbox)
- [x] Security module (audit, keyring persist/load, PBKDF2)

---

## 3. Partially Complete (Kısmen Tamam)

### Zustand Store'ları

**Durum:** `package.json`'da bağımlılık var, `src/stores/` dizini var ama **boş**.

- [ ] `src/stores/chatStore.ts` — sohbet geçmişi, mesaj state'i
- [ ] `src/stores/dashboardStore.ts` — sistem durumu state'i
- [ ] `src/stores/approvalStore.ts` — onay bekleme state'i
- [ ] `src/stores/skillsStore.ts` — skill listesi state'i
- [ ] Mevcut `useState` kullanımlarını Zustand'a taşı

### ClaudeClient (Bulut LLM)

**Durum:** `src/llm/claude.rs` implemente edildi ama **pipeline'a bağlı değil**. Ollama birincil LLM olarak çalışıyor.

- [ ] Orchestrator'a ikinci LLM olarak ekle
- [ ] Intent classification için Ollama, ağır analiz için Claude rotası
- [ ] Hibrit mod: kullanıcı onayıyla Claude çağrısı

### CommandRouter

**Durum:** `bridge/command_router.rs` tanımlandı ama **kullanılmıyor**. Orchestrator routing'i doğrudan yapıyor.

- [ ] Route'ları güncelle (8 ajan için)
- [ ] Orchestrator'da CommandRouter'ı opsiyonel olarak wire et
- [ ] EventBus ile entegre et

### Frontend Testleri

**Durum:** Vitest yapılandırıldı ama **henüz test dosyası yok**.

- [ ] `src/components/__tests__/ChatPanel.test.tsx`
- [ ] `src/components/__tests__/Dashboard.test.tsx`
- [ ] `src/components/__tests__/ApprovalPanel.test.tsx`
- [ ] `src/lib/__tests__/tauri.test.ts`
- [ ] `src/hooks/__tests__/useTauriEvent.test.ts`

### Rust → Wasm Derleme

**Durum:** `wasm_compile::compile_rust_source()` her zaman hata döndürür (wasm-pack gerekli).

- [ ] `wasm-pack` veya `cargo-wasi` ile entegrasyon
- [ ] Sandbox testleri için örnek WASM modülleri
- [ ] `compile_and_execute()` için WAT pipeline

---

## 4. Planlanan Özellikler

### Kısa Vade (Önümüzdeki 2 Hafta)

- [ ] **SQLCipher DB şifreleme**: Veritabanının tamamını şifrele. AES-GCM anahtarı keyring'den.
- [ ] **skills-manager UI**: React bileşeni (`src/components/skills-manager/`) — skill listeleme, ekleme/silme, tetikleme.
- [ ] **voice-ui**: React bileşeni (`src/components/voice-ui/`) — ses seviyesi göstergesi, kayıt düğmesi, TTS oynatma.
- [ ] **Behavior Tree Model**: 10+ başarılı skill çalışmasından sonra otomatik davranış modeli oluşturma (evolution.rs'de tanımlı, execution bekliyor).

### Orta Vade (1 Ay)

- [ ] **Vector index**: Embedding sayısı arttığında brute-force cosine similarity yerine HNSW/IVF indeksi.
- [ ] **Async pipeline**: Orchestrator'ı tam async yap (tokio::sync::RwLock ile Mutex yer değiştirme).
- [ ] **Multi-session**: Aynı anda birden fazla kullanıcı/konuşma.
- [ ] **Plugin sistemi**: Skill manifestosu yerine compiled plugin (WASM) desteği.
- [ ] **MCP tool zenginleştirme**: tools/call, resources/list, prompts/get desteği.

### Uzun Vade (3+ Ay)

- [ ] **Remote agent**: SSH üzerinden uzak sistemlerde ADLER ajanı çalıştırma.
- [ ] **Multi-modal**: Görsel girdi (OCR, screenshot analizi).
- [ ] **Distributed memory**: Birden fazla ADLER instance'ı arasında bellek paylaşımı.
- [ ] **Auto-scaling**: Ollama modelini otomatik değiştirme (küçük→büyük, task complexity'ye göre).
- [ ] **Mobile bridge**: Tauri mobile ile iOS/Android desteği.

---

## 5. Known Issues (Bilinen Sorunlar)

| # | Sorun | Etki | Öncelik |
|---|-------|------|---------|
| 1 | `-lvosk` linker hatası | Binary üretilemiyor | **KRITIK** |
| 2 | Mutex contention riski | Çoklu thread'de lock çakışması | ORTA |
| 3 | Cosine similarity O(n) | >10k embedding'de yavaşlama | DÜŞÜK |
| 4 | Blocking LLM çağrıları | GUI donması (async command ile çözülür) | ORTA |
| 5 | Sync-over-async pattern | Her LLM çağrısı yeni tokio runtime | DÜŞÜK |
| 6 | Zustand store boş | State management eksik | ORTA |
| 7 | Turkish/English mixed | Bazı loglar İngilizce kalmış | DÜŞÜK |
| 8 | Hiçbir migration testi yok | DB migration hataları fark edilmeyebilir | ORTA |

---

## 6. Kod Kalitesi Metrikleri

| Metrik | Değer | Hedef |
|--------|-------|-------|
| Rust dosya sayısı | 69 | - |
| TypeScript dosya sayısı | 8 | 15+ (UI genişlemesiyle) |
| `cargo check` | 0 warning, 0 error | 0 |
| Unit test sayısı | 20+ | 50+ |
| Test coverage | ~%15 | %60+ |
| CLI subcommand | 9 | 12+ |
| Agent sayısı | 8 | 10+ |
| Skill manifestosu | 2 | 10+ |

---

## 7. Bağımlılık Güncelleme Planı

| Bağımlılık | Mevcut | Hedef | Tarih |
|-----------|--------|-------|-------|
| wasmtime | 24.x | 27.x | 2026-Q3 |
| tauri | 2.x | 2.x (LTS) | Stabil |
| react | 19.1 | 19.x | Stabil |
| rusqlite | 0.32 | 0.34 | 2026-Q2 |
| ring | 0.17 | 0.18 | 2026-Q2 |

---

## 8. Dağıtım Planı

- [ ] **v0.2.0**: Stable build (vosk link sorunu çözüldü)
- [ ] **v0.3.0**: Skills UI + Zustand store'ları
- [ ] **v0.4.0**: SQLCipher DB + Claude hybrid mode
- [ ] **v1.0.0**: Production-ready (multi-session, plugin sistemi, test coverage %60+)
