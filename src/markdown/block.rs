//! The block rules: each function matches one rule of the grammar at `p` and
//! returns where the match ends, filling in the capture groups the grammar
//! gives scopes to.

use super::caps::Caps;
use super::fence;
use super::state::{Fence, Lang};
use super::text::Cx;

/// `(^|\G) {0,3}([-*_])( {0,2}\2){2,}[\t ]*$\n?`
pub(crate) fn separator(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let mut i = spaces(cx, p, 3);
    let mark = cx.get(i).filter(|b| matches!(b, b'-' | b'*' | b'_'))?;
    i += 1;
    let mut marks = 1;
    loop {
        let j = spaces(cx, i, 2);
        if cx.get(j) == Some(mark) {
            i = j + 1;
            marks += 1;
        } else {
            break;
        }
    }
    if marks < 3 {
        return None;
    }
    i = cx.skip(i, |b| matches!(b, b' ' | b'\t'));
    match cx.get(i) {
        None => Some(i),
        Some(b'\n') => Some(i + 1),
        Some(_) => None,
    }
}

/// The offset after up to `max` spaces at `p`.
fn spaces(cx: &Cx<'_>, p: usize, max: usize) -> usize {
    let mut i = p;
    while i < p + max && cx.get(i) == Some(b' ') {
        i += 1;
    }
    i
}

/// `(?:^|\G) {0,3}(#{1,6}\s+(.*?)(\s+#{1,6})?\s*)$`: group 1 holds the
/// heading, which [`heading_part`] splits up.
pub(crate) fn heading(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let start = spaces(cx, p, 3);
    let hashes = cx.skip(start, |b| b == b'#') - start;
    if !(1..=6).contains(&hashes) || cx.space_at(start + hashes) == 0 {
        return None;
    }
    caps.set(1, start, cx.end);
    Some(cx.end)
}

/// `(#{N})\s+(.*?)(?:\s+(#+))?\s*$`, the rule for a level-`level` heading,
/// matched inside the first group of [`heading`].
pub(crate) fn heading_part(cx: &Cx<'_>, p: usize, level: usize, caps: &mut Caps) -> Option<usize> {
    if (0..level).any(|k| cx.get(p + k) != Some(b'#')) || cx.space_at(p + level) == 0 {
        return None;
    }
    caps.set(1, p, p + level);
    let text_start = cx.skip_space(p + level);
    // The text ends where only blanks, or blanks and a closing run of `#`
    // that follows a blank, are left.
    let mut text_end = cx.end;
    while text_end > text_start && cx.space_before(text_end) > 0 {
        text_end -= cx.space_before(text_end);
    }
    let mut run_start = text_end;
    while run_start > text_start && cx.get(run_start - 1) == Some(b'#') {
        run_start -= 1;
    }
    if run_start < text_end && run_start > text_start && cx.space_before(run_start) > 0 {
        caps.set(3, run_start, text_end);
        text_end = run_start;
        while text_end > text_start && cx.space_before(text_end) > 0 {
            text_end -= cx.space_before(text_end);
        }
    }
    if text_end > text_start {
        caps.set(2, text_start, text_end);
    }
    Some(cx.end)
}

/// `^(={3,})(?=[\t ]*$\n?)` and the `-` form: a setext underline.
pub(crate) fn setext(cx: &Cx<'_>, p: usize, mark: u8) -> Option<usize> {
    if p != 0 {
        return None;
    }
    let end = cx.skip(p, |b| b == mark);
    if end - p < 3 {
        return None;
    }
    let rest = cx.skip(end, |b| matches!(b, b' ' | b'\t'));
    cx.at_eol(rest).then_some(end)
}

/// `(^|\G) {0,3}(>) ?`
pub(crate) fn quote(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = spaces(cx, p, 3);
    if cx.get(i) != Some(b'>') {
        return None;
    }
    caps.set(2, i, i + 1);
    Some(if cx.get(i + 1) == Some(b' ') {
        i + 2
    } else {
        i + 1
    })
}

/// `(^|\G)\s*(>) ?`, which keeps a quote open.
pub(crate) fn quote_while(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = cx.skip_space(p);
    if cx.get(i) != Some(b'>') {
        return None;
    }
    caps.set(2, i, i + 1);
    Some(if cx.get(i + 1) == Some(b' ') {
        i + 2
    } else {
        i + 1
    })
}

/// `(^|\G)( {0,3})([-*+])([\t ])`, a bullet list item.
pub(crate) fn bullet(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = spaces(cx, p, 3);
    if !matches!(cx.get(i), Some(b'-' | b'*' | b'+'))
        || !matches!(cx.get(i + 1), Some(b' ' | b'\t'))
    {
        return None;
    }
    caps.set(3, i, i + 1);
    Some(i + 2)
}

/// `(^|\G)( {0,3})([0-9]+[).])([\t ])`, a numbered list item.
pub(crate) fn ordered(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = spaces(cx, p, 3);
    let digits = cx.skip(i, |b| b.is_ascii_digit());
    if digits == i
        || !matches!(cx.get(digits), Some(b')' | b'.'))
        || !matches!(cx.get(digits + 1), Some(b' ' | b'\t'))
    {
        return None;
    }
    caps.set(3, i, digits + 1);
    Some(digits + 2)
}

/// `((^|\G)( {2,4}|\t))|^([\t ]*)$`, which keeps a list item open: an
/// indented line, or a blank one.
pub(crate) fn list_while(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if cx.at_start(p) {
        let n = spaces(cx, p, 4) - p;
        if n >= 2 {
            return Some(p + n);
        }
        if cx.get(p) == Some(b'\t') {
            return Some(p + 1);
        }
    }
    if p == 0 {
        let i = cx.skip(p, |b| matches!(b, b' ' | b'\t'));
        if cx.at_eol(i) {
            return Some(i);
        }
    }
    None
}

/// What opened a fence.
pub(crate) struct FenceOpen {
    pub(crate) end: usize,
    pub(crate) fence: Fence,
    /// The language to embed, or none for a fence that only holds text.
    pub(crate) lang: Option<Lang>,
}

/// The opening of a fence: leading whitespace, a run of three or more backticks
/// or tildes, and then either a language the grammar knows followed by its
/// attributes, or the rest of the line as the info string, which may not hold
/// a backtick (`(^|\G)(\s*)(``{3,}|~{3,})\s*` in the grammar).
///
/// Group 3 is the fence, 4 the language, 5 its attributes.
pub(crate) fn fence_open(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<FenceOpen> {
    if !cx.at_start(p) {
        return None;
    }
    let fence_start = cx.skip_space(p);
    let delim = cx.get(fence_start).filter(|b| matches!(b, b'`' | b'~'))?;
    let fence_end = cx.skip(fence_start, |b| b == delim);
    let len = fence_end - fence_start;
    if len < 3 {
        return None;
    }
    let indent = &cx.line[p.min(cx.line.len())..fence_start.min(cx.line.len())];
    let fence = Fence {
        delim,
        len: u8::try_from(len).unwrap_or(u8::MAX),
        indent: if indent.iter().all(|&b| b == b' ') {
            u8::try_from(indent.len()).map_or(Fence::MIXED, |n| n.min(Fence::MIXED - 1))
        } else {
            Fence::MIXED
        },
    };
    caps.set(3, fence_start, fence_end);
    let info = cx.skip_space(fence_end);
    if let Some(found) = fence::language(cx, info) {
        caps.set(4, info, found.name_end);
        caps.set(5, found.name_end, cx.end);
        return Some(FenceOpen {
            end: cx.end,
            fence,
            lang: Some(found.lang),
        });
    }
    if fence::has_backtick(cx, info) {
        return None;
    }
    if info < cx.end {
        caps.set(4, info, cx.end);
    }
    Some(FenceOpen {
        end: info,
        fence,
        lang: None,
    })
}

/// `(^|\G)(\2|\s{0,3})(\3)\s*$`, which closes the fence: the opening fence's
/// own run of characters, indented as the opening one or by at most three.
pub(crate) fn fence_close(cx: &Cx<'_>, p: usize, fence: Fence, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let start = cx.skip_space(p);
    // `\s{0,3}` counts characters, and `\2` repeats the opening indent.
    let mut width = 0;
    let mut at = p;
    while at < start {
        at += cx.space_at(at);
        width += 1;
    }
    let same_indent = width == usize::from(fence.indent)
        && (p..start).all(|i| cx.get(i) == Some(b' '))
        && fence.indent != Fence::MIXED;
    if width > 3 && !same_indent {
        return None;
    }
    let end = cx.skip(start, |b| b == fence.delim);
    if end - start != usize::from(fence.len) || !cx.only_space_after(end) {
        return None;
    }
    caps.set(3, start, end);
    Some(cx.end)
}

/// Whether the line holds nothing but a run of three or more backticks or
/// tildes, which ends the lines of an embedded language.
pub(crate) fn is_fence_line(cx: &Cx<'_>, p: usize) -> bool {
    let start = cx.skip_space(p);
    let end = cx.skip(start, |b| matches!(b, b'`' | b'~'));
    end - start >= 3 && cx.only_space_after(end)
}

/// `(^|\G)( {4}|\t)`, an indented code block line.
pub(crate) fn raw_block(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    if cx.starts(p, b"    ") {
        Some(p + 4)
    } else if cx.get(p) == Some(b'\t') {
        Some(p + 1)
    } else {
        None
    }
}

/// `(^|\G)\s*(<!--)`
pub(crate) fn comment_open(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = cx.skip_space(p);
    if !cx.starts(i, b"<!--") {
        return None;
    }
    caps.set(2, i, i + 4);
    Some(i + 4)
}

/// `-->`, which closes a comment.
pub(crate) fn comment_close(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.starts(p, b"-->") {
        caps.set(1, p, p + 3);
        Some(p + 3)
    } else {
        None
    }
}

/// `(^|\G) {0,3}(?=[^\t\n ])`, which opens a paragraph.
pub(crate) fn paragraph(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = spaces(cx, p, 3);
    cx.get(i)
        .is_some_and(|b| !matches!(b, b'\t' | b'\n' | b' '))
        .then_some(i)
}

/// `(^|\G)((?=\s*[-=]{3,}\s*$)| {4,}(?=[^\t\n ]))`, which keeps a paragraph
/// open: a setext underline, or a line indented by four spaces or more.
pub(crate) fn paragraph_while(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let start = cx.skip_space(p);
    let run = cx.skip(start, |b| matches!(b, b'-' | b'='));
    if run - start >= 3 && cx.only_space_after(run) {
        return Some(p);
    }
    let end = cx.skip(p, |b| b == b' ');
    (end - p >= 4
        && cx
            .get(end)
            .is_some_and(|b| !matches!(b, b'\t' | b'\n' | b' ')))
    .then_some(end)
}

/// `(^|\G)(\|)(?=[^|].+\|\s*$)`, which opens a table.
pub(crate) fn table(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if !cx.at_start(p) || cx.get(p) != Some(b'|') {
        return None;
    }
    cx.get(p + 1).filter(|&b| b != b'|')?;
    let mut last = cx.end;
    while last > p + 1 && cx.space_before(last) > 0 {
        last -= cx.space_before(last);
    }
    // The last non-blank byte is a `|` with at least one byte of `.+` between
    // it and the byte after the opening `|`.
    if last < p + 4 || cx.get(last - 1) != Some(b'|') {
        return None;
    }
    caps.set(2, p, p + 1);
    Some(p + 1)
}

/// `(^|\G)(?=\|)`, which keeps a table open.
pub(crate) fn table_while(cx: &Cx<'_>, p: usize) -> bool {
    cx.at_start(p) && cx.get(p) == Some(b'|')
}

/// `(?<=\|)\s*(:?-+:?)\s*(?=\|)`, the delimiter row of a table.
pub(crate) fn table_separator(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.before(p) != Some(b'|') {
        return None;
    }
    let start = cx.skip_space(p);
    let mut i = start;
    if cx.get(i) == Some(b':') {
        i += 1;
    }
    let dashes = cx.skip(i, |b| b == b'-');
    if dashes == i {
        return None;
    }
    i = dashes;
    if cx.get(i) == Some(b':') {
        i += 1;
    }
    let end = cx.skip_space(i);
    if cx.get(end) != Some(b'|') {
        return None;
    }
    caps.set(1, start, i);
    Some(end)
}

/// `(?<=\|)\s*(?=\S)((\\\||[^|])+)(?<=\S)\s*(?=\|)`, a table cell.
pub(crate) fn table_cell(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if cx.before(p) != Some(b'|') {
        return None;
    }
    let start = cx.skip_space(p);
    if !cx.non_space_at(start) {
        return None;
    }
    let mut stop = start;
    while let Some(b) = cx.get(stop) {
        match b {
            b'|' => break,
            b'\\' if cx.get(stop + 1) == Some(b'|') => stop += 2,
            _ => stop += 1,
        }
    }
    // Give back bytes until the cell ends on a non-blank that a `|` follows,
    // through blanks.
    let mut cell_end = stop;
    while cell_end > start {
        if cx.non_space_before(cell_end) {
            let bar = cx.skip_space(cell_end);
            if cx.get(bar) == Some(b'|') {
                caps.set(1, start, cell_end);
                return Some(bar);
            }
        }
        cell_end -= 1;
    }
    None
}

/// `\s*(\[)([^]]+?)(])(:)[\t ]*(?:(<)((?:\\[<>]|[^\n<>])*)(>)|(\S+?))[\t ]*
/// (?:((\().+?(\)))|((").+?("))|((').+?(')))?\s*$`, a link reference
/// definition such as `[label]: https://example.com "title"`.
pub(crate) fn link_def(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    let open = cx.skip_space(p);
    if cx.get(open) != Some(b'[') {
        return None;
    }
    let close = (open + 2..cx.end).find(|&i| cx.get(i) == Some(b']'))?;
    if (open + 1..close).any(|i| cx.get(i) == Some(b']')) || cx.get(close + 1) != Some(b':') {
        return None;
    }
    caps.set(1, open, open + 1);
    caps.set(2, open + 1, close);
    caps.set(3, close, close + 1);
    caps.set(4, close + 1, close + 2);
    let url = cx.skip(close + 2, |b| matches!(b, b' ' | b'\t'));
    cx.spend(cx.end.saturating_sub(url));
    if cx.over() {
        return None;
    }
    if cx.get(url) == Some(b'<') {
        let mut i = url + 1;
        while let Some(b) = cx.get(i) {
            match b {
                b'\\' if matches!(cx.get(i + 1), Some(b'<' | b'>')) => i += 2,
                b'\n' | b'<' | b'>' => break,
                _ => i += 1,
            }
        }
        if cx.get(i) == Some(b'>') {
            let mut with_angle = *caps;
            with_angle.set(5, url, url + 1);
            if i > url + 1 {
                with_angle.set(6, url + 1, i);
            }
            with_angle.set(7, i, i + 1);
            if link_def_tail(cx, i + 1, &mut with_angle, 9) {
                *caps = with_angle;
                return Some(cx.end);
            }
        }
    }
    // `(\S+?)`: the shortest destination that the rest can follow.
    let mut word_end = url;
    while cx.non_space_at(word_end) {
        word_end += cx.char_len(word_end);
    }
    for end in url + 1..=word_end {
        // Only the end of the word, or a title opener, can be followed by
        // what the rest of the rule accepts.
        if end != word_end && !matches!(cx.get(end), Some(b'(' | b'"' | b'\'')) {
            continue;
        }
        let mut plain = *caps;
        plain.set(8, url, end);
        if link_def_tail(cx, end, &mut plain, 9) {
            *caps = plain;
            return Some(cx.end);
        }
    }
    None
}

/// `[\t ]*(?:title)?\s*$` after a link reference definition's destination,
/// the title's groups numbered from `first`.
fn link_def_tail(cx: &Cx<'_>, p: usize, caps: &mut Caps, first: usize) -> bool {
    let at = cx.skip(p, |b| matches!(b, b' ' | b'\t'));
    let opener = matches!(cx.get(at), Some(b'(' | b'"' | b'\''));
    // A byte that is neither blank nor a title opener ends the rule at once.
    if !opener && cx.get(at).is_some_and(|b| b.is_ascii_graphic()) {
        return false;
    }
    if cx.only_space_after(at) {
        return true;
    }
    let (index, close) = match cx.get(at) {
        Some(b'(') => (first, b')'),
        Some(b'"') => (first + 3, b'"'),
        Some(b'\'') => (first + 6, b'\''),
        _ => return false,
    };
    // `.+?` and then the first closing byte that only blanks follow.
    let mut i = at + 2;
    while let Some(b) = cx.get(i) {
        if b == b'\n' {
            return false;
        }
        if b == close && cx.only_space_after(i + 1) {
            caps.set(index, at, i + 1);
            caps.set(index + 1, at, at + 1);
            caps.set(index + 2, i, i + 1);
            return true;
        }
        i += 1;
    }
    // The scan ran to the end of the line, so every start of this rule that
    // follows pays for it.
    cx.spend(i - at);
    false
}

/// `\A(?=(-{3,}))`: the start of front matter, which has no text of its own.
/// Whether the line is the first one is the lexer's to say.
pub(crate) fn front_matter_open(cx: &Cx<'_>, p: usize) -> Option<usize> {
    (p == 0 && cx.starts(0, b"---")).then_some(0)
}

/// `\A(-{3,})(.*)$`: the first line of front matter, whose dashes and the
/// text after them are groups 1 and 2.
pub(crate) fn front_matter_begin(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    if p != 0 || !cx.starts(0, b"---") {
        return None;
    }
    let dashes = cx.skip(0, |b| b == b'-');
    caps.set(1, 0, dashes);
    caps.set(2, dashes, cx.end);
    Some(cx.end)
}

/// `^(?: {0,3}\1-*[\t ]*|[\t ]*\.{3})$`, which ends front matter: the opening
/// dashes, indented by at most three spaces and followed by more dashes, or
/// three dots. The whole line is group 0.
pub(crate) fn front_matter_close(
    cx: &Cx<'_>,
    p: usize,
    dashes: usize,
    caps: &mut Caps,
) -> Option<usize> {
    if p != 0 {
        return None;
    }
    let blanks = cx.skip(0, |b| b == b' ');
    let with_dashes = blanks <= 3 && {
        let run = cx.skip(blanks, |b| b == b'-');
        run - blanks >= dashes && cx.at_eol(cx.skip(run, |b| matches!(b, b' ' | b'\t')))
    };
    let with_dots = {
        let start = cx.skip(0, |b| matches!(b, b' ' | b'\t'));
        cx.starts(start, b"...") && cx.at_eol(start + 3)
    };
    if !(with_dashes || with_dots) {
        return None;
    }
    caps.set(0, 0, cx.end);
    Some(cx.end)
}
