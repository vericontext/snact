# snact Demo Guide

## 0. Install

```bash
mkdir -p ~/dev/personal/playground/snact-demo
cd ~/dev/personal/playground/snact-demo

# Private repo — GitHub token 필요
curl -fsSL -H "Authorization: token $(gh auth token)" \
  https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash

snact --version
```

> repo가 public이면 token 없이 `curl -fsSL https://raw.githubusercontent.com/vericontext/snact/main/install.sh | bash`

---

## 1. Chrome 실행

```bash
# 브라우저 UI 보이게 실행 (데모용)
snact browser launch

# headless 모드
snact browser launch --headless
```

> 기본 포트 9222로 CDP 연결됨. 별도 터미널에서 실행 (Ctrl+C로 종료).

---

## 2. Smart Snapshot - 토큰 효율 확인

```bash
# 페이지 스냅샷
snact snap https://example.com

# 출력 예시:
# @e1 [link] "More information..."
# @e2 [heading] "Example Domain"
# --- 2 elements (vs Playwright MCP ~114K tokens)

# 특정 영역만 스냅
snact snap https://example.com --focus="body > div"

# JSON 출력 (AI 에이전트 모드)
snact snap https://example.com --output=json
```

---

## 3. Snap + Act 루프

### 3-1. 기본 클릭

```bash
snact snap https://example.com
# @e1 [link] "More information..." 확인 후:
snact click @e1
```

### 3-2. 폼 입력 (GitHub 로그인 페이지)

```bash
snact snap https://github.com/login

# 출력에서 username, password 필드의 @eN 번호 확인 후:
snact fill @eN "your-username"
snact fill @eM "your-password"

# Sign In 버튼 클릭
snact click @eK

# 스냅으로 결과 확인
snact snap https://github.com
```

### 3-3. 검색 플로우

```bash
snact snap https://www.google.com

# 검색 입력 필드 @eN 확인 후:
snact fill @eN "snact browser automation"
snact click @eM   # 검색 버튼

# 결과 페이지 스냅
snact snap
```

---

## 4. Dry-Run (안전한 사전 확인)

```bash
# 실제 실행 없이 어떤 액션인지 확인
snact click @e1 --dry-run
# {"action":"click","args":{"ref":"@e1"},"dry_run":true}

snact fill @e3 "test" --dry-run
# {"action":"fill","args":{"ref":"@e3","value":"test"},"dry_run":true}
```

---

## 5. Session 관리

```bash
# 로그인 후 세션 저장
snact session save github

# 세션 목록 확인
snact session list

# 브라우저 재시작 후 세션 복원
snact session load github

# 로그인 상태 유지 확인
snact snap https://github.com
```

---

## 6. Record & Replay

```bash
# 녹화 시작
snact record start "search-demo"

# 일련의 명령 실행
snact snap https://www.google.com
snact fill @eN "snact"
snact click @eM

# 녹화 종료
snact record stop

# 녹화 목록 확인
snact record list

# 리플레이
snact replay search-demo

# 느린 속도로 리플레이 (데모 시연용)
snact replay search-demo --speed=0.5
```

---

## 7. AI 에이전트 연동 (파이프 모드)

```bash
# 파이프로 연결하면 자동으로 JSON 출력
snact snap https://example.com | jq '.elements[] | .ref + " " + .role'

# 스크립트에서 활용
ELEMENTS=$(snact snap https://example.com --output=json)
echo "$ELEMENTS" | jq '.count'
```

---

## 8. Screenshot

```bash
snact screenshot --path=./demo-capture.png
# 현재 페이지 캡처 저장
```

---

## Demo Flow (권장 순서)

```
설치 → Chrome 실행 (별도 터미널) → snap (토큰 효율 강조)
→ click/fill (snap+act 루프)
→ dry-run (안전성)
→ session save/load (상태 유지)
→ record/replay (자동화)
→ JSON 파이프 (에이전트 연동)
```

## Troubleshooting

```bash
# Chrome이 이미 실행 중인 경우
lsof -i :9222

# 연결 안 될 때 - Chrome을 디버깅 포트와 함께 직접 실행
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --remote-debugging-port=9222 --no-first-run --no-default-browser-check

# 포트 변경
snact --port=9333 snap https://example.com
```
