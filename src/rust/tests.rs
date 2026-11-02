use indoc::indoc;

use super::state::Kind;
use crate::{Highlighter, Language, State};

fn highlighter() -> Highlighter {
    Highlighter::new(Language::Rust)
}

/// The innermost open context after a line, and how many are open.
fn top(h: &Highlighter, state: State) -> Kind {
    Kind::from_code(h.frame(state).kind)
}

fn depth(h: &Highlighter, state: State) -> u32 {
    h.frame(state).depth - 1
}

/// Lexes `text` and returns, per token, its text and innermost scope.
fn lex(text: &str) -> Vec<Vec<(String, String)>> {
    lex_with_states(text).0
}

fn lex_with_states(text: &str) -> (Vec<Vec<(String, String)>>, Vec<State>) {
    lex_with(&mut highlighter(), text)
}

fn lex_with(h: &mut Highlighter, text: &str) -> (Vec<Vec<(String, String)>>, Vec<State>) {
    let mut state = State::default();
    let (mut lines, mut states) = (Vec::new(), Vec::new());
    for line in text.split('\n') {
        let mut toks = Vec::new();
        let _ = h.line(line, &mut state, &mut toks);
        let mut start = 0;
        let mut row = Vec::new();
        for t in &toks {
            let end = t.end();
            let innermost = h.scopes(t.stack()).next().unwrap();
            row.push((line[start..end].to_string(), innermost.to_string()));
            start = end;
        }
        assert_eq!(start, line.len(), "tokens must cover the line: {line:?}");
        lines.push(row);
        states.push(state);
    }
    (lines, states)
}

/// The innermost scope of the first token whose text is `text`.
fn scope_of(text: &str, token: &str) -> String {
    let rows = lex(text);
    rows.iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {rows:?}"))
        .1
        .clone()
}

/// The full scope stack of the first token whose text is `token`.
fn stack_of(text: &str, token: &str) -> Vec<String> {
    let mut h = highlighter();
    let mut state = State::default();
    for line in text.split('\n') {
        let mut toks = Vec::new();
        let _ = h.line(line, &mut state, &mut toks);
        let mut start = 0;
        for t in &toks {
            let end = t.end();
            if &line[start..end] == token {
                let mut stack: Vec<String> = h.scopes(t.stack()).map(ToString::to_string).collect();
                stack.reverse();
                return stack;
            }
            start = end;
        }
    }
    panic!("no token {token:?}");
}

#[test]
fn nested_block_comments_span_lines() {
    let text = indoc! {"
        /* a /* b
        */ still */ let
    "};
    let mut h = highlighter();
    let (rows, states) = lex_with(&mut h, text);
    assert_eq!(rows[0][0].1, "comment.block.rust");
    assert_eq!(depth(&h, states[0]), 2, "two comments open after line 1");
    assert_eq!(
        stack_of(text, " still "),
        ["source.rust", "comment.block.rust"],
        "one comment still open after the inner `*/`"
    );
    assert_eq!(depth(&h, states[1]), 0);
    assert_eq!(rows[1].last().unwrap().1, "storage.type.rust");
}

#[test]
fn doc_comments() {
    assert_eq!(
        scope_of("/// text", "///"),
        "punctuation.definition.comment.rust"
    );
    assert_eq!(
        scope_of("/// text", " text"),
        "comment.line.documentation.rust"
    );
    assert_eq!(
        scope_of("// text", " text"),
        "comment.line.double-slash.rust"
    );
    assert_eq!(
        scope_of("/** d */", " d "),
        "comment.block.documentation.rust"
    );
    assert_eq!(scope_of("/**/", "/**/"), "comment.block.rust");
}

#[test]
fn raw_strings_resume_across_lines() {
    let text = indoc! {r##"
        let s = r#"a "q" b
        "# + "x";
    "##};
    let mut h = highlighter();
    let (rows, states) = lex_with(&mut h, text);
    assert_eq!(rows[0].last().unwrap().1, "string.quoted.double.rust");
    assert_eq!(top(&h, states[0]), Kind::Raw);
    assert_eq!(top(&h, states[1]), Kind::Top);
    assert_eq!(scope_of(text, "r"), "string.quoted.byte.raw.rust");
    assert_eq!(
        scope_of(text, "#"),
        "punctuation.definition.string.raw.rust"
    );
}

#[test]
fn strings_and_escapes() {
    assert_eq!(
        stack_of(r#"b"a\n""#, "\\"),
        [
            "source.rust",
            "string.quoted.double.rust",
            "constant.character.escape.rust",
            "constant.character.escape.backslash.rust"
        ]
    );
    assert_eq!(
        scope_of(r#""\x41""#, "x41"),
        "constant.character.escape.bit.rust"
    );
    assert_eq!(
        scope_of(r#""\u{1F600}""#, "1F600"),
        "constant.character.escape.unicode.rust"
    );
    assert_eq!(scope_of(r#""{name}""#, "name"), "meta.interpolation.rust");
    assert_eq!(scope_of(r#"b"x""#, "b"), "string.quoted.byte.raw.rust");
}

#[test]
fn unterminated_string_runs_to_the_end() {
    let mut h = highlighter();
    let (rows, states) = lex_with(&mut h, "let s = \"open\nstill string\nlet t");
    assert_eq!(top(&h, states[0]), Kind::Str);
    assert_eq!(rows[1][0].1, "string.quoted.double.rust");
    assert_eq!(rows[2][0].1, "string.quoted.double.rust", "never recovers");
}

#[test]
fn char_or_lifetime() {
    assert_eq!(
        scope_of("let c = 'a';", "a"),
        "string.quoted.single.char.rust"
    );
    assert_eq!(
        scope_of("fn f<'a>(x: &'a u8) {}", "a"),
        "entity.name.type.lifetime.rust"
    );
    assert_eq!(
        scope_of(r"let c = '\'';", "'"),
        "punctuation.definition.char.rust"
    );
    assert_eq!(
        scope_of(r"let c = b'x';", "b"),
        "string.quoted.byte.raw.rust"
    );
}

#[test]
fn numbers() {
    assert_eq!(
        scope_of("let n = 42u8;", "u8"),
        "entity.name.type.numeric.rust"
    );
    assert_eq!(
        scope_of("let n = 0xFFi64;", "0xFF"),
        "constant.numeric.hex.rust"
    );
    assert_eq!(
        scope_of("let n = 0b1010;", "0b1010"),
        "constant.numeric.bin.rust"
    );
    assert_eq!(
        scope_of("let n = 1_000.5e-3f64;", "."),
        "punctuation.separator.dot.decimal.rust"
    );
    assert_eq!(
        scope_of("let n = 1e-3;", "-"),
        "keyword.operator.exponent.sign.rust"
    );
    assert_eq!(
        scope_of("for i in 1..=3 {}", "1"),
        "constant.numeric.decimal.rust"
    );
    assert_eq!(scope_of("let n = 0b102;", "0b102"), "variable.other.rust");
}

#[test]
fn attributes_and_macros() {
    assert_eq!(
        scope_of("#[derive(Debug)]", "#"),
        "punctuation.definition.attribute.rust"
    );
    assert_eq!(
        scope_of("#[derive(Debug)]", "Debug"),
        "entity.name.type.rust"
    );
    assert_eq!(
        stack_of("#![allow(x)]", "["),
        [
            "source.rust",
            "meta.attribute.rust",
            "punctuation.brackets.attribute.rust"
        ]
    );
    assert_eq!(
        scope_of("println!(\"x\");", "println!"),
        "entity.name.function.macro.rust"
    );
    assert_eq!(
        scope_of("macro_rules! m { }", "macro_rules!"),
        "entity.name.function.macro.rules.rust"
    );
    assert_eq!(
        scope_of("$x:expr", "expr"),
        "variable.other.metavariable.specifier.rust"
    );
}

#[test]
fn definitions_calls_and_generics() {
    assert_eq!(
        stack_of("fn main() {", "main"),
        [
            "source.rust",
            "meta.function.definition.rust",
            "entity.name.function.rust"
        ]
    );
    assert_eq!(
        stack_of("fn main() {}\nlet x = 1;", "1"),
        ["source.rust", "constant.numeric.decimal.rust"],
        "the body brace closes the definition"
    );
    assert_eq!(
        stack_of("foo(a, b)", "a"),
        [
            "source.rust",
            "meta.function.call.rust",
            "variable.other.rust"
        ]
    );
    assert_eq!(
        scope_of("let v: Vec<u8> = x;", "Vec"),
        "entity.name.type.rust"
    );
    assert_eq!(
        scope_of("use std::io;", "std"),
        "entity.name.namespace.rust"
    );
    assert_eq!(scope_of("a < b", "<"), "keyword.operator.comparison.rust");
}

#[test]
fn a_saved_state_resumes_identically() {
    let mut h = highlighter();
    let mut state = State::default();
    let mut out = Vec::new();
    let _ = h.line("let s = \"open", &mut state, &mut out);
    let saved = state;
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let (mut sa, mut sb) = (saved, saved);
    let _ = h.line("still\" + 1", &mut sa, &mut a);
    let _ = h.line("still\" + 1", &mut sb, &mut b);
    assert_eq!(a, b);
    assert_eq!(sa, sb);
    assert_eq!(
        h.scopes(a[0].stack()).next(),
        Some("string.quoted.double.rust")
    );
}

#[test]
fn deep_nesting_degrades_without_panicking() {
    let text = format!("{}x{}", "/*".repeat(100), "*/".repeat(100));
    let mut h = highlighter();
    let (rows, states) = lex_with(&mut h, &text);
    assert_eq!(depth(&h, states[0]), 0);
    assert_eq!(rows[0][0].1, "comment.block.rust");
    let calls = format!("{}x{}", "f(".repeat(100), ")".repeat(100));
    lex(&calls);
}

#[test]
fn state_after_a_balanced_document_is_initial() {
    let (_, states) = lex_with_states("fn f() { /* a */ \"b\" }\n");
    assert_eq!(*states.last().unwrap(), State::default());
}

#[test]
fn an_uppercase_word_before_non_ascii_text_advances() {
    for line in ["R日", "Ra日", "a日x", "_日x", "Self日"] {
        let mut h = highlighter();
        let mut out = Vec::new();
        let report = h.line(line, &mut State::default(), &mut out);
        assert!(report.is_complete(), "{line:?}");
        assert_eq!(out.last().map(|t| t.end()), Some(line.len()));
    }
}
