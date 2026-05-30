pub const EXPLANATION: &str = r#"
 Shell Sort  셸 정렬
 ═══════════════════

 삽입 정렬의 개선판.
 간격(gap)을 두고 삽입 정렬하며
 gap을 줄여가면서 최종 정렬.

 Time:  O(n log² n)  Ciura gap 기준
        O(n²)        worst
 Space: O(1)         in-place

 gap = 4:
 [5, 3, 8, 1, 4, 2, 7, 6]
  ↑           ↑
  비교: 5 vs 4  →  swap

 gap = 2:
 [4, 3, 5, 1, 8, 2, 7, 6]
  ↑     ↑
  비교: 4 vs 5  →  keep

 gap = 1:  (= 삽입 정렬, 거의 정렬됨)

 Ciura gap sequence:
  1, 4, 10, 23, 57, 132, 301, 701

 키 포인트:
  • 불안정 정렬 (unstable sort)
  • 삽입 정렬보다 훨씬 빠름
  • gap 수열 선택이 성능에 영향
  • (시각화 구현 예정)
"#;
