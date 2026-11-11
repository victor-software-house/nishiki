use indoc::indoc;

use crate::arena::{Arena, ROOT};
use crate::language::Halt;
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
        stack_of(
            indoc! {"
            type A = B
            \x20\x20
            /** doc */"},
            " doc "
        )[1],
        "comment.block.documentation.ts",
        "a line of blanks ends the alias"
    );
    assert_eq!(
        stack_of(
            indoc! {"
            type A = B

            /** doc */"},
            " doc "
        )[1],
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
    // Four steps a byte cover one pass over the blanks and the lexer's own
    // count of the line.
    // A scan that read them again at each blank would not fit.
    let limits = Limits {
        work: Some(crate::Work {
            per_byte: 4,
            base: 0,
        }),
        ..Limits::default()
    };
    let mut h = Highlighter::new_with_limits(Language::TypeScript, limits);
    let line = format!("{}x", " ".repeat(200_000));
    let (rows, _) = lex_with(&mut h, &line);
    assert_eq!(rows[0].last().map(|(text, _)| text.as_str()), Some("x"));
    let mut state = State::default();
    let report = h.line(&line, &mut state, &mut Vec::new());
    assert!(!report.reached(crate::Limit::Work));
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
/// each gated rule meets every byte in many contexts.
/// A debug build fails when a rule matches at a byte outside its start set.
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
    let dialects = [
        Language::TypeScript,
        Language::JavaScript,
        Language::Jsx,
        Language::Tsx,
    ];
    for language in dialects.into_iter().cycle().take(400) {
        let mut h = Highlighter::new(language);
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
    let text = indoc! {"
        x = (
          <div
            a=\"1\"
          >
            text
          </div>
        );"};
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
    // The end of a type annotation before `{` takes the blanks.
    // So the brace is an object literal and not the start of a destructured
    // parameter, in the parentheses that the first line leaves open.
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

#[test]
fn unicode_spaces_are_blanks_as_in_the_grammars_whitespace_class() {
    // `\s` reads the Unicode White_Space characters, so a keyword followed by
    // one is still a keyword and a literal after one still starts.
    for space in [
        "\u{a0}", "\u{85}", "\u{1680}", "\u{2003}", "\u{202f}", "\u{3000}",
    ] {
        let text = format!("type{space}A = {{a: string}}");
        assert_eq!(scope_of(&text, "type"), "storage.type.type.ts", "{space:?}");
        let text = format!("x ={space}[1];");
        assert!(
            stack_of(&text, "[")
                .iter()
                .any(|s| s == "meta.array.literal.ts"),
            "{space:?}"
        );
    }
    // U+FEFF and U+200B are not White_Space.
    assert_ne!(
        scope_of("type\u{feff}A = {a: string}", "type"),
        "storage.type.type.ts"
    );
}

/// Lexes `lines` as JavaScript like [`until_in`].
fn until(
    base: &[&str],
    start: fn(&mut Arena, u32) -> u32,
    lines: &[&str],
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> Vec<(Option<usize>, Row)> {
    until_in(super::Dialect::JavaScript, base, start, lines, end, after)
}

/// Lexes `lines` in `dialect` from the state `start` makes over the scopes
/// `base`, under [`super::lex_until`] with `end` and `after`.
/// It returns, per line, where it halted and each token's text and full scope
/// stack.
/// The lines after a halt are not lexed.
fn until_in(
    dialect: super::Dialect,
    base: &[&str],
    start: fn(&mut Arena, u32) -> u32,
    lines: &[&str],
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> Vec<(Option<usize>, Row)> {
    use crate::language::{Context, Line};
    use crate::scratch::Scratch;
    use crate::{LineReport, Token};

    let mut arena = Arena::new(None);
    let mut stack = ROOT;
    for name in base {
        let scope = arena
            .named_scope(name)
            .unwrap_or_else(|| panic!("no room for {name:?}"));
        stack = arena
            .child(stack, scope)
            .unwrap_or_else(|| panic!("no room for {name:?}"));
    }
    let mut scratch = Scratch::default();
    let mut state = start(&mut arena, stack);
    let limits = Limits::default();
    let mut rows = Vec::new();
    for text in lines {
        let mut out: Vec<Token> = Vec::new();
        let mut report = LineReport::default();
        scratch.start_line();
        let mut cx = Context {
            scratch: &mut scratch,
            arena: &mut arena,
            limits: &limits,
            out: &mut out,
            report: &mut report,
            first_token: 0,
        };
        let line = Line {
            text,
            start: 0,
            embedded: false,
            anchored_at_root: false,
        };
        let (next, stopped) = super::lex_until(&mut cx, dialect, line, state, end, after);
        let halt = stopped.map(|stopped| stopped.at);
        state = next.state();
        let mut from = 0;
        let mut row = Row::new();
        for token in &out {
            let mut scopes: Vec<String> =
                arena.scopes(token.stack().0).map(str::to_owned).collect();
            scopes.reverse();
            row.push((text[from..token.end()].to_owned(), scopes));
            from = token.end();
        }
        assert_eq!(
            from,
            halt.unwrap_or(text.len()),
            "the tokens stop at the halt: {text:?}"
        );
        rows.push((halt, row));
        if halt.is_some() {
            break;
        }
    }
    rows
}

/// The end of an interpolation, as GraphQL passes it.
fn closing_brace(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'}')
}

/// Where each line of an interpolation halts.
fn interpolation(lines: &[&str]) -> Vec<Option<usize>> {
    until(&[], super::start_embedded, lines, &closing_brace, None)
        .into_iter()
        .map(|(halt, _)| halt)
        .collect()
}

#[test]
fn an_interpolation_ends_at_the_brace_that_no_context_is_waiting_for() {
    // Braces that code opens close in the code; braces in strings, comments
    // and nested templates are not told apart by counting.
    for (line, close) in [
        ("a.b(1) } c", 7),
        ("{x: 1}.x } c", 9),
        ("f(() => { return 1; }) } c", 23),
        ("\"}\" } c", 4),
        ("'}' + `a${b}` } c", 14),
        ("a /* } */ } c", 10),
        ("a // } ", usize::MAX),
    ] {
        let expected = (close != usize::MAX).then_some(close);
        assert_eq!(interpolation(&[line])[0], expected, "{line:?}");
    }
}

/// The end of a backtick attribute value, as Astro passes it.
fn backtick(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'`')
}

/// The end of an object literal entry, `(?<=})`.
fn after_brace(bytes: &[u8], p: usize) -> bool {
    p > 0 && bytes.get(p - 1) == Some(&b'}')
}

/// The innermost scope of `token` in `row`.
fn leaf_of(row: &Row, token: &str) -> String {
    row.iter()
        .find(|(text, _)| text == token)
        .and_then(|(_, scopes)| scopes.last().cloned())
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"))
}

#[test]
fn a_template_body_halts_at_a_backtick_outside_its_substitutions() {
    let rows = until_in(
        super::Dialect::Tsx,
        &[],
        super::start_template,
        &["a ${`b`} \\` c` d"],
        &backtick,
        None,
    );
    let (halt, row) = &rows[0];
    assert_eq!(*halt, Some(13));
    assert_eq!(
        leaf_of(row, "${"),
        "punctuation.definition.template-expression.begin.tsx"
    );
    assert_eq!(leaf_of(row, "\\`"), "constant.character.escape.tsx");
    let after_bracket = |bytes: &[u8], p: usize| p > 0 && bytes.get(p - 1) == Some(&b'>');
    let rows = until_in(
        super::Dialect::Tsx,
        &[],
        super::start_template,
        &["ab>c` d"],
        &after_bracket,
        None,
    );
    assert_eq!(rows[0].0, Some(3), "the body is not skipped past a halt");
}

#[test]
fn tsx_runs_the_object_literal_entry() {
    let rows = until_in(
        super::Dialect::Tsx,
        &[],
        super::start_object_literal,
        &["{a: <b />} x"],
        &after_brace,
        None,
    );
    let (halt, row) = &rows[0];
    assert_eq!(*halt, Some(10));
    assert_eq!(leaf_of(row, "a"), "meta.object-literal.key.tsx");
    assert_eq!(leaf_of(row, "b"), "entity.name.tag.tsx");
}

#[test]
fn an_interpolation_carries_its_state_to_the_next_line() {
    let rows = interpolation(&["f(1,", "  {a: 2}) + `x", "y${z}` } w"]);
    assert_eq!(rows, [None, None, Some(7)]);
}

/// The end of a script, as HTML passes it: `</script` in any case.
fn script_close(bytes: &[u8], p: usize) -> bool {
    bytes
        .get(p..p + "</script".len())
        .is_some_and(|text| text.eq_ignore_ascii_case(b"</script"))
}

/// Lexes `lines` as the inside of an HTML `<script>` element (see [`until`]).
fn script(lines: &[&str]) -> Vec<(Option<usize>, Row)> {
    until(
        &["text.html.basic", "meta.embedded.block.html"],
        super::start_script,
        lines,
        &script_close,
        None,
    )
}

/// The scopes of `token` in a row of [`script`], below the HTML and
/// `source.js` scopes that every token of a script has.
fn script_scopes<'a>(row: &'a Row, token: &str) -> &'a [String] {
    let (_, scopes) = row
        .iter()
        .find(|(text, _)| text == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {row:?}"));
    assert_eq!(
        scopes[..3],
        ["text.html.basic", "meta.embedded.block.html", "source.js"]
    );
    &scopes[3..]
}

#[test]
fn a_script_ends_before_a_close_tag_in_any_case_that_no_context_waits_for() {
    for (line, close) in [
        ("let a = 1; </script>", Some(11)),
        ("x</SCRIPT >", Some(1)),
        ("</Script", Some(0)),
        ("f({a: \"</script>\"}) </script>", Some(20)),
        ("if (a) { b(); </script>", None),
    ] {
        let rows = script(&[line]);
        assert_eq!(rows[0].0, close, "{line:?}");
    }
    let rows = script(&["let a = 1; </script>"]);
    assert_eq!(
        script_scopes(&rows[0].1, "let"),
        ["meta.var.expr.js", "storage.type.js"]
    );
}

#[test]
fn a_script_takes_its_comments_from_html_before_javascript() {
    let rows = script(&["  // a </script>"]);
    assert_eq!(rows[0].0, Some(7));
    assert_eq!(
        script_scopes(&rows[0].1, "  "),
        ["punctuation.whitespace.comment.leading.js"]
    );
    assert_eq!(
        script_scopes(&rows[0].1, "//"),
        [
            "comment.line.double-slash.js",
            "punctuation.definition.comment.js"
        ]
    );
    assert_eq!(
        script_scopes(&rows[0].1, " a "),
        ["comment.line.double-slash.js"]
    );
    // Only a lower-case close tag ends a comment.
    assert_eq!(script(&["// a </SCRIPT> b"])[0].0, None);
    let rows = script(&["/** a */ x", "/* b", " c </script> */"]);
    assert_eq!(
        script_scopes(&rows[0].1, "* a "),
        ["comment.block.js"],
        "not a doc comment"
    );
    assert_eq!(rows[1].0, None);
    assert_eq!(rows[2].0, Some(3));
    assert_eq!(script_scopes(&rows[2].1, " c "), ["comment.block.js"]);
}

#[test]
fn a_script_carries_its_state_to_the_next_line() {
    let rows = script(&["f(1,", "  2); </script>"]);
    assert_eq!(rows[0].0, None);
    assert_eq!(rows[1].0, Some(6));
    // The `if` statement around a block ends at `;`, `}` or the line's end,
    // not at the block's `}`, so it holds the rest of that line.
    let rows = script(&["if (a) {", "  // b </script>", "}</script>", "</script>"]);
    assert_eq!(rows[0].0, None);
    assert_eq!(rows[1].0, None, "a comment of the block is JavaScript's");
    assert_eq!(rows[2].0, None);
    assert_eq!(rows[3].0, Some(0));
}

#[test]
fn an_embedding_end_halts_where_a_rule_of_the_root_would_start() {
    fn at_c(bytes: &[u8], p: usize) -> bool {
        bytes.get(p) == Some(&b'c')
    }
    // The name `abc` is one match, which covers its `c`.
    // The `c` in the call's parentheses is inside a context.
    let rows = until(&[], super::start_embedded, &["abc + f(c) c"], &at_c, None);
    assert_eq!(rows[0].0, Some(11));
}

#[test]
fn an_embedding_after_halts_only_where_no_rule_matches() {
    fn digit_or_backslash(bytes: &[u8], p: usize) -> bool {
        matches!(bytes.get(p), Some(b'1' | b'\\'))
    }
    // The number rule matches at the `1`, so only the backslash halts.
    let rows = until(
        &[],
        super::start_embedded,
        &["1 + \\ 2"],
        &|_: &[u8], _: usize| false,
        Some(&digit_or_backslash),
    );
    assert_eq!(rows[0].0, Some(4));
}

#[test]
fn a_field_key_before_a_brace_at_column_zero_keeps_the_field_open() {
    let text = indoc! {"
        interface I {
          a: 1
          c }
         x
    "};
    let (lines, _) = lex(text);
    let field = [
        "source.ts",
        "meta.interface.ts",
        "meta.field.declaration.ts",
    ]
    .map(str::to_owned);
    assert_eq!(lines[2], vec![("  c }".to_owned(), field.to_vec())]);
    assert_eq!(lines[3][1].1[..3], field, "the field is still open");
    assert_eq!(lines[3][1].1[4], "variable.object.property.ts");
}

#[test]
fn a_modifier_without_a_name_after_it_is_the_parameter_name() {
    let text = "function f(readonly 1) {}";
    assert_eq!(scope_of(text, "readonly"), "variable.parameter.ts");
    assert_eq!(
        scope_of("function f(readonly x) {}", "readonly"),
        "storage.modifier.ts"
    );
}

#[test]
fn letters_and_word_characters_are_onigurumas() {
    // U+0363 is alphabetic in Rust's Unicode but not in Shiki's `[[:alpha:]]`,
    // so the name stops before it.
    assert_eq!(
        scope_of("let a\u{363}b = 1;", "a"),
        "variable.other.readwrite.ts"
    );
    // U+0300 is `\w` for Shiki, so `this` before it has no `\b` after it.
    assert_ne!(
        scope_of("x = this\u{300};", "this"),
        "variable.language.this.ts"
    );
}

#[test]
fn a_unicode_blank_is_a_blank_wherever_the_grammar_reads_s() {
    // `(?=\S)` does not end the arrow body at a no-break space, so the
    // declaration goes on to the next line.
    let (lines, _) = lex("let x = () =>\u{a0}\n  1");
    let one = &lines[1].last().expect("a token for 1").1;
    assert!(one.iter().any(|s| s == "meta.var.expr.ts"), "{one:?}");
    // `@internal` needs `\s` after it.
    let stack = stack_of("// @internal\u{a0}x", "internal");
    assert!(
        stack
            .iter()
            .any(|s| s == "storage.type.internaldeclaration.ts"),
        "{stack:?}"
    );
}

#[test]
fn a_keyword_rule_tests_word_boundaries_as_its_grammar_spells_them() {
    // `\b(interface)` does not hold after a mark, which `\w` reads.
    assert_ne!(
        scope_of("\u{300}interface I {}", "interface"),
        "storage.type.interface.ts"
    );
    // `(export)` has no `\b` before it.
    assert_eq!(
        scope_of("\u{300}export default 1;", "export"),
        "keyword.control.export.ts"
    );
    // `@internal\b` does not hold before a mark.
    let (lines, _) = lex("/** @internal\u{363} x */");
    assert!(
        !lines[0]
            .iter()
            .any(|(_, stack)| stack.iter().any(|s| s.starts_with("storage.type"))),
        "{lines:?}"
    );
}

#[test]
fn lookaheads_match_the_grammars_sets() {
    // `\s+(?=["$'_`[:alpha:]])`: the `[` opens the class `[:alpha:]`.
    assert_eq!(
        scope_of("namespace [a] = b;", "namespace"),
        "variable.other.readwrite.ts"
    );
    // `(?<=^)\s*(<)(?=NAME\s*>)`: no blank after `<`.
    assert_eq!(scope_of("< a>", "<"), "keyword.operator.relational.ts");
    // `\s*(:)(?=\s*TYPE+\s*=>)`: a type of at least one item.
    let (lines, _) = lex(":=> x");
    assert!(
        !lines[0]
            .iter()
            .any(|(_, stack)| stack.iter().any(|s| s == "meta.return.type.arrow.ts")),
        "{lines:?}"
    );
}

/// `(\d+)\s*(?=…:)`: Shiki's `\d` takes the Arabic-Indic digits.
#[test]
fn an_object_key_of_digits_reads_every_unicode_digit() {
    let text = indoc! {"
        const o = {
          ٤٢: 1,
        };
    "};
    assert_eq!(
        stack_of(text, "٤٢"),
        [
            "source.ts",
            "meta.var.expr.ts",
            "meta.objectliteral.ts",
            "meta.object.member.ts",
            "meta.object-literal.key.ts",
            "constant.numeric.decimal.ts",
        ]
    );
    assert_eq!(
        stack_of(text, ":"),
        [
            "source.ts",
            "meta.var.expr.ts",
            "meta.objectliteral.ts",
            "meta.object.member.ts",
            "meta.object-literal.key.ts",
            "punctuation.separator.key-value.ts",
        ]
    );
}
