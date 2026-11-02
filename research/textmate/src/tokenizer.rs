//! The line tokenizer: the TextMate engine's loop over a compiled grammar.
//!
//! A line starts by checking the `while` rule of each open rule; the first
//! that fails closes itself and every rule inside it. Then the patterns of the
//! innermost rule, and its `end` pattern, are searched from the cursor, and
//! the match that starts first wins (the earliest listed on a tie). A match
//! opens a rule, closes one, or only scopes text, and its capture groups add
//! scopes and may hand their text to other rules.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::grammar::{CaptureRule, Grammars, Kind, Pattern, Rule, RuleId};
use crate::regex::{Captures, Regex};

/// A persistent stack of scope names.
#[derive(Debug)]
pub struct ScopeNode {
    parent: Option<Scopes>,
    name: Rc<str>,
}

pub type Scopes = Rc<ScopeNode>;

fn push_scopes(base: &Option<Scopes>, names: &str) -> Option<Scopes> {
    let mut at = base.clone();
    for name in names.split_whitespace() {
        at = Some(Rc::new(ScopeNode {
            parent: at,
            name: Rc::from(name),
        }));
    }
    at
}

/// The names of a scope stack, outermost first.
pub fn scope_names(scopes: &Option<Scopes>) -> Vec<Rc<str>> {
    let mut out = Vec::new();
    let mut at = scopes.as_ref();
    while let Some(node) = at {
        out.push(Rc::clone(&node.name));
        at = node.parent.as_ref();
    }
    out.reverse();
    out
}

#[derive(Clone, Debug)]
struct Frame {
    rule: RuleId,
    enter_pos: isize,
    anchor_pos: isize,
    begin_eol: bool,
    /// The `end` or `while` pattern, with back-references filled in.
    end_pattern: Option<Rc<str>>,
    name_scopes: Option<Scopes>,
    content_scopes: Option<Scopes>,
}

/// The state between lines.
#[derive(Clone, Debug)]
pub struct State {
    frames: Vec<Frame>,
    started: bool,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub end: usize,
    pub scopes: Option<Scopes>,
}

struct LineTokens {
    tokens: Vec<Token>,
    last: usize,
}

impl LineTokens {
    fn produce(&mut self, scopes: &Option<Scopes>, end: usize) {
        if end <= self.last {
            return;
        }
        self.tokens.push(Token {
            end,
            scopes: scopes.clone(),
        });
        self.last = end;
    }
}

pub struct Engine {
    pub grammars: Grammars,
    regexes: RefCell<HashMap<String, Option<Rc<Regex>>>>,
    flat: RefCell<HashMap<RuleId, Rc<Vec<RuleId>>>>,
    /// Matcher steps spent, for the survey of cost.
    pub steps: RefCell<u64>,
}

const NO_LIMIT: u64 = u64::MAX / 4;

/// Replaces `\1`..`\9` in `source` by the regex-escaped text of the groups.
fn fill_backrefs(source: &str, text: &str, caps: &Captures) -> String {
    let mut out = String::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            if let Some(d) = chars[i + 1].to_digit(10).filter(|&d| d > 0) {
                let group = caps
                    .get(usize::try_from(d).unwrap_or(0))
                    .copied()
                    .flatten()
                    .map_or("", |(s, e)| &text[s..e]);
                for c in group.chars() {
                    if "-\\{}*+?|^$.,[]()# \t\n".contains(c) {
                        out.push('\\');
                    }
                    out.push(c);
                }
                i += 2;
                continue;
            }
            out.push(chars[i]);
            out.push(chars[i + 1]);
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// The name of a rule or capture, with `$1` replaced by capture group 1.
fn fill_name(name: &str, text: &str, caps: &Captures) -> String {
    if !name.contains('$') {
        return name.to_owned();
    }
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
            let d = chars[i + 1].to_digit(10).unwrap_or(0) as usize;
            if let Some(Some((s, e))) = caps.get(d) {
                out.push_str(&text[*s..*e]);
            }
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

impl Engine {
    pub fn new(grammars: Grammars) -> Engine {
        Engine {
            grammars,
            regexes: RefCell::new(HashMap::new()),
            flat: RefCell::new(HashMap::new()),
            steps: RefCell::new(0),
        }
    }

    pub fn start(&self, scope: &str) -> State {
        let root = self.grammars.roots[scope];
        let scopes = push_scopes(&None, scope);
        State {
            frames: vec![Frame {
                rule: root,
                enter_pos: -1,
                anchor_pos: -1,
                begin_eol: false,
                end_pattern: None,
                name_scopes: scopes.clone(),
                content_scopes: scopes,
            }],
            started: false,
        }
    }

    /// The rules a rule's patterns stand for, with includes expanded.
    fn flatten(&self, id: RuleId) -> Rc<Vec<RuleId>> {
        if let Some(found) = self.flat.borrow().get(&id) {
            return Rc::clone(found);
        }
        let mut out = Vec::new();
        let mut seen = vec![id];
        self.expand(&self.grammars.rules[id].patterns, &mut out, &mut seen);
        let out = Rc::new(out);
        self.flat.borrow_mut().insert(id, Rc::clone(&out));
        out
    }

    fn expand(&self, list: &[RuleId], out: &mut Vec<RuleId>, seen: &mut Vec<RuleId>) {
        for &id in list {
            let rule = &self.grammars.rules[id];
            if rule.kind == Kind::Include {
                if seen.contains(&id) {
                    continue;
                }
                seen.push(id);
                self.expand(&rule.patterns, out, seen);
            } else {
                out.push(id);
            }
        }
    }

    fn regex(&self, source: &str) -> Option<Rc<Regex>> {
        if let Some(found) = self.regexes.borrow().get(source) {
            return found.clone();
        }
        let compiled = Regex::new(source).ok().map(Rc::new);
        self.regexes
            .borrow_mut()
            .insert(source.to_owned(), compiled.clone());
        compiled
    }

    fn search(
        &self,
        re: &Regex,
        text: &str,
        from: usize,
        anchor: isize,
        allow_a: bool,
    ) -> Option<Captures> {
        let mut steps = NO_LIMIT;
        let anchor = usize::try_from(anchor).ok();
        let found = re.find_at(text, from, anchor, allow_a, &mut steps);
        *self.steps.borrow_mut() += NO_LIMIT - steps;
        found
    }

    /// Tokenizes `line` (no line ending) from `state`.
    pub fn tokenize_line(&self, line: &str, state: &mut State) -> Vec<Token> {
        if line.is_empty() {
            return Vec::new();
        }
        let text = format!("{line}\n");
        let mut lt = LineTokens {
            tokens: Vec::new(),
            last: 0,
        };
        let first = !state.started;
        state.started = true;
        let mut frames = std::mem::take(&mut state.frames);
        // vscode-textmate resets the positions of open frames per line.
        for frame in &mut frames {
            frame.enter_pos = -1;
            frame.anchor_pos = -1;
        }
        self.tokenize_string(&text, first, 0, &mut frames, &mut lt, true);
        state.frames = frames;
        let len = line.len();
        lt.tokens.retain(|t| t.end > 0);
        for token in &mut lt.tokens {
            token.end = token.end.min(len);
        }
        let mut last = 0;
        lt.tokens.retain(|t| {
            let keep = t.end > last;
            last = last.max(t.end);
            keep
        });
        lt.tokens
    }

    fn pattern_regex(&self, pattern: &Pattern, resolved: Option<&str>) -> Option<Rc<Regex>> {
        match resolved {
            Some(source) => self.regex(source),
            None => match &pattern.regex {
                Some(re) => Some(Rc::clone(re)),
                None => self.regex(&pattern.source),
            },
        }
    }

    fn tokenize_string(
        &self,
        text: &str,
        first_line: bool,
        start: usize,
        frames: &mut Vec<Frame>,
        lt: &mut LineTokens,
        check_while: bool,
    ) {
        let len = text.len();
        let mut pos = start;
        let mut anchor: isize = -1;
        let mut allow_a = first_line;
        if check_while {
            anchor = if frames.last().is_some_and(|f| f.begin_eol) { 0 } else { -1 };
            let mut i = 0;
            while i < frames.len() {
                let rule = &self.grammars.rules[frames[i].rule];
                if rule.kind != Kind::BeginWhile {
                    i += 1;
                    continue;
                }
                let source = frames[i].end_pattern.clone();
                let re = rule
                    .while_
                    .as_ref()
                    .and_then(|p| self.pattern_regex(p, source.as_deref()));
                let found = re.and_then(|re| self.search(&re, text, pos, anchor, allow_a));
                match found {
                    Some(caps) => {
                        let (s, e) = caps[0].unwrap_or((pos, pos));
                        let scopes = frames[i].content_scopes.clone();
                        lt.produce(&scopes, s);
                        self.handle_captures(text, &rule.end_captures, &caps, frames, i, lt);
                        lt.produce(&scopes, e);
                        anchor = isize::try_from(e).unwrap_or(0);
                        if e > pos {
                            pos = e;
                            allow_a = false;
                        }
                        i += 1;
                    }
                    None => {
                        frames.truncate(i);
                        break;
                    }
                }
            }
        }
        let mut stop = false;
        while !stop {
            stop = self.scan_next(text, len, &mut pos, &mut anchor, &mut allow_a, frames, lt);
        }
    }

    /// One step of the loop; true when the line is done.
    #[allow(clippy::too_many_arguments)]
    fn scan_next(
        &self,
        text: &str,
        len: usize,
        pos: &mut usize,
        anchor: &mut isize,
        allow_a: &mut bool,
        frames: &mut Vec<Frame>,
        lt: &mut LineTokens,
    ) -> bool {
        let Some(top) = frames.last() else {
            return true;
        };
        let rule = &self.grammars.rules[top.rule];
        let flat = self.flatten(top.rule);
        // The end pattern first, unless the rule applies it last.
        let end_re = (rule.kind == Kind::BeginEnd)
            .then(|| {
                rule.end
                    .as_ref()
                    .and_then(|p| self.pattern_regex(p, top.end_pattern.as_deref()))
            })
            .flatten();
        let mut best: Option<(usize, Captures, Option<RuleId>)> = None;
        let consider = |start: usize, caps: Captures, id: Option<RuleId>, best: &mut Option<(usize, Captures, Option<RuleId>)>| {
            if best.as_ref().is_none_or(|(s, _, _)| start < *s) {
                *best = Some((start, caps, id));
            }
        };
        let try_end = |best: &mut Option<(usize, Captures, Option<RuleId>)>| {
            if let Some(re) = &end_re
                && let Some(caps) = self.search(re, text, *pos, *anchor, *allow_a)
            {
                let start = caps[0].map_or(*pos, |c| c.0);
                consider(start, caps, None, best);
            }
        };
        if !rule.apply_end_last {
            try_end(&mut best);
        }
        for &id in flat.iter() {
            if best.as_ref().is_some_and(|(s, _, _)| *s == *pos) {
                break;
            }
            let child = &self.grammars.rules[id];
            let Some(re) = child.start.as_ref().and_then(|p| self.pattern_regex(p, None)) else {
                continue;
            };
            if let Some(caps) = self.search(&re, text, *pos, *anchor, *allow_a) {
                let start = caps[0].map_or(*pos, |c| c.0);
                consider(start, caps, Some(id), &mut best);
            }
        }
        if rule.apply_end_last {
            try_end(&mut best);
        }
        let Some((_, caps, matched)) = best else {
            let scopes = frames.last().and_then(|f| f.content_scopes.clone());
            lt.produce(&scopes, len);
            return true;
        };
        let (s, e) = caps[0].unwrap_or((*pos, *pos));
        let advanced = e > *pos;
        let depth = frames.len();
        match matched {
            None => {
                // The end pattern: close the rule.
                let scopes = frames[depth - 1].content_scopes.clone();
                lt.produce(&scopes, s);
                frames[depth - 1].content_scopes = frames[depth - 1].name_scopes.clone();
                let name_scopes = frames[depth - 1].name_scopes.clone();
                let rule = &self.grammars.rules[frames[depth - 1].rule];
                self.handle_captures(text, &rule.end_captures, &caps, frames, depth - 1, lt);
                lt.produce(&name_scopes, e);
                let popped = frames.pop().expect("a frame");
                *anchor = popped.anchor_pos;
                if !advanced && popped.enter_pos == isize::try_from(*pos).unwrap_or(0) {
                    frames.push(popped);
                    let scopes = frames.last().and_then(|f| f.content_scopes.clone());
                    lt.produce(&scopes, len);
                    return true;
                }
            }
            Some(id) => {
                let child = &self.grammars.rules[id];
                let outer = frames.last().and_then(|f| f.content_scopes.clone());
                lt.produce(&outer, s);
                let name = child.name.as_ref().map(|n| fill_name(n, text, &caps));
                let name_scopes = match &name {
                    Some(n) => push_scopes(&outer, n),
                    None => outer.clone(),
                };
                let before = frames.len();
                let end_pattern = match child.kind {
                    Kind::BeginEnd | Kind::BeginWhile => {
                        let pattern = child.end.as_ref().or(child.while_.as_ref());
                        pattern.map(|p| {
                            if p.has_backrefs {
                                Rc::from(fill_backrefs(&p.source, text, &caps))
                            } else {
                                Rc::from(p.source.as_str())
                            }
                        })
                    }
                    _ => None,
                };
                frames.push(Frame {
                    rule: id,
                    enter_pos: isize::try_from(*pos).unwrap_or(0),
                    anchor_pos: *anchor,
                    begin_eol: e == len,
                    end_pattern,
                    name_scopes: name_scopes.clone(),
                    content_scopes: name_scopes.clone(),
                });
                match child.kind {
                    Kind::BeginEnd | Kind::BeginWhile => {
                        self.handle_captures(text, &child.captures, &caps, frames, before, lt);
                        lt.produce(&name_scopes, e);
                        *anchor = isize::try_from(e).unwrap_or(0);
                        if let Some(content) = &child.content_name {
                            let content = fill_name(content, text, &caps);
                            frames[before].content_scopes = push_scopes(&name_scopes, &content);
                        }
                        if !advanced && same_rule_as_before(frames, before) {
                            frames.pop();
                            let scopes = frames.last().and_then(|f| f.content_scopes.clone());
                            lt.produce(&scopes, len);
                            return true;
                        }
                    }
                    Kind::Match => {
                        self.handle_captures(text, &child.captures, &caps, frames, before, lt);
                        lt.produce(&name_scopes, e);
                        frames.pop();
                        if !advanced {
                            if frames.len() > 1 {
                                frames.pop();
                            }
                            let scopes = frames.last().and_then(|f| f.content_scopes.clone());
                            lt.produce(&scopes, len);
                            return true;
                        }
                    }
                    Kind::Include => {}
                }
            }
        }
        if e > *pos {
            *pos = e;
            *allow_a = false;
        }
        false
    }

    /// Gives each capture group its scopes, and hands its text to the rules
    /// it names. `index` is the frame whose scopes the groups sit inside.
    fn handle_captures(
        &self,
        text: &str,
        captures: &[Option<CaptureRule>],
        caps: &Captures,
        frames: &mut Vec<Frame>,
        index: usize,
        lt: &mut LineTokens,
    ) {
        if captures.is_empty() {
            return;
        }
        let base = frames[index].content_scopes.clone();
        let max_end = caps[0].map_or(0, |c| c.1);
        let mut local: Vec<(Option<Scopes>, usize)> = Vec::new();
        for (i, capture) in captures.iter().enumerate().take(caps.len()) {
            let Some(capture) = capture else { continue };
            let Some((s, e)) = caps[i] else { continue };
            if s == e {
                continue;
            }
            if s > max_end {
                break;
            }
            while let Some((scopes, end)) = local.last() {
                if *end <= s {
                    lt.produce(scopes, *end);
                    local.pop();
                } else {
                    break;
                }
            }
            match local.last() {
                Some((scopes, _)) => lt.produce(scopes, s),
                None => lt.produce(&base, s),
            }
            if let Some(retokenize) = capture.retokenize {
                let name = capture.name.as_ref().map(|n| fill_name(n, text, caps));
                let name_scopes = match &name {
                    Some(n) => push_scopes(&base, n),
                    None => base.clone(),
                };
                let content = match &capture.content_name {
                    Some(c) => push_scopes(&name_scopes, &fill_name(c, text, caps)),
                    None => name_scopes.clone(),
                };
                let mut sub = frames.clone();
                sub.push(Frame {
                    rule: retokenize,
                    enter_pos: isize::try_from(s).unwrap_or(0),
                    anchor_pos: -1,
                    begin_eol: false,
                    end_pattern: None,
                    name_scopes,
                    content_scopes: content,
                });
                self.tokenize_string(&text[..e], lt.last == 0 && s == 0, s, &mut sub, lt, false);
                continue;
            }
            if let Some(name) = &capture.name {
                let from = local.last().map_or(&base, |(scopes, _)| scopes);
                let scopes = push_scopes(from, &fill_name(name, text, caps));
                local.push((scopes, e));
            }
        }
        while let Some((scopes, end)) = local.pop() {
            lt.produce(&scopes, end);
        }
    }
}

/// vscode-textmate's check that a rule pushed without advancing is not the
/// same rule that is already open at the same position.
fn same_rule_as_before(frames: &[Frame], pushed: usize) -> bool {
    let new = &frames[pushed];
    let mut at = pushed;
    while at > 0 {
        at -= 1;
        let frame = &frames[at];
        if frame.enter_pos != new.enter_pos {
            return false;
        }
        if frame.rule == new.rule {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn rule_of(grammars: &Grammars, id: RuleId) -> &Rule {
    &grammars.rules[id]
}
