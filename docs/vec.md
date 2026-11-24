# Vec<T>

`Vec<T>`는 힙에 연속적으로 저장되는 가변 길이 배열입니다. 인덱스 접근은 평균적으로 `O(1)`이고, 끝에 원소를 추가하는 `push`는 분할 상환 `O(1)`입니다. 중간 삽입·삭제는 뒤 원소를 이동하므로 `O(n)`입니다.

## 구조 다이어그램

![Vec의 연속 버퍼에서 초기화된 요소와 여유 용량을 보여주는 구조도](/assets/vec.svg)

그림은 `Vec`의 개념적 모델로, `len`만큼 초기화된 요소가 연속으로 놓이고 `capacity`까지 여유 공간이 남는 모습을 나타냅니다. 실제 버퍼 주소와 용량 증가 방식은 구현과 할당자에 따라 달라질 수 있습니다.

### 그림 단계별 읽기

- 위 버퍼에는 `len = 4`인 초기화된 `A`, `B`, `C`, `D`가 앞에서부터 연속으로 배치되어 인덱스 `0`부터 `3`을 차지합니다.
- 아래 버퍼도 같은 네 값이 앞의 네 칸에 있고, `capacity = 6`이므로 뒤의 두 칸은 아직 사용하지 않는 여유 공간입니다.
- 따라서 `push`로 다섯 번째 값을 넣으면 여유 칸을 사용하고, 여섯 칸을 모두 사용한 뒤 추가하면 재할당과 데이터 이동이 일어날 수 있습니다.
- 그림은 요소가 연속된다는 전제를 보여주며, 실제 버퍼 주소와 용량 증가 방식은 구현과 할당자에 따라 달라질 수 있습니다.

- 용량이 부족하면 재할당과 복사가 일어날 수 있으므로 크기를 알면 `with_capacity`를 사용합니다.
- `remove(0)`처럼 앞에서 반복 삭제하면 느립니다. 순서가 필요 없으면 `swap_remove`를 고려하세요.
- 인덱스 오류를 피하려면 `get`으로 `Option`을 반환받을 수 있습니다.

```rust
fn sum_even(values: &[i32]) -> i32 {
    values.iter().copied().filter(|value| value % 2 == 0).sum()
}

fn main() {
    let mut numbers = Vec::with_capacity(4);
    numbers.extend([1, 2, 3]);
    numbers.push(4);
    println!("짝수 합: {}", sum_even(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_only_even_numbers() {
        assert_eq!(sum_even(&[1, 2, 3, 4, 6]), 12);
    }

    #[test]
    fn empty_input_is_zero() {
        assert_eq!(sum_even(&[]), 0);
    }
}
```

프로젝트 폴더에서 `cargo test`를 실행하면 두 코드 블록을 각각 컴파일하고 테스트합니다. 첫 번째 블록은 짝수 합과 빈 입력을, 두 번째 블록은 경계 인덱스와 빈 수집 결과를 검증합니다.

## 실무 활용과 선택 기준

`Vec<T>`는 센서 샘플, 검색 결과, 배치 작업처럼 원소를 순서대로 모아 반복 처리하거나 인덱스로 빠르게 읽는 데이터에 적합합니다. 예상 원소 수를 알면 `with_capacity`로 재할당을 줄일 수 있고, 소유한 값을 모아 반환하면 호출자가 그 버퍼의 소유권을 갖습니다.

양 끝에서 자주 넣고 빼는 큐는 `VecDeque<T>`가 더 적합합니다. 중간 삽입과 삭제가 많거나 순서를 유지할 필요가 없으면 `Vec<T>`의 `remove` 또는 `swap_remove` 비용과 순서 변경을 비교해야 합니다. `Vec<T>`의 `get`은 범위를 벗어날 때 패닉 대신 `None`을 반환하지만, 대괄호 인덱싱은 패닉을 일으킬 수 있습니다.

`Vec<T>`의 원소는 버퍼 안에 연속으로 저장되지만, 용량을 초과하는 추가 작업은 재할당으로 버퍼 주소와 원소 참조를 무효화할 수 있습니다. 원소에 대한 불변 참조가 이후에도 사용되는 동안에는 가변 접근인 `push`를 수행할 수 없으며, 장기 보관이 필요하면 값을 복제하거나 소유권을 명확히 이전해야 합니다.

## 추가 예제

다음 예제는 반복 가능한 입력을 한 번에 `Vec<T>`로 수집하고 `get`으로 안전하게 조회합니다. 빈 입력과 범위를 벗어난 인덱스는 모두 `None`이 됩니다.

```rust
fn collect_and_lookup<I>(input: I, index: usize) -> Option<i32>
where
    I: IntoIterator<Item = i32>,
{
    let values: Vec<i32> = input.into_iter().collect();
    values.get(index).copied()
}

fn main() {
    let value = collect_and_lookup([10, 20, 30], 1);
    println!("조회 결과: {value:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_value_at_valid_index() {
        assert_eq!(collect_and_lookup([10, 20, 30], 2), Some(30));
    }

    #[test]
    fn returns_none_at_boundaries() {
        assert_eq!(collect_and_lookup([10, 20], 2), None);
        assert_eq!(collect_and_lookup(std::iter::empty(), 0), None);
    }
}
```

참고 자료:

- [Rust 표준 라이브러리: `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [The Rust Book: 벡터](https://doc.rust-lang.org/book/ch08-01-vectors.html)
