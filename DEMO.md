# snact Demo Guide

## 0. Install

```bash
curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash
snact --version
```

---

## 1. Launch Chrome

```bash
snact browser launch --background
```

---

## 2. Claude Code + snact — Live Demo

### Demo 1: Single-site lookup (One-shot)

```bash
snact browser launch --background
claude
```

Prompt:

```
Use snact to find the current price of the MacBook Pro 14" M4 Pro
on apple.com and tell me what storage options are available.
```

What Claude Code does:

```bash
snact snap https://www.apple.com/shop/buy-mac/macbook-pro
# # Buy MacBook Pro
# ## Model. Choose your size.
# > 14-inch — From $1,699 | 16-inch — From $2,699
# @e35 [input:radio] ...
# ## Chip. Choose from these powerful options.
# > M5 Pro — 12-core CPU, 16-core GPU | M5 Max — ...

snact read --focus=".as-productinfosection"
# ## Model. Choose your size.
# 14-inch — From $1699 or $141.58/mo.
# 16-inch — From $2699 or $224.91/mo.
# ## Chip. Choose from these powerful options.
# M5 Pro — 12-core CPU, 16-core GPU
# M5 Max — 16-core CPU, 40-core GPU
```

**One snap to understand page structure, one read to get pricing details.**
Playwright MCP sends the entire DOM (~30K+ tokens) per turn. snact does it in ~2-4K.

---

### Demo 2: Multi-site comparison (Complex)

```
Use snact to compare the MacBook Pro 14" M4 Pro prices:
1. apple.com (official)
2. bestbuy.com
3. amazon.com
Make a comparison table with price, storage options, and availability.
```

This task visits 3 sites, each requiring snap + read + data extraction.
**Token savings compound over 10+ turns:**

| | Playwright MCP | snact |
|--|--|--|
| Tokens per turn | ~30K-80K (full DOM) | ~2-4K (snap/read) |
| 10 turns total | ~300K-800K | ~20-40K |
| **Estimated cost** | **~$1-2** | **~$0.06-0.12** |

For international sites (e.g. Amazon showing KRW), use `--locale=en-US` to force USD pricing.

---

### Demo 3: Record & Replay — teach once, run forever

This demo shows the full loop: Claude Code records a workflow interactively, then replays it on demand with zero LLM tokens.

**Step 1 — record (Claude Code does it, you watch):**

Prompt:
```
Use snact to record a workflow called "npm-zustand" that checks the
zustand package page on npmjs.com — version and last publish date.
```

What Claude Code runs:
```bash
snact record start npm-zustand
snact snap https://www.npmjs.com/package/zustand
snact record stop
# Recording saved: npm-zustand (1 steps)
#   → .snact/workflows/npm-zustand.json
```

The snap output already surfaces version and publish date in the page header — no additional read needed.

**Step 2 — list to confirm:**

```bash
snact record list
# npm-zustand  (project)
```

**Step 3 — replay (next session, zero LLM calls):**

Prompt:
```
Replay npm-zustand and tell me the current version and publish date.
```

What Claude Code runs:
```bash
snact replay npm-zustand
```

```
Replay complete: 1/1 steps
---
# zustand
> 5.0.12 • Public • Published a month ago
@e1 [link] "Readme"
@e2 [link] "5.0.12" href="/package/zustand/v/5.0.12"
...
```

Claude reads the snap output and answers — one replay call, no navigation, no extra turns.

**Same prompt tomorrow:** identical two commands, same speed, zero tokens for the replay itself.

---

> **Note:** All data-gathering commands (`snap`, `read`, `eval`) and all actions (`click`, `fill`, `type`, `select`, `scroll`, `wait`, `screenshot`) are recorded and replayable.

---

### Why other tools can't do this

| | Playwright MCP | snact |
|--|--|--|
| One-shot simple (3-5 turns) | ~150K tokens | **~15K tokens** |
| One-shot complex (10+ turns) | ~500K+ tokens | **~30K tokens** |
| **Repeated execution** | **LLM cost every time** | **0 tokens** |
| Cron automation | Requires LLM API | Shell one-liner |
| Session persistence | Re-auth every time | `session load` |
| Page understanding | LLM parses raw DOM | **Section headings** included |

---

## 3. Individual Feature Demos

After the integrated demos above, individual features can be shown:

### snap — structure + actionable elements

```bash
snact snap https://github.com/trending
# # Trending
# ## NousResearch / hermes-agent
# > The agent that grows with you | Python | Star
# @e28 [link] href="/NousResearch/hermes-agent"
# ## microsoft / markitdown
# > Python tool for converting files... | Python | Star
# @e37 [link] href="/microsoft/markitdown"
```

Elements are grouped by section headings so the LLM understands page structure at a glance.

### read — full text content

```bash
snact read https://news.ycombinator.com --focus="table.itemlist"
# Extracts page text as markdown (no screenshots needed)
```

### click / fill — element interaction

```bash
snact snap https://example.com
snact click @e1

snact snap https://github.com/login
snact fill @e2 "username"
snact fill @e3 "password"
snact click @e5
```

### eval — custom JavaScript

```bash
snact eval "JSON.stringify(Array.from(document.querySelectorAll('h2')).map(h => h.textContent))"
```

### --focus — scope limiting

```bash
# Full page: 1774 elements (Wikipedia)
snact snap "https://en.wikipedia.org/wiki/Rust_(programming_language)"

# Article only: ~300 elements
snact snap "https://en.wikipedia.org/wiki/Rust_(programming_language)" --focus="#mw-content-text"
```

### --dry-run — safe preview

```bash
snact click @e1 --dry-run
# {"action":"click","args":{"ref":"@e1"},"dry_run":true}
```

### session — save/restore browser state

```bash
snact session save github
snact session list
snact session load github
```

### record/replay — workflow recording

```bash
snact record start "my-workflow"
# ... series of snap/click/fill commands ...
snact record stop

snact replay my-workflow              # instant replay
snact replay my-workflow --speed=0.5  # slow replay (for presentation)
```

### schema — JSON Schema introspection

```bash
snact schema          # full schema
snact schema snap     # specific command schema
```

---

## 4. Cleanup

```bash
snact browser stop
```

---

## Demo Flow (Recommended Order)

```
Install
  → snact browser launch --background
  → Demo 1: MacBook Pro pricing lookup      # snap + read power
  → Demo 2: Multi-site comparison (optional) # token savings compound
  → Demo 3: record/replay automation         # killer feature
  → Individual features (as needed)
  → snact browser stop
```

---

## Troubleshooting

```bash
snact browser status           # check Chrome status
snact browser stop             # force stop

# Manual Chrome launch (if connection fails)
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --remote-debugging-port=9222 --no-first-run --no-default-browser-check

# Change port
snact --port=9333 snap https://example.com
```
