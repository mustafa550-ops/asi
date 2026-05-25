# ADLER ASI

**Autonomous Digital Operator — Local-First, Offline-First, Otonom**

ADLER ASI, bir masaüstü asistanından öte, stratejik kararlarınızı destekleyen, veriyi anlamlı bir hikayeye dönüştüren ve operasyonel yükü hafifleten **özerk bir dijital operatör**dür. Rust çekirdeği üzerinde yükselir, tüm işlemler yerel donanımda gerçekleşir.

![Rust](https://img.shields.io/badge/Rust-1.93+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2-purple?logo=tauri)
![React](https://img.shields.io/badge/React-19-blue?logo=react)
![SQLite](https://img.shields.io/badge/SQLite-bundled-blue?logo=sqlite)
![Ollama](https://img.shields.io/badge/Ollama-local-green)
![wasmtime](https://img.shields.io/badge/wasmtime-24-sandbox)

---

## Vizyon

ADLER sadece bir analiz aracı değildir. Sisteminize bağlanan, donanımınızı kontrol eden, kripto piyasalarını analiz eden, dokümanlarınızı anlayan ve kendi kendini iyileştiren **çok ajanlı bir işletim seviyesi asistanıdır**.

### Temel İlkeler

- **Local-First**: Veri asla dışarı sızdırılmaz. Tüm işlem yerel donanımda (Rust çekirdek + SQLite).
- **Offline-First**: İnternet olmadan çalışır. Ollama varsayılan LLM'dir. Bulut API'leri (Claude) yalnızca onaylı hibrit modda.
- **Proaktif Müdahale**: Anomali tespit eder, self-healing uygular, öngörülü aksiyon alır.
- **Şeffaflık**: Her kararın kaynağı belirtilir (source attribution).
- **Güvenlik**: AES-256-GCM şifreleme, Wasm sandbox, onay hiyerarşisi.

---

## Teknoloji Yığını

| Katman | Teknoloji |
|--------|-----------|
| Çekirdek | Rust (edition 2021) |
| Masaüstü Köprüsü | Tauri v2 |
| Arayüz | React 19 + TypeScript + Vite 6 |
| Durum Yönetimi | Zustand 5 |
| Veritabanı | SQLite (rusqlite, bundled) |
| Vektör Arama | In-memory cosine similarity |
| Yerel LLM | Ollama (HTTP API) |
| Bulut LLM | Anthropic Claude API (opsiyonel) |
| WebSocket | tokio-tungstenite |
| CLI | clap 4 |
| Wasm | wasmtime 24 (fuel limits) |
| Ses (STT) | Vosk (offline) |
| Ses (TTS) | espeak-ng / supertonic / sine-wave |
| Ses (Capture) | cpal |
| Şifreleme | ring (AES-256-GCM, PBKDF2) |
| Git | libgit2 (git2) |
| Sistem | sysinfo |

---

## Kurulum

### Gereksinimler

- Rust toolchain (1.93+)
- Node.js + pnpm
- Ollama (yerel LLM için)
- Vosk modeli (ses tanıma için, opsiyonel)
- libvosk sistemsel kütüphanesi

### 1. Ollama'yı kur ve modeli indir

```bash
ollama pull qwen2.5:1.5b
```

### 2. Projeyi derle

```bash
pnpm install
cd src-tauri
cargo build
```

### 3. Vosk model (opsiyonel)

```bash
# https://alphacephei.com/vosk/models adresinden model indir
mkdir -p ~/.config/adler/vosk-model
# İndirilen modeli buraya çıkar
```

### 4. Çalıştır

```bash
# GUI modu (Tauri penceresi)
cargo run

# Headless/CLI modu
cargo run -- --help
```

---

## Kullanım

### CLI Komutları

```
adler --help

Commands:
  assimilate <repo_url>     GitHub reposunu asimile et
  skill-add <file_path>     Skill manifestosu yükle (.md)
  skill-list                Skill'leri listele
  skill-activate <name>     Skill'i aktifleştir
  skill-deactivate <name>   Skill'i devre dışı bırak
  skill-run <name> <task>   Skill'i manuel çalıştır
  skill-remove <name>       Skill'i sil
  diagnostic                Sistem teşhisi çalıştır
  status                    Sistem durumunu raporla
  security-audit            Güvenlik denetimi yap
```

### GUI Kullanımı

Tauri penceresi açıldığında 3 sekme görürsünüz:

- **Chat**: ADLER ile doğal dilde konuşma. "SXT'yi kontrol et", "kapı zilini kontrol et", "sistem durumu" gibi komutlar.
- **Dashboard**: Sistem durumu özeti (CPU, RAM, uptime).
- **Approval**: Onay gerektiren işlemler. Gözlemci modunda her kritik aksiyon burada onay bekler.

### Skill Manifestosu Yükleme

```bash
adler skill-add skills/Borsa_Analiz_Pro.md
```

Skill dosyası `.md` formatında yazılır, frontmatter ve adım adım talimatlar içerir. ADLER trigger kelimelerle eşleşen skill'i otomatik tetikler.

### Yapılandırma

Config dosyası `~/.config/adler/config.json` yolundadır:

```json
{
  "ollama_url": "http://localhost:11434",
  "ollama_model": "qwen2.5:1.5b",
  "db_path": "adler.db",
  "mcp_port": 9876,
  "approval_level": "SemiAutonomous"
}
```

---

## Hızlı Başlangıç

```bash
# 1. Ollama'yı başlat
ollama serve

# 2. ADLER'i başlat
cd src-tauri && cargo run

# 3. Chat'e yaz:
#    "sistem durumu" — sistem bilgilerini gösterir
#    "teşhis yap"    — sistem teşhisi çalıştırır
#    "doküman tara"  — proje dosyalarını indeksler
```

---

## Lisans

Özel kullanım. Detaylı bilgi için proje sahibiyle iletişime geçin.
