# 크루스칼 알고리즘

## 동작 다이어그램

![가중치 순서로 간선을 선택해 사이클을 피하는 크루스칼 과정](/assets/kruskal.svg)

간선을 가중치 오름차순으로 살펴보며 서로 다른 집합을 잇는 간선만 선택합니다. 그림에서는 1, 2, 4 가중치 간선이 선택되어 사이클 없는 MST를 만들고, 가중치 3 간선은 이미 연결된 두 정점을 잇기 때문에 건너뜁니다.

### 그림 단계별 읽기

- 후보 간선은 가중치 1의 A-B, 2의 B-C, 3의 A-C, 4의 C-D 순서로 검사합니다.
- A-B와 B-C를 선택하면 A, B, C가 연결되므로 A-C는 이미 연결된 두 정점을 다시 이어 사이클을 만들어 거절합니다.
- C-D는 서로 다른 집합을 연결하므로 선택하고, 그림의 MST 간선은 가중치 1, 2, 4이며 총합은 7입니다.
- 그림의 정점은 A, B, C, D 네 개이고 선택 간선 세 개가 모든 정점을 연결하므로 사이클 없는 신장 트리가 됩니다.


크루스칼 알고리즘은 간선의 가중치를 오름차순으로 살펴보면서 사이클을 만들지 않는 간선을 선택해 최소 신장 트리(MST)를 만드는 방법입니다. 간선마다 두 끝점이 이미 같은 집합에 속하는지 판정하면 사이클 여부를 알 수 있습니다. 이 판정에는 서로소 집합(Union-Find)을 사용하며, 아래 구현은 경로 압축과 크기 기준 합치기를 함께 사용합니다.

정점이 여러 연결 요소로 나뉘어 있으면 신장 트리 하나가 존재하지 않습니다. 이 경우에도 알고리즘은 각 연결 요소의 최소 신장 트리를 모은 최소 신장 포리스트를 반환합니다. 따라서 반환된 간선 수는 연결 그래프에서는 `정점 수 - 1`이고, 일반 그래프에서는 그보다 작을 수 있습니다. 정점 번호는 `0..vertex_count` 범위여야 하며, 범위를 벗어난 간선은 오류입니다. 자기 자신으로 향하는 간선은 사이클을 만들므로 무시합니다. 평행 간선은 각각 후보로 처리하고 더 가벼운 간선이 선택됩니다.

가중치는 `i64`의 음수도 허용합니다. 선택된 간선의 총합을 `i64::checked_add`로 계산하므로 중간 합계가 표현 범위를 벗어나면 최종 합이 범위 안에 들어올 수 있더라도 오류를 반환합니다. 같은 가중치의 간선이 여러 개면 선택된 포리스트는 입력 순서에 따라 달라질 수 있습니다.

## 복잡도와 한계

간선 수를 `E`, 정점 수를 `V`라고 하면 간선 정렬이 `O(E log E)`이고, Union-Find 연산들의 총 비용은 경로 압축과 크기 기준 합치기로 거의 선형인 `O(E α(V))`입니다. 정점 초기화를 포함한 전체 시간 복잡도는 `O(V + E log(E + 1))`, 추가 공간 복잡도는 `O(V + E)`입니다. 이 구현은 정점 번호를 연속된 `0..V` 정수로 표현하며, `usize`로 할당할 수 있는 크기의 그래프를 대상으로 합니다. 그래프가 연결되지 않아도 오류로 취급하지 않고 포리스트를 반환합니다.

참고 자료: [DSAR Chapter 24 - Minimum Spanning Tree](https://dsar.rantai.dev/docs/part-v/chapter-24/)

## 실행 방법

다음처럼 PowerShell에서 코드 블록을 `kruskal.rs`로 저장한 뒤 컴파일할 수 있습니다.

```powershell
rustc --test kruskal.rs -o kruskal_tests.exe
.\kruskal_tests.exe
```

## 실행 가능한 예제

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64,
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(count: usize) -> Self {
        Self {
            parent: (0..count).collect(),
            size: vec![1; count],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            let root = self.find(self.parent[value]);
            self.parent[value] = root;
        }
        self.parent[value]
    }

    fn unite(&mut self, left: usize, right: usize) -> bool {
        let mut left_root = self.find(left);
        let mut right_root = self.find(right);
        if left_root == right_root {
            return false;
        }
        if self.size[left_root] < self.size[right_root] {
            std::mem::swap(&mut left_root, &mut right_root);
        }
        self.parent[right_root] = left_root;
        self.size[left_root] += self.size[right_root];
        true
    }
}

fn minimum_spanning_forest(
    vertex_count: usize,
    mut edges: Vec<Edge>,
) -> Result<(Vec<Edge>, i64), &'static str> {
    if edges.iter().any(|edge| edge.from >= vertex_count || edge.to >= vertex_count) {
        return Err("edge endpoint is outside the vertex range");
    }
    edges.sort_by_key(|edge| edge.weight);
    let mut sets = DisjointSet::new(vertex_count);
    let mut chosen = Vec::new();
    let mut total = 0_i64;
    for edge in edges {
        if edge.from == edge.to || !sets.unite(edge.from, edge.to) {
            continue;
        }
        total = total.checked_add(edge.weight).ok_or("total weight overflow")?;
        chosen.push(edge);
    }
    Ok((chosen, total))
}

fn main() {
    let edges = vec![
        Edge { from: 0, to: 1, weight: 4 },
        Edge { from: 0, to: 2, weight: 1 },
        Edge { from: 1, to: 2, weight: 2 },
        Edge { from: 3, to: 3, weight: -10 },
        Edge { from: 3, to: 4, weight: -2 },
    ];
    let (forest, total) = minimum_spanning_forest(5, edges).expect("valid graph");
    println!("forest = {forest:?}, total weight = {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_mst_with_negative_and_parallel_edges() {
        let edges = vec![
            Edge { from: 0, to: 1, weight: 5 },
            Edge { from: 0, to: 1, weight: -2 },
            Edge { from: 1, to: 2, weight: 3 },
            Edge { from: 0, to: 2, weight: 9 },
            Edge { from: 2, to: 2, weight: -100 },
        ];
        let (chosen, total) = minimum_spanning_forest(3, edges).unwrap();
        assert_eq!(chosen.len(), 2);
        assert_eq!(total, 1);
    }

    #[test]
    fn returns_a_forest_for_disconnected_graph() {
        let edges = vec![Edge { from: 0, to: 1, weight: 7 }, Edge { from: 2, to: 3, weight: 1 }];
        let (chosen, total) = minimum_spanning_forest(5, edges).unwrap();
        assert_eq!(chosen.len(), 2);
        assert_eq!(total, 8);
    }

    #[test]
    fn rejects_bad_endpoint_and_overflow() {
        assert!(minimum_spanning_forest(2, vec![Edge { from: 0, to: 2, weight: 1 }]).is_err());
        assert!(minimum_spanning_forest(
            3,
            vec![Edge { from: 0, to: 1, weight: i64::MAX }, Edge { from: 1, to: 2, weight: 1 }],
        ).is_err());
    }
}
```
