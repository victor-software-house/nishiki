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
