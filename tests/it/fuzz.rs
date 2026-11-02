//! Every lexer finishes arbitrary lines within the default limits.
//!
//! Lines are drawn from a seeded generator over the characters lexers
//! branch on: brackets, quotes, comment and string openers, operators,
//! uppercase and lowercase letters, digits, blanks and non-ASCII text. A
//! lexer that stops advancing runs into the work limit, which the test
//! reports, so a hang shows up as a failure rather than a stuck run.

use nishiki::{Highlighter, Language, State};

/// The pieces lines are made of; several bytes long where a lexer reads
/// them as one construct.
const PIECES: &[&str] = &[
    "@media ",
    "@import url(",
    "@charset \"",
    "a:not(",
    "::before",
    "[a|=",
    "{ color: ",
    "#fff",
    "1.5e+3px",
    "U+4??",
    "!important",
    "/*",
    "*/",
    "\\0000e9",
    "---",
    "...",
    "? ",
    ": ",
    "- ",
    "|2",
    ">-",
    "&a ",
    "*a",
    "!!str ",
    "%YAML 1.2",
    "R",
    "a",
    "<a",
    "</a>",
    "/>",
    "&amp;",
    "x=<",
    "<>",
    "a:b",
    "_",
    "Z",
    "9",
    "0x",
    "1e",
    ".",
    "..",
    "::",
    "::<",
    "<",
    ">",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    "\"",
    "'",
    "r#\"",
    "\"#",
    "b'",
    "/*",
    "*/",
    "//",
    "///",
    "#[",
    "!",
    "=",
    "&",
    "|",
    "$",
    "\\",
    "`",
    "${",
    "#",
    "-",
    "*",
    ",",
    ";",
    " ",
    "\t",
    "日",
    "é",
    "🦀",
    "fn",
    "let",
    "self",
    "Self",
    "impl",
    "where",
    "macro_rules!",
    "<<",
    "EOF",
    "```",
    "~~~",
    "$(",
    "${",
    "define ",
    "endef",
    "ifeq ",
    "else",
    "endif",
    "\t@",
    ".PHONY",
    "$(shell ",
    ":=",
    "@@",
    "---",
    "+++",
    "***",
    "====",
    " - ",
    " ====",
    "diff --git a/",
    "index ",
    "Index: ",
    "Only in ",
    "\u{a0}",
    "\u{663}",
    "FROM ",
    " AS ",
    "ONBUILD ",
    "RUN ",
    "ENTRYPOINT",
    "\u{17f}",
    "\u{fb05}",
    "\u{212a}",
    "#if 0",
    "#endif",
    "switch",
    "case",
    "asm",
    "/**",
    "->",
    "%d",
    "--",
    "/*",
    "*/",
    "[",
    "]]",
    "'",
    "N'",
    "@@",
    "select ",
    "create or replace ",
    "drop table ",
    "double ",
    "left outer ",
    "timestamp",
    "count(",
    "a.b",
    "%{",
    "#{",
    "= ",
    "a=",
    "a-",
    "\u{e9}",
    "; ",
    "query ",
    "fragment ",
    " on ",
    "type ",
    "enum ",
    "union ",
    "schema ",
    "directive ",
    "implements ",
    "...",
    "${",
    "\"\"\"",
    "@",
    "!",
];

/// A xorshift generator: deterministic, so a failure reproduces from its
/// seed.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).unwrap_or(1)).unwrap_or(0)
    }
}

fn line(rng: &mut Rng) -> String {
    let len = rng.below(24);
    (0..len).map(|_| PIECES[rng.below(PIECES.len())]).collect()
}

#[test]
fn every_lexer_finishes_arbitrary_lines() {
    for &language in Language::ALL {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for document in 0..200 {
            let mut highlighter = Highlighter::new(language);
            let mut state = State::default();
            let mut tokens = Vec::new();
            for _ in 0..20 {
                let text = line(&mut rng);
                tokens.clear();
                let report = highlighter.line(&text, &mut state, &mut tokens);
                assert!(
                    report.is_complete(),
                    "{} document {document}: {text:?} reached {:?}",
                    language.id(),
                    report.limits().collect::<Vec<_>>()
                );
                assert_eq!(
                    tokens.last().map_or(0, |token| token.end()),
                    text.len(),
                    "{} document {document}: tokens cover {text:?}",
                    language.id()
                );
            }
        }
    }
}
