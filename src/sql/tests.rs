use crate::{Highlighter, Language, Limit, Limits, State, Work};

/// One lexed line: each token's text and its scope names, outermost first,
/// without the document's `source.sql`.
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
    lex_with(&mut Highlighter::new(Language::Sql), text)
}

/// The scopes of the first token whose text is `token`, outermost first,
/// without `source.sql`.
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
fn a_query_keyword_is_one_token_with_its_words() {
    assert_eq!(
        scopes_of("SELECT DISTINCT a", "SELECT DISTINCT"),
        names(&["keyword.other.DML.sql"])
    );
    assert_eq!(
        scopes_of("select  all a", "select  all"),
        names(&["keyword.other.DML.sql"])
    );
    assert_eq!(
        scopes_of("a left outer join b", "left outer join"),
        names(&["keyword.other.DML.sql"])
    );
    assert_eq!(
        scopes_of("select allx", "select"),
        names(&["keyword.other.DML.sql"]),
        "`all` gives back when a word goes on"
    );
}

#[test]
fn a_case_fold_of_the_unicode_engine_reads_a_long_s_as_s() {
    assert_eq!(
        scopes_of("\u{17f}elect 1", "\u{17f}elect"),
        names(&["keyword.other.DML.sql"])
    );
}

#[test]
fn a_type_has_its_size_numbered_apart() {
    let rows = lex("c varchar(10), d numeric(5,2)");
    let find = |text: &str| {
        rows[0]
            .iter()
            .find(|(t, _)| t == text)
            .map(|(_, scopes)| scopes.clone())
    };
    assert_eq!(find("varchar"), Some(names(&["storage.type.sql"])));
    assert_eq!(find("10"), Some(names(&["constant.numeric.sql"])));
    assert_eq!(find("numeric"), Some(names(&["storage.type.sql"])));
    assert_eq!(find("2"), Some(names(&["constant.numeric.sql"])));
    assert_eq!(
        scopes_of("t timestamp with time zone", " with time zone"),
        names(&["storage.type.sql"])
    );
}

#[test]
fn bit_varying_is_only_bit_because_the_plain_types_come_first() {
    let rows = lex("bit varying(5)");
    assert_eq!(rows[0][0], ("bit".to_owned(), names(&["storage.type.sql"])));
    assert_eq!(rows[0][1].1, Vec::<String>::new());
}

#[test]
fn create_names_what_it_makes_and_gives_back_a_type_when_no_name_follows() {
    let rows = lex("create table foo");
    assert_eq!(
        rows[0][0],
        (
            "create".to_owned(),
            names(&["meta.create.sql", "keyword.other.create.sql"])
        )
    );
    assert_eq!(
        scopes_of("create table foo", "foo"),
        names(&["meta.create.sql", "entity.name.function.sql"])
    );
    assert_eq!(
        scopes_of("create operator class\u{a0}[x", "class"),
        names(&["meta.create.sql", "entity.name.function.sql"]),
        "`operator class` has no name after it, so `operator` takes `class` as one"
    );
    assert_eq!(
        scopes_of("create table \"foo", "table"),
        names(&["keyword.other.sql"]),
        "an unclosed quote is no name"
    );
}

#[test]
fn drop_table_is_found_after_other_text_and_takes_cascade() {
    assert_eq!(
        scopes_of("x; drop table t cascade", " cascade"),
        names(&["meta.drop.sql", "keyword.other.cascade.sql"])
    );
    assert_eq!(
        scopes_of("drop tablespace t", "table"),
        names(&["meta.drop.sql", "keyword.other.sql"]),
        "`table` comes first in the alternation, so `space` is left over"
    );
    assert_eq!(
        scopes_of("drop tablespace t", "space t"),
        Vec::<String>::new()
    );
}

#[test]
fn operator_class_is_written_with_one_literal_space() {
    assert_eq!(
        scopes_of("drop operator class", "operator class"),
        names(&["meta.drop.sql", "keyword.other.sql"])
    );
    assert_eq!(
        scopes_of("drop operator  class", "operator"),
        names(&["meta.drop.sql", "keyword.other.sql"]),
        "with two spaces `operator` is the type and `class` is left over"
    );
    assert_eq!(
        scopes_of("drop operator  class", "  class"),
        Vec::<String>::new()
    );
}

#[test]
fn a_function_name_needs_its_parenthesis() {
    assert_eq!(
        scopes_of("select count(*)", "count"),
        names(&["support.function.aggregate.sql"])
    );
    assert_eq!(
        scopes_of("select count (*)", "count"),
        names(&["support.function.aggregate.sql"])
    );
    assert_eq!(scopes_of("select count", " count"), Vec::<String>::new());
    assert_eq!(
        scopes_of("select @@rowcount(", "rowcount"),
        names(&["support.function.globalvar.sql"])
    );
}

#[test]
fn a_hash_is_no_comment_and_a_double_dash_is() {
    let rows = lex("# a\n-- b");
    assert_eq!(rows[0][0].1, Vec::<String>::new());
    assert_eq!(
        rows[1][0],
        (
            "--".to_owned(),
            names(&[
                "comment.line.double-dash.sql",
                "punctuation.definition.comment.sql"
            ])
        )
    );
    assert_eq!(rows[1][1].1, names(&["comment.line.double-dash.sql"]));
}

#[test]
fn a_block_comment_nests_and_goes_on_across_lines() {
    let rows = lex("/* a /* b */\nstill */ x");
    assert_eq!(rows[1][0].1, names(&["comment.block"]));
    assert_eq!(rows[1].last().map(|(_, s)| s.clone()), Some(Vec::new()));
    assert_eq!(rows[0][1].1, names(&["comment.block"]));
    assert_eq!(rows[0][3].1, names(&["comment.block", "comment.block"]));
}

#[test]
fn a_string_closed_on_its_line_ignores_a_backslash_before_its_quote() {
    let rows = lex("'a\\'b'");
    let texts: Vec<&str> = rows[0].iter().map(|(text, _)| text.as_str()).collect();
    assert_eq!(texts, ["'", "a\\", "'", "b", "'"]);
    assert_eq!(rows[0][3].1, Vec::<String>::new());
    assert_eq!(
        rows[0][4].1,
        names(&[
            "string.quoted.single.sql",
            "punctuation.definition.string.begin.sql"
        ]),
        "the last quote opens a string that goes on"
    );
}

#[test]
fn a_string_that_goes_on_ends_on_a_later_line() {
    let rows = lex("select 'ab\ncd' x");
    assert_eq!(rows[1][0].1, names(&["string.quoted.single.sql"]));
    assert_eq!(
        rows[1][1].1,
        names(&[
            "string.quoted.single.sql",
            "punctuation.definition.string.end.sql"
        ])
    );
    let rows = lex("\"a#{b}c\"");
    assert_eq!(
        rows[0][3].1,
        names(&["string.quoted.double.sql", "string.interpolated.sql"]),
        "a double-quoted string with an interpolation is matched by the rule that has one"
    );
}

#[test]
fn a_bracket_closes_at_a_single_bracket_and_a_double_one_is_part_of_it() {
    let rows = lex("[a]]b] [c");
    assert_eq!(
        rows[0][2],
        (
            "]]".to_owned(),
            names(&["text.bracketed", "constant.character.bracket.sql"])
        )
    );
    assert_eq!(
        rows[0][4].1,
        names(&["text.bracketed", "punctuation.definition.bracket.end.sql"])
    );
    assert_eq!(rows[0][5].1, Vec::<String>::new());
}

#[test]
fn a_name_after_a_dot_is_a_table_even_when_it_is_a_number() {
    assert_eq!(
        scopes_of("dbo.users", "dbo"),
        names(&["constant.other.database-name.sql"])
    );
    assert_eq!(
        scopes_of("dbo.users", "users"),
        names(&["constant.other.table-name.sql"])
    );
    assert_eq!(
        scopes_of("a.1", "1"),
        names(&["constant.other.table-name.sql"])
    );
    assert_eq!(scopes_of("x 1.5", "1"), names(&["constant.numeric.sql"]));
}

#[test]
fn a_variable_and_an_empty_pair_of_parentheses() {
    assert_eq!(scopes_of("a @v b", "@v"), names(&["text.variable"]));
    assert_eq!(
        scopes_of("f ()", "("),
        names(&["meta.block.sql", "punctuation.section.scope.begin.sql"])
    );
}

#[test]
fn the_keyword_that_a_grammar_bug_extends_needs_literal_backslashes() {
    assert_eq!(
        scopes_of("create\\sor\\salter", "create\\sor\\salter"),
        names(&["keyword.other.sql"])
    );
    assert_eq!(
        scopes_of("priority x", "priority "),
        names(&["keyword.other.sql"])
    );
}

#[test]
#[cfg(feature = "lang-markdown")]
fn a_fence_embeds_sql_without_its_source_scope() {
    for info in ["sql", "ddl", "dml", "SQL"] {
        let mut highlighter = Highlighter::new(Language::Markdown);
        let rows = lex_with(&mut highlighter, &format!("```{info}\nselect 1"));
        assert_eq!(
            rows[1][0],
            (
                "select".to_owned(),
                names(&[
                    "markup.fenced_code.block.markdown",
                    "meta.embedded.block.sql",
                    "keyword.other.DML.sql"
                ])
            ),
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
    let mut highlighter = Highlighter::new_with_limits(Language::Sql, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let line = format!("select {}", "a.b, ".repeat(50));
    let report = highlighter.line(&line, &mut state, &mut tokens);
    assert!(report.reached(Limit::Work));
    assert_eq!(tokens[tokens.len() - 1].end(), line.len());
}

/// The matchers binary-search the word tables, which only works while each
/// is sorted and holds no word twice.
#[test]
fn word_tables_are_sorted() {
    use super::words::{FUNCTIONS, GLOBALS, KEYWORDS};
    let functions: Vec<&str> = FUNCTIONS.iter().map(|&(name, _)| name).collect();
    for (name, table) in [
        ("KEYWORDS", KEYWORDS),
        ("FUNCTIONS", functions.as_slice()),
        ("GLOBALS", GLOBALS),
    ] {
        for pair in table.windows(2) {
            assert!(pair[0] < pair[1], "{name}: {pair:?} is out of order");
        }
    }
}
