//! The compiled form of a `TextMate` grammar: static tables that
//! `oracle/engine-tables.ts` writes, with the includes already followed.
//!
//! Nothing is parsed at run time. A pattern's text is compiled to a regular
//! expression the first time a search needs it.
//!
//! The tables keep what never changes apart from what a search fills in: a
//! pattern's text and a list's rules are read-only data, while the compiled
//! patterns and the lists' scanners sit in cells. Each grammar allocates its
//! cells once per process, on first use.
//!
//! The blocks are on the heap, not in static arrays, because an empty
//! `OnceLock` is not all-zero bytes: `std`'s `Once` starts at a non-zero
//! state, so a static array of cells would sit in the shipped file's data.

use std::sync::OnceLock;

use super::regex::{LineKind, Regex, Sketch};
use super::scanner::Scanner;
use crate::language::Language;

/// The index that stands for no rule, pattern or name.
pub(crate) const NONE: usize = usize::MAX;

/// A scope-name reference with this bit set names a scope built from the
/// capture groups of a match, such as `keyword.$1`.
pub(crate) const DYNAMIC_NAME: u16 = 0x8000;

/// What a rule does when its pattern matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RuleKind {
    /// One pattern that scopes its text.
    Match,
    /// A `begin` pattern that opens a context and an `end` pattern that closes
    /// it.
    BeginEnd,
    /// A `begin` pattern that opens a context that lasts while a `while`
    /// pattern keeps matching at the start of each line.
    BeginWhile,
    /// Only a list of patterns: the root of a grammar, or a capture's rules.
    Container,
}

/// What a capture group of a match does.
#[derive(Debug)]
pub(crate) struct Cap {
    /// The group's number.
    pub(crate) index: usize,
    /// The scopes its text gets.
    pub(crate) name: &'static [u16],
    /// The scopes the rules inside it get, when it holds some.
    pub(crate) content_name: &'static [u16],
    /// The rule that lexes its text again, or [`NONE`].
    pub(crate) retokenize: usize,
}

/// One rule of a grammar.
#[derive(Debug)]
pub(crate) struct Rule {
    /// What the rule does.
    pub(crate) kind: RuleKind,
    /// The scopes the rule's text gets.
    pub(crate) name: &'static [u16],
    /// The scopes the rules inside a context get.
    pub(crate) content_name: &'static [u16],
    /// The pattern that opens the rule: the only one of a `Match`.
    pub(crate) begin: usize,
    /// The pattern that closes a `BeginEnd`.
    pub(crate) end: usize,
    /// The pattern a `BeginWhile` checks at the start of each line.
    pub(crate) until: usize,
    /// The groups of the `begin` or `match` pattern.
    pub(crate) captures: &'static [Cap],
    /// The groups of the `end` or `while` pattern.
    pub(crate) end_captures: &'static [Cap],
    /// The rules that can start inside the rule, in order, includes
    /// followed: an index into [`Grammar::lists`].
    pub(crate) list: usize,
    /// Whether the `end` pattern is tried after the patterns, not before.
    pub(crate) apply_end_last: bool,
}

/// An ordered list of rules that can start at one place. Its scanner, which
/// finds the next of them in a line, is kept in [`Cells::scanners`].
#[derive(Debug)]
pub(crate) struct List {
    /// The rules, in the order of the grammar.
    pub(crate) rules: &'static [usize],
}

impl List {
    /// A list of `rules`.
    pub(crate) const fn new(rules: &'static [usize]) -> List {
        List { rules }
    }
}

/// An include of a whole grammar that nishiki lexes by hand. The rule whose
/// list held the include opens a region. That language lexes the region. The
/// region ends where the rule's end matches. A rule listed around the include
/// also takes the line back where it matches.
///
/// [`Grammar::list_hand_offs`] names each list's hand-off. A [`List`] does
/// not. A [`List`] then stays two words, and the scans that index the lists
/// pay nothing. The list holds the rules before the include, then the rules
/// after it.
#[derive(Debug)]
pub(crate) struct HandOff {
    /// The language that lexes the region.
    pub(crate) language: Language,
    /// The rules listed before the include, an index into
    /// [`Grammar::lists`]. The engine tries them before the language's own
    /// patterns. It tries the end there too, unless the rule applies its end
    /// last.
    pub(crate) before: usize,
    /// The rules listed after the include. The engine tries them only where
    /// the language matches nothing.
    pub(crate) after: usize,
}

/// Where a pattern's compiled form is kept once a search needs it.
/// `None` inside for text the engine cannot compile, which never matches.
///
/// `Tokenizer::scan`, `search` and `probe` index the cells on every pattern
/// they try.
/// The cell's size sets the code for that index.
/// At 152 bytes LLVM emits one `imul`.
/// At 144 bytes it emits `lea` and `shl`, and at 128 bytes `mov` and `shl`.
/// Callgrind counts the two-instruction forms as one instruction more per
/// lookup.
/// On hardware they cost the same or less, since `imul` takes about three
/// cycles.
/// No size is a target, so no test holds one.
pub(crate) type RegexCell = OnceLock<Option<Regex>>;

/// Where a pattern's sketch is kept once a scanner needs it.
/// `None` inside for text the engine cannot parse, which never matches.
pub(crate) type SketchCell = OnceLock<Option<Sketch>>;

/// Where a list's scanner is kept once a search needs it.
pub(crate) type ScannerCell = OnceLock<Scanner>;

/// Rules that apply wherever a scope selector matches the scopes at the
/// cursor, besides the rules of the open rule.
#[derive(Debug)]
pub(crate) struct Injection {
    /// The scope selector, such as `L:text.html - comment`.
    pub(crate) selector: &'static str,
    /// The container rule whose list the injection tries.
    pub(crate) rule: usize,
}

/// Whether `source` holds a back-reference `\1`..`\9`, which an `end` or
/// `while` pattern fills in from the `begin` match.
const fn has_backrefs(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'\\' {
            if bytes[i + 1] >= b'1' && bytes[i + 1] <= b'9' {
                return true;
            }
            i += 1;
        }
        i += 1;
    }
    false
}

/// A pattern's text. Its compiled form is kept in [`Cells::regexes`].
#[derive(Debug)]
pub(crate) struct Pattern {
    /// The pattern as the grammar spells it.
    pub(crate) source: &'static str,
    /// Whether it holds `\1`..`\9`, so an `end` pattern is built per match.
    pub(crate) backrefs: bool,
}

impl Pattern {
    /// The pattern `source`.
    pub(crate) const fn new(source: &'static str) -> Pattern {
        Pattern {
            source,
            backrefs: has_backrefs(source),
        }
    }
}

/// A compiled grammar.
#[derive(Debug)]
pub(crate) struct Grammar {
    /// The root scope, such as `source.rust`.
    pub(crate) scope: &'static str,
    /// Every scope name the rules use.
    pub(crate) names: &'static [&'static str],
    /// The rules.
    pub(crate) rules: &'static [Rule],
    /// The lists of rules, shared by the rules that start the same ones.
    pub(crate) lists: &'static [List],
    /// The grammar's injections.
    pub(crate) injections: &'static [Injection],
    /// The patterns.
    pub(crate) patterns: &'static [Pattern],
    /// The rule of the whole grammar.
    pub(crate) root: usize,
    /// The regions the grammar hands to languages lexed by hand.
    pub(crate) hand_offs: &'static [HandOff],
    /// For each list, the index in [`Grammar::hand_offs`] of the hand-off
    /// whose include stood in it. Every step of the handing loop looks its
    /// open rule's list up here, so the lookup is one load. Empty in a grammar
    /// that hands nothing off.
    pub(crate) list_hand_offs: &'static [Option<u8>],
    /// The rules of the capture groups whose text can reach a hand-off, in
    /// the rule's own list or through a rule nested in it, sorted. Only such
    /// a capture runs the handing loop. Empty in a grammar that hands nothing
    /// off.
    pub(crate) handing_captures: &'static [usize],
    /// The cells a search fills on [`LineKind::Lean`] lines: see
    /// [`Grammar::cells`].
    pub(crate) cells: OnceLock<Cells>,
    /// The same for [`LineKind::Full`] lines.
    pub(crate) full_cells: OnceLock<Cells>,
}

/// Whether a grammar's line kinds run different programs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Folding {
    /// Every pattern compiles to one program for both kinds of line, so a
    /// line's kind is never looked at.
    #[default]
    Same,
    /// Some pattern's lean program differs from its full one.
    Differs,
}

/// The root scopes of the grammars where some pattern's lean program differs
/// from its full one: kept by hand, because only the Rust parser can tell,
/// and checked against every pattern by
/// `the_folding_table_matches_the_patterns`.
static FOLDING: &[&str] = &[
    "markdown.mermaid.codeblock",
    "source.asm.x86_64",
    "source.asp.vb.net",
    "source.batchfile",
    "source.cmake",
    "source.haskell",
    "source.julia",
    "source.pascal",
    "source.php",
    "source.powershell",
    "text.html.markdown",
];

/// The cells of a grammar that searches fill on one kind of line.
#[derive(Debug)]
pub(crate) struct Cells {
    /// The grammar whose cells these are, read only to fill a cell.
    pub(crate) grammar: &'static Grammar,
    /// The kind of line whose programs the cells hold.
    pub(crate) kind: LineKind,
    /// The cell of each pattern's compiled form, by the pattern's index.
    pub(crate) regexes: Box<[RegexCell]>,
    /// The cell of each pattern's sketch, by the pattern's index.
    pub(crate) sketches: Box<[SketchCell]>,
    /// The cell of each list's scanner, by the list's index.
    pub(crate) scanners: Box<[ScannerCell]>,
}

impl Grammar {
    /// The hand-off whose include stood in `list`, if any.
    pub(crate) fn hand_off(&self, list: usize) -> Option<&'static HandOff> {
        let index = (*self.list_hand_offs.get(list)?)?;
        self.hand_offs.get(usize::from(index))
    }

    /// The grammar's cells for lines of `kind`, allocated on first use.
    pub(crate) fn cells(&'static self, kind: LineKind) -> &'static Cells {
        self.cells_of(kind).get_or_init(move || Cells {
            grammar: self,
            kind,
            regexes: self.patterns.iter().map(|_| RegexCell::new()).collect(),
            sketches: self.patterns.iter().map(|_| SketchCell::new()).collect(),
            scanners: self.lists.iter().map(|_| ScannerCell::new()).collect(),
        })
    }

    /// Whether the grammar's line kinds run different programs.
    pub(crate) fn folding(&self) -> Folding {
        if FOLDING.contains(&self.scope) {
            Folding::Differs
        } else {
            Folding::Same
        }
    }

    /// Where the grammar keeps its cells for lines of `kind`.
    pub(crate) fn cells_of(&self, kind: LineKind) -> &OnceLock<Cells> {
        match kind {
            LineKind::Lean => &self.cells,
            LineKind::Full => &self.full_cells,
        }
    }
}

impl Cells {
    /// The compiled pattern at `index`, compiled on first use for the kind of
    /// line the cells serve, or `None` for text the engine cannot compile,
    /// which never matches.
    #[inline]
    pub(crate) fn regex(&self, index: usize) -> Option<&Regex> {
        self.regexes
            .get(index)?
            .get_or_init(move || {
                let source = self.grammar.patterns.get(index)?.source;
                let compiled = match self.sketches.get(index).and_then(OnceLock::get) {
                    Some(Some(sketch)) => Regex::from_sketch(sketch, source, self.kind),
                    Some(None) | None => Regex::for_grammar(source, self.kind),
                };
                compiled.ok()
            })
            .as_ref()
    }

    /// The sketch of the pattern at `index`, parsed on first use, or `None`
    /// for text the engine cannot parse.
    pub(crate) fn sketch(&self, index: usize) -> Option<&Sketch> {
        let source = self.grammar.patterns.get(index)?.source;
        self.sketches
            .get(index)?
            .get_or_init(move || Sketch::for_grammar(source).ok())
            .as_ref()
    }

    /// The scanner of the list at `index`, built on first use, or `None`
    /// when the grammar has no list there.
    pub(crate) fn scanner(&self, index: usize) -> Option<&Scanner> {
        let rules = self.grammar.lists.get(index)?.rules;
        Some(
            self.scanners
                .get(index)?
                .get_or_init(move || Scanner::build(self, rules)),
        )
    }
}
