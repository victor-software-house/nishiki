//! The grammar's regular expressions as hand-written matchers.
//!
//! Each matcher answers whether its expression matches starting at a given
//! position, and with which capture groups, including where backtracking
//! lands.
//! The text ends with a line feed that is not in the line.
//! `\s` matches that line feed, and `.` does not.

use memchr::memchr;

use super::engine::{Hit, Lexer};
use super::rules::Pattern;
use super::scope::S;
use super::state::Rule;
use crate::classes::{self, is_ascii_space};
use crate::utf8::char_at;

/// How the parts of a name are scoped.
#[derive(Clone, Copy)]
enum Names {
    /// The name of a tag.
    /// The namespace, the colon and the local name each carry their own
    /// scope.
    Tag,
    /// The name of a tag with nothing in it: each part also carries the scope
    /// of the whole name.
    NoContent,
    /// The name of an attribute.
    Attribute,
}

/// A byte of `[-0-9A-Z_a-z]`.
fn is_pi_name(b: u8) -> bool {
    b == b'-' || b == b'_' || b.is_ascii_alphanumeric()
}

/// A byte of `[-0-:A-Z_a-z]`: the local name of a tag with nothing in it.
fn is_plain_name(b: u8) -> bool {
    is_pi_name(b) || b == b':'
}

/// A byte of `[:A-Z_a-z]`: the start of an entity or document name.
fn is_name_start(b: u8) -> bool {
    b == b':' || b == b'_' || b.is_ascii_alphabetic()
}

/// A byte of `[-.0-:A-Z_a-z]`: the rest of an entity or document name.
fn is_name_rest(b: u8) -> bool {
    is_plain_name(b) || b == b'.'
}

impl Lexer<'_> {
    // Character classes.

    /// The character at `i` and its width.
    fn char_at(&self, i: usize) -> Option<(char, usize)> {
        if i == self.lim {
            return Some(('\n', 1));
        }
        char_at(self.s, i)
    }

    /// The width of the white space character at `i`, 0 when there is none.
    pub(super) fn space_at(&self, i: usize) -> usize {
        match self.at(i) {
            0 => 0,
            b if b < 0x80 => usize::from(is_ascii_space(b)),
            _ => match self.char_at(i) {
                Some((c, width)) if classes::is_space(c) => width,
                _ => 0,
            },
        }
    }

    /// The end of the run of white space from `i`: the regex `\s*`.
    pub(super) fn ws(&self, i: usize) -> usize {
        let (from, to) = self.space_run.get();
        if from <= i && i < to {
            self.spend(1);
            return to;
        }
        let mut j = i;
        while let width @ 1.. = self.space_at(j) {
            j += width;
        }
        self.spend(j - i + 1);
        self.space_run.set((i, j));
        j
    }

    /// The end of the run of `[-.:\w]` from `i`: the name of a tag or an
    /// attribute.
    fn name_end(&self, i: usize) -> usize {
        let (from, to) = self.name_run.get();
        if from <= i && i < to {
            self.spend(1);
            return to;
        }
        let mut j = i;
        while let Some(&b) = self.s.get(j) {
            if b < 0x80 {
                if !(b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.' | b':')) {
                    break;
                }
                j += 1;
            } else {
                match self.char_at(j) {
                    Some((c, width)) if classes::is_word(c) => j += width,
                    _ => break,
                }
            }
        }
        self.spend(j - i + 1);
        if j > i {
            self.name_run.set((i, j));
        }
        j
    }

    /// The end of the run of ASCII bytes from `i` that `class` accepts.
    fn scan(&self, i: usize, class: fn(u8) -> bool) -> usize {
        let mut j = i;
        while self.s.get(j).is_some_and(|&b| class(b)) {
            j += 1;
        }
        self.spend(j - i + 1);
        j
    }

    /// Whether the text at `i` is `word`.
    fn lit(&self, i: usize, word: &[u8]) -> bool {
        self.s
            .get(i..i + word.len())
            .is_some_and(|text| text == word)
    }

    /// The end of the first `>` at or after `i`, or the end of the text: the
    /// regex `[^>]*`.
    fn before_angle(&self, i: usize) -> usize {
        let (from, to) = self.angle_run.get();
        if from <= i && i <= to {
            self.spend(1);
            return to;
        }
        let i = i.min(self.lim);
        let end = memchr(b'>', &self.s[i..]).map_or(self.lim, |offset| i + offset);
        self.spend(end - i + 1);
        self.angle_run.set((i, end));
        end
    }

    // Names.

    /// The colon that splits the name `i..r` into a namespace and a local
    /// name: `(?:(name):)?local` with both parts at least one character.
    /// It is the first colon unless it starts or ends the name.
    fn colon(&self, i: usize, r: usize) -> Option<usize> {
        let at = i + memchr(b':', &self.s[i..r])?;
        (at > i && at + 1 < r).then_some(at)
    }

    /// `hit` with the parts of the name `i..r` scoped as `names` says.
    fn name_caps(&self, hit: Hit, i: usize, r: usize, names: Names) -> Hit {
        let colon = self.colon(i, r);
        let local_from = colon.map_or(i, |at| at + 1);
        let mut hit = hit;
        if let Some(at) = colon {
            hit = match names {
                Names::Tag => hit.cap(i, at, S::TagNamespace).nested(
                    at,
                    at + 1,
                    S::TagName,
                    S::NamespaceSeparator,
                ),
                Names::NoContent => hit.nested(i, at, S::TagName, S::TagNamespace).nested(
                    at,
                    at + 1,
                    S::TagName,
                    S::NamespaceSeparator,
                ),
                Names::Attribute => hit.cap(i, at, S::AttrNamespace).nested(
                    at,
                    at + 1,
                    S::AttrName,
                    S::NamespaceSeparator,
                ),
            };
        }
        match names {
            Names::Tag => hit.cap(local_from, r, S::TagLocal),
            Names::NoContent => hit.nested(local_from, r, S::TagName, S::TagLocal),
            Names::Attribute => hit.cap(local_from, r, S::AttrLocal),
        }
    }

    // Ends.

    /// The end pattern of `rule` at `p`.
    pub(super) fn end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        match rule {
            Rule::Pi => self
                .lit(p, b"?>")
                .then(|| Hit::new(p, p + 2).cap(p, p + 2, S::PunctTag)),
            Rule::Doctype => {
                let i = self.ws(p);
                (self.at(i) == b'>').then(|| Hit::new(p, i + 1).cap(i, i + 1, S::PunctTag))
            }
            Rule::Subset => {
                (self.at(p) == b']').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::PunctConstant))
            }
            Rule::EntityDecl => {
                (self.at(p) == b'>').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::PunctTag))
            }
            Rule::Comment => self
                .lit(p, b"-->")
                .then(|| Hit::new(p, p + 3).cap(p, p + 3, S::PunctComment)),
            Rule::NoContent => self.no_content_end(p),
            Rule::Tag => match self.at(p) {
                b'>' => Some(Hit::new(p, p + 1).cap(p, p + 1, S::PunctTag)),
                b'/' if self.at(p + 1) == b'>' => {
                    Some(Hit::new(p, p + 2).cap(p, p + 2, S::PunctTag))
                }
                _ => None,
            },
            Rule::JspProps | Rule::Jsp => self
                .lit(p, b"%>")
                .then(|| Hit::new(p, p + 2).cap(p, p + 2, S::EmbeddedEnd)),
            Rule::Cdata => self
                .lit(p, b"]]>")
                .then(|| Hit::new(p, p + 3).cap(p, p + 3, S::StringEnd)),
            Rule::Double => {
                (self.at(p) == b'"').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::StringEnd))
            }
            Rule::Single => {
                (self.at(p) == b'\'').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::StringEnd))
            }
            Rule::Root | Rule::CommentBad | Rule::CommentJsp => None,
        }
    }

    /// `(>)(</)(name)(>)`.
    fn no_content_end(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'>' || !self.lit(p + 1, b"</") {
            return None;
        }
        let i = p + 3;
        let r = self.scan(i, is_plain_name);
        if r == i || self.at(r) != b'>' {
            return None;
        }
        let hit = Hit::new(p, r + 1)
            .cap(p, p + 1, S::PunctTag)
            .cap(p + 1, p + 3, S::PunctTag);
        Some(
            self.name_caps(hit, i, r, Names::NoContent)
                .cap(r, r + 1, S::PunctTag),
        )
    }

    // Patterns.

    /// The pattern `pattern` at `p`.
    pub(super) fn pattern_at(&self, pattern: Pattern, p: usize) -> Option<Hit> {
        match pattern {
            Pattern::Pi => self.pi(p),
            Pattern::Doctype => self.doctype(p),
            Pattern::CommentJsp => self
                .lit(p, b"<%--")
                .then(|| Hit::new(p, p + 4).cap(p, p + 4, S::PunctComment)),
            Pattern::Comment => self
                .lit(p, b"<!--")
                .then(|| Hit::new(p, p + 4).cap(p, p + 4, S::PunctComment)),
            Pattern::NoContent => self.no_content(p),
            Pattern::Tag => self.tag(p),
            Pattern::Entity => self.entity(p),
            Pattern::Ampersand => (self.at(p) == b'&').then(|| Hit::new(p, p + 1)),
            Pattern::JspProps => self
                .lit(p, b"<%@")
                .then(|| Hit::new(p, p + 3).cap(p, p + 3, S::EmbeddedBegin)),
            Pattern::Jsp => self.jsp_begin(p),
            Pattern::Cdata => self
                .lit(p, b"<![CDATA[")
                .then(|| Hit::new(p, p + 9).cap(p, p + 9, S::StringBegin)),
            Pattern::PiAttr => self.pi_attribute(p),
            Pattern::Double => {
                (self.at(p) == b'"').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::StringBegin))
            }
            Pattern::Single => {
                (self.at(p) == b'\'').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::StringBegin))
            }
            Pattern::Subset => {
                (self.at(p) == b'[').then(|| Hit::new(p, p + 1).cap(p, p + 1, S::PunctConstant))
            }
            Pattern::EntityDecl => self.entity_declaration(p),
            Pattern::ParameterEntity => self.parameter_entity(p),
            Pattern::CommentBad => (self.lit(p, b"--") && self.at(p + 2) != b'>')
                .then(|| Hit::new(p, p + 2).cap(p, p + 2, S::InvalidComment)),
            Pattern::Attribute => self.attribute(p),
            Pattern::PageProp => ["page", "include", "taglib"]
                .iter()
                .find(|word| self.lit(p, word.as_bytes()))
                .map(|word| Hit::new(p, p + word.len())),
        }
    }

    /// `(<\?)\s*([-0-9A-Z_a-z]+)`.
    fn pi(&self, p: usize) -> Option<Hit> {
        if !self.lit(p, b"<?") {
            return None;
        }
        let i = self.ws(p + 2);
        let r = self.scan(i, is_pi_name);
        (r > i).then(|| {
            Hit::new(p, r)
                .cap(p, p + 2, S::PunctTag)
                .cap(i, r, S::TagName)
        })
    }

    /// `(<!)(DOCTYPE)\s+([:A-Z_a-z][-.0-:A-Z_a-z]*)`.
    fn doctype(&self, p: usize) -> Option<Hit> {
        if !self.lit(p, b"<!DOCTYPE") {
            return None;
        }
        let i = self.ws(p + 9);
        if i == p + 9 {
            return None;
        }
        let r = self.document_name(i)?;
        Some(
            Hit::new(p, r)
                .cap(p, p + 2, S::PunctTag)
                .cap(p + 2, p + 9, S::KeywordDoctype)
                .cap(i, r, S::DocumentRoot),
        )
    }

    /// The end of the name `[:A-Z_a-z][-.0-:A-Z_a-z]*` at `i`.
    fn document_name(&self, i: usize) -> Option<usize> {
        is_name_start(self.at(i)).then(|| self.scan(i + 1, is_name_rest))
    }

    /// `(<)((?:(name)(:))?local)(?=(\s[^>]*)?></\2>)`.
    fn no_content(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'<' {
            return None;
        }
        let i = p + 1;
        let r = self.scan(i, is_plain_name);
        if r == i {
            return None;
        }
        let j = if self.at(r) == b'>' {
            r
        } else {
            let width = self.space_at(r);
            if width == 0 {
                return None;
            }
            self.before_angle(r + width)
        };
        let name = &self.s[i..r];
        let closes = self.at(j) == b'>'
            && self.lit(j + 1, b"</")
            && self.lit(j + 3, name)
            && self.at(j + 3 + name.len()) == b'>';
        closes.then(|| {
            let hit = Hit::new(p, r).cap(p, p + 1, S::PunctTag);
            self.name_caps(hit, i, r, Names::NoContent)
        })
    }

    /// `(</?)(?:([-.\w]+)((:)))?([-.:\w]+)`.
    fn tag(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'<' {
            return None;
        }
        let i = if self.at(p + 1) == b'/' { p + 2 } else { p + 1 };
        let r = self.name_end(i);
        (r > i).then(|| {
            let hit = Hit::new(p, r).cap(p, i, S::PunctTag);
            self.name_caps(hit, i, r, Names::Tag)
        })
    }

    /// `(&)([:A-Z_a-z][-.0-:A-Z_a-z]*|#[0-9]+|#x\h+)(;)`.
    fn entity(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'&' {
            return None;
        }
        let first = self.at(p + 1);
        let r = if first == b'#' {
            let digits = self.scan(p + 2, |b| b.is_ascii_digit());
            if digits > p + 2 {
                digits
            } else if self.at(p + 2) == b'x' {
                let hex = self.scan(p + 3, |b| b.is_ascii_hexdigit());
                if hex == p + 3 {
                    return None;
                }
                hex
            } else {
                return None;
            }
        } else if is_name_start(first) {
            self.scan(p + 2, is_name_rest)
        } else {
            return None;
        };
        (self.at(r) == b';').then(|| {
            Hit::new(p, r + 1)
                .cap(p, p + 1, S::PunctConstant)
                .cap(r, r + 1, S::PunctConstant)
        })
    }

    /// `<%` with `[!=]?(?!--)`.
    fn jsp_begin(&self, p: usize) -> Option<Hit> {
        if !self.lit(p, b"<%") {
            return None;
        }
        let i = p + 2;
        let dashes = |at: usize| self.at(at) == b'-' && self.at(at + 1) == b'-';
        let end = if matches!(self.at(i), b'!' | b'=') && !dashes(i + 1) {
            i + 1
        } else if dashes(i) {
            return None;
        } else {
            i
        };
        Some(Hit::new(p, end).cap(p, end, S::EmbeddedBegin))
    }

    /// ` ([-A-Za-z]+)`: the whole match is the attribute.
    fn pi_attribute(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b' ' {
            return None;
        }
        let r = self.scan(p + 1, |b| b == b'-' || b.is_ascii_alphabetic());
        (r > p + 1).then(|| Hit::new(p, r))
    }

    /// `(<!)(ENTITY)\s+(%\s+)?(name)(\s+(?:SYSTEM|PUBLIC)\s+)?`.
    fn entity_declaration(&self, p: usize) -> Option<Hit> {
        if !self.lit(p, b"<!ENTITY") {
            return None;
        }
        let mut i = self.ws(p + 8);
        if i == p + 8 {
            return None;
        }
        let mut hit = Hit::new(p, p)
            .cap(p, p + 2, S::PunctTag)
            .cap(p + 2, p + 8, S::KeywordEntity);
        if self.at(i) == b'%' {
            let j = self.ws(i + 1);
            if j > i + 1 {
                hit = hit.cap(i, j, S::PunctEntity);
                i = j;
            }
        }
        let r = self.document_name(i)?;
        hit = hit.cap(i, r, S::EntityName);
        let mut end = r;
        let after = self.ws(r);
        if after > r {
            for word in ["SYSTEM", "PUBLIC"] {
                if self.lit(after, word.as_bytes()) {
                    let tail = self.ws(after + word.len());
                    if tail > after + word.len() {
                        hit = hit.cap(r, tail, S::KeywordEntityType);
                        end = tail;
                    }
                    break;
                }
            }
        }
        hit.end = end;
        Some(hit)
    }

    /// `(%)([:A-Z_a-z][-.0-:A-Z_a-z]*)(;)`.
    fn parameter_entity(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'%' {
            return None;
        }
        let r = self.document_name(p + 1)?;
        (self.at(r) == b';').then(|| {
            Hit::new(p, r + 1)
                .cap(p, p + 1, S::PunctConstant)
                .cap(r, r + 1, S::PunctConstant)
        })
    }

    /// `(?:^|\s+)(?:([-.\w]+)((:)))?([-.:\w]+)\s*=`.
    fn attribute(&self, p: usize) -> Option<Hit> {
        let i = if p == 0 && self.space_at(0) == 0 {
            0
        } else if self.space_at(p) > 0 {
            self.ws(p)
        } else {
            return None;
        };
        let r = self.name_end(i);
        if r == i {
            return None;
        }
        let e = self.ws(r);
        if self.at(e) != b'=' {
            return None;
        }
        Some(self.name_caps(Hit::new(p, e + 1), i, r, Names::Attribute))
    }
}
