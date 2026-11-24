# Rust 자료구조 노트

Rust의 자료구조와 알고리즘을 한국어로 학습하는 Axum 웹사이트입니다. 각 문서에는 개념, 복잡도, 실행 가능한 예제와 테스트, 참고자료가 들어 있습니다.

## 실행하기

Rust와 Cargo가 필요합니다. 프로젝트 폴더에서 다음을 실행하세요.

```powershell
cargo run
```

브라우저에서 <http://127.0.0.1:3000>을 엽니다. 기본 서버는 로컬 컴퓨터에서만 접근할 수 있습니다. 다른 포트를 사용하려면 PowerShell에서 `$env:PORT = "3001"`을 설정한 뒤 실행하세요. 종료하려면 `Ctrl+C`를 누릅니다.

## 테스트

```powershell
cargo test
```

서버와 렌더러의 단위 테스트를 실행합니다. 각 Markdown 파일의 모든 Rust 코드 블록을 임시 파일로 추출하여 일반 프로그램과 테스트 프로그램으로 각각 컴파일하고 실행합니다. 따라서 `rustc`와 플랫폼의 Rust 링크 도구가 필요합니다.

테마와 문서 사이드바 선택 및 브라우저 저장 동작의 JavaScript 테스트는 Node.js가 설치되어 있다면 별도로 실행할 수 있습니다.

```powershell
node --test tests/theme.test.cjs tests/sidebar.test.cjs
```

상단의 테마 선택 메뉴에서 밝은, 어두운, 따뜻한 테마를 고를 수 있습니다. 문서 페이지의 `메뉴 접기` 버튼으로 사이드바를 접거나 펼칠 수 있습니다. 두 선택은 브라우저 저장소에 보관되며 저장소를 사용할 수 없는 환경에서는 현재 페이지만 적용됩니다. JavaScript가 꺼져 있으면 사이드바는 항상 표시됩니다.

## 문서 편집

원본은 `docs/`에 주제별 Markdown으로 저장되어 있습니다. 문서는 컴파일 시 실행 파일에 포함되므로 수정 후 서버를 다시 빌드하고 실행해야 반영됩니다. 새 문서를 추가할 때는 `src/main.rs`의 `DOCUMENTS` 목록에도 등록하세요.

이 사이트는 자체 간단한 Markdown 렌더러를 사용합니다. 제목(`#{1..4}`), 문단, `-` 목록, 코드 펜스, 인라인 코드, 굵은 글씨(`**...**`), 표와 HTTP(S) 링크를 지원합니다. 표는 헤더와 세 개 이상의 하이픈으로 된 구분 행이 필요하고, 셀 안의 `|`는 `\|`로 이스케이프할 수 있습니다. 이미지 문법은 `src/diagrams.rs`에 등록된 번들 다이어그램만 표시하며 외부 이미지는 지원하지 않습니다. 중첩 목록 등 전체 Markdown 문법은 지원하지 않으며 HTML은 이스케이프됩니다. 외부 폰트를 불러올 수 없을 때는 시스템 글꼴을 사용합니다.

----

1. GitHub 저장소 만들기
GitHub 새 저장소 만들기에서 저장소 이름을 입력하고 공개 여부를 선택하세요. README, .gitignore, 라이선스는 추가하지 않고 빈 저장소로 만듭니다. 프로젝트에 이미 해당 파일들이 있습니다.

2. 프로젝트에서 Git 초기화

```sh
git init -b main
git add .
git status
git diff --cached
```

3. 커밋하고 GitHub에 올리기
아래 주소에서 사용자이름과 저장소이름을 실제 값으로 바꾸세요.

`
git commit -m "Initial commit"
git remote add origin https://github.com/사용자이름/저장소이름.git
git push -u origin main
```

 이후 변경 사항을 올릴 때는 다음을 실행하면 됩니다.

```sh
git add .
git status
git diff --cached
git commit -m "변경 내용"
git push
```
