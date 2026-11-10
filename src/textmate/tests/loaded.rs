//! Grammars a caller loads without the grammars they include, against Shiki.
//!
//! Each loads through the public API from `tests/engine/grammars/`, with no
//! language named for hand-offs.
//! An include of another language's whole grammar then goes to that
//! language's lexer, when its feature is on.
//! The output must match the fixtures, which Shiki wrote with every grammar
//! loaded.

use std::path::Path;

use super::{Embedding, compare, fixtures, parse, unknown_fences};
use crate::{Grammar, Grammars, Highlighter, Limits};

/// Checks the fixtures of `language` against a highlighter for the grammar
/// whose root scope is `root`.
///
/// The set holds only the grammars `names` lists, from
/// `tests/engine/grammars/<name>.json`.
/// A block of a language that neither the set nor a hand lexer covers is
/// left out, as [`unknown_fences`] finds it.
fn matches_shiki(names: &[&str], root: &str, language: &str) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/engine/grammars");
    let grammars: Grammars = names
        .iter()
        .map(|name| {
            let json = std::fs::read_to_string(dir.join(format!("{name}.json"))).unwrap();
            Grammar::from_json(&json).unwrap()
        })
        .collect();
    let root = grammars.get(root).unwrap();
    let mut failures = Vec::new();
    for path in fixtures(language) {
        let lines = parse(&path);
        let highlighter =
            Highlighter::for_grammar_with_limits(root, &grammars, Limits::unlimited()).unwrap();
        let skip = unknown_fences(highlighter.lexer().table, &lines, Embedding::Lexers);
        let found = compare(highlighter, &lines, &skip);
        if found.missed > 0 {
            failures.push(format!(
                "{}: {} of {} bytes differ{}",
                path.display(),
                found.missed,
                found.checked,
                found.message
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Markdown's fences embed most languages, so the test needs them all.
/// Its `php` fence includes both HTML and PHP, and PHP's lexer takes it.
#[cfg(feature = "all")]
#[test]
fn markdown_alone_matches_shiki() {
    matches_shiki(&["markdown"], "text.html.markdown", "markdown");
}

/// HTML hands `<script>` to JavaScript and `<style>` to CSS.
#[cfg(all(feature = "lang-javascript", feature = "lang-css"))]
#[test]
fn html_alone_matches_shiki() {
    matches_shiki(&["html"], "text.html.basic", "html");
}

/// Vue's template includes both html-derivative and HTML, and
/// html-derivative's lexer takes it.
/// Vue also includes rules from inside other grammars, which no lexer serves:
///
/// - HTML's `text.html.basic#attribute`;
/// - TypeScript's `source.ts#expression`, in its directives;
/// - CSS's and JSON's rules, in its style variables and its custom blocks.
///
/// So their grammars stay in the set.
/// TOML's lexer cannot halt, so its grammar stays in the set too, for the
/// `lang="toml"` blocks.
/// Without TypeScript's, the linker drops the directive rules, as
/// vscode-textmate does for a missing grammar.
#[cfg(feature = "all")]
#[test]
fn vue_with_its_included_grammars_matches_shiki() {
    let names = [
        "vue",
        "html",
        "typescript",
        "css",
        "json",
        "toml",
        "vue-directives",
        "vue-interpolations",
        "markdown-vue",
        "vue-sfc-style-variable-injection",
    ];
    matches_shiki(&names, "text.html.vue", "vue");
}

/// GraphQL's `${…}` includes JavaScript, TypeScript, JSX and TSX as
/// alternatives, and the first, JavaScript, takes it.
#[cfg(all(
    feature = "lang-typescript",
    feature = "lang-jsx",
    feature = "lang-tsx"
))]
#[test]
fn graphql_alone_matches_shiki() {
    matches_shiki(&["graphql"], "source.graphql", "graphql");
}
