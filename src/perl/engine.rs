//! Perl on the shared rule engine ([`crate::rules`]): the grammar's types, and
//! where an embedded language hands the line back.

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::arena::Arena;
use crate::classes::{is_ascii_word, is_word};
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;
use crate::utf8::{char_at, char_start};

/// The Perl lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Perl>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Perl>;

/// How many capture groups can be lexed again inside one another.
/// A heredoc's opener lexes the rest of its line again, and that rest can
/// hold another opener, so the depth follows the text.
/// Past it, a group keeps the scopes around it, and the line reports the
/// depth limit.
pub(crate) const MAX_NESTED: u32 = 64;

/// What a line of Perl keeps while it is lexed.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// How many capture groups are being lexed again around this point.
    nested: u32,
}

/// The `perl` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Perl;

impl Grammar for Perl {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Rule;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Perl";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A heredoc's begin, `(.*)\n?` after its label, takes the line feed.
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.definition().content
    }

    const EMBEDS: bool = true;

    fn rule_carried_state(rule: Rule) -> CarriedState {
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

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pattern, usize)> {
        table_scan(lexer)
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    /// Past [`MAX_NESTED`] groups lexed again around it, the text keeps the
    /// scopes around it.
    /// The line then reports the depth limit.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        if lexer.x.nested >= MAX_NESTED {
            lexer.record_depth();
            lexer.emit(to, base);
            return;
        }
        lexer.x.nested += 1;
        lexer.tokenize_capture(nested, from, to, base);
        lexer.x.nested -= 1;
    }
}

/// Where an embedding rule takes the line back from the language it embeds:
/// its end, and the patterns it lists before that language.
/// No Perl rule lists patterns after the language it embeds.
#[derive(Clone, Copy, Debug)]
pub(crate) enum End {
    /// A line of a raw heredoc, `^` to `\n`: its line feed.
    LineFeed,
    /// A line of an interpolated heredoc: its line feed, an escape or a
    /// variable.
    Interpolated,
    /// A POD `=begin html` block: `^(?:(=end)\s+(html)|(?==cut))`, a line
    /// that starts with `=end` and `html`, or with `=cut`.
    PodHtml,
    /// A rule that embeds nothing.
    Never,
}

impl crate::language::Halt for End {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        match *self {
            End::LineFeed | End::Interpolated => (from..=to).find(|&p| self.holds(s, p)),
            End::PodHtml => (from == 0 && self.holds(s, 0)).then_some(0),
            End::Never => None,
        }
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        match *self {
            End::LineFeed => matches!(s.get(p), Some(b'\n') | None),
            End::Interpolated => {
                matches!(s.get(p), Some(b'\n') | None) || escape(s, p) || variable(s, p)
            }
            End::PodHtml => p == 0 && pod_html_ends(s),
            End::Never => false,
        }
    }
}

/// `\\.`, which every pattern of `#escaped_char` starts with: a backslash
/// and a character other than the line feed.
fn escape(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'\\') && s.get(p + 1).is_some_and(|&b| b != b'\n')
}

/// Whether one of the patterns of `#variable` matches at `p`, as the
/// matchers in `matchers/variables.rs` decide it.
fn variable(s: &[u8], p: usize) -> bool {
    let no_word = |i: usize| !s.get(i).copied().is_some_and(is_ascii_word);
    match s.get(p) {
        Some(b'$') => {
            let next = s.get(p + 1).copied();
            let mark = next.is_some_and(|b| {
                matches!(b, b'&' | b'`' | b'\'' | b'+' | b'"' | b'0') || is_special(b)
            });
            let digits = s.get(p + 1..).map_or(0, |rest| {
                rest.iter().take_while(|b| b.is_ascii_digit()).count()
            });
            (mark && no_word(p + 2))
                || (digits > 0 && no_word(p + 1 + digits))
                || braced(s, p + 1)
                || named(s, p)
                || short(s, p)
        }
        Some(b'%' | b'@') => named(s, p) || short(s, p),
        _ => false,
    }
}

/// `[!#$%()*,-/:-@\[-_ab|~]`: the punctuation of a special variable.
pub(super) fn is_special(b: u8) -> bool {
    matches!(
        b,
        b'!' | b'#' | b'$' | b'%' | b'(' | b')' | b'*' | b','..=b'/' | b':'..=b'@' | b'['..=b'_'
            | b'a' | b'b' | b'|' | b'~'
    )
}

/// Where a name starts after the sigil at `p`, past its `#` if it has one.
fn name_start(s: &[u8], p: usize) -> usize {
    if s.get(p + 1) == Some(&b'#') {
        p + 2
    } else {
        p + 1
    }
}

/// One unit of `([$0-9A-Z_a-z]|::)` at `i`, or of `([$7A-Za-z]|::)` when
/// `first`.
fn name_unit(s: &[u8], i: usize, first: bool) -> Option<usize> {
    let b = *s.get(i)?;
    let one = if first {
        b == b'$' || b == b'7' || b.is_ascii_alphabetic()
    } else {
        b == b'$' || is_ascii_word(b)
    };
    if one {
        Some(i + 1)
    } else {
        s.get(i..)
            .is_some_and(|rest| rest.starts_with(b"::"))
            .then_some(i + 2)
    }
}

/// `([$%@](#)?)([$7A-Za-z]|::)([$0-9A-Z_a-z]|::)*\b` at `p`.
fn named(s: &[u8], p: usize) -> bool {
    let Some(mut at) = name_unit(s, name_start(s, p), true) else {
        return false;
    };
    loop {
        if boundary(s, at) {
            return true;
        }
        match name_unit(s, at, false) {
            Some(next) => at = next,
            None => return false,
        }
    }
}

/// `(\$\{)(?:[$7A-Za-z]|::)(?:[$0-9A-Z_a-z]|::)*(})` after the `$` at
/// `p - 1`.
fn braced(s: &[u8], p: usize) -> bool {
    if s.get(p) != Some(&b'{') {
        return false;
    }
    let Some(mut at) = name_unit(s, p + 1, true) else {
        return false;
    };
    while let Some(next) = name_unit(s, at, false) {
        at = next;
    }
    s.get(at) == Some(&b'}')
}

/// `([$%@](#)?)[0-9_]\b` at `p`.
fn short(s: &[u8], p: usize) -> bool {
    let at = name_start(s, p);
    s.get(at).is_some_and(|&b| b.is_ascii_digit() || b == b'_') && boundary(s, at + 1)
}

/// Whether a `\w` character stands at `i`.
fn word_at(s: &[u8], i: usize) -> bool {
    match s.get(i) {
        Some(&b) if b < 0x80 => is_ascii_word(b),
        Some(_) => char_at(s, i).is_some_and(|(c, _)| is_word(c)),
        None => false,
    }
}

/// The regex `\b` at `i`.
fn boundary(s: &[u8], i: usize) -> bool {
    let before = i > 0 && word_at(s, char_start(s, i - 1));
    before != word_at(s, i)
}

/// `(=end)\s+(html)|(?==cut)` at the start of `s`.
fn pod_html_ends(s: &[u8]) -> bool {
    if s.starts_with(b"=cut") {
        return true;
    }
    let Some(rest) = s.strip_prefix(b"=end") else {
        return false;
    };
    let blanks = rest.iter().take_while(|&&b| is_space(b)).count();
    blanks > 0 && rest[blanks..].starts_with(b"html")
}

impl rules::Embeds for Perl {
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
}

/// `\s` for an ASCII byte: the space, the tab, the line feed, the vertical
/// tab, the form feed and the carriage return.
pub(super) fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r')
}
