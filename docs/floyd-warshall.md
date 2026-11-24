# 플로이드–워셜 알고리즘

플로이드–워셜(Floyd–Warshall)은 모든 정점 쌍의 최단 거리를 동적으로 계산한다. 음의 간선을 허용하지만, 음의 사이클이 있으면 해당 거리들이 유한한 최솟값을 갖지 않으므로 결과 대신 오류를 반환하도록 이 문서에서 동작을 명확히 정한다. 이 문서의 예제는 파일로 저장할 때 PowerShell에서 `floyd_warshall.rs`라는 이름을 사용한다.

## 동작 다이어그램

![중간 정점을 거쳐 거리 행렬을 갱신하는 플로이드 워셜](/assets/floyd-warshall.svg)

그림은 중간 정점 B를 허용할 때 A에서 C의 직접 거리 11이 A→B→C의 거리 `1 + 1 = 2`로 바뀌는 과정을 나타냅니다. 알고리즘은 각 `k`를 중간 정점으로 삼아 `D[i][j]`와 `D[i][k] + D[k][j]` 중 작은 값을 선택합니다. 아래 Rust 코드 예제는 이 그림과 간선 가중치가 다릅니다.

### 그림 단계별 읽기

- 그림의 방향 간선은 A→B `1`, B→C `1`, A→C `11`이며 B가 중간 정점 `k`로 강조되어 있습니다.
- 초기 행렬에서 A 행은 `[0, 1, 11]`이고 B→C는 `1`이므로 A에서 C로 B를 거치는 후보는 `1 + 1 = 2`입니다.
- 갱신은 `D[A,C] = min(11, 2) = 2`이며, 그림의 다른 행렬 칸은 B와 C 사이의 직접 정보 또는 도달 불가 `∞`로 표시됩니다.
- 그림에는 음수 가중치와 사이클이 없습니다. 플로이드–워셜은 음수 간선도 다룰 수 있지만, 음의 사이클이 있으면 유한한 최단 거리가 없으므로 이 문서 구현은 오류를 반환합니다.


## 입력, 가정과 동작

입력은 정방 행렬 `weights`다. `weights[u][v] == Some(w)`는 방향 간선, `None`은 직접 간선이 없음을 뜻한다. 행렬이 비어 있거나 정방 행렬이 아니면 입력 오류다. 대각선의 입력값이 없으면 거리 0을 넣고, 입력된 자기 간선과 0 중 더 작은 값을 초기 거리로 사용한다. 따라서 양의 자기 간선은 0으로, 음의 자기 간선은 그대로 시작한다.

거리는 `Option<i64>`로 유지한다. `None`을 무한대 센티널로 더하지 않으며, 두 부분 경로가 모두 `Some`일 때만 `checked_add`한다. 합이 `i64` 범위를 벗어나면 `Overflow` 오류를 반환한다. 모든 중간 정점을 처리한 뒤 대각선의 값이 음수이면 음의 사이클이 존재한다고 보고 `NegativeCycle` 오류를 반환한다. 즉 이 구현은 음의 사이클이 포함된 행렬에서 부분적인 거리 행렬을 성공값으로 내보내지 않는다. 연결되지 않은 정점 쌍은 정상 성공 결과에서 `None`으로 남는다.

초기화는 `V²`개의 행렬 칸을 복사·정규화하고 대각선을 처리하므로 `O(V²)` 시간과 `O(V²)` 공간이 필요하다. 이어서 중간 정점 `k`, 출발 정점 `i`, 도착 정점 `j`를 모두 순회하며 `O(V³)` 시간에 완화한다. 마지막 음의 사이클 검사는 `O(V)`이므로 전체 시간 복잡도는 `O(V³)`이고 추가 결과 행렬 공간은 `O(V²)`이다. 입력 행렬 자체의 저장 공간도 `O(V²)`다.

## Rust 예제

```rust
#[derive(Debug, PartialEq, Eq)]
enum FloydWarshallError {
    EmptyGraph,
    NotSquare,
    Overflow,
    NegativeCycle,
}

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

    #[test]
    fn preserves_disconnected_pairs_as_none() {
        let graph = vec![vec![None, Some(2), None], vec![None, None, None], vec![None, None, None]];
        let result = floyd_warshall(&graph).unwrap();
        assert_eq!(result[0][0], Some(0));
        assert_eq!(result[0][1], Some(2));
        assert_eq!(result[0][2], None);
        assert_eq!(result[2][0], None);
    }

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

    #[test]
    fn reports_negative_cycles_explicitly() {
        let graph = vec![vec![None, Some(1)], vec![Some(-2), None]];
        assert_eq!(floyd_warshall(&graph), Err(FloydWarshallError::NegativeCycle));
    }

    #[test]
    fn reports_checked_add_overflow() {
        let graph = vec![
            vec![None, Some(i64::MAX), None],
            vec![None, None, Some(1)],
            vec![None, None, None],
        ];
        assert_eq!(floyd_warshall(&graph), Err(FloydWarshallError::Overflow));
    }

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
