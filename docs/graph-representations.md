# 그래프 표현

## 구조 다이어그램

![유향 그래프와 대응하는 인접 리스트](/assets/graph-representations.svg)

그림은 세 간선으로 이루어진 유향 그래프와 같은 관계를 인접 리스트로 적은 개념적 대응을 보여 줍니다. 실제 구현은 `Vec<Vec<usize>>`, 행렬, 간선 목록을 함께 유지할 수 있으며, 사용 목적과 그래프의 희소성에 따라 필요한 표현을 선택합니다.

### 그림 단계별 읽기

- 유향 그래프의 간선은 A → B, A → C, B → C 세 개이며 화살표 방향을 그대로 읽습니다.
- 출발 정점별 인접 리스트는 A → [B, C], B → [C], C → []로 대응합니다.
- A에서 B와 C로 나가는 간선이 있고 B에서 C로 나가는 간선이 있지만, 반대 방향 간선은 추가되지 않습니다.
- 따라서 리스트의 각 항목은 해당 정점에서 직접 도달할 수 있는 도착 정점만 나타내며 그래프의 방향을 보존합니다.

그래프는 정점(vertex)과 정점 사이의 간선(edge)으로 관계를 나타내는 자료구조입니다. 간선의 방향이 있으면 유향 그래프, 없으면 무향 그래프입니다. 같은 그래프도 어떤 질의를 자주 수행하는지에 따라 인접 리스트, 인접 행렬, 간선 목록으로 표현할 수 있습니다.

## 세 가지 표현 비교

- 인접 리스트: 각 정점에 연결된 이웃만 `Vec<Vec<usize>>`에 저장합니다. 공간은 `O(V + E)`이고 희소 그래프에 적합합니다. 한 정점의 이웃을 순회하는 데 `O(degree(v))`가 걸립니다.
- 인접 행렬: `matrix[u][v]`에 간선 존재 여부를 저장합니다. 공간은 `O(V^2)`이며 간선 존재를 `O(1)`에 확인할 수 있지만, 정점이 많고 간선이 적으면 낭비가 큽니다.
- 간선 목록: `(u, v)` 쌍을 모은 배열입니다. 공간은 `O(E)`이고 입력·출력이나 간선 전체 순회에 간단하지만, 특정 이웃이나 간선의 존재를 찾으려면 보통 `O(E)`를 확인해야 합니다.

아래 예제는 교육을 위해 세 표현을 동시에 유지합니다. 실제 프로그램에서는 메모리 비용을 고려해 필요한 한 가지를 선택하는 편이 좋습니다. 그래프 크기는 생성 시 고정하고, 모든 정점 번호는 `0..vertex_count` 범위여야 합니다. `add_edge`는 양쪽 정점을 먼저 검증한 다음에만 상태를 바꾸므로 잘못된 입력으로 일부 표현만 갱신되는 일을 막습니다.

유향 그래프에서는 `u -> v`만 저장합니다. 무향 그래프에서는 인접 리스트와 행렬에 양방향을 저장하지만, 간선 목록에는 논리적인 간선을 한 번만 저장합니다. 따라서 무향 간선 `(2, 1)`과 `(1, 2)`는 이 예제에서 같은 간선으로 취급되어 중복 삽입되지 않습니다.

```rust
#[derive(Debug, PartialEq, Eq)]
struct Graph {
    directed: bool,
    adjacency: Vec<Vec<usize>>,
    matrix: Vec<Vec<bool>>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new(vertex_count: usize, directed: bool) -> Self {
        Self {
            directed,
            adjacency: vec![Vec::new(); vertex_count],
            matrix: vec![vec![false; vertex_count]; vertex_count],
            edges: Vec::new(),
        }
    }

    fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    fn add_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.vertex_count() || to >= self.vertex_count() {
            return Err(format!("정점 범위 초과: {from} -> {to}"));
        }
        if self.matrix[from][to] {
            return Ok(());
        }

        self.adjacency[from].push(to);
        self.matrix[from][to] = true;
        if self.directed {
            self.edges.push((from, to));
        } else {
            if from != to {
                self.adjacency[to].push(from);
            }
            self.matrix[to][from] = true;
            self.edges.push((from.min(to), from.max(to)));
        }
        Ok(())
    }

    fn has_edge(&self, from: usize, to: usize) -> Result<bool, String> {
        if from >= self.vertex_count() || to >= self.vertex_count() {
            return Err(format!("정점 범위 초과: {from} -> {to}"));
        }
        Ok(self.matrix[from][to])
    }

    fn neighbors(&self, vertex: usize) -> Result<&[usize], String> {
        self.adjacency
            .get(vertex)
            .map(Vec::as_slice)
            .ok_or_else(|| format!("정점 범위 초과: {vertex}"))
    }
}

fn main() -> Result<(), String> {
    let mut directed = Graph::new(4, true);
    directed.add_edge(0, 1)?;
    directed.add_edge(0, 2)?;
    directed.add_edge(2, 3)?;
    println!("유향 0의 인접 리스트: {:?}", directed.neighbors(0)?);
    println!("행렬에서 0 -> 1: {}", directed.has_edge(0, 1)?);
    println!("간선 목록: {:?}", directed.edges);

    let mut undirected = Graph::new(3, false);
    undirected.add_edge(0, 1)?;
    println!("무향 1의 이웃: {:?}", undirected.neighbors(1)?);
    println!("무향 간선 목록: {:?}", undirected.edges);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_graph_keeps_direction() {
        let mut graph = Graph::new(3, true);
        graph.add_edge(0, 1).unwrap();
        assert_eq!(graph.neighbors(0).unwrap(), [1]);
        assert_eq!(graph.neighbors(1).unwrap(), []);
        assert!(graph.has_edge(0, 1).unwrap());
        assert!(!graph.has_edge(1, 0).unwrap());
        assert_eq!(graph.edges, [(0, 1)]);
    }

    #[test]
    fn undirected_graph_stores_both_directions_but_one_edge() {
        let mut graph = Graph::new(3, false);
        graph.add_edge(2, 0).unwrap();
        graph.add_edge(0, 2).unwrap();
        assert_eq!(graph.neighbors(0).unwrap(), [2]);
        assert_eq!(graph.neighbors(2).unwrap(), [0]);
        assert!(graph.has_edge(0, 2).unwrap());
        assert!(graph.has_edge(2, 0).unwrap());
        assert_eq!(graph.edges, [(0, 2)]);
    }

    #[test]
    fn undirected_self_loop_is_stored_once() {
        let mut graph = Graph::new(1, false);
        graph.add_edge(0, 0).unwrap();
        assert_eq!(graph.neighbors(0).unwrap(), [0]);
        assert_eq!(graph.edges, [(0, 0)]);
        assert!(graph.has_edge(0, 0).unwrap());
    }

    #[test]
    fn invalid_vertices_are_rejected_without_partial_update() {
        let mut graph = Graph::new(2, false);
        assert!(graph.add_edge(0, 2).is_err());
        assert_eq!(graph.edges, []);
        assert_eq!(graph.neighbors(0).unwrap(), []);
        assert!(graph.has_edge(0, 2).is_err());
        assert!(graph.neighbors(2).is_err());
    }

    #[test]
    fn empty_graph_and_duplicate_edges_are_safe() {
        let mut graph = Graph::new(0, true);
        assert_eq!(graph.vertex_count(), 0);
        assert!(graph.add_edge(0, 0).is_err());

        let mut graph = Graph::new(1, true);
        graph.add_edge(0, 0).unwrap();
        graph.add_edge(0, 0).unwrap();
        assert_eq!(graph.neighbors(0).unwrap(), [0]);
        assert_eq!(graph.edges, [(0, 0)]);
    }
}
```

## 선택 기준과 복잡도

정점 수 `V`, 간선 수 `E`, 정점 `v`의 차수 `degree(v)`를 사용하면 인접 리스트의 전체 순회는 `O(V + E)`이고, 인접 행렬의 전체 순회는 `O(V^2)`입니다. 인접 리스트는 BFS·DFS처럼 이웃을 따라가는 알고리즘의 기본 표현으로 자연스럽습니다. 반대로 간선 존재를 매우 자주 검사하고 `V`가 크지 않다면 행렬이 단순하고 빠릅니다. 간선 목록은 Kruskal 알고리즘처럼 모든 간선을 가중치 순으로 정렬하거나 파일 형식으로 주고받을 때 편리합니다.

이 예제의 행렬은 `bool`이므로 가중치가 필요하면 `Option<u32>` 또는 별도의 가중치 구조체로 바꿔야 합니다. 병렬 간선이나 같은 두 정점 사이의 여러 간선을 허용하는 다중 그래프라면 `bool` 행렬과 중복 방지 정책을 그대로 사용할 수 없습니다. 그 경우 인접 리스트에 간선 레코드를 여러 개 보관하거나 개수·가중치를 별도로 저장해야 합니다. 또한 입력 검증을 생략하고 인덱싱하면 잘못된 정점 번호에서 패닉이 발생하므로, 외부 입력을 받는 API는 이 예제처럼 `Result`를 반환하는 편이 안전합니다.

## 참고 자료

- [DSAR Chapter 15: Graph Representations](https://dsar.rantai.dev/docs/part-iii/chapter-15/)
- [Rust 표준 라이브러리: `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)

예제는 `graph_representations.rs`로 저장한 뒤 PowerShell에서 다음처럼 확인할 수 있습니다.

```powershell
rustc graph_representations.rs -o graph_representations.exe
.\graph_representations.exe
rustc --test graph_representations.rs -o graph_representations_tests.exe
.\graph_representations_tests.exe
```
