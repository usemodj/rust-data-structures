# KMP 문자열 검색

## 동작 다이어그램

![접두사 표를 사용해 겹치는 일치 위치를 찾는 KMP 과정](/assets/kmp.svg)

패턴 `ABABA`의 접두사-접미사 표(lps)를 미리 계산하면 텍스트 `ABABABA`에서 일치가 끝난 뒤에도 패턴을 처음부터 다시 비교하지 않고 위치 0과 2의 겹치는 일치를 찾을 수 있습니다. 표의 값은 패턴의 각 위치까지에서 재사용할 접두사의 길이입니다.

### 그림 단계별 읽기

- 패턴 `ABABA`의 인덱스 0, 1, 2, 3, 4에 대응하는 lps 값은 각각 0, 0, 1, 2, 3입니다.
- 텍스트 `ABABABA`의 바이트 오프셋 0에서 패턴이 먼저 일치하고, 일치 뒤 `lps[4] = 3`을 재사용해 처음 세 글자 `ABA`를 겹칩니다.
- 겹침을 이어 가면 두 번째 일치의 시작은 바이트 오프셋 2이며, 두 일치는 텍스트의 바이트 오프셋 2부터 4까지를 공유합니다.
- 그림의 ASCII 문자는 한 글자당 1바이트지만 UTF-8에서는 문자 수와 바이트 위치가 다를 수 있으므로, 예를 들어 `가나가`에서 `가`의 시작 바이트 오프셋은 0과 6입니다.

그림의 위치는 ASCII 예시에서는 바이트 오프셋과 같지만, Rust `&str`의 실제 결과는 UTF-8 바이트 오프셋입니다. 한글 같은 멀티바이트 문자는 문자 수와 위치가 다를 수 있으므로 문자 경계에서만 변환해야 합니다.

KMP(Knuth-Morris-Pratt) 알고리즘은 패턴이 어긋났을 때 텍스트 인덱스를 뒤로 돌리지 않고, 패턴 내부의 접두사와 접미사 정보로 다음 비교 위치를 정하는 문자열 검색 알고리즘입니다. 먼저 `lps`(longest proper prefix which is also suffix) 배열을 만들고, 검색 중 불일치가 발생하면 이 배열을 이용해 패턴만 이동합니다.

이 예제의 입력은 Rust의 `&str`입니다. Rust 문자열은 UTF-8이므로 알고리즘은 안전하게 `as_bytes()`로 바이트를 비교하지만, 반환하는 위치는 문자의 개수가 아니라 텍스트 안의 바이트 오프셋입니다. 예를 들어 `"가나가"`에서 `"가"`의 위치는 `[0, 6]`이며, `"가"`가 차지하는 길이는 3바이트입니다. 바이트 오프셋을 문자 오프셋으로 바꾸려면 `text[..offset].chars().count()`처럼 경계에서만 잘라야 합니다. 구현은 반환 후보를 `is_char_boundary`로 확인하므로 UTF-8 문자의 중간을 시작 위치로 보고하지 않습니다.

패턴이 비어 있으면 관례적으로 빈 문자열은 모든 유효한 UTF-8 경계에서 일치한다고 정의합니다. 따라서 `"가나"`와 빈 패턴의 결과는 `[0, 3, 6]`입니다. 일치한 뒤 접두사 정보를 이용해 검색을 이어가므로 겹치는 일치도 놓치지 않습니다. 예를 들어 `"aaaa"`에서 `"aa"`는 `[0, 1, 2]`입니다. 패턴과 텍스트가 모두 UTF-8 유효 문자열인 한 바이트 인덱싱은 경계를 침범하지 않으며, 반환된 오프셋으로 슬라이스할 때도 안전합니다.

## 복잡도와 한계

텍스트의 바이트 길이를 `N`, 패턴의 바이트 길이를 `M`이라고 할 때 `lps` 구성은 `O(M)`, 검색은 `O(N)`이므로 전체 시간 복잡도는 `O(N + M)`입니다. `lps`에 `O(M)` 공간이 필요하며 결과 `K`개를 저장하는 벡터까지 포함하면 추가 공간은 `O(M + K)`입니다. 이 구현의 위치 단위는 UTF-8 문자 수가 아닌 바이트 수이므로, 문자 단위 인덱스가 필요한 API에서는 별도의 변환이 필요합니다. 정규식, 대소문자 무시, 유니코드 정규화는 제공하지 않습니다.

참고 자료: [DSAR Chapter 34 - String Matching](https://dsar.rantai.dev/docs/part-vi/chapter-34/)

## 실행 방법

다음처럼 PowerShell에서 코드 블록을 `kmp.rs`로 저장한 뒤 컴파일할 수 있습니다.

```powershell
rustc --test kmp.rs -o kmp_tests.exe
.\kmp_tests.exe
```

## 실행 가능한 예제

```rust
fn build_lps(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut length = 0;
    let mut index = 1;
    while index < pattern.len() {
        if pattern[index] == pattern[length] {
            length += 1;
            lps[index] = length;
            index += 1;
        } else if length > 0 {
            length = lps[length - 1];
        } else {
            index += 1;
        }
    }
    lps
}

fn find_all(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        let mut positions = text.char_indices().map(|(index, _)| index).collect::<Vec<_>>();
        positions.push(text.len());
        return positions;
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let lps = build_lps(pattern_bytes);
    let mut matches = Vec::new();
    let (mut text_index, mut pattern_index) = (0, 0);

    while text_index < text_bytes.len() {
        if text_bytes[text_index] == pattern_bytes[pattern_index] {
            text_index += 1;
            pattern_index += 1;
            if pattern_index == pattern_bytes.len() {
                let start = text_index - pattern_bytes.len();
                if text.is_char_boundary(start) {
                    matches.push(start);
                }
                pattern_index = lps[pattern_index - 1];
            }
        } else if pattern_index > 0 {
            pattern_index = lps[pattern_index - 1];
        } else {
            text_index += 1;
        }
    }
    matches
}

fn main() {
    let text = "abababa";
    println!("matches: {:?}", find_all(text, "aba"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_overlapping_matches() {
        assert_eq!(find_all("aaaa", "aa"), vec![0, 1, 2]);
        assert_eq!(find_all("abababa", "aba"), vec![0, 2, 4]);
    }

    #[test]
    fn reports_utf8_byte_offsets() {
        assert_eq!(find_all("가나가", "가"), vec![0, 6]);
        assert_eq!(find_all("rust", "xyz"), Vec::<usize>::new());
    }

    #[test]
    fn empty_pattern_matches_every_utf8_boundary() {
        assert_eq!(find_all("가나", ""), vec![0, 3, 6]);
        assert_eq!(find_all("", ""), vec![0]);
    }
}
```
