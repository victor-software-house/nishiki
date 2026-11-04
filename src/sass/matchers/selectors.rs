//! Selectors: tag names, classes and the selector after a `}`.

use super::{Lexer, is_ident};
use crate::rules::Chars;

/// The tag names of the grammar's selector rule, but `h1` to `h6`.
const TAGS: &[&[u8]] = &[
    b"a",
    b"abbr",
    b"acronym",
    b"address",
    b"applet",
    b"area",
    b"article",
    b"aside",
    b"audio",
    b"b",
    b"base",
    b"big",
    b"blockquote",
    b"body",
    b"br",
    b"button",
    b"canvas",
    b"caption",
    b"cite",
    b"code",
    b"col",
    b"colgroup",
    b"datalist",
    b"dd",
    b"del",
    b"details",
    b"dfn",
    b"dialog",
    b"div",
    b"dl",
    b"dt",
    b"em",
    b"embed",
    b"eventsource",
    b"fieldset",
    b"figure",
    b"figcaption",
    b"footer",
    b"form",
    b"frame",
    b"frameset",
    b"head",
    b"header",
    b"hgroup",
    b"hr",
    b"html",
    b"i",
    b"iframe",
    b"img",
    b"input",
    b"ins",
    b"kbd",
    b"label",
    b"legend",
    b"li",
    b"link",
    b"map",
    b"mark",
    b"menu",
    b"meta",
    b"meter",
    b"nav",
    b"noframes",
    b"noscript",
    b"object",
    b"ol",
    b"optgroup",
    b"option",
    b"output",
    b"p",
    b"param",
    b"picture",
    b"pre",
    b"progress",
    b"q",
    b"samp",
    b"script",
    b"section",
    b"select",
    b"small",
    b"source",
    b"span",
    b"strike",
    b"strong",
    b"style",
    b"sub",
    b"summary",
    b"sup",
    b"table",
    b"tbody",
    b"td",
    b"textarea",
    b"tfoot",
    b"th",
    b"thead",
    b"time",
    b"title",
    b"tr",
    b"tt",
    b"ul",
    b"var",
    b"video",
    b"main",
    b"svg",
    b"rect",
    b"ruby",
    b"center",
    b"circle",
    b"ellipse",
    b"line",
    b"polyline",
    b"polygon",
    b"path",
    b"text",
    b"u",
    b"slot",
];

impl Lexer<'_> {
    /// `(?<![-(])\b(a|abbr|...|slot)\b(?![-)]|:\s)|&`.
    ///
    /// Every name is word characters, and a `\b` follows it.
    /// So a name matches only when it is the whole run of word characters at
    /// `p`, and no other name can stand in for it.
    /// After the name, a `-`, a `)` or a `:` before a blank refuses it.
    pub(super) fn tag(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'&' {
            return Some(p + 1);
        }
        if matches!(self.prev(p), Some(b'-' | b'(')) {
            return None;
        }
        let end = self.word_in(p, TAGS).or_else(|| self.heading(p))?;
        let refused = match self.at(end) {
            b'-' | b')' => true,
            b':' => self.space_at(end + 1) > 0,
            _ => false,
        };
        (!refused).then_some(end)
    }

    /// `\b(h[1-6])\b`.
    fn heading(&self, p: usize) -> Option<usize> {
        (self.boundary(p)
            && self.at(p) == b'h'
            && matches!(self.at(p + 1), b'1'..=b'6')
            && self.word_at(p + 2).is_none())
        .then_some(p + 2)
    }

    /// `\.|(?<=&)([-_])`: a class, or a `-` or `_` that goes on a parent
    /// selector.
    pub(super) fn class_start(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'.' => Some(p + 1),
            b'-' | b'_' if self.prev(p) == Some(b'&') => Some(p + 1),
            _ => None,
        }
    }

    /// `(?<=})(?![\n()]|[-0-9A-Z_a-z]+:)`: a selector right after a `}`.
    /// The begin is empty.
    pub(super) fn after_brace(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b'}') || matches!(self.at(p), b'\n' | b'(' | b')') {
            return None;
        }
        let run = self.skip(p, is_ident);
        (run == p || self.at(run) != b':').then_some(p)
    }
}
