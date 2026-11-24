# 정렬

## 동작 다이어그램

![섞인 수열을 비교하고 이동해 오름차순으로 만드는 정렬 다이어그램](/assets/sorting.svg)

그림은 `8, 3, 5, 1`처럼 섞인 일반적인 입력 수열을 비교·선택·이동하여 `1, 3, 5, 8`로 만드는 정렬의 공통 흐름을 나타냅니다. 구체적인 비교 순서는 알고리즘에 따라 달라집니다.

### 그림 단계별 읽기

- 시작 값은 `8, 3, 5, 1`이며, 그림은 네 값의 구체적인 인덱스나 비교 순서가 아니라 입력과 결과의 흐름을 보여 줍니다.
- 가운데의 `비교 · 선택 · 이동`은 알고리즘에 따라 값을 비교하고 위치를 바꾸는 변화를 뜻합니다.
- 화살표 아래 결과는 작은 값에서 큰 값 순서인 `1, 3, 5, 8`입니다.
- 그림의 전제는 오름차순 정렬이며, 오른쪽 설명처럼 중복 값이 있어도 값 자체는 결과에 보존됩니다.

결과는 작은 값에서 큰 값 순서이며, 같은 키의 상대 순서가 필요하면 안정 정렬을 선택하고 필요하지 않으면 불안정 정렬을 선택할 수 있습니다.

Rust 슬라이스의 `sort`는 안정 정렬이고 `sort_unstable`은 순서를 보존하지 않는 대신 보조 메모리를 적게 사용할 수 있는 정렬입니다. 비교 기반 정렬의 일반적인 시간 복잡도는 평균 `O(n log n)`이며, 최악 시간과 메모리 사용량은 선택한 구현과 입력에 좌우됩니다.

- 동일한 키를 가진 원소의 원래 순서가 중요하면 `sort_by`를 사용해 안정성을 유지하세요.
- 안정성이 필요 없고 성능·메모리가 우선이면 `sort_unstable`을 고려하세요.
- 비교 함수는 일관된 순서를 정의해야 합니다. 부동소수점 `NaN`처럼 전체 순서를 만들 수 없는 값은 별도 정책이 필요합니다.

```rust
fn sorted_scores(mut scores: Vec<i32>) -> Vec<i32> {
    scores.sort_unstable();
    scores
}

fn main() {
    println!("{:?}", sorted_scores(vec![8, 3, 5, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_ascending() {
        assert_eq!(sorted_scores(vec![8, 3, 5, 1]), vec![1, 3, 5, 8]);
    }

    #[test]
    fn keeps_duplicates() {
        assert_eq!(sorted_scores(vec![2, 1, 2]), vec![1, 2, 2]);
    }
}
```

테스트는 예제를 `sorting.rs`로 저장한 뒤 `rustc --test sorting.rs -o sorting_tests.exe`로 컴파일한 뒤 PowerShell에서 `.\sorting_tests.exe`를 실행합니다. 오름차순 정렬과 중복 보존을 검증합니다.

참고 자료:

- [Rust 표준 라이브러리: `slice::sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)
- [Rust 표준 라이브러리: `slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable)
