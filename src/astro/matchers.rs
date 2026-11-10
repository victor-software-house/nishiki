//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.

use super::engine::Lexer;
use super::grammar::{Pattern, Rule};
use crate::classes::{is_alnum, is_alpha, is_word};
use crate::html_words::{ENTITIES, EVENT_HANDLERS};
use crate::rules::Chars;

/// The void elements of `tags-void`, in the grammar's order.
/// No name is a prefix of another, so the first that matches is the one the
/// alternation takes.
const VOID_TAGS: &[&[u8]] = &[
    b"area", b"base", b"br", b"col", b"embed", b"hr", b"img", b"input", b"link", b"meta", b"param",
    b"source", b"track", b"wbr",
];

/// The language of a tag's body, from the tag and its `lang` or `type`
/// value: the selectors of the grammar's injections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Body {
    JavaScript,
    Json,
    TypeScript,
    Css,
    Less,
    PostCss,
    Sass,
    Scss,
    /// A body no injection Shiki loads applies to, which stays plain.
    Plain,
}

/// The body of a `<script>` tag, when `script`, or of a `<style>` tag, with
/// the `lang` value of its mode, `meta.lang.<lang>.astro`, when one is open.
pub(super) fn body_of(script: bool, lang: Option<&str>) -> Body {
    if script {
        match lang {
            None | Some("js" | "javascript" | "partytown" | "node") => Body::JavaScript,
            Some("json") => Body::Json,
            Some("ts" | "typescript") => Body::TypeScript,
            Some(_) => Body::Plain,
        }
    } else {
        match lang {
            None | Some("css") => Body::Css,
            Some("less") => Body::Less,
            Some("postcss") => Body::PostCss,
            Some("sass") => Body::Sass,
            Some("scss") => Body::Scss,
            Some(_) => Body::Plain,
        }
    }
}

/// What a language tag's `type` or `lang` value must be for a mode of
/// `tags-lang` to open.
#[derive(Clone, Copy, Debug)]
enum LangValue {
    /// `(?:text/)?(application/ld\+json)`.
    Json,
    /// `(module)`.
    Module,
    /// `(?:text/|application/)?([+/\w]+)`, recorded as group 3.
    Named,
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms follow the grammar's order: bodies, comments, tags,
    /// attributes, entities, the frontmatter and text.
    /// It is `#[inline]`, because the scan calls it from `engine`
    /// (`bench/AGENTS.md` rule 13).
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    #[inline]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        if pattern.yields_to_bodies() && self.body_opens(p) {
            return None;
        }
        match pattern {
            Pattern::BodyJavaScript => self.body_opens_in(p, Body::JavaScript),
            Pattern::BodyJson => self.body_opens_in(p, Body::Json),
            Pattern::BodyTypeScript => self.body_opens_in(p, Body::TypeScript),
            Pattern::BodyCss => self.body_opens_in(p, Body::Css),
            Pattern::BodyLess => self.body_opens_in(p, Body::Less),
            Pattern::BodyPostCss => self.body_opens_in(p, Body::PostCss),
            Pattern::BodySass => self.body_opens_in(p, Body::Sass),
            Pattern::BodyScss => self.body_opens_in(p, Body::Scss),
            Pattern::BodyJavaScriptEnd
            | Pattern::BodyJsonEnd
            | Pattern::BodyTypeScriptEnd
            | Pattern::BodyCssEnd
            | Pattern::BodyLessEnd
            | Pattern::BodyPostCssEnd
            | Pattern::BodySassEnd
            | Pattern::BodyScssEnd => self.starts(p, b"</").then_some(p),
            Pattern::LangJsonEnd | Pattern::LangModuleEnd | Pattern::LangNamedEnd => {
                (self.starts(p, b"</") || self.starts(p, b"/>")).then_some(p)
            }

            Pattern::Comments => self.lit(p, b"<!--"),
            Pattern::CommentsEnd => self.lit(p, b"-->"),
            Pattern::CommentsInvalid => self.comment_invalid(p),

            Pattern::TagNameComponent => {
                if !self.at(p).is_ascii_uppercase() || p >= self.lim() {
                    return None;
                }
                Some(self.skip(p + 1, |b| b.is_ascii_alphanumeric() || b == b'_'))
            }
            Pattern::TagNameCustom => {
                if !self.at(p).is_ascii_lowercase() || p >= self.lim() {
                    return None;
                }
                let head = self.skip_chars(p + 1, |c| matches!(c, '0'..=':') || is_word(c));
                if self.at(head) != b'-' || head >= self.lim() {
                    return None;
                }
                Some(self.skip_chars(head + 1, |c| {
                    c == '-' || matches!(c, '0'..=':') || is_word(c)
                }))
            }
            Pattern::TagNamePlain => {
                if !self.at(p).is_ascii_lowercase() || p >= self.lim() {
                    return None;
                }
                Some(self.skip_chars(p + 1, |c| c == '-' || matches!(c, '0'..=':') || is_word(c)))
            }
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
            Pattern::ScriptStartInBody => {
                if !self.prev_is(p, b'>') {
                    return None;
                }
                let name = self.lit(p, b"<")?;
                let end = self.lit(name, b"script")?;
                self.capture(1, p, name);
                self.capture(2, name, end);
                Some(end)
            }
            Pattern::StyleStartInBody => {
                if !self.prev_is(p, b'>') {
                    return None;
                }
                self.lit(p, b"<style")
            }
            Pattern::TagsRaw => self.raw_tag(p),
            Pattern::TagsRawEnd => self.payload_tag_end(p),
            Pattern::TagsLangScript => self.lang_tag(p, b"script"),
            Pattern::TagsLangStyle => self.lang_tag(p, b"style"),
            Pattern::TagsLangScriptEnd => self.lang_tag_end(p, b"script"),
            Pattern::TagsLangStyleEnd => self.lang_tag_end(p, b"style"),
            Pattern::LangJson => self.lang_mode(p, LangValue::Json),
            Pattern::LangModule => self.lang_mode(p, LangValue::Module),
            Pattern::LangNamed => self.lang_mode(p, LangValue::Named),
            Pattern::TagsLangStartAttributes | Pattern::TagsStartAttributes => {
                self.at_g(p).then_some(p)
            }
            Pattern::TagsLangStartAttributesEnd => {
                if self.starts(p, b"/>") {
                    return Some(p);
                }
                self.lit(p, b">")
            }
            Pattern::TagsStartAttributesEnd => {
                (self.starts(p, b">") || self.starts(p, b"/>")).then_some(p)
            }
            Pattern::TagsVoid => {
                let name = self.lit(p, b"<")?;
                let end = VOID_TAGS.iter().find_map(|tag| self.lit(name, tag))?;
                if !(self.space_at(end) > 0 || self.starts(end, b">") || self.starts(end, b"/>")) {
                    return None;
                }
                self.capture(1, p, name);
                self.capture(2, name, end);
                Some(end)
            }
            Pattern::TagsVoidEnd => {
                let start = if self.starts(p, b"/") { p + 1 } else { p };
                self.lit(start, b">")
            }
            Pattern::TagsGeneralEndEnd => self.lit_capture1(p, b">"),
            Pattern::TagsGeneralStartEnd => {
                let start = if self.starts(p, b"/") { p + 1 } else { p };
                let end = self.lit(start, b">")?;
                self.capture(1, p, end);
                Some(end)
            }

            Pattern::NestedAttributeNameAt0 => {
                (p <= self.lim() || self.at_eol(p)).then(|| p.max(self.lim()))
            }
            Pattern::AttributesEvents => self.event_name(p),
            Pattern::AttributesKeyvalue => {
                let end = self.attribute_name(p)?;
                self.capture(1, p, end);
                Some(end)
            }
            Pattern::AttributesEventsEnd | Pattern::AttributesKeyvalueEnd => {
                let next = self.space_end(p);
                (next < self.lim() && self.at(next) != b'=').then_some(p)
            }
            Pattern::EventEquals | Pattern::KeyvalueEquals => self.lit(p, b"="),
            Pattern::EventEqualsEnd | Pattern::KeyvalueEqualsEnd => {
                let ended = p > 0
                    && !self.prev_is(p, b'=')
                    && !self.space_before(p)
                    && self.at(self.space_end(p)) != b'=';
                (ended || self.starts(p, b"/>") || self.starts(p, b">")).then_some(p)
            }
            Pattern::InterpolationAt0 => self.lit(p, b"{"),
            Pattern::InterpolationAt0End => self.lit(p, b"}"),
            Pattern::InterpolationAt0At0 => {
                let start = self.space_end(p);
                (self.at_g(p) && self.starts(start, b"{")).then_some(start)
            }
            Pattern::InterpolationAt0At0End => self.prev_is(p, b'}').then_some(p),
            Pattern::AttributeLiteral => self.lit_capture1(p, b"`"),
            Pattern::AttributeLiteralEnd => self.lit(p, b"`"),
            Pattern::EventScript => {
                let opens = if self.starts(p, b"/") {
                    !self.starts(p + 1, b">")
                } else {
                    self.value_char(p)
                };
                opens.then_some(p)
            }
            Pattern::EventScriptEnd => (!self.at_g(p)).then_some(p),
            Pattern::EventScriptBare => {
                let end = self.unquoted_end(p);
                (end > p).then(|| {
                    self.capture(1, p, end);
                    end
                })
            }
            Pattern::EventScriptDouble => self.lit_capture1(p, b"\""),
            Pattern::EventScriptSingle => self.lit_capture1(p, b"'"),
            Pattern::EventScriptDoubleEnd
            | Pattern::EventScriptSingleEnd
            | Pattern::ValueQuotedEnd => self.payload_at(p).map(|width| p + width),
            Pattern::EventScriptDoubleRun => self.script_run(p, b'"'),
            Pattern::EventScriptSingleRun => self.script_run(p, b'\''),
            Pattern::EventScriptDoubleLineComment | Pattern::EventScriptSingleLineComment => {
                self.lit(p, b"//")
            }
            Pattern::EventScriptDoubleLineCommentEnd => self.line_comment_end(p, b'"'),
            Pattern::EventScriptSingleLineCommentEnd => self.line_comment_end(p, b'\''),
            Pattern::EventScriptDoubleBlockComment | Pattern::EventScriptSingleBlockComment => {
                self.lit(p, b"/*")
            }
            Pattern::EventScriptDoubleBlockCommentEnd => self.block_comment_end(p, b'"'),
            Pattern::EventScriptSingleBlockCommentEnd => self.block_comment_end(p, b'\''),
            Pattern::ValueUnquoted => {
                let end = self.unquoted_end(p);
                (end > p).then_some(end)
            }
            Pattern::ValueQuoted => {
                if !(self.starts(p, b"\"") || self.starts(p, b"'")) {
                    return None;
                }
                self.capture(1, p, p + 1);
                Some(p + 1)
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
            Pattern::AttributesInterpolatedEnd => self.lit_capture1(p, b"}"),

            Pattern::EntityNamed => self.named_entity(p),
            Pattern::EntityDecimal => {
                let digits = self.lit(p, b"&#")?;
                let end = self.skip(digits, |b| b.is_ascii_digit());
                (end > digits && self.starts(end, b";")).then(|| {
                    self.capture(1, p, p + 1);
                    end + 1
                })
            }
            Pattern::EntityHexadecimal => {
                let x = self.lit(p, b"&#")?;
                if !(self.starts(x, b"x") || self.starts(x, b"X")) {
                    return None;
                }
                let end = self.skip(x + 1, |b| b.is_ascii_hexdigit());
                (end > x + 1 && self.starts(end, b";")).then(|| {
                    self.capture(1, p, p + 1);
                    end + 1
                })
            }
            Pattern::EntityAmbiguous => {
                let name = self.lit(p, b"&")?;
                let end = self.skip(name, |b| b.is_ascii_alphanumeric());
                (end > name && self.starts(end, b";")).then_some(name)
            }

            Pattern::Frontmatter => {
                if p != 0 || !self.first_line {
                    return None;
                }
                let dashes = self.lit(p, b"---")?;
                let end = self.space_end(dashes);
                self.at_eol(end).then(|| {
                    self.capture(1, p, dashes);
                    end
                })
            }
            Pattern::FrontmatterEnd => self.frontmatter_end(p),
            Pattern::Text => {
                let after = p == 0
                    || self.prev_is(p, b'>')
                    || self.prev_is(p, b'}')
                    || (p >= 3 && self.text(p - 3, p) == b"---");
                after.then_some(p)
            }
            Pattern::TextEnd => {
                (self.starts(p, b"<") || self.starts(p, b"{") || self.at_eol(p)).then_some(p)
            }
        }
    }

    /// The language of the body that a language tag around the open rules
    /// holds.
    /// It comes from the injections' selectors.
    /// They read the tag, `meta.script.astro` or `meta.style.astro`, and the
    /// mode between it and the innermost rule, `meta.lang.<lang>.astro`,
    /// when one is open.
    /// A rule between them with a `source` scope of its own hides the tag
    /// ([`Rule::hides_bodies`]).
    fn body(&self) -> Body {
        let mut lang = None;
        for (index, rule) in self.open_rules() {
            if rule == Rule::TagsLangScript || rule == Rule::TagsLangStyle {
                return body_of(rule == Rule::TagsLangScript, lang);
            }
            if rule.hides_bodies() {
                return Body::Plain;
            }
            if lang.is_none() {
                lang = if rule == Rule::LangJson {
                    Some("json")
                } else if rule == Rule::LangModule {
                    Some("javascript")
                } else if rule == Rule::LangNamed {
                    Some(self.payload_text_of(index))
                } else {
                    None
                };
            }
        }
        Body::Plain
    }

    /// `(?<=>)(?!</)`, where some injection's selector holds: a body opens.
    fn body_opens(&self, p: usize) -> bool {
        self.prev_is(p, b'>') && !self.starts(p, b"</") && self.body() != Body::Plain
    }

    /// `(?<=>)(?!</)`, where the tag's body is in `body`.
    fn body_opens_in(&self, p: usize, body: Body) -> Option<usize> {
        (self.prev_is(p, b'>') && !self.starts(p, b"</") && self.body() == body).then_some(p)
    }

    /// `\G-?>|<!--(?!>)|<!-(?=-->)|--!>`.
    fn comment_invalid(&self, p: usize) -> Option<usize> {
        if self.at_g(p) {
            let start = if self.starts(p, b"-") { p + 1 } else { p };
            if let Some(end) = self.lit(start, b">") {
                return Some(end);
            }
        }
        if let Some(end) = self.lit(p, b"<!--") {
            return (!self.starts(end, b">")).then_some(end);
        }
        if let Some(end) = self.lit(p, b"<!-")
            && self.starts(end, b"-->")
        {
            return Some(end);
        }
        self.lit(p, b"--!>")
    }

    /// `(text)`, the whole match as group 1.
    fn lit_capture1(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// The end of the run of characters from `i` that `class` accepts.
    fn skip_chars(&self, mut i: usize, class: impl Fn(char) -> bool) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && class(c)
        {
            i = end;
        }
        i
    }

    /// Whether the character before `i` is a `\s`.
    fn space_before(&self, i: usize) -> bool {
        i > 0 && self.space_at(self.char_start_before(i)) > 0
    }

    /// `[^/>\s]*` from `i`.
    fn tag_name_end(&self, mut i: usize) -> usize {
        while i < self.lim() && !matches!(self.at(i), b'/' | b'>') && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        i
    }

    /// `(</)(.*?)\s*(>)|(/>)`.
    fn end_node(&mut self, p: usize) -> Option<usize> {
        if let Some(name) = self.lit(p, b"</")
            && let Some(close) = (name..self.lim()).find(|&q| self.at(q) == b'>')
        {
            let mut trimmed = close;
            while trimmed > name && self.space_before(trimmed) {
                trimmed = self.char_start_before(trimmed);
            }
            self.capture(1, p, name);
            self.capture(2, name, trimmed);
            self.capture(3, close, close + 1);
            return Some(close + 1);
        }
        let end = self.lit(p, b"/>")?;
        self.capture(4, p, end);
        Some(end)
    }

    /// `<([^!/<>?\s]+)(?=[^>]+is:raw).*?`: a tag whose start holds `is:raw`
    /// before its first `>`, with its name as group 1.
    ///
    /// The name gives characters back until an `is:raw` starts at least one
    /// character after it, so it ends at most one character before the
    /// last such `is:raw`.
    /// The lazy `.*?` takes nothing.
    fn raw_tag(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"<")?;
        let mut run = name;
        while run < self.lim()
            && !matches!(self.at(run), b'!' | b'/' | b'<' | b'>' | b'?')
            && self.space_at(run) == 0
        {
            run = self.char_end(run);
        }
        if run == name {
            return None;
        }
        let close = (name..self.lim())
            .find(|&q| self.at(q) == b'>')
            .unwrap_or(self.lim() + 1);
        self.spend((close - name) / 8);
        let last = (name + 1..close)
            .rev()
            .find(|&q| self.starts(q, b"is:raw"))?;
        let mut end = run;
        while end >= last {
            end = self.char_start_before(end);
        }
        (end > name).then(|| {
            self.capture(1, name, end);
            end
        })
    }

    /// `</\1\s*>|/>`, with `\1` the tag's name, the rule's payload.
    fn payload_tag_end(&self, p: usize) -> Option<usize> {
        if let Some(name) = self.lit(p, b"</")
            && let Some(width) = self.payload_at(name)
            && let Some(end) = self.lit(self.space_end(name + width), b">")
        {
            return Some(end);
        }
        self.lit(p, b"/>")
    }

    /// `<(tag)`.
    fn lang_tag(&mut self, p: usize, tag: &[u8]) -> Option<usize> {
        let name = self.lit(p, b"<")?;
        let end = self.lit(name, tag)?;
        self.capture(1, name, end);
        Some(end)
    }

    /// `</tag\s*>|/>`.
    fn lang_tag_end(&self, p: usize, tag: &[u8]) -> Option<usize> {
        if let Some(name) = self.lit(p, b"</")
            && let Some(end) = self.lit(name, tag)
            && let Some(end) = self.lit(self.space_end(end), b">")
        {
            return Some(end);
        }
        self.lit(p, b"/>")
    }

    /// `\G(?=\s*[^>]*?(type|lang)\s*=\s*(["']?)V\2)` with `V` the value
    /// `want` names: the first `type` or `lang` before the next `>` whose
    /// value matches. The named value is group 3.
    fn lang_mode(&mut self, p: usize, want: LangValue) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let mut q = p;
        while self.avail(q) && self.at(q) != b'>' {
            let name = self.lit(q, b"type").or_else(|| self.lit(q, b"lang"));
            if let Some(name) = name
                && let Some((value, end)) = self.lang_value(name, want)
            {
                if matches!(want, LangValue::Named) {
                    self.capture(3, value, end);
                }
                return Some(p);
            }
            q = self.char_end(q).max(q + 1);
        }
        None
    }

    /// `\s*=\s*(["']?)V\2` from `i`: the value's start and end.
    /// The quote is tried before none, as `(["']?)` is greedy.
    fn lang_value(&self, i: usize, want: LangValue) -> Option<(usize, usize)> {
        let eq = self.space_end(i);
        if !self.starts(eq, b"=") {
            return None;
        }
        let open = self.space_end(eq + 1);
        let quote = self.at(open);
        let widths: &[usize] = if matches!(quote, b'"' | b'\'') && open < self.lim() {
            &[1, 0]
        } else {
            &[0]
        };
        widths.iter().find_map(|&width| {
            let start = open + width;
            let closes = |end: usize| width == 0 || (self.at(end) == quote && end < self.lim());
            self.value_spans(start, want)
                .into_iter()
                .flatten()
                .find(|&(_, end)| closes(end))
        })
    }

    /// Where the value `want` names can match from `i`, in the order the
    /// regex tries them: with its optional prefix, then without.
    fn value_spans(&self, i: usize, want: LangValue) -> [Option<(usize, usize)>; 3] {
        match want {
            LangValue::Json => {
                let json = |at: usize| self.lit(at, b"application/ld+json").map(|end| (at, end));
                [self.lit(i, b"text/").and_then(json), json(i), None]
            }
            LangValue::Module => [self.lit(i, b"module").map(|end| (i, end)), None, None],
            LangValue::Named => {
                let named = |at: usize| {
                    let end = self.skip_chars(at, |c| c == '+' || c == '/' || is_word(c));
                    (end > at).then_some((at, end))
                };
                [
                    self.lit(i, b"text/").and_then(named),
                    self.lit(i, b"application/").and_then(named),
                    named(i),
                ]
            }
        }
    }

    /// ``[^/<=>`\s]`` at `i`, a character that can stand in an unquoted value
    /// besides `/`, and is not a quote.
    fn value_char(&self, i: usize) -> bool {
        i < self.lim()
            && !matches!(self.at(i), b'/' | b'<' | b'=' | b'>' | b'`')
            && self.space_at(i) == 0
    }

    /// ``([^"'/<=>`\s]|/(?!>))+`` from `i`: the end of an unquoted value.
    fn unquoted_end(&self, mut i: usize) -> usize {
        loop {
            if self.starts(i, b"/") {
                if self.starts(i + 1, b">") {
                    return i;
                }
                i += 1;
            } else if self.value_char(i) && !matches!(self.at(i), b'"' | b'\'') {
                i = self.char_end(i);
            } else {
                return i;
            }
        }
    }

    /// `([^\n"/]|/(?![*/]))+` with `quote` for `"`: a run of an event
    /// handler's quoted value, up to a comment or the quote.
    fn script_run(&self, p: usize, quote: u8) -> Option<usize> {
        let mut i = p;
        while i < self.lim() {
            let b = self.at(i);
            if b == quote || (b == b'/' && matches!(self.at(i + 1), b'*' | b'/')) {
                break;
            }
            i = self.char_end(i);
        }
        (i > p).then_some(i)
    }

    /// `(?=Q)|\n` with `Q` the quote: a `//` comment ends before the quote,
    /// or takes the line feed.
    fn line_comment_end(&self, p: usize, quote: u8) -> Option<usize> {
        if p < self.lim() && self.at(p) == quote {
            return Some(p);
        }
        (p == self.lim() && self.avail(p)).then_some(p + 1)
    }

    /// `(?=Q)|\*/` with `Q` the quote.
    fn block_comment_end(&self, p: usize, quote: u8) -> Option<usize> {
        if p < self.lim() && self.at(p) == quote {
            return Some(p);
        }
        self.lit(p, b"*/")
    }

    /// `(on(…))(?![-:\\w])`: `on` and one of html's event names, with the
    /// name and `on` as group 1.
    ///
    /// The lookahead's class escapes its backslash, so it holds `-`, `:`,
    /// `\` and `w`.
    /// Its optional tails are greedy, so the alternation tries longer names
    /// first.
    /// It gives a name back when the lookahead fails after it.
    /// So `onplayingw` is `onplay`, since `i` may follow a name.
    fn event_name(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"on")?;
        let rest = self.text(name, self.lim());
        let mut best: Option<usize> = None;
        for event in EVENT_HANDLERS {
            let word = event.as_bytes();
            if !rest.starts_with(word) || best.is_some_and(|len| len >= word.len()) {
                continue;
            }
            if !matches!(self.at(name + word.len()), b'-' | b':' | b'\\' | b'w') {
                best = Some(word.len());
            }
        }
        let end = name + best?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `([$@_[:alpha:]][-$.:_[:alnum:]]*)` at `p`.
    fn attribute_name(&self, p: usize) -> Option<usize> {
        let (c, first) = self.char_at(p)?;
        if p >= self.lim() || !(c == '$' || c == '@' || c == '_' || is_alpha(c)) {
            return None;
        }
        Some(self.skip_chars(first, |c| {
            matches!(c, '-' | '$' | '.' | ':' | '_') || is_alnum(c)
        }))
    }

    /// `(&)(?=[A-Za-z])(NAME)(;)` with `NAME` one of html's entity names,
    /// all letters and digits: the run of them up to a `;`.
    fn named_entity(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"&")?;
        if !self.at(name).is_ascii_alphabetic() || name >= self.lim() {
            return None;
        }
        let end = self.skip(name, |b| b.is_ascii_alphanumeric());
        self.spend((end - name) / 8);
        if !self.starts(end, b";") {
            return None;
        }
        let word = self.text(name, end);
        ENTITIES
            .binary_search_by(|entity| crate::bytes::order(entity.as_bytes(), word))
            .ok()?;
        self.capture(1, p, name);
        self.capture(2, name, end);
        self.capture(3, end, end + 1);
        Some(end + 1)
    }

    /// `(^|\G)(-{3})|\.{3}\s*$`: the frontmatter's closing `---` at the
    /// start of a line or where the TypeScript ended, as group 2, or `...`
    /// before the line's end.
    fn frontmatter_end(&mut self, p: usize) -> Option<usize> {
        if (p == 0 || self.at_g(p))
            && let Some(end) = self.lit(p, b"---")
        {
            self.capture(2, p, end);
            return Some(end);
        }
        let dots = self.lit(p, b"...")?;
        let end = self.space_end(dots);
        self.at_eol(end).then_some(end)
    }
}
