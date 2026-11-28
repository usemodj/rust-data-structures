# 누적 합

누적 합 배열은 여러 구간 합을 빠르게 계산하기 위해 앞에서부터 합을 저장합니다. 아래 예제는 반열린 구간 `[left, right)`을 사용하고 오버플로를 검사합니다.

`prefix[0] = 0`, `prefix[i + 1] = prefix[i] + input[i]`로 정의하면 반열린 구간 `[left, right)`의 합은 `prefix[right] - prefix[left]`입니다.

## 동작 다이어그램

![누적 합으로 반열린 구간의 합을 계산하는 과정](/assets/prefix-sum.svg)

배열 `[3, -1, 4, 1, 5]`의 누적 합은 `[0, 3, 2, 6, 7, 12]`입니다. `[1, 4)`는 인덱스 1, 2, 3만 포함하므로 `prefix[4] - prefix[1] = 7 - 3 = 4`입니다.

### 그림 단계별 읽기

- 입력 배열은 인덱스 `0`부터 `4`까지 `[3, -1, 4, 1, 5]`이고, 그림의 반열린 경계 `[1, 4)`는 인덱스 `1`, `2`, `3`만 포함하며 오른쪽 `4`는 포함하지 않습니다.
- 누적 합은 `prefix[0] = 0`에서 시작해 `[0, 3, 2, 6, 7, 12]`가 되므로 그림에 표시된 `prefix[1] = 3`과 `prefix[4] = 7`을 사용합니다.
- 구간 합은 `prefix[4] - prefix[1] = 7 - 3 = 4`이고, 원소를 직접 더해도 `-1 + 4 + 1 = 4`로 같은 결과입니다.
- 이 계산은 `prefix[i + 1] = prefix[i] + input[i]`과 반열린 구간이라는 전제에서 동작하며, 빈 구간 `[i, i)`는 두 같은 경계의 차이로 `0`이 됩니다.

오른쪽 끝 `4`는 포함하지 않는 경계이며, 이 규칙 덕분에 빈 구간 `[i, i)`의 합도 0으로 일관되게 표현됩니다.

누적 합 배열 생성은 `O(n)`, 유효한 각 구간 합 조회는 `O(1)`입니다. 데이터가 바뀌지 않고 조회가 많을 때 유용합니다. 원소를 수정하려면 뒤쪽 누적 합을 다시 계산하거나 펜윅 트리 또는 세그먼트 트리 등을 고려해야 합니다.

## 복잡도와 주의점

추가 공간은 `O(n)`입니다. 예제는 `i64`의 검사된 덧셈과 뺄셈을 사용하여 중간 누적 합이나 구간 합의 오버플로가 발생하면 `None`을 반환합니다. `left > right` 또는 `right > values.len()`인 범위도 `None`이며, 빈 구간의 합은 0입니다.

## 누적 합의 활용

- **매출·거래 분석:** 날짜별 매출을 누적해 원하는 기간의 매출 합계를 빠르게 조회합니다.
- **시계열 데이터:** 시간대별 방문자 수나 측정값을 누적해 여러 시간 구간의 합을 구합니다.
- **범위별 건수 계산:** 조건에 맞는 항목을 `1`, 나머지를 `0`으로 표시하면 구간 내 해당 항목의 개수를 셀 수 있습니다.
- **이미지 처리:** 2차원 누적 합을 이용하면 사각형 영역의 픽셀 값 합을 빠르게 계산할 수 있습니다.

예를 들어 일별 매출이 고정되어 있고 여러 기간의 합계를 조회한다면 누적 합을 한 번 `O(n)`에 만든 뒤 각 구간을 `O(1)`에 조회할 수 있습니다. 반대로 매출 값의 수정이 잦으면 변경 위치 뒤의 누적 합을 다시 계산해야 하므로 펜윅 트리나 세그먼트 트리를 고려할 수 있습니다.

## 실행 가능한 Rust 예제

### 예시로 보는 상태 변화 추적

아래 예제의 `values = [3, -1, 4, 1, 5]`로 `prefix_sums(&values)`를 호출합니다. 함수는 먼저 `prefix`에 `0`을 넣고, 각 `value`를 마지막 누적 합에 더한 `next`를 `prefix`에 추가합니다.

| 단계 | `value` | `prefix.last()?`가 가리키는 값 | `next` 계산 | 추가 후 `prefix` |
| --- | --- | --- | --- | --- |
| 초기 상태 | - | - | - | `[0]` |
| 1회 | `3` | `0` | `0 + 3 = 3` | `[0, 3]` |
| 2회 | `-1` | `3` | `3 + (-1) = 2` | `[0, 3, 2]` |
| 3회 | `4` | `2` | `2 + 4 = 6` | `[0, 3, 2, 6]` |
| 4회 | `1` | `6` | `6 + 1 = 7` | `[0, 3, 2, 6, 7]` |
| 5회 | `5` | `7` | `7 + 5 = 12` | `[0, 3, 2, 6, 7, 12]` |

반복이 끝나면 `prefix_sums`는 `Some(vec![0, 3, 2, 6, 7, 12])`을 반환하고, 예제의 `unwrap()`은 벡터를 꺼냅니다. `prefix[i]`는 `values`의 앞에서부터 `i`개 원소의 합입니다.

이어지는 `range_sum(&prefix, 1, 4)` 호출에서는 `left = 1`, `right = 4`입니다. 범위 검사 `left > right || right >= prefix.len()`은 거짓이고, `prefix[4] - prefix[1] = 7 - 3 = 4`이므로 `Some(4)`를 반환합니다. 이는 반열린 구간 `[1, 4)`의 원소 `-1 + 4 + 1`과 같으며, 예제는 `4`를 출력합니다.

덧셈이나 뺄셈이 `i64` 범위를 벗어나면 각각 `checked_add` 또는 `checked_sub`가 `None`을 반환합니다. `prefix_sums`에서는 `?`가 그 `None`을 즉시 반환하고, `range_sum`에서도 뺄셈 결과인 `None`이 반환됩니다.

```rust
fn prefix_sums(values: &[i64]) -> Option<Vec<i64>> {
    let mut prefix: Vec<i64> = Vec::with_capacity(values.len() + 1);
    prefix.push(0);
    for &value in values {
        let next = prefix.last()?.checked_add(value)?;
        prefix.push(next);
    }
    Some(prefix)
}

fn range_sum(prefix: &[i64], left: usize, right: usize) -> Option<i64> {
    if left > right || right >= prefix.len() {
        return None;
    }
    prefix[right].checked_sub(prefix[left])
}

fn main() {
    let values = [3, -1, 4, 1, 5];
    let prefix = prefix_sums(&values).unwrap();
    println!("{}", range_sum(&prefix, 1, 4).unwrap());
}

#[cfg(test)]
mod tests {
    use super::{prefix_sums, range_sum};

    #[test]
    fn builds_prefixes_and_answers_half_open_ranges() {
        let prefix = prefix_sums(&[3, -1, 4, 1, 5]).unwrap();
        assert_eq!(prefix, [0, 3, 2, 6, 7, 12]);
        assert_eq!(range_sum(&prefix, 1, 4), Some(4));
        assert_eq!(range_sum(&prefix, 0, 5), Some(12));
        assert_eq!(range_sum(&prefix, 2, 2), Some(0));
    }

    #[test]
    fn handles_empty_input_and_invalid_ranges() {
        let prefix = prefix_sums(&[]).unwrap();
        assert_eq!(prefix, [0]);
        assert_eq!(range_sum(&prefix, 0, 0), Some(0));
        assert_eq!(range_sum(&prefix, 1, 1), None);
        let prefix = prefix_sums(&[1, 2]).unwrap();
        assert_eq!(range_sum(&prefix, 2, 1), None);
        assert_eq!(range_sum(&prefix, 0, 3), None);
    }

    #[test]
    fn reports_accumulator_overflow() {
        assert_eq!(prefix_sums(&[i64::MAX, 1]), None);
    }
}
```

코드를 `prefix_sum.rs`로 저장한 뒤 PowerShell에서 `rustc --test prefix_sum.rs -o prefix_sum_tests.exe`로 컴파일하고 `.\prefix_sum_tests.exe`로 테스트합니다. `rustc prefix_sum.rs -o prefix_sum.exe`와 `.\prefix_sum.exe`로 예제도 실행할 수 있습니다.

## 참고 자료

- [CP-Algorithms: Prefix sums](https://cp-algorithms.com/data_structures/prefix-sum.html)
- [Rust standard library: checked_add](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
- [Rust standard library: checked_sub](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_sub)
