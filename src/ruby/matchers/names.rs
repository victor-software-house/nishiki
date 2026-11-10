//! The names an assignment, a hash key or a symbol gives.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The words a condition starts with, before the variable it assigns.
const CONDITIONS: &[&[u8]] = &[b"case", b"if", b"elsif", b"unless", b"until", b"while"];

/// The operator an assignment ends with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Assign {
    /// `((&&|\|\|)=)`
    Logical,
    /// `(([-%*+/]|\*\*|[\&^|]|<<|>>)=)`
    Augmented,
    /// `=[^=>]`, of which the condition form takes both characters and the
    /// line-start form only looks ahead.
    Plain,
}

/// Where an assigned variable's name starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lead {
    /// `^\s*`: at the start of the line.
    Line,
    /// `(?<!\.)\b(case|if|…|while)\b\s*(\()*?\s*`: after a condition's word.
    Condition,
}

impl Lexer<'_> {
    /// The end of the operator `assign` at `p`.
    fn assign_end(&self, p: usize, assign: Assign) -> Option<usize> {
        match assign {
            Assign::Logical => (self.starts(p, b"&&=") || self.starts(p, b"||=")).then_some(p + 3),
            Assign::Augmented => {
                let one = self.byte_in(p, |b| {
                    matches!(b, b'-' | b'%' | b'*' | b'+' | b'/' | b'&' | b'^' | b'|')
                });
                if one && self.byte_in(p + 1, |b| b == b'=') {
                    Some(p + 2)
                } else {
                    [b"**=", b"<<=", b">>="]
                        .iter()
                        .any(|op| self.starts(p, *op))
                        .then_some(p + 3)
                }
            }
            Assign::Plain => {
                let next = p + 1;
                (self.byte_in(p, |b| b == b'=')
                    && self.avail(next)
                    && !matches!(self.at(next), b'=' | b'>'))
                .then(|| self.char_end_nl(next))
            }
        }
    }

    /// Where the variable's name starts after `lead` at `p`, with where the
    /// condition's word ends for [`Lead::Condition`].
    fn assigned_start(&self, p: usize, lead: Lead) -> Option<(usize, usize)> {
        match lead {
            Lead::Line => (p == 0).then(|| (self.space_end(p), p)),
            Lead::Condition => {
                if !self.not_after_dot(p) {
                    return None;
                }
                let word = self.word_in(p, CONDITIONS)?;
                let parens = self.skip(self.space_end(word), |b| b == b'(');
                Some((self.space_end(parens), word))
            }
        }
    }

    /// An assignment to a local variable, from the start of a line or after a
    /// condition's word: the variable, then `assign`.
    ///
    /// `(\()*?` is lazy, but a `(` is neither a name nor a blank.
    /// It therefore takes every `(` before the name.
    fn assignment(&mut self, p: usize, lead: Lead, assign: Assign) -> Option<usize> {
        let (name, word) = self.assigned_start(p, lead)?;
        let name_end = self.local_end(name)?;
        let op = self.space_end(name_end);
        let op_end = self.assign_end(op, assign)?;
        let (name_group, op_group) = match lead {
            Lead::Line => (1, 2),
            Lead::Condition => {
                self.capture(1, p, word);
                (3, 4)
            }
        };
        self.capture(name_group, name, name_end);
        match (lead, assign) {
            (Lead::Line, Assign::Plain) => Some(op),
            (Lead::Condition, Assign::Plain) => Some(op_end),
            (_, Assign::Logical | Assign::Augmented) => {
                self.capture(op_group, op, op_end);
                Some(op_end)
            }
        }
    }

    /// `^\s*([_a-z][0-9A-Z_a-z]*)\s*((&&|\|\|)=)`
    pub(in crate::ruby) fn line_logical_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Line, Assign::Logical)
    }

    /// `(?<!\.)\b(case|…|while)\b\s*(\()*?\s*([_a-z][0-9A-Z_a-z]*)\s*((&&|\|\
    /// |)=)`
    pub(in crate::ruby) fn condition_logical_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Condition, Assign::Logical)
    }

    /// `^\s*([_a-z][0-9A-Z_a-z]*)\s*(([-%*+/]|\*\*|[\&^|]|<<|>>)=)`
    pub(in crate::ruby) fn line_augmented_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Line, Assign::Augmented)
    }

    /// `(?<!\.)\b(case|…|while)\b\s*(\()*?\s*([_a-z][0-9A-Z_a-z]*)\s*(([-%*+/
    /// ]|…)=)`
    pub(in crate::ruby) fn condition_augmented_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Condition, Assign::Augmented)
    }

    /// `^\s*([_a-z][0-9A-Z_a-z]*)\s*(?==[^=>])`
    pub(in crate::ruby) fn line_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Line, Assign::Plain)
    }

    /// `(?<!\.)\b(case|…|while)\b\s*(\()*?\s*([_a-z][0-9A-Z_a-z]*)\s*=[^=>]`
    pub(in crate::ruby) fn condition_assignment(&mut self, p: usize) -> Option<usize> {
        self.assignment(p, Lead::Condition, Assign::Plain)
    }

    /// The end of `(?>[A-Z_a-z]\w*[!?]?)` at `p`.
    fn key_name_end(&self, p: usize) -> Option<usize> {
        let end = self.name_end(p)?;
        Some(end + usize::from(self.byte_in(end, |b| b == b'!' || b == b'?')))
    }

    /// `(?>[A-Z_a-z]\w*[!?]?)(:)(?!:)`: a hash key written `key:`.
    pub(in crate::ruby) fn hash_key(&mut self, p: usize) -> Option<usize> {
        let colon = self.key_name_end(p)?;
        (self.byte_in(colon, |b| b == b':') && !self.byte_in(colon + 1, |b| b == b':')).then(|| {
            self.capture(1, colon, colon + 1);
            colon + 1
        })
    }

    /// `(?<!:)(:)(?>[A-Z_a-z]\w*[!?]?)(?=\s*=>)`: a symbol before `=>`.
    pub(in crate::ruby) fn arrow_key(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b':') || !self.byte_in(p, |b| b == b':') {
            return None;
        }
        let end = self.key_name_end(p + 1)?;
        self.starts(self.space_end(end), b"=>").then(|| {
            self.capture(1, p, p + 1);
            end
        })
    }

    /// `(?<!:)(:)(?>…)`: a symbol, its name or operator taken by the first of
    /// the atomic group's alternatives that matches.
    pub(in crate::ruby) fn symbol(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b':') || !self.byte_in(p, |b| b == b':') {
            return None;
        }
        let end = self.symbol_name_end(p + 1)?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// The end of the symbol's name at `p`:
    ///
    /// ```text
    /// [$A-Z_a-z]\w*(?>[!?]|=(?![=>]))?|===?|<=>|>[=>]?|<[<=]?|[%\&/`|]|\*\*?|=?~|[-+]@?|\[]=?|@@?[A-Z_a-z]\w*
    /// ```
    fn symbol_name_end(&self, p: usize) -> Option<usize> {
        if self.byte_in(p, |b| b.is_ascii_alphabetic() || b == b'_' || b == b'$') {
            let end = self.word_end(p + 1);
            let suffix = if self.byte_in(end, |b| b == b'!' || b == b'?') {
                1
            } else {
                usize::from(
                    self.byte_in(end, |b| b == b'=')
                        && !self.byte_in(end + 1, |b| b == b'=' || b == b'>'),
                )
            };
            return Some(end + suffix);
        }
        let optional =
            |at: usize, class: &dyn Fn(u8) -> bool| at + usize::from(self.byte_in(at, class));
        if self.starts(p, b"==") {
            return Some(optional(p + 2, &|b| b == b'='));
        }
        if self.starts(p, b"<=>") {
            return Some(p + 3);
        }
        match self.byte_in(p, |_| true).then(|| self.at(p)) {
            Some(b'>') => Some(optional(p + 1, &|b| b == b'=' || b == b'>')),
            Some(b'<') => Some(optional(p + 1, &|b| b == b'<' || b == b'=')),
            Some(b'%' | b'&' | b'/' | b'`' | b'|' | b'~') => Some(p + 1),
            Some(b'*') => Some(optional(p + 1, &|b| b == b'*')),
            Some(b'=') => self.byte_in(p + 1, |b| b == b'~').then_some(p + 2),
            Some(b'-' | b'+') => Some(optional(p + 1, &|b| b == b'@')),
            Some(b'[') => self
                .byte_in(p + 1, |b| b == b']')
                .then(|| optional(p + 2, &|b| b == b'=')),
            Some(b'@') => self.name_end(optional(p + 1, &|b| b == b'@')),
            _ => None,
        }
    }
}
