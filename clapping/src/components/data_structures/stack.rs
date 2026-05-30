pub const EXPLANATION: &str = r#"
 Stack  /  스택
 ──────────────────────────────

 LIFO — Last In, First Out
 마지막에 넣은 것이 먼저 나온다.

        push(40)
           ↓
         ┌────┐  ← top
         │ 40 │
         ├────┤
         │ 30 │
         ├────┤
         │ 20 │
         ├────┤
         │ 10 │  ← bottom
         └────┘
           ↓
        pop() → 40

 연산 복잡도
   push(x)   O(1)  맨 위에 추가
   pop()     O(1)  맨 위 제거 & 반환
   peek()    O(1)  맨 위 확인 (제거 없음)
   isEmpty   O(1)

 활용
   • 함수 콜 스택 (재귀)
   • 괄호 유효성 검사
   • DFS (깊이 우선 탐색)
   • 브라우저 뒤로가기
   • Undo / Redo
"#;
