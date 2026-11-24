# 고정 크기 슬라이딩 윈도우

고정 크기 슬라이딩 윈도우는 정확히 `k`개 원소로 이루어진 연속 구간을 한 칸씩 이동하며 처리합니다. 아래 예제는 각 윈도우의 합 중 최댓값을 구합니다.

## 동작 다이어그램

![크기 3인 창을 이동하며 합의 최댓값을 찾는 과정](/assets/sliding-window.svg)

`[2, 1, 5, 1, 3, 2]`에서 크기 3인 창의 합은 차례로 8, 7, 9, 6입니다. 세 번째 창 `[5, 1, 3]`의 합 9가 최댓값이며, 창을 옮길 때마다 빠지는 값과 들어오는 값만 갱신합니다.

### 그림 단계별 읽기

- 배열은 `[2, 1, 5, 1, 3, 2]`, 창 크기는 `3`이라는 전제에서 첫 창 `[2, 1, 5]`의 합을 `2 + 1 + 5 = 8`로 계산합니다.
- 한 칸 이동하면 `2`를 빼고 오른쪽의 `1`을 더해 `8 - 2 + 1 = 7`이 되며, 창은 인덱스 `1..4`의 `[1, 5, 1]`입니다.
- 다시 이동하면 `1`을 빼고 `3`을 더해 `7 - 1 + 3 = 9`가 되고, 창은 인덱스 `2..5`의 `[5, 1, 3]`으로 최댓값을 기록합니다.
- 마지막 창 `[1, 3, 2]`의 합은 `9 - 5 + 2 = 6`이므로 네 합 `8, 7, 9, 6` 중 결과는 `9`입니다.

따라서 각 창을 처음부터 다시 더하지 않고도 전체 배열을 한 번 순회할 수 있습니다.

첫 윈도우의 합을 구한 뒤 한 칸 이동할 때마다 왼쪽에서 빠지는 값을 빼고 오른쪽에 들어오는 값을 더합니다. 구간마다 모든 원소를 다시 더할 필요가 없습니다.

이 예제는 구간 합의 최댓값을 구합니다. 원소의 최솟값이나 최댓값을 유지하려면 합계만으로는 부족하며, 단조 덱 같은 별도의 자료구조를 사용할 수 있습니다.

## 복잡도와 주의점

입력 길이가 `n`이면 시간은 `O(n)`, 추가 공간은 `O(1)`입니다. 크기가 0이거나 입력보다 큰 윈도우는 `None`을 반환합니다. `i64` 검사 연산 중 오버플로가 발생해도 `None`을 반환하며 입력은 변경하지 않습니다.

## 실행 가능한 Rust 예제

```rust
fn maximum_window_sum(values: &[i64], window_size: usize) -> Option<i64> {
    if window_size == 0 || window_size > values.len() {
        return None;
    }

    let mut current = 0i64;
    for &value in &values[..window_size] {
        current = current.checked_add(value)?;
    }
    let mut best = current;

    for right in window_size..values.len() {
        current = current
            .checked_sub(values[right - window_size])?
            .checked_add(values[right])?;
        best = best.max(current);
    }
    Some(best)
}

fn main() {
    let values = [2, 1, 5, 1, 3, 2];
    println!("{:?}", maximum_window_sum(&values, 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_window_sum;

    #[test]
    fn finds_the_best_fixed_size_window() {
        assert_eq!(maximum_window_sum(&[2, 1, 5, 1, 3, 2], 3), Some(9));
    }

    #[test]
    fn handles_negative_values_and_one_element_windows() {
        assert_eq!(maximum_window_sum(&[-5, -2, -8], 2), Some(-7));
        assert_eq!(maximum_window_sum(&[4, -1, 3], 1), Some(4));
    }

    #[test]
    fn rejects_empty_zero_and_oversized_windows() {
        assert_eq!(maximum_window_sum(&[], 0), None);
        assert_eq!(maximum_window_sum(&[1, 2], 0), None);
        assert_eq!(maximum_window_sum(&[1, 2], 3), None);
    }

    #[test]
    fn reports_sum_overflow() {
        assert_eq!(maximum_window_sum(&[i64::MAX, 1], 2), None);
    }
}
```

코드를 `sliding_window.rs`로 저장한 뒤 PowerShell에서 `rustc --test sliding_window.rs -o sliding_window_tests.exe`로 컴파일하고 `.\sliding_window_tests.exe`로 테스트합니다. `rustc sliding_window.rs -o sliding_window.exe`와 `.\sliding_window.exe`로 예제도 실행할 수 있습니다.

## 참고 자료

- [CP-Algorithms: Sliding window technique](https://cp-algorithms.com/others/maximum_average_segment.html)
- [Rust standard library: Slice indexing](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust standard library: checked_add](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
