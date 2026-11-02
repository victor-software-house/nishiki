//! TypeScript, with the scopes of the TypeScript grammar that Shiki bundles.
//!
//! The lexer is a hand-written state machine over bytes. Its contexts (blocks,
//! calls, types, templates, comments and so on) are frames of the
//! highlighter's interned state.
//!
//! Each context is one begin/end rule of the grammar, and each rule is tried
//! in the grammar's order at every position, as `TextMate` does. The grammar
//! decides many rules by looking ahead on the line (an arrow function, a call
//! with type arguments, a name assigned a function), and the lexer reproduces
//! those lookaheads, quirks included:
//!
//! - `Number.MAX_VALUE` and the other `Number` constants get no scope, because
//!   the grammar numbers the capture that would scope them inside a lookahead.
//! - A bracket group in a lookahead nests three levels and no more, as the
//!   grammar's regular expressions do.
//! - An empty line is skipped and leaves the state as it is, as Shiki does, so
//!   the end rules that test for a blank line see only lines of blanks.
//! - A context that opens and closes without the line advancing keeps the rest
//!   of the line, as `TextMate` does for a grammar that loops.
//!
//!
//! Work is counted against [`Limits::work`](crate::Limits): every scan ahead
//! spends a step per byte it reads, so a line of any length is lexed in time
//! proportional to its length times the work limit's per-byte allowance.

#![expect(
    clippy::many_single_char_names,
    reason = "a scanner names positions by role: `p` where a rule starts, `e` where it ends, `j` and `k` for the cursor"
)]

mod bytes;
mod calls;
mod classes;
mod control;
mod decl;
mod directives;
mod ends;
mod enums;
mod exports;
mod expr;
mod functions;
mod imports;
mod jsdoc;
mod jsdoc_params;
mod jsx;
mod kind;
mod lexer;
mod limits;
mod look;
mod members;
mod namespaces;
mod num;
mod rules;
mod scope;
mod text;
mod typ;
mod type_aliases;

use kind::Kind;
pub(crate) use scope::NAMES;
use scope::S;

use crate::Token;
use crate::arena::{Arena, ROOT};
use crate::limits::{Limits, LineReport};

/// Which of the grammars to follow: TypeScript, or one of the three that
/// share a structure with it and add JSX.
///
/// The `javascript`, `jsx` and `tsx` grammars are one grammar with three
/// spellings of its scope names, so they lex alike. They differ from
/// `typescript` in the JSX rules, and in the rules that read an angle bracket
/// as a type argument or a cast.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Dialect {
    /// `typescript`: names end in `.ts`.
    TypeScript,
    /// `javascript`: names end in `.js`.
    JavaScript,
    /// `jsx`: names end in `.js.jsx`.
    Jsx,
    /// `tsx`: names end in `.tsx`.
    Tsx,
}

impl Dialect {
    /// The dialect at position `index` among the four.
    pub(crate) const fn of(index: u8) -> Dialect {
        match index {
            0 => Dialect::TypeScript,
            1 => Dialect::JavaScript,
            2 => Dialect::Jsx,
            _ => Dialect::Tsx,
        }
    }

    /// The dialect's position among the four, which numbers its table.
    pub(crate) const fn index(self) -> u16 {
        match self {
            Dialect::TypeScript => 0,
            Dialect::JavaScript => 1,
            Dialect::Jsx => 2,
            Dialect::Tsx => 3,
        }
    }

    /// Whether the dialect reads JSX, which `<` then starts a tag instead of
    /// type arguments or a cast.
    pub(crate) const fn jsx(self) -> bool {
        !matches!(self, Dialect::TypeScript)
    }
}

/// The state at the start of a document whose scopes sit on the stack `base`:
/// one context with `source.ts` (in `dialect`'s spelling) on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32, dialect: Dialect) -> u32 {
    let stack = arena.child(base, S::Source.scope(dialect)).unwrap_or(base);
    arena.push(ROOT, Kind::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the TypeScript lines of a fenced code block,
/// whose text sits on the stack `base` with no `source.ts` of its own, as in
/// Shiki's embedding. The marker kind keeps `\A`, which only the first line of
/// the document matches, from matching in a fence.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Kind::Started.code(), 0, base)
        .unwrap_or(ROOT)
}

/// The state at the start of the JavaScript inside an interpolation such as
/// `${...}`, whose text sits on `base`: like [`start_embedded`], with an end at
/// the first `}` that no context the code opened is waiting for.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "the GraphQL lexer's interpolation will call it")
)]
pub(crate) fn start_interpolation(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Kind::Interpolation.code(), 0, base)
        .unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it.
///
/// An empty line appends nothing and leaves the state as it is, which is how
/// Shiki treats it: the end rules that test for a blank line (`^\s*$`) never
/// run on an empty one.
pub(crate) fn line(
    arena: &mut Arena,
    limits: &Limits,
    dialect: Dialect,
    line: &str,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    if line.is_empty() {
        return state;
    }
    match dialect {
        Dialect::TypeScript => run::<0>(arena, limits, line, state, out, report),
        Dialect::JavaScript => run::<1>(arena, limits, line, state, out, report),
        Dialect::Jsx => run::<2>(arena, limits, line, state, out, report),
        Dialect::Tsx => run::<3>(arena, limits, line, state, out, report),
    }
}

/// Lexes the non-empty `line` as the dialect `D` numbers.
fn run<const D: u8>(
    arena: &mut Arena,
    limits: &Limits,
    line: &str,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> u32 {
    let mut lexer = lexer::Lexer::<D>::new(arena, limits, line, out, state, report);
    lexer.run();
    lexer.into_state()
}

#[cfg(test)]
mod tests;

/// Lexes `line` in `dialect` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
#[inline]
pub(crate) fn lex(
    cx: &mut crate::language::Context<'_>,
    dialect: Dialect,
    line: crate::language::Line<'_>,
    state: u32,
) -> u32 {
    cx.lex_from_start(line, |cx, rest| {
        self::line(cx.arena, cx.limits, dialect, rest, state, cx.out, cx.report)
    })
}

/// Lexes the JavaScript of an interpolation from `state`, which
/// [`start_interpolation`] made on an earlier line or this one, under the
/// crate's lexer contract with token ends counted from the start of the whole
/// line. The tokens stop before the `}` that ends it, and the returned offset
/// is where that `}` is: the embedding lexer reads it, and the state returned
/// is of no use after it. Without one, the state resumes on the next line.
#[expect(dead_code, reason = "the GraphQL lexer's interpolation will call it")]
pub(crate) fn lex_interpolation(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
) -> (u32, Option<usize>) {
    let mut close = None;
    let next = cx.lex_from_start(line, |cx, rest| {
        let (next, at) = interpolation_line(cx.arena, cx.limits, rest, state, cx.out, cx.report);
        close = at;
        next
    });
    (next, close.map(|at| at + line.start))
}

/// Lexes the non-empty `line` as JavaScript up to the `}` that ends the
/// interpolation, and returns the state after it and where that `}` is.
fn interpolation_line(
    arena: &mut Arena,
    limits: &Limits,
    line: &str,
    state: u32,
    out: &mut Vec<Token>,
    report: &mut LineReport,
) -> (u32, Option<usize>) {
    if line.is_empty() {
        return (state, None);
    }
    let mut lexer = lexer::Lexer::<1>::new(arena, limits, line, out, state, report);
    lexer.run();
    let close = lexer.halted_at();
    (lexer.into_state(), close)
}
