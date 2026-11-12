//! TypeScript, with the scopes of the TypeScript grammar that Shiki bundles.
//!
//! The lexer is a hand-written state machine over bytes. Its contexts (blocks,
//! calls, types, templates, comments and so on) are frames of the
//! highlighter's interned state.
//!
//! Each context is one begin/end rule of the grammar.
//! Each rule is tried in the grammar's order at every position, as `TextMate`
//! does.
//! The grammar decides many rules by looking ahead on the line: an arrow
//! function, a call with type arguments, a name assigned a function.
//! The lexer reproduces those lookaheads, quirks included:
//!
//! - `Number.MAX_VALUE` and the other `Number` constants get no scope, because
//!   the grammar numbers the capture that would scope them inside a lookahead.
//! - A bracket group in a lookahead nests three levels and no more, as the
//!   grammar's regular expressions do.
//! - An empty line is skipped and leaves the state as it is, as Shiki does. So
//!   the end rules that test for a blank line see only lines of blanks.
//! - A context that opens and closes without the line advancing keeps the rest
//!   of the line, as `TextMate` does for a grammar that loops.
//!
//! Work is counted against [`Limits::work`](crate::Limits).
//! Every scan ahead spends a step per byte it reads.
//! So a line of any length is lexed in time proportional to its length times
//! the work limit's per-byte allowance.

mod bytes;
mod calls;
mod chars;
mod classes;
mod comments;
mod control;
mod declarations;
mod directives;
mod ends;
mod enums;
mod exports;
mod expressions;
mod functions;
mod imports;
mod jsdoc;
mod jsdoc_parameters;
mod jsx;
mod lexer;
mod limits;
mod look;
mod members;
mod namespaces;
mod num;
mod regex;
mod rules;
mod scope;
mod searches;
mod state;
#[cfg(any(feature = "lang-svelte", feature = "_textmate"))]
mod svelte;
mod text;
mod type_aliases;
mod types;

use lexer::Halts;
pub(crate) use scope::NAMES;
use scope::S;
pub(crate) use searches::Scratch;
use state::Rule;

use crate::arena::{Arena, ROOT};
use crate::language::{Halt, Stopped};
use crate::limits::Finished;

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
    #[cfg(feature = "lang-typescript")]
    TypeScript,
    /// `javascript`: names end in `.js`.
    #[cfg(feature = "lang-javascript")]
    JavaScript,
    /// `jsx`: names end in `.js.jsx`.
    #[cfg(feature = "lang-jsx")]
    Jsx,
    /// `tsx`: names end in `.tsx`.
    #[cfg(feature = "lang-tsx")]
    Tsx,
}

impl Dialect {
    /// The index of `TypeScript`, the lexer's const parameter for it.
    pub(crate) const TYPESCRIPT: u8 = 0;
    /// The index of `JavaScript`.
    #[cfg(feature = "lang-javascript")]
    pub(crate) const JAVASCRIPT: u8 = 1;
    /// The index of JSX.
    #[cfg(feature = "lang-jsx")]
    pub(crate) const JSX: u8 = 2;
    /// The index of TSX.
    pub(crate) const TSX: u8 = 3;
    /// The bit that marks a lexer of a capture group that another grammar
    /// lexes again, added to the dialect's index.
    pub(crate) const BARE: u8 = 4;
    /// The bit that marks a lexer of the TypeScript or JavaScript that
    /// Svelte embeds, added to the dialect's index.
    /// Svelte's grammar injects its `$` prefixes there.
    pub(crate) const SVELTE: u8 = 8;

    /// The dialect at position `index` among the four, whatever
    /// [`Dialect::BARE`] and [`Dialect::SVELTE`] set.
    ///
    /// Evaluated at compile time for each `D` that [`line`] instantiates.
    /// So the arm of a dialect whose feature is off is never reached, and
    /// reaching it fails the build.
    pub(crate) const fn of(index: u8) -> Dialect {
        match index & !(Self::BARE | Self::SVELTE) {
            #[cfg(feature = "lang-typescript")]
            Self::TYPESCRIPT => Dialect::TypeScript,
            #[cfg(feature = "lang-javascript")]
            Self::JAVASCRIPT => Dialect::JavaScript,
            #[cfg(feature = "lang-jsx")]
            Self::JSX => Dialect::Jsx,
            #[cfg(feature = "lang-tsx")]
            Self::TSX => Dialect::Tsx,
            _ => panic!("the dialect's feature is off"),
        }
    }

    /// The dialect's position among the four, which numbers its table.
    pub(crate) const fn index(self) -> u16 {
        match self {
            #[cfg(feature = "lang-typescript")]
            Dialect::TypeScript => 0,
            #[cfg(feature = "lang-javascript")]
            Dialect::JavaScript => 1,
            #[cfg(feature = "lang-jsx")]
            Dialect::Jsx => 2,
            #[cfg(feature = "lang-tsx")]
            Dialect::Tsx => 3,
        }
    }

    /// Whether the dialect reads JSX, where `<` then starts a tag instead of
    /// type arguments or a cast: every dialect but `typescript`.
    pub(crate) const fn jsx(self) -> bool {
        self.index() != 0
    }
}

/// The state at the start of a document whose scopes sit on the stack `base`:
/// one context with `source.ts` (in `dialect`'s spelling) on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32, dialect: Dialect) -> u32 {
    let stack = arena.child(base, S::Source.scope(dialect)).unwrap_or(base);
    arena.push(ROOT, Rule::Top.code(), 0, stack).unwrap_or(ROOT)
}

/// The state at the start of the TypeScript lines of a fenced code block.
/// The block's text sits on the stack `base` with no `source.ts` of its own,
/// as in Shiki's embedding.
/// Here the marker rule keeps `\A`, which only the first line of
/// the document matches, from matching in a fence.
#[cfg(any(feature = "_start-embedded", feature = "lang-html"))]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Rule::Started.code(), 0, base)
        .unwrap_or(ROOT)
}

/// The state at the start of an HTML `<script>` element's JavaScript,
/// whose text sits on `base`.
/// It is one context with `source.js` on top of it, as HTML's grammar names
/// it.
/// The state also holds the two comment rules HTML puts before JavaScript's.
/// HTML lexes it with [`lex_until`] and an end at `</script`.
#[cfg(any(
    feature = "lang-html",
    all(
        test,
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    )
))]
pub(crate) fn start_script(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena
        .child(base, S::Source.scope(Dialect::JavaScript))
        .unwrap_or(base);
    arena
        .push(ROOT, Rule::Script.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of a TypeScript expression that another grammar
/// embeds, as Vue's directive values include `source.ts#expression`.
/// Its text sits on `base`, which holds the embedding's own scope.
/// The context runs the `expression` patterns only, with no statements.
#[cfg(feature = "lang-vue")]
pub(crate) fn start_expression(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Rule::Expression.code(), 0, base)
        .unwrap_or(ROOT)
}

/// The state at the start of TypeScript that another grammar embeds at one
/// entry of TypeScript's grammar, as Svelte does.
/// It is a context that runs that entry's patterns only.
/// Its text sits on `base`, which holds the embedding's own scope.
#[cfg(any(
    feature = "lang-svelte",
    feature = "lang-vue",
    feature = "lang-astro",
    all(
        test,
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    )
))]
fn start_entry(arena: &mut Arena, base: u32, rule: Rule) -> u32 {
    arena.push(ROOT, rule.code(), 0, base).unwrap_or(ROOT)
}

/// The start of `source.ts#object-literal`, which Svelte's `{ {a: 1} }`
/// includes, and of `source.tsx#object-literal`, included by Astro's.
#[cfg(any(feature = "lang-svelte", feature = "lang-tsx"))]
#[cfg(any(
    feature = "lang-svelte",
    feature = "lang-astro",
    all(
        test,
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    )
))]
pub(crate) fn start_object_literal(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryObjectLiteral)
}

/// The start of a template string's body, without its `string.template`
/// scope: `source.tsx#template-substitution-element` and
/// `source.tsx#string-character-escape`, the two patterns of the body, which
/// Astro's backtick attribute values include.
/// Unlike a template string, it has no end of its own: the end of the rule
/// that embeds it, the closing backtick, ends it.
#[cfg(feature = "lang-tsx")]
#[cfg(any(
    feature = "lang-astro",
    all(
        test,
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    )
))]
pub(crate) fn start_template(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryTemplate)
}

/// The start of `source.ts#object-binding-pattern`, which Svelte's
/// `{#each xs as {a, b}}` includes.
#[cfg(feature = "lang-svelte")]
pub(crate) fn start_object_binding(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryObjectBinding)
}

/// The start of `source.ts#object-binding-pattern-const`, which Svelte's
/// `{@const {a, b} = x}` includes.
#[cfg(feature = "lang-svelte")]
pub(crate) fn start_object_binding_const(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryObjectBindingConst)
}

/// The start of `source.ts#array-binding-pattern`, which Svelte's
/// `{#each xs as [a, b]}` includes.
#[cfg(feature = "lang-svelte")]
pub(crate) fn start_array_binding(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryArrayBinding)
}

/// The start of `source.ts#array-binding-pattern-const`, which Svelte's
/// `{@const [a, b] = x}` includes.
#[cfg(feature = "lang-svelte")]
pub(crate) fn start_array_binding_const(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryArrayBindingConst)
}

/// The start of the body of `source.ts#type-parameters`, copied by Svelte's
/// `generics="T extends U"`: its comments, modifiers, types, commas
/// and `=`, with no `<` or `>`.
#[cfg(feature = "lang-svelte")]
pub(crate) fn start_type_parameters(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryTypeParameters)
}

/// The start of the string of Vue's `generic` attribute, whose patterns
/// copy those of `source.ts#type-parameters` with no `const`.
#[cfg(feature = "lang-vue")]
pub(crate) fn start_generic_attribute(arena: &mut Arena, base: u32) -> u32 {
    start_entry(arena, base, Rule::EntryGenericAttribute)
}

/// Lexes the text of `line` from byte `from` and `state`, appending its
/// tokens with end offsets counted from `line`'s start.
/// It returns the state after it and where `halts` stopped it.
/// The bytes before `from` belong to the language that embeds this one.
/// Lookbehinds read them as `TextMate` does.
///
/// An empty text appends nothing and leaves the state as it is, which is how
/// Shiki treats it.
/// The end rules that test for a blank line (`^\s*$`) never run on an empty
/// one.
fn line(
    cx: &mut crate::language::Context<'_>,
    dialect: Dialect,
    line: &str,
    from: usize,
    state: u32,
    halts: Option<Halts<'_>>,
) -> (Finished, Option<usize>) {
    if line.len() <= from {
        return (Finished::unspent(state), None);
    }
    match dialect {
        #[cfg(feature = "lang-typescript")]
        Dialect::TypeScript => run::<{ Dialect::TYPESCRIPT }>(cx, line, from, state, halts),
        #[cfg(feature = "lang-javascript")]
        Dialect::JavaScript => run::<{ Dialect::JAVASCRIPT }>(cx, line, from, state, halts),
        #[cfg(feature = "lang-jsx")]
        Dialect::Jsx => run::<{ Dialect::JSX }>(cx, line, from, state, halts),
        #[cfg(feature = "lang-tsx")]
        Dialect::Tsx => run::<{ Dialect::TSX }>(cx, line, from, state, halts),
    }
}

/// Lexes the text of `line` from `from` as the dialect `D` numbers, and
/// returns the state after it and where `halts` stopped it.
/// Lexes `line` in `dialect` like [`lex_until`], for the TypeScript or
/// JavaScript that Svelte embeds: Svelte's grammar injects its `$` prefixes
/// into them, which [`Dialect::SVELTE`] turns on.
///
/// The engine for loaded grammars calls it too.
/// It does so when Svelte's grammar hands a region to the TypeScript or
/// JavaScript lexer.
#[cfg(any(
    feature = "lang-svelte",
    all(
        feature = "_textmate",
        any(feature = "lang-typescript", feature = "lang-javascript")
    )
))]
pub(crate) fn lex_until_svelte(
    cx: &mut crate::language::Context<'_>,
    dialect: Dialect,
    line: crate::language::Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let halts = Some(Halts { end, after });
    let (finished, halt) = match dialect {
        #[cfg(feature = "lang-typescript")]
        Dialect::TypeScript => run::<{ Dialect::TYPESCRIPT | Dialect::SVELTE }>(
            cx, line.text, line.start, state, halts,
        ),
        #[cfg(feature = "lang-javascript")]
        Dialect::JavaScript => run::<{ Dialect::JAVASCRIPT | Dialect::SVELTE }>(
            cx, line.text, line.start, state, halts,
        ),
        #[cfg(feature = "lang-jsx")]
        Dialect::Jsx => self::line(cx, dialect, line.text, line.start, state, halts),
        #[cfg(feature = "lang-tsx")]
        Dialect::Tsx => self::line(cx, dialect, line.text, line.start, state, halts),
    };
    (finished, halt.map(Stopped::halt))
}

fn run<const D: u8>(
    cx: &mut crate::language::Context<'_>,
    line: &str,
    from: usize,
    state: u32,
    halts: Option<Halts<'_>>,
) -> (Finished, Option<usize>) {
    let mut lexer = lexer::Lexer::<D>::new(cx, line, from, state, halts);
    lexer.run();
    let halt = lexer.halted_at();
    (lexer.finish_line(), halt)
}

#[cfg(all(
    test,
    feature = "lang-typescript",
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx"
))]
mod tests;

/// Lexes `line` in `dialect` from `state` and returns the state after it.
///
/// It follows the crate's one lexer contract
/// ([`Language::lex`](crate::Language)).
/// Token ends count from the start of the whole line.
#[inline]
pub(crate) fn lex(
    cx: &mut crate::language::Context<'_>,
    dialect: Dialect,
    line: crate::language::Line<'_>,
    state: u32,
) -> Finished {
    self::line(cx, dialect, line.text, line.start, state, None).0
}

/// Lexes as JavaScript a capture group of another grammar that `TextMate`
/// lexes again with this one.
/// `line.text` is the line up to the group's end, with no line feed after it.
/// The group starts at `line.start`.
/// Returns the state after it.
///
/// Only the event handlers of HTML and Astro need it, so only `JavaScript`
/// has the lexer that reads no line feed.
#[cfg(any(feature = "lang-html", feature = "lang-astro"))]
pub(crate) fn lex_capture(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
) -> Finished {
    if line.text.len() <= line.start {
        return Finished::unspent(state);
    }
    run::<{ Dialect::JAVASCRIPT | Dialect::BARE }>(cx, line.text, line.start, state, None).0
}

/// Lexes as TypeScript a capture group that Svelte's grammar lexes again
/// with TypeScript, with Svelte's `$` prefixes.
/// `line.text` is the line up to the group's end, with no line feed after
/// it, and the group starts at `line.start`.
/// It starts afresh on `base`, the group's stack, and returns the state after
/// it.
#[cfg(feature = "lang-svelte")]
pub(crate) fn lex_capture_svelte(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    base: u32,
) -> Finished {
    let state = start_embedded(cx.arena, base);
    run::<{ Dialect::TYPESCRIPT | Dialect::BARE | Dialect::SVELTE }>(
        cx, line.text, line.start, state, None,
    )
    .0
}

/// Lexes `line` in `dialect` from `state` like [`lex`], and halts where the
/// language that embeds this one takes the line back, which it returns.
///
/// Both tests apply only while no context of the code is open beyond the
/// one the embedding started:
///
/// - `end` holds the embedding rule's patterns listed before this grammar's. It
///   is tried at each place before this grammar's rules.
/// - `after` holds those listed after it. It is tried where none of this
///   grammar's rules matches.
///
/// A match that starts before a place and covers it wins.
/// The tokens stop at the halt.
/// The embedding lexer resumes there.
/// When the line ran to its end, it resumes on the next line with the
/// returned state.
pub(crate) fn lex_until(
    cx: &mut crate::language::Context<'_>,
    dialect: Dialect,
    line: crate::language::Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    let halts = Halts { end, after };
    let (finished, halt) = self::line(cx, dialect, line.text, line.start, state, Some(halts));
    (finished, halt.map(Stopped::halt))
}
