//! The inline rules: emphasis, code spans, escapes, links and images.
//!
//! Each function matches one rule of the grammar at `p`, returns where the
//! match ends and fills in the capture groups the grammar gives scopes to.
//! The regular expressions backtrack, and where that changes a result the
//! functions try the same alternatives in the same order.

use super::caps::Caps;
use super::limits::{MAX_BRACKET_DEPTH, MAX_PAREN_DEPTH};
use super::text::Cx;
use super::ticks::{CodeSpan, Ticks};

/// `&(?!([0-9A-Za-z]+|#[0-9]+|#x\h+);)`: an `&` that starts no entity.
pub(crate) fn ampersand(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if cx.get(p) != Some(b'&') {
        return None;
    }
    let i = p + 1;
    let named = cx.skip(i, |b| b.is_ascii_alphanumeric());
    let entity = (named > i && cx.get(named) == Some(b';'))
        || (cx.get(i) == Some(b'#') && {
            let digits = cx.skip(i + 1, |b| b.is_ascii_digit());
            let hex = if matches!(cx.get(i + 1), Some(b'x')) {
                cx.skip(i + 2, |b| b.is_ascii_hexdigit())
            } else {
                0
            };
            (digits > i + 1 && cx.get(digits) == Some(b';'))
                || (hex > i + 2 && cx.get(hex) == Some(b';'))
        });
    (!entity).then_some(p + 1)
}

/// `<(?![!$/?A-Za-z])`: a `<` that starts no tag.
pub(crate) fn bracket(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if cx.get(p) != Some(b'<') {
        return None;
    }
    let tag = cx
        .get(p + 1)
        .is_some_and(|b| matches!(b, b'!' | b'$' | b'/' | b'?') || b.is_ascii_alphabetic());
    (!tag).then_some(p + 1)
}

/// The bytes that follow a backslash to make an escape: the ASCII punctuation
/// `- ] ! # ( ) * + . > [ \ _ { }` and the backtick.
fn is_escapable(b: u8) -> bool {
    matches!(
        b,
        b'-' | b']'
            | b'!'
            | b'#'
            | b'('
            | b')'
            | b'*'
            | b'+'
            | b'.'
            | b'>'
            | b'['
            | b'\\'
            | b'_'
            | b'`'
            | b'{'
            | b'}'
    )
}

/// A backslash and the byte it escapes.
pub(crate) fn escape(cx: &Cx<'_>, p: usize) -> Option<usize> {
    (cx.get(p) == Some(b'\\') && cx.get(p + 1).is_some_and(is_escapable)).then_some(p + 2)
}

/// What a delimiter run opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Emphasis {
    Bold,
    Italic,
}

/// The opening delimiter of `emphasis` at `p`, as the grammar's `open` group
/// matches it: `**` or `__` for bold, `*` or `_` for italic.
///
/// A `*` opens when a word character follows or none precedes; a `_` only
/// when none precedes.
fn delimiter(cx: &Cx<'_>, p: usize, emphasis: Emphasis) -> Option<(u8, usize)> {
    let mark = cx.get(p).filter(|b| matches!(b, b'*' | b'_'))?;
    let len = match emphasis {
        Emphasis::Bold => 2,
        Emphasis::Italic => 1,
    };
    if (0..len).any(|k| cx.get(p + k) != Some(mark)) {
        return None;
    }
    let ok = match mark {
        b'*' => cx.word_at(p + len) || !cx.word_before(p),
        _ => !cx.word_before(p),
    };
    ok.then_some((mark, len))
}

/// The bold and italic begin rules: the opening delimiter, when a closing one
/// follows later in the line.
///
/// The grammar looks ahead over everything that cannot close the span: tags,
/// code spans, escapes and links count as units, and anything else is one
/// character, except that italic skips a doubled delimiter whole.
pub(crate) fn emphasis_open(
    cx: &Cx<'_>,
    ticks: &mut Ticks,
    p: usize,
    emphasis: Emphasis,
    caps: &mut Caps,
) -> Option<usize> {
    let (mark, len) = delimiter(cx, p, emphasis)?;
    let start = p + len;
    if !cx.non_space_at(start) {
        return None;
    }
    let mut q = start;
    let mut atoms = 0;
    loop {
        let before = q;
        q = emphasis_atom(cx, ticks, q, mark, len, emphasis).unwrap_or(q);
        if q == before {
            break;
        }
        atoms += 1;
        cx.spend(q - before);
        if cx.over() {
            return None;
        }
    }
    let closes = atoms > 0
        && cx.non_space_before(q)
        && (0..len).all(|k| cx.get(q + k) == Some(mark))
        && (mark == b'*' || !cx.word_at(q + len));
    if !closes {
        return None;
    }
    caps.set(1, p, start);
    Some(start)
}

/// One step of the look-ahead: the end of the unit at `q`, or none when the
/// look-ahead stops there.
fn emphasis_atom(
    cx: &Cx<'_>,
    ticks: &mut Ticks,
    q: usize,
    mark: u8,
    len: usize,
    emphasis: Emphasis,
) -> Option<usize> {
    let b = cx.get(q)?;
    match b {
        b'<' => {
            // `<[^>]*+>`
            let rest = cx.line.get(q + 1..cx.end.min(cx.line.len()))?;
            if let Some(gt) = memchr::memchr(b'>', rest) {
                return Some(q + 1 + gt + 1);
            }
        }
        b'`' => {
            if let Some(span) = code_span(cx, ticks, q) {
                return Some(span.end);
            }
        }
        b'\\' => {
            return Some(if cx.get(q + 1).is_some_and(is_escapable) {
                q + 2
            } else {
                q + 1
            });
        }
        b'[' => {
            if let Some(end) = bracketed_link(cx, q) {
                return Some(end);
            }
        }
        _ => {}
    }
    if emphasis == Emphasis::Italic && cx.starts(q, &[mark, mark]) {
        return Some(q + 2);
    }
    // `(?!(?<=\S)\k<open>).`
    let closes_here = cx.non_space_before(q) && (0..len).all(|k| cx.get(q + k) == Some(mark));
    (b != b'\n' && !closes_here).then_some(q + cx.char_len(q))
}

/// `(?<=\S)(\1)` at `p`: the closing delimiter of bold.
pub(crate) fn bold_close(cx: &Cx<'_>, p: usize, mark: u8, caps: &mut Caps) -> Option<usize> {
    if !cx.non_space_before(p) || !cx.starts(p, &[mark, mark]) {
        return None;
    }
    caps.set(1, p, p + 2);
    Some(p + 2)
}

/// `(?<=\S)(\1)((?!\1)|(?=\1\1))` at `p`: the closing delimiter of italic,
/// not followed by one more delimiter, or followed by two.
pub(crate) fn italic_close(cx: &Cx<'_>, p: usize, mark: u8, caps: &mut Caps) -> Option<usize> {
    if !cx.non_space_before(p) || cx.get(p) != Some(mark) {
        return None;
    }
    if cx.get(p + 1) == Some(mark) && !cx.starts(p + 1, &[mark, mark]) {
        return None;
    }
    caps.set(1, p, p + 1);
    Some(p + 1)
}

/// A run of backticks, then the first later run of the same length. When none
/// of that length follows, the opening run gives back backticks one at a time,
/// and the ones given back become text.
/// The closing run comes from the line's backtick runs.
#[inline(never)]
pub(crate) fn code_span(cx: &Cx<'_>, ticks: &mut Ticks, p: usize) -> Option<CodeSpan> {
    if cx.get(p) != Some(b'`') {
        return None;
    }
    let (span, steps) = ticks.code_span(cx.line, p, cx.end.min(cx.line.len()));
    cx.spend(steps);
    span
}

/// The raw rule: a code span, its backtick runs and its text.
pub(crate) fn raw(cx: &Cx<'_>, ticks: &mut Ticks, p: usize, caps: &mut Caps) -> Option<usize> {
    let span = code_span(cx, ticks, p)?;
    caps.set(1, p, p + span.open_len);
    if span.close > p + span.open_len {
        caps.set(2, p + span.open_len, span.close);
    }
    caps.set(3, span.close, span.end);
    Some(span.end)
}

/// `(?<!\\)(~{2,})(?!(?<=\w~~)_)((?:[^~]|(?!(?<![\\~])\1(?!~))~)*+)(\1)
/// (?!(?<=_\1)\w)`: a run of at least two `~`, the first later run of the
/// same length, and what lies between.
pub(crate) fn strikethrough(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'~') || cx.before(p) == Some(b'\\') {
        return None;
    }
    let run = cx.skip(p, |b| b == b'~') - p;
    let line_end = cx.end.min(cx.line.len());
    for open_len in (2..=run).rev() {
        let open_end = p + open_len;
        // `(?!(?<=\w~~)_)`
        if cx.get(open_end) == Some(b'_')
            && open_end >= 3
            && cx.get(open_end - 1) == Some(b'~')
            && cx.get(open_end - 2) == Some(b'~')
            && cx.word_before(open_end - 2)
        {
            continue;
        }
        cx.spend(line_end.saturating_sub(p + run));
        if cx.over() {
            return None;
        }
        let mut i = p + run;
        while let Some(rest) = cx.line.get(i..line_end) {
            let Some(tilde) = memchr::memchr(b'~', rest) else {
                break;
            };
            let start = i + tilde;
            let stop = cx.skip(start, |b| b == b'~').min(line_end);
            i = stop;
            let escaped = start > 0 && cx.get(start - 1) == Some(b'\\');
            if stop - start != open_len || escaped {
                continue;
            }
            // `(?!(?<=_\1)\w)`
            if cx.before(start) == Some(b'_') && cx.word_at(stop) {
                break;
            }
            caps.set(1, p, open_end);
            if start > open_end {
                caps.set(2, open_end, start);
            }
            caps.set(3, start, stop);
            return Some(stop);
        }
    }
    None
}

/// `(?<square>[^]\[\\]|\\.|\[\g<square>*+])*+`: the end of the label that
/// starts at `i`, where the closing `]` is. Brackets inside nest, and a
/// backslash escapes the byte after it.
fn label_end(cx: &Cx<'_>, mut i: usize, depth: usize) -> usize {
    while let Some(b) = cx.get(i) {
        match b {
            b']' => break,
            b'[' => {
                if depth >= MAX_BRACKET_DEPTH {
                    break;
                }
                let inner = label_end(cx, i + 1, depth + 1);
                if cx.get(inner) == Some(b']') {
                    i = inner + 1;
                } else {
                    break;
                }
            }
            b'\\' => {
                if cx.get(i + 1).is_some_and(|n| n != b'\n') {
                    i += 2;
                } else {
                    break;
                }
            }
            _ => i += 1,
        }
    }
    i
}

/// `((?<url>(?>[^()\s]+)|\(\g<url>*\))*)`: the end of the destination at `i`.
fn destination_end(cx: &Cx<'_>, mut i: usize, depth: usize) -> usize {
    while let Some(b) = cx.get(i) {
        if cx.space_at(i) > 0 {
            break;
        }
        match b {
            b'(' => {
                if depth >= MAX_PAREN_DEPTH {
                    break;
                }
                let inner = destination_end(cx, i + 1, depth + 1);
                if cx.get(inner) == Some(b')') {
                    i = inner + 1;
                } else {
                    break;
                }
            }
            b')' => break,
            _ => i += 1,
        }
    }
    i
}

/// Whether the label that opens at `p` (a `[`) closes, and where.
fn label(cx: &Cx<'_>, p: usize) -> Option<usize> {
    let end = label_end(cx, p + 1, 0);
    cx.spend(end - p);
    (cx.get(end) == Some(b']')).then_some(end)
}

/// The inline link or reference that the look-ahead of emphasis skips:
/// `\[label](( ?\[[^]]*+])|(\([\t ]*+<?(.*?)>?[\t
/// ]*+(("|')(.*?)\k<title>)?\)))`.
fn bracketed_link(cx: &Cx<'_>, p: usize) -> Option<usize> {
    let close = label(cx, p)?;
    let after = close + 1;
    // ` ?\[[^]]*+]`
    let bracket = if cx.get(after) == Some(b' ') {
        after + 1
    } else {
        after
    };
    if cx.get(bracket) == Some(b'[') {
        let rest = cx.line.get(bracket + 1..cx.end.min(cx.line.len()))?;
        if let Some(i) = memchr::memchr(b']', rest) {
            return Some(bracket + 1 + i + 1);
        }
    }
    if cx.get(after) != Some(b'(') {
        return None;
    }
    let start = cx.skip(after + 1, |b| matches!(b, b' ' | b'\t'));
    let angle = cx.get(start) == Some(b'<');
    angle_tail(cx, start + usize::from(angle))
        .or_else(|| angle.then(|| angle_tail(cx, start)).flatten())
}

/// `(.*?)>?[\t ]*+(("|')(.*?)\k<title>)?\)` from `i`: the end of the first
/// place the link can close.
fn angle_tail(cx: &Cx<'_>, i: usize) -> Option<usize> {
    let mut at = i;
    while let Some(b) = cx.get(at) {
        cx.spend(1);
        if b == b'\n' || cx.over() {
            return None;
        }
        let mut j = at;
        if cx.get(j) == Some(b'>') {
            j += 1;
        }
        j = cx.skip(j, |c| matches!(c, b' ' | b'\t'));
        match cx.get(j) {
            Some(b')') => return Some(j + 1),
            Some(quote @ (b'"' | b'\'')) => {
                let mut k = j + 1;
                while let Some(c) = cx.get(k) {
                    if c == b'\n' {
                        break;
                    }
                    if c == quote && cx.get(k + 1) == Some(b')') {
                        return Some(k + 2);
                    }
                    k += 1;
                }
                cx.spend(k - j);
                if cx.over() {
                    return None;
                }
            }
            _ => {}
        }
        at += 1;
    }
    None
}

/// What closes a link's title: `)` after `(`, or the quote that opened it.
fn title_close(open: u8) -> u8 {
    if open == b'(' { b')' } else { open }
}

/// Which grammar rule a destination belongs to. The link rule takes any
/// title that holds no closing byte; the image rule takes the first one that
/// the closing parenthesis follows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Destination {
    Link,
    Image,
}

/// `(\()[\t ]*((<)((?:\\[<>]|[^\n<>])*)(>)|((?<url>…)*))[\t ]*(title)?\s*(\))`
/// from the opening parenthesis at `open`, filling groups 5 to 21.
fn destination(cx: &Cx<'_>, open: usize, kind: Destination, caps: &mut Caps) -> Option<usize> {
    caps.set(5, open, open + 1);
    let start = cx.skip(open + 1, |b| matches!(b, b' ' | b'\t'));
    if cx.get(start) == Some(b'<') {
        let mut i = start + 1;
        while let Some(b) = cx.get(i) {
            match b {
                b'\\' if matches!(cx.get(i + 1), Some(b'<' | b'>')) => i += 2,
                b'\n' | b'<' | b'>' => break,
                _ => i += 1,
            }
        }
        if cx.get(i) == Some(b'>') {
            let mut angle = *caps;
            angle.set(7, start, start + 1);
            if i > start + 1 {
                angle.set(8, start + 1, i);
            }
            angle.set(9, i, i + 1);
            if let Some(end) = destination_tail(cx, i + 1, kind, &mut angle) {
                *caps = angle;
                return Some(end);
            }
        }
    }
    let url_end = destination_end(cx, start, 0);
    cx.spend(url_end - start);
    if url_end > start {
        caps.set(10, start, url_end);
    }
    destination_tail(cx, url_end, kind, caps)
}

/// `[\t ]*(title)?\s*(\))` after the destination, ending at the parenthesis.
fn destination_tail(cx: &Cx<'_>, p: usize, kind: Destination, caps: &mut Caps) -> Option<usize> {
    let at = cx.skip(p, |b| matches!(b, b' ' | b'\t'));
    let mut close = cx.skip_space(at);
    if let Some(open @ (b'(' | b'"' | b'\'')) = cx.get(at) {
        let first = match open {
            b'(' => 12,
            b'"' => 15,
            _ => 18,
        };
        let shut = title_close(open);
        let mut i = at + 1;
        // The link takes `[^()]*`, `[^"]*` or `[^']*`; the image takes `.+?`
        // up to the first closing byte that the final parenthesis follows.
        while let Some(b) = cx.get(i) {
            let stop = match kind {
                Destination::Link => b == shut || (open == b'(' && b == b'('),
                Destination::Image => b == b'\n' || (b == shut && i > at + 1 && fits(cx, i + 1)),
            };
            if stop {
                break;
            }
            i += 1;
        }
        cx.spend(i - at);
        if cx.over() {
            return None;
        }
        let closes =
            cx.get(i) == Some(shut) && (kind == Destination::Link || i > at + 1) && fits(cx, i + 1);
        if closes {
            caps.set(first, at, i + 1);
            caps.set(first + 1, at, at + 1);
            caps.set(first + 2, i, i + 1);
            close = cx.skip_space(i + 1);
        } else {
            return None;
        }
    }
    if cx.get(close) == Some(b')') {
        caps.set(21, close, close + 1);
        Some(close + 1)
    } else {
        None
    }
}

/// `\s*\)` at `p`.
fn fits(cx: &Cx<'_>, p: usize) -> bool {
    cx.get(cx.skip_space(p)) == Some(b')')
}

/// `(!?\[)(label)(])` and then the `(` of a destination.
fn inline_link(
    cx: &Cx<'_>,
    p: usize,
    bang: bool,
    kind: Destination,
    caps: &mut Caps,
) -> Option<usize> {
    let open = p + usize::from(bang);
    if bang && cx.get(p) != Some(b'!') {
        return None;
    }
    if cx.get(open) != Some(b'[') {
        return None;
    }
    let close = label(cx, open)?;
    caps.set(1, p, open + 1);
    if close > open + 1 {
        caps.set(2, open + 1, close);
    }
    caps.set(4, close, close + 1);
    if cx.get(close + 1) != Some(b'(') {
        return None;
    }
    destination(cx, close + 1, kind, caps)
}

/// `(!\[)(label)(])(\()…(\))`
pub(crate) fn image_inline(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    inline_link(cx, p, true, Destination::Image, caps)
}

/// `(\[)(label)(])(\()…(\))`
pub(crate) fn link_inline(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    inline_link(cx, p, false, Destination::Link, caps)
}

/// `(!\[)(label)(]) ?(\[)(.*?)(])`
pub(crate) fn image_reference(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.starts(p, b"![") {
        return None;
    }
    let close = label(cx, p + 1)?;
    let mut i = close + 1;
    if cx.get(i) == Some(b' ') {
        i += 1;
    }
    if cx.get(i) != Some(b'[') {
        return None;
    }
    let end = (i + 1..cx.end)
        .take_while(|&k| cx.get(k) != Some(b'\n'))
        .find(|&k| cx.get(k) == Some(b']'))?;
    caps.set(1, p, p + 2);
    if close > p + 2 {
        caps.set(2, p + 2, close);
    }
    caps.set(4, close, close + 1);
    caps.set(5, i, i + 1);
    if end > i + 1 {
        caps.set(6, i + 1, end);
    }
    caps.set(7, end, end + 1);
    Some(end + 1)
}

/// `(<)((?:mailto:)?[!#-'*+\--9=?A-Z^-~]+@[-0-9A-Za-z]+(?:\.[-0-9A-Za-z]+)*
/// )(>)`
pub(crate) fn link_email(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'<') {
        return None;
    }
    let mut i = p + 1;
    if cx.starts(i, b"mailto:") {
        i += 7;
    }
    let is_local = |b: u8| matches!(b, b'!' | b'#'..=b'\'' | b'*' | b'+' | b'-'..=b'9' | b'=' | b'?' | b'A'..=b'Z' | b'^'..=b'~');
    let local = cx.skip(i, is_local);
    if local == i || cx.get(local) != Some(b'@') {
        return None;
    }
    let is_label = |b: u8| b == b'-' || b.is_ascii_alphanumeric();
    let mut end = cx.skip(local + 1, is_label);
    if end == local + 1 {
        return None;
    }
    while cx.get(end) == Some(b'.') {
        let next = cx.skip(end + 1, is_label);
        if next == end + 1 {
            break;
        }
        end = next;
    }
    if cx.get(end) != Some(b'>') {
        return None;
    }
    caps.set(1, p, p + 1);
    caps.set(2, p + 1, end);
    Some(end + 1)
}

/// `(<)((?:https?|ftp)://.*?)(>)`
pub(crate) fn link_inet(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'<') {
        return None;
    }
    let i = p + 1;
    let scheme = if cx.starts(i, b"https") {
        5
    } else if cx.starts(i, b"http") {
        4
    } else if cx.starts(i, b"ftp") {
        3
    } else {
        return None;
    };
    if !cx.starts(i + scheme, b"://") {
        return None;
    }
    let end = (i + scheme + 3..cx.end)
        .take_while(|&k| cx.get(k) != Some(b'\n'))
        .find(|&k| cx.get(k) == Some(b'>'))?;
    caps.set(1, p, p + 1);
    caps.set(2, p + 1, end);
    caps.set(3, end, end + 1);
    Some(end + 1)
}

/// `(?<![]\\])`: no `]` or backslash before `p`.
fn not_after_bracket(cx: &Cx<'_>, p: usize) -> bool {
    !matches!(cx.before(p), Some(b']' | b'\\'))
}

/// `(?<![]\\])(\[)(label)(])(\[)([^]]*+)(])`
pub(crate) fn link_reference(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'[') || !not_after_bracket(cx, p) {
        return None;
    }
    let close = label(cx, p)?;
    if cx.get(close + 1) != Some(b'[') {
        return None;
    }
    let end = (close + 2..cx.end).find(|&k| cx.get(k) == Some(b']'))?;
    caps.set(1, p, p + 1);
    if close > p + 1 {
        caps.set(2, p + 1, close);
    }
    caps.set(4, close, close + 1);
    caps.set(5, close + 1, close + 2);
    if end > close + 2 {
        caps.set(6, close + 2, end);
    }
    caps.set(7, end, end + 1);
    Some(end + 1)
}

/// `(?<![]\\])(\[)(label)(]) ?(\[)(])`
pub(crate) fn link_reference_literal(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'[') || !not_after_bracket(cx, p) {
        return None;
    }
    let close = label(cx, p)?;
    let mut i = close + 1;
    if cx.get(i) == Some(b' ') {
        i += 1;
    }
    if cx.get(i) != Some(b'[') || cx.get(i + 1) != Some(b']') {
        return None;
    }
    caps.set(1, p, p + 1);
    if close > p + 1 {
        caps.set(2, p + 1, close);
    }
    caps.set(4, close, close + 1);
    caps.set(5, i, i + 1);
    caps.set(6, i + 1, i + 2);
    Some(i + 2)
}

/// `(?<![]\\])(\[)((?:[^]\[\\\s]|\\[]\[])+?)((?<!\\)])`
pub(crate) fn link_reference_shortcut(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.get(p) != Some(b'[') || !not_after_bracket(cx, p) {
        return None;
    }
    let start = p + 1;
    let mut i = start;
    loop {
        if i > start && cx.get(i) == Some(b']') && cx.before(i) != Some(b'\\') {
            caps.set(1, p, p + 1);
            caps.set(2, start, i);
            caps.set(3, i, i + 1);
            return Some(i + 1);
        }
        if cx.space_at(i) > 0 {
            return None;
        }
        match cx.get(i)? {
            b'\\' if matches!(cx.get(i + 1), Some(b'[' | b']')) => i += 2,
            b']' | b'[' | b'\\' => return None,
            _ => i += 1,
        }
    }
}
