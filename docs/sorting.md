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

Rust 슬라이스의 `sort`는 안정 정렬이고 `sort_unstable`은 같은 값의 기존 순서를 **보장하지 않는** 정렬입니다. 불안정 정렬이라고 해서 같은 값이 반드시 재배치되는 것은 아닙니다. `sort`는 추가 메모리를 할당할 수 있고 `sort_unstable`은 추가 할당 없이 제자리에서 정렬합니다. 표준 라이브러리의 두 메서드는 최악 `O(n log n)`을 보장하지만 구체적인 알고리즘은 Rust 버전에 따라 바뀔 수 있습니다.

## 정렬을 선택할 때

**안정성**은 비교 키가 같은 원소의 입력 순서를 정렬 후에도 유지하는 성질입니다. 예를 들어 점수가 같은 `민지`, `지훈`을 점수만으로 안정 정렬하면 두 사람의 상대 순서는 그대로입니다. 이름까지 비교 키에 넣으면 두 항목은 더 이상 같은 키가 아니므로 이름 기준으로 순서가 결정됩니다.

| 방법 | 시간 복잡도 | 추가 공간 | 안정성 및 활용 |
| --- | --- | --- | --- |
| 삽입 정렬 | 최선 `O(n)`, 평균·최악 `O(n²)` | `O(1)` | 안정적이며 작은 입력이나 거의 정렬된 입력에 적합 |
| 병합 정렬 | 최선·평균·최악 `O(n log n)` | 병합 정렬 문서의 구현은 `O(n)` | 안정적이며 큰 입력에서 예측 가능한 시간에 적합 |
| 퀵 정렬 | 평균 `O(n log n)`, 최악 `O(n²)` | 퀵 정렬 문서의 호출 스택은 `O(log n)` | 불안정하며 제자리 분할을 사용 |
| `sort` / `sort_by_key` | 최악 `O(n log n)` | 추가 할당 가능 | 안정성이 필요할 때 |
| `sort_unstable` / `sort_unstable_by_key` | 최악 `O(n log n)` | 추가 할당 없음 | 같은 키의 상대 순서가 중요하지 않을 때 |

표의 시간 복잡도는 비교와 키 계산 비용을 상수로 가정합니다. 시간과 공간은 알고리즘의 구현 방식과 입력에 따라 달라집니다. 비교 기반 정렬이라도 삽입 정렬처럼 최악이 `O(n²)`인 방법이 있고, 숫자의 범위나 표현을 활용하는 비비교 정렬도 있습니다. 복잡도만으로 실제 속도를 단정하기보다 데이터 크기, 중복, 정렬 상태와 메모리 제한을 기준으로 측정하세요. 정렬 키 계산이 비싸다면 키를 저장하는 대신 추가 메모리를 사용하는 `sort_by_cached_key`도 고려할 수 있습니다.

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

## 실용 예제 1: 안정 정렬과 복합 기준

학생 목록을 점수 오름차순으로 묶을 때, 점수가 같은 학생의 기존 순서를 보존하려면 `sort_by_key`를 사용합니다. 반면 순위를 **점수 내림차순, 같은 점수는 ID 오름차순**으로 정한다면 `(Reverse(score), id)`를 키로 사용합니다. 이 예제는 ID가 유일하다고 가정합니다. ID까지 같은 키가 존재하면 `sort_unstable_by_key`는 그 원소들의 상대 순서를 보장하지 않습니다.

```rust
use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Student {
    id: u32,
    name: &'static str,
    score: i32,
}

fn group_by_score(students: &mut [Student]) {
    // 안정 정렬: 같은 score의 기존 순서를 보존한다.
    students.sort_by_key(|student| student.score);
}

fn rank_by_score(students: &mut [Student]) {
    // 점수는 내림차순, 점수가 같으면 ID는 오름차순이다.
    students.sort_unstable_by_key(|student| (Reverse(student.score), student.id));
}

fn main() {
    let students = [
        Student { id: 2, name: "민지", score: 90 },
        Student { id: 9, name: "서연", score: 80 },
        Student { id: 1, name: "지훈", score: 90 },
    ];
    let mut groups = students.clone();
    group_by_score(&mut groups);
    println!("점수별 그룹: {groups:?}");

    let mut ranking = students;
    rank_by_score(&mut ranking);
    println!("순위: {ranking:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Student> {
        vec![
            Student { id: 2, name: "민지", score: 90 },
            Student { id: 9, name: "서연", score: 80 },
            Student { id: 1, name: "지훈", score: 90 },
        ]
    }

    #[test]
    fn stable_sort_preserves_equal_score_order() {
        let mut students = sample();
        group_by_score(&mut students);
        assert_eq!(students.iter().map(|student| student.id).collect::<Vec<_>>(), [9, 2, 1]);
    }

    #[test]
    fn ranking_uses_id_to_break_score_ties() {
        let mut students = sample();
        rank_by_score(&mut students);
        assert_eq!(students.iter().map(|student| student.id).collect::<Vec<_>>(), [1, 2, 9]);
    }

    #[test]
    fn handles_empty_single_and_extreme_scores() {
        let mut empty: Vec<Student> = vec![];
        group_by_score(&mut empty);
        rank_by_score(&mut empty);
        assert!(empty.is_empty());
        let mut one = vec![Student { id: 1, name: "하나", score: 0 }];
        group_by_score(&mut one);
        rank_by_score(&mut one);
        assert_eq!(one[0].id, 1);
        let mut extremes = vec![
            Student { id: 1, name: "최소", score: i32::MIN },
            Student { id: 2, name: "최대", score: i32::MAX },
        ];
        rank_by_score(&mut extremes);
        assert_eq!(extremes.iter().map(|student| student.id).collect::<Vec<_>>(), [2, 1]);
    }
}
```

안정 정렬 결과의 ID는 `9, 2, 1`입니다. 점수 `90`의 `민지`가 `지훈`보다 먼저였으므로 순서가 유지됩니다. 복합 키 순위는 `1, 2, 9`입니다. `Reverse`를 사용하면 `-score`로 음수를 만들 필요가 없어 `i32::MIN`을 부정할 때의 오버플로도 피할 수 있습니다.

## 실용 예제 2: 부동소수점과 NaN 정렬

`f64`는 `NaN` 때문에 일반적인 `Ord`를 구현하지 않습니다. `partial_cmp(...).unwrap()`은 `NaN`을 만나면 실패하므로, 모든 비트 패턴에 대해 일관된 순서가 필요한 경우 `f64::total_cmp`를 비교자로 사용합니다.

```rust
fn sort_floats(values: &mut [f64]) {
    values.sort_by(f64::total_cmp);
}

fn main() {
    let mut values = [3.0, f64::NAN, -0.0, 0.0, -2.0];
    sort_floats(&mut values);
    println!("{values:?}");
}

#[cfg(test)]
mod tests {
    use super::sort_floats;

    #[test]
    fn orders_signed_zero_infinities_and_nan() {
        let mut values = [f64::NAN, 0.0, f64::INFINITY, -0.0, f64::NEG_INFINITY];
        sort_floats(&mut values);
        assert_eq!(values[0], f64::NEG_INFINITY);
        assert_eq!(values[1].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(values[2].to_bits(), 0.0_f64.to_bits());
        assert_eq!(values[3], f64::INFINITY);
        assert!(values[4].is_nan());
    }

    #[test]
    fn handles_empty_and_duplicate_values() {
        let mut empty: [f64; 0] = [];
        sort_floats(&mut empty);
        assert!(empty.is_empty());
        let mut values = [2.5, 1.0, 2.5];
        sort_floats(&mut values);
        assert_eq!(values, [1.0, 2.5, 2.5]);
    }
}
```

`total_cmp`는 `-0.0`과 `+0.0`도 구분합니다. 위 예제의 양의 `NaN`은 무한대 뒤에 오지만, 모든 `NaN`을 무조건 끝에 보내는 정책은 아닙니다. 음의 `NaN` 등을 포함한 상세 순서는 `total_cmp`의 전체 순서를 따릅니다. 애플리케이션에서 `NaN`을 제외하거나 항상 마지막에 두어야 한다면 별도 정책을 정하세요.

각 Rust 코드 블록은 독립 프로그램입니다. 프로젝트 폴더에서 `cargo test --offline`을 실행하면 모든 문서의 Rust 코드 블록도 컴파일하고 테스트합니다. 삽입 정렬, 병합 정렬, 퀵 정렬의 구현은 왼쪽 학습 노트 메뉴에서 각각 확인할 수 있습니다.

## 참고 자료

- [Rust 표준 라이브러리: `slice::sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)
- [Rust 표준 라이브러리: `slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable)
- [Rust 표준 라이브러리: `slice::sort_by_key`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key)
- [Rust 표준 라이브러리: `f64::total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp)
