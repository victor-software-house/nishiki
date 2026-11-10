//! Where the languages HAML embeds hand the line back: the ends of HAML's
//! embedding rules, and the patterns those rules list before the language.
//!
//! Each test reads the line's bytes `s` alone, without the line feed.
//! The line's end, `s.len()`, stands for the line feed, which the grammar's
//! `\s` and `\R` take.
//! The matchers use the same tests, so a pattern and its halt give the same
//! answer.

use crate::classes::space_width;

/// The end of the run of `\s` characters from `i`, within the line.
fn blanks_end(s: &[u8], mut i: usize) -> usize {
    loop {
        let width = space_width(s, i);
        if width == 0 {
            return i;
        }
        i += width;
    }
}

/// `\1\s+|$\n*` at the line's start: the line starts with `indent` and a
/// `\s`, which may be the line feed, or the line is empty.
pub(in crate::haml) fn stays_in_filter(indent: &[u8], s: &[u8]) -> bool {
    s.is_empty()
        || (s.starts_with(indent) && (s.len() == indent.len() || space_width(s, indent.len()) > 0))
}

/// The end of a Ruby line at `p`, or a pattern the line lists before Ruby.
///
/// The end's `$` holds at the line's end and past its line feed.
/// Ruby's `(\.|::)\s*` takes the line feed after `- a.`, and Ruby then
/// halts past it, where vscode-textmate tries the line's end.
pub(in crate::haml) fn ruby_line(s: &[u8], p: usize) -> bool {
    p >= s.len()
        || block_end(s, p).is_some()
        || php_keyword(s, p).is_some()
        || php_include(s, p).is_some()
        || php_exception(s, p).is_some()
        || php_call(s, p, b"function").is_some()
        || php_call(s, p, b"use").is_some()
        || block_opener(s, p).is_some()
        || comment(s, p).is_some()
}

/// `(do|\{)( \|[*.]+\|)?$`, the first branch of a Ruby line's end: where
/// `do` or `{` ends, and where the match ends.
///
/// The optional group is tried first; without it, `$` must follow the opener.
pub(in crate::haml) fn block_end(s: &[u8], p: usize) -> Option<(usize, usize)> {
    let opener = if s.get(p..).is_some_and(|rest| rest.starts_with(b"do")) {
        p + 2
    } else if s.get(p) == Some(&b'{') {
        p + 1
    } else {
        return None;
    };
    if s.get(opener..).is_some_and(|rest| rest.starts_with(b" |")) {
        let marks = opener + 2;
        let close = marks
            + s[marks..]
                .iter()
                .take_while(|&&b| matches!(b, b'*' | b'.'))
                .count();
        if close > marks && s.get(close) == Some(&b'|') && close + 1 == s.len() {
            return Some((opener, close + 1));
        }
    }
    (opener == s.len()).then_some((opener, opener))
}

/// A blank run and then `word` at `p`, as `\s+` and the word: where the word
/// starts.
fn after_blanks(s: &[u8], p: usize, word: &[u8]) -> Option<usize> {
    let at = blanks_end(s, p);
    (at > p && s[at..].starts_with(word)).then_some(at)
}

/// `(?=[(\s])` at `i`: a `(` or a `\s`; the line's end stands for its line
/// feed.
fn paren_or_blank(s: &[u8], i: usize) -> bool {
    i == s.len() || s.get(i) == Some(&b'(') || space_width(s, i) > 0
}

/// `\s+((elseif|foreach|switch|declare|default|use))(?=[(\s])`: group 1's
/// start and end, and the match's end.
pub(in crate::haml) fn php_keyword(s: &[u8], p: usize) -> Option<(usize, usize, usize)> {
    [
        &b"elseif"[..],
        b"foreach",
        b"switch",
        b"declare",
        b"default",
        b"use",
    ]
    .iter()
    .find_map(|word| {
        let at = after_blanks(s, p, word)?;
        let end = at + word.len();
        paren_or_blank(s, end).then_some((at, end, end))
    })
}

/// `\s+((?:requir|includ)e_once)(?=[(\s])`.
pub(in crate::haml) fn php_include(s: &[u8], p: usize) -> Option<(usize, usize, usize)> {
    [&b"require_once"[..], b"include_once"]
        .iter()
        .find_map(|word| {
            let at = after_blanks(s, p, word)?;
            let end = at + word.len();
            paren_or_blank(s, end).then_some((at, end, end))
        })
}

/// `\s+(catch|try|throw|exception|finally|die)(?=[(\s]|\n*)`.
/// Its lookahead always holds, since `\n*` matches nothing.
pub(in crate::haml) fn php_exception(s: &[u8], p: usize) -> Option<usize> {
    [
        &b"catch"[..],
        b"try",
        b"throw",
        b"exception",
        b"finally",
        b"die",
    ]
    .iter()
    .find_map(|word| after_blanks(s, p, word).map(|at| at + word.len()))
}

/// `\s+(word\s*)((?=\())`, for `function` and `use`: group 1's start and end,
/// and the match's end.
///
/// Giving back a blank of `\s*` would leave a blank where `(` must stand.
pub(in crate::haml) fn php_call(s: &[u8], p: usize, word: &[u8]) -> Option<(usize, usize, usize)> {
    let at = after_blanks(s, p, word)?;
    let end = blanks_end(s, at + word.len());
    (s.get(end) == Some(&b'(')).then_some((at, end, end))
}

/// `([,<|]|do|\{)\s*(#.*)?$\n*`: a mark that opens a block, and the line's
/// end after blanks and a comment.
/// It ends at the line's end, before the line feed.
/// vscode-textmate then tries the Ruby line's end, `$`, which closes the line
/// there.
/// The engine does the same where a hand-over leaves no text.
///
/// Giving back a blank of `\s*` leaves a blank where `#` or the line's end
/// must stand.
pub(in crate::haml) fn block_opener(s: &[u8], p: usize) -> Option<usize> {
    let after = match s.get(p) {
        Some(b',' | b'<' | b'|' | b'{') => p + 1,
        Some(b'd') if s.get(p + 1) == Some(&b'o') => p + 2,
        _ => return None,
    };
    let at = blanks_end(s, after);
    (at == s.len() || s.get(at) == Some(&b'#')).then_some(s.len())
}

/// `#.*$`: a comment to the line's end.
pub(in crate::haml) fn comment(s: &[u8], p: usize) -> Option<usize> {
    (s.get(p) == Some(&b'#')).then_some(s.len())
}

/// `(?<!#)\{(?=.*(,|(do)|[{|}]|(#.*)|\R)\s*)`: a `{` that opens a tag's
/// attribute hash.
///
/// `.*` can stop anywhere, so the lookahead holds where one of its branches
/// starts anywhere after the `{`.
/// `\R` takes the line feed, so the lookahead holds on every line that has
/// one.
pub(in crate::haml) fn attributes_begin(s: &[u8], p: usize, nl: bool) -> bool {
    if s.get(p) != Some(&b'{') || (p > 0 && s[p - 1] == b'#') {
        return false;
    }
    let rest = &s[p + 1..];
    nl || rest
        .iter()
        .any(|&b| matches!(b, b',' | b'{' | b'|' | b'}' | b'#' | b'\r' | 0x0b | 0x0c))
        || rest.windows(2).any(|pair| pair == b"do")
        || rest.windows(2).any(|pair| pair == [0xc2, 0x85])
        || rest
            .windows(3)
            .any(|three| three == [0xe2, 0x80, 0xa8] || three == [0xe2, 0x80, 0xa9])
}

/// `\s*}(?!\s*,)(?!\s*\|)(?!#\{.*})`: the `}` that closes an attribute
/// hash, with the blanks before it.
///
/// Giving back a blank of `\s*` leaves a blank where `}` must stand.
pub(in crate::haml) fn attributes_end_at(s: &[u8], p: usize) -> Option<usize> {
    let close = blanks_end(s, p);
    if s.get(close) != Some(&b'}') {
        return None;
    }
    let after = close + 1;
    let next = blanks_end(s, after);
    if matches!(s.get(next), Some(b',' | b'|')) {
        return None;
    }
    if s[after..].starts_with(b"#{") && s[after + 2..].contains(&b'}') {
        return None;
    }
    Some(after)
}

/// Whether an attribute hash closes at `p`.
pub(in crate::haml) fn attributes_end(s: &[u8], p: usize) -> bool {
    attributes_end_at(s, p).is_some()
}

/// `\[(?=.+([],\[|]|(#.*))\s*)`: a `[` that opens a tag's object.
/// One character at least, then a `]`, `,`, `[`, `|` or `#`, must follow it.
pub(in crate::haml) fn object_begin(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'[')
        && s.get(p + 2..).is_some_and(|rest| {
            rest.iter()
                .any(|&b| matches!(b, b']' | b',' | b'[' | b'|' | b'#'))
        })
}

/// `\s*](?!.*(?!#\[)])`: the line's last `]`, with the blanks before it.
/// The lookahead fails where any `]` follows, since a `]` is never the `#`
/// of `#[`.
pub(in crate::haml) fn object_end_at(s: &[u8], p: usize) -> Option<usize> {
    let close = blanks_end(s, p);
    (s.get(close) == Some(&b']') && !s[close + 1..].contains(&b']')).then_some(close + 1)
}

/// Whether a tag's object closes at `p`.
pub(in crate::haml) fn object_end(s: &[u8], p: usize) -> bool {
    object_end_at(s, p).is_some()
}
