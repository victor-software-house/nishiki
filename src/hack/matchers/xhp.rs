//! XHP: elements, attributes, entities and comments in Hack code.

use super::super::engine::Lexer;
use crate::classes::{
    is_connector, is_digit, is_letter, is_letter_number, is_nonenclosing_mark, is_space, is_word,
};
use crate::rules::Chars;

/// `[_\p{L}]`: a character that starts an element or attribute name.
fn is_xhp_start(c: char) -> bool {
    c == '_' || is_letter(c)
}

/// `[-\p{L}\p{Mn}\p{Mc}\d\p{Nl}\p{Pc}]`: a character of an attribute name.
fn is_xhp_attribute(c: char) -> bool {
    c == '-'
        || is_letter(c)
        || is_nonenclosing_mark(c)
        || is_digit(c)
        || is_letter_number(c)
        || is_connector(c)
}

/// `[-:\p{L}\p{Mn}\p{Mc}\d\p{Nl}\p{Pc}]`: a character of an element name.
fn is_xhp_element(c: char) -> bool {
    c == ':' || is_xhp_attribute(c)
}

impl Lexer<'_> {
    /// `(?<=[(,\[{]|&&|\|\||[:=?]|=>|\Wreturn|^return|^)\s*(?=<[_\p{L}])`:
    /// where XHP starts, after a token that an expression follows.
    pub(super) fn xhp_open(&self, p: usize) -> Option<usize> {
        let after_return = p >= 6
            && self.text(p - 6, p) == b"return"
            && (p == 6 || self.char_before(p - 6).is_some_and(|c| !is_word(c)));
        let behind = p == 0
            || matches!(
                self.prev(p),
                Some(b'(' | b',' | b'[' | b'{' | b':' | b'=' | b'?')
            )
            || p >= 2 && matches!(self.text(p - 2, p), b"&&" | b"||" | b"=>")
            || after_return;
        if !behind {
            return None;
        }
        let at = self.space_end(p);
        (self.at(at) == b'<' && self.char_is(at + 1, is_xhp_start)).then_some(at)
    }

    /// `\s*(<)([_\p{L}][-:\p{L}\p{Mn}\p{Mc}\d\p{Nl}\p{Pc}]*+)(?=[/>\s])(?<!:)`:
    /// an element's open tag, whose name group 2 holds.
    /// The `*+` gives nothing back, so a name that ends with `:` fails.
    pub(super) fn xhp_tag(&mut self, p: usize) -> Option<usize> {
        let open = self.space_end(p);
        if self.at(open) != b'<' || !self.char_is(open + 1, is_xhp_start) {
            return None;
        }
        let end = self.chars_while(open + 1, is_xhp_element);
        let ahead = matches!(self.at(end), b'/' | b'>') || self.space_at(end) > 0;
        if !ahead || self.at(end - 1) == b':' {
            return None;
        }
        self.capture(1, open, open + 1);
        self.capture(2, open + 1, end);
        Some(end)
    }

    /// The end of an element, which repeats its name:
    ///
    /// ```text
    /// \s*(?<=</)(\2)(>)|(/>)|((?<=</)[ \S]*?)>
    /// ```
    ///
    /// The lookbehind of the first branch holds only where `\s*` takes
    /// nothing.
    pub(super) fn xhp_tag_end(&mut self, p: usize) -> Option<usize> {
        let after_close = p >= 2 && self.text(p - 2, p) == b"</";
        if after_close
            && let Some(name) = self.begin_text_at(p)
            && self.at(p + name) == b'>'
        {
            self.capture(1, p, p + name);
            self.capture(2, p + name, p + name + 1);
            return Some(p + name + 1);
        }
        if let Some(end) = self.lit(p, b"/>") {
            self.capture(3, p, end);
            return Some(end);
        }
        if !after_close {
            return None;
        }
        let mut q = p;
        while q < self.lim() {
            if self.at(q) == b'>' {
                self.capture(4, p, q);
                return Some(q + 1);
            }
            let (c, end) = self.char_at(q)?;
            if c != ' ' && is_space(c) {
                return None;
            }
            q = end;
        }
        None
    }

    /// `(?<!--)(>)`.
    pub(super) fn xhp_tag_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'>' || p >= self.lim() || p >= 2 && self.text(p - 2, p) == b"--" {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 1)
    }

    /// `(&)([0-9A-Za-z]+|#[0-9]+|#x\h+)(;)`.
    pub(super) fn xhp_entity(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'&' {
            return None;
        }
        let body = p + 1;
        let named = self.skip(body, |b| b.is_ascii_alphanumeric());
        let semi = if named > body && self.at(named) == b';' {
            named
        } else if self.at(body) == b'#' {
            let digits = self.skip(body + 1, |b| b.is_ascii_digit());
            if digits > body + 1 && self.at(digits) == b';' {
                digits
            } else if self.at(body + 1) == b'x' {
                let hex = self.skip(body + 2, |b| b.is_ascii_hexdigit());
                (hex > body + 2 && self.at(hex) == b';').then_some(hex)?
            } else {
                return None;
            }
        } else {
            return None;
        };
        self.capture(1, p, body);
        self.capture(2, body, semi);
        self.capture(3, semi, semi + 1);
        Some(semi + 1)
    }

    /// `&\S*;`: `\S*` takes every character up to white space and gives back
    /// to the last `;`.
    pub(super) fn xhp_loose_entity(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'&' {
            return None;
        }
        let mut q = p + 1;
        let mut semi = None;
        while q < self.lim() {
            let (c, end) = self.char_at(q)?;
            if is_space(c) {
                break;
            }
            if c == ';' {
                semi = Some(end);
            }
            q = end;
        }
        semi
    }

    /// `--\s*>`.
    pub(super) fn xhp_comment_end(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"--")?;
        let close = self.space_end(at);
        (self.at(close) == b'>' && close < self.lim()).then_some(close + 1)
    }

    /// `--(?!-*\s*>)`: dashes in a comment that do not end it.
    pub(super) fn xhp_comment_dashes(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"--")?;
        let close = self.space_end(self.skip(at, |b| b == b'-'));
        (self.at(close) != b'>' || close >= self.lim()).then_some(at)
    }

    /// An attribute's name, after white space and before `=`, `>`, white
    /// space, `/>` or a comment:
    ///
    /// ```text
    /// (?<!\S)([_\p{L}](?:[-\p{L}\p{Mn}\p{Mc}\d\p{Nl}\p{Pc}](?<!\.\.))*+)(?<!\.)(?=//|/\*|[=>\s]|/>)
    /// ```
    ///
    /// The class holds no `.`, so neither lookbehind for one can fail.
    pub(super) fn xhp_attribute_name(&self, p: usize) -> Option<usize> {
        if p > 0 && !self.char_before(p).is_some_and(is_space) {
            return None;
        }
        if !self.char_is(p, is_xhp_start) {
            return None;
        }
        let end = self.chars_while(p, is_xhp_attribute);
        let ahead = matches!(self.at(end), b'=' | b'>')
            || self.space_at(end) > 0
            || self.starts(end, b"//")
            || self.starts(end, b"/*")
            || self.starts(end, b"/>");
        ahead.then_some(end)
    }

    /// `=(?=\s*(?:["'{]|/\*|<|//|\n))`.
    /// Where the spaces run past the line feed, they give it back to `\n`.
    pub(super) fn xhp_assignment(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'=' {
            return None;
        }
        let at = self.space_end(p + 1);
        let ahead = matches!(self.at(at), b'"' | b'\'' | b'{' | b'<') && at < self.lim()
            || self.starts(at, b"/*")
            || self.starts(at, b"//")
            || at > self.lim();
        ahead.then_some(p + 1)
    }

    /// `"(?<!\\")` for `"`, and with `'` for `'`: a quote that no backslash
    /// escapes.
    pub(super) fn unescaped_quote(&self, p: usize, quote: u8) -> Option<usize> {
        (self.at(p) == quote && p < self.lim() && self.prev(p) != Some(b'\\')).then_some(p + 1)
    }
}
