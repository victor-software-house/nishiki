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

/// A document built on a highlighter that already met every state of
/// `text`, and styled its stacks under `theme`, with room for everything.
fn warm(
    highlighter: Highlighter,
    theme: Theme,
    text: usize,
    lines: usize,
    tokens: usize,
    tail: usize,
) -> Document {
    Document {
        highlighter,
        theme,
        text: String::with_capacity(text),
        carry: Carry::default(),
        lines: Vec::with_capacity(lines),
        tokens: Vec::with_capacity(tokens),
        garbage: 0,
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

/// Streaming an oracle fixture a second time allocates nothing
/// (`CODING_STANDARDS.md` 2.1).
///
/// The highlighter already met its states and styled its stacks under the
/// same theme, and the buffers have room.
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
        let mut second = warm(
            first.highlighter,
            first.theme,
            sizes.0,
            sizes.1,
            sizes.2,
            sizes.3,
        );
        let allocations = count(|| {
            stream(&mut second, &text, 3);
            second.finish();
            second.lex(Until::End);
        });
        assert_eq!(allocations, 0, "{language:?}: {allocations} allocations");
    }
}

/// A line of 100 KiB written four bytes at a time is lexed in bounded steps.
///
/// Past its first KiB, it costs at most nine times its length.
/// It is lexed once more whole when the stream ends.
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

/// One edit trace of `bench/traces/edits/`: its language, its first text and
/// its replacements.
struct Trace {
    language: Language,
    base: String,
    edits: Vec<(std::ops::Range<usize>, String)>,
}

/// Reads an edit trace (`bench/traces/README.md` has the format).
fn trace(path: &std::path::Path) -> Trace {
    let bytes = std::fs::read(path).expect("a readable trace");
    let mut at = 0;
    let mut line = || {
        let end = at
            + bytes[at..]
                .iter()
                .position(|&b| b == b'\n')
                .expect("a line");
        let text = std::str::from_utf8(&bytes[at..end])
            .expect("UTF-8")
            .to_owned();
        at = end + 1;
        text
    };
    let mut language = None;
    let base_len = loop {
        let row = line();
        let (key, value) = row.split_once(' ').expect("a key and a value");
        match key {
            "language" => language = Language::from_name(value),
            "base" => break value.parse::<usize>().expect("a length"),
            _ => {}
        }
    };
    let blob = |at: &mut usize, len: usize| {
        let text = std::str::from_utf8(&bytes[*at..*at + len])
            .expect("UTF-8")
            .to_owned();
        *at += len + 1;
        text
    };
    let base = blob(&mut at, base_len);
    let mut edits = Vec::new();
    while at < bytes.len() {
        let end = at
            + bytes[at..]
                .iter()
                .position(|&b| b == b'\n')
                .expect("a line");
        let row = std::str::from_utf8(&bytes[at..end])
            .expect("UTF-8")
            .to_owned();
        at = end + 1;
        let fields: Vec<usize> = row
            .split(' ')
            .skip(1)
            .map(|field| field.parse().expect("a number"))
            .collect();
        let text = blob(&mut at, fields[2]);
        edits.push((fields[0]..fields[1], text));
    }
    Trace {
        language: language.expect("a language nishiki lexes"),
        base,
        edits,
    }
}

/// Replaying nishiki's own history, each edit lexes its own lines and at most
/// two lines past them.
///
/// The research note's probe found the same with Shiki's states
/// (`openspec/research/2026-11-05-streaming.md`, section 3).
#[test]
fn an_edit_stops_where_the_states_agree() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("bench/traces/edits");
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("the edit traces")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    paths.sort();
    let mut pasts = Vec::new();
    for path in paths {
        let trace = trace(&path);
        let mut document = Document::new(trace.language, theme());
        document.push_str(&trace.base);
        document.finish();
        document.lex(Until::End);
        for (range, text) in &trace.edits {
            document
                .replace(range.clone(), text)
                .expect("a trace's range");
            let region = document.lines[document.lexed..]
                .iter()
                .take_while(|line| line.from.is_none())
                .count();
            let before = document.counts.completed;
            document.lex(Until::End);
            let lexed = document.counts.completed - before;
            pasts.push((lexed.saturating_sub(region), path.display().to_string()));
        }
    }
    let worst = pasts.iter().max().expect("some edits");
    assert!(
        worst.0 <= 2,
        "an edit in {} lexed {} lines past it",
        worst.1,
        worst.0
    );
}

/// After a lex, every token's style is in the highlighter's cache, the last
/// line's included, so `DocumentLine::styled` never styles from scopes.
#[test]
fn a_lex_keeps_every_token_style() {
    let mut document = Document::new(Language::Python, theme());
    let text = indoc::indoc! {"
        def f(x):
            return x + 1
        print(f"};
    stream(&mut document, text, 3);
    for index in 0..document.line_count() {
        let line = document.line(index).expect("a line");
        for token in line.tokens() {
            assert!(
                document
                    .highlighter
                    .cached_style(&document.theme, token.stack())
                    .is_some(),
                "line {} has a token with no kept style",
                index + 1
            );
        }
    }
}
