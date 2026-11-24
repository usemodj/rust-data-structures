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
    use super::quicksort;

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

PowerShell에서 내용을 `quicksort_example.rs`로 저장한 뒤 다음 명령으로 테스트와 실행 파일을 만들 수 있다.

```powershell
rustc --test quicksort_example.rs -o quicksort_example_tests.exe; .\quicksort_example_tests.exe
rustc quicksort_example.rs -o quicksort_example.exe; .\quicksort_example.exe
```

## 참고 자료

- [Data Structures and Algorithms in Rust: 퀵 정렬](https://dsar.rantai.dev/docs/part-ii/chapter-7/)
- [Wikipedia: Quicksort](https://en.wikipedia.org/wiki/Quicksort)
- [Rust 표준 라이브러리: `slice::swap`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap)
