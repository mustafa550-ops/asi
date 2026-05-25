---
name: documentation-first
description: "Use whenever writing or editing code — requires doc comments on every public function, exported component, and non-trivial type. Fires after every write/edit tool call: if pub fn or export is missing a doc comment, report it as an error and fix it before proceeding. Also use when reviewing code to check for missing or outdated documentation."
---

# Documentation First

Every public function, exported component, and significant type MUST have a documentation comment.

## Requirements

### Rust (`src-tauri/src/`)

Every `pub fn`, `pub struct`, `pub enum`, `pub trait` MUST have a `///` doc comment:

```rust
/// Short description of what this function does.
///
/// # Arguments
/// * `param` - description
///
/// # Returns
/// `Result<String, String>` — success message or error
pub fn my_function(param: &str) -> Result<String, String> {
```

### TypeScript/React (`src/`)

Every exported function, component, custom hook, and interface MUST have JSDoc:

```typescript
/** Renders the skill list with toggle/delete/run actions. Fetches data via Tauri invoke. */
export default function SkillsManager() {
```

### Comments explain WHY, not WHAT

Bad:
```rust
/// This function adds two numbers
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

Good:
```rust
/// Calculates the weighted score for semantic trigger matching.
/// Higher scores (50+) indicate a match. Uses Ollama confidence output.
pub fn semantic_score(input: &str) -> i32 {
```

## Enforcement

After every `write` or `edit` call:
1. Scan the modified file for `pub fn`, `pub struct`, `pub enum`, `export function`, `export default`
2. If any is missing a doc comment → flag as error
3. Fix by adding the missing comment before any other work
4. Never leave a public item undocumented

## Exceptions

- Private functions (`fn` without `pub` in Rust, no `export` in TS) — optional but encouraged
- Simple getters/setters (`pub fn name(&self) -> String`) — single-line doc comment OK
- Test functions (`#[cfg(test)]`) — no doc comment needed
- `fn main()` — no doc comment needed
- Generated/boilerplate code — add `// GENERATED: <source>` comment instead

## Why

Projects without documentation become unreadable after 3 months. Doc comments ensure:
- AI understands the codebase on next session
- Other developers (or yourself in 6 months) can navigate without reading implementation
- ARCHITECTURE.md and GUIDELINES.md stay in sync with actual code
