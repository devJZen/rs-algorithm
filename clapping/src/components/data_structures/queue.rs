pub const EXPLANATION: &str = r#"
 Queue  /  큐
 ──────────────────────────────

 FIFO — First In, First Out
 먼저 넣은 것이 먼저 나온다.

   enqueue(40)
        ↓
   rear              front
    ┌────┬────┬────┬────┐
    │ 40 │ 30 │ 20 │ 10 │
    └────┴────┴────┴────┘
                         ↓
                    dequeue() → 10

 연산 복잡도
   enqueue(x)  O(1)  뒤에 추가
   dequeue()   O(1)  앞에서 제거 & 반환
   peek()      O(1)  앞 확인 (제거 없음)
   isEmpty     O(1)

 종류
   • 일반 큐 (Linear Queue)
   • 원형 큐 (Circular Queue)
   • 덱 (Deque, 양쪽 삽입/삭제)
   • 우선순위 큐 (Priority Queue)

 활용
   • BFS (너비 우선 탐색)
   • 프린터 스풀링
   • CPU 스케줄링
   • 네트워크 패킷 처리
"#;
