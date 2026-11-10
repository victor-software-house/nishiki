//! The grammar compiler.
//! A `TextMate` grammar in Shiki's JSON form becomes plain data.
//! [`link::link`] turns a root grammar and the grammars it can include into the
//! engine's tables.
//!
//! It is written from scratch.
//! vscode-textmate and `oracle/engine-tables.ts` are references for its
//! behaviour only.
//!
//! The work is split in two:
//!
//! - [`compile`] reads one grammar. Its includes stay names, so a
//!   [`UnlinkedGrammar`] grammar depends on no other grammar.
//! - [`link::link`] follows the includes from a root grammar, through the
//!   grammars a caller loaded, and writes one flat [`Grammar`](super::Grammar).
//!
//! Includes inside a grammar resolve when it is linked, not when it is
//! compiled.
//! vscode-textmate compiles a repository rule with the repositories in scope
//! where the rule is first reached.
//! With nested repositories, that place depends on the order the link visits
//! rules in.

mod hand_lexed;
mod json;
pub(crate) mod link;

use json::Json;

/// The index of a rule in [`UnlinkedGrammar::rules`].
pub(crate) type RuleId = usize;

/// Named rules, sorted by name, so an include finds one by binary search.
/// Each name is there once.
pub(crate) type Repository = Vec<(String, RuleId)>;

/// One grammar, read from its JSON and not yet linked.
///
/// It is plain data: owned strings and indices, with no cells.
/// Every JSON object that can act as a rule is one [`UnlinkedRule`], once.
/// A rule's identity is its index, as vscode-textmate keys a compiled rule on
/// its JSON object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnlinkedGrammar {
    /// The grammar's `scopeName`, such as `source.rust`.
    pub(crate) scope: String,
    /// Every rule object of the grammar.
    pub(crate) rules: Vec<UnlinkedRule>,
    /// The grammar's top-level `patterns`.
    pub(crate) patterns: Vec<RuleId>,
    /// The grammar's top-level `repository`.
    pub(crate) repository: Repository,
    /// The grammar's own `injections`: a scope selector and its rule, in the
    /// order of the text.
    pub(crate) injections: Vec<(String, RuleId)>,
    /// The `injectionSelector` under which the grammar injects itself into
    /// others, when it has one.
    pub(crate) injection_selector: Option<String>,
    /// The scopes of the grammars the grammar injects itself into
    /// (`injectTo`).
    pub(crate) inject_to: Vec<String>,
}

/// One rule object of a grammar, as its JSON holds it.
///
/// Each field records what the linker asks of the object, with JavaScript's
/// reading of absent, `null` and empty values.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct UnlinkedRule {
    /// The `include`, when it is a string. In a list of patterns, such a rule
    /// is an include and nothing else.
    pub(crate) include: Option<String>,
    /// The scopes of `name`, split at blanks.
    pub(crate) name: Vec<String>,
    /// The scopes of `contentName`, split at blanks.
    pub(crate) content_name: Vec<String>,
    /// `match`, when it is truthy.
    pub(crate) matched: Option<String>,
    /// `begin`, when the key is present, even as `null`.
    pub(crate) begin: Option<String>,
    /// `end`, when it is truthy.
    pub(crate) end: Option<String>,
    /// `while`, when it is truthy.
    pub(crate) until: Option<String>,
    /// `captures`, unless missing or `null`.
    pub(crate) captures: Option<Vec<UnlinkedCapture>>,
    /// `beginCaptures`, unless missing or `null`.
    pub(crate) begin_captures: Option<Vec<UnlinkedCapture>>,
    /// `endCaptures`, unless missing or `null`.
    pub(crate) end_captures: Option<Vec<UnlinkedCapture>>,
    /// `whileCaptures`, unless missing or `null`.
    pub(crate) while_captures: Option<Vec<UnlinkedCapture>>,
    /// Whether `applyEndPatternLast` is `1` or `true`.
    pub(crate) apply_end_last: bool,
    /// The rules of `patterns`.
    ///
    /// A rule with no `patterns`, `match` or `begin` and a string `include`
    /// holds that include as its one pattern, as vscode-textmate reads it.
    pub(crate) patterns: Vec<RuleId>,
    /// Whether `patterns` is truthy. Only such a capture lexes its text again.
    pub(crate) has_patterns: bool,
    /// `repository`, when it is truthy.
    pub(crate) repository: Option<Repository>,
}

/// One capture group of a rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnlinkedCapture {
    /// The group's number, or `None` for a negative key.
    pub(crate) index: Option<usize>,
    /// The capture's object, read as a rule: its names, and its patterns
    /// when it has some.
    pub(crate) rule: RuleId,
}

/// Why a grammar did not compile.
#[derive(Debug)]
pub(crate) enum CompileError {
    /// The text is not JSON.
    Json(serde_json::Error),
    /// The grammar is not an object, or has no string `scopeName`.
    NoScope,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Json(error) => write!(f, "the grammar is not JSON: {error}"),
            CompileError::NoScope => f.write_str("the grammar has no string scopeName"),
        }
    }
}

impl std::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompileError::Json(error) => Some(error),
            CompileError::NoScope => None,
        }
    }
}

/// Reads a grammar in Shiki's JSON form.
pub(crate) fn compile(text: &str) -> Result<UnlinkedGrammar, CompileError> {
    let json = Json::parse(text).map_err(CompileError::Json)?;
    let scope = json
        .str("scopeName")
        .ok_or(CompileError::NoScope)?
        .to_owned();
    let mut reader = Reader { rules: Vec::new() };
    let patterns = reader.list(json.present("patterns"));
    let repository = reader.repository(json.get("repository"));
    let injections = match json.get("injections") {
        Some(Json::Object(entries)) => entries
            .iter()
            .map(|(selector, value)| (selector.clone(), reader.rule(value)))
            .collect(),
        Some(_) | None => Vec::new(),
    };
    let injection_selector = json.truthy("injectionSelector").map(Json::js_string);
    let inject_to = match json.get("injectTo") {
        Some(Json::Array(items)) => items.iter().map(Json::js_string).collect(),
        Some(_) | None => Vec::new(),
    };
    Ok(UnlinkedGrammar {
        scope,
        rules: reader.rules,
        patterns,
        repository,
        injections,
        injection_selector,
        inject_to,
    })
}

/// Reads rule objects into [`UnlinkedRule`]s.
struct Reader {
    /// The rules read so far.
    rules: Vec<UnlinkedRule>,
}

impl Reader {
    /// The rules of a `patterns` array. Anything else holds none.
    fn list(&mut self, value: Option<&Json>) -> Vec<RuleId> {
        match value {
            Some(Json::Array(items)) => items.iter().map(|item| self.rule(item)).collect(),
            Some(_) | None => Vec::new(),
        }
    }

    /// The truthy entries of a `repository` object, sorted by name.
    ///
    /// A falsy entry is left out, as the linker's lookup passes over it to an
    /// outer repository.
    /// The rules are read in the order of the text, so their numbers follow
    /// it.
    fn repository(&mut self, value: Option<&Json>) -> Repository {
        let mut repository: Repository = match value {
            Some(Json::Object(entries)) => entries
                .iter()
                .filter(|(_, rule)| rule.is_truthy())
                .map(|(name, rule)| (name.clone(), self.rule(rule)))
                .collect(),
            Some(_) | None => Vec::new(),
        };
        repository.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        repository
    }

    /// Reads one rule object, and the rules inside it, depth first.
    fn rule(&mut self, value: &Json) -> RuleId {
        let id = self.rules.len();
        self.rules.push(UnlinkedRule::default());
        let include = value.str("include").map(str::to_owned);
        let mut patterns = self.list(value.present("patterns"));
        if value.present("patterns").is_none()
            && value.truthy("match").is_none()
            && value.get("begin").is_none()
            && value.truthy("include").is_some()
            && let Some(target) = &include
        {
            let fallback = self.rules.len();
            self.rules.push(UnlinkedRule {
                include: Some(target.clone()),
                ..UnlinkedRule::default()
            });
            patterns.push(fallback);
        }
        let repository = value
            .truthy("repository")
            .map(|repository| self.repository(Some(repository)));
        let rule = UnlinkedRule {
            include,
            name: scopes(value.get("name")),
            content_name: scopes(value.get("contentName")),
            matched: value.truthy("match").map(Json::js_string),
            begin: value.get("begin").map(Json::js_string),
            end: value.truthy("end").map(Json::js_string),
            until: value.truthy("while").map(Json::js_string),
            captures: self.captures(value.present("captures")),
            begin_captures: self.captures(value.present("beginCaptures")),
            end_captures: self.captures(value.present("endCaptures")),
            while_captures: self.captures(value.present("whileCaptures")),
            apply_end_last: value
                .get("applyEndPatternLast")
                .is_some_and(Json::is_one_or_true),
            patterns,
            has_patterns: value.truthy("patterns").is_some(),
            repository,
        };
        if let Some(slot) = self.rules.get_mut(id) {
            *slot = rule;
        }
        id
    }

    /// The groups of a captures object, in the order of JavaScript's
    /// `Object.entries`.
    /// Whole-number keys come first, ascending, then the others in the order
    /// of the text.
    ///
    /// A key that is not a whole number is left out.
    /// So is a value that is not an object or an array.
    /// The linker sorts the groups by number after it compiles them, as
    /// `oracle/engine-tables.ts` does.
    fn captures(&mut self, value: Option<&Json>) -> Option<Vec<UnlinkedCapture>> {
        let entries: Vec<(String, &Json)> = match value? {
            Json::Object(entries) => entries.iter().map(|(k, v)| (k.clone(), v)).collect(),
            Json::Array(items) => items
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v))
                .collect(),
            Json::Null | Json::Bool(_) | Json::Number(_) | Json::String(_) => Vec::new(),
        };
        let mut ordered: Vec<(Option<u32>, usize)> = entries
            .iter()
            .enumerate()
            .map(|(at, (key, _))| (array_index(key), at))
            .collect();
        ordered.sort_by_key(|&(index, at)| (index.is_none(), index, at));
        let mut out = Vec::new();
        for (_, at) in ordered {
            let Some((key, capture)) = entries.get(at) else {
                continue;
            };
            let Some(index) = whole_number(key) else {
                continue;
            };
            if !matches!(capture, Json::Object(_) | Json::Array(_)) {
                continue;
            }
            out.push(UnlinkedCapture {
                index: usize::try_from(index).ok(),
                rule: self.rule(capture),
            });
        }
        Some(out)
    }
}

/// The scopes a `name` or `contentName` adds: its words, when it is a string.
fn scopes(value: Option<&Json>) -> Vec<String> {
    match value {
        Some(Json::String(text)) => text.split_whitespace().map(str::to_owned).collect(),
        Some(_) | None => Vec::new(),
    }
}

/// The key as a JavaScript array index: digits with no leading zero, below
/// 2³² − 1. `Object.entries` lists such keys first.
fn array_index(key: &str) -> Option<u32> {
    if key.is_empty() || (key.len() > 1 && key.starts_with('0')) {
        return None;
    }
    if !key.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    key.parse::<u32>().ok().filter(|&n| n != u32::MAX)
}

/// The whole number JavaScript's `Number(key)` reads from a capture key.
///
/// It reads blanks around the key, an empty key as zero, a sign, and a
/// fraction of zeros.
/// It does not read hexadecimal, exponents or `Infinity`, which no grammar
/// writes as a capture key.
fn whole_number(key: &str) -> Option<i64> {
    let key = key.trim();
    if key.is_empty() {
        return Some(0);
    }
    let (whole, fraction) = key.split_once('.').unwrap_or((key, ""));
    if !fraction.bytes().all(|b| b == b'0') {
        return None;
    }
    let digits = whole.strip_prefix(['+', '-']).unwrap_or(whole);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    whole.parse::<i64>().ok()
}

#[cfg(test)]
mod tests;
