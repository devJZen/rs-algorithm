pub const EXPLANATION: &str = r#"
 Linked List  /  연결 리스트
 ────────────────────────────

 각 원소(노드)가 값과 다음 노드의
 포인터를 함께 보유. 불연속 메모리.

 단방향 (Singly):
   Head
    ↓
   [10|→] → [20|→] → [30|→] → null

 양방향 (Doubly):
   [null|10|→] ↔ [←|20|→] ↔ [←|30|null]

 연산 복잡도
   Access   O(n)   순서대로 따라가야 함
   Search   O(n)
   Insert   O(1)   포인터만 변경 (위치 알 때)
   Delete   O(1)   포인터만 변경 (위치 알 때)

 vs Array
   삽입/삭제 빠름 (shift 불필요)
   임의 접근 느림 (인덱스 불가)
   메모리: 포인터 오버헤드 있음

 Rust에서는 소유권 때문에
 구현이 까다롭다 (Box, Rc, RefCell)
"#;
