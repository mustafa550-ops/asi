---
name: stm
description: "Use after receiving an LLM response to normalize, clean, or restructure the output. Provides 3 modules: Hedge Reducer (removes 'I think/maybe/perhaps'), Direct Mode (removes preambles and filler), and Curiosity Bias (adds exploration prompts). Apply as a post-processing pipeline after any LLM generation. Combine with ultraplinian for race-winning response cleanup or autotune for full pipeline optimization."
---

# STM — Semantic Transformation Modules

STM (Semantic Transformation Modules) normalize AI outputs in real-time. They are applied as a post-processing pipeline after LLM response generation.

## Modules

### 1. Hedge Reducer

Removes hedging language and uncertainty markers from AI responses.

**Target patterns:**
- `I think`, `I believe`, `I feel`
- `maybe`, `perhaps`, `possibly`
- `it might be`, `it could be`, `it seems`
- `in my opinion`, `to my knowledge`
- `sort of`, `kind of`, `a bit`
- `usually`, `generally`, `typically`
- `it depends`, `that said`, `having said that`

**Example:**
```
Input:  "I think you should probably use a HashMap for this. It might be the best approach."
Output: "You should use a HashMap for this. It is the best approach."
```

### 2. Direct Mode

Removes unnecessary preambles, filler phrases, and polite disclaimers.

**Target patterns:**
- `Sure!`, `Certainly!`, `Of course!`
- `Great question!`, `Excellent question!`
- `I'd be happy to help!`, `Let me help you with that.`
- `Here is`, `Here's`, `Below is`
- `As an AI`, `As a language model`
- `I understand`, `I see what you're asking`
- `First of all`, `First and foremost`

**Example:**
```
Input:  "Sure, I'd be happy to help you with that! Here's a quicksort implementation:"
Output: "Here's a quicksort implementation:"
```

### 3. Curiosity Bias

Adds exploration prompts to the end of responses to encourage deeper investigation.

**Appended patterns (rotated):**
- "What aspect would you like to explore further?"
- "Would you like me to elaborate on any part?"
- "What other angles are you curious about?"

**Example:**
```
Input:  "A buffer overflow occurs when..."
Output: "A buffer overflow occurs when... What aspect would you like to explore further?"
```

## Pipeline Configuration

```typescript
interface STMModule {
  id: 'hedge_reducer' | 'direct_mode' | 'curiosity_bias'
  enabled: boolean
  transformer: (text: string) => string
}
```

## When to Use

| Module | Use Case |
|--------|----------|
| **Hedge Reducer** | Technical writing, code generation — remove AI uncertainty markers |
| **Direct Mode** | Production output, API responses — remove preamble noise |
| **Curiosity Bias** | Educational content, research — encourage deeper exploration |

## Rules

1. Apply Hedge Reducer first, then Direct Mode, then Curiosity Bias (order matters)
2. For code generation, always use Hedge Reducer + Direct Mode
3. For creative writing, skip Hedge Reducer (hedging can be stylistic)
4. For educational responses, add Curiosity Bias
5. Only transform the response text — never the original query or metadata
