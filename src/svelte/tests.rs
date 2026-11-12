use indoc::indoc;

use crate::test_support::leaves::{LeafRow, leaf_in, merge_by_leaf};
use crate::{Language, test_support};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::rows::lex(Language::Svelte, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("svelte"), Some(Language::Svelte));
}

#[test]
fn a_tag_names_its_parts() {
    let row = lex("<div class=\"a\">hi</div>");
    assert_eq!(leaf_in(&row, "div"), "entity.name.tag.svelte");
    assert_eq!(leaf_in(&row, "class"), "entity.other.attribute-name.svelte");
}

/// `{ {a: 1} }` hands its object literal to TypeScript at the
/// `source.ts#object-literal` entry.
#[test]
fn an_interpolated_object_literal_starts_typescript_at_its_entry() {
    let row = lex("{ {a: 1} }");
    assert_eq!(leaf_in(&row, "a"), "meta.object-literal.key.ts");
    assert_eq!(leaf_in(&row, "1"), "constant.numeric.decimal.ts");
}

/// TypeScript lexes a `{@debug}` name again on its own, as Shiki does.
/// `else` there is a keyword, and `$x` takes Svelte's prefix.
#[test]
fn a_debug_name_is_lexed_again_by_typescript() {
    let row = lex("{@debug else, $x}");
    assert_eq!(leaf_in(&row, "else"), "keyword.control.conditional.ts");
    assert_eq!(leaf_in(&row, "$"), "punctuation.definition.variable.svelte");
    assert_eq!(leaf_in(&row, "x"), "variable.other.readwrite.ts");
}

/// The `if` earlier on the line puts `{:else}` in the mode that lexes `b`
/// as TypeScript.
#[test]
fn a_mode_reads_the_whole_line_before_the_tag() {
    let row = lex("{#if a}{:else b}");
    assert_eq!(leaf_in(&row, "b"), "variable.other.readwrite.ts");
}

/// An `{#each}` tag left open holds the next line's `{c}` as destructuring.
/// After its `}`, the destructuring rule opens at `{/each}` and closes there
/// without advancing, so the rest of the line keeps its scopes, as in Shiki.
/// The next line goes on inside it.
#[test]
fn destructuring_that_closes_without_advancing_stops_the_line() {
    let text = indoc! {"
        {#each a as b
        {c}{/each}
        d}"};
    let rows = test_support::rows::lex(Language::Svelte, text);
    let tag = [
        "source.svelte",
        "meta.special.each.svelte",
        "meta.special.start.svelte",
        "meta.embedded.expression.svelte",
        "source.ts",
    ];
    let brace = [
        &tag[..],
        &["punctuation.definition.binding-pattern.object.ts"],
    ]
    .concat();
    let cases: [(usize, std::ops::Range<usize>, &[&str]); 3] =
        [(1, 0..1, &brace), (1, 3..10, &tag), (2, 0..2, &tag)];
    for (row, bytes, want) in cases {
        let mut start = 0;
        for (token, scopes) in &rows[row] {
            let end = start + token.len();
            if start < bytes.end && bytes.start < end {
                assert_eq!(scopes, want, "{token:?} on line {row}");
            }
            start = end;
        }
    }
}

#[cfg(feature = "json")]
mod probe {
    use crate::svelte::engine::Svelte;
    use crate::svelte::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Svelte's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Svelte, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
