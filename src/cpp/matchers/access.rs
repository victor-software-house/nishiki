//! Member access: an object, `.` or `->`, a chain of names and the member.

use super::super::engine::Lexer;
use super::super::few::Few;
use crate::cpp::keys::TYPE_KEYS;
use crate::rules::Chars;

/// A way the object part of an access matches.
#[derive(Clone, Copy)]
struct Object {
    end: usize,
    /// Groups, as offsets from the first group of the object part, with
    /// their extents: at most the gap, its three comment parts and `this`.
    captures: Few<(usize, usize, usize), 5>,
}

impl Object {
    /// An object that is one group, `start..end`, followed by blanks up to
    /// `after`.
    fn one(group: usize, start: usize, end: usize, after: usize) -> Object {
        let mut captures = Few::new();
        captures.push((group, start, end));
        Object {
            end: after,
            captures,
        }
    }
}

/// The operator of an access.
#[derive(Clone, Copy)]
enum Operator {
    /// `.` or `.*`.
    Dot,
    /// `->` or `->*`.
    Arrow,
}

/// Where the operator of an access is looked for.
#[derive(Clone, Copy)]
pub(super) enum OperatorAt {
    /// After the object.
    AfterObject,
    /// Before the object, as a look behind.
    Before,
}

impl Lexer<'_> {
    /// `\b(?:…)\b` of the naming styles at `p`: the group offset (0 to 3) and
    /// the end, in the order the expression tries them.
    pub(super) fn style_ends(&self, p: usize) -> Few<(usize, usize), 4> {
        let mut out = Few::new();
        if !self.boundary(p) {
            return out;
        }
        let run = |from: usize, class: fn(u8) -> bool| {
            let mut end = from;
            while end < self.lim() && class(self.s[end]) {
                end += 1;
            }
            end
        };
        let lower = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
        let alnum = |b: u8| b.is_ascii_alphanumeric();
        let end = run(p, lower);
        if end > p && self.boundary(end) {
            out.push((0, end));
        }
        let first = run(p, alnum);
        if first > p && self.at(first) == b'_' && first < self.lim() {
            let end = run(first + 1, alnum);
            if self.boundary(end) {
                out.push((1, end));
            }
        }
        let low = run(p, |b| b.is_ascii_lowercase());
        if low > p && self.at(low).is_ascii_uppercase() && low < self.lim() {
            let end = run(low + 1, alnum);
            if self.boundary(end) {
                out.push((2, end));
            }
        }
        if self.at(p).is_ascii_uppercase() && p < self.lim() {
            let end = run(p + 1, |b| {
                b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'
            });
            if self.boundary(end) {
                out.push((3, end));
            }
        }
        out
    }

    /// `(?:GAP((?<!\w)this(?!\w))|(?:(?:\b(?:…)\b|((?<!\w)ID(?!\w)))|(?
    /// <=[])]))\s+{0,1})`.
    /// It finds the first way the object matches at `p`, in the order the
    /// expression tries them, for which `rest` matches what follows.
    /// It returns what `rest` found.
    /// `gap_groups` is 4 for the gap that names a comment's parts and 2 for
    /// the plain one.
    fn find_object<R>(
        &self,
        p: usize,
        gap_groups: usize,
        mut rest: impl FnMut(&Self, &Object) -> Option<R>,
    ) -> Option<(Object, R)> {
        let this = gap_groups;
        for gap in self.gaps(p) {
            let Some(end) = self.word_lit(gap.end, b"this") else {
                continue;
            };
            let mut captures = Few::new();
            captures.push((0, p, gap.end));
            if gap_groups == 2 {
                if gap.run {
                    captures.push((1, p, gap.end));
                }
            } else if let Some(parts) = gap.comment {
                for (i, (a, b)) in parts.iter().enumerate() {
                    captures.push((1 + i, *a, *b));
                }
            }
            captures.push((this, gap.end, end));
            let object = Object { end, captures };
            if let Some(found) = rest(self, &object) {
                return Some((object, found));
            }
        }
        for (style, end) in self.style_ends(p) {
            let object = Object::one(this + 1 + style, p, end, self.skip_space(end));
            if let Some(found) = rest(self, &object) {
                return Some((object, found));
            }
        }
        if !self.word_before(p) {
            for end in self.ident_ends(p) {
                if self.word_at(end).is_some() {
                    continue;
                }
                let object = Object::one(this + 5, p, end, self.skip_space(end));
                if let Some(found) = rest(self, &object) {
                    return Some((object, found));
                }
            }
        }
        if matches!(self.prev(p), Some(b']' | b')')) {
            let object = Object {
                end: self.skip_space(p),
                captures: Few::new(),
            };
            if let Some(found) = rest(self, &object) {
                return Some((object, found));
            }
        }
        None
    }

    /// `(\.\*?)` or `(->\*?)` at `p`: where it ends and which it is.
    fn access_operator(&self, p: usize) -> Option<(usize, Operator)> {
        if self.at(p) == b'.' && p < self.lim() {
            let star = self.at(p + 1) == b'*' && p + 1 < self.lim();
            return Some((p + 1 + usize::from(star), Operator::Dot));
        }
        if self.starts(p, b"->") {
            let star = self.at(p + 2) == b'*' && p + 2 < self.lim();
            return Some((p + 2 + usize::from(star), Operator::Arrow));
        }
        None
    }

    /// `ID\s+{0,1}(?:\.\*?|->\*?)\s+{0,1}` repeated from `from`, the longest
    /// first, followed by what `rest` matches.
    /// It returns where the chain ends and what `rest` found.
    /// The chain is followed forward, and the last end that `rest` accepts is
    /// kept.
    /// That is the end the longest-first search stops at, with no list of
    /// ends.
    fn find_chain_end<R>(
        &self,
        from: usize,
        mut rest: impl FnMut(&Self, usize) -> Option<R>,
    ) -> Option<(usize, R)> {
        let mut found = rest(self, from).map(|r| (from, r));
        let mut at = from;
        while let Some(name) = self.ident_ends(at).longest() {
            let op = self.skip_space(name);
            let Some((after, _)) = self.access_operator(op) else {
                break;
            };
            at = self.skip_space(after);
            if let Some(r) = rest(self, at) {
                found = Some((at, r));
            }
        }
        found
    }

    fn put_object(&mut self, base: usize, object: &Object) {
        for &(offset, a, b) in object.captures.iter() {
            self.capture(base + offset, a, b);
        }
    }

    /// The groups of an operator found at `at`: dot, then arrow.
    fn put_operator(&mut self, group: usize, at: usize, end: usize, operator: Operator) {
        let group = match operator {
            Operator::Dot => group,
            Operator::Arrow => group + 1,
        };
        self.capture(group, at, end);
    }

    /// `GAPOBJECT(?:(\.\*?)|(->\*?))`, or the form that begins with a look
    /// behind for an operator: the object and the operator after it.
    pub(super) fn object_operator(&mut self, p: usize, at: OperatorAt) -> Option<usize> {
        let start = match at {
            OperatorAt::AfterObject => p,
            OperatorAt::Before => {
                let behind = self.prev(p) == Some(b'.')
                    || self.ends_with_at(p, b".*")
                    || self.ends_with_at(p, b"->")
                    || self.ends_with_at(p, b"->*");
                if !behind {
                    return None;
                }
                self.skip_space(p)
            }
        };
        let (object, (end, operator)) =
            self.find_object(start, 4, |lexer, object| lexer.access_operator(object.end))?;
        self.put_object(1, &object);
        self.put_operator(11, object.end, end, operator);
        Some(end)
    }

    /// `OBJECT(?:(\.\*?)|(->\*?))((?:ID\s*(?:\.\*?|->\*?)\s*)*)\s*(~?ID)\s*(\
    /// ()`.
    pub(super) fn method_access(&mut self, p: usize) -> Option<usize> {
        let (object, (op_end, operator, chain_end, (name, name_end, paren))) =
            self.find_object(p, 4, |lexer, object| {
                let (op_end, operator) = lexer.access_operator(object.end)?;
                let (chain_end, member) = lexer.find_chain_end(op_end, |lexer, chain_end| {
                    let name = lexer.skip_space(chain_end);
                    let at = if lexer.at(name) == b'~' && name < lexer.lim() {
                        name + 1
                    } else {
                        name
                    };
                    let name_end = lexer.ident_ends(at).longest()?;
                    let paren = lexer.skip_space(name_end);
                    (lexer.at(paren) == b'(' && paren < lexer.lim())
                        .then_some((name, name_end, paren))
                })?;
                Some((op_end, operator, chain_end, member))
            })?;
        self.put_object(1, &object);
        self.put_operator(11, object.end, op_end, operator);
        self.capture(13, op_end, chain_end);
        self.capture(14, name, name_end);
        self.capture(15, paren, paren + 1);
        Some(paren + 1)
    }

    /// `OBJECT(?:(\.\*?)|(->\*?))((?:ID\s*(?:\.\*?|->\*?)\s*)*)\s*\b((?!TYPE\
    /// W)ID\b(?!\())`.
    /// A listed word that a non-word character follows is the whole run of
    /// word characters at the member.
    /// The list is sorted.
    pub(super) fn member_access(&mut self, p: usize) -> Option<usize> {
        let (object, (op_end, operator, chain_end, (name, end))) =
            self.find_object(p, 2, |lexer, object| {
                let (op_end, operator) = lexer.access_operator(object.end)?;
                let (chain_end, member) = lexer.find_chain_end(op_end, |lexer, chain_end| {
                    let name = lexer.skip_space(chain_end);
                    if !lexer.boundary(name) {
                        return None;
                    }
                    let run_end = lexer.word_end(name);
                    let excluded = lexer.ch(run_end).is_some()
                        && TYPE_KEYS.contains(lexer.text(name, run_end));
                    if excluded {
                        return None;
                    }
                    let end = lexer
                        .ident_ends(name)
                        .find(|&end| lexer.boundary(end) && lexer.at(end) != b'(')?;
                    Some((name, end))
                })?;
                Some((op_end, operator, chain_end, member))
            })?;
        self.put_object(1, &object);
        self.put_operator(9, object.end, op_end, operator);
        self.capture(11, op_end, chain_end);
        self.capture(12, name, end);
        Some(end)
    }
}
