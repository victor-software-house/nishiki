use indoc::indoc;

use crate::{Highlighter, Language, Limit, Limits, State};

/// Lexes `text` and returns, per line, each token's text with its scope stack.
fn lex(text: &str) -> Vec<Vec<(String, Vec<String>)>> {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut lines = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "tokens must cover the line: {line:?}");
        lines.push(row);
    }
    lines
}

/// The innermost scope of the first token of `text` that reads `token`.
fn scope_of(text: &str, token: &str) -> String {
    let lines = lex(text);
    let found = lines.iter().flatten().find(|(t, _)| t == token);
    let (_, scopes) = found.unwrap_or_else(|| panic!("no token {token:?} in {lines:?}"));
    scopes.last().cloned().unwrap_or_default()
}

/// The full scope stack of the first token of `text` that reads `token`.
fn stack_of(text: &str, token: &str) -> Vec<String> {
    let lines = lex(text);
    let found = lines.iter().flatten().find(|(t, _)| t == token);
    let (_, scopes) = found.unwrap_or_else(|| panic!("no token {token:?} in {lines:?}"));
    scopes.clone()
}

#[test]
fn empty_line_passes_over_the_state() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("> quote", &mut state, &mut tokens);
    let before = state;
    tokens.clear();
    let _ = highlighter.line("", &mut state, &mut tokens);
    assert_eq!(tokens, []);
    assert_eq!(state, before);
}

#[test]
fn a_line_of_spaces_is_lexed() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("   ", &mut state, &mut tokens);
    assert_eq!(tokens.len(), 1);
}

#[test]
fn emphasis_and_code() {
    let text = "a *b* **c** `d` ~~e~~";
    assert_eq!(scope_of(text, "b"), "markup.italic.markdown");
    assert_eq!(scope_of(text, "c"), "markup.bold.markdown");
    assert_eq!(scope_of(text, "d"), "markup.inline.raw.string.markdown");
    assert_eq!(scope_of(text, "e"), "markup.strikethrough.markdown");
}

#[test]
fn a_delimiter_with_no_closer_is_text() {
    assert_eq!(scope_of("a *b", "a *b"), "meta.paragraph.markdown");
    assert_eq!(scope_of("**b **", "**b **"), "meta.paragraph.markdown");
}

#[test]
fn heading_with_closing_hashes() {
    let text = "## title ##";
    assert_eq!(scope_of(text, "title"), "entity.name.section.markdown");
    let tokens = lex(text);
    assert_eq!(tokens[0].last().map(|(t, _)| t.as_str()), Some("##"));
    assert_eq!(
        tokens[0]
            .last()
            .and_then(|(_, s)| s.last())
            .map(String::as_str),
        Some("punctuation.definition.heading.markdown")
    );
}

#[test]
fn a_heading_needs_a_blank_after_the_hashes() {
    assert_eq!(scope_of("#x", "#x"), "meta.paragraph.markdown");
    assert_eq!(
        scope_of("####### x", "####### x"),
        "meta.paragraph.markdown"
    );
}

#[test]
fn links_and_images() {
    let text = r#"[t](http://u "ti") ![a](b) <https://x.y> [r][s]"#;
    assert_eq!(scope_of(text, "t"), "string.other.link.title.markdown");
    assert_eq!(scope_of(text, "http://u"), "markup.underline.link.markdown");
    assert_eq!(
        scope_of(text, "ti"),
        "string.other.link.description.title.markdown"
    );
    assert_eq!(scope_of(text, "b"), "markup.underline.link.image.markdown");
    assert_eq!(
        scope_of(text, "https://x.y"),
        "markup.underline.link.markdown"
    );
    assert_eq!(
        scope_of(text, "s"),
        "constant.other.reference.link.markdown"
    );
}

#[test]
fn rust_fence_embeds_the_rust_lexer_without_its_root() {
    let text = indoc! {"
        ```rust
        let x = 1;
        ```
    "};
    assert_eq!(
        stack_of(text, "let"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.rust",
            "keyword.other.rust",
            "storage.type.rust",
        ]
    );
    assert_eq!(
        stack_of(text, "rust"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "fenced_code.block.language.markdown",
        ]
    );
}

#[test]
fn rust_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```rust
        /* a
        b */
        ```
    "};
    assert_eq!(scope_of(text, "*/"), "comment.block.rust",);
}

#[test]
fn typescript_fence_embeds_the_typescript_lexer_without_its_root() {
    for info in ["ts", "typescript", "TS"] {
        let text = format!("```{info}\nconst x = 1;\n```\n");
        assert_eq!(
            stack_of(&text, "const"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.typescript",
                "meta.var.expr.ts",
                "storage.type.ts",
            ],
            "{info}"
        );
    }
}

#[test]
fn typescript_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```ts
        /* a
        b */
        ```
    "};
    assert_eq!(stack_of(text, "b ")[3], "comment.block.ts");
}

#[test]
fn a_shell_fence_embeds_the_shell_lexer_without_its_root() {
    for info in [
        "sh",
        "bash",
        "shell",
        "zsh",
        "BASH",
        "bash_profile",
        "{.bash .x}",
    ] {
        let text = format!("```{info}\nls -l\n```\n");
        assert_eq!(
            stack_of(&text, "ls"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.shellscript",
                "meta.statement.shell",
                "meta.statement.command.shell",
                "meta.statement.command.name.shell",
                "entity.name.function.call.shell",
                "entity.name.command.shell",
            ],
            "{info}"
        );
    }
}

#[test]
fn shell_text_after_a_containers_indent_is_not_at_the_start_of_the_line() {
    let text = indoc! {"
        - a

          ```sh
          brew update
          ```
    "};
    let plain = lex(text)
        .iter()
        .flatten()
        .filter(|(text, _)| text.contains("brew"))
        .all(|(_, scopes)| {
            scopes
                .last()
                .is_some_and(|s| s == "meta.embedded.block.shellscript")
        });
    assert!(plain, "Shiki leaves the command unscoped here");
}

#[test]
fn shell_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```sh
        echo 'a
        b'
        ```
    "};
    assert_eq!(stack_of(text, "b")[3], "meta.statement.shell");
}

#[test]
fn the_line_after_a_continuation_in_a_fence_is_anchored_at_its_start() {
    // The fence's `while` rule matches the line's start, so `\G` is there and
    // the shell grammar reads the whole line as a command name. A shell
    // document does not do this.
    let text = indoc! {"
        ```sh
        a \\
          b
        ```
    "};
    let lines = lex(text);
    assert_eq!(lines[2].len(), 1, "one token: {:?}", lines[2]);
    assert_eq!(lines[2][0].0, "  b");
    assert_eq!(
        lines[2][0].1.last().map(String::as_str),
        Some("meta.statement.command.name.shell")
    );
}

#[test]
fn a_shebang_is_not_special_inside_a_fence() {
    let text = indoc! {"
        ```ts
        #!/usr/bin/env node
        ```
    "};
    let shebang = lex(text)
        .iter()
        .flatten()
        .any(|(_, scopes)| scopes.iter().any(|s| s == "comment.line.shebang.ts"));
    assert!(!shebang, "only the first line of the document can be one");
}

#[test]
fn another_language_is_plain_text_with_its_info_string() {
    let text = indoc! {"
        ```python
        # not a heading
        ```
    "};
    assert_eq!(scope_of(text, "python"), "fenced_code.block.language");
    assert_eq!(
        stack_of(text, "# not a heading"),
        ["text.html.markdown", "markup.fenced_code.block.markdown"]
    );
}

#[test]
fn a_longer_fence_is_not_closed_by_a_shorter_one() {
    let text = indoc! {"
        ````
        ```
        still inside
        ````
        outside
    "};
    assert_eq!(
        stack_of(text, "still inside"),
        ["text.html.markdown", "markup.fenced_code.block.markdown"]
    );
    assert_eq!(scope_of(text, "outside"), "meta.paragraph.markdown");
}

#[test]
fn a_markdown_fence_embeds_markdown() {
    let text = indoc! {"
        ````md
        # inner
        ```rust
        fn f() {}
        ```
        ````
    "};
    assert_eq!(
        stack_of(text, "inner"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.markdown",
            "markup.heading.markdown",
            "heading.1.markdown",
            "entity.name.section.markdown",
        ]
    );
}

#[test]
fn a_list_continues_over_blank_lines_and_indented_lines() {
    let text = indoc! {"
        - a

          b
        c
    "};
    assert_eq!(
        stack_of(text, "b"),
        [
            "text.html.markdown",
            "markup.list.unnumbered.markdown",
            "meta.paragraph.markdown",
        ]
    );
    assert_eq!(
        stack_of(text, "c"),
        ["text.html.markdown", "meta.paragraph.markdown"]
    );
}

#[test]
fn a_quote_ends_at_a_line_without_the_marker() {
    let text = indoc! {"
        > a
        b
    "};
    assert_eq!(
        stack_of(text, "a"),
        [
            "text.html.markdown",
            "markup.quote.markdown",
            "meta.paragraph.markdown"
        ]
    );
    assert_eq!(
        stack_of(text, "b"),
        ["text.html.markdown", "meta.paragraph.markdown"]
    );
}

#[test]
fn setext_underline_follows_its_paragraph() {
    let text = indoc! {"
        Title
        =====
    "};
    assert_eq!(scope_of(text, "====="), "markup.heading.setext.1.markdown");
}

#[test]
fn table_cells_hold_inline_markup() {
    let text = "| a | *b* |";
    assert_eq!(scope_of(text, "b"), "markup.italic.markdown");
    assert_eq!(scope_of(text, "|"), "punctuation.definition.table.markdown");
}

#[test]
fn html_comment_spans_lines() {
    let text = indoc! {"
        <!-- a
        *b*
        -->
        *c*
    "};
    assert_eq!(scope_of(text, "*b*"), "comment.block.html");
    assert_eq!(scope_of(text, "c"), "markup.italic.markdown");
}

#[test]
fn unicode_whitespace_is_whitespace() {
    // `**x<NBSP>**` cannot close: `\S` must precede the closing `**`.
    assert_eq!(
        scope_of("**x\u{a0}**", "**x\u{a0}**"),
        "meta.paragraph.markdown"
    );
}

#[test]
fn nesting_stops_at_the_depth_limit() {
    let limits = Limits {
        depth: Some(8),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let text = format!("{}deep", "> ".repeat(100));
    let report = highlighter.line(&text, &mut state, &mut tokens);
    assert!(report.reached(Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(text.len()));
    let quotes = highlighter
        .scopes(tokens[tokens.len() - 1].stack())
        .filter(|name| *name == "markup.quote.markdown")
        .count();
    assert_eq!(quotes, 7, "the document frame is the eighth");
}

#[test]
fn hostile_lines_are_covered() {
    for unit in [
        "**", "[", "[](", "`", "``` a ` ", "~~ a ", "<a ", "_ a ", "![a](", "*a ",
    ] {
        let line = unit.repeat(20_000);
        let mut highlighter = Highlighter::new(Language::Markdown);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let report = highlighter.line(&line, &mut state, &mut tokens);
        assert!(
            report.limits().all(|limit| limit == Limit::Work),
            "{unit:?} reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
        assert_eq!(tokens.last().map(|t| t.end()), Some(line.len()), "{unit:?}");
    }
}

#[test]
fn the_state_resumes_from_any_line() {
    let text = indoc! {"
        - item
          ```rust
          fn a() {
              /* c
          "};
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut states = Vec::new();
    let mut tokens = Vec::new();
    let lines: Vec<&str> = text.split('\n').collect();
    for line in &lines {
        let _ = highlighter.line(line, &mut state, &mut tokens);
        states.push(state);
    }
    // Resume from the state after line 1: the same highlighter, which owns
    // the state, reaches the same state again.
    let mut resumed = states[1];
    let mut second = Vec::new();
    for line in &lines[2..] {
        let _ = highlighter.line(line, &mut resumed, &mut second);
    }
    assert_eq!(resumed, state);
}
