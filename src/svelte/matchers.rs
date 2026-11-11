//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.

use super::engine::{Lexer, closing_tag};
use super::grammar::{Pattern, Rule};
use crate::classes::{is_alnum, is_alpha};
use crate::rules::Chars;

/// The words before `\G` that pick the first mode, which embeds TypeScript:
/// `(?<=(if|key|then|catch|html|render).*?)\G`.
const PLAIN_MODES: &[&[u8]] = &[b"if", b"key", b"then", b"catch", b"html", b"render"];

/// The directive keywords of `attributes-directives`, in the grammar's order.
const DIRECTIVES: &[&[u8]] = &[
    b"on",
    b"use",
    b"bind",
    b"transition",
    b"in",
    b"out",
    b"animate",
    b"let",
    b"class",
    b"style",
];

/// The void elements of `tags-void`, in the grammar's order.
const VOID_TAGS: &[&[u8]] = &[
    b"area", b"base", b"br", b"col", b"embed", b"hr", b"img", b"input", b"link", b"meta", b"param",
    b"source", b"track", b"wbr",
];

/// The language a tag's body is in, from the tag and its `lang` or `type`
/// value: the selectors of the grammar's injections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Body {
    JavaScript,
    TypeScript,
    CoffeeScript,
    Css,
    PostCss,
    Scss,
    Less,
    Sass,
    /// A body no injection Shiki loads applies to, which stays plain.
    Plain,
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms follow the grammar's order: bodies, comments, special tags,
    /// tags, attributes and text.
    /// It is `#[inline]`, because the scan calls it from `engine`
    /// (`bench/AGENTS.md` rule 13).
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    #[inline]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::BodyJavaScript => self.body_opens(p, Body::JavaScript),
            Pattern::BodyTypeScriptBlock => self.body_opens(p, Body::TypeScript),
            Pattern::BodyCss => self.body_opens(p, Body::Css),
            Pattern::BodyPostCss => self.body_opens(p, Body::PostCss),
            Pattern::BodyCoffeeScript => self.body_opens(p, Body::CoffeeScript),
            Pattern::BodyScss => self.body_opens(p, Body::Scss),
            Pattern::BodyLess => self.body_opens(p, Body::Less),
            Pattern::BodySass => self.body_opens(p, Body::Sass),
            Pattern::BodyTypeScriptLine => (self.prev(p) == Some(b'>')
                && self.body() == Body::TypeScript
                && (p + 1..self.lim()).any(|q| closing_tag(self.text(q, self.lim()))))
            .then_some(p),
            Pattern::BodyTemplate => {
                (self.prev(p) == Some(b'>') && self.space_at(p) > 0 && self.in_template())
                    .then(|| p + self.space_at(p))
            }
            Pattern::BodyJavaScriptEnd
            | Pattern::BodyCssEnd
            | Pattern::BodyPostCssEnd
            | Pattern::BodyCoffeeScriptEnd
            | Pattern::BodyScssEnd
            | Pattern::BodyLessEnd
            | Pattern::BodySassEnd => self.starts(p, b"</").then_some(p),
            Pattern::TagsLangAt0End => {
                (self.starts(p, b"</") || self.starts(p, b"/>")).then_some(p)
            }
            Pattern::BodyTypeScriptLineEnd => closing_tag(self.text(p, self.lim())).then_some(p),
            Pattern::BodyTypeScriptBlockWhile => {
                let blanks = self.space_end(p);
                (p == 0 && !closing_tag(self.text(blanks, self.lim()))).then_some(p)
            }
            Pattern::BodyTemplateEnd => self.starts(p, b"</template").then_some(p),

            Pattern::Comments => self.lit(p, b"<!--"),
            Pattern::CommentsEnd => self.lit(p, b"-->"),
            Pattern::CommentsAt0 => {
                let end = self.lit(p, b"@component")?;
                self.capture(1, p, p + 1);
                self.capture(2, p + 1, end);
                Some(end)
            }
            Pattern::CommentsAt0End => self.starts(p, b"-->").then_some(p),
            Pattern::CommentsAt0At0 => (p..self.lim())
                .find(|&q| self.starts(q, b"-->"))
                .filter(|&q| q > p),
            Pattern::CommentsAt1 => self.comment_invalid(p),

            Pattern::NestedSpecialTagsKeywordsGroup2At0 => {
                if self.starts(p, b"if") {
                    return Some(p + 2);
                }
                let end = self.lit(p, b"else")?;
                let blanks = self.space_end(end);
                if blanks > end && self.starts(blanks, b"if") {
                    return Some(blanks + 2);
                }
                Some(end)
            }
            Pattern::NestedSpecialTagsKeywordsGroup2At1 => self.lit_in(p, &[b"each", b"key"]),
            Pattern::NestedSpecialTagsKeywordsGroup2At2 => {
                self.lit_in(p, &[b"await", b"then", b"catch"])
            }
            Pattern::NestedSpecialTagsKeywordsGroup2At3 => self.lit(p, b"snippet"),
            Pattern::NestedSpecialTagsKeywordsGroup2At4 => self.lit(p, b"html"),
            Pattern::NestedSpecialTagsKeywordsGroup2At5 => self.lit(p, b"render"),
            Pattern::NestedSpecialTagsKeywordsGroup2At6 => self.lit(p, b"debug"),
            Pattern::NestedSpecialTagsKeywordsGroup2At7 => self.lit(p, b"const"),
            Pattern::SpecialTagsKeywords => {
                if !matches!(self.at(p), b'#' | b'/' | b':' | b'@') || p >= self.lim() {
                    return None;
                }
                self.capture(1, p, p + 1);
                let end = self.tag_keyword(p + 1);
                self.capture(2, p + 1, end);
                Some(end)
            }
            Pattern::SpecialTagsVoid => self.special_tag(p, |b| b == b':' || b == b'@'),
            Pattern::SpecialTagsBlockBegin => self.special_tag(p, |b| b == b'#'),
            Pattern::SpecialTagsBlockEnd => self.special_tag(p, |b| b == b'/'),
            Pattern::SpecialTagsBlockBeginEnd | Pattern::SpecialTagsBlockEndEnd => {
                let end = self.lit(p, b"}")?;
                self.capture(1, p, end);
                Some(end)
            }

            Pattern::SpecialTagsModesAt0 => self.mode(p, PLAIN_MODES),
            Pattern::SpecialTagsModesAt1 => self.mode(p, &[b"snippet"]),
            Pattern::SpecialTagsModesAt2 => self.mode(p, &[b"const"]),
            Pattern::SpecialTagsModesAt3 => self.mode(p, &[b"each"]),
            Pattern::SpecialTagsModesAt4 => self.mode(p, &[b"await"]),
            Pattern::SpecialTagsModesAt5 => self.mode(p, &[b"debug"]),
            Pattern::SpecialTagsModesAt0End
            | Pattern::SpecialTagsModesAt1End
            | Pattern::SpecialTagsModesAt1At2End
            | Pattern::SpecialTagsModesAt1At3End
            | Pattern::SpecialTagsModesAt2End
            | Pattern::SpecialTagsModesAt2At3End
            | Pattern::SpecialTagsModesAt3End
            | Pattern::SpecialTagsModesAt3At1End
            | Pattern::SpecialTagsModesAt4End
            | Pattern::SpecialTagsModesAt4At1End
            | Pattern::SpecialTagsModesAt5End => (self.at(p) == b'}').then_some(p),
            Pattern::SpecialTagsModesAt1At0 => {
                if !self.at_g(p) {
                    return None;
                }
                let start = self.space_end(p);
                let name = self.ident(start)?;
                let end = self.space_end(name);
                (self.at(end) == b'<').then(|| {
                    self.capture(1, start, name);
                    end
                })
            }
            Pattern::SpecialTagsModesAt1At1 => (self.prev(p) == Some(b'<')).then_some(p),
            Pattern::SpecialTagsModesAt1At1End => (self.at(p) == b'>').then_some(p),
            Pattern::SpecialTagsModesAt1At2 => self.after_type_parameters(p).then_some(p),
            Pattern::SpecialTagsModesAt2At1 => {
                if !self.at_g(p) {
                    return None;
                }
                let start = self.space_end(p);
                let first = self.ident_start(start)?;
                let name = self.ident_end(start + first);
                (name > start + first).then(|| {
                    self.capture(1, start, name);
                    self.space_end(name)
                })
            }
            Pattern::SpecialTagsModesAt2At1End => matches!(self.at(p), b':' | b'=').then_some(p),
            Pattern::SpecialTagsModesAt2At2 => (self.at(p) == b':').then_some(p),
            Pattern::SpecialTagsModesAt2At2End | Pattern::SpecialTagsModesAt2At3 => {
                (self.at(p) == b'=').then_some(p)
            }
            Pattern::DestructuringConstAt0 | Pattern::DestructuringAt0 => {
                (self.at(p) == b'{').then_some(p)
            }
            Pattern::DestructuringConstAt0End
            | Pattern::DestructuringAt0End
            | Pattern::InterpolationAt0At0End => (self.prev(p) == Some(b'}')).then_some(p),
            Pattern::DestructuringConstAt1 | Pattern::DestructuringAt1 => {
                (self.at(p) == b'[').then_some(p)
            }
            Pattern::DestructuringConstAt1End | Pattern::DestructuringAt1End => {
                (self.prev(p) == Some(b']')).then_some(p)
            }
            Pattern::SpecialTagsModesAt3At0 | Pattern::SpecialTagsModesAt4At0 => {
                if !self.at_g(p) {
                    return None;
                }
                let end = self.space_end(p);
                (end < self.lim()).then_some(end)
            }
            Pattern::SpecialTagsModesAt3At0End => self.each_expression_ends(p).then_some(p),
            Pattern::SpecialTagsModesAt3At1 => {
                if let Some(end) = self.lit(p, b"as") {
                    self.capture(1, p, end);
                    return Some(end);
                }
                matches!(self.at(p), b',' | b'}').then_some(p)
            }
            Pattern::SpecialTagsModesAt3At1At1 => self.lit(p, b"("),
            Pattern::SpecialTagsModesAt3At1At1End => {
                if self.at(p) == b')' {
                    return Some(p + 1);
                }
                (self.at(p) == b'}').then_some(p)
            }
            Pattern::SpecialTagsModesAt3At1At2 => {
                let start = self.space_end(p);
                let name = self.ident(start)?;
                let end = self.space_end(name);
                self.capture(1, p, end);
                self.capture(2, start, name);
                Some(end)
            }
            Pattern::SpecialTagsModesAt3At1At3 | Pattern::SpecialTagsModesAt5At1 => {
                self.lit(p, b",")
            }
            Pattern::SpecialTagsModesAt4At0End => {
                let blanks = self.space_end(p);
                if blanks > p
                    && let Some(end) = self.lit(blanks, b"then")
                {
                    self.capture(1, blanks, end);
                    return Some(end);
                }
                (self.at(p) == b'}').then_some(p)
            }
            Pattern::SpecialTagsModesAt4At1 => {
                (p >= 4 && self.text(p - 4, p) == b"then" && self.word_at(p).is_none()).then_some(p)
            }
            Pattern::SpecialTagsModesAt5At0 => self.ident(p),

            Pattern::TagsNameAt0 => {
                let colon = self.lit(p, b"svelte:")?;
                if !self.at(colon).is_ascii_lowercase() {
                    return None;
                }
                let end = self.skip_chars(colon + 1, |c| {
                    c == '-' || c == ':' || crate::classes::is_word(c)
                });
                self.capture(1, p, colon - 1);
                self.capture(2, colon - 1, colon);
                self.capture(3, colon, end);
                Some(end)
            }
            Pattern::TagsNameAt1 => self.lit(p, b"slot"),
            Pattern::TagsNameAt2 => self.component_name(p),
            Pattern::TagsNameAt3 => {
                if !self.at(p).is_ascii_lowercase() {
                    return None;
                }
                let head = self.skip_chars(p + 1, |c| {
                    matches!(c, '0'..=':') || crate::classes::is_word(c)
                });
                if self.at(head) != b'-' {
                    return None;
                }
                Some(self.skip_chars(head + 1, |c| {
                    c == '-' || matches!(c, '0'..=':') || crate::classes::is_word(c)
                }))
            }
            Pattern::TagsNameAt4 => {
                if !self.at(p).is_ascii_lowercase() {
                    return None;
                }
                Some(self.skip_chars(p + 1, |c| {
                    c == '-' || matches!(c, '0'..=':') || crate::classes::is_word(c)
                }))
            }
            Pattern::NestedTagsNameAt2Group1At1 => self.lit(p, b"."),
            Pattern::TagsStartNode | Pattern::TagsGeneralStart => {
                let name = self.lit(p, b"<")?;
                let end = self.tag_name_end(name);
                self.capture(1, p, name);
                self.capture(2, name, end);
                Some(end)
            }
            Pattern::TagsGeneralEnd => {
                let name = self.lit(p, b"</")?;
                let end = self.tag_name_end(name);
                self.capture(1, p, name);
                self.capture(2, name, end);
                Some(end)
            }
            Pattern::TagsEndNode => self.end_node(p),
            Pattern::TagsLang => {
                let name = self.lit(p, b"<")?;
                let end = self.lit_in(name, &[b"script", b"style", b"template"])?;
                self.capture(1, name, end);
                Some(end)
            }
            Pattern::TagsLangEnd => self.tag_lang_end(p),
            Pattern::TagsLangAt0 => self.at_g(p).then(|| self.lang_attribute(p)).flatten(),
            Pattern::SpecialTagsModesAt1At3
            | Pattern::TagsLangStartAttributes
            | Pattern::TagsStartAttributes => self.at_g(p).then_some(p),
            Pattern::TagsLangStartAttributesEnd => {
                if self.starts(p, b"/>") {
                    return Some(p);
                }
                self.lit(p, b">")
            }
            Pattern::GenericsDoubleQuoted => self.generics(p, b'"'),
            Pattern::GenericsSingleQuoted => self.generics(p, b'\''),
            Pattern::GenericsDoubleQuotedEnd => self.lit_capture1(p, b"\""),
            Pattern::GenericsSingleQuotedEnd => self.lit_capture1(p, b"'"),

            Pattern::AttributesCommentsAt0 => self.lit(p, b"//").map(|_| self.lim()),
            Pattern::AttributesCommentsAt1 => self.lit(p, b"/*"),
            Pattern::AttributesCommentsAt1End => self.lit(p, b"*/"),
            Pattern::AttributesDirectivesKeywordsAt0 => self.lit_in(p, &[b"on", b"use", b"bind"]),
            Pattern::AttributesDirectivesKeywordsAt1 => {
                self.lit_in(p, &[b"transition", b"in", b"out", b"animate"])
            }
            Pattern::AttributesDirectivesKeywordsAt2 => self.lit(p, b"let"),
            Pattern::AttributesDirectivesKeywordsAt3 => self.lit_in(p, &[b"class", b"style"]),
            Pattern::AttributesDirectivesTypesAssignedAt0 => {
                let end = self.lit(p, b"this")?;
                (self.after_word(p, &[b"bind"]) && self.at_eol(end)).then_some(end)
            }
            Pattern::AttributesDirectivesTypesAssignedAt1
            | Pattern::AttributesDirectivesTypesAt1 => {
                self.after_word(p, &[b"bind"]).then(|| self.lim())
            }
            Pattern::AttributesDirectivesTypesAssignedAt2 => {
                self.after_word(p, &[b"class"]).then(|| self.lim())
            }
            Pattern::AttributesDirectivesTypesAssignedAt3 => {
                self.after_word(p, &[b"style"]).then(|| self.lim())
            }
            Pattern::AttributesDirectivesTypesAt0 => {
                self.after_word(p, &[b"on"]).then(|| self.lim())
            }
            Pattern::AttributesDirectivesTypesAt2 => self
                .after_word(p, &[b"use", b"transition", b"in", b"out", b"animate"])
                .then(|| self.lim()),
            Pattern::AttributesDirectivesTypesAt3 => self
                .after_word(p, &[b"let", b"class", b"style"])
                .then(|| self.lim()),
            Pattern::NestedTagsNameAt2Group1At0 | Pattern::NestedAttributesDirectivesGroup5At0 => {
                let end = self.word_end(p);
                (end > p).then_some(end)
            }
            Pattern::NestedAttributesDirectivesGroup5At1 => self.lit(p, b"|"),
            Pattern::AttributesDirectives => self.directive(p),
            Pattern::AttributesDirectivesEnd | Pattern::AttributesKeyvalueEnd => {
                let next = self.space_end(p);
                (next < self.lim() && self.at(next) != b'=').then_some(p)
            }
            Pattern::AttributesDirectivesAt0 | Pattern::AttributesKeyvalueAt0 => self.lit(p, b"="),
            Pattern::AttributesDirectivesAt0End | Pattern::AttributesKeyvalueAt0End => {
                let ended = self
                    .prev(p)
                    .is_some_and(|b| b != b'=' && !b.is_ascii_whitespace())
                    && self.at(self.space_end(p)) != b'=';
                (ended || self.starts(p, b"/>") || self.at(p) == b'>').then_some(p)
            }
            Pattern::InterpolationAt0 => self.lit(p, b"{"),
            Pattern::SpecialTagsVoidEnd | Pattern::InterpolationAt0End => self.lit(p, b"}"),
            Pattern::InterpolationAt0At0 => {
                (self.at_g(p) && self.at(self.space_end(p)) == b'{').then(|| self.space_end(p))
            }
            Pattern::AttributesValueAt1 => self.number_value(p),
            Pattern::AttributesValueAt2 => {
                let mut end = p;
                loop {
                    let b = self.at(end);
                    if end >= self.lim() {
                        break;
                    }
                    if b == b'/' {
                        if self.at(end + 1) == b'>' {
                            break;
                        }
                    } else if matches!(b, b'"' | b'\'' | b'<' | b'=' | b'>' | b'`')
                        || self.space_at(end) > 0
                    {
                        break;
                    }
                    end = self.char_end(end);
                }
                (end > p).then_some(end)
            }
            Pattern::AttributesValueAt3 => {
                if !matches!(self.at(p), b'"' | b'\'') || p >= self.lim() {
                    return None;
                }
                self.capture(1, p, p + 1);
                Some(p + 1)
            }
            Pattern::AttributesValueAt3End => self.begin_text_at(p).map(|width| p + width),
            Pattern::NestedAttributesKeyvalueGroup0At0 => self.lit(p, b"--").map(|_| self.lim()),
            Pattern::NestedAttributesKeyvalueGroup0At1 => Some(self.lim()).filter(|&end| end > p),
            Pattern::AttributesKeyvalue => {
                let end = self.attribute_name(p)?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::AttributesAttach => {
                if self.prev(p).is_some_and(|b| b == b':' || b == b'=') {
                    return None;
                }
                let start = self.space_end(p);
                let at = self.lit(start, b"{@attach")?;
                let width = self.space_at(at);
                (width > 0).then(|| {
                    self.capture(1, start, at + width);
                    at + width
                })
            }
            Pattern::AttributesInterpolated => {
                if self.prev(p).is_some_and(|b| b == b':' || b == b'=') {
                    return None;
                }
                let start = self.space_end(p);
                let end = self.lit(start, b"{")?;
                self.capture(1, start, end);
                Some(end)
            }
            Pattern::AttributesAttachEnd | Pattern::AttributesInterpolatedEnd => {
                self.lit_capture1(p, b"}")
            }
            Pattern::TagsVoid => {
                let name = self.lit(p, b"<")?;
                let end = self.lit_in(name, VOID_TAGS)?;
                let next = self.at(end);
                if !(self.space_at(end) > 0 || next == b'>' || self.starts(end, b"/>")) {
                    return None;
                }
                self.capture(1, p, name);
                self.capture(2, name, end);
                Some(end)
            }
            Pattern::TagsVoidEnd => {
                let start = if self.at(p) == b'/' { p + 1 } else { p };
                self.lit(start, b">")
            }
            Pattern::TagsGeneralEndEnd | Pattern::TagsGeneralStartEnd => {
                let start = if pattern == Pattern::TagsGeneralStartEnd && self.at(p) == b'/' {
                    p + 1
                } else {
                    p
                };
                let end = self.lit(start, b">")?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::TagsStartAttributesEnd => {
                (self.at(p) == b'>' || self.starts(p, b"/>")).then_some(p)
            }
            Pattern::ScopeAt4 => self
                .prev(p)
                .is_some_and(|b| b == b'>' || b == b'}')
                .then_some(p),
            Pattern::ScopeAt4End => matches!(self.at(p), b'<' | b'{').then_some(p),
        }
    }

    /// The language of the body of the language tag the innermost rule
    /// sits in.
    /// It comes from the injections' selectors: the tag (`meta.$1`) and its
    /// `lang` value (`meta.lang.$3`), when it has one.
    fn body(&self) -> Body {
        let top = self.top_rule();
        let (tag, lang) = if top == Rule::TagsLang {
            (self.begin_text(), None)
        } else if top == Rule::TagsLangAt0 {
            (self.parent_begin_text(), Some(self.begin_text()))
        } else {
            return Body::Plain;
        };
        let script_or_style = tag == "script" || tag == "style";
        match (tag, lang) {
            ("script", None) => Body::JavaScript,
            (_, Some("js" | "javascript")) if script_or_style => Body::JavaScript,
            (_, Some("ts" | "typescript")) if script_or_style => Body::TypeScript,
            (_, Some("coffee")) if script_or_style => Body::CoffeeScript,
            ("style", None | Some("css")) => Body::Css,
            ("style", Some("postcss")) => Body::PostCss,
            ("style", Some("scss")) => Body::Scss,
            ("style", Some("less")) => Body::Less,
            ("style", Some("sass")) => Body::Sass,
            _ => Body::Plain,
        }
    }

    /// Whether the open rules lie inside a template without `lang`, where the
    /// template body's selector holds.
    /// A rule between them that hides the template from it
    /// ([`Rule::hides_template`]) keeps it out, as `meta.lang.<lang>.svelte`
    /// does for a template with `lang`.
    fn in_template(&self) -> bool {
        self.open_rules()
            .find_map(|(index, rule)| {
                if rule == Rule::TagsLang {
                    Some(self.begin_text_of(index) == "template")
                } else {
                    rule.hides_template().then_some(false)
                }
            })
            .unwrap_or(false)
    }

    /// `(?<=>)(?!</)`, where the tag's body is in `body`.
    fn body_opens(&self, p: usize, body: Body) -> Option<usize> {
        (self.prev(p) == Some(b'>') && !self.starts(p, b"</") && self.body() == body).then_some(p)
    }

    /// `\G-?>|<!--(?!>)|<!-(?=-->)|--!>`.
    fn comment_invalid(&self, p: usize) -> Option<usize> {
        if self.at_g(p) {
            let start = if self.at(p) == b'-' { p + 1 } else { p };
            if let Some(end) = self.lit(start, b">") {
                return Some(end);
            }
        }
        if let Some(end) = self.lit(p, b"<!--") {
            return (self.at(end) != b'>').then_some(end);
        }
        if let Some(end) = self.lit(p, b"<!-")
            && self.starts(end, b"-->")
        {
            return Some(end);
        }
        self.lit(p, b"--!>")
    }

    /// The end of the first of `words` at `p`, with no word boundary.
    fn lit_in(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        words.iter().find_map(|word| self.lit(p, word))
    }

    /// `(text)`, the whole match as group 1.
    fn lit_capture1(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `else\s+if|[a-z]*` from `p`.
    fn tag_keyword(&self, p: usize) -> usize {
        if let Some(end) = self.lit(p, b"else") {
            let blanks = self.space_end(end);
            if blanks > end && self.starts(blanks, b"if") {
                return blanks + 2;
            }
        }
        self.skip(p, |b| b.is_ascii_lowercase())
    }

    /// `(\{)\s*(M(else\s+if|[a-z]*))`, with `M` the marks `mark` accepts:
    /// a special tag's begin, with its keyword as group 3.
    fn special_tag(&mut self, p: usize, mark: impl Fn(u8) -> bool) -> Option<usize> {
        let open = self.lit(p, b"{")?;
        let at = self.space_end(open);
        if at >= self.lim() || !mark(self.at(at)) {
            return None;
        }
        if self.at(at) == b'/' && matches!(self.at(at + 1), b'*' | b'/') {
            return None;
        }
        let end = self.tag_keyword(at + 1);
        self.capture(1, p, open);
        self.capture(2, at, end);
        self.capture(3, at + 1, end);
        Some(end)
    }

    /// A mode of a special tag: `(?<=(words).*?)\G`. One of `words` stands
    /// anywhere on the line before `\G`, even outside this tag.
    fn mode(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let before = self.text(0, p);
        words
            .iter()
            .any(|word| before.windows(word.len()).any(|w| w == *word))
            .then_some(p)
    }

    /// `(?<=>\s*\()`: a `(` before `p`, with blanks and a `>` before it.
    fn after_type_parameters(&self, p: usize) -> bool {
        if self.prev(p) != Some(b'(') {
            return false;
        }
        let before = self.text(0, p - 1);
        let rest = before.trim_ascii_end();
        rest.last() == Some(&b'>')
    }

    /// The end of an `{#each}` expression: `(?=(?:^\s*|\s+)(as)|\s*([,}]))`.
    fn each_expression_ends(&self, p: usize) -> bool {
        let blanks = self.space_end(p);
        ((blanks > p || p == 0) && self.starts(blanks, b"as"))
            || matches!(self.at(blanks), b',' | b'}') && blanks < self.lim()
    }

    /// The width of `[$_[:alpha:]]` at `i`.
    fn ident_start(&self, i: usize) -> Option<usize> {
        let (c, end) = self.char_at(i)?;
        (i < self.lim() && (c == '$' || c == '_' || is_alpha(c))).then_some(end - i)
    }

    /// The end of the run of `[$_[:alnum:]]` from `i`.
    fn ident_end(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && (c == '$' || c == '_' || is_alnum(c))
        {
            i = end;
        }
        i
    }

    /// `[$_[:alpha:]][$_[:alnum:]]*` at `i`.
    fn ident(&self, i: usize) -> Option<usize> {
        let first = self.ident_start(i)?;
        Some(self.ident_end(i + first))
    }

    /// The end of the run from `i` whose characters `class` accepts.
    fn skip_chars(&self, mut i: usize, class: impl Fn(char) -> bool) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && class(c)
        {
            i = end;
        }
        i
    }

    /// `[^/>\s]*` from `i`.
    fn tag_name_end(&self, mut i: usize) -> usize {
        while i < self.lim() && !matches!(self.at(i), b'/' | b'>') && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        i
    }

    /// `(\w+(?:\.\w+)+)|([A-Z]\w*)`: a dotted name as group 1, or a
    /// capitalised one as group 2.
    fn component_name(&mut self, p: usize) -> Option<usize> {
        let first = self.word_end(p);
        if first > p {
            let mut end = first;
            while self.at(end) == b'.' {
                let next = self.word_end(end + 1);
                if next == end + 1 {
                    break;
                }
                end = next;
            }
            if end > first {
                self.capture(1, p, end);
                return Some(end);
            }
        }
        if !self.at(p).is_ascii_uppercase() {
            return None;
        }
        self.capture(2, p, first);
        Some(first)
    }

    /// `(</)(.*?)\s*(>)|(/>)`.
    fn end_node(&mut self, p: usize) -> Option<usize> {
        if let Some(name) = self.lit(p, b"</") {
            let close = (name..self.lim()).find(|&q| self.at(q) == b'>');
            if let Some(close) = close {
                let text = self.text(name, close);
                let trimmed = name + text.trim_ascii_end().len();
                self.capture(1, p, name);
                self.capture(2, name, trimmed);
                self.capture(3, close, close + 1);
                return Some(close + 1);
            }
        }
        let end = self.lit(p, b"/>")?;
        self.capture(4, p, end);
        Some(end)
    }

    /// `</\1\s*>|/>`, with `\1` the tag's name, the rule's begin text.
    fn tag_lang_end(&mut self, p: usize) -> Option<usize> {
        if let Some(name) = self.lit(p, b"</")
            && let Some(width) = self.begin_text_at(name)
        {
            let close = self.space_end(name + width);
            if let Some(end) = self.lit(close, b">") {
                return Some(end);
            }
        }
        self.lit(p, b"/>")
    }

    /// `(?=\s*[^>]*?(type|lang)\s*=\s*(["']?)(?:text/)?(\w+)\2)` at `p`:
    /// the first `type` or `lang` before the next `>` whose value is a word,
    /// with its value as group 3.
    fn lang_attribute(&mut self, p: usize) -> Option<usize> {
        let mut q = p;
        while q < self.lim() && self.at(q) != b'>' {
            let name = self.lit(q, b"type").or_else(|| self.lit(q, b"lang"));
            if let Some(name) = name
                && let Some((quote, value, end)) = self.lang_value(name)
            {
                self.capture(1, q, name);
                self.capture(2, quote.0, quote.1);
                self.capture(3, value, end);
                return Some(p);
            }
            q = self.char_end(q);
        }
        None
    }

    /// `\s*=\s*(["']?)(?:text/)?(\w+)\2` from `i`: the quote's span, and
    /// the word's start and end.
    fn lang_value(&self, i: usize) -> Option<((usize, usize), usize, usize)> {
        let eq = self.space_end(i);
        if self.at(eq) != b'=' {
            return None;
        }
        let open = self.space_end(eq + 1);
        let quotes: &[usize] = if matches!(self.at(open), b'"' | b'\'') {
            &[1, 0]
        } else {
            &[0]
        };
        for &width in quotes {
            let start = open + width;
            for skip in [5, 0] {
                if skip == 5 && !self.starts(start, b"text/") {
                    continue;
                }
                let value = start + skip;
                let end = self.word_end(value);
                if end == value {
                    continue;
                }
                let closes = width == 0 || (self.at(end) == self.at(open) && end < self.lim());
                if closes {
                    return Some(((open, open + width), value, end));
                }
            }
        }
        None
    }

    /// `(generics)(=)(Q)` with `Q` the quote.
    fn generics(&mut self, p: usize, quote: u8) -> Option<usize> {
        let eq = self.lit(p, b"generics")?;
        let open = self.lit(eq, b"=")?;
        (self.at(open) == quote && open < self.lim()).then(|| {
            self.capture(1, p, eq);
            self.capture(2, eq, open);
            self.capture(3, open, open + 1);
            open + 1
        })
    }

    /// `(?<=(words):)`: one of `words` and a `:` end at `p`.
    fn after_word(&self, p: usize, words: &[&[u8]]) -> bool {
        p > 0
            && self.at(p - 1) == b':'
            && words.iter().any(|word| self.text(0, p - 1).ends_with(word))
    }

    /// `((?:--)?[$_[:alpha:]][-$_[:alnum:]]*)` at `p`.
    fn attribute_name(&self, p: usize) -> Option<usize> {
        let start = if self.starts(p, b"--") && self.ident_start(p + 2).is_some() {
            p + 2
        } else {
            p
        };
        let first = self.ident_start(start)?;
        Some(self.skip_chars(start + first, |c| {
            c == '-' || c == '$' || c == '_' || is_alnum(c)
        }))
    }

    /// `attributes-directives`' begin: the keyword (group 1), the `:` (2),
    /// the name before an `=` (3) or the name (4), and the modifiers (5).
    fn directive(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b'<') {
            return None;
        }
        let (keyword, colon) = DIRECTIVES.iter().find_map(|word| {
            let end = self.lit(p, word)?;
            (self.at(end) == b':').then_some((end, end + 1))
        })?;
        let name = self.attribute_name(colon)?;
        self.capture(1, p, keyword);
        self.capture(2, keyword, colon);
        if self.at(self.space_end(name)) == b'=' {
            self.capture(3, colon, name);
        } else {
            self.capture(4, colon, name);
        }
        let mut end = name;
        while self.at(end) == b'|' {
            let word = self.word_end(end + 1);
            if word == end + 1 {
                break;
            }
            self.capture(6, end, word);
            end = word;
        }
        self.capture(5, name, end);
        Some(end)
    }

    /// `(["'])([.0-9_]+[%\w]{0,4})(\1)|([.0-9_]+[%\w]{0,4})(?=\s|/?>)`.
    /// The first run takes every `[.0-9_]`: a shorter one leaves the second
    /// run more to cover, and `.` is not in it.
    fn number_value(&mut self, p: usize) -> Option<usize> {
        let quote = self.at(p);
        let quoted = matches!(quote, b'"' | b'\'') && p < self.lim();
        let start = if quoted { p + 1 } else { p };
        let digits = self.skip(start, |b| b == b'.' || b == b'_' || b.is_ascii_digit());
        if digits == start {
            return None;
        }
        let mut end = digits;
        for _ in 0..4 {
            match self.word_at(end) {
                Some(width) => end += width,
                None if self.at(end) == b'%' && end < self.lim() => end += 1,
                None => break,
            }
        }
        if quoted {
            if self.at(end) != quote || end >= self.lim() {
                return None;
            }
            self.capture(1, p, p + 1);
            self.capture(2, start, end);
            self.capture(3, end, end + 1);
            return Some(end + 1);
        }
        let follows = self.space_at(end) > 0 || self.at(end) == b'>' || self.starts(end, b"/>");
        follows.then(|| {
            self.capture(4, start, end);
            end
        })
    }
}
