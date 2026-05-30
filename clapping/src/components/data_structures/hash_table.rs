pub const EXPLANATION: &str = r#"
 Hash Table  /  해시 테이블
 ──────────────────────────────

 key → hash function → index → value
 평균 O(1)로 삽입/검색/삭제.

 구조:
   key    hash(key)  bucket
   "cat"  → h() → 2 → [("cat", 🐱)]
   "dog"  → h() → 5 → [("dog", 🐶)]
   "rat"  → h() → 2 → [("rat", 🐭)]
                  ↑ 충돌! (collision)

 충돌 해결
   Chaining  : 같은 bucket에 리스트로 연결
   Open Addr : 다른 빈 slot 탐색
     Linear Probing  : +1, +2, +3 ...
     Quadratic Probing: +1², +2², +3² ...
     Double Hashing  : 두 번째 해시 사용

 연산 복잡도
            Average   Worst
   Search   O(1)      O(n)  (모든 충돌)
   Insert   O(1)      O(n)
   Delete   O(1)      O(n)

 Load Factor = 원소수 / bucket수
   > 0.7 되면 rehashing 권장
"#;
