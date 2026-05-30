# clapping — Algorithm Visualizer

정렬 알고리즘을 터미널에서 ASCII 아트로 단계별로 학습하는 TUI 앱.

## 실행

```bash
# Rust가 없으면 먼저 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo run
```

## 화면 흐름

```
cargo run
    │
    ▼
┌──────────────────────────────────┐
│  algorithm  /▌                   │  ← /를 눌러 검색
├──────────────────────────────────┤
│ ▶ bubble      Bubble Sort  O(n²) │
│   selection   Selection Sort ... │
│   ...                            │
└──────────────────────────────────┘
    │  Enter
    ▼
┌──────────────────────────────────┐
│  algorithm  bubble  ✓            │
│  size  10▌  (2~30)               │  ← 숫자 입력 후 Enter
└──────────────────────────────────┘
    │  Enter
    ▼
┌──────────────────────────────────────────────────────┐
│  algoviz  │  Bubble Sort  │  step 3/61  │  ▶ playing │
├──────────────────────┬───────────────────────────────┤
│  Explanation         │  ┌──┬──┬──┬──┬──┐            │
│                      │  │ 1│ 4│ 8│ 5│ 2│            │
│  복잡도, 동작 방식   │  └──┴──┴──┴──┴──┘            │
│  ...                 │       ↑  ↑                    │
│                      │       i  j                    │
│                      │  8 > 5  →  SWAP!              │
└──────────────────────┴───────────────────────────────┘
```

## 선택 화면 조작

| 키 | 동작 |
|----|------|
| `/` | 검색 시작 (타이핑으로 필터) |
| `↑` / `↓` | 목록 이동 |
| `Enter` | 선택 |
| `Esc` | 검색 취소 |
| `q` | 종료 |

## 시각화 화면 조작

| 키 | 동작 |
|----|------|
| `Space` | 자동 재생 / 일시정지 |
| `→` | 한 단계 앞으로 |
| `←` | 한 단계 뒤로 |
| `r` | 처음으로 리셋 |
| `q` / `Esc` | 선택 화면으로 돌아가기 |

## 빠른 실행 (CLI 플래그로 선택 화면 건너뛰기)

```bash
cargo run -- -a bubble -n 10
cargo run -- -a selection -n 20
```

## 알고리즘 목록

| 키워드 | 이름 | 시각화 |
|--------|------|--------|
| `bubble` | Bubble Sort | ✓ |
| `selection` | Selection Sort | ✓ |
| `insertion` | Insertion Sort | ✓ |
| `merge` | Merge Sort | 예정 |
| `quick` | Quick Sort | 예정 |
| `heap` | Heap Sort | 예정 |
| `shell` | Shell Sort | 예정 |
| `tim` | Tim Sort | 예정 |
