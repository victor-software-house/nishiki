//! Checks that lexing a line allocates nothing once the arena holds the
//! states and stacks a document uses (`CODING_STANDARDS.md` 2.1).
//!
//! Each oracle fixture is lexed twice by one highlighter. The first pass fills
//! the arena, the scratch buffers and the token buffer; the second meets only
//! states the first interned, so any allocation it makes is per line.
//! `nishiki-alloc-count` counts per thread, and each language runs on a
//! thread of its own, so tests running in parallel do not disturb the count.

use nishiki::{Highlighter, Language, State, Token};
use nishiki_alloc_count::{CountingAllocator, count, measure};

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

/// YAML directives, which no fixture had. Each directive reads its name. The
/// name must be a view of the line, not a copy.
#[test]
fn lexing_yaml_directives_again_allocates_nothing() {
    let lines = [
        "%YAML 1.2",
        "%TAG ! tag:example.com,2000:",
        "%RESERVED name parameter",
        "---",
        "key: !local value",
    ]
    .map(str::to_owned);
    let mut highlighter = Highlighter::new(Language::Yaml);
    let mut tokens = Vec::new();
    lex(&mut highlighter, &lines, &mut tokens);
    let allocations = count(|| lex(&mut highlighter, &lines, &mut tokens));
    assert_eq!(allocations, 0, "allocations lexing the lines a second time");
}

/// Lines of several JavaScript texts each, which no fixture has. The texts
/// after the first on a line share a list of the line's matches. The scratch
/// keeps that list from one line to the next.
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
/// no fixture had. Each fence's first line switches the engine's scratch to
/// the other grammar. The first pass already built that grammar's caches.
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

/// A Ruby document whose stack deepens by one rule per call, as Shiki's
/// does. In `expect(<<~RUBY)` the here document takes the `)`, and the
/// call's rule stays open. Each call adds one state on top of the last. Every
/// later line shares that state. Twice the calls hold about twice the memory.
/// Lexing the document again allocates nothing.
#[test]
fn a_deepening_ruby_stack_costs_one_state_per_open_call() {
    let document = |calls: usize| -> Vec<String> {
        ["expect(<<~RUBY)", "  x", "RUBY"]
            .repeat(calls)
            .into_iter()
            .map(str::to_owned)
            .collect()
    };
    let peak = |lines: &[String]| {
        measure(|| {
            let mut highlighter = Highlighter::new(Language::Ruby);
            lex(&mut highlighter, lines, &mut Vec::new());
            highlighter
        })
        .peak
    };
    let (short, long) = (document(300), document(600));
    let _ = peak(&short);
    let (short_peak, long_peak) = (peak(&short), peak(&long));
    assert!(
        long_peak < short_peak * 3,
        "600 calls held {long_peak} bytes at most, 300 held {short_peak}"
    );
    let mut highlighter = Highlighter::new(Language::Ruby);
    let mut tokens = Vec::new();
    lex(&mut highlighter, &short, &mut tokens);
    let allocations = count(|| lex(&mut highlighter, &short, &mut tokens));
    assert_eq!(
        allocations, 0,
        "allocations lexing the document a second time"
    );
}
