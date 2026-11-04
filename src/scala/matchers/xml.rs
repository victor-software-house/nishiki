//! XML literals.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether `b` is `[0-9A-Z_a-z]`.
fn is_tag_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Whether `b` is `[-0-9A-Z_a-z]`.
fn is_tag_word_or_dash(b: u8) -> bool {
    b == b'-' || is_tag_word(b)
}

/// Whether `b` is `[-0-:A-Z_a-z]`: `-`, the digits, `:` and the letters.
fn is_local_name(b: u8) -> bool {
    b == b':' || is_tag_word_or_dash(b)
}

/// A tag name: where its optional namespace and its local name end.
#[derive(Clone, Copy, Debug)]
struct TagName {
    /// The end of the namespace, before its `:`, when there is one.
    namespace: Option<usize>,
    /// Where the local name starts.
    local: usize,
    /// Where the local name ends.
    end: usize,
}

impl Lexer<'_> {
    /// `(?:(ns)((:)))?(local)` at `i`, where `ns` is a first character of
    /// `first_ns` and more of `rest_ns`, and `local` a `[0-9A-Z_a-z]` and more
    /// of `[-0-:A-Z_a-z]`.
    ///
    /// The namespace comes first: its run must end at a `:`.
    /// The local name's class holds the namespace's and `:`, so both forms end
    /// at the same place.
    fn tag_name(&self, i: usize, rest_ns: fn(u8) -> bool) -> Option<TagName> {
        if !is_tag_word(self.at(i)) || i >= self.lim() {
            return None;
        }
        let ns_end = self.skip(i + 1, rest_ns);
        if self.at(ns_end) == b':' && ns_end < self.lim() && is_tag_word(self.at(ns_end + 1)) {
            let local = ns_end + 1;
            let end = self.skip(local + 1, is_local_name);
            return Some(TagName {
                namespace: Some(ns_end),
                local,
                end,
            });
        }
        let end = self.skip(i + 1, is_local_name);
        Some(TagName {
            namespace: None,
            local: i,
            end,
        })
    }

    /// Records a tag name's groups: the namespace in `ns`, the `((:))` pair in
    /// `colon` and `colon + 1`, and the local name in `local`.
    fn cap_tag_name(&mut self, start: usize, name: TagName, groups: [usize; 3]) {
        let [ns, colon, local] = groups;
        if let Some(ns_end) = name.namespace {
            self.cap(ns, start, ns_end);
            self.cap(colon, ns_end, ns_end + 1);
            self.cap(colon + 1, ns_end, ns_end + 1);
        }
        self.cap(local, name.local, name.end);
    }

    /// `(<)((?:(ns)((:)))?(local))(?=(\s[^>]*)?></\2>)`: the start of an
    /// element with no content.
    ///
    /// `[^>]*` runs to the first `>`, where `></` and the name must follow.
    pub(super) fn empty_element(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'<' || p >= self.lim() {
            return None;
        }
        let start = p + 1;
        let name = self.tag_name(start, is_tag_word)?;
        let close = if self.space_at(name.end) > 0 {
            name.end + memchr::memchr(b'>', self.s.get(name.end..)?)?
        } else {
            name.end
        };
        let tag = self.s.get(start..name.end)?;
        let rest = self.s.get(close..)?;
        if !(rest.starts_with(b"></")
            && rest.get(3..).is_some_and(|after| after.starts_with(tag))
            && rest.get(3 + tag.len()) == Some(&b'>'))
        {
            return None;
        }
        self.cap(1, p, start);
        self.cap_tag_name(start, name, [3, 4, 6]);
        Some(name.end)
    }

    /// `(>(<))/(?:(ns)((:)))?(local)(>)`, where `ns` is `[-0-9A-Z_a-z]+` and
    /// `local` is `[-0-:A-Z_a-z]*[0-9A-Z_a-z]`: the end of an element with no
    /// content.
    ///
    /// The local name must end at the `>`, on a `[0-9A-Z_a-z]`.
    pub(super) fn empty_element_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'>'
            || self.at(p + 1) != b'<'
            || self.at(p + 2) != b'/'
            || p + 2 >= self.lim()
        {
            return None;
        }
        let start = p + 3;
        let ns_end = self.skip(start, is_tag_word_or_dash);
        let with_ns = (ns_end > start && self.at(ns_end) == b':' && ns_end < self.lim())
            .then(|| {
                self.local_close(ns_end + 1)
                    .map(|end| (Some(ns_end), ns_end + 1, end))
            })
            .flatten();
        let (namespace, local, end) = match with_ns {
            Some(found) => found,
            None => (None, start, self.local_close(start)?),
        };
        self.cap(1, p, p + 2);
        self.cap(2, p + 1, p + 2);
        if let Some(ns_end) = namespace {
            self.cap(3, start, ns_end);
            self.cap(4, ns_end, ns_end + 1);
            self.cap(5, ns_end, ns_end + 1);
        }
        self.cap(6, local, end);
        self.cap(7, end, end + 1);
        Some(end + 1)
    }

    /// The end of `[-0-:A-Z_a-z]*[0-9A-Z_a-z]` at `i` when a `>` follows it.
    fn local_close(&self, i: usize) -> Option<usize> {
        let end = self.skip(i, is_local_name);
        (end > i && is_tag_word(self.at(end - 1)) && self.at(end) == b'>' && end < self.lim())
            .then_some(end)
    }

    /// `(</?)(?:(ns)((:)))?(local)(?=[^>]*?>)`: the start of a tag, where `ns`
    /// is `[0-9A-Z_a-z][-0-9A-Z_a-z]*`.
    ///
    /// The lookahead needs a `>` after the name.
    pub(super) fn tag_open(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'<' || p >= self.lim() {
            return None;
        }
        let start = p + 1 + usize::from(self.at(p + 1) == b'/');
        let name = self.tag_name(start, is_tag_word_or_dash)?;
        memchr::memchr(b'>', self.s.get(name.end..)?)?;
        self.cap(1, p, start);
        self.cap_tag_name(start, name, [2, 3, 5]);
        Some(name.end)
    }

    /// `(/?>)`.
    pub(super) fn tag_close(&mut self, p: usize) -> Option<usize> {
        let start = p + usize::from(self.at(p) == b'/');
        if self.at(start) != b'>' || start >= self.lim() {
            return None;
        }
        self.cap(1, p, start + 1);
        Some(start + 1)
    }

    /// ` (?:([-0-9A-Z_a-z]+)((:)))?([-A-Z_a-z]+)=`: an attribute's name.
    pub(super) fn attribute(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b' ' || p >= self.lim() {
            return None;
        }
        let start = p + 1;
        let local_eq = |i: usize| {
            let end = self.skip(i, |b| b == b'-' || b == b'_' || b.is_ascii_alphabetic());
            (end > i && self.at(end) == b'=' && end < self.lim()).then_some(end)
        };
        let ns_end = self.skip(start, is_tag_word_or_dash);
        let with_ns = (ns_end > start && self.at(ns_end) == b':')
            .then(|| local_eq(ns_end + 1).map(|end| (Some(ns_end), ns_end + 1, end)))
            .flatten();
        let (namespace, local, end) = match with_ns {
            Some(found) => found,
            None => (None, start, local_eq(start)?),
        };
        if let Some(ns_end) = namespace {
            self.cap(1, start, ns_end);
            self.cap(2, ns_end, ns_end + 1);
            self.cap(3, ns_end, ns_end + 1);
        }
        self.cap(4, local, end);
        Some(end + 1)
    }

    /// `(&)([:A-Z_a-z][-.0-:A-Z_a-z]*|#[0-9]+|#x\h+)(;)`.
    pub(super) fn entity(&mut self, p: usize) -> Option<usize> {
        let amp = self.lit_byte(p, b'&')?;
        let b = self.at(amp);
        let body = if b == b':' || b == b'_' || b.is_ascii_alphabetic() {
            Some(self.skip(amp + 1, |b| b == b'.' || is_local_name(b)))
        } else if b == b'#' {
            let digits = self.skip(amp + 1, |b| b.is_ascii_digit());
            if digits > amp + 1 && self.at(digits) == b';' {
                Some(digits)
            } else if self.at(amp + 1) == b'x' {
                let hex = self.skip(amp + 2, |b| b.is_ascii_hexdigit());
                (hex > amp + 2).then_some(hex)
            } else {
                None
            }
        } else {
            None
        }?;
        if self.at(body) != b';' || body >= self.lim() {
            return None;
        }
        self.cap(1, p, amp);
        self.cap(3, body, body + 1);
        Some(body + 1)
    }

    /// The end of the byte `byte` at `i`.
    pub(super) fn lit_byte(&self, i: usize, byte: u8) -> Option<usize> {
        (self.at(i) == byte && i < self.lim()).then_some(i + 1)
    }
}
