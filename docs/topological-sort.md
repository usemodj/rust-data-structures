# 위상 정렬과 사이클 탐지

방향 그래프의 위상 정렬은 각 간선의 출발 정점이 도착 정점보다 먼저 오도록 정점을 나열합니다. 다음 설명은 사이클을 탐지하는 방법과 Rust 구현을 다룹니다.

위상 순서는 그래프가 방향 비순환 그래프(DAG)일 때만 존재합니다. 유효한 순서는 여러 개일 수 있으며, 아래 예제는 칸(Kahn) 알고리즘으로 그중 하나를 구합니다.

## 동작 다이어그램

![진입 차수가 0인 정점부터 처리하는 위상 정렬 그래프](/assets/topological-sort.svg)

간선 `0 → 1`, `0 → 2`, `1 → 3`, `2 → 3`에서는 0의 진입 차수가 0이므로 먼저 큐에 넣습니다. 0을 처리한 뒤 1과 2가 준비되고, 둘을 처리하면 3이 준비되어 `0 → 1 → 2 → 3` 같은 유효한 순서가 됩니다.

### 그림 단계별 읽기

- 그림의 전제는 정점 `0`, `1`, `2`, `3`과 간선 `0 → 1`, `0 → 2`, `1 → 3`, `2 → 3`으로 이루어진 방향 비순환 그래프입니다.
- 처음에는 정점 `0`의 진입 차수가 `0`이어서 큐에서 먼저 처리하고, 간선 두 개를 제거한 뒤 정점 `1`과 `2`가 각각 준비됩니다.
- 큐 처리 순서에서 `1`과 `2`를 차례로 처리하면 정점 `3`으로 향하는 두 간선이 모두 처리되어 `3`의 진입 차수가 `0`이 됩니다.
- 그림의 결과 `0 → 1 → 2 → 3`은 각 간선의 출발 정점이 도착 정점보다 앞서는 유효한 위상 순서입니다.

화살표는 항상 출발 정점에서 도착 정점으로 향하므로, 결과 순서에서 모든 출발 정점이 도착 정점보다 앞섭니다.

먼저 각 정점의 진입 차수를 계산하고 진입 차수가 0인 정점을 큐에 넣습니다. 큐에서 정점을 하나씩 꺼내 이웃의 차수를 줄입니다. 전체 정점 `V`개보다 적게 처리했다면 사이클이 존재합니다.

## 복잡도와 주의점

정점 수가 `V`, 간선 수가 `E`일 때 시간 복잡도는 `O(V + E)`입니다. 예제에서 생성하는 인접 리스트를 포함한 추가 공간은 `O(V + E)`입니다.

정점 번호는 `0..vertex_count` 범위여야 하며, 잘못된 번호는 예제에서 패닉을 발생시킵니다. 실제 입력은 미리 검증하세요. 중복 간선은 각각 처리되며, 독립적인 정점들의 순서는 큐의 순서에 따라 달라집니다.

## 실행 가능한 Rust 예제

### `topological_sort()` 함수 설명

`topological_sort(vertex_count, edges)`는 정점 수와 방향 간선 목록을 받아 위상 순서를 `Some(Vec<usize>)`로 반환합니다. 각 간선 `(from, to)`는 `from`을 `to`보다 먼저 처리해야 한다는 뜻입니다. 간선의 정점 번호가 `0..vertex_count` 범위를 벗어나면 이 예제는 `assert!`에서 패닉을 발생시킵니다.

함수는 먼저 인접 리스트 `graph`와 각 정점의 진입 차수 `indegree`를 만들고, 진입 차수가 0인 정점을 `ready` 큐에 넣습니다. 큐에서 정점을 꺼내 결과 `order`에 추가할 때마다 그 정점에서 나가는 간선들의 도착 정점 진입 차수를 하나씩 줄입니다. 진입 차수가 0이 된 정점은 이제 선행 조건을 모두 만족하므로 큐에 추가됩니다.

큐 처리가 끝난 뒤 `order.len()`이 `vertex_count`와 같으면 모든 정점을 처리한 것이므로 위상 순서를 반환합니다. 더 작다면 사이클 때문에 처리할 수 없는 정점이 남은 것이므로 `None`을 반환합니다. 이 구현의 시간 복잡도는 `O(V + E)`입니다.

## 예시로 보는 상태 변화 추적

다음 예제에서 간선 `(from, to)`는 `from` 정점을 처리한 뒤 `to` 정점을 처리할 수 있다는 의존 관계를 나타냅니다.

```text
let edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
println!("{:?}", topological_sort(4, &edges));
```

예제 호출에서 `vertex_count = 4`이고 `edges`는 `[(0, 1), (0, 2), (1, 3), (2, 3)]`입니다. 함수가 간선을 `graph`에 저장하면 `[[1, 2], [3], [3], []]`가 됩니다. 이후 함수 안의 변수명을 그대로 사용하면 다음과 같이 진행됩니다.

- **초기 상태:** `graph = [[1, 2], [3], [3], []]`, `indegree = [0, 1, 1, 2]`, `ready = [0]`, `order = []`
- **1회 반복 (`ready.pop_front()`가 `vertex = 0` 반환):**
  - `order`에 `vertex`를 추가하여 `order = [0]`이 됩니다.
  - `graph[0]`의 `neighbor`는 `1`, `2`입니다. 각각의 `indegree`가 `1 -> 0`이 되므로 `ready = [1, 2]`가 됩니다.
- **2회 반복 (`vertex = 1`):**
  - `order = [0, 1]`이 됩니다.
  - `graph[1]`의 `neighbor = 3`의 `indegree`는 `2 -> 1`입니다. 아직 0이 아니므로 `ready`에 추가하지 않습니다. 이때 `ready = [2]`입니다.
- **3회 반복 (`vertex = 2`):**
  - `order = [0, 1, 2]`가 됩니다.
  - `graph[2]`의 `neighbor = 3`의 `indegree`는 `1 -> 0`입니다. 따라서 `ready`에 추가하고 `ready = [3]`이 됩니다.
- **4회 반복 (`vertex = 3`):**
  - `order = [0, 1, 2, 3]`이 됩니다.
  - `graph[3]`에는 이웃이 없으므로 `ready`는 비어 있고 반복문이 끝납니다.
- **반환:** `order.len() == vertex_count`이므로 `Some(vec![0, 1, 2, 3])`이 반환됩니다. `println!("{:?}", ...)`의 출력은 다음과 같습니다.

같은 진행을 함수의 변수 상태에 맞춰 표로 나타내면 다음과 같습니다.

| 단계 | `graph` | `ready.pop_front()` 결과 (`vertex`) | 처리한 `neighbor`와 `indegree` 변화 | 처리 후 `ready` | 처리 후 `order` |
| --- | --- | --- | --- | --- | --- |
| 시작 | `[[1, 2], [3], [3], []]` | - | `indegree = [0, 1, 1, 2]` | `[0]` | `[]` |
| 1회 | 변화 없음 | `0` | `1: 1 -> 0`, `2: 1 -> 0` | `[1, 2]` | `[0]` |
| 2회 | 변화 없음 | `1` | `3: 2 -> 1` | `[2]` | `[0, 1]` |
| 3회 | 변화 없음 | `2` | `3: 1 -> 0` | `[3]` | `[0, 1, 2]` |
| 4회 | 변화 없음 | `3` | `graph[3]`에 이웃 없음 | `[]` | `[0, 1, 2, 3]` |
| 반환 | 변화 없음 | - | `order.len() == vertex_count == 4` | - | `Some(vec![0, 1, 2, 3])` |

```text
Some([0, 1, 2, 3])
```

```rust
use std::collections::VecDeque;

/// 칸(Kahn) 알고리즘으로 방향 그래프의 위상 순서를 구합니다.
///
/// `edges`의 `(from, to)`는 `from`이 `to`보다 먼저 와야 함을 뜻합니다.
/// 모든 정점이 처리되면 유효한 순서를 반환하고, 사이클 때문에 처리되지
/// 않은 정점이 남으면 `None`을 반환합니다. 정점 인덱스는 범위 안이어야 합니다.
fn topological_sort(vertex_count: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); vertex_count];
    let mut indegree = vec![0usize; vertex_count];

    for &(from, to) in edges {
        assert!(from < vertex_count && to < vertex_count);
        graph[from].push(to);
        indegree[to] += 1;
    }

    let mut ready = VecDeque::new();
    for (vertex, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            ready.push_back(vertex);
        }
    }

    let mut order = Vec::with_capacity(vertex_count);
    while let Some(vertex) = ready.pop_front() {
        order.push(vertex);
        for &neighbor in &graph[vertex] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                ready.push_back(neighbor);
            }
        }
    }

    (order.len() == vertex_count).then_some(order)
}

fn main() {
    let edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
    println!("{:?}", topological_sort(4, &edges));
}

#[cfg(test)]
mod tests {
    use super::topological_sort;

    #[test]
    fn orders_a_dag() {
        let order = topological_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]).unwrap();
        let position: Vec<_> = (0..4)
            .map(|vertex| order.iter().position(|&item| item == vertex).unwrap())
            .collect();
        assert!(position[0] < position[1]);
        assert!(position[0] < position[2]);
        assert!(position[1] < position[3]);
        assert!(position[2] < position[3]);
    }

    #[test]
    fn detects_a_cycle() {
        assert_eq!(topological_sort(3, &[(0, 1), (1, 2), (2, 0)]), None);
    }

    #[test]
    fn handles_empty_and_disconnected_graphs() {
        assert_eq!(topological_sort(0, &[]), Some(Vec::new()));
        let order = topological_sort(3, &[]).unwrap();
        assert_eq!(order.len(), 3);
    }
}
```

코드를 `topological_sort.rs`로 저장한 뒤 PowerShell에서 `rustc --test topological_sort.rs -o topological_sort_tests.exe`로 컴파일하고 `.\topological_sort_tests.exe`로 테스트합니다. `rustc topological_sort.rs -o topological_sort.exe`와 `.\topological_sort.exe`로 예제도 실행할 수 있습니다.

## 실무 활용과 선택 기준

위상 정렬은 빌드 시스템의 의존 작업, 패키지 설치 순서, 과목 선수 조건, 데이터 처리 파이프라인처럼 **선행 작업이 있는 항목을 실행 순서로 바꾸는 데 활용**합니다. 칸 알고리즘은 진입 차수가 0인 작업을 준비 큐에 넣고 처리하므로 작업을 점진적으로 실행하는 시스템에 자연스럽습니다. DFS 기반 위상 정렬은 방문 상태로 사이클을 찾고 재귀 경로에서 순서를 만들 때 유용하지만, 깊은 그래프에서는 재귀 스택을 주의해야 합니다.

### 칸 알고리즘의 활용 예

- **빌드 시스템:** 의존하는 라이브러리를 먼저 빌드합니다.
- **패키지 설치:** 필요한 의존 패키지를 먼저 설치합니다.
- **과목 수강 계획:** 선수 과목을 먼저 수강하도록 순서를 정합니다.
- **작업·데이터 파이프라인:** 선행 단계가 끝난 작업부터 처리합니다.

예를 들어 `A → C`, `B → C`라면 A와 B를 먼저 처리한 뒤 C를 처리합니다. 더 이상 준비된 작업이 없는데 처리하지 못한 작업이 남아 있다면 순환 의존성이 존재합니다.

| 알고리즘 | 선택 기준 | 시간과 공간 |
| --- | --- | --- |
| 칸 알고리즘과 인접 리스트 | 진입 차수 0인 작업을 점진적으로 처리할 때 | `O(V + E)`, `O(V + E)` |
| DFS 기반 위상 정렬 | 재귀 경로와 방문 상태를 활용할 때 | `O(V + E)`, `O(V)` |
| 우선순위 큐를 쓰는 칸 알고리즘 | 여러 준비 작업 중 우선순위를 고정할 때 | `O((V + E) log V)`, `O(V + E)` |

칸 알고리즘과 DFS 기반 알고리즘 모두 시간 복잡도는 `O(V + E)`, 인접 리스트를 포함한 공간 복잡도는 `O(V + E)`입니다. 전제는 간선이 선행 작업에서 후행 작업으로 향하고 정점 인덱스가 유효하다는 것입니다. 사이클이 있으면 유효한 순서가 없으므로 오류로 보고해야 하며, 단절된 그래프는 여러 진입 차수 0 정점을 모두 처리하면 됩니다. 작업 간 우선순위가 필요하면 큐 대신 우선순위 큐를 사용해도 되지만, 결과의 순서만 바뀌고 복잡도는 달라질 수 있습니다.

## 추가 예제

다음 독립 프로그램은 의존 작업의 순서를 반환합니다. 잘못된 정점 인덱스와 사이클을 서로 다른 명시적 오류로 처리하고, 단절된 그래프와 빈 그래프도 정상적인 입력으로 처리합니다. `edges`의 `(from, to)`는 `from` 작업이 `to` 작업보다 먼저 실행되어야 한다는 뜻입니다.

```rust
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum TopologicalError {
    InvalidVertex(usize),
    Cycle,
}

fn topological_order(
    vertex_count: usize,
    edges: &[(usize, usize)],
) -> Result<Vec<usize>, TopologicalError> {
    let mut graph = vec![Vec::new(); vertex_count];
    let mut indegree = vec![0usize; vertex_count];
    for &(from, to) in edges {
        if from >= vertex_count {
            return Err(TopologicalError::InvalidVertex(from));
        }
        if to >= vertex_count {
            return Err(TopologicalError::InvalidVertex(to));
        }
        graph[from].push(to);
        indegree[to] += 1;
    }

    let mut ready = VecDeque::new();
    for (vertex, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            ready.push_back(vertex);
        }
    }

    let mut order = Vec::with_capacity(vertex_count);
    while let Some(vertex) = ready.pop_front() {
        order.push(vertex);
        for &neighbor in &graph[vertex] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                ready.push_back(neighbor);
            }
        }
    }
    if order.len() == vertex_count {
        Ok(order)
    } else {
        Err(TopologicalError::Cycle)
    }
}

fn main() {
    let dependencies = [(0, 2), (1, 2), (2, 3)];
    println!("{:?}", topological_order(4, &dependencies));
}

#[cfg(test)]
mod tests {
    use super::{topological_order, TopologicalError};

    fn respects_edges(order: &[usize], edges: &[(usize, usize)]) -> bool {
        edges.iter().all(|&(from, to)| {
            match (order.iter().position(|&item| item == from), order.iter().position(|&item| item == to)) {
                (Some(before), Some(after)) => before < after,
                _ => false,
            }
        })
    }

    #[test]
    fn orders_dependent_and_disconnected_tasks() {
        let edges = [(0, 2), (1, 2), (2, 3)];
        let order = topological_order(5, &edges).unwrap();
        assert_eq!(order.len(), 5);
        assert!(respects_edges(&order, &edges));
    }

    #[test]
    fn reports_cycle_and_invalid_indices() {
        assert_eq!(topological_order(3, &[(0, 1), (1, 2), (2, 0)]), Err(TopologicalError::Cycle));
        assert_eq!(topological_order(2, &[(0, 2)]), Err(TopologicalError::InvalidVertex(2)));
        assert_eq!(topological_order(2, &[(3, 0)]), Err(TopologicalError::InvalidVertex(3)));
    }

    #[test]
    fn accepts_empty_graph() {
        assert_eq!(topological_order(0, &[]), Ok(Vec::new()));
        assert_eq!(topological_order(3, &[]).unwrap().len(), 3);
    }
}
```

두 예제는 각각 독립 프로그램입니다. 프로젝트 폴더에서 `cargo test`를 실행하면 모든 Rust 코드 블록을 자동으로 추출해 실행과 테스트를 검증합니다.

## 참고 자료

- [CP-Algorithms: Topological sorting](https://cp-algorithms.com/graph/topological-sort.html)
- [Rust standard library: VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [Rust Reference: Integer overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
