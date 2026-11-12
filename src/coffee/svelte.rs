//! The tables of the CoffeeScript that Svelte embeds ([`SVELTE`]).
//!
//! Svelte's grammar injects its `$` prefixes there:
//! `L:(source.ts, source.js, source.coffee)`.
//! vscode-textmate tries an `L:` injection in every rule whose scopes its
//! selector matches.
//! Where both start at one place, it takes the injection over the rule's own
//! match, the end pattern included.
//! So each rule here tries [`Pattern::SVELTE`] first, then its end pattern,
//! then its own patterns.

use super::engine::{Coffee, SVELTE};
use super::grammar::{Pattern, Rule};
use crate::rules;

impl rules::Tabled for Coffee<SVELTE> {
    const WORDS_AT_START: bool = Pattern::WORDS_AT_START;
    const MIXED_BLANK_RUNS: bool = Pattern::MIXED_BLANK_RUNS;

    fn pattern_start(pattern: Pattern) -> rules::Start {
        pattern.start()
    }

    fn tables(rule: Rule) -> &'static rules::Tables<Self> {
        static TABLES: [std::sync::OnceLock<rules::Tables<Coffee<SVELTE>>>; Rule::ALL.len()] =
            [const { std::sync::OnceLock::new() }; Rule::ALL.len()];
        let index = usize::from(rule.code() & 0xff);
        TABLES[index].get_or_init(move || rules::Tables::build(injected(rule)))
    }
}

/// The definition of `rule`, with Svelte's patterns first.
/// The list lives as long as the program, as the tables built from it do.
fn injected(rule: Rule) -> rules::RuleDefinition<Coffee<SVELTE>> {
    let definition = rule.definition();
    let patterns: Vec<Pattern> = Pattern::SVELTE
        .into_iter()
        .chain(definition.end)
        .chain(definition.patterns.iter().copied())
        .collect();
    rules::RuleDefinition {
        name: definition.name,
        content: definition.content,
        end: None,
        backref: definition.backref,
        patterns: Box::leak(patterns.into_boxed_slice()),
    }
}
