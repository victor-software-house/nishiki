//! Blade's copy of PHP on the shared rule engine ([`crate::rules`]): the
//! grammar's types, and where an embedded language hands the line back.

use std::num::NonZeroU32;

use super::grammar::{Pattern, Rule, Sc};
use super::matchers::{name_char_at, name_start_at};
use super::scope::S;
use crate::arena::Arena;
use crate::php_family::halts::{quoted_to, sql_common};
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, Captures, Grammar, NameCase, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The lexer of Blade's PHP for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Blade>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Blade>;

/// The PHP that Shiki's `blade` grammar holds, its `language` entry.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Blade;

impl Grammar for Blade {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Rule;
    /// The place of the open rule nearest the root that shuts Blade's
    /// injection out ([`shuts_out`]).
    /// The root never does, so the place is never 0.
    type Extra = Option<NonZeroU32>;
    type Kept = Option<NonZeroU32>;

    const NAME: &'static str = "Blade";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A heredoc's begin, `(<<<)\s*("?)(HTML)(\2)(\s*)$`, takes the line feed.
    const CAPTURED_EOL: bool = true;
    /// Blade's injection holds inside the PHP of an echo at every depth.
    const DEPTH_HALTS: bool = true;
    const WATCHES_FRAMES: bool = true;

    fn scope(scope: Sc) -> ScopeName {
        match scope {
            Sc::S(scope) => ScopeName::Fixed(scope.scope()),
            Sc::Group(prefix, group, suffix) => ScopeName::Capture {
                prefix,
                group,
                suffix,
            },
        }
    }

    /// Every name the grammar builds from a capture lowers it, as
    /// `keyword.control.${1:/downcase}.php` does.
    fn dyn_name_case(_prefix: &'static str) -> NameCase {
        NameCase::Lowered
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<Sc> {
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.definition().content
    }

    const EMBEDS: bool = true;

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
    }

    fn pattern_act(pattern: Pattern) -> Act<Self> {
        pattern.definition().act
    }

    fn pattern_captures(pattern: Pattern) -> Captures<Self> {
        let definition = pattern.definition();
        (definition.captures, definition.nested)
    }

    fn pattern_definition(pattern: Pattern) -> (Act<Self>, Captures<Self>) {
        let definition = pattern.definition();
        (definition.act, (definition.captures, definition.nested))
    }

    /// The scan of the pattern tables.
    /// A match of a word table's head is the list that holds the word first.
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pattern, usize)> {
        let (start, pattern, end) = table_scan(lexer)?;
        match pattern.words() {
            Some((table, 0)) => {
                let list = lexer.word_list(table.words(), start)?;
                Some((start, Pattern::of_list(table, list), end))
            }
            Some(_) | None => Some((start, pattern, end)),
        }
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }

    fn extra(kept: Option<NonZeroU32>) -> Option<NonZeroU32> {
        kept
    }

    fn keep(extra: Option<NonZeroU32>) -> Option<NonZeroU32> {
        extra
    }

    fn frames_opened(lexer: &mut Lexer<'_>) {
        lexer.x = lexer
            .open_rules()
            .filter(|&(_, rule)| shuts_out(rule))
            .last()
            .and_then(|(at, _)| place(at));
    }

    fn frame_pushed(lexer: &mut Lexer<'_>) {
        let innermost = lexer.open_rules().next();
        if lexer.x.is_none()
            && let Some((at, rule)) = innermost
            && shuts_out(rule)
        {
            lexer.x = place(at);
        }
    }

    fn frames_popped(lexer: &mut Lexer<'_>) {
        let open = lexer.open_rules().next().map_or(0, |(at, _)| at + 1);
        if lexer
            .x
            .is_some_and(|at| usize::try_from(at.get()).is_ok_and(|at| at >= open))
        {
            lexer.x = None;
        }
    }

    fn deep_halts_open(lexer: &Lexer<'_>) -> bool {
        lexer.x.is_none()
    }
}

/// Where an embedding rule takes the line back from the language it embeds:
/// its end, and the patterns it lists before that language.
/// No rule lists patterns after the language.
#[derive(Clone, Copy, Debug)]
pub(crate) enum End {
    /// A heredoc, `<<<LABEL`: the line that closes it, or an interpolation.
    Heredoc(&'static [u8]),
    /// A nowdoc, `<<<'LABEL'`: the line that closes it.
    Nowdoc(&'static [u8]),
    /// SQL in a double-quoted string: its closing `"`, a comment, an escape,
    /// a quote that opens a SQL string, or an interpolation.
    DoubleSql,
    /// SQL in a single-quoted string: its closing `'`, a comment, an escape,
    /// `N'`, or a quote that opens a SQL string.
    SingleSql,
    /// A rule that embeds nothing.
    Never,
}

impl crate::language::Halt for End {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds(s, p))
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        match *self {
            End::Heredoc(label) => closes(s, p, label) || interpolation(s, p),
            End::Nowdoc(label) => closes(s, p, label),
            End::DoubleSql => {
                matches!(s.get(p), Some(b'"' | b'#' | b'\'' | b'`'))
                    || sql_common(s, p)
                    || interpolation(s, p)
            }
            End::SingleSql => match s.get(p) {
                Some(b'\'' | b'#') => true,
                Some(b'N') => s.get(p + 1) == Some(&b'\''),
                Some(b'`') => quoted_to(s, p + 1, b'`', b'\''),
                Some(b'"') => quoted_to(s, p + 1, b'"', b'\''),
                _ => sql_common(s, p),
            },
            End::Never => false,
        }
    }
}

impl crate::php_family::PhpFamily for Blade {}

/// The place `at` of an open rule, which is never 0 for a rule above the
/// root.
fn place(at: usize) -> Option<NonZeroU32> {
    u32::try_from(at).ok().and_then(NonZeroU32::new)
}

/// Whether `rule` shuts Blade's injection out: its name holds
/// `meta.embedded`, as the heredocs and nowdocs named for a language do.
fn shuts_out(rule: Rule) -> bool {
    matches!(
        rule,
        Rule::HeredocInteriorAt0
            | Rule::HeredocInteriorAt1
            | Rule::HeredocInteriorAt2
            | Rule::HeredocInteriorAt3
            | Rule::HeredocInteriorAt4Javascript
            | Rule::HeredocInteriorAt4Js
            | Rule::HeredocInteriorAt5
            | Rule::HeredocInteriorAt6
            | Rule::NowdocInteriorAt0
            | Rule::NowdocInteriorAt1
            | Rule::NowdocInteriorAt2
            | Rule::NowdocInteriorAt3
            | Rule::NowdocInteriorAt4Javascript
            | Rule::NowdocInteriorAt4Js
            | Rule::NowdocInteriorAt5
            | Rule::NowdocInteriorAt6
    )
}

impl rules::Embeds for Blade {
    type End = End;
    const HALTS: bool = true;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (End, Option<rules::FnHalt>) {
        (End::Never, None)
    }

    fn embed_halts_for(
        rule: Rule,
        _: &rules::Embedding,
        _: &Arena,
    ) -> (End, Option<rules::FnHalt>) {
        (rule.end_halt(), None)
    }

    /// A Blade heredoc's HTML runs on the HTML lexer up to its halts, which
    /// Blade's row does not offer for a whole document.
    fn lex_embedded_until(
        _: Rule,
        language: crate::Language,
        cx: &mut crate::language::Context<'_>,
        line: crate::language::Line<'_>,
        state: u32,
        (end, after): (
            &dyn crate::language::Halt,
            Option<&dyn crate::language::Halt>,
        ),
    ) -> Option<(crate::limits::Finished, Option<crate::language::Stopped>)> {
        (language == crate::Language::Blade)
            .then(|| crate::html::lex_blade_until(cx, line, state, end, after))
    }
}

/// `^\s*(LABEL)\b`: the line that closes a heredoc or a nowdoc, which
/// matches only at the line's start.
/// `\b` after the label's last letter needs a character past it that is no
/// word character.
fn closes(s: &[u8], p: usize, label: &[u8]) -> bool {
    if p != 0 {
        return false;
    }
    let at = s.iter().position(|&b| !is_space(b)).unwrap_or(s.len());
    s[at..].starts_with(label) && !word_at(s, at + label.len())
}

/// Whether a word character, `\w`, stands at `i` of `s`.
fn word_at(s: &[u8], i: usize) -> bool {
    crate::utf8::char_at(s, i).is_some_and(|(c, _)| crate::classes::is_word(c))
}

/// The patterns of `#interpolation`, in a heredoc or a double-quoted string:
/// - `\\[0-7]{1,3}`, `\\x\h{1,2}`, `\\u\{\h+}` and `\\["$\\efnrtv]`;
/// - `\{(?=\$.*?})`;
/// - `$name`, which `#variable-name` starts with in each of its forms, and
///   `${name}`.
///
/// Blade's names take `[_a-z\x7F-ÿ]`, so a name ends at a character past `ÿ`.
fn interpolation(s: &[u8], p: usize) -> bool {
    let next = s.get(p + 1).copied();
    match s.get(p) {
        Some(b'\\') => match next {
            Some(b'0'..=b'7' | b'"' | b'$' | b'\\' | b'e' | b'f' | b'n' | b'r' | b't' | b'v') => {
                true
            }
            Some(b'x') => s.get(p + 2).is_some_and(u8::is_ascii_hexdigit),
            Some(b'u') => {
                let digits = s.get(p + 3..).map_or(0, |rest| {
                    rest.iter().take_while(|b| b.is_ascii_hexdigit()).count()
                });
                s.get(p + 2) == Some(&b'{') && digits > 0 && s.get(p + 3 + digits) == Some(&b'}')
            }
            _ => false,
        },
        Some(b'{') => {
            next == Some(b'$')
                && s[p + 2..]
                    .iter()
                    .take_while(|&&b| b != b'\n')
                    .any(|&b| b == b'}')
        }
        Some(b'$') => match next {
            Some(b'{') => {
                let Some(mut end) = name_start_at(s, p + 2) else {
                    return false;
                };
                while let Some(next) = name_char_at(s, end) {
                    end = next;
                }
                s.get(end) == Some(&b'}')
            }
            Some(_) => name_start_at(s, p + 1).is_some(),
            None => false,
        },
        _ => false,
    }
}

/// `\s`.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}
