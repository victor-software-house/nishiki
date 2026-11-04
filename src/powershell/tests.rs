use indoc::indoc;

use crate::{Highlighter, Language, State};

/// One lexed line: each token's text and its scope names, outermost first.
type Row = Vec<(String, Vec<String>)>;

fn lex(text: &str) -> Vec<Row> {
    let mut highlighter = Highlighter::new(Language::PowerShell);
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

#[test]
fn the_names_are_known() {
    for name in ["powershell", "ps", "ps1", "pwsh"] {
        assert_eq!(Language::from_name(name), Some(Language::PowerShell));
    }
}

#[test]
fn a_function_names_its_scope_and_name() {
    let text = "function global:Get-Thing { param([Parameter(Mandatory = $true)][string]$Name) }";
    assert_eq!(leaf(text, "function"), "storage.type.powershell");
    assert_eq!(leaf(text, "global"), "storage.modifier.scope.powershell");
    assert_eq!(leaf(text, "Get-Thing"), "entity.name.function.powershell");
    assert_eq!(
        leaf(text, "Parameter"),
        "support.function.attribute.powershell"
    );
    assert_eq!(
        leaf(text, "Mandatory"),
        "variable.parameter.attribute.powershell"
    );
    assert_eq!(leaf(text, "true"), "constant.language.powershell");
    assert_eq!(leaf(text, "string"), "storage.type.powershell");
}

#[test]
fn a_pipeline_names_commands_variables_and_operators() {
    let text = "Get-ChildItem -Path $env:TEMP | Where-Object { $_.Length -gt 1kb }";
    assert_eq!(leaf(text, "Get-ChildItem"), "support.function.powershell");
    assert_eq!(leaf(text, "env:"), "support.variable.drive.powershell");
    assert_eq!(leaf(text, "_"), "support.variable.automatic.powershell");
    assert_eq!(leaf(text, ".Length"), "variable.other.member.powershell");
    assert_eq!(leaf(text, "-gt"), "keyword.operator.comparison.powershell");
    assert_eq!(leaf(text, "kb"), "keyword.other.powershell");
}

#[test]
fn a_hashtable_names_its_keys_and_numbers() {
    let text = r#"$x = @{ "a" = 1; b = 0x1Fl; c = 1.5mb }"#;
    assert_eq!(leaf(text, "b"), "variable.other.readwrite.powershell");
    assert_eq!(leaf(text, "0x1Fl"), "constant.numeric.hex.powershell");
    assert_eq!(leaf(text, "1.5"), "constant.numeric.integer.powershell");
    assert_eq!(leaf(text, "mb"), "keyword.other.powershell");
}

#[test]
fn a_string_interpolates_variables_and_substatements() {
    let text = r#""Hello $Name and $($x.a) `n ${env:Path}" -f 1"#;
    let rows = lex(text);
    let dollar = &rows[0][5].1;
    assert_eq!(
        dollar[dollar.len() - 3..],
        [
            "keyword.other.substatement.powershell",
            "punctuation.definition.substatement.powershell",
            "punctuation.section.embedded.substatement.begin.powershell",
        ]
    );
    assert_eq!(leaf(text, "`n"), "constant.character.escape.powershell");
    assert_eq!(leaf(text, "{"), "punctuation.section.braces.begin");
    assert_eq!(
        leaf(text, "-f"),
        "keyword.operator.string-format.powershell"
    );
}

#[test]
fn a_help_comment_names_its_keywords_and_argument() {
    let text = indoc! {"
        <#
        .SYNOPSIS
        .PARAMETER Name   The name.
        #>
    "};
    assert_eq!(
        leaf(text, "SYNOPSIS"),
        "keyword.operator.documentation.powershell"
    );
    assert_eq!(
        leaf(text, "Name   The name."),
        "keyword.operator.documentation.powershell"
    );
    assert_eq!(
        leaf(text, "#>"),
        "punctuation.definition.comment.block.end.powershell"
    );
}

#[test]
fn a_class_names_its_name() {
    let text = "class Foo : Bar { hidden [int] $n = 10 }";
    assert_eq!(leaf(text, "class"), "storage.type.powershell");
    assert_eq!(leaf(text, "Foo"), "entity.name.function");
    assert_eq!(leaf(text, "hidden"), "storage.modifier.powershell");
}

#[test]
fn a_here_string_runs_to_its_closing_line() {
    let rows = lex(indoc! {r#"
        @"
        here $x
        "@
    "#});
    assert_eq!(
        rows[1][0].1.last().map(String::as_str),
        Some("string.quoted.double.heredoc.powershell")
    );
    assert_eq!(
        rows[2][0].1.last().map(String::as_str),
        Some("punctuation.definition.string.end.powershell")
    );
}
