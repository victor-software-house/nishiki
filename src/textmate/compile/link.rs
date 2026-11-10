//! Linking: a root grammar, the grammars its includes can name and the
//! languages lexed by hand become one flat [`Grammar`].
//!
//! It follows `oracle/engine-tables.ts` step for step, so the tables are
//! equal to its tables, numbering included:
//!
//! 1. It compiles rules depth first from the root, as vscode-textmate does. A
//!    rule takes its number when it is first reached. A rule object is compiled
//!    once, so a cycle of includes is followed once.
//! 2. It adds the root's injections, then the grammars that inject themselves
//!    into the root.
//! 3. It replaces each list's includes with the rules they stand for.
//! 4. It interns the scope names and the lists, and writes the tables.
//!
//! An include of another grammar's scope resolves in this order:
//!
//! 1. a language in the hand-off list, as a hand-off, even when the set holds
//!    its grammar;
//! 2. a grammar in the set;
//! 3. else the include is dropped, as vscode-textmate drops an include of a
//!    grammar it has not loaded.
//!
//! An include of one rule of another grammar, such as
//! `source.ruby#escaped_char`, resolves in the set, a handed grammar
//! included.
//! A handed grammar serves only such includes.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use super::{Compiled, RawCapture, Repository, RuleId, hand_lexed};
use crate::language::Language;
use crate::textmate::grammar::{
    Cap, DYNAMIC_NAME, Folding, Grammar, HandOff, Injection, List, NONE, Pattern, Rule, RuleKind,
};
use crate::textmate::regex::Regex;

/// The most memory, in bytes, that the lazy DFA of one of a linked
/// grammar's scanners may hold in a cache.
/// It is the 4 MiB that `oracle/engine-tables.ts` writes by default.
const DFA_CACHE: usize = 1 << 22;

/// How deep rules may include each other while the link compiles them.
///
/// The link recurses once per level.
/// A repository can chain any number of entries, each including the next,
/// with no JSON nesting for the parser's own limit to stop.
/// The bound keeps such a grammar from overflowing the stack of a 2 MiB
/// thread.
/// The deepest grammar Shiki 4.6.0 bundles, `typst`, reaches 63.
const MAX_DEPTH: usize = 256;

/// The most hand-offs a grammar can hold.
/// The tokenizer looks a list's hand-off up as a `u8`, one load a step.
const MAX_HAND_OFFS: usize = 256;

/// Why a grammar did not link.
///
/// Each is a case `oracle/engine-tables.ts` refuses too.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LinkError {
    /// The hand-off list names a language no grammar can hand off to.
    NotHandLexed(Language),
    /// A list of rules includes two hand-lexed grammars.
    TwoHandOffs,
    /// A pattern that ends a hand-off's region uses `\G`.
    /// The tokenizer finds where such patterns start before the hand-lexed
    /// language runs, from places where `\G` cannot be known.
    AnchoredHalt(String),
    /// The grammar holds more hand-offs than a `u8` can number.
    TooManyHandOffs,
    /// The grammar names more scopes than a scope reference can number.
    TooManyNames,
    /// Rules include each other more than [`MAX_DEPTH`] deep, as a
    /// repository whose every entry includes the next does.
    TooDeep,
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::NotHandLexed(language) => {
                write!(f, "no grammar can hand a region to {}", language.id())
            }
            LinkError::TwoHandOffs => f.write_str("a list of rules includes two hand-offs"),
            LinkError::AnchoredHalt(pattern) => {
                write!(
                    f,
                    "a hand-off's region ends with a pattern that uses \\G: {pattern}"
                )
            }
            LinkError::TooManyHandOffs => {
                write!(f, "the grammar holds more than {MAX_HAND_OFFS} hand-offs")
            }
            LinkError::TooManyNames => f.write_str("the grammar names too many scopes"),
            LinkError::TooDeep => {
                write!(f, "rules include each other more than {MAX_DEPTH} deep")
            }
        }
    }
}

impl std::error::Error for LinkError {}

/// Links `root` with the grammars of `set` into the engine's tables.
///
/// The tables are leaked, so the tokenizer can key its caches on their
/// address.
///
/// `handed` lists the languages lexed by hand whose grammars are handed off.
/// With `None`, it is every such language whose grammar neither `root` nor
/// `set` holds.
pub(crate) fn link(
    root: &Compiled,
    set: &[&Compiled],
    handed: Option<&[Language]>,
) -> Result<&'static Grammar, LinkError> {
    let handed: Vec<(&str, Language)> = match handed {
        Some(languages) => languages
            .iter()
            .map(|&language| {
                hand_lexed::scope(language)
                    .map(|scope| (scope, language))
                    .ok_or(LinkError::NotHandLexed(language))
            })
            .collect::<Result<_, _>>()?,
        None => hand_lexed::all()
            .iter()
            .filter(|&&(scope, _)| root.scope != scope && set.iter().all(|g| g.scope != scope))
            .copied()
            .collect(),
    };
    let mut docs: Vec<(&str, Source)> = Vec::new();
    let mut fragments: Vec<(&str, Source)> = Vec::new();
    for (index, grammar) in set.iter().enumerate() {
        let scope = grammar.scope.as_str();
        let into = if handed.iter().any(|&(s, _)| s == scope) {
            &mut fragments
        } else {
            &mut docs
        };
        upsert(into, scope, Source::Set(index));
    }
    upsert(&mut docs, &root.scope, Source::Root);
    let mut builder = Builder {
        root,
        set,
        docs,
        fragments,
        handed,
        rules: Vec::new(),
        patterns: Vec::new(),
        pattern_ids: HashMap::new(),
        roots: HashMap::new(),
        compiled: HashMap::new(),
        hand_offs: Vec::new(),
        depth: 0,
        too_deep: false,
    };
    let root_rule = builder.root(&root.scope).unwrap_or(NONE);
    let mut injections: Vec<(&str, usize)> = root
        .injections
        .iter()
        .map(|(selector, rule)| {
            let env = [&root.repository];
            (selector.as_str(), builder.entry(Source::Root, *rule, &env))
        })
        .collect();
    let wrapped: Vec<&str> = root
        .patterns
        .iter()
        .filter_map(|&id| root.rules.get(id)?.include.as_deref())
        .filter(|include| {
            !include.is_empty()
                && !include.starts_with('#')
                && !include.starts_with('$')
                && !include.contains('#')
        })
        .collect();
    let docs = builder.docs.clone();
    for (scope, source) in docs {
        let grammar = builder.grammar(source);
        if scope == root.scope {
            continue;
        }
        let Some(selector) = &grammar.injection_selector else {
            continue;
        };
        let targeted = grammar
            .inject_to
            .iter()
            .any(|target| *target == root.scope || wrapped.contains(&target.as_str()));
        if !targeted {
            continue;
        }
        if let Some(rule) = builder.root(scope) {
            injections.push((selector.as_str(), rule));
        }
    }
    if builder.too_deep {
        return Err(LinkError::TooDeep);
    }
    builder.flatten();
    builder.finish(root_rule, &injections)
}

/// Puts `source` under `scope`, keeping the place of an earlier grammar of
/// the same scope, as a JavaScript `Map` does.
fn upsert<'a>(into: &mut Vec<(&'a str, Source)>, scope: &'a str, source: Source) {
    match into.iter_mut().find(|(s, _)| *s == scope) {
        Some(entry) => entry.1 = source,
        None => into.push((scope, source)),
    }
}

/// Where a grammar comes from: the root, or an index in the set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Source {
    /// The grammar being linked.
    Root,
    /// A grammar of the set, by index.
    Set(usize),
}

/// One entry of a list of rules while the link runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    /// A rule, by its number in the tables.
    Rule(usize),
    /// A hand-off, by its index in [`Builder::hand_offs`].
    HandOff(usize),
}

/// A rule while the link runs.
#[derive(Debug)]
struct Draft<'a> {
    /// What the rule does.
    kind: RuleKind,
    /// The scopes of its text.
    name: &'a [String],
    /// The scopes of the rules inside it.
    content_name: &'a [String],
    /// Its `begin` or `match` pattern, or [`NONE`].
    begin: usize,
    /// Its `end` pattern, or [`NONE`].
    end: usize,
    /// Its `while` pattern, or [`NONE`].
    until: usize,
    /// The groups of its `begin` or `match` pattern.
    captures: Vec<CapDraft<'a>>,
    /// The groups of its `end` or `while` pattern.
    end_captures: Vec<CapDraft<'a>>,
    /// Its list: includes followed once [`Builder::flatten`] has run.
    patterns: Vec<Entry>,
    /// Whether it applies its end after its patterns.
    apply_end_last: bool,
    /// Whether its list dropped an include.
    missing: bool,
}

/// A capture group while the link runs.
#[derive(Debug)]
struct CapDraft<'a> {
    /// The group's number, or [`NONE`].
    index: usize,
    /// The scopes of its text.
    name: &'a [String],
    /// The scopes of the rules inside it.
    content_name: &'a [String],
    /// The rule that lexes its text again, or [`NONE`].
    retokenize: usize,
}

/// A hand-off while its tables are written.
#[derive(Debug)]
struct HandOffRow {
    /// The list that held the include.
    list: usize,
    /// The language that lexes the region.
    language: Language,
    /// The list of the rules before the include.
    before: usize,
    /// The list of the rules after the include.
    after: usize,
}

/// The state of one link.
struct Builder<'a> {
    /// The grammar being linked.
    root: &'a Compiled,
    /// The grammars an include can name.
    set: &'a [&'a Compiled],
    /// The grammars whose whole rules an include can reach, by scope, in the
    /// order of the set.
    docs: Vec<(&'a str, Source)>,
    /// The handed grammars, which serve includes of one of their rules.
    fragments: Vec<(&'a str, Source)>,
    /// The scopes whose include is a hand-off, with the language.
    handed: Vec<(&'a str, Language)>,
    /// The rules, by number.
    rules: Vec<Draft<'a>>,
    /// The patterns, by number.
    patterns: Vec<&'a str>,
    /// The number of each pattern's text.
    pattern_ids: HashMap<&'a str, usize>,
    /// The root rule of each grammar compiled so far, by scope.
    roots: HashMap<&'a str, usize>,
    /// The number of each rule object compiled so far.
    compiled: HashMap<(Source, RuleId), usize>,
    /// The language of each hand-off.
    hand_offs: Vec<Language>,
    /// How many rules and grammar roots are being compiled, one inside the
    /// other.
    depth: usize,
    /// Whether the compilation went past [`MAX_DEPTH`].
    too_deep: bool,
}

/// The repositories an include can name, innermost last.
type Env<'a> = [&'a Repository];

impl<'a> Builder<'a> {
    /// The grammar of `source`.
    fn grammar(&self, source: Source) -> &'a Compiled {
        match source {
            Source::Root => self.root,
            Source::Set(index) => self.set.get(index).copied().unwrap_or(self.root),
        }
    }

    /// The entry that stands in a list for a hand-off to `language`.
    fn hand_off(&mut self, language: Language) -> Entry {
        if let Some(index) = self.hand_offs.iter().position(|&l| l == language) {
            return Entry::HandOff(index);
        }
        self.hand_offs.push(language);
        Entry::HandOff(self.hand_offs.len() - 1)
    }

    /// The number of the pattern `source`.
    fn pattern(&mut self, source: &'a str) -> usize {
        let next = self.patterns.len();
        let id = *self.pattern_ids.entry(source).or_insert(next);
        if id == next {
            self.patterns.push(source);
        }
        id
    }

    /// A new rule of `kind` with nothing in it.
    fn reserve(&mut self, kind: RuleKind) -> usize {
        self.rules.push(Draft {
            kind,
            name: &[],
            content_name: &[],
            begin: NONE,
            end: NONE,
            until: NONE,
            captures: Vec::new(),
            end_captures: Vec::new(),
            patterns: Vec::new(),
            apply_end_last: false,
            missing: false,
        });
        self.rules.len() - 1
    }

    /// The root rule of the grammar of `scope`, compiled on first use, or
    /// `None` when no grammar the link can reach whole has that scope.
    fn root(&mut self, scope: &str) -> Option<usize> {
        if let Some(&id) = self.roots.get(scope) {
            return Some(id);
        }
        let source = self.docs.iter().find(|&&(s, _)| s == scope)?.1;
        if !self.descend() {
            return None;
        }
        let grammar = self.grammar(source);
        let id = self.reserve(RuleKind::Container);
        self.roots.insert(grammar.scope.as_str(), id);
        let env = [&grammar.repository];
        let (patterns, missing) = self.list(source, &grammar.patterns, &env);
        if let Some(rule) = self.rules.get_mut(id) {
            rule.patterns = patterns;
            rule.missing = missing;
        }
        self.depth -= 1;
        Some(id)
    }

    /// The number of the rule object `rule` of `source`, compiled on first
    /// use with the repositories of `env`.
    fn entry(&mut self, source: Source, rule: RuleId, env: &Env<'a>) -> usize {
        if let Some(&id) = self.compiled.get(&(source, rule)) {
            return id;
        }
        let id = self.reserve(RuleKind::Container);
        self.compiled.insert((source, rule), id);
        self.rule(source, rule, Some(id), env, false);
        id
    }

    /// What an include of `target` from a rule of `source` stands for, or
    /// `None` when it resolves to nothing.
    fn include(&mut self, source: Source, target: &'a str, env: &Env<'a>) -> Option<Entry> {
        if let Some(&(_, language)) = self.handed.iter().find(|&&(s, _)| s == target) {
            return Some(self.hand_off(language));
        }
        if target == "$self" {
            let scope = self.grammar(source).scope.as_str();
            return self.root(scope).map(Entry::Rule);
        }
        if target == "$base" {
            let scope = self.root.scope.as_str();
            return self.root(scope).map(Entry::Rule);
        }
        if let Some(name) = target.strip_prefix('#') {
            let rule = env
                .iter()
                .rev()
                .find_map(|repository| lookup(repository, name))?;
            return Some(Entry::Rule(self.entry(source, rule, env)));
        }
        if let Some((other, name)) = target.split_once('#') {
            let other = self
                .docs
                .iter()
                .chain(&self.fragments)
                .find(|&&(s, _)| s == other)?
                .1;
            let grammar = self.grammar(other);
            let rule = lookup(&grammar.repository, name)?;
            return Some(Entry::Rule(self.entry(other, rule, &[&grammar.repository])));
        }
        self.root(target).map(Entry::Rule)
    }

    /// The entries of a list of rule objects, and whether the list lost any.
    ///
    /// An include that resolves to nothing leaves the list.
    /// A rule other than a match then leaves its own list too, when nothing
    /// is left in it, as vscode-textmate reads it.
    fn list(&mut self, source: Source, rules: &'a [RuleId], env: &Env<'a>) -> (Vec<Entry>, bool) {
        let grammar = self.grammar(source);
        let mut out = Vec::new();
        for &rule in rules {
            let Some(raw) = grammar.rules.get(rule) else {
                continue;
            };
            let id = match &raw.include {
                Some(target) => match self.include(source, target, env) {
                    None => continue,
                    Some(entry @ Entry::HandOff(_)) => {
                        out.push(entry);
                        continue;
                    }
                    Some(Entry::Rule(id)) => id,
                },
                None => self.rule(source, rule, None, env, false),
            };
            if self
                .rules
                .get(id)
                .is_some_and(|r| r.kind != RuleKind::Match && r.missing && r.patterns.is_empty())
            {
                continue;
            }
            out.push(Entry::Rule(id));
        }
        let missing = out.len() != rules.len();
        (out, missing)
    }

    /// The groups of `captures`, sorted by number, each with the rule that
    /// lexes its text again when it has patterns.
    fn captures(
        &mut self,
        source: Source,
        captures: Option<&'a Vec<RawCapture>>,
        env: &Env<'a>,
    ) -> Vec<CapDraft<'a>> {
        let grammar = self.grammar(source);
        let mut out = Vec::new();
        for capture in captures.map(Vec::as_slice).unwrap_or_default() {
            let Some(raw) = grammar.rules.get(capture.rule) else {
                continue;
            };
            let retokenize = if raw.has_patterns {
                self.rule(source, capture.rule, None, env, true)
            } else {
                NONE
            };
            out.push(CapDraft {
                index: capture.index.unwrap_or(NONE),
                name: &raw.name,
                content_name: &raw.content_name,
                retokenize,
            });
        }
        out.sort_by_key(|capture| capture.index);
        out
    }

    /// Steps one rule deeper, or records that the link went too deep and
    /// returns `false`.
    fn descend(&mut self) -> bool {
        if self.depth >= MAX_DEPTH {
            self.too_deep = true;
            return false;
        }
        self.depth += 1;
        true
    }

    /// Compiles the rule object `rule` of `source` into the rule `into`, or a
    /// new one, and returns its number.
    /// A capture's rule is compiled as a container of its patterns.
    ///
    /// Past [`MAX_DEPTH`], it compiles nothing and the link fails.
    fn rule(
        &mut self,
        source: Source,
        rule: RuleId,
        into: Option<usize>,
        outer: &Env<'a>,
        container: bool,
    ) -> usize {
        if !self.descend() {
            return into.unwrap_or_else(|| self.reserve(RuleKind::Container));
        }
        let id = self.rule_inside(source, rule, into, outer, container);
        self.depth -= 1;
        id
    }

    /// [`Builder::rule`] one level deeper.
    fn rule_inside(
        &mut self,
        source: Source,
        rule: RuleId,
        into: Option<usize>,
        outer: &Env<'a>,
        container: bool,
    ) -> usize {
        let grammar = self.grammar(source);
        let Some(raw) = grammar.rules.get(rule) else {
            return into.unwrap_or_else(|| self.reserve(RuleKind::Container));
        };
        let kind = if container {
            RuleKind::Container
        } else if raw.matched.is_some() {
            RuleKind::Match
        } else if raw.begin.is_some() {
            if raw.until.is_some() {
                RuleKind::BeginWhile
            } else {
                RuleKind::BeginEnd
            }
        } else {
            RuleKind::Container
        };
        let mut env = outer.to_vec();
        if kind == RuleKind::Container
            && let Some(repository) = &raw.repository
        {
            env.push(repository);
        }
        let id = into.unwrap_or_else(|| self.reserve(kind));
        if let Some(draft) = self.rules.get_mut(id) {
            draft.kind = kind;
            draft.name = &raw.name;
            draft.content_name = &raw.content_name;
            draft.apply_end_last = raw.apply_end_last;
        }
        let (mut begin, mut end, mut until) = (NONE, NONE, NONE);
        let (mut captures, mut end_captures) = (Vec::new(), Vec::new());
        match kind {
            RuleKind::Match => {
                begin = self.pattern(raw.matched.as_deref().unwrap_or_default());
                captures = self.captures(source, raw.captures.as_ref(), &env);
            }
            RuleKind::BeginEnd => {
                begin = self.pattern(raw.begin.as_deref().unwrap_or_default());
                let opening = raw.begin_captures.as_ref().or(raw.captures.as_ref());
                captures = self.captures(source, opening, &env);
                end = raw.end.as_deref().map_or(NONE, |end| self.pattern(end));
                let closing = raw.end_captures.as_ref().or(raw.captures.as_ref());
                end_captures = self.captures(source, closing, &env);
            }
            RuleKind::BeginWhile => {
                begin = self.pattern(raw.begin.as_deref().unwrap_or_default());
                let opening = raw.begin_captures.as_ref().or(raw.captures.as_ref());
                captures = self.captures(source, opening, &env);
                until = self.pattern(raw.until.as_deref().unwrap_or_default());
                let closing = raw.while_captures.as_ref().or(raw.captures.as_ref());
                end_captures = self.captures(source, closing, &env);
            }
            RuleKind::Container => {}
        }
        let (patterns, missing) = if kind == RuleKind::Match {
            (Vec::new(), false)
        } else {
            self.list(source, &raw.patterns, &env)
        };
        if let Some(draft) = self.rules.get_mut(id) {
            draft.begin = begin;
            draft.end = end;
            draft.until = until;
            draft.captures = captures;
            draft.end_captures = end_captures;
            if kind != RuleKind::Match {
                draft.patterns = patterns;
                draft.missing = missing;
            }
        }
        id
    }

    /// Replaces every container in a list with the rules of its own list,
    /// each container once, so only rules that match are left.
    fn flatten(&mut self) {
        let flat: Vec<Vec<Entry>> = (0..self.rules.len())
            .map(|id| {
                let mut out = Vec::new();
                let mut seen = HashSet::from([id]);
                if let Some(rule) = self.rules.get(id) {
                    expand(&self.rules, &rule.patterns, &mut out, &mut seen);
                }
                out
            })
            .collect();
        for (rule, patterns) in self.rules.iter_mut().zip(flat) {
            rule.patterns = patterns;
        }
    }

    /// Whether a rule reachable from `start` through the lists holds a
    /// hand-off.
    fn reaches_hand_off(&self, start: usize) -> bool {
        let mut seen = HashSet::new();
        let mut stack = vec![start];
        while let Some(id) = stack.pop() {
            if !seen.insert(id) {
                continue;
            }
            for entry in self
                .rules
                .get(id)
                .map(|r| r.patterns.as_slice())
                .unwrap_or_default()
            {
                match *entry {
                    Entry::HandOff(_) => return true,
                    Entry::Rule(rule) => stack.push(rule),
                }
            }
        }
        false
    }

    /// The rules of the capture groups whose text can reach a hand-off,
    /// sorted.
    fn handing_captures(&self) -> Vec<usize> {
        let mut handing = Vec::new();
        let mut seen = HashSet::new();
        for draft in &self.rules {
            for capture in draft.captures.iter().chain(&draft.end_captures) {
                if capture.retokenize != NONE && seen.insert(capture.retokenize) {
                    handing.push(capture.retokenize);
                }
            }
        }
        handing.retain(|&rule| self.reaches_hand_off(rule));
        handing.sort_unstable();
        handing
    }

    /// Interns the names and lists, and writes the tables.
    ///
    /// Nothing is leaked until no step can fail, so a refused grammar keeps
    /// no memory.
    fn finish(
        self,
        root: usize,
        injections: &[(&str, usize)],
    ) -> Result<&'static Grammar, LinkError> {
        let mut tables = Tables::default();
        let mut parts = Vec::with_capacity(self.rules.len());
        for draft in &self.rules {
            parts.push((
                tables.names_of(draft.name)?,
                tables.names_of(draft.content_name)?,
                tables.caps(&draft.captures)?,
                tables.caps(&draft.end_captures)?,
                tables.list_of(&self, draft)?,
            ));
        }
        if tables.hand_offs.len() > MAX_HAND_OFFS {
            return Err(LinkError::TooManyHandOffs);
        }
        let rules = self
            .rules
            .iter()
            .zip(parts)
            .map(
                |(draft, (name, content_name, captures, end_captures, list))| Rule {
                    kind: draft.kind,
                    name: leak(name),
                    content_name: leak(content_name),
                    begin: draft.begin,
                    end: draft.end,
                    until: draft.until,
                    captures: leak_captures(captures),
                    end_captures: leak_captures(end_captures),
                    list,
                    apply_end_last: draft.apply_end_last,
                },
            )
            .collect();
        let list_hand_offs = tables.list_hand_offs();
        let handing = self.handing_captures();
        let folding = if self.patterns.iter().any(|p| Regex::lean_differs(p)) {
            Folding::Differs
        } else {
            Folding::Same
        };
        let grammar = Grammar {
            scope: leak_str(&self.root.scope),
            names: leak(tables.names.iter().map(|name| leak_str(name)).collect()),
            rules: leak(rules),
            lists: leak(
                tables
                    .lists
                    .into_iter()
                    .map(|l| List::new(leak(l)))
                    .collect(),
            ),
            injections: leak(
                injections
                    .iter()
                    .map(|&(selector, rule)| Injection {
                        selector: leak_str(selector),
                        rule,
                    })
                    .collect(),
            ),
            patterns: leak(
                self.patterns
                    .iter()
                    .map(|p| Pattern::new(leak_str(p)))
                    .collect(),
            ),
            root,
            hand_offs: leak(
                tables
                    .hand_offs
                    .iter()
                    .map(|h| HandOff {
                        language: h.language,
                        before: h.before,
                        after: h.after,
                    })
                    .collect(),
            ),
            list_hand_offs: leak(list_hand_offs),
            handing_captures: leak(handing),
            dfa_cache: DFA_CACHE,
            folding,
            cells: OnceLock::new(),
            full_cells: OnceLock::new(),
        };
        Ok(Box::leak(Box::new(grammar)))
    }
}

/// Appends the rules of `list` to `out`, with each container's own rules in
/// its place, unless `seen` already holds it.
///
/// It walks with a stack of its own.
/// Containers can nest as deep as a repository chains entries.
/// [`MAX_DEPTH`] does not bound that.
/// A chain whose entries are compiled from the far end stays shallow while
/// it compiles.
fn expand(rules: &[Draft<'_>], list: &[Entry], out: &mut Vec<Entry>, seen: &mut HashSet<usize>) {
    let mut stack: Vec<(&[Entry], usize)> = vec![(list, 0)];
    while let Some(top) = stack.last_mut() {
        let (entries, at) = *top;
        let Some(&entry) = entries.get(at) else {
            stack.pop();
            continue;
        };
        top.1 += 1;
        match entry {
            Entry::Rule(id)
                if rules
                    .get(id)
                    .is_some_and(|rule| rule.kind == RuleKind::Container) =>
            {
                if seen.insert(id)
                    && let Some(rule) = rules.get(id)
                {
                    stack.push((&rule.patterns, 0));
                }
            }
            Entry::Rule(_) | Entry::HandOff(_) => out.push(entry),
        }
    }
}

/// The rule named `name` in `repository`.
fn lookup(repository: &Repository, name: &str) -> Option<RuleId> {
    let at = repository
        .binary_search_by(|(key, _)| key.as_str().cmp(name))
        .ok()?;
    repository.get(at).map(|&(_, rule)| rule)
}

/// The interned names, lists and hand-offs, as `oracle/engine-tables.ts`
/// numbers them while it writes the rules.
#[derive(Default)]
struct Tables {
    /// The scope names.
    names: Vec<String>,
    /// The number of each scope name.
    name_ids: HashMap<String, usize>,
    /// The lists of rules.
    lists: Vec<Vec<usize>>,
    /// The number of each list, by its hand-off and its rules.
    list_ids: HashMap<(Option<usize>, Vec<usize>), usize>,
    /// The hand-offs.
    hand_offs: Vec<HandOffRow>,
}

impl Tables {
    /// The references of `names`. The top bit marks a name built from a
    /// match's groups.
    fn names_of(&mut self, names: &[String]) -> Result<Vec<u16>, LinkError> {
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            let next = self.names.len();
            let id = *self.name_ids.entry(name.clone()).or_insert(next);
            if id == next {
                self.names.push(name.clone());
            }
            let id = u16::try_from(id)
                .ok()
                .filter(|&id| id < DYNAMIC_NAME)
                .ok_or(LinkError::TooManyNames)?;
            out.push(if name.contains('$') {
                id | DYNAMIC_NAME
            } else {
                id
            });
        }
        Ok(out)
    }

    /// For each list, the index of the hand-off whose include stood in it.
    /// Empty when the grammar hands nothing off.
    fn list_hand_offs(&self) -> Vec<Option<u8>> {
        if self.hand_offs.is_empty() {
            return Vec::new();
        }
        (0..self.lists.len())
            .map(|list| {
                let index = self.hand_offs.iter().position(|h| h.list == list)?;
                u8::try_from(index).ok()
            })
            .collect()
    }

    /// The groups of `captures`, with their names interned.
    fn caps(&mut self, captures: &[CapDraft<'_>]) -> Result<Vec<OwnedCapture>, LinkError> {
        let mut out = Vec::with_capacity(captures.len());
        for capture in captures {
            out.push(OwnedCapture {
                index: capture.index,
                name: self.names_of(capture.name)?,
                content_name: self.names_of(capture.content_name)?,
                retokenize: capture.retokenize,
            });
        }
        Ok(out)
    }

    /// The number of the list of `rules`, apart from any list with a
    /// hand-off when `hand_off` is one.
    fn plain(&mut self, rules: Vec<usize>, hand_off: Option<usize>) -> usize {
        let next = self.lists.len();
        let key = (hand_off, rules);
        if let Some(&id) = self.list_ids.get(&key) {
            return id;
        }
        self.lists.push(key.1.clone());
        self.list_ids.insert(key, next);
        next
    }

    /// The number of `owner`'s list.
    ///
    /// A list with a hand-off holds the rules before the include, then the
    /// rules after it.
    /// It stays apart from a plain list with the same rules.
    /// The hand-off names the list and its two parts.
    fn list_of(&mut self, builder: &Builder<'_>, owner: &Draft<'_>) -> Result<usize, LinkError> {
        let rules_of = |entries: &[Entry]| -> Vec<usize> {
            entries
                .iter()
                .filter_map(|entry| match *entry {
                    Entry::Rule(id) => Some(id),
                    Entry::HandOff(_) => None,
                })
                .collect()
        };
        let Some(at) = owner
            .patterns
            .iter()
            .position(|entry| matches!(entry, Entry::HandOff(_)))
        else {
            return Ok(self.plain(rules_of(&owner.patterns), None));
        };
        let (before, rest) = owner.patterns.split_at(at);
        let (hand_off, after) = match rest.split_first() {
            Some((&Entry::HandOff(index), after)) => (index, after),
            Some((&Entry::Rule(_), _)) | None => {
                return Ok(self.plain(rules_of(&owner.patterns), None));
            }
        };
        if after.iter().any(|entry| matches!(entry, Entry::HandOff(_))) {
            return Err(LinkError::TwoHandOffs);
        }
        let (before, after) = (rules_of(before), rules_of(after));
        let mut halts: Vec<usize> = before
            .iter()
            .chain(&after)
            .filter_map(|&id| builder.rules.get(id).map(|rule| rule.begin))
            .collect();
        if owner.kind == RuleKind::BeginEnd {
            halts.push(owner.end);
        }
        if let Some(anchored) = halts
            .iter()
            .filter_map(|&pattern| builder.patterns.get(pattern))
            .find(|source| uses_last_match_end(source))
        {
            return Err(LinkError::AnchoredHalt((*anchored).to_owned()));
        }
        let language = builder
            .hand_offs
            .get(hand_off)
            .copied()
            .ok_or(LinkError::TooManyHandOffs)?;
        let before_list = self.plain(before.clone(), None);
        let after_list = self.plain(after.clone(), None);
        let found = self.hand_offs.iter().position(|h| {
            h.language == language && h.before == before_list && h.after == after_list
        });
        let row = found.unwrap_or(self.hand_offs.len());
        if found.is_none() {
            self.hand_offs.push(HandOffRow {
                list: NONE,
                language,
                before: before_list,
                after: after_list,
            });
        }
        let mut joined = before;
        joined.extend(after);
        let list = self.plain(joined, Some(row));
        if let Some(entry) = self.hand_offs.get_mut(row) {
            entry.list = list;
        }
        Ok(list)
    }
}

/// Whether `source` uses `\G`, which matches where the last match ended.
fn uses_last_match_end(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes.get(i) == Some(&b'\\') {
            if bytes.get(i + 1) == Some(&b'G') {
                return true;
            }
            i += 1;
        }
        i += 1;
    }
    false
}

/// A capture group with its names interned, before it is leaked.
struct OwnedCapture {
    /// The group's number, or [`NONE`].
    index: usize,
    /// The references of the scopes of its text.
    name: Vec<u16>,
    /// The references of the scopes of the rules inside it.
    content_name: Vec<u16>,
    /// The rule that lexes its text again, or [`NONE`].
    retokenize: usize,
}

/// The groups of `captures` as the engine reads them, on the heap for the
/// rest of the process.
fn leak_captures(captures: Vec<OwnedCapture>) -> &'static [Cap] {
    leak(
        captures
            .into_iter()
            .map(|capture| Cap {
                index: capture.index,
                name: leak(capture.name),
                content_name: leak(capture.content_name),
                retokenize: capture.retokenize,
            })
            .collect(),
    )
}

/// `items` on the heap for the rest of the process.
fn leak<T>(items: Vec<T>) -> &'static [T] {
    Box::leak(items.into_boxed_slice())
}

/// A copy of `text` on the heap for the rest of the process.
fn leak_str(text: &str) -> &'static str {
    Box::leak(text.to_owned().into_boxed_str())
}
