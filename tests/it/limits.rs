//! Each limit, reached and not reached, through the public API.

use nishiki::{Highlighter, Language, Limit, Limits, LineReport, State, Token, Work};

/// Lexes `lines` in order and returns each line's tokens and report, and
/// the final state.
fn lex(highlighter: &mut Highlighter, lines: &[&str]) -> (Vec<(Vec<Token>, LineReport)>, State) {
    let mut state = State::default();
    let rows = lines
        .iter()
        .map(|line| {
            let mut tokens = Vec::new();
            let report = highlighter.line(line, &mut state, &mut tokens);
            assert_eq!(
                tokens.last().map_or(0, |token| token.end()),
                line.len(),
                "tokens cover {line:?}"
            );
            (tokens, report)
        })
        .collect();
    (rows, state)
}

fn innermost(highlighter: &Highlighter, token: Token) -> &str {
    highlighter.scopes(token.stack()).next().unwrap_or_default()
}

#[test]
fn a_long_line_is_one_token_and_keeps_the_state() {
    let limits = Limits {
        line_bytes: Some(8),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, state) = lex(&mut highlighter, &["let s = \"open", "short"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::LineBytes));
    assert_eq!(tokens.len(), 1);
    assert_eq!(innermost(&highlighter, tokens[0]), "source.rust");
    assert!(
        rows[1].1.is_complete(),
        "the next line is lexed from the old state"
    );
    assert_eq!(state, State::default());
}

#[test]
fn running_out_of_work_ends_the_line_and_keeps_its_state() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let line = format!("let s = \"{}\"; let t = 1;", r"\n".repeat(200));
    let (rows, _) = lex(&mut highlighter, &[&line, "x"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Work));
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        innermost(&highlighter, last),
        "string.quoted.double.rust",
        "the rest of the line keeps the string's stack"
    );
    assert_eq!(
        innermost(&highlighter, rows[1].0[0]),
        "string.quoted.double.rust",
        "the next line starts inside the string"
    );
}

#[test]
fn contexts_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, state) = lex(&mut highlighter, &["f(f(f(f(f(f(x))))))"]);
    assert!(rows[0].1.reached(Limit::Depth));
    assert_eq!(state, State::default());
}

#[test]
fn depth_can_be_raised_without_bound() {
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, Limits::unlimited());
    let deep = "/*".repeat(10_000);
    let close = "*/".repeat(10_000);
    let (rows, state) = lex(&mut highlighter, &[&deep, &close]);
    assert!(rows.iter().all(|(_, report)| report.is_complete()));
    assert_eq!(innermost(&highlighter, rows[1].0[0]), "comment.block.rust");
    assert_eq!(state, State::default(), "every comment closed");
}

#[test]
fn the_node_limit_falls_back_to_the_context_stack() {
    let limits = Limits {
        nodes: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Rust, limits);
    let (rows, _) = lex(&mut highlighter, &["fn main() { let x = \"s\"; }"]);
    assert!(rows[0].1.reached(Limit::Nodes));
}

#[test]
fn hostile_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Rust);
    for line in [
        "/*".repeat(100_000),
        "f(".repeat(100_000),
        "a::<".repeat(100_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..8],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn hostile_shell_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Shell);
    for line in [
        "$(".repeat(100_000),
        "{ ".repeat(100_000),
        "x=1 ".repeat(100_000),
        "case ".repeat(100_000),
        "alias -a ".repeat(50_000),
        format!("{}x=1", "alias -a ".repeat(50_000)),
        " ".repeat(300_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..4],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_raw_string_takes_any_number_of_hashes() {
    let hashes = "#".repeat(300);
    let line = format!("r{hashes}\"x\"{hashes};");
    let mut highlighter = Highlighter::new(Language::Rust);
    let (rows, state) = lex(&mut highlighter, &[&line]);
    assert!(rows[0].1.is_complete());
    assert_eq!(state, State::default());
}

#[test]
fn hostile_markdown_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    for unit in [
        "**", "[", "[](", "`", "``` a ` ", "~~ a ", "<a ", "_ a ", "![a](", "*a ", "> ", "- ",
        "|a ",
    ] {
        let line = unit.repeat(20_000);
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Work | Limit::Depth)),
            "{unit:?} reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn markdown_containers_past_the_depth_limit_are_not_opened() {
    let limits = Limits {
        depth: Some(4),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let (rows, state) = lex(&mut highlighter, &["> > > > > > deep"]);
    let (tokens, report) = &rows[0];
    assert!(report.reached(Limit::Depth));
    let last = tokens[tokens.len() - 1];
    assert_eq!(
        highlighter
            .scopes(last.stack())
            .filter(|name| *name == "markup.quote.markdown")
            .count(),
        3,
        "the document is the first of four contexts"
    );
    assert_ne!(state, State::default());
}

#[test]
fn a_markdown_fence_runs_its_rust_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let rust = format!("let s = \"{}\"; let t = 1;", r"\n".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```rust", &rust, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn a_markdown_fence_runs_its_typescript_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let ts = format!("let s = \"{}\"; let t = 1;", "x".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```ts", &ts, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn a_markdown_fence_runs_its_shell_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let sh = format!("echo {}", "$(".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```sh", &sh, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_typescript_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::TypeScript);
    for line in [
        "`${".repeat(100_000),
        "${".repeat(100_000),
        "(".repeat(100_000),
        "{[(".repeat(50_000),
        ")}]".repeat(50_000),
        "a<".repeat(100_000),
        "f<(a, ".repeat(50_000),
        "x = (a, b: ".repeat(50_000),
        "/*".repeat(100_000),
        "/**@param{".repeat(50_000),
    ] {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            &line[..8],
            report.limits().collect::<Vec<_>>()
        );
    }
}
