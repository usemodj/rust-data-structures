# 이진 검색 트리

## 구조 다이어그램

![왼쪽은 작고 오른쪽은 큰 이진 검색 트리와 중위 순회](/assets/binary-search-tree.svg)

그림은 각 노드의 왼쪽 키가 더 작고 오른쪽 키가 더 큰 BST의 개념과 중위 순회의 결과를 시각화합니다. 실제 구현은 노드 사이의 연결을 `Option<Box<Node>>`로 소유하고, `insert`와 `contains`가 비교 결과에 따라 왼쪽 또는 오른쪽 자식으로 내려갑니다.

### 그림 단계별 읽기

- 루트는 8이고, 왼쪽 자식 3은 8보다 작으며 오른쪽 자식 12는 8보다 큽니다.
- 3의 자식은 1과 6이고 12의 자식은 10과 15이므로 각 부분 트리에서도 왼쪽 값이 작고 오른쪽 값이 큽니다.
- 중위 순회는 왼쪽 부분 트리, 현재 노드, 오른쪽 부분 트리 순서로 방문합니다.
- 따라서 그림의 중위 순회 순서는 1 → 3 → 6 → 8 → 10 → 12 → 15이며 오름차순입니다.

## 중위 순회와 전위 순회 비교

![같은 이진 검색 트리에서 중위 순회는 1, 3, 6, 8, 10, 12, 15이고 전위 순회는 8, 3, 1, 6, 12, 10, 15인 방문 순서 비교](/assets/bst-traversals.svg)

두 그림은 위의 구조 다이어그램과 같은 트리입니다. 각 노드 옆의 작은 숫자는 방문 순서를 나타냅니다.

| 순회 | 방문 규칙 | 그림의 방문 결과 |
| --- | --- | --- |
| **중위 순회** (in-order) | 왼쪽 → 현재 → 오른쪽 | `1 → 3 → 6 → 8 → 10 → 12 → 15` |
| **전위 순회** (pre-order) | 현재 → 왼쪽 → 오른쪽 | `8 → 3 → 1 → 6 → 12 → 10 → 15` |

- 중위 순회는 루트 `8`을 방문하기 전에 왼쪽 부분 트리 `1, 3, 6`을 먼저 읽고, 이후 오른쪽 부분 트리 `10, 12, 15`를 읽습니다. BST에서는 결과가 키의 오름차순이 됩니다.
- 전위 순회는 루트 `8`을 먼저 방문하고 왼쪽 부분 트리 `3, 1, 6`, 오른쪽 부분 트리 `12, 10, 15` 순으로 방문합니다. 트리의 구조를 순서대로 살펴보거나 복사할 때 유용합니다.
- 두 방법 모두 노드 `n`개를 한 번씩 방문하므로 `O(n)` 시간이 걸립니다. 재귀 구현의 호출 스택은 트리 높이 `h`에 비례해 `O(h)`를 사용합니다.

## 후위 순회와 레벨 순회 비교

![같은 이진 검색 트리에서 후위 순회는 1, 6, 3, 10, 15, 12, 8이고 레벨 순회는 8, 3, 12, 1, 6, 10, 15인 방문 순서 비교](/assets/bst-extra-traversals.svg)

| 순회 | 방문 규칙 | 그림의 방문 결과 |
| --- | --- | --- |
| **후위 순회** (post-order) | 왼쪽 → 오른쪽 → 현재 | `1 → 6 → 3 → 10 → 15 → 12 → 8` |
| **레벨 순회** (level-order) | 위에서 아래로, 같은 레벨은 왼쪽부터 | `8 → 3 → 12 → 1 → 6 → 10 → 15` |

- 후위 순회는 왼쪽 부분 트리 `1, 6, 3`, 오른쪽 부분 트리 `10, 15, 12`를 방문한 뒤 루트 `8`을 방문합니다. 자식부터 처리하므로 트리 삭제나 하위 결과를 먼저 계산하는 작업에 유용합니다.
- 레벨 순회는 루트 `8`, 다음 레벨 `3, 12`, 마지막 레벨 `1, 6, 10, 15`를 차례로 방문합니다. 큐에서 노드를 꺼내 방문하고 왼쪽 자식과 오른쪽 자식을 순서대로 넣습니다.
- 두 순회 모두 `O(n)` 시간이 걸립니다. 후위 순회의 재귀 호출 스택은 트리 높이 `h`에 비례해 `O(h)`, 레벨 순회의 큐는 최대 너비 `w`에 비례해 `O(w)` 공간을 사용합니다. 결과 벡터 공간은 별도로 `O(n)`입니다.

## 네 가지 순회 단계별 따라가기

위 다이어그램의 같은 트리를 사용합니다. 중위·전위·후위 순회는 왼쪽 부분 트리를 오른쪽보다 먼저 살펴보는 깊이 우선 순회이며, 현재 노드를 방문하는 시점이 다릅니다.

### 중위 순회: 왼쪽 → 현재 → 오른쪽

| 단계 | 방문 | 설명 |
| --- | --- | --- |
| 1 | 1 | 8과 3의 왼쪽으로 계속 내려갑니다. |
| 2 | 3 | 왼쪽을 마쳤으므로 3을 방문합니다. |
| 3 | 6 | 3의 오른쪽을 방문합니다. |
| 4 | 8 | 왼쪽 부분 트리를 마치고 루트를 방문합니다. |
| 5 | 10 | 오른쪽 부분 트리에서 12의 왼쪽을 방문합니다. |
| 6 | 12 | 왼쪽을 마쳤으므로 12를 방문합니다. |
| 7 | 15 | 12의 오른쪽을 방문합니다. |

### 전위 순회: 현재 → 왼쪽 → 오른쪽

| 단계 | 방문 | 설명 |
| --- | --- | --- |
| 1 | 8 | 루트를 가장 먼저 방문합니다. |
| 2 | 3 | 8의 왼쪽 자식을 방문합니다. |
| 3 | 1 | 3의 왼쪽 자식을 방문합니다. |
| 4 | 6 | 3으로 돌아와 오른쪽 자식을 방문합니다. |
| 5 | 12 | 8로 돌아와 오른쪽 자식을 방문합니다. |
| 6 | 10 | 12의 왼쪽 자식을 방문합니다. |
| 7 | 15 | 12의 오른쪽 자식을 방문합니다. |

### 후위 순회: 왼쪽 → 오른쪽 → 현재

| 단계 | 방문 | 설명 |
| --- | --- | --- |
| 1 | 1 | 3의 왼쪽 자식을 먼저 방문합니다. |
| 2 | 6 | 3의 오른쪽 자식을 방문합니다. |
| 3 | 3 | 두 자식을 마친 뒤 3을 방문합니다. |
| 4 | 10 | 12의 왼쪽 자식을 방문합니다. |
| 5 | 15 | 12의 오른쪽 자식을 방문합니다. |
| 6 | 12 | 두 자식을 마친 뒤 12를 방문합니다. |
| 7 | 8 | 양쪽 부분 트리를 마친 뒤 루트를 방문합니다. |

전위는 부모 먼저, 중위는 자식 사이, 후위는 부모 마지막에 방문한다고 기억하면 편합니다.

### 재귀 호출과 반환 과정

세 깊이 우선 순회는 같은 순서로 왼쪽과 오른쪽 자식을 재귀 호출합니다. 차이는 **방문을 어느 위치에서 하느냐**입니다.

```text
순회(노드):
    노드가 비어 있으면 반환
    [전위 방문]
    순회(왼쪽 자식)
    [중위 방문]
    순회(오른쪽 자식)
    [후위 방문]
    반환
```

각 순회는 자신의 방문 위치 하나만 실행합니다. 아래 표는 같은 트리를 순회할 때의 공통 호출 흐름을 보여 줍니다. 스택은 **왼쪽이 먼저 호출된 노드, 오른쪽이 현재 노드**이며, 반환 행에서는 해당 노드가 빠진 직후 상태입니다. 빈 자식에 대한 호출과 즉시 반환은 간결하게 생략했습니다. `—`는 해당 시점에 그 순회가 방문하지 않는다는 뜻입니다.

| 동작 | 동작 후 호출 스택 | 전위 방문 | 중위 방문 | 후위 방문 |
| --- | --- | --- | --- | --- |
| 8 호출 | `8` | 8 | — | — |
| 3 호출 | `8 → 3` | 3 | — | — |
| 1 호출 | `8 → 3 → 1` | 1 | — | — |
| 1의 왼쪽 완료 | `8 → 3 → 1` | — | 1 | — |
| 1 반환 | `8 → 3` | — | — | 1 |
| 3의 왼쪽 완료 | `8 → 3` | — | 3 | — |
| 6 호출 | `8 → 3 → 6` | 6 | — | — |
| 6의 왼쪽 완료 | `8 → 3 → 6` | — | 6 | — |
| 6 반환 | `8 → 3` | — | — | 6 |
| 3 반환 | `8` | — | — | 3 |
| 8의 왼쪽 완료 | `8` | — | 8 | — |
| 12 호출 | `8 → 12` | 12 | — | — |
| 10 호출 | `8 → 12 → 10` | 10 | — | — |
| 10의 왼쪽 완료 | `8 → 12 → 10` | — | 10 | — |
| 10 반환 | `8 → 12` | — | — | 10 |
| 12의 왼쪽 완료 | `8 → 12` | — | 12 | — |
| 15 호출 | `8 → 12 → 15` | 15 | — | — |
| 15의 왼쪽 완료 | `8 → 12 → 15` | — | 15 | — |
| 15 반환 | `8 → 12` | — | — | 15 |
| 12 반환 | `8` | — | — | 12 |
| 8 반환 | `비어 있음` | — | — | 8 |

각 방문 열을 위에서 아래로 읽으면 앞의 전위·중위·후위 결과와 같습니다. 후위 방문은 실제로 반환 **직전**에 일어나며, 표의 스택은 그 반환이 끝난 뒤를 보여 줍니다. 재귀 호출 스택의 최대 크기는 트리 높이 `h`에 비례해 `O(h)`입니다.

### 레벨 순회: 큐의 변화

큐의 **왼쪽이 먼저 꺼내는 쪽**입니다. 앞에서 노드를 꺼내 방문한 뒤, 자식이 있으면 왼쪽부터 큐 뒤에 넣습니다.

| 단계 | 방문한 노드 | 큐에 추가한 자식 | 처리 후 큐 |
| --- | --- | --- | --- |
| 시작 | — | 루트 8 | `[8]` |
| 1 | 8 | 3, 12 | `[3, 12]` |
| 2 | 3 | 1, 6 | `[12, 1, 6]` |
| 3 | 12 | 10, 15 | `[1, 6, 10, 15]` |
| 4 | 1 | 없음 | `[6, 10, 15]` |
| 5 | 6 | 없음 | `[10, 15]` |
| 6 | 10 | 없음 | `[15]` |
| 7 | 15 | 없음 | `[]` |

큐가 비면 순회가 끝납니다. 결과는 `8 → 3 → 12 → 1 → 6 → 10 → 15`입니다. 같은 레벨의 노드가 그 자식보다 먼저 큐에 들어가므로 층별로 방문합니다.

아래 Rust 코드 예제는 설명을 위해 다른 키 집합을 사용하므로, 실행 결과의 숫자는 이 그림과 다릅니다.

이진 검색 트리(binary search tree, BST)는 각 노드의 왼쪽 부분 트리에 더 작은 키를, 오른쪽 부분 트리에 더 큰 키를 배치하는 이진 트리입니다. 이 문서의 구현은 키 중복을 허용하지 않으며, 이미 존재하는 값은 삽입하지 않습니다. Rust에서 `Option<Box<Node>>`를 사용하면 빈 자식은 `None`, 소유권을 가진 자식은 `Some(Box<Node>)`로 표현할 수 있습니다.

## 핵심 연산

삽입과 검색은 현재 노드와 키를 비교하면서 한 방향으로 내려갑니다. 중위 순회(in-order)는 왼쪽, 현재 노드, 오른쪽 순서로 방문하므로 BST에서는 항상 오름차순 결과를 만듭니다. 전위 순회(pre-order)는 현재 노드를 먼저 방문하므로 트리 구조를 살펴볼 때 유용합니다. 빈 트리에 삽입하면 새 루트가 되고, 빈 트리에서 검색하면 즉시 `false`를 반환합니다.

이 예제의 `insert`는 중복을 무시하고, `contains`는 재귀적으로 키를 찾습니다. `inorder`, `preorder`, `postorder`, `level_order`는 방문 결과를 호출자가 전달한 벡터에 추가합니다. 재귀 깊이는 트리 높이에 비례하므로, 극단적으로 편향된 입력을 아주 크게 처리할 때는 반복 구현이나 균형 트리를 고려해야 합니다.

```rust
use std::collections::VecDeque;

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: i32) {
        fn insert_at(node: &mut Option<Box<Node>>, value: i32) {
            match node {
                None => {
                    *node = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                }
                Some(current) if value < current.value => insert_at(&mut current.left, value),
                Some(current) if value > current.value => insert_at(&mut current.right, value),
                Some(_) => {} // 중복 키는 하나만 보관한다.
            }
        }

        insert_at(&mut self.root, value);
    }

    fn contains(&self, value: i32) -> bool {
        fn contains_at(node: &Option<Box<Node>>, value: i32) -> bool {
            match node {
                None => false,
                Some(current) if value == current.value => true,
                Some(current) if value < current.value => contains_at(&current.left, value),
                Some(current) => contains_at(&current.right, value),
            }
        }

        contains_at(&self.root, value)
    }

    fn inorder(&self, output: &mut Vec<i32>) {
        fn visit(node: &Option<Box<Node>>, output: &mut Vec<i32>) {
            if let Some(current) = node {
                visit(&current.left, output);
                output.push(current.value);
                visit(&current.right, output);
            }
        }

        visit(&self.root, output);
    }

    fn preorder(&self, output: &mut Vec<i32>) {
        fn visit(node: &Option<Box<Node>>, output: &mut Vec<i32>) {
            if let Some(current) = node {
                output.push(current.value);
                visit(&current.left, output);
                visit(&current.right, output);
            }
        }

        visit(&self.root, output);
    }

    fn postorder(&self, output: &mut Vec<i32>) {
        fn visit(node: &Option<Box<Node>>, output: &mut Vec<i32>) {
            if let Some(current) = node {
                visit(&current.left, output);
                visit(&current.right, output);
                output.push(current.value);
            }
        }

        visit(&self.root, output);
    }

    fn level_order(&self, output: &mut Vec<i32>) {
        let mut queue = VecDeque::new();
        if let Some(root) = self.root.as_deref() {
            queue.push_back(root);
        }
        while let Some(current) = queue.pop_front() {
            output.push(current.value);
            if let Some(left) = current.left.as_deref() {
                queue.push_back(left);
            }
            if let Some(right) = current.right.as_deref() {
                queue.push_back(right);
            }
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::new();
    for value in [8, 3, 10, 1, 6, 14, 4, 7, 3] {
        tree.insert(value);
    }

    let mut sorted = Vec::new();
    tree.inorder(&mut sorted);
    println!("중위 순회: {sorted:?}");
    println!("6 포함 여부: {}", tree.contains(6));

    let mut shape = Vec::new();
    tree.preorder(&mut shape);
    println!("전위 순회: {shape:?}");

    let mut postorder = Vec::new();
    tree.postorder(&mut postorder);
    println!("후위 순회: {postorder:?}");

    let mut level_order = Vec::new();
    tree.level_order(&mut level_order);
    println!("레벨 순회: {level_order:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_is_safe() {
        let tree = BinarySearchTree::new();
        let mut values = Vec::new();
        tree.inorder(&mut values);
        tree.postorder(&mut values);
        tree.level_order(&mut values);
        assert_eq!(values, []);
        assert!(!tree.contains(1));
    }

    #[test]
    fn insertion_search_and_traversals_work() {
        let mut tree = BinarySearchTree::new();
        for value in [8, 3, 10, 1, 6, 14, 4, 7] {
            tree.insert(value);
        }

        let mut inorder = Vec::new();
        tree.inorder(&mut inorder);
        assert_eq!(inorder, [1, 3, 4, 6, 7, 8, 10, 14]);
        assert!(tree.contains(1));
        assert!(tree.contains(14));
        assert!(!tree.contains(5));

        let mut preorder = Vec::new();
        tree.preorder(&mut preorder);
        assert_eq!(preorder, [8, 3, 1, 6, 4, 7, 10, 14]);
    }

    #[test]
    fn traversals_match_diagram_example() {
        let mut tree = BinarySearchTree::new();
        for value in [8, 3, 12, 1, 6, 10, 15] {
            tree.insert(value);
        }
        let mut inorder = Vec::new();
        let mut preorder = Vec::new();
        tree.inorder(&mut inorder);
        tree.preorder(&mut preorder);
        assert_eq!(inorder, [1, 3, 6, 8, 10, 12, 15]);
        assert_eq!(preorder, [8, 3, 1, 6, 12, 10, 15]);
        let mut postorder = Vec::new();
        let mut level_order = Vec::new();
        tree.postorder(&mut postorder);
        tree.level_order(&mut level_order);
        assert_eq!(postorder, [1, 6, 3, 10, 15, 12, 8]);
        assert_eq!(level_order, [8, 3, 12, 1, 6, 10, 15]);
    }

    #[test]
    fn extra_traversals_handle_skewed_and_single_node_trees() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        let mut postorder = vec![99];
        let mut level_order = vec![99];
        tree.postorder(&mut postorder);
        tree.level_order(&mut level_order);
        assert_eq!(postorder, [99, 1]);
        assert_eq!(level_order, [99, 1]);

        tree.insert(2);
        tree.insert(3);
        postorder.clear();
        level_order.clear();
        tree.postorder(&mut postorder);
        tree.level_order(&mut level_order);
        assert_eq!(postorder, [3, 2, 1]);
        assert_eq!(level_order, [1, 2, 3]);
    }

    #[test]
    fn duplicates_do_not_change_contents() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(5);
        tree.insert(5);
        let mut values = Vec::new();
        tree.inorder(&mut values);
        assert_eq!(values, [5]);
    }
}
```

## 복잡도와 주의점

높이를 `h`(루트만 있는 트리의 높이를 1로 보는 노드 수 기준)라고 하면 삽입, 검색은 `O(h)`, 순회는 모든 노드를 방문하므로 `O(n)`입니다. 균형이 잘 잡힌 경우 `h = O(log n)`이어서 삽입과 검색이 `O(log n)`이지만, 이미 정렬된 값을 순서대로 넣으면 한 줄로 기울어져 `h = O(n)`이고 두 연산도 `O(n)`이 됩니다. 순회 결과를 저장하는 벡터는 방문 결과 크기에 비례합니다. 깊이 우선 순회의 재귀 호출 스택은 높이에, 레벨 순회의 큐는 최대 너비에 비례합니다.

중복 키 정책은 자료구조의 계약입니다. 여기서는 중복을 버리므로 개수 세기나 동일 키의 여러 레코드가 필요하면 `(키, 개수)`를 저장하거나 중복을 한쪽 방향에 모으는 별도 정책이 필요합니다. 또한 일반 BST는 자동으로 균형을 유지하지 않습니다. 최악의 높이를 피하려면 AVL 트리, 레드-블랙 트리 또는 Rust 표준 라이브러리의 `BTreeSet`·`BTreeMap` 같은 균형 자료구조를 선택할 수 있습니다.

## 참고 자료

- [DSAR Chapter 12: Binary Search Trees](https://dsar.rantai.dev/docs/part-iii/chapter-12/)
- [Rust Book: `Box<T>`](https://doc.rust-lang.org/book/ch15-01-box.html)

예제는 `binary_search_tree.rs`로 저장한 뒤 PowerShell에서 다음처럼 확인할 수 있습니다.

```powershell
rustc binary_search_tree.rs -o binary_search_tree.exe
.\binary_search_tree.exe
rustc --test binary_search_tree.rs -o binary_search_tree_tests.exe
.\binary_search_tree_tests.exe
```
