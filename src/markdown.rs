// A deliberately small Markdown renderer for the site's own documentation.
// Raw HTML is escaped, links are HTTP(S), and images are restricted to bundled assets.
pub fn render(source: &str) -> String {
    let mut html = String::new();
    let mut in_code = false;
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
                html.push_str("</code></pre>\n");
            } else {
                html.push_str("<pre><code>");
            }
            in_code = !in_code;
            continue;
        }
        if in_code {
            html.push_str(&escape(line));
            html.push('\n');
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
                    let url = &after[..close];
                    if crate::diagrams::get(url).is_some() {
                        result.push_str(&format!(
                            "<span class=\"diagram-scroll\" tabindex=\"0\" aria-label=\"다이어그램: 가로 스크롤 가능\"><img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\"></span>",
                            escape(url), escape(&rest[..end])
                        ));
                        text = &after[close + 1..];
                        continue;
                    }
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
                    if url.starts_with("https://") || url.starts_with("http://") {
                        result.push_str(&format!(
                            "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                            escape(url),
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
