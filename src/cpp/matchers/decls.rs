//! Declarations: classes, structs, unions and enums, constructors and
//! destructors, function pointers' ends and `#include`.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::super::limits::MAX_ENDS;
use super::gaps::{Gap, GapForm};
use crate::cpp::matchers::{NEED_BRACKETS, NEED_PAREN};
use crate::rules::Chars;

/// The specifiers a constructor or destructor may start with.
const SPECIFIERS: &[&[u8]] = &[
    b"constexpr",
    b"consteval",
    b"explicit",
    b"mutable",
    b"virtual",
    b"inline",
    b"friend",
];

/// The calling conventions of the Microsoft compilers.
pub(super) const CALLING_CONVENTIONS: &[&[u8]] = &[
    b"__cdecl",
    b"__clrcall",
    b"__stdcall",
    b"__fastcall",
    b"__thiscall",
    b"__vectorcall",
];

/// The last iteration of a qualifier loop: where its gap starts, the gap, and
/// where its word ends.
type Qualifier = (usize, Gap, usize);

impl Lexer<'_> {
    /// The ends of an attribute at `p`, in the order the expression tries
    /// them: `\[\[.*?]]`, `__attribute(?:__)?\s*\(\s*\(.*?\)\s*\)`,
    /// `__declspec\(.*?\)` and `alignas\(.*?\)`, each lazy and not followed by
    /// `)`.
    pub(super) fn attribute_ends(&self, p: usize) -> Few<usize, { 2 * MAX_ENDS }> {
        let mut out = Few::new();
        let mut keep = |lexer: &Self, end: usize| {
            if lexer.at(end) != b')' || end >= lexer.lim() {
                out.push(end);
            }
        };
        if let Some(open) = self.lit(p, b"[[") {
            for j in self.occurrences(NEED_BRACKETS, open) {
                keep(self, j + 2);
            }
        }
        if let Some(word) = self.lit(p, b"__attribute") {
            let mut starts: Few<usize, 2> = Few::new();
            if let Some(w) = self.lit(word, b"__") {
                starts.push(w);
            }
            starts.push(word);
            for start in starts {
                let a = self.skip_space(start);
                if self.at(a) != b'(' || a >= self.lim() {
                    continue;
                }
                let b = self.skip_space(a + 1);
                if self.at(b) != b'(' || b >= self.lim() {
                    continue;
                }
                for k in self.occurrences(NEED_PAREN, b + 1) {
                    let q = self.skip_space(k + 1);
                    if self.at(q) == b')' && q < self.lim() {
                        keep(self, q + 1);
                    }
                }
            }
        }
        for open in [&b"__declspec("[..], b"alignas("] {
            if let Some(start) = self.lit(p, open) {
                for k in self.occurrences(NEED_PAREN, start) {
                    keep(self, k + 1);
                }
            }
        }
        out
    }

    /// The attribute ends at `p`, each as `Some`, then `None` for the form
    /// without an attribute.
    pub(super) fn attribute_options(&self, p: usize) -> Few<Option<usize>, { 2 * MAX_ENDS + 1 }> {
        let mut out = Few::new();
        for end in self.attribute_ends(p) {
            out.push(Some(end));
        }
        out.push(None);
        out
    }

    /// `((?<!\w)KW(?!\w))GAP(?:(?=\{)|(?:(ATTRIBUTE
    /// GAP)?((?:(?<!\w)ID(?!\w)GAP)*+)?(?:GAP(:(?!:)))?))` for the keywords
    /// `class`, `union` and `struct` after `typedef`.
    ///
    /// The possessive run of names and gaps.
    /// Each gap records its groups.
    /// A group that a later gap leaves out keeps an earlier gap's text, as in
    /// Oniguruma.
    pub(super) fn typedef_tag(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let word = self.word_lit(p, keyword)?;
        let first = self.gaps(word).next()?;
        self.cap(1, p, word);
        self.put_gap(2, GapForm::Parts, word, &first);
        let mut pos = first.end;
        if self.at(pos) == b'{' && pos < self.lim() {
            return Some(pos);
        }
        if let Some(&attr) = self.attribute_ends(pos).first() {
            let gap = self.gaps(attr).next()?;
            self.cap(6, pos, attr);
            self.put_gap(7, GapForm::Parts, attr, &gap);
            pos = gap.end;
        }
        let names = pos;
        while let Some(end) = self.word_ident(pos) {
            let gap = self.gaps(end).next()?;
            self.put_gap(12, GapForm::Parts, end, &gap);
            pos = gap.end;
        }
        if pos > names {
            self.cap(11, names, pos);
        }
        for gap in self.gaps(pos) {
            if self.at(gap.end) == b':' && gap.end < self.lim() && self.at(gap.end + 1) != b':' {
                self.put_gap(16, GapForm::Parts, pos, &gap);
                self.cap(20, gap.end, gap.end + 1);
                return Some(gap.end + 1);
            }
        }
        Some(pos)
    }

    /// `(?!override\W|override\$|final\W|final\$)` at `p`.
    fn not_contextual(&self, p: usize) -> bool {
        for word in [&b"override"[..], b"final"] {
            if let Some(end) = self.lit(p, word)
                && (self.ch(end).is_some_and(|(c, _)| !super::chars::is_word(c))
                    || self.at(end) == b'$')
            {
                return false;
            }
        }
        true
    }

    /// `(?=\S)(?![:A-Za-{])` at `p`.
    fn ahead_declarator_end(&self, p: usize) -> bool {
        let Some((c, _)) = self.ch(p) else {
            return false;
        };
        !super::chars::is_space(c) && !matches!(c, ':' | 'A'..='Z' | 'a'..='{')
    }

    /// `((?<!\w)KW(?!\w))GAP((?<!\w)ID(?!\w))(GAP?(?:[\&*]GAP)*[\&*])?GAP\b(?!
    /// override\W|…)((?<!\w)ID(?!\w))GAP(?=\S)(?![:A-Za-{])` for `struct`,
    /// `union`, `enum` and `class` in a declaration.
    pub(super) fn standard_declare(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let word = self.word_lit(p, keyword)?;
        for first in self.gaps(word) {
            if self.word_before(first.end) {
                continue;
            }
            for ty in self.ident_ends(first.end) {
                if self.word_at(ty).is_some() {
                    continue;
                }
                let found = self
                    .find_reference_chain(ty, |lexer, chain| {
                        lexer
                            .declared_name(chain.0)
                            .map(|name| (chain.0, Some(chain), name))
                    })
                    .or_else(|| self.declared_name(ty).map(|name| (ty, None, name)));
                let Some((after, chain, (third, name, fourth))) = found else {
                    continue;
                };
                self.cap(1, p, word);
                self.put_gap(2, GapForm::Plain, word, &first);
                self.cap(4, first.end, ty);
                if let Some((end, f, l)) = chain {
                    self.cap(5, ty, end);
                    if let Some(gap) = f {
                        self.put_gap(6, GapForm::Plain, ty, &gap);
                    }
                    if let Some((from, gap)) = l {
                        self.put_gap(8, GapForm::Plain, from, &gap);
                    }
                }
                self.put_gap(10, GapForm::Plain, after, &third);
                self.cap(12, third.end, name);
                self.put_gap(13, GapForm::Plain, name, &fourth);
                return Some(fourth.end);
            }
        }
        None
    }

    /// `GAP\b(?!override\W|…)((?<!\w)ID(?!\w))GAP(?=\S)(?![:A-Za-{])` from
    /// `after`: the declared name of [`Lexer::standard_declare`], as the gap
    /// before it, its end and the gap after it.
    fn declared_name(&self, after: usize) -> Option<(Gap, usize, Gap)> {
        self.gaps(after).find_map(|third| {
            let at = third.end;
            if !self.boundary(at) || !self.not_contextual(at) || self.word_before(at) {
                return None;
            }
            self.ident_ends(at)
                .filter(|&name| self.word_at(name).is_none())
                .find_map(|name| {
                    let fourth = self
                        .gaps(name)
                        .find(|gap| self.ahead_declarator_end(gap.end))?;
                    Some((third, name, fourth))
                })
        })
    }

    /// The calling conventions at `p`, then none: where each ends.
    pub(super) fn convention_ends(&self, p: usize) -> Few<usize, 7> {
        let mut out = Few::new();
        for word in CALLING_CONVENTIONS {
            if let Some(e) = self.lit(p, word) {
                out.push(e);
            }
        }
        out.push(p);
        out
    }

    /// The calling convention at `p`, if any: `(?:__(?:cdec|…)l)?`.
    fn calling_convention(&self, p: usize) -> usize {
        CALLING_CONVENTIONS
            .iter()
            .find_map(|word| self.lit(p, word))
            .unwrap_or(p)
    }

    /// `((?<!\w)ID(?!\w)(?=\())` at `p`, with `~` before when `tilde`.
    fn ident_ahead_paren(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.ident_ends(p)
            .find(|&end| self.word_at(end).is_none() && self.at(end) == b'(' && end < self.lim())
    }

    /// `(?:(?:KW)GAP)*` from `pos`, every way, the longest first.
    /// It finds the first way for which `rest` matches what follows, given the
    /// loop's end and its last keyword's end and gap.
    /// It returns what `rest` found.
    /// `last` is the iteration before `pos`.
    fn find_specifier_loop<R>(
        &self,
        pos: usize,
        last: Option<(usize, Gap)>,
        rest: &mut impl FnMut(&Self, usize, Option<(usize, Gap)>) -> Option<R>,
    ) -> Option<R> {
        let Some(_depth) = self.deeper() else {
            return rest(self, pos, last);
        };
        for word in SPECIFIERS {
            if let Some(end) = self.lit(pos, word) {
                for gap in self.gaps(end) {
                    if let Some(found) = self.find_specifier_loop(gap.end, Some((end, gap)), rest) {
                        return Some(found);
                    }
                }
            }
        }
        rest(self, pos, last)
    }

    /// `^GAP((?:(?:KW)GAP)*)((?:__…l)?)GAP((?<!\w)ID(?!\w)(?=\())`.
    pub(super) fn constructor_inline(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for first in self.gaps(0) {
            let found = self.find_specifier_loop(first.end, None, &mut |lexer, loop_end, last| {
                for convention in [lexer.calling_convention(loop_end), loop_end] {
                    for gap in lexer.gaps(convention) {
                        if let Some(end) = lexer.ident_ahead_paren(gap.end) {
                            return Some((loop_end, last, convention, gap, end));
                        }
                    }
                }
                None
            });
            let Some((loop_end, last, convention, gap, end)) = found else {
                continue;
            };
            self.put_gap(1, GapForm::Parts, 0, &first);
            self.cap(5, first.end, loop_end);
            if let Some((from, g)) = last {
                self.put_gap(6, GapForm::Parts, from, &g);
            }
            self.cap(10, loop_end, convention);
            self.put_gap(11, GapForm::Parts, convention, &gap);
            self.cap(15, gap.end, end);
            return Some(end);
        }
        None
    }

    /// `^GAP((?:__…l)?)GAP((?:(?:KW)GAP)*)(~(?<!\w)ID(?!\w)(?=\())`.
    pub(super) fn destructor_inline(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for first in self.gaps(0) {
            for convention in [self.calling_convention(first.end), first.end] {
                for second in self.gaps(convention) {
                    let found =
                        self.find_specifier_loop(second.end, None, &mut |lexer, loop_end, last| {
                            if lexer.at(loop_end) != b'~' || loop_end >= lexer.lim() {
                                return None;
                            }
                            let end = lexer.ident_ahead_paren(loop_end + 1)?;
                            Some((loop_end, last, end))
                        });
                    if let Some((loop_end, last, end)) = found {
                        self.put_gap(1, GapForm::Parts, 0, &first);
                        self.cap(5, first.end, convention);
                        self.put_gap(6, GapForm::Parts, convention, &second);
                        self.cap(10, second.end, loop_end);
                        if let Some((from, g)) = last {
                            self.put_gap(11, GapForm::Parts, from, &g);
                        }
                        self.cap(15, loop_end, end);
                        return Some(end);
                    }
                }
            }
        }
        None
    }

    /// `(?:GAP((?<!\w)(?:override|…)(?!\w)))+(?=\s*[\n\r;={])` from
    /// `pos`, every way, the longest first.
    /// It finds the first way for which `rest` matches what follows, given the
    /// loop's end and its last iteration's gap and word.
    /// It returns what `rest` found.
    /// `count` iterations come before `pos`, the last of them `last`.
    fn find_qualifier_loop<R>(
        &self,
        pos: usize,
        count: usize,
        last: Option<Qualifier>,
        rest: &mut impl FnMut(&Self, usize, Qualifier) -> Option<R>,
    ) -> Option<R> {
        let _depth = self.deeper()?;
        let words: &[&[u8]] = &[b"override", b"volatile", b"noexcept", b"final", b"const"];
        for gap in self.gaps(pos) {
            if let Some(end) = self.word_in(gap.end, words)
                && let Some(found) =
                    self.find_qualifier_loop(end, count + 1, Some((pos, gap, end)), rest)
            {
                return Some(found);
            }
        }
        let last = last.filter(|_| count > 0 && self.ahead_qualifier_end_pub(pos))?;
        rest(self, pos, last)
    }

    /// `(\))GAP(?:(?:GAP(KW))+(?=\s*[\n\r;={])GAP)?(?=[\n),;=>{])(?!\()`.
    pub(super) fn function_pointer_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let ahead = |lexer: &Self, at: usize| {
            let b = lexer.at(at);
            matches!(b, b'\n' | b')' | b',' | b';' | b'=' | b'>' | b'{')
                && (at < lexer.lim() || lexer.nl)
                && lexer.at(at) != b'('
        };
        for first in self.gaps(p + 1) {
            let found =
                self.find_qualifier_loop(first.end, 0, None, &mut |lexer, loop_end, last| {
                    let gap = lexer.gaps(loop_end).find(|gap| ahead(lexer, gap.end))?;
                    Some((loop_end, last, gap))
                });
            if let Some((loop_end, last, gap)) = found {
                self.cap(1, p, p + 1);
                self.put_gap(2, GapForm::Parts, p + 1, &first);
                self.put_gap(6, GapForm::Parts, last.0, &last.1);
                self.cap(10, last.1.end, last.2);
                self.put_gap(11, GapForm::Parts, loop_end, &gap);
                return Some(gap.end);
            }
            if ahead(self, first.end) {
                self.cap(1, p, p + 1);
                self.put_gap(2, GapForm::Parts, p + 1, &first);
                return Some(first.end);
            }
        }
        None
    }
}
