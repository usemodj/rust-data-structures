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

## 슬라이딩 윈도우의 활용

- **매출·시계열 분석:** 최근 7일간 매출 합계나 평균을 날짜별로 계산합니다.
- **요청 수 집계:** 일정 시간 동안 들어온 요청 수를 추적해 과도한 트래픽을 감지합니다.
- **문자열 탐색:** 연속된 부분 문자열에서 특정 패턴이나 조건을 찾습니다.
- **최대 구간 합:** 배열에서 길이가 `k`인 연속 구간 중 합이 가장 큰 구간을 찾습니다.

예를 들어 일별 매출의 최근 7일 합계를 한 날짜씩 계산할 때, 이전 합계에서 범위를 벗어난 하루의 매출을 빼고 새 날짜의 매출을 더할 수 있습니다. 매번 7일치 전체를 다시 더하지 않아도 됩니다. 문자열 조건이나 시간 기준 요청 수처럼 다른 문제에는 윈도우 크기와 유지할 정보에 맞는 별도 구현이 필요합니다.

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

## 예제: 최근 7일 매출 합계와 평균

다음은 실제 매출이 아닌 예시 데이터입니다. `daily_sales`가 날짜 오름차순의 빠진 날짜 없는 일별 기록이라고 가정하며, 날짜를 검증하거나 누락된 날짜를 채우지는 않습니다. 각 날짜에 해당일을 포함한 최근 최대 7일의 매출 합계와 평균을 구합니다. 처음 6일은 지금까지 기록된 일수로 평균을 계산합니다.

`VecDeque`가 가득 차면 가장 오래된 매출을 먼저 제거한 뒤 새 매출을 더합니다. 입력이 비었거나 `window_size`가 0이면 빈 결과를 반환하며, 합계가 `u64` 범위를 벗어나면 `Err(SalesError::Overflow)`를 반환합니다. 평균은 `f64` 근삿값이므로 정확한 금액 계산에는 별도의 반올림 규칙이 필요합니다.

```rust
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DailySales<'a> {
    pub date: &'a str,
    pub revenue: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RollingSalesSummary<'a> {
    pub date: &'a str,
    pub window_days: usize,
    pub sum: u64,
    pub average: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SalesError {
    Overflow,
}

pub fn rolling_sales_stats<'a>(
    daily_sales: &[DailySales<'a>],
    window_size: usize,
) -> Result<Vec<RollingSalesSummary<'a>>, SalesError> {
    if window_size == 0 || daily_sales.is_empty() {
        return Ok(Vec::new());
    }

    let mut window = VecDeque::new();
    let mut current_sum = 0u64;
    let mut results = Vec::with_capacity(daily_sales.len());

    for entry in daily_sales {
        if window.len() == window_size {
            // 반환된 Option<T>가 Some(v)이면 내부 값 v를 꺼내 evicted에 넘깁니다.
            // None이면 "full window is not empty"라는 원인 설명을 남기고 프로그램을 패닉 종료시킵니다. 
            // 즉, 이 줄에 도달할 때는 window가 절대 비어 있으면 안 된다는 개발자의 전제(불변식)를 코드로 드러내는 역할을 합니다.
            let evicted = window.pop_front().expect("full window is not empty");
            current_sum -= evicted;
        }
        current_sum = current_sum.checked_add(entry.revenue).ok_or(SalesError::Overflow)?;
        window.push_back(entry.revenue);

        let window_days = window.len();
        results.push(RollingSalesSummary {
            date: entry.date,
            window_days,
            sum: current_sum,
            average: current_sum as f64 / window_days as f64,
        });
    }

    Ok(results)
}

fn main() -> Result<(), SalesError> {
    let daily_sales = [
        DailySales { date: "2026-03-01", revenue: 100_000 },
        DailySales { date: "2026-03-02", revenue: 120_000 },
        DailySales { date: "2026-03-03", revenue: 90_000 },
        DailySales { date: "2026-03-04", revenue: 150_000 },
        DailySales { date: "2026-03-05", revenue: 110_000 },
        DailySales { date: "2026-03-06", revenue: 130_000 },
        DailySales { date: "2026-03-07", revenue: 140_000 },
        DailySales { date: "2026-03-08", revenue: 160_000 },
        DailySales { date: "2026-03-09", revenue: 100_000 },
    ];
    for summary in rolling_sales_stats(&daily_sales, 7)? {
        println!(
            "{} ({}일): 합계 {}원, 평균 {:.2}원",
            summary.date, summary.window_days, summary.sum, summary.average
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{rolling_sales_stats, DailySales, SalesError};

    #[test]
    fn calculates_partial_and_full_windows_with_dates() {
        let sales = [
            DailySales { date: "2026-03-01", revenue: 100_000 },
            DailySales { date: "2026-03-02", revenue: 120_000 },
            DailySales { date: "2026-03-03", revenue: 90_000 },
            DailySales { date: "2026-03-04", revenue: 150_000 },
            DailySales { date: "2026-03-05", revenue: 110_000 },
            DailySales { date: "2026-03-06", revenue: 130_000 },
            DailySales { date: "2026-03-07", revenue: 140_000 },
            DailySales { date: "2026-03-08", revenue: 160_000 },
            DailySales { date: "2026-03-09", revenue: 100_000 },
        ];
        let results = rolling_sales_stats(&sales, 7).unwrap();
        assert_eq!(results.len(), 9);
        assert_eq!((results[0].date, results[0].window_days, results[0].sum), ("2026-03-01", 1, 100_000));
        assert_eq!(results[0].average, 100_000.0);
        assert_eq!((results[6].date, results[6].window_days, results[6].sum), ("2026-03-07", 7, 840_000));
        assert_eq!(results[6].average, 120_000.0);
        assert_eq!((results[7].date, results[7].window_days, results[7].sum), ("2026-03-08", 7, 900_000));
        assert!((results[7].average - 900_000.0 / 7.0).abs() < 1e-9);
        assert_eq!((results[8].date, results[8].window_days, results[8].sum), ("2026-03-09", 7, 880_000));
    }

    #[test]
    fn handles_empty_zero_and_large_windows() {
        let sales = [DailySales { date: "2026-03-01", revenue: 100 }];
        assert!(rolling_sales_stats(&[], 7).unwrap().is_empty());
        assert!(rolling_sales_stats(&sales, 0).unwrap().is_empty());
        let results = rolling_sales_stats(&sales, usize::MAX).unwrap();
        assert_eq!((results[0].window_days, results[0].sum), (1, 100));
    }

    #[test]
    fn reports_overflow_but_evicts_before_adding() {
        let sales = [
            DailySales { date: "2026-03-01", revenue: u64::MAX },
            DailySales { date: "2026-03-02", revenue: 1 },
        ];
        assert_eq!(rolling_sales_stats(&sales, 2), Err(SalesError::Overflow));
        let results = rolling_sales_stats(&sales, 1).unwrap();
        assert_eq!(results[1].sum, 1);
    }
}
```

위 예제의 출력은 다음과 같습니다.

```text
2026-03-01 (1일): 합계 100000원, 평균 100000.00원
2026-03-02 (2일): 합계 220000원, 평균 110000.00원
2026-03-03 (3일): 합계 310000원, 평균 103333.33원
2026-03-04 (4일): 합계 460000원, 평균 115000.00원
2026-03-05 (5일): 합계 570000원, 평균 114000.00원
2026-03-06 (6일): 합계 700000원, 평균 116666.67원
2026-03-07 (7일): 합계 840000원, 평균 120000.00원
2026-03-08 (7일): 합계 900000원, 평균 128571.43원
2026-03-09 (7일): 합계 880000원, 평균 125714.29원
```

예제를 `rolling_sales.rs`로 저장한 뒤 `rustc --test rolling_sales.rs -o rolling_sales_tests.exe`와 `.\rolling_sales_tests.exe`로 테스트할 수 있습니다. 시간 복잡도는 `O(n)`, 반환하는 결과를 포함한 추가 공간은 `O(n)`입니다.

## 참고 자료

- [CP-Algorithms: Sliding window technique](https://cp-algorithms.com/others/maximum_average_segment.html)
- [Rust standard library: Slice indexing](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust standard library: checked_add](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
