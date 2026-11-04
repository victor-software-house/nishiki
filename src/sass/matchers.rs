//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or what giving them back leaves to test.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//! `^` holds only at the start of the line, since Shiki tokenizes each line
//! alone.

mod lines;
mod selectors;
mod values;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[-0-9A-Z_a-z]`.
fn is_ident(b: u8) -> bool {
    matches!(b, b'-' | b'_') || b.is_ascii_alphanumeric()
}

/// Whether `b` is in the grammar's `[-:a-z]`.
fn is_pseudo(b: u8) -> bool {
    matches!(b, b'-' | b':') || b.is_ascii_lowercase()
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments and whole-line rules;
    /// - strings, interpolations and brackets;
    /// - values: variables, numbers, units, operators and functions;
    /// - at-rules;
    /// - selectors and properties.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::RootAt0 => self.block_comment(p),
            Pat::RootAt0End => self.block_comment_end(p),
            Pat::CommentTag => self.starts_before(p, b"{{").then_some(p),
            Pat::CommentTagEnd => self.starts(p, b"}}").then_some(p),
            Pat::CommentParam => {
                let end = self.lit(p, b"@")?;
                let word = self.word_end(end);
                (word > end).then_some(word)
            }
            Pat::RootAt1 => self.formatter_action(p),
            Pat::RootAt2 => self.css_variables_import(p),
            Pat::RootAt2End | Pat::DoubleSlashEnd => self.line_end(p),
            Pat::PlaceholderSelectorEnd => self.line_end(p).or_else(|| {
                let blank = self.space_at(p);
                (blank > 0).then_some(p + blank)
            }),
            Pat::RootAt9End => self.map_end(p),
            Pat::RootAt20End => self
                .line_end(p)
                .or_else(|| (self.at(p) == b'(').then_some(p)),
            Pat::RootAt21End => self.at_rule_end(p),
            Pat::RootAt22End => self.ends_before(p, b"-#(),.>[_"),
            Pat::RootAt23End => self.ends_before(p, b"(),.>["),
            Pat::RootAt24End => self.ends_before(p, b"(),>["),
            Pat::RootAt30End => self.property_end(p),
            Pat::ImportQuotes => self.import_quotes(p),
            Pat::DoubleSlash => self.lit(p, b"//"),
            Pat::DoubleQuoted | Pat::DoubleQuotedEnd => self.lit(p, b"\""),
            Pat::SingleQuoted | Pat::SingleQuotedEnd => self.lit(p, b"'"),
            Pat::QuotedInterpolation | Pat::Interpolation => self.lit(p, b"#{"),
            Pat::QuotedInterpolationEnd | Pat::InterpolationEnd => self.lit(p, b"}"),
            Pat::CurlyBrackets => matches!(self.at(p), b'{' | b'}').then_some(p + 1),
            Pat::Variable | Pat::VariableRoot => self.variable(p),
            Pat::RootAt9 => {
                let end = self.variable(p)?;
                (self.at(end) == b':').then_some(end)
            }
            Pat::Numeric => self.numeric(p),
            Pat::Operator => self.operator(p),
            Pat::Unit => self.unit(p),
            Pat::Comma => {
                if self.at(p) == b',' {
                    return Some(p + 1);
                }
                self.word_in(p, &[b"and", b"or"])
            }
            Pat::PlaceholderSelector => {
                (self.at(p) == b'%' && !self.digit_before(p) && self.digit_end(p + 1) == p + 1)
                    .then_some(p + 1)
            }
            Pat::RgbValue => self.rgb_value(p),
            Pat::Flag => {
                let end = self.lit(p, b"!")?;
                [b"important".as_slice(), b"default", b"optional", b"global"]
                    .iter()
                    .find_map(|word| self.lit(end, word))
            }
            Pat::Function => self.function_name(p),
            Pat::FunctionContent => [b"url(".as_slice(), b"format(", b"attr("]
                .iter()
                .any(|opening| self.starts_before(p, opening))
                .then_some(p),
            Pat::FunctionContentEnd => {
                if p >= self.lim() {
                    return None;
                }
                let end = self.char_end(p);
                (self.at(end) == b')').then_some(end)
            }
            Pat::ReservedWords => self.word_in(
                p,
                &[
                    b"false", b"from", b"in", b"not", b"null", b"through", b"to", b"true",
                ],
            ),
            Pat::ParentSelector => self.lit(p, b"&"),
            Pat::PropertyValue => {
                let end = self.skip(p, is_ident);
                (end > p).then_some(end)
            }
            Pat::Semicolon => self.lit(p, b";"),
            Pat::Dotdotdot => self.lit(p, b"..."),
            Pat::RootAt17 => self.include(p),
            Pat::RootAt17End => matches!(self.at(p), b'\n' | b'(').then_some(p),
            Pat::RootAt18 => (p == 0).then(|| self.lit(p, b"@use")).flatten(),
            Pat::RootAt18End | Pat::RootAt19End => (self.at(p) == b'\n').then_some(p),
            Pat::RootAt18At0 => self.lit(p, b"as").or_else(|| self.lit(p, b"with")),
            Pat::ParenthesisOpen => self.lit(p, b"("),
            Pat::ParenthesisClose | Pat::RootAt29 | Pat::RootAt30At13 => self.lit(p, b")"),
            Pat::Colon | Pat::RootAt30 => self.lit(p, b":"),
            Pat::RootAt19 => self.import(p),
            Pat::RootAt20 => self.mixin(p),
            Pat::RootAt20At0 => {
                let end = self.dash_word_end(p);
                (end > p).then_some(end)
            }
            Pat::RootAt21 => self.lit(p, b"@"),
            Pat::RootAt22 => self.tag(p),
            Pat::PseudoClass => {
                let end = self.lit(p, b":")?;
                let run = self.skip(end, is_pseudo);
                (run > end).then_some(run)
            }
            Pat::RootAt23 => self.lit(p, b"#"),
            Pat::RootAt24 => self.class_start(p),
            Pat::RootAt25 => self.lit(p, b"["),
            Pat::RootAt25End => self.lit(p, b"]"),
            Pat::RootAt25At2 => matches!(self.at(p), b'$' | b'*' | b'^' | b'~').then_some(p + 1),
            Pat::RootAt26 => self.line_pseudo_class(p),
            Pat::Module => self.module(p),
            Pat::RootAt28 | Pat::RootAt30At12 => {
                let end = self.dash_word_end(p);
                (self.at(end) == b'(').then_some(end + 1)
            }
            Pat::RootAt30At0 => {
                if self.prev(p) != Some(b':') {
                    return None;
                }
                let end = self.skip(p, |b| b == b'-' || b.is_ascii_lowercase());
                let blank = self.space_at(end);
                (end > p && blank > 0).then_some(end + blank)
            }
            Pat::RootAt30At11 => self.custom_property(p),
            Pat::RootAt34 => self.after_brace(p),
            Pat::RootAt34End => {
                let blank = self.space_at(p);
                if blank > 0 {
                    return Some(p + blank);
                }
                matches!(self.at(p), b'\n' | b')' | b',' | b'.' | b'[').then_some(p)
            }
            Pat::RootAt36 => {
                let end = self.skip(p, |b| b == b'-' || b.is_ascii_lowercase());
                (end > p && (self.at(end) == b':' || self.starts(end, b"#{"))).then_some(end)
            }
        }
    }

    /// `\$[-0-9A-Z_a-z]+`.
    fn variable(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"$")?;
        let run = self.skip(end, is_ident);
        (run > end).then_some(run)
    }

    /// The end of a run of `[-\w]` from `i`.
    fn dash_word_end(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            if self.at(i) == b'-' {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }

    /// Whether the line has `text` just before `p`.
    fn starts_before(&self, p: usize, text: &[u8]) -> bool {
        p >= text.len() && self.starts(p - text.len(), text)
    }

    /// Whether the character before `p` is a `\d`.
    fn digit_before(&self, p: usize) -> bool {
        if p == 0 {
            return false;
        }
        let start = self.char_start_before(p);
        self.digit_end(start) == p
    }

    /// Whether the character before `p` is a `\s`.
    fn space_before(&self, p: usize) -> bool {
        if p == 0 {
            return false;
        }
        let start = self.char_start_before(p);
        self.space_at(start) == p - start
    }

    /// `$\n?|(?=[...\s])`: the end of the line, or a lookahead for a blank or
    /// one of `set`.
    /// The first branch comes first, so at the line's end the match takes the
    /// line feed.
    fn ends_before(&self, p: usize, set: &[u8]) -> Option<usize> {
        self.line_end(p)
            .or_else(|| (set.contains(&self.at(p)) || self.space_at(p) > 0).then_some(p))
    }

    /// `$\n?`: the end of the line, with its line feed when it has one.
    fn line_end(&self, p: usize) -> Option<usize> {
        if !self.at_eol(p) {
            return None;
        }
        Some(if self.at(p) == b'\n' { p + 1 } else { p })
    }
}
