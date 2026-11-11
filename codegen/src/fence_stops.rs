//! The check of `Language::pops_host` in `src/language.rs`.
//!
//! A grammar whose root holds a `match` that can match the empty text can stop
//! there without advancing.
//! vscode-textmate then pops the innermost rule, which in a Markdown fence is
//! the fence's body.
//!
//! Markdown lexes the fences of the languages `pops_host` names, so that they
//! can report it.
//! The check lists the committed grammars that Markdown's fences include and
//! that can stop at their root.
//! It fails when that list differs from the languages `pops_host` names, and
//! when a fence's grammar is neither committed nor in [`UNCHECKED`].

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use nishiki::unstable::grammar::RuleId;
use nishiki::unstable::regex::{Dialect, Node, parse};

use crate::error::Error;
use crate::source::Source;

/// The scopes that Markdown's fences include and the check cannot read, each
/// with why.
///
/// - `source.groovy`: `oracle/licences.ts` refuses its grammar, so it is not
///   committed.
/// - `source.ignore`, `source.js.regexp`, `source.twig` and `source.yang`:
///   Shiki bundles no grammar with the scope.
pub(crate) const UNCHECKED: &[(&str, &str)] = &[
    (
        "source.groovy",
        "oracle/licences.ts refuses groovy, which states no licence",
    ),
    ("source.ignore", "Shiki bundles no grammar with this scope"),
    (
        "source.js.regexp",
        "Shiki bundles no grammar with this scope",
    ),
    ("source.twig", "Shiki bundles no grammar with this scope"),
    ("source.yang", "Shiki bundles no grammar with this scope"),
];

/// The line that closes a method of the `impl Language` block.
const METHOD_END: &str = "    }";

/// The repository entries of Markdown's grammar that hold its fences start
/// with this.
const FENCE: &str = "fenced_code_block";

/// Checks `Language::pops_host` against the grammars of
/// `tests/engine/grammars/` under `root`.
/// Returns the names of the fence grammars that can stop at their root.
pub(crate) fn check(root: &Path) -> Result<Vec<&'static str>, Error> {
    let grammars = Source::all(root)?;
    let markdown = grammars
        .iter()
        .copied()
        .find(|grammar| grammar.name == "markdown")
        .ok_or_else(|| Error::Shape("no markdown grammar is committed".to_owned()))?;
    let mut fenced = BTreeSet::new();
    for (key, id) in markdown.entries().iter() {
        if key.starts_with(FENCE) {
            includes(markdown, *id, &mut fenced)?;
        }
    }
    let by_scope: BTreeMap<&str, Source> = grammars
        .iter()
        .map(|grammar| (grammar.scope(), *grammar))
        .collect();
    let mut stopping = Vec::new();
    let mut wrong = Vec::new();
    for &scope in &fenced {
        let unchecked = UNCHECKED.iter().any(|&(other, _)| other == scope);
        match (by_scope.get(scope), unchecked) {
            (Some(grammar), false) => {
                if stops_at_root(*grammar)? {
                    stopping.push(grammar.name);
                }
            }
            (None, true) => {}
            (Some(_), true) => wrong.push(format!("{scope} is committed, but UNCHECKED lists it")),
            (None, false) => wrong.push(format!(
                "a fence includes {scope}, which is neither committed nor in UNCHECKED"
            )),
        }
    }
    for &(scope, _) in UNCHECKED {
        if !fenced.contains(scope) {
            wrong.push(format!("UNCHECKED lists {scope}, which no fence includes"));
        }
    }
    if !wrong.is_empty() {
        return Err(Error::Check(wrong.join("\n")));
    }
    stopping.sort_unstable();
    compare(&stopping, &pops_host(root)?)?;
    Ok(stopping)
}

/// Adds to `into` the scope of each other grammar that the rule `id` and the
/// rules inside it include.
fn includes(source: Source, id: RuleId, into: &mut BTreeSet<&'static str>) -> Result<(), Error> {
    let rule = source.rule(id)?;
    if let Some(include) = rule.include.as_deref()
        && (include.starts_with("source.") || include.starts_with("text."))
    {
        into.insert(include.split_once('#').map_or(include, |(scope, _)| scope));
    }
    let captures = [
        &rule.captures,
        &rule.begin_captures,
        &rule.end_captures,
        &rule.while_captures,
    ];
    let inner = rule
        .patterns
        .iter()
        .copied()
        .chain(
            captures
                .into_iter()
                .flatten()
                .flat_map(|list| list.iter())
                .map(|capture| capture.rule),
        )
        .chain(
            rule.repository
                .iter()
                .flat_map(|repository| repository.iter())
                .map(|&(_, id)| id),
        );
    for id in inner {
        includes(source, id, into)?;
    }
    Ok(())
}

/// Whether a root rule of `source` is a `match` that can match the empty text.
/// It follows `$self`, `$base` and the grammar's own repository.
fn stops_at_root(source: Source) -> Result<bool, Error> {
    stops(source, source.patterns(), &mut BTreeSet::new())
}

/// Whether one of `ids` is a `match` that can match the empty text, or leads
/// to one through an include of the grammar's own rules.
/// `seen` holds the rules already tried.
fn stops(source: Source, ids: &[RuleId], seen: &mut BTreeSet<RuleId>) -> Result<bool, Error> {
    for &id in ids {
        if !seen.insert(id) {
            continue;
        }
        let rule = source.rule(id)?;
        let stop = match rule.include.as_deref() {
            Some("$self" | "$base") => stops(source, source.patterns(), seen)?,
            Some(include) => match include
                .strip_prefix('#')
                .and_then(|key| source.entry_id(key))
            {
                Some(entry) => {
                    let found = source.rule(entry)?;
                    if found.matched.is_some() || found.begin.is_some() {
                        stops(source, &[entry], seen)?
                    } else {
                        stops(source, &found.patterns, seen)?
                    }
                }
                None => false,
            },
            None if rule.begin.is_some() => false,
            None => match rule.matched.as_deref() {
                Some(pattern) => can_match_empty(pattern),
                None => stops(source, &rule.patterns, seen)?,
            },
        };
        if stop {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether `pattern` can match the empty text.
/// A pattern the parser does not read counts as able to.
fn can_match_empty(pattern: &str) -> bool {
    match parse(pattern, Dialect::Grammar) {
        Ok(parsed) => empty(&parsed.root),
        Err(_) => true,
    }
}

/// Whether `node` can match the empty text.
/// A lookaround, an anchor and a reference to a group can.
fn empty(node: &Node) -> bool {
    match node {
        Node::Empty
        | Node::Lookaround { .. }
        | Node::Anchor(_)
        | Node::BackReference(..)
        | Node::NamedBackReference(..)
        | Node::Subroutine(_)
        | Node::NamedSubroutine(_) => true,
        Node::Char(_) | Node::Any { .. } | Node::Class(_) => false,
        Node::Sequence(items) => items.iter().all(empty),
        Node::Alternation(items) | Node::Folds(items) => items.iter().any(empty),
        Node::Repeat { node, min, .. } => *min == 0 || empty(node),
        Node::Group { node, .. } | Node::Atomic(node) => empty(node),
    }
}

/// The languages `Language::pops_host` in `src/language.rs` under `root`
/// names, as Shiki names their grammars, sorted.
fn pops_host(root: &Path) -> Result<Vec<String>, Error> {
    let path = root.join("src/language.rs");
    let text = fs::read_to_string(&path).map_err(|source| Error::Io {
        path: path.clone(),
        source,
    })?;
    if !text.contains("fn pops_host") {
        return Err(Error::Shape(
            "src/language.rs has no method pops_host".to_owned(),
        ));
    }
    let named: BTreeSet<String> = text
        .lines()
        .skip_while(|line| !line.contains("fn pops_host"))
        .take_while(|line| *line != METHOD_END)
        .flat_map(|line| line.split("Language::").skip(1))
        .map(|rest| {
            let variant = rest
                .split(|c: char| !c.is_ascii_alphanumeric())
                .next()
                .unwrap_or_default();
            kebab(variant)
        })
        .collect();
    Ok(named.into_iter().collect())
}

/// `variant` in kebab case, as Shiki names a grammar: `ObjectiveCpp` is
/// `objective-cpp`.
fn kebab(variant: &str) -> String {
    let mut name = String::new();
    let mut lower = false;
    for c in variant.chars() {
        if lower && c.is_ascii_uppercase() {
            name.push('-');
        }
        lower = c.is_ascii_lowercase();
        name.push(c.to_ascii_lowercase());
    }
    name
}

/// Fails when the grammars that can stop at their root differ from the
/// languages `pops_host` names.
fn compare(stopping: &[&str], named: &[String]) -> Result<(), Error> {
    if stopping.iter().eq(named.iter()) {
        return Ok(());
    }
    Err(Error::Check(format!(
        "fence grammars that can stop at their root: {}\nlanguages Language::pops_host names: {}",
        listed(stopping),
        listed(named)
    )))
}

/// `names` joined by commas, or `none`.
pub(crate) fn listed<T: AsRef<str>>(names: &[T]) -> String {
    if names.is_empty() {
        return "none".to_owned();
    }
    let names: Vec<&str> = names.iter().map(AsRef::as_ref).collect();
    names.join(", ")
}
