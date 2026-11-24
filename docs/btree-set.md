# BTreeSet

## 구조 다이어그램

![정렬된 고유 키를 담는 BTreeSet 트리](/assets/btree-set.svg)

그림은 비교 순서에 따라 키를 정렬하고 중복을 하나로 취급하는 개념적 B-tree 모델입니다. 실제 Rust 구현의 노드 용량이나 메모리 배치를 보장하는 그림이 아니라, 정렬·고유성·범위 검색의 관계를 설명합니다.

### 그림 단계별 읽기

- 루트 노드에는 20과 40이 있고, 두 키는 `Ord` 순서의 경계를 만듭니다.
- 세 자식은 왼쪽부터 20보다 작은 10, 15, 20과 40 사이의 25, 30, 40보다 큰 45, 50을 담습니다.
- 왼쪽 자식부터 루트의 경계 키를 거쳐 오른쪽 자식으로 읽으면 10 → 15 → 20 → 25 → 30 → 40 → 45 → 50 순서가 됩니다.
- 그림의 키는 정렬되어 있고 중복 없이 한 번씩만 나타나므로 `BTreeSet`의 고유 원소와 대응합니다.

`BTreeSet<T>`는 정렬된 순서로 서로 다른 값을 저장하는 표준 라이브러리의 집합 자료구조입니다. 값의 순서는 `Ord` 구현으로 결정되며, 같은 순서로 비교되는 값은 하나만 보관됩니다. 해시가 아니라 비교를 사용하므로 `T: Ord`가 필요합니다.

## 핵심 연산

- `BTreeSet::new` 또는 `BTreeSet::from`으로 집합을 만듭니다.
- `insert`는 값이 새로 추가되었으면 `true`, 이미 같은 값이 있으면 `false`를 반환합니다.
- `contains`, `get`, `remove`로 검색·참조·삭제를 합니다.
- `range`는 정렬 순서의 일부 구간을 순회합니다.
- `union`, `intersection`, `difference`, `symmetric_difference`로 집합 연산을 합니다.
- `first`, `last`, `pop_first`, `pop_last`로 양 끝의 값을 다룰 수 있습니다.

```rust
use std::collections::BTreeSet;

fn main() {
    let mut numbers = BTreeSet::new();
    numbers.insert(3);
    numbers.insert(1);
    numbers.insert(3); // 중복은 한 번만 저장됩니다.

    assert_eq!(numbers.iter().copied().collect::<Vec<_>>(), vec![1, 3]);
    assert!(numbers.contains(&1));
    assert_eq!(numbers.range(1..=2).copied().collect::<Vec<_>>(), vec![1]);

    let even = BTreeSet::from([2, 4]);
    let merged: BTreeSet<_> = numbers.union(&even).copied().collect();
    assert_eq!(merged.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_unique_values_in_order() {
        let mut set = BTreeSet::new();
        assert!(set.insert(2));
        assert!(set.insert(1));
        assert!(!set.insert(2));
        assert_eq!(set.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn supports_range_and_difference() {
        let set = BTreeSet::from([1, 2, 3, 4]);
        assert_eq!(set.range(2..4).copied().collect::<Vec<_>>(), vec![2, 3]);
        let removed = BTreeSet::from([2, 4]);
        let remaining: Vec<_> = set.difference(&removed).copied().collect();
        assert_eq!(remaining, vec![1, 3]);
    }
}
```

## 복잡도

- 검색, 삽입, 삭제는 일반적인 B-tree 높이를 기준으로 `O(log n)`입니다.
- `contains`와 `get`도 일반적으로 `O(log n)`입니다.
- 전체 순회는 `O(n)`이며 정렬된 순서로 결과를 냅니다.
- `range`는 시작 위치를 찾는 비용을 포함해 일반적으로 `O(log n + k)`이며, `k`는 반환된 원소 수입니다.
- 집합 반복자의 복잡도는 두 입력 집합의 크기와 실제 출력에 따라 달라집니다.

위 복잡도는 균형 잡힌 B-tree라는 자료구조 특성에 따른 일반적인 경계입니다. 특정 Rust 버전의 내부 노드 표현이나 메모리 배치는 안정적인 API 계약으로 간주하면 안 됩니다. `BTreeSet`은 해시 테이블이 아니므로 해시 함수가 필요하지 않지만, 비교 순서가 일관되어야 합니다.

## 주의할 점

- `Ord`와 `Eq`의 의미가 서로 맞지 않으면 예상하지 못한 중복 판정이 발생할 수 있습니다. 특히 `cmp(a, b) == Ordering::Equal`인 값은 같은 원소로 취급됩니다.
- 집합에 들어 있는 값을 정렬 순서를 바꾸는 방식으로 변경하면 안 됩니다. Rust의 대여 규칙은 대부분의 문제를 막지만, 내부 가변성이나 전역 상태로 비교 결과를 바꾸면 논리적 오류가 생깁니다.
- 키의 소유권을 빌려 검색하려면 `Borrow` 기반 조회 API와 호환되는 타입을 사용할 수 있습니다. 예를 들어 `BTreeSet<String>`에서 `&str`로 `contains`를 호출할 수 있습니다.
- 순서가 필요 없고 평균적인 키 조회만 필요하면 `HashSet`이 더 적합할 수 있습니다. 반대로 범위 검색, 최솟값·최댓값, 결정적인 순회 순서가 필요하면 `BTreeSet`이 적합합니다.

## PowerShell에서 확인하기

각 Rust 코드 블록을 별도 파일로 저장하면 다음처럼 컴파일하고 테스트할 수 있습니다. 기존 프로그램은 `btree_set.rs`로 저장합니다. 파일 이름의 하이픈 대신 밑줄을 사용하면 `rustc`가 만드는 crate 이름 규칙과 충돌하지 않습니다.

- `rustc --edition 2021 btree_set.rs -o btree_set.exe`
- `./btree_set.exe`
- `rustc --edition 2021 --test btree_set.rs -o btree_set_tests.exe`
- `./btree_set_tests.exe`

## 실무 활용과 선택 기준

`BTreeSet`은 날짜, 버전, 점수처럼 정렬된 고유 값에서 범위 조회, 최솟값·최댓값, 결정적인 순회가 필요한 작업에 적합합니다. 검색·삽입·삭제는 일반적으로 `O(log n)`이고, `range`는 시작 위치 탐색을 포함해 `O(log n + k)`이며 `k`는 반환 원소 수입니다. 순서가 필요 없고 평균적인 존재 확인만 중요하면 `HashSet`이 더 적합할 수 있습니다.

값의 정렬은 `Ord`에 따르며 `cmp`가 `Ordering::Equal`을 반환하는 값은 하나로 취급됩니다. 따라서 `Ord`와 `Eq`의 의미가 일관되어야 하고, 집합에 저장한 값의 비교 결과가 바뀌지 않아야 합니다. 해시 함수는 필요하지 않지만 비교 순서가 안정적이어야 하며, 범위 끝점의 포함 여부는 `start..end`와 `start..=end` 같은 범위 표기로 명확히 선택하세요.

## 추가 예제

다음 예제는 정렬된 점수에서 70점 이상 90점 이하를 범위로 조회합니다.

```rust
use std::collections::BTreeSet;

fn scores_between(scores: &[u32], minimum: u32, maximum: u32) -> Vec<u32> {
    if minimum > maximum {
        return Vec::new();
    }
    let scores: BTreeSet<_> = scores.iter().copied().collect();
    scores.range(minimum..=maximum).copied().collect()
}

fn main() {
    let selected = scores_between(&[60, 75, 75, 90, 95], 70, 90);
    println!("selected scores: {:?}", selected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_is_sorted_and_unique() {
        assert_eq!(scores_between(&[90, 70, 90, 80], 70, 90), vec![70, 80, 90]);
    }

    #[test]
    fn reversed_range_is_empty() {
        assert!(scores_between(&[70, 80], 90, 70).is_empty());
    }
}
```

추가 예제 블록을 `btree_set_additional.rs`로 저장하면 기존 예제와 별도로 `rustc --edition 2021 --test btree_set_additional.rs -o btree_set_additional_tests.exe`와 `./btree_set_additional_tests.exe`로 테스트할 수 있습니다.

## 출처

- [Rust 표준 라이브러리: BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- [Rust 표준 라이브러리: collections 개요](https://doc.rust-lang.org/std/collections/index.html)
- [Rust Reference: 비교 연산자와 Ord 관련 내용](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
