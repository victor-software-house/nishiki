//! Checks that lexing a line allocates nothing once the arena holds the
//! states and stacks a document uses (`CODING_STANDARDS.md` 2.1).
//!
//! Each oracle fixture is lexed twice by one highlighter. The first pass fills
//! the arena, the scratch buffers and the token buffer; the second meets only
//! states the first interned, so any allocation it makes is per line.
//! `nishiki-alloc-count` counts per thread, and each language runs on a
//! thread of its own, so tests running in parallel do not disturb the count.

use nishiki::{Highlighter, Language, State, Token};
use nishiki_alloc_count::{CountingAllocator, count};

use crate::oracle::{every_language, fixtures, parse};

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// The languages that still allocate per line, each until its owner fixes
/// it. The test fails when one of them stops allocating, so the list only
/// shrinks.
const ALLOCATING: &[&str] = &[];

/// Lexes `lines` as one document from the start.
fn lex(highlighter: &mut Highlighter, lines: &[String], tokens: &mut Vec<Token>) {
    let mut state = State::default();
    for line in lines {
        tokens.clear();
        let _ = highlighter.line(line, &mut state, tokens);
    }
}

/// The fixtures of `language` whose second pass allocated, as reports, or
/// a report that a listed language no longer allocates.
fn second_pass_failures(language: Language) -> Vec<String> {
    let mut allocating = Vec::new();
    for fixture in fixtures(language.id()) {
        let lines: Vec<String> = parse(&fixture).into_iter().map(|line| line.text).collect();
        let mut highlighter = Highlighter::new(language);
        let mut tokens = Vec::new();
        lex(&mut highlighter, &lines, &mut tokens);
        let allocations = count(|| lex(&mut highlighter, &lines, &mut tokens));
        if allocations > 0 {
            allocating.push(format!(
                "{}: {allocations} allocations lexing it a second time",
                fixture.display()
            ));
        }
    }
    let listed = ALLOCATING.contains(&language.id());
    match (listed, allocating.is_empty()) {
        (false, _) => allocating,
        (true, false) => Vec::new(),
        (true, true) => vec![format!(
            "{} no longer allocates per line: remove it from ALLOCATING",
            language.id()
        )],
    }
}

#[test]
fn lexing_a_line_again_allocates_nothing() {
    every_language(second_pass_failures);
}

/// Lines of several JavaScript texts each, which no fixture has: the texts
/// after the first on a line share a list of the line's matches, which the
/// scratch keeps from one line to the next.
#[test]
fn lexing_event_handlers_again_allocates_nothing() {
    let lines = [
        r#"<a onclick="f(a)" onmouseover="g`x`" onblur="h(b)">x</a>"#,
        r#"<b onclick="(1)" onfocus="`${a}`"><i onload="k()">"#,
        r#"<p onclick="a < b" onkeyup="c(d, `e`)" onchange="f">"#,
    ]
    .map(str::to_owned);
    let mut highlighter = Highlighter::new(Language::Html);
    let mut tokens = Vec::new();
    lex(&mut highlighter, &lines, &mut tokens);
    let allocations = count(|| lex(&mut highlighter, &lines, &mut tokens));
    assert_eq!(allocations, 0, "allocations lexing the lines a second time");
}

/// A Markdown document with fences in two of the engine's languages, which
/// no fixture had: each fence's first line switches the engine's scratch to
/// the other grammar, whose caches the first pass already built.
#[cfg(all(feature = "lang-powershell", feature = "lang-scala"))]
#[test]
fn switching_between_engine_grammars_again_allocates_nothing() {
    let lines: Vec<String> = indoc::indoc! {r#"
        ```powershell
        Get-ChildItem | Where-Object { $_.Length -gt 0 }
        ```

        ```scala
        object Main { def main(args: Array[String]): Unit = println("x") }
        ```

        ```powershell
        Write-Output "done"
        ```
    "#}
    .lines()
    .map(str::to_owned)
    .collect();
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut tokens = Vec::new();
    lex(&mut highlighter, &lines, &mut tokens);
    let allocations = count(|| lex(&mut highlighter, &lines, &mut tokens));
    assert_eq!(
        allocations, 0,
        "allocations lexing the document a second time"
    );
}

/// A Markdown document with Batch and PowerShell fences: Batch's grammar is
/// the engine's one with an injection, whose selectors the first switch to
/// it parses and a second pass reuses.
#[cfg(all(feature = "lang-bat", feature = "lang-powershell"))]
#[test]
fn switching_to_a_grammar_with_injections_again_allocates_nothing() {
    let lines: Vec<String> = indoc::indoc! {r#"
        ```bat
        for %%f in (*.txt) do (echo %%f)
        ```

        ```powershell
        Write-Output "between"
        ```

        ```batch
        if exist out (del out) else (echo none)
        ```
    "#}
    .lines()
    .map(str::to_owned)
    .collect();
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut tokens = Vec::new();
    lex(&mut highlighter, &lines, &mut tokens);
    let allocations = count(|| lex(&mut highlighter, &lines, &mut tokens));
    assert_eq!(
        allocations, 0,
        "allocations lexing the document a second time"
    );
}
