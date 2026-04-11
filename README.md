<p align="center">
  <strong>snact</strong><br>
  <em>AI agent-optimized browser CLI &mdash; snap + act</em>
</p>

<p align="center">
  <a href="https://github.com/vericontext/snact/actions/workflows/ci.yml"><img src="https://github.com/vericontext/snact/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/vericontext/snact/releases/latest"><img src="https://img.shields.io/github/v/release/vericontext/snact" alt="Release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

---

snact lets AI agents control browsers with extreme token efficiency. One `snap` returns the page structure and every actionable element in a format an LLM can parse in a single turn.

```
$ snact snap https://github.com/trending

# Trending
@e19 [link] href="/trending"
@e20 [link] href="/trending/developers"

## NousResearch / hermes-agent
@e28 [link] href="/NousResearch/hermes-agent"
@e29 [link] href="/NousResearch/hermes-agent/stargazers"

## microsoft / markitdown
@e37 [link] href="/microsoft/markitdown"
@e38 [link] href="/microsoft/markitdown/stargazers"

$ snact click @e28    # navigate to repo
$ snact read --focus="main"   # read page content as markdown
```

## Why snact?

|  | Playwright MCP | snact |
|--|----------------|-------|
| **Tokens per page** | ~3,000 (full accessibility tree) | **~30** with `--focus` |
| **Repeated task cost** | Full LLM call each time | **0** (workflow replay) |
| **Page understanding** | Requires LLM to parse raw DOM | **Section headings** included in snap |
| **Session persistence** | None (re-auth every time) | `session save/load` |
| **Automation** | Requires LLM API | `cron + replay` (zero dependencies) |
| **Shadow DOM** | Limited | **Full** (CDP direct) |
| **Install** | npm + browser binary | **Single binary** |

### The core insight

Most browser automation tools send the entire page state to the LLM on every turn. snact sends only what matters: interactable elements grouped by page sections, with just enough context for the LLM to decide what to do next.

For repeated workflows, snact goes further: **record once, replay forever** with zero LLM cost.

## Installation

```bash
# One-line install (macOS / Linux)
curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash

# From source
cargo install --path crates/snact-cli

# Verify
snact --version
```

## Quick start

### 1. Launch Chrome

```bash
snact browser launch --background
```

### 2. Snap &mdash; see what's on the page

```bash
snact snap https://example.com
```

```
# Example Domain
@e1 [link] href="https://iana.org/domains/example"
(1 elements)
```

Elements are grouped by section headings from the page. Each `@eN` reference is stable until the next snap.

### 3. Read &mdash; understand page content

```bash
snact read https://example.com
```

```
# Example Domain
This domain is for use in documentation examples.
Learn more
```

`snap` tells you what you can **do**. `read` tells you what you can **see**. Together they replace screenshot loops.

### 4. Act &mdash; interact with elements

```bash
snact fill @e2 "user@example.com"
snact fill @e3 "password"
snact click @e1
snact wait navigation
snact snap                          # re-snap after navigation
```

### 5. Session &mdash; persist browser state

```bash
snact session save github           # cookies + localStorage
snact session load github           # restore later
```

### 6. Record & Replay &mdash; zero LLM cost

```bash
snact record start login-flow
snact snap https://app.example.com/login
snact fill @e1 "user@example.com"
snact click @e3
snact wait navigation
snact record stop

# Day 2, 3, 4... — no LLM, no tokens, ~100ms
snact replay login-flow
```

## Commands

| Command | Description |
|---------|-------------|
| `snap [url]` | Extract interactable elements with section context |
| `read [url]` | Read visible text as structured markdown |
| `click <@ref>` | Click an element |
| `fill <@ref> <value>` | Set input field value (clears existing) |
| `type <@ref> <text>` | Type text character by character (for autocomplete) |
| `select <@ref> <value>` | Select option in a dropdown |
| `scroll [direction]` | Scroll the page |
| `screenshot [--file]` | Capture page as PNG |
| `wait <condition>` | Wait for navigation, CSS selector, or timeout (ms) |
| `session save\|load\|list\|delete` | Manage browser sessions |
| `record start\|stop\|list\|delete` | Record command sequences |
| `replay <name>` | Replay a recorded workflow |
| `browser launch\|stop\|status` | Manage Chrome instance |
| `schema [command]` | JSON Schema introspection |
| `mcp` | Start MCP server (JSON-RPC over stdio) |

### Global flags

```
--port <PORT>     Chrome debugging port [default: 9222]
--output <FMT>    Output format: text, json, ndjson [default: auto-detect]
--dry-run         Preview action without executing
--lang <LANG>     Accept-Language header [default: en-US]
--focus <SEL>     CSS selector to limit scope (snap/read)
--verbose         Debug logging
```

## AI agent integration

### Claude Code (CLI)

snact works as a native CLI tool &mdash; no MCP configuration needed:

```bash
snact browser launch --background
claude
# "Use snact to check my GitHub PR review queue"
```

### MCP server

For Claude Desktop or any MCP client:

```json
{
  "mcpServers": {
    "snact": {
      "command": "snact",
      "args": ["mcp"]
    }
  }
}
```

### Piped / scripted

```bash
# Auto-detects JSON when piped
snact snap https://example.com | jq '.elements | keys[]'

# NDJSON streaming
snact snap https://example.com --output=ndjson
```

## Architecture

```
AI Agent (Claude, GPT, ...)
    │ CLI stdout/stdin
    ▼
┌─────────────────────────────────────┐
│  snact-cli    Thin CLI shell (clap) │
├─────────────────────────────────────┤
│  snact-core   Domain logic          │
│  ┌─────┐ ┌────┐ ┌───────┐ ┌─────┐  │
│  │Snap │ │Read│ │Action │ │Rec/ │  │
│  │     │ │    │ │       │ │Play │  │
│  └──┬──┘ └──┬─┘ └───┬───┘ └──┬──┘  │
│     └───┬───┘       │        │     │
│    Element Map   Session     │     │
│    (@eN refs)    Storage     │     │
├─────────────────────────────────────┤
│  snact-cdp    CDP transport         │
│  WebSocket + ~25 hand-written cmds  │
└─────────────┬───────────────────────┘
              │ WebSocket (CDP)
       ┌──────▼──────┐
       │   Chrome     │
       └─────────────┘
```

**Three-crate workspace** &mdash; `cdp` handles Chrome protocol, `core` is the library, `cli` is a thin shell.

### How contextual snap works

1. **`DOMSnapshot.captureSnapshot`** &mdash; Full flattened DOM including Shadow DOM
2. **`Accessibility.getFullAXTree`** &mdash; Semantic roles, names, properties
3. **Merge** &mdash; Join DOM nodes with AX nodes by `backendNodeId`
4. **Extract context** &mdash; Collect headings (h1-h6) and text blocks from the snapshot
5. **Filter** &mdash; Keep only interactable elements, exclude hidden/aria-hidden
6. **Compress** &mdash; Assign `@eN` refs, group by section headings, append nearby text

Result: the LLM sees page structure **and** actionable elements in one call.

### Design decisions

- **Hand-written CDP types** over generated bindings. snact needs ~25 commands; hand-writing keeps binary small and compile times under 30s.
- **Disk-based state** between invocations. snact connects, works, exits. Element maps, sessions, and workflows persist as JSON.
- **`backendNodeId`** as element identifier &mdash; stable within a page load. Selector hints stored as backup for replay self-healing.
- **Single-threaded tokio** &mdash; snact does one thing at a time.

## Data storage

All state lives in `~/.local/share/snact/` (Linux) or `~/Library/Application Support/snact/` (macOS):

```
snact/
├── element_map.json      # Current @eN → element mappings
├── sessions/{name}.json  # Saved browser sessions
├── workflows/{name}.json # Recorded workflows
└── recording.json        # Active recording state
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, project structure, and commit conventions.

## License

MIT
