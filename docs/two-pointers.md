# 투 포인터

투 포인터 기법은 배열이나 문자열의 두 위치를 가리키는 인덱스를 이동시키며 전체 후보를 효율적으로 줄이는 방법이다. 이동 규칙은 문제의 불변식에 의존한다. 이 문서의 예제는 입력이 오름차순으로 정렬되어 있고, 각 원소가 `i32`이며, 서로 다른 두 인덱스의 합을 찾는다고 가정한다. 정렬되지 않은 입력에는 먼저 정렬하거나 다른 방법을 사용해야 하며, 정렬 비용은 별도로 계산해야 한다.

## 동작 다이어그램

![정렬된 배열에서 왼쪽과 오른쪽 포인터가 목표 합을 찾는 과정](/assets/two-pointers.svg)

정렬된 `[1, 2, 4, 8, 11]`에서 양끝을 가리키며 시작합니다. `1 + 11`은 10보다 크므로 오른쪽을 줄이고, `1 + 8`은 작으므로 왼쪽을 늘립니다. 결국 `2 + 8 = 10`을 찾아 인덱스 `(1, 3)`을 반환합니다.

### 그림 단계별 읽기

- 입력 배열은 오름차순 `[1, 2, 4, 8, 11]`이고 목표 합은 `10`이라는 전제에서 왼쪽 `L`은 인덱스 `0`의 `1`, 오른쪽 `R`은 인덱스 `4`의 `11`을 가리킵니다.
- 첫 단계의 `1 + 11 = 12`는 목표보다 크므로 `R`을 인덱스 `3`의 `8`로 한 칸 왼쪽 이동합니다.
- 다음 단계의 `1 + 8 = 9`는 목표보다 작으므로 `L`을 인덱스 `1`의 `2`로 한 칸 오른쪽 이동합니다.
- 마지막에 `2 + 8 = 10`이 되어 그림의 결과처럼 인덱스 `(1, 3)`의 쌍을 반환합니다.

두 포인터는 각자 한 방향으로만 이동하므로 이미 확인한 조합을 다시 보지 않습니다.

두 포인터를 양끝에 두고 합이 목표보다 작으면 왼쪽 포인터를 오른쪽으로, 크면 오른쪽 포인터를 왼쪽으로 이동한다. 합이 같으면 답을 반환한다. 각 포인터가 한 방향으로만 이동하므로 이미 정렬된 입력에서 탐색 시간 복잡도는 `O(n)`, 추가 공간 복잡도는 `O(1)`이다. 정렬까지 포함하면 보통 시간 복잡도는 `O(n log n)`이며, 원래 인덱스가 필요하면 값과 인덱스를 함께 보존해야 한다.

## 실행 가능한 Rust 예제

```rust
fn two_sum_sorted(values: &[i32], target: i32) -> Option<(usize, usize)> {
    if values.len() < 2 {
        return None;
    }
    let (mut left, mut right) = (0, values.len() - 1);
    while left < right {
        let sum = values[left] as i64 + values[right] as i64;
        if sum == target as i64 {
            return Some((left, right));
        }
        if sum < target as i64 {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

fn main() {
    let values = [1, 2, 4, 8, 11];
    println!("{:?}", two_sum_sorted(&values, 10));
}

#[cfg(test)]
mod tests {
    use super::two_sum_sorted;

    #[test]
    fn finds_pair_in_sorted_values() {
        assert_eq!(two_sum_sorted(&[1, 2, 4, 8, 11], 10), Some((1, 3)));
    }

    #[test]
    fn handles_duplicates_negative_values_and_no_pair() {
        assert_eq!(two_sum_sorted(&[2, 2, 3], 4), Some((0, 1)));
        assert_eq!(two_sum_sorted(&[-5, -2, 1, 7], 2), Some((0, 3)));
        assert_eq!(two_sum_sorted(&[1, 2, 3], 20), None);
    }

    #[test]
    fn handles_short_inputs_and_wide_integer_sum() {
        assert_eq!(two_sum_sorted(&[], 0), None);
        assert_eq!(two_sum_sorted(&[1], 1), None);
        assert_eq!(two_sum_sorted(&[i32::MAX - 1, i32::MAX], -3), None);
    }
}
```

PowerShell에서 파일을 `two_pointers_example.rs`로 저장한 뒤 다음처럼 실행할 수 있다.

```powershell
rustc --test two_pointers_example.rs -o two_pointers_example_tests.exe; .\two_pointers_example_tests.exe
rustc two_pointers_example.rs -o two_pointers_example.exe; .\two_pointers_example.exe
```

## 참고 자료

- [CP-Algorithms: Two pointers method](https://cp-algorithms.com/others/two_pointers.html)
- [USACO Guide: Two Pointers](https://usaco.guide/silver/two-pointers)
- [Rust 표준 라이브러리: slice](https://doc.rust-lang.org/std/primitive.slice.html)
