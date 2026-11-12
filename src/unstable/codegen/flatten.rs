//! A grammar's rules and pattern lists, with its includes flattened.
//!
//! The walk starts at the grammar's top-level patterns, the rule `Root`.
//! It names each pattern by where it is found:
//!
//! - a repository entry takes its key in Pascal case, such as `StringQuoted`
//!   for `string-quoted`;
//! - a pattern in a list takes the list's id, `At` and its position, such as
//!   `RootAt3`;
//! - a begin's end takes the begin's id and `End`.
//!
//! A language can name an inline pattern itself, in
//! [`RuleTables::pattern_names`].
//! A match named after its first group, `$1`, becomes one pattern for each
//! alternative of the group, each with its own place in the list.
//!
//! A rule a begin opens takes the begin's id, and lists its own patterns.
//! An entry with patterns and no expression adds its patterns in its place.
//! `$self`, or the grammar's own scope name, adds the top-level patterns.
//! An include of another grammar adds what [`RuleTables::externals`] says.
//! A list being expanded, `Root` included, adds nothing when it is reached
//! again.
//!
//! A pattern reached again keeps its first id, so an include repeats no
//! pattern.
//!
//! A list reached again while it is being expanded adds nothing there.
//! When `Root` takes its patterns, the top-level list counts as being
//! expanded, so a `$self` in it adds nothing.
//! [`RuleTables::reentry`] sets whether that holds within one rule's list
//! or in the whole walk.
//! An include of a repository key the grammar lacks adds nothing, as
//! vscode-textmate skips it.
//!
//! Two patterns or rules with different contents never share an id.
//! The scripts let the later one replace the earlier one.
//! The walk fails instead.
//!
//! The walk ports the copies of `flatten`, `expand` and `register` in the
//! `oracle/<language>-grammar.ts` scripts.
//! [`RuleTables`] holds what each copy does differently.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use super::error::GenerateError;
use super::split;
use super::tables::{
    Act, CaptureLists, External, LonePattern, Pattern, Reentry, Rule, RuleTables, Tables,
};
use crate::textmate::compile::{RuleId, Text, UnlinkedCapture, UnlinkedGrammar, UnlinkedRule};

/// The key of the top-level list in the set of lists being expanded.
/// No repository key gives it in Pascal case, so a key such as `root`,
/// whose lists take the id `Root`, does not share it.
const TOP_LEVEL: &str = "$self";

/// The tables of `grammar`, read as `language` says.
///
/// # Errors
///
/// [`GenerateError`] when an include names another grammar, when a rule has
/// nothing to match, or when two patterns or rules take one id.
pub fn flatten(grammar: &UnlinkedGrammar, language: &RuleTables) -> Result<Tables, GenerateError> {
    let mut walk = Walk {
        grammar,
        language,
        tables: Tables::default(),
        patterns: HashMap::new(),
        capture_lists: HashMap::new(),
        expanding: HashSet::new(),
        opened: Vec::new(),
    };
    let root = match language.reentry {
        Reentry::WithinRule => {
            walk.expanding.insert(TOP_LEVEL.to_owned());
            walk.list(&grammar.patterns, "Root")?
        }
        Reentry::WithinWalk => {
            walk.list(&grammar.patterns, "Root")?;
            for (index, list, id) in std::mem::take(&mut walk.opened) {
                walk.expanding.clear();
                walk.tables.rules[index].patterns = walk.list(list, &id)?;
            }
            walk.expanding.clear();
            walk.expanding.insert(TOP_LEVEL.to_owned());
            walk.list(&grammar.patterns, "Root")?
        }
    };
    let index = walk.put_rule("Root", None)?;
    walk.tables.rules[index].patterns = root;
    Ok(walk.tables)
}

/// The state of one walk over a grammar.
#[derive(Debug)]
struct Walk<'a> {
    /// The grammar.
    grammar: &'a UnlinkedGrammar,
    /// What the language does differently.
    language: &'a RuleTables,
    /// The tables so far.
    tables: Tables,
    /// Each pattern's index in the tables, by id.
    patterns: HashMap<String, usize>,
    /// Each capture list's rule id, by the list's text.
    capture_lists: HashMap<String, String>,
    /// The ids of the lists being expanded, in the scope
    /// [`RuleTables::reentry`] sets.
    expanding: HashSet<String>,
    /// Each rule the walk opened: its index in the tables, its pattern list
    /// and its id, for the second pass of [`Reentry::WithinWalk`].
    opened: Vec<(usize, &'a [RuleId], String)>,
}

impl<'a> Walk<'a> {
    /// The patterns of `list`, whose patterns take ids from `context`.
    ///
    /// A match named after its first group counts as one entry for each
    /// alternative.
    fn list(&mut self, list: &'a [RuleId], context: &str) -> Result<Vec<usize>, GenerateError> {
        let grammar = self.grammar;
        let mut out = Vec::new();
        let mut index = 0;
        for &rule in list {
            let object: &'a UnlinkedRule = &grammar.rules[rule];
            let Some(include) = &object.include else {
                let split = match object.matched.as_deref() {
                    Some(matched) => split::alternatives(
                        matched,
                        &object.name,
                        self.language.split,
                        &self.inline_id(context, index),
                    )?,
                    None => None,
                };
                if let Some(regexes) = split {
                    for regex in regexes {
                        let id = self.inline_id(context, index);
                        out.push(self.register_match(rule, &id, regex)?);
                        index += 1;
                    }
                } else {
                    let id = self.inline_id(context, index);
                    out.extend(self.expand(rule, &id)?);
                    index += 1;
                }
                continue;
            };
            let at = index;
            index += 1;
            if include == "$self" || *include == grammar.scope {
                if self.expanding.insert(TOP_LEVEL.to_owned()) {
                    out.extend(self.list(&grammar.patterns, "Root")?);
                    self.expanding.remove(TOP_LEVEL);
                }
                continue;
            }
            let external = self
                .language
                .externals
                .iter()
                .find(|&&(name, _)| name == &**include);
            if let Some(&(_, external)) = external {
                match external {
                    External::AddsNothing => {}
                    External::Begin { begin, end } => {
                        let id = self.inline_id(context, at);
                        out.push(self.made_begin(&id, begin, end)?);
                    }
                }
                continue;
            }
            let target = include
                .strip_prefix('#')
                .and_then(|key| Some((key, entry(&grammar.repository, key)?)));
            let Some((key, target)) = target else {
                if include.starts_with('#') {
                    continue;
                }
                return Err(GenerateError::Include {
                    rule: context.to_owned(),
                    include: include.to_string(),
                });
            };
            let id = pascal(key, self.language.key_spellings);
            if acts(&grammar.rules[target]) {
                out.extend(self.expand(target, &id)?);
            } else if self.expanding.insert(id.clone()) {
                out.extend(self.expand(target, &id)?);
                self.expanding.remove(&id);
            }
        }
        Ok(out)
    }

    /// The patterns of a rule's own list, `list`, whose patterns take ids
    /// from `context`.
    /// Under [`Reentry::WithinRule`], the lists being expanded around the
    /// rule do not count inside it.
    fn rule_list(
        &mut self,
        list: &'a [RuleId],
        context: &str,
    ) -> Result<Vec<usize>, GenerateError> {
        match self.language.reentry {
            Reentry::WithinRule => {
                let around = std::mem::take(&mut self.expanding);
                let patterns = self.list(list, context);
                self.expanding = around;
                patterns
            }
            Reentry::WithinWalk => self.list(list, context),
        }
    }

    /// The id of the inline pattern at `index` in the list `context`: the
    /// language's name for it, or `<context>At<index>`.
    fn inline_id(&self, context: &str, index: usize) -> String {
        let id = format!("{context}At{index}");
        self.language
            .pattern_names
            .iter()
            .find(|&&(derived, _)| derived == id)
            .map_or(id, |&(_, name)| name.to_owned())
    }

    /// The patterns of the rule object `rule`, named `id`.
    fn expand(&mut self, rule: RuleId, id: &str) -> Result<Vec<usize>, GenerateError> {
        let object: &'a UnlinkedRule = &self.grammar.rules[rule];
        if acts(object) {
            return Ok(vec![self.register(rule, id)?]);
        }
        if let [lone] = *object.patterns {
            let takes_id = match self.language.lone_pattern {
                LonePattern::OwnId => false,
                LonePattern::ListId => {
                    let object = &self.grammar.rules[lone];
                    object.include.is_none() && acts(object)
                }
            };
            if takes_id {
                return Ok(vec![self.register(lone, id)?]);
            }
        }
        if object.has_patterns {
            return self.list(&object.patterns, id);
        }
        Err(GenerateError::NoAct { id: id.to_owned() })
    }

    /// The index of the match of `rule` with the expression `regex`, named
    /// `id`, which it adds the first time.
    fn register_match(
        &mut self,
        rule: RuleId,
        id: &str,
        regex: String,
    ) -> Result<usize, GenerateError> {
        if let Some(known) = self.known(id, &regex, Act::Match)? {
            return Ok(known);
        }
        self.captures(self.grammar.rules[rule].captures.as_deref())?;
        self.put(id.to_owned(), regex, Act::Match)
    }

    /// The index of the pattern of `rule`, named `id`, which it adds the first
    /// time.
    /// A begin also adds its end, then its rule.
    /// A rule with neither a match nor a begin with an end is
    /// [`GenerateError::NoAct`], before any id is looked up.
    /// `expand` calls it only for a rule with an expression, so a
    /// list never takes a pattern's id.
    ///
    /// A begin's capture lists can reach the begin itself.
    /// The begin, its end and its rule are then written before the lists
    /// return, and the begin returns that index.
    fn register(&mut self, rule: RuleId, id: &str) -> Result<usize, GenerateError> {
        let object: &'a UnlinkedRule = &self.grammar.rules[rule];
        if let Some(matched) = &object.matched {
            return self.register_match(rule, id, matched.to_string());
        }
        let (Some(begin), Some(end)) = (&object.begin, &object.end) else {
            return Err(GenerateError::NoAct { id: id.to_owned() });
        };
        if let Some(known) = self.known(id, begin, Act::Begin)? {
            return Ok(known);
        }
        self.captures(
            object
                .begin_captures
                .as_deref()
                .or(object.captures.as_deref()),
        )?;
        if let Some(known) = self.known(id, begin, Act::Begin)? {
            return Ok(known);
        }
        let opened = self.put(id.to_owned(), begin.to_string(), Act::Begin)?;
        self.captures(
            object
                .end_captures
                .as_deref()
                .or(object.captures.as_deref()),
        )?;
        let closed = self.put(format!("{id}End"), end.to_string(), Act::End)?;
        let index = self.put_rule(id, Some(closed))?;
        self.opened.push((index, &object.patterns, id.to_owned()));
        self.tables.rules[index].patterns = self.rule_list(&object.patterns, id)?;
        Ok(opened)
    }

    /// The index of a begin that the language makes up, named `id`, which it
    /// adds the first time with its end and a rule with no patterns.
    fn made_begin(&mut self, id: &str, begin: &str, end: &str) -> Result<usize, GenerateError> {
        if let Some(known) = self.known(id, begin, Act::Begin)? {
            return Ok(known);
        }
        let opened = self.put(id.to_owned(), begin.to_owned(), Act::Begin)?;
        let closed = self.put(format!("{id}End"), end.to_owned(), Act::End)?;
        self.put_rule(id, Some(closed))?;
        Ok(opened)
    }

    /// The index of the pattern `id`, or `None` when no pattern has that id
    /// yet.
    ///
    /// # Errors
    ///
    /// [`GenerateError::IdTaken`] when the pattern `id` has another
    /// expression than `regex`, or another act than `act`.
    fn known(&self, id: &str, regex: &str, act: Act) -> Result<Option<usize>, GenerateError> {
        let Some(&index) = self.patterns.get(id) else {
            return Ok(None);
        };
        let pattern = &self.tables.patterns[index];
        if pattern.regex != regex || pattern.act != act {
            return Err(GenerateError::IdTaken { id: id.to_owned() });
        }
        Ok(Some(index))
    }

    /// Writes a pattern, and returns its index.
    ///
    /// A pattern written again unchanged keeps its first place.
    /// A capture list can reach a match while the match waits for its
    /// captures, and the match then keeps the place the list gave it.
    ///
    /// # Errors
    ///
    /// [`GenerateError::IdTaken`] when the id names another expression or
    /// act, such as a begin's end id `FooEnd` and a repository entry
    /// `foo-end`.
    fn put(&mut self, id: String, regex: String, act: Act) -> Result<usize, GenerateError> {
        if let Some(known) = self.known(&id, &regex, act)? {
            return Ok(known);
        }
        let index = self.tables.patterns.len();
        self.patterns.insert(id.clone(), index);
        self.tables.patterns.push(Pattern { id, regex, act });
        Ok(index)
    }

    /// Writes an empty rule, and returns its index.
    ///
    /// # Errors
    ///
    /// [`GenerateError::IdTaken`] when a rule already has the id, such as a
    /// capture list's `Nested0` and a begin under the key `nested0`.
    fn put_rule(&mut self, id: &str, end: Option<usize>) -> Result<usize, GenerateError> {
        if self.tables.rules.iter().any(|known| known.id == id) {
            return Err(GenerateError::IdTaken { id: id.to_owned() });
        }
        self.tables.rules.push(Rule {
            id: id.to_owned(),
            end,
            patterns: Vec::new(),
        });
        Ok(self.tables.rules.len() - 1)
    }

    /// Adds a rule for each capture whose own pattern list is new, in group
    /// order, when the language reads such lists as rules.
    fn captures(&mut self, captures: Option<&[UnlinkedCapture]>) -> Result<(), GenerateError> {
        match self.language.capture_lists {
            CaptureLists::ByHand => Ok(()),
            CaptureLists::Rules => {
                let mut groups = captures
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|capture| Some((capture.index?, capture.rule)))
                    .filter(|&(_, rule)| self.grammar.rules[rule].has_patterns)
                    .collect::<Vec<_>>();
                groups.sort_by_key(|&(group, _)| group);
                for (_, rule) in groups {
                    self.capture_list(rule)?;
                }
                Ok(())
            }
        }
    }

    /// The rule of the pattern list of `capture`, which it adds the first
    /// time a list with its text is found.
    fn capture_list(&mut self, capture: RuleId) -> Result<(), GenerateError> {
        let list: &'a [RuleId] = &self.grammar.rules[capture].patterns;
        let mut text = String::new();
        for &rule in list {
            shape(self.grammar, rule, &mut text);
        }
        if self.capture_lists.contains_key(&text) {
            return Ok(());
        }
        let id = format!("Nested{}", self.capture_lists.len());
        self.capture_lists.insert(text, id.clone());
        let index = self.put_rule(&id, None)?;
        self.opened.push((index, list, id.clone()));
        self.tables.rules[index].patterns = self.rule_list(list, &id)?;
        Ok(())
    }
}

/// Whether `object` has an expression of its own, a `match` or a `begin`.
const fn acts(object: &UnlinkedRule) -> bool {
    object.matched.is_some() || object.begin.is_some()
}

/// The rule object of the repository entry `key`.
fn entry(repository: &[(Text, RuleId)], key: &str) -> Option<RuleId> {
    repository
        .binary_search_by(|(name, _)| name.as_ref().cmp(key))
        .ok()
        .map(|found| repository[found].1)
}

/// A repository key in Pascal case: `string-quoted` gives `StringQuoted`.
///
/// It first respells the first place each of `spellings` names.
/// It then splits at `-`, `_` and `.`, and raises the first letter of each
/// part.
fn pascal(key: &str, spellings: &[(&str, &str)]) -> String {
    let mut key = key.to_owned();
    for (wrong, right) in spellings {
        key = key.replacen(wrong, right, 1);
    }
    key.split(['-', '_', '.'])
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}

/// Appends a text that two rule objects share only when their keys and values
/// are equal, nested rules included.
///
/// Two capture lists with equal text become one rule, as the scripts compare
/// them by their JSON.
fn shape(grammar: &UnlinkedGrammar, rule: RuleId, out: &mut String) {
    let object = &grammar.rules[rule];
    let _ = write!(
        out,
        "({:?} {:?} {:?} {:?} {:?} {:?} {:?} {} {}",
        object.include,
        object.name,
        object.content_name,
        object.matched,
        object.begin,
        object.end,
        object.until,
        object.apply_end_last,
        object.has_patterns,
    );
    for captures in [
        &object.captures,
        &object.begin_captures,
        &object.end_captures,
        &object.while_captures,
    ] {
        out.push('[');
        for capture in captures.as_deref().unwrap_or_default() {
            let _ = write!(out, "{:?}:", capture.index);
            shape(grammar, capture.rule, out);
        }
        out.push(']');
    }
    out.push('[');
    for &pattern in object.patterns.iter() {
        shape(grammar, pattern, out);
    }
    out.push(']');
    for (key, entry) in object.repository.as_deref().unwrap_or_default() {
        let _ = write!(out, "{key:?}=");
        shape(grammar, *entry, out);
    }
    out.push(')');
}
