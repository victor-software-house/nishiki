//! The patterns of ERB's injection, as plain functions of the line.
//! The injection opens Ruby code anywhere in an ERB document except in a
//! comment or in ERB itself.
//! Plain functions let the CSS and JavaScript lexers halt at them too.
//!
//! The grammar's patterns, in order:
//!
//! 1. `^(\s*)(?=<%+#(?![^%]*%>))`: the blanks before a comment that the line
//!    does not close;
//! 2. `^(\s*)(?=<%(?![^%]*%>))`: the blanks before code that the line does not
//!    close;
//! 3. `<%+#`: a comment, up to `%>`;
//! 4. `<%+(?!>)[-=]?(?![^%]*%>)`: code that the line does not close, named
//!    `meta.embedded.block.erb`;
//! 5. `<%+(?!>)[-=]?`: code, named `meta.embedded.line.erb`.
//!
//! The code of 4 and 5 runs up to `(-?%)>`.
//! `[^%]*%>` cannot step over a `%`.
//! It therefore matches where the first `%` ahead is followed by `>`.

/// One of the injection's patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Erb {
    /// The blanks at the start of a line, before a comment it does not close.
    CommentWrap,
    /// The blanks at the start of a line, before code it does not close.
    TagWrap,
    /// `<%#`: a comment.
    Comment,
    /// Code that the line does not close.
    Block,
    /// Code.
    Line,
}

/// The first of the injection's patterns that matches at `p` of `s`, and
/// where its begin match ends.
/// `s` ends where the text a pattern may read ends.
pub(super) fn at(s: &[u8], p: usize) -> Option<(Erb, usize)> {
    if p == 0 {
        let q = blanks_end(s, 0);
        if comment_begin(s, q).is_some_and(|r| !closes(s, r)) {
            return Some((Erb::CommentWrap, q));
        }
        if s.get(q..).is_some_and(|rest| rest.starts_with(b"<%")) && !closes(s, q + 2) {
            return Some((Erb::TagWrap, q));
        }
    }
    if let Some(end) = comment_begin(s, p) {
        return Some((Erb::Comment, end));
    }
    code_begin(s, p)
}

/// Whether one of the injection's patterns matches at `p` of `s`: a halt
/// for the lexers of the CSS and JavaScript that ERB holds.
pub(super) fn begins(s: &[u8], p: usize) -> bool {
    (p == 0 || s.get(p) == Some(&b'<')) && at(s, p).is_some()
}

/// The end of `<%+#` at `p`: `<`, a run of `%` and `#`, since fewer `%`
/// would leave a `%` where the `#` must be.
pub(super) fn comment_begin(s: &[u8], p: usize) -> Option<usize> {
    let run = percents(s, p)?;
    (s.get(p + 1 + run) == Some(&b'#')).then_some(p + 2 + run)
}

/// The begin of code at `p`, `<%+(?!>)[-=]?`, tried as Oniguruma tries it:
/// - the most `%` first;
/// - the `-` or `=` before none;
/// - the block form, whose lookahead must find no `%>`, before the line form.
pub(super) fn code_begin(s: &[u8], p: usize) -> Option<(Erb, usize)> {
    let run = percents(s, p)?;
    let ends = |k: usize| {
        let after = p + 1 + k;
        (s.get(after) != Some(&b'>')).then(|| {
            let sign = usize::from(matches!(s.get(after), Some(b'-' | b'=')));
            (after, after + sign)
        })
    };
    for k in (1..=run).rev() {
        let Some((bare, signed)) = ends(k) else {
            continue;
        };
        if signed > bare && !closes(s, signed) {
            return Some((Erb::Block, signed));
        }
        if !closes(s, bare) {
            return Some((Erb::Block, bare));
        }
    }
    (1..=run)
        .rev()
        .find_map(ends)
        .map(|(_, signed)| (Erb::Line, signed))
}

/// The number of `%` after a `<` at `p`, when there is at least one.
fn percents(s: &[u8], p: usize) -> Option<usize> {
    if s.get(p) != Some(&b'<') {
        return None;
    }
    let run = s
        .get(p + 1..)
        .unwrap_or_default()
        .iter()
        .take_while(|&&b| b == b'%')
        .count();
    (run > 0).then_some(run)
}

/// `[^%]*%>` at `p`: whether the first `%` from `p` is followed by `>`.
fn closes(s: &[u8], p: usize) -> bool {
    let rest = s.get(p..).unwrap_or_default();
    memchr::memchr(b'%', rest).is_some_and(|at| rest.get(at + 1) == Some(&b'>'))
}

/// The end of the run of `\s` characters from `p`.
fn blanks_end(s: &[u8], mut p: usize) -> usize {
    loop {
        let n = match s.get(p) {
            Some(&b) if b < 0x80 => usize::from(crate::classes::is_ascii_space(b)),
            Some(_) => crate::space::len(&s[p..]),
            None => 0,
        };
        if n == 0 {
            return p;
        }
        p += n;
    }
}

/// The end of the code's `(-?%)>` at `p`, when it matches there: `-%>` or
/// `%>`.
pub(super) fn code_end(s: &[u8], p: usize) -> Option<usize> {
    let rest = s.get(p..).unwrap_or_default();
    if rest.starts_with(b"-%>") {
        Some(p + 3)
    } else if rest.starts_with(b"%>") {
        Some(p + 2)
    } else {
        None
    }
}

/// Where `.*?(?=-?%>)` ends after a `#` at `p`: the first place from `p + 1`
/// where `-%>` or `%>` starts.
pub(super) fn hash_comment_end(s: &[u8], p: usize) -> Option<usize> {
    let from = p + 1;
    let rest = s.get(from..).unwrap_or_default();
    let percent = memchr::memmem::find(rest, b"%>")? + from;
    Some(if percent > from && s[percent - 1] == b'-' {
        percent - 1
    } else {
        percent
    })
}

/// The halt of the Ruby inside code: its end, `(-?%)>`, or the comment
/// pattern `(#).*?(?=-?%>)`.
/// The code tries the comment pattern before Ruby's patterns.
pub(super) fn ruby_halt(s: &[u8], p: usize) -> bool {
    code_end(s, p).is_some() || (s.get(p) == Some(&b'#') && hash_comment_end(s, p).is_some())
}

/// The state of the Ruby of ERB code whose text sits on `base`.
#[cfg(feature = "lang-erb")]
pub(super) fn ruby_start(arena: &mut crate::arena::Arena, base: u32) -> u32 {
    crate::ruby::start_embedded(arena, base)
}

/// Lexes the Ruby of ERB code from `state` up to `halt`.
#[cfg(feature = "lang-erb")]
pub(super) fn lex_ruby(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
    halt: &dyn crate::language::Halt,
) -> (crate::limits::Finished, Option<crate::language::Stopped>) {
    crate::ruby::lex_until(cx, line, state, halt, None)
}

/// Without the ERB feature no ERB document is lexed, so no code is open.
#[cfg(not(feature = "lang-erb"))]
pub(super) fn ruby_start(_arena: &mut crate::arena::Arena, _base: u32) -> u32 {
    unreachable!("ERB code opens only in an ERB document")
}

/// Without the ERB feature no ERB document is lexed, so no code is open.
#[cfg(not(feature = "lang-erb"))]
pub(super) fn lex_ruby(
    _cx: &mut crate::language::Context<'_>,
    _line: crate::language::Line<'_>,
    _state: u32,
    _halt: &dyn crate::language::Halt,
) -> (crate::limits::Finished, Option<crate::language::Stopped>) {
    unreachable!("ERB code opens only in an ERB document")
}
