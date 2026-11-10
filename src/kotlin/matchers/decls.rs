//! Declarations, calls, references and type annotations.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The `\s*(?<GROUP><…>)?` that ends several expressions: the blanks
    /// after `from`, then a balanced angle group when one follows.
    /// It returns the end of the match and the group's range.
    /// Without a group the match keeps the blanks.
    /// `\s*` is greedy, and the group is optional.
    fn blanks_and_generics(&self, from: usize) -> (usize, Option<(usize, usize)>) {
        let blanks = self.space_end(from);
        match self.angle_group(blanks) {
            Some(end) => (end, Some((blanks, end))),
            None => (blanks, None),
        }
    }

    /// `\b(class|(?:fun\s+)?interface)\s+(name)\s*(?<GROUP>…)?`.
    ///
    /// `\s+` gives blanks back only to a name that starts with a blank.
    /// Such a name cannot match.
    /// One blank run therefore decides.
    pub(super) fn class_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let keyword = self.lit(p, b"class").or_else(|| {
            let fun = self.lit(p, b"fun")?;
            let blanks = self.space_end(fun);
            if blanks == fun {
                return None;
            }
            self.lit(blanks, b"interface")
        });
        let keyword = keyword.or_else(|| self.lit(p, b"interface"))?;
        self.declaration_tail(p, keyword)
    }

    /// `\b(typealias)\s+(name)\s*(?<GROUP>…)?`.
    pub(super) fn type_alias(&mut self, p: usize) -> Option<usize> {
        let keyword = self.lit(p, b"typealias").filter(|_| self.boundary(p))?;
        self.declaration_tail(p, keyword)
    }

    /// The `\s+(name)\s*(?<GROUP>…)?` after a declaration's keyword, which
    /// ends at `keyword`.
    fn declaration_tail(&mut self, p: usize, keyword: usize) -> Option<usize> {
        let name = self.space_end(keyword);
        if name == keyword {
            return None;
        }
        let name_end = self.name_at(name)?;
        let (end, generics) = self.blanks_and_generics(name_end);
        self.capture(1, p, keyword);
        self.capture(2, name, name_end);
        if let Some((from, to)) = generics {
            self.capture(3, from, to);
        }
        Some(end)
    }

    /// `\b(object)(?:\s+(name))?`: there is no `\b` after the keyword.
    pub(super) fn object(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let keyword = self.lit(p, b"object")?;
        self.capture(1, p, keyword);
        let name = self.space_end(keyword);
        if name > keyword
            && let Some(end) = self.name_at(name)
        {
            self.capture(2, name, end);
            return Some(end);
        }
        Some(keyword)
    }

    /// `\b(va[lr])\b\s*(?<GROUP>…)?`.
    pub(super) fn variable_declaration(&mut self, p: usize) -> Option<usize> {
        let keyword = self
            .lit(p, b"val")
            .or_else(|| self.lit(p, b"var"))
            .filter(|&end| self.boundary(p) && self.word_at(end).is_none())?;
        let (end, generics) = self.blanks_and_generics(keyword);
        self.capture(1, p, keyword);
        if let Some((from, to)) = generics {
            self.capture(2, from, to);
        }
        Some(end)
    }

    /// `\b(fun)\b\s*(?<GROUP>…)?\s*(?:(?:(\w+)\.)?(name))?`.
    ///
    /// The tail is optional.
    /// The first way it matches decides.
    /// With a receiver, the name follows the dot.
    /// When no name follows, the receiver is given up and the word itself is
    /// the name.
    pub(super) fn function(&mut self, p: usize) -> Option<usize> {
        let keyword = self.keyword(p, b"fun")?;
        let (after_generics, generics) = self.blanks_and_generics(keyword);
        let start = self.space_end(after_generics);
        let receiver_end = self.word_end(start);
        let named = if receiver_end > start && self.at(receiver_end) == b'.' {
            self.name_at(receiver_end + 1)
                .map(|end| (Some((start, receiver_end)), receiver_end + 1, end))
        } else {
            None
        };
        let named = named.or_else(|| self.name_at(start).map(|end| (None, start, end)));
        self.capture(1, p, keyword);
        if let Some((from, to)) = generics {
            self.capture(2, from, to);
        }
        let Some((receiver, name, end)) = named else {
            return Some(start);
        };
        if let Some((from, to)) = receiver {
            self.capture(4, from, to);
        }
        self.capture(5, name, end);
        Some(end)
    }

    /// `\??\.?(name)\s*(?<GROUP>…)?\s*(?=[({])`.
    ///
    /// The prefix and the blanks have one way to match.
    /// A different split would start the name at a character that cannot
    /// start one.
    /// Another would end the blanks before a character that is neither `(` nor
    /// `{`.
    pub(super) fn function_call(&mut self, p: usize) -> Option<usize> {
        let mut name = p;
        if self.at(name) == b'?' {
            name += 1;
        }
        if self.at(name) == b'.' {
            name += 1;
        }
        let name_end = self.name_at(name)?;
        let (after, generics) = self.blanks_and_generics(name_end);
        let end = self.space_end(after);
        if !matches!(self.at(end), b'(' | b'{') {
            return None;
        }
        self.capture(1, name, name_end);
        if let Some((from, to)) = generics {
            self.capture(2, from, to);
        }
        Some(end)
    }

    /// `\??::(name)`.
    pub(super) fn method_reference(&mut self, p: usize) -> Option<usize> {
        let colons = if self.at(p) == b'?' { p + 1 } else { p };
        let name = self.lit(colons, b"::")?;
        let end = self.name_at(name)?;
        self.capture(1, name, end);
        Some(end)
    }

    /// `\b(\w=)\s*(=)`: one word character, `=`, blanks and `=`.
    pub(super) fn key(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = p + self.word_at(p)?;
        let first = self.lit(name, b"=")?;
        let second = self.space_end(first);
        let end = self.lit(second, b"=")?;
        self.capture(1, p, first);
        self.capture(2, second, end);
        Some(end)
    }

    /// `(?<![:?]):\s*([?\w\s]|->|(?<GROUP>[(<]([^"'()<>]|\g<GROUP>)+[)>]))+`.
    ///
    /// A blank is an item of the repetition as well as a character of `\s*`.
    /// The repetition therefore decides.
    /// It needs one item, and the blanks give one back when nothing else
    /// follows them.
    /// The items share no first character.
    /// Each step has one way to match.
    pub(super) fn type_annotation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b':' || self.prev_is(p, b':') || self.prev_is(p, b'?') {
            return None;
        }
        let mut at = p + 1;
        let mut items = 0_usize;
        loop {
            self.spend(1);
            let next = match self.at(at) {
                b'(' | b'<' => self.bracket_group(at),
                b'-' => self.lit(at, b"->"),
                b'?' => Some(at + 1),
                _ => match (self.word_at(at), self.space_at(at)) {
                    (Some(width), _) | (None, width @ 1..) => Some(at + width),
                    (None, 0) => None,
                },
            };
            let Some(next) = next else { break };
            at = next;
            items += 1;
        }
        (items > 0).then_some(at)
    }
}
