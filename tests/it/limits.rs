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
fn hostile_json_lines_finish_within_the_default_limits() {
    let long_string = format!("\"{}\"", "x".repeat(300_000));
    for language in [Language::Json, Language::Jsonc, Language::Jsonl] {
        let mut highlighter = Highlighter::new(language);
        for line in [
            "[".repeat(100_000),
            "{\"a\":".repeat(100_000),
            "{[".repeat(50_000),
            "]}".repeat(50_000),
            format!("\"{}", "\\n".repeat(100_000)),
            format!("\"{}", "\\u12".repeat(60_000)),
            format!("\"{}", "\\".repeat(100_000)),
            long_string.clone(),
            "/*".repeat(100_000),
            "/**".repeat(100_000),
            "//".repeat(100_000),
            "-".repeat(300_000),
            "1e".repeat(100_000),
            "0.".repeat(100_000),
            "tr".repeat(100_000),
            "\u{a0}".repeat(100_000),
            "x".repeat(300_000),
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
}

#[test]
fn hostile_toml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Toml);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("a = ", "[", 100_000, ""),
        line("a = ", "{", 100_000, ""),
        line("a = ", "{a=", 50_000, ""),
        line("a = ", "[{", 50_000, ""),
        line("", "[", 100_000, ""),
        line("", "[[", 100_000, ""),
        line("", "]", 100_000, ""),
        line("[a", ".a", 100_000, ""),
        line("[", "a.", 100_000, "]"),
        line("", "a.", 100_000, ""),
        line("", "a.", 100_000, "= 1"),
        line("", "a . ", 50_000, "x"),
        line("", "\"a\".", 50_000, "x"),
        line("{", "a.", 100_000, "x"),
        line("a = {", "\"a\".", 50_000, "x"),
        line("a = {", "\"a\".", 50_000, "x y = 1"),
        line("a = {", "a.", 100_000, "x y = 1"),
        line("a = {", "\"a\" = 1,", 30_000, ""),
        line("a = {", "a.b = 1, ", 30_000, ""),
        line("", "\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("", "\"\" ", 100_000, ""),
        line("", "\"a\" ", 60_000, "= 1"),
        line("a = ", "= ", 100_000, ""),
        line("", "=", 300_000, ""),
        line("a = ", "=", 300_000, ""),
        line("", "a ", 100_000, ""),
        line("a = \"", "\\", 100_000, ""),
        line("a = \"", "\\u12", 60_000, ""),
        line("a = \"\"\"", "\"", 100_000, ""),
        line("a = \"\"\"", "\"\"", 100_000, ""),
        line("a = '''", "''", 100_000, ""),
        line("a = \"", "x", 300_000, "\""),
        line("", "#", 300_000, ""),
        line("a = 1 ", "# ", 100_000, ""),
        line("a = ", "1_", 100_000, ""),
        line("a = ", "1e", 100_000, ""),
        line("a = ", "0.", 100_000, ""),
        line("a = ", "1979-05-27T", 30_000, ""),
        line("a = ", "07:32:00.", 30_000, ""),
        line("a = ", "0x", 100_000, ""),
        line("a = [", "1 ", 100_000, ""),
        line("a = [", ",", 100_000, ""),
        line("a = [", "x ", 100_000, ""),
        line("", "x", 300_000, ""),
        line("", "\u{a0}", 100_000, ""),
        line("a = 1", "\u{a0}", 100_000, "x"),
        line("", "\u{3000}", 100_000, "[a]"),
    ];
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..4.min(line.len())],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn hostile_make_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Make);
    for line in [
        "$(".repeat(100_000),
        "${".repeat(100_000),
        "$($".repeat(100_000),
        "$(a ".repeat(100_000),
        "$(shell ".repeat(50_000),
        "$(origin ".repeat(50_000),
        format!("$(origin {}", "a".repeat(300_000)),
        format!("$(origin {} ", " ".repeat(300_000)),
        format!("$(MAKE{}", " ".repeat(300_000)),
        "$(a,(".repeat(50_000),
        "$(call (".repeat(50_000),
        "$(call {".repeat(50_000),
        "\\\\".repeat(100_000),
        "#".repeat(300_000),
        " #".repeat(100_000),
        "a:".repeat(100_000),
        "a: ".repeat(100_000),
        "a:=".repeat(100_000),
        format!("{} = x", "a ".repeat(100_000)),
        format!("{} = x", "a+".repeat(100_000)),
        format!("\t{}", "@-+".repeat(100_000)),
        format!("define {}", "x ".repeat(100_000)),
        format!("else {}", "ifeq".repeat(75_000)),
        format!("else {} x", " ".repeat(300_000)),
        "\u{a0}".repeat(100_000),
        "$".repeat(300_000),
        "x".repeat(300_000),
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
fn a_markdown_fence_runs_its_makefile_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let make = format!("x = {}", "$(a) ".repeat(100));
    let (rows, _) = lex(&mut highlighter, &["```makefile", &make, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_diff_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Diff);
    for line in [
        "@@ ".repeat(100_000),
        format!("@@{}", " ".repeat(300_000)),
        format!("@@ {}", "@ ".repeat(100_000)),
        "@@\u{a0}".repeat(100_000),
        format!("==== {}", " - ".repeat(100_000)),
        format!("{} ====", " - ".repeat(100_000)),
        format!("{}====", " ".repeat(300_000)),
        format!("diff {}", "a ".repeat(100_000)),
        format!("diff {}", "a".repeat(300_000)),
        format!("index {}", "a".repeat(300_000)),
        format!("Only in {}", ":".repeat(300_000)),
        "1,".repeat(100_000),
        "\u{663}".repeat(100_000),
        format!("*** {} ****", "x".repeat(300_000)),
        "-".repeat(300_000),
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
fn a_markdown_fence_runs_its_diff_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let diff = format!("@@ {} @@", "x".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```diff", &diff, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_dockerfile_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Dockerfile);
    for line in [
        "\"".repeat(300_000),
        "'".repeat(300_000),
        "\\".repeat(150_000),
        "\\\"".repeat(100_000),
        "\"a\\'".repeat(60_000),
        format!("FROM {}", "a ".repeat(100_000)),
        format!("FROM {}", "a".repeat(300_000)),
        format!("FROM {}as", "a ".repeat(100_000)),
        format!("FROM a {}", "AS".repeat(100_000)),
        format!("ONBUILD{}x", " ".repeat(300_000)),
        format!("{}RUN", " ".repeat(300_000)),
        format!("{}x", "\u{a0}".repeat(100_000)),
        format!("{}#", "\t".repeat(300_000)),
        format!("RUN {}", "\u{17f}".repeat(100_000)),
        "x".repeat(300_000),
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
fn a_markdown_fence_runs_its_dockerfile_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let dockerfile = format!("RUN {}", "\"a\" ".repeat(100));
    let (rows, _) = lex(&mut highlighter, &["```dockerfile", &dockerfile, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn a_markdown_fence_runs_its_json_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let json = format!("[{}]", "1,".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```json", &json, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_python_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Python);
    let lines = [
        "[".repeat(100_000),
        "(".repeat(100_000),
        "{".repeat(100_000),
        "x[".repeat(50_000),
        "f(".repeat(50_000),
        "a.".repeat(100_000),
        "lambda ".repeat(50_000),
        "class ".repeat(50_000),
        "def ".repeat(50_000),
        "@".repeat(100_000),
        format!("@d(x){}x", " ".repeat(100_000)),
        " ".repeat(300_000),
        format!("x{}", " ".repeat(300_000)),
        format!("\"{}", "{".repeat(100_000)),
        format!("\"{}", "{ ".repeat(50_000)),
        format!("\"{}", "{{}".repeat(50_000)),
        format!("\"{}", "{%".repeat(50_000)),
        format!("\"{}", "%(".repeat(50_000)),
        format!("\"{}", "\\N{{".repeat(50_000)),
        format!("f\"{}", "{".repeat(100_000)),
        format!("f\"{}", "{:".repeat(50_000)),
        format!("f\"{}\"", "{:{".repeat(50_000)),
        format!("f\"{}", "{x!r:>".repeat(20_000)),
        format!("\"{}", "{:0".repeat(50_000)),
        format!("\"{}", "\\".repeat(100_000)),
        format!("\"\"\"{}", "\\\"".repeat(50_000)),
        format!("r\"{}", "(?".repeat(50_000)),
        format!("r\"{}", "[".repeat(100_000)),
        format!("r\"{}", "(?P<a ".repeat(30_000)),
        format!("r\"{}", "(?(".repeat(30_000)),
        format!("r\"{}", "{1,".repeat(50_000)),
        format!("r\"{}", "\\1".repeat(50_000)),
        "#".repeat(300_000),
        "# type: ".repeat(30_000),
        "1_".repeat(100_000),
        "0x".repeat(100_000),
        "1e".repeat(100_000),
        "0.".repeat(100_000),
        "x".repeat(300_000),
        "_A".repeat(100_000),
        "\\".repeat(100_000),
        "if ".repeat(60_000),
        "yield from ".repeat(30_000),
        "#!".repeat(100_000),
    ];
    for line in lines {
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
fn a_markdown_fence_runs_its_python_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let py = format!("x = [{}]", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```py", &py, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
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

#[test]
fn hostile_yaml_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Yaml);
    for line in [
        "[".repeat(100_000),
        "{a: ".repeat(50_000),
        "{? ".repeat(50_000),
        "? : ".repeat(50_000),
        "&a ".repeat(100_000),
        "!t ".repeat(100_000),
        "- ".repeat(100_000),
        "a: ".repeat(100_000),
        format!("{}\t", "# ".repeat(100_000)),
        "\"\\".repeat(100_000),
        "x ".repeat(150_000),
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
fn a_markdown_fence_runs_its_yaml_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let yaml = format!("a: {}", "&b ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```yaml", &yaml, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_jsx_lines_finish_within_the_default_limits() {
    for language in [Language::JavaScript, Language::Jsx, Language::Tsx] {
        let mut highlighter = Highlighter::new(language);
        for line in [
            "x = <a>".repeat(50_000),
            "<a><b>".repeat(50_000),
            "x = <a b={".repeat(30_000),
            "x = <a b=\"".repeat(30_000),
            "x = <a b='".repeat(30_000),
            "</a>".repeat(100_000),
            "(<".repeat(100_000),
            "x = <a:b.c-d ".repeat(30_000),
            "<>{<>".repeat(50_000),
            "<a {...b} ".repeat(30_000),
            "&amp".repeat(100_000),
            "x = <T extends ".repeat(30_000),
        ] {
            let (rows, _) = lex(&mut highlighter, &[&line]);
            let report = rows[0].1;
            assert!(
                report
                    .limits()
                    .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
                "{language:?} {:?} reached {:?}",
                &line[..8],
                report.limits().collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn hostile_java_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Java);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "{", 100_000, ""),
        line("", "(", 100_000, ""),
        line("", "[", 100_000, ""),
        line("", "<", 100_000, ""),
        line("", "a<", 100_000, ""),
        line("", "A<", 100_000, ">"),
        line("", "}", 100_000, ""),
        line("", ">", 100_000, ""),
        line("", "new ", 50_000, ""),
        line("", "@", 100_000, ""),
        line("", "@A(", 50_000, ""),
        line("", "@A ", 50_000, ""),
        line("", "class ", 50_000, ""),
        line("", "enum ", 50_000, ""),
        line("", "record ", 40_000, ""),
        line("", "public ", 40_000, "class A"),
        line("", "interface ", 30_000, ""),
        line("", "import ", 40_000, ""),
        line("", "package ", 40_000, ""),
        line("", "int ", 50_000, ""),
        line("", "int ", 50_000, "x;"),
        line("", "A ", 100_000, "x;"),
        line("", "A a, ", 50_000, ";"),
        line("", "int[] ", 50_000, "x;"),
        line("", "int x = 1; ", 20_000, ""),
        line("", "A.", 100_000, "x"),
        line("", "a.", 100_000, "b y;"),
        line("", "a.b ", 50_000, ""),
        line("", "A.B<", 50_000, ""),
        line("", "x(", 100_000, ""),
        line("", "x()", 100_000, ""),
        line("", ".x()", 100_000, ""),
        line("", "a b ", 50_000, "("),
        line("", "void m(", 50_000, ""),
        line("", "int a, ", 50_000, ") {"),
        line("", "throws ", 50_000, ""),
        line("", "extends ", 50_000, ""),
        line("", "implements A, ", 30_000, ""),
        line("", "\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("", "\"\"\"", 100_000, ""),
        line("\"", "\\", 100_000, ""),
        line("\"", "\\\"", 100_000, ""),
        line("\"\"\"", "\\\"\"\"", 50_000, ""),
        line("", "/*", 100_000, ""),
        line("", "/**", 100_000, ""),
        line("", "//", 100_000, ""),
        line("/** ", "{@link A#b(", 30_000, ""),
        line("/** ", "@param ", 40_000, ""),
        line("", "0x", 100_000, ""),
        line("", "0x1p", 50_000, ""),
        line("", "1e", 100_000, ""),
        line("", "1_", 100_000, ""),
        line("", "1.", 100_000, ""),
        line("", ".5", 100_000, ""),
        line("", "1f", 100_000, ""),
        line("", "x", 300_000, ""),
        line("", "$", 300_000, ""),
        line("", "_a", 100_000, ""),
        line("", "->", 100_000, ""),
        line("", "<<", 100_000, "="),
        line("", "?", 300_000, ""),
        line("", ":", 300_000, ""),
        line("", " ", 300_000, ""),
        line("x", " ", 300_000, "y"),
        line("", "\t", 300_000, "//"),
        line("", "\u{a0}", 100_000, ""),
        line("", "日", 100_000, ""),
        line("", "try ", 50_000, ""),
        line("", "catch (", 50_000, ""),
        line("", "for (int i = 0; ", 30_000, ""),
        line("", "case ", 50_000, ""),
        line("", "assert ", 50_000, ""),
        line("", "enum E { A(", 30_000, ""),
        line("", "module m { ", 30_000, ""),
        line("", "open module ", 30_000, ""),
        line("", "static ", 50_000, ""),
        line("", "final ", 50_000, "int a;"),
        line("", "var ", 50_000, "x = 1;"),
        line("", "a=", 100_000, ""),
        line("", "(A) ", 50_000, ""),
        line("", "A::", 100_000, ""),
        line("", "A[]::new ", 30_000, ""),
        line("", "? extends ", 30_000, ""),
        line("", "& ", 100_000, ""),
    ];
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report.limits().all(|limit| limit == Limit::Depth),
            "{:?} reached {:?}",
            &line[..6.min(line.len())],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_markdown_fence_runs_its_java_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let java = format!("f({});", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```java", &java, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}

#[test]
fn hostile_c_lines_finish_within_the_default_limits() {
    let lines = [
        "(".repeat(100_000),
        "[".repeat(100_000),
        "{".repeat(100_000),
        "x[".repeat(50_000),
        "f(".repeat(50_000),
        "a.".repeat(100_000),
        "a->".repeat(60_000),
        "a.b.c(".repeat(40_000),
        "a .b ".repeat(40_000),
        "/*".repeat(100_000),
        "//".repeat(100_000),
        "/**".repeat(60_000),
        "/** @a ".repeat(30_000),
        "\"".repeat(100_000),
        "'".repeat(100_000),
        "\\".repeat(100_000),
        format!("\"{}", "\\".repeat(100_000)),
        format!("\"{}", "%1$".repeat(50_000)),
        format!("\"{}", "%-5.5ld".repeat(30_000)),
        format!("\"{}", "%0000000000".repeat(20_000)),
        "#define ".repeat(50_000),
        "#define X(a,".repeat(30_000),
        "#if defined(".repeat(30_000),
        "#error \"".repeat(30_000),
        "#include <".repeat(40_000),
        "0x".repeat(100_000),
        "1e".repeat(100_000),
        "1'".repeat(100_000),
        "0x1.p".repeat(40_000),
        "...".repeat(60_000),
        " ".repeat(300_000),
        format!("x{}", " ".repeat(300_000)),
        format!("{}x", " ".repeat(300_000)),
        "x".repeat(300_000),
        "x or(".repeat(40_000),
        "switch(".repeat(40_000),
        "switch ".repeat(40_000),
        "case ".repeat(60_000),
        "case 1:".repeat(40_000),
        "asm(".repeat(50_000),
        "asm volatile ".repeat(30_000),
        "?".repeat(100_000),
        "? : ".repeat(40_000),
        "int (".repeat(40_000),
        "foo(bar(".repeat(30_000),
        "\u{a0}".repeat(100_000),
        "/*\u{a0}".repeat(60_000),
    ];
    let mut highlighter = Highlighter::new(Language::C);
    for line in lines {
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
fn a_markdown_fence_runs_its_c_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let c = format!("int x[] = {{{}}};", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```c", &c, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
