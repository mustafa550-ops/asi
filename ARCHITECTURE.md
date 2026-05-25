# ADLER ASI — Mimari Dokümanı

> **Versiyon:** 0.1.0  
> **Mimari Karar Kaydı:** Bu doküman, sistemin neden bu teknolojilerle inşa edildiğini, bileşenlerin nasıl haberleştiğini ve katmanlı yapının mantığını açıklar.

---

## 1. Neden Bu Teknolojiler?

### Rust (Çekirdek)

**Karar:** Tüm kritik altyapı Rust ile yazıldı: bellek yönetimi, ajan orkestrasyonu, donanım kontrolü, Wasm sandbox.

**Gerekçe:**
- **Bellek güvenliği:** Ownership modeli sayesinde segfault, use-after-free, buffer overflow riski yok. GPIO, ses capture, WebSocket gibi düşük seviyeli işlemler güvende.
- **Sıfır maliyetli soyutlama:** Agent trait'leri, pipeline adımları, event bus — hiçbiri runtime overhead eklemez.
- **Wasm entegrasyonu:** wasmtime ile aynı dilde yazılmış native runtime, C FFI kirliliği yok.
- **libgit2 bağlamaları:** git2 crate'i ile saf Git operasyonları (clone, commit, branch) sürece dahil.

### Tauri v2 (Masaüstü Köprüsü)

**Karar:** Electron yerine Tauri.

**Gerekçe:**
- **Boyut:** ~5MB binary (Electron'da ~150MB). RAM kullanımı %90 daha düşük.
- **Güvenlik:** Rust tarafında çalışan IPC, frontend'in sisteme erişimi sınırlı. CSP null policy.
- **Event sistemi:** Tauri Events, Rust'tan React'e canlı broadcast için ideal (EventBus).

### SQLite + rusqlite (Veritabanı)

**Karar:** PostgreSQL veya başka bir server-based DB yerine embedded SQLite.

**Gerekçe:**
- **Local-First:** Veri asla dışarı çıkmaz. DB dosyası yerel diskte.
- **Sıfır yapılandırma:** Kurulum gerekmez. `rusqlite` bundled feature ile derlenir.
- **FTS5:** Full-text search desteği sayesinde keyword arama doğal olarak çalışır.
- **WAL mode:** Concurrent okuma/yazma, memory manager ve skill registry aynı anda erişebilir.

### Ollama (Yerel LLM)

**Karar:** OpenAI/Claude API'leri yerine öncelikle yerel model.

**Gerekçe:**
- **Offline-First:** İnternet yokken çalışır.
- **Gizlilik:** Komutlar ve bağlam dışarı sızmaz.
- **Maliyet:** API ücreti yok. qwen2.5:1.5b gibi küçük modeller hızlı inference sağlar.
- **Fallback:** Claude API opsiyonel olarak wire edildi, ağır analizlerde kullanılmak üzere.

### wasmtime (Wasm Sandbox)

**Karar:** Asimile edilen üçüncü taraf kodları WASM'de çalıştır.

**Gerekçe:**
- **İzolasyon:** Bellek güvenliği garantisi. Ana sistem çökmez.
- **Fuel limits:** Sonsuz döngü koruması (varsayılan 100k fuel).
- **Hafiflik:** Her WASM modülü ayrı process değil, aynı runtime'da izole instance.

### ring (Şifreleme)

**Karar:** OpenSSL yerine ring (Rust native crypto).

**Gerekçe:**
- **Saf Rust:** C bağımlılığı yok, derleme sorunu çıkarmaz.
- **Minimal API:** AES-256-GCM + PBKDF2 yeterli. FIPS sertifikası gerekmiyor.
- **Audit geçmişi:** Google tarafından kullanılıyor, düzenli güvenlik denetimi.

---

## 2. Katmanlı Mimari

```
┌─────────────────────────────────────────────────────┐
│                  REACT 19 + TYPESCRIPT               │
│  ChatPanel   Dashboard   ApprovalPanel   Skills UI   │
├─────────────────────────────────────────────────────┤
│                  TAURI IPC BRIDGE                     │
│  invoke() komutlar       EventBus broadcast          │
├─────────────────────────────────────────────────────┤
│                     RUST CORE                         │
│                                                       │
│  ┌─────────────┐  ┌────────────────┐                  │
│  │ Orchestrator │  │ MemoryManager  │                  │
│  │ (pipeline)   │  │ (short+long)   │                  │
│  ├─────────────┤  ├────────────────┤                  │
│  │ 8 Agents     │  │ SelfHealing    │                  │
│  │ (trait impl) │  │ (diagnose+fix) │                  │
│  ├─────────────┤  ├────────────────┤                  │
│  │ SkillSystem  │  │ WasmSandbox    │                  │
│  │ (registry+   │  │ (fuel limits)  │                  │
│  │  executor)   │  │                │                  │
│  ├─────────────┤  ├────────────────┤                  │
│  │ Assimilation │  │ MCP Server     │                  │
│  │ (clone→adapt)│  │ (JSON-RPC WS)  │                  │
│  └─────────────┘  └────────────────┘                  │
├─────────────────────────────────────────────────────┤
│                     SQLITE                            │
│  embeddings  edge_history  strategic_memory           │
│  skill_registry  module_registry  tool_registry       │
├─────────────────────────────────────────────────────┤
│                   OLLAMA (Local LLM)                  │
│  Intent Classification  Text Generation  Embeddings   │
└─────────────────────────────────────────────────────┘
```

---

## 3. Bileşen İletişimi

### 3.1 Frontend ↔ Backend (Tauri IPC)

İki yönlü iletişim kanalı vardır:

**Komut (invoke):** React → Rust
- `send_command(text)` → Kullanıcı mesajını pipeline'a gönderir
- `approve_action(id)` / `reject_action(id)` → Onay döngüsü
- `get_context()` → Konuşma bağlamını alır

**Olay (Event):** Rust → React (canlı broadcast)
- `pipeline-step` → Pipeline adım ilerlemesi
- `pipeline-error` → Hata durumu
- `pipeline-complete` → Pipeline tamamlandı
- `approval-required` → Kullanıcı onayı bekleniyor
- `voice-output` → Ses çıktısı hazır

### 3.2 Pipeline Akışı

Her kullanıcı komutu `Orchestrator::run_pipeline()` ile işlenir:

```
User Command
    │
    ▼
Step 0: Skill Trigger Check
    ├── SkillRegistry::find_by_trigger(text)
    ├── Match → SkillExecutor::execute() → return
    └── No match → continue
    │
    ▼
Step 1: Intent Analysis
    ├── ContextManager::push("user", text)
    ├── Ollama intent classification
    └── (sorgu/eylem/analiz/donanım/kripto/sistem/dokuman/ses)
    │
    ▼
Step 2: Agent Delegation
    ├── Filter agents by can_handle() + intent
    └── Select best-matching agent(s)
    │
    ▼
Step 3: Plan
    ├── Ollama plan generation
    └── Action plan string
    │
    ▼
Step 4: Execute
    ├── Agent::execute() with Supervisor retry (max 3)
    ├── MemoryManager::store_long_term() during execution
    └── EventBus broadcast on each step
    │
    ▼
Step 5: Confirmation Loop (if Observer mode)
    ├── EventBus::emit("approval-required")
    ├── User approve/reject via Tauri
    └── Wait or proceed
    │
    ▼
Step 6: Report
    ├── Aggregate results
    ├── MemoryManager::store_long_term()
    └── Return final report
```

### 3.3 Agent Hiyerarşisi

8 ajan, `Agent` trait'i ile tanımlanır:

| Ajan | Görev | Tetikleyici | Kaynak |
|------|-------|-------------|--------|
| **Intent Judge** | Niyet sınıflandırma | niyet, intent, ne yapmalı | `agents/intent_judge.rs` |
| **Diagnostic Agent** | Hata teşhisi, log analizi | hata, error, arıza | `agents/diagnostic.rs` |
| **Hardware Controller** | GPIO, röle, sensör kontrolü | role, gpio, sensor, kapı | `agents/hardware/` |
| **Market Analyst** | Kripto analizi, sinyal üretimi | kripto, borsa, piyasa | `agents/market_analyst/` |
| **System Manager** | Sistem izleme (CPU/RAM) | sistem, ram, cpu | `agents/system_manager.rs` |
| **Document Analyst** | Dosya tarama, RAG sorgulama | dokuman, belge, indeksle | `agents/document_analyst/` |
| **Voice Handler** | Ses giriş/çıkış, wake word | ses, voice, dinle, konuş | `agents/voice_handler/` |
| **Supervisor Agent** | Hata durumunda yeniden deneme | supervisor, retry, optimize | `agents/supervisor.rs` |

### 3.4 Bellek Sistemi (Memory Manager)

İki katmanlı bellek:

**Kısa Süreli Bellek** (`short_term: Vec<String>`)
- Son 50 mesajı tutar
- Pipeline içinde bağlam sağlar
- Token bütçesi aşıldığında en eski mesaj düşer

**Uzun Süreli Bellek (RAG)**
- Vektör gömme (cosine similarity)
- FTS5 keyword arama
- Hibrit arama (semantik + keyword)
- Source attribution: her cevap hangi dosyadan geldiğini belirtir

**Bilgi Grafiği** (`edge_history`)
- Yönlü graf (parent→child)
- Skill evolution, bug fix, user feedback ilişkileri
- Atadan soyuna traverse (DFS/BFS)

**Stratejik Bellek** (`strategic_memory`)
- Karar/deneyim kaydı
- Confidence skoru + zaman ayrışması (time decay)
- Yüksek güvenilirlikli kararları sorgulama

### 3.5 MCP Sunucusu

WebSocket üzerinden JSON-RPC 2.0 sunucusu (`ws://127.0.0.1:9876`).

- Diğer uygulamalar ADLER'in yeteneklerini MCP üzerinden çağırabilir
- `tools/list` → kayıtlı araçları listeler
- Custom tool registration → yeni yetenekler eklenebilir
- Ayrı bir tokio thread'inde çalışır, ana pipeline'ı bloklamaz

---

## 4. Veritabanı Şeması

### 6 Tablo

```sql
-- Vektör gömme deposu (cosine similarity arama)
embeddings (id, content, embedding BLOB, source, timestamp, category)

-- FTS5 sanal tablosu (keyword arama)
embeddings_fts (content, source, category) -- content=embeddings

-- Bilgi grafiği (yönlü çizge)
edge_history (id, parent_id, child_id, type, diff, created_at)

-- Stratejik karar/deneyim hafızası
strategic_memory (id, context, decision, outcome CHECK, confidence CHECK, updated_at)

-- Skill kayıtları
skill_registry (id, name UNIQUE, triggers, steps, logic_code, active, version, ...)

-- Asimile modül kayıtları
module_registry (id, name UNIQUE, path, dependencies, ...)

-- Araç kayıtları
tool_registry (id, name UNIQUE, description, approval_required, ...)
```

---

## 5. Skill Sistemi

### Skill Manifestosu (`.md`)

```markdown
# Skill: Borsa_Analiz_Pro

## Meta
- **Description:** Binance API ile alım satım sinyalleri üretir.
- **Triggers:** ["SXT'yi kontrol et", "bottom fishing sinyali"]
- **Approval:** required

## Steps
1. Veri Çek (Binance WebSocket)
2. Trend Analizi (Ollama)
3. Risk Yönetimi

## Logic
```python
def analyze(symbol):
    return check_bottom_fishing(symbol)
``````
```

### Skill Yaşam Döngüsü

1. **Yükleme:** `adler skill-add file.md` → ayrıştır → DB'ye kaydet
2. **Eşleşme:** Kullanıcı komutu trigger substring veya semantik (Ollama) ile eşleşir
3. **Çalıştırma:** Adımlar sırayla icra edilir (shell/Python/JS/Wasm)
4. **Versiyonlama:** Her güncelleme versiyon numarasını artırır
5. **Evrim:** 10+ başarılı çalışmadan sonra davranış modeli türetme

---

## 6. Asimilasyon Motoru

GitHub reposundan kod alıp ADLER'e entegre eden 6 adımlı pipeline:

1. **Clone** → libgit2 ile repo kopyalama
2. **Analyze** → Dil/framework tespiti (Cargo.toml, package.json, requirements.txt)
3. **Split** → core/interface/config kategorizasyonu
4. **Adapt** → Cross-language dönüşüm (Python/JS/Go/Ruby/C# → Rust)
5. **Sandbox** → Wasmtime'da derleme + test
6. **Register** → module_registry'e kaydetme

---

## 7. Güvenlik Mimarisi

```
┌────────────────────────────┐
│      Keyring (AES-GCM)     │  API anahtarları
│      encryption.rs         │  ring::aead
├────────────────────────────┤
│      SecurityAuditor       │  Config + env taraması
│      audit.rs              │
├────────────────────────────┤
│      WasmSandbox           │  3. parti kod izolasyonu
│      (fuel limits)         │  wasmtime::Config
├────────────────────────────┤
│      Approval Levels       │  Observer / SemiAutonomous / Strategic
│      agents/mod.rs         │  Onay döngüsü
└────────────────────────────┘
```

---

## 8. Self-Healing

Hata durumunda otomatik müdahale mekanizması:

1. **Hata Sınıflandırma** → module_not_found / syntax_error / type_mismatch / connection_error / panic (seviye: KRITIK→DUSUK)
2. **Log Analizi** → Regex ile hata kodu + satır tespiti
3. **Patch Üretimi** → Otomatik syntax düzeltme (dry-run + retry)
4. **Git Commit** → `[ADLER-SELFHEAL]` mesajıyla otomatik commit

---

## 9. CLI Mimarisi

```
main.rs
  ├── args > 1 → cli::run_from_args()
  │                 ├── clap Parser → Commands enum
  │                 └── headless pipeline execution
  └── else → tauri::Builder → lib.rs::run()
                ├── init_app_state()
                ├── 4 Tauri commands
                └── GUI window (1200x800)
```

9 CLI subcommand (assimilate, skill-add/list/activate/deactivate/run/remove, diagnostic, status, security-audit).

---

## 10. Mimari Kararlar (ADR)

| Karar | Seçim | Alternatifler | Gerekçe |
|-------|-------|---------------|---------|
| DB Şifreleme | Yok (SQLCipher planlandı) | SQLCipher | AES-GCM mevcut, DB şifrelemesi sonraki faz |
| LLM İstemcisi | Ollama (birincil) | Claude, OpenAI | Offline çalışma, gizlilik, maliyet |
| Vektör Arama | Brute-force cosine similarity | sqlite-vss, pgvector | Küçük dataset (~10k embedding) için yeterli |
| Async Runtime | Tokio per-call | Global runtime | Basitlik, taşınabilirlik |
| State Yönetimi | Mutex<T> | RwLock, actor model | Sync çağrılar için yeterli |
| Frontend State | Zustand (boş) | Redux, Jotai | Planlandı, henüz implemente edilmedi |
| UI Routing | useState tab | React Router | 3 sekme için yönlendirici gereksiz |
| Ses Sentezi | 3-tier fallback | Sadece bir motor | Offline/online/fallback garantisi |
