//! Statements and declarations: `syntax`, `package`, `import`, options,
//! messages, enums, services, oneofs, reserved ranges and fields.
//!
//! Every name run below stops at a byte its class lacks, and what follows a
//! name is a blank, a `=`, a `,`, a `>`, a `;` or a `)`.
//! Giving a name's byte back leaves a name byte where that follower must
//! stand, so no run gives anything back.

use super::{Lexer, NameStart, is_dotted_name, is_name};
use crate::rules::Chars;

/// What `(\w+|\(\w+(\.\w+)*\))(\.\w+)*` matched.
pub(super) struct OptionName {
    /// The end of the first group, the name or the parenthesized name.
    pub(super) first: usize,
    /// The last `(\.\w+)` inside the parentheses.
    pub(super) inner: Option<(usize, usize)>,
    /// The last `(\.\w+)` after the first group.
    pub(super) outer: Option<(usize, usize)>,
    /// The end of the match.
    pub(super) end: usize,
}

impl Lexer<'_> {
    /// `\s*(syntax)\s*(=)\s*("proto[23]")\s*(;)`.
    pub(super) fn syntax(&mut self, p: usize) -> Option<usize> {
        let keyword = self.space_end(p);
        let keyword_end = self.lit(keyword, b"syntax")?;
        let equals = self.space_end(keyword_end);
        let equals_end = self.lit(equals, b"=")?;
        let value = self.space_end(equals_end);
        let value_end = self.lit(value, b"\"proto")?;
        if !matches!(self.at(value_end), b'2' | b'3') || self.at(value_end + 1) != b'"' {
            return None;
        }
        let semicolon = self.space_end(value_end + 2);
        let end = self.lit(semicolon, b";")?;
        self.cap(1, keyword, keyword_end);
        self.cap(2, equals, equals_end);
        self.cap(3, value, value_end + 2);
        self.cap(4, semicolon, end);
        Some(end)
    }

    /// `\s*(package)\s+([.\w]+)\s*(;)`.
    pub(super) fn package(&mut self, p: usize) -> Option<usize> {
        let keyword = self.space_end(p);
        let keyword_end = self.lit(keyword, b"package")?;
        let name = self.space_end(keyword_end);
        let name_end = self.dotted_word_end(name);
        if name == keyword_end || name_end == name {
            return None;
        }
        let semicolon = self.space_end(name_end);
        let end = self.lit(semicolon, b";")?;
        self.cap(1, keyword, keyword_end);
        self.cap(2, name, name_end);
        self.cap(3, semicolon, end);
        Some(end)
    }

    /// `\s*(import)\s+(weak|public)?\s*("[^"]+")\s*(;)`.
    ///
    /// The modifier is tried first; without a path after it, the match is
    /// tried again without it, where a quote must stand at the modifier.
    pub(super) fn import(&mut self, p: usize) -> Option<usize> {
        let keyword = self.space_end(p);
        let keyword_end = self.lit(keyword, b"import")?;
        let after = self.space_end(keyword_end);
        if after == keyword_end {
            return None;
        }
        let modifier = [b"weak".as_slice(), b"public"]
            .iter()
            .find_map(|word| self.lit(after, word))
            .filter(|&end| self.at(self.space_end(end)) == b'"');
        let path = self.space_end(modifier.unwrap_or(after));
        if self.at(path) != b'"' {
            return None;
        }
        let close = self.skip(path + 1, |b| b != b'"');
        if close == path + 1 || self.at(close) != b'"' {
            return None;
        }
        let semicolon = self.space_end(close + 1);
        let end = self.lit(semicolon, b";")?;
        self.cap(1, keyword, keyword_end);
        if let Some(modifier_end) = modifier {
            self.cap(2, after, modifier_end);
        }
        self.cap(3, path, close + 1);
        self.cap(4, semicolon, end);
        Some(end)
    }

    /// `(option)\s+(\w+|\(\w+(\.\w+)*\))(\.\w+)*\s*(=)`.
    pub(super) fn option_statement(&mut self, p: usize) -> Option<usize> {
        let keyword_end = self.lit(p, b"option")?;
        let name_start = self.space_end(keyword_end);
        if name_start == keyword_end {
            return None;
        }
        let name = self.option_name(name_start)?;
        let equals = self.space_end(name.end);
        let end = self.lit(equals, b"=")?;
        self.cap(1, p, keyword_end);
        self.cap(2, name_start, name.first);
        if let Some((start, stop)) = name.inner {
            self.cap(3, start, stop);
        }
        if let Some((start, stop)) = name.outer {
            self.cap(4, start, stop);
        }
        self.cap(5, equals, end);
        Some(end)
    }

    /// `(\w+|\(\w+(\.\w+)*\))(\.\w+)*`: an option's name.
    /// A repeated group records its last repetition.
    pub(super) fn option_name(&self, p: usize) -> Option<OptionName> {
        let (first, inner) = if self.at(p) == b'(' {
            let word = self.word_end(p + 1);
            if word == p + 1 {
                return None;
            }
            let (parts, inner) = self.dotted_parts(word);
            if self.at(parts) != b')' {
                return None;
            }
            (parts + 1, inner)
        } else {
            let word = self.word_end(p);
            if word == p {
                return None;
            }
            (word, None)
        };
        let (end, outer) = self.dotted_parts(first);
        Some(OptionName {
            first,
            inner,
            outer,
            end,
        })
    }

    /// `(\.\w+)*` from `i`: its end and its last repetition.
    fn dotted_parts(&self, mut i: usize) -> (usize, Option<(usize, usize)>) {
        let mut last = None;
        while self.at(i) == b'.' {
            let end = self.word_end(i + 1);
            if end == i + 1 {
                break;
            }
            last = Some((i, end));
            i = end;
        }
        (i, last)
    }

    /// `(message|extend)(\s+)([A-Z_a-z][.0-9A-Z_a-z]*)(\s*)(\{)?` and
    /// `(enum)(\s+)([A-Za-z][0-9A-Z_a-z]*)(\s*)(\{)?`.
    /// The grammar names groups 1 and 3.
    pub(super) fn block_head(
        &mut self,
        p: usize,
        keywords: &[&[u8]],
        start: NameStart,
    ) -> Option<usize> {
        let keyword_end = keywords.iter().find_map(|word| self.lit(p, word))?;
        let name = self.space_end(keyword_end);
        if name == keyword_end {
            return None;
        }
        let name_end = match start {
            NameStart::Any => (self.at(name) == b'_' || self.at(name).is_ascii_alphabetic())
                .then(|| self.skip(name + 1, is_dotted_name)),
            NameStart::Letter => self
                .at(name)
                .is_ascii_alphabetic()
                .then(|| self.skip(name + 1, is_name)),
        }?;
        let brace = self.space_end(name_end);
        self.cap(1, p, keyword_end);
        self.cap(3, name, name_end);
        Some(if self.at(brace) == b'{' {
            brace + 1
        } else {
            brace
        })
    }

    /// `(service)\s+([A-Za-z][.0-9A-Z_a-z]*)\s*\{?` and
    /// `(oneof)\s+([A-Za-z][0-9A-Z_a-z]*)\s*\{?`.
    pub(super) fn named_head(
        &mut self,
        p: usize,
        keyword: &[u8],
        class: impl Fn(u8) -> bool,
    ) -> Option<usize> {
        let keyword_end = self.lit(p, keyword)?;
        let name = self.space_end(keyword_end);
        if name == keyword_end || !self.at(name).is_ascii_alphabetic() {
            return None;
        }
        let name_end = self.skip(name + 1, class);
        let brace = self.space_end(name_end);
        self.cap(1, p, keyword_end);
        self.cap(2, name, name_end);
        Some(if self.at(brace) == b'{' {
            brace + 1
        } else {
            brace
        })
    }

    /// `(\d+)(\s+(to)\s+(\d+))?`.
    /// The optional group is taken whole or not at all.
    pub(super) fn reserved_range(&mut self, p: usize) -> Option<usize> {
        let first = self.digit_end(p);
        if first == p {
            return None;
        }
        self.cap(1, p, first);
        let to = self.space_end(first);
        let Some(to_end) = self.lit(to, b"to").filter(|_| to > first) else {
            return Some(first);
        };
        let last = self.space_end(to_end);
        let last_end = self.digit_end(last);
        if last == to_end || last_end == last {
            return Some(first);
        }
        self.cap(3, to, to_end);
        self.cap(4, last, last_end);
        Some(last_end)
    }

    /// `([A-Za-z][0-9A-Z_a-z]*)\s*(=)\s*(-?0[Xx]\h+|-?[0-9]+)`.
    /// `0x` with no hex digit after it falls to the decimal branch, as `0`.
    pub(super) fn enum_value(&mut self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_alphabetic() {
            return None;
        }
        let name_end = self.skip(p + 1, is_name);
        let equals = self.space_end(name_end);
        let equals_end = self.lit(equals, b"=")?;
        let value = self.space_end(equals_end);
        let digits = if self.at(value) == b'-' {
            value + 1
        } else {
            value
        };
        let end = self.integer(digits)?;
        self.cap(1, p, name_end);
        self.cap(2, equals, equals_end);
        self.cap(3, value, end);
        Some(end)
    }

    /// `0[Xx]\h+|[0-9]+` from `i`.
    fn integer(&self, i: usize) -> Option<usize> {
        if self.at(i) == b'0'
            && matches!(self.at(i + 1), b'X' | b'x')
            && self.at(i + 2).is_ascii_hexdigit()
        {
            return Some(self.skip(i + 3, |b| b.is_ascii_hexdigit()));
        }
        self.at(i)
            .is_ascii_digit()
            .then(|| self.skip(i + 1, |b| b.is_ascii_digit()))
    }

    /// `\s*(optional|repeated|required)?\s*(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\
    /// h+|[0-9]+)`.
    ///
    /// The label is tried first and then left out, so `required x = 1` takes
    /// `required` as the type.
    /// `\.?[.\w]+` is a run of `[.\w]`: the optional dot gives itself back
    /// when the run would be empty.
    pub(super) fn field(&mut self, p: usize) -> Option<usize> {
        let label = self.space_end(p);
        let labelled = [b"optional".as_slice(), b"repeated", b"required"]
            .iter()
            .find_map(|word| self.lit(label, word))
            .and_then(|label_end| {
                self.field_rest(self.space_end(label_end))
                    .map(|rest| (label_end, rest))
            });
        let (label_end, rest) = match labelled {
            Some((label_end, rest)) => (Some(label_end), rest),
            None => (None, self.field_rest(label)?),
        };
        if let Some(label_end) = label_end {
            self.cap(1, label, label_end);
        }
        let [kind, name, equals, number] = rest;
        self.cap(2, kind.0, kind.1);
        self.cap(3, name.0, name.1);
        self.cap(4, equals.0, equals.1);
        self.cap(5, number.0, number.1);
        Some(number.1)
    }

    /// `(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\h+|[0-9]+)` from `i`: the spans of
    /// its four groups.
    fn field_rest(&self, i: usize) -> Option<[(usize, usize); 4]> {
        let kind_end = self.dotted_word_end(i);
        let name = self.space_end(kind_end);
        let name_end = self.word_end(name);
        if kind_end == i || name == kind_end || name_end == name {
            return None;
        }
        let equals = self.space_end(name_end);
        let equals_end = self.lit(equals, b"=")?;
        let number = self.space_end(equals_end);
        let number_end = self.integer(number)?;
        Some([
            (i, kind_end),
            (name, name_end),
            (equals, equals_end),
            (number, number_end),
        ])
    }

    /// `\s*(map)\s*(<)\s*(\.?[.\w]+)\s*,\s*(\.?[.\w]+)\s*(>)\s+(\w+)\s*(=)\s*(\
    /// d+)`.
    pub(super) fn map_field(&mut self, p: usize) -> Option<usize> {
        let keyword = self.space_end(p);
        let keyword_end = self.lit(keyword, b"map")?;
        let open = self.space_end(keyword_end);
        let open_end = self.lit(open, b"<")?;
        let key = self.space_end(open_end);
        let key_end = self.dotted_word_end(key);
        let comma = self.space_end(key_end);
        let comma_end = self.lit(comma, b",")?;
        let value = self.space_end(comma_end);
        let value_end = self.dotted_word_end(value);
        let close = self.space_end(value_end);
        let close_end = self.lit(close, b">")?;
        let name = self.space_end(close_end);
        let name_end = self.word_end(name);
        let equals = self.space_end(name_end);
        if key_end == key || value_end == value || name == close_end || name_end == name {
            return None;
        }
        let equals_end = self.lit(equals, b"=")?;
        let number = self.space_end(equals_end);
        let number_end = self.digit_end(number);
        if number_end == number {
            return None;
        }
        self.cap(1, keyword, keyword_end);
        self.cap(2, open, open_end);
        self.cap(3, key, key_end);
        self.cap(4, value, value_end);
        self.cap(5, close, close_end);
        self.cap(6, name, name_end);
        self.cap(7, equals, equals_end);
        self.cap(8, number, number_end);
        Some(number_end)
    }
}
