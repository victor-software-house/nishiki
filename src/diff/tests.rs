use crate::{Highlighter, Language, Limits, State, Work};

/// One lexed line: each token's text and its innermost scope name.
type Row = Vec<(String, String)>;

fn lex_line(highlighter: &mut Highlighter, line: &str) -> Row {
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = highlighter.line(line, &mut state, &mut tokens);
    assert!(report.is_complete(), "{line:?} reached a limit");
    let mut start = 0;
    let mut row = Vec::new();
    for token in &tokens {
        let innermost = highlighter
            .scopes(token.stack())
            .next()
            .unwrap_or_default()
            .to_owned();
        row.push((line[start..token.end()].to_owned(), innermost));
        start = token.end();
    }
    assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
    row
}

fn lex(line: &str) -> Row {
    lex_line(&mut Highlighter::new(Language::Diff), line)
}

/// A row from `(text, innermost scope)` pairs.
fn row(parts: &[(&str, &str)]) -> Row {
    parts
        .iter()
        .map(|&(text, scope)| (text.to_owned(), scope.to_owned()))
        .collect()
}

#[test]
fn an_empty_line_has_no_tokens() {
    assert_eq!(lex(""), Row::new());
}

#[test]
fn a_unified_hunk_header_splits_at_its_captures() {
    assert_eq!(
        lex("@@ -1,2 +3,4 @@ fn main()"),
        row(&[
            ("@@", "punctuation.definition.range.diff"),
            (" ", "meta.diff.range.unified"),
            ("-1,2 +3,4", "meta.toc-list.line-number.diff"),
            (" ", "meta.diff.range.unified"),
            ("@@", "punctuation.definition.range.diff"),
            (" fn main()", "source.diff"),
        ])
    );
}

#[test]
fn a_hunk_header_gives_back_one_space_when_only_spaces_lie_between() {
    assert_eq!(
        lex("@@ @@"),
        row(&[
            ("@@", "punctuation.definition.range.diff"),
            (" ", "meta.toc-list.line-number.diff"),
            ("@@", "punctuation.definition.range.diff"),
        ])
    );
    assert_eq!(lex("@@@@"), row(&[("@@@@", "source.diff")]));
}

#[test]
fn a_hunk_header_reads_non_ascii_white_space_as_white_space() {
    assert_eq!(
        lex("@@\u{a0}-1\u{a0}@@"),
        row(&[
            ("@@", "punctuation.definition.range.diff"),
            ("\u{a0}", "meta.diff.range.unified"),
            ("-1", "meta.toc-list.line-number.diff"),
            ("\u{a0}", "meta.diff.range.unified"),
            ("@@", "punctuation.definition.range.diff"),
        ])
    );
}

#[test]
fn a_normal_diff_range_covers_the_line() {
    for line in ["3c3", "1,2d0", "12a13,15", "1,2,3c4"] {
        assert_eq!(
            lex(line),
            row(&[(line, "meta.diff.range.normal")]),
            "{line}"
        );
    }
    for line in ["3x3", "3c", "c3", "1,c3", "3c3 "] {
        assert_eq!(lex(line), row(&[(line, "source.diff")]), "{line}");
    }
    assert_eq!(
        lex("\u{663}c\u{663}"),
        row(&[("\u{663}c\u{663}", "meta.diff.range.normal")]),
        "an Arabic-Indic digit is a digit"
    );
}

#[test]
fn a_context_diff_has_its_own_headers() {
    assert_eq!(
        lex("*** 1,4 ****"),
        row(&[
            ("***", "punctuation.definition.range.diff"),
            (" 1,4 ", "meta.diff.range.context"),
            ("****", "punctuation.definition.range.diff"),
        ])
    );
    assert_eq!(
        lex("--- 1,4 ----"),
        row(&[
            ("---", "punctuation.definition.range.diff"),
            (" 1,4 ", "meta.diff.range.context"),
            ("----", "punctuation.definition.range.diff"),
        ])
    );
    assert_eq!(
        lex("*** file1.txt\tThu Jan 11"),
        row(&[
            ("***", "punctuation.definition.from-file.diff"),
            (" file1.txt\tThu Jan 11", "meta.diff.header.from-file"),
        ])
    );
    assert_eq!(
        lex("***************"),
        row(&[("***************", "punctuation.definition.separator.diff")])
    );
}

#[test]
fn a_lone_three_dashes_is_a_separator_and_an_inserted_line_may_be_empty() {
    assert_eq!(
        lex("---"),
        row(&[("---", "punctuation.definition.separator.diff")])
    );
    assert_eq!(
        lex("+++"),
        row(&[
            ("+", "punctuation.definition.inserted.diff"),
            ("++", "markup.inserted.diff")
        ])
    );
    assert_eq!(
        lex("+++ b/x"),
        row(&[
            ("+++", "punctuation.definition.to-file.diff"),
            (" b/x", "meta.diff.header.to-file"),
        ])
    );
}

#[test]
fn only_a_lone_angle_bracket_or_one_before_a_space_marks_a_line() {
    assert_eq!(lex(">x"), row(&[(">x", "source.diff")]));
    assert_eq!(
        lex("> x"),
        row(&[
            (">", "punctuation.definition.inserted.diff"),
            (" x", "markup.inserted.diff"),
        ])
    );
    assert_eq!(lex("<x"), row(&[("<x", "source.diff")]));
}

#[test]
fn an_equals_header_ends_before_the_last_dash_between_spaces() {
    assert_eq!(
        lex("==== a - b - c"),
        row(&[
            ("====", "punctuation.definition.from-file.diff"),
            (" a - b", "meta.diff.header.from-file"),
            (" - c", "source.diff"),
        ])
    );
}

#[test]
fn the_to_file_rule_matches_after_other_text_when_the_line_ends_in_equals() {
    assert_eq!(
        lex("x - y ===="),
        row(&[
            ("x", "source.diff"),
            (" ", "meta.diff.header.to-file"),
            ("-", "punctuation.definition.to-file.diff"),
            (" y ", "meta.diff.header.to-file"),
            ("====", "punctuation.definition.to-file.diff"),
        ])
    );
    assert_eq!(
        lex("==== a - b - c ===="),
        row(&[
            ("====", "punctuation.definition.from-file.diff"),
            (" a - b", "meta.diff.header.from-file"),
            (" ", "meta.diff.header.to-file"),
            ("-", "punctuation.definition.to-file.diff"),
            (" c ", "meta.diff.header.to-file"),
            ("====", "punctuation.definition.to-file.diff"),
        ])
    );
}

#[test]
fn a_command_header_needs_two_words_or_a_dash() {
    assert_eq!(
        lex("diff -r a b"),
        row(&[("diff -r a b", "meta.diff.header.command")])
    );
    assert_eq!(
        lex("diff a\u{a0}b"),
        row(&[("diff a\u{a0}b", "meta.diff.header.command")])
    );
    assert_eq!(lex("diff ab"), row(&[("diff ab", "source.diff")]));
    assert_eq!(lex("diff  a b"), row(&[("diff  a b", "source.diff")]));
    assert_eq!(
        lex("diff --git a/x b/x"),
        row(&[("diff --git a/x b/x", "meta.diff.header.git")])
    );
}

#[test]
fn an_index_line_wants_seven_to_forty_hex_digits_on_the_left() {
    assert_eq!(
        lex("index 1234567..abcdefa 100644"),
        row(&[("index 1234567..abcdefa 100644", "meta.diff.index.git")])
    );
    assert_eq!(
        lex("index 123456..abcdefa"),
        row(&[("index 123456..abcdefa", "source.diff")])
    );
    let long = "a".repeat(41);
    let line = format!("index {long}..abcdefa");
    assert_eq!(lex(&line), row(&[(&line, "source.diff")]));
    let line = format!("index abcdefa..{long}");
    assert_eq!(lex(&line), row(&[(&line, "meta.diff.index.git")]));
}

#[test]
fn an_index_header_and_an_only_in_line() {
    assert_eq!(
        lex("Index: src/a.c"),
        row(&[
            ("Index", "meta.diff.index"),
            (":", "punctuation.separator.key-value.diff"),
            (" ", "meta.diff.index"),
            ("src/a.c", "meta.toc-list.file-name.diff"),
        ])
    );
    assert_eq!(lex("Index: "), row(&[("Index: ", "source.diff")]));
    assert_eq!(
        lex("Only in a: b"),
        row(&[("Only in a: b", "meta.diff.only-in")])
    );
    assert_eq!(lex("Only in a:b"), row(&[("Only in a:b", "source.diff")]));
}

#[test]
fn a_comment_runs_to_the_end_of_the_line_and_the_next_line_starts_afresh() {
    let mut highlighter = Highlighter::new(Language::Diff);
    assert_eq!(
        lex_line(&mut highlighter, "# note"),
        row(&[
            ("#", "punctuation.definition.comment.diff"),
            (" note", "comment.line.number-sign.diff"),
        ])
    );
    assert_eq!(
        lex_line(&mut highlighter, "text"),
        row(&[("text", "source.diff")])
    );
}

#[test]
fn a_markdown_fence_embeds_diff_without_its_source_scope() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    for line in ["```diff", "- old"] {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, &mut tokens);
    }
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        highlighter.scopes(last.stack()).collect::<Vec<_>>(),
        [
            "markup.deleted.diff",
            "meta.embedded.block.diff",
            "markup.fenced_code.block.markdown",
            "text.html.markdown"
        ]
    );
}

#[test]
fn a_fence_by_any_of_the_grammars_names_embeds_diff() {
    for info in ["diff", "patch", "rej", "DIFF"] {
        let mut highlighter = Highlighter::new(Language::Markdown);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let _ = highlighter.line(&format!("```{info}"), &mut state, &mut tokens);
        tokens.clear();
        let _ = highlighter.line("+ new", &mut state, &mut tokens);
        let first = tokens[0];
        assert_eq!(
            highlighter.scopes(first.stack()).next(),
            Some("punctuation.definition.inserted.diff"),
            "{info}"
        );
    }
}

#[test]
fn a_fence_inside_a_quote_gets_no_anchored_rule() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    for line in ["> ```diff", "> + new"] {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, &mut tokens);
    }
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        highlighter.scopes(last.stack()).next(),
        Some("meta.embedded.block.diff"),
        "`^` matches only at the start of the line"
    );
}

#[test]
fn running_out_of_work_leaves_the_rest_of_the_line_plain() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 4,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Diff, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = format!("@@ {} @@", "x".repeat(200));
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Work));
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].end(), line.len());
}
