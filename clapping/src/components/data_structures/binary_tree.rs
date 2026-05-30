pub const EXPLANATION: &str = r#"
 Binary Tree  /  이진 트리
 ──────────────────────────────

 각 노드가 최대 2개의 자식을 가지는
 계층적 자료구조.

          [1]          ← root
         /   \
       [2]   [3]       ← level 1
      /   \     \
    [4]   [5]   [6]    ← level 2

 용어
   root    : 최상위 노드 (부모 없음)
   leaf    : 자식 없는 노드
   height  : root에서 leaf까지 최대 깊이
   depth   : root에서 해당 노드까지 거리

 순회 방법
   전위 (Preorder)  : root → left → right
   중위 (Inorder)   : left → root → right
   후위 (Postorder) : left → right → root

 완전 이진 트리 (Complete):
   마지막 레벨 제외 꽉 참, 왼쪽부터 채움

 포화 이진 트리 (Full):
   모든 노드가 0개 또는 2개의 자식
"#;
