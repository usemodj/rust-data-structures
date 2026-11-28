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


## 예제로 보는 상태 변화 추적

앞의 예제와 같은 그래프를 사용한다. 간선은 코드에 적힌 순서대로 검사하며, 시작 정점은 `0`이다.

```text
정점: 0, 1, 2
간선: (0 -> 1, 가중치 4), (0 -> 2, 가중치 5), (1 -> 2, 가중치 -2)
시작 정점: 0
```

거리 배열은 `[정점 0, 정점 1, 정점 2]` 순서다. `∞`는 아직 도달할 수 없어 코드에서 `None`으로 저장된 상태를 뜻한다. 간선 `(u, v, w)`를 처리할 때 `u`가 도달 불가능하면 건너뛰고, 그렇지 않으면 `거리[u] + w`를 후보 거리로 계산한다. 후보가 현재 `거리[v]`보다 작을 때만 갱신한다.

### 초기화

모든 거리를 `None`으로 시작하고, 시작 정점 `0`의 거리만 `Some(0)`으로 설정한다.

```text
[0, ∞, ∞]
```

### 첫 번째 완화 반복

이번 반복의 `changed`는 `false`로 시작한다. 이 구현은 간선을 순서대로 처리하면서 거리 배열을 즉시 갱신하므로, 앞에서 갱신한 거리를 같은 반복 안에서 뒤쪽 간선이 사용할 수 있다.

| 순서 | 간선 | 후보 거리와 비교 | 처리 후 거리 | 변화 |
| --- | --- | --- | --- | --- |
| 1 | `0 -> 1 (4)` | `0 + 4 = 4`; `1`은 아직 `None` | `[0, 4, ∞]` | 갱신 |
| 2 | `0 -> 2 (5)` | `0 + 5 = 5`; `2`는 아직 `None` | `[0, 4, 5]` | 갱신 |
| 3 | `1 -> 2 (-2)` | `4 + (-2) = 2`; 기존 `5`보다 작음 | `[0, 4, 2]` | 갱신 |

적어도 한 번 갱신되었으므로 반복 끝에서도 `changed`는 `true`다. 특히 세 번째 간선은 첫 번째 간선에서 같은 반복 중 갱신한 `거리[1] = 4`를 사용해 `거리[2]`를 `5`에서 `2`로 낮춘다.

### 두 번째 완화 반복과 조기 종료

다시 `changed = false`로 시작한다. 현재 거리 `[0, 4, 2]`에서 모든 간선을 확인하지만 더 짧은 거리는 나오지 않는다.

| 간선 | 후보 거리와 비교 | 처리 후 거리 | 변화 |
| --- | --- | --- | --- |
| `0 -> 1 (4)` | `0 + 4 = 4`; 기존 `4`와 같음 | `[0, 4, 2]` | 없음 |
| `0 -> 2 (5)` | `0 + 5 = 5`; 기존 `2`보다 큼 | `[0, 4, 2]` | 없음 |
| `1 -> 2 (-2)` | `4 + (-2) = 2`; 기존 `2`와 같음 | `[0, 4, 2]` | 없음 |

반복 전체에서 갱신이 없으므로 `changed`가 계속 `false`다. 따라서 나머지 반복을 생략하고 조기 종료한다. 간선 순서가 달라지면 같은 반복 안에서 이어지는 간선이 사용할 수 있는 갱신 결과가 달라질 수 있지만, 최대 `V - 1`회 반복하면 최단 거리를 전파할 수 있다.

### 음의 사이클 검사와 최종 결과

조기 종료 후에도 간선을 한 번 더 확인한다. 어떤 도달 가능한 간선이라도 거리를 더 줄일 수 있으면 도달 가능한 음의 사이클이 있는 것이므로 오류를 반환한다. 이 예제에서는 세 간선 모두 거리를 줄이지 못하므로 음의 사이클 오류 없이 다음 결과를 반환한다.

```text
[Some(0), Some(4), Some(2)]
```
## 입력, 가정과 동작

간선은 `(시작 정점, 끝 정점, 가중치)`로 표현하고 정점 번호 `v`는 `0 <= v < vertex_count`여야 한다. `source`는 출발 정점의 인덱스다. 반환 배열의 `distance[v]`는 `source`에서 정점 `v`까지의 최단 거리를 나타낸다. 코드에서 `distance[source]`를 `Some(0)`으로 초기화하며, `source`도 `0 <= source < vertex_count` 범위여야 한다. 예제에서 `source = 0`이면 결과 `[Some(0), Some(4), Some(2)]`는 정점 0에서 각 정점까지의 거리를 뜻한다.

`vertex_count == 0`, 시작 정점 범위 초과, 간선 끝점 범위 초과는 입력 오류다. 평행 간선과 자기 자신으로 가는 간선을 허용한다. 거리는 `Option<i64>`로 표현하므로 `None`은 도달 불가능을 뜻하며, 임의의 큰 정수를 무한대의 센티널로 사용하지 않는다.

각 완화에서 `checked_add`를 사용한다. 거리 계산이 `i64` 범위를 벗어나면 `Overflow` 오류를 반환한다. 시작 정점에서 도달 가능한 음의 사이클이 발견되면 `ReachableNegativeCycle` 오류를 반환한다. 도달할 수 없는 부분 그래프의 음의 사이클은 결과에 영향을 주지 않으므로 오류가 아니다.

초기화는 모든 거리를 `None`으로 만들고 시작 정지만 `Some(0)`으로 설정한다. 이후 최대 `V - 1`회 모든 간선을 순회하며 완화하고, 한 번도 갱신되지 않으면 일찍 종료한다. 마지막으로 한 번 더 순회해 갱신 가능한 간선이 있는지 확인하면 도달 가능한 음의 사이클을 검출할 수 있다.

시간 복잡도는 초기화 `O(V)`, 본 완화 `O(VE)`, 최종 사이클 검사 `O(E)`이므로 전체 `O(V + VE)`이다. 공간 복잡도는 거리 배열 `O(V)`이며 입력 간선 저장 공간 `O(E)`는 별도다. 간선 목록은 방향 간선으로 해석한다.

## 어디에 사용되는가

벨만–포드는 간선 가중치에 음수가 있을 수 있고, 한 시작점에서의 최단 거리와 도달 가능한 음의 사이클 여부를 함께 알아야 할 때 유용하다.

### 음수 비용이 포함된 경로

운송비 할인이나 보상처럼 일부 구간의 비용이 음수로 표현될 수 있는 그래프에서 최단 경로를 구할 수 있다. 단, 시작점에서 도달 가능한 음의 사이클이 있으면 비용을 계속 낮출 수 있어 최단 거리가 존재하지 않으며, 알고리즘은 이를 오류로 보고한다.

### 환율 차익 거래 탐지

통화를 정점으로, 환전 비율을 간선으로 나타낼 수 있다. 환전 비율 `r`을 가중치 `-ln(r)`로 바꾸면, 환전 사이클의 가중치 합이 음수인 경우 원래 환율의 곱이 1보다 커져 차익 거래 가능성을 뜻한다. 실제 시장에서는 수수료, 호가 차이, 유동성, 환전 시점 등을 추가로 고려해야 하므로 이 모델만으로 실제 수익이 보장되지는 않는다.

### 차분 제약식의 모순 확인

`x_v <= x_u + c` 형태의 제약을 `u -> v` 가중치 `c`인 간선으로 바꾸어 그래프에 담을 수 있다. 모든 변수에 도달할 수 있도록 가상의 시작점을 추가해 벨만–포드를 실행했을 때 음의 사이클이 있으면 제약을 동시에 만족시킬 수 없다.

### 거리 벡터 라우팅

라우터가 이웃에게 목적지까지의 거리 정보를 반복해서 전달하고, 그 정보를 바탕으로 경로 비용을 갱신하는 거리 벡터 방식은 벨만–포드의 점화식에 기반한다. RIP는 이런 방식의 대표적인 라우팅 프로토콜이며, 실제 프로토콜은 루프나 잘못된 거리 증가를 다루기 위한 별도 규칙도 사용한다.

간선 가중치가 모두 음수가 아니고 그래프가 큰 경우에는 일반적으로 `O((V + E) log V)` 수준의 우선순위 큐 기반 다익스트라가 더 효율적이다. 음수 간선을 허용하거나 음의 사이클을 검사해야 할 때 벨만–포드를 고려한다.
## Rust 예제

```rust
/// Bellman-Ford 계산 중 발생할 수 있는 오류다.
/// `bellman_ford`가 반환하며, 아래 테스트에서 각 오류 상황을 확인한다.
#[derive(Debug, PartialEq, Eq)]
enum BellmanFordError {
    /// 정점이 하나도 없는 그래프를 전달했다.
    EmptyGraph,
    /// 시작 정점 번호가 그래프 범위를 벗어났다.
    InvalidSource,
    /// 간선의 시작 또는 끝 정점 번호가 그래프 범위를 벗어났다.
    InvalidVertex,
    /// 경로 거리의 덧셈 결과가 i64 범위를 벗어났다.
    Overflow,
    /// 시작 정점에서 도달 가능한 음의 사이클이 발견됐다.
    ReachableNegativeCycle,
}

/// 시작 정점에서 모든 정점까지의 최단 거리를 계산한다.
///
/// `source`는 출발 정점 인덱스이며, 반환 거리 배열은 이 정점에서 각 정점까지의 값을 담는다.
/// 유효 범위는 `0 <= source < vertex_count`다.
///
/// 최대 V - 1회 간선을 완화한 뒤, 도달 가능한 음의 사이클을 검사한다.
/// 사용처: 예제 main과 아래 다섯 테스트가 호출한다.
/// 오류는 입력 검증, checked_add, 음의 사이클 검사 단계에서 반환한다.
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

/// 예제 그래프를 만들고 `bellman_ford` 결과를 출력한다.
/// 일반 실행 파일로 컴파일할 때 실행되며, `rustc --test`에서는 테스트 하네스가 대신 실행된다.
fn main() {
    let edges = [(0, 1, 4), (0, 2, 5), (1, 2, -2)];
    println!("{:?}", bellman_ford(3, &edges, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 도달할 수 없는 정점의 거리가 None으로 남는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 테스트 하네스가 자동 호출한다.
    #[test]
    fn keeps_unreachable_vertices_as_none() {
        let edges = [(0, 1, 3)];
        assert_eq!(bellman_ford(3, &edges, 0).unwrap(), vec![Some(0), Some(3), None]);
    }

    /// 시작 정점에서 닿지 않는 음의 사이클은 오류가 아닌지 확인한다.
    /// 사용처: `rustc --test` 실행 시 테스트 하네스가 자동 호출한다.
    #[test]
    fn ignores_unreachable_negative_cycle() {
        let edges = [(0, 1, 2), (2, 3, -2), (3, 2, 1)];
        assert_eq!(bellman_ford(4, &edges, 0).unwrap(), vec![Some(0), Some(2), None, None]);
    }

    /// 시작 정점에서 도달 가능한 음의 사이클을 오류로 보고하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 테스트 하네스가 자동 호출한다.
    #[test]
    fn reports_reachable_negative_cycle() {
        let edges = [(0, 1, 1), (1, 2, -3), (2, 1, 1)];
        assert_eq!(
            bellman_ford(3, &edges, 0),
            Err(BellmanFordError::ReachableNegativeCycle)
        );
    }

    /// 경로 거리 덧셈이 i64 범위를 벗어날 때 오류를 반환하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 테스트 하네스가 자동 호출한다.
    #[test]
    fn reports_checked_add_overflow() {
        let edges = [(0, 1, i64::MAX), (1, 2, 1)];
        assert_eq!(bellman_ford(3, &edges, 0), Err(BellmanFordError::Overflow));
    }

    /// 빈 그래프, 잘못된 시작 정점, 잘못된 간선 끝점을 거부하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 테스트 하네스가 자동 호출한다.
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
