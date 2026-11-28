mod diagrams;
mod markdown;

use axum::{
    Router,
    extract::{Path, RawQuery},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing::get,
};

struct Document {
    slug: &'static str,
    title: &'static str,
    category: &'static str,
    description: &'static str,
    markdown: &'static str,
}

static DOCUMENTS: &[Document] = &[
    Document {
        slug: "vec",
        title: "동적 배열 · Vec",
        category: "자료구조",
        description: "연속된 메모리에 데이터를 담는 가장 기본적인 컬렉션",
        markdown: include_str!("../docs/vec.md"),
    },
    Document {
        slug: "vecdeque",
        title: "큐와 덱 · VecDeque",
        category: "자료구조",
        description: "양쪽 끝에서 빠르게 추가하고 제거하는 방법",
        markdown: include_str!("../docs/vecdeque.md"),
    },
    Document {
        slug: "hashmap",
        title: "해시 맵 · HashMap",
        category: "자료구조",
        description: "키를 사용해 값을 효율적으로 저장하고 찾기",
        markdown: include_str!("../docs/hashmap.md"),
    },
    Document {
        slug: "stack",
        title: "스택 · Vec",
        category: "자료구조",
        description: "마지막에 넣은 데이터를 먼저 꺼내는 스택",
        markdown: include_str!("../docs/stack.md"),
    },
    Document {
        slug: "hashset",
        title: "해시 집합 · HashSet",
        category: "자료구조",
        description: "중복 없이 값을 저장하고 빠르게 포함 여부 확인하기",
        markdown: include_str!("../docs/hashset.md"),
    },
    Document {
        slug: "binary-heap",
        title: "우선순위 큐 · BinaryHeap",
        category: "자료구조",
        description: "우선순위가 가장 높은 항목부터 꺼내기",
        markdown: include_str!("../docs/binary-heap.md"),
    },
    Document {
        slug: "btree-map",
        title: "정렬 맵 · BTreeMap",
        category: "자료구조",
        description: "키를 정렬된 순서로 저장하고 범위 검색하기",
        markdown: include_str!("../docs/btree-map.md"),
    },
    Document {
        slug: "linked-list",
        title: "연결 리스트 · LinkedList",
        category: "자료구조",
        description: "노드로 연결된 양방향 리스트와 컬렉션 선택 기준",
        markdown: include_str!("../docs/linked-list.md"),
    },
    Document {
        slug: "btree-set",
        title: "정렬 집합 · BTreeSet",
        category: "자료구조",
        description: "중복 없이 값을 정렬해 저장하고 범위 검색하기",
        markdown: include_str!("../docs/btree-set.md"),
    },
    Document {
        slug: "union-find",
        title: "서로소 집합 · Union-Find",
        category: "자료구조",
        description: "서로 연결된 원소를 효율적으로 관리하기",
        markdown: include_str!("../docs/union-find.md"),
    },
    Document {
        slug: "binary-search-tree",
        title: "이진 탐색 트리",
        category: "자료구조",
        description: "노드의 크기 관계를 이용해 탐색하고 순회하기",
        markdown: include_str!("../docs/binary-search-tree.md"),
    },
    Document {
        slug: "graph-representations",
        title: "그래프 표현",
        category: "자료구조",
        description: "인접 리스트, 인접 행렬, 간선 목록의 차이 이해하기",
        markdown: include_str!("../docs/graph-representations.md"),
    },
    Document {
        slug: "binary-search",
        title: "이진 탐색",
        category: "알고리즘",
        description: "정렬된 데이터에서 탐색 범위를 절반씩 줄이기",
        markdown: include_str!("../docs/binary-search.md"),
    },
    Document {
        slug: "sorting",
        title: "정렬",
        category: "알고리즘",
        description: "데이터를 순서대로 정리하는 기본 원리",
        markdown: include_str!("../docs/sorting.md"),
    },
    Document {
        slug: "bfs",
        title: "너비 우선 탐색 · BFS",
        category: "알고리즘",
        description: "큐를 이용해 그래프를 가까운 곳부터 탐색하기",
        markdown: include_str!("../docs/bfs.md"),
    },
    Document {
        slug: "dfs",
        title: "깊이 우선 탐색 · DFS",
        category: "알고리즘",
        description: "그래프의 경로를 깊이 탐색하고 방문 순서 알아보기",
        markdown: include_str!("../docs/dfs.md"),
    },
    Document {
        slug: "dijkstra",
        title: "다익스트라 최단 경로",
        category: "알고리즘",
        description: "음이 아닌 가중치 그래프에서 최단 거리 찾기",
        markdown: include_str!("../docs/dijkstra.md"),
    },
    Document {
        slug: "merge-sort",
        title: "병합 정렬",
        category: "알고리즘",
        description: "분할과 병합으로 데이터를 정렬하기",
        markdown: include_str!("../docs/merge-sort.md"),
    },
    Document {
        slug: "two-pointers",
        title: "투 포인터",
        category: "알고리즘",
        description: "두 위치를 움직여 정렬된 데이터의 쌍 찾기",
        markdown: include_str!("../docs/two-pointers.md"),
    },
    Document {
        slug: "topological-sort",
        title: "위상 정렬",
        category: "알고리즘",
        description: "의존 관계에 맞는 실행 순서를 찾고 순환 감지하기",
        markdown: include_str!("../docs/topological-sort.md"),
    },
    Document {
        slug: "insertion-sort",
        title: "삽입 정렬",
        category: "알고리즘",
        description: "원소를 정렬된 부분의 올바른 위치에 삽입하기",
        markdown: include_str!("../docs/insertion-sort.md"),
    },
    Document {
        slug: "prefix-sum",
        title: "누적 합",
        category: "알고리즘",
        description: "구간 합을 빠르게 계산하는 전처리 기법",
        markdown: include_str!("../docs/prefix-sum.md"),
    },
    Document {
        slug: "sliding-window",
        title: "슬라이딩 윈도",
        category: "알고리즘",
        description: "고정 길이 구간을 이동하며 효율적으로 계산하기",
        markdown: include_str!("../docs/sliding-window.md"),
    },
    Document {
        slug: "kruskal",
        title: "최소 신장 트리 · 크루스칼",
        category: "알고리즘",
        description: "유니온 파인드로 최소 비용의 연결 간선 선택하기",
        markdown: include_str!("../docs/kruskal.md"),
    },
    Document {
        slug: "kmp",
        title: "문자열 검색 · KMP",
        category: "알고리즘",
        description: "접두사 정보를 활용해 문자열 패턴을 효율적으로 찾기",
        markdown: include_str!("../docs/kmp.md"),
    },
    Document {
        slug: "bellman-ford",
        title: "벨만-포드 최단 경로",
        category: "알고리즘",
        description: "음수 간선을 포함한 그래프의 최단 거리와 음수 사이클 확인하기",
        markdown: include_str!("../docs/bellman-ford.md"),
    },
    Document {
        slug: "floyd-warshall",
        title: "플로이드-워셜",
        category: "알고리즘",
        description: "모든 정점 쌍 사이의 최단 거리 구하기",
        markdown: include_str!("../docs/floyd-warshall.md"),
    },
    Document {
        slug: "knapsack",
        title: "동적 계획법 · 0/1 배낭",
        category: "알고리즘",
        description: "제한된 용량에서 최대 가치를 선택하기",
        markdown: include_str!("../docs/knapsack.md"),
    },
    Document {
        slug: "quicksort",
        title: "퀵 정렬",
        category: "알고리즘",
        description: "기준 원소를 중심으로 분할하여 정렬하기",
        markdown: include_str!("../docs/quicksort.md"),
    },
];

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/docs/{slug}", get(document))
        .route("/docs/dijkstra-visual", get(dijkstra_visual))
        .route("/search", get(search_documents))
        .route("/style.css", get(stylesheet))
        .route("/theme.js", get(theme_script))
        .route("/sidebar.js", get(sidebar_script))
        .route("/assets/dijkstra-algorithm-steps.png", get(dijkstra_algorithm_image))
        .route("/assets/dijkstra-shortest-path-diagram.png", get(dijkstra_shortest_path_diagram))
        .route("/assets/{filename}", get(diagram))
        .fallback(not_found)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. .env 파일을 읽어 프로세스 환경 변수로 로드
    // (로컬에 .env 파일이 없거나 선택적일 때는 .ok()로 무시할 수도 있음)
    let _ = dotenvy::dotenv();    
    let host = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "6000".into()).parse::<u16>()?;
    let address = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("Rust 자료구조 노트: http://{}", listener.local_addr()?);
    axum::serve(listener, app()).await?;
    Ok(())
}

async fn home() -> Html<String> {
    let mut cards = String::new();
    for (index, doc) in DOCUMENTS.iter().enumerate() {
        cards.push_str(&format!(
            "<a class=\"card\" href=\"/docs/{}\" data-search=\"{} {} {}\"><span class=\"card-top\"><span class=\"card-number\">{:02}</span><span class=\"card-arrow\" aria-hidden=\"true\">↗</span></span><span class=\"card-category\">{}</span><h3>{}</h3><p>{}</p><span class=\"card-link\">문서 읽기 <span aria-hidden=\"true\">→</span></span></a>",
            doc.slug, doc.title, doc.category, doc.description, index + 1, doc.category, doc.title, doc.description
        ));
    }
    let count = format!("{:02}", DOCUMENTS.len());
    let content = format!(
        r##"
        <section class="hero"><div class="hero-copy"><span class="eyebrow"><span class="dot"></span> RUST STUDY GUIDE</span>
        <h1>자료구조와 알고리즘,<br><em>Rust답게.</em></h1>
        <p>개념부터 코드까지, 한 걸음씩. Rust로 직접 만들고 테스트하며 자료구조와 알고리즘을 익혀보세요.</p>
        <div class="hero-actions"><a class="button primary" href="/docs/vec">학습 시작하기 <span aria-hidden="true">↗</span></a><a class="button secondary" href="#topics">전체 주제 보기 <span aria-hidden="true">↓</span></a></div></div>
        <div class="hero-art" aria-hidden="true"><div class="art-label">THE BUILDING BLOCKS <span>01 — {count}</span></div><div class="art-code"><div><i></i><i></i><i></i></div><p><span>let</span> curiosity = <b>vec!</b>[</p><p class="indent">"자료구조",</p><p class="indent">"알고리즘",</p><p class="indent">"Rust 🦀",</p><p>];</p></div><div class="art-footer"><span>LEARN BY BUILDING</span><strong>RUST / 101</strong></div></div></section>
        <section class="topics" id="topics"><div class="section-heading"><div><span class="eyebrow muted">EXPLORE THE NOTES</span><h2>무엇을 배워볼까요?</h2><p>필요한 주제부터 편하게 시작하세요.</p></div><span class="count">{count}개의 학습 노트</span></div>
        <label class="search"><span aria-hidden="true">⌕</span><input id="search" type="search" placeholder="주제 검색하기 (예: Vec, 탐색)" aria-label="주제 검색"></label>
        <div class="cards">{cards}</div><p id="no-results" class="no-results" hidden>검색 결과가 없습니다. 다른 검색어를 입력해 보세요.</p></section>
        <section class="note"><span class="note-icon">✳</span><div><strong>직접 실행하며 배워보세요.</strong><p>모든 노트에는 설명과 함께 실행 가능한 예제, 테스트, 참고자료가 담겨 있습니다.</p></div></section>
        <script>const s=document.querySelector('#search');const cards=[...document.querySelectorAll('.card')];s.addEventListener('input',()=>{{const q=s.value.trim().toLowerCase();let n=0;cards.forEach(c=>{{const match=c.dataset.search.toLowerCase().includes(q);c.hidden=!match;if(match)n++}});document.querySelector('#no-results').hidden=n!==0}});</script>
    "##
    );
    Html(layout("홈", &content, None))
}

// Decode the query parameter from an HTML GET form without accepting invalid UTF-8.
fn search_query(raw: Option<&str>) -> Result<String, &'static str> {
    let encoded = raw
        .unwrap_or("")
        .split('&')
        .find_map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            (key == "q").then_some(value)
        })
        .unwrap_or("");
    let bytes = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    fn hex(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' => {
                let high = bytes.get(index + 1).and_then(|b| hex(*b));
                let low = bytes.get(index + 2).and_then(|b| hex(*b));
                match (high, low) {
                    (Some(high), Some(low)) => decoded.push(high * 16 + low),
                    _ => return Err("잘못된 검색어 인코딩입니다."),
                }
                index += 3;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    let query = String::from_utf8(decoded).map_err(|_| "잘못된 검색어 인코딩입니다.")?;
    let query = query.trim();
    if query.chars().count() > 200 {
        return Err("검색어는 200자 이내로 입력해 주세요.");
    }
    Ok(query.to_owned())
}

fn matches_document(doc: &Document, query: &str) -> bool {
    let text = format!(
        "{} {} {} {}",
        doc.title, doc.category, doc.description, doc.markdown
    )
    .to_lowercase();
    query
        .split_whitespace()
        .all(|word| text.contains(&word.to_lowercase()))
}

// Select a readable body line, preferring the one containing the most query words.
// Keep an index map so Unicode lowercase expansions cannot break snippet boundaries.
fn body_preview(markdown: &str, query: &str) -> Option<String> {
    const MAX_CHARS: usize = 160;
    const LEADING_CONTEXT: usize = 35;
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    let mut best: Option<(usize, usize, Vec<char>)> = None;

    for line in markdown.lines() {
        let line = line.trim();
        if line.is_empty()
            || line.starts_with('#')
            || line.starts_with("```")
            || line.starts_with("![")
        {
            continue;
        }
        let line = line.strip_prefix("- ").unwrap_or(line).replace('`', "");
        let chars: Vec<char> = line.chars().collect();
        if chars.is_empty() {
            continue;
        }
        let mut normalized = String::new();
        let mut offsets = Vec::new();
        for (index, ch) in chars.iter().enumerate() {
            for lower in ch.to_lowercase() {
                normalized.push(lower);
                offsets.push(index);
            }
        }
        let mut score = 0;
        let mut focus = None;
        for word in &words {
            if let Some(byte_index) = normalized.find(word) {
                score += 1;
                let lower_index = normalized[..byte_index].chars().count();
                let original_index = offsets.get(lower_index).copied().unwrap_or(0);
                focus = Some(focus.map_or(original_index, |old: usize| old.min(original_index)));
            }
        }
        if best
            .as_ref()
            .is_none_or(|(best_score, _, _)| score > *best_score)
        {
            best = Some((score, focus.unwrap_or(0), chars));
        }
    }

    let (_, focus, chars) = best?;
    let start = focus.saturating_sub(LEADING_CONTEXT);
    let end = (start + MAX_CHARS).min(chars.len());
    let mut preview: String = chars[start..end].iter().collect();
    if start > 0 {
        preview.insert(0, '…');
    }
    if end < chars.len() {
        preview.push('…');
    }
    Some(preview)
}

async fn search_documents(RawQuery(raw): RawQuery) -> (StatusCode, Html<String>) {
    let query = match search_query(raw.as_deref()) {
        Ok(query) => query,
        Err(message) => {
            let content = format!(
                "<section class=\"search-results\"><h1>검색할 수 없습니다</h1><p>{}</p><a href=\"/\">전체 주제 보기 →</a></section>",
                markdown::escape(message)
            );
            return (
                StatusCode::BAD_REQUEST,
                Html(layout("검색 오류", &content, None)),
            );
        }
    };
    let mut content = String::from(
        "<section class=\"search-results\"><span class=\"eyebrow\">SEARCH THE NOTES</span><h1>문서 검색</h1>",
    );
    if query.is_empty() {
        content.push_str("<p>상단 검색창에 찾고 싶은 내용을 입력해 주세요.</p>");
    } else {
        let results: Vec<_> = DOCUMENTS
            .iter()
            .filter(|doc| matches_document(doc, &query))
            .collect();
        content.push_str(&format!(
            "<p><strong>{}</strong> 검색 결과 {}개</p>",
            markdown::escape(&query),
            results.len()
        ));
        if results.is_empty() {
            content.push_str("<p>일치하는 문서가 없습니다. 다른 검색어를 입력해 보세요.</p>");
        } else {
            content.push_str("<div class=\"results-list\">");
            for doc in results {
                let preview = body_preview(doc.markdown, &query)
                    .unwrap_or_else(|| doc.description.to_owned());
                content.push_str(&format!("<a class=\"result-card\" href=\"/docs/{}\"><span>{}</span><h2>{}</h2><p>{}</p><p class=\"result-preview\">{}</p><strong>문서 읽기 →</strong></a>", markdown::escape(doc.slug), markdown::escape(doc.category), markdown::escape(doc.title), markdown::escape(doc.description), markdown::escape(&preview)));
            }
            content.push_str("</div>");
        }
    }
    content.push_str("</section>");
    (
        StatusCode::OK,
        Html(layout("문서 검색", &content, Some(&query))),
    )
}

async fn document(Path(slug): Path<String>) -> impl IntoResponse {
    match DOCUMENTS.iter().find(|doc| doc.slug == slug) {
        Some(doc) => {
            let index = DOCUMENTS
                .iter()
                .position(|item| item.slug == doc.slug)
                .unwrap();
            let navigation = document_navigation(index);
            let content = format!(
                "<div class=\"doc-layout\"><div class=\"doc-navigation\"><div class=\"doc-toolbar\"><button id=\"sidebar-toggle\" class=\"sidebar-toggle\" type=\"button\" aria-controls=\"docs-sidebar\" aria-expanded=\"true\" hidden>메뉴 접기</button></div>{}</div><article class=\"article\"><div class=\"breadcrumbs\"><a href=\"/\">홈</a><span>/</span><span>{}</span></div><div class=\"doc-label\"><span class=\"dot\"></span> {} · 학습 노트 {:02}</div><div class=\"markdown\">{}</div>{}</article></div><script src=\"/sidebar.js\" defer></script>",
                sidebar(Some(doc.slug)),
                doc.category,
                doc.category,
                index + 1,
                markdown::render(doc.markdown),
                navigation
            );
            (StatusCode::OK, Html(layout(doc.title, &content, None)))
        }
        None => not_found().await,
    }
}

fn document_navigation(index: usize) -> String {
    let previous = index.checked_sub(1).and_then(|i| DOCUMENTS.get(i))
        .map(|doc| format!("<a class=\"next previous\" href=\"/docs/{}\"><span>이전 학습 노트</span><strong>← {}</strong></a>", doc.slug, doc.title))
        .unwrap_or_else(|| "<a class=\"next previous\" href=\"/\"><span>학습 시작</span><strong>← 전체 주제 보기</strong></a>".into());
    let next = DOCUMENTS.get(index + 1)
        .map(|doc| format!("<a class=\"next\" href=\"/docs/{}\"><span>다음 학습 노트</span><strong>{} →</strong></a>", doc.slug, doc.title))
        .unwrap_or_else(|| "<a class=\"next\" href=\"/\"><span>모든 학습 완료</span><strong>전체 주제 보기 →</strong></a>".into());
    format!("<nav class=\"doc-nav\" aria-label=\"학습 노트 이동\">{previous}{next}</nav>")
}

async fn stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("style.css"),
    )
}

async fn theme_script() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        include_str!("theme.js"),
    )
}

async fn sidebar_script() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        include_str!("sidebar.js"),
    )
}

async fn diagram(Path(filename): Path<String>) -> impl IntoResponse {
    match diagrams::get(&format!("/assets/{filename}")) {
        Some(svg) => (
            [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
            svg,
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn dijkstra_visual() -> impl IntoResponse {
    // Read HTML as bytes and convert to string, skipping any BOM
    let bytes = include_bytes!("../docs/dijkstra-visual.html");
    let html_str = if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        // Skip UTF-8 BOM
        std::str::from_utf8(&bytes[3..]).unwrap_or_default()
    } else {
        std::str::from_utf8(bytes).unwrap_or_default()
    };
    
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
        ],
        html_str,
    )
}

async fn dijkstra_algorithm_image() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        include_bytes!("../docs/assets/dijkstra-algorithm-steps.png").to_vec(),
    )
}

async fn dijkstra_shortest_path_diagram() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        include_bytes!("../docs/assets/dijkstra-shortest-path-diagram.png").to_vec(),
    )
}

async fn not_found() -> (StatusCode, Html<String>) {
    let content = "<section class=\"missing\"><span class=\"eyebrow\">404 / NOT FOUND</span><h1>페이지를 찾을 수 없어요.</h1><p>주소를 확인하거나 학습 노트로 돌아가 보세요.</p><a class=\"button primary\" href=\"/\">홈으로 돌아가기 →</a></section>";
    (
        StatusCode::NOT_FOUND,
        Html(layout("페이지를 찾을 수 없습니다", content, None)),
    )
}

fn sidebar(active: Option<&str>) -> String {
    let mut result = String::from(
        "<aside id=\"docs-sidebar\" class=\"sidebar\" aria-label=\"학습 노트 메뉴\"><a class=\"back-link\" href=\"/\">← 전체 주제</a><p class=\"side-title\">학습 노트</p>",
    );
    for category in ["자료구조", "알고리즘"] {
        result.push_str(&format!("<p class=\"side-category\">{category}</p>"));
        for doc in DOCUMENTS.iter().filter(|doc| doc.category == category) {
            let current = if active == Some(doc.slug) {
                " class=\"active\" aria-current=\"page\""
            } else {
                ""
            };
            result.push_str(&format!(
                "<a href=\"/docs/{}\"{}>{}</a>",
                doc.slug, current, doc.title
            ));
        }
    }
    result.push_str("<div class=\"side-bottom\">한 줄씩, 차근차근.<br><strong>Keep learning.</strong></div></aside>");
    result
}

fn layout(title: &str, content: &str, search_query: Option<&str>) -> String {
    let search_value = markdown::escape(search_query.unwrap_or(""));
    format!(
        r#"<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="description" content="Rust로 배우는 자료구조와 알고리즘 학습 노트"><title>{} | Rust 자료구조 노트</title><script src="/theme.js"></script><link rel="stylesheet" href="/style.css"></head><body><header class="header"><div class="header-inner"><a class="brand" href="/"><span class="brand-mark">R<span>.</span></span><span>RUST<span class="brand-light"> / STUDY NOTES</span></span></a><form class="header-search" action="/search" method="get" role="search"><label class="sr-only" for="global-search">문서 검색</label><input id="global-search" type="search" name="q" placeholder="문서 검색" value="{search_value}" maxlength="200"><button type="submit">검색</button></form><div class="theme-control"><label class="sr-only" for="theme-select">화면 테마</label><select id="theme-select" aria-label="화면 테마"><option value="light">밝은 테마</option><option value="dark">어두운 테마</option><option value="sepia">따뜻한 테마</option></select></div><nav aria-label="주 메뉴"><a href="/#topics">학습 노트</a><a href="https://doc.rust-lang.org/book/" target="_blank" rel="noopener noreferrer">Rust 공식 문서 ↗</a></nav></div></header><main>{}</main><footer><div><a class="brand footer-brand" href="/"><span class="brand-mark">R<span>.</span></span><span>RUST<span class="brand-light"> / STUDY NOTES</span></span></a><p>Rust로 배우는 자료구조와 알고리즘.</p></div><span>BUILT FOR CURIOUS MINDS · RUST</span></footer></body><script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script><script>mermaid.initialize({{ startOnLoad: true, theme: 'default' }}); mermaid.contentLoaded();</script></html>"#,
        markdown::escape(title),
        content
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn home_lists_all_documents_and_updates_the_count() {
        let html = home().await.0;
        assert!(html.contains(&format!("{:02}개의 학습 노트", DOCUMENTS.len())));
        for doc in DOCUMENTS {
            assert!(html.contains(&format!("href=\"/docs/{}\"", doc.slug)));
        }
    }

    #[test]
    fn preview_prefers_matching_body_and_handles_unicode_and_long_lines() {
        let source = format!(
            "# 제목\n첫 문단입니다.\n{}이진 탐색 목표{}\n",
            "앞".repeat(180),
            "뒤".repeat(180)
        );
        let preview = body_preview(&source, "탐색").unwrap();
        assert!(preview.contains("이진 탐색 목표"));
        assert!(preview.starts_with('…') && preview.ends_with('…'));
        assert!(preview.chars().count() <= 162);
        assert_eq!(
            body_preview("# 제목\n\n- 본문 내용\n", "제목만"),
            Some("본문 내용".into())
        );
        assert_eq!(body_preview("# 제목\n```rust\n```\n", "제목"), None);
        let mixed = format!("{}İstanbul 결과", "가".repeat(100));
        assert!(
            body_preview(&mixed, "İSTANBUL")
                .unwrap()
                .contains("İstanbul")
        );
    }

    #[test]
    fn preview_is_escaped_before_insertion_into_html() {
        let preview = body_preview("# 제목\n본문 <script>alert('x')</script>", "script").unwrap();
        let escaped = markdown::escape(&preview);
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;script&gt;"));
    }

    #[test]
    fn search_query_decodes_form_encoding_and_rejects_bad_input() {
        assert_eq!(
            search_query(Some("other=1&q=%EA%B2%80%EC%83%89+Vec")),
            Ok("검색 Vec".to_owned())
        );
        assert_eq!(search_query(Some("q=%26%2B")), Ok("&+".to_owned()));
        assert_eq!(search_query(None), Ok(String::new()));
        assert!(search_query(Some("q=%GG")).is_err());
        assert!(search_query(Some("q=%FF")).is_err());
        assert!(search_query(Some(&format!("q={}", "a".repeat(201)))).is_err());
    }

    #[tokio::test]
    async fn search_matches_title_and_body_and_handles_empty_or_unsafe_queries() {
        let (_, Html(title)) = search_documents(RawQuery(Some("q=VEC".into()))).await;
        assert!(title.contains("class=\"result-card\" href=\"/docs/vec\""));
        let (_, Html(body)) = search_documents(RawQuery(Some("q=checked_add".into()))).await;
        assert!(body.contains("class=\"result-card\" href=\"/docs/knapsack\""));
        assert!(body.contains("class=\"result-preview\""));
        assert!(body.contains("checked_add"));
        let (_, Html(korean)) =
            search_documents(RawQuery(Some("q=%EB%B0%B0%EB%82%AD".into()))).await;
        assert!(korean.contains("class=\"result-card\" href=\"/docs/knapsack\""));
        let (_, Html(empty)) = search_documents(RawQuery(None)).await;
        assert!(empty.contains("찾고 싶은 내용을 입력"));
        let (_, Html(none)) =
            search_documents(RawQuery(Some("q=unfindable_123456789".into()))).await;
        assert!(none.contains("일치하는 문서가 없습니다"));
        let (_, Html(escaped)) = search_documents(RawQuery(Some("q=%3Cscript%3E".into()))).await;
        assert!(!escaped.contains("<script>"));
        assert!(escaped.contains("&lt;script&gt;"));
        let (status, _) = search_documents(RawQuery(Some("q=%FF".into()))).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn expanded_algorithm_documents_include_practical_examples() {
        for slug in [
            "binary-search",
            "bfs",
            "dfs",
            "dijkstra",
            "merge-sort",
            "topological-sort",
        ] {
            let doc = DOCUMENTS.iter().find(|doc| doc.slug == slug).unwrap();
            assert!(doc.markdown.contains("## 실무 활용과 선택 기준"), "{slug}");
            assert!(doc.markdown.contains("## 추가 예제"), "{slug}");
            assert!(doc.markdown.matches("```rust").count() >= 2, "{slug}");
            let rendered = markdown::render(doc.markdown);
            assert!(
                rendered.contains("<table>"),
                "{slug} must render its comparison table"
            );
            assert!(
                rendered.contains("<strong>"),
                "{slug} must render bold emphasis"
            );
        }
    }

    #[test]
    fn all_diagrams_have_step_by_step_explanations() {
        for doc in DOCUMENTS {
            let description = doc
                .markdown
                .split_once("### 그림 단계별 읽기")
                .unwrap_or_else(|| panic!("{} needs a diagram explanation", doc.slug))
                .1;
            let section = description.split("\n## ").next().unwrap();
            assert!(
                section
                    .lines()
                    .filter(|line| line.starts_with("- "))
                    .count()
                    >= 3,
                "{} needs at least three explanation steps",
                doc.slug
            );
        }
    }

    #[tokio::test]
    async fn all_documents_have_accessible_bundled_diagrams() {
        for doc in DOCUMENTS {
            let filename = format!("{}.svg", doc.slug);
            let path = format!("/assets/{filename}");
            let svg = diagrams::get(&path).expect("document diagram must be registered");
            assert!(
                svg.contains("<svg") && svg.contains("<title") && svg.contains("<desc"),
                "{} needs an accessible SVG",
                doc.slug
            );
            assert!(
                !svg.contains("<script") && !svg.contains("<image"),
                "{} must not load active or external content",
                doc.slug
            );
            let html = markdown::render(doc.markdown);
            let prefix = format!("<img src=\"{path}\" alt=\"");
            let alt = html
                .split_once(&prefix)
                .and_then(|(_, rest)| rest.split_once('"'))
                .map(|(alt, _)| alt)
                .expect("document must render its diagram");
            assert!(
                !alt.trim().is_empty(),
                "{} needs alternative text",
                doc.slug
            );
            let response = diagram(Path(filename)).await.into_response();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                response.headers().get(header::CONTENT_TYPE).unwrap(),
                "image/svg+xml; charset=utf-8"
            );
        }
        let response = diagram(Path("unknown.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(diagrams::get("/assets/../btree-map.svg").is_none());
    }

    #[test]
    fn kmp_diagram_explains_the_overlapping_offsets() {
        let text = b"ABABABA";
        let pattern = b"ABABA";
        let matches: Vec<_> = text
            .windows(pattern.len())
            .enumerate()
            .filter_map(|(index, window)| (window == pattern).then_some(index))
            .collect();
        assert_eq!(matches, [0, 2]);
        let shared_start = matches[0].max(matches[1]);
        let shared_end = (matches[0] + pattern.len()).min(matches[1] + pattern.len());
        assert_eq!((shared_start, shared_end), (2, 5)); // Half-open range 2..5.
        let doc = DOCUMENTS.iter().find(|doc| doc.slug == "kmp").unwrap();
        assert!(doc.markdown.contains("오프셋 2부터 4까지를 공유"));
    }

    #[test]
    fn prefix_sum_diagram_matches_half_open_range_example() {
        let values = [3, -1, 4, 1, 5];
        let mut prefix = vec![0];
        for value in values {
            prefix.push(prefix.last().unwrap() + value);
        }
        assert_eq!(prefix, [0, 3, 2, 6, 7, 12]);
        assert_eq!(prefix[4] - prefix[1], 4);
        let svg = diagrams::get("/assets/prefix-sum.svg").unwrap();
        assert!(svg.contains("prefix[4] − prefix[1] = 7 − 3 = 4"));
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "prefix-sum")
            .unwrap();
        assert!(doc.markdown.contains("prefix[4] - prefix[1] = 7 - 3 = 4"));
    }

    #[tokio::test]
    async fn btree_diagram_is_accessible_and_matches_sorted_keys() {
        use std::collections::BTreeMap;
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "btree-map")
            .unwrap();
        let html = markdown::render(doc.markdown);
        assert!(html.contains("<img src=\"/assets/btree-map.svg\" alt=\"B-트리 개념도:"));
        assert!(html.contains("<table>"));
        assert!(html.contains("<th scope=\"col\" class=\"align-left\">HashMap</th>"));
        let svg = include_str!("btree-map.svg");
        assert!(svg.contains("<title id=\"title\">"));
        assert!(svg.contains("<desc id=\"description\">"));
        let map: BTreeMap<_, _> = [
            (40, "F"),
            (10, "A"),
            (50, "H"),
            (20, "C"),
            (15, "B"),
            (30, "E"),
            (45, "G"),
            (25, "D"),
        ]
        .into_iter()
        .collect();
        let order = map
            .keys()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(" → ");
        assert!(svg.contains(&order));
        let response = diagram(Path("btree-map.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn bst_traversal_diagram_is_rendered_and_has_correct_orders() {
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search-tree")
            .unwrap();
        let html = markdown::render(doc.markdown);
        assert!(html.contains("<img src=\"/assets/bst-traversals.svg\" alt=\"같은 이진 검색 트리"));
        let svg = diagrams::get("/assets/bst-traversals.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc"));
        assert!(svg.contains("1 → 3 → 6 → 8 → 10 → 12 → 15"));
        assert!(svg.contains("8 → 3 → 1 → 6 → 12 → 10 → 15"));
        let response = diagram(Path("bst-traversals.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn bst_extra_traversal_diagram_is_rendered_and_has_correct_orders() {
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search-tree")
            .unwrap();
        let html = markdown::render(doc.markdown);
        assert!(
            html.contains(
                "<img src=\"/assets/bst-extra-traversals.svg\" alt=\"같은 이진 검색 트리"
            )
        );
        let svg = diagrams::get("/assets/bst-extra-traversals.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc"));
        assert!(svg.contains("1 → 6 → 3 → 10 → 15 → 12 → 8"));
        assert!(svg.contains("8 → 3 → 12 → 1 → 6 → 10 → 15"));
        let response = diagram(Path("bst-extra-traversals.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml; charset=utf-8"
        );
    }

    #[test]
    fn bst_step_tables_match_traversals_and_queue_states() {
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search-tree")
            .unwrap();
        let steps = doc
            .markdown
            .split("## 네 가지 순회 단계별 따라가기")
            .nth(1)
            .unwrap();
        for (heading, expected) in [
            ("중위 순회", vec![1, 3, 6, 8, 10, 12, 15]),
            ("전위 순회", vec![8, 3, 1, 6, 12, 10, 15]),
            ("후위 순회", vec![1, 6, 3, 10, 15, 12, 8]),
        ] {
            let section = steps.split(&format!("### {heading}:")).nth(1).unwrap();
            let table = section.split("### ").next().unwrap();
            let visits: Vec<i32> = table
                .lines()
                .filter_map(|line| {
                    let cells: Vec<_> = line.split('|').map(str::trim).collect();
                    if cells.len() >= 4 && cells[1].parse::<usize>().is_ok() {
                        Some(cells[2].parse().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            assert_eq!(visits, expected, "{heading}");
        }
        let queue = steps.split("### 레벨 순회: 큐의 변화").nth(1).unwrap();
        for row in [
            "| 시작 | — | 루트 8 | `[8]` |",
            "| 1 | 8 | 3, 12 | `[3, 12]` |",
            "| 2 | 3 | 1, 6 | `[12, 1, 6]` |",
            "| 3 | 12 | 10, 15 | `[1, 6, 10, 15]` |",
            "| 4 | 1 | 없음 | `[6, 10, 15]` |",
            "| 5 | 6 | 없음 | `[10, 15]` |",
            "| 6 | 10 | 없음 | `[15]` |",
            "| 7 | 15 | 없음 | `[]` |",
        ] {
            assert!(queue.contains(row), "missing queue row: {row}");
        }
        let html = markdown::render(doc.markdown);
        assert!(html.contains("네 가지 순회 단계별 따라가기"));
        assert!(html.contains("레벨 순회: 큐의 변화"));
        assert!(html.contains("<code>[12, 1, 6]</code>"));
    }

    #[test]
    fn bst_recursive_trace_matches_call_stack_and_visits() {
        fn stack_label(stack: &[i32]) -> String {
            if stack.is_empty() {
                "비어 있음".to_owned()
            } else {
                stack
                    .iter()
                    .map(i32::to_string)
                    .collect::<Vec<_>>()
                    .join(" → ")
            }
        }
        fn trace(node: i32, stack: &mut Vec<i32>, rows: &mut Vec<String>) {
            let (left, right) = match node {
                8 => (Some(3), Some(12)),
                3 => (Some(1), Some(6)),
                12 => (Some(10), Some(15)),
                _ => (None, None),
            };
            stack.push(node);
            rows.push(format!(
                "| {node} 호출 | `{}` | {node} | — | — |",
                stack_label(stack)
            ));
            if let Some(left) = left {
                trace(left, stack, rows);
            }
            rows.push(format!(
                "| {node}의 왼쪽 완료 | `{}` | — | {node} | — |",
                stack_label(stack)
            ));
            if let Some(right) = right {
                trace(right, stack, rows);
            }
            stack.pop();
            rows.push(format!(
                "| {node} 반환 | `{}` | — | — | {node} |",
                stack_label(stack)
            ));
        }

        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search-tree")
            .unwrap();
        let section = doc
            .markdown
            .split("### 재귀 호출과 반환 과정")
            .nth(1)
            .unwrap()
            .split("### 레벨 순회: 큐의 변화")
            .next()
            .unwrap();
        let mut expected = Vec::new();
        trace(8, &mut Vec::new(), &mut expected);
        let rows: Vec<_> = section
            .lines()
            .filter(|line| line.starts_with("| "))
            .skip(2)
            .collect();
        assert_eq!(
            rows,
            expected.iter().map(String::as_str).collect::<Vec<_>>()
        );
        let html = markdown::render(doc.markdown);
        assert!(html.contains("재귀 호출과 반환 과정"));
        assert!(html.contains("<code>8 → 3 → 1</code>"));
    }

    #[tokio::test]
    async fn lower_bound_explanation_matches_search_steps() {
        let values = [1, 2, 2, 2, 5, 8];
        let (mut left, mut right) = (0, values.len());
        let mut steps = Vec::new();
        while left < right {
            let middle = left + (right - left) / 2;
            steps.push((left, right, middle, values[middle]));
            if values[middle] < 2 {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        assert_eq!(steps, [(0, 6, 3, 2), (0, 3, 1, 2), (0, 1, 0, 1)]);
        assert_eq!(left, 1);
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search")
            .unwrap();
        let section = doc
            .markdown
            .split("## Lower bound 탐색 과정")
            .nth(1)
            .unwrap()
            .split("## Upper bound 다이어그램")
            .next()
            .unwrap();
        for text in [
            "[0, 6)",
            "[0, 3)",
            "[0, 1)",
            "[1, 1)",
            "left = middle + 1",
            "right = middle",
            "배열 길이 `len`",
            "[1, 4)",
        ] {
            assert!(
                section.contains(text),
                "missing lower bound explanation: {text}"
            );
        }
        let html = markdown::render(doc.markdown);
        assert!(html.contains("Lower bound 탐색 과정"));
        assert!(html.contains("<code>[1, 1)</code>"));
        assert!(html.contains("<img src=\"/assets/lower-bound.svg\" alt=\"정렬된 배열"));
        let svg = diagrams::get("/assets/lower-bound.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc"));
        for text in ["[0, 6)", "[0, 3)", "[0, 1)", "[1, 1)", "lower bound = 1"] {
            assert!(svg.contains(text));
        }
        let response = diagram(Path("lower-bound.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn upper_bound_diagram_matches_search_and_is_served() {
        let values = [1, 2, 2, 2, 5, 8];
        let (mut left, mut right) = (0, values.len());
        let mut steps = Vec::new();
        while left < right {
            let middle = left + (right - left) / 2;
            steps.push((left, right, middle, values[middle]));
            if values[middle] <= 2 {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        assert_eq!(steps, [(0, 6, 3, 2), (4, 6, 5, 8), (4, 5, 4, 5)]);
        assert_eq!(left, 4);
        let doc = DOCUMENTS
            .iter()
            .find(|doc| doc.slug == "binary-search")
            .unwrap();
        let html = markdown::render(doc.markdown);
        assert!(html.contains("<img src=\"/assets/upper-bound.svg\" alt=\"정렬된 배열"));
        for text in [
            "left = middle + 1",
            "right = middle",
            "반환할 수 있는 경계로 보존됩니다",
            "원소 인덱스로 직접 접근할 수는 없습니다",
        ] {
            assert!(
                html.contains(text),
                "missing upper bound explanation: {text}"
            );
        }
        let svg = diagrams::get("/assets/upper-bound.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc"));
        for text in ["[0, 6)", "[4, 6)", "[4, 5)", "[4, 4)", "upper bound = 4"] {
            assert!(svg.contains(text));
        }
        let response = diagram(Path("upper-bound.svg".to_owned()))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/svg+xml; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn document_sidebar_toggle_has_accessible_progressive_markup() {
        let response = document(Path("vec".to_owned())).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let html = std::str::from_utf8(&bytes).unwrap();
        assert!(html.contains("<aside id=\"docs-sidebar\" class=\"sidebar\""));
        assert!(html.contains("aria-controls=\"docs-sidebar\" aria-expanded=\"true\" hidden"));
        assert!(html.contains("<script src=\"/sidebar.js\" defer></script>"));
        let script = sidebar_script().await.into_response();
        assert_eq!(
            script.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/javascript; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn theme_controls_and_script_are_available() {
        let html = layout("문서", "본문", None);
        assert!(html.contains("<script src=\"/theme.js\"></script>"));
        assert!(html.contains("id=\"theme-select\""));
        for theme in ["light", "dark", "sepia"] {
            assert!(html.contains(&format!("value=\"{theme}\"")));
        }
        let response = theme_script().await.into_response();
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/javascript; charset=utf-8"
        );
    }

    #[test]
    fn header_search_is_available_on_document_layouts() {
        let html = layout("문서", "본문", None);
        assert!(html.contains("action=\"/search\" method=\"get\""));
        assert!(html.contains("name=\"q\""));
        assert!(!html.contains("value=\"vec\""));
    }

    #[test]
    fn navigation_links_to_adjacent_documents_and_home_at_boundaries() {
        let first = document_navigation(0);
        assert!(first.contains("class=\"next previous\" href=\"/\""));
        assert!(first.contains(&format!(
            "class=\"next\" href=\"/docs/{}\"",
            DOCUMENTS[1].slug
        )));

        for index in 1..DOCUMENTS.len() - 1 {
            let html = document_navigation(index);
            assert!(html.contains(&format!(
                "class=\"next previous\" href=\"/docs/{}\"",
                DOCUMENTS[index - 1].slug
            )));
            assert!(html.contains(&format!(
                "class=\"next\" href=\"/docs/{}\"",
                DOCUMENTS[index + 1].slug
            )));
        }

        let last = document_navigation(DOCUMENTS.len() - 1);
        assert!(last.contains(&format!(
            "class=\"next previous\" href=\"/docs/{}\"",
            DOCUMENTS[DOCUMENTS.len() - 2].slug
        )));
        assert!(last.contains("class=\"next\" href=\"/\""));
    }

    #[test]
    fn documents_have_unique_slugs_and_required_sections() {
        for (i, doc) in DOCUMENTS.iter().enumerate() {
            assert!(!DOCUMENTS[..i].iter().any(|other| other.slug == doc.slug));
            assert!(doc.markdown.starts_with('#'));
            assert!(doc.markdown.contains("```rust"));
            assert!(doc.markdown.contains("#[cfg(test)]"));
            assert!(doc.markdown.contains("https://"));
        }
    }

    #[test]
    fn markdown_examples_compile_and_pass_their_tests() {
        use std::{fs, process::Command};
        let dir = std::env::temp_dir().join(format!("rust-study-examples-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create temporary example directory");
        for doc in DOCUMENTS {
            let normalized = doc.markdown.replace("\r\n", "\n");
            let blocks: Vec<_> = normalized.split("```rust\n").skip(1).collect();
            assert!(
                !blocks.is_empty(),
                "{} must contain a Rust example",
                doc.slug
            );
            for (index, block) in blocks.into_iter().enumerate() {
                let (code, _) = block
                    .split_once("```")
                    .expect("Rust example must have a closing code fence");
                let name = format!("{}-{}", doc.slug, index + 1);
                let source = dir.join(format!("{name}.rs"));
                let executable = dir.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
                let test_executable =
                    dir.join(format!("{name}-test{}", std::env::consts::EXE_SUFFIX));
                fs::write(&source, code).expect("write example");
                for (output, test) in [(&executable, false), (&test_executable, true)] {
                    let mut compiler = Command::new("rustc");
                    compiler
                        .arg("--edition=2024")
                        .arg("--crate-name")
                        .arg(doc.slug.replace('-', "_"));
                    if test {
                        compiler.arg("--test");
                    }
                    let result = compiler
                        .arg(&source)
                        .arg("-o")
                        .arg(output)
                        .output()
                        .expect("run rustc");
                    assert!(
                        result.status.success(),
                        "{} compilation failed: {}",
                        name,
                        String::from_utf8_lossy(&result.stderr)
                    );
                    let result = Command::new(output).output().expect("run compiled example");
                    assert!(
                        result.status.success(),
                        "{} execution failed:\n{}\n{}",
                        name,
                        String::from_utf8_lossy(&result.stdout),
                        String::from_utf8_lossy(&result.stderr)
                    );
                }
            }
        }
        fs::remove_dir_all(dir).expect("remove temporary examples");
    }

    #[test]
    fn sidebar_marks_current_document() {
        let output = sidebar(Some("vec"));
        assert!(output.contains("href=\"/docs/vec\" class=\"active\" aria-current=\"page\""));
    }
}
