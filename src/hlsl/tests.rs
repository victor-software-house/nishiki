use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Hlsl);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

/// The innermost scope of the first token whose text is `token`.
fn leaf(text: &str, token: &str) -> String {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
        .pop()
        .unwrap_or_default()
}

/// The innermost scope of the token that covers the first byte of `needle`
/// in the one-line `text`, or `""` where the text is plain.
fn scope_of(text: &str, needle: &str) -> String {
    let at = text
        .find(needle)
        .unwrap_or_else(|| panic!("no {needle:?} in {text:?}"));
    let mut start = 0;
    for (token, mut scopes) in lex(text).into_iter().flatten() {
        let end = start + token.len();
        if (start..end).contains(&at) {
            return scopes.pop().map_or_else(String::new, |leaf| {
                if leaf == "source.hlsl" {
                    String::new()
                } else {
                    leaf
                }
            });
        }
        start = end;
    }
    panic!("no token covers {needle:?} in {text:?}");
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("hlsl"), Some(Language::Hlsl));
}

#[test]
fn a_name_before_a_paren_is_a_function_unless_an_earlier_pattern_lists_it() {
    let text = "float4(x) + Texture2D (y) + max(a, b) + Foo_1(z)";
    assert_eq!(leaf(text, "float4"), "storage.type.basic.hlsl");
    assert_eq!(leaf(text, "Texture2D"), "support.function.hlsl");
    assert_eq!(leaf(text, "max"), "support.function.hlsl");
    assert_eq!(leaf(text, "Foo_1"), "support.function.hlsl");
}

#[test]
fn fx_values_match_in_any_case_wherever_they_stand() {
    let text = "x = max; y = Less_Equal; Texture2D t; BlendOp = add";
    assert_eq!(
        leaf(text, "max"),
        "support.constant.property-value.fx.blendop.hlsl"
    );
    assert_eq!(
        leaf(text, "Less_Equal"),
        "support.constant.property-value.fx.comparisonfunc.hlsl"
    );
    assert_eq!(leaf(text, "Texture2D"), "support.type.texture.hlsl");
    assert_eq!(
        leaf(text, "BlendOp"),
        "meta.object-literal.key.fx.blendstate.hlsl"
    );
    assert_eq!(
        leaf(text, "add"),
        "support.constant.property-value.fx.blendop.hlsl"
    );
}

#[test]
fn a_semantic_follows_a_colon_and_at_most_one_blank() {
    let text =
        "float4 p : SV_Position; float2 uv :texcoord12; float c : COLORx; v = c ? a : color;";
    assert_eq!(
        leaf(text, "SV_Position"),
        "support.variable.semantic.sm4.hlsl"
    );
    assert_eq!(leaf(text, "texcoord12"), "support.variable.semantic.hlsl");
    assert_eq!(leaf(text, "color"), "support.variable.semantic.hlsl");
    assert_eq!(scope_of("float c :  COLOR;", "COLOR"), "");
    assert_eq!(
        leaf("float4 t : SV_Target7, u : SV_Target8;", "SV_Target7"),
        "support.variable.semantic.sm4.hlsl"
    );
    assert_eq!(scope_of("float4 u : SV_Target8;", "SV_Target8"), "");
}

#[test]
fn a_directive_starts_its_line_and_needs_no_end_of_word() {
    assert_eq!(
        leaf("  #  ifdef DEBUG", "  #  ifdef"),
        "keyword.preprocessor.hlsl"
    );
    assert_eq!(leaf("#ifx", "#if"), "keyword.preprocessor.hlsl");
    assert_eq!(
        leaf("#include \"a.hlsli\"", "\""),
        "string.quoted.double.hlsl"
    );
    assert_eq!(scope_of("x #define", "#define"), "");
}

#[test]
fn numbers_follow_the_grammar_s_backtracking() {
    assert_eq!(leaf("x = 1.5x", "1."), "constant.numeric.decimal.hlsl");
    assert_eq!(leaf("x = a.5f", ".5f"), "constant.numeric.decimal.hlsl");
    assert_eq!(leaf("x = 2.5F", "2.5F"), "constant.numeric.decimal.hlsl");
    assert_eq!(leaf("x = 3f + 4", "3f"), "constant.numeric.decimal.hlsl");
    assert_eq!(leaf("x = 0x1F", "0x1F"), "constant.numeric.hex.hlsl");
    assert_eq!(scope_of("x = 0x1G", "0x1G"), "");
}

#[test]
fn a_block_comment_and_a_string_run_across_lines() {
    let rows = lex(indoc! {r#"
        /* a
        b */ int c;
        string s = "x\"y\\
    "#});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("comment.line.block.hlsl")
    );
    assert!(rows[1].iter().any(|(text, scopes)| text == "int"
        && scopes.last().map(String::as_str) == Some("storage.type.basic.hlsl")));
    assert!(rows[2].iter().any(|(text, scopes)| text == "\\\""
        && scopes.last().map(String::as_str) == Some("constant.character.escape.hlsl")));
}

#[cfg(feature = "json")]
mod probe {
    use crate::hlsl::engine::Hlsl;
    use crate::hlsl::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with HLSL's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<Hlsl, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
