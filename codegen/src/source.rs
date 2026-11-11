//! A grammar of `tests/engine/grammars/`, as the writers and checks read it.

use std::fs;
use std::path::Path;

use nishiki::unstable::grammar::{Repository, RuleId, UnlinkedGrammar, UnlinkedRule, unlinked};
use nishiki::{Grammar, GrammarError};

use crate::error::Error;

/// The directory of the grammars, from the repository root.
const DIR: &str = "tests/engine/grammars";

/// A grammar of `tests/engine/grammars/` that a writer reads.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Source {
    /// The grammar's file name, without `.json`.
    pub(crate) name: &'static str,
    /// The grammar's rules, as its JSON holds them.
    grammar: &'static UnlinkedGrammar,
}

impl Source {
    /// Reads `tests/engine/grammars/<name>.json` under `root`.
    pub(crate) fn load(root: &Path, name: &'static str) -> Result<Source, Error> {
        let path = root.join(Source::path_of(name));
        let text = fs::read_to_string(&path).map_err(|source| Error::Io {
            path: path.clone(),
            source,
        })?;
        Source::parse(name, &text).map_err(|source| Error::Grammar { path, source })
    }

    /// Reads the grammar named `name` from its JSON `text`.
    pub(crate) fn parse(name: &'static str, text: &str) -> Result<Source, GrammarError> {
        let grammar = Grammar::from_json(text)?;
        Ok(Source {
            name,
            grammar: unlinked(grammar),
        })
    }

    /// Reads every grammar of `tests/engine/grammars/` under `root`, sorted by
    /// name.
    /// `tables.json` there lists the engine's test tables, not a grammar.
    pub(crate) fn all(root: &Path) -> Result<Vec<Source>, Error> {
        let dir = root.join(DIR);
        let entries = fs::read_dir(&dir).map_err(|source| Error::Io {
            path: dir.clone(),
            source,
        })?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|source| Error::Io {
                path: dir.clone(),
                source,
            })?;
            let file = entry.file_name();
            if let Some(name) = file.to_str().and_then(|file| file.strip_suffix(".json"))
                && name != "tables"
            {
                names.push(name.to_owned());
            }
        }
        names.sort();
        names
            .into_iter()
            .map(|name| Source::load(root, name.leak()))
            .collect()
    }

    /// The grammar's `scopeName`.
    pub(crate) fn scope(self) -> &'static str {
        &self.grammar.scope
    }

    /// The grammar's top-level repository: each entry's name and rule, sorted
    /// by name.
    pub(crate) fn entries(self) -> &'static Repository {
        &self.grammar.repository
    }

    /// The rule of the grammar's top-level repository named `key`, when it
    /// has one.
    pub(crate) fn entry_id(self, key: &str) -> Option<RuleId> {
        let repository = &self.grammar.repository;
        let index = repository
            .binary_search_by(|(name, _)| (**name).cmp(key))
            .ok()?;
        repository.get(index).map(|&(_, id)| id)
    }

    /// The grammar file's path from the repository root.
    pub(crate) fn path(self) -> String {
        Source::path_of(self.name)
    }

    /// The path of the grammar file named `name`, from the repository root.
    fn path_of(name: &str) -> String {
        format!("{DIR}/{name}.json")
    }

    /// The grammar's top-level patterns.
    pub(crate) fn patterns(self) -> &'static [RuleId] {
        &self.grammar.patterns
    }

    /// The rule with index `id`.
    pub(crate) fn rule(self, id: RuleId) -> Result<&'static UnlinkedRule, Error> {
        self.grammar
            .rules
            .get(id)
            .ok_or_else(|| Error::Shape(format!("{}: no rule {id}", self.name)))
    }

    /// The rule of the grammar's top-level repository named `key`.
    pub(crate) fn entry(self, key: &str) -> Result<&'static UnlinkedRule, Error> {
        self.entry_in(&self.grammar.repository, key)
    }

    /// The rule of `rule`'s own repository named `key`.
    pub(crate) fn nested_entry(
        self,
        rule: &UnlinkedRule,
        key: &str,
    ) -> Result<&'static UnlinkedRule, Error> {
        let repository = rule
            .repository
            .as_ref()
            .ok_or_else(|| Error::Shape(format!("{}: no repository holds {key}", self.name)))?;
        self.entry_in(repository, key)
    }

    /// The rule of `repository` named `key`.
    /// A repository is sorted by name.
    pub(crate) fn entry_in(
        self,
        repository: &Repository,
        key: &str,
    ) -> Result<&'static UnlinkedRule, Error> {
        let (_, id) = repository
            .binary_search_by(|(name, _)| (**name).cmp(key))
            .ok()
            .and_then(|index| repository.get(index))
            .ok_or_else(|| Error::Shape(format!("{}: no repository entry {key}", self.name)))?;
        self.rule(*id)
    }

    /// The `index`th pattern of `rule`.
    pub(crate) fn pattern(
        self,
        rule: &UnlinkedRule,
        index: usize,
    ) -> Result<&'static UnlinkedRule, Error> {
        let id = rule
            .patterns
            .get(index)
            .ok_or_else(|| Error::Shape(format!("{}: no pattern {index}", self.name)))?;
        self.rule(*id)
    }

    /// The pattern text of the rule that `path` picks under repository entry
    /// `key`: each index picks one of the patterns of the rule before it.
    pub(crate) fn pattern_at(self, key: &str, path: &[usize]) -> Result<&'static str, Error> {
        let mut rule = self.entry(key)?;
        for &index in path {
            rule = self.pattern(rule, index)?;
        }
        self.text(rule)
    }

    /// The one rule of `ids` whose scope is `name`.
    pub(crate) fn named(self, ids: &[RuleId], name: &str) -> Result<&'static UnlinkedRule, Error> {
        let mut found = Vec::new();
        for &id in ids {
            let rule = self.rule(id)?;
            if has_name(rule, name) {
                found.push(rule);
            }
        }
        match found.as_slice() {
            [rule] => Ok(*rule),
            rules => Err(Error::Shape(format!(
                "{}: {} patterns named {name}",
                self.name,
                rules.len()
            ))),
        }
    }

    /// `rule`'s `match`, or its `begin` when it has no `match`.
    pub(crate) fn text(self, rule: &'static UnlinkedRule) -> Result<&'static str, Error> {
        rule.matched
            .as_deref()
            .or(rule.begin.as_deref())
            .ok_or_else(|| Error::Shape(format!("{}: a rule with no match or begin", self.name)))
    }
}

/// Whether `rule`'s `name` is the one scope `name`.
pub(crate) fn has_name(rule: &UnlinkedRule, name: &str) -> bool {
    matches!(&*rule.name, [scope] if scope == name)
}
