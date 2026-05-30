# ADLER ASI'ye Katkıda Bulunma (Contributing)

ADLER projesi geniş çaplı ve otonom bir sistem olduğu için geliştirme sürecinde aşağıdaki kurallara kesinlikle uyulması gerekmektedir:

## Branch (Dal) Yapısı
- Yeni bir özellik için `feature/<ozellik-adi>` veya `feat/<ozellik-adi>` kullanın.
- Hata düzeltmeleri için `fix/<hata-adi>` kullanın.
- Otonom (Self-Healing) işlemler `adler/auto-heal` adlı branch'lere düşer.

## Commit Mesajları (Conventional Commits)
Her commit mesajı Conventional Commits standardında olmalıdır:
```
<type>(<scope>): <subject>

<body>
```
- `type`: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
- Örnek: `feat(core): add new memory consolidation agent`

## Kod Standartları
- Rust kodu için `cargo fmt` ve `cargo clippy` çalıştırılmadan commit yapılmamalıdır. (Pre-commit hook ile engellenmiştir)
- TypeScript kodu için `pnpm run format` ve `pnpm run lint` komutlarını kullanın.
- Async/await yapısı zorunludur.
- Rust kodunda `Result<T, E>` döndürülmesi ve uygun error mapping (`thiserror` gibi) yapılması esastır.

## Testler
- Eklenen her yeni Rust modülü için unit test yazılması beklenmektedir.
- Tauri ve React tarafı için `vitest` kullanılarak component/hook testleri eklenmelidir.

Teşekkürler!
