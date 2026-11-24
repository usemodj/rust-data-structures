# 벨만–포드 알고리즘

벨만–포드(Bellman–Ford)는 하나의 시작 정점에서 모든 정점까지의 최단 거리를 구한다. 음의 가중치 간선을 허용하며, 시작 정점에서 도달할 수 있는 음의 사이클이 있으면 최단 거리가 정의되지 않는다는 사실도 보고한다. 이 문서의 예제는 파일로 저장할 때 PowerShell에서 `bellman_ford.rs`라는 이름을 사용한다.

## 동작 다이어그램

![음의 간선을 포함한 반복 완화 과정](/assets/bellman-ford.svg)

그림의 A→B 가중치 4와 B→C 가중치 −2를 차례로 완화하면 거리는 `[0, 4, 2]`가 됩니다. 모든 간선을 최대 `V−1`회 반복하고 추가 갱신이 있으면 도달 가능한 음의 사이클을 검사합니다.

### 그림 단계별 읽기

- 그림의 방향 간선과 가중치는 A→B `4`, A→C `5`, B→C `−2`이며 A가 시작 정점입니다.
- 초기 거리는 `[0, ∞, ∞]`이고, 한 번째 반복에서 A→B를 적용해 B=`4`, A→C를 적용해 C=`5`가 됩니다.
- 이어 B→C를 적용해 C를 `4 + (−2) = 2`로 완화하므로 결과가 `[0, 4, 2]`가 되며, 두 번째 반복에는 변화가 없습니다.
- 그림에는 음의 간선은 있지만 사이클은 없습니다. 벨만–포드는 이런 음수 간선을 허용하고, `V−1`회 뒤에도 갱신되면 시작 정점에서 도달 가능한 음의 사이클로 판단합니다.


## 입력, 가정과 동작

간선은 `(시작 정점, 끝 정점, 가중치)`로 표현하고 정점 번호는 `0..vertex_count`여야 한다. `vertex_count == 0`, 시작 정점 범위 초과, 간선 끝점 범위 초과는 입력 오류다. 평행 간선과 자기 자신으로 가는 간선을 허용한다. 거리는 `Option<i64>`로 표현하므로 `None`은 도달 불가능을 뜻하며, 임의의 큰 정수를 무한대의 센티널로 사용하지 않는다.

각 완화에서 `checked_add`를 사용한다. 거리 계산이 `i64` 범위를 벗어나면 `Overflow` 오류를 반환한다. 시작 정점에서 도달 가능한 음의 사이클이 발견되면 `ReachableNegativeCycle` 오류를 반환한다. 도달할 수 없는 부분 그래프의 음의 사이클은 결과에 영향을 주지 않으므로 오류가 아니다.

초기화는 모든 거리를 `None`으로 만들고 시작 정지만 `Some(0)`으로 설정한다. 이후 최대 `V - 1`회 모든 간선을 순회하며 완화하고, 한 번도 갱신되지 않으면 일찍 종료한다. 마지막으로 한 번 더 순회해 갱신 가능한 간선이 있는지 확인하면 도달 가능한 음의 사이클을 검출할 수 있다.

시간 복잡도는 초기화 `O(V)`, 본 완화 `O(VE)`, 최종 사이클 검사 `O(E)`이므로 전체 `O(V + VE)`이다. 공간 복잡도는 거리 배열 `O(V)`이며 입력 간선 저장 공간 `O(E)`는 별도다. 간선 목록은 방향 간선으로 해석한다.

## Rust 예제

```rust
#[derive(Debug, PartialEq, Eq)]
enum BellmanFordError {
    EmptyGraph,
    InvalidSource,
    InvalidVertex,
    Overflow,
    ReachableNegativeCycle,
}

fn bellman_ford(
    vertex_count: usize,
    edges: &[(usize, usize, i64)],
    source: usize,
) -> Result<Vec<Option<i64>>, BellmanFordError> {
    if vertex_count == 0 {
        return Err(BellmanFordError::EmptyGraph);
    }
    if source >= vertex_count {
        return Err(BellmanFordError::InvalidSource);
    }
    if edges.iter().any(|&(from, to, _)| from >= vertex_count || to >= vertex_count) {
        return Err(BellmanFordError::InvalidVertex);
    }

    let mut distance: Vec<Option<i64>> = vec![None; vertex_count];
    distance[source] = Some(0);

    for _ in 1..vertex_count {
        let mut changed = false;
        for &(from, to, weight) in edges {
            let Some(from_distance) = distance[from] else { continue };
            let candidate = from_distance
                .checked_add(weight)
                .ok_or(BellmanFordError::Overflow)?;
            if distance[to].is_none_or(|current| candidate < current) {
                distance[to] = Some(candidate);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    for &(from, to, weight) in edges {
        let Some(from_distance) = distance[from] else { continue };
        let candidate = from_distance
            .checked_add(weight)
            .ok_or(BellmanFordError::Overflow)?;
        if distance[to].is_none_or(|current| candidate < current) {
            return Err(BellmanFordError::ReachableNegativeCycle);
        }
    }
    Ok(distance)
}

fn main() {
    let edges = [(0, 1, 4), (0, 2, 5), (1, 2, -2)];
    println!("{:?}", bellman_ford(3, &edges, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_unreachable_vertices_as_none() {
        let edges = [(0, 1, 3)];
        assert_eq!(bellman_ford(3, &edges, 0).unwrap(), vec![Some(0), Some(3), None]);
    }

    #[test]
    fn ignores_unreachable_negative_cycle() {
        let edges = [(0, 1, 2), (2, 3, -2), (3, 2, 1)];
        assert_eq!(bellman_ford(4, &edges, 0).unwrap(), vec![Some(0), Some(2), None, None]);
    }

    #[test]
    fn reports_reachable_negative_cycle() {
        let edges = [(0, 1, 1), (1, 2, -3), (2, 1, 1)];
        assert_eq!(
            bellman_ford(3, &edges, 0),
            Err(BellmanFordError::ReachableNegativeCycle)
        );
    }

    #[test]
    fn reports_checked_add_overflow() {
        let edges = [(0, 1, i64::MAX), (1, 2, 1)];
        assert_eq!(bellman_ford(3, &edges, 0), Err(BellmanFordError::Overflow));
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(bellman_ford(0, &[], 0), Err(BellmanFordError::EmptyGraph));
        assert_eq!(bellman_ford(2, &[], 2), Err(BellmanFordError::InvalidSource));
        assert_eq!(bellman_ford(2, &[(0, 2, 1)], 0), Err(BellmanFordError::InvalidVertex));
    }
}
```

## 테스트 실행

코드를 `bellman_ford.rs`로 저장한 뒤 PowerShell에서 `rustc --test bellman_ford.rs -o bellman_ford_tests.exe`로 컴파일하고 `.\bellman_ford_tests.exe`를 실행한다. `Option::is_none_or` 사용을 위해 Rust 1.82 이상이 필요하다.

## 참고 자료

- [DSAR Chapter 22: Single-Source Shortest Paths](https://dsar.rantai.dev/docs/part-v/chapter-22/)
- [Rust: i64 checked_add](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
