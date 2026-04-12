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

snact lets AI agents control browsers with extreme token efficiency. One `snap` returns page structure, section content, and every actionable element &mdash; enough for an LLM to understand and act in a single turn.

```
$ snact snap https://www.apple.com/shop/buy-mac/macbook-pro

# Buy MacBook Pro

## Model. Choose your size.
> 14-inch — From $1,699 or $141.58/mo. | 16-inch — From $2,699 or $224.91/mo.
@e35 [input:radio] "14-inch" selected
@e36 [input:radio] "16-inch"

## Chip. Choose from these powerful options.
> M5 Pro — 12-core CPU, 16-core GPU | M5 Max — 16-core CPU, 40-core GPU
@e40 [link]

$ snact click @e36
ok
---
## Model. Choose your size.                    # ← auto re-snap included
> 16-inch — Available with M5 Pro or M5 Max
@e35 [input:radio] "14-inch"
@e36 [input:radio] "16-inch" selected
```

Every action automatically returns a fresh page snapshot &mdash; no manual re-snap needed.

## Why snact?

|  | Playwright MCP | Playwright CLI | snact |
|--|----------------|----------------|-------|
| **Architecture** | Persistent MCP server | Daemon + CLI | **Stateless CLI** |
| **After click/fill** | Snapshot in response | Manual re-snapshot | **Snapshot in response** |
| **Tokens per page** | ~3K-50K¹ (full A11Y tree) | ~1K-13K¹ (YAML snapshot) | **~50-4K** (measured) |
| **Page understanding** | Raw accessibility tree | YAML refs | **Section headings + content summaries** |
| **Repeated task cost** | Full LLM call | Full LLM call | **0** (workflow replay) |
| **Session persistence** | Config-based | `--persistent` flag | `session save/load` (one command) |
| **Cron automation** | Requires LLM API | Requires LLM API | **Shell one-liner** |
| **Custom JS execution** | `browser_evaluate` | `eval` / `run-code` | **`eval`** |
| **Locale/Geo override** | Via `run-code` | Via config | **`--locale` / `--geo` flags** |
| **Shadow DOM** | Limited | Limited | **Full** (CDP direct) |
| **Install** | npm + Playwright | npm + Playwright | **Single binary** (Rust) |
| **Multi-browser** | Chromium/FF/WebKit | Chromium/FF/WebKit | Chrome only |

<details>
<summary>Token measurements (click to expand)</summary>

Measured with `wc -c / 4` on actual snap output (1 token &approx; 4 chars):

| Site | snact (full) | snact (`--focus`) |
|------|-------------|-------------------|
| example.com | 46 | &mdash; |
| GitHub Login | 172 | 60 |
| GitHub Trending | 2,152 | 614 |
| Hacker News | 2,670 | &mdash; |
| Apple MacBook Pro | 2,546 | &mdash; |
| StackOverflow | 4,363 | &mdash; |
| NYTimes | 2,417 | &mdash; |

Simple pages: 50-200 tokens. Typical pages: 2K-4K. With `--focus`: 60-600.

</details>

<sup>¹ Playwright token estimates from [scrolltest.medium.com](https://scrolltest.medium.com/playwright-mcp-burns-114k-tokens-per-test-the-new-cli-uses-27k-heres-when-to-use-each-65dabeaac7a0) (MCP ~114K per test session, CLI ~27K). Per-page figures extrapolated. snact numbers are directly measured.</sup>

### The core insight

Most browser automation tools dump the entire page state to the LLM on every turn. snact sends only interactable elements grouped by section headings, with content summaries between them. For typical pages this is **2-4K tokens** (measured) vs Playwright's reported 3-50K.

After every action (click, fill, type, select, scroll), snact automatically waits for the page to settle and returns a fresh snapshot &mdash; matching Playwright MCP's auto-snapshot behavior.

For repeated workflows, snact is the only tool that offers **record once, replay forever** with zero LLM cost.

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
# Uses persistent profile (~/.local/share/snact/profiles/default/)
# Cookies and login state persist between sessions
# Use --profile=work for separate profiles
```

### 2. Snap &mdash; structure + content + elements

```bash
snact snap https://github.com/trending
```

```
# Trending

## NousResearch / hermes-agent
> The agent that grows with you | Python | Star
@e28 [link] href="/NousResearch/hermes-agent"

## microsoft / markitdown
> Python tool for converting files and office documents to Markdown. | Python | Star
@e37 [link] href="/microsoft/markitdown"
```

Section headings group elements. `>` lines summarize content (prices, descriptions, options). Each `@eN` reference is stable until the next snap.

### 3. Act &mdash; actions return updated state

```bash
snact click @e28
```

```
ok
---
# NousResearch/hermes-agent
> The agent that grows with you. Build AI agents...
@e1 [link] "Code" href="/NousResearch/hermes-agent"
@e2 [link] "Issues" href="/NousResearch/hermes-agent/issues"
...
```

Every mutation action (click, fill, type, select, scroll) automatically returns a fresh snap. No manual re-snap needed. Use `--no-snap` to disable.

### 4. Read &mdash; full text content

```bash
snact read https://example.com --focus="main"
```

```
# Example Domain
This domain is for use in documentation examples.
Learn more
```

`snap` gives you structure + elements + summaries. `read` gives you full text when you need deeper content.

### 4b. Eval &mdash; custom JavaScript extraction

When snap/read can't capture dynamic content (e.g. Amazon product cards):

```bash
snact eval "JSON.stringify(Array.from(document.querySelectorAll('.product')).map(p => ({
  title: p.querySelector('h2')?.textContent,
  price: p.querySelector('.price')?.textContent
})))"
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
snact fill @e1 "user@example.com" --no-snap
snact click @e3 --no-snap
snact wait navigation
snact record stop

# Day 2, 3, 4... — no LLM, no tokens, ~100ms
snact replay login-flow
```

## Commands

| Command | Description |
|---------|-------------|
| `snap [url]` | Page structure + section summaries + interactable elements |
| `read [url]` | Full visible text as structured markdown |
| `click <@ref>` | Click element (returns updated snap) |
| `fill <@ref> <value>` | Set input value (returns updated snap) |
| `type <@ref> <text>` | Type character by character (returns updated snap) |
| `select <@ref> <value>` | Select dropdown option (returns updated snap) |
| `scroll [direction]` | Scroll page (returns updated snap) |
| `screenshot [--file]` | Capture page as PNG |
| `eval <expression>` | Execute JavaScript on the page (for custom data extraction) |
| `wait <condition>` | Wait for navigation, CSS selector, or timeout (ms) |
| `session save\|load\|list\|delete` | Manage browser sessions |
| `record start\|stop\|list\|delete` | Record command sequences |
| `replay <name>` | Replay a recorded workflow |
| `browser launch [--profile]` | Manage Chrome (persistent profile by default) |
| `schema [command]` | JSON Schema introspection |
| `mcp` | Start MCP server (JSON-RPC over stdio) |
| `init` | Create AGENT.md in current directory (for Claude Code) |

### Global flags

```
--port <PORT>       Chrome debugging port [default: 9222]
--output <FMT>      Output format: text, json, ndjson [default: text]
--dry-run           Preview action without executing
--no-snap           Skip automatic re-snap after actions
--profile <NAME>    Browser profile name [default: "default"] (browser launch)
--lang <LANG>       Accept-Language header [default: en-US]
--locale <LOCALE>   JS navigator.language override (e.g. en-US, ja-JP)
--geo <LAT,LON>     Geolocation override (e.g. "37.7749,-122.4194")
--user-agent <UA>   Custom User-Agent string
--focus <SEL>       CSS selector to limit scope (snap/read)
--verbose         Debug logging
```

## Snap output format

snact's snap output is designed for LLM comprehension:

```
## Section Heading
> Content summary: prices, options, descriptions (up to 300 chars)
@e1 [role] "label" id="..." href="..." expanded desc="Opens in new tab"
@e2 [input:text] "Search" placeholder="..." required
```

| Component | Purpose |
|-----------|---------|
| `## Heading` | Page section structure (h1-h6) |
| `> summary` | Key text content from that section |
| `@eN` | Stable element reference for actions |
| `[role]` | Semantic role (button, link, textbox, etc.) |
| `"label"` | Accessible name |
| `id=`, `href=` | Key attributes |
| `expanded`, `collapsed` | Dropdown/accordion state |
| `selected` | Active tab/option |
| `required`, `readonly` | Form field constraints |
| `desc="..."` | Accessibility description |
| `— nearby text` | Structurally related content |

## AI agent integration

### Claude Code (CLI)

snact works as a native CLI tool &mdash; no MCP configuration needed:

```bash
snact browser launch --background
claude
# "Use snact to find the MacBook Pro M4 Pro price on apple.com"
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
snact snap https://example.com --output=json | jq '.elements | keys[]'
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
│  │     │ │    │ │+ snap │ │Play │  │
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

### How contextual snap works

1. **`DOMSnapshot.captureSnapshot`** &mdash; Full flattened DOM including Shadow DOM
2. **`Accessibility.getFullAXTree`** &mdash; Semantic roles, names, descriptions, properties
3. **Merge** &mdash; Join DOM nodes with AX nodes by `backendNodeId`
4. **Extract context** &mdash; Headings, text blocks (DOM + JS fallback for SPAs)
5. **Filter** &mdash; Keep only interactable elements, exclude hidden/aria-hidden
6. **Compress** &mdash; Group by section headings, add content summaries, assign `@eN` refs

### Auto re-snap after actions

Every mutation action (click, fill, type, select, scroll) automatically:

1. Executes the action via CDP
2. **Waits for settle** &mdash; detects navigation (waits for page load, 3s timeout) or SPA mutation (300ms settle)
3. **Takes a fresh snap** on the same transport connection
4. Returns `ok\n---\n{snap output}` so the LLM sees updated state in one turn

This matches Playwright MCP's auto-snapshot behavior while using 4-10x fewer tokens.

### Design decisions

- **Hand-written CDP types** over generated bindings &mdash; ~25 commands, fast compile
- **Disk-based state** between invocations &mdash; element maps, sessions, workflows as JSON
- **`backendNodeId`** as element identifier &mdash; stable within a page load, selector hints for replay
- **Text output by default** &mdash; optimized for LLM comprehension, not JSON parsing
- **Single-threaded tokio** &mdash; one thing at a time

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
