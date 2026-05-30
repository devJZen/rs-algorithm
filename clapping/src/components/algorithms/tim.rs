pub const EXPLANATION: &str = r#"
 Tim Sort  팀 정렬
 ═════════════════

 Merge Sort + Insertion Sort 혼합.
 Python, Java(Arrays.sort)의
 실제 기본 정렬 알고리즘.

 Time:  O(n log n)  worst
        O(n)        best (natural run)
 Space: O(n)

 1. Run 탐지: 이미 정렬된 구간을
    찾아 run으로 등록 (최소 32~64)

 2. Insertion Sort: run이 너무 짧으면
    minrun 크기까지 삽입 정렬로 확장

 3. Merge: 스택에 쌓인 run들을
    갈런트 병합(galloping)으로 합침

  [3,1,4] [1,5,9] [2,6,5]
    run1    run2    run3
       \   /
    [1,1,3,4,5,9]
           \   /
     [1,1,2,3,4,5,5,6,9]

 키 포인트:
  • 안정 정렬 (stable sort)
  • 현실 데이터에 최적화됨
  • 이미 정렬된 구간 활용
  • (시각화 구현 예정)
"#;
