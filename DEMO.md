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

### 시나리오: 매일 타임시트 제출이 귀찮다

회사 타임시트 시스템에 매일 같은 폼을 채워야 한다고 가정합니다.  
브라우저에서 타임시트 사이트에 직접 로그인한 뒤:

**Day 1 — 한 번만 가르친다:**

```
Use snact to save my current browser session as "work".
Then fill in today's timesheet on the current page:
8 hours on "Platform", category "Development".
After submitting, record the whole thing as "timesheet".
```

Claude Code가 폼을 찾아 채우고, 제출하고, 전체 과정을 녹화합니다.

```
Done. Tomorrow just say "submit timesheet".
```

---

**Day 2, 3, 4 ... — 두 글자면 끝:**

```
submit timesheet
```

```bash
# Claude Code가 실행하는 전부:
snact session load work
snact replay timesheet
```

```
Done. 8h submitted for today.
```

**LLM 추론: 0턴. 토큰: 0. 시간: 3초.**

매일 타임시트 사이트에 들어가서 폼 클릭하던 2분이 사라집니다.  
한 달이면 **40분 절약, LLM 비용 0**.

---

### 더 나아가: 완전 자동화

```
Write a cron job that submits my timesheet automatically at 6pm every weekday.
```

```bash
# crontab -e
0 18 * * 1-5 snact session load work && snact replay timesheet
```

이제 타임시트를 **영원히 신경 쓰지 않아도 됩니다.**  
Claude Code도, API 키도, 토큰도 필요 없습니다.

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
