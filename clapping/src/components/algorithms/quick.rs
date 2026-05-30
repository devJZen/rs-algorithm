pub const EXPLANATION: &str = r#"
 Quick Sort  퀵 정렬
 ═══════════════════

 피벗(pivot)을 기준으로 작은 값은
 왼쪽, 큰 값은 오른쪽으로 분할.
 분할된 양쪽을 재귀적으로 정렬.

 Time:  O(n log n)  average
        O(n²)       worst (bad pivot)
 Space: O(log n)    call stack

 pivot = 4
 [3, 5, 8, 1, 4, 2]
           ↑
  [3,1,2] [4] [5,8]
    ↓            ↓
  [1,2,3]     [5,8]

 결과: [1,2,3,4,5,8]

 피벗 선택 전략:
  • 첫/마지막 원소 (단순, 위험)
  • 중간값 (median-of-three)
  • 랜덤 (실용적)

 키 포인트:
  • 불안정 정렬 (unstable sort)
  • 실무에서 가장 많이 쓰임
  • 캐시 효율이 좋음
  • (시각화 구현 예정)
"#;
