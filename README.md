# ADLER ASI - Autonomous Digital Operator

**Local-First, Offline-First, Otonom ve Çoklu Ajanlı İşletim Seviyesi Asistanı**

ADLER ASI, stratejik kararlarınızı destekleyen, veriyi anlamlı bir hikayeye dönüştüren ve operasyonel yükü hafifleten özerk bir dijital operatör platformudur. Pnpm ve Cargo Workspace mimarisinde tasarlanmış dev bir monorepodur.

## Mevcut Durum (v0.2.0 - Alpha)
- **Rust Çekirdek (Kernel):** Otonom ajan hiyerarşisi, EventBus altyapısı ve SQLite tabanlı Memory Manager asimile edilmiş ve aktif olarak çalışmaktadır.
- **Tauri Köprüsü (Bridge):** Rust çekirdeği ile React arayüzü arasındaki çift yönlü (Sıfır-Mock) iletişim protokolü (commands ve events) tamamlanmıştır.
- **Sıradaki Aşama:** React 19 ve TailwindCSS v4 ile Otonom Dashboard ve Arayüz (UI) entegrasyonu (Faz 5).

## Proje Yapısı (Monorepo)

```text
adler-asi/
├── apps/
│   ├── desktop/      # Tauri & React Masaüstü İstemcisi
│   ├── core/         # Headless Rust Çekirdeği (Kernel)
│   └── voice/        # Sesli Asistan Servisi
├── packages/
│   ├── shared-types/ # Ortak Tip Tanımlamaları
│   └── ui-kit/       # Ortak React Bileşen Kütüphanesi
├── config/           # Genel Konfigürasyon Dosyaları
└── scripts/          # Otomasyon ve Araç Scriptleri
```

## Hızlı Başlangıç

### Ön Gereksinimler
- **Node.js**: >= 20.x
- **pnpm**: >= 9.x (`npm install -g pnpm`)
- **Rust**: 1.80+ (`rustup default stable`)
- **Sistem Kütüphaneleri** (Linux için): `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

### Kurulum

1. Repoyu klonlayın:
   ```bash
   git clone https://github.com/your-username/adler-asi.git
   cd adler-asi
   ```

2. Konfigürasyonları kopyalayın:
   ```bash
   cp .env.example .env
   ```

3. Bağımlılıkları yükleyin:
   ```bash
   pnpm install
   ```

4. Geliştirme ortamını başlatın:
   ```bash
   pnpm run dev
   ```

## Katkıda Bulunma

Proje akışına ve kodlama standartlarına uyum sağlamak için lütfen [CONTRIBUTING.md](./CONTRIBUTING.md) dosyasını okuyun.

## Lisans

Bu proje MIT Lisansı ile lisanslanmıştır. Daha fazla bilgi için [LICENSE](./LICENSE) dosyasına bakınız.
