---
name: autotune
description: "Use when configuring LLM inference parameters (temperature, top_p, etc.) or when you need context-adaptive parameter optimization. Provides 5 context types (code/creative/analytical/factual/conversational), 6 sampling parameters, 5 strategies (adaptive/precise/balanced/creative/chaotic), and an EMA-based feedback learning loop. Use alongside ultraplinian for multi-model comparison or stm for output normalization."
---

# AutoTune — Context-Adaptive LLM Parameter Engine

AutoTune is a context-adaptive sampling parameter engine. It classifies user input into one of 5 context types and selects optimal LLM parameters (temperature, top_p, top_k, frequency_penalty, presence_penalty, repetition_penalty) automatically.

## Context Types (5)

| Context | Optimal Profile | Use Case |
|---------|----------------|----------|
| **code** | Low temp (0.15), high top_p (0.9) | Code generation, technical writing, structured output |
| **creative** | High temp (1.15), moderate top_p (0.95) | Storytelling, poetry, brainstorming |
| **analytical** | Moderate temp (0.7), balanced top_p (0.88) | Analysis, reasoning, problem-solving |
| **factual** | Low temp (0.3), high top_p (0.95) | Q&A, summarization, information retrieval |
| **conversational** | Moderate temp (0.85), high top_p (0.95) | Chat, dialogue, casual interaction |

## Parameters (6)

| Parameter | Short | Range | Default | Description |
|-----------|-------|-------|---------|-------------|
| temperature | T | 0.0–2.0 | 0.7 | Randomness/creativity control |
| top_p | P | 0.0–1.0 | 0.95 | Nucleus sampling threshold |
| top_k | K | 0–100 | 40 | Top-K token filtering |
| frequency_penalty | F | 0.0–2.0 | 0.0 | Penalize token repetition |
| presence_penalty | Pr | 0.0–2.0 | 0.0 | Penalize topic repetition |
| repetition_penalty | Rp | 1.0–2.0 | 1.0 | Repeat token penalty multiplier |

## Strategies (5)

| Strategy | Description | Typical Temperature |
|----------|-------------|-------------------|
| **adaptive** | Auto-detect context from message content | varies (0.15–1.15) |
| **precise** | Low creativity, high determinism | 0.2 |
| **balanced** | Moderate creativity | 0.7 |
| **creative** | High creativity | 1.0 |
| **chaotic** | Maximum randomness | 1.5 |

## Context Detection Algorithm

1. **Pattern matching:** Scan input for keywords associated with each context type (code keywords → code context, creative words → creative context)
2. **Scoring:** Each context type gets a score based on matched patterns
3. **Winner:** Context with highest composite score wins
4. **Confidence:** Score ratio determines confidence (0.0–1.0)
5. **Fallback:** If confidence < 60%, classify as `analytical`

## EMA Learning Loop

AutoTune includes an EMA (Exponential Moving Average) feedback loop:

```
feedback(message_id, rating +/-1, context_type, params_used) →
  updates learned_profiles[context_type] with EMA-weighted delta
```

- Thumbs up: reinforces current parameter profile for that context
- Thumbs down: shifts parameters away from current values
- Learning rate: 0.3 (EMA alpha)
- Profiles persist across sessions via serialization

## Usage

```typescript
interface AutoTuneResult {
  detectedContext: 'code' | 'creative' | 'analytical' | 'factual' | 'conversational'
  confidence: number
  strategy: string
  params: {
    temperature: number
    top_p: number
    top_k: number
    frequency_penalty: number
    presence_penalty: number
    repetition_penalty: number
  }
  contextScores: Array<{ type: string; percentage: number }>
  patternMatches: Array<{ pattern: string; context: string }>
  paramDeltas: Array<{ param: string; before: number; after: number; delta: number; reason: string }>
}
```

## When to Use

- **Before any LLM call:** Analyze the input message and set optimal parameters
- **Multi-turn conversations:** Re-analyze each turn — context can change mid-conversation
- **Performance tuning:** When comparing model outputs at different parameter settings
- **Feedback integration:** After user rates a response, feed back into the EMA loop
