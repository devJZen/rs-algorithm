pub const EXPLANATION: &str = r#"
 Selection Sort  /  선택 정렬
 ────────────────────────────

 정렬되지 않은 구간에서 최솟값을
 찾아 맨 앞 원소와 교환합니다.
 매 패스마다 하나씩 확정됩니다.

 복잡도 (Complexity)
   Time  Best    O(n²)
         Avg     O(n²)
         Worst   O(n²)
   Space         O(1)

 특징
   Stable   : No   ✗
   In-place : Yes  ✓

 동작 방식
   ① 미정렬 구간에서 최솟값 탐색 (j)
   ② 현재 위치(i)와 최솟값(min) 교환
   ③ i를 한 칸 오른쪽으로 이동
   ④ 반복 → 하나씩 앞에 고정

 특이점
   비교 횟수: 항상 n(n-1)/2
   swap 횟수: 최대 n-1 (매우 적음)
   데이터 이동이 적어야 할 때 유리
"#;

pub fn steps(data: &[i32]) -> Vec<(Vec<i32>, usize, usize)> {
    let mut arr = data.to_vec();
    let n = arr.len();
    let mut result = vec![(arr.clone(), n, n)];

    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            result.push((arr.clone(), min_idx, j));
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
            result.push((arr.clone(), i, min_idx));
        }
    }
    result.push((arr.clone(), n, n));
    result
}
