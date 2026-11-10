use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::ShaderLab);
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
                if leaf == "source.shaderlab" {
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
fn the_names_are_known() {
    assert_eq!(Language::from_name("shaderlab"), Some(Language::ShaderLab));
    assert_eq!(Language::from_name("shader"), Some(Language::ShaderLab));
}

#[test]
fn a_block_lexes_hlsl_with_unity_names_after_it() {
    let rows = lex(indoc! {"
        CGPROGRAM
        fixed4 c = fixed4(1, 1, 1, 1); float4 p = mul(UNITY_MATRIX_MVP, v);
        ENDCG
    "});
    let leaf_of = |token: &str| {
        rows[1]
            .iter()
            .find(|(text, _)| text == token)
            .and_then(|(_, scopes)| scopes.last().cloned())
            .unwrap_or_else(|| panic!("no token {token:?}"))
    };
    assert_eq!(
        rows[0][0].1.last().map(String::as_str),
        Some("keyword.other")
    );
    assert_eq!(leaf_of("fixed4"), "storage.type.basic.shaderlab");
    assert_eq!(leaf_of("float4"), "storage.type.basic.hlsl");
    assert_eq!(leaf_of("mul"), "support.function.hlsl");
    assert_eq!(
        leaf_of("UNITY_MATRIX_MVP"),
        "support.variable.transformations.shaderlab"
    );
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("keyword.other")
    );
}

#[test]
fn hlsl_wins_a_tie_with_a_unity_word() {
    let rows = lex(indoc! {"
        CGPROGRAM
        x = fixed4(1);
        ENDCG
    "});
    let fixed = rows[1]
        .iter()
        .find(|(text, _)| text == "fixed4")
        .map(|(_, scopes)| scopes.last().cloned());
    assert_eq!(fixed, Some(Some("support.function.hlsl".to_owned())));
}

#[test]
fn a_block_ends_mid_line_but_not_in_an_hlsl_comment() {
    let rows = lex(indoc! {"
        HLSLPROGRAM
        /* ENDHLSL */ float x; ENDHLSL Pass
    "});
    let comment = rows[1].iter().find(|(text, _)| text == " ENDHLSL ");
    assert_eq!(
        comment.and_then(|(_, scopes)| scopes.last().cloned()),
        Some("comment.line.block.hlsl".to_owned())
    );
    assert_eq!(
        scope_of("x ENDHLSL Pass", "Pass"),
        "storage.type.structure.shaderlab"
    );
    let end = rows[1].iter().find(|(text, _)| text == "ENDHLSL");
    assert_eq!(
        end.and_then(|(_, scopes)| scopes.last().cloned()),
        Some("keyword.other".to_owned())
    );
    let pass = rows[1].iter().find(|(text, _)| text == "Pass");
    assert_eq!(
        pass.and_then(|(_, scopes)| scopes.last().cloned()),
        Some("storage.type.structure.shaderlab".to_owned())
    );
}

#[test]
fn words_outside_a_block_match_in_any_case() {
    let text = "ZWRITE off Cull BACK Blend srcalpha OneMinusSrcAlpha";
    assert_eq!(leaf(text, "ZWRITE"), "support.type.propertyname.shaderlab");
    assert_eq!(
        leaf(text, "off"),
        "support.constant.property-value.shaderlab"
    );
    assert_eq!(
        leaf(text, "BACK"),
        "support.constant.property-value.shaderlab"
    );
    assert_eq!(
        leaf(text, "srcalpha"),
        "support.constant.property-value.blendfactors.shaderlab"
    );
}

#[test]
fn a_bracketed_name_is_a_reference_unless_a_declaration_follows() {
    assert_eq!(
        leaf("Cull [_Cull]", "[_Cull]"),
        "support.variable.reference.shaderlab"
    );
    let rows = lex("[Toggle] _A (\"A\", Float) = 0");
    assert_eq!(rows[0][0].0, "[");
    assert_eq!(
        rows[0][0].1.last().map(String::as_str),
        Some("meta.attribute.shaderlab")
    );
    assert_eq!(
        leaf("[Toggle] _A (\"A\", Float) = 0", "Toggle"),
        "support.type.attributename.shaderlab"
    );
    assert_eq!(
        leaf("[Toggle] _A (\"A\", Float) = 0", "_A ("),
        "support.variable.declaration.shaderlab"
    );
}

#[test]
fn numbers_and_strings_follow_the_grammar() {
    assert_eq!(leaf("Offset 1.5x", "1."), "constant.numeric.shaderlab");
    assert_eq!(leaf("Offset 1. 2", "1"), "constant.numeric.shaderlab");
    assert_eq!(scope_of(r#"Name "a\"x" Pass"#, "x\""), "");
}

#[cfg(feature = "json")]
mod probe {
    use crate::shaderlab::engine::ShaderLab;
    use crate::shaderlab::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with ShaderLab's matchers
    /// (see [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<ShaderLab, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
