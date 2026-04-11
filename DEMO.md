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

snact의 핵심: **처음 한 번만 LLM, 이후는 영구히 무료**.

```bash
snact browser launch --background
claude
```

---

### 시나리오: 매일 아침 GitHub PR 리뷰 현황 파악

브라우저에서 GitHub에 로그인한 뒤:

**Day 1 — 한 번만 가르친다:**

```
Use snact to save my current GitHub session,
then record checking my PR review queue as "pr-check".
```

Claude Code가 현재 로그인 상태 저장 → `github.com/pulls` 이동 → 목록 snap → 녹화 완료.

```
Done. Say "check PRs" any time.
```

---

**Day 2, 3 ... — 한 마디면 끝:**

```
check PRs
```

```bash
# Claude Code가 실행하는 전부:
snact session load github
snact replay pr-check
```

```
2 PRs need your review:
- #142 "Add MCP server" — requested 3h ago
- #138 "Fix CDP connection drop" — requested yesterday
```

**LLM 추론: 0턴. 토큰: 0. 시간: 2초.**

---

### 더 나아가: 아침마다 자동으로

```
Write a cron script that runs pr-check every morning at 9am
and prints the result to my terminal.
```

```bash
# crontab -e
0 9 * * 1-5 snact session load github && snact replay pr-check
```

GitHub 탭을 열지 않아도 PR 현황이 터미널에 표시됩니다.  
**Claude Code도, API 키도, 토큰도 필요 없습니다.**

---

### 왜 다른 도구로는 안 되나

| | Playwright MCP | snact |
|--|--|--|
| 첫 실행 | LLM 비용 | LLM 비용 |
| **매일 실행** | **매번 LLM 비용** | **토큰 0** |
| cron 자동화 | LLM API 필요 | 쉘 1줄로 가능 |
| 로그인 유지 | 매번 재인증 | `session load` 1줄 |

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
