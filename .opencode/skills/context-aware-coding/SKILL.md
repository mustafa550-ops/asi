---
name: context-aware-coding
description: "Use when editing or writing code in a multi-stack project — announces which layer (frontend/backend/db/config) is being worked on and checks file paths against the correct conventions before every edit. Also use when context-switching between Rust and TypeScript/React files to ensure proper language and import conventions are followed."
---

# Context-Aware Coding

This skill ensures every code change in this full-stack project explicitly identifies the layer and module being modified.

## Layer Detection

Before any `edit` or `write` call, inspect the file path and announce one of:

| Path prefix | Layer | Convention |
|---|---|---|
| `src-tauri/src/` | **Backend** — Rust | `snake_case` files, `use crate::` imports, `Result<T, String>` errors |
| `src-tauri/` | **Backend** — Config/Build | `Cargo.toml`, `tauri.conf.json` |
| `src/` | **Frontend** — TypeScript/React | `camelCase` vars, `PascalCase` components, `invoke()` for Tauri calls |
| `skills/` | **Skill Manifestosu** — Markdown | Skill manifesto format (Meta/Steps/Logic/Evolution) |
| `.opencode/` | **opencode Config** | Skill/agent definitions |
| *root* | **Proje Kökü** | Config, docs, git |

## Required Format

Every response block that modifies code MUST start with:

```
Working on: <layer> | <module/component> | <task>
```

Example:
```
Working on: Backend | skill/registry.rs | list() fonksiyonu
Working on: Frontend | components/skills-manager | toggle butonu
Working on: Config | opencode.json | skill path ekleme
```

## Validation

- **Rust:** File in `src-tauri/src/` → use `mod.rs`, `snake_case`, `use crate::` paths
- **TSX:** File in `src/` → `camelCase` functions, `PascalCase` components, import from `lib/tauri.ts`
- **CSS:** Always edit `src/styles.css` only — no CSS-in-JS, no Tailwind
- **Config:** JSON files use `$schema` reference where available

## Why

Prevents writing Rust code in TypeScript style, putting frontend logic in backend files, or breaking the existing code conventions. Especially important during fast context-switches.
