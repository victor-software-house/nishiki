use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf, scopes_of};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::PostCss, line)).remove(0)
}

#[test]
fn the_name_is_known() {
    assert_eq!(Language::from_name("postcss"), Some(Language::PostCss));
}

#[test]
fn a_rule_names_its_selector_and_properties() {
    let row = lex(".card > a:hover { color: #fff; margin: 0 2px !important; }");
    assert_eq!(
        leaf_in(&row, ".card"),
        "entity.other.attribute-name.class.css.postcss"
    );
    assert_eq!(
        leaf_in(&row, ":hover"),
        "entity.other.attribute-name.pseudo-class.css.postcss"
    );
    assert_eq!(
        leaf_in(&row, "color"),
        "support.type.property-name.css.postcss"
    );
    assert_eq!(
        leaf_in(&row, "#fff"),
        "constant.other.color.rgb-value.css.postcss"
    );
    assert_eq!(leaf_in(&row, "px"), "keyword.other.unit.css.postcss");
    assert_eq!(
        leaf_in(&row, "!important"),
        "keyword.other.important.css.postcss"
    );
}

#[test]
fn variables_and_includes_take_their_scopes() {
    let row = lex("$gap: 4px; --brand: red;");
    assert_eq!(leaf_in(&row, "$gap"), "variable.parameter.postcss");
    assert_eq!(leaf_in(&row, "--brand"), "variable.parameter.postcss");
    let row = lex("@include icon;");
    assert_eq!(
        leaf_in(&row, "@include"),
        "keyword.control.at-rule.css.postcss"
    );
}

/// The property-value rule's name holds a comma, which vscode-textmate keeps
/// in the first of its two scopes.
#[test]
fn a_property_value_keeps_the_comma_in_its_first_scope() {
    assert_eq!(
        scopes_of(Language::PostCss, "a { display: block }", "block"),
        [
            "source.css.postcss",
            "meta.property-list.css.postcss",
            "meta.property-value.css.postcss,",
            "support.constant.property-value.css.postcss",
        ]
    );
}

/// A `</style` tag, where a host such as Svelte ends PostCSS.
fn style_end(bytes: &[u8], p: usize) -> bool {
    bytes
        .get(p..)
        .is_some_and(|rest| rest.starts_with(b"</style"))
}

/// Embedded, PostCSS halts at the host's end once only its root rule is open.
#[test]
fn an_embedded_line_halts_at_the_hosts_end() {
    let embedded = test_support::Embedded {
        start: super::start,
        lex: super::lex,
        lex_until: super::lex_until,
    };
    let line = "a { color: red; }</style>";
    let rows = test_support::lex_embedded(
        embedded,
        &["source.svelte"],
        &[line],
        Some((&style_end, None)),
    );
    assert_eq!(rows[0].0, line.find("</style"));
}

#[cfg(feature = "json")]
mod probe {
    use crate::postcss::engine::PostCss;
    use crate::postcss::grammar::Pat;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with PostCSS's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pat::ALL, |arena, pat, case| {
            lexer::<PostCss, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pat, p))
            })
        });
    }
}
