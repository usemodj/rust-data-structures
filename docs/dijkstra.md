# 다익스트라 알고리즘

다익스트라 알고리즘은 하나의 시작 정점에서 모든 정점까지의 최단 거리를 구한다. 간선 가중치는 음수가 아니어야 한다. 가장 작은 현재 거리를 가진 미확정 정점을 선택하고, 그 정점에서 나가는 간선을 완화(relax)하는 과정을 반복한다. 우선순위 큐를 사용하면 일반적인 희소 그래프에서 효율적이다.

## 동작 다이어그램

![음이 아닌 가중치에서 최단 거리를 확정하는 다익스트라 알고리즘](/assets/dijkstra.svg)

그림에서는 A에서 B와 C를 완화한 뒤 더 작은 거리 1의 C를 확정하고, C를 통해 D를 거리 3으로 갱신합니다. 모든 간선 가중치가 음이 아니므로 확정한 거리는 다시 줄어들지 않습니다. 아래 Rust 코드와 테스트는 이 그림과 다른 그래프를 사용합니다.

### 그림 단계별 읽기

- 그림의 방향 간선과 가중치는 A→B `4`, A→C `1`, B→D `1`, C→D `2`입니다.
- A를 확정하면 임시 거리가 B=`4`, C=`1`이 되고, 더 작은 C를 다음에 확정합니다.
- C→D를 완화해 D를 `1 + 2 = 3`으로 갱신하며, B→D 후보 `4 + 1 = 5`보다 작아 최종 결과는 `[0, 4, 1, 3]`입니다.
- 그림의 모든 가중치는 음수가 아니고 음수 사이클도 표시되지 않습니다. 이 비음수 전제가 깨지면 확정 거리의 불변식이 성립하지 않아 다익스트라를 사용할 수 없습니다.


이 문서의 구현은 `u64` 가중치와 거리를 사용한다. `checked_add`로 거리 덧셈의 오버플로를 검사하며, 오버플로하는 후보는 최단 거리 후보로 사용하지 않는다. 따라서 결과의 `None`은 도달 불가능하거나 최단 거리를 `u64`로 표현할 수 없음을 뜻한다. 모든 간선의 정점 번호는 유효해야 한다. 외부 입력을 `u64`로 변환하기 전에는 음수 여부도 검증해야 한다. 음수 간선에는 다익스트라를 사용할 수 없고, 필요하면 벨만-포드 같은 알고리즘을 사용해야 한다.

정점 수를 `V`, 간선 수를 `E`라고 할 때 이 구현은 힙에 같은 정점의 오래된 후보를 여러 개 넣을 수 있다. 일반적인 다중 그래프에서 시간은 `O(V + (E + 1) log(E + 1))`, 추가 공간은 `O(V + E)`이며, 단순 그래프에서는 흔히 `O((V + E) log V)`로 표현한다. 꺼낸 거리가 현재 거리와 다르면 오래된 후보이므로 건너뛴다.

## 실행 가능한 Rust 예제

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(graph: &[Vec<(usize, u64)>], start: usize) -> Vec<Option<u64>> {
    let mut distance = vec![None; graph.len()];
    if start >= graph.len() {
        return distance;
    }

    let mut heap = BinaryHeap::new();
    distance[start] = Some(0);
    heap.push(Reverse((0_u64, start)));

    while let Some(Reverse((cost, vertex))) = heap.pop() {
        if distance[vertex] != Some(cost) {
            continue;
        }
        for &(next, weight) in &graph[vertex] {
            let Some(candidate) = cost.checked_add(weight) else {
                continue; // u64 오버플로 후보는 무시한다.
            };
            if distance[next].map_or(true, |old| candidate < old) {
                distance[next] = Some(candidate);
                heap.push(Reverse((candidate, next)));
            }
        }
    }
    distance
}

fn main() {
    let graph = vec![vec![(1, 4), (2, 1)], vec![(3, 1)], vec![(1, 2), (3, 5)], vec![]];
    println!("{:?}", dijkstra(&graph, 0));
}

#[cfg(test)]
mod tests {
    use super::dijkstra;

    #[test]
    fn finds_shortest_paths_and_unreachable_vertices() {
        let graph = vec![vec![(1, 4), (2, 1)], vec![(3, 1)], vec![(1, 2)], vec![], vec![]];
        assert_eq!(dijkstra(&graph, 0), vec![Some(0), Some(3), Some(1), Some(4), None]);
    }

    #[test]
    fn handles_zero_weight_edges_and_invalid_start() {
        let graph = vec![vec![(1, 0)], vec![(2, 0)], vec![]];
        assert_eq!(dijkstra(&graph, 0), vec![Some(0), Some(0), Some(0)]);
        assert_eq!(dijkstra(&graph, 9), vec![None, None, None]);
    }

    #[test]
    fn ignores_overflowing_candidate() {
        let graph = vec![vec![(1, u64::MAX)], vec![(2, 1)], vec![]];
        assert_eq!(dijkstra(&graph, 0), vec![Some(0), Some(u64::MAX), None]);
    }
}
```

PowerShell에서 파일을 `dijkstra_example.rs`로 저장한 뒤 다음처럼 실행할 수 있다. 이 코드는 안정 Rust의 `map_or`와 `checked_add`를 사용한다.

```powershell
rustc --test dijkstra_example.rs -o dijkstra_example_tests.exe; .\dijkstra_example_tests.exe
rustc dijkstra_example.rs -o dijkstra_example.exe; .\dijkstra_example.exe
```

## 실무 활용과 선택 기준

다익스트라는 지도 길찾기, 네트워크 라우팅, 게임의 이동 비용처럼 **간선 비용이 음수가 아니고 하나의 출발점에서 여러 목적지까지의 최단 경로가 필요한 상황**에 적합합니다. 경로 자체가 필요하면 각 정점의 이전 정점을 저장하고 목적지에서 시작점까지 역추적합니다. 음수 가중치가 있으면 벨만-포드가 적합하며, 모든 정점 쌍의 결과가 필요하면 플로이드-워셜을 고려합니다. 비가중치 그래프는 BFS가 `O(V + E)`로 더 단순하고, 비음수 가중치에서 목표 하나만 찾을 때는 휴리스틱을 쓸 수 있는 A*가 실용적일 수 있습니다.

| 알고리즘 | 선택 기준 | 시간과 추가 공간 |
| --- | --- | --- |
| BFS | 모든 간선 비용이 같을 때 | `O(V + E)`, `O(V)` |
| 다익스트라 | 음이 아닌 가중치, 단일 출발점 최단 경로 | `O(V + (E + 1) log(E + 1))`, `O(V + E)` |
| 벨만-포드 | 음수 가중치 또는 음수 사이클 검사가 필요할 때 | `O(V + VE)`, `O(V)` |

아래 구현처럼 같은 정점의 후보를 힙에 여러 번 넣는 경우, 다중 그래프에서 시간 복잡도는 `O(V + (E + 1) log(E + 1))`, 추가 공간은 `O(V + E)`입니다. 단순 그래프에서는 흔히 `O((V + E) log V)`로 표현합니다. 전제는 모든 가중치가 음수가 아니고 정점 인덱스가 유효하며, 거리 합이 선택한 정수 타입의 범위에 들어온다는 것입니다. 외부 입력은 알고리즘에 넘기기 전에 검증하고, 오버플로를 무한히 큰 거리로 간주하지 말고 오류로 구분해야 합니다.

## 추가 예제

다음 독립 프로그램은 음이 아닌 `u64` 가중치 그래프에서 목적지까지의 거리와 정점 경로를 함께 복원합니다. 그래프의 모든 간선을 먼저 검사하므로 도달하지 않는 부분의 잘못된 정점도 명시적 오류가 됩니다. 시작점이나 목적지가 잘못된 경우, 거리 덧셈이 `u64` 범위를 넘는 경우, 경로가 없는 경우를 각각 구분합니다. 이 예제는 목적지에 도착한 뒤에도 도달 가능한 간선을 계속 검사하므로, 목적지까지의 거리가 표현 가능해도 다른 경로에서 오버플로가 발생하면 오류를 반환합니다.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
enum ShortestPathError {
    InvalidVertex(usize),
    Overflow,
}

fn shortest_path(
    graph: &[Vec<(usize, u64)>],
    start: usize,
    goal: usize,
) -> Result<Option<(u64, Vec<usize>)>, ShortestPathError> {
    if start >= graph.len() {
        return Err(ShortestPathError::InvalidVertex(start));
    }
    if goal >= graph.len() {
        return Err(ShortestPathError::InvalidVertex(goal));
    }
    for edges in graph {
        for &(next, _) in edges {
            if next >= graph.len() {
                return Err(ShortestPathError::InvalidVertex(next));
            }
        }
    }

    let mut distance = vec![None; graph.len()];
    let mut previous = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();
    distance[start] = Some(0);
    heap.push(Reverse((0_u64, start)));

    while let Some(Reverse((cost, vertex))) = heap.pop() {
        if distance[vertex] != Some(cost) {
            continue;
        }
        for &(next, weight) in &graph[vertex] {
            let candidate = cost.checked_add(weight).ok_or(ShortestPathError::Overflow)?;
            if distance[next].map_or(true, |old| candidate < old) {
                distance[next] = Some(candidate);
                previous[next] = Some(vertex);
                heap.push(Reverse((candidate, next)));
            }
        }
    }

    let Some(cost) = distance[goal] else {
        return Ok(None);
    };
    let mut path = vec![goal];
    let mut current = goal;
    while current != start {
        current = previous[current].expect("reachable vertex has a predecessor");
        path.push(current);
    }
    path.reverse();
    Ok(Some((cost, path)))
}

fn main() {
    let graph = vec![vec![(1, 4), (2, 1)], vec![(3, 1)], vec![(1, 2), (3, 5)], vec![], vec![]];
    println!("{:?}", shortest_path(&graph, 0, 3));
}

#[cfg(test)]
mod tests {
    use super::{shortest_path, ShortestPathError};

    #[test]
    fn restores_a_nonnegative_weight_path() {
        let graph = vec![vec![(1, 4), (2, 1)], vec![(3, 1)], vec![(1, 2), (3, 5)], vec![], vec![]];
        assert_eq!(shortest_path(&graph, 0, 3), Ok(Some((4, vec![0, 2, 1, 3]))));
    }

    #[test]
    fn reports_unreachable_and_invalid_vertices() {
        let graph = vec![vec![(1, 2)], vec![], vec![]];
        assert_eq!(shortest_path(&graph, 0, 2), Ok(None));
        assert_eq!(shortest_path(&graph, 4, 0), Err(ShortestPathError::InvalidVertex(4)));
        assert_eq!(shortest_path(&[vec![(3, 1)]], 0, 0), Err(ShortestPathError::InvalidVertex(3)));
    }

    #[test]
    fn reports_distance_overflow() {
        let graph = vec![vec![(1, u64::MAX)], vec![(2, 1)], vec![]];
        assert_eq!(shortest_path(&graph, 0, 2), Err(ShortestPathError::Overflow));
    }
}
```

두 예제는 각각 독립 프로그램입니다. 프로젝트 폴더에서 `cargo test`를 실행하면 모든 Rust 코드 블록을 자동으로 추출해 실행과 테스트를 검증합니다.

## 참고 자료

- [CP-Algorithms: Dijkstra](https://cp-algorithms.com/graph/dijkstra.html)
- [Rust 표준 라이브러리: BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- [Rust 표준 라이브러리: checked_add](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)
