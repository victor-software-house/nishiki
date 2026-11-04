//! The here documents whose name says which language their lines are in,
//! such as `<<~SQL`.
//! The module holds their openers, the `while` line test that keeps them
//! open, and the line that closes them.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The names a here document of each embedded language may end in, in the
/// grammar's order: the alternatives of `HTML`, `HAML`, `XML`, `SQL`,
/// `G(?:RAPHQL|QL)`, `CSS`, `CPP`, `C`, `J(?:S|AVASCRIPT)`, `JQUERY`,
/// `SH(?:|ELL)`, `LUA`, `RUBY`, `YA?ML` and `SLIM`.
pub(in crate::ruby) const EMBEDDED_NAMES: [&[&[u8]]; 15] = [
    &[b"HTML"],
    &[b"HAML"],
    &[b"XML"],
    &[b"SQL"],
    &[b"GRAPHQL", b"GQL"],
    &[b"CSS"],
    &[b"CPP"],
    &[b"C"],
    &[b"JS", b"JAVASCRIPT"],
    &[b"JQUERY"],
    &[b"SH", b"SHELL"],
    &[b"LUA"],
    &[b"RUBY"],
    &[b"YAML", b"YML"],
    &[b"SLIM"],
];

/// The four matchers below are not inlined.
/// The matcher dispatch has an arm for each of them per language, sixty in
/// all.
/// Inlined, they made the dispatch too large to inline into the engine's
/// call, at a cost to every Ruby pattern tried.
/// They run only at `<<` and on a here document's lines.
impl Lexer<'_> {
    /// The end of `(?:[_\w]+_|)NAME` at `p`, where `names` are NAME's
    /// alternatives and what follows is not `\w`.
    ///
    /// NAME is all word characters, and a non-word character follows it.
    /// It therefore ends where the run of word characters at `p` does.
    /// The run before it is empty, or at least two characters that end in
    /// `_`.
    fn embedded_name_end(&self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let end = self.word_end(p);
        let word = self.text(p, end);
        names
            .iter()
            .any(|name| {
                word.strip_suffix(*name).is_some_and(|prefix| {
                    prefix.is_empty() || (prefix.len() >= 2 && prefix.ends_with(b"_"))
                })
            })
            .then_some(end)
    }

    /// The end of ``<<[-~]?(["'`]?)((?:[_\w]+_|)NAME)\b\1`` at `p`.
    fn embedded_opener(&self, p: usize, names: &[&[u8]]) -> Option<usize> {
        let marks = self.lit(p, b"<<")?;
        let quote_at = marks + usize::from(self.byte_in(marks, |b| b == b'-' || b == b'~'));
        let quote = self
            .byte_in(quote_at, |b| matches!(b, b'"' | b'\'' | b'`'))
            .then(|| self.at(quote_at));
        let name = quote_at + usize::from(quote.is_some());
        let end = self.embedded_name_end(name, names)?;
        match quote {
            Some(q) => self.byte_in(end, |b| b == q).then_some(end + 1),
            None => Some(end),
        }
    }

    /// ``(?=(?><<[-~]?(["'`]?)((?:[_\w]+_|)NAME)\b\1))``: where the here
    /// document's outer rule opens, before its opener.
    #[inline(never)]
    pub(in crate::ruby) fn embedded_heredoc_ahead(
        &mut self,
        p: usize,
        names: &[&[u8]],
    ) -> Option<usize> {
        self.embedded_opener(p, names).map(|_| p)
    }

    /// ``(?><<[-~]?(["'`]?)((?:[_\w]+_|)NAME)\b\1)``: the opener.
    #[inline(never)]
    pub(in crate::ruby) fn embedded_heredoc(&mut self, p: usize, names: &[&[u8]]) -> Option<usize> {
        self.embedded_opener(p, names)
    }

    /// Where the name at the start of the line ends, after its indentation.
    fn leading_name(&self, names: &[&[u8]]) -> Option<usize> {
        self.embedded_name_end(self.space_end(0), names)
    }

    /// `^\s*((?:[_\w]+_|)NAME)$\n?`: the line that closes the here document.
    #[inline(never)]
    pub(in crate::ruby) fn embedded_heredoc_end(
        &mut self,
        p: usize,
        names: &[&[u8]],
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.leading_name(names).filter(|&end| self.at_eol(end))?;
        Some(end + usize::from(end == self.lim() && self.nl))
    }

    /// `^(?!\s*((?:[_\w]+_|)NAME)\s*$)`: the here document's lines go on
    /// until one holds only its name.
    #[inline(never)]
    pub(in crate::ruby) fn embedded_heredoc_goes_on(
        &mut self,
        p: usize,
        names: &[&[u8]],
    ) -> Option<usize> {
        let closes = self
            .leading_name(names)
            .is_some_and(|end| self.at_eol(self.space_end(end)));
        (p == 0 && !closes).then_some(p)
    }
}
