# ADLER ASI — Kodlama Standartları ve Katkı Rehberi

> Bu doküman, ADLER ASI projesine katkıda bulunan herkesin uyması gereken kuralları, kod yazım standartlarını ve iş akışını tanımlar.

---

## 1. Dil ve İletişim

- **Kod içi tanımlayıcılar:** İngilizce (değişken adları, fonksiyonlar, struct'lar)
- **Kullanıcı çıktısı:** Türkçe (CLI mesajları, hata bildirimleri, loglar)
- **Yorumlar:** Türkçe veya İngilizce — tutarlı olmak kaydıyla. "Neden" sorusunu yanıtlamalıdır.
- **Commit mesajları:** İngilizce, `[ADLER-SELFHEAL]` prefix'i self-healing commit'leri için.

---

## 2. Rust Kuralları

### 2.1 Derleme

- **Edition:** 2021
- **Minimum rustc:** 1.93
- **Zorunlu:** `cargo check` 0 warning, 0 error
- **Warning seviyesi:** `default` (extra lint eklenmeyecek)

### 2.2 Bellek ve Güvenlik

- **`unsafe` yasaktır.** Tüm güvenlik Rust'ın ownership modeliyle sağlanır.
- **`unwrap()` yalnızca testlerde kullanılır.** Production kodunda `map_err()` + `?` veya `Result` dönüşü zorunludur.
- **`todo!()` / `unimplemented!()` / `panic!()` yayın kodunda bulunamaz.**

### 2.3 Hata Yönetimi

```rust
// İyi
fn read_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Config okunamadi: {}", e))?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

// Kötü
fn read_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}
```

### 2.4 Async/Thread

- **`async/await`** varsayılan pattern'dir. Callback hell yasaktır.
- **`sync` wrapper'lar** kabul edilebilir (örn. OllamaClient'ın `generate_sync()`'i), ancak arka planda tokio runtime kullanmalıdır.
- **`std::thread::spawn`** yalnızca uzun ömürlü arka plan işlemleri için (MCP server, voice listener).

### 2.5 Dosya Düzeni

- Her modül kendi dizininde, `mod.rs` ile dışa açılır.
- Büyüyen modüller alt modüllere bölünür (örn. `assimilation/` → `cloner`, `analyzer`, `splitter`, `adapter`, `registry`, `dependency`, `rollback`).
- Bir dosya 400 satırı geçiyorsa bölünmelidir.

### 2.6 Struct/Enum Convention

```rust
pub struct AppConfig {        // PascalCase
    pub ollama_url: String,   // snake_case
    db_path: String,          // özel alanlar pub değil
}

pub enum ApprovalLevel {      // PascalCase
    Observer,
    SemiAutonomous,
    Strategic,
}
```

### 2.7 Trait Kullanımı

- `Agent` trait'i tüm ajanlar için zorunludur.
- Yeni bir ajan eklerken `name()`, `description()`, `can_handle()`, `execute()` implemente edilmelidir.
- `Orchestrator::register_agent()` ile pipeline'a eklenmelidir.

### 2.8 Test

- Her modül `#[cfg(test)] mod tests { ... }` içermelidir.
- `cargo test` çalıştırılabilir olmalıdır.
- Test fonksiyonları içe aktarımlar için `use super::*` kullanır.
- Beklenen hata durumları `#[should_panic]` yerine `assert!(result.is_err())` ile test edilir.
- Mock/stub kullanımında dummy struct'lar yalnızca test modülü içinde tanımlanır.

---

## 3. TypeScript/React Kuralları

### 3.1 TypeScript

- **strict mode** zorunludur.
- `any` yasaktır. Bilinmeyen tipler için `unknown` kullanılır.
- `as` keyword'ü yalnızca DOM event'lerinde kullanılır.
- İsimlendirme: `camelCase` (değişken/fonksiyon), `PascalCase` (bileşen/type/interface).

### 3.2 React

- React 19 patterns kullanılır (server component yok, client-side only).
- Bileşenler fonksiyonel olmalıdır. Class component yasaktır.
- State yönetimi: yakında gelecek Zustand store'ları kullanılacak. Şimdilik `useState` + prop drilling.
- Custom hook'lar `use` prefix'i ile `hooks/` dizininde tanımlanır.
- Her bileşen kendi dizininde `index.tsx` olarak yer alır (örn. `components/chat/ChatPanel.tsx`).

### 3.3 Tauri API

- `invoke()` çağrıları `lib/tauri.ts` wrapper'ı üzerinden yapılır.
- Event listener'lar `hooks/useTauriEvent.ts` hook'u ile yönetilir.
- Non-Tauri ortam (browser dev) için mock fallback'ler `lib/tauri.ts` içinde tanımlıdır.

### 3.4 Stil

- `styles.css` tek stil dosyasıdır. CSS-in-JS / Tailwind kullanılmaz.
- Tema: GitHub-dark inspired koyu tema — yeni bileşenler bu temayla uyumlu olmalıdır.
- `id` yerine `className` tercih edilir.

---

## 4. Git İş Akışı

### 4.1 Branch Yapısı

```
main                        → kararlı sürüm
feature/<skill-adi>          → yeni özellik
fix/<hata-kodu>              → hata düzeltmesi
adler/auto-heal              → self-healing otomatik commit'leri
```

### 4.2 Commit Mesajı

```
<tip>(<kapsam>): <açıklama>

[Türler]
feat:     Yeni özellik
fix:      Hata düzeltmesi
refactor: Kod yeniden düzenleme (KULLANMA — "refactor" kelimesi yasak)
style:    Biçim değişikliği (lint/format)
docs:     Dokümantasyon
test:     Test ekleme/düzeltme
chore:    Araç/bağımlılık değişikliği

[Özel]
[ADLER-SELFHEAL]  Self-healing otomatik commit'leri

[Örnek]
feat(skill): subprocess Python/JS/Shell icra destegi eklendi
fix(voice): wake_word was_triggered reset sorunu cozuldu
[ADLER-SELFHEAL] diagnostic.rs'taki module_path uyarisi giderildi
```

### 4.3 Pull Request

- PR'lar `main` branch'ine açılır.
- `cargo check` temiz olmalıdır.
- Self-healing commit'leri `adler/auto-heal` branch'inde toplanır, periyodik olana `main`'e merge edilir.

---

## 5. Skill Manifestosu Formatı

Yeni bir skill eklerken aşağıdaki formata uyulmalıdır:

```markdown
# Skill: <Ad>

## Meta
- **Description:** <Kısa açıklama>
- **Tool:** <Local_Python / Anthropic_API / Shell>
- **Bridge:** <Tauri_FS_Command / Rust_Core>
- **Triggers:** ["<tetikleyici1>", "<tetikleyici2>"]
- **Approval:** required | auto | strategic

## Steps
1. <Adım 1>
2. <Adım 2>
3. ...

## Logic
```python
# Gömülü kod (Python/JS/Shell)
def run():
    pass
``````

## Evolution
- **v1.0:** <İlk sürüm>
- **v1.1:** <Güncelleme>
```

- **Steps:** Kullanıcıya gösterilecek adımlar. `$ ` ile başlayan adımlar shell komutu olarak çalıştırılır.
- **Logic:** Doğrudan subprocess'te çalıştırılır (Python/Node/Shell/Wasm).
- **Triggers:** Case-insensitive substring matching + Ollama semantic fallback.

---

## 6. Yasaklar

| Madde | Gerekçe |
|-------|---------|
| `unsafe` bloklar | Bellek güvenliği ihlali |
| ORM (Diesel, SeaORM) | SQL kontrolü kaybolur, fazladan bağımlılık |
| "refactor" kelimesi | Proje felsefesine aykırı — "yeniden yapılandırma" yok |
| `unwrap()` (production) | Panic riski |
| Çift tırnak içinde TODO yorumları | Hiçbir zaman yapılmaz |
| Callback hell | Async/await her zaman tercih edilir |
| Electron | Tauri varken Electron kullanılmaz |

---

## 7. Entegrasyon Standartları

### 7.1 Yeni Ajan Eklerken

1. `agents/<name>/` veya `agents/<name>.rs` oluştur
2. `Agent` trait'ini implemente et (`name()`, `description()`, `can_handle()`, `execute()`)
3. `lib.rs`'de `init_app_state()` içinde `orchestrator.register_agent()` ile kaydet
4. `bridge/command_router.rs`'e route ekle (opsiyonel)
5. Test ekle

### 7.2 Yeni DB Tablosu Eklerken

1. `db/schema.rs`'e CREATE TABLE ekle
2. `db/mod.rs`'e migration varsa ekle
3. `db/` altında yeni bir dosyada CRUD fonksiyonları yaz
4. `Index` eklemeyi unutma

### 7.3 Yeni CLI Komutu Eklerken

1. `cli/mod.rs`'de `Commands` enum'ına varyant ekle
2. `run()` match kolunu yaz
3. `--help` çıktısını kontrol et

---

## 8. Performans Kuralları

- **Vector search** brute-force O(n). Embedding sayısı > 10k'ye ulaştığında vektör indeksi eklenmeli.
- **Mutex lock** süresi kısa tutulmalı. Uzun süreli işlemler lock dışında yapılmalı.
- **LLM çağrıları** blocking senkron wrapper kullanır. GUI'yi bloklamamak için Tauri'in async command'leri tercih edilir.
- **Dosya işlemleri** (assimilation, document reader) için buffer size'ına dikkat edilmeli.

---

## 9. Güvenlik Kuralları

- API anahtarları **keyring** içinde AES-256-GCM ile şifrelenmelidir.
- Üçüncü taraf kodlar **Wasm sandbox** içinde çalıştırılmalıdır, asla direkt exec edilmemelidir.
- Kullanıcı onayı gerektiren işlemler (finansal, donanımsal kalıcı değişiklik) `ApprovalLevel::Observer` modunda onay beklemelidir.
- Hassas veri loglanmamalıdır.
- Config'deki `approval_level` alanı `full` ise uyarı verilmelidir (SecurityAudit).

---

## 10. Ortam Gereksinimleri

| Bağımlılık | Zorunlu | Versiyon |
|-----------|---------|----------|
| Rust | Evet | 1.93+ |
| pnpm | Evet | 9+ |
| Ollama | Evet | 0.5+ |
| libvosk | Ses için | 0.3+ |
| espeak-ng | TTS için | 1.51+ |
| Node.js | UI build için | 20+ |
| wasm-pack | Rust→Wasm için | Opsiyonel |
