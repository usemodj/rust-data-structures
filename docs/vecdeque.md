# VecDeque<T>

`VecDeque<T>`는 링 버퍼 기반의 양방향 큐입니다. 앞과 뒤에서 `push`·`pop`하는 연산은 분할 상환 `O(1)`이며, 임의 인덱스 접근은 `O(1)`입니다. 중간 삽입·삭제는 원소 이동 때문에 `O(n)`입니다.

## 구조 다이어그램

![VecDeque의 논리적 FIFO 순서와 래핑되는 링 버퍼를 보여주는 구조도](/assets/vecdeque.svg)

그림은 논리적 앞에서 뒤로 이어지는 FIFO 순서와 물리 배열의 끝을 넘어 래핑될 수 있는 저장 방식을 단순화한 개념도입니다. 실제 내부 분할과 인덱스 배치는 작업 이력과 용량에 따라 달라지며, 필요하면 `make_contiguous`가 요소를 이동할 수 있습니다.

### 그림 단계별 읽기

- 논리적 큐의 순서는 앞의 `A`, 다음 `B`, 뒤의 `C`이며, `pop_front`는 `A`를 꺼내고 `push_back`은 `C` 뒤에 값을 넣는 방향입니다.
- 물리 배열의 인덱스는 `0`부터 `5`까지인 여섯 칸으로 그려져 있고, `A`는 인덱스 `4`, `B`는 `5`, `C`는 `0`에 저장되어 있습니다.
- 인덱스 `5` 다음에 인덱스 `0`으로 이어지는 화살표가 배열 끝에서 시작으로의 래핑과 논리적 순서 `A`, `B`, `C`를 나타냅니다.
- 이 그림은 용량 `6`에서 세 값이 두 구간으로 나뉜다는 전제의 개념도이며, 실제 위치는 작업 이력에 따라 달라지고 연속 슬라이스가 필요하면 `make_contiguous`가 값을 이동할 수 있습니다.

- `pop_front`와 `push_back` 조합은 FIFO 큐에 적합합니다.
- 내부 저장 공간이 두 조각으로 나뉠 수 있으므로 연속 슬라이스가 필요하면 `make_contiguous`가 원소를 이동시킬 수 있습니다.
- 용량 확장은 재할당을 유발할 수 있으므로 예상 크기를 알면 `with_capacity`를 사용합니다.

```rust
use std::collections::VecDeque;

fn process_queue(input: &[i32]) -> Vec<i32> {
    let mut queue = VecDeque::from(input.to_vec());
    let mut output = Vec::new();
    while let Some(value) = queue.pop_front() {
        output.push(value * 2);
    }
    output
}

fn main() {
    println!("{:?}", process_queue(&[1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_in_fifo_order() {
        assert_eq!(process_queue(&[1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn handles_empty_queue() {
        assert!(process_queue(&[]).is_empty());
    }
}
```

프로젝트 폴더에서 `cargo test`를 실행하면 두 코드 블록을 각각 컴파일하고 테스트합니다. 첫 번째 블록은 FIFO 순서와 빈 큐를, 두 번째 블록은 용량 변경과 용량 0 동작을 확인합니다.

## 실무 활용과 선택 기준

`VecDeque<T>`는 작업 대기열, 이벤트 처리, 로그의 최근 항목처럼 양끝에서 원소를 처리하는 데이터에 적합합니다. `push_back`과 `pop_front`를 조합하면 FIFO가 되고, 최근 항목만 유지할 때는 오래된 항목을 `pop_front`로 제거할 수 있습니다.

단순한 후입선출 스택이나 인덱스 중심의 연속 버퍼는 `Vec<T>`가 더 단순하고 효율적입니다. 양끝 작업이 필요하지 않고 중간 위치를 자주 수정한다면 사용 패턴에 맞는 다른 컬렉션을 검토해야 하며, `LinkedList<T>`는 포인터와 노드 할당 비용 때문에 실제 연결 리스트 특성이 필요할 때만 선택하는 편이 좋습니다.

`VecDeque<T>`의 `with_capacity`는 최소 용량을 예약할 뿐 고정 용량을 보장하지 않습니다. 용량을 초과해 `push`하면 재할당이 발생할 수 있고, 재할당은 기존 원소에 대한 참조와 슬라이스를 무효화할 수 있습니다. `pop_front`로 얻은 값은 큐에서 소유권이 이동하며, 용량 0인 정책 큐는 삽입 전에 명시적으로 처리해야 무한한 확장을 피할 수 있습니다.

## 추가 예제

다음 예제는 고정 크기의 최근 항목 큐를 구현합니다. 용량이 0이면 기존 항목을 비우고 입력을 버립니다. 용량이 줄어든 경우에도 오래된 항목부터 제거해 새 한도를 지킵니다.

```rust
use std::collections::VecDeque;

fn push_recent(queue: &mut VecDeque<String>, value: String, capacity: usize) {
    if capacity == 0 {
        queue.clear();
        return;
    }
    while queue.len() >= capacity {
        queue.pop_front();
    }
    queue.push_back(value);
}

fn main() {
    let mut recent = VecDeque::new();
    for value in ["첫째", "둘째", "셋째"] {
        push_recent(&mut recent, value.to_owned(), 2);
    }
    println!("최근 항목: {recent:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_newest_items() {
        let mut queue = VecDeque::new();
        push_recent(&mut queue, "a".to_owned(), 2);
        push_recent(&mut queue, "b".to_owned(), 2);
        push_recent(&mut queue, "c".to_owned(), 2);
        assert_eq!(queue.into_iter().collect::<Vec<_>>(), vec!["b", "c"]);
    }

    #[test]
    fn shrinking_capacity_and_zero_keep_the_bound() {
        let mut queue = VecDeque::from(["a".to_owned(), "b".to_owned(), "c".to_owned()]);
        push_recent(&mut queue, "d".to_owned(), 2);
        assert_eq!(queue.iter().map(String::as_str).collect::<Vec<_>>(), vec!["c", "d"]);
        push_recent(&mut queue, "ignored".to_owned(), 0);
        assert!(queue.is_empty());
    }
}
```

참고 자료:

- [Rust 표준 라이브러리: `VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [Rust 표준 라이브러리 컬렉션 개요](https://doc.rust-lang.org/std/collections/index.html)
