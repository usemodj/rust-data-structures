// A deliberately small Markdown renderer for the site's own documentation.
// Raw HTML is escaped, links are HTTP(S), and images are restricted to bundled assets.
pub fn render(source: &str) -> String {
    let mut html = String::new();
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_buffer = String::new();
    let mut in_list = false;
    let mut paragraph = String::new();

    fn flush_paragraph(html: &mut String, paragraph: &mut String) {
        if !paragraph.is_empty() {
            html.push_str("<p>");
            html.push_str(&inline(paragraph));
            html.push_str("</p>\n");
            paragraph.clear();
        }
    }

    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            flush_paragraph(&mut html, &mut paragraph);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            if in_code {
                if code_lang == "mermaid" {
                    html.push_str("<div class=\"mermaid\">\n");
                    html.push_str(&code_buffer);
                    html.push_str("\n</div>\n");
                } else if !code_lang.is_empty() {
                    html.push_str("</code></pre>\n");
                } else {
                    html.push_str("</code></pre>\n");
                }
                code_lang.clear();
                code_buffer.clear();
            } else {
                let lang = trimmed.trim_start_matches("```").trim();
                code_lang = if !lang.is_empty() { lang.to_string() } else { String::new() };
                if code_lang == "mermaid" {
                    // Mermaid blocks don't output opening tag yet, wait for content
                } else if !code_lang.is_empty() {
                    html.push_str(&format!("<pre><code class=\"language-{}\">", escape(&code_lang)));
                } else {
                    html.push_str("<pre><code>");
                }
            }
            in_code = !in_code;
            continue;
        }
        if in_code {
            if code_lang == "mermaid" {
                code_buffer.push_str(line);
                code_buffer.push('\n');
            } else {
                html.push_str(&escape(line));
                html.push('\n');
            }
            continue;
        }
        if trimmed.is_empty() {
            flush_paragraph(&mut html, &mut paragraph);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            continue;
        }
        if trimmed.contains('|') {
            let headers = table_cells(trimmed);
            let alignments = lines.peek().and_then(|next| {
                if headers.len() < 2 {
                    None
                } else {
                    table_alignments(next.trim(), headers.len())
                }
            });
            if let Some(alignments) = alignments {
                flush_paragraph(&mut html, &mut paragraph);
                if in_list {
                    html.push_str("</ul>\n");
                    in_list = false;
                }
                lines.next(); // Consume the delimiter row.
                html.push_str("<div class=\"table-scroll\" tabindex=\"0\" role=\"region\" aria-label=\"표: 가로 스크롤 가능\"><table><thead><tr>");
                for (header, alignment) in headers.iter().zip(&alignments) {
                    html.push_str(&format!(
                        "<th scope=\"col\" class=\"align-{alignment}\">{}</th>",
                        inline(header)
                    ));
                }
                html.push_str("</tr></thead><tbody>");
                while let Some(next) = lines.peek() {
                    let row = next.trim();
                    if row.is_empty()
                        || !row.contains('|')
                        || row.starts_with("```")
                        || row.starts_with('#')
                        || row.starts_with("- ")
                    {
                        break;
                    }
                    let cells = table_cells(row);
                    html.push_str("<tr>");
                    for (index, alignment) in alignments.iter().enumerate() {
                        let cell = cells.get(index).map(String::as_str).unwrap_or("");
                        html.push_str(&format!(
                            "<td class=\"align-{alignment}\">{}</td>",
                            inline(cell)
                        ));
                    }
                    html.push_str("</tr>");
                    lines.next();
                }
                html.push_str("</tbody></table></div>\n");
                continue;
            }
        }
        let heading = (1..=4).rev().find_map(|level| {
            let prefix = format!("{} ", "#".repeat(level));
            trimmed.strip_prefix(&prefix).map(|text| (level, text))
        });
        if let Some((level, text)) = heading {
            flush_paragraph(&mut html, &mut paragraph);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str(&format!("<h{level}>{}</h{level}>\n", inline(text)));
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            flush_paragraph(&mut html, &mut paragraph);
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!("<li>{}</li>\n", inline(item)));
        } else {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(trimmed);
        }
    }
    flush_paragraph(&mut html, &mut paragraph);
    if in_list {
        html.push_str("</ul>\n");
    }
    if in_code {
        html.push_str("</code></pre>\n");
    }
    html
}

// Split a simple table row while preserving pipes inside inline code or escaped as \\|.
fn table_cells(line: &str) -> Vec<String> {
    let line = line.trim().strip_prefix('|').unwrap_or(line.trim());
    let mut chars = line.chars().peekable();
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut in_code = false;
    let mut ended_with_separator = false;
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek() == Some(&'|') {
            chars.next();
            cell.push('|');
            ended_with_separator = false;
        } else if ch == '`' {
            in_code = !in_code;
            cell.push(ch);
            ended_with_separator = false;
        } else if ch == '|' && !in_code {
            cells.push(cell.trim().to_owned());
            cell.clear();
            ended_with_separator = true;
        } else {
            cell.push(ch);
            ended_with_separator = false;
        }
    }
    if !ended_with_separator || !cell.trim().is_empty() {
        cells.push(cell.trim().to_owned());
    }
    cells
}

fn table_alignments(line: &str, columns: usize) -> Option<Vec<&'static str>> {
    let cells = table_cells(line);
    if cells.len() != columns {
        return None;
    }
    cells
        .iter()
        .map(|cell| {
            let left = cell.starts_with(':');
            let right = cell.ends_with(':');
            let dashes = cell.strip_prefix(':').unwrap_or(cell);
            let dashes = dashes.strip_suffix(':').unwrap_or(dashes);
            if dashes.len() < 3 || !dashes.bytes().all(|b| b == b'-') {
                return None;
            }
            Some(match (left, right) {
                (true, true) => "center",
                (false, true) => "right",
                _ => "left",
            })
        })
        .collect()
}

pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn inline(mut text: &str) -> String {
    let mut result = String::new();
    while !text.is_empty() {
        if let Some(rest) = text.strip_prefix("![") {
            if let Some(end) = rest.find("](") {
                let after = &rest[end + 2..];
                if let Some(close) = after.find(')') {
                    let alt_text = &rest[..end];
                    let url = &after[..close];
                    // Normalize relative paths like ./assets/... to /assets/...
                    let normalized_url = if url.starts_with("./") {
                        &url[2..]
                    } else if url.starts_with("/") {
                        url
                    } else {
                        url
                    };
                    let asset_url = if normalized_url.starts_with("assets/") {
                        format!("/{}", normalized_url)
                    } else {
                        normalized_url.to_string()
                    };
                    // Check if it's an SVG diagram or any other image
                    if crate::diagrams::get(&asset_url).is_some() {
                        result.push_str(&format!(
                            "<span class=\"diagram-scroll\" tabindex=\"0\" aria-label=\"다이어그램: 가로 스크롤 가능\"><img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\"></span>",
                            escape(&asset_url), escape(alt_text)
                        ));
                    } else {
                        // Regular image (like PNG)
                        result.push_str(&format!(
                            "<img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\">",
                            escape(&asset_url), escape(alt_text)
                        ));
                    }
                    text = &after[close + 1..];
                    continue;
                }
            }
        }
        if let Some(rest) = text.strip_prefix('`') {
            if let Some(end) = rest.find('`') {
                result.push_str("<code>");
                result.push_str(&escape(&rest[..end]));
                result.push_str("</code>");
                text = &rest[end + 1..];
                continue;
            }
        }
        if let Some(rest) = text.strip_prefix("**") {
            if let Some(end) = rest.find("**") {
                if end > 0 {
                    result.push_str("<strong>");
                    result.push_str(&inline(&rest[..end]));
                    result.push_str("</strong>");
                    text = &rest[end + 2..];
                    continue;
                }
            }
        }
        if let Some(rest) = text.strip_prefix('[') {
            if let Some(end) = rest.find("](") {
                let after = &rest[end + 2..];
                if let Some(close) = after.find(')') {
                    let url = &after[..close];
                    // Normalize relative paths like ./assets/... to /assets/...
                    let normalized_url = if url.starts_with("./") {
                        &url[2..]
                    } else {
                        url
                    };
                    // Allow https://, http://, /, or relative paths starting with assets/
                    if url.starts_with("https://") || url.starts_with("http://") || url.starts_with("/") || normalized_url.starts_with("assets/") {
                        let final_url = if normalized_url.starts_with("assets/") {
                            format!("/{}", normalized_url)
                        } else {
                            normalized_url.to_string()
                        };
                        result.push_str(&format!(
                            "<a href=\"{}\" {}>{}</a>",
                            escape(&final_url),
                            if final_url.starts_with("https://") || final_url.starts_with("http://") {
                                "target=\"_blank\" rel=\"noopener noreferrer\""
                            } else {
                                ""
                            },
                            inline(&rest[..end])
                        ));
                        text = &after[close + 1..];
                        continue;
                    }
                }
            }
        }
        let ch = text.chars().next().expect("text is not empty");
        result.push_str(&escape(&ch.to_string()));
        text = &text[ch.len_utf8()..];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_supported_markdown_and_escapes_html() {
        let html = render(
            "# 제목\n\n본문 `Vec<T>`\n\n- [공식 문서](https://doc.rust-lang.org/)\n\n```rust\nlet x = <script>;\n```\n",
        );
        assert!(html.contains("<h1>제목</h1>"));
        assert!(html.contains("<code>Vec&lt;T&gt;</code>"));
        assert!(html.contains("href=\"https://doc.rust-lang.org/\""));
        assert!(html.contains("let x = &lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn renders_quicksort_partition_steps() {
        let html = render(include_str!("../docs/quicksort.md"));
        assert!(html.contains("<img src=\"/assets/quicksort.svg\""));
        assert!(html.contains("<img src=\"/assets/quicksort-steps.svg\" alt=\"퀵 정렬 예제"));
        let svg = crate::diagrams::get("/assets/quicksort-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for (stage, array, indices) in [
            ("1", "[2, 3, 7, 3, 1, 8, 9]", "0, 0, 6"),
            ("2", "[2, 3, 7, 3, 1, 8, 9]", "1, 1, 6"),
            ("3", "[2, 3, 7, 3, 1, 8, 9]", "1, 2, 6"),
            ("4", "[2, 3, 8, 3, 1, 7, 9]", "1, 2, 5"),
            ("5", "[2, 3, 1, 3, 8, 7, 9]", "1, 2, 4"),
            ("6", "[2, 1, 3, 3, 8, 7, 9]", "2, 3, 4"),
            ("7", "[2, 1, 3, 3, 8, 7, 9]", "2, 4, 4"),
        ] {
            let row = svg
                .lines()
                .find(|line| line.contains(&format!(">{stage}</text>")))
                .unwrap();
            assert!(
                row.contains(array) && row.contains(indices),
                "wrong quicksort stage {stage}"
            );
        }
        assert!(svg.contains("최종 결과: [1, 2, 3, 3, 7, 8, 9]"));
    }

    #[test]
    fn renders_kmp_prefix_and_overlapping_match_steps() {
        let html = render(include_str!("../docs/kmp.md"));
        assert!(html.contains("<img src=\"/assets/kmp.svg\""));
        assert!(html.contains("<img src=\"/assets/kmp-steps.svg\" alt=\"텍스트 ABABABA"));
        assert!(html.contains("main"));
        let svg = crate::diagrams::get("/assets/kmp-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for text in [
            "text = ABABABA, pattern = ABABA",
            "lps[2]=1",
            "lps[3]=2",
            "lps[4]=3",
            "t=4, p=4",
            "일치 [0..5)",
            "t=5, p=3",
            "t=6, p=4",
            "일치 [2..7)",
            "결과는 [0, 2]",
        ] {
            assert!(svg.contains(text), "missing KMP step: {text}");
        }
    }

    #[test]
    fn renders_dijkstra_steps_with_stale_candidates() {
        let html = render(include_str!("../docs/dijkstra.md"));
        assert!(html.contains("<img src=\"/assets/dijkstra.svg\""));
        assert!(html.contains("<img src=\"/assets/dijkstra-steps.svg\" alt=\"다익스트라 예제"));
        assert!(html.contains("간선을 완화하지 않고 건너뜁니다"));
        let svg = crate::diagrams::get("/assets/dijkstra-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for text in [
            "dist = [0, 4, 1, ∞]",
            "heap: [(3,1), (4,1), (6,3)]",
            "heap: [(4,1), (4,3), (6,3)]",
            "현재 distance[1] = Some(3)",
            "4 ≠ 3 → 오래된 후보 건너뜀",
            "Reverse((6,3))",
            "현재 거리: [0, 3, 1, 4]",
        ] {
            assert!(svg.contains(text), "missing Dijkstra step: {text}");
        }
    }

    #[test]
    fn renders_merge_sort_steps_with_stable_tie() {
        let html = render(include_str!("../docs/merge-sort.md"));
        assert!(html.contains("<img src=\"/assets/merge-sort.svg\""));
        assert!(html.contains("<img src=\"/assets/merge-sort-steps.svg\" alt=\"병합 정렬 예제"));
        assert!(html.contains("분할과 병합 따라가기"));
        assert!(html.contains("<code>[1(L), 1(R), 3, 4, 5]</code>"));
        let svg = crate::diagrams::get("/assets/merge-sort-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for (stage, comparison, selected, result) in [
            ("1", "1(L) · 1(R)", "1(L)", "[1(L)]"),
            ("2", "5 · 1(R)", "1(R)", "[1(L), 1(R)]"),
            ("3", "5 · 3", "3", "[1(L), 1(R), 3]"),
            ("4", "5 · 4", "4", "[1(L), 1(R), 3, 4]"),
            ("5", "5 · 없음", "남은 5", "[1(L), 1(R), 3, 4, 5]"),
        ] {
            let row = svg
                .lines()
                .find(|line| line.contains(&format!(">{stage}</text>")))
                .unwrap();
            assert!(
                row.contains(comparison) && row.contains(selected) && row.contains(result),
                "wrong merge stage {stage}"
            );
        }
        assert!(svg.contains("같은 값은 왼쪽을 먼저"));
    }

    #[test]
    fn renders_dfs_stack_diagram_and_trace() {
        let html = render(include_str!("../docs/dfs.md"));
        assert!(html.contains("<img src=\"/assets/dfs.svg\""));
        assert!(html.contains("<img src=\"/assets/dfs-stack-steps.svg\" alt=\"양방향 간선"));
        assert!(html.contains("스택과 방문 순서 따라가기"));
        assert!(html.contains("<code>[2, 1]</code>"));
        assert!(html.contains("<code>[2, 2]</code>"));
        let svg = crate::diagrams::get("/assets/dfs-stack-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for (stage, stack, order) in [
            ("시작", "[0]", "[]"),
            ("0 처리", "[2, 1]", "[0]"),
            ("1 처리", "[2, 3]", "[0, 1]"),
            ("3 처리", "[2]", "[0, 1, 3]"),
            ("2 처리", "[]", "[0, 1, 3, 2]"),
        ] {
            let row = svg
                .lines()
                .find(|line| line.contains(&format!(">{stage}</text>")))
                .unwrap();
            assert!(row.contains(stack), "wrong stack for {stage}");
            assert!(row.contains(order), "wrong order for {stage}");
        }
        assert!(svg.contains("오른쪽이 맨 위"));
    }

    #[test]
    fn renders_bfs_queue_trace_and_distance_table() {
        let html = render(include_str!("../docs/bfs.md"));
        assert!(html.contains("<img src=\"/assets/bfs.svg\""));
        assert!(html.contains("<img src=\"/assets/bfs-queue-steps.svg\" alt=\"방향 그래프"));
        assert!(html.contains("큐와 거리 따라가기"));
        assert!(html.contains("반복 3: 정점 2 처리"));
        assert!(html.contains("<div class=\"table-scroll\""));
        assert!(html.contains("<code>[1, 2]</code>"));
        assert!(html.contains("<code>[Some(0), Some(1), Some(1), Some(2)]</code>"));
        assert!(html.contains("큐에도 다시 넣지 않습니다"));
    }

    #[test]
    fn bfs_diagram_queue_rows_match_distance_trace() {
        let svg = crate::diagrams::get("/assets/bfs-queue-steps.svg").unwrap();
        assert!(svg.contains("<title") && svg.contains("<desc") && svg.contains("viewBox="));
        for (stage, queue, distance) in [
            ("시작", "[0]", "[0, –, –, –]"),
            ("0 처리", "[1, 2]", "[0, 1, 1, –]"),
            ("1 처리", "[2, 3]", "[0, 1, 1, 2]"),
            ("2 처리", "[3]", "[0, 1, 1, 2]"),
            ("3 처리", "[]", "[0, 1, 1, 2]"),
        ] {
            let row = svg
                .lines()
                .find(|line| line.contains(&format!(">{stage}</text>")))
                .unwrap();
            assert!(row.contains(queue), "wrong queue for {stage}");
            assert!(row.contains(distance), "wrong distance for {stage}");
        }
        assert!(svg.contains("큐에 다시 넣지 않습니다"));
    }

    #[test]
    fn renders_sorting_guide_examples_and_comparison_table() {
        let html = render(include_str!("../docs/sorting.md"));
        assert!(html.contains("<img src=\"/assets/sorting.svg\""));
        assert!(html.contains("<h2>정렬을 선택할 때</h2>"));
        assert!(html.contains("<div class=\"table-scroll\""));
        assert!(html.contains("<h2>실용 예제 1: 안정 정렬과 복합 기준</h2>"));
        assert!(html.contains("<h2>실용 예제 2: 부동소수점과 NaN 정렬</h2>"));
        assert!(html.contains("sort_unstable_by_key"));
        assert!(html.contains("f64::total_cmp"));
    }

    #[test]
    fn renders_bold_without_interpreting_html_or_code() {
        let html = render(
            "**굵은 `Vec<T>`와 [문서](https://doc.rust-lang.org/)**\n\n`**코드**`\n\n```rust\n**not bold**\n```\n\n**<script>**",
        );
        assert!(html.contains(
            "<strong>굵은 <code>Vec&lt;T&gt;</code>와 <a href=\"https://doc.rust-lang.org/\""
        ));
        assert!(html.contains("<code>**코드**</code>"));
        assert!(html.contains("<pre><code>**not bold**"));
        assert!(html.contains("<strong>&lt;script&gt;</strong>"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn renders_tables_with_alignment_code_links_and_safe_html() {
        let html = render(
            "| **항목** | 링크 | 코드 |\n| :--- | :---: | ---: |\n| <script> | [문서](https://doc.rust-lang.org/) | `a|b` |\n| 안전 \\| 값 | 두 번째 | 3 |\n",
        );
        assert!(html.contains("<div class=\"table-scroll\""));
        assert!(html.contains("<th scope=\"col\" class=\"align-left\"><strong>항목</strong></th>"));
        assert!(html.contains("class=\"align-center\""));
        assert!(html.contains("class=\"align-right\"><code>a|b</code>"));
        assert!(html.contains("안전 | 값"));
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
        assert!(html.contains("href=\"https://doc.rust-lang.org/\""));
        assert!(!render("| a | b |\n| -- | --- |\n| x | y |").contains("<table>"));
        assert!(!render("```text\n| a | b |\n| --- | --- |\n```").contains("<table>"));
    }

    #[test]
    fn renders_only_the_bundled_diagram_and_escapes_alt_text() {
        let html = render("![키 \"<script>\"](/assets/btree-map.svg)");
        assert!(html.contains("<img src=\"/assets/btree-map.svg\""));
        assert!(html.contains("alt=\"키 &quot;&lt;script&gt;&quot;\""));
        assert!(html.contains("class=\"diagram-scroll\""));
        assert!(!html.contains("<script>"));
        for source in [
            "![x](https://example.com/image.svg)",
            "![x](/assets/../private.svg)",
            "![x](javascript:alert)",
        ] {
            assert!(!render(source).contains("<img"));
        }
        assert!(!render("```rust\n![x](/assets/btree-map.svg)\n```").contains("<img"));
    }

    #[test]
    fn unsafe_links_and_html_are_not_interpreted() {
        let html = render("[x](javascript:alert)\n\n<img src=x onerror=alert(1)>");
        assert!(!html.contains("<a "));
        assert!(!html.contains("<img"));
        assert!(html.contains("&lt;img"));
    }
}
