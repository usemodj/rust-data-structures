# HashMap<K, V>

## 구조 다이어그램

![키에서 버킷으로 이어지는 HashMap 해시 흐름](/assets/hashmap.svg)

이 그림은 키를 해시해 버킷을 찾고 충돌을 비교로 처리하는 개념적 흐름입니다. 실제 Rust 구현의 내부 배치나 반복 순서를 나타내지 않으며, 해시 충돌이 발생할 수 있음을 보여 줍니다.

### 그림 단계별 읽기

- 입력에는 키와 값 `"사과" : 3`, `"배" : 5`가 있고, 각 키는 `Hash + Eq`의 전제 아래 해시 흐름을 거칩니다.
- `h(key) % 용량`은 그림에서 버킷 위치를 정하는 개념이며, 오른쪽 버킷 배열의 인덱스는 `0`부터 `3`까지입니다.
- 버킷 `0`은 비어 있고, 버킷 `1`에는 `사과 → 3`, 버킷 `2`에는 `배 → 5`가 표시되어 있습니다.
- 버킷 `3`의 `충돌 시 함께 탐색`은 같은 위치에 온 후보를 동등성 비교로 확인한다는 뜻이며, 특정 내부 충돌 자료구조를 뜻하지 않습니다.
- 그림은 예시 용량과 배치만 보여 주므로 실제 Rust의 내부 저장 방식, 버킷 수, 반복 순서는 이 그림만으로 정할 수 없습니다.


`HashMap<K, V>`는 키와 값을 해시로 매핑합니다. 삽입·조회·삭제는 평균 `O(1)`이지만, 해시 충돌이 많으면 최악 `O(n)`이 될 수 있습니다. 키는 `Eq`와 `Hash`를 구현해야 합니다.

- `entry` API는 조회 후 삽입을 한 번에 표현해 중복 조회를 피합니다.
- 반복 순서는 보장되지 않습니다. 정렬된 결과가 필요하면 키를 별도로 정렬하세요.
- 키를 맵에 넣은 동안 `Hash` 또는 `Eq` 결과를 바꾸면 맵을 정상적으로 조회할 수 없으므로 키를 변경하지 않아야 합니다.

```rust
use std::collections::HashMap;

fn count_words(words: &[&str]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for &word in words {
        *counts.entry(word.to_owned()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = count_words(&["rust", "map", "rust"]);
    println!("rust: {}", counts["rust"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_repeated_words() {
        let counts = count_words(&["a", "b", "a"]);
        assert_eq!(counts.get("a"), Some(&2));
        assert_eq!(counts.get("b"), Some(&1));
    }

    #[test]
    fn empty_input_has_no_keys() {
        assert!(count_words(&[]).is_empty());
    }
}
```

각 Rust 코드 블록은 별도 파일로 저장합니다. 기존 예제는 `hashmap.rs`로 저장한 후 `rustc --test hashmap.rs -o hashmap_tests.exe`로 컴파일하고 PowerShell에서 `.\hashmap_tests.exe`를 실행합니다. 같은 단어의 누적 횟수와 빈 입력을 검증합니다.

## 실무 활용과 선택 기준

`HashMap`은 단어 수, 사용자 ID별 상태, 설정 이름과 값처럼 키로 빠르게 값을 찾고 갱신하는 작업에 적합합니다. 평균적인 삽입·조회·삭제가 `O(1)`이므로 순서보다 키 기반 접근과 처리량이 중요할 때 선택합니다. 키 순회 순서가 필요하면 `BTreeMap`을, 중복 없는 값만 필요하면 `HashSet`을 고려하세요.

반복 순서는 보장되지 않으므로 출력이나 직렬화가 결정적이어야 하면 키를 별도로 정렬하거나 순서가 필요한 컬렉션을 사용해야 합니다. 키는 `Eq`와 `Hash`를 구현해야 하고, 같은 것으로 비교되는 키는 같은 해시를 가져야 합니다. 맵에 저장한 뒤 `Hash` 또는 `Eq` 결과가 바뀌지 않도록 하며, 공격자가 입력을 통제하는 환경에서는 해시 빌더 선택도 검토합니다.

## 추가 예제

다음 예제는 `entry` API로 이벤트별 횟수를 집계하며, 빈 입력과 반복 키를 함께 검증합니다.

```rust
use std::collections::HashMap;

fn count_events(events: &[&str]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for &event in events {
        *counts.entry(event.to_owned()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = count_events(&["login", "view", "login"]);
    println!("login: {}", counts["login"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_repeated_events() {
        assert_eq!(count_events(&["a", "b", "a"]).get("a"), Some(&2));
    }

    #[test]
    fn empty_events_have_no_entries() {
        assert!(count_events(&[]).is_empty());
    }
}
```

추가 예제 블록을 `hashmap_additional.rs`로 저장하면 기존 예제와 별도로 `rustc --test hashmap_additional.rs -o hashmap_additional_tests.exe`와 `./hashmap_additional_tests.exe`로 테스트할 수 있습니다.

참고 자료:

- [Rust 표준 라이브러리: `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Rust Book: 해시 맵](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
