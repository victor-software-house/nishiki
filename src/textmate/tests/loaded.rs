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

/// A fixture that still differs from Shiki, and why.
/// Each names a gap in the hand-off that a later change closes.
struct Gap {
    fixture: &'static str,
    cause: &'static str,
}

/// Checks the fixtures of `language` against a highlighter for the grammar
/// whose root scope is `root`.
///
/// The set holds only the grammars `names` lists, from
/// `tests/engine/grammars/<name>.json`.
/// A block of a language that neither the set nor a hand lexer covers is
/// left out, as [`unknown_fences`] finds it.
///
/// Each fixture in `gaps` must still differ.
/// One that matches fails the test until its entry goes.
fn matches_shiki(names: &[&str], root: &str, language: &str, gaps: &[Gap]) {
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
        let stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap();
        let gap = gaps.iter().find(|gap| gap.fixture == stem);
        match (gap, found.missed) {
            (None, 0) | (Some(_), 1..) => {}
            (Some(gap), 0) => failures.push(format!(
                "{}: matches Shiki now, so its gap is closed: {}",
                path.display(),
                gap.cause
            )),
            (None, missed) => failures.push(format!(
                "{}: {} of {} bytes differ{}",
                path.display(),
                missed,
                found.checked,
                found.message
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// Markdown's fences embed most languages, so the test needs them all.
/// Its `php` fence includes both HTML and PHP, and PHP's lexer takes it.
#[cfg(feature = "all")]
#[test]
fn markdown_alone_matches_shiki() {
    let attributes = "attribute names in an HTML fence differ from Shiki's";
    let gaps = [
        Gap {
            fixture: "invalid-tags",
            cause: attributes,
        },
        Gap {
            fixture: "perl-heredoc-tags-1",
            cause: attributes,
        },
        Gap {
            fixture: "perl-heredoc-tags-2",
            cause: attributes,
        },
    ];
    matches_shiki(&["markdown"], "text.html.markdown", "markdown", &gaps);
}

/// HTML hands `<script>` to JavaScript and `<style>` to CSS.
#[cfg(all(feature = "lang-javascript", feature = "lang-css"))]
#[test]
fn html_alone_matches_shiki() {
    matches_shiki(&["html"], "text.html.basic", "html", &[]);
}

/// Vue's template includes both html-derivative and HTML, and
/// html-derivative's lexer takes it.
/// HTML's grammar stays in the set, because Vue includes some of its rules,
/// such as `text.html.basic#attribute`, which no lexer serves.
#[cfg(feature = "all")]
#[test]
fn vue_with_html_matches_shiki() {
    let names = [
        "vue",
        "html",
        "vue-directives",
        "vue-interpolations",
        "markdown-vue",
        "vue-sfc-style-variable-injection",
    ];
    let directives = "vue-directives' injection misses the tags html-derivative's lexer reads";
    let gaps = [
        "coverage",
        "playground-app",
        "playground-header",
        "vitepress-image",
        "vitepress-layout",
        "vitepress-nav-menu-group",
    ]
    .map(|fixture| Gap {
        fixture,
        cause: directives,
    });
    matches_shiki(&names, "text.html.vue", "vue", &gaps);
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
    matches_shiki(&["graphql"], "source.graphql", "graphql", &[]);
}
