//! Compiles TextMate grammars (JSON, as Shiki ships them) into one table of
//! rules, resolving includes across the loaded grammars.
//!
//! It reproduces the one behaviour of vscode-textmate that changes what a
//! grammar means: a rule whose patterns all include a grammar that is not
//! loaded is dropped, and so is every rule that is left with no patterns
//! because of it.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use serde_json::Value;

use crate::regex::Regex;

pub type RuleId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One regular expression, with captures.
    Match,
    /// `begin` to `end`, with patterns between.
    BeginEnd,
    /// `begin`, and each following line must match `while`.
    BeginWhile,
    /// Only patterns: a grammar, a repository entry, or a capture's rules.
    Include,
}

/// A pattern's source and, unless it holds back-references that only a match
/// of `begin` can fill in, its compiled form.
#[derive(Debug)]
pub struct Pattern {
    pub source: String,
    pub regex: Option<Rc<Regex>>,
    pub has_backrefs: bool,
}

#[derive(Debug, Default, Clone)]
pub struct CaptureRule {
    pub name: Option<String>,
    pub content_name: Option<String>,
    /// The rule that lexes the captured text again.
    pub retokenize: Option<RuleId>,
}

#[derive(Debug)]
pub struct Rule {
    pub kind: Kind,
    pub name: Option<String>,
    pub content_name: Option<String>,
    /// `match` or `begin`.
    pub start: Option<Pattern>,
    pub end: Option<Pattern>,
    pub while_: Option<Pattern>,
    pub captures: Vec<Option<CaptureRule>>,
    pub end_captures: Vec<Option<CaptureRule>>,
    pub patterns: Vec<RuleId>,
    pub has_missing: bool,
    pub apply_end_last: bool,
    /// Patterns compile errors, for the survey.
    pub error: Option<String>,
}

impl Rule {
    fn empty(kind: Kind) -> Rule {
        Rule {
            kind,
            name: None,
            content_name: None,
            start: None,
            end: None,
            while_: None,
            captures: Vec::new(),
            end_captures: Vec::new(),
            patterns: Vec::new(),
            has_missing: false,
            apply_end_last: false,
            error: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Grammars {
    pub rules: Vec<Rule>,
    /// The rule of each grammar's top-level patterns, by scope name.
    pub roots: HashMap<String, RuleId>,
    /// Patterns that failed to compile.
    pub errors: Vec<String>,
}

struct Builder {
    docs: HashMap<String, Value>,
    out: Grammars,
    repo: HashMap<(String, String), RuleId>,
}

impl Grammars {
    /// Loads the grammars `scopes` names from `dir/<scope>.json`.
    pub fn load(dir: &Path, scopes: &[&str]) -> Grammars {
        let mut docs = HashMap::new();
        for scope in scopes {
            let text = fs::read_to_string(dir.join(format!("{scope}.json")))
                .unwrap_or_else(|e| panic!("{scope}: {e}; run tools/dump-grammars.ts"));
            docs.insert((*scope).to_owned(), serde_json::from_str(&text).expect("JSON"));
        }
        let mut builder = Builder {
            docs,
            out: Grammars::default(),
            repo: HashMap::new(),
        };
        for scope in scopes {
            builder.root(scope);
        }
        builder.out
    }
}

fn captures_of(builder: &mut Builder, scope: &str, value: Option<&Value>) -> Vec<Option<CaptureRule>> {
    let Some(Value::Object(map)) = value else {
        return Vec::new();
    };
    let mut out: Vec<Option<CaptureRule>> = Vec::new();
    for (key, capture) in map {
        let Ok(index) = key.parse::<usize>() else { continue };
        if out.len() <= index {
            out.resize(index + 1, None);
        }
        let retokenize = capture
            .get("patterns")
            .is_some()
            .then(|| builder.rule(scope, capture, None));
        out[index] = Some(CaptureRule {
            name: capture.get("name").and_then(Value::as_str).map(str::to_owned),
            content_name: capture
                .get("contentName")
                .and_then(Value::as_str)
                .map(str::to_owned),
            retokenize,
        });
    }
    out
}

fn has_backrefs(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' {
            if bytes[i + 1].is_ascii_digit() && bytes[i + 1] != b'0' {
                return true;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    false
}

impl Builder {
    fn pattern(&mut self, source: &str) -> Pattern {
        let backrefs = has_backrefs(source);
        let regex = if backrefs {
            None
        } else {
            match Regex::new(source) {
                Ok(re) => Some(Rc::new(re)),
                Err(e) => {
                    self.out.errors.push(format!("{}: {source}", e.0));
                    None
                }
            }
        };
        Pattern {
            source: source.to_owned(),
            regex,
            has_backrefs: backrefs,
        }
    }

    fn root(&mut self, scope: &str) -> Option<RuleId> {
        if let Some(&id) = self.out.roots.get(scope) {
            return Some(id);
        }
        let doc = self.docs.get(scope)?.clone();
        let id = self.reserve(Kind::Include);
        self.out.roots.insert(scope.to_owned(), id);
        let list = doc.get("patterns").and_then(Value::as_array).cloned().unwrap_or_default();
        let (patterns, missing) = self.patterns(scope, &list);
        self.out.rules[id].patterns = patterns;
        self.out.rules[id].has_missing = missing;
        Some(id)
    }

    fn reserve(&mut self, kind: Kind) -> RuleId {
        self.out.rules.push(Rule::empty(kind));
        self.out.rules.len() - 1
    }

    fn repository(&mut self, scope: &str, name: &str) -> Option<RuleId> {
        let key = (scope.to_owned(), name.to_owned());
        if let Some(&id) = self.repo.get(&key) {
            return Some(id);
        }
        let entry = self.docs.get(scope)?.get("repository")?.get(name)?.clone();
        let id = self.reserve(Kind::Include);
        self.repo.insert(key, id);
        self.rule(scope, &entry, Some(id));
        Some(id)
    }

    fn include(&mut self, scope: &str, target: &str) -> Option<RuleId> {
        if target == "$self" || target == "$base" {
            return self.root(scope);
        }
        if let Some(name) = target.strip_prefix('#') {
            return self.repository(scope, name);
        }
        match target.split_once('#') {
            Some((other, name)) => self.docs.contains_key(other).then(|| self.repository(other, name)).flatten(),
            None => self.root(target),
        }
    }

    /// The compiled patterns of a list, and whether any were dropped.
    fn patterns(&mut self, scope: &str, list: &[Value]) -> (Vec<RuleId>, bool) {
        let mut out = Vec::new();
        for pattern in list {
            let id = match pattern.get("include").and_then(Value::as_str) {
                Some(target) => match self.include(scope, target) {
                    Some(id) => id,
                    None => continue,
                },
                None => self.rule(scope, pattern, None),
            };
            let rule = &self.out.rules[id];
            let droppable = !matches!(rule.kind, Kind::Match);
            if droppable && rule.has_missing && rule.patterns.is_empty() {
                continue;
            }
            out.push(id);
        }
        let missing = out.len() != list.len();
        (out, missing)
    }

    /// Compiles one rule object, into `into` when its id was reserved.
    fn rule(&mut self, scope: &str, value: &Value, into: Option<RuleId>) -> RuleId {
        let kind = if value.get("begin").is_some() {
            if value.get("while").is_some() {
                Kind::BeginWhile
            } else {
                Kind::BeginEnd
            }
        } else if value.get("match").is_some() {
            Kind::Match
        } else {
            Kind::Include
        };
        let id = into.unwrap_or_else(|| self.reserve(kind));
        let text = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_owned);
        let mut rule = Rule::empty(kind);
        rule.name = text("name");
        rule.content_name = text("contentName");
        rule.apply_end_last = value.get("applyEndPatternLast").is_some_and(|v| v.as_i64() == Some(1) || v.as_bool() == Some(true));
        match kind {
            Kind::Match => {
                rule.start = Some(self.pattern(&text("match").unwrap_or_default()));
                rule.captures = captures_of(self, scope, value.get("captures"));
            }
            Kind::BeginEnd | Kind::BeginWhile => {
                rule.start = Some(self.pattern(&text("begin").unwrap_or_default()));
                let shorthand = value.get("captures");
                rule.captures = captures_of(self, scope, value.get("beginCaptures").or(shorthand));
                if kind == Kind::BeginEnd {
                    rule.end = Some(self.pattern(&text("end").unwrap_or_default()));
                    rule.end_captures =
                        captures_of(self, scope, value.get("endCaptures").or(shorthand));
                } else {
                    rule.while_ = Some(self.pattern(&text("while").unwrap_or_default()));
                    rule.end_captures =
                        captures_of(self, scope, value.get("whileCaptures").or(shorthand));
                }
            }
            Kind::Include => {}
        }
        self.out.rules[id] = rule;
        if matches!(kind, Kind::BeginEnd | Kind::BeginWhile | Kind::Include) {
            let list = value.get("patterns").and_then(Value::as_array).cloned().unwrap_or_default();
            let (patterns, missing) = self.patterns(scope, &list);
            let rule = &mut self.out.rules[id];
            rule.patterns = patterns;
            rule.has_missing = missing;
        }
        id
    }
}
