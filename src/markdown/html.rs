//! The rules of the grammar's `#html` blocks other than the comment, and of
//! the wrapper that emphasis opens at a tag: each function matches one rule
//! at `p`. html-derivative takes the text they hold.

use super::caps::Caps;
use super::text::Cx;
use crate::case_fold;

/// The tag names of a raw HTML block: `script`, `style` and `pre`, in any
/// case.
const RAW_TAGS: [&[u8]; 3] = [b"script", b"style", b"pre"];

/// The end of the raw tag name at `p`, in any case, a character above ASCII
/// standing for the letters it folds to ([`case_fold::literal_end`]).
fn raw_tag(cx: &Cx<'_>, p: usize) -> Option<usize> {
    let text = cx.line.get(..cx.end.min(cx.line.len()))?;
    RAW_TAGS
        .iter()
        .find_map(|tag| case_fold::literal_end(text, p, tag))
}

/// The start of each `</script>`, `</style>` or `</pre>`, in any case, from
/// `from` to the end of the text.
fn raw_closes<'a>(cx: &'a Cx<'a>, from: usize) -> impl Iterator<Item = usize> + 'a {
    let end = cx.end.min(cx.line.len());
    let rest = cx.line.get(from..end).unwrap_or_default();
    cx.spend(rest.len() / 16);
    memchr::memchr_iter(b'<', rest)
        .map(move |at| from + at)
        .filter(|&at| {
            cx.get(at + 1) == Some(b'/')
                && raw_tag(cx, at + 2).is_some_and(|end| cx.get(end) == Some(b'>'))
        })
}

/// `(?i)(^|\G)\s*(?=<(script|style|pre)(\s|$|>)(?!.*?</(script|style|pre)>))`,
/// which opens a raw HTML block: one its line does not close.
pub(crate) fn raw_open(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = cx.skip_space(p);
    if cx.get(i) != Some(b'<') {
        return None;
    }
    let name = raw_tag(cx, i + 1)?;
    let ends = cx.space_at(name) > 0 || cx.at_eol(name) || cx.get(name) == Some(b'>');
    (ends && raw_closes(cx, name).next().is_none()).then_some(i)
}

/// `(?i)(.*)((</)(script|style|pre)(>))`, which closes a raw HTML block at
/// the last closing tag of the line.
pub(crate) fn raw_close(cx: &Cx<'_>, p: usize, caps: &mut Caps) -> Option<usize> {
    let at = raw_closes(cx, p).last()?;
    let end = raw_tag(cx, at + 2)? + 1;
    caps.set(1, p, at);
    caps.set(2, at, end);
    caps.set(3, at, at + 2);
    caps.set(4, at + 2, end - 1);
    caps.set(5, end - 1, end);
    Some(end)
}

/// `(?i)^(?!.*</(script|style|pre)>)`, which keeps the lines of a raw HTML
/// block going up to the closing one. `^` matches at the start of the line
/// only, so a block inside a container ends with its first line.
pub(crate) fn raw_while(cx: &Cx<'_>, p: usize) -> Option<usize> {
    (p == 0 && raw_closes(cx, 0).next().is_none()).then_some(p)
}

/// The two other HTML blocks:
/// `(?i)(^|\G)\s*(?=</?[A-Za-z]+[^\&/;gt\s]*(\s|$|/?>))` and `(^|\G)\s*(?=(<(?:
/// [-0-9A-Za-z](/?>|\s.*?>)|/[-0-9A-Za-z]>))\s*$)`, which match the same text.
pub(crate) fn block_open(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if !cx.at_start(p) {
        return None;
    }
    let i = cx.skip_space(p);
    (cx.get(i) == Some(b'<') && (tag_start(cx, i) || short_tag_line(cx, i))).then_some(i)
}

/// `</?[A-Za-z]+[^\&/;gt\s]*(\s|$|/?>)` at `p`, under `(?i)`, which keeps
/// `G` and `T` out of the class too.
///
/// A position past the letters is reachable when one of the letters ends
/// there, or when the byte before it is in the class and that position is
/// reachable; the match needs a reachable position that ends the tag.
fn tag_start(cx: &Cx<'_>, p: usize) -> bool {
    let start = p + 1 + usize::from(cx.get(p + 1) == Some(b'/'));
    let letters = cx.skip(start, |b| b.is_ascii_alphabetic());
    if letters == start {
        return false;
    }
    let mut k = start + 1;
    loop {
        if cx.space_at(k) > 0 || cx.at_eol(k) || cx.get(k) == Some(b'>') || cx.starts(k, b"/>") {
            cx.spend((k - p) / 16);
            return true;
        }
        let in_class = cx.get(k).is_some_and(|b| {
            !matches!(b, b'&' | b'/' | b';' | b'g' | b't' | b'G' | b'T') && cx.space_at(k) == 0
        });
        if k >= letters && !in_class {
            cx.spend((k - p) / 16);
            return false;
        }
        k += 1;
    }
}

/// `(<(?:[-0-9A-Za-z](/?>|\s.*?>)|/[-0-9A-Za-z]>))\s*$` at `p`: a tag with a
/// one-character name that ends the line.
fn short_tag_line(cx: &Cx<'_>, p: usize) -> bool {
    let name = |b: Option<u8>| b.is_some_and(|b| b == b'-' || b.is_ascii_alphanumeric());
    if cx.get(p + 1) == Some(b'/') {
        return name(cx.get(p + 2)) && cx.get(p + 3) == Some(b'>') && cx.only_space_after(p + 4);
    }
    if !name(cx.get(p + 1)) {
        return false;
    }
    let k = p + 2;
    if cx.get(k) == Some(b'>') {
        return cx.only_space_after(k + 1);
    }
    if cx.starts(k, b"/>") {
        return cx.only_space_after(k + 2);
    }
    let space = cx.space_at(k);
    if space == 0 || k >= cx.line.len() {
        return false;
    }
    let end = cx.end.min(cx.line.len());
    let rest = cx.line.get(k + space..end).unwrap_or_default();
    cx.spend(rest.len() / 16);
    memchr::memchr_iter(b'>', rest).any(|at| cx.only_space_after(k + space + at + 1))
}

/// `^(?!\s*$)`, which keeps an HTML block going up to a blank line. `^`
/// matches at the start of the line only, so a block inside a container
/// ends with its first line.
pub(crate) fn block_while(cx: &Cx<'_>, p: usize) -> Option<usize> {
    (p == 0 && !cx.only_space_after(0)).then_some(p)
}

/// `(?=<[^>]*?>)`, the rule that emphasis tries first, which hands a tag to
/// html-derivative.
pub(crate) fn wrap_open(cx: &Cx<'_>, p: usize) -> Option<usize> {
    if cx.get(p) != Some(b'<') {
        return None;
    }
    let end = cx.end.min(cx.line.len());
    let rest = cx.line.get(p + 1..end).unwrap_or_default();
    cx.spend(rest.len() / 16);
    memchr::memchr(b'>', rest).map(|_| p)
}
