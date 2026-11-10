use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Zig, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("zig"), Some(Language::Zig));
}

#[test]
fn inline_is_a_loop_keyword_unless_fn_follows() {
    let row = lex("inline for (x) |a| a and b");
    assert_eq!(leaf_in(&row, "inline"), "keyword.control.repeat.zig");
    assert_eq!(leaf_in(&row, "|"), "keyword.operator.bitwise.zig");
    assert_eq!(leaf_in(&row, "and"), "keyword.default.zig");
    let row = lex("pub inline fn Foo(a: u32) !void {}");
    assert_eq!(leaf_in(&row, "inline"), "keyword.storage.zig");
    assert_eq!(leaf_in(&row, "fn"), "storage.type.function.zig");
    assert_eq!(leaf_in(&row, "Foo"), "entity.name.type.zig");
    assert_eq!(leaf_in(&row, "u32"), "keyword.type.integer.zig");
    assert_eq!(leaf_in(&row, "void"), "keyword.type.zig");
}

/// `1e+` gives its sign back to end at a word boundary.
/// `1.e` matches the float rule as `1`, which comes before the invalid one.
#[test]
fn numbers_end_where_shiki_ends_them() {
    let row = lex("const x = 0x1.8p3 + 1e+ + 1.e+ + 1x;");
    assert_eq!(leaf_in(&row, "0x1.8p3"), "constant.numeric.hexfloat.zig");
    assert_eq!(leaf_in(&row, "1e"), "constant.numeric.invalid.zig");
    assert_eq!(leaf_in(&row, "1"), "constant.numeric.float.zig");
    assert_eq!(leaf_in(&row, "e"), "variable.zig");
    assert_eq!(leaf_in(&row, "1x"), "constant.numeric.invalid.zig");
}

#[test]
fn a_documentation_comment_names_its_todo() {
    let row = lex("/// doc TODO: x");
    assert_eq!(leaf_in(&row, "/// doc "), "comment.line.documentation.zig");
    assert_eq!(leaf_in(&row, "TODO:"), "keyword.todo.zig");
}

#[test]
fn quoted_names_are_strings() {
    let row = lex(r#"fn @"a b"() void {} @"x""#);
    assert_eq!(leaf_in(&row, "fn"), "storage.type.function.zig");
    assert_eq!(
        leaf_in(&row, r#" @"a b""#),
        "entity.name.function.string.zig"
    );
    assert_eq!(leaf_in(&row, r#"@"x""#), "variable.string.zig");
}

/// The comparison rule comes first, so `<<` is two `<` operators.
#[test]
fn a_shift_is_two_comparisons() {
    let row = lex("var p: [*c]u8 = @intCast(a <<%= 2);");
    assert_eq!(leaf_in(&row, "*c"), "keyword.operator.c-pointer.zig");
    assert_eq!(leaf_in(&row, "@intCast"), "support.function.builtin.zig");
    assert_eq!(leaf_in(&row, "<<"), "keyword.operator.comparison.zig");
    assert_eq!(leaf_in(&row, "%="), "keyword.operator.arithmetic.zig");
    assert_eq!(leaf_in(&row, "2"), "constant.numeric.float.zig");
}

#[cfg(feature = "json")]
mod probe {
    use crate::test_support::probe::{answer_cases, lexer};
    use crate::zig::engine::Zig;
    use crate::zig::grammar::Pattern;

    /// Answers the cases of `oracle/onig.ts` with Zig's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Zig, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
