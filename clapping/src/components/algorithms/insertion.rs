pub const EXPLANATION: &str = r#"
 Insertion Sort  /  삽입 정렬
 ─────────────────────────────

 이미 정렬된 부분에 새 원소를
 적절한 위치에 끼워 넣습니다.
 카드 패를 손에 쥐는 방식과 동일.

 복잡도 (Complexity)
   Time  Best    O(n)
         Avg     O(n²)
         Worst   O(n²)
   Space         O(1)

 특징
   Stable   : Yes  ✓
   In-place : Yes  ✓

 동작 방식
   ① i번째 원소를 key로 선택
   ② 정렬된 구간(0~i-1)을 오른쪽에서
      key보다 큰 원소를 한 칸씩 shift
   ③ 빈 자리에 key 삽입
   ④ i를 증가시키며 반복

 활용
   거의 정렬된 데이터에 매우 강함
   Tim Sort 내부 알고리즘으로 사용
   작은 배열에서 Quick Sort보다 빠름
"#;

pub fn steps(data: &[i32]) -> Vec<(Vec<i32>, usize, usize)> {
    let mut arr = data.to_vec();
    let n = arr.len();
    let mut result = vec![(arr.clone(), n, n)];

    for i in 1..n {
        let mut j = i;
        while j > 0 {
            result.push((arr.clone(), j - 1, j));
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                result.push((arr.clone(), j - 1, j));
                j -= 1;
            } else {
                break;
            }
        }
    }
    result.push((arr.clone(), n, n));
    result
}
