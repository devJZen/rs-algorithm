pub const EXPLANATION: &str = r#"
 Array  /  배열
 ─────────────────────────────

 같은 타입 원소를 메모리에 연속으로
 저장. 인덱스로 O(1) 시간에 접근.

 메모리 구조:
   index   0    1    2    3
         ┌────┬────┬────┬────┐
   value │ 10 │ 20 │ 30 │ 40 │
         └────┴────┴────┴────┘
          ↑
          base address

 연산 복잡도
   Access   O(1)   인덱스 직접 접근
   Search   O(n)   순차 탐색
   Insert   O(n)   중간 삽입 (shift)
   Delete   O(n)   중간 삭제 (shift)
   Append   O(1)*  끝에 추가 (amortized)

 특징
   • 캐시 지역성 우수
   • 고정 크기 (static array)
   • 동적 배열: 크기 가변 (Rust의 Vec)
   • 연속 메모리 → 예측 가능한 성능
"#;
