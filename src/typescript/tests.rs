use indoc::indoc;

use crate::{Highlighter, Language, Limits, State};

/// The tokens of one line: each one's text and its full scope stack.
type Row = Vec<(String, Vec<String>)>;

fn highlighter() -> Highlighter {
    Highlighter::new(Language::TypeScript)
}

/// Open contexts after a line, not counting the document's own and the marker
/// that a non-empty first line leaves under everything it opens.
fn depth(h: &Highlighter, state: State) -> u32 {
    h.frame(state).depth.saturating_sub(2)
}

/// Lexes `text` line by line and returns, per line, each token's text and its
/// full scope stack, with the state after each line.
fn lex(text: &str) -> (Vec<Row>, Vec<State>) {
    lex_with(&mut highlighter(), text)
}

fn lex_with(h: &mut Highlighter, text: &str) -> (Vec<Row>, Vec<State>) {
    let mut state = State::default();
    let (mut lines, mut states) = (Vec::new(), Vec::new());
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let _ = h.line(line, &mut state, &mut tokens);
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let end = token.end();
            let mut scopes: Vec<String> = h.scopes(token.stack()).map(str::to_owned).collect();
            scopes.reverse();
            row.push((line[start..end].to_string(), scopes));
            start = end;
        }
        assert_eq!(start, line.len(), "tokens must cover the line: {line:?}");
        lines.push(row);
        states.push(state);
    }
    (lines, states)
}

/// The full scope stack of the first token whose text is `token`.
fn stack_of(text: &str, token: &str) -> Vec<String> {
    let (lines, _) = lex(text);
    lines
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?}"))
        .1
}

/// The innermost scope of the first token whose text is `token`.
fn scope_of(text: &str, token: &str) -> String {
    stack_of(text, token).last().cloned().unwrap_or_default()
}

#[test]
fn keywords_and_names() {
    assert_eq!(scope_of("const x = 1;", "const"), "storage.type.ts");
    assert_eq!(scope_of("const x = 1;", "x"), "variable.other.constant.ts");
    assert_eq!(scope_of("const x = 1;", "1"), "constant.numeric.decimal.ts");
    assert_eq!(scope_of("let y;", "y"), "variable.other.readwrite.ts");
    assert_eq!(scope_of("foo(1);", "foo"), "entity.name.function.ts");
}

#[test]
fn arrow_function_assigned_to_a_name_is_a_function() {
    assert_eq!(
        scope_of("const f = (a: number) => a;", "f"),
        "entity.name.function.ts"
    );
    assert_eq!(
        scope_of("const o = { run: async () => 1 };", "run"),
        "entity.name.function.ts"
    );
    assert_eq!(
        scope_of("this.cb = function () {};", "cb"),
        "entity.name.function.ts"
    );
    assert_eq!(
        scope_of("const v = { run: 1 };", "run"),
        "meta.object-literal.key.ts"
    );
}

#[test]
fn template_literals_nest() {
    let text = "`a ${b + `c ${d}`} e`";
    assert_eq!(
        stack_of(text, "d"),
        [
            "source.ts",
            "string.template.ts",
            "meta.template.expression.ts",
            "meta.embedded.line.ts",
            "string.template.ts",
            "meta.template.expression.ts",
            "meta.embedded.line.ts",
            "variable.other.readwrite.ts"
        ]
    );
    assert_eq!(
        stack_of(text, " e"),
        ["source.ts", "string.template.ts"],
        "the outer template resumes after the inner one"
    );
}

#[test]
fn state_resumes_across_lines() {
    let text = indoc! {"
        /* open
        still */ let a
        `x
        ${y}` + z
    "};
    let mut h = highlighter();
    let (rows, states) = lex_with(&mut h, text);
    assert_eq!(depth(&h, states[0]), 1, "a block comment is open");
    assert_eq!(rows[1][0].1.last().unwrap(), "comment.block.ts");
    assert_eq!(depth(&h, states[1]), 0, "the comment closed");
    assert_eq!(depth(&h, states[2]), 1, "a template is open");
    assert_eq!(depth(&h, states[3]), 0, "the template closed");
}

#[test]
fn doc_comment_tags() {
    let text = "/** @param {string} name the name */";
    assert_eq!(scope_of(text, "param"), "storage.type.class.jsdoc");
    assert_eq!(scope_of(text, "name"), "variable.other.jsdoc");
    assert_eq!(
        scope_of("/** {@link Foo} */", "Foo"),
        "variable.other.description.jsdoc"
    );
}

#[test]
fn an_empty_line_leaves_the_state_alone() {
    let mut highlighter = highlighter();
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("type A = B & {", &mut state, &mut tokens);
    let open = state;
    tokens.clear();
    let _ = highlighter.line("", &mut state, &mut tokens);
    assert_eq!(tokens, []);
    assert_eq!(state, open);
}

#[test]
fn blank_line_ends_a_type_alias_only_when_not_empty() {
    assert_eq!(
        stack_of("type A = B\n  \n/** doc */", " doc ")[1],
        "comment.block.documentation.ts",
        "a line of blanks ends the alias"
    );
    assert_eq!(
        stack_of("type A = B\n\n/** doc */", " doc ")[1],
        "meta.type.declaration.ts",
        "an empty line is skipped, so the alias is still open"
    );
}

#[test]
fn shebang_only_on_the_first_line() {
    assert_eq!(
        scope_of("#!/usr/bin/env node", "/usr/bin/env node"),
        "comment.line.shebang.ts"
    );
    assert_ne!(
        scope_of("a\n#!/usr/bin/env node", "#"),
        "punctuation.definition.comment.ts"
    );
}

#[test]
fn constants_of_number_have_no_scope() {
    // Shiki's grammar numbers the capture of these names inside a lookahead,
    // so they get no scope of their own.
    assert_eq!(
        stack_of("x = Number.MAX_VALUE;", "MAX_VALUE"),
        ["source.ts"]
    );
    assert_eq!(
        scope_of("x = a.length;", "length"),
        "support.variable.property.ts"
    );
}

#[test]
fn nesting_past_the_limit_is_plain_text() {
    let limits = Limits {
        depth: Some(8),
        ..Limits::default()
    };
    let mut h = Highlighter::new_with_limits(Language::TypeScript, limits);
    let deep = format!("{}x{}", "(".repeat(40), ")".repeat(40));
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = h.line(&deep, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Depth));
    assert!(h.frame(state).depth <= 8);
    assert_eq!(tokens.last().map(|token| token.end()), Some(deep.len()));
}

#[test]
fn a_lookahead_is_bounded_by_the_work_limit() {
    // Every `(` asks whether an arrow function follows, and the answer needs
    // the rest of the line, so a small work limit ends the line early.
    let limits = Limits {
        work: Some(crate::Work {
            per_byte: 1,
            base: 0,
        }),
        ..Limits::default()
    };
    let mut h = Highlighter::new_with_limits(Language::TypeScript, limits);
    let line = format!("f = {}", "(a, ".repeat(2000));
    let mut state = State::default();
    let mut tokens = Vec::new();
    let report = h.line(&line, &mut state, &mut tokens);
    assert!(report.reached(crate::Limit::Work));
    assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
}

#[test]
fn long_blanks_are_linear() {
    let line = format!("{}x", " ".repeat(200_000));
    let (rows, _) = lex(&line);
    assert_eq!(rows[0].last().unwrap().0, "x");
}

#[test]
fn a_rule_that_consumes_the_newline_still_lets_an_end_fire_at_the_line_end() {
    // `as` takes the blanks after it, which include the newline, and the
    // object member that holds it ends at that line end all the same.
    let text = indoc! {"
        x = {
          a = b as
          | C
        }
    "};
    assert_eq!(
        stack_of(text, "C"),
        [
            "source.ts",
            "meta.objectliteral.ts",
            "meta.object.member.ts",
            "variable.other.readwrite.ts"
        ]
    );
    assert_eq!(
        stack_of(text, "  | "),
        ["source.ts", "meta.objectliteral.ts"],
        "the member and the type after `as` are closed"
    );
}

#[test]
fn a_method_without_a_body_leaves_the_next_method_a_name() {
    let text = indoc! {"
        x = {
          @tag({ a: 1 })
          async run(url: string) {}
        }
    "};
    assert_eq!(
        stack_of(text, "async"),
        [
            "source.ts",
            "meta.objectliteral.ts",
            "meta.method.declaration.ts",
            "storage.modifier.async.ts"
        ]
    );
}

#[test]
fn doc_comment_optional_parameter_needs_its_closing_bracket() {
    let closed = "/** @param {string} [name=5] n */";
    assert_eq!(
        scope_of(closed, "["),
        "punctuation.definition.optional-value.begin.bracket.square.jsdoc"
    );
    assert_eq!(scope_of(closed, "5"), "source.embedded.ts");
    let open = "/** @param {string} [name n */";
    assert_eq!(
        stack_of(open, "["),
        ["source.ts", "comment.block.documentation.ts"],
        "without a `]` the bracket is plain text"
    );
}

/// Lines over every printable ASCII byte and the grammar's keywords, so that
/// each gated rule meets every byte in many contexts. A debug build fails when
/// a rule matches at a byte outside the set it is gated on.
#[test]
fn gated_rules_match_only_inside_their_start_sets() {
    const WORDS: &[&str] = &[
        "const",
        "let",
        "var",
        "function",
        "class",
        "interface",
        "enum",
        "type",
        "import",
        "export",
        "declare",
        "namespace",
        "module",
        "async",
        "await",
        "yield",
        "new",
        "typeof",
        "as",
        "satisfies",
        "extends",
        "implements",
        "if",
        "else",
        "for",
        "while",
        "switch",
        "case",
        "return",
        "this",
        "super",
        "static",
        "public",
        "private",
        "readonly",
        "abstract",
        "keyof",
        "infer",
        "in",
        "of",
        "void",
        "null",
        "true",
        "undefined",
        "=>",
        "...",
        "?.",
        "??",
        "${",
        "/**",
        "*/",
        "/*",
        "//",
        "@param",
        "{@link",
        "<T>",
        "<",
        ">",
        "(",
        ")",
        "{",
        "}",
        "[",
        "]",
        "\"",
        "'",
        "`",
        "/",
        "\\",
        "é",
        "日",
        "$",
        "_",
        "0x",
        "1e",
        ".",
        ",",
        ";",
        ":",
        "=",
        "!",
        "?",
        "|",
        "&",
        "+",
        "-",
        "*",
        "%",
        "~",
        "^",
        "#",
        "@",
        " ",
        " ",
        "  ",
        "\t",
    ];
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..400 {
        let mut h = highlighter();
        let mut state = State::default();
        let mut out = Vec::new();
        for _ in 0..40 {
            let mut line = String::new();
            for _ in 0..next() % 30 {
                if next() % 5 == 0 {
                    line.push(char::from(32 + u8::try_from(next() % 95).unwrap_or(0)));
                } else {
                    let word = usize::try_from(next()).unwrap_or(0) % WORDS.len();
                    line.push_str(WORDS[word]);
                }
            }
            out.clear();
            let _ = h.line(&line, &mut state, &mut out);
        }
    }
}

/// The full scope stack of the first token that reads `token`, in `language`.
fn stack_in(language: Language, text: &str, token: &str) -> Vec<String> {
    let (lines, _) = lex_with(&mut Highlighter::new(language), text);
    lines
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?}"))
        .1
}

#[test]
fn the_dialects_differ_in_the_suffix_of_their_names() {
    for (language, suffix) in [
        (Language::TypeScript, "ts"),
        (Language::JavaScript, "js"),
        (Language::Jsx, "js.jsx"),
        (Language::Tsx, "tsx"),
    ] {
        assert_eq!(
            stack_in(language, "const x = 1;", "const"),
            [
                format!("source.{suffix}"),
                format!("meta.var.expr.{suffix}"),
                format!("storage.type.{suffix}"),
            ],
            "{language:?}"
        );
    }
}

#[test]
fn a_tag_is_jsx_after_an_operator_in_the_jsx_dialects() {
    let text = "const a = <div id=\"x\">hi &amp; {y}</div>;";
    for language in [Language::JavaScript, Language::Jsx, Language::Tsx] {
        let stack = stack_in(language, text, "div");
        assert!(
            stack.iter().any(|scope| scope.starts_with("meta.tag")),
            "{language:?}: {stack:?}"
        );
    }
    let stack = stack_in(Language::TypeScript, "const a = <div>x;", "div");
    assert!(
        !stack.iter().any(|scope| scope.starts_with("meta.tag")),
        "typescript has no JSX: {stack:?}"
    );
}

#[test]
fn a_component_name_has_its_own_scope() {
    let stack = stack_in(Language::Tsx, "x = <Foo.Bar a=\"1\" />;", "Foo.Bar");
    assert_eq!(
        stack[stack.len() - 2..],
        ["entity.name.tag.tsx", "support.class.component.tsx"]
    );
    let stack = stack_in(Language::Tsx, "x = <a b=\"1\" />;", "a");
    assert_eq!(
        stack.last().map(String::as_str),
        Some("entity.name.tag.tsx")
    );
}

#[test]
fn a_type_parameter_list_with_extends_is_an_arrow_function_in_tsx() {
    let text = "const f = <T extends object>(x: T) => x;";
    assert_eq!(
        scope_in(Language::Tsx, text, "f"),
        "entity.name.function.tsx"
    );
}

/// The innermost scope of the first token that reads `token`, in `language`.
fn scope_in(language: Language, text: &str, token: &str) -> String {
    stack_in(language, text, token).pop().unwrap_or_default()
}

#[test]
fn jsx_state_continues_across_lines() {
    let text = "x = (\n  <div\n    a=\"1\"\n  >\n    text\n  </div>\n);";
    let mut h = Highlighter::new(Language::Jsx);
    let (rows, states) = lex_with(&mut h, text);
    assert!(depth(&h, states[1]) > 0, "the tag is open");
    assert!(
        depth(&h, states[5]) == depth(&h, states[0]),
        "the tag closed"
    );
    assert!(
        rows[4]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|s| s == "meta.jsx.children.js.jsx"))
    );
}

#[test]
fn a_type_annotation_takes_the_blanks_before_a_brace() {
    // The end of a type annotation before `{` takes the blanks, so the brace is
    // an object literal and not the start of a destructured parameter, in the
    // parentheses that the first line leaves open.
    let text = indoc! {"
        c() {
          return (
            false
        }
        u(n: number): void {
          this.x()
        }
    "};
    let (rows, _) = lex(text);
    let stack = rows
        .into_iter()
        .flatten()
        .find(|(token, _)| token.contains("this.x()"))
        .map(|(_, stack)| stack)
        .unwrap_or_default();
    assert!(
        stack.iter().any(|s| s == "meta.objectliteral.ts"),
        "{stack:?}"
    );
    assert!(
        !stack
            .iter()
            .any(|s| s == "meta.parameter.object-binding-pattern.ts"),
        "{stack:?}"
    );
}
