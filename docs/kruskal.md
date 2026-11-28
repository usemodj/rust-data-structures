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

## 크루스칼 알고리즘의 활용

크루스칼 알고리즘은 후보 연결 비용을 무방향 간선의 가중치로 나타낼 수 있을 때, 모든 지점을 최소 총비용으로 연결하는 데 활용합니다.

- **통신·전력망 설계:** 거점 사이의 케이블이나 전력선 설치 비용을 줄이는 연결망을 찾습니다.
- **도로·철도 계획:** 지점 간 건설 비용을 바탕으로 모든 지점을 연결하는 기본 경로를 구합니다.
- **컴퓨터 네트워크:** 장비를 연결하는 후보 회선 중 총비용이 작은 연결 구조를 찾습니다.
- **데이터 군집화:** 유사도를 거리로 표현해 최소 신장 트리를 만든 뒤, 긴 간선을 제거하는 방식으로 항목을 그룹화할 수 있습니다.

예를 들어 네 지점이 모두 연결된 후보 간선 그래프라면, 각 간선의 설치비를 가중치로 사용해 사이클 없이 총 설치비가 최소인 연결을 찾습니다. 후보 경로가 부족해 그래프가 연결되지 않으면 하나의 신장 트리가 아니라 연결 요소별 최소 신장 포리스트가 만들어집니다. 실제 설계에서는 최소 비용 외에도 용량, 지형, 장애 대비와 같은 제약을 별도로 고려해야 합니다.

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

### 함수 설명

- **`DisjointSet::new(count) -> Self`:** `count`개의 정점으로 서로소 집합을 만듭니다. 각 정점이 자기 자신의 부모가 되도록 `parent`를 초기화하고, 각 집합의 크기를 `1`로 설정합니다.
- **`find(&mut self, value) -> usize`:** `value`가 속한 집합의 대표 정점(루트)을 반환합니다. 부모를 따라 루트를 찾은 뒤 경로상의 부모를 루트로 바꾸는 **경로 압축**을 적용하므로 이후 탐색을 빠르게 합니다. `value`는 집합의 정점 범위 안이어야 합니다.
- **`unite(&mut self, left, right) -> bool`:** `find`로 두 정점의 루트를 찾습니다. 이미 같은 집합이면 합치지 않고 `false`를 반환합니다. 다른 집합이면 작은 집합을 큰 집합에 연결하고 루트의 크기를 갱신한 뒤 `true`를 반환합니다. 두 정점 모두 집합의 범위 안이어야 합니다.
- **`minimum_spanning_forest(vertex_count, edges) -> Result<(Vec<Edge>, i64), &'static str>`:** 정점 수와 무방향 간선 목록을 받아 크루스칼 알고리즘을 실행합니다. 간선을 가중치순으로 정렬하고 자기 간선이나 사이클을 만드는 간선을 건너뛰어 선택한 간선과 총 가중치를 `Ok((chosen, total))`로 반환합니다. 정점 번호가 범위를 벗어나거나 가중치의 중간 합계가 오버플로하면 `Err`를 반환합니다. 연결되지 않은 그래프는 오류가 아니라 최소 신장 포리스트를 반환합니다.
- **`main()`:** 예제 간선을 만들고 `minimum_spanning_forest`를 호출해 결과를 출력합니다. 이 예제에서 오류가 발생하면 `expect("valid graph")`가 패닉을 발생시킵니다.

### 선택된 간선 목록의 의미

반환값 `Ok((chosen, total))`에서 `chosen`은 각 연결 요소의 정점들을 **사이클 없이 최소 총 가중치로 연결하기 위해 선택한 간선 목록**이고, `total`은 그 간선들의 가중치 합계입니다. 간선 `(from, to, weight)`는 두 정점 사이의 무방향 연결과 그 비용을 나타냅니다.

아래 `finds_mst_with_negative_and_parallel_edges` 테스트에서는 다음 간선이 선택됩니다.

- `(0, 1, -2)`: 정점 `0`과 `1`을 가중치 `-2`로 연결합니다.
- `(1, 2, 3)`: 정점 `1`과 `2`를 가중치 `3`으로 연결합니다.

세 정점이 모두 연결되고 `total = -2 + 3 = 1`입니다. 자기 자신을 잇는 `(2, 2, -100)`은 가중치가 작아도 정점을 추가로 연결하지 못하고 사이클을 만들므로 선택하지 않습니다. 그래프가 연결되지 않은 경우에는 모든 정점을 하나로 연결할 수 없어 각 연결 요소의 트리들을 모은 포리스트가 반환됩니다.

### 예시로 보는 상태 변화 추적

아래 `main()`은 `vertex_count = 5`와 다섯 간선을 `minimum_spanning_forest`에 전달합니다. 간선을 `(from, to, weight)`로 표시하면, 가중치로 정렬된 `edges`는 `[(3, 3, -10), (3, 4, -2), (0, 2, 1), (1, 2, 2), (0, 1, 4)]`입니다.

처음에는 각 정점이 별도 집합이므로 `sets.parent = [0, 1, 2, 3, 4]`, `sets.size = [1, 1, 1, 1, 1]`, `chosen = []`, `total = 0`입니다. `parent`는 각 정점의 부모를 저장하고, `size`는 루트의 집합 크기를 저장합니다.

| 단계 | 검사한 `edge` | 처리 결과 | 처리 후 `sets.parent` | 처리 후 `chosen` | `total` |
| --- | --- | --- | --- | --- | --- |
| 초기 | - | 각 정점이 별도 집합 | `[0, 1, 2, 3, 4]` | `[]` | `0` |
| 1 | `(3, 3, -10)` | 자기 자신을 잇는 간선이므로 건너뜀 | `[0, 1, 2, 3, 4]` | `[]` | `0` |
| 2 | `(3, 4, -2)` | 집합 `{3}`과 `{4}`를 합침 | `[0, 1, 2, 3, 3]` | `[(3, 4, -2)]` | `-2` |
| 3 | `(0, 2, 1)` | 집합 `{0}`과 `{2}`를 합침 | `[0, 1, 0, 3, 3]` | `[(3, 4, -2), (0, 2, 1)]` | `-1` |
| 4 | `(1, 2, 2)` | `{1}`을 더 큰 집합 `{0, 2}`에 합침 | `[0, 0, 0, 3, 3]` | `[(3, 4, -2), (0, 2, 1), (1, 2, 2)]` | `1` |
| 5 | `(0, 1, 4)` | 두 정점이 이미 같은 집합에 있어 건너뜀 | `[0, 0, 0, 3, 3]` | `[(3, 4, -2), (0, 2, 1), (1, 2, 2)]` | `1` |

#### 단계별 변수 값 자세히 보기

`sets.size`는 루트의 값만 현재 집합 크기로 사용합니다. 루트가 아닌 정점의 `size` 값은 합치기 전의 값이 남을 수 있습니다. 아래 `chosen`의 간선도 표와 같이 `(from, to, weight)`로 줄여 표시합니다.

- **초기화 및 정렬**
  - `vertex_count = 5`이고 모든 간선의 끝점이 `0..5` 범위에 있어 검사를 통과합니다.
  - `edges.sort_by_key(|edge| edge.weight)` 실행 후 가중치는 `-10, -2, 1, 2, 4` 순서입니다.
  - `sets.parent = [0, 1, 2, 3, 4]`, `sets.size = [1, 1, 1, 1, 1]`, `chosen = []`, `total = 0`입니다.
- **1단계: `edge = (3, 3, -10)`**
  - `edge.from == edge.to`가 참이므로 `unite`를 호출하지 않고 `continue`합니다.
  - `sets.parent`, `sets.size`, `chosen`, `total`은 모두 초기 상태 그대로입니다.
- **2단계: `edge = (3, 4, -2)`**
  - `unite(3, 4)`에서 `find(3) = 3`, `find(4) = 4`이므로 `left_root = 3`, `right_root = 4`입니다.
  - `size[3] = 1`, `size[4] = 1`이므로 루트를 교환하지 않습니다. `parent[4] = 3`, `size[3] = 2`로 바꾸고 `true`를 반환합니다.
  - `sets.parent = [0, 1, 2, 3, 3]`, `sets.size = [1, 1, 1, 2, 1]`이 됩니다.
  - `total = 0 + (-2) = -2`, `chosen = [(3, 4, -2)]`입니다.
- **3단계: `edge = (0, 2, 1)`**
  - `unite(0, 2)`에서 `left_root = find(0) = 0`, `right_root = find(2) = 2`입니다.
  - 두 루트의 크기가 모두 `1`이므로 교환하지 않고 `parent[2] = 0`, `size[0] = 2`로 바꿉니다.
  - `sets.parent = [0, 1, 0, 3, 3]`, `sets.size = [2, 1, 1, 2, 1]`입니다.
  - `total = -2 + 1 = -1`, `chosen = [(3, 4, -2), (0, 2, 1)]`입니다.
- **4단계: `edge = (1, 2, 2)`**
  - `unite(1, 2)`에서 `find(1) = 1`, `find(2) = 0`입니다. `find(2)`는 `parent[2] = 0`을 따라 루트 `0`을 찾고 경로 압축을 적용하지만, `parent[2]`가 이미 `0`이어서 배열은 변하지 않습니다.
  - 처음에는 `left_root = 1`, `right_root = 0`이고 각각의 크기는 `1`, `2`입니다. 더 큰 집합이 왼쪽 루트가 되도록 교환하면 `left_root = 0`, `right_root = 1`입니다.
  - `parent[1] = 0`, `size[0] = 3`으로 바꾸어 `sets.parent = [0, 0, 0, 3, 3]`, `sets.size = [3, 1, 1, 2, 1]`이 됩니다.
  - `total = -1 + 2 = 1`, `chosen = [(3, 4, -2), (0, 2, 1), (1, 2, 2)]`입니다.
- **5단계: `edge = (0, 1, 4)`**
  - `unite(0, 1)`에서 `find(0) = 0`, `find(1) = 0`이므로 `left_root = right_root = 0`입니다. 경로 압축 후에도 `parent[1]`은 `0`입니다.
  - 이미 같은 집합에 있어 `unite`가 `false`를 반환합니다. 간선을 추가하지 않으므로 `sets.parent = [0, 0, 0, 3, 3]`, `sets.size = [3, 1, 1, 2, 1]`, `chosen`은 4단계와 같고 `total = 1`입니다.

최종 집합은 `{0, 1, 2}`와 `{3, 4}`입니다. 두 집합을 잇는 간선이 없어 하나의 트리가 아닌 최소 신장 포리스트를 반환합니다. `minimum_spanning_forest`의 결과는 선택된 세 간선과 총 가중치 `1`을 담은 `Ok((chosen, 1))`입니다.

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
