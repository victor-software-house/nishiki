use crate::Language;
use crate::test_support::{self, LeafRow, leaf_in, merge_by_leaf};

fn lex(line: &str) -> LeafRow {
    merge_by_leaf(test_support::lex(Language::Proto, line)).remove(0)
}

#[test]
fn the_names_are_known() {
    assert_eq!(Language::from_name("proto"), Some(Language::Proto));
    assert_eq!(Language::from_name("protobuf"), Some(Language::Proto));
}

#[test]
fn the_file_statements_name_their_parts() {
    let row = lex(r#"syntax = "proto3";"#);
    assert_eq!(leaf_in(&row, "syntax"), "keyword.other.proto");
    assert_eq!(
        leaf_in(&row, r#""proto3""#),
        "string.quoted.double.proto.syntax"
    );
    let row = lex(r#"import public "a/b.proto";"#);
    assert_eq!(leaf_in(&row, "public"), "keyword.other.proto");
    assert_eq!(
        leaf_in(&row, r#""a/b.proto""#),
        "string.quoted.double.proto.import"
    );
}

/// The label is tried first and then left out, so `required x = 1` takes
/// `required` as the type.
#[test]
fn a_field_without_a_type_takes_its_label_as_the_type() {
    let row = lex("message Foo { required x = 1; repeated .a.B y = 2 [deprecated = true]; }");
    assert_eq!(leaf_in(&row, "Foo"), "entity.name.class.message.proto");
    assert_eq!(leaf_in(&row, "required"), "storage.type.proto");
    assert_eq!(leaf_in(&row, "repeated"), "storage.modifier.proto");
    assert_eq!(leaf_in(&row, ".a.B"), "storage.type.proto");
    assert_eq!(leaf_in(&row, "deprecated"), "support.other.proto");
    assert_eq!(leaf_in(&row, "true"), "constant.language.proto");
}

#[test]
fn a_map_field_names_its_types() {
    let row = lex("message M { map<string, int32> m = 3; }");
    assert_eq!(
        leaf_in(&row, "<"),
        "punctuation.definition.typeparameters.begin.proto"
    );
    assert_eq!(leaf_in(&row, "int32"), "storage.type.proto");
    assert_eq!(leaf_in(&row, "m"), "variable.other.proto");
}

#[test]
fn enums_and_services_name_their_members() {
    let row = lex("enum E { A = 0; B = -0x1F; }");
    assert_eq!(leaf_in(&row, "E"), "entity.name.class.proto");
    assert_eq!(leaf_in(&row, "-0x1F"), "constant.numeric.proto");
    let row = lex("service S { rpc Get (stream Req) returns (Resp); }");
    assert_eq!(leaf_in(&row, "Get"), "entity.name.function");
    assert_eq!(leaf_in(&row, "stream"), "keyword.other.proto");
    assert_eq!(leaf_in(&row, "Resp"), "entity.name.class.proto");
}

/// The fraction's digits give themselves back to reach a word boundary
/// after the dot.
#[test]
fn a_number_before_a_letter_ends_at_its_dot() {
    let row = lex("option (my.opt).x = 1.5x;");
    assert_eq!(leaf_in(&row, "option"), "keyword.other.proto");
    assert_eq!(leaf_in(&row, "1."), "constant.numeric.proto");
    assert_eq!(leaf_in(&row, "5x"), "source.proto");
}

#[cfg(feature = "json")]
mod probe {
    use crate::proto::engine::Proto;
    use crate::proto::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Protobuf's matchers (see
    /// [`answer_cases`]). The text of a case is the start of a document.
    /// `\A` then holds at its first byte, as it does for Oniguruma.
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Proto, _>(arena, case, |lexer| {
                lexer.first_line = true;
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}

mod generated {
    use crate::proto::engine::Proto;
    use crate::proto::grammar::Pattern;
    use crate::proto::matchers::EXPRESSIONS;
    use crate::test_support::differential::read_groups;

    /// The table lists every pattern once, in the enum's order.
    #[test]
    fn the_expressions_cover_every_pattern() {
        let listed = EXPRESSIONS.map(|expression| expression.pattern);
        assert_eq!(format!("{listed:?}"), format!("{:?}", Pattern::ALL));
    }

    /// Each matcher records exactly the groups the lexer reads after its
    /// pattern matches.
    /// A group it left out would reach the lexer empty.
    #[test]
    fn each_matcher_records_the_groups_the_lexer_reads() {
        for expression in &EXPRESSIONS {
            assert_eq!(
                expression.records,
                read_groups::<Proto>(expression.pattern),
                "{:?}",
                expression.pattern
            );
        }
    }

    /// Each generated matcher matches what the regex engine matches.
    #[cfg(feature = "_textmate")]
    #[test]
    fn the_matchers_match_the_regex_engine() {
        let compared =
            crate::test_support::differential::differential::<Proto>("proto", &EXPRESSIONS);
        assert!(compared > 0, "the differential test compared nothing");
    }

    /// On `xoptional foo bar = 1`, the field pattern fails at 0 and matches
    /// at 1.
    /// The failure at 0 leaves a skip that stops before 1.
    /// There the keyword `optional` can start the prefix
    /// (`field-pattern-fixes` design, section 3.3).
    #[test]
    fn a_skip_keeps_a_match_inside_the_failed_run() {
        use crate::arena::Arena;
        use crate::test_support::probe::{After, Anchor, Case, lexer};

        let text = "xoptional foo bar = 1";
        let case = Case {
            text,
            p: 0,
            anchor: Anchor::Free,
            after: After::LineFeed,
        };
        let ends = lexer::<Proto, _>(&mut Arena::new(None), &case, |lexer| {
            [0, 1].map(|p| {
                lexer
                    .probe_matcher(p, false, |lexer, p| lexer.matcher(Pattern::Field, p))
                    .map(|(end, _)| end)
            })
        });
        assert_eq!(ends, [None, Some(text.len())]);
    }
}
