//! The patterns of Blade's injection, as plain functions of the line.
//! The injection opens Blade's own rules wherever it applies
//! ([`Rule::blade_injection`](super::state::Rule::blade_injection)).
//!
//! The grammar's patterns, in order:
//!
//! 1. `\{\{--`: a comment, up to `--}}`;
//! 2. `(?<!@)\{\{\{`, 3. `(?<![@{])\{\{`, 4. `(?<!@)\{!!` and 5. `(@)\{\{`:
//!    echoes, up to `}}}`, `}}`, `!!}` and `}}`;
//! 6. a directive of [`ARGS_PHP`] and its `(`, up to `)`;
//! 7. a directive of [`ARGS_COMMENT`] and its `(`, up to `)`;
//! 8. a directive of [`KEYWORDS`] alone;
//! 9. a directive of [`BLOCKS`], after an optional `end`, alone;
//! 10. `@php` or `@setup`, up to `@endphp` or `@endsetup`;
//! 11. `@endphp` or `@endsetup` and its `(`, up to `)`;
//! 12. `@endphp` or `@endsetup` alone;
//! 13. any other directive and its `(`, up to `)`;
//! 14. any other directive alone;
//! 15. `(:[-a-z]+)(=)(")`: a bound attribute, up to `"`;
//! 16. `^(\s*)(?=<\?(?![^?]*\?>))`: the blanks before PHP that the line does
//!     not close;
//! 17. `<\?(?i:php|=)?(?![^?]*\?>)`: PHP that the line does not close;
//! 18. `<\?(?i:php|=)?`: PHP, up to `>`.
//!
//! The directives 6 to 14 need no word character, `[0-9@-Z_a-z]`, before
//! their `@`.

use super::engine::{Hit, Lexer, Part, Pattern};
use super::scope::S;
use super::state::{Injection, Rule};
use crate::language::Halt;

/// One of the injection's patterns, or of the patterns of Blade's rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Blade {
    /// 1: `{{--`.
    Comment,
    /// 2: `{{{`.
    EchoTriple,
    /// 3: `{{`.
    Echo,
    /// 4: `{!!`.
    EchoRaw,
    /// 5: `@{{`.
    EchoEscaped,
    /// 6: a directive whose `(…)` holds PHP. The value is where `(` stands.
    Directive(usize),
    /// 7 and 11: a directive whose `(…)` is a comment. The value is where `(`
    /// stands; `ends_php` is set for `@endphp(`.
    ArgsComment { paren: usize, ends_php: bool },
    /// 8 and 9: a directive alone, `keyword.blade`.
    Keyword,
    /// 10: `@php` or `@setup`.
    PhpSection,
    /// 12: `@endphp` or `@endsetup` alone.
    EndPhp,
    /// 13: any other directive and its `(`. The value is where `(` stands.
    Custom(usize),
    /// 14: any other directive alone.
    Function,
    /// 15: a bound attribute. The value is where `=` stands.
    Attribute(usize),
    /// 16: the blanks at the start of a line before PHP it does not close.
    PhpWrap,
    /// 17: PHP that its line does not close.
    PhpBlock,
    /// 18: PHP, up to `>`.
    PhpLine,
    /// In a comment: the blanks at the start of a line before PHP it does not
    /// close.
    CommentPhpWrap,
    /// In a comment: PHP that its line does not close.
    CommentPhpBlock,
    /// In a comment: PHP, up to `>`.
    CommentPhpLine,
    /// `\G(\s*)((\?))(?=>)`, the empty code of `<? ?>`. The value is where `?`
    /// stands.
    EmptyTag(usize),
    /// `\G`, which opens the code of `<? … ?>`.
    LineCode,
    /// `(` in a directive's comment.
    Paren,
    /// `[^()]+` in a directive's comment.
    ArgsText,
}

/// The directives whose `(…)` holds PHP, lowered: `(?i:auth|break|…)`.
const ARGS_PHP: [&[u8]; 71] = [
    b"auth",
    b"aware",
    b"break",
    b"can",
    b"canany",
    b"cannot",
    b"case",
    b"checked",
    b"choice",
    b"class",
    b"component",
    b"continue",
    b"dd",
    b"disabled",
    b"dump",
    b"each",
    b"elsecan",
    b"elsecanany",
    b"elsecannot",
    b"elseif",
    b"empty",
    b"env",
    b"error",
    b"extends",
    b"finished",
    b"for",
    b"foreach",
    b"forelse",
    b"guest",
    b"hassection",
    b"if",
    b"include",
    b"includefirst",
    b"includeif",
    b"includeunless",
    b"includewhen",
    b"inject",
    b"isset",
    b"js",
    b"json",
    b"lang",
    b"livewire",
    b"method",
    b"once",
    b"php",
    b"prepend",
    b"prependonce",
    b"production",
    b"props",
    b"push",
    b"pushif",
    b"pushonce",
    b"readonly",
    b"required",
    b"section",
    b"sectionmissing",
    b"selected",
    b"servers",
    b"slack",
    b"slot",
    b"stack",
    b"story",
    b"style",
    b"switch",
    b"task",
    b"unless",
    b"unset",
    b"use",
    b"vite",
    b"while",
    b"yield",
];
/// The directives whose `(…)` is a comment, lowered.
const ARGS_COMMENT: [&[u8]; 37] = [
    b"append",
    b"default",
    b"else",
    b"endauth",
    b"endcan",
    b"endcanany",
    b"endcannot",
    b"endcomponent",
    b"endempty",
    b"endenv",
    b"enderror",
    b"endfinished",
    b"endfor",
    b"endforeach",
    b"endforelse",
    b"endguest",
    b"endif",
    b"endisset",
    b"endlang",
    b"endonce",
    b"endprepend",
    b"endprependonce",
    b"endproduction",
    b"endpush",
    b"endpushif",
    b"endpushonce",
    b"endsection",
    b"endslot",
    b"endstory",
    b"endswitch",
    b"endtask",
    b"endunless",
    b"endwhile",
    b"overwrite",
    b"parent",
    b"show",
    b"stop",
];
/// The directives that stand alone, as the grammar spells them.
const KEYWORDS: [&[u8]; 15] = [
    b"append",
    b"break",
    b"continue",
    b"csrf",
    b"default",
    b"each",
    b"else",
    b"livewireScripts",
    b"livewireStyles",
    b"overwrite",
    b"parent",
    b"sectionMissing",
    b"show",
    b"stack",
    b"stop",
];
/// The directives that open or close a block, lowered, after an optional `end`.
const BLOCKS: [&[u8]; 32] = [
    b"auth",
    b"can",
    b"canany",
    b"cannot",
    b"component",
    b"empty",
    b"env",
    b"error",
    b"finished",
    b"for",
    b"foreach",
    b"forelse",
    b"guest",
    b"if",
    b"isset",
    b"lang",
    b"once",
    b"prepend",
    b"prependonce",
    b"production",
    b"push",
    b"pushif",
    b"pushonce",
    b"section",
    b"session",
    b"slot",
    b"story",
    b"switch",
    b"task",
    b"unless",
    b"verbatim",
    b"while",
];

/// The first of the injection's patterns that matches at `p` of `s`, with
/// where its match ends.
pub(super) fn at(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    match s.get(p)? {
        b'{' => echo(s, p),
        b'@' => {
            if s.get(p + 1..p + 3) == Some(b"{{") {
                return Some((Blade::EchoEscaped, p + 3));
            }
            directive(s, p)
        }
        b':' => attribute(s, p),
        _ if p == 0
            && let Some(found) = php_wrap(s) =>
        {
            Some(found)
        }
        b'<' => php(s, p),
        _ => None,
    }
}

/// Patterns 1 to 4 at a `{`.
fn echo(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    let rest = &s[p..];
    let before = p.checked_sub(1).and_then(|at| s.get(at)).copied();
    if rest.starts_with(b"{{--") {
        return Some((Blade::Comment, p + 4));
    }
    if rest.starts_with(b"{{{") && before != Some(b'@') {
        return Some((Blade::EchoTriple, p + 3));
    }
    if rest.starts_with(b"{{") && !matches!(before, Some(b'@' | b'{')) {
        return Some((Blade::Echo, p + 2));
    }
    if rest.starts_with(b"{!!") && before != Some(b'@') {
        return Some((Blade::EchoRaw, p + 3));
    }
    None
}

/// Patterns 6 to 14 at an `@`.
fn directive(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    let before = p.checked_sub(1).and_then(|at| s.get(at)).copied();
    if before.is_some_and(is_directive_word) {
        return None;
    }
    let name = p + 1;
    let letters = name
        + s[name..]
            .iter()
            .take_while(|b| b.is_ascii_alphabetic())
            .count();
    let word = &s[name..letters];
    let paren = blanks_end(s, letters);
    let opens = s.get(paren) == Some(&b'(');
    if opens && listed(&ARGS_PHP, word, true) {
        return Some((Blade::Directive(paren), paren + 1));
    }
    if opens && listed(&ARGS_COMMENT, word, true) {
        return Some((
            Blade::ArgsComment {
                paren,
                ends_php: false,
            },
            paren + 1,
        ));
    }
    let bounded = !word_at(s, letters);
    if bounded && listed(&KEYWORDS, word, false) {
        return Some((Blade::Keyword, letters));
    }
    if bounded
        && (listed(&BLOCKS, word, true)
            || word
                .strip_prefix(b"end")
                .is_some_and(|rest| listed(&BLOCKS, rest, true)))
    {
        return Some((Blade::Keyword, letters));
    }
    if bounded && (word.eq_ignore_ascii_case(b"php") || word.eq_ignore_ascii_case(b"setup")) {
        return Some((Blade::PhpSection, letters));
    }
    let ends_php = word.eq_ignore_ascii_case(b"endphp") || word.eq_ignore_ascii_case(b"endsetup");
    if ends_php && opens {
        return Some((
            Blade::ArgsComment {
                paren,
                ends_php: true,
            },
            paren + 1,
        ));
    }
    if ends_php && bounded {
        return Some((Blade::EndPhp, letters));
    }
    let mut end = word_run(s, name);
    if end == name {
        return None;
    }
    if s.get(end..end + 3) == Some(b"::w") {
        end = end + 2 + s[end + 2..].iter().take_while(|&&b| b == b'w').count();
    }
    let paren = blanks_end(s, end);
    if s.get(paren) == Some(&b'(') {
        return Some((Blade::Custom(paren), paren + 1));
    }
    (!word_at(s, end)).then_some((Blade::Function, end))
}

/// Pattern 15 at a `:`.
fn attribute(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    let name = p + 1;
    let end = name
        + s[name..]
            .iter()
            .take_while(|&&b| b == b'-' || b.is_ascii_lowercase())
            .count();
    (end > name && s.get(end..end + 2) == Some(b"=\"")).then_some((Blade::Attribute(end), end + 2))
}

/// Patterns 17 and 18 at a `<`.
fn php(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    let end = php_open_end(s, p)?;
    Some(if closes(s, p + 2) {
        (Blade::PhpLine, end)
    } else {
        (Blade::PhpBlock, end)
    })
}

/// Pattern 16 at the start of the line: blanks, then `<?` that the line does
/// not close.
fn php_wrap(s: &[u8]) -> Option<(Blade, usize)> {
    let q = blanks_end_any(s, 0);
    (s.get(q..q + 2) == Some(b"<?") && !closes(s, q + 2)).then_some((Blade::PhpWrap, q))
}

/// The end of `<\?(?i:php|=)?` at `p`.
pub(super) fn php_open_end(s: &[u8], p: usize) -> Option<usize> {
    if s.get(p..p + 2) != Some(b"<?") {
        return None;
    }
    let q = p + 2;
    if s.get(q..q + 3)
        .is_some_and(|w| w.eq_ignore_ascii_case(b"php"))
    {
        return Some(q + 3);
    }
    Some(q + usize::from(s.get(q) == Some(&b'=')))
}

/// `[^?]*\?>` at `p`: whether the first `?` from `p` is followed by `>`.
fn closes(s: &[u8], p: usize) -> bool {
    let rest = s.get(p..).unwrap_or_default();
    memchr::memchr(b'?', rest).is_some_and(|at| rest.get(at + 1) == Some(&b'>'))
}

/// Whether `word` is in `list`, with or without regard to ASCII case.
fn listed(list: &[&[u8]], word: &[u8], fold: bool) -> bool {
    !word.is_empty()
        && list.iter().any(|entry| {
            if fold {
                entry.eq_ignore_ascii_case(word)
            } else {
                *entry == word
            }
        })
}

/// The end of the run of spaces and tabs from `p`.
fn blanks_end(s: &[u8], p: usize) -> usize {
    p + s
        .get(p..)
        .unwrap_or_default()
        .iter()
        .take_while(|&&b| b == b' ' || b == b'\t')
        .count()
}

/// The end of the run of `\s` characters from `p`.
fn blanks_end_any(s: &[u8], mut p: usize) -> usize {
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

/// Whether a word character, `\w`, starts at `p`.
fn word_at(s: &[u8], p: usize) -> bool {
    crate::utf8::char_at(s, p).is_some_and(|(c, _)| crate::classes::is_word(c))
}

/// The end of the run of word characters, `\w+`, from `p`.
fn word_run(s: &[u8], mut p: usize) -> usize {
    while let Some((c, len)) = crate::utf8::char_at(s, p) {
        if !crate::classes::is_word(c) {
            break;
        }
        p += len;
    }
    p
}

/// A rule of Blade's, as [`Rule::blade`] gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BladeRule {
    /// `{{--` up to `--}}`.
    Comment,
    /// The blanks before `<?` in a comment that the line does not close.
    CommentPhpWrap,
    /// `<?php` in a comment that the line does not close, up to `?>`.
    CommentPhpBlock,
    /// `<?php` in a comment, up to `>`.
    CommentPhpLine,
    /// The blanks before `<?` that the line does not close.
    PhpWrap,
    /// `<?php` that the line does not close, up to `?>`.
    PhpBlock,
    /// `<?php` up to `>`.
    PhpLine,
    /// The code of such a tag, up to `?` before `>`.
    PhpLineCode,
    /// `{{{` up to `}}}`.
    EchoTriple,
    /// `{{` up to `}}`.
    Echo,
    /// `{!!` up to `!!}`.
    EchoRaw,
    /// `@{{` up to `}}`.
    EchoEscaped,
    /// A directive and its PHP, up to `)`.
    Directive,
    /// Any other directive and its PHP, up to `)`.
    DirectiveCustom,
    /// A directive whose `(…)` is a comment, up to `)`.
    ArgsComment,
    /// `(` up to `)` in such a comment.
    Balance,
    /// `@php` up to `@endphp`.
    PhpSection,
    /// A bound attribute, up to `"`.
    Attribute,
}

impl BladeRule {
    /// Whether the rule's text is PHP, which Blade's PHP lexer takes.
    pub(super) fn holds_php(self) -> bool {
        match self {
            BladeRule::CommentPhpBlock
            | BladeRule::PhpBlock
            | BladeRule::PhpLineCode
            | BladeRule::EchoTriple
            | BladeRule::Echo
            | BladeRule::EchoRaw
            | BladeRule::EchoEscaped
            | BladeRule::Directive
            | BladeRule::DirectiveCustom
            | BladeRule::PhpSection
            | BladeRule::Attribute => true,
            BladeRule::Comment
            | BladeRule::CommentPhpWrap
            | BladeRule::CommentPhpLine
            | BladeRule::PhpWrap
            | BladeRule::PhpLine
            | BladeRule::ArgsComment
            | BladeRule::Balance => false,
        }
    }

    /// The end of the rule's end pattern at `p`, for a rule whose text is PHP.
    /// The PHP lexer tests it while none of its rules is open.
    pub(super) fn php_end(self, s: &[u8], p: usize) -> Option<usize> {
        let rest = s.get(p..).unwrap_or_default();
        let lit = |end: &[u8]| rest.starts_with(end).then_some(p + end.len());
        match self {
            BladeRule::CommentPhpBlock | BladeRule::PhpBlock => lit(b"?>"),
            BladeRule::PhpLineCode => (rest.starts_with(b"?>")).then_some(p + 1),
            BladeRule::EchoTriple => lit(b"}}}"),
            BladeRule::Echo | BladeRule::EchoEscaped => lit(b"}}"),
            BladeRule::EchoRaw => lit(b"!!}"),
            BladeRule::Directive | BladeRule::DirectiveCustom => lit(b")"),
            BladeRule::Attribute => lit(b"\""),
            BladeRule::PhpSection => ends_section(s, p).then_some(p),
            BladeRule::Comment
            | BladeRule::CommentPhpWrap
            | BladeRule::CommentPhpLine
            | BladeRule::PhpWrap
            | BladeRule::PhpLine
            | BladeRule::ArgsComment
            | BladeRule::Balance => None,
        }
    }
}

impl Rule {
    /// The rule as one of Blade's, or `None` for a rule of another grammar.
    pub(super) const fn blade(self) -> Option<BladeRule> {
        Some(match self {
            Rule::BladeComment => BladeRule::Comment,
            Rule::BladeCommentPhpWrap => BladeRule::CommentPhpWrap,
            Rule::BladeCommentPhpBlock => BladeRule::CommentPhpBlock,
            Rule::BladeCommentPhpLine => BladeRule::CommentPhpLine,
            Rule::BladePhpWrap => BladeRule::PhpWrap,
            Rule::BladePhpBlock => BladeRule::PhpBlock,
            Rule::BladePhpLine => BladeRule::PhpLine,
            Rule::BladePhpLineCode => BladeRule::PhpLineCode,
            Rule::BladeEchoTriple => BladeRule::EchoTriple,
            Rule::BladeEcho => BladeRule::Echo,
            Rule::BladeEchoRaw => BladeRule::EchoRaw,
            Rule::BladeEchoEscaped => BladeRule::EchoEscaped,
            Rule::BladeDirective => BladeRule::Directive,
            Rule::BladeDirectiveCustom => BladeRule::DirectiveCustom,
            Rule::BladeArgsComment => BladeRule::ArgsComment,
            Rule::BladeBalance => BladeRule::Balance,
            Rule::BladePhpSection => BladeRule::PhpSection,
            Rule::BladeAttribute => BladeRule::Attribute,
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
            | Rule::VueComment
            | Rule::VueCommentKv
            | Rule::VueSelfClosing
            | Rule::VueBlock
            | Rule::VueElement
            | Rule::VueTagStuff
            | Rule::VueScriptTagStuff
            | Rule::VueStyleTagStuff
            | Rule::VueScriptStuff
            | Rule::VueStyleStuff
            | Rule::VueLangGroup
            | Rule::VueBody
            | Rule::VueBodyWhile
            | Rule::VueTemplateBody
            | Rule::VueTemplateTag
            | Rule::VueTemplateTagEnd
            | Rule::VueCapitalized
            | Rule::VueCapitalizedAttrs
            | Rule::VueDirective
            | Rule::VueExprString
            | Rule::VueExprBare
            | Rule::VueExpr => return None,
        })
    }
}

/// `(?<![0-9@-Z_a-z])(?=@(?i:end(?:php|setup))\b)`: the end of an `@php`
/// section, before `@endphp` or `@endsetup`.
fn ends_section(s: &[u8], p: usize) -> bool {
    if s.get(p) != Some(&b'@')
        || p.checked_sub(1)
            .and_then(|at| s.get(at))
            .is_some_and(|&b| is_directive_word(b))
    {
        return false;
    }
    let name = p + 1;
    let letters = name
        + s[name..]
            .iter()
            .take_while(|b| b.is_ascii_alphabetic())
            .count();
    let word = &s[name..letters];
    (word.eq_ignore_ascii_case(b"endphp") || word.eq_ignore_ascii_case(b"endsetup"))
        && !word_at(s, letters)
}

/// Whether `b` is a byte of `[0-9@-Z_a-z]`, which may not stand before a
/// directive's `@`.
fn is_directive_word(b: u8) -> bool {
    b.is_ascii_digit() || matches!(b, b'@'..=b'Z' | b'_' | b'a'..=b'z')
}

/// Whether one of the injection's patterns matches at `p` of `s`: the halt of
/// Blade's PHP where the injection applies.
pub(super) fn begins(s: &[u8], p: usize) -> bool {
    at(s, p).is_some()
}

/// The halts of the PHP of a Blade rule.
pub(super) struct PhpHalt {
    /// The rule that holds the PHP.
    pub(super) rule: BladeRule,
    /// How the injection applies to the rule's text.
    pub(super) injection: Injection,
}

impl Halt for PhpHalt {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds(s, p))
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        (self.injection == Injection::Leads && begins(s, p)) || self.rule.php_end(s, p).is_some()
    }

    fn deep(&self) -> Option<&dyn Halt> {
        match self.injection {
            Injection::Applies | Injection::Leads => Some(&begins),
            Injection::ShutOut => None,
        }
    }

    fn deep_loses_ties(&self) -> bool {
        self.injection == Injection::Applies
    }
}

impl Pattern {
    /// The pattern of a match of `blade`: [`Pattern::Blade`] where it opens
    /// a rule, [`Pattern::BladeMatch`] where it opens none.
    /// The two join the arms of the other patterns that open a rule or not,
    /// so the HTML engine calls each step from one place.
    pub(super) fn blade(blade: Blade) -> Self {
        if Lexer::blade_opens(blade).is_some() {
            Pattern::Blade(blade)
        } else {
            Pattern::BladeMatch(blade)
        }
    }
}

/// The patterns of a Blade comment at `p`: PHP, which the comment marks as
/// illegal.
pub(super) fn comment_php(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    if p == 0
        && let Some((Blade::PhpWrap, end)) = php_wrap(s)
    {
        return Some((Blade::CommentPhpWrap, end));
    }
    match php(s, p)? {
        (Blade::PhpBlock, end) => Some((Blade::CommentPhpBlock, end)),
        (_, end) => Some((Blade::CommentPhpLine, end)),
    }
}

/// `\G(\s*)((\?))(?=>)` at `p`: the empty code of `<? ?>`, with where its `?`
/// stands.
pub(super) fn empty_tag(s: &[u8], p: usize) -> Option<usize> {
    let q = blanks_end_any(s, p);
    (s.get(q..q + 2) == Some(b"?>")).then_some(q)
}

/// `[^()]+` at `p`, inside a directive's comment: where the run ends, up to
/// `end`, the line's last place.
pub(super) fn args_text_end(s: &[u8], p: usize, end: usize) -> Option<usize> {
    let rest = s.get(p..).unwrap_or_default();
    let run = memchr::memchr2(b'(', b')', rest).map_or(end, |at| p + at);
    (run > p).then_some(run)
}

impl Lexer<'_> {
    /// The scopes of a Blade rule's name on `base`, as [`Lexer::name_stack`]
    /// gives them.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_name_stack(&mut self, base: u32, rule: Rule) -> u32 {
        let Some(rule) = rule.blade() else {
            return base;
        };
        let scope = match rule {
            BladeRule::PhpWrap
            | BladeRule::PhpLineCode
            | BladeRule::Balance
            | BladeRule::Attribute => return base,
            BladeRule::Comment => S::BladeComment,
            BladeRule::CommentPhpWrap => S::BladePhpInComment,
            BladeRule::CommentPhpBlock => S::BladePhpBlockInComment,
            BladeRule::CommentPhpLine => S::BladePhpLineInComment,
            BladeRule::PhpBlock => S::PhpBlock,
            BladeRule::PhpLine => S::PhpLine,
            BladeRule::EchoTriple
            | BladeRule::Echo
            | BladeRule::EchoRaw
            | BladeRule::EchoEscaped => S::BladeEcho,
            BladeRule::Directive | BladeRule::ArgsComment => S::BladeDirective,
            BladeRule::DirectiveCustom => S::BladeDirectiveCustom,
            BladeRule::PhpSection => S::BladeEmbeddedBlock,
        };
        self.path(base, &[scope])
    }

    /// The scopes of a Blade rule's content on its name's stack `name`.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_content_stack(&mut self, name: u32, rule: Rule) -> u32 {
        let Some(rule) = rule.blade() else {
            return name;
        };
        if rule.holds_php() {
            return self.path(name, &[S::SourcePhp]);
        }
        if rule == BladeRule::ArgsComment {
            return self.path(name, &[S::BladeCommentArgs]);
        }
        name
    }

    /// The first position at or after `p` where a pattern or the end of a
    /// Blade rule whose text is not PHP can match.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_candidate(&self, rule: Rule, p: usize, cut: usize) -> Option<usize> {
        let rule = rule.blade()?;
        let rest = self.s.get(p..cut.min(self.lim)).unwrap_or(&[]);
        let find = |found: Option<usize>| {
            self.spend(found.unwrap_or(rest.len()) / 16);
            found.map(|at| at + p)
        };
        match rule {
            BladeRule::Comment => {
                if p == 0 {
                    return Some(0);
                }
                find(memchr::memchr2(b'-', b'<', rest))
            }
            BladeRule::CommentPhpWrap
            | BladeRule::PhpWrap
            | BladeRule::ArgsComment
            | BladeRule::Balance => (p <= self.end_pos()).then_some(p),
            BladeRule::CommentPhpLine | BladeRule::PhpLine => {
                let found = find(memchr::memchr(b'>', rest));
                match (found, self.anchor.filter(|&at| at >= p)) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (a, b) => a.or(b),
                }
            }
            BladeRule::CommentPhpBlock
            | BladeRule::PhpBlock
            | BladeRule::PhpLineCode
            | BladeRule::EchoTriple
            | BladeRule::Echo
            | BladeRule::EchoRaw
            | BladeRule::EchoEscaped
            | BladeRule::Directive
            | BladeRule::DirectiveCustom
            | BladeRule::PhpSection
            | BladeRule::Attribute => None,
        }
    }

    /// The end pattern at `p` of a Blade rule whose text is not PHP.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_end_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let rule = rule.blade()?;
        let one = |len: usize| Hit::new(Pattern::End, p, p + len);
        match rule {
            BladeRule::Comment => self.starts(p, b"--}}").then(|| one(4)),
            BladeRule::CommentPhpWrap | BladeRule::PhpWrap => self.erb_wrap_end(p),
            BladeRule::CommentPhpLine | BladeRule::PhpLine => (self.at(p) == b'>').then(|| one(1)),
            BladeRule::ArgsComment | BladeRule::Balance => (self.at(p) == b')').then(|| one(1)),
            BladeRule::CommentPhpBlock
            | BladeRule::PhpBlock
            | BladeRule::PhpLineCode
            | BladeRule::EchoTriple
            | BladeRule::Echo
            | BladeRule::EchoRaw
            | BladeRule::EchoEscaped
            | BladeRule::Directive
            | BladeRule::DirectiveCustom
            | BladeRule::PhpSection
            | BladeRule::Attribute => None,
        }
    }

    /// The patterns at `p` of a Blade rule whose text is not PHP, in the
    /// grammar's order.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_patterns_at(&self, rule: Rule, p: usize) -> Option<Hit> {
        let rule = rule.blade()?;
        let text = &self.s[..self.lim];
        let hit = |(blade, end): (Blade, usize)| Hit::new(Pattern::blade(blade), p, end);
        match rule {
            BladeRule::Comment => comment_php(text, p).map(hit),
            BladeRule::CommentPhpWrap | BladeRule::PhpWrap => {
                php_open_end(text, p).map(|end| hit((Blade::PhpBlock, end)))
            }
            BladeRule::CommentPhpLine | BladeRule::PhpLine => {
                if !self.g(p) {
                    return None;
                }
                if let Some(question) = empty_tag(text, p) {
                    return Some(hit((Blade::EmptyTag(question), question + 1)));
                }
                Some(hit((Blade::LineCode, p)))
            }
            BladeRule::ArgsComment | BladeRule::Balance => {
                if self.at(p) == b'(' {
                    return Some(hit((Blade::Paren, p + 1)));
                }
                args_text_end(text, p, self.end_pos()).map(|end| hit((Blade::ArgsText, end)))
            }
            BladeRule::CommentPhpBlock
            | BladeRule::PhpBlock
            | BladeRule::PhpLineCode
            | BladeRule::EchoTriple
            | BladeRule::Echo
            | BladeRule::EchoRaw
            | BladeRule::EchoEscaped
            | BladeRule::Directive
            | BladeRule::DirectiveCustom
            | BladeRule::PhpSection
            | BladeRule::Attribute => None,
        }
    }

    /// The rule that a pattern of Blade's opens, or `None` for a match rule.
    pub(super) fn blade_opens(blade: Blade) -> Option<Rule> {
        Some(match blade {
            Blade::Comment => Rule::BladeComment,
            Blade::EchoTriple => Rule::BladeEchoTriple,
            Blade::Echo => Rule::BladeEcho,
            Blade::EchoRaw => Rule::BladeEchoRaw,
            Blade::EchoEscaped => Rule::BladeEchoEscaped,
            Blade::Directive(_) => Rule::BladeDirective,
            Blade::ArgsComment { .. } => Rule::BladeArgsComment,
            Blade::PhpSection => Rule::BladePhpSection,
            Blade::Custom(_) => Rule::BladeDirectiveCustom,
            Blade::Attribute(_) => Rule::BladeAttribute,
            Blade::PhpWrap => Rule::BladePhpWrap,
            Blade::PhpBlock => Rule::BladePhpBlock,
            Blade::PhpLine => Rule::BladePhpLine,
            Blade::CommentPhpWrap => Rule::BladeCommentPhpWrap,
            Blade::CommentPhpBlock => Rule::BladeCommentPhpBlock,
            Blade::CommentPhpLine => Rule::BladeCommentPhpLine,
            Blade::LineCode => Rule::BladePhpLineCode,
            Blade::Paren => Rule::BladeBalance,
            Blade::Keyword
            | Blade::EndPhp
            | Blade::Function
            | Blade::EmptyTag(_)
            | Blade::ArgsText => return None,
        })
    }

    /// Emits the capture groups of a Blade begin match, on the rule's name
    /// stack `base`.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_begin_captures(
        &mut self,
        base: u32,
        blade: Blade,
        start: usize,
        end: usize,
    ) {
        match blade {
            Blade::Comment => self.capture(base, start, end, &[S::BladeCommentBegin]),
            Blade::EchoTriple | Blade::Echo | Blade::EchoRaw => {
                self.capture(base, start, end, &[S::BladeConstructBegin]);
            }
            Blade::EchoEscaped => {
                self.capture(base, start, start + 1, &[S::BladeRoundBegin, S::BladeIndex]);
                self.capture(base, start + 1, end, &[S::BladeRoundBegin]);
            }
            Blade::Directive(paren)
            | Blade::ArgsComment {
                paren,
                ends_php: false,
            } => {
                self.capture(base, start, paren, &[S::BladeKeyword]);
                self.capture(base, paren, end, &[S::BladeArgsBegin]);
            }
            Blade::ArgsComment {
                paren,
                ends_php: true,
            } => {
                self.capture(base, start, paren, &[S::PhpEnd]);
                self.capture(base, paren, end, &[S::BladeArgsBegin]);
            }
            Blade::Custom(paren) => {
                self.capture(base, start, paren, &[S::BladeFunction]);
                self.capture(base, paren, end, &[S::BladeArgsBegin]);
            }
            Blade::Attribute(equals) => {
                let word = self.intern(start, equals);
                let attribute = self.dynamic(
                    base,
                    &[
                        Part::Lit("meta.attribute.unrecognized."),
                        Part::Name(word),
                        Part::Lit(".html"),
                    ],
                );
                let name = self.path(attribute, &[S::AttributeName]);
                self.capture_stack(base, start, equals, name);
                let separator = self.path(attribute, &[S::KeyValue]);
                self.capture_stack(base, equals, equals + 1, separator);
                let quote = self.path(attribute, &[S::StringDouble]);
                self.capture_stack(base, equals + 1, end, quote);
            }
            Blade::PhpWrap | Blade::CommentPhpWrap => {
                self.capture(base, start, end, &[S::PhpLeading]);
            }
            Blade::PhpSection
            | Blade::PhpBlock
            | Blade::PhpLine
            | Blade::CommentPhpBlock
            | Blade::CommentPhpLine => self.capture(base, start, end, &[S::PhpBegin]),
            Blade::Paren
            | Blade::LineCode
            | Blade::Keyword
            | Blade::EndPhp
            | Blade::Function
            | Blade::EmptyTag(_)
            | Blade::ArgsText => {}
        }
    }

    /// Emits the scopes of a Blade match rule, on the content `base` of the
    /// rule it sits in.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_match_captures(
        &mut self,
        base: u32,
        blade: Blade,
        start: usize,
        end: usize,
    ) {
        match blade {
            Blade::Keyword => self.capture(base, start, end, &[S::BladeKeyword]),
            Blade::EndPhp => self.capture(base, start, end, &[S::PhpEnd]),
            Blade::Function => self.capture(base, start, end, &[S::BladeFunction]),
            Blade::EmptyTag(question) => {
                let tag = self.path(base, &[S::PhpEmptyTag]);
                let code = self.path(tag, &[S::SourcePhp]);
                self.capture_stack(base, start, question, code);
                let punctuation = self.path(tag, &[S::PhpEnd, S::SourcePhp]);
                self.capture_stack(base, question, end, punctuation);
            }
            Blade::ArgsText => self.tok(base, end),
            Blade::Comment
            | Blade::EchoTriple
            | Blade::Echo
            | Blade::EchoRaw
            | Blade::EchoEscaped
            | Blade::Directive(_)
            | Blade::ArgsComment { .. }
            | Blade::PhpSection
            | Blade::Custom(_)
            | Blade::Attribute(_)
            | Blade::PhpWrap
            | Blade::PhpBlock
            | Blade::PhpLine
            | Blade::CommentPhpWrap
            | Blade::CommentPhpBlock
            | Blade::CommentPhpLine
            | Blade::LineCode
            | Blade::Paren => {}
        }
    }

    /// Emits the capture groups of a Blade rule's end match, on its name stack
    /// `base`.
    #[cold]
    #[inline(never)]
    pub(super) fn blade_end_captures(&mut self, base: u32, rule: Rule, start: usize, end: usize) {
        let Some(rule) = rule.blade() else {
            return;
        };
        match rule {
            BladeRule::Comment => self.capture(base, start, end, &[S::BladeCommentEnd]),
            BladeRule::CommentPhpWrap | BladeRule::PhpWrap if end > start => {
                self.capture(base, start, end, &[S::PhpTrailing]);
            }
            BladeRule::CommentPhpBlock | BladeRule::PhpBlock => {
                self.capture(base, start, start + 1, &[S::PhpEnd, S::SourcePhp]);
                self.capture(base, start + 1, end, &[S::PhpEnd]);
            }
            BladeRule::PhpLineCode => self.capture(base, start, end, &[S::PhpEnd, S::SourcePhp]),
            BladeRule::CommentPhpLine | BladeRule::PhpLine => {
                self.capture(base, start, end, &[S::PhpEnd]);
            }
            BladeRule::EchoTriple | BladeRule::Echo | BladeRule::EchoRaw => {
                self.capture(base, start, end, &[S::BladeConstructEnd]);
            }
            BladeRule::EchoEscaped => self.capture(base, start, end, &[S::BladeRoundEnd]),
            BladeRule::Directive | BladeRule::DirectiveCustom | BladeRule::ArgsComment => {
                self.capture(base, start, end, &[S::BladeArgsEnd]);
            }
            BladeRule::Attribute => {
                self.capture(base, start, end, &[S::StringDouble, S::StringEnd]);
            }
            BladeRule::CommentPhpWrap
            | BladeRule::PhpWrap
            | BladeRule::Balance
            | BladeRule::PhpSection => {}
        }
    }
}

/// The first of Blade's own rules that matches at `p` of `s`.
/// The rules are the injection's patterns without the PHP tags, as a Blade
/// heredoc includes them (see [`super::state::ROOT_BLADE_HEREDOC`]).
pub(super) fn rules_at(s: &[u8], p: usize) -> Option<(Blade, usize)> {
    at(s, p)
        .filter(|&(blade, _)| !matches!(blade, Blade::PhpWrap | Blade::PhpBlock | Blade::PhpLine))
}

/// The first place at or after `from` where one of Blade's own rules
/// matches in `s`, as [`rules_at`] gives them.
pub(super) fn find_rules(s: &[u8], from: usize) -> Option<usize> {
    let mut p = from;
    loop {
        if rules_at(s, p).is_some() {
            return Some(p);
        }
        p += 1;
        p += s
            .get(p..)?
            .iter()
            .position(|b| matches!(b, b'{' | b'@' | b':'))?;
    }
}

/// The first place at or after `from` where one of the injection's patterns
/// matches in `s`.
pub(super) fn find(s: &[u8], from: usize) -> Option<usize> {
    let mut p = from;
    if p == 0 {
        if at(s, 0).is_some() {
            return Some(0);
        }
        p = 1;
    }
    while let Some(i) = s
        .get(p..)?
        .iter()
        .position(|b| matches!(b, b'{' | b'@' | b':' | b'<'))
    {
        let q = p + i;
        if at(s, q).is_some() {
            return Some(q);
        }
        p = q + 1;
    }
    None
}

/// The state of the PHP of a Blade rule whose text sits on `base`.
#[cfg(feature = "lang-blade")]
pub(super) fn php_start(arena: &mut crate::arena::Arena, base: u32) -> u32 {
    crate::blade::php_start(arena, base)
}

/// Lexes the PHP of a Blade rule from `state` up to `halts`.
#[cfg(feature = "lang-blade")]
pub(super) fn lex_php(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
    halts: crate::language::Halts<'_>,
) -> (crate::limits::Finished, Option<crate::language::Stopped>) {
    crate::blade::lex_php_until(cx, line, state, halts)
}

/// Without the Blade feature no Blade document is lexed, so no PHP is open.
#[cfg(not(feature = "lang-blade"))]
pub(super) fn php_start(_arena: &mut crate::arena::Arena, _base: u32) -> u32 {
    unreachable!("Blade's PHP opens only in a Blade document")
}

/// Without the Blade feature no Blade document is lexed, so no PHP is open.
#[cfg(not(feature = "lang-blade"))]
pub(super) fn lex_php(
    _cx: &mut crate::language::Context<'_>,
    _line: crate::language::Line<'_>,
    _state: u32,
    _halts: crate::language::Halts<'_>,
) -> (crate::limits::Finished, Option<crate::language::Stopped>) {
    unreachable!("Blade's PHP opens only in a Blade document")
}
