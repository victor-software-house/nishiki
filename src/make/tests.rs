use crate::{Highlighter, Language, Limit, Limits, State, Work};

/// One lexed line: each token's text and its scope names, outermost first,
/// without the document's `source.makefile`.
type Row = Vec<(String, Vec<String>)>;

fn lex_with(highlighter: &mut Highlighter, text: &str) -> Vec<Row> {
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
            scopes.remove(0);
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "the tokens cover the line {line:?}");
        rows.push(row);
    }
    rows
}

fn lex(text: &str) -> Vec<Row> {
    lex_with(&mut Highlighter::new(Language::Make), text)
}

/// The scopes of the first token whose text is `token`, outermost first,
/// without `source.makefile`.
fn scopes_of(text: &str, token: &str) -> Vec<String> {
    lex(text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

fn names(scopes: &[&str]) -> Vec<String> {
    scopes.iter().map(ToString::to_string).collect()
}

#[test]
fn an_assignment_names_its_variable_and_its_operator() {
    assert_eq!(
        scopes_of("CC := gcc", "CC"),
        names(&["variable.other.makefile"])
    );
    assert_eq!(
        scopes_of("CC := gcc", ":="),
        names(&["punctuation.separator.key-value.makefile"])
    );
    assert_eq!(
        scopes_of("a+=b", "+="),
        names(&["punctuation.separator.key-value.makefile"]),
        "the name gives back the plus"
    );
    assert_eq!(scopes_of("a+=b", "a"), names(&["variable.other.makefile"]));
}

#[test]
fn a_variable_reference_has_its_own_punctuation_and_a_name() {
    let rows = lex("x = $(CC) $@");
    let row = &rows[0];
    assert!(row.iter().any(|(text, scopes)| text == "$("
        && scopes
            == &names(&[
                "string.interpolated.makefile",
                "punctuation.definition.variable.makefile"
            ])));
    assert!(
        row.iter().any(|(text, scopes)| text == ")"
            && scopes
                == &names(&[
                    "string.interpolated.makefile",
                    "punctuation.definition.variable.makefile"
                ])),
        "the closing parenthesis has the same punctuation"
    );
    assert_eq!(
        scopes_of("x = $@", "$@"),
        names(&["variable.language.makefile"])
    );
}

#[test]
fn a_function_call_scopes_its_name_by_the_name() {
    assert_eq!(
        scopes_of("x = $(filter-out a,b)", "filter-out"),
        names(&[
            "string.interpolated.makefile",
            "meta.scope.function-call.makefile",
            "support.function.filter-out.makefile"
        ])
    );
    assert_eq!(
        scopes_of("x = $(filter-out a,b)", ","),
        names(&[
            "string.interpolated.makefile",
            "meta.scope.function-call.makefile",
            "punctuation.separator.delim\x65ter.comma.makefile"
        ])
    );
    assert_eq!(
        scopes_of("x = $(filter-x a)", "filter-x a"),
        names(&["string.interpolated.makefile", "variable.other.makefile"]),
        "a word that is no function is a variable name"
    );
}

#[test]
fn a_builtin_variable_is_a_language_variable_only_before_the_closing_parenthesis() {
    assert_eq!(
        scopes_of("x = $(MAKE)", "MAKE"),
        names(&["string.interpolated.makefile", "variable.language.makefile"])
    );
    assert_eq!(
        scopes_of("x = $(MAKE )", "MAKE"),
        names(&["string.interpolated.makefile", "variable.language.makefile"])
    );
    assert_eq!(
        scopes_of("x = $(MAKEX)", "MAKEX"),
        names(&["string.interpolated.makefile", "variable.other.makefile"])
    );
}

#[test]
fn recipe_flags_make_the_name_of_their_scope() {
    let rows = lex("\t@-echo hi");
    assert_eq!(
        rows[0][1],
        (
            "@-".to_owned(),
            names(&["meta.scope.recipe.makefile", "keyword.control.@-.makefile"])
        )
    );
}

#[test]
fn a_dollar_at_the_end_of_a_recipe_line_is_no_variable() {
    let rows = lex("\techo $");
    assert_eq!(
        rows[0].last().map(|(_, scopes)| scopes.clone()),
        Some(names(&["meta.scope.recipe.makefile"]))
    );
    let rows = lex("x = $");
    assert_eq!(
        rows[0].last().map(|(_, scopes)| scopes.clone()),
        Some(names(&["variable.language.makefile"]))
    );
}

#[test]
fn a_special_target_is_scoped_without_its_dot_in_the_name() {
    assert_eq!(
        scopes_of(".PHONY: all", ".PHONY"),
        names(&[
            "meta.scope.target.makefile",
            "support.function.target.PHONY.makefile"
        ])
    );
    assert_eq!(
        scopes_of(".PHONY x: all", ".PHONY"),
        names(&[
            "meta.scope.target.makefile",
            "entity.name.function.target.makefile"
        ]),
        "a special target stands alone"
    );
}

#[test]
fn a_target_ends_with_its_line_unless_a_backslash_continues_it() {
    let rows = lex("all: a \\\n b\nx = 1");
    assert_eq!(
        rows[1][0].1,
        names(&[
            "meta.scope.target.makefile",
            "meta.scope.prerequisites.makefile"
        ])
    );
    assert_eq!(rows[2][0].1, names(&["variable.other.makefile"]));
}

#[test]
fn a_define_runs_to_its_endef_and_endef_has_the_scope_of_override() {
    let rows = lex("define X\n$(a)\nendef\ny = 1");
    assert_eq!(
        rows[1][0].1,
        names(&[
            "meta.scope.conditional.makefile",
            "string.interpolated.makefile",
            "punctuation.definition.variable.makefile"
        ])
    );
    assert_eq!(
        rows[2][0],
        (
            "endef".to_owned(),
            names(&[
                "meta.scope.conditional.makefile",
                "keyword.control.override.makefile"
            ])
        )
    );
    assert_eq!(rows[3][0].1, names(&["variable.other.makefile"]));
}

#[test]
fn a_conditional_runs_to_its_endif_with_its_branches() {
    let rows = lex("ifeq ($(A),b)\nx = 1\nelse\nx = 2\nendif\nz = 3");
    assert_eq!(
        rows[0][0],
        (
            "ifeq".to_owned(),
            names(&[
                "meta.scope.conditional.makefile",
                "keyword.control.ifeq.makefile"
            ])
        )
    );
    assert_eq!(
        rows[2][0],
        (
            "else".to_owned(),
            names(&[
                "meta.scope.conditional.makefile",
                "keyword.control.else.makefile"
            ])
        )
    );
    assert_eq!(
        rows[4][0].1,
        names(&[
            "meta.scope.conditional.makefile",
            "keyword.control.endif.makefile"
        ])
    );
    assert_eq!(rows[5][0].1, names(&["variable.other.makefile"]));
}

#[test]
fn a_comment_may_continue_over_lines_and_an_escaped_hash_starts_none() {
    let rows = lex("x = 1 # a \\\nb\ny = 2");
    assert_eq!(
        rows[1][0].1,
        names(&["comment.line.number-sign.makefile"]),
        "the comment goes on"
    );
    assert_eq!(rows[2][0].1, names(&["variable.other.makefile"]));
    let rows = lex("a: b \\# c");
    assert!(
        rows[0]
            .iter()
            .all(|(_, scopes)| !scopes.iter().any(|s| s.starts_with("comment"))),
        "an escaped hash is no comment"
    );
}

#[test]
fn an_unclosed_reference_takes_the_line_feed_and_leaves_the_assignment_open() {
    let rows = lex("x = $(\nz = 1");
    assert_eq!(
        rows[1],
        vec![("z = 1".to_owned(), Vec::<String>::new())],
        "the reference ended at the line feed that the assignment waits for"
    );
}

#[test]
fn a_fence_embeds_a_makefile_with_its_start_of_line_anchor() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let rows = lex_with(&mut highlighter, "```makefile\n\tFOO = bar\n```");
    assert_eq!(
        rows[1][1],
        (
            "FOO".to_owned(),
            names(&[
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.makefile",
                "variable.other.makefile"
            ])
        ),
        "in a fence `\\G` is at the start of every line, so a tab starts an assignment"
    );
}

#[test]
fn a_fence_by_any_of_the_grammars_names_embeds_a_makefile() {
    for info in ["makefile", "Makefile", "GNUmakefile", "OCamlMakefile"] {
        let mut highlighter = Highlighter::new(Language::Markdown);
        let rows = lex_with(&mut highlighter, &format!("```{info}\nA = b"));
        assert!(
            rows[1]
                .iter()
                .any(|(_, scopes)| scopes.iter().any(|s| s == "meta.embedded.block.makefile")),
            "{info}"
        );
    }
}

#[test]
fn running_out_of_work_leaves_the_rest_of_the_line_in_the_open_rules() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 8,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Make, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = format!("x = {}", "$(a) ".repeat(50));
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(Limit::Work));
    assert_eq!(tokens[tokens.len() - 1].end(), line.len());
}
