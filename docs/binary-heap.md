# BinaryHeap

## 구조 다이어그램

![부모가 자식보다 크거나 같은 BinaryHeap 최대 힙](/assets/binary-heap.svg)

그림은 `BinaryHeap`의 개념적 최대 힙 규칙인 부모가 자식보다 크거나 같다는 조건을 보여 줍니다. 트리와 배열 인덱스의 대응은 설명을 위한 것이며, 실제 컬렉션의 메모리 표현 전체를 뜻하지는 않습니다.

### 그림 단계별 읽기

- 트리의 값은 루트 `90`, 그 아래 `70`, `60`, 마지막 줄 `40`, `50`, `30`, `20`이며 모든 부모가 자식보다 크거나 같습니다.
- 루트 `90`이 최댓값이므로 최대 힙에서 우선순위가 가장 높은 원소가 루트에 놓인다는 핵심 전제를 보여 줍니다.
- 배열은 인덱스 `0`부터 `6`까지 `[90, 70, 60, 40, 50, 30, 20]`으로 트리의 완전 이진 구조를 나타냅니다.
- 0부터 시작하는 배열에서 부모 인덱스 `i`의 자식은 `2i+1`, `2i+2`라는 대응을 그림이 설명합니다.
- 힙 조건은 전체 배열이 오름차순이라는 뜻이 아니며, 그림의 배열과 트리는 개념적 대응이므로 순회 순서까지 보장하지 않습니다.


`BinaryHeap<T>`는 최대 힙이다. 가장 큰 원소를 빠르게 꺼내야 할 때 사용하며, `peek`은 최댓값을 빌리고 `pop`은 최댓값을 제거한다. 최소 힙이 필요하면 `std::cmp::Reverse<T>`로 원소를 감싼다.

## 복잡도와 주의점

- `push`는 힙을 복구하는 데 최대 O(log n)이 걸리며, 용량이 부족해 재할당하면 한 번의 호출이 O(n)까지 걸릴 수 있다.
- `pop`은 O(log n)이다.
- `peek`은 O(1)이다.
- 임의 원소의 검색은 힙의 우선순위를 이용하지 못하므로 O(n)이다.
- 순회 순서는 정렬 순서가 아니다. 전체 정렬이 필요하면 `sort`를 사용한다.
- 힙에 저장된 원소의 순서 비교 결과를 바꾸면 안 된다. `Ord`가 논리적으로 변경되면 힙 불변성이 깨질 수 있다.
- `pop`과 `peek`은 힙이 비었을 때 `None`을 반환한다.

## 완전한 예제

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut max_heap = BinaryHeap::from([4, 1, 7]);
    println!("largest = {:?}", max_heap.peek());
    println!("next = {:?}", max_heap.pop());

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(4));
    min_heap.push(Reverse(1));
    println!("smallest = {:?}", min_heap.pop().map(|value| value.0));
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    #[test]
    fn max_heap_returns_largest_first() {
        let mut heap = BinaryHeap::from([3, 9, 1]);
        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn reverse_makes_a_min_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(3));
        heap.push(Reverse(1));
        assert_eq!(heap.pop(), Some(Reverse(1)));
    }
}
```

## 대표 활용처

[`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)은 입력 순서가 아니라 우선순위가 가장 높은 항목을 먼저 처리하는 우선순위 큐에 적합하다. Rust의 기본 구현은 최대 힙이며, [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)로 감싸면 최소 힙으로 사용할 수 있다.

- 다익스트라 최단 경로: 현재까지의 누적 비용이 가장 작은 노드부터 꺼내 인접 간선을 완화한다. `Reverse((비용, 노드))`는 비용이 같을 때 노드의 순서로도 비교한다. 음의 가중치가 있는 그래프에는 다익스트라를 사용하면 안 된다.
- 스트리밍 상위·하위 K개: 상위 K개를 찾을 때 최소 힙에 후보만 보관하고 가장 작은 후보를 제거한다. 반대로 하위 K개는 최대 힙을 사용할 수 있다. 입력 전체를 저장하고 정렬하지 않아도 된다.
- 이벤트 시뮬레이션과 스케줄러: 가장 이른 만료 시각이나 가장 높은 중요도의 작업부터 처리한다.
- K-way 병합: 정렬된 K개 목록이나 스트림에서 각 목록의 선두 원소를 힙에 넣고, 하나를 꺼낼 때마다 해당 목록의 다음 원소를 넣는다. 각 추출과 삽입은 `O(log K)`이며 전체 원소가 `N`개일 때 보통 `O(N log K)`에 병합한다.

## 상위 K개 점수 추출

`Reverse`로 만든 최소 힙의 루트는 현재 후보 중 가장 작은 점수다. 새 점수를 넣은 후 K개를 초과하면 그 점수를 제거하므로 상위 K개만 남는다. 점수 `n`개에 대한 시간은 `O(n log(K + 1))`, 추가 공간은 `O(min(n, K))`이다. 삽입 직후 잠시 K + 1개가 들어갈 수 있다. `K = 0`이면 빈 결과, `K`가 입력 길이 이상이면 모든 점수를 반환한다.

`into_iter`의 결과 순서는 정렬 순서가 아니다. 정렬된 출력이 필요하면 반환 벡터를 별도로 정렬해야 하며, 그 비용도 고려해야 한다.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn top_k_scores(scores: &[i32], k: usize) -> Vec<i32> {
    if k == 0 {
        return Vec::new();
    }

    // 루트에는 현재 상위 K개 중 가장 작은 점수가 위치한다.
    let mut min_heap = BinaryHeap::new();

    for &score in scores {
        min_heap.push(Reverse(score));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }

    min_heap.into_iter().map(|Reverse(s)| s).collect()
}

fn main() {
    let data = [42, 7, 99, 15, 88, 23, 64];
    let top3 = top_k_scores(&data, 3);
    println!("상위 3개 (순서 미보장): {:?}", top3);
}

#[cfg(test)]
mod tests {
    use super::top_k_scores;

    fn sorted_top_k(scores: &[i32], k: usize) -> Vec<i32> {
        let mut result = top_k_scores(scores, k);
        result.sort_unstable();
        result
    }

    #[test]
    fn keeps_the_largest_scores_without_assuming_heap_order() {
        assert_eq!(sorted_top_k(&[42, 7, 99, 15, 88, 23, 64], 3), vec![64, 88, 99]);
    }

    #[test]
    fn handles_zero_empty_and_oversized_k() {
        assert!(top_k_scores(&[1, 2], 0).is_empty());
        assert!(top_k_scores(&[], 3).is_empty());
        assert_eq!(sorted_top_k(&[3, 1, 2], 10), vec![1, 2, 3]);
    }

    #[test]
    fn preserves_duplicates_and_negative_values() {
        assert_eq!(sorted_top_k(&[-5, 2, 2, -1, 0], 3), vec![0, 2, 2]);
        assert_eq!(sorted_top_k(&[-5, -2, -9], 1), vec![-2]);
    }
}
```

## 언제 다른 자료구조가 더 적합할까요?

- 중간 원소의 임의 검색·삭제·우선순위 갱신: `BinaryHeap`은 루트 접근에 최적화되어 있으며 임의 항목을 빠르게 찾아 갱신하는 API를 기본 제공하지 않는다. 요구사항에 맞는 인덱스나 별도 자료구조를 고려한다.
- 정렬 순회나 범위 조회: [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)과 [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)은 키 순서 순회와 `range`를 지원한다.
- 원소의 전순서가 없는 경우: 힙의 원소는 `Ord`가 필요하다. `f64`는 NaN 때문에 `Ord`를 구현하지 않으므로 전순서와 NaN 정책을 명확히 정의한 래퍼나 적절한 정수 표현을 고려한다.

## PowerShell에서 테스트

실행할 Rust 코드 블록 하나를 `binary_heap.rs`로 저장하고 다음을 실행한다. 모든 예제를 한꺼번에 확인하려면 프로젝트 폴더에서 `cargo test`를 실행한다.

- `rustc --test .\binary_heap.rs -o .\binary-heap-tests.exe`
- `.\binary-heap-tests.exe`

일반 실행은 `rustc .\binary_heap.rs -o .\binary-heap.exe; .\binary-heap.exe`로 확인한다.

## 참고 자료

- [Rust 표준 라이브러리 BinaryHeap 문서](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- [Rust 표준 라이브러리 Reverse 문서](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)
- [Rust Book BinaryHeap 관련 표준 컬렉션 개요](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
