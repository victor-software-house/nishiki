//! Where each of LaTeX's own regular expressions matches.
//!
//! TeX's matchers answer the expressions of the TeX rules the grammar
//! includes ([`crate::tex`]).

use memchr::{memchr, memchr2, memchr3, memrchr};

use super::engine::{self, Lexer};
use super::grammar::Rx;
use super::words::{TEXT_SYMBOL, WORD_BYTES, WORD_ENDS, WORD_LISTS};
use crate::case_fold;
use crate::classes::{is_alpha, is_number, is_word};
use crate::rules::Chars;
use crate::tex::matchers::listed;

/// The languages a `minted` or `lstlisting` argument names, in the
/// alternation's order, by the pattern that names them.
const BASH: &[&[u8]] = &[b"bash"];
const CSS: &[&[u8]] = &[b"css"];
const HASKELL: &[&[u8]] = &[b"hs", b"haskell"];
const HTML: &[&[u8]] = &[b"html"];
const JAVA: &[&[u8]] = &[b"java"];
const JULIA: &[&[u8]] = &[b"jl", b"julia"];
const JAVASCRIPT: &[&[u8]] = &[b"js", b"javascript"];
const LUA: &[&[u8]] = &[b"lua"];
const PYTHON: &[&[u8]] = &[b"py", b"python", b"sage"];
const RUBY: &[&[u8]] = &[b"rb", b"ruby"];
const RUST: &[&[u8]] = &[b"rust"];
const TYPESCRIPT: &[&[u8]] = &[b"ts", b"typescript"];
const XML: &[&[u8]] = &[b"xml"];
const YAML: &[&[u8]] = &[b"yaml"];
/// The languages only `robust-externalize` names.
const ASYMPTOTE: &[&[u8]] = &[b"asy", b"asymptote"];
const CPP: &[&[u8]] = &[b"c", b"cpp"];
const GNUPLOT: &[&[u8]] = &[b"gnuplot"];
const TIKZ: &[&[u8]] = &[b"tikz", b"tikzpicture"];

/// The math environments `AllBalancedEnvAt0` names, other than the
/// `matrix` and `NiceMatrix` families.
const MATH_ENVS: &[&[u8]] = &[
    b"array",
    b"+array",
    b"equation",
    b"eqnarray",
    b"IEEEeqnarray",
    b"subeqnarray",
    b"multline",
    b"align",
    b"aligned",
    b"alignat",
    b"alignedat",
    b"flalign",
    b"flaligned",
    b"flalignat",
    b"split",
    b"gather",
    b"gathered",
    b"cases",
    b"+cases",
    b"dcases",
    b"rcases",
    b"drcases",
    b"math",
    b"displaymath",
    b"mini",
    b"maxi",
    b"argmini",
    b"argmaxi",
];

/// The table environments `AllBalancedEnvAt2` names.
const TABLES: &[&[u8]] = &[
    b"tabular",
    b"tabular*",
    b"tabularx",
    b"tabulary",
    b"xltabular",
    b"longtable",
    b"tabu",
    b"longtabu",
    b"tblr",
    b"longtblr",
    b"talltblr",
    b"NiceTabular",
    b"NiceTabular*",
    b"NiceTabularX",
    b"booktabs",
];

/// The sectioning commands `SectionsMacro` names.
const SECTIONS: &[&[u8]] = &[
    b"section",
    b"subsection",
    b"subsubsection",
    b"paragraph",
    b"subparagraph",
    b"chapter",
    b"part",
    b"addpart",
    b"addchap",
    b"addsec",
    b"minisec",
    b"frametitle",
];

/// The commands `NewcommandMacro` names.
const NEWCOMMANDS: &[&[u8]] = &[
    b"newcommand",
    b"renewcommand",
    b"newrobustcmd",
    b"renewrobustcmd",
    b"DeclareRobustCommand",
];

/// The commands `AllOtherMacroAt0` names.
const BREAKS: &[&[u8]] = &[
    b"newline",
    b"pagebreak",
    b"clearpage",
    b"linebreak",
    b"pause",
];

/// The prefixes a citation command takes before `cite`, the empty one first.
const CITE_PREFIXES: &[&[u8]] = &[
    b"",
    b"Auto",
    b"auto",
    b"foot",
    b"full",
    b"footfull",
    b"no",
    b"ref",
    b"short",
    b"Text",
    b"text",
    b"Paren",
    b"paren",
    b"Smart",
    b"smart",
    b"Fvol",
    b"Pvol",
    b"fvol",
    b"pvol",
    b"vol",
];

/// The suffixes a citation command takes after `cite` or `citeal`.
const CITE_SUFFIXES: &[&[u8]] = &[
    b"", b"p", b"s", b"t", b"author", b"year", b"yearpar", b"title", b"url", b"date",
];

/// The punctuation of a citation key, `[-.:_\p{Alphabetic}\p{N}]`.
const KEY: &[u8] = b"-.:_";

/// The punctuation of a label, `[!*,-/:^_\p{Alphabetic}\p{N}]`.
const LABEL: &[u8] = b"!*,-./:^_";

/// What one code argument of an inline code command holds: a delimited
/// body, or a braced one.
#[derive(Clone, Copy)]
enum Body {
    /// `([^…])(.*?)(\N)`: the delimiter, the text up to its next copy and
    /// that copy.
    Delimited {
        open: usize,
        text: usize,
        close: usize,
        end: usize,
    },
    /// `(\{)(.*?)(})`: the brace, the text up to the next `}` and the `}`.
    Braced { open: usize, close: usize },
}

/// What follows a citation command's arguments: an optional `<…>`, where
/// the `[…]` after it start, and where the `{` of the keys stands.
#[derive(Clone, Copy)]
struct CitationTail {
    angle: Option<(usize, usize)>,
    brackets: usize,
    brace: usize,
}

impl Lexer<'_> {
    /// Whether `rx` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per expression: the table of what each matches"
    )]
    pub(crate) fn matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        match rx {
            Rx::RootAt0 => self.space_after_short_command(p),
            Rx::SongsEnvAt0 => {
                let (_, close) =
                    self.env_name(self.space_end(p), b"\\begin{", |name| name == b"songs")?;
                let brace = self.lit(close + 1, b"{")?;
                let last = memrchr(b'}', self.text(brace, self.lim()))?;
                let end = brace + last + 1;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::SongsEnvAt0End => self.capped_end(p, b"\\end{songs}", true),
            Rx::MacroWithArgsTokenizer => self.macro_with_args(p),
            Rx::EmbeddedCodeEnvAt0 => {
                self.listing_begin(p, |name| engine::is_named(name, engine::LISTING, false))
            }
            Rx::EmbeddedCodeEnvAt0End => self.backref_end(p, b"\\end{").map(|(_, _, end)| end),
            Rx::MultilineOptionalArgNoHighlight => {
                if !self.at_g(p) && !self.prev_is(p, b'}') {
                    return None;
                }
                self.lit(self.space_end(p), b"[")
            }
            Rx::MultilineOptionalArgNoHighlightEnd => self.lit(p, b"]"),
            Rx::EmbeddedCodeEnvAt0At2 => self.lang_arg(p, BASH),
            Rx::EmbeddedCodeEnvAt0At2End | Rx::EmbeddedCodeEnvAt0At18End => {
                self.line_closes(p, engine::closes_listing)
            }
            Rx::EmbeddedCodeEnvAt0At4 => self.lang_arg(p, CSS),
            Rx::EmbeddedCodeEnvAt0At6 => self.lang_arg(p, HASKELL),
            Rx::EmbeddedCodeEnvAt0At7 => self.lang_arg(p, HTML),
            Rx::EmbeddedCodeEnvAt0At8 => self.lang_arg(p, JAVA),
            Rx::EmbeddedCodeEnvAt0At9 => self.lang_arg(p, JULIA),
            Rx::EmbeddedCodeEnvAt0At10 => self.lang_arg(p, JAVASCRIPT),
            Rx::EmbeddedCodeEnvAt0At11 => self.lang_arg(p, LUA),
            Rx::EmbeddedCodeEnvAt0At12 => self.lang_arg(p, PYTHON),
            Rx::EmbeddedCodeEnvAt0At13 => self.lang_arg(p, RUBY),
            Rx::EmbeddedCodeEnvAt0At14 => self.lang_arg(p, RUST),
            Rx::EmbeddedCodeEnvAt0At15 => self.lang_arg(p, TYPESCRIPT),
            Rx::EmbeddedCodeEnvAt0At16 => self.lang_arg(p, XML),
            Rx::EmbeddedCodeEnvAt0At17 => self.lang_arg(p, YAML),
            Rx::EmbeddedCodeEnvAt0At18 => {
                if !self.after_arg(p) {
                    return None;
                }
                Some(self.letters_arg(p).unwrap_or(p))
            }
            Rx::EmbeddedCodeEnvAt1 => self.code_env_begin(p, engine::ASY),
            Rx::EmbeddedCodeEnvAt1End => self.code_env_end(p, engine::ASY),
            Rx::EmbeddedCodeEnvAt1At1 | Rx::DocumentclassUsepackageMacroAt1 => {
                if !self.after_arg(p) {
                    return None;
                }
                let end = self.lit(p, b"{")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt1At1End => {
                let end = self.lit(p, b"}")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt2 => self.code_env_begin(p, engine::CPP),
            Rx::EmbeddedCodeEnvAt2End => self.code_env_end(p, engine::CPP),
            Rx::EmbeddedCodeEnvAt3 => self.code_env_begin(p, engine::DOT),
            Rx::EmbeddedCodeEnvAt3End => self.code_env_end(p, engine::DOT),
            Rx::EmbeddedCodeEnvAt4 => self.code_env_begin(p, engine::GNUPLOT),
            Rx::EmbeddedCodeEnvAt4End => self.code_env_end(p, engine::GNUPLOT),
            Rx::EmbeddedCodeEnvAt5 => self.code_env_begin(p, engine::HSCODE),
            Rx::EmbeddedCodeEnvAt5End => self.code_env_end(p, engine::HSCODE),
            Rx::EmbeddedCodeEnvAt5At2 => (p == 0).then_some(0),
            Rx::EmbeddedCodeEnvAt5At2End => self.line_closes(p, engine::closes_hscode),
            Rx::EmbeddedCodeEnvAt6 => self.code_env_begin(p, engine::JAVA),
            Rx::EmbeddedCodeEnvAt6End => self.code_env_end(p, engine::JAVA),
            Rx::EmbeddedCodeEnvAt6At2End => self.line_closes(p, engine::closes_java),
            Rx::EmbeddedCodeEnvAt7 => self.code_env_begin(p, engine::JL),
            Rx::EmbeddedCodeEnvAt7End => self.code_env_end(p, engine::JL),
            Rx::EmbeddedCodeEnvAt7At2End => self.line_closes(p, engine::closes_jl),
            Rx::EmbeddedCodeEnvAt8 => self.code_env_begin(p, engine::JULIA),
            Rx::EmbeddedCodeEnvAt8End => self.code_env_end(p, engine::JULIA),
            Rx::EmbeddedCodeEnvAt8At2End => self.line_closes(p, engine::closes_julia),
            Rx::EmbeddedCodeEnvAt9 => self.code_env_begin(p, engine::LUA),
            Rx::EmbeddedCodeEnvAt9End => self.code_env_end(p, engine::LUA),
            Rx::EmbeddedCodeEnvAt9At2End => self.line_closes(p, engine::closes_lua),
            Rx::EmbeddedCodeEnvAt10 => self.code_env_begin(p, engine::PY),
            Rx::EmbeddedCodeEnvAt10End => self.code_env_end(p, engine::PY),
            Rx::EmbeddedCodeEnvAt10At2End => self.line_closes(p, engine::closes_py),
            Rx::EmbeddedCodeEnvAt11 => self.code_env_begin(p, engine::PYLAB),
            Rx::EmbeddedCodeEnvAt11End => self.code_env_end(p, engine::PYLAB),
            Rx::EmbeddedCodeEnvAt11At2End => self.line_closes(p, engine::closes_pylab),
            Rx::EmbeddedCodeEnvAt12 => self.code_env_begin(p, engine::SAGE),
            Rx::EmbeddedCodeEnvAt12End => self.code_env_end(p, engine::SAGE),
            Rx::EmbeddedCodeEnvAt12At2End => self.line_closes(p, engine::closes_sage),
            Rx::EmbeddedCodeEnvAt13 => self.code_env_begin(p, engine::SCALA),
            Rx::EmbeddedCodeEnvAt13End => self.code_env_end(p, engine::SCALA),
            Rx::EmbeddedCodeEnvAt13At2End => self.line_closes(p, engine::closes_scalacode),
            Rx::EmbeddedCodeEnvAt14 => self.code_env_begin(p, engine::SYMPY),
            Rx::EmbeddedCodeEnvAt14End => self.code_env_end(p, engine::SYMPY),
            Rx::EmbeddedCodeEnvAt14At2End => self.line_closes(p, engine::closes_sympy),
            Rx::EmbeddedCodeEnvAt15 => {
                let (start, close) = self.env_name(self.space_end(p), b"\\begin{", any_code_env)?;
                let mut end = close + 1;
                if self.at(end) == b'['
                    && end < self.lim()
                    && let Some(last) = memrchr(b']', self.text(end + 1, self.lim()))
                {
                    end += last + 2;
                }
                if self.at(end) == b'{'
                    && end < self.lim()
                    && let Some(last) = memrchr(b'}', self.text(end + 1, self.lim()))
                {
                    end += last + 2;
                }
                self.cap(1, start, close);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt15End => {
                let (_, _, end) = self.backref_end(p, b"\\end{")?;
                Some(self.blank_line_end(end))
            }
            Rx::EmbeddedCodeEnvAt16 => {
                let at = if p == 0 { self.space_end(0) } else { p };
                let (start, close) = self.env_name(at, b"\\begin{", engine::cache_name)?;
                let mut i = close + 1;
                for _ in 0..2 {
                    match self.closed_at(i, b'[', b']') {
                        Some(end) => i = end,
                        None => break,
                    }
                }
                (self.at(i) == b'{' && i < self.lim()).then(|| {
                    self.cap(1, p, close + 1);
                    self.cap(2, start, close);
                    i
                })
            }
            Rx::EmbeddedCodeEnvAt16End => {
                let (_, _, end) = self.backref_end(p, b"\\end{")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt16At0 => self.cache_lang(p, ASYMPTOTE),
            Rx::EmbeddedCodeEnvAt16At0End => {
                self.env_name(p, b"\\end{", engine::cache_name).map(|_| p)
            }
            Rx::EmbeddedCodeEnvAt16At0At0 => self.at_g(p).then_some(p),
            Rx::EmbeddedCodeEnvAt16At0At0End => {
                let brace = self.lit(p, b"}")?;
                let end = self.space_end(brace);
                (end >= self.lim()).then(|| {
                    self.cap(1, p, brace);
                    end
                })
            }
            Rx::EmbeddedCodeEnvAt16At1 => self.cache_lang(p, BASH),
            Rx::EmbeddedCodeEnvAt16At1At1 => {
                if p != 0 {
                    return None;
                }
                let end = self.space_end(0);
                self.cap(1, 0, end);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt16At1At1End => self.line_closes(p, engine::closes_cache),
            Rx::EmbeddedCodeEnvAt16At2 => self.cache_lang(p, CPP),
            Rx::EmbeddedCodeEnvAt16At3 => self.cache_lang(p, CSS),
            Rx::EmbeddedCodeEnvAt16At4 => self.cache_lang(p, GNUPLOT),
            Rx::EmbeddedCodeEnvAt16At5 => self.cache_lang(p, HASKELL),
            Rx::EmbeddedCodeEnvAt16At6 => self.cache_lang(p, HTML),
            Rx::EmbeddedCodeEnvAt16At7 => self.cache_lang(p, JAVA),
            Rx::EmbeddedCodeEnvAt16At8 => self.cache_lang(p, JULIA),
            Rx::EmbeddedCodeEnvAt16At9 => self.cache_lang(p, JAVASCRIPT),
            Rx::EmbeddedCodeEnvAt16At10 => self.cache_lang(p, LUA),
            Rx::EmbeddedCodeEnvAt16At11 => self.cache_lang(p, PYTHON),
            Rx::EmbeddedCodeEnvAt16At12 => self.cache_lang(p, RUBY),
            Rx::EmbeddedCodeEnvAt16At13 => self.cache_lang(p, RUST),
            Rx::EmbeddedCodeEnvAt16At14 => self.cache_lang(p, TYPESCRIPT),
            Rx::EmbeddedCodeEnvAt16At15 => self.cache_lang(p, XML),
            Rx::EmbeddedCodeEnvAt16At16 => self.cache_lang(p, YAML),
            Rx::EmbeddedCodeEnvAt16At17 => self.cache_lang(p, TIKZ),
            Rx::EmbeddedCodeEnvAt16At18 => {
                if !self.at_g(p) {
                    return None;
                }
                let end = self.lit(p, b"{")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::EmbeddedCodeEnvAt17 => {
                self.listing_begin(p, |name| engine::is_named(name, engine::TERMINAL, true))
            }
            Rx::EmbeddedCodeEnvAt17At1 => {
                if !self.after_arg(p) {
                    return None;
                }
                self.letters_arg(p)
            }
            Rx::EmbeddedCodeEnvAt17At1End => self.line_closes(p, engine::closes_terminal),
            Rx::VerbatimEnvAt0 => self.named_env(p, b"\\begin{", |name| {
                engine::is_named(
                    name,
                    &[
                        b"fboxverbatim",
                        b"boxedverbatim",
                        b"Verbatim",
                        b"verbatim",
                        b"spverbatim",
                    ],
                    true,
                )
            }),
            Rx::VerbatimEnvAt1 => {
                let open = self.lit(self.space_end(p), b"\\begin{VerbatimOut}{")?;
                let close = memchr(b'}', self.text(open, self.lim()))?;
                let end = open + close + 1;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::VerbatimEnvAt1End => self.capped_end(p, b"\\end{VerbatimOut}", false),
            Rx::VerbatimEnvAt2 => self.whole_env(p, b"alltt"),
            Rx::VerbatimEnvAt2End => self.capped_end(p, b"\\end{alltt}", false),
            Rx::VerbatimEnvAt2At0 => {
                let name = self.lit(p, b"\\")?;
                let end = self.skip(name, |b| b.is_ascii_alphabetic());
                (end > name).then(|| {
                    self.cap(1, p, name);
                    end
                })
            }
            Rx::VerbatimEnvAt3 => self.named_env(p, b"\\begin{", |name| {
                name == b"comment" || name == b"Comment"
            }),
            Rx::DocumentEnvAt0 => self.whole_env(p, b"document"),
            Rx::DocumentEnvAt1 => {
                let end = self.lit(self.space_end(p), b"\\end{document}")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::AllBalancedEnvAt0 => {
                let start = self.space_end(p);
                let (name, close) = self.env_name(start, b"\\begin{", math_env)?;
                let after = close + 1;
                let end = self.blank_line_end(after);
                self.cap(1, start, start + 6);
                self.cap(2, start, start + 1);
                self.cap(3, start + 6, name);
                self.cap(4, name, close);
                self.cap(5, close, after);
                if end > after {
                    self.cap(6, after, end);
                }
                Some(end)
            }
            Rx::AllBalancedEnvAt0End => {
                let start = self.space_end(p);
                let (name, close, after) = self.backref_end(start, b"\\end{")?;
                self.cap(1, start, start + 4);
                self.cap(2, start, start + 1);
                self.cap(3, start + 4, name);
                self.cap(4, name, close);
                self.cap(5, close, after);
                Some(self.blank_line_end(after))
            }
            Rx::AllBalancedEnvAt0At0 => {
                (self.at(p) == b'&' && p < self.lim() && !self.prev_is(p, b'\\')).then_some(p + 1)
            }
            Rx::LabelMacro => self.command_args(p, &[b"zlabel", b"label"], usize::MAX),
            Rx::OptionalArgBracketAt0 => {
                let close = self.closed_at(p, b'[', b']')? - 1;
                self.cap(1, p, p + 1);
                self.cap(2, p + 1, close);
                self.cap(3, close, close + 1);
                Some(close + 1)
            }
            Rx::LabelMacroAt0 => self.nonempty_run(p, LABEL),
            Rx::AllBalancedEnvAt1 => {
                let start = self.space_end(p);
                let mut end = self.lit(start, b"\\begin{empheq}")?;
                if self.at(end) == b'['
                    && end < self.lim()
                    && let Some(last) = memrchr(b']', self.text(end + 1, self.lim()))
                {
                    end += last + 2;
                }
                self.cap(1, start, end);
                Some(end)
            }
            Rx::AllBalancedEnvAt1End => {
                let start = self.space_end(p);
                let end = self.lit(start, b"\\end{empheq}")?;
                self.cap(1, start, end);
                Some(end)
            }
            Rx::AllBalancedEnvAt2 => {
                let (name, close) = self.env_name(self.space_end(p), b"\\begin{", |name| {
                    TABLES.contains(&name)
                })?;
                let after = close + 1;
                let end = self.blank_line_end(after);
                self.cap(1, p, end);
                self.cap(2, name, close);
                if end > after {
                    self.cap(3, after, end);
                }
                Some(end)
            }
            Rx::AllBalancedEnvAt2End => {
                let (name, close, after) = self.backref_end(self.space_end(p), b"\\end{")?;
                let end = self.blank_line_end(after);
                self.cap(1, p, end);
                self.cap(2, name, close);
                Some(end)
            }
            Rx::AllBalancedEnvAt3 => self.named_env(p, b"\\begin{", |name| {
                [&b"itemize"[..], b"enumerate", b"description", b"list"].contains(&name)
            }),
            Rx::AllBalancedEnvAt3End => {
                let (_, _, after) = self.backref_end(p, b"\\end{")?;
                let end = self.blank_line_end(after);
                self.cap(1, p, end);
                Some(end)
            }
            Rx::AllBalancedEnvAt4 => self.whole_env(p, b"tikzpicture"),
            Rx::AllBalancedEnvAt4End => self.capped_end(p, b"\\end{tikzpicture}", true),
            Rx::AllBalancedEnvAt5 => self.whole_env(p, b"frame"),
            Rx::AllBalancedEnvAt5End => self.capped_end(p, b"\\end{frame}", false),
            Rx::AllBalancedEnvAt6 => {
                self.named_env(p, b"\\begin{", |name| name == b"mpost" || name == b"mpost*")
            }
            Rx::AllBalancedEnvAt8 => self.named_env(p, b"\\begin{", alphabetic_env),
            Rx::DocumentclassUsepackageMacro => {
                let name = self.lit(p, b"\\")?;
                let end = self
                    .lit(name, b"usepackage")
                    .or_else(|| self.lit(name, b"documentclass"))?;
                self.opens_arg(end).then(|| {
                    self.cap(1, p, end);
                    self.cap(2, p, name);
                    end
                })
            }
            Rx::DocumentclassUsepackageMacroEnd => self.prev_is(p, b'}').then_some(p),
            Rx::MultilineOptionalArg => {
                if !self.at_g(p) {
                    return None;
                }
                self.lit(p, b"[")
            }
            Rx::InputMacro => self.command_brace(p, &[b"include", b"input"]),
            Rx::SectionsMacro => {
                let name = self.lit(p, b"\\")?;
                let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
                if !SECTIONS.contains(&self.text(name, name_end)) {
                    return None;
                }
                let star = self.star_end(name_end);
                let brace = self.lazy_brackets_to_brace(star, 2)?;
                self.cap(1, p, star);
                self.cap(2, p, name);
                self.cap(3, name, name_end);
                self.cap(4, star, brace);
                self.cap(5, brace, brace + 1);
                Some(brace + 1)
            }
            Rx::TextFontMacroAt0 => self.command_brace(p, &[b"emph"]),
            Rx::TextFontMacroAt1 => self.command_brace(p, &[b"textit"]),
            Rx::TextFontMacroAt2 => self.command_brace(p, &[b"textbf"]),
            Rx::TextFontMacroAt3 => self.command_brace(p, &[b"texttt"]),
            Rx::CitationMacro => self.citation(p),
            Rx::AutocitesArgAt0 => {
                let parens = self.closed_groups(p, b'(', b')', 2);
                let brackets = self.closed_groups(parens, b'[', b']', 2);
                let key = self.lit(brackets, b"{")?;
                let key_end = self.nonempty_run(key, KEY)?;
                let after = self.lit(key_end, b"}")?;
                self.cap(1, p, parens);
                self.cap(2, parens, brackets);
                self.cap(3, brackets, key);
                self.cap(4, key, key_end);
                self.cap(5, key_end, after);
                self.cap(6, after, self.lim());
                Some(self.lim())
            }
            Rx::OptionalArgParenthesisNoHighlightAt0 => self.delimited_pair(p, b'(', b')'),
            Rx::OptionalArgBracketNoHighlightAt0 => self.delimited_pair(p, b'[', b']'),
            Rx::OptionalArgAngleNoHighlightAt0 => self.delimited_pair(p, b'<', b'>'),
            Rx::CitationMacroAt0 => {
                if self.at(p) != b'%' || p >= self.lim() {
                    return None;
                }
                self.cap(1, p, self.lim());
                self.cap(2, p, p + 1);
                Some(self.lim())
            }
            Rx::CitationMacroAt1 => self.nonempty_run(p, b"-.:"),
            Rx::ReferencesMacroAt0 => {
                let (star, brace) = self.reference(p, b"ref", b"Ref")?;
                self.cap(1, p, star);
                self.cap(2, p, p + 1);
                self.cap(3, brace, brace + 1);
                Some(brace + 1)
            }
            Rx::ReferencesMacroAt1 => {
                let (star, brace) = self.reference(p, b"refrange", b"Refrange")?;
                let first = self.nonempty_run(brace + 1, LABEL)?;
                let second_open = self.lit(first, b"}")?;
                let second = self.lit(second_open, b"{")?;
                let second_end = self.nonempty_run(second, LABEL)?;
                let end = self.lit(second_end, b"}")?;
                self.cap(1, p, star);
                self.cap(2, p, p + 1);
                self.cap(3, brace, brace + 1);
                self.cap(4, brace + 1, first);
                self.cap(5, first, second_open);
                self.cap(6, second_open, second);
                self.cap(7, second, second_end);
                self.cap(8, second_end, end);
                Some(end)
            }
            Rx::ReferencesMacroAt2 => self.command_brace(p, &[b"bibentry"]),
            Rx::ReferencesMacroAt2At0 => self.nonempty_run(p, b".:"),
            Rx::AllOtherMacroAt0 => {
                let name = self.lit(p, b"\\")?;
                self.word_in(name, BREAKS)
            }
            Rx::AllOtherMacroAt1 => self.command_args(p, &[b"marginpar"], usize::MAX),
            Rx::AllOtherMacroAt2 => self.command_args(p, &[b"footnote"], usize::MAX),
            Rx::AllOtherMacroAt3 => {
                let name = self.lit(p, b"\\")?;
                let end = self.keyword(name, b"item")?;
                self.cap(1, p, name);
                Some(end)
            }
            Rx::AllOtherMacroAt4 => {
                let name = self.lit(p, b"\\")?;
                let end = self.word_end(name);
                let lists = listed(self.text(name, end), WORD_BYTES, &WORD_ENDS, &WORD_LISTS);
                (lists & TEXT_SYMBOL != 0).then(|| {
                    self.cap(1, p, name);
                    self.cap(2, name, end);
                    end
                })
            }
            Rx::AllOtherMacroAt5 => {
                let name = self.lit(p, b"\\")?;
                let end = self.expl3_variable(name)?;
                self.cap(1, p, name);
                Some(end)
            }
            Rx::HyperrefMacroAt0 => {
                let start = self.space_end(p);
                let name = self.lit(start, b"\\h")?;
                let end = [&b"ref"[..], b"yperref", b"yperimage"]
                    .into_iter()
                    .find_map(|rest| self.lit(name, rest))?;
                self.opens_arg(end).then(|| {
                    self.cap(1, start, end);
                    self.cap(2, start, start + 1);
                    end
                })
            }
            Rx::HyperrefMacroAt0At1 => {
                if !self.after_arg(p) {
                    return None;
                }
                let after = self.closed_at(p, b'{', b'}')?;
                let two = self
                    .closed_at(after, b'{', b'}')
                    .and_then(|i| self.closed_at(i, b'{', b'}'))
                    .filter(|&i| self.at(i) == b'{' && i < self.lim());
                let brace = two
                    .or_else(|| (self.at(after) == b'{' && after < self.lim()).then_some(after))?;
                self.cap(1, p, p + 1);
                self.cap(2, p + 1, after - 1);
                self.cap(3, after - 1, after);
                self.cap(4, brace, brace + 1);
                Some(brace + 1)
            }
            Rx::HyperrefMacroAt0At2 => {
                if !self.after_arg(p) {
                    return None;
                }
                if let Some(after) = self.closed_at(p, b'{', b'}')
                    && self.at(after) == b'{'
                    && after < self.lim()
                {
                    self.cap(1, p, p + 1);
                    self.cap(2, after - 1, after);
                    self.cap(3, after, after + 1);
                    return Some(after + 1);
                }
                let end = self.lit(p, b"{")?;
                self.cap(3, p, end);
                Some(end)
            }
            Rx::HyperrefMacroAt1 => {
                let start = self.space_end(p);
                let name = self.lit(start, b"\\")?;
                let name_end = self.lit(name, b"url").or_else(|| self.lit(name, b"path"))?;
                let after = self.closed_at(name_end, b'{', b'}')?;
                self.cap(1, start, name_end);
                self.cap(2, start, name);
                self.cap(3, name_end, name_end + 1);
                self.cap(4, name_end + 1, after - 1);
                self.cap(5, after - 1, after);
                Some(after)
            }
            Rx::NewcommandMacro => self.newcommand(p),
            Rx::UnbalancedEnvAt0 => {
                let start = self.space_end(p);
                let keyword = if self.lit(start, b"\\begin{").is_some() {
                    &b"\\begin{"[..]
                } else {
                    b"\\end{"
                };
                self.named_env(p, keyword, alphabetic_env)
            }
            Rx::VerbMacroAt0 => {
                let name = self.verb_name(p)?;
                let scan = self.space_end(name);
                let scan_end = self.lit(scan, b"\\scantokens")?;
                let end = self.lit(scan_end, b"{")?;
                self.cap(1, p, name);
                self.cap(2, p, p + 1);
                self.cap(3, scan, scan_end);
                self.cap(4, scan, scan + 1);
                self.cap(5, scan_end, end);
                Some(end)
            }
            Rx::VerbMacroAt1 => self.verb(p),
            Rx::InlineCodeMacroAt0 => self.addplot(p),
            Rx::InlineCodeMacroAt0End => {
                let start = self.space_end(p);
                let end = self.lit(start, b"};")?;
                self.cap(1, start, end);
                Some(end)
            }
            Rx::InlineCodeMacroAt0At0 => self.lit(p, b"%"),
            Rx::InlineCodeMacroAt1 => self.mint(p),
            Rx::InlineCodeMacroAt2 => {
                let name = self.lit(p, b"\\")?;
                let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
                let word = self.text(name, name_end);
                let inline = word.len() > 6
                    && word.ends_with(b"inline")
                    && word.iter().all(u8::is_ascii_lowercase);
                if !inline {
                    return None;
                }
                self.inline_code(p, name_end, letter_or_brace)
            }
            Rx::InlineCodeMacroAt3 => {
                let name = self.lit(p, b"\\")?;
                let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
                if !python_inline(self.text(name, name_end)) {
                    return None;
                }
                self.inline_code(p, name_end, |c| {
                    c.is_ascii_alphabetic()
                        || matches!(c, ']' | '(' | ')' | ',' | ';' | '[' | '{' | '}')
                        || crate::classes::is_space(c)
                })
            }
            Rx::InlineCodeMacroAt4 => {
                let name = self.lit(p, b"\\")?;
                let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
                let word = self.text(name, name_end);
                let stem = word
                    .strip_suffix(b"c")
                    .or_else(|| word.strip_suffix(b"v"))
                    .unwrap_or(word);
                let julia = [&b"jl"[..], b"julia"].contains(&word)
                    || [&b"jl"[..], b"julia"].contains(&stem);
                if !julia {
                    return None;
                }
                self.inline_code(p, name_end, letter_or_brace)
            }
            Rx::InlineCodeMacroAt5 => {
                self.command_brace(p, &[b"directlua", b"luadirect", b"luaexec"])
            }
            Rx::InlineCodeMacroAt6 => self.cache_me_code(p, Some(ASYMPTOTE)),
            Rx::InlineCodeMacroAt7 => self.cache_me_code(p, Some(BASH)),
            Rx::InlineCodeMacroAt7At1 => {
                if !self.prev_is(p, b']') {
                    return None;
                }
                let end = self.lit(p, b"{")?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::InlineCodeMacroAt8 => self.cache_me_code(p, Some(CPP)),
            Rx::InlineCodeMacroAt9 => self.cache_me_code(p, Some(CSS)),
            Rx::InlineCodeMacroAt10 => self.cache_me_code(p, Some(GNUPLOT)),
            Rx::InlineCodeMacroAt11 => self.cache_me_code(p, Some(HASKELL)),
            Rx::InlineCodeMacroAt12 => self.cache_me_code(p, Some(HTML)),
            Rx::InlineCodeMacroAt13 => self.cache_me_code(p, Some(JAVA)),
            Rx::InlineCodeMacroAt14 => self.cache_me_code(p, Some(JULIA)),
            Rx::InlineCodeMacroAt15 => self.cache_me_code(p, Some(JAVASCRIPT)),
            Rx::InlineCodeMacroAt16 => self.cache_me_code(p, Some(LUA)),
            Rx::InlineCodeMacroAt17 => self.cache_me_code(p, Some(PYTHON)),
            Rx::InlineCodeMacroAt18 => self.cache_me_code(p, Some(RUBY)),
            Rx::InlineCodeMacroAt19 => self.cache_me_code(p, Some(RUST)),
            Rx::InlineCodeMacroAt20 => self.cache_me_code(p, Some(TYPESCRIPT)),
            Rx::InlineCodeMacroAt21 => self.cache_me_code(p, Some(XML)),
            Rx::InlineCodeMacroAt22 => self.cache_me_code(p, Some(YAML)),
            Rx::InlineCodeMacroAt23 => self.cache_me_code(p, Some(TIKZ)),
            Rx::InlineCodeMacroAt24 => self.cache_me_code(p, None),
            Rx::InlineCodeMacroAt25 => {
                let name = self.lit(p, b"\\sys_get_shell:nnN")?;
                let name = self.lit(name, b"TF").unwrap_or(name);
                let name = name + self.space_at(name);
                let end = self.lit(name, b"{")?;
                self.cap(1, p, name);
                self.cap(2, p, p + 1);
                self.cap(3, name, end);
                Some(end)
            }
            Rx::DisplayMathAt0 => self.lit(p, b"\\["),
            Rx::DisplayMathAt0End => self.lit(p, b"\\]"),
            Rx::DisplayMathAt1 => self.lit(p, b"$$"),
            Rx::InlineMathAt0 => self.lit(p, b"\\("),
            Rx::InlineMathAt0End => self.lit(p, b"\\)"),
            Rx::InlineMathAt1 => {
                let end = self.lit(p, b"$")?;
                (self.at(end) != b'$').then_some(end)
            }
            Rx::InlineMathAt1End => {
                (self.at(p) == b'$' && p < self.lim() && !self.prev_is(p, b'$')).then_some(p + 1)
            }
            Rx::ColumnSpecials => {
                if !matches!(self.at(p), b'<' | b'>') || p >= self.lim() {
                    return None;
                }
                let end = self.lit(p + 1, b"{$}")?;
                self.cap(1, p + 1, p + 2);
                self.cap(2, p + 3, end);
                Some(end)
            }
            Rx::SongsChordsAt1 => self.lit(p, b"^"),
            Rx::SongsEnvAt1 => {
                let start = self.space_end(p);
                let end = self.lit(start, b"\\beginsong")?;
                (self.at(end) == b'{' && end < self.lim()).then(|| {
                    self.cap(1, start, end);
                    self.cap(2, start, start + 1);
                    end
                })
            }
            Rx::SongsEnvAt1End => {
                let end = self.lit(p, b"\\endsong")?;
                self.cap(1, p, end);
                self.cap(2, p, p + 1);
                Some(self.blank_line_end(end))
            }
            Rx::MultilineArgNoHighlight => {
                if !self.at_g(p) {
                    return None;
                }
                self.lit(p, b"{")
            }
            Rx::SongsEnvAt1At2 => {
                let after = self.at_g(p) || self.prev_is(p, b']') || self.prev_is(p, b'}');
                after.then(|| self.space_end(p))
            }
            Rx::SongsEnvAt1At2End => {
                let end = self.space_end(p);
                self.lit(end, b"\\endsong").map(|_| end)
            }
        }
    }

    /// `(?<=\\(?:[@\w]|…|[@\w]{6}))\s` at `p`: a blank after a command of up
    /// to six characters.
    fn space_after_short_command(&self, p: usize) -> Option<usize> {
        let width = self.space_at(p);
        if width == 0 {
            return None;
        }
        let mut i = p;
        for _ in 0..6 {
            let c = self.char_before(i)?;
            if c != '@' && !is_word(c) {
                return None;
            }
            i = self.char_start_before(i);
            if self.prev_is(i, b'\\') {
                return Some(p + width);
            }
        }
        None
    }

    /// `keyword` at `i` and the name up to the next `}`, when `named` accepts
    /// it: where the name starts, and where its `}` stands.
    fn env_name(
        &self,
        i: usize,
        keyword: &[u8],
        named: impl Fn(&[u8]) -> bool,
    ) -> Option<(usize, usize)> {
        let start = self.lit(i, keyword)?;
        let close = start + memchr(b'}', self.text(start, self.lim()))?;
        named(self.text(start, close)).then_some((start, close))
    }

    /// `(\s*KEYWORD(NAME)})` at `p`, the name one that `named` accepts.
    fn named_env(
        &mut self,
        p: usize,
        keyword: &[u8],
        named: impl Fn(&[u8]) -> bool,
    ) -> Option<usize> {
        let (start, close) = self.env_name(self.space_end(p), keyword, named)?;
        self.cap(1, p, close + 1);
        self.cap(2, start, close);
        Some(close + 1)
    }

    /// `(\s*\\begin\{NAME})` at `p`.
    fn whole_env(&mut self, p: usize, name: &[u8]) -> Option<usize> {
        let (_, close) = self.env_name(self.space_end(p), b"\\begin{", |text| text == name)?;
        self.cap(1, p, close + 1);
        Some(close + 1)
    }

    /// `(TEXT)` at `p`, with `(?:\s*\n)?` after it when `line`.
    fn capped_end(&mut self, p: usize, text: &[u8], line: bool) -> Option<usize> {
        let end = self.lit(p, text)?;
        let end = if line { self.blank_line_end(end) } else { end };
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?:\s*\n)?` at `i`: the rest of the line with its line feed when the
    /// rest is blank, and `i` otherwise.
    fn blank_line_end(&self, i: usize) -> usize {
        let end = self.space_end(i);
        if end > self.lim() { end } else { i }
    }

    /// `keyword`, the text the innermost rule's begin captured, and `}` at
    /// `i`: where the text starts, where it ends, and the end.
    fn backref_end(&self, i: usize, keyword: &[u8]) -> Option<(usize, usize, usize)> {
        let start = self.lit(i, keyword)?;
        let close = start + self.payload_at(start)?;
        let end = self.lit(close, b"}")?;
        Some((start, close, end))
    }

    /// `(?:^\s*)?\\begin\{(NAME)}(?=[\[{])` at `p`.
    fn listing_begin(&mut self, p: usize, named: impl Fn(&[u8]) -> bool) -> Option<usize> {
        let at = if p == 0 { self.space_end(0) } else { p };
        let (start, close) = self.env_name(at, b"\\begin{", named)?;
        self.opens_arg(close + 1).then(|| {
            self.cap(1, start, close);
            close + 1
        })
    }

    /// `\s*\\begin\{NAME\*?}(?:\[[-0-9A-Z_a-z]*])?(?=[\[{]|\s*$)` at `p`.
    fn code_env_begin(&self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let (_, close) = self.env_name(self.space_end(p), b"\\begin{", |name| {
            engine::is_named(name, names, true)
        })?;
        let after = close + 1;
        if self.at(after) == b'[' && after < self.lim() {
            let inner = self.skip(after + 1, |b| {
                b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
            });
            if self.at(inner) == b']' && inner < self.lim() && self.opens_or_ends(inner + 1) {
                return Some(inner + 1);
            }
        }
        self.opens_or_ends(after).then_some(after)
    }

    /// `\s*\\end\{NAME\*?}` at `p`.
    fn code_env_end(&self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let (_, close) = self.env_name(self.space_end(p), b"\\end{", |name| {
            engine::is_named(name, names, true)
        })?;
        Some(close + 1)
    }

    /// `(?=[\[{]|\s*$)` at `i`.
    fn opens_or_ends(&self, i: usize) -> bool {
        self.opens_arg(i) || self.space_end(i) >= self.lim()
    }

    /// `(?=[\[{])` at `i`.
    fn opens_arg(&self, i: usize) -> bool {
        matches!(self.at(i), b'[' | b'{') && i < self.lim()
    }

    /// `(?:\G|(?<=]))` at `p`.
    fn after_arg(&self, p: usize) -> bool {
        self.at_g(p) || self.prev_is(p, b']')
    }

    /// `^\s*(?=\\end\{…})` at `p`, the environment one that `closes`
    /// accepts.
    fn line_closes(&self, p: usize, closes: fn(&[u8], usize) -> bool) -> Option<usize> {
        (p == 0 && closes(self.text(0, self.lim()), 0)).then(|| self.space_end(0))
    }

    /// `(?:\G|(?<=]))(\{)(LANGUAGE)(})` at `p`.
    fn lang_arg(&mut self, p: usize, langs: &[&[u8]]) -> Option<usize> {
        if !self.after_arg(p) {
            return None;
        }
        let (start, close) = self.env_name(p, b"{", |name| langs.contains(&name))?;
        self.cap(1, p, start);
        self.cap(2, start, close);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

    /// `(\{)([A-Za-z]*)(})` at `p`.
    fn letters_arg(&mut self, p: usize) -> Option<usize> {
        let start = self.lit(p, b"{")?;
        let close = self.skip(start, |b| b.is_ascii_alphabetic());
        let end = self.lit(close, b"}")?;
        self.cap(1, p, start);
        self.cap(2, start, close);
        self.cap(3, close, end);
        Some(end)
    }

    /// `\G(\{)(?:__|[a-z\s]*)(?i:LANGUAGE)` at `p`.
    /// The run of `[a-z\s]` gives back a character at a time from its end
    /// until the language follows.
    fn cache_lang(&mut self, p: usize, langs: &[&[u8]]) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let start = self.lit(p, b"{")?;
        let end = self
            .lit(start, b"__")
            .and_then(|i| self.lang_ci(i, langs))
            .or_else(|| {
                let mut run = start;
                loop {
                    let width = if self.at(run).is_ascii_lowercase() && run < self.lim() {
                        1
                    } else {
                        self.space_at(run)
                    };
                    if width == 0 {
                        break;
                    }
                    run += width;
                }
                loop {
                    if let Some(end) = self.lang_ci(run, langs) {
                        return Some(end);
                    }
                    if run <= start {
                        return None;
                    }
                    run = self.char_start_before(run);
                }
            })?;
        self.cap(1, p, start);
        Some(end)
    }

    /// The first of `langs`, in order, at `i` without regard to case: where it
    /// ends.
    fn lang_ci(&self, i: usize, langs: &[&[u8]]) -> Option<usize> {
        let text = self.text(0, self.lim());
        langs
            .iter()
            .find_map(|lang| case_fold::literal_end(text, i, lang))
    }

    /// `((\\)cacheMeCode)(?=\[(?i:LANGUAGE)\b|\{)` at `p`, or
    /// `(?=[\[{])` without a language.
    fn cache_me_code(&mut self, p: usize, langs: Option<&[&[u8]]>) -> Option<usize> {
        let end = self.lit(p, b"\\cacheMeCode")?;
        let ahead = match langs {
            Some(langs) => {
                let text = self.text(0, self.lim());
                let named = self.lit(end, b"[").is_some_and(|i| {
                    langs.iter().any(|lang| {
                        case_fold::literal_end(text, i, lang).is_some_and(|e| self.boundary(e))
                    })
                });
                named || (self.at(end) == b'{' && end < self.lim())
            }
            None => self.opens_arg(end),
        };
        ahead.then(|| {
            self.cap(1, p, end);
            self.cap(2, p, p + 1);
            end
        })
    }

    /// `((\\)NAME)(\{)` at `p`, with the first of `names` that a `{`
    /// follows.
    fn command_brace(&mut self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let name_end = names.iter().find_map(|word| {
            self.lit(name, word)
                .filter(|&e| self.at(e) == b'{' && e < self.lim())
        })?;
        self.cap(1, p, name_end);
        self.cap(2, p, name);
        self.cap(3, name_end, name_end + 1);
        Some(name_end + 1)
    }

    /// `((\\)NAME)((?:\[[^\[]*?]){0,max})(\{)` at `p`, with the first of
    /// `names` that the groups and the `{` follow.
    fn command_args(&mut self, p: usize, names: &[&[u8]], max: usize) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let (name_end, brace) = names.iter().find_map(|word| {
            let end = self.lit(name, word)?;
            self.lazy_brackets_to_brace(end, max)
                .map(|brace| (end, brace))
        })?;
        self.cap(1, p, name_end);
        self.cap(2, p, name);
        self.cap(3, name_end, brace);
        self.cap(4, brace, brace + 1);
        Some(brace + 1)
    }

    /// `(?:\[[^\[]*?]){0,max}` at `i` and then a `{`: where the `{` stands.
    /// A group ends at the first `]` that a `[` or the `{` follows, since its
    /// text holds no `[`.
    fn lazy_brackets_to_brace(&self, mut i: usize, max: usize) -> Option<usize> {
        let lim = self.lim();
        let mut groups = 0;
        loop {
            match self.at(i) {
                b'{' if i < lim => return Some(i),
                b'[' if i < lim && groups < max => groups += 1,
                _ => return None,
            }
            let mut j = i + 1;
            loop {
                let k = j + memchr2(b'[', b']', self.text(j, lim))?;
                if self.at(k) == b'[' {
                    return None;
                }
                match self.at(k + 1) {
                    b'[' if k + 1 < lim => {
                        i = k + 1;
                        break;
                    }
                    b'{' if k + 1 < lim => return Some(k + 1),
                    _ => j = k + 1,
                }
            }
        }
    }

    /// `OPEN[^CLOSE]*CLOSE` at `i`: the end after the first `CLOSE`.
    fn closed_at(&self, i: usize, open: u8, close: u8) -> Option<usize> {
        if self.at(i) != open || i >= self.lim() {
            return None;
        }
        memchr(close, self.text(i + 1, self.lim())).map(|k| i + k + 2)
    }

    /// `(?:OPEN[^CLOSE]*CLOSE){0,max}` at `i`: where the groups end.
    fn closed_groups(&self, mut i: usize, open: u8, close: u8, max: usize) -> usize {
        for _ in 0..max {
            match self.closed_at(i, open, close) {
                Some(end) => i = end,
                None => break,
            }
        }
        i
    }

    /// `(OPEN)[^OPEN]*?(CLOSE)` at `p`.
    fn delimited_pair(&mut self, p: usize, open: u8, close: u8) -> Option<usize> {
        if self.at(p) != open || p >= self.lim() {
            return None;
        }
        let k = p + 1 + memchr2(open, close, self.text(p + 1, self.lim()))?;
        if self.at(k) != close {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, k, k + 1);
        Some(k + 1)
    }

    /// The end of a `*` at `i`, or `i`.
    fn star_end(&self, i: usize) -> usize {
        if self.at(i) == b'*' && i < self.lim() {
            i + 1
        } else {
            i
        }
    }

    /// The end of the run from `i` of characters that are alphabetic,
    /// numbers, or in `punct`.
    fn label_run(&self, mut i: usize, punct: &[u8]) -> usize {
        while let Some((c, end)) = self.char_at(i) {
            let ok = if c.is_ascii() {
                c.is_ascii_alphanumeric() || u8::try_from(c).is_ok_and(|b| punct.contains(&b))
            } else {
                is_alpha(c) || is_number(c)
            };
            if !ok {
                break;
            }
            i = end;
        }
        i
    }

    /// [`Lexer::label_run`] when it holds one character or more.
    fn nonempty_run(&self, i: usize, punct: &[u8]) -> Option<usize> {
        let end = self.label_run(i, punct);
        (end > i).then_some(end)
    }

    /// The end of the run from `i` of `[@\p{Alphabetic}]` characters.
    fn letter_run(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && (c == '@' || is_alpha(c))
        {
            i = end;
        }
        i
    }

    /// The end of the run from `i` of `\p{Alphabetic}` characters.
    fn alpha_run(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && is_alpha(c)
        {
            i = end;
        }
        i
    }

    /// `\s*((\\)\p{Alphabetic}+)(\{)(\\?\p{Alphabetic}+\*?)(})`, up to two
    /// `(\[)([^]]*)(])` and an optional `(\{)([^{}]*)(})` at `p`.
    fn macro_with_args(&mut self, p: usize) -> Option<usize> {
        let start = self.space_end(p);
        let name = self.lit(start, b"\\")?;
        let open = self.alpha_run(name);
        if open == name {
            return None;
        }
        let arg = self.lit(open, b"{")?;
        let letters = if self.at(arg) == b'\\' && arg < self.lim() {
            arg + 1
        } else {
            arg
        };
        let letters_end = self.alpha_run(letters);
        if letters_end == letters {
            return None;
        }
        let close = self.star_end(letters_end);
        let mut end = self.lit(close, b"}")?;
        self.cap(1, start, open);
        self.cap(2, start, name);
        self.cap(3, open, arg);
        self.cap(4, arg, close);
        self.cap(5, close, end);
        for _ in 0..2 {
            let Some(after) = self.closed_at(end, b'[', b']') else {
                break;
            };
            self.cap(6, end, end + 1);
            self.cap(7, end + 1, after - 1);
            self.cap(8, after - 1, after);
            end = after;
        }
        if self.at(end) == b'{' && end < self.lim() {
            let text_end = self.skip(end + 1, |b| b != b'{' && b != b'}');
            if self.at(text_end) == b'}' && text_end < self.lim() {
                self.cap(9, end, end + 1);
                self.cap(10, end + 1, text_end);
                self.cap(11, text_end, text_end + 1);
                end = text_end + 1;
            }
        }
        Some(end)
    }

    /// `\\` and a name that `\w*` and `stem` or `upper` spell, a `*`, an
    /// optional `[…]` and `{` at `p`: where the name and its `*` end, and
    /// where the `{` stands.
    fn reference(&self, p: usize, stem: &[u8], upper: &[u8]) -> Option<(usize, usize)> {
        let name = self.lit(p, b"\\")?;
        let name_end = self.word_end(name);
        let word = self.text(name, name_end);
        if !word.ends_with(stem) && !word.ends_with(upper) {
            return None;
        }
        let star = self.star_end(name_end);
        let brace = self.closed_at(star, b'[', b']').unwrap_or(star);
        (self.at(brace) == b'{' && brace < self.lim()).then_some((star, brace))
    }

    /// `CitationMacro`: a citation command, its arguments and the `{` of its
    /// keys.
    ///
    /// The arguments repeat: up to two `(…)`, up to two `[…]` and a `{key}`.
    /// Each round is the only way its text can match, so the rounds run
    /// forward.
    ///
    /// When the tail after them, an optional `<…>`, any `[…]` and a `{`,
    /// fails, the regex gives back rounds.
    /// The first round it gives back to that can take the tail is the last
    /// one that held no `(…)`.
    fn citation(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
        if !cite_name(self.text(name, name_end)) {
            return None;
        }
        let args = self.star_end(name_end);
        let mut i = args;
        let mut last_plain = None;
        loop {
            let parens = self.closed_groups(i, b'(', b')', 2);
            let brackets = self.closed_groups(parens, b'[', b']', 2);
            let Some(key) = self.lit(brackets, b"{") else {
                break;
            };
            let key_end = self.label_run(key, KEY);
            let Some(round_end) = self.lit(key_end, b"}") else {
                break;
            };
            if parens == i {
                last_plain = Some(i);
            }
            i = round_end;
        }
        let (args_end, tail) = if let Some(tail) = self.citation_tail(i) {
            (i, tail)
        } else {
            let start = last_plain?;
            (start, self.citation_tail(start)?)
        };
        let CitationTail {
            angle,
            brackets,
            brace,
        } = tail;
        self.cap(1, p, args);
        self.cap(2, p, name);
        self.cap(3, args, args_end);
        if let Some((start, end)) = angle {
            self.cap(4, start, end);
        }
        self.cap(5, brackets, brace);
        self.cap(6, brace, brace + 1);
        Some(brace + 1)
    }

    /// `(<[^]<>]*>)?((?:\[[^]]*])*)(\{)` at `i`: the `<…>`, where the `[…]`
    /// start, and where the `{` stands.
    fn citation_tail(&self, i: usize) -> Option<CitationTail> {
        let mut angle = None;
        let mut j = i;
        if self.at(i) == b'<'
            && i < self.lim()
            && let Some(k) = memchr3(b']', b'<', b'>', self.text(i + 1, self.lim()))
            && self.at(i + 1 + k) == b'>'
        {
            angle = Some((i, i + k + 2));
            j = i + k + 2;
        }
        let brackets = j;
        while let Some(end) = self.closed_at(j, b'[', b']') {
            j = end;
        }
        (self.at(j) == b'{' && j < self.lim()).then_some(CitationTail {
            angle,
            brackets,
            brace: j,
        })
    }

    /// `(?:[cgl]_+[@_\p{Alphabetic}]+_[a-z]+|[qs]_[@_\p{Alphabetic}]+[@\
    /// p{Alphabetic}])` at `i`: an expl3 variable or constant name.
    ///
    /// The runs give back from their ends, so the match ends at the last `_`
    /// that a lowercase letter follows, or at the last letter.
    fn expl3_variable(&self, i: usize) -> Option<usize> {
        let kind = self.at(i);
        if i >= self.lim() {
            return None;
        }
        let in_class = |c: char| c == '@' || c == '_' || is_alpha(c);
        match kind {
            b'c' | b'g' | b'l' => {
                self.lit(i + 1, b"_")?;
                let mut last = None;
                let mut j = i + 1;
                let mut count = 0;
                while let Some((c, end)) = self.char_at(j)
                    && in_class(c)
                {
                    if c == '_'
                        && count >= 2
                        && self.at(end).is_ascii_lowercase()
                        && end < self.lim()
                    {
                        last = Some(end);
                    }
                    count += 1;
                    j = end;
                }
                last.map(|letters| self.skip(letters, |b| b.is_ascii_lowercase()))
            }
            b'q' | b's' => {
                let start = self.lit(i + 1, b"_")?;
                let mut last = None;
                let mut j = start;
                while let Some((c, end)) = self.char_at(j)
                    && in_class(c)
                {
                    if c != '_' && j > start {
                        last = Some(end);
                    }
                    j = end;
                }
                last
            }
            _ => None,
        }
    }

    /// `NewcommandMacro`: a command that defines a command, the name it
    /// defines, its optional arguments and the `{` of its body.
    fn newcommand(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let name_end = self.skip(name, |b| b.is_ascii_alphabetic());
        if !NEWCOMMANDS.contains(&self.text(name, name_end)) {
            return None;
        }
        let star = self.star_end(name_end);
        let open = self.lit(star, b"{");
        let command = open.unwrap_or(star);
        let letters = self.lit(command, b"\\")?;
        let letters_end = self.letter_run(letters);
        if letters_end == letters {
            return None;
        }
        let command_end = self.star_end(letters_end);
        let close = self.lit(command_end, b"}");
        let mut i = close.unwrap_or(command_end);
        let mut last = None;
        for _ in 0..2 {
            let Some(end) = self.closed_at(i, b'[', b']') else {
                break;
            };
            last = Some((i, end));
            i = end;
        }
        if self.at(i) != b'{' || i >= self.lim() {
            return None;
        }
        self.cap(1, p, star);
        self.cap(2, p, name);
        if open.is_some() {
            self.cap(3, star, star + 1);
        }
        self.cap(4, command, command_end);
        self.cap(5, command, letters);
        if close.is_some() {
            self.cap(6, command_end, command_end + 1);
        }
        if let Some((start, end)) = last {
            self.cap(7, start, start + 1);
            self.cap(8, end - 1, end);
        }
        self.cap(9, i, i + 1);
        Some(i + 1)
    }

    /// `\\(?:[Vv]|spv)erb\*?` at `p`: where it ends.
    fn verb_name(&self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"\\")?;
        let stem = self.lit(name, b"spv").or_else(|| {
            (matches!(self.at(name), b'V' | b'v') && name < self.lim()).then_some(name + 1)
        })?;
        let end = self.lit(stem, b"erb")?;
        Some(self.star_end(end))
    }

    /// `VerbMacroAt1`: `\verb`, its delimiter, the text up to the next copy of
    /// it, and that copy or the end of the line.
    ///
    /// The delimiter is the first character after the blanks: any one after a
    /// blank, or one that is not a letter.
    /// When none stands there, the blanks give back their last character.
    fn verb(&mut self, p: usize) -> Option<usize> {
        let name = self.verb_name(p)?;
        let blanks = self.space_end(name);
        let open = match self.char_at(blanks) {
            Some((c, _)) if blanks < self.lim() && (blanks > name || !c.is_ascii_alphabetic()) => {
                blanks
            }
            _ if blanks > name => self.char_start_before(blanks),
            _ => return None,
        };
        let text = if open == self.lim() {
            open + 1
        } else {
            self.char_end(open)
        };
        let delimiter = self.text(open, text);
        let close = if open == self.lim() {
            text
        } else {
            memchr::memmem::find(self.text(text, self.lim()), delimiter)
                .map_or(self.lim(), |k| text + k)
        };
        let end = if close < self.lim() {
            close + delimiter.len()
        } else {
            close
        };
        self.cap(1, p, name);
        self.cap(2, p, p + 1);
        self.cap(3, open, text);
        self.cap(4, text, close);
        self.cap(5, close, end);
        Some(end)
    }

    /// `InlineCodeMacroAt1`: `\mint` or `\mintinline`, an optional `[…]`,
    /// `{language}` and the code.
    fn mint(&mut self, p: usize) -> Option<usize> {
        let mint = self.lit(p, b"\\mint")?;
        for name_end in [Some(mint), self.lit(mint, b"inline")]
            .into_iter()
            .flatten()
        {
            let found = self.optional_bracket(name_end, |lexer, i| {
                let open = lexer.lit(i, b"{")?;
                let close = lexer.skip(open, |b| b.is_ascii_alphabetic());
                let after = lexer.lit(close, b"}")?;
                lexer
                    .code_body(after, letter_or_brace)
                    .map(|body| (i, close, body))
            });
            if let Some((bracket_end, (open, close, body))) = found {
                self.cap(1, p, name_end);
                self.cap(2, p, p + 1);
                self.cap(3, name_end, bracket_end);
                self.cap(4, open, open + 1);
                self.cap(5, close, close + 1);
                return Some(self.cap_body(body, 6));
            }
        }
        None
    }

    /// `((?:\[[^\[]*?])?)` and the code of an inline code command at
    /// `name_end`, the delimiter any character that `excluded` rejects.
    fn inline_code(
        &mut self,
        p: usize,
        name_end: usize,
        excluded: fn(char) -> bool,
    ) -> Option<usize> {
        let (bracket_end, body) =
            self.optional_bracket(name_end, |lexer, i| lexer.code_body(i, excluded))?;
        self.cap(1, p, name_end);
        self.cap(2, p, p + 1);
        self.cap(3, name_end, bracket_end);
        Some(self.cap_body(body, 4))
    }

    /// `(?:\[[^\[]*?])?` at `i` and then `rest`, tried in the regex's order:
    /// the group up to each `]` before the next `[`, nearest first, and then
    /// no group.
    fn optional_bracket<T>(
        &self,
        i: usize,
        rest: impl Fn(&Self, usize) -> Option<T>,
    ) -> Option<(usize, T)> {
        if self.at(i) == b'[' && i < self.lim() {
            let mut j = i + 1;
            while let Some(k) = memchr2(b'[', b']', self.text(j, self.lim())) {
                let at = j + k;
                if self.at(at) == b'[' {
                    break;
                }
                if let Some(found) = rest(self, at + 1) {
                    return Some((at + 1, found));
                }
                j = at + 1;
            }
        }
        rest(self, i).map(|found| (i, found))
    }

    /// `(?:([DELIMITER])(.*?)(\N)|(\{)(.*?)(}))` at `i`, the delimiter any
    /// character that `excluded` rejects.
    fn code_body(&self, i: usize, excluded: fn(char) -> bool) -> Option<Body> {
        let (c, text) = self.char_at(i)?;
        if i < self.lim() && !excluded(c) {
            let delimiter = self.text(i, text);
            let k = memchr::memmem::find(self.text(text, self.lim()), delimiter)?;
            let close = text + k;
            return Some(Body::Delimited {
                open: i,
                text,
                close,
                end: close + delimiter.len(),
            });
        }
        if c != '{' || i >= self.lim() {
            return None;
        }
        let close = i + 1 + memchr(b'}', self.text(i + 1, self.lim()))?;
        Some(Body::Braced { open: i, close })
    }

    /// Records `body` as groups `first` to `first + 5`: the first three for
    /// a delimited body, the last three for a braced one.
    fn cap_body(&mut self, body: Body, first: usize) -> usize {
        match body {
            Body::Delimited {
                open,
                text,
                close,
                end,
            } => {
                self.cap(first, open, text);
                self.cap(first + 1, text, close);
                self.cap(first + 2, close, end);
                end
            }
            Body::Braced { open, close } => {
                self.cap(first + 3, open, open + 1);
                self.cap(first + 4, open + 1, close);
                self.cap(first + 5, close, close + 1);
                close + 1
            }
        }
    }

    /// `InlineCodeMacroAt0`: `\addplot`, its options, `gnuplot`, more options
    /// and the `{` of the gnuplot code.
    fn addplot(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"\\addplot")?;
        let plus = if self.at(name) == b'+' && name < self.lim() {
            name + 1
        } else {
            name
        };
        let (first, (second, gnuplot, brace)) = self.greedy_brackets(plus, |lexer, i| {
            let gnuplot = lexer.space_end(i);
            let after = lexer.lit(gnuplot, b"gnuplot")?;
            let (second, brace) = lexer.greedy_brackets(lexer.space_end(after), |lexer, j| {
                let brace = lexer.space_end(j);
                lexer.lit(brace, b"{").map(|_| brace)
            })?;
            Some((second, gnuplot, brace))
        })?;
        self.cap(1, p, name);
        self.cap(2, p, p + 1);
        if let Some((start, end)) = first {
            self.cap(3, start, end);
        }
        self.cap(4, gnuplot, gnuplot + 7);
        if let Some((start, end)) = second {
            self.cap(5, start, end);
        }
        self.cap(6, brace, brace + 1);
        Some(brace + 1)
    }

    /// `(\[[^\[]*])*` at `i` and then `rest`, tried in the regex's order.
    ///
    /// Each group takes its text up to the last `]` before the next `[`.
    /// A group whose last `]` a `[` follows can be followed by another.
    /// When `rest` fails, the last group gives back to its earlier `]`s, from
    /// the right; then the group before it does; then no group is tried.
    /// The first value is the last group's span.
    fn greedy_brackets<T>(
        &self,
        i: usize,
        rest: impl Fn(&Self, usize) -> Option<T>,
    ) -> Option<(Option<(usize, usize)>, T)> {
        let lim = self.lim();
        let next_open = |from: usize| memchr(b'[', self.text(from, lim)).map_or(lim, |k| from + k);
        let mut last = None;
        let mut s = i;
        while self.at(s) == b'[' && s < lim {
            let q = next_open(s + 1);
            if memchr(b']', self.text(s + 1, q)).is_none() {
                break;
            }
            last = Some(s);
            if q < lim && self.at(q - 1) == b']' {
                s = q;
            } else {
                break;
            }
        }
        if let Some(mut s) = last {
            let mut q = next_open(s + 1);
            loop {
                let mut end = q;
                while let Some(k) = memrchr(b']', self.text(s + 1, end)) {
                    let close = s + 1 + k;
                    if let Some(found) = rest(self, close + 1) {
                        return Some((Some((s, close + 1)), found));
                    }
                    end = close;
                }
                if s <= i {
                    break;
                }
                q = s;
                let Some(k) = memrchr(b'[', self.text(i, s)) else {
                    break;
                };
                s = i + k;
            }
        }
        rest(self, i).map(|found| (None, found))
    }
}

/// `(?:[A-Za-z]*code|lstlisting|minted|pyglist)\*?`.
fn any_code_env(name: &[u8]) -> bool {
    let stem = name.strip_suffix(b"*").unwrap_or(name);
    engine::LISTING.contains(&stem)
        || stem
            .strip_suffix(b"code")
            .is_some_and(|head| head.iter().all(u8::is_ascii_alphabetic))
}

/// `\p{Alphabetic}+\*?`.
fn alphabetic_env(name: &[u8]) -> bool {
    let stem = name.strip_suffix(b"*").unwrap_or(name);
    std::str::from_utf8(stem).is_ok_and(|stem| !stem.is_empty() && stem.chars().all(is_alpha))
}

/// The math environments of `AllBalancedEnvAt0`, with an optional `!` or
/// `*`.
fn math_env(name: &[u8]) -> bool {
    let stem = name
        .strip_suffix(b"!")
        .or_else(|| name.strip_suffix(b"*"))
        .unwrap_or(name);
    if MATH_ENVS.contains(&stem) {
        return true;
    }
    let plain = stem.strip_prefix(b"+").unwrap_or(stem);
    let matrix = plain
        .strip_suffix(b"matrix")
        .is_some_and(|head| head.iter().all(u8::is_ascii_alphabetic));
    let nice = |name: &[u8]| name == b"NiceMatrix" || name == b"NiceArray";
    let sized = stem
        .split_first()
        .is_some_and(|(first, rest)| b"BVbpv".contains(first) && nice(rest));
    matrix || nice(stem) || sized
}

/// The citation commands: an optional prefix, `cite` or `Cite`, an optional
/// `al`, an optional suffix and any of `A`, `N` and `P`.
fn cite_name(word: &[u8]) -> bool {
    CITE_PREFIXES.iter().any(|prefix| {
        let Some(rest) = word.strip_prefix(*prefix) else {
            return false;
        };
        let Some(rest) = rest
            .strip_prefix(b"cite")
            .or_else(|| rest.strip_prefix(b"Cite"))
        else {
            return false;
        };
        [Some(rest), rest.strip_prefix(b"al")]
            .into_iter()
            .flatten()
            .any(|rest| {
                CITE_SUFFIXES.iter().any(|suffix| {
                    rest.strip_prefix(*suffix)
                        .is_some_and(|tail| tail.iter().all(|b| b"ANP".contains(b)))
                })
            })
    })
}

/// The commands of `InlineCodeMacroAt3`: `\py` and its kin, with an optional
/// `c` or `v`, and `\pyq`, `\pycq` and `\pyif`.
fn python_inline(word: &[u8]) -> bool {
    const STEMS: &[&[u8]] = &[
        b"py",
        b"pycon",
        b"pylab",
        b"pylabcon",
        b"sympy",
        b"sympycon",
    ];
    let stem = word.strip_suffix(b"c").or_else(|| word.strip_suffix(b"v"));
    STEMS.contains(&word)
        || stem.is_some_and(|stem| STEMS.contains(&stem))
        || [&b"pyq"[..], b"pycq", b"pyif"].contains(&word)
}

/// `[A-Za-{]`: what an inline code delimiter cannot be.
fn letter_or_brace(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '{'
}
