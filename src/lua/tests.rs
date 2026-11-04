use indoc::indoc;

use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

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

/// Shiki lexes the body of `ffi.cdef[[...]]` as C; the body stays the
/// embedding rule's content until the engine can run another lexer inside a
/// rule.
#[test]
fn an_ffi_cdef_body_is_left_unlexed() {
    let row = lex("ffi.cdef[[ int x; ]]");
    assert_eq!(
        leaf_in(&row, "[["),
        "punctuation.definition.string.begin.lua"
    );
    assert_eq!(leaf_in(&row, " int x; "), "meta.embedded.lua");
    assert_eq!(leaf_in(&row, "]]"), "punctuation.definition.string.end.lua");
}

#[cfg(feature = "json")]
mod probe {

    use serde_json::{Value, json};

    use crate::arena::Arena;
    use crate::language::Parts;
    use crate::limits::{Limits, LineReport};
    use crate::lua::engine::{Lexer, Scratch};
    use crate::lua::grammar::Pat;
    use crate::rules::Text;

    /// Runs the matchers on the cases in the file that `PROBE_IN` names, one
    /// JSON object per line (`rx`, `text`, `p`, `g`, `nl`), and writes their
    /// answers to the file `PROBE_OUT` names. `oracle/onig.ts` compares them
    /// with Oniguruma's. The text of a case is the start of a document, so
    /// `\A` holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        let input = std::env::var("PROBE_IN").expect("PROBE_IN");
        let output = std::env::var("PROBE_OUT").expect("PROBE_OUT");
        let cases = std::fs::read_to_string(input).expect("cases");
        let mut out = String::new();
        let mut arena = Arena::new(None);
        let limits = Limits::default();
        for line in cases.lines() {
            let case: Value = serde_json::from_str(line).expect("a case");
            let name = case["rx"].as_str().expect("rx");
            let pat = Pat::ALL
                .iter()
                .copied()
                .find(|pat| format!("{pat:?}") == name)
                .expect("a known pattern");
            let text = case["text"].as_str().expect("text");
            let p = usize::try_from(case["p"].as_u64().expect("p")).expect("p");
            let mut tokens = Vec::new();
            let mut report = LineReport::default();
            let mut scratch = Scratch::default();
            let parts = Parts {
                arena: &mut arena,
                limits: &limits,
                out: &mut tokens,
                report: &mut report,
            };
            let mut lexer = Lexer::new(
                parts,
                Text {
                    bytes: text.as_bytes(),
                    start: 0,
                    anchored: false,
                },
                crate::arena::ROOT,
                &mut scratch,
            );
            lexer.nl = case["nl"].as_bool().unwrap_or(true);
            lexer.first_line = true;
            let g = case["g"].as_bool().unwrap_or(false);
            let answer = lexer.probe_matcher(p, g, |lexer, p| lexer.matcher(pat, p));
            let value = answer.map_or(Value::Null, |(end, caps)| {
                json!({ "end": end, "caps": caps.iter().map(|c| c.map(|(a, b)| [a, b])).collect::<Vec<_>>() })
            });
            out.push_str(&value.to_string());
            out.push('\n');
        }
        std::fs::write(output, out).expect("write");
    }
}
