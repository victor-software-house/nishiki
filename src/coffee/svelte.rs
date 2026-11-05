//! The tables of the CoffeeScript that Svelte embeds ([`SVELTE`]).
//!
//! Svelte's grammar injects its `$` prefixes there:
//! `L:(source.ts, source.js, source.coffee)`.
//! vscode-textmate tries an `L:` injection in every rule whose scopes its
//! selector matches, and takes it over the rule's own match where both start
//! at one place, the end pattern included.
//! So each rule here tries [`Pat::SVELTE`] first, then its end pattern, then
//! its own patterns.

use super::engine::{Coffee, SVELTE};
use super::grammar::{Pat, Rule};
use crate::rules;

impl rules::Tabled for Coffee<SVELTE> {
    fn pat_start(pat: Pat) -> rules::Start {
        pat.start()
    }

    fn tables(rule: Rule) -> &'static rules::Tables<Self> {
        static TABLES: [std::sync::OnceLock<rules::Tables<Coffee<SVELTE>>>; Rule::ALL.len()] =
            [const { std::sync::OnceLock::new() }; Rule::ALL.len()];
        let index = usize::from(rule.code() & 0xff);
        TABLES[index].get_or_init(move || rules::Tables::build(injected(rule)))
    }
}

/// The data of `rule` with Svelte's patterns first.
/// The list lives as long as the program, as the tables built from it do.
fn injected(rule: Rule) -> rules::RuleData<Coffee<SVELTE>> {
    let data = rule.data();
    let patterns: Vec<Pat> = Pat::SVELTE
        .into_iter()
        .chain(data.end)
        .chain(data.patterns.iter().copied())
        .collect();
    rules::RuleData {
        name: data.name,
        content: data.content,
        end: None,
        backref: data.backref,
        patterns: Box::leak(patterns.into_boxed_slice()),
    }
}
