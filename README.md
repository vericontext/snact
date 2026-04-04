# snact

**AI agent-optimized browser CLI** — snap + act

snact lets AI agents control browsers with extreme token efficiency. Instead of dumping entire accessibility trees (~3000 tokens), snact extracts only interactable elements and outputs them in a compressed format (~30 tokens).

```
$ snact snap https://news.ycombinator.com --focus "nav"
@e1 [link] "Hacker News" href="https://news.ycombinator.com"
@e2 [link] "new" href="newest"
@e3 [link] "past" href="front"
@e4 [link] "comments" href="newcomments"
@e5 [link] "ask" href="ask"
@e6 [link] "show" href="show"
@e7 [link] "jobs" href="jobs"
@e8 [link] "submit" href="submit"
(8 elements)

$ snact click @e2
ok

$ snact fill @e1 "user@example.com"
ok
```

## Why snact?

| | Playwright MCP | agent-browser | snact |
|---|---|---|---|
| Tokens per page | ~3000 | ~500 | **~30** (with `--focus`) |
| Tool schemas overhead | 25 tools/request | Low | **Zero** (CLI) |
| Repeated task cost | Full LLM call each time | Full LLM call | **0** (workflow replay) |
| Shadow DOM | Limited | Limited | **Full** (CDP direct) |
| Install | npm + browser | cargo | **Single binary** |

## Installation

### One-line install (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash
```

Installs a pre-built binary to `/usr/local/bin/snact`. Supports macOS (arm64/x86_64) and Linux (arm64/x86_64).

### Specific version

```bash
SNACT_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash
```

### From source

```bash
# Requires Rust 1.75+
cargo install --path crates/snact-cli
```

### Build from repo

```bash
git clone https://github.com/vericontext/snact.git
cd snact
cargo build --release
# Binary at ./target/release/snact
```

## Quick Start

### 1. Start Chrome with remote debugging

```bash
# Option A: Let snact launch it
snact browser launch --headless

# Option B: Launch manually
google-chrome --headless=new --remote-debugging-port=9222
```

### 2. Snap — extract interactable elements

```bash
# Full page scan
snact snap https://example.com

# Focus on a specific area (minimizes tokens)
snact snap https://example.com --focus "form.login"
```

Output format — one element per line, minimal tokens:

```
@e1 [button] "Sign In"
@e2 [input:email] "Email" placeholder="user@example.com"
@e3 [input:password] "Password"
@e4 [link] "Forgot password?" href="/reset"
@e5 [checkbox] "Remember me"
```

### 3. Act — execute actions using element references

```bash
snact fill @e2 "user@example.com"
snact fill @e3 "mypassword"
snact click @e1
snact wait navigation
snact snap   # re-snap after navigation
```

### 4. Session — persist browser state

```bash
snact session save github-work    # Save cookies + localStorage
snact session load github-work    # Restore in a new session
snact session list
snact session delete github-work
```

### 5. Record & Replay — zero LLM cost for repeated tasks

```bash
# Record
snact record start login-flow
snact snap https://app.example.com/login
snact fill @e1 "user@example.com"
snact fill @e2 "password"
snact click @e3
snact wait navigation
snact record stop

# Replay (no LLM calls, ~100ms)
snact replay login-flow
```

## Commands

| Command | Description |
|---------|-------------|
| `snact snap [url]` | Extract interactable elements from the page |
| `snact click <@ref>` | Click an element |
| `snact fill <@ref> <value>` | Set input field value |
| `snact type <@ref> <text>` | Type text character by character |
| `snact select <@ref> <value>` | Select option in a `<select>` element |
| `snact scroll [direction]` | Scroll the page (up/down/left/right) |
| `snact screenshot` | Capture page screenshot |
| `snact wait <condition>` | Wait for navigation, selector, or timeout (ms) |
| `snact session <save\|load\|list\|delete>` | Manage browser sessions |
| `snact record <start\|stop\|list\|delete>` | Record command workflows |
| `snact replay <name>` | Replay a recorded workflow |
| `snact browser launch` | Launch Chrome with remote debugging |

### Global Options

```
--port <PORT>    Chrome debugging port [default: 9222]
--verbose        Enable debug logging
--output <FMT>   Output format: text or json [default: text]
```

## Architecture

```
┌──────────────────────────────────────────┐
│              AI Agent (LLM)              │
│         Claude, GPT, local models        │
└──────────────┬───────────────────────────┘
               │ CLI (stdout/stdin)
┌──────────────▼───────────────────────────┐
│            snact CLI (snact-cli)          │
│           clap subcommands               │
├──────────────────────────────────────────┤
│          snact Core (snact-core)          │
│  ┌──────┐ ┌──────┐ ┌───────┐ ┌────────┐ │
│  │ Snap │ │Action│ │Session│ │Record/ │ │
│  │      │ │      │ │       │ │Replay  │ │
│  └──┬───┘ └──┬───┘ └───┬───┘ └────┬───┘ │
│     └────┬───┘         │          │      │
│     Element Map        │          │      │
│     (@eN registry)     │          │      │
├──────────────────────────────────────────┤
│          snact CDP (snact-cdp)           │
│  WebSocket transport + ~25 CDP commands  │
└──────────────┬───────────────────────────┘
               │ WebSocket (CDP)
        ┌──────▼──────┐
        │Chrome/Chromium│
        └─────────────┘
```

### Crate Structure

```
snact/
├── crates/
│   ├── snact-cdp/     # CDP transport layer (WebSocket, Chrome discovery)
│   ├── snact-core/    # Domain logic (snap, action, session, record)
│   └── snact-cli/     # Binary entry point
```

- **snact-cdp** — Direct CDP communication over WebSocket. ~25 hand-written commands instead of generated bindings, keeping the binary small and compile times fast.
- **snact-core** — Smart snapshot extraction (DOMSnapshot + Accessibility tree merge), element filtering, action execution, session persistence, workflow recording.
- **snact-cli** — Thin CLI shell using clap derive macros.

## How Smart Snapshot Works

1. **`DOMSnapshot.captureSnapshot`** — Gets the full flattened DOM including Shadow DOM in a single CDP call
2. **`Accessibility.getFullAXTree`** — Gets semantic roles, names, and properties
3. **Merge** — Joins DOM nodes with accessibility nodes by `backendNodeId`
4. **Filter** — Keeps only interactable elements (buttons, links, inputs, etc.), excludes hidden elements
5. **Compress** — Assigns `@eN` references, outputs one element per line with only decision-relevant attributes

The element map (`@e1` → `backendNodeId`) is persisted to disk so subsequent commands can resolve references without re-snapping.

## Design Decisions

- **Hand-written CDP types** over generated libraries (`chromiumoxide` = 60K lines). snact needs ~25 commands; hand-writing keeps binary small and compile times under 30s.
- **Disk-based state** between invocations. snact connects to Chrome, does its work, and exits. Element maps, sessions, and workflows persist as JSON files.
- **`backendNodeId` as element identifier** — Stable within a page load, more reliable than CSS selectors. Selector hints are stored as backup for replay self-healing.
- **Single-threaded tokio runtime** — snact does one thing at a time. No need for multi-threaded parallelism.

## Data Storage

snact stores all state in `~/.local/share/snact/` (Linux) or `~/Library/Application Support/snact/` (macOS):

```
snact/
├── element_map.json     # Current @eN → element mappings
├── screenshot.png       # Latest screenshot
├── sessions/            # Saved browser sessions
│   └── {name}.json
├── workflows/           # Recorded workflows
│   └── {name}.json
└── recording.json       # Active recording state (if any)
```

## License

MIT
