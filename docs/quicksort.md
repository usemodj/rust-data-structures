# 퀵 정렬

## 동작 다이어그램

![피벗 5를 기준으로 작은 값 같은 값 큰 값으로 나누는 퀵 정렬 다이어그램](/assets/quicksort.svg)

그림은 입력 `5, 2, 5, 8, 3, 5`에서 피벗 `5`를 기준으로 `2, 3`인 작은 구간, `5, 5, 5`인 같은 구간, `8`인 큰 구간을 만드는 3-way 분할을 보여 줍니다. 같은 구간은 이미 피벗과 같으므로 다시 분할하지 않습니다.

### 그림 단계별 읽기

- 시작 입력은 `5, 2, 5, 8, 3, 5`이고, 그림의 전제는 피벗을 `5`로 정해 각 값을 피벗과 비교하는 것입니다.
- 피벗보다 작은 값 `2, 3`은 왼쪽의 작은 구간으로, 피벗과 같은 값 `5, 5, 5`는 가운데 구간으로 보냅니다.
- 피벗보다 큰 값 `8`은 오른쪽의 큰 구간으로 보내 세 구간을 만듭니다.
- 가운데의 같은 값 구간은 다시 나누지 않고, 작은 구간과 큰 구간만 재귀 정렬합니다.

작은 구간과 큰 구간만 재귀적으로 정렬하며, 제자리 교환 과정에서 같은 값의 상대 순서는 보장하지 않습니다.

퀵 정렬은 피벗을 기준으로 작은 값, 같은 값, 큰 값의 세 구간으로 나눈 뒤 작은 문제를 반복해서 정렬하는 비교 정렬 알고리즘이다. 이 구현은 첫 원소, 가운데 원소, 마지막 원소 중 중앙값을 피벗으로 선택해 이미 정렬되었거나 역순인 입력에서 흔한 나쁜 분할을 줄인다. 같은 값 구간을 한 번에 제외하는 3-way 분할도 중복이 많은 입력에 유리하다.

평균 및 일반적인 기대 시간 복잡도는 `O(n log n)`이고, 피벗 선택이 계속 편향되면 최악 시간은 `O(n²)`이다. 작은 쪽 구간만 재귀 호출하고 큰 쪽은 반복 처리하므로 호출 스택의 추가 공간은 `O(log n)` 수준으로 제한된다. 다만 최악의 분할에서도 이 전략은 스택을 `O(log n)`으로 제한하지만 시간은 `O(n²)`일 수 있다. 제자리 정렬이므로 별도의 `O(n)` 배열을 만들지 않으며 안정 정렬이 아니어서 같은 값의 상대 순서는 보장하지 않는다.

정수 비교와 교환만 사용하므로 덧셈이나 뺄셈으로 피벗을 계산하지 않는다. 따라서 `i32`의 최댓값과 최솟값을 포함해도 산술 오버플로가 발생하지 않는 것이 이 구현의 오버플로 정책이다. 재귀 깊이와 실행 시간은 매우 큰 입력에서 여전히 한계가 될 수 있으므로, 최악 시간 보장이 필요하면 힙 정렬이나 표준 라이브러리 정렬처럼 다른 알고리즘을 고려해야 한다.

## 예제 함수 설명

- **`quicksort(values: &mut [i32])`:** 정수 슬라이스를 오름차순으로 제자리 정렬한다. 결과를 반환하는 대신 전달받은 슬라이스를 직접 바꾼다. 길이가 0 또는 1이면 이미 정렬되어 있으므로 처리하지 않는다.
- **`median_of_three(a: i32, b: i32, c: i32) -> i32`:** 세 값 중 크기가 가운데인 값을 반환한다. `quicksort`는 현재 슬라이스의 첫 값, 가운데 값, 마지막 값을 전달해 피벗을 정한다. 같은 값이 있어도 비교로 중앙값을 선택한다.
- **`main()`:** 예제 벡터 `[9, 3, 7, 3, 1, 8, 2]`를 만들고 `quicksort`를 호출한 뒤 `[1, 2, 3, 3, 7, 8, 9]`를 출력한다.

`quicksort`의 분할에서는 `less`, `current`, `greater`가 구간의 경계를 나타낸다. 현재 값이 피벗보다 작으면 작은 구간으로 옮기고 `less`와 `current`를 늘린다. 같으면 `current`만 늘린다. 크면 `greater`를 줄이고 그 위치의 값과 교환한다. 이때 교환되어 들어온 값은 아직 확인하지 않았으므로 `current`를 그대로 둔다.

분할이 끝나면 `split_at_mut`로 작은 구간과 큰 구간을 분리한다. 피벗과 같은 가운데 구간은 다시 정렬하지 않는다. 두 구간 중 작은 쪽은 재귀 호출하고 큰 쪽은 `values`에 다시 지정해 반복 처리하므로 재귀 호출 깊이를 줄인다.

아래 테스트 함수들은 Rust 테스트 하네스가 실행한다.

- **`first_partition_matches_diagram()`:** 예제 입력의 첫 분할 과정을 테스트에서 직접 재현해 피벗, 각 단계의 배열과 경계를 확인한 뒤 정렬 결과를 확인한다.
- **`sorts_duplicates_and_unsorted_values()`:** 중복이 포함된 정렬되지 않은 입력이 오름차순으로 정렬되는지 확인한다.
- **`handles_empty_single_sorted_and_reverse_inputs()`:** 빈 입력, 원소 하나, 이미 정렬된 입력, 역순 입력을 확인한다.
- **`matches_standard_sort_for_small_inputs()`:** `-1`, `0`, `1`로 만들 수 있는 길이 7의 모든 입력에 대해 결과를 표준 `sort()`와 비교한다.
- **`handles_extreme_i32_values_without_arithmetic_overflow()`:** `i32::MIN`과 `i32::MAX`가 포함된 입력의 정렬 결과를 확인한다.

## 예시로 보는 전체 상태 변화 추적

아래 `Rust 예제의 3-way 분할 따라가기`에서 첫 분할을 자세히 확인할 수 있다. 여기서는 같은 `main()` 입력이 첫 분할 이후 정렬을 마칠 때까지 추적한다. 첫 분할 뒤 전체 배열은 `[2, 1, 3, 3, 8, 7, 9]`이고, 작은 구간 `[2, 1]`이 큰 구간 `[8, 7, 9]`보다 짧으므로 작은 구간을 먼저 재귀 호출한다.

### 작은 구간 `[2, 1]`의 재귀 정렬

첫 값 `2`, 가운데 값 `1`, 마지막 값 `1`의 중앙값은 `1`이다. 아래 인덱스와 경계 `(less, current, greater)`는 **현재 슬라이스 `[2, 1]` 기준**이며, 초기 경계는 `(0, 0, 2)`다.

| 단계 | 비교와 처리 | 처리 후 현재 슬라이스 | 처리 후 경계 |
| --- | --- | --- | --- |
| 1 | `2 > 1`: `greater`를 줄이고 인덱스 0과 1을 교환 | `[1, 2]` | `(0, 0, 1)` |
| 2 | `1 = 1`: `current`를 늘림 | `[1, 2]` | `(0, 1, 1)` |

`current == greater`이므로 분할이 끝난다. 작은 구간은 `[]`, 같은 구간은 `[1]`, 큰 구간은 `[2]`다. 더 짧은 빈 구간을 재귀 호출하고 `[2]`를 반복 처리하지만 둘 다 길이가 2보다 작아 바로 끝난다. 원래 호출로 돌아오면 전체 배열은 `[1, 2, 3, 3, 8, 7, 9]`다.

### 큰 구간 `[8, 7, 9]`의 반복 정렬

원래 호출은 남은 큰 구간 `[8, 7, 9]`을 `values`에 지정해 반복 처리한다. 첫 값 `8`, 가운데 값 `7`, 마지막 값 `9`의 중앙값은 `8`이다. 이번 경계와 인덱스는 **현재 슬라이스 `[8, 7, 9]` 기준**이며, 초기 경계는 `(0, 0, 3)`이다.

| 단계 | 비교와 처리 | 처리 후 현재 슬라이스 | 처리 후 경계 |
| --- | --- | --- | --- |
| 1 | `8 = 8`: `current`를 늘림 | `[8, 7, 9]` | `(0, 1, 3)` |
| 2 | `7 < 8`: 인덱스 0과 1을 교환하고 `less`와 `current`를 늘림 | `[7, 8, 9]` | `(1, 2, 3)` |
| 3 | `9 > 8`: `greater`를 줄이고 인덱스 2와 2를 교환 | `[7, 8, 9]` | `(1, 2, 2)` |

분할 결과는 작은 `[7]`, 같은 `[8]`, 큰 `[9]`다. 작은 구간과 큰 구간의 길이가 같으므로 코드의 `else` 분기에서 `[9]`를 재귀 호출하고 `[7]`을 반복 처리한다. 두 구간 모두 원소가 하나여서 더 이상 분할하지 않는다.

최종적으로 `main()`의 전체 벡터는 `[1, 2, 3, 3, 7, 8, 9]`가 되고, 이 값이 출력된다.

## 실행 가능한 Rust 예제

```rust
fn quicksort(mut values: &mut [i32]) {
    while values.len() >= 2 {
        let middle = values.len() / 2;
        let last = values.len() - 1;
        let pivot = median_of_three(values[0], values[middle], values[last]);

        let mut less = 0;
        let mut current = 0;
        let mut greater = values.len();
        while current < greater {
            match values[current].cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    values.swap(less, current);
                    less += 1;
                    current += 1;
                }
                std::cmp::Ordering::Equal => current += 1,
                std::cmp::Ordering::Greater => {
                    greater -= 1;
                    values.swap(current, greater);
                }
            }
        }

        let (left, rest) = values.split_at_mut(less);
        let (_, right) = rest.split_at_mut(greater - less);
        if left.len() < right.len() {
            quicksort(left);
            values = right;
        } else {
            quicksort(right);
            values = left;
        }
    }
}

fn median_of_three(a: i32, b: i32, c: i32) -> i32 {
    if (a <= b && b <= c) || (c <= b && b <= a) {
        b
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        a
    } else {
        c
    }
}

fn main() {
    let mut values = vec![9, 3, 7, 3, 1, 8, 2];
    quicksort(&mut values);
    println!("{values:?}");
}

#[cfg(test)]
mod tests {
    use super::{median_of_three, quicksort};

    #[test]
    fn first_partition_matches_diagram() {
        let mut values = vec![9, 3, 7, 3, 1, 8, 2];
        let pivot = median_of_three(values[0], values[values.len() / 2], values[values.len() - 1]);
        assert_eq!(pivot, 3);
        let (mut less, mut current, mut greater) = (0, 0, values.len());
        let mut trace = Vec::new();
        while current < greater {
            match values[current].cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    values.swap(less, current);
                    less += 1;
                    current += 1;
                }
                std::cmp::Ordering::Equal => current += 1,
                std::cmp::Ordering::Greater => {
                    greater -= 1;
                    values.swap(current, greater);
                }
            }
            trace.push((values.clone(), less, current, greater));
        }
        assert_eq!(trace, vec![
            (vec![2, 3, 7, 3, 1, 8, 9], 0, 0, 6),
            (vec![2, 3, 7, 3, 1, 8, 9], 1, 1, 6),
            (vec![2, 3, 7, 3, 1, 8, 9], 1, 2, 6),
            (vec![2, 3, 8, 3, 1, 7, 9], 1, 2, 5),
            (vec![2, 3, 1, 3, 8, 7, 9], 1, 2, 4),
            (vec![2, 1, 3, 3, 8, 7, 9], 2, 3, 4),
            (vec![2, 1, 3, 3, 8, 7, 9], 2, 4, 4),
        ]);
        quicksort(&mut values);
        assert_eq!(values, [1, 2, 3, 3, 7, 8, 9]);
    }

    #[test]
    fn sorts_duplicates_and_unsorted_values() {
        let mut values = vec![9, 3, 7, 3, 1, 8, 2];
        quicksort(&mut values);
        assert_eq!(values, vec![1, 2, 3, 3, 7, 8, 9]);
    }

    #[test]
    fn handles_empty_single_sorted_and_reverse_inputs() {
        let mut empty = Vec::new();
        quicksort(&mut empty);
        assert!(empty.is_empty());

        let mut single = vec![42];
        quicksort(&mut single);
        assert_eq!(single, vec![42]);

        let mut sorted = vec![1, 2, 3, 4, 5];
        quicksort(&mut sorted);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);

        let mut reverse = vec![5, 4, 3, 2, 1];
        quicksort(&mut reverse);
        assert_eq!(reverse, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn matches_standard_sort_for_small_inputs() {
        for mut encoded in 0..3_usize.pow(7) {
            let mut values = Vec::new();
            for _ in 0..7 {
                values.push((encoded % 3) as i32 - 1);
                encoded /= 3;
            }
            let mut expected = values.clone();
            expected.sort();
            quicksort(&mut values);
            assert_eq!(values, expected);
        }
    }

    #[test]
    fn handles_extreme_i32_values_without_arithmetic_overflow() {
        let mut values = vec![i32::MAX, 0, i32::MIN, -1, i32::MAX, i32::MIN];
        quicksort(&mut values);
        assert_eq!(
            values,
            vec![i32::MIN, i32::MIN, -1, 0, i32::MAX, i32::MAX]
        );
    }
}
```

## Rust 예제의 3-way 분할 따라가기

![퀵 정렬 예제 9, 3, 7, 3, 1, 8, 2에서 피벗 3을 기준으로 일곱 번 비교하여 작은 구간 2, 1, 같은 구간 3, 3, 큰 구간 8, 7, 9를 만들고 최종 결과 1, 2, 3, 3, 7, 8, 9를 얻는 단계](/assets/quicksort-steps.svg)

위쪽 그림과 달리 이 단계별 그림은 Rust `main`의 입력 `[9, 3, 7, 3, 1, 8, 2]`를 사용합니다. 첫 값 `9`, 가운데 값 `3`, 마지막 값 `2`의 중앙값이므로 피벗은 `3`입니다.

`less`, `current`, `greater`를 각각 `l`, `c`, `g`라고 쓰면, 매 단계의 구간은 `[0, l)`이 피벗보다 작고, `[l, c)`가 같고, `[c, g)`는 아직 비교하지 않았으며, `[g, n)`은 피벗보다 큽니다. 초기 경계는 `(0, 0, 7)`입니다. 아래 **교환 번호는 값이 아니라 배열 인덱스**이고 경계는 각 단계가 끝난 직후입니다.

| 단계 | 현재 값과 동작 | 처리 후 배열 | `(l, c, g)` |
| --- | --- | --- | --- |
| 1 | `9 > 3`: `g`를 줄이고 인덱스 0과 6 교환 | `[2, 3, 7, 3, 1, 8, 9]` | `(0, 0, 6)` |
| 2 | `2 < 3`: 작은 구간을 늘림 | `[2, 3, 7, 3, 1, 8, 9]` | `(1, 1, 6)` |
| 3 | `3 = 3`: 같은 구간을 늘림 | `[2, 3, 7, 3, 1, 8, 9]` | `(1, 2, 6)` |
| 4 | `7 > 3`: `g`를 줄이고 인덱스 2와 5 교환 | `[2, 3, 8, 3, 1, 7, 9]` | `(1, 2, 5)` |
| 5 | `8 > 3`: `g`를 줄이고 인덱스 2와 4 교환 | `[2, 3, 1, 3, 8, 7, 9]` | `(1, 2, 4)` |
| 6 | `1 < 3`: 인덱스 1과 2 교환 후 작은 구간을 늘림 | `[2, 1, 3, 3, 8, 7, 9]` | `(2, 3, 4)` |
| 7 | `3 = 3`: 같은 구간을 늘려 `c == g`가 됨 | `[2, 1, 3, 3, 8, 7, 9]` | `(2, 4, 4)` |

큰 값을 뒤쪽과 교환한 뒤에는 새로 들어온 값이 아직 미확인이므로 `current`를 늘리지 않습니다. 첫 분할 결과는 작은 `[2, 1]`, 같은 `[3, 3]`, 큰 `[8, 7, 9]`입니다. 같은 구간은 건너뛰고 작은 구간과 큰 구간을 각각 정렬하면 최종 결과는 `[1, 2, 3, 3, 7, 8, 9]`입니다.

PowerShell에서 내용을 `quicksort_example.rs`로 저장한 뒤 다음 명령으로 테스트와 실행 파일을 만들 수 있다.

```powershell
rustc --test quicksort_example.rs -o quicksort_example_tests.exe; .\quicksort_example_tests.exe
rustc quicksort_example.rs -o quicksort_example.exe; .\quicksort_example.exe
```

## 참고 자료

- [Data Structures and Algorithms in Rust: 퀵 정렬](https://dsar.rantai.dev/docs/part-ii/chapter-7/)
- [Wikipedia: Quicksort](https://en.wikipedia.org/wiki/Quicksort)
- [Rust 표준 라이브러리: `slice::swap`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap)
