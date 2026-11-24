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

## 참고 자료

- [Data Structures and Algorithms in Rust: 배낭 문제](https://dsar.rantai.dev/docs/part-iv/chapter-16/)
- [Wikipedia: Knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem)
- [Rust 표준 라이브러리: `u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)
