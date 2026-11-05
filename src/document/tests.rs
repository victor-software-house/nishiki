//! What the oracle comparison in `tests/it/stream.rs` cannot see: how often
//! a document lexes each line, and what streaming allocates.

use nishiki_alloc_count::count;

use super::{Carry, Counts, Document, LastLine, Stream, Tail, Until};
use crate::test_support::oracle_documents;
use crate::{Highlighter, Language, Limits, Theme, ThemeSource};

fn theme() -> Theme {
    Theme::new(ThemeSource::default())
}

/// Pushes `text` in pieces of up to `size` characters, lexing after each.
fn stream(document: &mut Document, text: &str, size: usize) {
    let mut from = 0;
    for (index, (at, _)) in text.char_indices().enumerate() {
        if index > 0 && index % size == 0 {
            document.push_str(&text[from..at]);
            document.lex(Until::End);
            from = at;
        }
    }
    document.push_str(&text[from..]);
    document.lex(Until::End);
}

/// A line feed that arrives alone completes a line whose text the last line
/// already lexed, so the line keeps those tokens.
#[test]
fn a_line_feed_alone_lexes_no_completed_line() {
    let mut document = Document::new(Language::Rust, theme());
    let text = indoc::indoc! {"
        fn main() {
            let x = 1;
        }
    "};
    stream(&mut document, text, 1);
    assert_eq!(document.counts.completed, 0);
    assert_eq!(document.line_count(), 4);
}

/// A line completed in the delta that changes its text is lexed once, and
/// lexing again lexes nothing.
#[test]
fn a_completed_line_is_lexed_once() {
    let mut document = Document::new(Language::Rust, theme());
    document.push_str("let x = 1;\nlet y");
    document.lex(Until::End);
    document.push_str(" = 2;\n");
    document.lex(Until::End);
    document.lex(Until::End);
    assert_eq!(document.counts.completed, 2);
    assert_eq!(document.counts.tail, 2);
}

/// A delta inside a line lexes only the last line.
#[test]
fn a_delta_inside_a_line_lexes_only_the_last_line() {
    let mut document = Document::new(Language::Python, theme());
    let text = indoc::indoc! {"
        def f():
            return 1
    "};
    stream(&mut document, text, 64);
    let before = document.counts;
    document.push_str("x = ");
    document.lex(Until::End);
    assert_eq!(document.counts.completed, before.completed);
    assert_eq!(document.counts.tail, before.tail + 1);
}

/// A document built around a highlighter that already met every state of
/// `text`, with room for all of it.
fn warm(
    highlighter: Highlighter,
    text: usize,
    lines: usize,
    tokens: usize,
    tail: usize,
) -> Document {
    Document {
        highlighter,
        theme: theme(),
        text: String::with_capacity(text),
        carry: Carry::default(),
        lines: Vec::with_capacity(lines),
        tokens: Vec::with_capacity(tokens),
        lexed: 0,
        tail: LastLine {
            tokens: Vec::with_capacity(tail),
            lexed: None,
        },
        tail_rule: Tail::default(),
        stream: Stream::Open,
        dirty: None,
        taken_lines: 1,
        counts: Counts::default(),
    }
}

/// Streaming an oracle fixture a second time, through a highlighter that
/// already met its states and into buffers with room, allocates nothing
/// (`CODING_STANDARDS.md` 2.1).
#[test]
fn streaming_allocates_nothing_per_line() {
    for (language, text) in oracle_documents() {
        let mut first = Document::new_with_limits(language, theme(), Limits::default());
        stream(&mut first, &text, 3);
        first.finish();
        first.lex(Until::End);
        let sizes = (
            first.text.capacity(),
            first.lines.capacity(),
            first.tokens.capacity(),
            first.tail.tokens.capacity(),
        );
        let mut second = warm(first.highlighter, sizes.0, sizes.1, sizes.2, sizes.3);
        let allocations = count(|| {
            stream(&mut second, &text, 3);
            second.finish();
            second.lex(Until::End);
        });
        assert_eq!(allocations, 0, "{language:?}: {allocations} allocations");
    }
}

/// A line of 100 KiB written four bytes at a time is lexed in bounded steps:
/// past its first KiB, at most nine times its length, and once more whole
/// when the stream ends.
#[test]
fn a_long_line_is_lexed_in_bounded_steps() {
    let mut line = String::from("[");
    while line.len() < 100 * 1024 {
        line.push_str("1, ");
    }
    let mut document = Document::new(Language::Json, theme());
    for piece in line.as_bytes().chunks(4) {
        document.push(piece);
        document.lex(Until::End);
    }
    document.finish();
    document.lex(Until::End);
    let first_kib = 1024 * (1024 / 4 + 1);
    let bound = 9 * line.len() + line.len() + first_kib;
    assert!(
        document.counts.tail_bytes <= bound,
        "{} bytes lexed for a line of {}, over {bound}",
        document.counts.tail_bytes,
        line.len()
    );
}
