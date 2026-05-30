pub const EXPLANATION: &str = r#"
 Binary Search Tree  /  이진 탐색 트리
 ──────────────────────────────────────

 이진 트리 + 정렬 조건:
   왼쪽 < 현재 노드 < 오른쪽

          [8]
         /   \
       [3]   [10]
      /   \     \
    [1]   [6]   [14]
         /   \
       [4]   [7]

 연산 복잡도
            Average   Worst (편향)
   Search   O(log n)  O(n)
   Insert   O(log n)  O(n)
   Delete   O(log n)  O(n)

 편향 트리 문제
   값이 정렬된 순서로 삽입되면
   한쪽으로 길게 늘어져 O(n)으로 퇴화.

   1 → 2 → 3 → 4 → 5
   ↓
   [1] → [2] → [3] → [4] → [5]
   (linked list 와 동일해짐)

 해결책: 자가 균형 트리
   AVL Tree, Red-Black Tree
"#;
