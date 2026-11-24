# 너비 우선 탐색(BFS)

너비 우선 탐색은 시작 정점에서 가까운 정점부터 방문합니다. 인접 리스트 그래프에서 각 정점을 한 번씩 큐에 넣으면 시간 복잡도는 `O(V + E)`, 추가 공간은 `O(V)`입니다. 가중치가 없는 그래프에서 최단 간선 수를 구할 수 있습니다. 서로 다른 가중치가 모두 음이 아닌 경우에는 다익스트라 알고리즘을 사용할 수 있습니다.

## 동작 다이어그램

![큐를 사용해 레벨 순서로 방문하는 너비 우선 탐색](/assets/bfs.svg)

그림은 시작 정점 A를 큐에서 꺼낸 뒤 B와 C를 넣고, 같은 레벨의 정점을 먼저 처리하는 흐름을 보여 줍니다. 방문 표시는 큐에 넣을 때 적용해 중복 삽입을 막습니다.

### 그림 단계별 읽기

- 그림의 방향 간선은 A→B, A→C, B→D, B→E, C→F이며 가중치 표시는 없습니다.
- 0단계에서 A를 처리하면 큐가 `[B, C]`가 되고, B를 처리한 뒤에는 C가 먼저 남아 `[C, D, E]`가 됩니다.
- 레벨 1은 B와 C, 레벨 2는 D, E, F이므로 큐는 레벨 순서를 유지하며 완료 시 `[]`가 됩니다.
- 이 그림은 무가중치 순회를 전제로 하며 음수 가중치나 사이클을 표시하지 않습니다. 사이클이 있는 방향 그래프에서는 방문 표시로 재방문을 막습니다.


방문 표시를 큐에 넣을 때 하므로 같은 정점이 여러 번 큐에 들어가는 것을 막습니다. 그래프가 방향 그래프라면 인접 리스트를 방향에 맞게 구성해야 합니다.

```rust
use std::collections::VecDeque;

fn bfs_distances(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut distance = vec![None; graph.len()];
    if start >= graph.len() {
        return distance;
    }
    let mut queue = VecDeque::new();
    distance[start] = Some(0);
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        let next_distance = distance[node].expect("queued nodes are visited") + 1;
        for &next in &graph[node] {
            if distance[next].is_none() {
                distance[next] = Some(next_distance);
                queue.push_back(next);
            }
        }
    }
    distance
}

fn main() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    println!("{:?}", bfs_distances(&graph, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_shortest_unweighted_distances() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(bfs_distances(&graph, 0), vec![Some(0), Some(1), Some(1), Some(2)]);
    }

    #[test]
    fn leaves_unreachable_nodes_unknown() {
        let graph = vec![vec![1], vec![], vec![]];
        assert_eq!(bfs_distances(&graph, 0), vec![Some(0), Some(1), None]);
    }
}
```

테스트는 예제를 `bfs.rs`로 저장하고 `rustc --test bfs.rs -o bfs_tests.exe`로 컴파일한 뒤 PowerShell에서 `.\bfs_tests.exe`를 실행합니다. 최단 거리와 도달할 수 없는 정점을 모두 확인합니다.

## 실무 활용과 선택 기준

BFS는 무가중치 그래프에서 시작점으로부터 간선 수가 가장 적은 경로를 찾아야 할 때 적합합니다. 네트워크의 최소 홉 수, 소셜 그래프의 가까운 관계 탐색, 격자에서 장애물을 피해 이동하는 문제처럼 **모든 간선의 비용이 같다는 전제**가 있을 때 사용할 수 있습니다.

| 알고리즘 | 가중치 조건과 용도 | 시간과 추가 공간 |
| --- | --- | --- |
| BFS | 모든 간선 비용이 같고 최소 간선 수 경로를 찾을 때 | `O(V + E)`, `O(V)` |
| 0-1 BFS | 간선 가중치가 0 또는 1일 때 | `O(V + E)`, 구현에 따라 최대 `O(V + E)` |
| 다익스트라 | 음이 아닌 일반 가중치의 최단 경로를 찾을 때 | `O(V + (E + 1) log(E + 1))`, `O(V + E)` |

- 인접 리스트 BFS의 시간 복잡도는 `O(V + E)`, 추가 공간 복잡도는 `O(V)`입니다.
- BFS는 간선의 가중치를 사용하지 않으므로 가중치가 서로 다르면 최단 경로를 보장하지 않습니다. 모든 가중치가 0 또는 1이면 0-1 BFS, 음이 아닌 일반 가중치면 다익스트라, 음수 가중치가 있으면 벨만-포드처럼 문제에 맞는 방법을 선택해야 합니다.
- 방향 그래프에서는 간선 방향을 따라가며, 단절된 정점은 시작점에서 도달할 수 없습니다. 인접 리스트의 정점 번호가 범위를 벗어날 수 있는 외부 입력이라면 반드시 간선을 검증해야 합니다.

## 추가 예제

다음 예제는 무가중치 그래프에서 최단 경로의 정점 목록을 실제로 복원합니다. 잘못된 시작점이나 도착점은 `None`으로 처리하고, 범위를 벗어난 간선은 건너뛰므로 결과는 잘못된 간선을 제거한 그래프의 경로입니다. 입력 오류를 감추면 안 되는 서비스에서는 잘못된 간선을 거부하도록 별도 검증해야 합니다. 사이클이 있어도 방문 표시로 종료하며, 시작점과 도착점이 같으면 정점 하나로 된 경로를 반환합니다.

```rust
use std::collections::VecDeque;

fn shortest_path(graph: &[Vec<usize>], start: usize, goal: usize) -> Option<Vec<usize>> {
    if start >= graph.len() || goal >= graph.len() {
        return None;
    }
    let mut previous = vec![None; graph.len()];
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if node == goal {
            break;
        }
        for &next in &graph[node] {
            if next < graph.len() && !visited[next] {
                visited[next] = true;
                previous[next] = Some(node);
                queue.push_back(next);
            }
        }
    }

    if !visited[goal] {
        return None;
    }
    let mut path = Vec::new();
    let mut current = goal;
    path.push(current);
    while current != start {
        current = previous[current]?;
        path.push(current);
    }
    path.reverse();
    Some(path)
}

fn main() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    println!("{:?}", shortest_path(&graph, 0, 3));
}

#[cfg(test)]
mod tests {
    use super::shortest_path;

    #[test]
    fn restores_a_shortest_path() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert_eq!(shortest_path(&graph, 0, 3), Some(vec![0, 1, 3]));
    }

    #[test]
    fn handles_cycle_same_endpoint_and_disconnected_graph() {
        let graph = vec![vec![1], vec![0, 2], vec![1], vec![]];
        assert_eq!(shortest_path(&graph, 0, 0), Some(vec![0]));
        assert_eq!(shortest_path(&graph, 0, 3), None);
    }

    #[test]
    fn rejects_invalid_vertices_and_skips_invalid_edges() {
        let graph = vec![vec![9, 1], vec![], vec![]];
        assert_eq!(shortest_path(&graph, 0, 1), Some(vec![0, 1]));
        assert_eq!(shortest_path(&graph, 0, 4), None);
        assert_eq!(shortest_path(&graph, 4, 0), None);
    }
}
```

문서의 모든 Rust 코드 블록은 저장소의 문서 검증 설정에 포함되어 `cargo test`로 함께 검증할 수 있습니다.

참고 자료:

- [Rust 표준 라이브러리: `VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [CP-Algorithms: 너비 우선 탐색](https://cp-algorithms.com/graph/breadth-first-search.html)
