# BTreeMap

`BTreeMap<K, V>`는 키를 정렬된 순서로 저장하는 맵이다. 키로 값을 찾고, 범위 검색이나 정렬된 순회가 필요할 때 유용하다. 같은 키를 다시 `insert`하면 이전 값이 반환되며 새 값으로 교체된다.

## B-트리 정렬 구조 다이어그램

![B-트리 개념도: 루트의 20과 40을 기준으로 자식 키가 나뉘고 10, 15, 20, 25, 30, 40, 45, 50 순서로 순회한다.](/assets/btree-map.svg)

루트 노드의 키 `20`, `40`을 기준으로 왼쪽에는 더 작은 키, 가운데에는 두 키 사이의 값, 오른쪽에는 더 큰 키가 있다. 각 키에는 `A`부터 `H`까지의 값이 연결되어 있으며, 맵을 순회하면 자식 노드와 부모 노드의 키가 함께 오름차순으로 출력된다.

이 그림은 구조를 이해하기 위한 개념도다. 실제 `BTreeMap`의 노드 크기와 키 배치는 구현과 삽입·삭제 이력에 따라 달라지며, 작은 맵은 루트 노드 하나만 가질 수도 있다. 보장되는 것은 키의 `Ord` 순서에 따른 순회다.

### 그림 단계별 읽기

- 루트 노드에는 `20 : C`, `40 : F`가 정렬되어 있고, 이 두 키가 자식 노드를 세 구간으로 나누는 기준입니다.
- 왼쪽 자식 노드의 `10 : A`, `15 : B`는 `20`보다 작고, 가운데의 `25 : D`, `30 : E`는 `20`과 `40` 사이입니다.
- 오른쪽 자식 노드의 `45 : G`, `50 : H`는 `40`보다 크며, 이 구간 규칙이 키를 찾아가는 핵심 전제입니다.
- 그림의 순회 결과는 `10 → 15 → 20 → 25 → 30 → 40 → 45 → 50`으로 부모와 자식 노드의 키를 함께 오름차순으로 읽습니다.
- 실제 노드 크기와 배치는 구현 및 변경 이력에 따라 달라질 수 있으므로, 이 그림은 특정 내부 배치를 보장하지 않고 `Ord`에 따른 오름차순 순회 개념만 설명합니다.

## 복잡도와 주의점

- `get`, `insert`, `remove`는 O(log n)이다.
- `iter`는 키 오름차순으로 순회하며 전체 순회는 O(n)이다.
- `range`는 범위의 시작 위치를 찾는 비용과 반환 원소 수를 합쳐 O(log n + k)로 볼 수 있다.
- 키는 `Ord`를 구현해야 한다. 키가 맵 안에 있는 동안 순서 비교 결과를 바꾸면 안 된다.
- `HashMap`과 달리 키 순서를 제공하지만, 해시 기반 평균 O(1) 조회가 필요하면 `HashMap`이 더 적합할 수 있다.
- 인덱스 연산은 키가 없으면 패닉하므로, 일반 조회에는 `get`을 사용한다.

## 완전한 예제

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();
    scores.insert("alice", 10);
    scores.insert("bob", 20);
    scores.insert("carol", 15);

    for (name, score) in &scores {
        println!("{name}: {score}");
    }
    println!("bob = {:?}", scores.get("bob"));
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn map_keeps_keys_sorted_and_replaces_values() {
        let mut map = BTreeMap::new();
        assert_eq!(map.insert(2, "old"), None);
        assert_eq!(map.insert(1, "one"), None);
        assert_eq!(map.insert(2, "new"), Some("old"));
        assert_eq!(map.get(&2), Some(&"new"));
        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, vec![1, 2]);
    }
}
```

## 언제 BTreeMap을 쓰면 좋은가요?

[`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)은 키의 `Ord` 전순서를 기준으로 B-트리에 항목을 저장한다. 삽입·조회·삭제는 일반적으로 `O(log n)`이며, 키 순서 순회와 범위 조회를 지원한다. 반면 [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)은 순회 순서를 보장하지 않는다.

- 키 정렬 순서가 필요할 때: 순회는 키의 오름차순이다. 설정 파일이나 직렬화 출력의 순서를 일정하게 유지하거나 시간순 로그와 리더보드를 만들 때 유용하다. 값 순위로 정렬하려면 그 순서에 맞는 키를 별도로 설계해야 한다.
- 범위나 경계 키가 필요할 때: `range`로 날짜·시간·가격 구간을 조회하고, `first_key_value`, `last_key_value`, `pop_first`, `pop_last`로 양끝을 다룰 수 있다. 문자열 접두사 조회는 그 접두사에 맞는 정렬 범위와 종료 조건을 신중히 정해야 한다.
- 해시 대신 순서 비교가 적합할 때: 키는 `Ord`만 있으면 되며 해시 충돌의 영향을 받지 않는다. 다만 `f32`와 `f64` 자체는 `Ord`를 구현하지 않으므로 NaN을 포함한 전순서를 정의하는 래퍼나 별도의 키 정책이 필요하다.
- 메모리 지역성을 고려할 때: B-트리 노드는 여러 키와 자식을 묶어 저장하므로 노드마다 포인터를 따라가는 트리에 비해 지역성이 유리할 수 있다. 실제 성능은 데이터와 작업에 따라 측정하는 것이 좋다.

## 범위 쿼리: 시간대 조회

`range(600..=840)`은 양쪽 경계를 포함해 오전 10시부터 오후 2시까지의 이벤트를 키 순서로 읽는다. 반환되는 항목 수를 `k`라 하면 범위 시작점을 찾고 결과를 순회하는 데 일반적으로 `O(log n + k)`가 걸린다.

```rust
use std::collections::BTreeMap;

fn timeline() -> BTreeMap<i32, &'static str> {
    let mut timeline = BTreeMap::new();
    timeline.insert(540, "업무 시작 (09:00)");
    timeline.insert(600, "스탠드업 미팅 (10:00)");
    timeline.insert(720, "점심시간 (12:00)");
    timeline.insert(840, "코드 리뷰 (14:00)");
    timeline.insert(1080, "퇴근 (18:00)");
    timeline
}

fn main() {
    for (minutes, event) in timeline().range(600..=840) {
        println!("{minutes}분: {event}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_includes_both_bounds_in_key_order() {
        let map = timeline();
        let events: Vec<_> = map.range(600..=840).map(|(&time, &event)| (time, event)).collect();
        assert_eq!(events, vec![(600, "스탠드업 미팅 (10:00)"), (720, "점심시간 (12:00)"), (840, "코드 리뷰 (14:00)")]);
        assert_eq!(map.range(900..1000).count(), 0);
    }
}
```

## 양끝 키 조회와 제거

`first_key_value`는 가장 작은 키와 값을 빌리고, `pop_last`는 가장 큰 항목을 맵에서 꺼내 소유권을 반환한다. 빈 맵에서는 둘 다 `None`을 반환한다.

```rust
use std::collections::BTreeMap;

fn prices() -> BTreeMap<i32, &'static str> {
    let mut prices = BTreeMap::new();
    prices.insert(100, "Item A");
    prices.insert(250, "Item B");
    prices.insert(50, "Item C");
    prices
}

fn main() {
    let mut prices = prices();
    println!("최저가: {:?}", prices.first_key_value()); // Some((&50, &"Item C"))
    println!("최고가 꺼내기: {:?}", prices.pop_last()); // Some((250, "Item B"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_removes_boundary_keys() {
        let mut map = prices();
        assert_eq!(map.first_key_value(), Some((&50, &"Item C")));
        assert_eq!(map.pop_last(), Some((250, "Item B")));
        assert_eq!(map.last_key_value(), Some((&100, &"Item A")));
        map.clear();
        assert_eq!(map.first_key_value(), None);
        assert_eq!(map.pop_last(), None);
    }
}
```

## BTreeMap과 HashMap 선택 기준

| 상황 / 기준 | BTreeMap | HashMap |
| --- | --- | --- |
| 단일 키 조회·삽입·삭제 | `O(log n)` | 평균 `O(1)` |
| 정렬 순회 | 키 오름차순 지원 | 키를 수집해 별도 정렬 필요 |
| 범위 검색 | `range` 지원 | 일반적으로 항목을 순회해 조건 검사 |
| 키 타입 | `K: Ord` | `K: Eq + Hash` |

실제 속도는 키의 비교·해싱 비용과 데이터에 따라 다르다.

순서가 필요 없는 단순 조회는 `HashMap`을 우선 고려하고, 정렬 순회·범위 조회·최솟값이나 최댓값 키 접근이 중요하다면 `BTreeMap`이 적합하다.

## PowerShell에서 테스트

실행할 Rust 코드 블록 하나를 `btree_map.rs`로 저장한 뒤 실행한다. 모든 예제를 한꺼번에 확인하려면 프로젝트 폴더에서 `cargo test`를 실행한다.

- `rustc --test .\btree_map.rs -o .\btree-map-tests.exe`
- `.\btree-map-tests.exe`

일반 실행은 `rustc .\btree_map.rs -o .\btree-map.exe; .\btree-map.exe`로 확인한다.

## 참고 자료

- [Rust 표준 라이브러리 BTreeMap 문서](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [Rust Book BTreeMap을 포함한 맵 소개](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust 표준 라이브러리 Ord 문서](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
