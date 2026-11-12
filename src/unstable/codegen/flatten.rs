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
//! A match or begin named after a group, such as `$1`, becomes one pattern
//! for each alternative of the group.
//! In a list, each takes its own place.
//! A repository entry's patterns take the entry's id, `At` and their
//! position, such as `FontFeaturesAt2`.
//! [`RuleTables::capture_names`] sets what a match becomes when only a
//! capture's name holds a group's text.
//!
//! A rule a begin opens takes the begin's id, and lists its own patterns.
//! An entry with patterns and no expression adds its patterns in its place.
//! `$self`, or the grammar's own scope name, adds the top-level patterns.
//! An include of another grammar adds what [`RuleTables::externals`] says.
//! A list being expanded, `Root` included, adds nothing when it is reached
//! again.
//!
//! The matches another grammar injects, [`RuleTables::injected`], follow
//! every other pattern.
//! A rule of [`RuleTables::added_rules`] tries the patterns of a list in
//! another grammar, walked after `Root`'s list.
//! An include of the language's own scope with a key, such as
//! `source.css#escapes`, adds the language's own entry, in any list.
//!
//! A pattern reached again keeps its first id, so an include repeats no
//! pattern.
//! [`RuleTables::identity`] sets whether the walk finds it again by its id
//! or by its contents.
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
    Act, AddedRule, CaptureLists, CaptureNames, External, Identity, Injection, LonePattern,
    Pattern, Reentry, Rule, RuleTables, Tables,
};
use crate::textmate::compile::{RuleId, Text, UnlinkedCapture, UnlinkedGrammar, UnlinkedRule};

/// The tables of `grammar`, read as `language` says.
///
/// `others` holds each grammar that an [`External::Pattern`], the
/// [`Injection`] or an [`AddedRule`] of `language` names, by file name.
///
/// # Errors
///
/// [`GenerateError`] when an include names another grammar, when a rule has
/// nothing to match, or when two patterns or rules take one id.
pub fn flatten(
    grammar: &UnlinkedGrammar,
    others: &[(&str, &UnlinkedGrammar)],
    language: &RuleTables,
) -> Result<Tables, GenerateError> {
    let mut walk = Walk {
        grammar,
        own: grammar,
        others,
        language,
        tables: Tables::default(),
        patterns: HashMap::new(),
        capture_lists: HashMap::new(),
        contents: HashMap::new(),
        expanding: HashSet::new(),
        expanding_top_level: false,
        opened: Vec::new(),
    };
    walk.expanding_top_level = language.reentry == Reentry::WithinRule;
    let first = walk.list(&grammar.patterns, "Root")?;
    for &added in language.added_rules {
        walk.add_rule(added)?;
    }
    let root = match language.reentry {
        Reentry::WithinRule => first,
        Reentry::WithinWalk => {
            for (index, list_grammar, list, id) in std::mem::take(&mut walk.opened) {
                walk.expanding.clear();
                walk.expanding_top_level = false;
                walk.grammar = list_grammar;
                let patterns = walk.list(list, &id);
                walk.grammar = grammar;
                walk.tables.rules[index].patterns = patterns?;
            }
            walk.expanding.clear();
            walk.expanding_top_level = true;
            walk.list(&grammar.patterns, "Root")?
        }
    };
    if let Some(injection) = language.injected {
        walk.inject(injection)?;
    }
    let index = walk.put_rule("Root", None)?;
    walk.tables.rules[index].patterns = root;
    Ok(walk.tables)
}

/// The state of one walk over a grammar.
#[derive(Debug)]
struct Walk<'a> {
    /// The grammar whose rule objects the walk reads now: the language's
    /// own, or the grammar of a rule in [`RuleTables::added_rules`].
    grammar: &'a UnlinkedGrammar,
    /// The language's own grammar.
    own: &'a UnlinkedGrammar,
    /// The grammars that [`External::Pattern`], [`Injection`] and
    /// [`AddedRule`] read, by file name.
    others: &'a [(&'a str, &'a UnlinkedGrammar)],
    /// What the language does differently.
    language: &'a RuleTables,
    /// The tables so far.
    tables: Tables,
    /// Each pattern's index in the tables, by id.
    patterns: HashMap<String, usize>,
    /// Each capture list's rule id, by the list's text.
    capture_lists: HashMap<String, String>,
    /// Each pattern's index in the tables by its rule object's text, under
    /// [`Identity::Content`].
    /// A dropped rule's begin maps to `None`.
    contents: HashMap<String, Option<usize>>,
    /// The ids of the lists being expanded, in the scope
    /// [`RuleTables::reentry`] sets.
    expanding: HashSet<String>,
    /// Whether the top-level list is being expanded, in the same scope.
    /// It stays out of [`Walk::expanding`], so no repository key, such as
    /// `root` or `$self`, can share it.
    expanding_top_level: bool,
    /// Each rule the walk opened: its index in the tables, the grammar that
    /// holds its pattern list, the list and its id, for the second pass of
    /// [`Reentry::WithinWalk`].
    opened: Vec<(usize, &'a UnlinkedGrammar, &'a [RuleId], String)>,
}

impl<'a> Walk<'a> {
    /// The patterns of `list`, whose patterns take ids from `context`.
    ///
    /// A match or begin named after a group counts as one entry for each
    /// alternative.
    fn list(&mut self, list: &'a [RuleId], context: &str) -> Result<Vec<usize>, GenerateError> {
        let grammar = self.grammar;
        let mut out = Vec::new();
        let mut index = 0;
        for &rule in list {
            let object: &'a UnlinkedRule = &grammar.rules[rule];
            let Some(include) = &object.include else {
                if self.drops(object) {
                    continue;
                }
                if let Some(regexes) = self.variants(rule, &self.inline_id(context, index))? {
                    for regex in regexes {
                        let id = self.inline_id(context, index);
                        out.extend(self.register(rule, &id, Some(regex))?);
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
                if !self.expanding_top_level {
                    self.expanding_top_level = true;
                    out.extend(self.list(&grammar.patterns, "Root")?);
                    self.expanding_top_level = false;
                }
                continue;
            }
            if let Some(external) = self.external(include) {
                match external {
                    External::AddsNothing
                    | External::Embeds
                    | External::Missing
                    | External::DropsItsEntry
                    | External::CaptureRule { .. } => {}
                    External::Begin { begin, end } => {
                        let id = self.inline_id(context, at);
                        out.push(self.made_begin(&id, begin, end)?);
                    }
                    External::Pattern { grammar: name, key } => {
                        out.push(self.other_match(include, name, key)?);
                    }
                }
                continue;
            }
            let own_key = include
                .strip_prefix(&*self.own.scope)
                .and_then(|rest| rest.strip_prefix('#'));
            if let Some(key) = own_key {
                let walked = std::mem::replace(&mut self.grammar, self.own);
                let patterns = self.key(key);
                self.grammar = walked;
                out.extend(patterns?);
                continue;
            }
            let Some(key) = include.strip_prefix('#') else {
                return Err(GenerateError::Include {
                    rule: context.to_owned(),
                    include: include.to_string(),
                });
            };
            out.extend(self.key(key)?);
        }
        Ok(out)
    }

    /// The patterns that the repository entry `key` of the grammar being
    /// walked adds.
    ///
    /// A key the grammar lacks adds nothing, as vscode-textmate skips such
    /// an include.
    /// So does an entry with no expression of its own while it is being
    /// expanded.
    fn key(&mut self, key: &str) -> Result<Vec<usize>, GenerateError> {
        let grammar = self.grammar;
        let Some(target) = entry(&grammar.repository, key) else {
            return Ok(Vec::new());
        };
        let id = pascal(key, self.language.key_spellings);
        if acts(&grammar.rules[target]) {
            return self.expand(target, &id);
        }
        if !self.expanding.insert(id.clone()) {
            return Ok(Vec::new());
        }
        let patterns = self.expand(target, &id);
        self.expanding.remove(&id);
        patterns
    }

    /// Adds the rule `added`, whose patterns are a list in another grammar.
    ///
    /// The walk reads that list's rule objects from the other grammar.
    /// An include of this grammar's scope with a key, such as
    /// `source.css#escapes`, adds this grammar's entry.
    ///
    /// # Errors
    ///
    /// [`GenerateError::AddedRule`] when `others` lacks the grammar, the
    /// grammar lacks the key, or a position along the path is missing.
    fn add_rule(&mut self, added: AddedRule) -> Result<(), GenerateError> {
        let failed = || GenerateError::AddedRule {
            id: added.id.to_owned(),
        };
        let other = self.other(added.grammar).ok_or_else(failed)?;
        let mut rule = entry(&other.repository, added.key).ok_or_else(failed)?;
        for &position in added.path {
            rule = *other.rules[rule]
                .patterns
                .get(position)
                .ok_or_else(failed)?;
        }
        let list: &'a [RuleId] = &other.rules[rule].patterns;
        let index = self.put_rule(added.id, None)?;
        self.opened.push((index, other, list, added.id.to_owned()));
        let walked = std::mem::replace(&mut self.grammar, other);
        let patterns = self.rule_list(list, added.id);
        self.grammar = walked;
        self.tables.rules[index].patterns = patterns?;
        Ok(())
    }

    /// What the include of another grammar `include` adds, from
    /// [`RuleTables::externals`].
    fn external(&self, include: &str) -> Option<External> {
        self.language
            .externals
            .iter()
            .find(|&&(name, _)| name == include)
            .map(|&(_, external)| external)
    }

    /// The index of the match under `key` in the grammar `name`, which it
    /// adds the first time.
    ///
    /// # Errors
    ///
    /// [`GenerateError::ExternalPattern`] when `others` lacks the grammar,
    /// the grammar lacks the key, or the entry is not a match without capture
    /// lists.
    fn other_match(
        &mut self,
        include: &str,
        name: &str,
        key: &str,
    ) -> Result<usize, GenerateError> {
        let failed = || GenerateError::ExternalPattern {
            include: include.to_owned(),
        };
        let other = self.other(name).ok_or_else(failed)?;
        let object = &other.rules[entry(&other.repository, key).ok_or_else(failed)?];
        let lists = object
            .captures
            .as_deref()
            .unwrap_or_default()
            .iter()
            .any(|capture| other.rules[capture.rule].has_patterns);
        let Some(matched) = object.matched.as_deref().filter(|_| !lists) else {
            return Err(failed());
        };
        let id = format!("{}{}", pascal(name, &[]), pascal(key, &[]));
        self.put(id, matched.to_string(), Act::Match)
    }

    /// The grammar of `others` named `name`.
    fn other(&self, name: &str) -> Option<&'a UnlinkedGrammar> {
        self.others
            .iter()
            .find(|&&(file, _)| file == name)
            .map(|&(_, other)| other)
    }

    /// Adds each match that `injection` names, after every other pattern.
    ///
    /// # Errors
    ///
    /// [`GenerateError::Injection`] when `others` lacks the grammar, the
    /// grammar has no injection under the selector, or the injection is not
    /// one match for each id.
    fn inject(&mut self, injection: Injection) -> Result<(), GenerateError> {
        let failed = || GenerateError::Injection {
            selector: injection.selector.to_owned(),
        };
        let other = self.other(injection.grammar).ok_or_else(failed)?;
        let &(_, rule) = other
            .injections
            .iter()
            .find(|(selector, _)| &**selector == injection.selector)
            .ok_or_else(failed)?;
        let patterns = &other.rules[rule].patterns;
        if patterns.len() != injection.ids.len() {
            return Err(failed());
        }
        for (&pattern, id) in patterns.iter().zip(injection.ids) {
            let matched = other.rules[pattern].matched.as_deref().ok_or_else(failed)?;
            self.put((*id).to_owned(), matched.to_owned(), Act::Match)?;
        }
        Ok(())
    }

    /// Whether the list entry `object` is left out of its list: its own
    /// patterns are one [`External::DropsItsEntry`] include.
    fn drops(&self, object: &UnlinkedRule) -> bool {
        let [only] = *object.patterns else {
            return false;
        };
        self.grammar.rules[only]
            .include
            .as_deref()
            .and_then(|include| self.external(include))
            == Some(External::DropsItsEntry)
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
                let top_level = std::mem::replace(&mut self.expanding_top_level, false);
                let patterns = self.list(list, context);
                self.expanding = around;
                self.expanding_top_level = top_level;
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
            return self.acting(rule, id);
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
                return self.acting(lone, id);
            }
        }
        if object.has_patterns {
            return self.list(&object.patterns, id);
        }
        Err(GenerateError::NoAct { id: id.to_owned() })
    }

    /// The patterns of `rule`, a match or begin, named `id`.
    ///
    /// A rule named after a group gives one pattern for each alternative,
    /// as a list of them would.
    /// They take the ids `<id>At0`, `<id>At1` and so on.
    fn acting(&mut self, rule: RuleId, id: &str) -> Result<Vec<usize>, GenerateError> {
        let Some(regexes) = self.variants(rule, id)? else {
            return Ok(self.register(rule, id, None)?.into_iter().collect());
        };
        let mut out = Vec::new();
        for (index, regex) in regexes.into_iter().enumerate() {
            let variant = self.inline_id(id, index);
            out.extend(self.register(rule, &variant, Some(regex))?);
        }
        Ok(out)
    }

    /// The expression of each pattern that `rule` becomes when its name or
    /// content name holds a group's text, or `None`.
    ///
    /// A match whose capture names alone hold one becomes what
    /// [`RuleTables::capture_names`] says.
    ///
    /// # Errors
    ///
    /// [`GenerateError::Split`], naming `id`, when the split cannot read the
    /// group.
    fn variants(&self, rule: RuleId, id: &str) -> Result<Option<Vec<String>>, GenerateError> {
        let object = &self.grammar.rules[rule];
        let Some(regex) = object.matched.as_deref().or(object.begin.as_deref()) else {
            return Ok(None);
        };
        let names = object.name.iter().chain(object.content_name.iter());
        let split = split::alternatives(regex, names.map(|name| &**name), self.language.split, id)?;
        match (
            split,
            self.language.capture_names,
            object.matched.as_deref(),
        ) {
            (Some(split), _, _) => Ok(Some(split)),
            (None, CaptureNames::SplitsOptionalGroup, Some(matched)) => {
                let names = object
                    .captures
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .flat_map(|capture| self.grammar.rules[capture.rule].name.iter());
                split::optional_alternatives(matched, names.map(|name| &**name), id)
            }
            (None, CaptureNames::Kept | CaptureNames::SplitsOptionalGroup, _) => Ok(None),
        }
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
    /// time, or `None` for a rule the walk drops.
    /// A begin also adds its end, then its rule.
    ///
    /// A rule with neither a match nor a begin with an end is
    /// [`GenerateError::NoAct`], before any id is looked up.
    /// `expand` calls it only for a rule with an expression, so a
    /// list never takes a pattern's id.
    ///
    /// A begin's capture lists can reach the begin itself.
    /// The begin, its end and its rule are then written before the lists
    /// return, and the begin returns that index.
    ///
    /// A rule whose own list holds an [`External::Embeds`] include can then
    /// list a body.
    /// One whose own list holds an [`External::Missing`] include can be
    /// dropped.
    ///
    /// `variant`, when given, replaces the rule's match or begin, for one
    /// alternative of a split.
    /// The walk finds such a pattern again by its id only, since every
    /// alternative has the rule's contents.
    fn register(
        &mut self,
        rule: RuleId,
        id: &str,
        variant: Option<String>,
    ) -> Result<Option<usize>, GenerateError> {
        let object: &'a UnlinkedRule = &self.grammar.rules[rule];
        let content = match (self.language.identity, &variant) {
            (Identity::Id, _) | (Identity::Content, Some(_)) => None,
            (Identity::Content, None) => {
                let mut text = String::new();
                shape(self.grammar, rule, &mut text);
                if let Some(&known) = self.contents.get(&text) {
                    return Ok(known);
                }
                Some(text)
            }
        };
        if let Some(matched) = &object.matched {
            let regex = variant.unwrap_or_else(|| matched.to_string());
            let index = self.register_match(rule, id, regex)?;
            if let Some(text) = content {
                self.contents.insert(text, Some(index));
            }
            return Ok(Some(index));
        }
        let (Some(begin), Some(end)) = (&object.begin, &object.end) else {
            return Err(GenerateError::NoAct { id: id.to_owned() });
        };
        let begin = variant.unwrap_or_else(|| begin.to_string());
        if let Some(known) = self.known(id, &begin, Act::Begin)? {
            return Ok(Some(known));
        }
        self.captures(
            object
                .begin_captures
                .as_deref()
                .or(object.captures.as_deref()),
        )?;
        if let Some(known) = self.known(id, &begin, Act::Begin)? {
            return Ok(Some(known));
        }
        let rules_before = self.tables.rules.len();
        let opened = self.put(id.to_owned(), begin, Act::Begin)?;
        if let Some(text) = content {
            self.contents.insert(text, Some(opened));
        }
        self.captures(
            object
                .end_captures
                .as_deref()
                .or(object.captures.as_deref()),
        )?;
        let closed = self.put(format!("{id}End"), end.to_string(), Act::End)?;
        let index = self.put_rule(id, Some(closed))?;
        self.opened
            .push((index, self.grammar, &object.patterns, id.to_owned()));
        let patterns = self.rule_list(&object.patterns, id)?;
        let own = object
            .patterns
            .iter()
            .filter_map(|&pattern| self.external(self.grammar.rules[pattern].include.as_deref()?))
            .collect::<Vec<_>>();
        let embeds = own.contains(&External::Embeds);
        if !embeds && patterns.is_empty() && own.contains(&External::Missing) {
            if rules_before != index {
                return Err(GenerateError::DroppedBeginCaptures { id: id.to_owned() });
            }
            self.drop_begin(opened, index);
            return Ok(None);
        }
        if embeds && one_backref(end) {
            if !patterns.is_empty() {
                return Err(GenerateError::EmbedsBesidePatterns { id: id.to_owned() });
            }
            let body = self.made_begin(&format!("{id}Body"), "\\G", &format!("(?={end})"))?;
            self.tables.rules[index].patterns = vec![body];
            self.opened.retain(|&(at, ..)| at != index);
            return Ok(Some(opened));
        }
        self.tables.rules[index].patterns = patterns;
        Ok(Some(opened))
    }

    /// Drops the begin at the index `opened`, its end, and its rule at the
    /// index `rule`, with what the walk keeps about them.
    /// The dropped begin's contents then map to `None`.
    ///
    /// They are the last two patterns and the last rule written.
    /// A rule the begin's list reached was dropped before it, and its end
    /// captures added no rule.
    fn drop_begin(&mut self, opened: usize, rule: usize) {
        debug_assert_eq!(self.tables.patterns.len(), opened + 2, "begin and end last");
        debug_assert_eq!(self.tables.rules.len(), rule + 1, "the begin's rule last");
        self.tables.patterns.truncate(opened);
        self.tables.rules.truncate(rule);
        self.patterns.retain(|_, &mut index| index < opened);
        for known in self.contents.values_mut() {
            if *known == Some(opened) {
                *known = None;
            }
        }
        self.opened.retain(|&(at, ..)| at < rule);
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
            CaptureLists::Rules | CaptureLists::NamedRules => {
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
    ///
    /// A list of one [`External::CaptureRule`] include becomes that rule,
    /// with no patterns.
    /// Under [`CaptureLists::NamedRules`], a list of one include of a
    /// repository key takes the key's id.
    fn capture_list(&mut self, capture: RuleId) -> Result<(), GenerateError> {
        let list: &'a [RuleId] = &self.grammar.rules[capture].patterns;
        let mut text = String::new();
        for &rule in list {
            shape(self.grammar, rule, &mut text);
        }
        if self.capture_lists.contains_key(&text) {
            return Ok(());
        }
        let lone = match *list {
            [only] => self.grammar.rules[only].include.as_deref(),
            _ => None,
        };
        if let Some(External::CaptureRule { id }) = lone.and_then(|include| self.external(include))
        {
            self.capture_lists.insert(text, id.to_owned());
            self.put_rule(id, None)?;
            return Ok(());
        }
        let key = lone.and_then(|include| include.strip_prefix('#'));
        let id = match (self.language.capture_lists, key) {
            (CaptureLists::NamedRules, Some(key)) => pascal(key, self.language.key_spellings),
            (CaptureLists::NamedRules | CaptureLists::Rules | CaptureLists::ByHand, _) => {
                format!("Nested{}", self.capture_lists.len())
            }
        };
        self.capture_lists.insert(text, id.clone());
        let index = self.put_rule(&id, None)?;
        self.opened.push((index, self.grammar, list, id.clone()));
        self.tables.rules[index].patterns = self.rule_list(list, &id)?;
        Ok(())
    }
}

/// Whether `object` has an expression of its own, a `match` or a `begin`.
const fn acts(object: &UnlinkedRule) -> bool {
    object.matched.is_some() || object.begin.is_some()
}

/// Whether the end `end` repeats exactly one group of its begin, such as
/// `\1` in `^(?!\1\s+)`.
///
/// A group's number is every digit after the `\`, so `\10` repeats group
/// 10.
/// vscode-textmate reads an end the same way, with `\\(\d+)`.
/// It leaves `\k<name>` alone, so that form names a group of the end
/// itself and repeats no group of the begin.
/// An escaped backslash before a digit, as in `\\1`, repeats no group.
fn one_backref(end: &str) -> bool {
    let bytes = end.as_bytes();
    let mut groups = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            index += 1;
            continue;
        }
        let digits = end[index + 1..]
            .bytes()
            .take_while(u8::is_ascii_digit)
            .count();
        if digits > 0 {
            groups.push(&end[index + 1..=index + digits]);
        }
        index += 1 + digits.max(1);
    }
    groups.sort_unstable();
    groups.dedup();
    groups.len() == 1
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
