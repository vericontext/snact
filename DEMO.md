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
# Launch with visible UI (for demos)
snact browser launch

# Headless mode
snact browser launch --headless
```

> Connects via CDP on port 9222. Run in a separate terminal (Ctrl+C to stop).

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

## 9. Claude Code + snact (AI Agent Demo)

The real power of snact: let an AI agent drive the browser autonomously.
Make sure Chrome is running (`snact browser launch` in a separate terminal).

### 9-1. Google Search (Multi-Step)

Tell Claude Code:
```
Use snact to search Google for "snact browser automation" and give me
the titles and URLs of the first 3 results. Run snact --help first.
```

What Claude Code does autonomously:
```
snact --help                              # learn the workflow
snact snap https://www.google.com         # find search elements
snact fill @e8 "snact browser automation" # fill search box
snact click @e12                          # click search button
snact snap                                # read results
→ returns structured answer
```

### 9-2. Hacker News Top Stories

```
Use snact to get the top 5 story titles from Hacker News.
```

### 9-3. GitHub Repo Info (No Login Required)

```
Use snact to check the star count and latest commit message
on the github.com/vericontext/snact repo.
```

### 9-4. Wikipedia Summary

```
Use snact to go to the Wikipedia page for "Rust (programming language)"
and list the table of contents.
```

### 9-5. Product Hunt Today

```
Use snact to find today's top 3 products on Product Hunt
with their names and taglines.
```

### Tips for Claude Code Demo

- Always have Chrome running first: `snact browser launch`
- Claude Code will run `snact --help` to learn the workflow automatically
- The `--output=json` is auto-detected when piped, so Claude Code gets structured data
- Use `--dry-run` to show safety: Claude Code can preview before acting
- The entire snap → act → snap loop typically takes 2-5 seconds

---

## Demo Flow (Recommended Order)

```
Install → Launch Chrome (separate terminal) → snap (token efficiency)
→ click/fill (snap+act loop)
→ dry-run (safety)
→ session save/load (state persistence)
→ record/replay (automation)
→ JSON pipe (agent integration)
→ Claude Code autonomous demo (the wow moment)
```

## Troubleshooting

```bash
# Check if Chrome is already running
lsof -i :9222

# If connection fails — launch Chrome manually with debugging port
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --remote-debugging-port=9222 --no-first-run --no-default-browser-check

# Change port
snact --port=9333 snap https://example.com
```
