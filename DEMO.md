# snact Demo Guide

## 0. Install

```bash
mkdir -p ~/dev/personal/playground/snact-demo
cd ~/dev/personal/playground/snact-demo

# Private repo — requires GitHub token
curl -fsSL -H "Authorization: token $(gh auth token)" \
  https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash

snact --version
```

> If repo is public: `curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash`

---

## 1. Launch Chrome

```bash
# Launch in background — terminal stays free (recommended)
snact browser launch --background

# Launch with visible UI — blocks terminal (Ctrl+C to stop)
snact browser launch

# Headless mode
snact browser launch --headless --background

# Check status / stop
snact browser status
snact browser stop
```

> Connects via CDP on port 9222.

---

## 2. Smart Snapshot — Token Efficiency

```bash
# Snapshot a page
snact snap https://example.com

# Example output:
# @e1 [link] "More information..."
# @e2 [heading] "Example Domain"
# --- 2 elements (vs Playwright MCP ~114K tokens)

# Focus on a specific area
snact snap https://example.com --focus="body > div"

# JSON output (AI agent mode)
snact snap https://example.com --output=json
```

---

## 3. Snap + Act Loop

### 3-1. Basic Click

```bash
snact snap https://example.com
# See @e1 [link] "More information..." then:
snact click @e1
```

### 3-2. Form Input (GitHub Login Page)

```bash
snact snap https://github.com/login

# Find username/password field @eN numbers in output:
snact fill @eN "your-username"
snact fill @eM "your-password"

# Click Sign In button
snact click @eK

# Snap to verify result
snact snap https://github.com
```

### 3-3. Search Flow

```bash
snact snap https://www.google.com

# Find search input @eN then:
snact fill @eN "snact browser automation"
snact click @eM   # search button

# Snap the results page
snact snap
```

---

## 4. Dry-Run (Safe Preview)

```bash
# Preview actions without executing
snact click @e1 --dry-run
# {"action":"click","args":{"ref":"@e1"},"dry_run":true}

snact fill @e3 "test" --dry-run
# {"action":"fill","args":{"ref":"@e3","value":"test"},"dry_run":true}
```

---

## 5. Session Management

```bash
# Save session after login
snact session save github

# List saved sessions
snact session list

# Restore session after browser restart
snact session load github

# Verify login state persisted
snact snap https://github.com
```

---

## 6. Record & Replay

```bash
# Start recording
snact record start "search-demo"

# Execute a sequence of commands
snact snap https://www.google.com
snact fill @eN "snact"
snact click @eM

# Stop recording
snact record stop

# List recordings
snact record list

# Replay the workflow
snact replay search-demo

# Slow replay (for demo presentation)
snact replay search-demo --speed=0.5
```

---

## 7. AI Agent Integration (Pipe Mode)

```bash
# Piped output auto-detects JSON
snact snap https://example.com | jq '.elements[] | .ref + " " + .role'

# Use in scripts
ELEMENTS=$(snact snap https://example.com --output=json)
echo "$ELEMENTS" | jq '.count'
```

---

## 8. Screenshot

```bash
snact screenshot --file=./demo-capture.png
# Captures the current page
```

---

## 9. Claude Code + snact — Killer Demo

This is the real payoff: hand Claude Code a natural language task, and it autonomously drives the browser — snap, act, snap, answer — with no human intervention.

### Setup (one-time)

```bash
# 1. Launch Chrome in background (stays up, doesn't block terminal)
snact browser launch --background

# 2. Open a new Claude Code session in the same directory
claude
```

Claude Code reads `AGENT.md` automatically and knows the full snact workflow.

---

### Demo A — Live Web Research

**You type:**
```
Use snact to find the current price of the MacBook Pro 14" M4 Pro on apple.com
and tell me what storage options are available.
```

**Claude Code runs autonomously:**
```
snact snap https://www.apple.com/shop/buy-mac/macbook-pro/14-inch
# @e3  [link]   "14-inch MacBook Pro"
# @e12 [button] "Buy" price="$1,999"
# @e15 [select] "Storage" id="storage-select"
# ...

snact snap --focus="#configure-form"
# @e1 [option] "512GB SSD"
# @e2 [option] "1TB SSD"
# @e3 [option] "2TB SSD"
```

**Claude Code replies:**
```
MacBook Pro 14" M4 Pro starts at $1,999.
Storage options: 512GB SSD / 1TB SSD / 2TB SSD
```

Total LLM calls: **2 snaps, 0 human steps**.

---

### Demo B — Multi-Step Form Automation

**You type:**
```
Use snact to go to github.com/login, fill in username "demo-user"
and password "demo-pass", but use --dry-run so nothing actually happens.
Show me what would execute.
```

**Claude Code runs:**
```
snact snap https://github.com/login
# @e1 [textbox] "Username or email address" id="login_field"
# @e2 [textbox] "Password"                  id="password"
# @e3 [button]  "Sign in"

snact fill @e1 "demo-user" --dry-run
# {"status":"dry_run","action":"fill","args":{"ref":"@e1","value":"demo-user"}}

snact fill @e2 "demo-pass" --dry-run
# {"status":"dry_run","action":"fill","args":{"ref":"@e2","value":"demo-pass"}}

snact click @e3 --dry-run
# {"status":"dry_run","action":"click","args":{"ref":"@e3"}}
```

**Why this matters:** Agent previewed a 3-step form workflow with exact refs, zero execution risk, full auditability.

---

### Demo C — Scrape + Summarize

**You type:**
```
Use snact to get the top 5 stories from Hacker News right now
with their point counts. Give me a one-line summary of each.
```

**Claude Code runs:**
```
snact snap https://news.ycombinator.com --output ndjson | head -20
# {"ref":"@e1","role":"link","name":"Show HN: I built X ...","tag":"a"}
# {"ref":"@e2","role":"link","name":"Ask HN: Why does Y ...","tag":"a"}
# {"ref":"@e3","role":"link","name":"Postgres just got ...","tag":"a"}
# ...
```

**Why NDJSON matters:** Claude Code pipes the stream through `head` — reads only the first 20 elements instead of loading the entire page, saving tokens on element-heavy pages.

---

### Demo D — Session + Replay (Zero LLM Cost)

**You type:**
```
Use snact to log in to GitHub with my credentials, save the session,
then record a workflow that visits my notifications page.
```

**Claude Code runs:**
```
snact snap https://github.com/login
snact fill @e1 "your-username"
snact fill @e2 "your-password"
snact click @e3
snact wait navigation
snact session save github-auth

snact record start check-notifications
snact snap https://github.com/notifications
snact record stop
```

**Next time, you just say:**
```
Check my GitHub notifications using the saved workflow.
```

**Claude Code runs:**
```
snact session load github-auth
snact replay check-notifications
```

Zero LLM reasoning. Zero tokens on the replay.

---

### Demo E — Schema-Guided Usage

**You type:**
```
What exact JSON does snact snap return? Show me the output schema.
```

**Claude Code runs:**
```
snact schema snap
```

**Returns:**
```json
{
  "description": "Extract interactable elements ...",
  "output": {
    "json": {
      "type": "object",
      "properties": {
        "count": { "type": "integer" },
        "elements": {
          "additionalProperties": {
            "properties": {
              "role": { "type": "string" },
              "name": { "type": "string" },
              "tag":  { "type": "string" },
              ...
            }
          }
        }
      }
    }
  }
}
```

Agent now knows the exact output shape — no hallucinated field names, no trial-and-error.

---

### Why This Works

| Problem with other tools | snact solution |
|--------------------------|----------------|
| Playwright MCP: ~114K tokens per session | snap output: 5–50 tokens per element |
| Agents guess parameter names | `snact schema` returns exact JSON Schema |
| Browser state lost between calls | `snact session save/load` persists everything |
| Repeated workflows burn LLM budget | `snact record/replay` = zero tokens |
| Agent can't tell if action is safe | `--dry-run` on every mutation |
| Web content could inject instructions | Prompt injection detection built in |

---

### Cleanup

```bash
snact browser stop
```

---

## Demo Flow (Recommended Order)

```
Install
  → snact browser launch --background    # Chrome up, terminal free
  → snact snap (token efficiency demo)   # vs Playwright ~114K
  → snact click/fill (snap+act loop)     # @eN refs
  → snact --dry-run (safety demo)        # preview before act
  → snact session save/load              # state persistence
  → snact record/replay                  # zero-LLM replay
  → snact schema                         # introspection
  → Claude Code natural language demo    # ← the wow moment
  → snact browser stop                   # clean up
```

---

## Troubleshooting

```bash
# Check Chrome status
snact browser status

# Force stop if stuck
snact browser stop

# If connection still fails — launch Chrome manually
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --remote-debugging-port=9222 --no-first-run --no-default-browser-check

# Change port
snact --port=9333 snap https://example.com
```
