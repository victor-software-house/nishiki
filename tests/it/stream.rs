//! Streams every oracle fixture into a `Document` and compares each line with
//! Shiki's records (openspec streaming, the `streaming` spec).
//!
//! The fixtures are Shiki's output for the whole text, so a document whose
//! final lines match them matches Shiki however its deltas were cut.

use nishiki::{Document, Language, LineKind, Tail, Theme, ThemeSource, Until};

use crate::oracle::{Line, Tokens, compare, every_language, fixtures, parse, report};

/// How a test cuts a fixture's text into deltas.
#[derive(Clone, Copy, Debug)]
enum Cut {
    /// Runs of word characters, runs of blanks and single punctuation
    /// characters, at most four characters each, as a model's tokens come.
    Tokens,
    /// One to seven bytes at a time, from a seeded generator, so many deltas
    /// end inside a character.
    Bytes(u64),
    /// The whole text at once, lexed through the middle line first.
    Whole,
    /// Token-sized deltas, with the last line lexed again only once it has
    /// doubled, so most deltas leave it lexed in part.
    Deferred,
}

/// What kind of character `c` is, for cutting token-sized deltas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    Word,
    Blank,
    Other,
}

fn class(c: char) -> Class {
    match c {
        c if c.is_alphanumeric() || c == '_' => Class::Word,
        c if c.is_whitespace() => Class::Blank,
        _ => Class::Other,
    }
}

/// The byte ends of `text`'s token-sized deltas.
fn token_ends(text: &str) -> Vec<usize> {
    let mut ends = Vec::new();
    let mut run = 0;
    let mut last = None;
    for (at, c) in text.char_indices() {
        let now = class(c);
        if at > 0 && (Some(now) != last || now == Class::Other || run == 4) {
            ends.push(at);
            run = 0;
        }
        run += 1;
        last = Some(now);
    }
    ends.push(text.len());
    ends
}

/// The byte ends of deltas of one to seven bytes, from `seed`.
fn byte_ends(len: usize, seed: u64) -> Vec<usize> {
    let mut state = seed | 1;
    let mut ends = Vec::new();
    let mut at = 0;
    while at < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let step = usize::try_from(state % 7).unwrap_or(0) + 1;
        at = (at + step).min(len);
        ends.push(at);
    }
    ends
}

/// `text` streamed into a document of `language` as `cut` says, finished and
/// lexed to its end.
fn streamed(language: Language, text: &str, cut: Cut) -> Document {
    let mut document = Document::new(language, Theme::new(ThemeSource::default()));
    let bytes = text.as_bytes();
    let ends = match cut {
        Cut::Tokens => token_ends(text),
        Cut::Deferred => {
            document.set_tail(Tail::Amortized {
                short: 0,
                growth: 1,
            });
            token_ends(text)
        }
        Cut::Bytes(seed) => byte_ends(bytes.len(), seed),
        Cut::Whole => {
            document.push_str(text);
            document.lex(Until::Line(document.line_count() / 2));
            Vec::new()
        }
    };
    let mut from = 0;
    for end in ends {
        document.push(&bytes[from..end]);
        document.lex(Until::End);
        from = end;
    }
    document.finish();
    document.lex(Until::End);
    document
}

/// The fixtures of `language` whose streamed lines differ from Shiki's, as
/// reports, under every cut.
fn stream_failures(language: Language) -> Vec<String> {
    let mut failures = Vec::new();
    for (number, path) in fixtures(language.id()).into_iter().enumerate() {
        let lines: Vec<Line> = parse(&path);
        let text = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let seed = u64::try_from(number).unwrap_or(0) + 0x9e37_79b9;
        for cut in [Cut::Tokens, Cut::Bytes(seed), Cut::Whole, Cut::Deferred] {
            let document = streamed(language, &text, cut);
            assert_eq!(
                document.text(),
                text,
                "{}: {cut:?} lost text",
                path.display()
            );
            assert_eq!(
                document.line_count(),
                lines.len(),
                "{}: {cut:?}",
                path.display()
            );
            let mut index = 0;
            let (checked, misses) = compare(&lines, |_| {
                let line = document.line(index);
                index += 1;
                tokens_of(&document, line)
            });
            if !misses.is_empty() {
                failures.push(format!("{cut:?}: {}", report(&path, checked, &misses)));
            }
            for index in 0..document.line_count() {
                let line = document.line(index);
                let kind = line.map(|line| line.kind());
                assert_eq!(
                    kind,
                    Some(LineKind::Exact),
                    "{}: {cut:?} line {}",
                    path.display(),
                    index + 1
                );
            }
        }
    }
    failures
}

/// A line's tokens as the oracle comparison reads them.
fn tokens_of(document: &Document, line: Option<nishiki::DocumentLine<'_>>) -> Tokens {
    let Some(line) = line else {
        return Vec::new();
    };
    assert!(
        line.report().is_complete(),
        "{:?} reached a limit",
        line.text()
    );
    line.tokens()
        .iter()
        .map(|token| {
            let mut scopes: Vec<&str> = document.highlighter().scopes(token.stack()).collect();
            scopes.reverse();
            (token.end(), scopes.join(" "))
        })
        .collect()
}

#[test]
fn every_language_streams_as_shiki_lexes() {
    every_language(stream_failures);
}

/// Bytes that are not UTF-8, cut anywhere, read as `String::from_utf8_lossy`
/// reads them whole.
#[test]
fn bytes_that_are_not_utf8_become_replacement_characters() {
    let bytes: &[u8] = b"ok \xe2\x82 \xf0\x9f\x98\x80 \xff\xfe x\xc3\xa9\xed\xa0\x80 end\xe2\x82";
    let whole = String::from_utf8_lossy(bytes);
    for seed in 1..64 {
        let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
        let mut from = 0;
        for end in byte_ends(bytes.len(), seed) {
            document.push(&bytes[from..end]);
            from = end;
        }
        document.finish();
        assert_eq!(document.text(), whole, "seed {seed}");
    }
}

/// Each delta changes only the lines it touched, and the changes list those
/// lines.
#[test]
fn changes_hold_every_line_a_delta_touched() {
    let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
    document.push_str("let a = 1;\nlet b");
    document.lex(Until::End);
    let first = document.take_changes();
    assert_eq!(
        first.map(|changes| (changes.start, changes.old_end, changes.new_end)),
        Some((0, 1, 2))
    );
    assert_eq!(document.take_changes(), None);
    document.push_str(indoc::indoc! {"
         = 2;
        let c = 3;
    "});
    document.lex(Until::End);
    let second = document.take_changes();
    assert_eq!(
        second.map(|changes| (changes.start, changes.old_end, changes.new_end)),
        Some((1, 2, 4))
    );
}

/// A `\r` at the end of a delta waits for the next one to show whether it
/// begins a line ending.
#[test]
fn a_carriage_return_before_a_line_feed_ends_the_line() {
    let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
    document.push_str("let a = 1;\r");
    document.lex(Until::End);
    assert_eq!(document.line(0).map(|line| line.text()), Some("let a = 1;"));
    document.push_str("\nlet b = 2;");
    document.lex(Until::End);
    assert_eq!(document.line(0).map(|line| line.text()), Some("let a = 1;"));
    assert_eq!(document.line(1).map(|line| line.text()), Some("let b = 2;"));
}

/// A seeded generator of small numbers, for the edit tests.
struct Rng(u64);

impl Rng {
    fn below(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        usize::try_from(self.0 % u64::try_from(bound.max(1)).unwrap_or(1)).unwrap_or(0)
    }
}

/// A random character boundary of `text`.
fn boundary(text: &str, rng: &mut Rng) -> usize {
    let mut at = rng.below(text.len() + 1);
    while !text.is_char_boundary(at) {
        at -= 1;
    }
    at
}

/// A random edit of `text`: a range of up to 40 bytes and a piece of `source`
/// of up to 40 bytes to put there.
fn random_edit(text: &str, source: &str, rng: &mut Rng) -> (std::ops::Range<usize>, String) {
    let start = boundary(text, rng);
    let mut end = (start + rng.below(41)).min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let from = boundary(source, rng);
    let mut to = (from + rng.below(41)).min(source.len());
    while !source.is_char_boundary(to) {
        to -= 1;
    }
    (start..end, source[from..to].to_owned())
}

/// Each line of `text` lexed from the start by a fresh highlighter, as the
/// oracle comparison reads tokens.
fn one_shot(language: Language, text: &str) -> Vec<Tokens> {
    let mut highlighter = nishiki::Highlighter::new(language);
    let mut state = nishiki::State::default();
    let mut tokens = Vec::new();
    text.split('\n')
        .map(|line| {
            tokens.clear();
            let _ = highlighter.line(line, &mut state, &mut tokens);
            tokens
                .iter()
                .map(|token| {
                    let mut scopes: Vec<&str> = highlighter.scopes(token.stack()).collect();
                    scopes.reverse();
                    (token.end(), scopes.join(" "))
                })
                .collect()
        })
        .collect()
}

#[expect(
    clippy::expect_used,
    reason = "a test helper; a refused edit fails the test"
)]
/// The fixtures of `language` that differ after edits, as reports.
/// Twelve random edits land on each fixture, lexed after some of them, so
/// some edits merge before a lex.
/// The result must equal lexing its text from the start.
/// Undoing the edits, last first, must give Shiki's scopes again.
fn edit_failures(language: Language) -> Vec<String> {
    let mut failures = Vec::new();
    for (number, path) in fixtures(language.id()).into_iter().enumerate() {
        let lines: Vec<Line> = parse(&path);
        let original = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let mut rng = Rng(u64::try_from(number).unwrap_or(0) * 7919 + 0x5851_f42d);
        let mut document = streamed(language, &original, Cut::Whole);
        let mut text = original.clone();
        let mut undo = Vec::new();
        for edit in 0..12 {
            let (range, piece) = random_edit(&text, &original, &mut rng);
            let old = text[range.clone()].to_owned();
            text.replace_range(range.clone(), &piece);
            document
                .replace(range.clone(), &piece)
                .expect("a range on character boundaries");
            undo.push((range.start..range.start + piece.len(), old));
            match edit % 3 {
                0 => document.lex(Until::End),
                1 => document.lex(Until::Line(rng.below(document.line_count()))),
                _ => {}
            }
        }
        document.lex(Until::End);
        assert_eq!(document.text(), text, "{}", path.display());
        let expected = one_shot(language, &text);
        let actual: Vec<Tokens> = (0..document.line_count())
            .map(|index| tokens_of(&document, document.line(index)))
            .collect();
        if let Some(line) = expected.iter().zip(&actual).position(|(a, b)| a != b) {
            failures.push(format!(
                "{}: after the edits, line {} differs from lexing the text from the start",
                path.display(),
                line + 1
            ));
            continue;
        }
        for (range, old) in undo.into_iter().rev() {
            document
                .replace(range, &old)
                .expect("an undo on character boundaries");
            if rng.below(2) == 0 {
                document.lex(Until::End);
            }
        }
        document.lex(Until::End);
        assert_eq!(document.text(), original, "{}", path.display());
        let mut index = 0;
        let (checked, misses) = compare(&lines, |_| {
            let line = document.line(index);
            index += 1;
            tokens_of(&document, line)
        });
        if !misses.is_empty() {
            failures.push(format!("undone edits: {}", report(&path, checked, &misses)));
        }
    }
    failures
}

#[test]
fn every_language_edits_as_shiki_lexes() {
    every_language(edit_failures);
}

/// A range outside the text, or inside a character, is refused and changes
/// nothing.
#[test]
fn a_bad_range_leaves_the_document_unchanged() {
    let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
    document.push_str("let é = 1;");
    document.lex(Until::End);
    assert_eq!(
        document.replace(3..20, "x"),
        Err(nishiki::EditError::OutOfBounds)
    );
    let reversed = std::ops::Range { start: 5, end: 4 };
    assert_eq!(
        document.replace(reversed, "x"),
        Err(nishiki::EditError::OutOfBounds)
    );
    assert_eq!(
        document.replace(5..6, "x"),
        Err(nishiki::EditError::NotCharBoundary)
    );
    assert_eq!(document.text(), "let é = 1;");
}

/// A reset at a small node limit lexes the document again from a fresh
/// highlighter, and gives the tokens a fresh highlighter gives.
#[test]
fn a_reset_lexes_again_from_a_fresh_highlighter() {
    let limits = nishiki::Limits {
        nodes: Some(64),
        ..nishiki::Limits::default()
    };
    let text = (0..40)
        .map(|n| format!("let v{n} = \"{n}\";"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut document =
        Document::new_with_limits(Language::Rust, Theme::new(ThemeSource::default()), limits);
    document.push_str(&text);
    document.finish();
    document.lex(Until::End);
    document.reset();
    assert_eq!(
        document.line(0).map(|line| line.kind()),
        Some(LineKind::Stale)
    );
    document.lex(Until::End);
    let mut fresh = nishiki::Highlighter::new_with_limits(Language::Rust, limits);
    let mut state = nishiki::State::default();
    let mut tokens = Vec::new();
    for (index, line) in text.split('\n').enumerate() {
        tokens.clear();
        let _ = fresh.line(line, &mut state, &mut tokens);
        let ours = document.line(index).map(|line| line.tokens().len());
        assert_eq!(ours, Some(tokens.len()), "line {}", index + 1);
    }
}

/// A push after `finish` opens the document again, even when it adds no
/// text yet: an empty delta, or the first byte of a character.
#[test]
fn a_push_after_finish_opens_the_document_again() {
    let theme = || Theme::new(ThemeSource::default());
    let deltas: [&[u8]; 2] = [b"", b"\xe2"];
    for delta in deltas {
        let mut document = Document::new(Language::Rust, theme());
        document.push_str("let a");
        document.finish();
        document.lex(Until::End);
        assert_eq!(
            document.line(0).map(|line| line.kind()),
            Some(LineKind::Exact)
        );
        let _ = document.take_changes();
        document.push(delta);
        document.lex(Until::End);
        assert!(
            matches!(
                document.line(0).map(|line| line.kind()),
                Some(LineKind::Provisional { .. })
            ),
            "after {delta:?}"
        );
        assert_eq!(
            document
                .take_changes()
                .map(|changes| (changes.start, changes.new_end)),
            Some((0, 1)),
            "after {delta:?}"
        );
    }
}

/// An edit to a line that a delta completed, before any lex, lexes that
/// line again from its new text.
/// The last line's tokens from before the delta name that line, and the
/// edit must not let them stand in for it.
#[test]
fn an_edit_to_a_line_just_completed_lexes_it_again() {
    let cases = [
        ("x\nabcdefghij", 3..10, ""),
        ("x\nab", 2..4, "\"\""),
        ("x\nab", 3..3, "cd"),
    ];
    for (text, range, replacement) in cases {
        let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
        document.push_str(text);
        document.lex(Until::End);
        document.push_str("\n");
        assert_eq!(document.replace(range.clone(), replacement), Ok(()));
        document.finish();
        document.lex(Until::End);
        let expected = one_shot(Language::Rust, document.text());
        for (index, want) in expected.iter().enumerate() {
            assert_eq!(
                &tokens_of(&document, document.line(index)),
                want,
                "{text:?} with {range:?} replaced by {replacement:?}, line {}",
                index + 1
            );
        }
    }
}

/// A lex that defers the last line still reports it changed, since the line
/// turns from stale to provisional.
#[test]
fn a_deferred_lex_reports_the_last_line_changed() {
    let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
    document.set_tail(Tail::Amortized {
        short: 4,
        growth: 2,
    });
    document.push_str("let value");
    document.lex(Until::End);
    let _ = document.take_changes();
    document.push_str("s");
    assert!(document.take_changes().is_some());
    assert_eq!(
        document.line(0).map(|line| line.kind()),
        Some(LineKind::Stale)
    );
    document.lex(Until::End);
    assert!(matches!(
        document.line(0).map(|line| line.kind()),
        Some(LineKind::Provisional { .. })
    ));
    assert_eq!(
        document
            .take_changes()
            .map(|changes| (changes.start, changes.new_end)),
        Some((0, 1))
    );
}

#[expect(
    clippy::expect_used,
    reason = "a test helper; a missing theme fails the test"
)]
/// Every theme of the oracle, sorted by file name.
#[cfg(feature = "json")]
fn oracle_themes() -> Vec<Theme> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("oracle/themes");
    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .expect("the themes directory exists")
        .map(|entry| entry.expect("a readable directory entry").path())
        .collect();
    paths.sort();
    paths
        .iter()
        .map(|path| {
            let json = std::fs::read_to_string(path).expect("a readable theme");
            Theme::from_json(&json).expect("a theme Shiki loads")
        })
        .collect()
}

/// The lines of `document` whose styles differ from what `theme` gives each
/// token's scopes, as reports.
#[cfg(feature = "json")]
fn style_misses(document: &Document, theme: &Theme, label: &str) -> Vec<String> {
    let mut misses = Vec::new();
    for index in 0..document.line_count() {
        let Some(line) = document.line(index) else {
            continue;
        };
        let want: Vec<(usize, nishiki::Style)> = line
            .tokens()
            .iter()
            .map(|token| {
                let scopes = document.highlighter().scopes(token.stack());
                (token.end(), theme.style(scopes))
            })
            .collect();
        let have: Vec<(usize, nishiki::Style)> = line.styled().collect();
        if have != want {
            misses.push(format!("{label}: line {}", index + 1));
        }
    }
    misses
}

/// The fixtures of `language` whose styled tokens differ from `Theme::style`.
/// Each streams in token-sized deltas under the first oracle theme.
/// The theme switches to the second at the middle delta.
/// After the stream ends, `set_theme` switches to the third.
#[cfg(feature = "json")]
fn styled_failures(language: Language) -> Vec<String> {
    let themes = oracle_themes();
    let [first, second, third, ..] = themes.as_slice() else {
        return vec!["the oracle has fewer than three themes".to_owned()];
    };
    let mut failures = Vec::new();
    for path in fixtures(language.id()) {
        let lines: Vec<Line> = parse(&path);
        let text = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let mut document = Document::new(language, first.clone());
        let bytes = text.as_bytes();
        let ends = token_ends(&text);
        let middle = ends.len() / 2;
        let mut from = 0;
        for (step, end) in ends.into_iter().enumerate() {
            document.push(&bytes[from..end]);
            document.lex(Until::End);
            if step == middle {
                document.set_theme(second.clone());
            }
            from = end;
        }
        document.finish();
        document.lex(Until::End);
        let label = path.display().to_string();
        failures.extend(style_misses(
            &document,
            second,
            &format!("{label}, switched mid-stream"),
        ));
        document.set_theme(third.clone());
        failures.extend(style_misses(
            &document,
            third,
            &format!("{label}, after set_theme"),
        ));
    }
    failures
}

#[cfg(feature = "json")]
#[test]
fn every_language_streams_styled_as_the_theme_styles() {
    every_language(styled_failures);
}
