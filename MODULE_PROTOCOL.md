# ADLER ASI — Module (Skill) Development Protocol

ADLER ASI (Rust + Tauri + SQLite) sistemine yepyeni bir modül (yetenek/entegrasyon) ekleneceği veya değiştirileceği zaman AI kod ajanlarının izlemesi gereken **mutlak yaşam döngüsü** protokolüdür.

---

## Faz 1: Keşif & Bildirge (Discovery & Manifest)
1. **İsimlendirme ve Çakışma Kontrolü:** `src-tauri/src/modules/` (native modüller için) veya `src-tauri/src/agents/` (core ajanlar için) dizinlerini kontrol et.
2. **Bildirge (module.yaml):** Modül dizininin içine `module.yaml` (veya `manifest.md`) oluştur:
   - `id`: küçük-harf-tireli (örn: `google-calendar`, `file-watcher`)
   - `version`: semver (örn: 1.0.0)
   - `description`: Ajanlar tarafından okunabilecek tek satırlık net özet.
   - `permissions`: Rust OS erişimleri (örn. ağ izni, FS yazma izni).
   - `events`: Tauri EventBus üzerinden yayınlanacak (publish) veya abone olunacak (subscribe) `Event` varyantları.

---

## Faz 2: Geliştirme (Implementation - KURAL 9 Uyumlu)
Her modül geliştirilmesi, `INSTRUCTIONS.md` içerisindeki **KURAL 9 (Fullstack Orkestrasyonu)** sırasına göre yapılır:
1. **Veritabanı (`db`):** Modül persistent veri tutacaksa, `rusqlite` kullanılarak ana DB içinde kendi izole tablosunu açmalıdır.
2. **Çekirdek Mantık (`core`):** Rust tarafında modül kodu yazılır. 
   - HTTP istekleri için **sadece** `reqwest` kullanılır.
   - Credentials (API key vb.) asla koda gömülemez (Keyring/AES-GCM kullanılır).
   - Zamanlanmış görevler (cron) için `tokio::spawn` ile background worker kullanılır.
3. **Bridge (Tauri IPC):** Frontend'in çağıracağı araçlar (tools) `#[tauri::command]` ile yazılır ve yalnızca JSON string veya Serde uyumlu Struct döner.
4. **State (Zustand):** `src/stores/<modul_adi>Store.ts` içinde Global State Pattern oluşturulur.
5. **UI (React):** `src/components/` içinde görsel bileşenler hazırlanır. `invoke` doğrudan React bileşeninden değil, Zustand store'dan çağrılır.

---

## Faz 3: Kayıt & Yetkilendirme (Registration)
1. **Orchestrator Kaydı:** 
   - Rust: `lib.rs` içerisindeki `tauri::generate_handler!` makrosuna yeni komutlar eklenir.
   - SQLite: Dinamik tetiklenebilir bir beceri ise `skill_registry` tablosuna (veya `server/mod.rs` fallback JSON'una) eklenir.
2. **Event Bus Kaydı:** 
   - Modül yeni bir Tauri Event'i (örn. `github-issue-updated`) fırlatıyorsa, bu event dokümante edilmeli ve Frontend tarafında `listen()` ile yakalanmalıdır.

---

## Faz 4: Doğrulama (Backend Truth Enforcer)
Kodu tamamladığını iddia etmeden önce şu denetimleri geçmek zorundasın:
- `module.yaml` veya `manifest` geçerli mi?
- Kodda API key, token (Secret sızıntısı) var mı?
- `Backend Truth Enforcer` kurallarına uyuldu mu? (Frontend'de `mockData`, sahte JSON dizileri kullanmak YASAKTIR. Tüm veriler Rust backend'den gelmelidir).
- `cargo check` ve `tsc --noEmit` hatasız geçiyor mu?

---

## Faz 5: Yönlendirme ve Kalkan Kuralları (Orchestrator Agent İçin)
Kullanıcı "X entegrasyonu yaz", "Y modülü oluştur" dediğinde `intent_judge` ve `interrupt_guard` (Approval Levels) şu kuralları işler:

### Intent Classification
- `MODULE_CREATE` → Yeni modül. (Developer Agent'a yönlendir)
- `MODULE_INTEGRATE` → Mevcut modülü hesaba bağla (örn. OAuth). (Integration Agent'a yönlendir)
- `MODULE_MODIFY` → Mevcut modülü değiştir. (Developer Agent + Test kontrolü)

### Güvenlik Seviyesi (Trust Check)
- Modül dış ağa erişiyorsa (OAuth/API) veya `src/` dışında bir yere dosya yazıyorsa -> **Bloke et, Onay İste (Trust: USER)**.
- Modül sadece `src-tauri/src/modules/<id>/` içinde çalışıyorsa -> **Otomatik İzin Ver (Trust: PROJECT)**.

### Kullanıcı Diyalog Akışı
1. **Acknowledge:** "Anladım, `<modul_adi>` için çalışmaya başlıyorum."
2. **Discover:** "Mevcut sistemde şu modüller var. Yeni modül şunları yapacak."
3. **Confirm Scope:** "Bu modül için [Network/FS] yetkisi gerekiyor. Onaylıyor musun?"
4. **Delegate & Validate:** Üretimi yap ve `Backend Truth Enforcer` süzgecinden geçir.
5. **Present:** "Modül hazır ve entegre edildi. Etkinleştirmek veya test etmek ister misin?"
