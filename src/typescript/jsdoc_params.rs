//! The `@typedef` name and optional `[name=default]` parameters of doc
//! comments.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::classes;

/// The pieces of an optional parameter, `[name=default]`.
struct OptionalParam {
    /// The `=`, the start of the default and its end.
    default: Option<(usize, usize, usize)>,
    /// The closing bracket, when there is one.
    close: Option<usize>,
    /// Where the match ends.
    end: usize,
}

impl<const D: u8> Lexer<'_, D> {
    /// `(?:[^*/@\s]|\*[^/])+` as the name of a `@typedef`.
    pub(crate) fn doc_typedef_name(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        while j <= self.len {
            let c = self.byte(j);
            if c == b'*' {
                if self.byte(j + 1) == b'/' {
                    break;
                }
                j += 2;
            } else if matches!(c, b'/' | b'@') || self.space_at(j) {
                break;
            } else {
                j += self.char_len(j);
            }
        }
        if j == p {
            return None;
        }
        self.token(p, j.min(self.len), &[S::EntityNameTypeInstanceJsdoc]);
        Some(j)
    }

    /// `[name=default]`, an optional parameter in a doc comment, checked in
    /// full before anything is emitted.
    /// The grammar's pattern has to match to the closing bracket or to the end
    /// of the comment.
    ///
    /// `(?:(?:\[])?\.[$\w]+)*`
    ///
    /// Blanks before the closing bracket are not part of the value.
    ///
    /// `((?:[^*\s]|\*[^/\s])+)?`
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn optional_param_ahead(&self, p: usize) -> Option<OptionalParam> {
        let name_char = |b: u8| b == b'$' || b == b'_' || b.is_ascii_alphanumeric();
        let mut j = self.ws(p + 1);
        let name_start = j;
        while name_char(self.byte(j)) {
            j += 1;
        }
        self.spend(j - name_start + 1);
        if j == name_start {
            return None;
        }
        loop {
            let k = if self.starts(j, "[]") { j + 2 } else { j };
            if self.byte(k) != b'.' {
                break;
            }
            let mut e = k + 1;
            while name_char(self.byte(e)) {
                e += 1;
            }
            self.spend(e - k);
            if e == k + 1 {
                break;
            }
            j = e;
        }
        let mut end = j;
        let mut default = None;
        let k = self.ws(j);
        if self.byte(k) == b'=' {
            let value = self.ws(k + 1);
            let mut e = value;
            let mut depth = 0usize;
            while e < self.len && !self.starts(e, "*/") {
                match self.byte(e) {
                    b'[' => depth += 1,
                    b']' if depth == 0 => break,
                    b']' => depth -= 1,
                    b'"' | b'\'' => {
                        if let Some(q) = self.quoted(e) {
                            e = q;
                            continue;
                        }
                    }
                    _ => {}
                }
                e += 1;
            }
            self.spend(e - value + 1);
            let mut value_end = e;
            while value_end > value
                && let Some(c) = self.prev_char(value_end)
                && classes::is_space(c)
            {
                value_end -= c.len_utf8();
            }
            default = Some((k, value, value_end));
            end = e;
        }
        let m = self.ws(end);
        if self.byte(m) == b']' {
            let mut t = m + 1;
            while t < self.len && !self.space_at(t) && self.byte(t) != b'*' {
                t += self.char_len(t);
            }
            self.spend(t - m);
            return Some(OptionalParam {
                default,
                close: Some(m),
                end: t,
            });
        }
        self.starts(m, "*/").then_some(OptionalParam {
            default,
            close: None,
            end: m,
        })
    }

    pub(crate) fn doc_optional_param(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'[' {
            return None;
        }
        let found = self.optional_param_ahead(p)?;
        let scope = S::VariableOtherJsdoc;
        self.flush(p);
        self.capture(
            Rule::Top,
            p,
            p + 1,
            &[
                scope,
                S::PunctuationDefinitionOptionalValueBeginBracketSquareJsdoc,
            ],
        );
        if let Some((eq, value, value_end)) = found.default {
            self.capture(Rule::Top, p + 1, eq, &[scope]);
            self.capture(
                Rule::Top,
                eq,
                eq + 1,
                &[scope, S::KeywordOperatorAssignmentJsdoc],
            );
            self.capture(Rule::Top, eq + 1, value, &[scope]);
            self.capture(Rule::Top, value, value_end, &[scope, S::SourceEmbedded]);
        }
        if let Some(close) = found.close {
            self.fill(Rule::Top, close, &[scope]);
            self.capture(
                Rule::Top,
                close,
                close + 1,
                &[
                    scope,
                    S::PunctuationDefinitionOptionalValueEndBracketSquareJsdoc,
                ],
            );
            if found.end > close + 1 {
                self.capture(
                    Rule::Top,
                    close + 1,
                    found.end,
                    &[scope, S::InvalidIllegalSyntaxJsdoc],
                );
            }
        } else {
            self.fill(Rule::Top, found.end, &[scope]);
        }
        Some(found.end)
    }
}
