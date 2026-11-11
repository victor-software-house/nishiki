//! The parameter lists of functions and of generic types: names, then a
//! type.

use super::super::engine::Lexer;
use super::types::Lead;
use crate::rules::{CachedRuns, Chars};

/// Whether white space may stand before each comma of a name list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::go) enum Commas {
    /// `name(?:,\s*name)*`
    Tight,
    /// `name(?:\s*,\s*name)*`
    Spaced,
}

/// The type expressions a parameter list's second group allows after the
/// alternative of function types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Params {
    /// A generic type's parameters: `[*.~\w]+` items and bracketed lists.
    Generic,
    /// A function's parameters: marked names with optional brackets.
    Function,
}

impl Lexer<'_> {
    /// The end of the last name of `name(?:,\s*name)*` at `p`.
    /// `name` gives the end of a name.
    /// A comma that no name follows ends the list before it.
    pub(in crate::go) fn names(
        &self,
        p: usize,
        name: fn(&Self, usize) -> usize,
        commas: Commas,
    ) -> Option<usize> {
        let mut end = name(self, p);
        if end == p {
            return None;
        }
        loop {
            let comma = match commas {
                Commas::Tight => end,
                Commas::Spaced => self.spaces(end),
            };
            if comma >= self.lim() || self.at(comma) != b',' {
                return Some(end);
            }
            let next = self.spaces(comma + 1);
            let next_end = name(self, next);
            if next_end == next {
                return Some(end);
            }
            end = next_end;
        }
    }

    /// The end of the `\w` run at `p`, as a `name` for [`Lexer::names`].
    pub(in crate::go) fn word_name(&self, p: usize) -> usize {
        if p < self.lim() && self.word_at(p).is_some() {
            self.word_run(p)
        } else {
            p
        }
    }

    /// `(?:\b\w+,\s*)+{0,1}\b\w+` at `p`: the names before a parameter's
    /// type.
    /// A shorter list leaves a comma, and the blanks after it cannot take one.
    /// Only the whole list is therefore tried.
    fn parameter_names(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.names(p, Self::word_name, Commas::Tight)
    }

    /// ```text
    /// ((?:\b\w+,\s*)+{0,1}\b\w+)\s+(?=(?:\s*CH)+{0,1}B\b(?:struct|interface)\b\s*\{)
    /// ```
    pub(in crate::go) fn generic_param_types_at3(&mut self, p: usize) -> Option<usize> {
        let names = self.parameter_names(p)?;
        let blank = self.spaces(names);
        if blank == names {
            return None;
        }
        self.after_chans(blank, Lead::Spaces, |lx, q, _| {
            let word = lx.word_in(lx.marks(q), &[b"interface", b"struct"])?;
            lx.opens_curly(lx.spaces(word)).then_some(())
        })?;
        self.capture(1, p, names);
        Some(blank)
    }

    /// `(?:(?<=\()|^\s*)((?:\b\w+,\s*)+(?:/[*/].*)?)$`
    pub(in crate::go) fn generic_param_types_at4(&mut self, p: usize) -> Option<usize> {
        let after_paren = p > 0 && p <= self.lim() && self.at(p - 1) == b'(';
        let start = if after_paren {
            p
        } else if p == 0 {
            self.spaces(p)
        } else {
            return None;
        };
        let mut i = start;
        while !self.word_before(i) && i < self.lim() && self.word_at(i).is_some() {
            let word = self.word_run(i);
            if word >= self.lim() || self.at(word) != b',' {
                break;
            }
            i = self.spaces(word + 1);
        }
        if i == start {
            return None;
        }
        let end = self.trailing_comment(i).unwrap_or(i);
        if !self.at_eol(end) {
            return None;
        }
        self.capture(1, start, end);
        Some(end)
    }

    /// `((?:\b\w+,\s*)+{0,1}\b\w+)\s+((?:\s*CH)+{0,1}(?:F|G))`, with F
    /// function types and G a generic type's parameter types.
    pub(in crate::go) fn generic_param_types_at5(&mut self, p: usize) -> Option<usize> {
        self.parameter(p, Params::Generic)
    }

    /// `((?:\b\w+,\s*)+{0,1}\b\w+)\s+((?:\s*CH)+{0,1}(?:F|G))`, with F
    /// function types and G a function's parameter types.
    pub(in crate::go) fn function_param_types_at5(&mut self, p: usize) -> Option<usize> {
        self.parameter(p, Params::Function)
    }

    /// Names, blanks, then a type: the shared shape of the two expressions
    /// above.
    ///
    /// The second group is not optional.
    /// A shorter list of names would leave a comma where the blanks must be.
    /// Blanks given back leave the type before a blank.
    /// Only a `chan` prefix could start there, and it starts after the blanks
    /// too.
    fn parameter(&mut self, p: usize, params: Params) -> Option<usize> {
        let names = self.parameter_names(p)?;
        let start = self.spaces(names);
        if start == names {
            return None;
        }
        let end = self.after_chans(start, Lead::Spaces, |lx, q, _| {
            lx.function_types(q, params).or_else(|| match params {
                Params::Generic => lx.generic_param_type(q),
                Params::Function => lx.function_param_type(q),
            })
        })?;
        self.capture(1, p, names);
        self.capture(2, start, end);
        Some(end)
    }

    /// `[]*.\[\w]+{0,1}(?:\bfunc\b\([^)]+{0,1}\)(?:\s*CH)+{0,1}\s*)+R?`, with
    /// R `(?:[*.\w]+|\([^)]+{0,1}\))` for a generic type's parameters and
    /// `(?:[]*.\[\w]+|\([^)]+{0,1}\))` for a function's.
    ///
    /// `func` ends the run before it only where `(` follows it.
    /// The run cannot take that `(`.
    /// The repeats and the result then end the expression.
    /// They are taken when they match.
    fn function_types(&self, p: usize, params: Params) -> Option<usize> {
        let run = self.type_run(p);
        let word = run.checked_sub(4).filter(|&at| at >= p)?;
        let mut i = self.function_type(word)?;
        while let Some(next) = self.function_type(i) {
            i = next;
        }
        let result = match params {
            Params::Generic => self.star_dot_words(i),
            Params::Function => self.type_run(i),
        };
        Some(if result > i {
            result
        } else {
            self.parens(i).unwrap_or(i)
        })
    }

    /// The end of `\bfunc\b\([^)]+{0,1}\)(?:\s*CH)+{0,1}\s*` at `p`.
    fn function_type(&self, p: usize) -> Option<usize> {
        let word = self.keyword(p, b"func")?;
        let close = self.parens(word)?;
        Some(self.spaces(self.chans(close, Lead::Spaces)))
    }

    /// The end of `(?:(?:[*.~\w]+|L)[*.\w]+{0,1})+` at `p`, with L the list
    /// of [`Lexer::type_list`].
    fn generic_param_type(&self, p: usize) -> Option<usize> {
        let mut i = p;
        loop {
            let run = self.words_and(i, |b| matches!(b, b'*' | b'.' | b'~'));
            let item = if run > i {
                run
            } else if let Some(end) = self.type_list(i) {
                end
            } else {
                break;
            };
            i = self.star_dot_words(item);
        }
        (i > p).then_some(i)
    }

    /// The end of `(?:B[*.\w]+(?:\[[^]]+])?[*.\w]+{0,1})+` at `p`.
    ///
    /// Each repeat gives its marks back a character at a time until a run
    /// of `[*.\w]` follows them.
    fn function_param_type(&self, p: usize) -> Option<usize> {
        let mut i = p;
        'repeats: loop {
            let marks = self.marks(i);
            for start in (i..=marks).rev() {
                let name = self.star_dot_words(start);
                if name > start {
                    let index = (name < self.lim() && self.at(name) == b'[')
                        .then(|| self.find_byte(name + 1, b']'))
                        .flatten()
                        .filter(|&close| close > name + 1)
                        .map_or(name, |close| close + 1);
                    i = self.star_dot_words(index);
                    continue 'repeats;
                }
            }
            break;
        }
        (i > p).then_some(i)
    }

    /// `\b([.\w]+)`
    pub(in crate::go) fn generic_param_types_at8(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.function_param_types_at8(p)
    }

    /// `([.\w]+)`
    pub(in crate::go) fn function_param_types_at8(&mut self, p: usize) -> Option<usize> {
        let end = self.dot_words(p);
        (end > p).then(|| {
            self.capture(1, p, end);
            end
        })
    }
}
