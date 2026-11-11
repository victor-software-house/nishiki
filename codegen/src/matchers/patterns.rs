//! The patterns of a grammar, named and ordered as the language's `Pattern`
//! enum names and orders them.
//!
//! `oracle/proto-grammar.ts` writes that enum.
//! This walk follows its rules:
//!
//! - a repository entry's pattern takes the entry's key, in Pascal case;
//! - any other pattern takes its parent's name, `At` and its index;
//! - a begin rule's end takes the begin's name and `End`;
//! - `$self` names the top-level patterns from `Root`;
//! - a name already taken is not walked again.

use nishiki::unstable::grammar::{RuleId, UnlinkedCapture, UnlinkedRule};

use crate::error::Error;
use crate::source::Source;

/// One pattern of a grammar.
#[derive(Debug)]
pub(super) struct Pattern {
    /// Its variant of the language's `Pattern` enum, such as `CommentsAt1End`.
    pub(super) name: String,
    /// Its regular expression.
    pub(super) regex: &'static str,
    /// The groups the lexer reads after a match: each capture with scopes.
    pub(super) reads: Vec<usize>,
}

/// Each pattern of `source`, in the enum's order.
pub(super) fn patterns(source: Source) -> Result<Vec<Pattern>, Error> {
    let mut walk = Walk {
        source,
        found: Vec::new(),
    };
    walk.list(source.patterns(), "Root")?;
    Ok(walk.found)
}

/// A walk over a grammar's patterns.
struct Walk {
    /// The grammar.
    source: Source,
    /// The patterns named so far, in order.
    found: Vec<Pattern>,
}

impl Walk {
    /// Names the patterns of `ids`, the list of the rule named `parent`.
    fn list(&mut self, ids: &[RuleId], parent: &str) -> Result<(), Error> {
        for (index, &id) in ids.iter().enumerate() {
            let rule = self.source.rule(id)?;
            match rule.include.as_deref() {
                Some("$self") => self.list(self.source.patterns(), "Root")?,
                Some(include) => {
                    let key = include.strip_prefix('#').ok_or_else(|| {
                        Error::Shape(format!(
                            "{}: an outside include {include}",
                            self.source.name
                        ))
                    })?;
                    self.rule(self.source.entry(key)?, &pascal(key))?;
                }
                None => self.rule(rule, &format!("{parent}At{index}"))?,
            }
        }
        Ok(())
    }

    /// Names `rule` as `name`, or the patterns it holds.
    /// A scope of the rule that holds `$N` is an error.
    fn rule(&mut self, rule: &'static UnlinkedRule, name: &str) -> Result<(), Error> {
        if rule
            .name
            .iter()
            .chain(rule.content_name.iter())
            .any(|scope| scope.contains('$'))
        {
            return Err(self.shape(name, "has a scope that reads a group"));
        }
        if let Some(regex) = rule.matched.as_deref() {
            return self.add(name, regex, rule.captures.as_deref());
        }
        if let Some(begin) = rule.begin.as_deref() {
            let end = rule.end.as_deref().ok_or_else(|| {
                Error::Shape(format!(
                    "{}: {name} has a begin and no end",
                    self.source.name
                ))
            })?;
            if self.has(name, begin)? {
                return Ok(());
            }
            if back_reference(end) {
                return Err(self.shape(name, "has an end that repeats a begin group"));
            }
            let captures = rule.captures.as_deref();
            self.add(name, begin, rule.begin_captures.as_deref().or(captures))?;
            self.add(
                &format!("{name}End"),
                end,
                rule.end_captures.as_deref().or(captures),
            )?;
            return self.list(&rule.patterns, name);
        }
        if rule.patterns.is_empty() {
            return Err(Error::Shape(format!(
                "{}: {name} is an empty rule",
                self.source.name
            )));
        }
        self.list(&rule.patterns, name)
    }

    /// The error that the pattern `name` has a shape the writer does not cover.
    fn shape(&self, name: &str, what: &str) -> Error {
        Error::Shape(format!("{}: {name} {what}", self.source.name))
    }

    /// Whether a pattern named `name` was found, with the regex `regex`.
    /// The same name on another regex is an error.
    fn has(&self, name: &str, regex: &str) -> Result<bool, Error> {
        match self.found.iter().find(|pattern| pattern.name == name) {
            None => Ok(false),
            Some(pattern) if pattern.regex == regex => Ok(true),
            Some(_) => Err(Error::Shape(format!(
                "{}: {name} names two patterns",
                self.source.name
            ))),
        }
    }

    /// Adds the pattern `name`, unless it was found already.
    ///
    /// The writer covers patterns whose lexer reads only the captures with
    /// scopes.
    /// A capture with patterns of its own, or with a scope that holds `$N`,
    /// is an error.
    fn add(
        &mut self,
        name: &str,
        regex: &'static str,
        captures: Option<&[UnlinkedCapture]>,
    ) -> Result<(), Error> {
        if self.has(name, regex)? {
            return Ok(());
        }
        let mut reads = Vec::new();
        for capture in captures.unwrap_or_default() {
            let rule = self.source.rule(capture.rule)?;
            if !rule.patterns.is_empty() {
                return Err(self.shape(name, "has a capture with patterns"));
            }
            if rule.name.iter().any(|scope| scope.contains('$')) {
                return Err(self.shape(name, "has a scope that reads a group"));
            }
            if let Some(index) = capture.index.filter(|&index| index > 0)
                && !rule.name.is_empty()
                && !reads.contains(&index)
            {
                reads.push(index);
            }
        }
        reads.sort_unstable();
        self.found.push(Pattern {
            name: name.to_owned(),
            regex,
            reads,
        });
        Ok(())
    }
}

/// Whether `regex` holds a back-reference `\1` to `\9`.
fn back_reference(regex: &str) -> bool {
    let bytes = regex.as_bytes();
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        if byte == b'\\' {
            if bytes
                .get(index + 1)
                .is_some_and(|next| (b'1'..=b'9').contains(next))
            {
                return true;
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    false
}

/// `key` in Pascal case: each piece between `-`, `_` and `.` capitalised.
fn pascal(key: &str) -> String {
    key.split(['-', '_', '.'])
        .map(|piece| {
            let mut chars = piece.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}
