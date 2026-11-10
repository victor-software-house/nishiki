//! The rules of the `vue` grammar that Shiki bundles, from
//! vuejs/language-tools.
//! They cover a single-file component's blocks, the tags of its template
//! that Vue names itself, and their directives.
//!
//! The grammar's injection grammars (`vue-directives`, `vue-interpolations`
//! and the style variable injection) do not apply in Shiki's output.
//! So `{{ x }}` in a template is text, and a directive on an HTML tag is an
//! HTML attribute.
//! Directives apply only in the tags of Vue's own rules.
//! Those are a block's start tag, a nested `<template>`, and a tag whose name
//! starts with a capital.

use memchr::memchr;

use super::engine::{Hit, Lexer, Pattern};
use super::scope::S;
use super::state::Rule;
use crate::classes::is_word;
#[cfg(feature = "lang-vue")]
use crate::typescript::Dialect;

/// A language that a block names in its `lang` attribute, in the order the
/// grammar tries them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Lang {
    Md,
    Html,
    Pug,
    Stylus,
    Postcss,
    Sass,
    Css,
    Scss,
    Less,
    Js,
    Ts,
    Jsx,
    Tsx,
    Coffee,
    Json,
    Jsonc,
    Json5,
    Yaml,
    Toml,
    Graphql,
    Vue,
}

impl Lang {
    /// Every language, in the grammar's order.
    const ALL: [Lang; 21] = [
        Lang::Md,
        Lang::Html,
        Lang::Pug,
        Lang::Stylus,
        Lang::Postcss,
        Lang::Sass,
        Lang::Css,
        Lang::Scss,
        Lang::Less,
        Lang::Js,
        Lang::Ts,
        Lang::Jsx,
        Lang::Tsx,
        Lang::Coffee,
        Lang::Json,
        Lang::Jsonc,
        Lang::Json5,
        Lang::Yaml,
        Lang::Toml,
        Lang::Graphql,
        Lang::Vue,
    ];

    /// The values of `lang` that name the language.
    fn words(self) -> &'static [&'static [u8]] {
        match self {
            Lang::Md => &[b"md"],
            Lang::Html => &[b"html"],
            Lang::Pug => &[b"pug"],
            Lang::Stylus => &[b"stylus"],
            Lang::Postcss => &[b"postcss"],
            Lang::Sass => &[b"sass"],
            Lang::Css => &[b"css"],
            Lang::Scss => &[b"scss"],
            Lang::Less => &[b"less"],
            Lang::Js => &[b"js"],
            Lang::Ts => &[b"ts"],
            Lang::Jsx => &[b"jsx"],
            Lang::Tsx => &[b"tsx"],
            Lang::Coffee => &[b"coffee"],
            Lang::Json => &[b"json"],
            Lang::Jsonc => &[b"jsonc"],
            Lang::Json5 => &[b"json5"],
            Lang::Yaml => &[b"yaml"],
            Lang::Toml => &[b"toml"],
            Lang::Graphql => &[b"gql", b"graphql"],
            Lang::Vue => &[b"vue"],
        }
    }

    /// The language a payload's bits name.
    pub(crate) fn from_bits(bits: u32) -> Lang {
        usize::try_from(bits)
            .ok()
            .and_then(|index| Lang::ALL.get(index).copied())
            .unwrap_or(Lang::Js)
    }

    /// The language's bits in a payload.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn bits(self) -> u32 {
        self as u32
    }

    /// Whether the grammar of the language exists here.
    /// nishiki does not lex Pug, Stylus, `PostCSS` or JSON5, and lexes Sass
    /// and `CoffeeScript` only with their features. vscode-textmate skips a
    /// rule whose only pattern includes a missing grammar, so such a block
    /// opens no body: its text stays the block's.
    pub(crate) const fn has_grammar(self) -> bool {
        match self {
            Lang::Pug | Lang::Stylus | Lang::Postcss | Lang::Json5 => false,
            Lang::Sass => cfg!(feature = "lang-sass"),
            Lang::Coffee => cfg!(feature = "lang-coffee"),
            Lang::Md
            | Lang::Html
            | Lang::Css
            | Lang::Scss
            | Lang::Less
            | Lang::Js
            | Lang::Ts
            | Lang::Jsx
            | Lang::Tsx
            | Lang::Json
            | Lang::Jsonc
            | Lang::Yaml
            | Lang::Toml
            | Lang::Graphql
            | Lang::Vue => true,
        }
    }

    /// Whether its body in a block ends at `</script`, as TypeScript's and
    /// TSX's do, not at any `</`.
    pub(crate) const fn ends_at_script(self) -> bool {
        matches!(self, Lang::Ts | Lang::Tsx)
    }
}

/// What a block's tag name opened: a block whose `lang` names a language,
/// or one of the grammar's named blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Block {
    /// A block whose `lang` attribute names a language.
    Lang(Lang),
    /// `<template>`.
    Template,
    /// `<script>`.
    Script,
    /// `<style>`.
    Style,
    /// Any other tag.
    Other,
}

impl Block {
    /// The block's bits in a payload.
    pub(crate) fn bits(self) -> u32 {
        match self {
            Block::Lang(lang) => lang.bits(),
            Block::Template => 32,
            Block::Script => 33,
            Block::Style => 34,
            Block::Other => 35,
        }
    }

    /// The block a payload's bits name.
    pub(crate) fn from_bits(bits: u32) -> Block {
        match bits {
            32 => Block::Template,
            33 => Block::Script,
            34 => Block::Style,
            35 => Block::Other,
            lang => Block::Lang(Lang::from_bits(lang)),
        }
    }
}

/// Which element's end a multi-line block's rules stop at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Element {
    Script,
    Style,
}

/// The directive rule that matched, which names its scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Directive {
    /// `v-for`, `v-if`, `v-else-if` or `v-else`.
    Control,
    /// Any other `v-` name, or `:`, `.`, `@` or `#`.
    Original,
}

/// A match of one of Vue's patterns.
///
/// It holds no data, so `Pattern` keeps a plain tag byte.
/// The block, language or template tag of a match is in its `Hit::bits`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VuePattern {
    /// `<!-- @name`: a comment that holds JSON.
    CommentKv,
    /// `<!--`.
    Comment,
    /// `<name` of a tag that its line closes with `/>`.
    SelfClosing,
    /// `<` of a block.
    Block,
    /// A block's tag name; its bits are the `Block`'s.
    Element,
    /// The `lang` attribute of a block whose start tag spans lines; its bits
    /// are the `Lang`'s.
    LangGroup,
    /// `<template` of a nested template; its bits are the `TemplateTag`'s.
    TemplateTag,
    /// The inside of a nested template, from its start tag; its bits are
    /// the `TemplateTag`'s.
    TemplateTagEnd,
    /// `<Name` of a tag whose name starts with a capital.
    Capitalized,
    /// The name of a directive the grammar's control rule takes, such as
    /// `v-if`.
    DirectiveControl,
    /// The name of any other directive.
    /// Its hit's fifth slot holds the [`Argument`]'s code.
    DirectiveOriginal,
    /// `.name`: a directive's modifier.
    Modifier,
    /// `=` and the quote of a directive's value.
    ExprString,
    /// `=` before a directive's value that no quote opens.
    ExprBare,
}

/// What follows a directive's name: no argument, `[expression]`, or a plain
/// name.
/// The directive's [`Hit`] carries it in its fifth slot, as
/// [`Argument::code`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Argument {
    /// No argument.
    Absent,
    /// A dynamic argument in brackets, such as `[key]`.
    Bracketed,
    /// A plain argument, such as `click` in `@click`.
    Plain,
}

impl Argument {
    /// The argument as a [`Hit`] slot holds it.
    pub(super) const fn code(self) -> usize {
        match self {
            Argument::Absent => 0,
            Argument::Bracketed => 1,
            Argument::Plain => 2,
        }
    }

    /// The argument that [`Argument::code`] gave `code`.
    pub(super) const fn from_code(code: usize) -> Argument {
        match code {
            1 => Argument::Bracketed,
            2 => Argument::Plain,
            _ => Argument::Absent,
        }
    }
}

/// Which of the grammar's two rules a nested `<template>` opened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TemplateTag {
    /// `(<)(template)\b(>)`: a start tag with no attributes.
    Bare,
    /// `(<)(template)(?=\s|/?>)`: a start tag that may hold attributes.
    Attributes,
}

impl TemplateTag {
    pub(crate) const fn bits(self) -> u32 {
        match self {
            TemplateTag::Bare => 1,
            TemplateTag::Attributes => 2,
        }
    }

    pub(crate) const fn from_bits(bits: u32) -> TemplateTag {
        if bits == 1 {
            TemplateTag::Bare
        } else {
            TemplateTag::Attributes
        }
    }
}

/// A byte of `[-0-:A-Za-z]`, the grammar's tag name characters.
fn is_name_byte(b: u8) -> bool {
    b == b'-' || (b'0'..=b':').contains(&b) || b.is_ascii_alphabetic()
}

/// Whether the character at `i` of `s` is a word character, as `\b` reads
/// it.
fn word_at(s: &[u8], i: usize) -> bool {
    crate::utf8::char_at(s, i).is_some_and(|(c, _)| is_word(c))
}

/// Whether the character that ends at `i` of `s` is a word character.
fn word_before(s: &[u8], i: usize) -> bool {
    if i == 0 {
        return false;
    }
    let start = crate::utf8::char_start(s, i - 1);
    word_at(s, start)
}

/// `\b` at `i` of `s`.
fn boundary(s: &[u8], i: usize) -> bool {
    word_before(s, i) != word_at(s, i)
}

/// The end of `[ \t…]*` from `i`: the blanks `\s` takes on a line.
pub(super) fn blanks_end(s: &[u8], mut i: usize) -> usize {
    while s.get(i).is_some_and(|&b| crate::classes::is_ascii_space(b)) {
        i += 1;
    }
    i
}

/// The value of a `lang` attribute at `i` of `s`, when `\blang\s*=\s*`
/// starts there: where the value starts, and the quote before it, if any.
fn lang_value(s: &[u8], i: usize) -> Option<(usize, Option<u8>)> {
    if !s.get(i..).is_some_and(|rest| rest.starts_with(b"lang")) || word_before(s, i) {
        return None;
    }
    let q = blanks_end(s, i + 4);
    if s.get(q) != Some(&b'=') {
        return None;
    }
    let q = blanks_end(s, q + 1);
    match s.get(q) {
        Some(&quote @ (b'"' | b'\'')) => Some((q + 1, Some(quote))),
        Some(_) | None => Some((q, None)),
    }
}

/// Whether `word` starts at `at` of `s` with `\b` after it.
fn word_ends(s: &[u8], at: usize, word: &[u8]) -> Option<usize> {
    let end = at + word.len();
    (s.get(at..end) == Some(word) && boundary(s, end)).then_some(end)
}

/// Which tag a block's `lang` lookahead runs in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlockTag {
    /// `<template>`, whose `lang="html"` is the template's own rule, so the
    /// lookahead skips HTML.
    Template,
    /// Any other tag.
    Other,
}

/// The first of `langs`, in their order, that the lookahead
/// `[^>]*\blang\s*=\s*(["']?)LANG\b\2` finds from `from` of `s`, or none.
/// In the lookahead, a language's words follow the quote, and the same quote
/// follows them.
/// `[^>]*` stops at the first `>`, so the attribute lies before it.
fn lang_ahead(s: &[u8], from: usize, tag: BlockTag) -> Option<Lang> {
    let stop = memchr(b'>', s.get(from..).unwrap_or_default()).map_or(s.len(), |at| at + from);
    let window = s.get(from..stop).unwrap_or_default();
    Lang::ALL.into_iter().find(|&lang| {
        if tag == BlockTag::Template && lang == Lang::Html {
            return false;
        }
        let mut at = 0;
        while let Some(found) = memchr(b'l', window.get(at..).unwrap_or_default()) {
            let i = from + at + found;
            at += found + 1;
            let Some((value, quote)) = lang_value(s, i) else {
                continue;
            };
            let named = lang.words().iter().any(|word| {
                word_ends(s, value, word).is_some_and(|end| match quote {
                    Some(quote) => s.get(end) == Some(&quote),
                    None => true,
                })
            });
            if named {
                return true;
            }
        }
        false
    })
}

/// The lookahead `(?=\blang\s*=\s*["']?WORD\b)` at `i` of `s` for one of
/// `words`.
fn lang_at(s: &[u8], i: usize, words: &[&[u8]]) -> bool {
    lang_value(s, i)
        .is_some_and(|(value, _)| words.iter().any(|word| word_ends(s, value, word).is_some()))
}

/// The `lang` values that a multi-line `<script>` block hands to its own
/// rules: `tsx??|jsx|coffee`.
const SCRIPT_LANGS: &[&[u8]] = &[b"ts", b"tsx", b"jsx", b"coffee"];

/// The `lang` values that a multi-line `<style>` block hands to its own
/// rules.
const STYLE_LANGS: &[&[u8]] = &[b"scss", b"stylus", b"less", b"postcss"];

/// `</` at `p` of `s`: the end of a block's body.
pub(super) fn body_end(s: &[u8], p: usize) -> bool {
    s.get(p..).is_some_and(|rest| rest.starts_with(b"</"))
}

/// `</name` followed by `>` or white space at `p` of `s`.
/// The end of the line counts as white space: Shiki's lines end in a line feed.
fn end_tag(s: &[u8], p: usize, name: &[u8]) -> bool {
    let after = p + 2 + name.len();
    body_end(s, p)
        && s.get(p + 2..after) == Some(name)
        && s.get(after)
            .is_none_or(|&b| b == b'>' || crate::classes::is_ascii_space(b))
}

/// `</script[>\s]` at `p` of `s`.
pub(super) fn script_end(s: &[u8], p: usize) -> bool {
    end_tag(s, p, b"script")
}

/// `</style[>\s]` at `p` of `s`.
pub(super) fn style_end(s: &[u8], p: usize) -> bool {
    end_tag(s, p, b"style")
}

/// `</template[>\s]` at `p` of `s`.
pub(super) fn template_end(s: &[u8], p: usize) -> bool {
    end_tag(s, p, b"template")
}

/// `"` at `p` of `s`: the end of a directive's value in double quotes.
pub(super) fn double_quote(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'"')
}

/// `'` at `p` of `s`.
pub(super) fn single_quote(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'\'')
}

/// A backtick at `p` of `s`.
pub(super) fn backtick(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'`')
}

/// `(?=([>\s]|/>))` at `p` of `s`: the end of a directive's value that no
/// quote opened.
pub(super) fn bare_end(s: &[u8], p: usize) -> bool {
    match s.get(p) {
        Some(&b'>') | None => true,
        Some(&b'/') => s.get(p + 1) == Some(&b'>'),
        Some(&b) => crate::classes::is_ascii_space(b),
    }
}

impl Lexer<'_> {
    /// The end of the run of `[-0-:A-Za-z]` from `p`.
    fn name_run(&self, p: usize) -> usize {
        let mut q = p;
        while q < self.lim && is_name_byte(self.s[q]) {
            q += 1;
        }
        self.spend((q - p) / 8);
        q
    }

    /// The patterns of a component's root: a comment, a tag that its line
    /// closes with `/>`, or the `<` of a block.
    #[cold]
    #[inline(never)]
    pub(super) fn vue_root_patterns(&self, p: usize) -> Option<Hit> {
        if self.at(p) != b'<' {
            return None;
        }
        if self.starts(p, b"<!--") {
            let at = self.skip_ascii_ws(p + 4);
            if self.at(at) == b'@' {
                let mut end = at + 1;
                while let Some((c, n)) = self.char_at(end) {
                    if c == '$' || is_word(c) {
                        end += n;
                    } else {
                        break;
                    }
                }
                if end > at + 1 && self.ws_len(end) > 0 {
                    return Some(
                        Hit::new(Pattern::Vue(VuePattern::CommentKv), p, end)
                            .with([at, at + 1, 0, 0, 0], 0),
                    );
                }
            }
            return Some(Hit::new(Pattern::Vue(VuePattern::Comment), p, p + 4));
        }
        let name_end = self.name_run(p + 1);
        if name_end > p + 1 {
            let window = self.s.get(name_end..self.lim).unwrap_or_default();
            if let Some(gt) = memchr(b'>', window).map(|at| at + name_end)
                && gt >= name_end + 2
                && self.s[gt - 1] == b'/'
            {
                return Some(
                    Hit::new(Pattern::Vue(VuePattern::SelfClosing), p, name_end)
                        .with([p + 1, name_end, 0, 0, 0], 0),
                );
            }
        }
        Some(Hit::new(Pattern::Vue(VuePattern::Block), p, p + 1))
    }

    /// The patterns of a block after its `<`: a block whose `lang` names a
    /// language, `template`, `script`, `style`, or any other tag.
    #[inline(never)]
    pub(super) fn vue_block_patterns(&self, p: usize) -> Option<Hit> {
        let run = self.name_run(p);
        if run == p {
            return None;
        }
        let text = &self.s[..self.lim];
        let element = |block: Block, end: usize| {
            Hit::new(Pattern::Vue(VuePattern::Element), p, end)
                .with([p, end, 0, 0, 0], block.bits())
        };
        if let Some(end) = (p + 1..=run).rev().find(|&end| boundary(text, end)) {
            let tag = if self.s.get(p..run) == Some(b"template") {
                BlockTag::Template
            } else {
                BlockTag::Other
            };
            if let Some(lang) = lang_ahead(text, end, tag) {
                return Some(element(Block::Lang(lang), end));
            }
        }
        let named = |name: &[u8]| {
            let end = p + name.len();
            (self.s.get(p..end) == Some(name)
                && (self.ws_len(end) > 0 || self.at(end) == b'>' || self.starts(end, b"/>")))
            .then_some(end)
        };
        if let Some(end) = named(b"template") {
            return Some(element(Block::Template, end));
        }
        if let Some(end) = named(b"script") {
            return Some(element(Block::Script, end));
        }
        if let Some(end) = named(b"style") {
            return Some(element(Block::Style, end));
        }
        Some(element(Block::Other, run))
    }

    /// `(</)(\1)\s*(?=>)` at `p`: the end tag of the block named by the
    /// interned `word`.
    #[inline(never)]
    pub(super) fn vue_element_end(&self, p: usize, word: u32) -> Option<Hit> {
        if !self.starts(p, b"</") {
            return None;
        }
        let name = self.word(word).as_bytes();
        let name_end = p + 2 + name.len();
        if self.s.get(p + 2..name_end) != Some(name) {
            return None;
        }
        let gt = self.skip_ascii_ws(name_end);
        (self.at(gt) == b'>')
            .then(|| Hit::new(Pattern::End, p, gt).with([p + 2, name_end, 0, 0, 0], 0))
    }

    /// `(?<=>)` at `p`: a body opens right after the start tag's `>`.
    pub(super) fn after_gt(&self, p: usize) -> bool {
        p > 0 && self.s.get(p - 1) == Some(&b'>')
    }

    /// The patterns of a block's element rule, after its tag name.
    #[inline(never)]
    pub(super) fn vue_element_patterns(&self, p: usize, block: Block) -> Option<Hit> {
        let zero = |rule: Rule| Some(Hit::new(Pattern::Zero(rule), p, p));
        match block {
            Block::Script => self
                .g(p)
                .then(|| Hit::new(Pattern::Zero(Rule::VueScriptStuff), p, p)),
            Block::Style => self
                .g(p)
                .then(|| Hit::new(Pattern::Zero(Rule::VueStyleStuff), p, p)),
            Block::Lang(Lang::Html) => self
                .g(p)
                .then(|| Hit::new(Pattern::Zero(Rule::VueTagStuff), p, p)),
            Block::Lang(lang) => {
                if self.g(p) {
                    return zero(Rule::VueTagStuff);
                }
                if !self.after_gt(p) || !lang.has_grammar() {
                    return None;
                }
                if lang.ends_at_script() && !self.script_end_ahead(p) {
                    return zero(Rule::VueBodyWhile);
                }
                zero(Rule::VueBody)
            }
            Block::Template => {
                if self.g(p) {
                    return zero(Rule::VueTagStuff);
                }
                self.after_gt(p)
                    .then(|| Hit::new(Pattern::Zero(Rule::VueTemplateBody), p, p))
            }
            Block::Other => {
                if self.g(p) {
                    return zero(Rule::VueTagStuff);
                }
                self.after_gt(p)
                    .then(|| Hit::new(Pattern::Zero(Rule::VueBody), p, p))
            }
        }
    }

    /// `(?=[^\n]*</script[>\s])` at `p`: the line closes the script.
    fn script_end_ahead(&self, p: usize) -> bool {
        let text = &self.s[..self.lim];
        let mut at = p;
        while let Some(found) = memchr(b'<', text.get(at..).unwrap_or_default()) {
            if script_end(text, at + found) {
                return true;
            }
            at += found + 1;
        }
        false
    }

    /// The patterns of a multi-line `<script>` or `<style>` block, after its
    /// tag name:
    /// - the start tag's attributes up to a `lang` that names one of the
    ///   block's own languages;
    /// - the rule that such a `lang` opens;
    /// - the default body.
    #[inline(never)]
    pub(super) fn vue_stuff_patterns(&self, p: usize, element: Element) -> Option<Hit> {
        let text = &self.s[..self.lim];
        let langs = match element {
            Element::Script => SCRIPT_LANGS,
            Element::Style => STYLE_LANGS,
        };
        let tag_stuff = match element {
            Element::Script => Rule::VueScriptTagStuff,
            Element::Style => Rule::VueStyleTagStuff,
        };
        if self.g(p) && !lang_at(text, p, langs) {
            return Some(Hit::new(Pattern::Zero(tag_stuff), p, p));
        }
        let group: &[Lang] = match element {
            Element::Script => &[Lang::Ts, Lang::Tsx, Lang::Jsx, Lang::Coffee],
            Element::Style => &[Lang::Scss, Lang::Stylus, Lang::Less, Lang::Postcss],
        };
        if let Some(&lang) = group.iter().find(|lang| lang_at(text, p, lang.words())) {
            return Some(
                Hit::new(Pattern::Vue(VuePattern::LangGroup), p, p).with([0; 5], lang.bits()),
            );
        }
        self.after_gt(p)
            .then(|| Hit::new(Pattern::Zero(Rule::VueBody), p, p))
    }

    /// The patterns of the rule that a `lang` opens in a multi-line block:
    /// the start tag's attributes, then the body.
    #[inline(never)]
    pub(super) fn vue_lang_group_patterns(&self, p: usize, lang: Lang) -> Option<Hit> {
        if self.g(p) {
            return Some(Hit::new(Pattern::Zero(Rule::VueTagStuff), p, p));
        }
        if !self.after_gt(p) || !lang.has_grammar() {
            return None;
        }
        if lang.ends_at_script() && !self.script_end_ahead(p) {
            return Some(Hit::new(Pattern::Zero(Rule::VueBodyWhile), p, p));
        }
        Some(Hit::new(Pattern::Zero(Rule::VueBody), p, p))
    }

    /// The end `(?=\blang\s*=\s*["']?(…)\b)|(>)` of a multi-line block's
    /// attributes, at `p`.
    #[inline(never)]
    pub(super) fn vue_stuff_tag_end(&self, p: usize, element: Element) -> Option<Hit> {
        let langs = match element {
            Element::Script => SCRIPT_LANGS,
            Element::Style => STYLE_LANGS,
        };
        if lang_at(&self.s[..self.lim], p, langs) {
            return Some(Hit::new(Pattern::End, p, p));
        }
        (self.at(p) == b'>').then(|| Hit::new(Pattern::End, p, p + 1))
    }

    /// The patterns of a tag's attributes in Vue's rules: the directives,
    /// then the HTML attribute rules.
    #[inline(never)]
    pub(super) fn vue_attribute_patterns(&self, p: usize) -> Option<Hit> {
        self.vue_directive(p).or_else(|| self.html_attribute(p))
    }

    /// A directive's name at `p`.
    fn vue_directive(&self, p: usize) -> Option<Hit> {
        let b = self.at(p);
        if b == b'v' && self.starts(p, b"v-") {
            for (word, loop_word) in [
                (&b"v-for"[..], true),
                (b"v-else-if", false),
                (b"v-if", false),
                (b"v-else", false),
            ] {
                let end = p + word.len();
                if self.starts(p, word)
                    && (matches!(self.at(end), b')' | b'/' | b'=' | b'>' | b'\n')
                        || self.ws_len(end) > 0)
                {
                    return Some(
                        Hit::new(Pattern::Vue(VuePattern::DirectiveControl), p, end)
                            .with([0; 5], u32::from(loop_word)),
                    );
                }
            }
        }
        let (name_end, colon_end) = match b {
            b'v' if self.starts(p, b"v-") => {
                let mut q = p + 2;
                while let Some((c, n)) = self.char_at(q) {
                    if c == '-' || is_word(c) {
                        q += n;
                    } else {
                        break;
                    }
                }
                if q == p + 2 {
                    return None;
                }
                let colon = if self.at(q) == b':' { q + 1 } else { q };
                (q, colon)
            }
            b'.' | b':' | b'@' | b'#' => (p + 1, p + 1),
            _ => return None,
        };
        let mut end = colon_end;
        let mut argument = (0, 0, Argument::Absent);
        if self.at(end) == b'[' {
            let close = memchr(b']', self.s.get(end + 1..self.lim).unwrap_or_default())
                .map(|at| at + end + 1);
            if let Some(close) = close {
                argument = (end, close, Argument::Bracketed);
                end = close + 1;
            }
        }
        if argument.2 == Argument::Absent {
            let mut q = end;
            while let Some((c, n)) = self.char_at(q) {
                if c == '-' || is_word(c) {
                    q += n;
                } else {
                    break;
                }
            }
            if q > end {
                argument = (end, q, Argument::Plain);
                end = q;
            }
        }
        Some(
            Hit::new(Pattern::Vue(VuePattern::DirectiveOriginal), p, end).with(
                [
                    name_end,
                    colon_end,
                    argument.0,
                    argument.1,
                    argument.2.code(),
                ],
                u32::from(b),
            ),
        )
    }

    /// The end `(?=\s*[^=\s])` of a directive at `p`.
    #[inline(never)]
    pub(super) fn vue_directive_end(&self, p: usize) -> Option<Hit> {
        let q = self.skip_ws(p);
        (q < self.lim && self.at(q) != b'=').then(|| Hit::new(Pattern::End, p, p))
    }

    /// The patterns of a directive: a modifier `(\.)([-\w]*)` of the
    /// grammar's other directives, then the value rules.
    #[inline(never)]
    pub(super) fn vue_directive_patterns(&self, p: usize, directive: Directive) -> Option<Hit> {
        let b = self.at(p);
        if directive == Directive::Original && b == b'.' {
            let mut q = p + 1;
            while let Some((c, n)) = self.char_at(q) {
                if c == '-' || is_word(c) {
                    q += n;
                } else {
                    break;
                }
            }
            return Some(Hit::new(Pattern::Vue(VuePattern::Modifier), p, q));
        }
        if b != b'=' {
            return None;
        }
        let q = self.skip_ws(p + 1);
        match self.at(q) {
            quote @ (b'"' | b'\'' | b'`') => Some(
                Hit::new(Pattern::Vue(VuePattern::ExprString), p, q + 1)
                    .with([q, 0, 0, 0, 0], u32::from(quote)),
            ),
            _ if q < self.lim => Some(Hit::new(Pattern::Vue(VuePattern::ExprBare), p, p + 1)),
            _ => None,
        }
    }

    /// Vue's own patterns in a template's HTML: a nested `<template>` and a
    /// capitalized tag.
    /// html-derivative's root patterns follow them (`patterns_at`).
    #[inline(never)]
    pub(super) fn vue_html_stuff(&self, p: usize) -> Option<Hit> {
        if self.at(p) == b'<' {
            if self.starts(p + 1, b"template") {
                let end = p + 9;
                if self.at(end) == b'>' {
                    return Some(
                        Hit::new(Pattern::Vue(VuePattern::TemplateTag), p, end + 1)
                            .with([0; 5], TemplateTag::Bare.bits()),
                    );
                }
                if self.ws_len(end) > 0 || self.starts(end, b"/>") {
                    return Some(
                        Hit::new(Pattern::Vue(VuePattern::TemplateTag), p, end)
                            .with([0; 5], TemplateTag::Attributes.bits()),
                    );
                }
            }
            if self.at(p + 1).is_ascii_uppercase() {
                let end = self.name_run(p + 2);
                if self.ws_len(end) > 0 || self.at(end) == b'>' || self.starts(end, b"/>") {
                    return Some(
                        Hit::new(Pattern::Vue(VuePattern::Capitalized), p, end)
                            .with([p + 1, end, 0, 0, 0], 0),
                    );
                }
            }
        }
        None
    }

    /// The end `(?=/>)|((</)(template)(?=[>\s]))` of a nested template's
    /// inside, at `p`.
    #[inline(never)]
    pub(super) fn vue_template_tag_end(&self, p: usize) -> Option<Hit> {
        if self.starts(p, b"/>") {
            return Some(Hit::new(Pattern::End, p, p));
        }
        let end = p + 10;
        (self.starts(p, b"</template") && (self.at(end) == b'>' || self.ws_len(end) > 0))
            .then(|| Hit::new(Pattern::End, p, end))
    }

    /// The patterns of a nested template's inside.
    /// They are its attributes after the tag name when the start tag may hold
    /// them, then the template's HTML.
    /// The patterns at `p`, in a Vue component, of a rule that shares the
    /// root's arm.
    /// Such a rule is Vue's root, a template's body, a nested template's
    /// inside, or a block's element.
    #[cold]
    #[inline(never)]
    pub(super) fn vue_template_patterns(&self, rule: Rule, p: usize) -> Template {
        if rule == Rule::Root {
            return Template::Done(self.vue_root_patterns(p));
        }
        let Some(rule) = rule.vue() else {
            return Template::Root;
        };
        let tag_stuff_first = match rule {
            VueRule::Element => {
                let block = Block::from_bits(super::state::payload_bits(self.top().payload));
                if block != Block::Lang(Lang::Html) {
                    return Template::Done(self.vue_element_patterns(p, block));
                }
                true
            }
            VueRule::TemplateTagEnd => {
                TemplateTag::from_bits(self.top().payload) == TemplateTag::Attributes
            }
            VueRule::TemplateBody => false,
            VueRule::Comment
            | VueRule::CommentKv
            | VueRule::SelfClosing
            | VueRule::Block
            | VueRule::TagStuff
            | VueRule::ScriptTagStuff
            | VueRule::StyleTagStuff
            | VueRule::ScriptStuff
            | VueRule::StyleStuff
            | VueRule::LangGroup
            | VueRule::Body
            | VueRule::BodyWhile
            | VueRule::TemplateTag
            | VueRule::Capitalized
            | VueRule::CapitalizedAttrs
            | VueRule::Directive
            | VueRule::ExprString
            | VueRule::ExprBare
            | VueRule::Expr => return Template::Done(None),
        };
        if tag_stuff_first && self.g(p) {
            return Template::Done(Some(Hit::new(Pattern::Zero(Rule::VueTagStuff), p, p)));
        }
        self.vue_html_stuff(p)
            .map_or(Template::Root, |hit| Template::Done(Some(hit)))
    }

    /// The end pattern of a Vue rule at `p`.
    #[cold]
    #[inline(never)]
    pub(super) fn vue_end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let rule = rule.vue()?;
        let b = self.at(p);
        let one = |len: usize| Hit::new(Pattern::End, p, p + len);
        let zero = || Hit::new(Pattern::End, p, p);
        match rule {
            VueRule::Comment | VueRule::CommentKv => self.starts(p, b"-->").then(|| one(3)),
            VueRule::SelfClosing => self.starts(p, b"/>").then(|| one(2)),
            VueRule::Block => (b == b'>').then(|| one(1)),
            VueRule::Element => self.vue_element_end(p, super::state::name(self.top().payload)),
            VueRule::TagStuff => {
                if self.starts(p, b"/>") {
                    Some(zero())
                } else {
                    (b == b'>').then(|| one(1))
                }
            }
            VueRule::CapitalizedAttrs => (b == b'>' || self.starts(p, b"/>")).then(zero),
            VueRule::ScriptTagStuff => self.vue_stuff_tag_end(p, Element::Script),
            VueRule::StyleTagStuff => self.vue_stuff_tag_end(p, Element::Style),
            VueRule::ScriptStuff => script_end(self.s, p).then(zero),
            VueRule::StyleStuff => style_end(self.s, p).then(zero),
            VueRule::LangGroup => {
                let ends = match Lang::from_bits(self.top().payload) {
                    Lang::Ts | Lang::Tsx | Lang::Jsx | Lang::Coffee => script_end(self.s, p),
                    Lang::Md
                    | Lang::Html
                    | Lang::Pug
                    | Lang::Stylus
                    | Lang::Postcss
                    | Lang::Sass
                    | Lang::Css
                    | Lang::Scss
                    | Lang::Less
                    | Lang::Js
                    | Lang::Json
                    | Lang::Jsonc
                    | Lang::Json5
                    | Lang::Yaml
                    | Lang::Toml
                    | Lang::Graphql
                    | Lang::Vue => style_end(self.s, p),
                };
                ends.then(zero)
            }
            VueRule::TemplateBody => template_end(self.s, p).then(zero),
            VueRule::TemplateTag | VueRule::Capitalized => {
                if b == b'>' {
                    Some(one(1))
                } else {
                    self.starts(p, b"/>").then(|| one(2))
                }
            }
            VueRule::TemplateTagEnd => self.vue_template_tag_end(p),
            VueRule::Directive => self.vue_directive_end(p),
            VueRule::ExprString => (u32::from(b) == self.top().payload).then(|| one(1)),
            VueRule::ExprBare => bare_end(&self.s[..self.lim], p).then(zero),
            VueRule::Body | VueRule::BodyWhile | VueRule::Expr => None,
        }
    }

    /// The patterns of a Vue rule at `p`, but for the rules that may hold a
    /// template's HTML, which share the root's arm (`vue_template_patterns`).
    #[cold]
    #[inline(never)]
    pub(super) fn vue_patterns_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let rule = rule.vue()?;
        let b = self.at(p);
        match rule {
            VueRule::SelfClosing => self
                .g(p)
                .then(|| Hit::new(Pattern::Zero(Rule::VueTagStuff), p, p)),
            VueRule::Block => self.vue_block_patterns(p),
            VueRule::TagStuff
            | VueRule::ScriptTagStuff
            | VueRule::StyleTagStuff
            | VueRule::CapitalizedAttrs => self.vue_attribute_patterns(p),
            VueRule::ScriptStuff => self.vue_stuff_patterns(p, Element::Script),
            VueRule::StyleStuff => self.vue_stuff_patterns(p, Element::Style),
            VueRule::LangGroup => {
                self.vue_lang_group_patterns(p, Lang::from_bits(self.top().payload))
            }
            VueRule::TemplateTag => self.g(p).then(|| {
                let tag = TemplateTag::from_bits(self.top().payload);
                Hit::new(Pattern::Vue(VuePattern::TemplateTagEnd), p, p).with([0; 5], tag.bits())
            }),
            VueRule::Capitalized => self
                .g(p)
                .then(|| Hit::new(Pattern::Zero(Rule::VueCapitalizedAttrs), p, p)),
            VueRule::Directive => {
                let directive = if self.top().payload == 1 {
                    Directive::Control
                } else {
                    Directive::Original
                };
                self.vue_directive_patterns(p, directive)
            }
            VueRule::ExprString => (p > 0
                && matches!(self.s.get(p - 1), Some(b'"' | b'\'' | b'`')))
            .then(|| Hit::new(Pattern::Zero(Rule::VueExpr), p, p)),
            VueRule::ExprBare => (p < self.lim && !matches!(b, b'"' | b'\'' | b'`'))
                .then(|| Hit::new(Pattern::Zero(Rule::VueExpr), p, p)),
            VueRule::Comment
            | VueRule::CommentKv
            | VueRule::Element
            | VueRule::Body
            | VueRule::BodyWhile
            | VueRule::TemplateBody
            | VueRule::TemplateTagEnd
            | VueRule::Expr => None,
        }
    }
}

/// The lexer that reads a Vue body or directive value.
#[cfg(feature = "lang-vue")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Lexed {
    Css,
    Scss,
    Less,
    /// JavaScript, TypeScript, JSX or TSX.
    Script(Dialect),
    Yaml,
    Graphql,
    /// Sass, through `Language::Sass`.
    #[cfg(feature = "lang-sass")]
    Sass,
    /// CoffeeScript, through `Language::CoffeeScript`.
    #[cfg(feature = "lang-coffee")]
    Coffee,
    /// A TypeScript expression: a directive's value.
    Expression,
}

/// The lexer of `lang`'s body; none for a language that nishiki does not
/// lex, whose body is plain text.
#[cfg(feature = "lang-vue")]
fn lexed(lang: Lang) -> Option<Lexed> {
    match lang {
        Lang::Css => Some(Lexed::Css),
        Lang::Scss => Some(Lexed::Scss),
        Lang::Less => Some(Lexed::Less),
        Lang::Js => Some(Lexed::Script(Dialect::JavaScript)),
        Lang::Ts => Some(Lexed::Script(Dialect::TypeScript)),
        Lang::Jsx => Some(Lexed::Script(Dialect::Jsx)),
        Lang::Tsx => Some(Lexed::Script(Dialect::Tsx)),
        Lang::Yaml => Some(Lexed::Yaml),
        Lang::Graphql => Some(Lexed::Graphql),
        #[cfg(feature = "lang-sass")]
        Lang::Sass => Some(Lexed::Sass),
        #[cfg(not(feature = "lang-sass"))]
        Lang::Sass => None,
        #[cfg(feature = "lang-coffee")]
        Lang::Coffee => Some(Lexed::Coffee),
        #[cfg(not(feature = "lang-coffee"))]
        Lang::Coffee => None,
        Lang::Md
        | Lang::Html
        | Lang::Pug
        | Lang::Stylus
        | Lang::Postcss
        | Lang::Json
        | Lang::Jsonc
        | Lang::Json5
        | Lang::Toml
        | Lang::Vue => None,
    }
}

/// The language of the body that `parent`, a rule with `payload`, opened.
pub(super) fn body_lang(parent: Rule, payload: u32) -> Option<Lang> {
    if parent == Rule::VueElement {
        return match Block::from_bits(super::state::payload_bits(payload)) {
            Block::Lang(lang) => Some(lang),
            Block::Template | Block::Script | Block::Style | Block::Other => None,
        };
    }
    if parent == Rule::VueScriptStuff {
        return Some(Lang::Js);
    }
    if parent == Rule::VueStyleStuff {
        return Some(Lang::Css);
    }
    (parent == Rule::VueLangGroup).then(|| Lang::from_bits(payload))
}

/// What reads the text of `rule`, a Vue body or directive value inside
/// `parent`, and where the text ends.
/// The answer is the lexer, if any, and the halt.
/// The halt is none for a body that runs to the end of each line.
#[cfg(feature = "lang-vue")]
pub(super) fn target(rule: Rule, parent: Rule, payload: u32) -> (Option<Lexed>, Option<VueHalt>) {
    if rule == Rule::VueExpr {
        let halt: VueHalt = if parent != Rule::VueExprString {
            bare_end
        } else if payload == u32::from(b'\'') {
            single_quote
        } else if payload == u32::from(b'`') {
            backtick
        } else {
            double_quote
        };
        return (Some(Lexed::Expression), Some(halt));
    }
    let lang = body_lang(parent, payload);
    if rule == Rule::VueBodyWhile {
        return (lang.and_then(lexed), None);
    }
    let script_group =
        lang.is_some_and(|lang| matches!(lang, Lang::Ts | Lang::Tsx | Lang::Jsx | Lang::Coffee));
    let halt: VueHalt = if parent == Rule::VueScriptStuff
        || (parent == Rule::VueLangGroup && script_group)
        || (parent == Rule::VueElement && lang.is_some_and(Lang::ends_at_script))
    {
        script_end
    } else if parent == Rule::VueStyleStuff || parent == Rule::VueLangGroup {
        style_end
    } else {
        body_end
    };
    (lang.and_then(lexed), Some(halt))
}

/// Where a Vue body or directive value ends: a test of the text at a place.
#[cfg(feature = "lang-vue")]
pub(super) type VueHalt = fn(&[u8], usize) -> bool;

/// A halt that never stops a line, for a body that a `while` rule holds.
#[cfg(feature = "lang-vue")]
pub(super) fn never(_: &[u8], _: usize) -> bool {
    false
}

/// The first place at or after `from` of `s` where `halt` holds.
#[cfg(feature = "lang-vue")]
pub(super) fn halt_at(s: &[u8], from: usize, halt: VueHalt) -> Option<usize> {
    (from..=s.len()).find(|&at| halt(s, at))
}

/// The scope of `lang`'s body in a block.
pub(super) fn lang_scope(lang: Lang) -> S {
    match lang {
        Lang::Md => S::TextHtmlMarkdown,
        Lang::Html => S::TextHtmlDerivative,
        Lang::Pug => S::TextPug,
        Lang::Stylus => S::SourceStylus,
        Lang::Postcss => S::SourcePostcss,
        Lang::Sass => S::SourceSass,
        Lang::Css => S::SourceCss,
        Lang::Scss => S::SourceScss,
        Lang::Less => S::SourceLess,
        Lang::Js => S::SourceJs,
        Lang::Ts => S::SourceTs,
        Lang::Jsx => S::SourceJsx,
        Lang::Tsx => S::SourceTsx,
        Lang::Coffee => S::SourceCoffee,
        Lang::Json => S::SourceJson,
        Lang::Jsonc => S::SourceJsonComments,
        Lang::Json5 => S::SourceJson5,
        Lang::Yaml => S::SourceYaml,
        Lang::Toml => S::SourceToml,
        Lang::Graphql => S::SourceGraphql,
        Lang::Vue => S::TextHtmlVue,
    }
}

/// A rule of Vue's, as [`Rule::vue`] names it.
/// The HTML dispatchers send every Vue rule to one out-of-line function,
/// which matches on this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum VueRule {
    Comment,
    CommentKv,
    SelfClosing,
    Block,
    Element,
    TagStuff,
    ScriptTagStuff,
    StyleTagStuff,
    ScriptStuff,
    StyleStuff,
    LangGroup,
    Body,
    BodyWhile,
    TemplateBody,
    TemplateTag,
    TemplateTagEnd,
    Capitalized,
    CapitalizedAttrs,
    Directive,
    ExprString,
    ExprBare,
    Expr,
}

impl Rule {
    /// The Vue rule this is, or none for HTML's and ERB's own rules.
    /// Each entry to Vue's code tests it, or the feature, first.
    #[cfg(feature = "lang-vue")]
    pub(super) const fn vue(self) -> Option<VueRule> {
        Some(match self {
            Rule::VueComment => VueRule::Comment,
            Rule::VueCommentKv => VueRule::CommentKv,
            Rule::VueSelfClosing => VueRule::SelfClosing,
            Rule::VueBlock => VueRule::Block,
            Rule::VueElement => VueRule::Element,
            Rule::VueTagStuff => VueRule::TagStuff,
            Rule::VueScriptTagStuff => VueRule::ScriptTagStuff,
            Rule::VueStyleTagStuff => VueRule::StyleTagStuff,
            Rule::VueScriptStuff => VueRule::ScriptStuff,
            Rule::VueStyleStuff => VueRule::StyleStuff,
            Rule::VueLangGroup => VueRule::LangGroup,
            Rule::VueBody => VueRule::Body,
            Rule::VueBodyWhile => VueRule::BodyWhile,
            Rule::VueTemplateBody => VueRule::TemplateBody,
            Rule::VueTemplateTag => VueRule::TemplateTag,
            Rule::VueTemplateTagEnd => VueRule::TemplateTagEnd,
            Rule::VueCapitalized => VueRule::Capitalized,
            Rule::VueCapitalizedAttrs => VueRule::CapitalizedAttrs,
            Rule::VueDirective => VueRule::Directive,
            Rule::VueExprString => VueRule::ExprString,
            Rule::VueExprBare => VueRule::ExprBare,
            Rule::VueExpr => VueRule::Expr,
            Rule::Root
            | Rule::Comment
            | Rule::Cdata
            | Rule::Doctype
            | Rule::DoctypeString
            | Rule::Processing
            | Rule::Tag
            | Rule::Element
            | Rule::ElementStart
            | Rule::StyleWrap
            | Rule::StyleTag
            | Rule::OpenStyle
            | Rule::BodyCss
            | Rule::ScriptWrap
            | Rule::ScriptTag
            | Rule::ScriptHead
            | Rule::ScriptAttributes
            | Rule::ScriptJsOuter
            | Rule::BodyJs
            | Rule::ScriptTemplateOuter
            | Rule::OpenScript
            | Rule::BodyTemplate
            | Rule::ScriptOtherOuter
            | Rule::BodyOther
            | Rule::Attribute
            | Rule::Value
            | Rule::ValueCss
            | Rule::ValueJs
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::StringPlain
            | Rule::StringCss
            | Rule::StringJs
            | Rule::JsLineComment
            | Rule::JsBlockComment
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap
            | Rule::ErbComment
            | Rule::ErbBlock
            | Rule::ErbLine
            | Rule::BladeComment
            | Rule::BladeCommentPhpWrap
            | Rule::BladeCommentPhpBlock
            | Rule::BladeCommentPhpLine
            | Rule::BladePhpWrap
            | Rule::BladePhpBlock
            | Rule::BladePhpLine
            | Rule::BladePhpLineCode
            | Rule::BladeEchoTriple
            | Rule::BladeEcho
            | Rule::BladeEchoRaw
            | Rule::BladeEchoEscaped
            | Rule::BladeDirective
            | Rule::BladeDirectiveCustom
            | Rule::BladeArgsComment
            | Rule::BladeBalance
            | Rule::BladePhpSection
            | Rule::BladeAttribute => return None,
        })
    }

    /// None: without the `lang-vue` feature no rule is Vue's.
    /// The engine's entries to Vue's code then fold away, and a build
    /// without Vue keeps none of its code.
    #[cfg(not(feature = "lang-vue"))]
    pub(super) const fn vue(self) -> Option<VueRule> {
        None
    }
}

/// What Vue's own patterns decide at a place of a rule that may hold a
/// template's HTML.
pub(super) enum Template {
    /// The rule's answer: a match, or none.
    Done(Option<Hit>),
    /// html-derivative's root patterns decide, after Vue's own.
    Root,
}
