//! LaTeX on the shared rule engine ([`crate::rules`]).
//!
//! It holds the grammar's types, what its matchers remember about a line, and
//! where an embedded language hands the line back.

use super::grammar::{Expr, Nested, Pat, Rule, Sc};
use super::scope::S;
use crate::arena::Arena;
use crate::classes::space_width;
use crate::language::{Context, Halt, Language, Line, StopKind, Stopped};
use crate::limits::Finished;
use crate::rules::{self, Act, Captures, FnHalt, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The LaTeX lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Latex>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Latex>;

/// How many capture groups can be lexed again inside one another.
/// A macro's arguments lex their text again, and those lex their own, so
/// the depth stays small; the bound only stops a runaway.
/// Past it, a group keeps its own scopes, and the line reports the depth
/// limit.
pub(crate) const MAX_NESTED: u32 = 64;

/// The `latex` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Latex;

/// What a line of LaTeX keeps for its matchers.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// How many capture groups are being lexed again around this point.
    nested: u32,
    /// The inline code groups the line defers to Python or Julia
    /// ([`Grammar::deferred`]).
    deferrals: Option<Box<rules::Deferrals>>,
}

impl Grammar for Latex {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = Option<Box<rules::Deferrals>>;

    const NAME: &'static str = "LaTeX";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 12;
    /// Environments such as `tabular` take the line feed after `\begin{…}`.
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

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    /// A sectioning command's name scope repeats the command, so it is built
    /// from the payload instead ([`Grammar::rule_name_delimiter`]).
    fn rule_name(rule: Rule) -> Scopes<Sc> {
        if matches!(rule, Rule::SectionsMacro) {
            return Sc::NO_SCOPES;
        }
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.data().content
    }

    /// A code environment's rule carries the language it embeds, and an
    /// environment's rule the name its end repeats.
    /// A sectioning command carries its name, group 3, which its name scope
    /// `meta.function.section.$3.latex` repeats.
    fn rule_payload(rule: Rule) -> Payload {
        if matches!(rule, Rule::SectionsMacro) {
            return Payload::Backref(3);
        }
        rule.payload()
    }

    fn rule_name_delimiter(rule: Rule) -> Option<&'static str> {
        matches!(rule, Rule::SectionsMacro).then_some("meta.function.section.")
    }

    fn rule_name_suffix(rule: Rule) -> &'static str {
        if matches!(rule, Rule::SectionsMacro) {
            ".latex"
        } else {
            ""
        }
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

    /// TeX's matchers answer the expressions of the TeX rules the grammar
    /// includes; this module's answer the rest.
    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        match pat.expr() {
            Expr::Shared(rx) => lexer.tex_matcher(rx, p),
            Expr::Own(rx) => lexer.matcher(rx, p),
        }
    }

    /// The code of `\py{…}` and `\jl{…}`, which Python and Julia lex
    /// ([`rules::Embeds::lex_nested`]).
    const DEFERS_NESTED: bool = true;

    fn defers_nested(nested: Nested) -> Option<Language> {
        nested.rule.deferred()
    }

    /// The deferrals' buffers go from line to line; the depth starts at 0.
    fn extra(kept: Self::Kept) -> Self::Extra {
        Extra {
            deferrals: kept,
            ..Extra::default()
        }
    }

    fn keep(extra: Self::Extra) -> Self::Kept {
        extra.deferrals
    }

    fn deferred(extra: &mut Self::Extra) -> Option<&mut Option<Box<rules::Deferrals>>> {
        Some(&mut extra.deferrals)
    }

    fn deferred_kept(kept: &mut Self::Kept) -> Option<&mut Option<Box<rules::Deferrals>>> {
        Some(kept)
    }

    /// Lexes the group again with its rule, under the stack the engine
    /// gives and the group's own scopes.
    /// Past [`MAX_NESTED`] groups lexed again around it, the text keeps the
    /// group's scopes alone.
    /// The line then reports the depth limit.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        let inner = lexer.path(base, nested.scopes);
        if lexer.x.nested >= MAX_NESTED {
            lexer.record_depth();
            lexer.emit(to, inner);
            return;
        }
        lexer.x.nested += 1;
        lexer.retokenise(nested.rule, from, to, inner);
        lexer.x.nested -= 1;
    }
}

/// A code environment hands its body back at its end: a line that starts
/// with `\end{…}`, or the `}` of a braced argument.
impl rules::Embeds for Latex {
    type End = FnHalt;
    const HALTS: bool = true;

    /// Every embedding rule picks its halt by the rule
    /// ([`rules::Embeds::embed_halts_for`]); this one never holds.
    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (never, None)
    }

    fn embed_halts_for(rule: Rule, _: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (rule.end_halt(), None)
    }

    /// `\directlua{…}` lists TeX's braces after Lua.
    fn embeds_after_own(rule: Rule) -> bool {
        rule.embeds_after_own()
    }

    /// A language without halts of its own, such as Java, lexes the text up
    /// to the first place where the rule's end holds.
    /// It lexes that text as if the line ended there.
    ///
    /// The end holds only while the language has no rule open at the start
    /// of the line.
    /// vscode-textmate tries the end only then.
    /// So a string that Julia leaves open keeps the next line's `\end{jlcode}`
    /// inside it.
    /// A `}` inside one of its tokens there ends the code early, where Shiki
    /// would read on.
    /// A language with halts stops by itself.
    fn lex_embedded_until(
        _rule: Rule,
        language: Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        state: u32,
        (end, after): (&dyn Halt, Option<&dyn Halt>),
    ) -> Option<(Finished, Option<Stopped>)> {
        if let Some(halted) = language.lex_until(cx, line, state, end, after) {
            return Some(halted);
        }
        if state != 0 && cx.arena.frame(state).depth > 1 {
            return None;
        }
        let at = end.first(line.text.as_bytes(), line.start, line.text.len())?;
        let finished = match line.text.get(..at) {
            Some(text) if at > line.start => {
                language.lex_embedded(cx, Line { text, ..line }, state)
            }
            _ => Finished::unspent(state),
        };
        Some((
            finished,
            Some(Stopped {
                at,
                kind: StopKind::Halt,
            }),
        ))
    }

    /// Python or Julia lexes the code group of `\py{…}` or `\jl{…}` from
    /// its root, under its own scope.
    /// The grammar's include of `source.python` or `source.julia` puts it
    /// there.
    fn lex_nested(
        language: Language,
        cx: &mut Context<'_>,
        line: Line<'_>,
        base: u32,
    ) -> Option<Finished> {
        let state = language.start(cx.arena, base);
        Some(language.lex_embedded(cx, line, state))
    }
}

/// A halt that never holds: the generated `Rule::end_halt` of a rule that
/// embeds no language.
pub(super) fn never(_: &[u8], _: usize) -> bool {
    false
}

/// `}`: the end of a braced code argument.
pub(super) fn closes_brace(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'}')
}

/// The environments `minted`, `lstlisting` and `pyglist`, which name their
/// language in an argument.
pub(super) const LISTING: &[&[u8]] = &[b"minted", b"lstlisting", b"pyglist"];
/// Asymptote's environments.
pub(super) const ASY: &[&[u8]] = &[b"asy", b"asycode"];
/// C++'s environment.
pub(super) const CPP: &[&[u8]] = &[b"cppcode"];
/// Graphviz's environments.
pub(super) const DOT: &[&[u8]] = &[b"dot2tex", b"dotcode"];
/// gnuplot's environment.
pub(super) const GNUPLOT: &[&[u8]] = &[b"gnuplot"];
/// Haskell's environment.
pub(super) const HSCODE: &[&[u8]] = &[b"hscode"];
/// Java's environments.
pub(super) const JAVA: &[&[u8]] = &[
    b"javacode",
    b"javaverbatim",
    b"javablock",
    b"javaconcode",
    b"javaconsole",
    b"javaconverbatim",
];
/// Julia's environments with the `jl` stem.
pub(super) const JL: &[&[u8]] = &[
    b"jlcode",
    b"jlverbatim",
    b"jlblock",
    b"jlconcode",
    b"jlconsole",
    b"jlconverbatim",
];
/// Julia's environments with the `julia` stem.
pub(super) const JULIA: &[&[u8]] = &[
    b"juliacode",
    b"juliaverbatim",
    b"juliablock",
    b"juliaconcode",
    b"juliaconsole",
    b"juliaconverbatim",
];
/// Lua's environments.
pub(super) const LUA: &[&[u8]] = &[b"luacode", b"luadraw"];
/// Python's environments with the `py` stem.
pub(super) const PY: &[&[u8]] = &[
    b"pycode",
    b"pyverbatim",
    b"pyblock",
    b"pyconcode",
    b"pyconsole",
    b"pyconverbatim",
];
/// Python's environments with the `pylab` stem.
pub(super) const PYLAB: &[&[u8]] = &[
    b"pylabcode",
    b"pylabverbatim",
    b"pylabblock",
    b"pylabconcode",
    b"pylabconsole",
    b"pylabconverbatim",
];
/// The `SageTeX` environments, and the Python ones of `pythontex`.
pub(super) const SAGE: &[&[u8]] = &[
    b"sageblock",
    b"sagesilent",
    b"sageverbatim",
    b"sageexample",
    b"sagecommandline",
    b"python",
    b"pythonq",
    b"pythonrepl",
];
/// Scala's environment.
pub(super) const SCALA: &[&[u8]] = &[b"scalacode"];
/// The `SymPy` environments.
pub(super) const SYMPY: &[&[u8]] = &[
    b"sympycode",
    b"sympyverbatim",
    b"sympyblock",
    b"sympyconcode",
    b"sympyconsole",
    b"sympyconverbatim",
];
/// The `terminal` environment.
pub(super) const TERMINAL: &[&[u8]] = &[b"terminal"];

/// `^\s*(?=\\end\{(?:minted|lstlisting|pyglist)})`.
pub(super) fn closes_listing(s: &[u8], p: usize) -> bool {
    closes_env(s, p, LISTING, false)
}

/// `^\s*(?=\\end\{(?:RobExt)?(?:CacheMeCode|PlaceholderPathFromCode\*?
/// |PlaceholderFromCode\*?|SetPlaceholderCode\*?)})`.
pub(super) fn closes_cache(s: &[u8], p: usize) -> bool {
    env_name_ahead(s, p).is_some_and(cache_name)
}

/// `^\s*(?=\\end\{hscode\*?})`.
pub(super) fn closes_hscode(s: &[u8], p: usize) -> bool {
    closes_env(s, p, HSCODE, true)
}

/// `^\s*(?=\\end\{java(?:code|…)\*?})`.
pub(super) fn closes_java(s: &[u8], p: usize) -> bool {
    closes_env(s, p, JAVA, true)
}

/// `^\s*(?=\\end\{jl(?:code|…)\*?})`.
pub(super) fn closes_jl(s: &[u8], p: usize) -> bool {
    closes_env(s, p, JL, true)
}

/// `^\s*(?=\\end\{julia(?:code|…)\*?})`.
pub(super) fn closes_julia(s: &[u8], p: usize) -> bool {
    closes_env(s, p, JULIA, true)
}

/// `^\s*(?=\\end\{lua(?:code|draw)\*?})`.
pub(super) fn closes_lua(s: &[u8], p: usize) -> bool {
    closes_env(s, p, LUA, true)
}

/// `^\s*(?=\\end\{py(?:code|…)\*?})`.
pub(super) fn closes_py(s: &[u8], p: usize) -> bool {
    closes_env(s, p, PY, true)
}

/// `^\s*(?=\\end\{pylab(?:code|…)\*?})`.
pub(super) fn closes_pylab(s: &[u8], p: usize) -> bool {
    closes_env(s, p, PYLAB, true)
}

/// `^\s*(?=\\end\{(?:sageblock|…|pythonq??|pythonrepl)\*?})`.
pub(super) fn closes_sage(s: &[u8], p: usize) -> bool {
    closes_env(s, p, SAGE, true)
}

/// `^\s*(?=\\end\{scalacode\*?})`.
pub(super) fn closes_scalacode(s: &[u8], p: usize) -> bool {
    closes_env(s, p, SCALA, true)
}

/// `^\s*(?=\\end\{sympy(?:code|…)\*?})`.
pub(super) fn closes_sympy(s: &[u8], p: usize) -> bool {
    closes_env(s, p, SYMPY, true)
}

/// `^\s*(?=\\end\{terminal\*?})`.
pub(super) fn closes_terminal(s: &[u8], p: usize) -> bool {
    closes_env(s, p, TERMINAL, true)
}

/// `^\s*(?=\\end\{(names)\*?})` at `p`, with the `*` when `star`.
fn closes_env(s: &[u8], p: usize, names: &[&[u8]], star: bool) -> bool {
    env_name_ahead(s, p).is_some_and(|name| is_named(name, names, star))
}

/// Whether `name` is one of `names`, or one of them and a `*` when `star`.
pub(super) fn is_named(name: &[u8], names: &[&[u8]], star: bool) -> bool {
    let stem = if star {
        name.strip_suffix(b"*").unwrap_or(name)
    } else {
        name
    };
    names.contains(&stem)
}

/// `(?:RobExt)?(?:CacheMeCode|PlaceholderPathFromCode\*?|PlaceholderFromCode\*?
/// |SetPlaceholderCode\*?)`: the environments of the `robust-externalize`
/// package.
pub(super) fn cache_name(name: &[u8]) -> bool {
    let rest = name.strip_prefix(b"RobExt").unwrap_or(name);
    let rest = rest
        .strip_suffix(b"*")
        .filter(|stem| *stem != b"CacheMeCode")
        .unwrap_or(rest);
    [
        &b"CacheMeCode"[..],
        b"PlaceholderPathFromCode",
        b"PlaceholderFromCode",
        b"SetPlaceholderCode",
    ]
    .contains(&rest)
}

/// The name of the `\\end{name}` that the blanks at the start of the line
/// lead to, when `p` is the line's start.
fn env_name_ahead(s: &[u8], p: usize) -> Option<&[u8]> {
    if p != 0 {
        return None;
    }
    let mut at = 0;
    while at < s.len() {
        let width = space_width(s, at);
        if width == 0 {
            break;
        }
        at += width;
    }
    let rest = s.get(at..)?.strip_prefix(b"\\end{")?;
    let close = memchr::memchr(b'}', rest)?;
    rest.get(..close)
}
