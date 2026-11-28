# 서로소 집합과 Union-Find

## 구조 다이어그램

![서로소 집합과 경로 압축을 보여 주는 구조 다이어그램](/assets/union-find.svg)

그림은 부모 포인터로 연결된 두 개의 서로소 집합과 `find`가 경로를 루트에 가깝게 줄이는 개념을 단순화해 보여 줍니다. 실제 구현은 그림의 노드와 화살표를 `parent` 및 `size` 벡터로 관리하며, `union`의 크기 기준 결합과 `find`의 경로 압축을 코드로 수행합니다.

### 그림 단계별 읽기

- 집합 A에서는 0, 2, 3이 부모 포인터로 루트 1을 가리키므로 대표가 1인 하나의 집합을 이룹니다.
- 집합 B에서는 4가 5를 가리키고 5가 루트 6을 가리켜 `find(4)` 전 경로가 4 → 5 → 6입니다.
- `find(4)`를 수행하면 경로 압축으로 4가 6을 직접 가리키는 4 → 6 경로가 됩니다.
- A와 B 사이에는 부모 포인터가 없으므로 두 집합은 서로 연결되지 않고, 각 대표는 각각 1과 6입니다.

서로소 집합(disjoint set)은 원소를 겹치지 않는 여러 그룹으로 나누어 관리하는 자료구조입니다. Union-Find 또는 DSU(Disjoint Set Union)라고도 부릅니다. 대표적인 연산은 원소가 속한 그룹의 대표를 찾는 `find`와 두 그룹을 합치는 `union`입니다.

## 표현과 연산

각 원소를 부모 배열의 인덱스로 표현합니다. 루트는 자기 자신을 부모로 가리키고, 루트가 그룹의 대표입니다. 두 가지 최적화를 함께 사용합니다.

- 경로 압축(path compression): `find`가 루트까지 올라간 뒤 방문한 모든 노드가 루트를 직접 가리키게 합니다.
- 랭크 또는 크기에 의한 합치기(union by rank/size): 두 루트를 합칠 때 낮은 트리를 높은 트리 밑에 붙입니다. 랭크는 높이의 상한이고, 크기는 집합의 원소 수입니다.
- 두 최적화를 함께 사용하면 연산 `m`회의 총 비용은 `O(m α(n))`이며, `α`는 매우 느리게 증가하는 역 아커만 함수입니다. 따라서 한 연산의 상각 비용은 사실상 상수에 가깝지만, 모든 개별 호출이 엄밀히 `O(1)`이라는 뜻은 아닙니다.

### 함수 설명

- **`DisjointSet::new(n) -> Self`:** `0..n` 범위의 원소를 각각 독립된 집합으로 초기화합니다. `parent[x] = x`, `size[x] = 1`로 시작합니다.
- **`find(&mut self, x) -> usize`:** 원소 `x`가 속한 집합의 대표 정점(루트)을 반환합니다. 부모를 따라 루트를 찾고 방문한 원소가 루트를 직접 가리키도록 경로를 압축합니다. `x`가 범위를 벗어나면 패닉이 발생합니다.
- **`union(&mut self, a, b) -> bool`:** `find`로 두 원소의 루트를 찾고, 서로 다른 집합이면 작은 집합을 큰 집합에 연결한 뒤 `true`를 반환합니다. 이미 같은 집합이면 `false`를 반환합니다. 루트의 크기는 `size`에 갱신하며, 잘못된 인덱스는 `find`에서 패닉을 발생시킵니다.
- **`connected(&mut self, a, b) -> bool`:** 두 원소의 루트가 같은지 비교해 같은 집합이면 `true`, 아니면 `false`를 반환합니다. 내부에서 `find`를 호출하므로 경로 압축으로 `parent`가 변경될 수 있으며, 잘못된 인덱스는 패닉을 발생시킵니다.
- **`main()`:** 다섯 원소를 생성한 뒤 `0`, `1`, `2`를 한 집합으로, `3`, `4`를 다른 집합으로 합칩니다. `assert!`로 `0`과 `2`는 연결되어 있고 `0`과 `3`은 연결되지 않았는지 확인합니다.

```rust
#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        assert!(x < self.parent.len(), "element index out of bounds");
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root; // 경로 압축
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return false;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a; // 작은 집합을 큰 집합에 연결
        self.size[root_a] += self.size[root_b];
        true
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() {
    let mut groups = DisjointSet::new(5);
    groups.union(0, 1);
    groups.union(1, 2);
    groups.union(3, 4);
    assert!(groups.connected(0, 2));
    assert!(!groups.connected(0, 3));
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn joins_groups_and_detects_redundant_union() {
        let mut dsu = DisjointSet::new(4);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert!(!dsu.union(0, 2));
        assert!(dsu.connected(0, 2));
        assert!(!dsu.connected(0, 3));
    }

    #[test]
    fn isolated_element_is_its_own_group() {
        let mut dsu = DisjointSet::new(2);
        assert!(dsu.connected(1, 1));
        assert!(!dsu.connected(0, 1));
    }
}
```

## 복잡도와 선택

- `n`개 원소를 초기화하는 비용은 `O(n)`이고 추가 공간도 `O(n)`입니다.
- 경로 압축과 크기 기준 합치기를 함께 쓰면 `m`번의 `find`·`union` 시퀀스가 `O(m α(n))`입니다.
- 랭크 기준 합치기는 각 루트의 랭크를 유지하고, 크기 기준 합치기는 각 루트의 원소 수를 유지합니다. 둘 중 하나만 써도 트리 높이를 제어하지만, 두 최적화를 함께 쓰는 것이 표준적인 선택입니다.
- 이 자료구조는 그룹 병합에는 강하지만 그룹 분할, 삭제, 과거 상태로의 되돌리기를 직접 지원하지 않습니다. 그런 요구에는 별도의 오프라인 처리, 롤백 DSU, 또는 다른 자료구조가 필요합니다.

## 주의할 점

- 원소는 보통 `0..n` 범위의 정수 ID로 미리 등록합니다. 이 예제는 잘못된 인덱스를 패닉으로 처리하므로 외부 입력은 호출 전에 검증해야 합니다.
- `find`가 부모 배열을 수정하므로 Rust에서는 `&mut self`가 필요합니다. 동시에 여러 개의 가변 참조로 `find`를 호출할 수 없습니다.
- `union`의 반환값은 두 그룹이 실제로 합쳐졌는지를 나타냅니다. 이미 연결된 두 원소를 합치면 `false`입니다.
- 연결성만 필요하면 대표의 ID 자체를 의미 있는 값으로 해석하지 마세요. 어떤 루트가 대표가 되는지는 합치기 순서와 최적화에 따라 달라질 수 있습니다.
- DSU는 연결 여부를 빠르게 유지하지만, 각 그룹의 모든 원소를 순회하거나 그룹별 순서를 유지하는 기능은 제공하지 않습니다.

## PowerShell에서 확인하기

프로그램을 `union_find.rs`로 저장한 뒤 다음 명령을 실행할 수 있습니다. 하이픈이 아닌 밑줄 파일명을 사용해야 `rustc` crate 이름 규칙과 자연스럽게 맞습니다.

- `rustc --edition 2021 union_find.rs -o union_find.exe`
- `./union_find.exe`
- `rustc --edition 2021 --test union_find.rs -o union_find_tests.exe`
- `./union_find_tests.exe`

## 출처

- [cp-algorithms: Disjoint Set Union](https://cp-algorithms.com/data_structures/disjoint_set_union.html)
- [Robert E. Tarjan, 효율적인 집합 합치기 알고리즘](https://doi.org/10.1145/321879.321884)
- [Princeton Algorithms: Union-Find](https://algs4.cs.princeton.edu/15uf/)
