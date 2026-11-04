use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};
#[cfg(feature = "lang-c")]
use crate::{Highlighter, Limit, Limits, State, Work};

/// Lexes the lines of `text` in order from one start state.
fn lex_lines(text: &str) -> Vec<LeafRow> {
    merge_by_leaf(test_support::lex(Language::Lua, text))
}

fn lex(line: &str) -> LeafRow {
    lex_lines(line).remove(0)
}

#[test]
fn a_long_string_closes_only_at_its_own_level() {
    let row = lex("local s = [==[a]]b]==] x");
    assert_eq!(
        leaf_in(&row, "[==["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&row, "a]]b"), "string.quoted.other.multiline.lua");
    assert_eq!(
        leaf_in(&row, "]==] "),
        "punctuation.definition.string.end.lua"
    );
    assert_eq!(leaf_in(&row, "x"), "variable.other.lua");
}

#[test]
fn a_long_string_with_no_equals_signs_runs_across_lines() {
    let rows = lex_lines(indoc! {"
        x = [[
        a]] y
    "});
    assert_eq!(
        leaf_in(&rows[0], "[["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&rows[1], "a"), "string.quoted.other.multiline.lua");
    assert_eq!(
        leaf_in(&rows[1], "]] "),
        "punctuation.definition.string.end.lua"
    );
    assert_eq!(leaf_in(&rows[1], "y"), "variable.other.lua");
}

#[test]
fn a_long_comment_closes_only_at_its_own_level() {
    let row = lex("--[==[ c ]] ]==] y");
    assert_eq!(
        leaf_in(&row, "--[==["),
        "punctuation.definition.comment.begin.lua"
    );
    assert_eq!(leaf_in(&row, " c ]] "), "comment.block.lua");
    assert_eq!(
        leaf_in(&row, "]==]"),
        "punctuation.definition.comment.end.lua"
    );
    assert_eq!(leaf_in(&row, "y"), "variable.other.lua");
}

#[test]
fn a_long_comment_with_no_equals_signs_runs_across_lines() {
    let rows = lex_lines(indoc! {"
        --[[ a
        b ]] y
    "});
    assert_eq!(
        leaf_in(&rows[0], "--[["),
        "punctuation.definition.comment.begin.lua"
    );
    assert_eq!(leaf_in(&rows[1], "b "), "comment.block.lua");
    assert_eq!(
        leaf_in(&rows[1], "]]"),
        "punctuation.definition.comment.end.lua"
    );
    assert_eq!(leaf_in(&rows[1], "y"), "variable.other.lua");
}

#[test]
fn a_documentation_comment_names_its_tags_and_types() {
    let row = lex("---@param name? string");
    assert_eq!(leaf_in(&row, "@param"), "storage.type.annotation.lua");
    assert_eq!(leaf_in(&row, "name"), "entity.name.variable.lua");
    assert_eq!(leaf_in(&row, "?"), "keyword.operator.lua");
    assert_eq!(leaf_in(&row, "string"), "support.type.lua");
}

/// The `@enum`, `@cast` and `@diagnostic` tags end at `(?=\n)`, which leaves
/// the line feed to the comment, so the comment ends with its line.
#[test]
fn a_documentation_tag_that_ends_its_line_ends_before_the_line_feed() {
    let rows = lex_lines(indoc! {"
        ---@enum Color
        local x = 1
        ---@cast x string
        return x
        ---@diagnostic disable
        local y
    "});
    assert_eq!(leaf_in(&rows[0], "Color"), "variable.lua");
    assert_eq!(leaf_in(&rows[1], "local"), "keyword.local.lua");
    assert_eq!(leaf_in(&rows[2], "string"), "support.type.lua");
    assert_eq!(leaf_in(&rows[3], "return"), "keyword.control.lua");
    assert_eq!(leaf_in(&rows[4], "disable"), "keyword.other.unit");
    assert_eq!(leaf_in(&rows[5], "local"), "keyword.local.lua");
}

#[test]
fn a_function_head_names_its_parameters() {
    let row = lex("local function f(a, ...) end");
    assert_eq!(leaf_in(&row, "local"), "keyword.local.lua");
    assert_eq!(leaf_in(&row, "f"), "entity.name.function.lua");
    assert_eq!(leaf_in(&row, "a"), "variable.parameter.function.lua");
    assert_eq!(
        leaf_in(&row, ")"),
        "punctuation.definition.parameters.finish.lua"
    );
    assert_eq!(leaf_in(&row, "end"), "keyword.control.lua");
}

#[test]
fn a_binary_exponent_needs_its_sign() {
    let row = lex("print(0x1p4, 3.5e-2, 0xA.)");
    assert_eq!(
        leaf_in(&row, "0x1"),
        "constant.numeric.float.hexadecimal.lua"
    );
    assert_eq!(leaf_in(&row, "p4, "), "source.lua");
    assert_eq!(leaf_in(&row, "3.5e-2"), "constant.numeric.float.lua");
    assert_eq!(
        leaf_in(&row, "0xA."),
        "constant.numeric.float.hexadecimal.lua"
    );
}

#[test]
fn the_standard_library_is_named_unless_it_is_a_member() {
    let row = lex(r#"string.format("%d\n", math.pi)"#);
    assert_eq!(
        leaf_in(&row, "string.format"),
        "support.function.library.lua"
    );
    assert_eq!(leaf_in(&row, r"\n"), "constant.character.escape.lua");
    assert_eq!(leaf_in(&row, "math.pi"), "constant.language.lua");
    let row = lex("t.x = function() end");
    assert_eq!(leaf_in(&row, "x"), "entity.name.function.lua");
}

#[test]
fn labels_and_attributes_are_named() {
    assert_eq!(leaf_in(&lex("goto continue"), "continue"), "string.tag.lua");
    let row = lex("::continue::");
    assert_eq!(
        leaf_in(&row, "::"),
        "punctuation.section.embedded.begin.lua"
    );
    assert_eq!(
        leaf_in(&lex("local x <const> = 1"), "<const>"),
        "storage.type.attribute.lua"
    );
}

/// The body of `ffi.cdef[[...]]` is C, up to the bracket that closes it.
#[cfg(feature = "lang-c")]
#[test]
fn an_ffi_cdef_body_lexes_as_c() {
    let row = lex("ffi.cdef[[ int x; ]]");
    assert_eq!(
        leaf_in(&row, "[["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&row, "int"), "storage.type.built-in.primitive.c");
    assert_eq!(leaf_in(&row, "]]"), "punctuation.definition.string.end.lua");
}

/// A cdef body with `=` signs goes back to Lua only at a bracket of its own
/// level. The `]]` of `a[b[1]]` stays C.
#[cfg(feature = "lang-c")]
#[test]
fn an_ffi_cdef_body_closes_only_at_its_own_level() {
    let rows = lex_lines(indoc! {"
        ffi.cdef[=[
        int a[b[1]];
        ]=] x
    "});
    assert_eq!(
        leaf_in(&rows[1], "int"),
        "storage.type.built-in.primitive.c"
    );
    assert!(
        rows[1]
            .iter()
            .all(|(_, scope)| scope != "punctuation.definition.string.end.lua"),
        "{:?}",
        rows[1]
    );
    assert_eq!(
        leaf_in(&rows[2], "]=]"),
        "punctuation.definition.string.end.lua"
    );
    assert_eq!(leaf_in(&rows[2], "x"), "variable.other.lua");
}

/// A line in a cdef body is lexed twice: as Lua, then with C.
/// Both passes spend the line's one work budget.
/// Under a tight limit, a long body line runs out of work.
/// Its tokens still cover the line.
#[cfg(feature = "lang-c")]
#[test]
fn a_long_cdef_body_under_a_tight_work_limit_still_covers_its_lines() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 8,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Lua, limits);
    let mut state = State::default();
    let body = "int a[b[1]]; ".repeat(200);
    for (index, line) in ["ffi.cdef[[", body.as_str(), "]] x"]
        .into_iter()
        .enumerate()
    {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert_eq!(tokens.last().map(|token| token.end()), Some(line.len()));
        if index == 1 {
            assert!(report.reached(Limit::Work));
        }
    }
}

#[cfg(feature = "json")]
mod probe {
    use crate::lua::engine::Lua;
    use crate::lua::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Lua's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Lua, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
