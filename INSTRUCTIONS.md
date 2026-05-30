# ADLER ASI — Zorunlu AI Çalışma Kuralları

> **Bağlayıcılık:** Bu dosyadaki tüm kurallar mutlaktır. Hiçbir kural esnetilemez, atlanamaz veya "iyi niyetle" ihlal edilemez. İhlal tespit edilirse önceki adıma dönülür ve düzeltme yapılır.

---

## KURAL 1: Mimari Standartlara Uyum

Yazdığın her satır kod, aşağıdaki dosyalardaki tüm kararlara ve standartlara **%100 uygun** olmalıdır (bu dosyalar sistem tarafından otomatik yüklenir):

1. `CLAUDE.md` — Proje özellikleri, vizyon, tüm feature kararları
2. `ARCHITECTURE.md` — Teknik mimari, ADR'ler, katman iletişimi
3. `GUIDELINES.md` — Kodlama standartları, yasaklar, iş akışı
4. `MODULE_PROTOCOL.md` — Yeni modül/yetenek geliştirme ve ajan orkestrasyon kuralları

---

## KURAL 2: Yetki Haritasına Uy

ARCHITECTURE.md içindeki `## 12. Yetki Haritası (AI Edit Scope)` bölümüne uy. Talep türüne göre izin verilen dizinler dışında **kesinlikle** dosya oluşturma veya düzenleme yapma.

Katman değiştirirken (frontend ↔ backend ↔ config) AI'ın context'i sıfırlanmış gibi davran — önce hedef katmanın kurallarını oku, sonra kod yaz.

---

## KURAL 3: Dokümantasyon Zorunluluğu

| Öge | Kural |
|-----|-------|
| `pub fn` (Rust) | Dokümantasyon yoksa kod geçerli değildir |
| `export` (TypeScript) | JSDoc/@param/@returns zorunludur |
| React component | Props type + davranış açıklaması zorunludur |
| Zustand store | Her state alanı ve action için açıklama zorunludur |
| Tauri command | Giriş/çıkış tipleri + hata durumları belirtilmeli |

---

## KURAL 4: Skill Sistemi Davranışı (Beceri Kuralları)

- Projenin kendi ADLER skill sistemi ile `.opencode/skills/` (AI-level skills) **farklı sistemlerdir**.
- ADLER skill pipeline'ı (Rust): Kullanıcı komutu → Skill Trigger Check (Phase 0) — ilk eşleşen skill çalışır, `return` ile erken çıkar. Hiçbir skill eşleşmezse Intent→Delegate→Plan→Execute→Confirm full pipeline'ı çalışır.
- `.opencode/skills/` ve `.opencode/skills-ecc/`: AI'ın geliştirme sırasında başvurması için AI-level referans skill'lerdir. ADLER pipeline'ına eklenmezler.
- Yeni AI-level skill eklerken `.opencode/skills/` altına SKILL.md formatında ekle.
- **Tauri Fallback Kuralı:** Frontend tarafında becerileri veya komutları çağırırken mutlaka Tauri IPC'nin (`isTauri()`) desteklenmediği tarayıcı ortamlarında HTTP üzerinden (Port 1421) fallback mekanizmasını kullanacak şekilde kodla.

---

## KURAL 5: Kod Güvenliği

| Kural | Açıklama |
|-------|----------|
| `unsafe` yasak | Rust'ta `unsafe` bloklar kesinlikle kullanılmaz |
| `unwrap()` yasak | Production kodunda `?` veya `map_err` zorunlu |
| `expect()` yasak | Production'da `unwrap()` ile aynı panic riski — `map_err` + `?` kullan |
| `todo!()` yasak | Yayın kodunda bulunamaz |
| `any` yasak | TypeScript'te `any` yerine `unknown` kullan |
| "refactor" yasak | Commit mesajında veya kodda bu kelime kullanılmaz |
| Callback hell yasak | Async/await zorunlu |
| Electron yasak | Tauri v2 dışında masaüstü framework'ü kullanılmaz |
| ORM yasak | Diesel, SeaORM, Prisma gibi ORM'ler kullanılmaz |

---

## KURAL 6: Test Zorunluluğu

- Her Rust modülü `#[cfg(test)] mod tests { ... }` içermelidir.
- Yeni bir fonksiyon/feature eklendiğinde test de eklenmelidir.
- `cargo test` çalıştırılabilir olmalıdır.
- `cargo check` 0 warning, 0 error ile geçmelidir.
- Frontend TypeScript: `npx tsc --noEmit` hatasız geçmelidir.
- Frontend testleri: `pnpm vitest run` tüm testleri geçmelidir.

---

## KURAL 7: İletişim ve Dil

| Bağlam | Dil |
|--------|-----|
| Kod içi tanımlayıcılar | İngilizce (değişken, fonksiyon, struct) |
| Kullanıcı çıktısı | Türkçe (CLI, log, hata mesajı) |
| Dokümantasyon yorumları | Tutarlı olmak kaydıyla Türkçe veya İngilizce |
| Commit mesajları | İngilizce, `[ADLER-SELFHEAL]` prefix'i self-healing için |

---

## KURAL 8: Yetkilendirme Seviyeleri

| Seviye | AI Yetkisi |
|--------|-----------|
| **Observer** (varsayılan) | Finansal/donanımsal kalıcı değişikliklerde kullanıcı onayı beklenir |
| **Yarı-Otonom** | Rutin işlemler otomatik, kritik kararlar sorulur |
| **Tam Yetki** | Yalnızca kullanıcının onayladığı stratejik modellerde geçerlidir |

Observer modunda çalışıyorsan, "Bu işlem onay gerektiriyor, onaylıyor musun?" diye sor.

---

## KURAL 9: Fullstack Orkestrasyon Yol Haritası (Roadmap Skeleton)

Bir tam yığın (full-stack) özellik geliştirirken **asla** rastgele dosyalara atlanamaz. Geliştirme süreci mutlak surette aşağıdaki hiyerarşik "İskelet Sistemi" sırasına uygun olarak işlenmelidir:

1. **Faz 1: Veri ve Şema (Database/Models Katmanı)**
   - Öncelikle SQLite tablo yapıları, struct'lar ve Entity tanımları (`src-tauri/src/db/`) yapılır.
   - Rust tipleri (`Serialize, Deserialize`) belirlenir.
2. **Faz 2: Çekirdek Mantık ve Ajan (Kernel/Agent Katmanı)**
   - Veriyi işleyecek olan ilgili Rust ajanının (örn. `src-tauri/src/agents/`) logic'i yazılır.
   - Orchestrator (`core/orchestrator.rs`) veya LLM Prompt entegrasyonu tamamlanır.
3. **Faz 3: Tauri IPC ve Köprü (Bridge Katmanı)**
   - Rust fonksiyonları `#[tauri::command]` ile dışa açılır ve `tauri::generate_handler!` listesine eklenir.
   - Mutlaka `server/mod.rs` (HTTP Fallback) tarafına, tarayıcıda test edilebilmesi için mock/fallback endpoint'i eklenir.
4. **Faz 4: Durum Yönetimi (State/Store Katmanı)**
   - `src/stores/` altında Zustand store'u oluşturulur (örn. `voiceStore.ts`).
   - Sadece bu store dosyası `invoke("...")` çağrılarını barındırır. Component'ler asla doğrudan `invoke` çağırmaz!
5. **Faz 5: Kullanıcı Arayüzü (UI/Component Katmanı)**
   - React component'leri (`src/components/`) inşa edilir.
   - Bu component'ler doğrudan Zustand store'una bağlanır ve yalnızca "UI mantığına" odaklanır.

*Bu sıralama ihlal edilemez. Her faz tamamlanıp derlendiğinde (`cargo check` / `tsc --noEmit`) bir sonraki faza geçilir.*

---

*Bu dosya ADLER ASI için AI çalışma kurallarını tanımlar. CLAUDE.md + ARCHITECTURE.md + GUIDELINES.md ile birlikte okunmalıdır.*
