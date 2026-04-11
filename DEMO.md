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

snact의 진짜 가치는 **반복 워크플로우**에 있습니다.  
Claude Code가 처음 한 번 배우면, 이후 실행은 LLM 없이 돌아갑니다.

### Setup (one-time)

```bash
snact browser launch --background
claude   # AGENT.md를 자동으로 읽고 snact 워크플로우를 습득
```

---

### Demo A — 첫 번째 실행: Claude Code가 워크플로우를 배운다

**당신이 입력:**
```
Use snact to log in to our internal dashboard at app.example.com
with email "me@company.com" and password "••••••".
After login, navigate to the Reports tab, click "Export CSV",
then save this entire session and record it as "daily-report".
```

**Claude Code가 자율 실행:**
```bash
snact snap https://app.example.com/login
# @e1 [textbox] "Email"     id="email"
# @e2 [textbox] "Password"  id="password"
# @e3 [button]  "Sign In"

snact fill @e1 "me@company.com"
snact fill @e2 "••••••"
snact click @e3
snact wait navigation

snact session save work          # 쿠키 + localStorage 전체 저장

snact record start daily-report  # 이후 동작 녹화 시작
snact snap https://app.example.com/dashboard --focus="#main-nav"
# @e1 [link] "Overview"
# @e2 [link] "Reports"    ← 여기
# @e3 [link] "Settings"

snact click @e2
snact wait navigation
snact snap --focus="#export-controls"
# @e1 [button] "Export CSV"

snact click @e1
snact record stop
```

**Claude Code 응답:**
```
Done. Session saved as "work", workflow recorded as "daily-report".
Tomorrow just say: "run the daily report"
```

---

### Demo B — 이후 매일: 토큰 0, 시간 2초

**당신이 입력:**
```
run the daily report
```

**Claude Code가 실행:**
```bash
snact session load work          # 로그인 상태 복원 (브라우저 재시작해도)
snact replay daily-report        # 녹화된 워크플로우 그대로 실행
```

**Claude Code 응답:**
```
Done. CSV exported to Downloads folder.
```

LLM 추론: **0턴**.  
토큰: **0**.  
소요 시간: **2초**.

Playwright MCP로 같은 작업을 매일 하면 매번 전체 LLM 비용이 발생합니다.

---

### Demo C — 세션이 살아있는 한 무엇이든

로그인 후 `session save`를 해두면, 이후 모든 인증 필요 작업에 재사용됩니다.

**당신이 입력:**
```
Use snact to check my GitHub notifications and tell me
if there's anything urgent. Use the saved github session.
```

**Claude Code가 실행:**
```bash
snact session load github        # 로그인 없이 바로 인증 상태 진입
snact snap https://github.com/notifications --focus=".notifications-list"
# @e1 [link] "PR #142 needs your review — vericontext/snact"
# @e2 [link] "You were mentioned in Issue #89"
# @e3 [link] "CI failed on branch main"
```

**Claude Code 응답:**
```
3 notifications, 1 urgent:
- CI failed on main branch (vericontext/snact) — needs immediate attention
```

GitHub에 로그인하는 LLM 비용: **0**.  
알림 읽는 snap 1회: **~30 tokens**.

---

### Demo D — 반복 폼 제출: 사람이 할 일을 에이전트에게

**당신이 입력:**
```
Use snact to submit expense reports for the following 3 items
to expenses.company.com. Use the saved work session.
- Airfare $450, category: Travel, date: 2026-04-01
- Hotel $320, category: Accommodation, date: 2026-04-01
- Dinner $85, category: Meals, date: 2026-04-02
```

**Claude Code가 실행 (3회 반복):**
```bash
snact session load work
snact snap https://expenses.company.com/new --focus="#expense-form"
# @e1 [textbox]  "Amount"
# @e2 [combobox] "Category"
# @e3 [textbox]  "Date"
# @e4 [button]   "Submit"

snact fill @e1 "450"
snact select @e2 "Travel"
snact fill @e3 "2026-04-01"
snact click @e4 --dry-run    # 제출 전 미리보기
# {"status":"dry_run","action":"click","args":{"ref":"@e4"}}
snact click @e4              # 확인 후 실제 제출
snact wait navigation
# (반복 × 3)
```

사람이 폼 3개를 수동으로 채우면 10분.  
snact로 에이전트가 처리: **30초, 확인 1번**.

---

### Demo E — 스케줄 자동화: LLM 없는 cron

Claude Code에게 한 번 시키면, 이후는 cron으로 돌아갑니다.

**당신이 입력:**
```
Use snact to record a workflow that checks our status page
at status.example.com and screenshots it. Save it as "status-check".
Then write a shell script that runs it every hour without needing me.
```

**Claude Code가 생성하는 스크립트:**
```bash
#!/bin/bash
# status-check.sh — runs without LLM

snact browser launch --background
snact session load monitor
snact replay status-check
snact screenshot --file="/logs/status-$(date +%H%M).png"
snact browser stop
```

```bash
# crontab -e
0 * * * * /usr/local/bin/status-check.sh
```

이제 매 시간 자동 실행. Claude Code 불필요. 토큰 불필요. API 키 불필요.

---

### 왜 이게 dramatic한가

| | Playwright MCP | snact |
|--|--|--|
| 첫 실행 | LLM 추론 필요 | LLM 추론 필요 |
| **10번째 실행** | **매번 동일 LLM 비용** | **토큰 0** |
| **100번째 실행** | **100× LLM 비용** | **토큰 0** |
| 로그인 만료 시 | 재로그인 전체 flow | `session load` 1줄 |
| cron/자동화 | LLM API 필요 | 쉘 스크립트만으로 가능 |
| 팀원과 공유 | 불가 (세션 개인) | `session` + `record` 파일 공유 |

**snact의 핵심 가치: 처음 한 번만 LLM을 쓰고, 이후는 영구히 무료로 반복.**

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
