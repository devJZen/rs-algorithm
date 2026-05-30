pub const EXPLANATION: &str = r#"
 Heap  /  힙
 ──────────────────────────────

 완전 이진 트리 + 힙 조건:
   Max-Heap: 부모 ≥ 자식 (항상)
   Min-Heap: 부모 ≤ 자식 (항상)

 Max-Heap 예시:
          [9]          ← 항상 최댓값
         /   \
       [5]   [8]
      /   \  /
    [2]  [4][7]

 배열로 표현 (index 1부터):
   [_, 9, 5, 8, 2, 4, 7]
         ↑
   parent(i) = i/2
   left(i)   = 2i
   right(i)  = 2i+1

 연산 복잡도
   peek()    O(1)      최댓값/최솟값 확인
   insert()  O(log n)  삽입 후 heapify-up
   delete()  O(log n)  루트 제거 후 heapify-down

 활용
   • Priority Queue 구현
   • Heap Sort
   • Dijkstra 최단경로
   • 상위 K개 원소 추출
"#;
