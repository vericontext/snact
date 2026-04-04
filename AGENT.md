---
agent-guide: true
name: snact
description: AI agent-optimized browser CLI for token-efficient browser automation
---

# snact Agent Guide

## Quick Reference

```bash
snact snap <url> [--focus "selector"]   # Extract interactable elements
snact click @e1                          # Click element
snact fill @e1 "value"                   # Set input value
snact type @e1 "text"                    # Type character by character
snact select @e1 "option"               # Select dropdown option
snact scroll down [--amount 500]         # Scroll page
snact screenshot [--file path.png]       # Capture screenshot
snact wait navigation|<selector>|<ms>    # Wait for condition
```

## Critical Invariants

### Element References
- Format: `@e<number>` (e.g., `@e1`, `@e42`)
- Only valid after a `snact snap` command
- References are invalidated after page navigation — always re-snap
- References map to `backendNodeId` which is stable within a single page load

### Command Sequencing
- Actions are stateless between invocations — each `snact` call connects, acts, and exits
- No implicit waits — use `snact wait` explicitly between navigation-triggering actions
- Always `snap` before acting — the element map must exist on disk

### Typical Flow
```bash
snact snap <url>           # 1. Navigate and extract elements
snact fill @e1 "value"     # 2. Interact with elements
snact click @e2            # 3. Trigger action
snact wait navigation      # 4. Wait for page change
snact snap                 # 5. Re-snap new page (no URL = current page)
```

### Output Modes
- **Text mode** (default in terminal): Human-readable, one element per line
- **JSON mode** (default when piped): Machine-parseable structured output
- Override with `--output json` or `--output text`
- JSON schema is stable across versions; text format may change

### Dry Run
- Use `--dry-run` on any mutation command to preview without executing
- Validates inputs and shows what would happen
- Does not connect to Chrome

### Error Handling
- Non-zero exit code on failure
- Error messages on stderr
- Element-not-found errors mean the element map is stale — re-snap

## Token Budget

| Operation | Approximate Tokens |
|-----------|-------------------|
| `snap` (full page) | 5–50 per element |
| `snap --focus "selector"` | 5–50 per element (fewer elements) |
| Action response (`ok`) | 1 |
| `screenshot` response | 1 (path only, not image data) |

## Session & Recording

```bash
# Sessions persist cookies + localStorage across invocations
snact session save <name>
snact session load <name>

# Recordings replay without LLM calls
snact record start <name>
# ... execute commands ...
snact record stop
snact replay <name>          # Instant replay, no LLM needed
```
