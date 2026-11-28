# 플로이드–워셜 알고리즘

플로이드–워셜(Floyd–Warshall)은 모든 정점 쌍의 최단 거리를 동적으로 계산한다. 음의 간선을 허용하지만, 음의 사이클이 있으면 해당 거리들이 유한한 최솟값을 갖지 않으므로 결과 대신 오류를 반환하도록 이 문서에서 동작을 명확히 정한다. 이 문서의 예제는 파일로 저장할 때 PowerShell에서 `floyd_warshall.rs`라는 이름을 사용한다.

## 동작 다이어그램

![코드 예제의 방향 그래프와 k=1 경유 후 최단 거리 행렬](/assets/floyd-warshall.svg)

그림은 아래 코드 예제와 같은 방향 그래프의 입력 간선과 거리 행렬 변화를 보여줍니다. 간선 `0 -> 1`(4), `0 -> 2`(11), `1 -> 2`(-2)에서 중간 정점 `k = 1`을 거치면 `0 -> 1 -> 2`의 비용 `4 + (-2) = 2`가 직접 비용 `11`보다 작아 `D[0][2]`가 갱신됩니다.

### 그림 단계별 읽기

- 방향 간선은 `0 → 1` 가중치 `4`, `0 → 2` 가중치 `11`, `1 → 2` 가중치 `-2`이며, 다른 직접 간선은 없습니다.
- 대각선을 0으로 초기화한 직후의 거리는 `[[0, 4, 11], [∞, 0, -2], [∞, ∞, 0]]`입니다.
- `k = 0`에서는 거리가 바뀌지 않습니다. `k = 1`에서는 `D[0][2] = min(11, D[0][1] + D[1][2]) = min(11, 4 + (-2)) = 2`로 갱신됩니다.
- `k = 2`에서는 더 짧은 경로가 없어 최종 행렬은 `[[0, 4, 2], [∞, 0, -2], [∞, ∞, 0]]`입니다. 음수 간선은 있지만 음의 사이클은 없습니다.

## 예제로 보는 상태 변화 추적

아래 Rust 예제의 그래프를 그대로 사용한다. 행은 출발 정점 `i`, 열은 도착 정점 `j`이며, `∞`는 코드의 `None`을 뜻한다.

```text
정점: 0, 1, 2
간선: 0 -> 1 (4), 0 -> 2 (11), 1 -> 2 (-2)
```

### 초기화

입력 `weights`를 `distance`에 복사하고 대각선의 `None`을 `Some(0)`으로 바꾼다. 원본 입력은 바뀌지 않는다.

```text
        도착 0  1   2
출발 0      0   4  11
출발 1      ∞   0  -2
출발 2      ∞   ∞   0
```

이후 `k = 0, 1, 2` 순서로 경유 정점을 허용한다. 각 `k`에서 `i`, `j`를 순서대로 살펴보고 `distance[i][k]`와 `distance[k][j]`가 모두 존재할 때만 두 값을 더한다. 후보가 기존 거리보다 작거나 기존 거리가 `None`이면 즉시 갱신한다.

### `k = 0`: 정점 0을 경유

정점 1과 2에서는 정점 0에 도달할 수 없으므로 해당 출발 행을 건너뛴다. 정점 0에서 출발하는 후보는 `0 + 0`, `0 + 4`, `0 + 11`로 모두 기존 거리와 같다. 따라서 행렬에 변화가 없다.

```text
[[0, 4, 11],
 [∞, 0, -2],
 [∞, ∞,  0]]
```

### `k = 1`: 정점 1을 경유

정점 0에서 1까지의 거리 `4`를 사용해 다음과 같이 비교한다.

| 도착 정점 `j` | 경유 후보 | 기존 거리와 비교 | 처리 |
| --- | --- | --- | --- |
| `0` | `distance[1][0]`이 `None` | 후보를 계산할 수 없음 | 건너뜀 |
| `1` | `4 + 0 = 4` | 기존 `4`와 같음 | 변화 없음 |
| `2` | `4 + (-2) = 2` | 기존 `11`보다 작음 | `distance[0][2]`를 `2`로 갱신 |

정점 1에서 출발하는 후보는 기존 거리와 같고, 정점 2에서는 정점 1에 도달할 수 없어 건너뛴다. 이 단계가 끝난 거리 행렬은 다음과 같다.

```text
[[0, 4,  2],
 [∞, 0, -2],
 [∞, ∞,  0]]
```

### `k = 2`: 정점 2를 경유하고 결과 확인

정점 2에서 정점 0과 1로 가는 경로는 없으므로 그 열의 후보는 건너뛴다. 정점 2로 가는 후보는 각각 `2 + 0 = 2`, `-2 + 0 = -2`, `0 + 0 = 0`으로 기존 값과 같아 갱신되지 않는다.

모든 경유 정점을 처리한 뒤 대각선은 `[0, 0, 0]`이므로 음의 사이클 오류가 없다. 최종 반환값은 다음과 같다.

```text
Ok([[Some(0), Some(4), Some(2)],
    [None, Some(0), Some(-2)],
    [None, None, Some(0)]])
```

## 입력, 가정과 동작

입력 매개변수 `weights`는 `n × n` 크기의 방향 그래프 인접 행렬이다. `weights[u][v]`에서 행 `u`는 출발 정점, 열 `v`는 도착 정점을 뜻한다. 값이 `Some(w)`이면 `u -> v` 직접 간선의 가중치가 `w`이고, `None`이면 직접 간선이 없다는 뜻이다. 이 값은 직접 간선 비용이지, 아직 중간 정점을 거쳐 계산한 최단 거리가 아니다. 예제에서는 `weights[0][1] == Some(4)`, `weights[1][2] == Some(-2)`, `weights[2][0] == None`이다.

행렬은 비어 있지 않은 정방 행렬이어야 한다. 대각선의 입력값이 없으면 거리 0을 넣고, 입력된 자기 간선과 0 중 더 작은 값을 초기 거리로 사용한다. 따라서 양의 자기 간선은 0으로, 음의 자기 간선은 그대로 시작한다. 함수는 입력을 읽기 전용 참조로 받고 작업용 거리 행렬에 복사하므로 원본 `weights`는 바뀌지 않는다.

거리는 `Option<i64>`로 유지한다. `None`을 무한대 센티널로 더하지 않으며, 두 부분 경로가 모두 `Some`일 때만 `checked_add`한다. 합이 `i64` 범위를 벗어나면 `Overflow` 오류를 반환한다. 모든 중간 정점을 처리한 뒤 대각선의 값이 음수이면 음의 사이클이 존재한다고 보고 `NegativeCycle` 오류를 반환한다. 즉 이 구현은 음의 사이클이 포함된 행렬에서 부분적인 거리 행렬을 성공값으로 내보내지 않는다. 연결되지 않은 정점 쌍은 정상 성공 결과에서 `None`으로 남는다.

초기화는 `V²`개의 행렬 칸을 복사·정규화하고 대각선을 처리하므로 `O(V²)` 시간과 `O(V²)` 공간이 필요하다. 이어서 중간 정점 `k`, 출발 정점 `i`, 도착 정점 `j`를 모두 순회하며 `O(V³)` 시간에 완화한다. 마지막 음의 사이클 검사는 `O(V)`이므로 전체 시간 복잡도는 `O(V³)`이고 추가 결과 행렬 공간은 `O(V²)`이다. 입력 행렬 자체의 저장 공간도 `O(V²)`다.

## 어디에 사용되는가

플로이드–워셜은 한 출발점의 경로만 찾는 것이 아니라 모든 출발점과 도착점 조합의 최단 거리를 미리 구하고 싶을 때 적합하다. 결과 행렬이 만들어지면 각 정점 쌍의 거리를 빠르게 조회할 수 있다.

- **교통·운송 경로 사전 계산:** 도시나 정류장 수가 제한된 노선망에서 모든 지점 사이의 최단 이동 시간이나 비용을 미리 계산한다. 이후 여러 경로 질의에 같은 결과 행렬을 재사용할 수 있다.
- **네트워크 비용 분석:** 서버나 장비 사이의 통신 지연·비용을 간선 가중치로 두고, 모든 장비 쌍 사이에서 가장 낮은 비용의 경로를 찾는다.
- **음수 가중치가 있는 그래프:** 보상이나 할인처럼 음수 간선이 존재해도 음의 사이클이 없다면 최단 경로를 계산할 수 있다. 이 구현은 음의 사이클이 하나라도 발견되면 `NegativeCycle` 오류를 반환한다.
- **차익 거래 및 음의 사이클 탐지:** 환율 `r`을 가중치 `-ln(r)`로 바꾸면, 환전 사이클의 가중치 합이 음수인 경우를 탐지할 수 있다. 실제 거래에서는 수수료, 호가 차이, 유동성 등을 함께 고려해야 한다.
- **도달 가능성 계산의 변형:** 거리 대신 불리언 값을 사용하고 논리 연산으로 갱신하면 그래프에서 어떤 정점이 다른 정점에 도달할 수 있는지(전이적 폐쇄)를 계산할 수 있다. 이는 이 문서의 거리 행렬 구현과는 별도의 변형이다.

정점 수가 크거나 간선이 드문 그래프, 또는 출발점 하나의 거리만 필요한 경우에는 `O(V³)` 시간과 `O(V²)` 공간이 부담될 수 있다. 음이 아닌 가중치의 단일 출발점 문제에는 다익스트라, 음수 간선이 있는 단일 출발점 문제에는 벨만–포드가 더 알맞을 수 있다.

## Rust 예제

```rust
/// floyd_warshall이 입력 오류 또는 최단 거리를 정의할 수 없는 상황에서 반환하는 오류다.
/// 아래 테스트에서 각 오류가 반환되는 조건을 확인한다.
#[derive(Debug, PartialEq, Eq)]
enum FloydWarshallError {
    /// 입력 그래프에 정점이 없다.
    EmptyGraph,
    /// 인접 행렬의 행 중 길이가 서로 다른 행이 있다.
    NotSquare,
    /// 두 경로 거리의 합이 `i64` 범위를 벗어난다.
    Overflow,
    /// 음의 사이클로 인해 유한한 최단 거리를 정의할 수 없다.
    NegativeCycle,
}

/// 모든 정점 쌍 사이의 최단 거리를 계산한다.
///
/// `weights`는 읽기 전용 정방 인접 행렬이다. `weights[u][v]`는 `u -> v` 직접 간선의 가중치이며, `None`은 직접 간선이 없음을 뜻한다.
/// 이 행렬을 작업용 `distance`로 복사해 갱신하므로 입력 원본은 변경되지 않는다.
///
/// 각 정점을 차례로 경유 정점으로 허용하며 거리를 갱신한다.
/// 사용처: 예제 `main`과 아래 다섯 테스트에서 호출한다.
/// 빈 행렬, 정방 행렬이 아닌 입력, 덧셈 오버플로, 음의 사이클은 오류로 반환한다.

fn floyd_warshall(
    weights: &[Vec<Option<i64>>],
) -> Result<Vec<Vec<Option<i64>>>, FloydWarshallError> {
    let n = weights.len();
    if n == 0 {
        return Err(FloydWarshallError::EmptyGraph);
    }
    if weights.iter().any(|row| row.len() != n) {
        return Err(FloydWarshallError::NotSquare);
    }

    let mut distance = weights.to_vec();
    for i in 0..n {
        if distance[i][i].is_none_or(|value| value > 0) {
            distance[i][i] = Some(0);
        }
    }

    for k in 0..n {
        for i in 0..n {
            let Some(left) = distance[i][k] else { continue };
            for j in 0..n {
                let Some(right) = distance[k][j] else { continue };
                let candidate = left
                    .checked_add(right)
                    .ok_or(FloydWarshallError::Overflow)?;
                if distance[i][j].is_none_or(|current| candidate < current) {
                    distance[i][j] = Some(candidate);
                }
            }
        }
    }

    if (0..n).any(|i| distance[i][i].is_some_and(|value| value < 0)) {
        return Err(FloydWarshallError::NegativeCycle);
    }
    Ok(distance)
}

/// 예제 그래프를 구성하고 모든 쌍의 거리 행렬을 출력한다.
/// 일반 실행 시 프로그램 진입점으로 실행된다. `rustc --test`에서는 테스트 하네스가 테스트를 실행한다.
fn main() {
    let graph = vec![
        vec![None, Some(4), Some(11)],
        vec![None, None, Some(-2)],
        vec![None, None, None],
    ];
    println!("{:?}", floyd_warshall(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 도달할 수 없는 정점 쌍은 `None`, 대각선 거리는 0으로 남는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 Rust 테스트 하네스가 자동 호출한다.
    #[test]
    fn preserves_disconnected_pairs_as_none() {
        let graph = vec![vec![None, Some(2), None], vec![None, None, None], vec![None, None, None]];
        let result = floyd_warshall(&graph).unwrap();
        assert_eq!(result[0][0], Some(0));
        assert_eq!(result[0][1], Some(2));
        assert_eq!(result[0][2], None);
        assert_eq!(result[2][0], None);
    }

    /// 음의 사이클이 없을 때 음수 간선으로 더 짧은 경로를 찾는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 Rust 테스트 하네스가 자동 호출한다.
    #[test]
    fn uses_negative_edges_without_a_cycle() {
        let graph = vec![
            vec![None, Some(4), Some(10)],
            vec![None, None, Some(-3)],
            vec![None, None, None],
        ];
        let result = floyd_warshall(&graph).unwrap();
        assert_eq!(result[0][2], Some(1));
    }

    /// 음의 사이클이 있을 때 `NegativeCycle` 오류를 반환하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 Rust 테스트 하네스가 자동 호출한다.
    #[test]
    fn reports_negative_cycles_explicitly() {
        let graph = vec![vec![None, Some(1)], vec![Some(-2), None]];
        assert_eq!(floyd_warshall(&graph), Err(FloydWarshallError::NegativeCycle));
    }

    /// 경로 거리 합이 오버플로할 때 `Overflow` 오류를 반환하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 Rust 테스트 하네스가 자동 호출한다.
    #[test]
    fn reports_checked_add_overflow() {
        let graph = vec![
            vec![None, Some(i64::MAX), None],
            vec![None, None, Some(1)],
            vec![None, None, None],
        ];
        assert_eq!(floyd_warshall(&graph), Err(FloydWarshallError::Overflow));
    }

    /// 빈 행렬과 정방 행렬이 아닌 입력을 거부하는지 확인한다.
    /// 사용처: `rustc --test` 실행 시 Rust 테스트 하네스가 자동 호출한다.
    #[test]
    fn rejects_empty_and_non_square_input() {
        assert_eq!(floyd_warshall(&[]), Err(FloydWarshallError::EmptyGraph));
        let not_square = vec![vec![None], vec![None, None]];
        assert_eq!(floyd_warshall(&not_square), Err(FloydWarshallError::NotSquare));
    }
}
```

## 테스트 실행

코드를 `floyd_warshall.rs`로 저장한 뒤 PowerShell에서 `rustc --test floyd_warshall.rs -o floyd_warshall_tests.exe`로 컴파일하고 `.\floyd_warshall_tests.exe`를 실행한다. `Option::is_none_or` 사용을 위해 Rust 1.82 이상이 필요하다.

## 참고 자료

- [DSAR Chapter 23: All-Pairs Shortest Paths](https://dsar.rantai.dev/docs/part-v/chapter-23/)
- [Rust: i64 checked_add](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
