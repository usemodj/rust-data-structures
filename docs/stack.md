# Vec로 구현하는 스택

스택은 마지막에 넣은 값이 먼저 나오는 LIFO 자료구조다. Rust 표준 라이브러리에는 전용 스택 타입 대신 `Vec<T>`를 스택으로 사용하는 관용적인 방법이 있다. `push`로 top에 넣고 `pop`으로 top에서 꺼내며, `last`로 top을 빌릴 수 있다.

## 구조 다이어그램

![Vec의 끝을 top으로 사용하는 스택의 LIFO 구조도](/assets/stack.svg)

그림은 `Vec`의 끝을 스택의 top으로 사용하는 개념적 모델입니다. 실제 `Vec`는 일반적인 연속 버퍼이며, 스택이라는 동작 규칙은 `push`와 `pop`을 끝에서 사용해 얻는 추상화입니다.

### 그림 단계별 읽기

- 아래에서 위로 `첫 번째`, `두 번째`, `세 번째`가 쌓여 있으며, 가장 위의 `세 번째`가 `Vec`의 끝이자 `top`입니다.
- `push`는 top 위에 값을 추가하고 `pop`은 top에서 값을 꺼내므로, 데이터 이동 방향은 `세 번째`에서 `두 번째`, `첫 번째` 순서입니다.
- 그림의 `pop → 세 번째 → 두 번째 → 첫 번째`는 마지막에 넣은 값이 먼저 나오는 LIFO 전제를 구체적으로 보여줍니다.
- 세 값이 들어 있는 개념적 스택을 나타낸 것이며, 실제 `Vec`의 용량과 재할당 여부는 그림에 표시하지 않았습니다.

## 복잡도와 주의점

- `push`와 `pop`은 상각 O(1)이다. `Vec`의 용량이 부족하면 `push`가 재할당하면서 해당 순간 O(n)이 될 수 있다.
- `last`, `len`, `is_empty`는 O(1)이다.
- 중간 인덱스의 삽입과 삭제는 뒤 원소를 이동하므로 O(n)이다.
- 스택의 top은 `Vec`의 끝이다. 앞에서 `insert`와 `remove`를 반복하면 스택의 장점을 잃는다.
- 원소 수를 미리 알면 `with_capacity`로 재할당을 줄일 수 있다.
- `pop`은 값이 없을 때 패닉하지 않고 `Option<T>`의 `None`을 반환한다.

## 완전한 예제

```rust
fn main() {
    let mut stack = Vec::with_capacity(3);
    stack.push("first");
    stack.push("second");
    stack.push("third");

    println!("top = {:?}", stack.last());
    while let Some(value) = stack.pop() {
        println!("popped = {value}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_behaves_as_lifo_stack() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
```

## PowerShell에서 테스트

문서를 임시 Rust 파일로 저장한 뒤 다음 명령을 실행한다.

- `rustc --test .\stack.rs -o .\stack-tests.exe`
- `.\stack-tests.exe`

실행 예제까지 확인하려면 `rustc .\stack.rs -o .\stack.exe; .\stack.exe`를 사용한다. 문서의 코드 블록 내용만 `stack.rs`로 저장해야 한다.

## 참고 자료

- [Rust 표준 라이브러리 Vec 문서](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Rust Vec 사용 설명서](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rustonomicon Vec 구현과 복잡도 참고](https://doc.rust-lang.org/nomicon/vec/vec.html)
