use core::fmt;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize, // exclusive
}

impl Span {
    pub fn merge(a: Span, b: Span) -> Span {
        Span {
            start: a.start,
            end: b.end,
        }
    }
    pub fn highlight_span(&self) -> String {
        let source = fs::read_to_string("demo/main.su").expect("failed to read file");
        let start = self.start.min(source.len());
        let end = self.end.min(source.len());
        // Precompute line starts
        let mut line_starts = vec![0];
        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }

        // Find line index
        let line_idx = match line_starts.binary_search(&start) {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        let line_start = line_starts[line_idx];
        let line_end = source[line_start..]
            .find('\n')
            .map(|i| line_start + i)
            .unwrap_or(source.len());

        let line = &source[line_start..line_end];

        let line_no = line_idx + 1;

        // Column calculation
        let col_start = start - line_start;
        let col_len = (end - start).max(1);

        let gutter = format!("{line_no} | ");
        let mut marker = String::new();
        marker.push_str(&" ".repeat(gutter.len() + col_start));
        marker.push_str(&"^".repeat(col_len));

        let mut out = String::new();

        // Previous line (context)
        if line_idx > 0 {
            let prev_start = line_starts[line_idx - 1];
            let prev_end = line_start - 1;
            let prev_line = &source[prev_start..prev_end];
            out.push_str(&format!("{} | {}\n", line_no - 1, prev_line));
        }

        // Current line
        out.push_str(&format!("{}{}\n", gutter, line));
        out.push_str(&format!("{}\n", marker));

        // Next line (context)
        if line_end < source.len() {
            let next_start = line_end + 1;
            let next_end = source[next_start..]
                .find('\n')
                .map(|i| next_start + i)
                .unwrap_or(source.len());
            let next_line = &source[next_start..next_end];
            out.push_str(&format!("{} | {}\n", line_no + 1, next_line));
        }

        out
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n{}", self.highlight_span())
    }
}
