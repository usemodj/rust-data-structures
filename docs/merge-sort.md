# 병합 정렬

## 동작 다이어그램

![배열을 나눈 뒤 정렬된 부분을 두 포인터로 병합하는 병합 정렬 다이어그램](/assets/merge-sort.svg)

그림처럼 `5, 1, 4, 3`을 두 부분으로 나누고 각각 `1, 5`와 `3, 4`로 정렬한 뒤, 각 부분의 앞을 가리키는 두 포인터로 더 작은 값을 차례로 고릅니다. 그 결과 `1, 3, 4, 5`가 됩니다.

### 그림 단계별 읽기

- 시작 입력 `5, 1, 4, 3`을 왼쪽 `5, 1`과 오른쪽 `4, 3` 두 부분으로 나눕니다.
- 각 부분을 정렬해 왼쪽을 `1, 5`, 오른쪽을 `3, 4`로 바꿉니다.
- 두 정렬된 부분의 앞 원소를 두 포인터로 비교하며 더 작은 값을 차례로 결과에 넣습니다.
- 병합 결과는 `1, 3, 4, 5`이며, 전제는 두 입력 부분이 이미 정렬되어 있어 앞에서부터 선택할 수 있다는 것입니다.

분할이 끝나면 병합 결과가 정렬된 상태가 되며, 같은 값에서는 왼쪽 원소를 먼저 선택하는 구현이 안정성을 유지합니다.

병합 정렬은 배열을 절반씩 나누어 정렬한 다음, 정렬된 두 부분 배열을 하나로 병합하는 안정적인 비교 정렬 알고리즘이다. 입력을 더 이상 나눌 수 없을 때까지 재귀적으로 분할하고, 병합 단계에서 두 포인터로 작은 원소를 선택한다.

원소 수를 `n`이라고 하면 분할 깊이는 `O(log n)`, 각 깊이에서 병합하는 총 비용은 `O(n)`이므로 시간 복잡도는 최선, 평균, 최악 모두 `O(n log n)`이다. 보조 배열을 포함한 추가 공간 복잡도는 `O(n)`이며, 이 구현은 같은 값의 상대적 순서를 보존하는 안정 정렬이다. 일반적인 배열 병합 정렬은 제자리(in-place) 정렬이 아니다.

## 실행 가능한 Rust 예제

```rust
fn merge_sort(values: &mut [i32]) {
    let middle = values.len() / 2;
    if values.len() < 2 {
        return;
    }
    merge_sort(&mut values[..middle]);
    merge_sort(&mut values[middle..]);

    let left = values[..middle].to_vec();
    let right = values[middle..].to_vec();
    let (mut i, mut j, mut output) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            values[output] = left[i];
            i += 1;
        } else {
            values[output] = right[j];
            j += 1;
        }
        output += 1;
    }
    while i < left.len() {
        values[output] = left[i];
        i += 1;
        output += 1;
    }
    while j < right.len() {
        values[output] = right[j];
        j += 1;
        output += 1;
    }
}

fn main() {
    let mut values = vec![5, 1, 4, 1, 3];
    merge_sort(&mut values);
    println!("{:?}", values);
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn sorts_unsorted_values_and_duplicates() {
        let mut values = vec![5, 1, 4, 1, 3];
        merge_sort(&mut values);
        assert_eq!(values, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn handles_empty_single_and_sorted_inputs() {
        let mut empty = Vec::new();
        merge_sort(&mut empty);
        assert!(empty.is_empty());
        let mut one = vec![7];
        merge_sort(&mut one);
        assert_eq!(one, vec![7]);
        let mut sorted = vec![1, 2, 3];
        merge_sort(&mut sorted);
        assert_eq!(sorted, vec![1, 2, 3]);
    }
}
```

PowerShell에서 파일을 `merge_sort_example.rs`로 저장한 뒤 다음처럼 실행할 수 있다.

```powershell
rustc --test merge_sort_example.rs -o merge_sort_example_tests.exe; .\merge_sort_example_tests.exe
rustc merge_sort_example.rs -o merge_sort_example.exe; .\merge_sort_example.exe
```

## 실무 활용과 선택 기준

병합 정렬은 대용량 데이터를 일정한 성능으로 정렬하거나, 외부 저장 장치의 정렬된 조각을 병합할 때 활용합니다. **같은 키를 가진 레코드의 기존 순서를 보존해야 하는 데이터 처리에는 안정 정렬이 유리합니다.**

| 선택지 | 적합한 상황 | 시간과 추가 공간 |
| --- | --- | --- |
| 병합 정렬 | 안정성과 최악의 성능 보장이 필요할 때 | 최선·평균·최악 `O(n log n)`, `O(n)` |
| 퀵 정렬 | 추가 공간을 줄이고 평균 성능을 중시할 때 | 평균 `O(n log n)`, 최악 `O(n²)`, 구현에 따라 `O(log n)` |
| 힙 정렬 | 안정성 없이 최악 시간과 작은 추가 공간이 필요할 때 | 최악 `O(n log n)`, 추가 공간 `O(1)` |

- 최선, 평균, 최악의 시간 복잡도가 모두 `O(n log n)`이어야 하면 병합 정렬을 선택할 수 있습니다.
- 안정성이 필요 없고 평균적인 메모리 사용량을 줄여야 하면 일반적인 퀵 정렬이나 힙 정렬을 대안으로 검토합니다. 퀵 정렬은 평균 `O(n log n)`이지만 최악 `O(n²)`이 될 수 있고, 힙 정렬은 최악에도 `O(n log n)`이지만 안정적이지 않습니다.
- 병합 정렬은 보조 공간 `O(n)`이 필요하므로 제자리 정렬이 중요한 경우에는 적합하지 않을 수 있습니다. 이미 정렬된 데이터가 많아 비교 횟수를 줄이고 싶다면 입력 특성에 맞는 삽입 정렬도 대안이며, 최악 시간은 `O(n²)`입니다.
- 병합 과정에서 같은 키일 때 왼쪽 원소를 먼저 선택해야 안정성이 유지됩니다.

## 추가 예제

다음 프로그램은 키가 같은 레코드의 순서를 보존하는 안정 병합 정렬을 보여 줍니다. 빈 입력과 중복 키를 테스트하며, `order` 값으로 원래 순서가 유지되는지 확인합니다.

```rust
#[derive(Clone, Debug, PartialEq)]
struct Record {
    key: i32,
    order: &'static str,
}

fn merge_sort(records: &mut [Record]) {
    if records.len() < 2 {
        return;
    }
    let middle = records.len() / 2;
    merge_sort(&mut records[..middle]);
    merge_sort(&mut records[middle..]);

    let left = records[..middle].to_vec();
    let right = records[middle..].to_vec();
    let (mut i, mut j, mut output) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i].key <= right[j].key {
            records[output] = Record {
                key: left[i].key,
                order: left[i].order,
            };
            i += 1;
        } else {
            records[output] = Record {
                key: right[j].key,
                order: right[j].order,
            };
            j += 1;
        }
        output += 1;
    }
    while i < left.len() {
        records[output] = Record {
            key: left[i].key,
            order: left[i].order,
        };
        i += 1;
        output += 1;
    }
    while j < right.len() {
        records[output] = Record {
            key: right[j].key,
            order: right[j].order,
        };
        j += 1;
        output += 1;
    }
}

fn main() {
    let mut records = vec![
        Record { key: 2, order: "첫째" },
        Record { key: 1, order: "둘째" },
        Record { key: 2, order: "셋째" },
    ];
    merge_sort(&mut records);
    println!("{:?}", records);
}

#[cfg(test)]
mod tests {
    use super::{merge_sort, Record};

    #[test]
    fn preserves_order_for_duplicate_keys() {
        let mut records = vec![
            Record { key: 2, order: "첫째" },
            Record { key: 1, order: "둘째" },
            Record { key: 2, order: "셋째" },
        ];
        merge_sort(&mut records);
        assert_eq!(
            records,
            vec![
                Record { key: 1, order: "둘째" },
                Record { key: 2, order: "첫째" },
                Record { key: 2, order: "셋째" },
            ]
        );
    }

    #[test]
    fn handles_empty_input_and_all_duplicates() {
        let mut empty = Vec::new();
        merge_sort(&mut empty);
        assert!(empty.is_empty());

        let mut duplicates = vec![
            Record { key: 4, order: "첫째" },
            Record { key: 4, order: "둘째" },
            Record { key: 4, order: "셋째" },
        ];
        merge_sort(&mut duplicates);
        assert_eq!(duplicates[0].order, "첫째");
        assert_eq!(duplicates[1].order, "둘째");
        assert_eq!(duplicates[2].order, "셋째");
    }
}
```

각 Rust 코드 블록은 독립 프로그램입니다. 프로젝트 폴더에서 `cargo test`를 실행하면 모든 Rust 코드 블록을 자동으로 추출해 검증합니다.

## 참고 자료

- [Wikipedia: Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
- [Rust 표준 라이브러리: slice](https://doc.rust-lang.org/std/primitive.slice.html)
