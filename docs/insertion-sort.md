# 삽입 정렬

## 동작 다이어그램

![정렬된 접두부에서 큰 원소를 오른쪽으로 밀고 값을 삽입하는 삽입 정렬 다이어그램](/assets/insertion-sort.svg)

그림은 정렬된 접두부 `2, 4`에 다음 값 `3`을 삽입하는 과정을 보여 줍니다. `3`보다 큰 `4`를 오른쪽으로 한 칸 이동해 빈자리를 만든 뒤 그 자리에 `3`을 넣어 `2, 3, 4`를 만듭니다.

### 그림 단계별 읽기

- 전제는 정렬된 접두부 `[2, 4]` 뒤에 다음 값 `3`이 놓인 상태입니다.
- `3`과 비교했을 때 큰 값 `4`를 오른쪽으로 한 칸 이동해 접두부 안에 빈자리를 만듭니다.
- 빈자리가 생긴 뒤 `3`을 그 위치로 옮기면 배열의 순서가 `2, 3, 4`로 바뀝니다.
- 결과에서도 접두부가 오름차순으로 정렬되어 있으며, 그림은 이 한 원소 삽입 단계만 보여 줍니다.

이처럼 각 단계가 끝날 때마다 접두부가 정렬되어 있고, 같은 값보다 큰 원소만 이동하므로 같은 값의 상대 순서를 보존합니다.

삽입 정렬은 정렬된 접두부를 한 원소씩 확장하는 안정적인 비교 정렬입니다. 작은 입력이나 거의 정렬된 입력에 적합합니다.

다음 원소보다 큰 값들을 오른쪽으로 한 칸씩 이동하고 빈 자리에 원소를 삽입합니다. 같은 값은 이동하지 않으므로 상대 순서가 보존됩니다. 큰 무작위 입력에는 보통 `O(n log n)` 비교 정렬이 더 적합합니다.

## 복잡도와 주의점

이미 정렬된 입력의 최선 시간은 `O(n)`, 평균과 역순 입력의 최악 시간은 `O(n²)`입니다. 추가 공간은 `O(1)`이며 안정 정렬입니다. 빈 슬라이스에서는 반복문이 실행되지 않습니다.

이 예제는 `i32` 값을 비교하고 이동하므로 값에 대한 산술 오버플로는 발생하지 않습니다. 비교 키나 합계를 별도로 계산한다면 적절한 자료형과 검사 연산을 사용하세요.

## 삽입 정렬의 활용

- **작은 데이터:** 구현이 간단하고 추가 공간이 `O(1)`이어서 적은 수의 원소를 정렬할 때 유용합니다.
- **거의 정렬된 데이터:** 이동할 원소가 적으면 빠르게 처리할 수 있으며, 이미 정렬된 입력은 `O(n)`에 처리합니다.
- **정렬된 목록에 새 항목 삽입:** 새 항목보다 큰 원소를 이동해 알맞은 위치에 넣는 데 같은 원리를 활용할 수 있습니다.
- **다른 정렬 알고리즘의 보조 단계:** 분할된 작은 구간을 정렬하는 데 삽입 정렬을 사용할 수 있습니다.

큰 무작위 입력은 평균 시간 복잡도가 `O(n²)`이므로 보통 `O(n log n)` 정렬을 선택하는 편이 좋습니다. 배열 기반 목록에 항목을 하나 삽입할 때도 뒤에 있는 원소들을 이동해야 하므로 최대 `O(n)`이 걸릴 수 있습니다.

## 실행 가능한 Rust 예제

### 예시로 보는 상태 변화 추적

아래 예제의 `values = [5, 2, 4, 2, 1]`을 사용합니다. 각 반복에서 `index` 위치의 값을 `value`에 보관하고, `position`을 왼쪽으로 이동하며 `value`보다 큰 원소를 오른쪽으로 복사합니다. 마지막으로 `values[position]`에 `value`를 넣습니다.

| 단계 | `index` | `value` | `while`문에서 이동한 후의 `values` | 최종 `position` | 삽입 후 `values` |
| --- | --- | --- | --- | --- | --- |
| 초기 상태 | - | - | - | - | `[5, 2, 4, 2, 1]` |
| 1회 | `1` | `2` | `[5, 5, 4, 2, 1]` | `0` | `[2, 5, 4, 2, 1]` |
| 2회 | `2` | `4` | `[2, 5, 5, 2, 1]` | `1` | `[2, 4, 5, 2, 1]` |
| 3회 | `3` | `2` | `[2, 4, 5, 5, 1]` → `[2, 4, 4, 5, 1]` | `1` | `[2, 2, 4, 5, 1]` |
| 4회 | `4` | `1` | `[2, 2, 4, 5, 5]` → `[2, 2, 4, 4, 5]` → `[2, 2, 2, 4, 5]` → `[2, 2, 2, 4, 5]` | `0` | `[1, 2, 2, 4, 5]` |

이동 중에는 원소를 복사하기 때문에 배열에 같은 값이 일시적으로 중복되어 보입니다. 보관한 `value`를 최종 `position`에 넣으면 정렬된 접두부가 한 칸 늘어납니다. 3회 반복에서는 앞의 `2`가 `value = 2`보다 크지 않아 이동하지 않으므로, 두 `2`의 상대 순서도 유지됩니다. 4회 반복의 마지막 이동은 `2`를 이미 `2`가 있는 칸에 복사하므로 배열의 표시가 변하지 않지만 `position`은 `0`이 됩니다. 그러면 비교를 멈추고 `1`을 맨 앞에 넣습니다.

`insertion_sort(&mut values)`는 반환값 없이 `values`를 직접 정렬하며, 예제의 출력은 `[1, 2, 2, 4, 5]`입니다.

```rust
fn insertion_sort(values: &mut [i32]) {
    for index in 1..values.len() {
        let value = values[index];
        let mut position = index;
        while position > 0 && values[position - 1] > value {
            values[position] = values[position - 1];
            position -= 1;
        }
        values[position] = value;
    }
}

fn main() {
    let mut values = [5, 2, 4, 2, 1];
    insertion_sort(&mut values);
    println!("{values:?}");
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn sorts_unsorted_values_and_duplicates() {
        let mut values = [5, 2, 4, 2, 1];
        insertion_sort(&mut values);
        assert_eq!(values, [1, 2, 2, 4, 5]);
    }

    #[test]
    fn handles_empty_and_single_element_slices() {
        let mut empty: [i32; 0] = [];
        insertion_sort(&mut empty);
        assert_eq!(empty, []);
        let mut one = [7];
        insertion_sort(&mut one);
        assert_eq!(one, [7]);
    }

    #[test]
    fn leaves_sorted_input_and_orders_reverse_input() {
        let mut sorted = [1, 2, 3];
        insertion_sort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
        let mut reverse = [3, 2, 1];
        insertion_sort(&mut reverse);
        assert_eq!(reverse, [1, 2, 3]);
    }
}
```

코드를 `insertion_sort.rs`로 저장한 뒤 PowerShell에서 `rustc --test insertion_sort.rs -o insertion_sort_tests.exe`로 컴파일하고 `.\insertion_sort_tests.exe`로 테스트합니다. `rustc insertion_sort.rs -o insertion_sort.exe`와 `.\insertion_sort.exe`로 예제도 실행할 수 있습니다.

## 참고 자료

- [Wikipedia: Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort)
- [Rust standard library: Slice indexing](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust standard library: Slices](https://doc.rust-lang.org/std/primitive.slice.html)
