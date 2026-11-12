use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its innermost scope, or an empty
/// string where the token has only the document's scope.
type Row = Vec<(String, String)>;

/// The document's scope, which every token carries.
const SOURCE: &str = "markdown.mermaid.codeblock";

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::Mermaid);
    let mut state = State::default();
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let leaf = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .find(|scope| scope != SOURCE)
                .unwrap_or_default();
            row.push((line[start..token.end()].to_owned(), leaf));
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
}

#[test]
fn the_names_are_known() {
    for name in ["mermaid", "mmd"] {
        assert_eq!(Language::from_name(name), Some(Language::Mermaid));
    }
}

#[test]
fn a_class_relation_gives_its_arrow_back_from_the_first_name() {
    let text = indoc! {r#"
        classDiagram
          A-->B : uses
          Animal "1" --o "*" Duck
    "#};
    assert_eq!(leaf(text, "A"), "entity.name.type.class.mermaid");
    assert_eq!(leaf(text, "-->"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "B"), "entity.name.type.class.mermaid");
    assert_eq!(leaf(text, "uses"), "string");
    assert_eq!(leaf(text, "\"1\""), "keyword.control.mermaid");
    assert_eq!(leaf(text, "--o"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "Duck"), "entity.name.type.class.mermaid");
}

#[test]
fn members_added_from_outside_a_class_name_their_parts() {
    let text = indoc! {"
        classDiagram
          Animal : +isMammal()
          Animal : +int age
    "};
    assert_eq!(leaf(text, "+"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "isMammal"), "entity.name.function.mermaid");
    assert_eq!(leaf(text, "("), "punctuation.parenthesis.open.mermaid");
    assert_eq!(leaf(text, ")"), "punctuation.parenthesis.closed.mermaid");
    assert_eq!(leaf(text, "int"), "storage.type.mermaid");
    assert_eq!(leaf(text, "age"), "entity.name.variable.field.mermaid");
}

#[test]
fn a_class_body_names_fields_methods_and_generics() {
    let text = indoc! {"
        classDiagram
          class Duck~T~ {
            +String beakColor
            +swim(int x) bool
            -List~int~ items
          }
    "};
    assert_eq!(leaf(text, "Duck"), "entity.name.type.class.mermaid");
    assert_eq!(
        leaf(text, "~"),
        "punctuation.definition.typeparameters.begin.mermaid"
    );
    assert_eq!(leaf(text, "T"), "storage.type.mermaid");
    assert_eq!(
        leaf(text, "beakColor"),
        "entity.name.variable.field.mermaid"
    );
    assert_eq!(leaf(text, "swim"), "entity.name.function.mermaid");
    assert_eq!(leaf(text, "x"), "entity.name.variable.parameter.mermaid");
    assert_eq!(leaf(text, "bool"), "storage.type.mermaid");
    assert_eq!(leaf(text, "List"), "storage.type.mermaid");
    assert_eq!(leaf(text, "items"), "entity.name.variable.field.mermaid");
}

#[test]
fn flowchart_links_take_their_text_and_labels() {
    let text = indoc! {"
        graph TD
          A[Start] -->|yes| B(Round)
          B -- text --> C{Rhombus}
          C -.-> D((Circle))
    "};
    assert_eq!(leaf(text, "TD"), "entity.name.function.mermaid");
    assert_eq!(leaf(text, "Start"), "string");
    assert_eq!(leaf(text, "-->|"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "yes"), "string");
    assert_eq!(leaf(text, "B "), "variable");
    assert_eq!(leaf(text, " text "), "string");
    assert_eq!(leaf(text, "-.->"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "(("), "keyword.control.mermaid");
    assert_eq!(leaf(text, "Circle"), "string");
}

#[test]
fn a_click_gives_the_space_before_its_tooltip_back() {
    let text = indoc! {r#"
        graph TD
          click A callback "tip"
          click B "https://x"
    "#};
    assert_eq!(leaf(text, "A "), "variable");
    assert_eq!(leaf(text, "callback"), "variable");
    assert_eq!(leaf(text, "\"tip\""), "string");
    assert_eq!(leaf(text, "B"), "variable");
    assert_eq!(leaf(text, "\"https://x\""), "string");
}

/// The innermost scope of each byte of `row`, as `lex` gives it.
fn byte_leaves(row: &[(String, String)]) -> Vec<&str> {
    row.iter()
        .flat_map(|(text, leaf)| std::iter::repeat_n(leaf.as_str(), text.len()))
        .collect()
}

/// A click's tooltip, `("*.*")`, ends at the line's last `"`, as in Shiki.
/// The text after it is left to the other patterns, and a click with no `"`
/// after its name is no click.
/// Each row is the text and scope of Shiki 4.6.0's tokens.
#[test]
fn a_clicks_tooltip_ends_at_its_last_quote() {
    let lines = [
        "graph TD",
        r#"  click B "https://example.com"\"#,
        r#"  click B "a" x\"#,
        "  click B x",
        r#"  click B "a b"#,
    ];
    let rows = lex(&lines.join("\n"));
    let key = "keyword.control.mermaid";
    let lead = [
        ("  ", ""),
        ("click", key),
        (" ", ""),
        ("B", "variable"),
        (" ", ""),
    ];
    let want: [Vec<(&str, &str)>; 4] = [
        [
            &lead[..],
            &[("\"https://example.com\"", "string"), ("\\", "")],
        ]
        .concat(),
        [
            &lead[..],
            &[
                ("\"a\"", "string"),
                (" ", ""),
                ("x", "variable"),
                ("\\", ""),
            ],
        ]
        .concat(),
        vec![
            ("  ", ""),
            ("click ", "variable"),
            ("B ", "variable"),
            ("x", "variable"),
        ],
        [
            &lead[..],
            &[("\"", "string"), ("a ", "variable"), ("b", "variable")],
        ]
        .concat(),
    ];
    for (row, want) in rows[1..].iter().zip(want) {
        let want: Vec<&str> = want
            .iter()
            .flat_map(|(text, leaf)| std::iter::repeat_n(*leaf, text.len()))
            .collect();
        assert_eq!(byte_leaves(row), want, "{row:?}");
    }
}

#[test]
fn a_node_name_that_ends_in_a_dash_gives_it_back() {
    let rows = lex(indoc! {"
        graph TD
          A-x E
    "});
    let texts: Vec<_> = rows[1].iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(texts, ["  ", "A-x ", "E"]);
    assert_eq!(rows[1][1].1, "variable");
}

#[test]
fn classes_and_class_definitions_name_their_parts() {
    let text = indoc! {"
        graph TD
          class A,B fancy
          classDef fancy fill:#f9f
    "};
    assert_eq!(leaf(text, "A,B"), "variable");
    assert_eq!(leaf(text, "fancy"), "string");
    assert_eq!(leaf(text, "fill:#f9f"), "string");
}

#[test]
fn sequence_participants_messages_and_notes_name_their_parts() {
    let text = indoc! {"
        sequenceDiagram
          participant B as Bob
          Alice->>Bob: Hello
          Note right of Bob: thinks
          Note over Alice,Bob: both
          option x
    "};
    assert_eq!(leaf(text, "B"), "variable");
    assert_eq!(leaf(text, "as"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "Bob"), "string");
    assert_eq!(leaf(text, "Alice"), "variable");
    assert_eq!(leaf(text, "->>"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "Hello"), "string");
    assert_eq!(leaf(text, "right of"), "entity.name.function.mermaid");
    assert_eq!(leaf(text, ","), "keyword.control.mermaid");
    assert_eq!(leaf(text, "option"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "x"), "string");
}

#[test]
fn a_participant_keeps_its_name_up_to_the_line_feed() {
    let text = indoc! {"
        sequenceDiagram
          participant Alice
    "};
    assert_eq!(leaf(text, "Alice"), "variable");
}

#[test]
fn a_state_transition_gives_its_arrow_back_from_the_name() {
    let text = indoc! {r#"
        stateDiagram-v2
          [*] --> Still
          Still-->Moving : go
          state "Long name" as L {
          }
          note right of Still : a note
          S1 : description
    "#};
    assert_eq!(leaf(text, "[*]"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "Still"), "variable");
    assert_eq!(leaf(text, "Moving"), "variable");
    assert_eq!(leaf(text, "go"), "string");
    assert_eq!(leaf(text, "\"Long name\""), "string");
    assert_eq!(leaf(text, "note right of"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "a note"), "string");
    assert_eq!(leaf(text, "description"), "string");
}

#[test]
fn an_entity_relation_gives_a_cardinality_back_from_the_name() {
    let text = indoc! {r#"
        erDiagram
          CUSTOMER ||--o{ ORDER : places
          A1+--1 B : x
          CUSTOMER {
            string name PK "the name"
          }
    "#};
    assert_eq!(leaf(text, "||--o{"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "places"), "string");
    assert_eq!(leaf(text, "A"), "variable");
    assert_eq!(leaf(text, "1+--1"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "string"), "storage.type.mermaid");
    assert_eq!(leaf(text, "PK"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "\"the name\""), "string");
}

#[test]
fn a_gantt_task_runs_to_its_last_colon() {
    let text = indoc! {"
        gantt
          dateFormat YYYY-MM-DD
          excludes weekends
          Task one :done, a1, 2014-01-01, 30d
    "};
    assert_eq!(leaf(text, "YYYY-MM-DD"), "entity.name.function.mermaid");
    assert_eq!(leaf(text, "weekends"), "string");
    assert_eq!(leaf(text, " Task one "), "string");
    assert_eq!(leaf(text, "done"), "entity.name.function.mermaid");
}

#[test]
fn a_mind_map_shape_gives_a_parenthesis_back_to_close_itself() {
    let rows = lex(indoc! {"
        mindmap
          root((center))
            A((
            ::icon(fa fa-book)
    "});
    let row = |i: usize| -> Vec<(&str, &str)> {
        rows[i]
            .iter()
            .map(|(t, s)| (t.as_str(), s.as_str()))
            .collect()
    };
    assert_eq!(
        row(1),
        [
            ("  root", "variable"),
            ("((", "keyword.control.mermaid"),
            ("center", "string"),
            ("))", "keyword.control.mermaid"),
        ]
    );
    assert_eq!(
        row(2),
        [
            ("    A", "variable"),
            ("(", "keyword.control.mermaid"),
            ("(", "keyword.control.mermaid")
        ]
    );
    assert_eq!(
        row(3),
        [
            ("    ::icon", "keyword.control.mermaid"),
            ("(", "punctuation.parenthesis.open.mermaid"),
            ("fa fa-book", "string"),
            (")", "punctuation.parenthesis.close.mermaid"),
        ]
    );
}

#[test]
fn git_graph_options_name_their_values() {
    let text = indoc! {r#"
        gitGraph
          commit id: "x" tag: "v1"
          branch dev order: 2
    "#};
    assert_eq!(leaf(text, "commit"), "keyword.control.mermaid");
    assert_eq!(leaf(text, "\"v1\""), "string");
    assert_eq!(leaf(text, "dev"), "variable");
    assert_eq!(leaf(text, "2"), "constant.numeric.decimal.mermaid");
}

#[test]
fn a_document_is_plain_until_a_diagram_starts_and_a_fence_ends_it() {
    let text = indoc! {"
        hello A-->B
        graph TD
        sequenceDiagram
        ```
        sequenceDiagram
    "};
    let rows = lex(text);
    assert_eq!(rows[0], [("hello A-->B".to_owned(), String::new())]);
    assert_eq!(leaf(text, "graph"), "keyword.control.mermaid");
    assert_eq!(rows[2][0].1, "variable");
    assert_eq!(rows[4][0].1, "keyword.control.mermaid");
}

#[test]
fn keywords_fold_case_but_diagram_words_do_not() {
    assert_eq!(lex("GRAPH TD")[0], [("GRAPH TD".to_owned(), String::new())]);
    let classes = indoc! {"
        classDiagram
          CLASS Foo
          cla\u{17f}s Bar
    "};
    assert_eq!(leaf(classes, "CLASS"), "keyword.control.mermaid");
    assert_eq!(leaf(classes, "cla\u{17f}s"), "keyword.control.mermaid");
    let click = indoc! {"
        graph TD
          clic\u{212a} A cb \"x\"
    "};
    assert_eq!(leaf(click, "clic\u{212a}"), "keyword.control.mermaid");
}

#[cfg(feature = "json")]
mod probe {
    use crate::mermaid::engine::Mermaid;
    use crate::mermaid::grammar::Pattern;
    use crate::test_support::probe::{answer_cases, lexer};

    /// Answers the cases of `oracle/onig.ts` with Mermaid's matchers (see
    /// [`answer_cases`]).
    #[test]
    #[ignore = "a tool: it needs a cases file"]
    fn matchers_answer_the_probe_cases() {
        answer_cases(&Pattern::ALL, |arena, pattern, case| {
            lexer::<Mermaid, _>(arena, case, |lexer| {
                lexer.probe_matcher(case.p, lexer.g, |lexer, p| lexer.matcher(pattern, p))
            })
        });
    }
}
