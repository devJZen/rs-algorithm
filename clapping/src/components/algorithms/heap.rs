pub const EXPLANATION: &str = r#"
 Heap Sort  힙 정렬
 ══════════════════

 최대 힙(Max-Heap)을 이용.
 힙 꼭대기(최댓값)를 맨 끝으로
 보내고 힙 크기를 줄이며 반복.

 Time:  O(n log n)  항상
 Space: O(1)        in-place

 배열을 최대 힙으로 변환:
       8
      / \
     5   3      →  [8,5,3,1,4]
    / \
   1   4

 1단계: 루트(8)를 끝으로
       5
      / \
     4   3      →  [5,4,3,1,8]
    /
   1

 2단계: 루트(5)를 끝-1로
   ... 반복

 키 포인트:
  • 불안정 정렬 (unstable sort)
  • 최악에도 O(n log n) 보장
  • Priority Queue 구현에 핵심
  • (시각화 구현 예정)
"#;
