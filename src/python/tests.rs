use indoc::indoc;

use crate::test_support::rows::Row;
use crate::{Language, test_support};

fn lex(text: &str) -> Vec<Row> {
    test_support::rows::lex(Language::Python, text)
}

/// The scopes of the first token whose text is `token`, outermost first.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    test_support::scopes::scopes_of(Language::Python, text, token)
}

fn last(text: &str, token: &str) -> String {
    scopes_of(text, token).pop().unwrap_or_default()
}

#[test]
fn a_function_definition() {
    assert_eq!(
        scopes_of("def f(a): pass", "f"),
        [
            "source.python",
            "meta.function.python",
            "entity.name.function.python"
        ]
    );
    assert_eq!(
        last("async def f(): pass", "async"),
        "storage.type.function.async.python"
    );
    assert_eq!(
        last("def f(self, x): pass", "self"),
        "variable.parameter.function.language.special.self.python"
    );
}

#[test]
fn a_class_and_its_bases() {
    assert_eq!(
        last("class A(B, metaclass=M): pass", "A"),
        "entity.name.type.class.python"
    );
    assert_eq!(
        last("class A(B, metaclass=M): pass", "B"),
        "entity.other.inherited-class.python"
    );
    assert_eq!(
        last("class A(B, metaclass=M): pass", "metaclass"),
        "support.type.metaclass.python"
    );
}

#[test]
fn a_decorator() {
    assert_eq!(last("@property", "property"), "support.type.python");
    assert_eq!(
        last("@app.route(\"/\")", "route"),
        "entity.name.function.decorator.python"
    );
}

#[test]
fn numbers_in_every_base() {
    for (text, token, scope) in [
        ("x = 0xFF", "FF", "constant.numeric.hex.python"),
        ("x = 0b101", "101", "constant.numeric.bin.python"),
        ("x = 0o17", "17", "constant.numeric.oct.python"),
        ("x = 1_000", "1_000", "constant.numeric.dec.python"),
        ("x = 1.5e-3j", "1.5e-3", "constant.numeric.float.python"),
        ("x = 10L", "10", "constant.numeric.bin.python"),
        ("x = 08", "8", "invalid.illegal.dec.python"),
    ] {
        assert_eq!(last(text, token), scope, "{text}");
    }
}

#[test]
fn a_string_prefix_and_its_escapes() {
    assert_eq!(last("x = b'a\\n'", "b"), "storage.type.string.python");
    assert_eq!(
        last("x = 'a\\n'", "\\n"),
        "constant.character.escape.python"
    );
    assert_eq!(
        last("x = '%s'", "%s"),
        "constant.character.format.placeholder.other.python"
    );
    assert_eq!(
        last("x = '{0}'", "{0}"),
        "constant.character.format.placeholder.other.python"
    );
}

#[test]
fn an_f_string_has_expressions_in_its_braces() {
    assert_eq!(last("x = f'{y}'", "y"), "meta.fstring.python");
    assert_eq!(last("x = f'{y!r:>5}'", "!r"), "storage.type.format.python");
    assert_eq!(
        last("x = f'{a+b}'", "+"),
        "keyword.operator.arithmetic.python"
    );
}

#[test]
fn a_raw_string_that_starts_with_r_is_a_regular_expression() {
    assert_eq!(last("x = r'a+'", "+"), "keyword.operator.quantifier.regexp");
    assert_eq!(
        last("x = r'(?P<n>x)'", "?P<n>"),
        "entity.name.tag.named.group.regexp"
    );
    assert_eq!(last("x = R'a+'", "a+"), "string.quoted.raw.single.python");
}

#[test]
fn a_docstring_is_a_statement_that_starts_a_line() {
    let text = indoc! {"
        def f():
            \"\"\"Doc.\"\"\"
            x = \"\"\"not\"\"\"
    "};
    assert_eq!(last(text, "Doc."), "string.quoted.docstring.multi.python");
    assert_eq!(last(text, "not"), "string.quoted.multi.python");
}

/// `(\1)|(\n)` closes a single-quoted docstring at the line feed after a
/// backslash.
/// A single-quoted string goes on there.
/// The line after the docstring is code.
#[test]
fn a_single_quoted_docstring_closes_at_a_line_feed_after_a_backslash() {
    let text = indoc! {r#"
        '\n\
        The \"quick\"\n\
    "#};
    let rows = lex(text);
    let end = rows[0]
        .last()
        .map(|(t, s)| (t.as_str(), s.last().map(String::as_str)));
    assert_eq!(end, Some(("\\", Some("constant.language.python"))));
    assert_eq!(scopes_of(text, "The"), ["source.python"]);
}

#[test]
fn a_triple_quoted_string_continues_across_lines() {
    let text = indoc! {"
        x = '''a
        b'''
        y = 1"};
    let rows = lex(text);
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.multi.python")
    );
    let y = rows[2].iter().find(|(t, _)| t == "y").map(|(_, s)| s.len());
    assert_eq!(y, Some(1));
}

#[test]
fn brackets_continue_across_lines() {
    let rows = lex("f(1,\n  2)");
    assert!(
        rows[1]
            .iter()
            .any(|(t, s)| t == "2" && s.contains(&"meta.function-call.python".to_owned()))
    );
}

#[test]
fn an_empty_line_leaves_the_state_as_it_is() {
    let rows = lex(indoc! {"
        x = '''a

        b'''"});
    assert_eq!(rows[1].len(), 0);
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("string.quoted.multi.python")
    );
}

#[test]
fn keywords_names_and_builtins() {
    assert_eq!(last("if x: pass", "if"), "keyword.control.flow.python");
    assert_eq!(last("print(x)", "print"), "support.function.builtin.python");
    assert_eq!(
        last("x = ValueError", "ValueError"),
        "support.type.exception.python"
    );
    assert_eq!(
        last("x = MAX_SIZE", "MAX_SIZE"),
        "constant.other.caps.python"
    );
    assert_eq!(last("x.attr", "attr"), "meta.attribute.python");
    assert_eq!(
        last("lambda a: a", "lambda"),
        "storage.type.function.lambda.python"
    );
    assert_eq!(
        last("from a import b", "from"),
        "keyword.control.import.python"
    );
}

#[test]
fn a_non_ascii_name_is_a_name() {
    assert_eq!(
        last("def ünï(): pass", "ünï"),
        "entity.name.function.python"
    );
}
