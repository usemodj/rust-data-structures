# KMP 문자열 검색

## 동작 다이어그램

![패턴 ABABA의 lps 표와 텍스트 ABABABA에서 위치 0과 2의 겹치는 일치](/assets/kmp.svg)

![텍스트 ABABABA와 패턴 ABABA를 비교해 lps 표 0, 0, 1, 2, 3을 만들고 시작 위치 0과 2에서 겹치는 일치를 찾는 단계별 과정](/assets/kmp-steps.svg)

단계별 그림은 텍스트 `ABABABA`, 패턴 `ABABA`를 사용하며 아래 Rust 테스트에서도 같은 입력을 검증합니다. `main`의 소문자 `abababa`와 패턴 `aba`는 이 그림과 다른 예제입니다. 패턴 `ABABA`의 접두사-접미사 표(lps)를 미리 계산하면 텍스트 `ABABABA`에서 일치가 끝난 뒤에도 패턴을 처음부터 다시 비교하지 않고 위치 0과 2의 겹치는 일치를 찾을 수 있습니다. 표의 값은 패턴의 각 위치까지에서 재사용할 접두사의 길이입니다.

### 그림 단계별 읽기

- **lps 준비:** 패턴 인덱스 0, 1, 2, 3, 4의 문자는 각각 `A`, `B`, `A`, `B`, `A`이고, `lps`는 `[0, 0, 1, 2, 3]`입니다. `i=1`에서 `B`와 `A`가 달라 `lps[1]=0`이 되고, 이후 `A=A`, `B=B`, `A=A`가 이어져 `lps[2]=1`, `lps[3]=2`, `lps[4]=3`이 됩니다.
- **첫 비교:** 텍스트 바이트 오프셋 `t=0..4`에서 패턴 인덱스 `p=0..4`가 차례로 모두 일치합니다. `t=4, p=4`의 `A=A` 뒤 완전 일치 구간은 반열린 구간 `[0..5)`이고, 시작 오프셋 `0`을 기록합니다.
- **이동:** 완전 일치 직후 `p`를 0으로 되돌리지 않고 `lps[4]=3`으로 설정합니다. 이미 확인한 접두사 `ABA`를 재사용하므로 텍스트 `t=5`의 `B`와 패턴 `p=3`의 `B`만 비교하면 되고, 다음 후보 시작은 `5-3=2`입니다. 텍스트 인덱스 `t`는 뒤로 가지 않습니다.
- **겹치는 두 번째 일치:** `t=5, p=3`과 `t=6, p=4`가 각각 `B=B`, `A=A`로 일치해 `[2..7)` 및 시작 오프셋 `2`를 기록합니다. 두 일치 `[0..5)`와 `[2..7)`는 바이트 오프셋 2부터 4까지를 공유하고, 결과는 `[0, 2]`입니다.
- **상태 읽기:** 그림의 `t`는 텍스트 바이트 인덱스, `p`는 패턴 바이트 인덱스이며 `[start..end)`는 끝을 포함하지 않는 구간입니다. 이 예에서는 검색 끝 상태가 `t=7`이므로 텍스트를 모두 소비한 것입니다.

그림의 위치는 ASCII 예시에서는 바이트 오프셋과 같지만, Rust `&str`의 실제 결과는 UTF-8 바이트 오프셋입니다. 한글 같은 멀티바이트 문자는 문자 수와 위치가 다를 수 있으므로 문자 경계에서만 변환해야 합니다.

KMP(Knuth-Morris-Pratt) 알고리즘은 패턴이 어긋났을 때 텍스트 인덱스를 뒤로 돌리지 않고, 패턴 내부의 접두사와 접미사 정보로 다음 비교 위치를 정하는 문자열 검색 알고리즘입니다. 먼저 `lps`(longest proper prefix which is also suffix) 배열을 만들고, 검색 중 불일치가 발생하면 이 배열을 이용해 패턴만 이동합니다.

## LPS의 의미와 용도

LPS는 *Longest Proper Prefix which is also a Suffix*의 약자로, 접미사와 일치하는 가장 긴 **진접두사**의 길이입니다. 진접두사는 문자열 전체를 제외한 접두사입니다. `lps[i]`는 패턴의 처음부터 `i`번째 바이트까지에 대해 이 길이를 저장합니다.

예를 들어 `aba`의 진접두사는 빈 문자열, `a`, `ab`이고 접미사는 빈 문자열, `a`, `ba`, `aba`입니다. 양쪽에 공통으로 나타나는 가장 긴 진접두사는 `a`이므로 LPS는 `1`입니다. 각 위치까지 계산한 `aba`의 LPS 배열은 `[0, 0, 1]`입니다.

KMP는 검색 중 불일치가 나거나 패턴 전체가 일치한 뒤에도 LPS를 이용해 이미 확인한 부분을 재활용하고 다음 비교 위치를 정합니다. 예를 들어 `aba`가 일치한 뒤 `lps[2] = 1`을 사용하면 마지막 `a`를 다음 후보의 첫 `a`로 재사용할 수 있습니다. 텍스트를 뒤로 되돌리지 않으면서 `abababa` 안의 겹치는 일치 위치 `0`, `2`, `4`를 찾을 수 있습니다.

이 예제의 입력은 Rust의 `&str`입니다. Rust 문자열은 UTF-8이므로 알고리즘은 안전하게 `as_bytes()`로 바이트를 비교하지만, 반환하는 위치는 문자의 개수가 아니라 텍스트 안의 바이트 오프셋입니다. 예를 들어 `"가나가"`에서 `"가"`의 위치는 `[0, 6]`이며, `"가"`가 차지하는 길이는 3바이트입니다. 바이트 오프셋을 문자 오프셋으로 바꾸려면 `text[..offset].chars().count()`처럼 경계에서만 잘라야 합니다. 구현은 반환 후보를 `is_char_boundary`로 확인하므로 UTF-8 문자의 중간을 시작 위치로 보고하지 않습니다.

패턴이 비어 있으면 관례적으로 빈 문자열은 모든 유효한 UTF-8 경계에서 일치한다고 정의합니다. 따라서 `"가나"`와 빈 패턴의 결과는 `[0, 3, 6]`입니다. 일치한 뒤 접두사 정보를 이용해 검색을 이어가므로 겹치는 일치도 놓치지 않습니다. 예를 들어 `"aaaa"`에서 `"aa"`는 `[0, 1, 2]`입니다. 패턴과 텍스트가 모두 UTF-8 유효 문자열인 한 바이트 인덱싱은 경계를 침범하지 않으며, 반환된 오프셋으로 슬라이스할 때도 안전합니다.

## KMP의 활용

KMP는 긴 텍스트에서 **하나의 고정된 패턴이 나타나는 위치**를 찾을 때 사용할 수 있습니다.

- **문서·로그 검색:** 긴 문서나 로그에서 특정 단어 또는 오류 메시지의 위치를 찾습니다.
- **생물정보학:** DNA 염기서열에서 특정 염기 패턴이 나타나는 위치를 찾습니다.
- **데이터 처리:** 텍스트 안의 고정된 구분자나 반복 패턴을 찾아 분석합니다.
- **겹치는 패턴 검색:** 예를 들어 `aaaa`에서 `aa`가 시작하는 위치 `0`, `1`, `2`를 모두 찾습니다.

LPS를 미리 계산하면 반복되는 부분을 다시 비교하는 일을 줄이며, 패턴 길이 `M`과 텍스트 길이 `N`에 대해 최악의 경우에도 `O(N + M)`에 검색합니다. 정규식이나 여러 패턴을 동시에 검색하는 기능이 필요하다면 다른 도구나 알고리즘을 고려해야 합니다. 아래 구현은 UTF-8 바이트를 정확히 비교하므로 대소문자 무시와 유니코드 정규화는 별도 처리가 필요합니다.

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

### 함수 설명

- **`build_lps(pattern: &[u8]) -> Vec<usize>`:** 패턴의 각 위치까지에서 접미사와 일치하는 가장 긴 진접두사의 길이를 계산합니다. 진접두사는 해당 구간 전체가 아닌 접두사입니다. 예를 들어 `ABABA`의 결과는 `[0, 0, 1, 2, 3]`입니다. 불일치 시 이 표를 사용해 패턴의 다음 비교 위치를 정합니다.
- **`find_all(text: &str, pattern: &str) -> Vec<usize>`:** `build_lps`로 만든 표를 이용해 텍스트에서 패턴이 시작되는 위치를 모두 찾습니다. 겹치는 일치도 포함하며, 결과는 문자 번호가 아닌 UTF-8 바이트 오프셋입니다. 빈 패턴은 텍스트의 모든 UTF-8 문자 경계와 끝에서 일치합니다.
- **`main()`:** `find_all("abababa", "aba")`를 호출해 `matches: [0, 2, 4]`를 출력합니다.

### 예시로 보는 상태 변화 추적

아래 `main()`의 `text = "abababa"`, `pattern = "aba"`를 사용합니다. 두 문자열은 ASCII이므로 이 예제에서는 각 문자가 1바이트이고, 표의 인덱스는 바이트 오프셋입니다.

#### 1. `build_lps`로 패턴 표 만들기

초기값은 `lps = [0, 0, 0]`, `length = 0`, `index = 1`입니다. `length`는 현재까지 일치한 접두사의 길이입니다.

| 비교 전 `index` | 비교 | 처리 후 `length` | 처리 후 `index` | 처리 후 `lps` |
| --- | --- | --- | --- | --- |
| `1` | `pattern[1] = 'b'`, `pattern[0] = 'a'`: 불일치 | `0` | `2` | `[0, 0, 0]` |
| `2` | `pattern[2] = 'a'`, `pattern[0] = 'a'`: 일치 | `1` | `3` | `[0, 0, 1]` |

`index = 3`이 패턴 길이와 같아 반복이 끝나고, `build_lps`는 `[0, 0, 1]`을 반환합니다.

#### 2. `find_all`로 검색하기

초기값은 `text_index = 0`, `pattern_index = 0`, `matches = []`입니다. 아래 표의 처리 후 `pattern_index`에는 완전 일치 뒤 `lps`를 적용한 값까지 반영했습니다.

| 비교 전 `text_index` | 비교 전 `pattern_index` | 바이트 비교 | 처리 후 `text_index` | 처리 후 `pattern_index` | 처리 후 `matches` |
| --- | --- | --- | --- | --- | --- |
| `0` | `0` | `a = a` | `1` | `1` | `[]` |
| `1` | `1` | `b = b` | `2` | `2` | `[]` |
| `2` | `2` | `a = a`, 완전 일치 | `3` | `1` | `[0]` |
| `3` | `1` | `b = b` | `4` | `2` | `[0]` |
| `4` | `2` | `a = a`, 완전 일치 | `5` | `1` | `[0, 2]` |
| `5` | `1` | `b = b` | `6` | `2` | `[0, 2]` |
| `6` | `2` | `a = a`, 완전 일치 | `7` | `1` | `[0, 2, 4]` |

완전 일치할 때마다 `start = text_index - pattern_bytes.len()`으로 각각 `0`, `2`, `4`를 기록합니다. 이후 `pattern_index = lps[2] = 1`로 되돌려 이미 일치한 접두사 `a`를 재사용하므로 겹치는 일치도 찾습니다. 이 입력에서는 불일치가 없지만, 검색 중 불일치가 발생하면 `pattern_index`가 0보다 클 때 `lps[pattern_index - 1]`로 되돌리고 같은 텍스트 바이트를 다시 비교합니다.

`text_index = 7`이 되어 검색이 끝나면 `[0, 2, 4]`를 반환하고, `main`은 `matches: [0, 2, 4]`를 출력합니다.

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
    fn builds_the_lps_steps_shown_above() {
        assert_eq!(build_lps(b"ABABA"), vec![0, 0, 1, 2, 3]);
    }

    #[test]
    fn traces_the_diagram_comparisons_and_overlaps() {
        let text = b"ABABABA";
        let pattern = b"ABABA";
        let lps = build_lps(pattern);
        assert_eq!(lps, [0, 0, 1, 2, 3]);
        let (mut t, mut p) = (0, 0);
        let mut trace = Vec::new();
        while t < text.len() {
            let (before_t, before_p) = (t, p);
            let mut found = None;
            if text[t] == pattern[p] {
                t += 1;
                p += 1;
                if p == pattern.len() {
                    found = Some(t - pattern.len());
                    p = lps[p - 1];
                }
            } else if p > 0 {
                p = lps[p - 1];
            } else {
                t += 1;
            }
            trace.push((before_t, before_p, t, p, found));
        }
        assert_eq!(trace, vec![
            (0, 0, 1, 1, None),
            (1, 1, 2, 2, None),
            (2, 2, 3, 3, None),
            (3, 3, 4, 4, None),
            (4, 4, 5, 3, Some(0)),
            (5, 3, 6, 4, None),
            (6, 4, 7, 3, Some(2)),
        ]);
        assert_eq!(find_all("ABABABA", "ABABA"), vec![0, 2]);
    }

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
