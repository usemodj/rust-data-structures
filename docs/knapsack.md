# 0/1 배낭 문제와 동적 계획법

## 동작 다이어그램

![용량 5에서 무게와 가치가 최적인 물건을 고르는 0과 1 배낭 과정](/assets/knapsack.svg)

용량 5인 배낭에서 무게 3·가치 9와 무게 2·가치 6인 물건을 각각 한 번씩 골라 총 무게 5, 가치 15를 얻습니다. 무게 4·가치 12인 물건은 선택하면 다른 조합과 함께 용량을 넘으므로 최적 선택에서 제외됩니다.

### 그림 단계별 읽기

- 배낭의 용량은 5이고, 후보 물건 A는 무게 3·가치 9, B는 무게 2·가치 6, C는 무게 4·가치 12입니다.
- 0/1 규칙에 따라 A와 B를 각각 한 번씩 선택하고, 선택한 물건을 쪼개거나 반복해서 담지 않습니다.
- A와 B의 총 무게는 3 + 2 = 5로 용량 5에 맞고, 총 가치는 9 + 6 = 15입니다.
- C는 무게 4이므로 A 또는 B와 함께 고르면 무게가 각각 7 또는 6이 되어 용량 5를 넘으므로 그림에서는 미선택입니다.

각 물건은 선택하거나 선택하지 않는 0/1 결정만 가능하며 쪼개거나 여러 번 담지 않습니다. 따라서 가치-무게 비율만으로 판단하지 않고, 용량을 큰 쪽에서 갱신하는 동적 계획법을 사용합니다.

0/1 배낭 문제는 각 물건을 최대 한 번만 골라 용량을 넘지 않으면서 가치의 합을 최대로 만드는 문제다. 물건을 쪼갤 수 없으므로 단순히 가치 대 무게 비율로 고르는 탐욕법은 일반적으로 최적해를 보장하지 않는다.

`dp[c]`를 현재까지 본 물건으로 용량 `c`를 채웠을 때의 최대 가치라고 두고, 각 물건을 한 번씩 처리한다. 같은 물건을 다시 사용하지 않으려면 용량을 큰 쪽에서 작은 쪽으로 순회해야 한다. 물건의 무게가 `w`, 가치가 `v`일 때 `dp[c] = max(dp[c], dp[c - w] + v)`를 적용한다. 빈 입력, 0 용량, 0 무게 물건도 처리한다.

물건의 가치는 `u64`로 표현하고 덧셈은 `checked_add`로 수행한다. 합이 `u64::MAX`를 넘으면 조용히 래핑하지 않고 `Overflow` 오류를 반환하는 것이 이 예제의 오버플로 정책이다. 시간 복잡도는 물건 수를 `n`, 배낭 용량을 `C`라 할 때 초기화를 포함해 `O((n + 1)(C + 1))`, 추가 공간 복잡도는 `O(C + 1)`이다. 따라서 용량이 매우 크거나 물건 수가 많으면 시간과 메모리가 급격히 증가한다. 이 구현은 정수 무게와 0/1 선택만 지원하며, 실제 실행 환경의 메모리 한계를 초과하는 용량은 입력 전에 제한해야 한다.

## 어디에 사용되는가

0/1 배낭은 한정된 자원 안에서 각각 한 번만 선택할 수 있는 후보들의 총 가치를 최대화할 때 사용할 수 있다. 다음은 후보별 자원 사용량과 가치가 고정되어 있고 가치가 합산된다고 가정한 예다.

- **화물 선택:** 차량의 적재 무게를 용량으로, 각 화물의 무게와 운송 가치를 물건의 무게와 가치로 두고 운송할 화물을 고른다.
- **예산에 맞는 프로젝트 선정:** 예산을 용량으로, 각 프로젝트의 비용과 예상 효과를 무게와 가치로 두고 수행할 프로젝트를 고른다.
- **저장 공간 배분:** 저장 장치의 용량을 한도로, 각 파일의 크기와 중요도를 무게와 가치로 두고 보관할 파일을 고른다.

아래 첫 번째 예제는 정수 자원 한 가지와 음이 아닌 정수 가치만 다루며 최대 가치만 반환한다. 선택 목록은 뒤의 역추적 예제에서 구한다. 프로젝트 간 선행 관계나 여러 종류의 자원 제한처럼 추가 제약이 있다면 이 구현만으로는 표현할 수 없어 문제에 맞는 확장이 필요하다. 용량이 매우 크면 이 동적 계획법의 시간과 메모리 사용량도 커진다.

## 함수 설명

- **`knapsack(items: &[Item], capacity: usize) -> Result<u64, KnapsackError>`:** 물건 목록을 읽기 전용으로 받아 용량을 넘지 않고 얻을 수 있는 최대 가치를 계산한다. 반환값은 최대 가치이며 선택한 물건 목록은 포함하지 않는다. `dp[c]`에는 현재까지 처리한 물건 중 용량 `c` 이하로 담을 수 있는 최대 가치를 저장한다.
- **`main()`:** 예제 물건 세 개와 용량 `5`로 `knapsack`을 호출한다. 성공하면 `최대 가치: 15`를 출력하고, 두 오류 종류에 따라 각각 오류 메시지를 출력한다.

`capacity`는 배낭에 담을 수 있는 물건들의 총 무게 상한이다. 총 무게가 이 값과 같을 필요는 없고, 이하여야 한다. 예를 들어 `capacity = 5`이면 무게 3인 A와 무게 2인 B를 함께 담을 수 있다.

`dp`의 크기는 `capacity + 1`이다. 용량 `0`부터 `capacity`까지 각각의 최대 가치를 저장하기 때문이다. 예를 들어 `capacity = 5`이면 `dp`는 6칸이고, `dp[3]`은 용량이 3일 때의 최대 가치, `dp[5]`는 최종 답이다. `knapsack`은 이 길이를 계산하고 공간을 확보해 모든 칸을 0으로 초기화한다. 길이 계산이 넘치거나 공간 확보에 실패하면 `CapacityTooLarge`를 반환한다.

용량을 초과하는 물건은 건너뛰고, 나머지는 용량을 역순으로 순회하며 기존 가치와 물건을 추가한 후보를 비교한다. 역순 순회는 같은 물건을 한 번의 처리에서 반복해 사용하는 것을 방지한다. 무게가 0인 물건도 각 용량에서 한 번만 더한다. 후보 가치를 `checked_add`로 계산할 때 합이 `u64` 범위를 벗어나면 `Overflow`를 반환한다. 계산이 끝나면 `Ok(dp[capacity])`를 반환한다.

테스트 함수는 Rust 테스트 하네스가 실행하며 모두 `knapsack`을 호출한다.

- **`finds_optimal_zero_one_selection()`:** 예제 물건에서 최대 가치가 `15`인지 확인한다.
- **`handles_empty_items_zero_capacity_and_zero_weight()`:** 빈 물건 목록, 용량 0, 무게 0인 물건을 확인한다.
- **`skips_items_that_do_not_fit()`:** 용량보다 무거운 물건을 선택하지 않는지 확인한다.
- **`rejects_capacity_length_overflow()`:** `capacity + 1`을 계산할 수 없을 때 `CapacityTooLarge`를 반환하는지 확인한다.
- **`reports_value_overflow_instead_of_wrapping()`:** 가치 합이 넘칠 때 `Overflow`를 반환하는지 확인한다.

## 예시로 보는 상태 변화 추적

아래 `main()`의 물건 A `(무게 3, 가치 9)`, B `(무게 2, 가치 6)`, C `(무게 4, 가치 12)`를 순서대로 처리하고 배낭 용량은 `5`로 둔다. `dp`의 인덱스는 용량 `0`부터 `5`까지이며, `dp[c]`는 지금까지 처리한 물건으로 용량 `c`를 넘지 않고 얻을 수 있는 최대 가치다.

처음에는 물건을 하나도 처리하지 않았으므로 `dp = [0, 0, 0, 0, 0, 0]`이다. 각 물건에 대해 용량 `c`를 `5`부터 해당 물건의 무게까지 역순으로 순회한다. 각 행의 `dp`는 해당 비교를 마친 직후의 상태다.

| 물건 | 용량 `c` | 후보 `dp[c - 무게] + 가치` | 기존 `dp[c]`와 비교 | 처리 후 `dp` |
| --- | --- | --- | --- | --- |
| A | 5 | `dp[2] + 9 = 9` | `0`에서 `9`로 갱신 | `[0, 0, 0, 0, 0, 9]` |
| A | 4 | `dp[1] + 9 = 9` | `0`에서 `9`로 갱신 | `[0, 0, 0, 0, 9, 9]` |
| A | 3 | `dp[0] + 9 = 9` | `0`에서 `9`로 갱신 | `[0, 0, 0, 9, 9, 9]` |
| B | 5 | `dp[3] + 6 = 15` | `9`에서 `15`로 갱신 | `[0, 0, 0, 9, 9, 15]` |
| B | 4 | `dp[2] + 6 = 6` | 기존 `9`가 더 큼 | `[0, 0, 0, 9, 9, 15]` |
| B | 3 | `dp[1] + 6 = 6` | 기존 `9`가 더 큼 | `[0, 0, 0, 9, 9, 15]` |
| B | 2 | `dp[0] + 6 = 6` | `0`에서 `6`으로 갱신 | `[0, 0, 6, 9, 9, 15]` |
| C | 5 | `dp[1] + 12 = 12` | 기존 `15`가 더 큼 | `[0, 0, 6, 9, 9, 15]` |
| C | 4 | `dp[0] + 12 = 12` | `9`에서 `12`로 갱신 | `[0, 0, 6, 9, 12, 15]` |

물건보다 작은 용량은 순회하지 않는다. B를 처리할 때 용량 `4`는 용량 `2`보다 먼저 계산되므로 아직 B가 반영되지 않은 `dp[2] = 0`을 사용한다. 반대로 작은 용량부터 갱신하면 `dp[2] = 6`을 다시 사용해 B를 두 번 담는 잘못된 후보 `12`가 생긴다.

최종 결과는 `Ok(dp[5]) = Ok(15)`이고 `main()`은 `최대 가치: 15`를 출력한다. 이 값은 A와 B를 각각 한 번 선택한 가치 `9 + 6`에 해당하지만, 함수 자체는 선택한 물건 목록을 반환하지 않는다.

## 실행 가능한 Rust 예제

```rust
#[derive(Clone, Copy, Debug)]
struct Item {
    weight: usize,
    value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KnapsackError {
    Overflow,
    CapacityTooLarge,
}

fn knapsack(items: &[Item], capacity: usize) -> Result<u64, KnapsackError> {
    let len = capacity.checked_add(1).ok_or(KnapsackError::CapacityTooLarge)?;
    let mut dp = Vec::new();
    dp.try_reserve_exact(len).map_err(|_| KnapsackError::CapacityTooLarge)?;
    dp.resize(len, 0_u64);

    for &item in items {
        if item.weight > capacity {
            continue;
        }
        for current_capacity in (item.weight..=capacity).rev() {
            let candidate = dp[current_capacity - item.weight]
                .checked_add(item.value)
                .ok_or(KnapsackError::Overflow)?;
            if candidate > dp[current_capacity] {
                dp[current_capacity] = candidate;
            }
        }
    }

    Ok(dp[capacity])
}

fn main() {
    let items = [
        Item { weight: 3, value: 9 },
        Item { weight: 2, value: 6 },
        Item { weight: 4, value: 12 },
    ];
    match knapsack(&items, 5) {
        Ok(value) => println!("최대 가치: {value}"),
        Err(KnapsackError::Overflow) => println!("가치 합이 u64 범위를 넘었습니다."),
        Err(KnapsackError::CapacityTooLarge) => println!("용량이 너무 큽니다."),
    }
}

#[cfg(test)]
mod tests {
    use super::{knapsack, Item, KnapsackError};

    #[test]
    fn finds_optimal_zero_one_selection() {
        let items = [
            Item { weight: 3, value: 9 },
            Item { weight: 2, value: 6 },
            Item { weight: 4, value: 12 },
        ];
        assert_eq!(knapsack(&items, 5), Ok(15));
    }

    #[test]
    fn handles_empty_items_zero_capacity_and_zero_weight() {
        assert_eq!(knapsack(&[], 10), Ok(0));
        assert_eq!(knapsack(&[Item { weight: 1, value: 7 }], 0), Ok(0));
        let items = [
            Item { weight: 0, value: 5 },
            Item { weight: 2, value: 8 },
        ];
        assert_eq!(knapsack(&items, 2), Ok(13));
    }

    #[test]
    fn skips_items_that_do_not_fit() {
        let items = [Item { weight: 4, value: 100 }];
        assert_eq!(knapsack(&items, 3), Ok(0));
    }

    #[test]
    fn rejects_capacity_length_overflow() {
        assert_eq!(knapsack(&[], usize::MAX), Err(KnapsackError::CapacityTooLarge));
    }

    #[test]
    fn reports_value_overflow_instead_of_wrapping() {
        let items = [
            Item { weight: 1, value: u64::MAX },
            Item { weight: 1, value: 1 },
        ];
        assert_eq!(knapsack(&items, 2), Err(KnapsackError::Overflow));
    }
}
```

PowerShell에서 내용을 `knapsack_example.rs`로 저장한 뒤 다음 명령으로 테스트와 실행 파일을 만들 수 있다.

```powershell
rustc --test knapsack_example.rs -o knapsack_example_tests.exe; .\knapsack_example_tests.exe
rustc knapsack_example.rs -o knapsack_example.exe; .\knapsack_example.exe
```

## 선택한 물건 역추적하기

앞의 1차원 `dp`는 최대 가치만 남기므로 선택 목록을 알기 어렵다. 다음 독립 예제는 `dp[i][c]`에 앞에서부터 `i`개의 물건을 사용해 용량 `c` 이하로 얻을 수 있는 최대 가치를 저장한다. 마지막 행에서 시작해 이전 행과 값이 다르면 해당 물건을 선택한 것이므로 인덱스를 기록하고 남은 용량에서 그 무게를 뺀다. 값이 같으면 선택하지 않는다. 최적해가 여러 개면 이 규칙에 따라 뒤쪽 물건을 선택하지 않은 해를 우선한다.

반환값의 물건 목록은 입력 `items`의 **0부터 시작하는 인덱스**를 입력 순서대로 담는다. 같은 무게와 가치를 가진 물건도 서로 다른 인덱스로 구별한다. 무게가 0인 물건은 남은 용량이 바뀌지 않아도 이전 행으로 이동하므로 최대 한 번만 선택한다. 이 방식은 시간과 추가 공간 모두 `O((n + 1)(C + 1))`이 필요하며, 앞의 1차원 방식보다 더 많은 공간을 사용한다.

### 예시로 보는 상태 변화 추적

아래 `main()`의 입력은 A `(인덱스 0, 무게 3, 가치 9)`, B `(인덱스 1, 무게 2, 가치 6)`, C `(인덱스 2, 무게 4, 가치 12)`이고 용량은 `5`다. `dp[i][c]`는 앞에서부터 `i`개의 물건만 고려했을 때 용량 `c` 이하로 얻을 수 있는 최대 가치다. `rows = 4`, `columns = 6`이므로 `dp`는 물건을 처리한 개수 `0`부터 `3`까지 4개 행, 용량 `0`부터 `5`까지 6개 열을 가진다.

#### 1. `dp` 표 채우기

첫 행은 물건을 고르지 않은 상태이므로 모두 `0`이다. 각 물건에 대해 용량을 `0`부터 `5`까지 순회해 새 행을 만든다. 물건이 들어가면 이전 행의 기존 가치 `dp[i - 1][c]`와 선택 후보 `dp[i - 1][c - 무게] + 가치` 중 큰 값을 저장하고, 들어가지 않으면 기존 가치만 저장한다.

| 처리한 물건 | 행 | 용량 0 | 용량 1 | 용량 2 | 용량 3 | 용량 4 | 용량 5 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 없음 | `dp[0]` | 0 | 0 | 0 | 0 | 0 | 0 |
| A | `dp[1]` | 0 | 0 | 0 | 9 | 9 | 9 |
| A, B | `dp[2]` | 0 | 0 | 6 | 9 | 9 | 15 |
| A, B, C | `dp[3]` | 0 | 0 | 6 | 9 | 12 | 15 |

각 행을 채우는 계산은 다음과 같다. `previous`는 바로 이전 `dp` 행을 가리킨다.

- **`i = 0`:** 물건이 없으므로 `[0, 0, 0, 0, 0, 0]`이다.
- **`i = 1`, A:** `c = 0..=2`에서는 A를 담을 수 없어 이전 값 `0`을 유지한다. `c = 3..=5`에서는 `previous[c - 3] + 9 = 9`가 기존 값보다 크므로 `[0, 0, 0, 9, 9, 9]`가 된다.
- **`i = 2`, B:** `c = 0..=1`은 B를 담을 수 없어 `0`을 유지한다. `c = 2`는 `max(0, previous[0] + 6) = 6`, `c = 3`은 `max(9, previous[1] + 6) = 9`, `c = 4`는 `max(9, previous[2] + 6) = 9`, `c = 5`는 `max(9, previous[3] + 6) = 15`다. 결과 행은 `[0, 0, 6, 9, 9, 15]`다.
- **`i = 3`, C:** `c = 0..=3`은 C를 담을 수 없어 이전 값 `0, 0, 6, 9`를 유지한다. `c = 4`는 `max(9, previous[0] + 12) = 12`, `c = 5`는 `max(15, previous[1] + 12) = 15`다. 결과 행은 `[0, 0, 6, 9, 12, 15]`다.

후보는 항상 **이전 행**에서 계산하므로 현재 물건을 같은 행에서 여러 번 사용하지 않는다. 모든 행을 채운 뒤 최대 가치는 `dp[3][5] = 15`다.

#### 2. 선택한 물건 역추적

`remaining = 5`, `selected = []`로 시작해 `i`를 `3`부터 `1`까지 거꾸로 순회한다. 현재 행과 이전 행의 값이 다르면 물건 `i - 1`을 선택하고 그 무게를 `remaining`에서 뺀다. 표의 `selected`는 각 단계가 끝난 직후의 인덱스 목록이다.

| `i` | 물건 | 비교 전 `remaining` | 현재 행과 이전 행 비교 | 처리 | 처리 후 `remaining` | 처리 후 `selected` |
| --- | --- | --- | --- | --- | --- | --- |
| 3 | C (인덱스 2) | 5 | `dp[3][5] = 15`, `dp[2][5] = 15` | 값이 같아 건너뜀 | 5 | `[]` |
| 2 | B (인덱스 1) | 5 | `dp[2][5] = 15`, `dp[1][5] = 9` | B 선택, 무게 2를 뺌 | 3 | `[1]` |
| 1 | A (인덱스 0) | 3 | `dp[1][3] = 9`, `dp[0][3] = 0` | A 선택, 무게 3을 뺌 | 0 | `[1, 0]` |

역순으로 모인 `selected`를 뒤집으면 `[0, 1]`이 된다. 함수는 `Ok((15, [0, 1]))`을 반환하고, `main()`은 최대 가치와 두 물건을 출력한다. 값이 같은 경우에는 다른 최적 선택이 있더라도 현재 물건을 선택하지 않는다는 점에 주의한다.

### 2차원 DP 계산과 역추적 상세 과정

현재 보고 계신 동적 계획법 · 0/1 배낭 노트의 선택한 물건 역추적하기(`knapsack_with_items`) 예제에서, 입력 `items` 물건 0 (무게 3, 가치 9), 물건 1 (무게 2, 가치 6), 물건 2 (무게 4, 가치 12)와 `capacity = 5`를 기준으로 2차원 `dp` 테이블 채우기와 역추적(`selected`, `remaining`) 상태 변화를 추적해 보겠습니다.

#### 1. 2차원 dp 테이블 채우기 (`rows = 4`, `columns = 6`)

`dp[i][c]`는 앞에서부터 `i`개(`0..i`) 물건만 고려해 용량 `c`(0부터 5까지) 이하로 얻을 수 있는 최대 가치입니다.

**`i = 0` (물건 없음):** `[0, 0, 0, 0, 0, 0]`

**`i = 1` (물건 0: 무게 3, 가치 9 처리):**

- `c = 0..=2`: `item.weight <= c`가 거짓이므로 이전 행 값 0 유지
- `c = 3..=5`: `previous[c - 3] + 9 = 0 + 9 = 9`로 갱신 → `[0, 0, 0, 9, 9, 9]`

**`i = 2` (물건 1: 무게 2, 가치 6 처리):**

- `c = 0..=1`: 0 유지
- `c = 2`: `max(0, previous[0] + 6) = 6`
- `c = 3`: `max(9, previous[1] + 6) = 9` (기존 9가 더 큼)
- `c = 4`: `max(9, previous[2] + 6) = 9` (기존 9가 더 큼)
- `c = 5`: `max(9, previous[3] + 6) = 15` (9 + 6으로 갱신)

결과 행: `[0, 0, 6, 9, 9, 15]`

**`i = 3` (물건 2: 무게 4, 가치 12 처리):**

- `c = 0..=3`: 0, 0, 6, 9 유지
- `c = 4`: `max(9, previous[0] + 12) = 12` (0 + 12로 갱신)
- `c = 5`: `max(15, previous[1] + 12) = 15` (기존 15가 더 큼)

결과 행: `[0, 0, 6, 9, 12, 15]`

#### 2. 역추적 루프 상태 변화 (`i`를 3에서 1까지 역순 순회)

초기값은 `remaining = 5`, `selected = []`입니다. 각 단계에서 `dp[i][remaining] != dp[i - 1][remaining]`이면 물건 `i - 1`을 선택하고 `remaining`에서 `items[i - 1].weight`를 뺍니다.

- **`i = 3` (물건 2 검사, `remaining = 5`):** `dp[3][5]`는 15, `dp[2][5]`도 15로 같으므로 미선택. `remaining`은 5, `selected`는 `[]` 그대로입니다.
- **`i = 2` (물건 1 검사, `remaining = 5`):** `dp[2][5]`는 15, `dp[1][5]`는 9로 다르므로 인덱스 1 선택. `selected`가 `[1]`이 되고 `remaining`은 `5 - 2 = 3`이 됩니다.
- **`i = 1` (물건 0 검사, `remaining = 3`):** `dp[1][3]`은 9, `dp[0][3]`은 0으로 다르므로 인덱스 0 선택. `selected`가 `[1, 0]`이 되고 `remaining`은 `3 - 3 = 0`이 됩니다.

반복 종료 후 `.reverse()`: `selected`가 `[0, 1]`로 뒤집히며 최종적으로 `(15, vec![0, 1])`을 반환합니다.

#### 3. 역추적 단계 요약

| 루프 `i` | 검사 물건 (`i - 1`) | 시작 `remaining` | `dp[i][rem]` vs `dp[i-1][rem]` | 판정 | 종료 후 `remaining` | 종료 후 `selected` |
| --- | --- | --- | --- | --- | --- | --- |
| 3 | 물건 2 (`w=4`, `v=12`) | 5 | 15 vs 15 (같음) | 미선택 | 5 | `[]` |
| 2 | 물건 1 (`w=2`, `v=6`) | 5 | 15 vs 9 (다름) | 선택 (1) | 3 | `[1]` |
| 1 | 물건 0 (`w=3`, `v=9`) | 3 | 9 vs 0 (다름) | 선택 (0) | 0 | `[1, 0]` → 역순 `[0, 1]` |

동점(`dp[i][rem] == dp[i - 1][rem]`)일 때 건너뛰기 때문에, 같은 가치를 내는 조합이 여러 개일 때는 뒤쪽 인덱스 물건을 고르지 않는 해가 자연스럽게 선택됩니다.

```rust
#[derive(Clone, Copy, Debug)]
struct Item {
    weight: usize,
    value: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum KnapsackError {
    SizeOverflow,
    AllocationFailed,
    ValueOverflow,
}

fn knapsack_with_items(
    items: &[Item],
    capacity: usize,
) -> Result<(u64, Vec<usize>), KnapsackError> {
    let columns = capacity.checked_add(1).ok_or(KnapsackError::SizeOverflow)?;
    let rows = items.len().checked_add(1).ok_or(KnapsackError::SizeOverflow)?;
    let mut dp: Vec<Vec<u64>> = Vec::new();
    dp.try_reserve_exact(rows).map_err(|_| KnapsackError::AllocationFailed)?;

    let mut first_row = Vec::new();
    first_row.try_reserve_exact(columns).map_err(|_| KnapsackError::AllocationFailed)?;
    first_row.resize(columns, 0);
    dp.push(first_row);

    for (index, item) in items.iter().enumerate() {
        let previous = &dp[index];
        let mut row = Vec::new();
        row.try_reserve_exact(columns).map_err(|_| KnapsackError::AllocationFailed)?;
        for c in 0..columns {
            let mut best = previous[c];
            if item.weight <= c {
                let candidate = previous[c - item.weight]
                    .checked_add(item.value)
                    .ok_or(KnapsackError::ValueOverflow)?;
                best = best.max(candidate);
            }
            row.push(best);
        }
        dp.push(row);
    }

    let mut selected = Vec::new();
    let mut remaining = capacity;
    for i in (1..=items.len()).rev() {
        if dp[i][remaining] != dp[i - 1][remaining] {
            selected.try_reserve(1).map_err(|_| KnapsackError::AllocationFailed)?;
            selected.push(i - 1);
            remaining -= items[i - 1].weight;
        }
    }
    selected.reverse();
    Ok((dp[items.len()][capacity], selected))
}

fn main() {
    let items = [
        Item { weight: 3, value: 9 },
        Item { weight: 2, value: 6 },
        Item { weight: 4, value: 12 },
    ];
    match knapsack_with_items(&items, 5) {
        Ok((value, selected)) => {
            println!("최대 가치: {value}, 선택한 인덱스: {selected:?}");
            for index in selected {
                println!("물건 {index}: {:?}", items[index]);
            }
        }
        Err(error) => eprintln!("계산 실패: {error:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{knapsack_with_items, Item, KnapsackError};

    #[test]
    fn reconstructs_the_optimal_selection() {
        let items = [
            Item { weight: 3, value: 9 },
            Item { weight: 2, value: 6 },
            Item { weight: 4, value: 12 },
        ];
        assert_eq!(knapsack_with_items(&items, 5), Ok((15, vec![0, 1])));
    }

    #[test]
    fn handles_zero_weight_and_zero_capacity() {
        let items = [
            Item { weight: 0, value: 5 },
            Item { weight: 0, value: 3 },
            Item { weight: 1, value: 10 },
        ];
        assert_eq!(knapsack_with_items(&items, 0), Ok((8, vec![0, 1])));
        assert_eq!(knapsack_with_items(&[], 0), Ok((0, vec![])));
    }

    #[test]
    fn skips_later_items_on_ties() {
        let items = [
            Item { weight: 2, value: 6 },
            Item { weight: 2, value: 6 },
        ];
        assert_eq!(knapsack_with_items(&items, 2), Ok((6, vec![0])));
        assert_eq!(knapsack_with_items(&items, 4), Ok((12, vec![0, 1])));
    }

    #[test]
    fn reports_size_and_value_overflow() {
        assert_eq!(knapsack_with_items(&[], usize::MAX), Err(KnapsackError::SizeOverflow));
        let items = [
            Item { weight: 0, value: u64::MAX },
            Item { weight: 0, value: 1 },
        ];
        assert_eq!(knapsack_with_items(&items, 0), Err(KnapsackError::ValueOverflow));
    }
}
```

`SizeOverflow`는 행 또는 열의 개수를 계산할 수 없을 때, `AllocationFailed`는 필요한 공간을 확보하지 못할 때, `ValueOverflow`는 후보 가치의 합이 `u64` 범위를 벗어날 때 반환한다. 위 코드를 별도의 `knapsack_with_items.rs`로 저장한 뒤 다음과 같이 실행한다.

```powershell
rustc --test knapsack_with_items.rs -o knapsack_with_items_tests.exe; .\knapsack_with_items_tests.exe
rustc knapsack_with_items.rs -o knapsack_with_items.exe; .\knapsack_with_items.exe
```

## 참고 자료

- [Data Structures and Algorithms in Rust: 배낭 문제](https://dsar.rantai.dev/docs/part-iv/chapter-16/)
- [Wikipedia: Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem)
- [Rust 표준 라이브러리: `u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)
