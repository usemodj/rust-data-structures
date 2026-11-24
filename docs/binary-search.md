# 이진 검색

## 동작 다이어그램

![정렬된 배열에서 탐색 범위를 절반씩 줄여 목표를 찾는 이진 검색 다이어그램](/assets/binary-search.svg)

그림은 `[왼쪽, 오른쪽)` 범위의 가운데 값을 목표와 비교하는 과정을 보여 줍니다. 가운데 7이 목표 9보다 작으면 왼쪽을 옮기고, 11이 크면 오른쪽을 옮겨 `[4, 5)`처럼 목표가 들어갈 수 있는 범위만 남깁니다.

### 그림 단계별 읽기

- 전제는 오름차순 배열 `1, 3, 5, 7, 9, 11, 13`에서 목표 `9`를 찾고, 경계를 `[왼쪽, 오른쪽)`으로 표현하는 것입니다.
- 1회 범위 `[0, 7)`의 가운데 값 `7`은 `9`보다 작으므로 왼쪽 경계를 `4`로 옮겨 2회 범위 `[4, 7)`을 남깁니다.
- 2회 범위의 가운데 값 `11`은 `9`보다 크므로 오른쪽 경계를 `5`로 옮깁니다.
- 마지막 범위 `[4, 5)`에는 `9`만 남아 목표를 발견합니다.

이 범위를 유지하면 `lower_bound`는 목표 이상인 첫 인덱스를 반환하며, 목표가 없어도 정확한 삽입 위치를 가리킵니다.

정렬된 슬라이스에서 이진 검색은 매 단계 탐색 범위를 절반으로 줄여 `O(log n)` 시간에 동작하고 추가 공간은 `O(1)`입니다. 입력이 오름차순으로 정렬되어 있어야 하며, 정렬되지 않은 입력에서는 결과를 보장하지 않습니다.

다음 예제의 `lower_bound`는 목표 이상인 첫 인덱스를 반환합니다. 목표가 존재하지 않아도 삽입 위치를 얻을 수 있습니다.

```rust
fn lower_bound(values: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = values.len();
    while left < right {
        let middle = left + (right - left) / 2;
        if values[middle] < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    left
}

fn main() {
    let values = [1, 3, 3, 7, 9];
    println!("삽입 위치: {}", lower_bound(&values, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_equal_position() {
        assert_eq!(lower_bound(&[1, 3, 3, 7], 3), 1);
    }

    #[test]
    fn returns_insertion_positions() {
        assert_eq!(lower_bound(&[2, 4, 6], 1), 0);
        assert_eq!(lower_bound(&[2, 4, 6], 5), 2);
        assert_eq!(lower_bound(&[2, 4, 6], 9), 3);
        assert_eq!(lower_bound(&[], 2), 0);
        assert_eq!(lower_bound(&[1, 2, 2, 2, 5, 8], 2), 1);
    }
}
```

테스트는 예제를 `binary_search.rs`로 저장하고 `rustc --test binary_search.rs -o binary_search_tests.exe`로 컴파일한 뒤 PowerShell에서 `.\binary_search_tests.exe`를 실행합니다. 중복 값에서 첫 위치와 범위를 벗어난 삽입 위치를 확인합니다.

## 실무 활용과 선택 기준

이진 검색은 정렬된 로그의 특정 시각, 버전 목록, 점수 구간처럼 삽입과 조회 위치를 빠르게 찾아야 하는 곳에 활용합니다. **중복 값이 있으면 시작과 끝 경계를 함께 구하는 방식이 유용합니다.**

| 선택지 | 적합한 상황 | 시간과 추가 공간 |
| --- | --- | --- |
| `lower_bound` | 목표 이상인 첫 삽입 위치가 필요할 때 | `O(log n)`, `O(1)` |
| `binary_search` | 정렬된 데이터에서 값의 존재 여부만 확인할 때 | `O(log n)`, `O(1)` |
| 선형 검색 | 데이터가 정렬되지 않았거나 매우 작을 때 | `O(n)`, `O(1)` |

- 정렬 상태를 유지하면서 삽입 위치만 필요하면 `lower_bound`를 사용합니다.
- 정확히 같은 값의 존재 여부만 확인하면 표준 라이브러리의 `binary_search`가 간결합니다.
- 조건을 만족하는 첫 위치를 찾는 문제라면 `partition_point`가 적합하지만, 조건 함수가 앞에서는 참이고 뒤에서는 거짓인 단조성을 지켜야 합니다.
- 정렬되지 않은 입력에는 이진 검색을 적용할 수 없으므로 먼저 정렬해야 하며, 정렬 비용은 `O(n log n)`입니다. 정렬이 이미 되어 있으면 검색은 `O(log n)`, 선형 검색은 `O(n)`입니다.

## Lower bound 탐색 과정

![정렬된 배열 1, 2, 2, 2, 5, 8에서 목표 2의 lower bound는 1이고 upper bound는 4이며 탐색 범위가 0부터 6, 0부터 3, 0부터 1, 1부터 1로 줄어드는 과정](/assets/lower-bound.svg)

`lower_bound`는 오름차순 배열에서 **목표 이상인 첫 원소의 인덱스**를 반환합니다. 같은 값이 여러 개라면 가장 앞의 위치이며, 목표가 없어도 정렬 순서를 유지하는 삽입 위치를 얻습니다.

다음 upper bound 그림과 같은 배열 `[1, 2, 2, 2, 5, 8]`에서 목표 `2`를 찾습니다. 탐색 범위 `[left, right)`는 왼쪽 경계를 포함하고 오른쪽 경계는 포함하지 않습니다.

- 가운데 값이 목표보다 **작으면** 가운데와 그 앞은 답이 될 수 없으므로 `left = middle + 1`로 제외합니다.
- 가운데 값이 목표 **이상**이면 가운데 인덱스도 답일 수 있으므로 `right = middle`로 경계를 옮깁니다. 가운데 원소는 다음 비교 범위에서는 빠지지만, 그 인덱스는 반환할 수 있는 경계로 남습니다.

| 단계 | 탐색 범위 `[left, right)` | 가운데 값과 판단 | 다음 범위 |
| --- | --- | --- | --- |
| 1 | `[0, 6)` | 인덱스 3의 `2`는 목표 이상 → `right = 3` | `[0, 3)` |
| 2 | `[0, 3)` | 인덱스 1의 `2`는 목표 이상 → `right = 1` | `[0, 1)` |
| 3 | `[0, 1)` | 인덱스 0의 `1`은 목표보다 작음 → `left = 1` | `[1, 1)` |

`left == right`가 되면 탐색을 끝내고 `1`을 반환합니다. 목표 이상인 원소가 없거나 배열이 비어 있으면 배열 길이 `len`을 반환합니다. 이는 삽입할 수 있는 위치지만 배열 원소로 직접 접근할 수는 없습니다.

**차이점:** lower bound는 목표 **이상**인 첫 위치를, upper bound는 목표보다 **큰** 첫 위치를 찾습니다. 이 예시의 결과는 각각 `1`과 `4`이므로 같은 값 `2`의 범위는 `[1, 4)`입니다.

## Upper bound 다이어그램

![정렬된 배열 1, 2, 2, 2, 5, 8에서 목표 2의 lower bound는 1, upper bound는 4이며 탐색 범위가 0부터 6, 4부터 6, 4부터 5, 4부터 4로 줄어드는 과정](/assets/upper-bound.svg)

`upper_bound`는 오름차순 배열에서 **목표보다 큰 첫 원소의 인덱스**를 반환합니다. 목표와 같은 값이 여러 개라면 그 값들을 모두 지난 위치입니다. 그림에서는 `[1, 2, 2, 2, 5, 8]`에서 목표 `2`보다 큰 첫 값은 인덱스 `4`의 `5`입니다.

탐색 범위 `[left, right)`는 `left`를 포함하고 `right`는 포함하지 않습니다. 범위를 반으로 줄일 때는 다음 규칙을 사용합니다.

- 가운데 값이 목표 **이하**이면, 정렬 순서상 가운데와 그 앞의 값은 답이 될 수 없으므로 `left = middle + 1`로 제외합니다.
- 가운데 값이 목표보다 **크면**, 가운데 인덱스도 정답일 수 있으므로 `right = middle`로 경계를 옮깁니다. 가운데 원소는 다음 비교 범위에서는 빠지지만, 그 인덱스는 반환할 수 있는 경계로 보존됩니다.

| 단계 | 탐색 범위 `[left, right)` | 가운데 값과 판단 | 다음 범위 |
| --- | --- | --- | --- |
| 1 | `[0, 6)` | 인덱스 3의 `2`는 목표 이하이므로 그 위치까지 제외 → `left = 4` | `[4, 6)` |
| 2 | `[4, 6)` | 인덱스 5의 `8`은 목표보다 크므로 경계를 후보로 보존 → `right = 5` | `[4, 5)` |
| 3 | `[4, 5)` | 인덱스 4의 `5`는 목표보다 크므로 경계를 후보로 보존 → `right = 4` | `[4, 4)` |

`left == right`가 되면 탐색을 끝내고 `4`를 반환합니다. `lower_bound`는 목표 **이상**인 첫 위치 `1`이므로, 목표와 같은 값들은 `[1, 4)`에 있으며 개수는 `4 - 1 = 3`입니다. 목표보다 큰 원소가 없거나 배열이 비어 있으면 `upper_bound`는 배열 길이 `len`을 반환합니다. 이는 유효한 삽입 위치지만 원소 인덱스로 직접 접근할 수는 없습니다.

## 추가 예제

다음 프로그램은 정렬된 슬라이스에서 목표 값의 시작 위치(lower bound)와 끝 다음 위치(upper bound)를 구해 중복 값의 범위를 반환합니다. 빈 입력, 모든 값이 같은 입력, 범위가 없는 입력도 테스트합니다.

```rust
fn lower_bound(values: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = values.len();
    while left < right {
        let middle = left + (right - left) / 2;
        if values[middle] < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    left
}

fn upper_bound(values: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = values.len();
    while left < right {
        let middle = left + (right - left) / 2;
        if values[middle] <= target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    left
}

fn equal_range(values: &[i32], target: i32) -> (usize, usize) {
    let start = lower_bound(values, target);
    (start, upper_bound(values, target))
}

fn main() {
    let values = [1, 2, 2, 2, 5, 8];
    let range = equal_range(&values, 2);
    println!("2의 범위: {:?}", range);
}

#[cfg(test)]
mod tests {
    use super::{equal_range, upper_bound};

    #[test]
    fn upper_bound_matches_diagram_and_boundary_cases() {
        let values = [1, 2, 2, 2, 5, 8];
        assert_eq!(upper_bound(&values, 2), 4);
        assert_eq!(upper_bound(&values, 0), 0);
        assert_eq!(upper_bound(&values, 8), values.len());
        assert_eq!(upper_bound(&[], 2), 0);
    }

    #[test]
    fn finds_duplicate_range() {
        assert_eq!(equal_range(&[1, 2, 2, 2, 5], 2), (1, 4));
    }

    #[test]
    fn handles_empty_and_all_duplicate_inputs() {
        assert_eq!(equal_range(&[], 3), (0, 0));
        assert_eq!(equal_range(&[4, 4, 4], 4), (0, 3));
    }

    #[test]
    fn returns_empty_range_when_target_is_absent() {
        assert_eq!(equal_range(&[1, 3, 5], 4), (2, 2));
    }
}
```

## `partition_point`와 `binary_search_by_key`의 동작 원리

아래 실용 예제에서 사용하는 `events.partition_point(...)`와 `users.binary_search_by_key(...)`는 **Rust 표준 라이브러리가 슬라이스에 제공하는 메서드**입니다. 따라서 예제 안에서 직접 정의하지 않아도 사용할 수 있습니다.

다음은 원리를 살펴보기 위해 작성한 **학습용 구현**입니다. 표준 라이브러리의 실제 소스가 아니라 같은 역할을 하는 독립적인 자유 함수이며, 호출할 때 `partition_point(&values, ...)`처럼 슬라이스를 첫 인자로 전달합니다.

- `partition_point`는 조건이 참인 구간의 끝 인덱스를 반환합니다. 데이터는 조건에 대해 **참이 먼저, 거짓이 나중**인 순서로 나뉘어 있어야 합니다. 모두 참이면 `len`, 모두 거짓이거나 비어 있으면 `0`입니다.
- `binary_search_by_key`는 키 기준으로 오름차순 정렬된 슬라이스에서 일치하는 인덱스를 `Ok`로, 없을 때 삽입 위치를 `Err`로 반환합니다. 키가 중복되면 일치하는 인덱스 중 하나가 반환되며 어느 것인지는 보장하지 않습니다.

```rust
use std::cmp::Ordering;

// 조건이 참인 원소들이 거짓인 원소들보다 앞에 있어야 한다.
fn partition_point<T, F>(values: &[T], mut predicate: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let middle = left + (right - left) / 2;
        if predicate(&values[middle]) {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    left
}

// values는 key_of가 반환하는 키 기준 오름차순이어야 한다.
fn binary_search_by_key<T, K, F>(values: &[T], key: &K, mut key_of: F) -> Result<usize, usize>
where
    K: Ord,
    F: FnMut(&T) -> K,
{
    let (mut left, mut right) = (0, values.len());
    while left < right {
        let middle = left + (right - left) / 2;
        match key_of(&values[middle]).cmp(key) {
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
            Ordering::Equal => return Ok(middle),
        }
    }
    Err(left)
}

fn main() {
    let values = [1, 2, 2, 2, 5, 8];
    let first_greater = partition_point(&values, |value| *value <= 2);
    println!("2의 upper bound: {first_greater}");

    let users = [(10, "민지"), (20, "지훈"), (40, "서연")];
    println!("ID 20: {:?}", binary_search_by_key(&users, &20, |user| user.0));
    println!("ID 30: {:?}", binary_search_by_key(&users, &30, |user| user.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_point_handles_boundaries_and_duplicates() {
        let values = [1, 2, 2, 2, 5, 8];
        for target in 0..=9 {
            let ours = partition_point(&values, |value| *value <= target);
            let standard = values.partition_point(|value| *value <= target);
            assert_eq!(ours, standard);
        }
        assert_eq!(partition_point::<i32, _>(&[], |_| true), 0);
        assert_eq!(partition_point(&values, |_| false), 0);
        assert_eq!(partition_point(&values, |_| true), values.len());
    }

    #[test]
    fn binary_search_handles_matches_and_insertion_positions() {
        let users = [(10, "a"), (20, "b"), (40, "c")];
        assert_eq!(binary_search_by_key(&users, &20, |user| user.0), Ok(1));
        assert_eq!(binary_search_by_key(&users, &5, |user| user.0), Err(0));
        assert_eq!(binary_search_by_key(&users, &30, |user| user.0), Err(2));
        assert_eq!(binary_search_by_key(&users, &50, |user| user.0), Err(3));
        assert_eq!(binary_search_by_key::<i32, _, _>(&[], &20, |value| *value), Err(0));
        let duplicates = [1, 2, 2, 2, 5];
        let index = binary_search_by_key(&duplicates, &2, |value| *value).unwrap();
        assert_eq!(duplicates[index], 2);
    }
}
```

두 구현 모두 범위를 절반씩 줄이므로 `O(log n)`번 비교하고 `O(1)`의 추가 공간을 사용합니다. 정렬이나 조건의 단조성이 깨지면 결과를 보장하지 않습니다. 실무에서는 직접 구현하기보다 아래 예제처럼 표준 메서드를 사용하면 검증된 구현을 간결하게 활용할 수 있습니다.

## 실용 예제 1: 시간 범위의 로그 찾기

타임스탬프가 오름차순으로 정렬된 로그에서 `[start, end)` 시간 범위의 기록을 찾습니다. `partition_point`는 조건이 참인 구간의 끝 위치를 반환하므로, `time < start`와 `time < end`의 경계를 구하면 됩니다. 시작 시각은 포함하고 종료 시각은 제외하며, 같은 시각의 로그도 빠짐없이 포함합니다. 예제의 시각은 정수 초입니다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Event {
    time: u64,
    message: &'static str,
}

// events는 time 기준 오름차순이어야 한다.
fn events_in_range(events: &[Event], start: u64, end: u64) -> &[Event] {
    if start >= end {
        return &events[0..0];
    }
    let first = events.partition_point(|event| event.time < start);
    let last = events.partition_point(|event| event.time < end);
    &events[first..last]
}

fn main() {
    let events = [
        Event { time: 10, message: "시작" },
        Event { time: 20, message: "연결" },
        Event { time: 20, message: "요청" },
        Event { time: 30, message: "종료" },
    ];
    println!("{:?}", events_in_range(&events, 20, 30));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_start_and_duplicates_but_excludes_end() {
        let events = [
            Event { time: 10, message: "a" },
            Event { time: 20, message: "b" },
            Event { time: 20, message: "c" },
            Event { time: 30, message: "d" },
        ];
        assert_eq!(events_in_range(&events, 20, 30), &events[1..3]);
        assert_eq!(events_in_range(&events, 10, 31), &events[..]);
        assert!(events_in_range(&events, 21, 30).is_empty());
        assert!(events_in_range(&events, 30, 30).is_empty());
        assert!(events_in_range(&events, 30, 20).is_empty());
        assert!(events_in_range(&[], 0, 100).is_empty());
    }
}
```

예제에서는 `[20, 30)`에 해당하는 `연결`과 `요청`이 반환됩니다. 시작이 종료 이상이면 빈 슬라이스를 반환하는 정책을 사용합니다. 두 번의 이진 검색은 `O(log n)`이며 반환한 슬라이스는 원본을 빌리므로 복사하지 않습니다. 결과를 모두 읽는 비용은 별도로 결과 개수 `k`에 비례해 `O(k)`입니다.

## 실용 예제 2: 정렬된 ID로 레코드 찾기

ID가 오름차순이며 중복이 없는 레코드에서는 `binary_search_by_key`로 일치하는 항목을 찾을 수 있습니다. 결과는 찾았을 때 `Ok(인덱스)`, 없을 때 `Err(정렬을 유지하는 삽입 인덱스)`입니다. 중복 ID를 허용하면 일치하는 항목 중 어느 인덱스가 반환될지는 보장되지 않습니다.

```rust
#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
    name: &'static str,
}

// users는 id 기준 오름차순이며 id가 유일해야 한다.
fn find_user(users: &[User], id: u32) -> Result<&User, usize> {
    users.binary_search_by_key(&id, |user| user.id)
        .map(|index| &users[index])
}

fn main() {
    let users = [
        User { id: 10, name: "민지" },
        User { id: 20, name: "지훈" },
        User { id: 40, name: "서연" },
    ];
    println!("ID 20: {:?}", find_user(&users, 20));
    println!("ID 30 삽입 위치: {:?}", find_user(&users, 30));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_records_or_insertion_positions() {
        let users = [
            User { id: 10, name: "a" },
            User { id: 20, name: "b" },
            User { id: 40, name: "c" },
        ];
        assert_eq!(find_user(&users, 20), Ok(&users[1]));
        assert_eq!(find_user(&users, 5), Err(0));
        assert_eq!(find_user(&users, 30), Err(2));
        assert_eq!(find_user(&users, 50), Err(3));
        assert_eq!(find_user(&[], 20), Err(0));
    }
}
```

ID `20`은 두 번째 레코드이고, 없는 ID `30`의 삽입 위치는 인덱스 `2`입니다. 검색은 `O(log n)`이고 추가 공간은 `O(1)`입니다. 단, 벡터 중간에 실제로 삽입하면 뒤 원소를 옮기는 데 `O(n)`이 걸릴 수 있습니다.

각 Rust 코드 블록은 독립 프로그램입니다. 프로젝트 폴더에서 `cargo test`를 실행하면 모든 Rust 코드 블록을 자동으로 추출해 검증합니다.

참고 자료:

- [Rust 표준 라이브러리: `slice::binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
- [Rust 표준 라이브러리: `slice::binary_search_by_key`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by_key)
- [Rust 표준 라이브러리: `slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point)
