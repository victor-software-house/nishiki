//! nishiki's bytes format for a compiled grammar, before linking.
//!
//! The bytes open with a header of 16 bytes:
//!
//! - [`MAGIC`], eight bytes;
//! - [`VERSION`], a little-endian `u32`;
//! - the length of the body, a little-endian `u32`.
//!
//! The body holds the fields of [`Compiled`] in order.
//! A count or index is a little-endian `u32`.
//! A string is its length, then its UTF-8 bytes.
//! An optional value is a flag byte, `0` or `1`, then the value when the flag
//! is `1`.
//!
//! Reading checks everything before it trusts the tables:
//!
//! - each count against the bytes left, before it allocates;
//! - each string as UTF-8;
//! - each rule index against the number of rules;
//! - the shape the compiler writes, as [`check_shape`] states it.
//!
//! The shape check keeps crafted bytes from making the linker recurse
//! through a cycle or a chain deeper than any JSON grammar holds.

use super::Reason;
use crate::textmate::compile::{Compiled, RawCapture, RawRule, Repository, RuleId};

/// The bytes a compiled grammar opens with.
const MAGIC: &[u8; 8] = b"nishiki\0";

/// The version of the layout below.
/// Any change to the layout bumps it.
const VERSION: u32 = 1;

/// The length of the header: the magic, the version and the body's length.
const HEADER: usize = 16;

/// How deep rules may nest inside each other.
///
/// `serde_json` reads JSON at most 128 levels deep.
/// A rule sits at least one level below the rule that holds it, so no
/// compiled JSON grammar nests deeper.
const MAX_DEPTH: usize = 128;

/// The fewest bytes a rule takes: twelve flags and three counts of empty
/// lists.
const MIN_RULE: usize = 12 + 3 * 4;

/// Writes `grammar` in the format.
///
/// A count past `u32::MAX`, which only a grammar over 4 GiB can hold, is
/// written as `u32::MAX`.
/// [`read`] then refuses the bytes, since they hold fewer items than it
/// counts.
pub(super) fn write(grammar: &Compiled) -> Vec<u8> {
    let mut body = Writer(Vec::new());
    body.compiled(grammar);
    let mut out = Vec::with_capacity(HEADER + body.0.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&number(body.0.len()).to_le_bytes());
    out.extend_from_slice(&body.0);
    out
}

/// Reads bytes [`write`] wrote, and checks them.
pub(super) fn read(bytes: &[u8]) -> Result<Compiled, Reason> {
    if bytes.get(..MAGIC.len()) != Some(MAGIC.as_slice()) {
        return Err(Reason::NotGrammarBytes);
    }
    let mut reader = Reader {
        bytes,
        at: MAGIC.len(),
        rule_starts: Vec::new(),
    };
    let found = reader.u32()?;
    if found != VERSION {
        return Err(Reason::Version {
            found,
            expected: VERSION,
        });
    }
    let length = reader.length()?;
    if bytes.len() - HEADER != length {
        return Err(Reason::Malformed { at: HEADER });
    }
    let grammar = reader.compiled()?;
    if reader.at != bytes.len() {
        return Err(Reason::Malformed { at: reader.at });
    }
    check_shape(&grammar, &reader.rule_starts)?;
    Ok(grammar)
}

/// `n` as the format's `u32`, or `u32::MAX` when it does not fit.
fn number(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The body, as it is written.
struct Writer(Vec<u8>);

impl Writer {
    fn compiled(&mut self, grammar: &Compiled) {
        let Compiled {
            scope,
            rules,
            patterns,
            repository,
            injections,
            injection_selector,
            inject_to,
        } = grammar;
        self.string(scope);
        self.count(rules.len());
        for rule in rules {
            self.rule(rule);
        }
        self.ids(patterns);
        self.repository(repository);
        self.repository(injections);
        self.option(injection_selector.as_deref(), Writer::string);
        self.strings(inject_to);
    }

    fn rule(&mut self, rule: &RawRule) {
        let RawRule {
            include,
            name,
            content_name,
            matched,
            begin,
            end,
            until,
            captures,
            begin_captures,
            end_captures,
            while_captures,
            apply_end_last,
            patterns,
            has_patterns,
            repository,
        } = rule;
        self.option(include.as_deref(), Writer::string);
        self.strings(name);
        self.strings(content_name);
        for text in [matched, begin, end, until] {
            self.option(text.as_deref(), Writer::string);
        }
        for list in [captures, begin_captures, end_captures, while_captures] {
            self.option(list.as_deref(), Writer::captures);
        }
        self.flag(*apply_end_last);
        self.ids(patterns);
        self.flag(*has_patterns);
        self.option(repository.as_ref(), Writer::repository);
    }

    fn captures(&mut self, captures: &[RawCapture]) {
        self.count(captures.len());
        for RawCapture { index, rule } in captures {
            self.option(*index, Writer::count);
            self.count(*rule);
        }
    }

    fn repository(&mut self, entries: &Repository) {
        self.count(entries.len());
        for (name, rule) in entries {
            self.string(name);
            self.count(*rule);
        }
    }

    fn ids(&mut self, ids: &[RuleId]) {
        self.count(ids.len());
        for &id in ids {
            self.count(id);
        }
    }

    fn strings(&mut self, texts: &[String]) {
        self.count(texts.len());
        for text in texts {
            self.string(text);
        }
    }

    fn string(&mut self, text: &str) {
        self.count(text.len());
        self.0.extend_from_slice(text.as_bytes());
    }

    fn option<T>(&mut self, value: Option<T>, write: impl FnOnce(&mut Writer, T)) {
        self.flag(value.is_some());
        if let Some(value) = value {
            write(self, value);
        }
    }

    fn flag(&mut self, flag: bool) {
        self.0.push(u8::from(flag));
    }

    fn count(&mut self, n: usize) {
        self.0.extend_from_slice(&number(n).to_le_bytes());
    }
}

/// The bytes being read, and where.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
    /// Where each rule starts, so a shape error names its rule's offset.
    rule_starts: Vec<usize>,
}

impl Reader<'_> {
    fn compiled(&mut self) -> Result<Compiled, Reason> {
        let scope = self.string()?;
        let count = self.count(MIN_RULE)?;
        let mut rules = Vec::with_capacity(count);
        for _ in 0..count {
            self.rule_starts.push(self.at);
            rules.push(self.rule()?);
        }
        Ok(Compiled {
            scope,
            rules,
            patterns: self.ids()?,
            repository: self.repository()?,
            injections: self.repository()?,
            injection_selector: self.option(Reader::string)?,
            inject_to: self.strings()?,
        })
    }

    fn rule(&mut self) -> Result<RawRule, Reason> {
        Ok(RawRule {
            include: self.option(Reader::string)?,
            name: self.strings()?,
            content_name: self.strings()?,
            matched: self.option(Reader::string)?,
            begin: self.option(Reader::string)?,
            end: self.option(Reader::string)?,
            until: self.option(Reader::string)?,
            captures: self.option(Reader::captures)?,
            begin_captures: self.option(Reader::captures)?,
            end_captures: self.option(Reader::captures)?,
            while_captures: self.option(Reader::captures)?,
            apply_end_last: self.flag()?,
            patterns: self.ids()?,
            has_patterns: self.flag()?,
            repository: self.option(Reader::repository)?,
        })
    }

    /// A captures list: each capture takes at least a flag and a rule index.
    fn captures(&mut self) -> Result<Vec<RawCapture>, Reason> {
        let count = self.count(1 + 4)?;
        let mut captures = Vec::with_capacity(count);
        for _ in 0..count {
            captures.push(RawCapture {
                index: self.option(Reader::length)?,
                rule: self.length()?,
            });
        }
        Ok(captures)
    }

    /// A repository: each entry takes at least a string's length and a rule
    /// index.
    fn repository(&mut self) -> Result<Repository, Reason> {
        let count = self.count(4 + 4)?;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push((self.string()?, self.length()?));
        }
        Ok(entries)
    }

    fn ids(&mut self) -> Result<Vec<RuleId>, Reason> {
        let count = self.count(4)?;
        (0..count).map(|_| self.length()).collect()
    }

    fn strings(&mut self) -> Result<Vec<String>, Reason> {
        let count = self.count(4)?;
        (0..count).map(|_| self.string()).collect()
    }

    fn string(&mut self) -> Result<String, Reason> {
        let start = self.at;
        let length = self.count(1)?;
        let text = self.take(length)?;
        String::from_utf8(text.to_vec()).map_err(|_not_utf8| Reason::Malformed { at: start })
    }

    fn option<T>(
        &mut self,
        read: impl FnOnce(&mut Self) -> Result<T, Reason>,
    ) -> Result<Option<T>, Reason> {
        if self.flag()? {
            read(self).map(Some)
        } else {
            Ok(None)
        }
    }

    fn flag(&mut self) -> Result<bool, Reason> {
        let at = self.at;
        match self.take(1)? {
            [0] => Ok(false),
            [1] => Ok(true),
            _ => Err(Reason::Malformed { at }),
        }
    }

    /// A count of items that each take at least `size` bytes, checked against
    /// the bytes left before anything is allocated for them.
    fn count(&mut self, size: usize) -> Result<usize, Reason> {
        let at = self.at;
        let count = self.length()?;
        let left = self.bytes.len() - self.at;
        if count > left / size {
            return Err(Reason::Malformed { at });
        }
        Ok(count)
    }

    fn length(&mut self) -> Result<usize, Reason> {
        let at = self.at;
        usize::try_from(self.u32()?).map_err(|_too_large| Reason::Malformed { at })
    }

    fn u32(&mut self) -> Result<u32, Reason> {
        let at = self.at;
        let bytes = self.take(4)?;
        let bytes: [u8; 4] = bytes
            .try_into()
            .map_err(|_not_four| Reason::Malformed { at })?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn take(&mut self, n: usize) -> Result<&[u8], Reason> {
        let at = self.at;
        let end = at.checked_add(n).ok_or(Reason::Malformed { at })?;
        let taken = self.bytes.get(at..end).ok_or(Reason::Malformed { at })?;
        self.at = end;
        Ok(taken)
    }
}

/// Checks that `grammar`'s rules have the shape the compiler writes.
///
/// - Every rule index is in range.
/// - Every rule is named exactly once: by the grammar's top level, or by a rule
///   with a lower index.
/// - No rule sits more than [`MAX_DEPTH`] levels deep.
///
/// `starts` holds each rule's offset, for the error.
fn check_shape(grammar: &Compiled, starts: &[usize]) -> Result<(), Reason> {
    let count = grammar.rules.len();
    let mut depth: Vec<Option<usize>> = vec![None; count];
    let top = grammar
        .patterns
        .iter()
        .chain(grammar.repository.iter().map(|(_, id)| id))
        .chain(grammar.injections.iter().map(|(_, id)| id));
    for &id in top {
        place(&mut depth, id, 1, HEADER)?;
    }
    for (id, rule) in grammar.rules.iter().enumerate() {
        let at = starts.get(id).copied().unwrap_or(HEADER);
        let Some(Some(level)) = depth.get(id).copied() else {
            return Err(Reason::Malformed { at });
        };
        for child in children(rule) {
            if child <= id {
                return Err(Reason::Malformed { at });
            }
            place(&mut depth, child, level + 1, at)?;
        }
    }
    Ok(())
}

/// Records that rule `id` sits at `level`, named from the rule at offset
/// `at`.
/// It fails when `id` is out of range, already named, or too deep.
fn place(depth: &mut [Option<usize>], id: RuleId, level: usize, at: usize) -> Result<(), Reason> {
    match depth.get_mut(id) {
        Some(slot @ None) if level <= MAX_DEPTH => {
            *slot = Some(level);
            Ok(())
        }
        Some(_) | None => Err(Reason::Malformed { at }),
    }
}

/// The rules `rule` names: its patterns, its captures' rules and its
/// repository's rules.
fn children(rule: &RawRule) -> impl Iterator<Item = RuleId> + '_ {
    let captures = [
        &rule.captures,
        &rule.begin_captures,
        &rule.end_captures,
        &rule.while_captures,
    ]
    .into_iter()
    .flatten()
    .flatten()
    .map(|capture| capture.rule);
    let repository = rule.repository.iter().flatten().map(|&(_, id)| id);
    rule.patterns
        .iter()
        .copied()
        .chain(captures)
        .chain(repository)
}
