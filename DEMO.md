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

### Demo 1: 실제 정보 조회 (One-shot)

```bash
snact browser launch --background
claude
```

프롬프트:

```
Use snact to find the current price of the MacBook Pro 14" M4 Pro
on apple.com and tell me what storage options are available.
```

Claude Code가 하는 일:

```bash
snact snap https://www.apple.com/shop/buy-mac/macbook-pro
# # Buy MacBook Pro
# ## Model. Choose your size.
# @e35 [input:radio] ...
# ## Chip. Choose from these powerful options.
# @e40 [link] ...

snact read --focus=".as-productinfosection"
# ## Model. Choose your size.
# 14-inch — From $1699 or $141.58/mo.
# 16-inch — From $2699 or $224.91/mo.
# ## Chip. Choose from these powerful options.
# M5 Pro — 12-core CPU, 16-core GPU
# M5 Max — 16-core CPU, 40-core GPU
```

**snap 한 번에 페이지 구조를 파악하고, read 한 번에 가격/옵션을 읽어옵니다.**  
Playwright MCP라면 매 턴 전체 DOM(수만 토큰)을 보내야 할 것을 snact는 수백 토큰으로 해결.

---

### Demo 2: 더 복잡한 리서치 — 여러 사이트 비교

```
Use snact to compare the MacBook Pro 14" M4 Pro prices:
1. apple.com (official)
2. bestbuy.com
3. amazon.com
Make a comparison table with price, storage options, and availability.
```

이 태스크는 3개 사이트를 방문해서 각각 snap → read → 정보 추출.  
**턴 수가 10+ 되면서 토큰 절약이 누적됩니다:**

| | Playwright MCP | snact |
|--|--|--|
| 턴당 전송 토큰 | ~30K-80K (전체 DOM) | ~1-3K (snap/read) |
| 10턴 누적 | ~300K-800K | ~10-30K |
| **비용 차이** | **~$1-2** | **~$0.03-0.10** |

---

### Demo 3: 반복 자동화 — 한 번 가르치면 영구 무료

**Day 1 — 한 번만 가르친다:**

```
Save my current browser session as "apple".
Then record checking MacBook Pro pricing as "mbp-price".
```

Claude Code가 세션 저장 → apple.com 이동 → snap + read → 녹화 완료.

```
Done. Say "check mbp price" any time.
```

**Day 2, 3, 4 ... — 토큰 0:**

```
check mbp price
```

```bash
# Claude Code가 실행하는 전부:
snact session load apple
snact replay mbp-price
```

```
MacBook Pro 14" M4 Pro — $1,999
Storage: 512GB / 1TB / 2TB / 4TB
```

**LLM 추론: 0턴. 토큰: 0. 시간: 2초.**

---

### 왜 다른 도구로는 안 되나

| | Playwright MCP | snact |
|--|--|--|
| 1회성 간단 (3-5턴) | ~150K tokens | **~15K tokens** |
| 1회성 복잡 (10+턴) | ~500K+ tokens | **~30K tokens** |
| **반복 실행** | **매번 LLM 비용** | **토큰 0** |
| cron 자동화 | LLM API 필요 | 쉘 1줄로 가능 |
| 세션 유지 | 매번 재인증 | `session load` 1줄 |
| 페이지 이해 | LLM이 raw DOM 파싱 | **섹션 헤딩** 포함 |

---

## 3. 개별 기능 데모

위 통합 데모 후, 개별 기능을 보여줄 수 있습니다.

### snap — 구조 + 액션 요소

```bash
snact snap https://github.com/trending
# # Trending
# ## NousResearch / hermes-agent
# @e28 [link] href="/NousResearch/hermes-agent"
# ## microsoft / markitdown
# @e37 [link] href="/microsoft/markitdown"
```

섹션 헤딩으로 그루핑되어 LLM이 한눈에 구조를 파악합니다.

### read — 내용 파악

```bash
snact read https://news.ycombinator.com --focus="table.itemlist"
# 페이지 텍스트를 마크다운으로 추출 (스크린샷 불필요)
```

### click / fill — 요소 조작

```bash
snact snap https://example.com
snact click @e1

snact snap https://github.com/login
snact fill @e2 "username"
snact fill @e3 "password"
snact click @e5
```

### --focus — 범위 제한

```bash
# 전체 페이지: 1774 elements (Wikipedia)
snact snap "https://en.wikipedia.org/wiki/Rust_(programming_language)"

# 본문만: ~300 elements
snact snap "https://en.wikipedia.org/wiki/Rust_(programming_language)" --focus="#mw-content-text"
```

### --dry-run — 안전 미리보기

```bash
snact click @e1 --dry-run
# {"action":"click","args":{"ref":"@e1"},"dry_run":true}
```

### session — 세션 저장/복원

```bash
snact session save github
snact session list
snact session load github
```

### record/replay — 워크플로우 녹화

```bash
snact record start "my-workflow"
# ... 일련의 snap/click/fill 명령 ...
snact record stop

snact replay my-workflow          # 즉시 재생
snact replay my-workflow --speed=0.5  # 느린 재생 (데모용)
```

### schema — JSON Schema 인트로스펙션

```bash
snact schema          # 전체 스키마
snact schema snap     # 특정 커맨드 스키마
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
  → Demo 1: MacBook Pro 가격 조회          # snap + read 위력
  → Demo 2: 멀티사이트 비교 (optional)     # 토큰 절약 누적
  → Demo 3: record/replay 자동화           # 킬러 피처
  → 개별 기능 (필요 시)
  → snact browser stop
```

---

## Troubleshooting

```bash
snact browser status           # Chrome 상태 확인
snact browser stop             # 강제 종료

# Chrome 수동 실행 (연결 실패 시)
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --remote-debugging-port=9222 --no-first-run --no-default-browser-check

# 포트 변경
snact --port=9333 snap https://example.com
```
