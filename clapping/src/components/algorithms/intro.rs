// 인트로 / 공통 비교표 (나중에 사용)
pub const COMPARISON_TABLE: &str = r#"
 Sorting Algorithms 비교
 ══════════════════════════════════════════════

 Algorithm      Best      Avg       Worst   Space  Stable
 ─────────────────────────────────────────────────────────
 Bubble         O(n)      O(n²)     O(n²)   O(1)    ✓
 Selection      O(n²)     O(n²)     O(n²)   O(1)    ✗
 Insertion      O(n)      O(n²)     O(n²)   O(1)    ✓
 Shell          O(n logn) O(n log²n) O(n²)  O(1)    ✗
 Merge          O(n logn) O(n logn) O(n logn) O(n)  ✓
 Quick          O(n logn) O(n logn) O(n²)   O(logn) ✗
 Heap           O(n logn) O(n logn) O(n logn) O(1)  ✗
 Tim            O(n)      O(n logn) O(n logn) O(n)  ✓
"#;
