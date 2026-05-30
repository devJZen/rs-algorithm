pub const EXPLANATION: &str = r#"
 Graph  /  그래프
 ──────────────────────────────

 정점(Vertex)과 간선(Edge)의 집합.
 G = (V, E)

 무방향 그래프:           방향 그래프:
   A ─── B                A ──→ B
   │  ╲  │                │      │
   │   ╲ │                ↓      ↓
   C ─── D                C ←── D

 표현 방법
   인접 행렬 (Adjacency Matrix):
     공간 O(V²), 간선 확인 O(1)
     ┌         ┐
     │ 0 1 1 0 │
     │ 1 0 0 1 │
     │ 1 0 0 1 │
     │ 0 1 1 0 │
     └         ┘

   인접 리스트 (Adjacency List):
     공간 O(V+E), 간선 확인 O(degree)
     A: [B, C]
     B: [A, D]

 탐색 알고리즘
   BFS (너비 우선): Queue 사용, O(V+E)
   DFS (깊이 우선): Stack/재귀, O(V+E)

 활용
   최단경로, 네트워크, SNS, 지도
"#;
