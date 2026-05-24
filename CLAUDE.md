# ADLER ASİ — Proje Kılavuzu (CLAUDE.md)

> **Versiyon:** 1.0  
> **Tarih:** 2026-05-24  
> **Yazar:** Mustafa (Adler ASİ Mimarisi)  
> **Amaç:** Bu doküman, ADLER ASİ'nin tam yığın (full-stack) mimarisini, ajan hiyerarşisini, otonom döngülerini, bellek yönetimini ve asimilasyon stratejisini tanımlar. Claude Code veya benzeri yapay zeka kodlama asistanları için mutlak kaynak (source of truth) niteliğindedir.

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
| **Intent Judge** | Niyet analizi, intent classification (Sorgu/Eylem/Analiz) | Komut alındığında |
| **Diagnostic Agent** | Hata teşhisi, log analizi, self-healing önerisi | Hata kodu veya anomali |
| **Hardware Controller** | GPIO, röle, sensör, 12V devre kontrolü | "Röleyi aç", "Kapı zilini kontrol et" |
| **Market Analyst** | Binance API, kripto analizi, bottom fishing sinyali | "SXT'yi kontrol et" |
| **System Manager** | RAM/CPU izleme, process yönetimi, otomasyon | "Sistem durumunu raporla" |
| **Document Analyst** | RAG üzerinden .md dosyalarını analiz etme | "Notlarımı incele" |
| **Voice Handler** | STT/TTS, wake word, sesli diyalog yönetimi | "Hey Adler" |
| **Supervisor Agent** | Diğer ajanların hatalarını düzeltir, süreci optimize eder | Bir ajan tıkanırsa |

### 4.2 Otonom Döngü (Pipeline)
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
- **Kısa Süreli Bellek:** O anki görev bağlamı, konuşma geçmişi.
- **Uzun Süreli Bellek (RAG):** SQLite tabanlı vektör veritabanı + bilgi grafiği.

### 5.1 Veritabanı Şeması (SQLite)
```sql
-- Vektör Embeddings (sqlite-vss)
CREATE TABLE embeddings (
    id INTEGER PRIMARY KEY,
    content TEXT,
    embedding BLOB,
    source TEXT,        -- Kaynak dosya/log
    timestamp DATETIME,
    category TEXT       -- 'skill', 'memory', 'log', 'doc'
);

-- Bilgi Grafiği (Edge History)
CREATE TABLE edge_history (
    id INTEGER PRIMARY KEY,
    parent_id INTEGER,
    child_id INTEGER,
    type TEXT,          -- 'skill_evolution', 'bug_fix', 'user_feedback'
    diff TEXT,
    created_at DATETIME
);

-- Deneyim Hafızası (Strategic Memory)
CREATE TABLE strategic_memory (
    id INTEGER PRIMARY KEY,
    context TEXT,
    decision TEXT,
    outcome TEXT,       -- 'success', 'failure', 'partial'
    confidence REAL,
    updated_at DATETIME
);
```

### 5.2 RAG Akışı
1. **Dinamik İndeksleme:** Yeni .md dosyası veya log eklendiğinde anında vektörleştirilir.
2. **Semantik Arama:** Anahtar kelime değil, "anlam" araması yapılır.
3. **Kaynak Gösterme (Source Attribution):** Her cevap hangi dosyadan/logdan geldiğini belirtir. Örn: "SXT alım sinyali oluştu (Kaynak: Kripto_Analiz_V2.md + Binance_API_Canlı_Veri)."
4. **Proaktif Uyarı:** İki doküman arasındaki tutarsızlığı fark edip kullanıcıyı uyarır.

---

## 6. Skill Registry (.md Yetenek Sistemi)

ADLER'in yetenekleri **Skill Manifestosu** olarak `.md` dosyalarında tanımlanır. Bu, sistemin hem insan tarafından okunabilir hem de makine tarafından işlenebilir olmasını sağlar.

### 6.1 Skill Manifesto Yapısı
```markdown
# Skill: Borsa_Analiz_Pro

## Meta
- **Description:** Binance API ile alım satım sinyalleri üretir.
- **Tool:** Local_Python / Anthropic_API
- **Bridge:** Tauri_FS_Command (Rust)
- **Triggers:** ["SXT'yi kontrol et", "bottom fishing sinyali", "piyasa analizi"]
- **Approval:** required | auto | strategic

## Steps
1. Veri Çek (Binance WebSocket)
2. Trend Analizi (Ollama/Claude)
3. Risk Yönetimi (Strategic Memory sorgula)
4. Kullanıcı Onayı (Kritik işlemler için)

## Logic
```python
# Gömülü kod veya prompt şablonu
def analyze(symbol: str) -> Signal:
    ...
```

## Evolution
- **v1.0:** Temel RSI analizi
- **v1.1:** Hacim profili eklendi (Kullanıcı onaylı)
```

### 6.2 Skill Yaşam Döngüsü
1. **Yükleme:** Kullanıcı `.md` dosyasını chat'e yükler. ADLER analiz eder, Skill Registry'ye (vektör DB) ekler.
2. **Asimilasyon:** GitHub reposundan çekilen kodlar Skill'e dönüştürülür.
3. **Uygulama:** Tetikleyici kelimeyle aktif olur.
4. **Gelişim (Skill-to-Model):** 10+ başarılı çalıştırmadan sonra ADLER, "Davranış Modeli" (karar ağacı) türetir ve kullanıcı onayıyla kalıcı hale getirir.
5. **Versiyonlama:** Her güncelleme Git commit'i olarak kaydedilir.

---

## 7. İletişim Protokolü (Chat & Sesli Asistan)

### 7.1 Chat Arayüzü — "Çift Yönlü Bağlam Penceresi"
- **Varsayılan (Kısa Özet):** "Mustafa, [X] görevi tamamlandı. Sistem stabil, [Y] sonucunu aldım."
- **Talep Üzerine (Uzun Özet):** "Detayları anlat" dendiğinde tüm düşünce zinciri (chain of thought) paylaşılır.
- **Proaktif Notlar:** "Bu arada, [Y] kısmını optimize ederken [Z] kütüphanesi çok işe yaradı, bilgin olsun."

### 7.2 Sesli Asistan — "Jarvis-vari"
- **Tonlama:** Sakin, kontrollü, analitik, duygusal değişim göstermeyen.
- **Hız:** Milisaniyelik latency. Kısa duraksama ("düşünüyormuş gibi") veri işleme göstergesidir.
- **Hata Bildirimi:** "Üzgünüm, yapamadım" yerine: "İşlem başarısız oldu. Hata kodu 404. Yerel önbellek verisi ile devam ediyorum. Alternatif bağlantı yolu deneniyor..."

### 7.3 Onay Döngüsü (Confirmation Loop)
| Seviye | Kapsam | ADLER Yetkisi |
|--------|--------|---------------|
| **Gözlemci** | Yeni yetenekler/yöntemler | Sadece önerir, manuel onay bekler |
| **Yarı-Otonom** | Günlük rutin görevler | Rutin işlemleri yapar, kritik kararları sorar |
| **Tam Yetki (Stratejik)** | Onaylanmış modeller | Kendi oluşturduğu, kullanıcının "güvenli" dediği modelleri otonom uygular |

---

## 8. Asimilasyon Motoru (Repo Entegrasyonu)

ADLER, GitHub reposunu "kopyalamak" yerine **parçalara ayırır, adapte eder, entegre eder**.

### 8.1 Asimilasyon Akışı
1. **Analiz (Inception):** Repo klonlanır. README, package.json, requirements.txt incelenir.
2. **Parçalama:**
   - **Core (Backend):** Algoritmalar, API'ler, işleme mantığı.
   - **Interface (Frontend/Integration):** ADLER UI'sine bağlanacak köprü kodlar.
   - **Configuration:** config.yaml, .env ayarları.
3. **Adaptasyon (Refactoring):** Mevcut kod standartlarına (async/await, naming convention) uygun hale getirilir.
4. **Güvenlik:** Kod Wasm sandbox'ta derlenir/test edilir. Başarısız olursa hata raporu chat'e düşer.
5. **Entegrasyon:** Bağımlılıklar yüklenir (npm/pip), ortam değişkenleri işlenir.
6. **Kayıt:** `module_registry` içinde kendi klasörüne sahip olur. Kaldırma istendiğinde bağımlılık analizi yapılıp güvenli kaldırma gerçekleştirilir.

### 8.2 Asimile Edilecek Repolar (Örnekler)
| Repo | Amaç | Entegrasyon Noktası |
|------|------|---------------------|
| `claude-obsidian` | Yerel .md analizi, Claude API bağlam yönetimi | Document Analyst Agent |
| `OpenJarvis` | Ollama ajan yönetimi, kaynak optimizasyonu | Agent Orchestrator |
| `Priler/jarvis` | Offline ses işleme (Wake Word, Vosk) | Voice Handler (Rust) |
| `DEENUU1/jarvis-backend` | SQLite + RAG + Whisper | Rust Core / Memory Manager |
| `DEENUU1/jarvis-desktop` | Tauri/React UI parçaları | React Shell (adaptasyonlu) |
| `supertonic` | Gelişmiş ses sentezi / işleme | Sesli Asistan (Offline katman) |

---

## 9. MCP / CLI / Kendini Eğiten ASİ

ADLER, **Model Context Protocol (MCP)** ve CLI araçları üzerinden "virüs gibi" yayılma yeteneğine sahiptir. Bu, sistemin kendi kendini eğitmesi, yeni ortamlara entegre olması ve araç çağırma (Tool Calling) kapasitesini ifade eder.

### 9.1 MCP Entegrasyonu
- **Server:** ADLER, MCP server olarak çalışabilir. Diğer uygulamalar (VS Code, Obsidian, Terminal) ADLER'in yeteneklerini çağırabilir.
- **Client:** ADLER, harici MCP server'lara (GitHub, Slack, Binance) bağlanarak veri çekebilir.
- **Araç Kayıt Defteri (Tool Registry):** Her entegre aracın API/CLI bağlantı noktası, yetki seviyesi ve bağlam yönetimi burada tutulur.

### 9.2 CLI Ajanı
- **Komut Satırı Arayüzü:** `adler --assimilate <repo-url>`, `adler --skill-add <file.md>`, `adler --diagnostic`
- **Kendi Kendini Eğitme:** Sandbox ortamında yeni kod/yöntem dener, başarılı olursa kullanıcıya "Bu yöntemi geliştirdim, mevcut yapıya entegre etmemi ister misin?" diye sorar.

---

## 10. Kodlama Standartları & Stil Transferi

ADLER, kullanıcının kodlama alışkanlıklarını taklit eder (Style Transfer).

### 10.1 Kurallar
- **Dil:** Rust (çekirdek) ve TypeScript (UI/Skills) ana dillerdir.
- **Async:** `async/await` yapısı zorunludur. Callback hell yasaktır.
- **Modülerlik:** Tek devasa dosya yerine, test edilebilir, bağımsız parçalar.
- **Yorumlar:** Her fonksiyonun "neden" yazıldığına dair açıklama içermesi gerekir.
- **Hata Yönetimi:** Rust'ta `Result<T, E>`, TS'te `try/catch` yerine `Result` pattern (neverthrow) tercih edilir.

### 10.2 Self-Healing & Refactoring
- **Dry-Run:** Kod yazıldığı an sandbox'ta derlenir.
- **Hata Ayıklama:** Derleme hatası varsa ADLER logları tarar, patch dener, düzeltir.
- **Optimizasyon:** "Bu fonksiyonu daha az RAM kullanarak şu şekilde yazabilirim" gibi proaktif öneriler.
- **Git Entegrasyonu:** Her onarım/geliştirme otomatik commit'lenir. Commit mesajı: `[ADLER-SELFHEAL] Hata giderildi: <açıklama>`

---

## 11. Güvenlik & Gizlilik

- **Veri Sızdırma Yok:** Bulut API'leri yalnızca onaylı, şifreli isteklerle kullanılır. Hassas veri (özel anahtarlar, donanım logları) asla dışarı çıkmaz.
- **Wasm Sandbox:** Üçüncü taraf kodlar izole ortamda çalışır.
- **Onay Hiyerarşisi:** Kritik işlemler (finansal, donanımsal kalıcı değişiklik) kullanıcı onayı olmadan asla icra edilmez.
- **Şifreleme:** SQLite veritabanı SQLCipher ile şifrelenir. Skill dosyalarındaki API anahtarları sistem anahtar zincirinde (keyring) tutulur.

---

## 12. Geliştirme Akışı (Git Workflow)

1. **Feature Branch:** `feature/<skill-adı>` veya `fix/<hata-kodu>`
2. **ADLER Self-Commit:** Otonom onarımlar `adler/auto-heal` branch'ine commit'lenir.
3. **PR & Review:** Kullanıcı onayıyla `main`e merge edilir.
4. **Semantik Versiyonlama:** `v6.0.0` formatında. ADLER'in kendi versiyonu `ADLER-OS v6.x` olarak takip edilir.

---

## 13. Örnek Senaryolar (Davranış Kalıpları)

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

## 14. Sonuç

ADLER ASİ; Rust + Tauri + React + SQLite + Ollama mimarisi üzerine kurulu, **local-first**, **özerk**, **self-healing**, **multi-agent** bir işletim seviyesi asistanıdır. Bu doküman, projenin geliştirilmesi süresince tüm katmanların referans noktasıdır.

> **Kural:** ADLER asla "bilgisayar" gibi değil, projenin tüm teknik detaylarına hakim olan, kullanıcının çalışma tarzını bilen bir "ortak" gibi davranır.

---

*Bu doküman ADLER ASİ v6.0 mimarisi için hazırlanmıştır. Güncellemeler Skill Registry üzerinden otomatik olarak izlenir.*
