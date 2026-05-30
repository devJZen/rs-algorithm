pub const EXPLANATION: &str = r#"
 Bubble Sort  /  거품 정렬
 ─────────────────────────

 인접한 두 원소를 비교하여 더 큰
 값을 오른쪽으로 밀어냅니다.
 한 패스가 끝나면 최댓값이
 맨 오른쪽에 고정됩니다.

 복잡도 (Complexity)
   Time  Best    O(n)
         Avg     O(n²)
         Worst   O(n²)
   Space         O(1)

 특징
   Stable   : Yes  ✓
   In-place : Yes  ✓

 동작 방식
   ① 인접한 두 원소 비교 (i, j=i+1)
   ② 왼쪽이 더 크면 교환 (swap)
   ③ 끝까지 반복 → 최댓값 고정
   ④ 고정 범위를 줄이며 반복

 활용
   거의 정렬된 데이터에 유리
   실무에서는 잘 쓰지 않음
"#;

pub fn steps(data: &[i32]) -> Vec<(Vec<i32>, usize, usize)> {
    let mut arr = data.to_vec();
    let n = arr.len();
    // n을 sentinel로 사용: 마커 없음 (초기/완료 상태)
    let mut result = vec![(arr.clone(), n, n)];

    for i in 0..n {
        for j in 0..n - i - 1 {
            result.push((arr.clone(), j, j + 1));
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                result.push((arr.clone(), j, j + 1));
            }
        }
    }
    result.push((arr.clone(), n, n));
    result
}
