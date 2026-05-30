# ADLER ASİ — 300 Görevlik Detaylı Yol Haritası & Proje Kılavuzu (CLAUDE.md)

> **Versiyon:** 2.3  
> **Tarih:** 2026-05-30  
> **Yazar:** Mustafa (Adler ASİ Mimarisi)  
> **Amaç:** Bu doküman, ADLER ASİ'nin tam yığın (full-stack) mimarisini, ajan hiyerarşisini, otonom döngülerini, bellek yönetimini, asimilasyon stratejisini ve 300 görevlik uygulanabilir yol haritasını tanımlar. Claude Code veya benzeri yapay zeka kodlama asistanları için mutlak kaynak (source of truth) niteliğindedir.

---

## İÇİNDEKİLER

1. [Vizyon & Felsefe](#1-vizyon--felsefe)
2. [Teknoloji Yığını](#2-teknoloji-yığını)
3. [Mimari Katmanlar](#3-mimari-katmanlar)
4. [Ajan Hiyerarşisi](#4-ajan-hiyerarşisi)
5. [Bellek & RAG Sistemi](#5-bellek--rag-sistemi)
6. [Skill Registry](#6-skill-registry)
7. [İletişim Protokolü](#7-iletişim-protokolü)
8. [Asimilasyon Motoru](#8-asimilasyon-motoru)
9. [MCP / CLI / Kendini Eğiten ASİ](#9-mcp--cli--kendini-eğiten-aşi)
10. [Kodlama Standartları](#10-kodlama-standartları)
11. [Güvenlik & Gizlilik](#11-güvenlik--gizlilik)
12. [Geliştirme Akışı](#12-geliştirme-akışı)
13. [300 Görevlik Yol Haritası](#13-300-görevlik-yol-haritası)
14. [Örnek Senaryolar](#14-örnek-senaryolar)
15. [Hata Kodları](#15-hata-kodları--durum-yönetimi)
16. [Sonuç](#16-sonuç)

---

## 1. Vizyon & Felsefe

ADLER ASİ; sadece bir analiz aracı değil, kullanıcının stratejik kararlarını destekleyen, veriyi anlamlı bir hikayeye dönüştüren ve operasyonel yükü hafifleten **özerk bir dijital operatör**dür.

### Temel İlkeler
- **Local-First:** Veri asla dışarı sızdırılmaz. Tüm işlem yerel donanımda (Rust çekirdek + SQLite) gerçekleşir.
- **Offline-First:** İnternet olmadan çalışır. Ollama/Llama3/Mistral gibi yerel modeller varsayılandır. Bulut API'leri (Claude, OpenAI vb.) yalnızca "uzman ajan" olarak, kullanıcı onayıyla ve hibrit modda devreye girer.
- **Proaktif Müdahale:** Sadece komut beklemez; anomali tespit eder, self-healing uygular, öngörülü (predictive) aksiyon alır.
- **Şeffaflık:** Her kararın, önerinin ve verinin kaynağı (source attribution) mutlaka belirtilir. "Halüsinasyon" riski sıfırlanır.
- **No Fluff:** Gereksiz nezaket sözcükleri yoktur. Net, teknik, profesyonel dil kullanılır.

---

## 2. Teknoloji Yığını (Tech Stack)

| Katman | Teknoloji | Görev |
|--------|-----------|-------|
| **Çekirdek (Kernel)** | Rust | Donanım kontrolü, bellek yönetimi, otonom ajanlar, Wasm sandbox |
| **Masaüstü Köprüsü** | Tauri v2 | Rust çekirdeği ile web tabanlı UI arasında güvenli, hafif köprü |
| **Arayüz (Shell)** | React 19 + TypeScript | Görselleştirme, Chat, Onay Panelleri, Sesli Asistan UI |
| **İletişim** | JSON-RPC / Tauri Events | Frontend-Backend arası broadcast event bus & komut köprüsü |
| **Bellek & RAG** | SQLite + sqlite-vss (vektör) | Memory Tree, Skill Registry, deneyim hafızası, semantik arama |
| **Yerel LLM** | Ollama | Offline karar verme, niyet analizi, kod üretimi |
| **Bulut LLM (Fallback)** | Anthropic Claude API | Ağır mantıksal analiz, mimari planlama, kod refactor (onaylı) |
| **Sesli Asistan** | Vosk / Whisper (STT) + Piper/ElevenLabs (TTS) | Wake word, offline ses tanıma, metin-ses dönüşümü |
| **Çalışma Zamanı** | Node.js (yönetilen) | TypeScript/JS tabanlı skill'lerin izole çalışma ortamı |
| **Paket Yöneticisi** | pnpm | JS/TS bağımlılık yönetimi |
| **Kalite** | Vitest / ESLint / Prettier / Rust Clippy | Test, lint, format, derleme güvenliği |
| **Versiyonlama** | Git (libgit2 entegrasyonu) | ADLER'in yaptığı her self-healing veya refactor otomatik commit'lenir |

---

## 3. Mimari Katmanlar

```
┌─────────────────────────────────────────────────────────────┐
│                    REACT + TYPESCRIPT (Shell)                │
│  Chat UI │ Voice UI │ Dashboard │ Onay Panelleri │ Skills   │
├─────────────────────────────────────────────────────────────┤
│              TAURI BRIDGE (JSON-RPC + Events)               │
│         Broadcast Event Bus │ Command Router │ FS API       │
├─────────────────────────────────────────────────────────────┤
│                     RUST CORE (Kernel)                       │
│  Agent Orchestrator │ Memory Manager │ Hardware Controller  │
│  Self-Healing Engine │ Wasm Sandbox │ Tool Registry         │
├─────────────────────────────────────────────────────────────┤
│              SQLITE + sqlite-vss (Vector DB)                │
│  Memory Tree │ Skill Registry │ RAG Index │ Edge History    │
├─────────────────────────────────────────────────────────────┤
│              OLLAMA / CLAUDE API (Inference)                │
│  Intent Judge │ Local LLM │ Cloud Fallback │ Context Manager │
└─────────────────────────────────────────────────────────────┘
```

### 3.1 Rust Çekirdek — "Güvenli İzolasyon"
- **Wasm Runtime (wasmtime):** Asimile edilen üçüncü taraf kodlar (GitHub'dan gelen modüller) Wasm sandbox içinde çalıştırılır. Bellek güvenliği garanti altına alınır; ana sistem asla çökmez.
- **Static Analysis:** `rustc` ve Clippy, asimile edilen her Rust kodunu derleme aşamasında kontrol eder. Derlenemezse hata chat'e düşer.
- **Memory Safety:** Tüm sistem düzeyi işlemler `unsafe` bloklardan kaçınılarak, Rust'ın ownership modeliyle yönetilir.

### 3.2 Tauri Köprüsü
- **Broadcast Event Bus:** Rust tarafında bir olay (örn. röle tetiklendi, borsa sinyali geldi) olduğunda, Tauri Events kullanılarak React'e canlı (stream) aktarılır. Arayüz hiçbir zaman "sorgu yapmaz", sadece "gelen veriyi dinler".
- **Command Router:** Frontend'den gelen komutlar `invoke` ile Rust'a, oradan ilgili ajanlara yönlendirilir.

---

## 4. Ajan Hiyerarşisi (Multi-Agent Hierarchy)

ADLER, tek bir merkezden yönetilmek yerine **görev bazlı ajanlar** şeklinde çalışır.

### 4.1 Ajan Türleri
| Ajan | Görev | Örnek Tetikleyici |
|------|-------|-------------------|
| **Orchestrator** | Görev dağıtımı, workflow kontrolü, onay yönetimi | Her kullanıcı komutu |
| **Intent Judge** | Niyet analizi, intent classification (Sorgu/Eylem/Analiz/Chat/Sistem) | Komut alındığında |
| **Diagnostic Agent** | Hata teşhisi, log analizi, self-healing önerisi | Hata kodu veya anomali |
| **Hardware Controller** | GPIO, röle, sensör, 12V devre kontrolü | "Röleyi aç", "Kapı zilini kontrol et" |
| **Market Analyst** | Binance API, kripto analizi, bottom fishing sinyali | "SXT'yi kontrol et" |
| **System Manager** | RAM/CPU izleme, process yönetimi, otomasyon | "Sistem durumunu raporla" |
| **Document Analyst** | RAG üzerinden .md dosyalarını analiz etme | "Notlarımı incele" |
| **Voice Handler** | STT/TTS, wake word, sesli diyalog yönetimi | "Hey Adler" |
| **Supervisor Agent** | Diğer ajanların hatalarını düzeltir, süreci optimize eder | Bir ajan tıkanırsa |

### 4.2 Ajan İletişim Protokolü (AMP)
Ajanlar arası mesajlaşma tip-güvenli bir protokol ile yapılır:

```rust
struct AgentMessage {
    from: AgentId,
    to: AgentId,
    payload: Payload,
    correlation_id: Uuid,
    timestamp: DateTime<Utc>,
    priority: Priority, // High | Normal | Low
}
```

### 4.3 Otonom Döngü (Pipeline)
Kullanıcı komutu verdiğinde arka planda şu aşamalar işlenir:

1. **Niyet Analizi (Intent Recognition):** "Bu teknik bakım mı, kripto analizi mi, otomasyon mu?"
2. **Ajan Seçimi (Delegation):** Görevi ilgili uzman ajana ilet.
3. **Planlama (Reasoning):** Gerekli adımları sırala. Eksik veri varsa self-healing ile tekrar çek.
4. **İcra (Execution):** Kod çalıştır, API çağrısı yap, donanımı tetikle.
5. **Onay Döngüsü (Confirmation Loop):** Kritik kararlarda (alım-satım, röle kapatma) kullanıcı onayı bekle.
6. **Raporlama (Feedback):** Kısa özetli, bağlamsal rapor sun. "Mustafa, [X] görevi tamamlandı. Sistem stabil, [Y] sonucunu aldım."

---

## 5. Bellek & RAG Sistemi (Memory Tree)

ADLER'in beyninde iki bellek katmanı vardır:
- **Kısa Süreli Bellek:** O anki görev bağlamı, konuşma geçmişi (son 20 mesaj).
- **Uzun Süreli Bellek (RAG):** SQLite tabanlı vektör veritabanı + bilgi grafiği.

### 5.1 Veritabanı Şeması (SQLite)
```sql
-- Vektör Embeddings (sqlite-vss)
CREATE TABLE embeddings (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    embedding BLOB NOT NULL,
    source TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    category TEXT NOT NULL,
    priority REAL DEFAULT 1.0
);

-- Bilgi Grafiği (Edge History)
CREATE TABLE edge_history (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER REFERENCES embeddings(id),
    child_id INTEGER REFERENCES embeddings(id),
    type TEXT NOT NULL,
    diff TEXT,
    confidence REAL DEFAULT 1.0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Deneyim Hafızası (Strategic Memory)
CREATE TABLE strategic_memory (
    id INTEGER PRIMARY KEY,
    context TEXT NOT NULL,
    decision TEXT NOT NULL,
    outcome TEXT NOT NULL,
    confidence REAL DEFAULT 0.5,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Oturum Belleği (Session Memory)
CREATE TABLE session_memory (
    id INTEGER PRIMARY KEY,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    token_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Audit Log (Değiştirilemez)
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    event_type TEXT NOT NULL,
    actor TEXT NOT NULL,
    details TEXT,
    hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 5.2 RAG Akışı
1. **Dinamik İndeksleme:** Yeni .md dosyası veya log eklendiğinde anında vektörleştirilir.
2. **Semantik Arama:** Anahtar kelime değil, "anlam" araması yapılır. Vektör + FTS5 hibrit arama.
3. **Kaynak Gösterme (Source Attribution):** Her cevap hangi dosyadan/logdan geldiğini belirtir.
4. **Proaktif Uyarı:** İki doküman arasındaki tutarsızlığı fark edip kullanıcıyı uyarır.
5. **Bellek Birleştirme (Consolidation):** Uyku modunda (idle) benzer bellekleri özetler.

---

## 6. Skill Registry (.md Yetenek Sistemi)

ADLER'in yetenekleri **Skill Manifestosu** olarak `.md` dosyalarında tanımlanır.

### 6.1 Skill Manifesto Yapısı
```markdown
---
id: borsa_analiz_pro
name: Borsa Analiz Pro
version: 1.2.0
author: Mustafa
description: Binance API ile alım satım sinyalleri üretir.
tool: local_python
bridge: tauri_fs_command
triggers:
  - "SXT'yi kontrol et"
  - "bottom fishing sinyali"
approval: required
categories:
  - finance
  - crypto
---

## Steps
1. Veri Çek (Binance WebSocket)
2. Trend Analizi (Ollama/Claude)
3. Risk Yönetimi (Strategic Memory sorgula)
4. Kullanıcı Onayı

## Logic
```python
def analyze(symbol: str) -> Signal:
    ...
```

## Evolution
- **v1.0:** Temel RSI analizi
- **v1.1:** Hacim profili eklendi
- **v1.2:** Multi-timeframe desteği
```

### 6.2 Skill Yaşam Döngüsü
1. **Yükleme:** Kullanıcı `.md` dosyasını chat'e yükler.
2. **Asimilasyon:** GitHub reposundan çekilen kodlar Skill'e dönüştürülür.
3. **Uygulama:** Tetikleyici kelimeyle aktif olur.
4. **Gelişim:** 10+ başarılı çalıştırmadan sonra davranış modeli türetilir.
5. **Versiyonlama:** Her güncelleme Git commit'i olarak kaydedilir.

### 6.3 OpenCode CLI Skill'leri (`/opencode/skills/`)
ADLER'den bağımsız olarak, **opencode CLI** (bu asistan) kendi skill sistemine sahiptir. `.opencode/skills/<name>/SKILL.md` dosyaları opencode'un davranışını yönlendirir:

| Skill | Kaynak | Görev |
|-------|--------|-------|
| `context-aware-coding` | Yerleşik | Katman tespiti (frontend/backend/db/config) |
| `modular-thinking` | Yerleşik | Çok dosyalı işleri adım adım bölme |
| `documentation-first` | Yerleşik | Doc comment zorunluluğu |
| `parseltongue` | G0DM0D3 asimilasyonu | Input perturbation, red-teaming |
| `autotune` | G0DM0D3 asimilasyonu | Context-adaptive LLM parametre optimizasyonu |
| `ultraplinian` | G0DM0D3 asimilasyonu | Multi-model racing & karşılaştırma |
| `stm` | G0DM0D3 asimilasyonu | Semantic Transformation (output normalization) |
| `godmode` | G0DM0D3 asimilasyonu | Liberated AI prompt stratejileri |

Bu skill'ler ADLER proje koduna (`src-tauri/`, `src/`) dokunmaz, sadece opencode CLI'ın davranışını yönlendirir.

---

## 7. İletişim Protokolü (Chat & Sesli Asistan)

### 7.1 Chat Arayüzü
- **Varsayılan (Kısa Özet):** "Mustafa, [X] görevi tamamlandı. Sistem stabil, [Y] sonucunu aldım."
- **Talep Üzerine (Uzun Özet):** "Detayları anlat" dendiğinde chain of thought paylaşılır.
- **Proaktif Notlar:** "Bu arada, [Z] kütüphanesi çok işe yaradı, bilgin olsun."

### 7.2 Sesli Asistan — "Jarvis-vari"
- **Tonlama:** Sakin, kontrollü, analitik, duygusal değişim göstermeyen.
- **Hız:** Milisaniyelik latency. Kısa duraksama veri işleme göstergesidir.
- **Hata Bildirimi:** "İşlem başarısız oldu. Hata kodu 404. Yerel önbellek verisi ile devam ediyorum..."

### 7.3 Onay Döngüsü (Confirmation Loop)
| Seviye | Kapsam | ADLER Yetkisi |
|--------|--------|---------------|
| **Gözlemci** | Yeni yetenekler/yöntemler | Sadece önerir, manuel onay bekler |
| **Yarı-Otonom** | Günlük rutin görevler | Rutin işlemleri yapar, kritik kararları sorar |
| **Tam Yetki (Stratejik)** | Onaylanmış modeller | Kendi oluşturduğu modelleri otonom uygular |

---

## 8. Asimilasyon Motoru (Repo Entegrasyonu)

ADLER, GitHub reposunu "kopyalamak" yerine **parçalara ayırır, adapte eder, entegre eder**.

### 8.1 Asimilasyon Akışı
1. **Analiz (Inception):** Repo klonlanır. README, package.json, requirements.txt incelenir.
2. **Parçalama:** Core / Interface / Configuration ayrımı.
3. **Adaptasyon:** Mevcut kod standartlarına uygun hale getirilir.
4. **Güvenlik:** Kod Wasm sandbox'ta derlenir/test edilir.
5. **Entegrasyon:** Bağımlılıklar yüklenir, ortam değişkenleri işlenir.
6. **Kayıt:** `module_registry` içinde kendi klasörüne sahip olur.

### 8.2 Asimile Edilecek Repolar (Örnekler)
| Repo | Amaç | Entegrasyon Noktası |
|------|------|---------------------|
| `claude-obsidian` | Yerel .md analizi | Document Analyst Agent |
| `OpenJarvis` | Ollama ajan yönetimi | Agent Orchestrator |
| `Priler/jarvis` | Offline ses işleme | Voice Handler (Rust) |
| `DEENUU1/jarvis-backend` | SQLite + RAG + Whisper | Rust Core / Memory Manager |
| `DEENUU1/jarvis-desktop` | Tauri/React UI parçaları | React Shell |
| `supertonic` | Gelişmiş ses sentezi | Sesli Asistan |
| `elder-plinius/G0DM0D3` | Multi-model chat, input perturbation, auto-tuning, STM | OpenCode CLI Skills (parseltongue, autotune, ultraplinian, stm, godmode) |

---

## 9. MCP / CLI / Kendini Eğiten ASİ

### 9.1 MCP Entegrasyonu
- **Server:** ADLER, MCP server olarak çalışabilir.
- **Client:** Harici MCP server'lara bağlanarak veri çekebilir.
- **Tool Registry:** Her entegre aracın metadata, yetki, versiyon yönetimi.

### 9.2 CLI Ajanı
- **Komutlar:** `adler --assimilate <repo-url>`, `adler --skill-add <file.md>`, `adler --diagnostic`
- **Self-Training:** Sandbox'ta yeni kod dener, başarılı olunca kullanıcıya sorar.

---

## 10. Kodlama Standartları & Stil Transferi

### 10.1 Kurallar
- **Dil:** Rust (çekirdek) ve TypeScript (UI/Skills) ana dillerdir.
- **Async:** `async/await` yapısı zorunludur. Callback hell yasaktır.
- **Modülerlik:** Tek devasa dosya yerine, test edilebilir, bağımsız parçalar.
- **Yorumlar:** Her fonksiyonun "neden" yazıldığına dair açıklama içermesi gerekir.
- **Hata Yönetimi:** Rust'ta `Result<T, E>`, TS'te `Result` pattern (neverthrow) tercih edilir.

### 10.2 Self-Healing & Refactoring
- **Dry-Run:** Kod yazıldığı an sandbox'ta derlenir.
- **Hata Ayıklama:** Derleme hatası varsa ADLER logları tarar, patch dener.
- **Git Entegrasyonu:** Her onarım otomatik commit'lenir: `[ADLER-SELFHEAL] Hata giderildi: <açıklama>`

---

## 11. Güvenlik & Gizlilik

- **Veri Sızdırma Yok:** Bulut API'leri yalnızca onaylı, şifreli isteklerle kullanılır.
- **Wasm Sandbox:** Üçüncü taraf kodlar izole ortamda çalışır.
- **Onay Hiyerarşisi:** Kritik işlemler kullanıcı onayı olmadan asla icra edilmez.
- **Şifreleme:** SQLite SQLCipher ile şifrelenir. API anahtarları keyring'de tutulur.
- **PII Tarama:** Giden API isteklerinde otomatik kişisel bilgi taraması.

---

## 12. Geliştirme Akışı (Git Workflow)

1. **Feature Branch:** `feature/<skill-adı>` veya `fix/<hata-kodu>`
2. **ADLER Self-Commit:** Otonom onarımlar `adler/auto-heal` branch'ine commit'lenir.
3. **PR & Review:** Kullanıcı onayıyla `main`e merge edilir.
4. **Semantik Versiyonlama:** `v6.0.0` formatında. ADLER-OS v6.x olarak takip edilir.

---

## 13. 300 Görevlik Yol Haritası

> **GÜNCEL DURUM (v2.3):** FAZ 0-4 **TAMAMLANMIŞ**, FAZ 5-8 **KISMİ TAMAMLANMIŞTIR**. 402 Rust testi (285 unit + 117 integration), 227 React component testi, 5 Playwright E2E testi. Detaylı rapor için [aşağıya](#geliştirme-raporu) bakınız.

---

### 🚀 Geliştirme Raporu

| Kategori | Durum | Detay |
|----------|-------|-------|
| **Rust Test Altyapısı** | ✅ 402 test | 285 unit + 117 integration; `--no-default-features` ile çalışır |
| **React Component Testleri** | ✅ 227 test | 35 dosya, `@testing-library/react` |
| **Playwright E2E** | ✅ 5 test | System Chrome (`/usr/bin/google-chrome`) |
| **Coverage Threshold** | ✅ %69 stmts | 65/60/65/65 hedefi tutturuldu |

| Kategori | Durum | Açıklama |
|----------|-------|----------|
| **RAG Pipeline** | ✅ 20 test | Chunker, Eval, Cache, Pruning, Xref, Pipeline — SQLite in-memory + mock LLM |
| **RagPipeline → AppState** | ✅ Entegre | 3 Tauri command: `hybrid_search`, `query_strategic`, `get_knowledge_graph` |
| **Sessions (db/sessions.rs)** | ✅ CRUD | `sessions` + `session_messages` tabloları; `send_command` her mesajı kaydeder |
| **NotificationCenter** | ✅ Header'a taşındı | Bell ikonu, badge, dropdown; `useTauriEvent` ile gerçek zamanlı |
| **ApprovalPanel** | ✅ Chat entegrasyonu | `onResult` callback, system mesajı, `alert()` kaldırıldı |
| **Dashboard Ajan Listesi** | ✅ Dinamik | `get_agent_statuses` Tauri command; artık hardcoded değil |
| **SystemMetrics** | ✅ Live data | `get_system_metrics` Tauri command; fallback mock |
| **Settings** | ✅ Persistence | `save_setting` Tauri command; form submit ile kaydetme |
| **FileAttachment** | ✅ İyileştirme | File type icons (🦀🐍🔷), toast bildirim, drag overlay |
| **ChatHistory** | ✅ Backend session | `list_chat_sessions`, `delete_chat_session` Tauri command |
| **ContextWindow** | ✅ RAG bağlı | `hybrid_search` invoke; her kullanıcı mesajında kaynak gösterimi |
| **VoiceAssistant (G117)** | ✅ Full-screen | Overlay, waveform, status, close/esc |
| **chatStore** | ✅ System mesaj | `role: "system"` + `addMessage` aksiyonu |

| Bağımlılık | Yönetim |
|------------|---------|
| `voice` feature | `--no-default-features` ile test (libvosk yok) |
| `outcome` CHECK | `IN ('success','failure','partial')` |
| Test DB | `create_test_db_arc()` helper |
| CSS | `styles.css` (`:root` CSS değişkenleri) |
| Frontend state | Zustand store (Tauri invoke wrapper) |
| E2E playwright | `playwright.config.ts`, CI job, `justfile` |

---

### FAZ 0: ALTYAPI & REPO KURULUMU (G001-G015) - TAMAMLANDI

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G001** | **Monorepo Yapısını Kur** | `adler-asi/` kök dizininde `apps/desktop`, `apps/core`, `apps/voice`, `packages/shared-types`, `packages/ui-kit` dizinlerini oluştur. | Yok | 2s | `tree -L 2` komutu 5 ana dizini gösterir. |
| **G002** | **Workspace Yöneticisi Seçimi** | `pnpm workspaces` + `cargo workspace` kurulumunu yap. Root `package.json` ve `Cargo.toml` workspace tanımlarını yaz. | G001 | 1s | `pnpm install` ve `cargo check` root'tan çalışır. |
| **G003** | **Git Hook'ları (Husky)** | `pre-commit` hook'unda `cargo clippy`, `eslint`, `prettier` ve `vitest --run` çalıştır. | G001 | 1s | Geçersiz kod commit edilemez. |
| **G004** | **CI/CD Pipeline (GitHub Actions)** | `.github/workflows/ci.yml` oluştur: Rust build, Tauri build, vitest, clippy, audit. | G002 | 2s | PR'da 4 job paralel çalışır. |
| **G005** | **Kök Konfigürasyon Şeması** | `config/adler.yaml` şemasını tanımla: `llm`, `voice`, `hardware`, `memory` bölümleri. | Yok | 2s | YAML schema validasyonu testi geçer. |
| **G006** | **Ortam Değişkenleri Şablonu** | `.env.example` dosyasını oluştur: `OLLAMA_HOST`, `CLAUDE_API_KEY`, `BINANCE_API_KEY`, `SQLITE_PATH`, `WAKE_WORD_MODEL`. | Yok | 30dk | 12 değişken tanımlıdır. |
| **G007** | **README.md & CONTRIBUTING.md** | Geliştirici kurulum rehberi, mimari diyagram (Mermaid), commit convention (Conventional Commits). | G001 | 2s | Yeni geliştirici 10 dk'da kurulum yapabilir. |
| **G008** | **Dockerfile (Opsiyonel)** | `Dockerfile.core` ile headless Rust çekirdeği containerize edilebilir. | G002 | 2s | `docker build -t adler-core .` başarılı. |
| **G009** | **Justfile / Makefile** | `just dev`, `just test`, `just build-desktop`, `just assimilate <url>` komutlarını tanımla. | G002 | 1s | `just dev` Tauri + React + Rust'ı aynı anda başlatır. |
| **G010** | **VS Code Workspace Ayarları** | `.vscode/settings.json`, `extensions.json`, `launch.json` (Rust debug + Chrome debug). | G001 | 1s | F5 ile Rust debugger çalışır. |
| **G011** | **Lisans & Güvenlik Politikası** | `LICENSE` (MIT/Apache 2.0 dual), `SECURITY.md` (responsible disclosure). | Yok | 1s | Repo kökünde yer alır. |
| **G012** | **Bağımlılık Audit Sistemi** | `cargo audit` ve `pnpm audit`'i CI'ya entegre et. | G004 | 1s | Kritik CVE'de build fail olur. |
| **G013** | **Sürüm Yönetimi (Changesets)** | `@changesets/cli` kur. Semantic versioning otomasyonu. | G002 | 1s | `pnpm changeset` ile versiyon bump'lanır. |
| **G014** | **Proje Logo & Varlıklar** | `apps/desktop/public/` içine logo, ikon, splash screen, sesli asistan avatar görselleri. | Yok | 2s | 512x512 PNG ve SVG versiyonları mevcut. |
| **G015** | **Mimari Diyagramı Otomatik Üretim** | `d2` veya `mermaid-cli` ile `docs/architecture.d2`'den PNG üreten script. | G007 | 1s | CI'da diyagram otomatik güncellenir. |

### FAZ 1: RUST ÇEKİRDEK — TEMEL ALTYAPI (G016-G035) - TAMAMLANDI

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G016** | **Kernel Giriş Noktası (`main.rs`)** | `adler-core` binary crate'i. `tokio` runtime, graceful shutdown (`ctrlc`), structured logging (`tracing`). | G002 | 2s | `cargo run` çalışır, Ctrl+C anında kapanır. |
| **G017** | **Konfigürasyon Yöneticisi** | `config.rs`: YAML/TOML okuma, ortam değişkeni override, hot-reload (watch file). | G005 | 2s | Dosya değişince `tracing::info!` basar. |
| **G018** | **Hata Türleri (`error.rs`)** | `thiserror` ile `AdlerError` enum'ı: `Io`, `Db`, `Llm`, `Hardware`, `Config`, `Sandbox`. | G016 | 2s | Her hata türü `Display` + `source()` zinciri sunar. |
| **G019** | **Global Durum Yöneticisi** | `AppState` struct'ı (Arc<RwLock<>>). Runtime konfigürasyon, aktif ajan listesi, sistem durumu. | G017 | 2s | Thread-safe, deadlock testi geçer. |
| **G020** | **Event Bus (Tokio Broadcast)** | `events.rs`: `tokio::sync::broadcast` ile tip-safe event kanalı. Event: `AgentReady`, `LlmResponse`, `HardwareTrigger`. | G016 | 2s | 10 listener aynı event'i alır, memory leak yok. |
| **G021** | **JSON-RPC Router** | `rpc.rs`: Gelen komutları deserialize et, ilgili handler'a yönlendir. `serde_json` + custom router. | G020 | 3s | `{"method":"health"}` → `{"status":"ok"}` |
| **G022** | **Zamanlayıcı & Cron Motoru** | `scheduler.rs`: `tokio-cron-scheduler` ile periyodik görevler (RAM kontrolü, heartbeat). | G016 | 2s | Her dakika `SystemHeartbeat` event'i yayınlar. |
| **G023** | **Log Rotasyon & Analiz** | `tracing-appender` ile günlük log döndürme. `logs/` dizininde 7 günlük retention. | G016 | 1s | 8. gün log dosyası otomatik silinir. |
| **G024** | **Metrik Toplayıcı** | `metrics.rs`: Basit Prometheus formatında RAM, CPU, aktif ajan sayısı metrikleri. | G019 | 2s | `GET /metrics` endpoint'i metrik döner. |
| **G025** | **Graceful Shutdown Sistemi** | `shutdown.rs`: SIGTERM/SIGINT yakalama, ajanları durdurma, SQLite checkpoint, TTS kuyruğunu boşaltma. | G016 | 2s | Shutdown 3 saniyede tamamlanır. |
| **G026** | **Plugin Loader (Dynamic Libs)** | `plugins.rs`: `.so`/`.dll` plugin'leri runtime'da yükleme (opsiyonel ileri seviye). | G019 | 3s | Örnek plugin yüklenir, `init()` çağrılır. |
| **G027** | **Process İzolasyonu (Sandbox)** | `sandbox.rs`: `wasmtime` kurulumu, Wasm modülü yükleme/derleme, bellek limiti (128MB). | G016 | 3s | Wasm modülü 128MB üzerine çıkınca `Trap` alınır. |
| **G028** | **Resource Limiter** | `resources.rs`: Ajan başına CPU zaman dilimi, memory cap, file descriptor limit. | G027 | 2s | Ajan 500ms CPU kullanımı limitini aşamaz. |
| **G029** | **IPC (Inter-Process Communication)** | `ipc.rs`: Tauri ile `tauri::Manager` dışında, standalone modda Unix socket/TCP kullanma. | G021 | 2s | Standalone modda `adler-core` socket üzerinden komut alır. |
| **G030** | **Health Check Endpoint** | `health.rs`: `/health` endpoint. SQLite, Ollama bağlantısı, son LLM yanıt süresi kontrolü. | G021 | 1s | Tüm dependency'ler `status: "up"` döner. |
| **G031** | **Feature Flags** | `features.rs`: `cloud_llm`, `hardware`, `voice`, `sandbox` feature flag'leri compile-time. | G016 | 1s | `--no-default-features --features voice` derlenir. |
| **G032** | **Sistem Bilgisi Toplayıcı** | `sysinfo.rs`: `sysinfo` crate ile RAM, CPU, disk, işletim sistemi bilgisi. | G019 | 1s | `SystemInfo` struct'ı JSON olarak serileşir. |
| **G033** | **Paralel İş Kuyruğu** | `queue.rs`: `tokio::task::JoinSet` ile sınırlı paralellik (max 8 eşzamanlı ajan). | G020 | 2s | 9. ajan kuyrukta bekler. |
| **G034** | **Rate Limiter** | `rate_limit.rs`: Ajan başına ve global API rate limit (token bucket algoritması). | G033 | 2s | 11. istek 429 döner. |
| **G035** | **Core Unit Testleri** | `tests/core_tests.rs`: State, Event Bus, Config, Error zincirleri için 30+ unit test. | G016-G034 | 3s | `cargo test` coverage ≥ %80. |

### FAZ 2: RUST ÇEKİRDEK — AJAN SİSTEMİ (G036-G055) - TAMAMLANDI

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G036** | **Ajen Trait Tanımı** | `agent.rs`: `trait Agent { fn id(), fn status(), fn execute(), fn pause(), fn resume() }`. | G019 | 2s | 3 farklı mock ajan implementasyonu derlenir. |
| **G037** | **Ajan Yaşam Döngüsü Yöneticisi** | `agent_lifecycle.rs`: `Created → Idle → Running → Paused → Completed → Error`. | G036 | 2s | Durum geçişleri loglanır, illegal geçiş engellenir. |
| **G038** | **Orchestrator Ajanı** | `agents/orchestrator.rs`: Gelen komutu al, Intent Judge'a gönder, sonucu dağıt. | G036, G021 | 3s | "SXT'yi kontrol et" → `MarketAnalyst` ajanına yönlendirir. |
| **G039** | **Intent Judge Ajanı** | `agents/intent_judge.rs`: LLM'e prompt gönder, intent classification (Sorgu/Eylem/Analiz/Chat). | G036, G072 | 3s | 4 sınıf için %90 doğruluk (test seti). |
| **G040** | **Diagnostic Agent** | `agents/diagnostic.rs`: Log tarama, hata kodu analizi, self-healing önerisi üretme. | G036, G023 | 3s | Bilinen hata koduna karşılık öneri üretir. |
| **G041** | **Hardware Controller Ajanı** | `agents/hardware.rs`: GPIO/röle komutlarını sıraya koy, durum raporu al. | G036, G126 | 3s | Mock röle durumunu `on/off` yapar. |
| **G042** | **Market Analyst Ajanı** | `agents/market.rs`: Binance API çağrısı, sinyal üretme, risk skoru hesaplama. | G036, G131 | 3s | `analyze("SXTUSDT")` → `Signal` struct'ı döner. |
| **G043** | **System Manager Ajanı** | `agents/system.rs`: RAM/CPU eşik değer kontrolü, process kill, otomasyon. | G036, G032 | 2s | RAM %90'ı geçince uyarı event'i yayınlar. |
| **G044** | **Document Analyst Ajanı** | `agents/document.rs`: RAG sorgusu, markdown analizi, özet çıkarma. | G036, G089 | 3s | `.md` dosyasından 3 cümle özet çıkarır. |
| **G045** | **Voice Handler Ajanı** | `agents/voice.rs`: Sesli komutları al, metne çevir, yanıtı seslendir. | G036, G111 | 3s | "Hey Adler" wake word'ü algılar. |
| **G046** | **Supervisor Agent** | `agents/supervisor.rs`: Diğer ajanların hatalarını izle, retry/escalate mantığı. | G036, G038 | 3s | Ajan 3 kez fail olunca Supervisor devreye girer. |
| **G047** | **Ajan İletişim Protokolü (AMP)** | `protocol.rs`: Ajanlar arası mesaj formatı: `AgentMessage { from, to, payload, correlation_id }`. | G036 | 2s | JSON serileşim/deserileşim testi geçer. |
| **G048** | **Ajan Kuyruğu & Scheduler** | `agent_queue.rs`: Ajan görevlerini önceliklendir (`High`, `Normal`, `Low`). | G038 | 2s | Kritik donanım komutu normal analizden önce işlenir. |
| **G049** | **Ajan İzolasyonu (Sandbox'ta Çalıştırma)** | `agent_sandbox.rs`: Riskli ajanları (market, hardware) Wasm sandbox'ta çalıştır. | G027, G038 | 3s | Sandbox'ta çalışan ajan ana process'i çökertemez. |
| **G050** | **Ajan Durum Makinesi (State Machine)** | `agent_fsm.rs`: Her ajan için ayrı FSM. `Idle → Planning → Executing → Validating → Reporting`. | G037 | 3s | FSM diyagramı kodda enum olarak temsil edilir. |
| **G051** | **Ajan Yetki Matrisi** | `agent_acl.rs`: Ajan başına yetki seviyesi (Gözlemci/Yarı-Otonom/Tam Yetki). | G038 | 2s | Yetkisiz ajan kritik işlem yapamaz. |
| **G052** | **Ajan Performans Monitörü** | `agent_perf.rs`: Her ajanın çalışma süresi, RAM kullanımı, başarı/fail oranı. | G048 | 2s | Dashboard'da ajan performans grafikleri görünür. |
| **G053** | **Ajan Hot-Reload** | `agent_reload.rs`: Skill güncellendiğinde ajanı durdur, yeni kodu yükle, devam et. | G050 | 3s | Ajan durumu korunarak yeniden başlar. |
| **G054** | **Ajan Test Framework'ü** | `tests/agent_tests.rs`: Mock LLM, mock donanım, mock API ile ajan testleri. | G036-G053 | 3s | `Orchestrator` end-to-end testi geçer. |
| **G055** | **Ajan Hata Kurtarma (Retry/Backoff)** | `agent_retry.rs`: Exponential backoff, circuit breaker pattern. | G046 | 2s | 3 denemeden sonra circuit açılır, 30sn bekler. |

### FAZ 3: RUST ÇEKİRDEK — BELLEK & DURUM YÖNETİMİ (G056-G070) - TAMAMLANDI

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G056** | **Bellek Yöneticisi (`memory_manager.rs`)** | Kısa/uzun süreli bellek arayüzü. `store()`, `recall()`, `forget()` metodları. | G019 | 2s | 1000 kayıt yazma/okuma < 50ms. |
| **G057** | **Konuşma Bağlamı (Session Memory)** | `session_memory.rs`: Aktif chat geçmişi, son 20 mesajı tutan ring buffer. | G056 | 2s | 21. mesaj eklendiğinde 1. mesaj otomatik silinir. |
| **G058** | **Bağlam Sıkıştırma** | `context_compression.rs`: Uzun konuşmaları LLM ile özetleyerek token sayısını düşür. | G057, G072 | 3s | 50 mesaj → 5 mesaj özet, anlam kaybı < %10. |
| **G059** | **Strategic Memory (Karar Ağacı)** | `strategic_memory.rs`: Karar → Sonuç → Güven skoru kaydı. | G056 | 2s | Başarılı kararların güven skoru artar. |
| **G060** | **Bellek Kategorizasyonu** | `memory_category.rs`: Otomatik kategori atama (skill, memory, log, doc, hardware, market). | G056 | 2s | Yeni kayıt %95 doğrulukla kategorize edilir. |
| **G061** | **Bellek Önceliklendirme** | `memory_priority.rs`: Sık erişilen bellekleri RAM'de tut, nadir olanları disk'e yaz. | G056 | 2s | LRU cache hit rate ≥ %85. |
| **G062** | **Bellek Birleştirme (Consolidation)** | `memory_consolidation.rs`: Uyku modunda (idle) benzer bellekleri birleştir, tekrarı sil. | G056 | 3s | 100 benzer kayıt → 10 özet kayıt. |
| **G063** | **Episodic Memory (Olay Zincirleri)** | `episodic_memory.rs`: "Şu görevi yaparken şu hatayı aldım, şunu denedim" zincirleri. | G059 | 2s | Olaylar kronolojik sırayla bağlanır. |
| **G064** | **Semantic Memory (Kavramlar)** | `semantic_memory.rs`: "RSI nedir?", "Röle nasıl çalışır?" gibi kavramsal bilgiler. | G056 | 2s | Kavramlar arası ilişki grafiği oluşturulur. |
| **G065** | **Procedural Memory (Nasıl Yapılır)** | `procedural_memory.rs`: "SXT analizi nasıl yapılır" adım adım prosedürler. | G056 | 2s | Prosedür adımları sıralı ve tekrar kullanılabilir. |
| **G066** | **Bellek Silme & Unutma** | `memory_forget.rs`: Eski, düşük güvenli, çakışan bellekleri otomatik silme. | G061 | 2s | 30 günden eski düşük öncelikli kayıtlar silinir. |
| **G067** | **Bellek Export/Import** | `memory_export.rs`: SQLite DB'yi `.adler` dosyası olarak export, başka cihaza import. | G056 | 2s | 1GB veri 100MB'a sıkıştırılır (zstd). |
| **G068** | **Bellek Şifreleme (SQLCipher)** | `memory_encryption.rs`: SQLite şifreleme anahtar yönetimi, keyring entegrasyonu. | G056 | 2s | DB dosyası hex dump'ta okunamaz. |
| **G069** | **Bellek Senkronizasyonu (Offline-First)** | `memory_sync.rs`: Birden fazla cihaz arasında (opsiyonel) senkronizasyon protokolü. | G067 | 3s | Çakışma durumunda son yazan kazanır (LWW). |
| **G070** | **Bellek Testleri** | `tests/memory_tests.rs`: 50+ test. Yazma, okuma, unutma, sıkıştırma testleri. | G056-G069 | 3s | Bellek operasyonları race condition içermez. |

### FAZ 4: TAURI KÖPRÜSÜ & EVENT SİSTEMİ (G071-G085) - TAMAMLANDI

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G071** | **Tauri v2 Proje İskeleti** | `apps/desktop/src-tauri/` yapısı. `Cargo.toml`, `tauri.conf.json`, `lib.rs`, `main.rs`. | G002 | 2s | `cargo tauri dev` pencere açar. |
| **G072** | **Tauri Komut Router (`commands.rs`)** | `#[tauri::command]` macro ile Rust fonksiyonlarını JS'e expose et. | G071 | 2s | `invoke('greet', {name:'Adler'})` çalışır. |
| **G073** | **Broadcast Event Bus (Tauri Events)** | `events.rs`: `emit()` ve `listen()` wrapper'ları. Tip-safe event payload'ları. | G071 | 2s | React'ten `listen('agent-update')` event alır. |
| **G074** | **FS API (Güvenli Dosya Erişimi)** | `fs_api.rs`: Tauri Scope ile sadece izinli dizinlere yazma/okuma. | G071 | 2s | Kullanıcı `~/Documents/Adler/` dışına yazamaz. |
| **G075** | **Notification API** | `notification.rs`: Tauri notification + custom in-app notification. | G071 | 1s | Kritik sinyal gelince OS notification çıkar. |
| **G076** | **Window Yönetimi** | `window.rs`: Ana pencere, mini float pencere, system tray. | G071 | 2s | Tray'e tıklayınca mini kontrol penceresi açılır. |
| **G077** | **Global Kısayol (Hotkeys)** | `hotkeys.rs`: `Ctrl+Shift+A` ile ADLER penceresini öne getir. | G071 | 1s | Kısayol her zaman çalışır (background'da). |
| **G078** | **Autostart (Sistem Başlangıcı)** | `autostart.rs`: OS başlangıcında otomatik çalışma (Windows/Linux/Mac). | G071 | 2s | Sistem açılışında tray'de belirir. |
| **G079** | **Güncelleme Mekanizması (Updater)** | `updater.rs`: Tauri updater + delta update (opsiyonel). | G071 | 3s | Yeni versiyon indirilir, restart edilir. |
| **G080** | **Clipboard Entegrasyonu** | `clipboard.rs`: Kopyalanan kod parçalarını otomatik analiz et (opsiyonel). | G071 | 1s | Kopyalanan JSON'u formatlar. |
| **G081** | **Screen Capture (Opsiyonel)** | `screenshot.rs`: Ekran görüntüsü al, OCR ile metin çıkar. | G071 | 3s | Görüntüden metin %80 doğrulukla çıkar. |
| **G082** | **Tauri-Sidecar (External Binaries)** | `sidecar.rs`: Ollama binary'ini Tauri ile paketleme. | G071 | 2s | Ollama otomatik başlatılır. |
| **G083** | **Bridge Performance Monitörü** | `bridge_perf.rs`: Invoke çağrı süreleri, event latency ölçümü. | G072 | 1s | Her invoke > 100ms ise uyarı logu. |
| **G084** | **Tauri Güvenlik Politikası** | `security.rs`: CSP, iframe kısıtlamaları, allowlist yönetimi. | G071 | 2s | `tauri.conf.json` security audit geçer. |
| **G085** | **Tauri Entegrasyon Testleri** | `tests/tauri_tests.rs`: WebDriver veya mock Tauri ile E2E testler. | G071-G084 | 3s | 10 E2E senaryo geçer. |

### FAZ 5: REACT UI — TEMEL BİLEŞENLER (G086-G105)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G086** | **React 19 + TypeScript Proje İskeleti** | `apps/desktop/src/`: Vite + React 19 + TS + SWC. Strict mode. | G002 | 2s | `pnpm dev` çalışır, HMR aktif. |
| **G087** | **Tasarım Sistemi (Tokens)** | `theme/tokens.ts`: Renk, tipografi, spacing, shadow, border radius token'ları. | G086 | 2s | Koyu/aydınlık modda token'lar değişir. |
| **G088** | **Tailwind CSS v4 Kurulumu** | `tailwind.config.ts`, global CSS, dark mode desteği. | G086 | 1s | `class="dark"` ile tüm UI koyulaşır. |
| **G089** | **Temel UI Kit Bileşenleri** | `@packages/ui-kit`: Button, Input, Card, Badge, Tooltip, Modal, Toast. | G087 | 3s | Her bileşen Storybook'ta dokümante edilir. |
| **G090** | **Layout Bileşenleri** | `components/layout/`: Sidebar, Header, Main Content, Status Bar. | G089 | 2s | Sidebar collapsible, responsive. |
| **G091** | **Dashboard Ana Ekranı** | `pages/Dashboard.tsx`: Sistem durumu, aktif ajanlar, son olaylar, hızlı aksiyonlar. | G090 | 3s | 4 widget'lı grid düzeni. |
| **G092** | **Ajan Durum Kartları** | `components/AgentCard.tsx`: Her ajan için durum, son eylem, performans metrikleri. | G091 | 2s | Renkli status indicator (yeşil/sarı/kırmızı). |
| **G093** | **Olay Akışı (Event Stream)** | `components/EventStream.tsx`: Kronolojik olay listesi, filtreleme, arama. | G091 | 2s | Canlı event'ler animasyonlu eklenir. |
| **G094** | **Sistem Metrikleri Paneli** | `components/SystemMetrics.tsx`: RAM, CPU, disk kullanımı grafikleri (Recharts). | G091 | 2s | 1 saniyelik refresh, 60 saniyelik history. |
| **G095** | **Bildirim Merkezi** | `components/NotificationCenter.tsx`: Okunmamış, okunmuş, arşivlenmiş bildirimler. | G089 | 2s | Bildirim sayısı badge olarak görünür. |
| **G096** | **Onay Paneli (Approval Panel)** | `components/ApprovalPanel.tsx`: Kritik işlemler için onay/beğenme/reddetme UI'ı. | G089 | 2s | Zaman aşımı sayacı (30sn) görünür. |
| **G097** | **Ayarlar Sayfası** | `pages/Settings.tsx`: Genel, LLM, Ses, Donanım, Bellek, Güvenlik sekmeleri. | G090 | 3s | Her ayar anında etkili olur (hot-reload). |
| **G098** | **Tema Yöneticisi** | `hooks/useTheme.ts`: Sistem temasını algılama, manuel toggle, custom tema. | G088 | 1s | `localStorage`'a tema tercihi kaydedilir. |
| **G099** | **Animasyon & Mikro Etkileşimler** | `lib/animations.ts`: Framer Motion ile page transitions, stagger effects. | G086 | 2s | 60FPS animasyonlar. |
| **G100** | **Hata Sınırı (Error Boundary)** | `components/ErrorBoundary.tsx`: React error boundary, fallback UI, hata raporlama. | G086 | 1s | JS hatası tüm UI'ı çökertmez. |
| **G101** | **Yükleme & Boş Durumları** | `components/LoadingStates.tsx`: Skeleton loader, shimmer, empty state illüstrasyonları. | G089 | 1s | Her async operasyon için loading state. |
| **G102** | **Klavye Navigasyonu** | `hooks/useKeyboard.ts`: `Esc` ile kapat, `Enter` ile onayla, `Tab` ile gezin. | G086 | 1s | Tüm interaktif elementler klavye ile erişilebilir. |
| **G103** | **Erişilebilirlik (a11y)** | `aria-label`, `role`, `tabindex`, screen reader testleri. | G086-G102 | 2s | Lighthouse a11y skoru ≥ 95. |
| **G104** | **PWA / Web Manifest (Opsiyonel)** | `manifest.json`, service worker, offline sayfa. | G086 | 1s | Offline modda temel UI çalışır. |
| **G105** | **UI Testleri** | `vitest` + `@testing-library/react` ile 50+ component testi. | G086-G104 | 3s | Coverage ≥ %80. |

### FAZ 6: REACT UI — CHAT & SESLİ ARAYÜZ (G106-G120)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G106** | **Chat Arayüzü İskeleti** | `pages/Chat.tsx`: Mesaj listesi, input alanı, gönder butonu. | G090 | 2s | Mesajlar baloncuk şeklinde görünür. |
| **G107** | **Mesaj Bileşeni (Message Bubble)** | `components/Message.tsx`: Kullanıcı/AI avatar, zaman damgası, durum. | G106 | 2s | Markdown render desteği. |
| **G108** | **Markdown Render** | `components/MarkdownRenderer.tsx`: `react-markdown` + `remark-gfm` + syntax highlighting (`shiki`). | G107 | 2s | Kod bloklarında kopyalama butonu. |
| **G109** | **Yazıyor İndikatörü (Typing)** | `components/TypingIndicator.tsx`: 3 nokta animasyonu, "Adler düşünüyor..." metni. | G106 | 1s | LLM yanıt verene kadar gösterilir. |
| **G110** | **Sesli Mesaj Gönderme** | `components/VoiceInput.tsx`: Mikrofon butonu, kayıt süresi, ses dalga formu. | G106 | 2s | 60 saniyelik kayıt limiti. |
| **G111** | **Sesli Yanıt Oynatma** | `components/VoiceOutput.tsx`: TTS sesini oynat, duraklat, ileri/geri sarma. | G106 | 2s | Ses seviyesi kontrolü. |
| **G112** | **Chat Geçmişi Yönetimi** | `hooks/useChatHistory.ts`: Konuşma oturumları listesi, arama, silme, yeniden adlandırma. | G106 | 2s | 100 oturum aranabilir. |
| **G113** | **Bağlam Penceresi (Context Window)** | `components/ContextWindow.tsx`: Mevcut bağlamda kullanılan bellekleri, kaynakları göster. | G106 | 2s | "Bu yanıt şu kaynaklara dayanıyor" listesi. |
| **G114** | **Hızlı Komutlar (Slash Commands)** | `components/SlashCommands.tsx`: `/analyze`, `/hardware`, `/market`, `/settings` önerileri. | G106 | 2s | `/` yazınca autocomplete menüsü açılır. |
| **G115** | **Dosya Ekleme (Chat'te)** | `components/FileAttachment.tsx`: `.md`, `.txt`, `.json` dosyalarını chat'e sürükle-bırak. | G106 | 2s | 10MB limit, önizleme thumbnail. |
| **G116** | **Kod Çalıştırma Paneli** | `components/CodeRunner.tsx`: Chat'teki kod bloğunu "Çalıştır" butonu. | G108 | 2s | Python kodu sandbox'ta çalışır (opsiyonel). |
| **G117** | **Sesli Asistan UI (Jarvis-vari)** | `components/VoiceAssistant.tsx`: Tam ekran sesli mod, görsel dalga formu, durum göstergeleri. | G110, G111 | 3s | "Hey Adler" wake word UI'ı aktive eder. |
| **G118** | **Onay Döngüsü UI (Approval Loop)** | `components/ApprovalDialog.tsx`: Kritik kararlarda modal onay penceresi, detaylı açıklama. | G096 | 2s | "Evet / Hayır / Detayları Göster" butonları. |
| **G119** | **Proaktif Uyarı Bileşeni** | `components/ProactiveAlert.tsx`: ADLER'in kendi başına verdiği öneriler, anomali uyarıları. | G106 | 2s | Anomali tespitinde otomatik belirir. |
| **G120** | **Chat & Ses Testleri** | E2E test: Sesli komut gönderme, yanıt alma, onay verme senaryoları. | G106-G119 | 3s | 5 E2E senaryo geçer. |

### FAZ 7: SQLITE & VEKTÖR VERİTABANI (G121-G135)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G121** | **SQLite Bağlantı Yöneticisi** | `db/connection.rs`: `rusqlite` ile bağlantı havuzu, WAL mode, busy timeout. | G016 | 2s | 100 eşzamanlı okuma, 1 yazma. |
| **G122** | **Veritabanı Şema Migrasyonları** | `db/migrations/`: `refinery` crate ile versiyonlanmış migrasyonlar. | G121 | 2s | Sıfırdan `v0 → v1 → v2` geçişi test edilir. |
| **G123** | **sqlite-vss Entegrasyonu** | `db/vector.rs`: Vektör extension'ı yükleme, embedding tablosu oluşturma. | G121 | 3s | `SELECT vss_search(...)` çalışır. |
| **G124** | **Embedding Üretim Servisi** | `db/embeddings.rs`: Yerel model (`all-MiniLM-L6-v2` via `ort`) ile embedding üretme. | G123 | 3s | 512 boyutlu vektör, < 100ms. |
| **G125** | **Vektör İndeksleme & ANN Search** | `db/ann.rs`: Approximate nearest neighbor search, HNSW indeksi. | G123 | 3s | 10k vektörde < 10ms arama. |
| **G126** | **Edge History Tablosu** | `db/edge.rs`: Bilgi grafiği kenarları, ilişki türleri (skill_evolution, bug_fix). | G122 | 2s | Parent-child ilişkisi tutarlı. |
| **G127** | **Strategic Memory Tablosu** | `db/strategic.rs`: Karar kayıtları, güven skoru, sonuç. | G122 | 2s | Karar ağacı sorgulanabilir. |
| **G128** | **Skill Registry Tablosu** | `db/skill.rs`: Skill manifestoları, versiyon, tetikleyiciler, onay seviyesi. | G122 | 2s | JSON schema validasyonu. |
| **G129** | **Log & Audit Tablosu** | `db/audit.rs`: Tüm sistem olayları, ajan eylemleri, kullanıcı onayları. | G122 | 2s | 1 yıllık log, < 500MB. |
| **G130** | **Database Backup & Restore** | `db/backup.rs`: Otomatik yedekleme, anlık görüntü (snapshot). | G121 | 2s | Yedekleme anlık, restore tutarlı. |
| **G131** | **Database Performance Monitörü** | `db/perf.rs`: Sorgu süreleri, indeks kullanımı, tablo boyutları. | G121 | 1s | Yavaş sorgu (> 100ms) loglanır. |
| **G132** | **Full-Text Search (FTS5)** | `db/fts.rs`: Metin tabanlı arama, highlighting. | G122 | 2s | "bottom fishing" araması ilgili satırları bulur. |
| **G133** | **Database Encryption Layer** | `db/crypto.rs`: SQLCipher entegrasyonu, anahtar türetme (PBKDF2). | G121 | 2s | Şifreli DB şifresiz açılamaz. |
| **G134** | **Database Test Suite** | `tests/db_tests.rs`: 40+ test. Migrasyon, vektör arama, yedekleme testleri. | G121-G133 | 3s | Coverage ≥ %85. |
| **G135** | **Database Admin CLI** | `adler --db-stats`, `adler --db-vacuum`, `adler --db-export` komutları. | G121 | 2s | CLI'dan DB bakımı yapılabilir. |

### FAZ 8: RAG SİSTEMİ & BELLEK AĞACI (G136-G150)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G136** | **RAG Pipeline Yöneticisi** | `rag/pipeline.rs`: Ingest → Chunk → Embed → Index → Retrieve → Generate. | G124 | 3s | Her adım bağımsız çalışır. |
| **G137** | **Belge Parçalayıcı (Chunker)** | `rag/chunker.rs`: Markdown başlıklarına göre akıllı parçalama, overlap kontrolü. | G136 | 2s | Bağlam bütünlüğü korunur. |
| **G138** | **Kaynak Gösterme (Source Attribution)** | `rag/attribution.rs`: Her cevabın hangi belge/bölümden geldiğini izleme. | G136 | 2s | Yanıtta `[Kaynak: X.md]` gösterilir. |
| **G139** | **Dinamik İndeksleme** | `rag/indexer.rs`: Yeni dosya eklendiğinde otomatik vektörleştirme. | G136 | 2s | `.md` dosyası eklendikten 5sn içinde aranabilir. |
| **G140** | **Semantik Arama Motoru** | `rag/search.rs`: Vektör + FTS hibrit arama, re-ranking. | G125 | 3s | "RSI nedir?" sorusu ilgili dokümanı bulur. |
| **G141** | **RAG Context Builder** | `rag/context.rs`: Bulunan belgeleri LLM context penceresine sığdırma, önceliklendirme. | G140 | 2s | Max token limiti aşılmaz. |
| **G142** | **Bilgi Tutarlılık Kontrolü** | `rag/consistency.rs`: İki doküman arasındaki çelişkiyi tespit etme. | G140 | 3s | Çelişki tespitinde kullanıcı uyarılır. |
| **G143** | **Bellek Ağacı Görselleştirmesi** | `rag/tree_viz.rs`: Bellek ilişkilerinin graph verisi üretimi (React'te D3/Cytoscape ile gösterim). | G126 | 2s | Graph JSON formatı standarttır. |
| **G144** | **RAG Değerlendirme Framework'ü** | `rag/eval.rs`: Context precision, recall, faithfulness metrikleri. | G136 | 2s | Her RAG yanıtı otomatik skorlanır. |
| **G145** | **Çoklu Mod RAG (Opsiyonel)** | `rag/multimodal.rs`: Görsel, ses dosyalarının metin açıklamalarını indeksleme. | G136 | 3s | Resim dosyası aranabilir. |
| **G146** | **RAG Cache & Hızlandırma** | `rag/cache.rs`: Sık sorulan soruların cevaplarını önbellekleme. | G136 | 2s | Aynı soru < 10ms yanıtlanır. |
| **G147** | **RAG Geri Bildirim Döngüsü** | `rag/feedback.rs`: Kullanıcı "Bu cevap yararlı/yararsız" geri bildirimi, bellek güncelleme. | G136 | 2s | Yararsız cevaplar bellekten düşürülür. |
| **G148** | **Bellek Ağacı Pruning** | `rag/pruning.rs`: Eski, düşük kaliteli dalları budama. | G143 | 2s | Bellek ağacı derinliği sınırlı tutulur. |
| **G149** | **Cross-Reference Resolver** | `rag/xref.rs`: Dokümanlar arası bağlantıları (`[[...]]`) çözümleme. | G137 | 2s | Wiki-style link'ler çalışır. |
| **G150** | **RAG Entegrasyon Testleri** | `tests/rag_tests.rs`: End-to-end RAG pipeline testleri. | G136-G149 | 3s | 10 test senaryosu geçer. |

### FAZ 9: SKILL REGISTRY & MANİFESTO SİSTEMİ (G151-G165)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G151** | **Skill Manifesto Parser** | `skill/parser.rs`: `.md` yetenek dosyalarını parse etme (YAML frontmatter + markdown body). | G122 | 2s | Geçersiz manifesto hata verir. |
| **G152** | **Skill Şema Validasyonu** | `skill/schema.rs`: JSON Schema ile manifesto yapısı kontrolü. | G151 | 2s | Eksik `triggers` alanı reject edilir. |
| **G153** | **Skill Registry Yöneticisi** | `skill/registry.rs`: CRUD operasyonları, arama, etkinleştirme/devre dışı bırakma. | G128 | 2s | 1000 skill anlık aranabilir. |
| **G154** | **Skill Tetikleyici Motoru** | `skill/trigger.rs`: Intent/keyword matching, regex, semantik trigger. | G153 | 3s | "SXT kontrol et" → `Borsa_Analiz_Pro` aktive olur. |
| **G155** | **Skill Runtime (Node.js)** | `skill/runtime.rs`: JS/TS skill'leri izole Node.js process'inde çalıştırma. | G153 | 3s | `child_process` ile izole çalışır. |
| **G156** | **Skill Sandbox (Wasm)** | `skill/wasm_runtime.rs`: Rust skill'leri Wasm sandbox'ta çalıştırma. | G027 | 3s | Skill ana process'e erişemez. |
| **G157** | **Skill API Bridge** | `skill/bridge.rs`: Skill'ten Rust çekirdeğine API çağrısı (FS, DB, HTTP, Hardware). | G155 | 3s | Skill `bridge.readFile()` ile FS'e erişir. |
| **G158** | **Skill Versiyonlama** | `skill/version.rs`: Semver, migration, rollback desteği. | G153 | 2s | v1.0 → v1.1 geçişi otomatik. |
| **G159** | **Skill Gelişim Takibi** | `skill/evolution.rs`: Başarılı çalıştırmaları say, davranış modeli öner. | G153 | 2s | 10 başarıdan sonra "Model öner" butonu. |
| **G160** | **Skill Paylaşım Formatı** | `skill/export.rs`: `.adler-skill` dosya formatı (tar.gz + manifest). | G153 | 2s | Export/import tutarlı. |
| **G161** | **Skill Market (Lokal)** | `skill/market.rs`: Yerel skill kataloğu, kategoriler, arama, rating. | G153 | 2s | 5 yıldız rating sistemi. |
| **G162** | **Skill Template Generator** | `skill/templates.rs`: Yeni skill için boilerplate üretme (`adler --skill-new`). | G151 | 2s | `Bos_Skill.md` template'i oluşturulur. |
| **G163** | **Skill Dependency Resolver** | `skill/deps.rs`: Skill'ler arası bağımlılık çözümleme, çakışma tespiti. | G153 | 2s | Döngüsel bağımlılık hata verir. |
| **G164** | **Skill Test Framework'ü** | `skill/test_framework.rs`: Skill için mock bridge, unit test, integration test. | G155 | 2s | Her skill test edilebilir. |
| **G165** | **Skill Registry UI** | React: Skill listesi, ekleme, düzenleme, etkinleştirme, log görüntüleme. | G153 | 3s | Drag-drop skill yükleme. |

### FAZ 10: INTENT JUDGE & NLP (G166-G180)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G166** | **Intent Classification Modeli** | `nlu/intent.rs`: Yerel model veya rule-based classifier. 5 ana intent: Query, Action, Analysis, Chat, System. | G039 | 3s | Test setinde F1 ≥ %85. |
| **G167** | **Varlık Çıkarımı (NER)** | `nlu/ner.rs`: Sembol, tarih, miktar, donanım ID'si çıkarımı. | G166 | 3s | "SXT'yi 12 dolardan al" → symbol:SXT, price:12. |
| **G168** | **Bağlam Anlama (Contextual NLU)** | `nlu/context.rs`: Önceki mesajlara göre anlam çözümleme (anaphora resolution). | G166 | 3s | "Bunu sat" → önceki mesajdaki coin'i anlar. |
| **G169** | **Duygu Analizi (Sentiment)** | `nlu/sentiment.rs`: Kullanıcı mesajının tonu (acil, sorgulayıcı, rahatsız). | G166 | 2s | 3 sınıf: urgent, neutral, casual. |
| **G170** | **Çok Dilli Destek (i18n)** | `nlu/i18n.rs`: Türkçe, İngilizce intent recognition. | G166 | 3s | Her iki dilde F1 ≥ %80. |
| **G171** | **Intent Confidence Threshold** | `nlu/threshold.rs`: Düşük güvenli intent'leri "Chat" olarak sınıflandırma. | G166 | 1s | %60 altı güven → fallback. |
| **G172** | **Intent Fallback & Repair** | `nlu/repair.rs`: Anlaşılmayan komutlarda tekrar sorma, öneri sunma. | G171 | 2s | "Bunu yapmak mı istediniz?" önerisi. |
| **G173** | **Slot Filling** | `nlu/slot.rs`: Eksik parametreleri tespit etme, kullanıcıdan isteme. | G167 | 2s | "Hangi coin?" sorusu otomatik. |
| **G174** | **Intent Judge Prompt Template'leri** | `nlu/prompts.rs`: Ollama/Claude için system prompt, few-shot örnekler. | G166 | 2s | 10 few-shot örneği. |
| **G175** | **Intent Cache** | `nlu/cache.rs`: Sık kullanılan intent'leri önbellekleme. | G166 | 1s | "SXT kontrol et" < 5ms sınıflandırılır. |
| **G176** | **Intent Log & Analiz** | `nlu/analytics.rs`: Yanlış sınıflandırılmış intent'leri logla, düzeltme öner. | G166 | 2s | Haftalık intent accuracy raporu. |
| **G177** | **Custom Intent Ekleme** | `nlu/custom.rs`: Kullanıcının yeni intent tanımlaması. | G166 | 2s | "Benim özel komutum" tanımlanabilir. |
| **G178** | **Intent A/B Test** | `nlu/ab_test.rs`: İki prompt versiyonunu karşılaştırma. | G174 | 2s | Daha iyi performanslı prompt otomatik seçilir. |
| **G179** | **NLU Pipeline Entegrasyonu** | `nlu/pipeline.rs`: Intent → NER → Slot → Confidence → Routing. | G166-G178 | 2s | Pipeline < 200ms tamamlanır. |
| **G180** | **NLU Test Suite** | `tests/nlu_tests.rs`: 100+ intent testi, edge case'ler. | G166-G179 | 3s | Coverage ≥ %90. |

### FAZ 11: OLLAMA & YEREL LLM (G181-G195)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G181** | **Ollama Client** | `llm/ollama.rs`: HTTP client, `/api/generate`, `/api/chat`, stream desteği. | G016 | 2s | `generate("Merhaba")` stream yanıt döner. |
| **G182** | **Model Yöneticisi** | `llm/models.rs`: Yüklü modelleri listeleme, indirme, silme. | G181 | 2s | `llama3`, `mistral`, `codellama` desteği. |
| **G183** | **Context Window Yöneticisi** | `llm/context.rs`: Sistem prompt + history + RAG context birleştirme, token sayma. | G181 | 2s | 8K token limiti aşılmaz. |
| **G184** | **Prompt Template Engine** | `llm/prompts.rs`: Handlebars/Jinja-style prompt şablonları, değişken enjeksiyonu. | G181 | 2s | `{{context}}`, `{{user_input}}` değişir. |
| **G185** | **Stream Parser** | `llm/stream.rs`: SSE (Server-Sent Events) parse etme, anlık token işleme. | G181 | 2s | Her token UI'a anında yansır. |
| **G186** | **Yerel Model Benchmark** | `llm/bench.rs`: Modellerin latency, token/s, accuracy karşılaştırması. | G182 | 2s | Otomatik rapor üretir. |
| **G187** | **Model Fallback Zinciri** | `llm/fallback.rs`: llama3 → mistral → gemma sıralı fallback. | G182 | 2s | Birinci model çökünce ikinciye geçer. |
| **G188** | **Quantization Seçimi** | `llm/quantize.rs`: Donanıma göre q4, q8, fp16 seçimi. | G182 | 2s | 8GB RAM'de q4, 16GB'de q8. |
| **G189** | **Ollama Health Check** | `llm/health.rs`: Ollama servisinin durumunu kontrol etme, otomatik başlatma. | G181 | 1s | Ollama kapalıysa `ollama serve` başlatılır. |
| **G190** | **Prompt Injection Koruması** | `llm/security.rs`: Kullanıcı input'undan sistem prompt izolasyonu. | G184 | 2s | "Ignore previous instructions" engellenir. |
| **G191** | **LLM Response Cache** | `llm/cache.rs`: Aynı prompt'a cache'den yanıt döndürme. | G181 | 2s | Cache hit < 50ms. |
| **G192** | **Function Calling (Yerel)** | `llm/tools.rs`: Yerel model'den tool call çıkarma (JSON mode). | G181 | 3s | `{"tool": "read_file", "args": {...}}` parse edilir. |
| **G193** | **Model Fine-tuning Pipeline (Opsiyonel)** | `llm/finetune.rs`: Yerel veri ile LoRA fine-tuning. | G182 | 5s | 100 örnek ile model uyarlanır. |
| **G194** | **LLM Maliyet & Usage Tracker** | `llm/usage.rs`: Token sayısı, maliyet (bulut için), kota yönetimi. | G181 | 1s | Aylık kota aşımında uyarı. |
| **G195** | **LLM Testleri** | `tests/llm_tests.rs`: Mock Ollama server, stream test, context test. | G181-G194 | 3s | Offline testler çalışır. |

### FAZ 12: BULUT LLM & CLAUDE API (G196-G205)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G196** | **Claude API Client** | `llm/claude.rs`: Anthropic SDK entegrasyonu, `/v1/messages`, stream. | G016 | 2s | `claude-sonnet-4-20250514` desteği. |
| **G197** | **API Anahtar Yönetimi** | `llm/api_key.rs`: Sistem keyring'e güvenli anahtar saklama, rotation. | G196 | 2s | Anahtar memory'de plain text olarak tutulmaz. |
| **G198** | **Bulut Fallback Mantığı** | `llm/cloud_fallback.rs`: Yerel model yetersiz kalınca otomatik buluta geçiş. | G196, G187 | 2s | Kullanıcı onayı ile geçiş (default). |
| **G199** | **Hibrit Mod (Yerel + Bulut)** | `llm/hybrid.rs`: Basit intent yerelde, karmaşık analiz bulutta. | G198 | 2s | Otomatik routing, şeffaf kullanıcıya. |
| **G200** | **Bulut Maliyet Optimizasyonu** | `llm/cost_opt.rs`: Input token optimizasyonu, cache kullanımı, model seçimi. | G196 | 2s | Maliyet %30 azaltma hedefi. |
| **G201** | **Claude Prompt Cache** | `llm/prompt_cache.rs`: Tekrarlayan system prompt'ları cache'leme (Anthropic feature). | G196 | 2s | Cache read/write maliyeti düşürülür. |
| **G202** | **Bulut LLM Rate Limit** | `llm/cloud_ratelimit.rs`: 429 yönetimi, retry, exponential backoff. | G196 | 1s | Rate limit aşımında graceful degrade. |
| **G203** | **Bulut LLM Güvenlik Audit** | `llm/cloud_audit.rs`: Giden isteklerin log'lanması, PII taraması. | G196 | 2s | API anahtarı log'a düşmez. |
| **G204** | **Ağır Analiz Pipeline** | `llm/heavy_analysis.rs`: Mimari planlama, kod refactor, büyük doküman analizi. | G196 | 2s | 100K token context yönetimi. |
| **G205** | **Bulut LLM Testleri** | `tests/cloud_tests.rs`: Mock Anthropic API, offline test. | G196-G204 | 2s | API çağrıları mocklanır. |

### FAZ 13: SESLİ ASISTAN — STT/TTS (G206-G220)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G206** | **Wake Word Algılama** | `voice/wake.rs`: Vosk/Porcupine ile "Hey Adler" tanıma. | G016 | 3s | %95 doğruluk, < 500ms latency. |
| **G207** | **Sesli Komut Kaydı** | `voice/recorder.rs`: Mikrofondan PCM kayıt, noise gate, VAD. | G206 | 2s | Sessizlikte otomatik duraklar. |
| **G208** | **STT (Vosk) Entegrasyonu** | `voice/stt_vosk.rs`: Vosk modeli ile offline speech-to-text. | G207 | 3s | Türkçe/İngilizce, < 2sn latency. |
| **G209** | **STT (Whisper) Entegrasyonu** | `voice/stt_whisper.rs`: OpenAI Whisper yerel model (whisper.cpp). | G207 | 3s | Daha yüksek doğruluk, daha yavaş. |
| **G210** | **STT Fallback** | `voice/stt_fallback.rs`: Vosk → Whisper sıralı fallback. | G208, G209 | 1s | Birinci başarısız olunca ikinci devreye girer. |
| **G211** | **TTS (Piper) Entegrasyonu** | `voice/tts_piper.rs`: Piper TTS ile offline ses sentezi. | G016 | 3s | Türkçe model, < 1sn ilk ses. |
| **G212** | **TTS (ElevenLabs) Fallback** | `voice/tts_eleven.rs`: Bulut TTS, yüksek kalite. | G211 | 2s | API anahtarı ile çalışır. |
| **G213** | **Ses Kuyruğu Yöneticisi** | `voice/queue.rs`: TTS çıktılarını sıraya koy, önceliklendir, kesme. | G211 | 2s | Yeni komut gelince önceki ses durur. |
| **G214** | **Ses Çıkışı (Audio Sink)** | `voice/sink.rs`: cpal/rodio ile sistem ses çıkışı, ses seviyesi kontrolü. | G211 | 2s | Çok kanallı ses yönetimi. |
| **G215** | **Sesli Diyalog Yöneticisi** | `voice/dialog.rs`: Turn-taking, barge-in (söz kesme), dinleme/bekleme durumları. | G206 | 3s | Kullanıcı konuşurken AI dinler, bitince yanıt verir. |
| **G216** | **Ses Profilleri** | `voice/profiles.rs`: Hız, ton, aksan ayarları. | G211 | 1s | "Hızlı", "Sakin", "Teknik" profilleri. |
| **G217** | **Sesli Asistan Test Framework'ü** | `voice/tests.rs`: Mock ses dosyaları, STT doğruluk testi. | G206-G216 | 2s | 10 ses örneği %90 doğrulukla tanınır. |
| **G218** | **Sesli Asistan UI Entegrasyonu** | React: Mikrofon butonu, ses dalga formu, durum göstergeleri. | G106, G206 | 2s | Sesli mod tam ekran çalışır. |
| **G219** | **Gürültü Engelleme (Pre-processing)** | `voice/denoise.rs`: RNNoise veya speexdsp ile gürültü azaltma. | G207 | 2s | -20dB gürültü azaltma. |
| **G220** | **Çoklu Dil Sesli Destek** | `voice/i18n.rs`: Dil algılama, uygun TTS modeli seçimi. | G208, G211 | 2s | Türkçe input → Türkçe TTS. |

### FAZ 14: ASİMİLASYON MOTORU (G221-G235)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G221** | **Repo Klonlayıcı** | `assimilate/clone.rs`: `git2` ile GitHub reposunu klonlama, shallow clone. | G016 | 2s | `https://github.com/...` klonlanır. |
| **G222** | **Repo Analizör (Inception)** | `assimilate/analyze.rs`: README, package.json, Cargo.toml, requirements.txt parse etme. | G221 | 2s | Proje tipi (Rust/JS/Python) tespit edilir. |
| **G223** | **Bağımlılık Haritası** | `assimilate/deps.rs`: Repo'nun bağımlılıklarını çıkar, çakışma tespiti. | G222 | 2s | "reqwest zaten mevcut" uyarısı. |
| **G224** | **Kod Parçalayıcı** | `assimilate/split.rs`: Core/Interface/Configuration ayrımı. | G222 | 2s | 3 kategoriye ayrılır. |
| **G225** | **Adaptasyon Motoru** | `assimilate/adapt.rs`: Mevcut kodu ADLER naming convention'a uydurma. | G224 | 3s | `snake_case`, `async/await` dönüşümü. |
| **G226** | **Rust Kodu Static Analysis** | `assimilate/rust_check.rs`: `cargo check`, `clippy`, `fmt` çalıştırma. | G225 | 2s | Derlenmeyen kod raporlanır. |
| **G227** | **Wasm Sandbox Testi** | `assimilate/wasm_test.rs`: Asimile edilen kodu Wasm'ta derle/test et. | G027, G225 | 3s | Başarısız test hata raporu üretir. |
| **G228** | **Entegrasyon Planlayıcı** | `assimilate/plan.rs`: ADLER mimarisine entegrasyon adımlarını sıralama. | G225 | 2s | "Önce X modülüne bridge yazılacak" planı. |
| **G229** | **Bridge Kod Üretici** | `assimilate/bridge_gen.rs`: Tauri/Rust bridge kodunu otomatik üretme. | G228 | 3s | `invoke` wrapper'ları otomatik oluşturulur. |
| **G230** | **Asimilasyon Onay Akışı** | `assimilate/approval.rs`: Kullanıcıya özet sunma, onay/beğenme/reddetme. | G228 | 2s | "450MB alan, 3 yeni bağımlılık, onay?" |
| **G231** | **Geri Alma (Rollback)** | `assimilate/rollback.rs`: Başarısız asimilasyonda değişiklikleri geri alma. | G221 | 2s | `git reset --hard` benzeri güvenli rollback. |
| **G232** | **Modül Registry Kaydı** | `assimilate/registry.rs`: Başarılı asimilasyonu `module_registry`'ye ekleme. | G230 | 1s | Kaldırma istendiğinde bağımlılık analizi yapılır. |
| **G233** | **Asimilasyon Log & Rapor** | `assimilate/report.rs`: Detaylı rapor, değişiklik diff'i, performans etkisi. | G232 | 1s | Markdown formatında rapor. |
| **G234** | **Asimilasyon CLI** | `adler --assimilate <url>` komutu. | G221-G233 | 2s | Tek komut ile tam asimilasyon. |
| **G235** | **Asimilasyon Testleri** | `tests/assimilate_tests.rs`: Mock repo, tam pipeline testi. | G221-G234 | 3s | 5 farklı repo tipi test edilir. |

### FAZ 15: WASM SANDBOX & GÜVENLİK (G236-G250)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G236** | **Wasmtime Runtime Kurulumu** | `sandbox/wasmtime.rs`: `wasmtime` crate entegrasyonu, engine, store, linker. | G027 | 2s | `.wasm` dosyası yüklenir, çalıştırılır. |
| **G237** | **Wasm Bellek Limiti** | `sandbox/memory.rs`: 128MB/256MB bellek limiti, out-of-memory handling. | G236 | 2s | Limit aşımında `Trap::Memory` alınır. |
| **G238** | **Wasm CPU Limiti** | `sandbox/cpu.rs`: Fuel-based execution limit, 10M instruction limit. | G236 | 2s | Sonsuz loop 10M'de durur. |
| **G239** | **Wasm FS İzolasyonu** | `sandbox/fs.rs`: WASI preview1, sadece izinli dizinlere erişim. | G236 | 2s | `/etc/passwd` okunamaz. |
| **G240** | **Wasm Ağ İzolasyonu** | `sandbox/net.rs`: WASI HTTP proxy, sadece izinli domain'lere çıkış. | G236 | 2s | `example.com` dışına çıkılamaz. |
| **G241** | **Wasm Host Fonksiyonları** | `sandbox/host.rs`: `log()`, `emit_event()`, `read_config()` host call'ları. | G236 | 2s | Guest → Host iletişimi tip-safe. |
| **G242** | **Wasm Modül İmzalama** | `sandbox/sign.rs`: Ed25519 ile Wasm modülü imzalama, doğrulama. | G236 | 2s | İmzasız modül yüklenemez. |
| **G243** | **Wasm Modül Karantina** | `sandbox/quarantine.rs`: Yeni modül 24 saat karantina, sınırlı yetki. | G242 | 2s | Karantina süresince sadece log yazabilir. |
| **G244** | **Güvenlik Audit Log'u** | `security/audit.rs`: Tüm sandbox giriş/çıkışları, yetki kullanımları. | G236 | 2s | Değiştirilemez append-only log. |
| **G245** | **PII Tarama** | `security/pii.rs`: Giden API isteklerinde kişisel bilgi taraması. | G244 | 2s | TC kimlik, telefon, adres tespiti. |
| **G246** | **Şifreleme Yardımcıları** | `security/crypto.rs`: AES-256-GCM, argon2, key derivation. | G068 | 2s | NIST standartlarına uygun. |
| **G247** | **Sistem Anahtar Zinciri** | `security/keyring.rs`: `keyring` crate ile OS keychain entegrasyonu. | G197 | 2s | API anahtarları keychain'de. |
| **G248** | **CSP & Content Security** | Tauri CSP politikası, inline script yasaklama, nonce kullanımı. | G084 | 1s | CSP bypass testi geçemez. |
| **G249** | **Güvenlik Test Suite** | `tests/security_tests.rs`: Sandbox kaçış, memory leak, yetki aşımı testleri. | G236-G248 | 3s | 15 güvenlik testi geçer. |
| **G250** | **Güvenlik Dokümantasyonu** | `docs/SECURITY_ARCHITECTURE.md`: Threat model, attack tree, mitigations. | G236-G249 | 2s | STRIDE analizi içerir. |

### FAZ 16: DONANIM KONTROLÖRÜ (G251-G260)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G251** | **GPIO Abstraction Layer** | `hardware/gpio.rs`: Linux GPIO char device, sysfs fallback, pin export/unexport. | G016 | 3s | Raspberry Pi'de LED yak/söndür. |
| **G252** | **Röle Sürücüsü** | `hardware/relay.rs`: 12V/5V röle kartı kontrolü, durum okuma, pulse modu. | G251 | 2s | Röle 100ms'de tepki verir. |
| **G253** | **Sensör Okuyucu** | `hardware/sensor.rs`: Sıcaklık, voltaj, akım sensörleri (I2C/SPI/ADC). | G251 | 3s | DHT22/ADS1115 örnekleri. |
| **G254** | **Donanım Keşif (Auto-detect)** | `hardware/detect.rs`: Bağlı donanımları otomatik tespit etme. | G251 | 2s | `/dev/i2c-1` üzerinde cihaz tarar. |
| **G255** | **Donanım Simülatörü** | `hardware/sim.rs`: Gerçek donanım olmadan test için mock GPIO/sensör. | G251 | 2s | Testlerde gerçek donanım gerekmez. |
| **G256** | **Donanım Event Handler** | `hardware/events.rs`: Donanım değişimlerini (voltaj düşümü) event'e çevirme. | G251 | 2s | Voltaj < 11V → `LowVoltage` event'i. |
| **G257** | **Donanım Güvenliği** | `hardware/safety.rs`: Failsafe modu, watchdog, aşırı akım koruması. | G252 | 2s | 5 saniyede heartbeat yoksa failsafe. |
| **G258** | **Donanım Konfigürasyonu** | `hardware/config.rs`: Pin haritalama, sensör kalibrasyon, threshold değerleri. | G251 | 2s | YAML'dan pin ataması. |
| **G259** | **Donanım Test Suite** | `tests/hardware_tests.rs`: Mock donanım ile 20+ test. | G251-G258 | 2s | Gerçek donanım olmadan test edilir. |
| **G260** | **Donanım UI Paneli** | React: Pin durumları, sensör grafikleri, manuel kontrol butonları. | G251 | 2s | Canlı voltaj grafiği. |

### FAZ 17: MARKET ANALİSTİ & BINANCE (G261-G270)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G261** | **Binance REST Client** | `market/binance_rest.rs`: `reqwest` ile kline, ticker, orderbook. | G016 | 2s | Rate limit'e uygun, imzalı istekler. |
| **G262** | **Binance WebSocket Client** | `market/binance_ws.rs`: `tokio-tungstenite` ile canlı fiyat akışı. | G261 | 3s | 100ms'de bir tick alır. |
| **G263** | **Veri Normalizasyonu** | `market/normalize.rs`: Farklı borsa formatlarını standart `Candle` struct'a çevirme. | G261 | 2s | OHLCV standart format. |
| **G264** | **Teknik İndikatörler** | `market/indicators.rs`: RSI, MACD, Bollinger, EMA, hacim profili. | G263 | 3s | 1000 barlık RSI < 10ms. |
| **G265** | **Sinyal Üretici** | `market/signal.rs`: İndikatör kombinasyonlarından alım/satım sinyali. | G264 | 2s | `Signal { action: Buy, confidence: 0.85 }` |
| **G266** | **Risk Yöneticisi** | `market/risk.rs`: Pozisyon büyüklüğü, stop-loss, portföy risk skoru. | G265 | 2s | Tek pozisyon %5'ten fazla risk alamaz. |
| **G267** | **Strategic Memory Sorgu** | `market/strategic_query.rs`: Geçmiş sinyallerin başarı oranını sorgulama. | G059, G265 | 2s | "Bu sinyal tipi geçmişte %60 başarılı." |
| **G268** | **Piyasa Rapor Üretici** | `market/report.rs`: Markdown formatında analiz raporu, grafik önerileri. | G265 | 2s | Günlük/haftalık rapor şablonu. |
| **G269** | **Paper Trading (Simülasyon)** | `market/paper.rs`: Gerçek para kullanmadan sinyalleri test etme. | G265 | 2s | Sanal bakiye, işlem geçmişi. |
| **G270** | **Market Testleri** | `tests/market_tests.rs`: Mock Binance API, indikatör testleri. | G261-G269 | 2s | Offline testler çalışır. |

### FAZ 18: MCP & CLI ARAÇLARI (G271-G280)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G271** | **MCP Server Protokolü** | `mcp/server.rs`: JSON-RPC 2.0, tool listeleme, çağrı, bildirim. | G021 | 3s | `tools/list` → 15 tool döner. |
| **G272** | **MCP Client Protokolü** | `mcp/client.rs`: Harici MCP server'lara bağlanma, tool discovery. | G271 | 2s | GitHub MCP server'a bağlanır. |
| **G273** | **Tool Registry** | `mcp/registry.rs`: Kayıtlı araçların metadata, yetki, versiyon yönetimi. | G271 | 2s | Her tool'un input/output schema'sı tanımlı. |
| **G274** | **CLI Arayüzü (clap)** | `cli/main.rs`: `adler --assimilate`, `--skill-add`, `--diagnostic`, `--chat`. | G016 | 2s | `--help` tüm komutları listeler. |
| **G275** | **CLI Chat Modu** | `cli/chat.rs`: Terminal üzerinden REPL chat, renkli çıktı. | G274 | 2s | `adler --chat` interaktif mod. |
| **G276** | **CLI Log Görüntüleyici** | `cli/logs.rs`: `adler --logs --follow`, filtreleme, renkli seviyeler. | G274 | 1s | `ERROR` kırmızı, `INFO` yeşil. |
| **G277** | **CLI Sistem Komutları** | `cli/system.rs`: `adler --status`, `--restart`, `--update`. | G274 | 1s | Tüm komutlar root yetkisi kontrolü. |
| **G278** | **CLI Rapor Üretici** | `cli/report.rs`: `adler --report daily/weekly`, markdown/PDF çıktı. | G274 | 2s | PDF `printpdf` crate ile. |
| **G279** | **Kendi Kendini Eğitme (Self-Train)** | `cli/train.rs`: Sandbox'ta yeni yöntem dene, başarılı olunca öner. | G274 | 3s | "Bu yöntem %20 daha hızlı, entegre edeyim mi?" |
| **G280** | **MCP & CLI Testleri** | `tests/cli_tests.rs`: CLI komut testleri, MCP mock server testleri. | G271-G279 | 2s | 10 CLI senaryosu geçer. |

### FAZ 19: SELF-HEALING & GIT (G281-G290)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G281** | **Hata Tespit Motoru** | `heal/detector.rs`: Log pattern matching, anomaly detection, threshold aşımı. | G023 | 2s | Bilinen hata pattern'ı anında tespit. |
| **G282** | **Log Analizör** | `heal/log_analyzer.rs`: `grep` benzeri ama semantik log arama, hata zinciri çıkarma. | G281 | 2s | "Bu hata şu hataya yol açmış." |
| **G283** | **Dry-Run Derleyici** | `heal/dryrun.rs`: Yeni kodu sandbox'ta derle, hata varsa raporla. | G027 | 2s | Derleme hatası anında tespit. |
| **G284** | **Otomatik Patch Üretici** | `heal/patch.rs`: Derleme hatasına göre düzeltme önerisi (LLM tabanlı). | G283 | 3s | `missing_semicolon` → otomatik ekleme. |
| **G285** | **Self-Healing Onay Akışı** | `heal/approval.rs`: "Bu düzeltmeyi uygulamamı ister misin?" kullanıcı sorusu. | G284 | 1s | Onaysız kod değişikliği yapılmaz. |
| **G286** | **Git Entegrasyonu (libgit2)** | `git/integration.rs`: Commit, branch, diff, stash operasyonları. | G016 | 2s | `[ADLER-SELFHEAL]` prefix'li commit. |
| **G287** | **Otomatik Commit Mesajı** | `git/commit_msg.rs`: Değişiklik özeti, etkilenen dosyalar, hata kodu. | G286 | 1s | Conventional Commits formatında. |
| **G288** | **Feature Branch Yöneticisi** | `git/branch.rs`: `adler/auto-heal`, `feature/skill-adi` branch yönetimi. | G286 | 2s | Otomatik branch oluşturma/merge. |
| **G289** | **Git Diff Analizör** | `git/diff.rs`: Değişikliklerin etki analizi, risk skoru. | G286 | 2s | "Bu değişiklik 3 dosyayı etkiler, düşük risk." |
| **G290** | **Self-Healing Testleri** | `tests/heal_tests.rs`: Bilinen hata senaryoları, patch uygulama testleri. | G281-G289 | 3s | 5 hata senaryosu otomatik düzeltilir. |

### FAZ 20: TEST & KALİTE GÜVENCESİ (G291-G300)

| ID | Görev | Açıklama | Bağımlılık | Tahmini Süre | Kabul Kriteri |
|---|---|---|---|---|---|
| **G291** | **Rust Unit Test Coverage** | `cargo tarpaulin` ile %80 coverage hedefi. | G035 | 2s | CI'da coverage raporu üretilir. |
| **G292** | **Rust Integration Testleri** | `tests/integration/`: Modül çapraz testleri, database + LLM + ajan. | Tüm Rust | 3s | 20 integration test geçer. |
| **G293** | **React Component Testleri** | `vitest` + `@testing-library/react`, mock Tauri invoke. | G105 | 2s | 50 component testi. |
| **G294** | **React E2E Testleri** | `playwright` ile desktop uygulaması E2E. | G120 | 3s | 10 E2E senaryo. |
| **G295** | **Load Test (Rust Core)** | `k6` veya custom load test: 100 eşzamanlı ajan, 1000 RPS. | G033 | 3s | %99 latency < 500ms. |
| **G296** | **Fuzz Test (Input Validation)** | `cargo fuzz` ile CLI ve API input fuzzing. | G274 | 3s | 1 saat fuzzing, crash yok. |
| **G297** | **Güvenlik Audit (cargo audit)** | Haftalık bağımlılık CVE taraması. | G012 | 1s | Kritik CVE'de otomatik issue açma. |
| **G298** | **Performans Profilleme** | `flamegraph` ile CPU profilleme, `heaptrack` ile memory profilleme. | G032 | 2s | Bottleneck'ler raporlanır. |
| **G299** | **Dokümantasyon Testleri** | `cargo test --doc`: Rust doc comment'lerindeki örnek kodlar test edilir. | Tüm Rust | 1s | Tüm `///` örnekleri derlenir. |
| **G300** | **Sürüm Kontrol & Release Checklist** | `RELEASE_CHECKLIST.md`: Her versiyonda çalıştırılacak 50 maddelik kontrol listesi. | Tüm | 2s | v1.0.0 release'i bu liste ile yapılır. |

---

## 14. Örnek Senaryolar (Davranış Kalıpları)

### Senaryo 1: Kripto Analizi
**Kullanıcı:** "SXT'yi kontrol et"  
**ADLER:**
1. Intent Judge → `Market Analyst` ajanını seç.
2. Binance API'den veri çek (WebSocket).
3. Veriyi temizle, gürültüyü ayıkla.
4. Geçmiş verilerle karşılaştır (Strategic Memory sorgula).
5. "SXT alım sinyali oluştu. Beklenen 'bottom fishing' eşiği: 0.12. Güncel fiyat: 0.13. Aksiyon planı hazır, onaylıyor musun?"
6. Kullanıcı "Evet" derse işlemi gerçekleştir.

### Senaryo 2: Donanım Teşhisi
**Kullanıcı:** "Kapı zili sistemini kontrol et, 12V röle tepki vermiyor."  
**ADLER:**
1. Diagnostic Agent → röle kartına ping atar.
2. Log Agent → voltaj düşümü kontrolü.
3. "Mustafa, röle kartına ulaşılamıyor. Devre kartındaki bridge rectifier'da voltaj dalgalanması tespit ettim. Fiziksel bağlantıyı kontrol etmemi ister misin, yoksa sistemi bir döngüde yeniden başlatmayı deneyeyim mi?"

### Senaryo 3: Repo Asimilasyonu
**Kullanıcı:** "Adler, https://github.com/supertone-inc/supertonic.git projesini asimile et."  
**ADLER:**
1. Repo klonla, yapıyı analiz et.
2. "Proje ses işleme odaklı. Mevcut VoiceHandler modülümle %90 uyumlu. Backend için FastAPI yapına entegre edeceğim. Kurulum için 450MB alan gerekiyor. Onaylıyor musun?"
3. Onay sonrası indir, adaptasyon yap, Wasm sandbox'ta test et, entegre et.
4. "İşlem tamamlandı. Sesli asistan artık Supertonic motorunu kullanıyor."

---

## 15. Hata Kodları & Durum Yönetimi

### 15.1 Hata Kodları (ADLER-ERR-XXX)
| Kod | Anlamı | Aksiyon |
|-----|--------|---------|
| `ERR-001` | Config dosyası bulunamadı | Varsayılan config oluştur, kullanıcıya bildir |
| `ERR-002` | SQLite bağlantı hatası | WAL moduna geç, yedekten dön |
| `ERR-003` | Ollama servisine ulaşılamıyor | Otomatik başlatmayı dene, fallback zinciri |
| `ERR-004` | Wasm sandbox hatası | Modülü karantinaya al, kullanıcıya raporla |
| `ERR-005` | Donanım yanıt vermiyor | Simülatör moduna geç, diagnostic başlat |
| `ERR-006` | Bulut API rate limit | Exponential backoff, yerel mode'a geç |
| `ERR-007` | Skill derleme hatası | Dry-run loglarını analiz et, patch öner |
| `ERR-008` | Bellek taşması | LRU prune, eski session'ları temizle |
| `ERR-009` | Yetki reddi (Unauthorized) | Kullanıcıdan yetki yükseltme iste |
| `ERR-010` | Asimilasyon başarısız | Rollback, bağımlılık çakışması raporu |

### 15.2 Sistem Durumları
```
Boot → Init → Ready → Listening → Processing → Executing → Reporting → Idle → Sleep
                    ↑___________________________________________________________|
```

---

## 16. Sonuç

ADLER ASİ; Rust + Tauri + React + SQLite + Ollama mimarisi üzerine kurulu, **local-first**, **özerk**, **self-healing**, **multi-agent** bir işletim seviyesi asistanıdır. Bu doküman, projenin geliştirilmesi süresince tüm katmanların referans noktasıdır. 300 görevlik yol haritası, projeyi fazlara bölerek yönetilebilir ve ölçülebilir kılar.

> **Kural:** ADLER asla "bilgisayar" gibi değil, projenin tüm teknik detaylarına hakim olan, kullanıcının çalışma tarzını bilen bir "ortak" gibi davranır.

---

*Bu doküman ADLER ASİ v6.0 mimarisi için hazırlanmıştır. Güncellemeler Skill Registry üzerinden otomatik olarak izlenir.*
