//! Python on the shared rule engine ([`crate::rules`]): the grammar's types
//! and the lookahead answers its matchers keep for a line.

use std::cell::Cell;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Python lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Python>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Python>;

/// The capture groups the grammar tokenizes with patterns of their own:
/// none.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Nested {}

/// The `python` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Python;

/// What a lookahead's search for an [`Ahead`] thing has found so far.
#[derive(Clone, Copy, Debug)]
enum Seen {
    Unknown,
    Absent,
    At(usize),
}

/// A thing a lookahead looks for on the rest of the line, found once for the
/// whole line.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Ahead {
    /// The last `}`.
    CloseBrace,
    /// The last `{`.
    OpenBrace,
    /// The last `]`.
    CloseBracket,
    /// The last `import`.
    Import,
    /// The last `%}`.
    PercentBrace,
    /// The last `:` or `\`.
    ColonOrBackslash,
}

impl Ahead {
    /// How many things there are to look for.
    const COUNT: usize = 6;

    /// Its slot in [`Extra`].
    fn slot(self) -> usize {
        match self {
            Ahead::CloseBrace => 0,
            Ahead::OpenBrace => 1,
            Ahead::CloseBracket => 2,
            Ahead::Import => 3,
            Ahead::PercentBrace => 4,
            Ahead::ColonOrBackslash => 5,
        }
    }
}

/// The bytes [`Lexer::next_stop`] stops at.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Stops {
    /// A quote or `}`.
    QuoteOrClose,
    /// A quote, `{` or `}`.
    QuoteOrBrace,
}

impl Stops {
    /// Its slot in [`Extra`].
    fn slot(self) -> usize {
        match self {
            Stops::QuoteOrClose => 0,
            Stops::QuoteOrBrace => 1,
        }
    }
}

/// What the Python matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of word characters found: where it starts and ends.
    /// A search from inside the run ends where the run does.
    pub(crate) word_run: Cell<(usize, usize)>,
    /// The last word looked up in the word lists: where it starts and which
    /// lists have it.
    pub(crate) word_mask: Cell<(usize, u8)>,
    /// The last place each [`Ahead`] thing stands, once looked for.
    ahead: [Cell<Seen>; Ahead::COUNT],
    /// The last search for a quote or `}`, and one for a quote, `{` or `}`:
    /// where it started and what it found.
    pub(crate) stops: [Cell<(usize, usize)>; 2],
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            word_run: Cell::new((usize::MAX, 0)),
            word_mask: Cell::new((usize::MAX, 0)),
            ahead: [const { Cell::new(Seen::Unknown) }; Ahead::COUNT],
            stops: [const { Cell::new((usize::MAX, 0)) }; 2],
        }
    }
}

impl Lexer<'_> {
    /// Where `what` last stands in the line.
    /// A lookahead for it then asks one question, however many positions it
    /// is tried at.
    pub(crate) fn last(&self, what: Ahead) -> Option<usize> {
        let slot = &self.x.ahead[what.slot()];
        match slot.get() {
            Seen::At(place) => return Some(place),
            Seen::Absent => return None,
            Seen::Unknown => {}
        }
        self.spend(self.s.len());
        let found = match what {
            Ahead::CloseBrace => memchr::memrchr(b'}', self.s),
            Ahead::OpenBrace => memchr::memrchr(b'{', self.s),
            Ahead::CloseBracket => memchr::memrchr(b']', self.s),
            Ahead::Import => memchr::memmem::rfind(self.s, b"import"),
            Ahead::PercentBrace => memchr::memmem::rfind(self.s, b"%}"),
            Ahead::ColonOrBackslash => {
                memchr::memrchr(b':', self.s).max(memchr::memrchr(b'\\', self.s))
            }
        };
        slot.set(found.map_or(Seen::Absent, Seen::At));
        found
    }

    /// The first position from `from` where one of `stops` stands.
    /// It is the end of the line when none does.
    pub(crate) fn next_stop(&self, from: usize, stops: Stops) -> usize {
        let slot = &self.x.stops[stops.slot()];
        let (start, answer) = slot.get();
        if start <= from && from <= answer {
            return answer;
        }
        let rest = self.s.get(from..).unwrap_or(&[]);
        let found = match stops {
            Stops::QuoteOrBrace => rest
                .iter()
                .position(|b| matches!(b, b'"' | b'\'' | b'{' | b'}')),
            Stops::QuoteOrClose => memchr::memchr3(b'"', b'\'', b'}', rest),
        };
        self.spend(found.map_or(rest.len(), |at| at + 1));
        let answer = found.map_or(self.s.len(), |at| from + at);
        slot.set((from, answer));
        answer
    }
}

impl Grammar for Python {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Python";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 7;

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
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.data().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.data().backref.map_or(Payload::None, Payload::Backref)
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

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        table_scan(lexer)
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
