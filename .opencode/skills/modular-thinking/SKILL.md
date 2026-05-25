---
name: modular-thinking
description: "Use when the task touches 3+ files, spans multiple layers (frontend + backend), or requires distinct sequential steps. Enforces single-responsibility task breakdown: create a todolist, keep exactly one item in_progress, finish it completely before moving to the next, and report progress after each item. NEVER attempt to implement two independent changes in parallel."
---

# Modular Thinking

This skill enforces disciplined, single-responsibility task breakdown for multi-file or multi-layer work.

## When to Trigger

Automatically when the user's request:
- Touches 3+ files
- Spans frontend AND backend
- Has distinct sequential phases (e.g., "once X works, then do Y")
- Is a feature, not a bug fix

## Workflow

### 1. Create a todolist

Use `todowrite` to break the work into items. Each item MUST be:
- A single responsibility (one file change, or one conceptual change)
- Independently testable/verifiable
- Maximum one layer (frontend OR backend, never both)

### 2. One at a time

- Exactly ONE item is `in_progress` at any time
- Do NOT move to the next item until the current one is fully `completed`
- If blocked, mark the blocker as a follow-up and ask the user

### 3. Report after each item

After completing each todolist item, append a status line:

```
✓ <item description>
Next: <next item description>
```

### 4. No parallel work

NEVER:
- Edit two unrelated files in one edit call
- Implement backend + frontend changes in the same response
- Refactor while adding features in the same item
- Start work on the next item before the current item is verified

## Example Flow

```
User: "Skills Manager UI'ye search ekle, backend'de de filtreleme yap"

Assistant: [creates todolist]
  1. Backend | registry.rs | search_by_name() fonksiyonu
  2. Frontend | SkillsManager.tsx | search input + state
  3. Frontend | styles.css | search input stilleri
  4. Backend | lib.rs | yeni Tauri command

Assistant: [works on item 1, reports]
  ✓ Backend | search_by_name() eklendi
  Next: SkillsManager.tsx search input
```

Not: "Frontend'den başla, 1. maddede ne döneceğini bilince backend'e geç" gibi bağımlı item varsa, önce backend item'ı yapılır.
