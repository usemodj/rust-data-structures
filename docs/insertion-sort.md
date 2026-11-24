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

## 실행 가능한 Rust 예제

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
