# Module: supertonic

## Meta
- **Source:** https://github.com/supertone-inc/supertonic
- **Author:** Supertone Inc.
- **License:** MIT (code) + OpenRAIL-M (model)
- **Assimilated:** 2026-05-25
- **Files:** Multi-language SDK examples (Python, Rust, C++, C#, Go, Swift, Java, Flutter, JS, Web)
- **Stack:** ONNX Runtime, multi-language

## Architecture
ONNX-based TTS engine with 99M parameter model:
- **Audio encoder** (AE) — speech autoencoder
- **Text-to-latent** (TTL) — flow-matching based text→latent
- **Voice styles** — JSON configs for preset voices (M1-M5, F1-F5)
- **Expression tags** — 10 inline tags (`<laugh>`, `<breath>`, `<sigh>`, etc.)

31 languages supported, 44.1kHz WAV output.

## ADLER Integration Points
| Component | ADLER Counterpart | Priority |
|-----------|------------------|----------|
| `rust/src/` — ONNX inference (ort + ndarray + hound) | VoiceHandler TTS | **Critical** |
| Voice style JSON format | Voice profile storage | High |
| Expression tags (`<laugh>` etc.) | Natural TTS output | Medium |
| Python `supertonic serve` HTTP API | Local TTS microservice | Medium |

## Key Patterns Extracted
1. **ONNX Runtime in Rust** — `ort` crate for model loading + inference
2. **ndarray + hound** — Audio tensor processing → WAV output
3. **Voice style presets** — JSON-based voice configuration
4. **Batch synthesis** — Multiple text/voice pairs in one inference call
5. **Flow-matching TTS** — Denoising steps (5-12) for quality/speed tradeoff

## Status
- [x] Clone
- [x] Analyze (ONNX TTS, multi-language SDKs)
- [x] Split
- [ ] Adapt (pipeline skipped — Rust ONNX pattern needs manual port)
- [x] Test
- [x] Register
