//! The script line, the import directives, keywords and constants.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The words of `keyword.control.catch-exception.dart`.
pub(super) const EXCEPTIONS: &[&[u8]] =
    &[b"try", b"on", b"catch", b"finally", b"throw", b"rethrow"];

/// The words of `keyword.control.dart`.
pub(super) const CONTROL: &[&[u8]] = &[
    b"break",
    b"case",
    b"continue",
    b"default",
    b"do",
    b"else",
    b"for",
    b"if",
    b"in",
    b"switch",
    b"while",
    b"when",
];

/// The words of `storage.modifier.dart`.
pub(super) const MODIFIERS: &[&[u8]] = &[
    b"macro",
    b"augment",
    b"static",
    b"final",
    b"const",
    b"required",
    b"late",
];

/// Whether a word of [`ASYNCHRONY`] may take a `*` after it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Star {
    /// It may, as `async*` does.
    Optional,
    /// It may not.
    Never,
}

/// The words of `(sync(\*)?|async(\*)?|await|yield(\*)?)`, in its order.
const ASYNCHRONY: &[(&[u8], Star)] = &[
    (b"sync", Star::Optional),
    (b"async", Star::Optional),
    (b"await", Star::Never),
    (b"yield", Star::Optional),
];

/// The types the grammar names `support.class.dart`.
pub(super) const PRIMITIVES: &[&[u8]] = &[b"bool", b"num", b"int", b"double", b"dynamic"];

/// The words of `keyword.declaration.dart` that are a single word and take no
/// lookahead.
const DECLARATIONS: &[&[u8]] = &[
    b"abstract",
    b"sealed",
    b"base",
    b"interface",
    b"class",
    b"enum",
    b"extends",
    b"extension",
    b"external",
    b"factory",
    b"implements",
    b"mixin",
    b"native",
    b"operator",
    b"typedef",
    b"with",
    b"covariant",
];

/// The directives of the grammar's `meta.declaration.dart` rule, in its order.
/// Each is a first word, and the word after blanks that some take, such as
/// `part\s+of`.
const DIRECTIVES: &[(&[u8], Option<&[u8]>)] = &[
    (b"augment", Some(b"library")),
    (b"library", None),
    (b"import", Some(b"augment")),
    (b"import", None),
    (b"part", Some(b"of")),
    (b"part", None),
    (b"export", None),
];

impl Lexer<'_> {
    /// `^(#!.*)$`: the script line.
    pub(super) fn shebang(&self, p: usize) -> Option<usize> {
        (p == 0 && self.starts(p, b"#!")).then(|| self.lim())
    }

    /// `^\w*\b(…)\b` for the directives of [`DIRECTIVES`].
    ///
    /// The `\w*` takes nothing.
    /// After it, `\b` and a word must follow, and a `\b` between two word
    /// characters cannot hold.
    pub(super) fn directive(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        DIRECTIVES.iter().find_map(|&(first, second)| {
            let mut end = self.lit(p, first)?;
            if let Some(second) = second {
                let blanks = self.space_end(end);
                if blanks == end {
                    return None;
                }
                end = self.lit(blanks, second)?;
            }
            self.word_at(end).is_none().then_some(end)
        })
    }

    /// `(?<!\$)\b(a|b|…)\b(?!\$)` for the words `words`.
    /// No `$` touches the word.
    pub(super) fn plain_word(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.prev_is(p, b'$') {
            return None;
        }
        let end = self.word_in(p, words)?;
        (self.at(end) != b'$').then_some(end)
    }

    /// `(?<![$.])\b(new)\b(?!\$)`.
    pub(super) fn new_keyword(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.plain_word(p, &[b"new"])
    }

    /// `(?<!\$)\b(sync(\*)?|async(\*)?|await|yield(\*)?)\b(?!\$)`.
    ///
    /// A `*` is taken only when a word character follows it.
    /// The `\b` after the `*` needs one.
    /// Without it the word ends before the `*`, where `\b` holds.
    pub(super) fn asynchrony(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') || !self.boundary(p) {
            return None;
        }
        let &(word, star) = ASYNCHRONY.iter().find(|&&(word, _)| self.starts(p, word))?;
        let end = p + word.len();
        if star == Star::Optional && self.at(end) == b'*' && self.word_at(end + 1).is_some() {
            return Some(end + 1);
        }
        (self.word_at(end).is_none() && self.at(end) != b'$').then_some(end)
    }

    /// The `keyword.declaration.dart` pattern, `(?<!\$)\b(…)\b(?!\$)`.
    /// Its words are [`DECLARATIONS`], `extension\s+type`, `get(?![(<])` and
    /// `set(?![(<])`.
    ///
    /// `extension type` is one match when `type` ends a word.
    /// `get` and `set` match only when no `(` or `<` follows.
    pub(super) fn declaration(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') || !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        if word == b"extension" {
            let blanks = self.space_end(end);
            if blanks > end
                && let Some(after) = self.lit(blanks, b"type")
                && self.word_at(after).is_none()
                && self.at(after) != b'$'
            {
                return Some(after);
            }
        }
        let found = DECLARATIONS.contains(&word)
            || (matches!(word, b"get" | b"set") && !matches!(self.at(end), b'(' | b'<'));
        (found && self.at(end) != b'$').then_some(end)
    }
}
