# LinkedList

`std::collections::LinkedList<T>`는 각 원소가 다음과 이전 원소를 가리키는 이중 연결 리스트다. 리스트의 앞과 뒤에서 원소를 삽입하거나 제거하는 연산을 제공한다.

## 구조 다이어그램

![prev와 next로 연결된 LinkedList의 이중 연결 노드 구조도](/assets/linked-list.svg)

그림은 각 노드가 값과 `prev`·`next` 연결을 갖는 개념적 모델입니다. 실제 표준 라이브러리의 노드 배치와 포인터 표현을 그대로 묘사한 것은 아니며, 노드 할당과 연결 관리 비용은 구현에 따라 달라질 수 있습니다.

### 그림 단계별 읽기

- 앞에서 뒤로 `노드 1`의 값 `A`, `노드 2`의 값 `B`, `노드 3`의 값 `C`가 세 노드로 연결되어 있습니다.
- 각 노드의 `next`는 뒤쪽 다음 노드를 가리키고, 각 노드의 `prev`는 앞쪽 이전 노드를 가리키므로 가운데 연결은 양방향 `prev ↔ next`입니다.
- 왼쪽의 `앞`에서 `A`부터 `B`, `C`로 순회할 수 있고, 오른쪽의 `뒤`에서는 `C`부터 역방향으로 `B`, `A`를 따라갈 수 있습니다.
- 양끝 바깥은 리스트의 경계라는 전제이며, 그림은 포인터의 구체적 표현이나 실제 노드 메모리 배치를 나타내지 않는 개념도입니다.

## 복잡도와 주의점

- `push_front`, `push_back`, `pop_front`, `pop_back`은 O(1)이다.
- `front`, `back`은 O(1)이다.
- 인덱스로 임의 위치에 접근하는 연산은 제공되지 않으며, 순회로 위치를 찾아야 하므로 O(n)이다.
- 노드가 분리되어 있어 순차 접근의 캐시 지역성이 낮고, 각 노드의 포인터 관리 비용이 추가된다.
- 따라서 일반적인 스택과 큐에는 연속 메모리를 사용하는 `Vec` 또는 `VecDeque`가 보통 더 빠르고 메모리 효율적이다. 양 끝 작업과 큐가 필요하면 특히 `VecDeque`를 먼저 고려한다.
- `LinkedList`는 연결 리스트의 특성이 실제로 필요한 경우에만 선택하는 편이 좋다. 예를 들어 `append`로 두 리스트를 연결할 수 있지만, 중간 위치를 찾는 데는 순회가 필요하다.

## 완전한 예제

```rust
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back("middle");
    list.push_front("first");
    list.push_back("last");

    println!("front = {:?}", list.front());
    println!("back = {:?}", list.back());
    while let Some(value) = list.pop_front() {
        println!("removed = {value}");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn list_supports_both_ends() {
        let mut list = LinkedList::new();
        list.push_back(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }
}
```

## PowerShell에서 테스트

프로젝트 폴더에서 `cargo test`를 실행하면 두 Rust 코드 블록을 각각 프로그램과 테스트로 컴파일하고 실행한다.

## 실무 활용과 선택 기준

`LinkedList<T>`는 두 리스트를 `append`로 이어 붙이거나 양끝에서 소유한 노드를 이동시키는 작업처럼 연결 리스트의 구조가 직접 이점이 되는 경우에 사용할 수 있습니다. 예를 들어 편집 버퍼의 구간을 이어 붙이는 작업에서는 리스트 간 이동을 표현하기 쉽습니다.

대부분의 순차 수집은 연속 메모리와 인덱스 접근을 제공하는 `Vec<T>`가 적합하고, 일반적인 양끝 큐는 `VecDeque<T>`가 캐시 지역성과 메모리 효율 면에서 유리합니다. `LinkedList<T>`는 임의 위치를 찾는 데 순회가 필요하므로, 위치를 먼저 찾아야 하는 중간 삽입이 자동으로 O(1)이 되지는 않습니다.

`append`는 오른쪽 리스트의 원소를 왼쪽 리스트로 이동시키며 오른쪽 리스트는 비워집니다. `append`는 두 리스트를 가변 참조로 받아 노드를 이동하며, 해당 가변 참조가 살아 있는 동안 충돌하는 다른 참조를 사용할 수 없습니다. 각 노드의 할당과 포인터 관리 비용, 낮은 캐시 지역성도 실제 메모리 사용량과 성능에 반영됩니다.

## 추가 예제

다음 예제는 두 리스트를 `append`로 결합한 뒤 양끝에서 하나씩 처리합니다. 비어 있는 리스트의 양끝 처리도 `None`으로 안전하게 확인합니다.

```rust
use std::collections::LinkedList;

fn append_and_take_ends(
    mut left: LinkedList<i32>,
    mut right: LinkedList<i32>,
) -> (Option<i32>, Option<i32>, LinkedList<i32>) {
    left.append(&mut right);
    let front = left.pop_front();
    let back = left.pop_back();
    (front, back, left)
}

fn main() {
    let left = LinkedList::from([1, 2]);
    let right = LinkedList::from([3, 4]);
    let (front, back, remaining) = append_and_take_ends(left, right);
    println!("앞 = {front:?}, 뒤 = {back:?}, 남은 항목 = {remaining:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_and_takes_both_ends() {
        let (front, back, remaining) = append_and_take_ends(
            LinkedList::from([1, 2]),
            LinkedList::from([3, 4]),
        );
        assert_eq!(front, Some(1));
        assert_eq!(back, Some(4));
        assert_eq!(remaining.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn empty_lists_return_none_at_both_ends() {
        let (front, back, remaining) = append_and_take_ends(
            LinkedList::new(),
            LinkedList::new(),
        );
        assert_eq!(front, None);
        assert_eq!(back, None);
        assert!(remaining.is_empty());
    }
}
```

## 참고 자료

- [Rust 표준 라이브러리 LinkedList 문서](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
- [Rust 표준 라이브러리 VecDeque 문서](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [Rust Book에서 적절한 자료구조 선택하기](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
