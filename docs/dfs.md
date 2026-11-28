# 깊이 우선 탐색(DFS)

깊이 우선 탐색(Depth-First Search, DFS)은 시작 정점에서 한 경로를 가능한 한 깊게 따라간 뒤, 더 진행할 수 없으면 이전 정점으로 돌아가 다른 경로를 탐색하는 그래프 순회 방법이다. 인접 리스트를 사용하면 각 정점과 간선을 한 번씩 처리하므로 연결된 그래프에서 효율적이다. 재귀 호출 또는 명시적인 스택으로 구현할 수 있다.

## 동작 다이어그램

![스택으로 깊은 경로를 먼저 방문하는 깊이 우선 탐색](/assets/dfs.svg)

그림은 A에서 시작해 스택의 top에 있는 정점을 꺼내 A, B, D, E, C, F 순서로 깊게 방문하는 예입니다. 인접 정점을 역순으로 스택에 넣으면 원하는 인접 리스트 순서를 유지할 수 있습니다.

### 그림 단계별 읽기

- 그림의 방향 간선은 A→B, A→C, B→D, B→E, C→F이며 간선 가중치는 표시되지 않습니다.
- A를 먼저 꺼내 스택이 `[C, B]`가 되고, B를 꺼내면 `[C, E, D]`가 되어 top의 D를 먼저 방문합니다.
- 방문 순서는 A, B, D, E, C, F이며, B의 깊은 경로가 끝난 뒤 C와 그 자식 F로 되돌아갑니다.
- 이 그림에는 음수 가중치와 사이클이 없지만, 실제 DFS는 가중치 최단 경로 알고리즘이 아니며 방문 표시로 사이클 재방문을 막습니다.


## 알고리즘

- 시작 정점을 스택에 넣는다.
- 정점을 꺼낼 때 방문 여부를 확인하고, 처음 방문하는 경우 표시한 뒤 인접한 미방문 정점을 스택에 넣는다.
- 스택이 빌 때까지 반복한다.
- 방문 표시를 사용하면 사이클이 있어도 같은 정점을 반복 처리하지 않는다.

정점 수를 `V`, 간선 수를 `E`라고 할 때 이 인접 리스트 DFS의 시간 복잡도는 `O(V + E)`이다. 이 구현은 정점을 꺼낼 때 방문 표시하므로 같은 정점이 스택에 여러 번 들어갈 수 있어 추가 공간은 최악 `O(V + E)`이다. 인접 행렬을 사용하면 각 정점의 이웃을 찾는 비용 때문에 일반적으로 `O(V^2)` 시간이 든다. 인접 리스트의 모든 정점 번호는 유효해야 한다.

## 실행 가능한 Rust 예제

```rust
fn dfs(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    if start >= graph.len() {
        return Vec::new();
    }

    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(vertex) = stack.pop() {
        if visited[vertex] {
            continue;
        }
        visited[vertex] = true;
        order.push(vertex);

        // 인접 리스트의 앞쪽 항목부터 탐색하도록 역순으로 넣는다.
        for &next in graph[vertex].iter().rev() {
            if !visited[next] {
                stack.push(next);
            }
        }
    }
    order
}

fn main() {
    let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
    println!("{:?}", dfs(&graph, 0));
}

#[cfg(test)]
mod tests {
    use super::dfs;

    #[test]
    fn visits_reachable_vertices_in_depth_first_order() {
        let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        assert_eq!(dfs(&graph, 0), vec![0, 1, 3, 2]);
    }

    #[test]
    fn handles_cycle_and_isolated_start() {
        let graph = vec![vec![1], vec![0, 2], vec![1], vec![]];
        assert_eq!(dfs(&graph, 0), vec![0, 1, 2]);
        assert_eq!(dfs(&graph, 3), vec![3]);
    }

    #[test]
    fn rejects_out_of_range_start() {
        assert!(dfs(&[], 0).is_empty());
    }

    #[test]
    fn follows_documented_stack_states() {
        let graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let mut visited = vec![false; graph.len()];
        let mut stack = vec![0];
        let mut order = Vec::new();
        let mut trace = Vec::new();
        while let Some(vertex) = stack.pop() {
            if visited[vertex] {
                continue;
            }
            visited[vertex] = true;
            order.push(vertex);
            for &next in graph[vertex].iter().rev() {
                if !visited[next] {
                    stack.push(next);
                }
            }
            trace.push((vertex, stack.clone(), order.clone()));
        }
        assert_eq!(trace, vec![
            (0, vec![2, 1], vec![0]),
            (1, vec![2, 3], vec![0, 1]),
            (3, vec![2], vec![0, 1, 3]),
            (2, vec![], vec![0, 1, 3, 2]),
        ]);
        assert_eq!(dfs(&graph, 0), order);
    }

    #[test]
    fn queued_duplicates_do_not_repeat_visits() {
        let graph = vec![vec![1, 2], vec![2], vec![]];
        assert_eq!(dfs(&graph, 0), vec![0, 1, 2]);
    }
}
```

## DFS 예제의 스택과 방문 순서 따라가기

![양방향 간선 0과 1, 0과 2, 1과 3인 그래프의 DFS에서 초기 스택은 0이고, 정점 0 처리 후 2와 1, 정점 1 처리 후 2와 3, 정점 3 처리 후 2, 정점 2 처리 후 빈 스택이며 방문 순서는 0, 1, 3, 2인 단계별 변화](/assets/dfs-stack-steps.svg)

위쪽 A~F 다이어그램과 달리, 이 그림은 바로 위 Rust 예제의 `graph = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]]`와 시작점 `0`을 사용합니다. 각 간선은 양쪽 인접 리스트에 들어 있으므로 `0—1`, `0—2`, `1—3`을 양방향으로 오갈 수 있습니다. **스택의 오른쪽이 맨 위(top)**이며, 각 행은 정점을 꺼내 처리한 뒤의 상태입니다.

| 단계 | 처리 후 스택 | 방문 순서 | 설명 |
| --- | --- | --- | --- |
| 시작 | `[0]` | `[]` | 시작점을 스택에 넣지만 아직 방문 표시하지 않습니다. |
| 0 처리 | `[2, 1]` | `[0]` | `0`을 방문하고 이웃 `[1, 2]`를 역순으로 넣습니다. |
| 1 처리 | `[2, 3]` | `[0, 1]` | `1`을 방문하고 `3`을 넣습니다. `0`은 이미 방문하여 넣지 않습니다. |
| 3 처리 | `[2]` | `[0, 1, 3]` | `3`을 방문합니다. 이웃 `1`은 이미 방문했습니다. |
| 2 처리 | `[]` | `[0, 1, 3, 2]` | `2`를 방문합니다. 이웃 `0`은 이미 방문했고 스택이 비어 종료합니다. |

- `0`을 처리할 때 `2`를 먼저, `1`을 나중에 넣으므로 `1`이 먼저 나옵니다. 그다음 `3`을 깊게 탐색한 뒤 스택에 남아 있던 `2`로 돌아갑니다.
- 이 구현은 정점을 **꺼낼 때** 방문 표시합니다. 이미 방문한 이웃은 넣지 않지만, 아직 꺼내지 않은 정점은 여러 경로에서 스택에 중복으로 들어갈 수 있습니다. 나중에 중복 항목을 꺼내면 `visited[vertex]`를 확인해 건너뜁니다.
- 예를 들어 방향 그래프 `vec![vec![1, 2], vec![2], vec![]]`에서는 `0` 처리 후 `[2, 1]`이고, `1` 처리 시 아직 방문하지 않은 `2`를 다시 넣어 `[2, 2]`가 됩니다. `2`를 처음 꺼내 방문한 뒤 남은 `2`는 건너뜁니다.
- 위 그림의 그래프는 모든 인접 정점 번호가 유효합니다. 시작점이 범위를 벗어나면 빈 방문 순서를 반환하며, 인접 정점 번호가 잘못된 입력은 이 기본 예제에서 별도로 검사해야 합니다.

PowerShell에서 파일을 `dfs_example.rs`로 저장한 뒤 다음처럼 실행할 수 있다.

```powershell
rustc --test dfs_example.rs -o dfs_example_tests.exe; .\dfs_example_tests.exe
rustc dfs_example.rs -o dfs_example.exe; .\dfs_example.exe
```

## 실무 활용과 선택 기준

DFS는 방향 그래프에서 특정 정점에 도달할 수 있는지, 의존성이나 상태 공간을 끝까지 따라가며 탐색해야 하는지 확인할 때 적합합니다. 파일 시스템 순회, 의존성 그래프의 순환 확인, **백트래킹의 탐색 순서처럼 경로의 깊이가 중요한 문제**에 활용할 수 있습니다.

| 선택지 | 적합한 상황 | 시간과 추가 공간 |
| --- | --- | --- |
| DFS 인접 리스트 | 도달성, 순환 확인, 깊은 상태 공간 탐색 | `O(V + E)`, 최악 `O(V + E)` |
| BFS 인접 리스트 | 무가중치 최단 경로가 필요할 때 | `O(V + E)`, `O(V)` |
| DFS 인접 행렬 | 행렬 표현을 이미 사용하며 그래프 순회가 필요할 때 | `O(V²)`, `O(V)` |

- 인접 리스트 DFS의 시간 복잡도는 `O(V + E)`입니다. 아래 구현처럼 스택에서 꺼낼 때 방문 표시하면 같은 정점이 여러 번 쌓일 수 있으므로 추가 공간은 최악 `O(V + E)`입니다. 인접 행렬은 이웃을 확인하는 데 `O(V^2)` 시간이 걸릴 수 있습니다.
- DFS는 방문 가능성은 판정하지만 최단 경로를 보장하지 않습니다. 무가중치 그래프의 최소 간선 수 경로는 BFS를, 서로 다른 음이 아닌 가중치의 최단 경로는 다익스트라를 선택해야 합니다.
- 아래 예제는 방향 그래프의 간선 방향만 따릅니다. 사이클은 방문 표시로 처리하고, 비연결 정점은 도달하지 못한 것으로 처리합니다. 외부 입력의 잘못된 정점 번호는 무시하며, 시작점이나 도착점이 범위를 벗어나면 `false`를 반환합니다.

## 추가 예제

다음 예제는 명시적 스택을 사용하는 방향 그래프 도달 여부 판정입니다. 빈 그래프, 사이클, 비연결 그래프, 잘못된 시작점과 도착점을 테스트하며, 인접 리스트에 포함된 잘못된 간선은 안전하게 건너뜁니다.

```rust
fn reachable(graph: &[Vec<usize>], start: usize, goal: usize) -> bool {
    if start >= graph.len() || goal >= graph.len() {
        return false;
    }

    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];
    while let Some(vertex) = stack.pop() {
        if visited[vertex] {
            continue;
        }
        visited[vertex] = true;
        if vertex == goal {
            return true;
        }
        for &next in &graph[vertex] {
            if next < graph.len() && !visited[next] {
                stack.push(next);
            }
        }
    }
    false
}

fn main() {
    let graph = vec![vec![1], vec![2], vec![], vec![0]];
    println!("{}", reachable(&graph, 0, 2));
}

#[cfg(test)]
mod tests {
    use super::reachable;

    #[test]
    fn follows_direction_and_handles_a_cycle() {
        let graph = vec![vec![1], vec![2], vec![0], vec![]];
        assert!(reachable(&graph, 0, 2));
        assert!(!reachable(&graph, 2, 3));
        assert!(reachable(&graph, 1, 0));
        let directed = vec![vec![1], vec![]];
        assert!(!reachable(&directed, 1, 0));
    }

    #[test]
    fn distinguishes_disconnected_vertices_and_empty_input() {
        let graph = vec![vec![1], vec![], vec![3], vec![]];
        assert!(reachable(&graph, 0, 1));
        assert!(!reachable(&graph, 0, 3));
        assert!(!reachable(&[], 0, 0));
    }

    #[test]
    fn rejects_invalid_vertices_and_edges() {
        let graph = vec![vec![99, 1], vec![], vec![]];
        assert!(reachable(&graph, 0, 1));
        assert!(!reachable(&graph, 0, 2));
        assert!(!reachable(&graph, 3, 0));
        assert!(!reachable(&graph, 0, 3));
    }
}
```

문서의 모든 Rust 코드 블록은 저장소의 문서 검증 설정에 포함되어 `cargo test`로 함께 검증할 수 있습니다.

## 참고 자료

- [CP-Algorithms: Depth First Search](https://cp-algorithms.com/graph/depth-first-search.html)
- [Rust 표준 라이브러리: Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
