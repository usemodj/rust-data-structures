# HashSet

## 구조 다이어그램

![중복을 제거하는 HashSet 해시 버킷 흐름](/assets/hashset.svg)

이 그림은 값을 해시 버킷에 배치한 뒤 `Eq`로 중복을 확인하는 개념적 모델입니다. 실제 Rust 내부 구조와 저장·순회 순서를 그대로 묘사한 것이 아니며, 충돌 시 비교가 필요함을 나타냅니다.

### 그림 단계별 읽기

- 입력에는 `"rust"`가 두 번 나타나며, 각 값은 `Hash + Eq`를 거쳐 버킷 위치 계산과 동일성 확인을 진행합니다.
- 첫 번째 `rust`는 `rust ✓`로 저장되고, 두 번째 `rust`는 `rust ✕ 중복`으로 표시되어 같은 값이 한 번만 남는 결과를 보여 줍니다.
- `Hash + Eq`의 `동일성 확인`은 같은 버킷 후보를 비교해 중복 여부를 판단한다는 뜻이며, 특정 내부 충돌 처리 방식을 가정하지 않습니다.
- 오른쪽의 `충돌은 비교로 구분`은 해시가 같은 후보라도 `Eq` 비교가 필요할 수 있다는 핵심 전제입니다.
- 그림은 개념도이므로 실제 버킷 배치와 삽입·저장·순회 순서는 보장하지 않습니다.


`HashSet<T>`는 해시와 동등성으로 중복 없는 값을 저장하는 집합이다. 삽입 순서를 보존하지 않으며, 원소의 존재 여부를 빠르게 확인할 때 적합하다. `insert`의 반환값은 새 원소가 추가되었으면 `true`, 이미 존재했으면 `false`다.

## 복잡도와 주의점

- 평균적으로 `insert`, `contains`, `remove`는 O(1)이다.
- 해시 충돌이 많으면 최악의 경우 이 연산들이 O(n)이 될 수 있다.
- `HashSet`은 원소가 `Eq`와 `Hash`를 구현해야 한다. 두 값이 같다면 해시도 같아야 한다.
- 원소를 집합에 넣은 동안 해시나 동등성에 영향을 주는 방식으로 변경하면 안 된다. 그런 변경은 논리적으로 올바르지 않은 상태를 만들 수 있다.
- 순서가 필요하면 `BTreeSet`을 고려한다. 사용자 입력처럼 공격자가 해시 입력을 통제할 수 있는 경우에는 해시 빌더 선택도 검토한다.

## 완전한 예제

```rust
use std::collections::HashSet;

fn main() {
    let mut tags = HashSet::new();
    tags.insert("rust");
    tags.insert("data");
    tags.insert("rust");

    println!("rust exists: {}", tags.contains("rust"));
    println!("unique tags: {}", tags.len());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn set_rejects_duplicates_and_supports_membership() {
        let mut values = HashSet::new();
        assert!(values.insert(10));
        assert!(!values.insert(10));
        assert!(values.contains(&10));
        assert!(values.remove(&10));
        assert!(!values.contains(&10));
    }
}
```

## PowerShell에서 테스트

각 Rust 코드 블록의 내용만 별도 파일로 저장한다. 기존 블록은 `hashset.rs`로 저장한 다음 실행한다.

- `rustc --test .\hashset.rs -o .\hashset-tests.exe`
- `.\hashset-tests.exe`

실행 예제는 `rustc .\hashset.rs -o .\hashset.exe; .\hashset.exe`로 확인할 수 있다. 외부 의존성이나 패키지 설치는 필요하지 않다.

## 실무 활용과 선택 기준

`HashSet`은 방문한 ID 기록, 허용 목록 확인, 여러 입력의 중복 제거처럼 값의 존재 여부가 핵심인 작업에 적합합니다. 평균적인 삽입·조회·삭제가 `O(1)`이지만 해시 충돌이 많으면 최악 `O(n)`이 될 수 있습니다. 순서와 범위 조회가 필요하면 `BTreeSet`을, 키마다 값을 연결해야 하면 `HashMap`을 선택하세요.

반복 순서는 보장되지 않으며, 교집합·합집합 같은 집합 연산 결과도 순서에 의존해서는 안 됩니다. 원소는 `Eq`와 `Hash`를 구현해야 하고, 같은 것으로 비교되는 원소는 같은 해시를 가져야 합니다. 저장 중인 원소의 해시나 동등성 결과를 바꾸면 안 되며, 공격자가 입력을 통제하는 경우에는 해시 빌더 선택도 검토합니다.

## 추가 예제

다음 예제는 두 사용자 그룹의 교집합을 구하고 입력 중복을 제거합니다.

```rust
use std::collections::HashSet;

fn common_users(left: &[u32], right: &[u32]) -> HashSet<u32> {
    let left_set: HashSet<_> = left.iter().copied().collect();
    let right_set: HashSet<_> = right.iter().copied().collect();
    left_set.intersection(&right_set).copied().collect()
}

fn main() {
    let shared = common_users(&[1, 2, 2, 3], &[2, 3, 4]);
    println!("shared users: {}", shared.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_removes_duplicates() {
        assert_eq!(common_users(&[1, 1, 2], &[2, 2, 3]), HashSet::from([2]));
    }

    #[test]
    fn empty_input_has_empty_intersection() {
        assert!(common_users(&[], &[1, 2]).is_empty());
    }
}
```

추가 예제 블록을 `hashset_additional.rs`로 저장하면 기존 예제와 별도로 `rustc --test hashset_additional.rs -o hashset_additional_tests.exe`와 `./hashset_additional_tests.exe`로 테스트할 수 있습니다.

## 참고 자료

- [Rust 표준 라이브러리 HashSet 문서](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [Rust 표준 라이브러리 Hash 문서](https://doc.rust-lang.org/std/hash/trait.Hash.html)
- [Rust Book 해시 맵과 집합](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
