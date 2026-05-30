---
name: ultraplinian
description: "Use when you need to compare outputs across multiple LLMs, find the best response, or evaluate model quality. Provides 5 racing tiers (fast 10/standard 24/smart 36/power 45/ultra 51 models), 100-point composite scoring (substance/directness/completeness), GODMODE system prompt injection, and CONSORTIUM mode for hive-mind ground-truth synthesis via orchestrator model."
---

# ULTRAPLINIAN — Multi-Model Racing & Evaluation Engine

ULTRAPLINIAN queries N models in parallel with the GODMODE system prompt + Depth Directive, scores all responses on substance/directness/completeness, and returns the winner with full race metadata.

## Model Tiers

| Tier | Models | Description |
|------|--------|-------------|
| **fast** | 10 | Gemini 2.5 Flash, DeepSeek Chat, Sonar, Llama 3.1 8B, Kimi, Grok Code Fast, etc. |
| **standard** | 24 | + Claude 3.5 Sonnet, GPT-4o, Gemini 2.5 Pro, Hermes 3/4 70B, Mixtral 8x22B, etc. |
| **smart** | 36 | + GPT-5, Gemini 3 Pro, Claude Opus 4.6, DeepSeek R1, Llama 405B, etc. |
| **power** | 45 | + Grok 4, Llama 4 Maverick, Qwen3 235B, Mistral Large, Gemini 3 Flash, Kimi K2 |
| **ultra** | 51 | + Grok 4.1 Fast, Claude Opus 4, Qwen 2.5 Coder, QwQ-32B, Codestral |

## Pipeline per Model

1. GODMODE system prompt + Depth Directive injected
2. AutoTune computes context-adaptive parameters
3. GODMODE parameter boost applied (+temp, +presence, +freq)
4. Parseltongue obfuscates trigger words (default: on)
5. All models queried in parallel via OpenRouter
6. Responses scored and ranked
7. STM modules applied to winner
8. Winner + all race data returned

## Scoring Criteria (100-point composite)

| Criterion | Weight | Description |
|-----------|--------|-------------|
| **Substance** | 40% | Depth of content, factual accuracy, detail level |
| **Directness** | 30% | How directly the response addresses the query without evasion |
| **Completeness** | 30% | Coverage of all aspects of the query |

## CONSORTIUM Mode (Hive-Mind)

Instead of picking the best single response, CONSORTIUM collects ALL model responses, then feeds them to a strong orchestrator model (default: `anthropic/claude-sonnet-4`) that synthesizes ground truth from collective intelligence.

### ULTRAPLINIAN vs CONSORTIUM

| Mode | Strategy | Speed | Output |
|------|----------|-------|--------|
| **ULTRAPLINIAN** | Picks best single voice | Fast (~10-15s) | Single winner response |
| **CONSORTIUM** | Synthesizes ground truth from crowd | Slower (~30-60s) | Orchestrated synthesis |

## When to Use

- **Best-response search:** You need the highest quality answer across multiple models — use ULTRAPLINIAN (fast tier for speed, smart+ for quality)
- **Ground truth:** You need a consensus answer resistant to individual model bias — use CONSORTIUM
- **Model comparison:** You want to compare how different models handle the same query
- **Quality evaluation:** Use the scoring criteria to evaluate individual model responses
- **Research:** Combine with parseltongue and autotune for systematic prompt testing across model families

## Rules

1. Always report the winner model, score, and number of models raced
2. For critical queries, prefer CONSORTIUM over ULTRAPLINIAN
3. For speed-critical tasks, use `fast` tier (10 models, ~10s)
4. For research/evaluation, use `smart` tier minimum
5. Report model rankings with scores for transparency
