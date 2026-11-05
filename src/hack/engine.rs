//! Hack on the shared rule engine ([`crate::rules`]): the grammar's types, and
//! where an embedded language hands the line back.

use super::grammar::{Pat, Rule, Sc};
use super::scope::S;
use crate::arena::Arena;
use crate::php_family::halts::{quoted_to, sql_common};
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Hack lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Hack>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Hack>;

/// The `hack` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hack;

impl Grammar for Hack {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "Hack";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A doc comment's begin, `/\*\*(?:#@\+)?\s*$`, and a heredoc's take the
    /// line feed.
    const CAPTURED_EOL: bool = true;

    /// No scope of the grammar is built from a capture.
    fn scope(scope: Sc) -> ScopeName {
        let Sc::S(scope) = scope;
        ScopeName::Fixed(scope.scope())
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(kind, from, to, base);
    }
}

/// Where an embedding rule takes the line back from the language it embeds:
/// its end, and the patterns it lists before that language.
/// The `Document` rule lists HTML first and Hack's patterns after it, which
/// [`rules::Embeds::embeds_after_own`] covers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum End {
    /// SQL in a double-quoted string: its closing `"`, a `(`, a comment, an
    /// escape, a quote, or an interpolation.
    DoubleSql,
    /// SQL in a single-quoted string: its closing `'`, a `(`, a comment, an
    /// escape, or a quote that opens a SQL string.
    SingleSql,
    /// A rule that embeds nothing, or the `Document` rule, which has no end.
    Never,
}

impl crate::language::Halt for End {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds(s, p))
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        match *self {
            End::DoubleSql => {
                matches!(s.get(p), Some(b'"' | b'(' | b'#' | b'\'' | b'`'))
                    || sql_common(s, p)
                    || interpolation(s, p)
            }
            End::SingleSql => match s.get(p) {
                Some(b'\'' | b'(' | b'#') => true,
                Some(b'`') => quoted_to(s, p + 1, b'`', b'\''),
                Some(b'"') => quoted_to(s, p + 1, b'"', b'\''),
                _ => sql_common(s, p),
            },
            End::Never => false,
        }
    }
}

impl crate::php_family::PhpFamily for Hack {}

impl rules::Embeds for Hack {
    type End = End;

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

/// The patterns of `#interpolation` in a double-quoted string:
/// - `\\[0-7]{1,3}`, `\\x\h{1,2}` and `\\["$\\nrt]`;
/// - `(\{\$.*?})`;
/// - `(\$[A-Z_a-z][0-9A-Z_a-z]*…)`.
fn interpolation(s: &[u8], p: usize) -> bool {
    let next = s.get(p + 1).copied();
    match s.get(p) {
        Some(b'\\') => match next {
            Some(b'0'..=b'7' | b'"' | b'$' | b'\\' | b'n' | b'r' | b't') => true,
            Some(b'x') => s.get(p + 2).is_some_and(u8::is_ascii_hexdigit),
            _ => false,
        },
        Some(b'{') => {
            next == Some(b'$')
                && s[p + 2..]
                    .iter()
                    .take_while(|&&b| b != b'\n')
                    .any(|&b| b == b'}')
        }
        Some(b'$') => next.is_some_and(|b| b == b'_' || b.is_ascii_alphabetic()),
        _ => false,
    }
}
