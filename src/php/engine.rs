//! PHP on the shared rule engine ([`crate::rules`]): the grammar's types, and
//! where an embedded language hands the line back.

use super::grammar::{Pat, Rule, Sc};
use super::scope::S;
use crate::arena::Arena;
use crate::php_family::halts::{quoted_to, sql_common};
use crate::php_family::matchers::{is_name, is_name_start};
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, Captures, Grammar, NameCase, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The PHP lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Php>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Php>;

/// The `php` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Php;

impl Grammar for Php {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "PHP";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A heredoc's begin, `(<<<)\s*("?)(HTML)(\2)(\s*)$`, takes the line feed.
    const CAPTURED_EOL: bool = true;

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
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.data().content
    }

    const EMBEDS: bool = true;

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        pat.data().act
    }

    fn pat_captures(pat: Pat) -> Captures<Self> {
        let data = pat.data();
        (data.captures, data.nested)
    }

    fn pat_data(pat: Pat) -> (Act<Self>, Captures<Self>) {
        let data = pat.data();
        (data.act, (data.captures, data.nested))
    }

    /// The scan of the pattern tables.
    /// A match of a word table's head is the list that holds the word first.
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        let (start, pat, end) = table_scan(lexer)?;
        match pat.words() {
            Some((table, 0)) => {
                let list = lexer.word_list(table.words(), start)?;
                Some((start, Pat::of_list(table, list), end))
            }
            Some(_) | None => Some((start, pat, end)),
        }
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}

/// Where an embedding rule takes the line back from the language it embeds:
/// its end, and the patterns it lists before that language.
/// No PHP rule lists patterns after the language, except the Markdown fence,
/// which [`rules::Embeds::embeds_after_own`] covers.
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
    /// A rule that embeds nothing, or the Markdown fence, which has no end.
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

impl crate::php_family::PhpFamily for Php {}

impl rules::Embeds for Php {
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

    fn embeds_after_own(rule: Rule) -> bool {
        rule.embeds_after_own()
    }
}

/// `^\s*(LABEL)(?![0-9A-Z_a-z\x7F-\x{10FFFF}])`: the line that closes a
/// heredoc or a nowdoc, which matches only at the line's start.
fn closes(s: &[u8], p: usize, label: &[u8]) -> bool {
    if p != 0 {
        return false;
    }
    let at = s.iter().position(|&b| !is_space(b)).unwrap_or(s.len());
    s[at..].starts_with(label) && !s.get(at + label.len()).copied().is_some_and(is_name)
}

/// The patterns of `#interpolation`, in a heredoc or a double-quoted string:
/// - `\\[0-7]{1,3}`, `\\x\h{1,2}`, `\\u\{\h+}` and `\\[$\\efnrtv]`;
/// - `\{(?=\$.*?})`;
/// - `$name`, which `#variable-name` starts with in each of its forms, and
///   `${name}`.
fn interpolation(s: &[u8], p: usize) -> bool {
    let next = s.get(p + 1).copied();
    match s.get(p) {
        Some(b'\\') => match next {
            Some(b'0'..=b'7' | b'$' | b'\\' | b'e' | b'f' | b'n' | b'r' | b't' | b'v') => true,
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
                let name = s.get(p + 2).copied().is_some_and(is_name_start);
                let end = s[p + 2..]
                    .iter()
                    .position(|&b| !is_name(b))
                    .map_or(s.len(), |n| p + 2 + n);
                name && s.get(end) == Some(&b'}')
            }
            Some(b) => is_name_start(b),
            None => false,
        },
        _ => false,
    }
}

/// `\s`.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}
