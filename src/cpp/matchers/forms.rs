//! Declaration forms: `#include`, template parameters, enumerators, lambda
//! introducers, banners and the builtin types.

use super::super::engine::Lexer;
use super::super::few::Few;
use super::chars::is_space;
use super::gaps::{Gap, GapForm};
use crate::cpp::keys::{PRIMITIVE_KEYS, STDINT_KEYS};
use crate::rules::Chars;

/// A way a part of a template parameter matches: where it ends and the
/// groups it records, each with its start and end.
type Caps = (usize, Few<(usize, usize, usize), 7>);

/// What follows a template parameter's first part: where the match ends, the
/// `=` of a default and the `,` after it.
#[derive(Clone, Copy)]
struct ParamTail {
    end: usize,
    equals: Option<usize>,
    comma: Option<usize>,
}

/// The end of [`Lexer::over_qualified`]: the name, the gap after it, the
/// gaps inside and after `[]`, and where the match ends.
#[derive(Clone, Copy)]
struct OverTail {
    name: usize,
    gap: Gap,
    brackets: Option<(Gap, Gap)>,
    end: usize,
}

/// What ends the line of a directive, for [`Lexer::directive_tail`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum TailEnd {
    /// The end of the line or a line comment.
    Line,
    /// Those or a `;`.
    LineOrSemicolon,
}

impl Lexer<'_> {
    /// `(?:\n|$)` and `(?=//)`, and for [`TailEnd::LineOrSemicolon`]
    /// `(?=//|;)`, as the end of a directive's line: where the match ends.
    fn directive_tail(&self, pos: usize, end: TailEnd) -> Option<usize> {
        if self.nl && pos == self.lim() {
            return Some(pos + 1);
        }
        if self.at_eol(pos)
            || self.starts(pos, b"//")
            || (end == TailEnd::LineOrSemicolon && self.at(pos) == b';' && pos < self.lim())
        {
            return Some(pos);
        }
        None
    }

    /// `^GAP((#)\s+{0,1}(include(?:|_next)))\b\s+{0,1}…`.
    pub(super) fn include(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        for first in self.gaps(0) {
            if self.at(first.end) != b'#' || first.end >= self.lim() {
                continue;
            }
            let hash = first.end;
            let name = self.skip_space(hash + 1);
            let Some(word) = self.lit(name, b"include") else {
                continue;
            };
            for end in [Some(word), self.lit(word, b"_next")].into_iter().flatten() {
                if !self.boundary(end) {
                    continue;
                }
                let rest = self.skip_space(end);
                if let Some(done) = self.include_target(p, &first, hash, name, end, rest) {
                    return Some(done);
                }
            }
        }
        None
    }

    /// What follows `#include `: `<…>`, `"…"`, a macro name or nothing.
    /// They are tried in that order, and the head's groups are set once a
    /// target matches.
    fn include_target(
        &mut self,
        _p: usize,
        first: &Gap,
        hash: usize,
        name: usize,
        end: usize,
        rest: usize,
    ) -> Option<usize> {
        let head = |lexer: &mut Self| {
            lexer.put_gap(1, GapForm::Plain, 0, first);
            lexer.cap(3, hash, end);
            lexer.cap(4, hash, hash + 1);
            lexer.cap(5, name, end);
        };
        for (open, limit, group, gap_base) in [(b'<', b'>', 6, 9), (b'"', b'"', 11, 14)] {
            if self.at(rest) != open || rest >= self.lim() {
                continue;
            }
            let mut run_end = rest + 1;
            while run_end < self.lim() && self.s[run_end] != limit {
                run_end += 1;
            }
            if run_end >= self.lim() && self.nl {
                run_end = self.lim() + 1;
            }
            for k in (rest + 1..=run_end).rev() {
                let closed = (k < self.lim() && self.s[k] == limit).then_some(k + 1);
                for after in [closed, Some(k)].into_iter().flatten() {
                    for gap in self.gaps(after) {
                        let Some(tail) = self.directive_tail(gap.end, TailEnd::Line) else {
                            continue;
                        };
                        head(self);
                        self.cap(group, rest, tail);
                        self.cap(group + 1, rest, rest + 1);
                        if after > k {
                            self.cap(group + 2, k, k + 1);
                        }
                        self.put_gap(gap_base, GapForm::Plain, after, &gap);
                        return Some(tail);
                    }
                }
            }
        }
        for gap in self.gaps(rest) {
            let Some(mut stop) = self.ident_ends(gap.end).longest() else {
                continue;
            };
            while self.at(stop) == b'.' && stop < self.lim() {
                let Some(next) = self.ident_ends(stop + 1).longest() else {
                    break;
                };
                stop = next;
            }
            for second in self.gaps(stop) {
                if let Some(tail) = self.directive_tail(second.end, TailEnd::LineOrSemicolon) {
                    head(self);
                    self.cap(16, rest, tail);
                    self.put_gap(17, GapForm::Plain, rest, &gap);
                    self.put_gap(19, GapForm::Plain, stop, &second);
                    return Some(tail);
                }
            }
        }
        for gap in self.gaps(rest) {
            if let Some(tail) = self.directive_tail(gap.end, TailEnd::LineOrSemicolon) {
                head(self);
                self.put_gap(21, GapForm::Plain, rest, &gap);
                return Some(tail);
            }
        }
        None
    }

    /// The longest `ID` at `p`.
    /// This form has no boundaries.
    fn plain_ident(&self, p: usize) -> Option<usize> {
        self.ident_ends(p).longest()
    }

    /// `GAP(?:(?:(?:(ID)|((?:ID\s+)+)(ID))|(ID)\s*(\.\.\.)\s*(ID))|(?<!\
    /// w)(template)…)\s*(?:(=)\s*ID)?(?:(,)|(?=>|$))`.
    ///
    /// The first part matches in this order:
    /// - as a single name;
    /// - as words and then a name (`ID\s+` repeated, then `ID`);
    /// - as a pack (`ID\s*...\s*ID`);
    /// - as a template template parameter.
    pub(super) fn template_parameter(&mut self, p: usize) -> Option<usize> {
        for gap in self.gaps(p) {
            let Some(((_, caps), tail)) = self.template_parameter_head(gap.end) else {
                continue;
            };
            self.put_gap(1, GapForm::Plain, p, &gap);
            for (g, x, y) in caps {
                self.cap(g, x, y);
            }
            if let Some(equals) = tail.equals {
                self.cap(16, equals, equals + 1);
            }
            if let Some(comma) = tail.comma {
                self.cap(17, comma, comma + 1);
            }
            return Some(tail.end);
        }
        None
    }

    /// The first part of a template parameter at `at`, in the order of
    /// [`Lexer::template_parameter`], with the first tail that matches after
    /// it.
    ///
    /// The run of words is followed forward, keeping the last stop that
    /// matches.
    /// That stop is the longest the greedy `(?:ID\s+)+` gives.
    fn template_parameter_head(&self, at: usize) -> Option<(Caps, ParamTail)> {
        let with_tail = |part: Caps| Some((part, self.template_parameter_tail(part.0)?));
        let name = self
            .ident_ends(at)
            .find_map(|end| with_tail((end, [(3, at, end)].into_iter().collect())));
        if name.is_some() {
            return name;
        }
        let mut words = None;
        let mut pos = at;
        while let Some(end) = self.plain_ident(pos) {
            let stop = self.skip_space(end);
            if stop == end {
                break;
            }
            let found = self.ident_ends(stop).find_map(|end| {
                with_tail((end, [(4, at, stop), (5, stop, end)].into_iter().collect()))
            });
            if found.is_some() {
                words = found;
            }
            pos = stop;
        }
        if words.is_some() {
            return words;
        }
        if let Some(name) = self.plain_ident(at) {
            let dots = self.skip_space(name);
            if let Some(after) = self.lit(dots, b"...") {
                let target = self.skip_space(after);
                let pack = self.ident_ends(target).find_map(|end| {
                    let caps = [(6, at, name), (7, dots, after), (8, target, end)];
                    with_tail((end, caps.into_iter().collect()))
                });
                if pack.is_some() {
                    return pack;
                }
            }
        }
        self.template_template(at).and_then(with_tail)
    }

    /// `\s*(?:(=)\s*ID)?(?:(,)|(?=>|$))` at `end`: the default is tried
    /// first, its name longest first, then left out.
    fn template_parameter_tail(&self, end: usize) -> Option<ParamTail> {
        let a = self.skip_space(end);
        let defaults = (self.at(a) == b'=' && a < self.lim()).then(|| {
            self.ident_ends(self.skip_space(a + 1))
                .map(move |stop| (stop, Some(a)))
        });
        defaults
            .into_iter()
            .flatten()
            .chain([(a, None)])
            .find_map(|(stop, equals)| {
                if self.at(stop) == b',' && stop < self.lim() {
                    return Some(ParamTail {
                        end: stop + 1,
                        equals,
                        comma: Some(stop),
                    });
                }
                let ahead = (self.at(stop) == b'>' && stop < self.lim()) || self.at_eol(stop);
                ahead.then_some(ParamTail {
                    end: stop,
                    equals,
                    comma: None,
                })
            })
    }

    /// `(?<!\w)(template)\s*(<)\s*(ID)\s*((?:ID)?)\s*(>)\s*(class|typename)(?:\
    /// s+(ID))?`.
    fn template_template(&self, at: usize) -> Option<Caps> {
        if self.word_before(at) {
            return None;
        }
        let word = self.lit(at, b"template")?;
        let open = self.skip_space(word);
        if self.at(open) != b'<' || open >= self.lim() {
            return None;
        }
        let first_at = self.skip_space(open + 1);
        let first = self.plain_ident(first_at)?;
        let second_at = self.skip_space(first);
        let second = self.plain_ident(second_at).unwrap_or(second_at);
        let close = self.skip_space(second);
        if self.at(close) != b'>' || close >= self.lim() {
            return None;
        }
        let keyword_at = self.skip_space(close + 1);
        let keyword = self.lit_any(keyword_at, &[b"class", b"typename"])?;
        let mut caps: Few<_, 7> = [
            (9, at, word),
            (10, open, open + 1),
            (11, first_at, first),
            (12, second_at, second),
            (13, close, close + 1),
            (14, keyword_at, keyword),
        ]
        .into_iter()
        .collect();
        let name_at = self.skip_space(keyword);
        if name_at > keyword
            && let Some(name) = self.plain_ident(name_at)
        {
            caps.push((15, name_at, name));
            return Some((name, caps));
        }
        Some((keyword, caps))
    }

    /// `\b(KW)GAP((?<!\w)ID(?!\w))GAP(?:(CHAIN)GAP)?((?:(?<!\w)ID(?!\w))?)GAP(?
    /// :\[GAP]GAP)?(?=[\n),])` for `struct`, `enum`, `union` and `class`.
    ///
    /// The reference part and its gap are tried first, then left out.
    pub(super) fn over_qualified(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, keyword)?;
        for first in self.gaps(word) {
            if self.word_before(first.end) {
                continue;
            }
            for ty in self.ident_ends(first.end) {
                if self.word_at(ty).is_some() {
                    continue;
                }
                for second in self.gaps(ty) {
                    let found = self
                        .find_reference_chain(second.end, |lexer, chain| {
                            lexer.gaps(chain.0).find_map(|gap| {
                                let tail = lexer.over_qualified_tail(gap.end)?;
                                Some((Some((chain, gap)), gap.end, tail))
                            })
                        })
                        .or_else(|| {
                            let tail = self.over_qualified_tail(second.end)?;
                            Some((None, second.end, tail))
                        });
                    let Some((chain, after, tail)) = found else {
                        continue;
                    };
                    self.cap(1, p, word);
                    self.put_gap(2, GapForm::Plain, word, &first);
                    self.cap(4, first.end, ty);
                    self.put_gap(5, GapForm::Plain, ty, &second);
                    if let Some(((cend, f, l), gap)) = chain {
                        self.cap(7, second.end, cend);
                        if let Some(g) = f {
                            self.put_gap(8, GapForm::Plain, second.end, &g);
                        }
                        if let Some((from, g)) = l {
                            self.put_gap(10, GapForm::Plain, from, &g);
                        }
                        self.put_gap(12, GapForm::Plain, cend, &gap);
                    }
                    self.cap(14, after, tail.name);
                    self.put_gap(15, GapForm::Plain, tail.name, &tail.gap);
                    if let Some((inner, last)) = tail.brackets {
                        self.put_gap(17, GapForm::Plain, tail.gap.end + 1, &inner);
                        self.put_gap(19, GapForm::Plain, inner.end + 1, &last);
                    }
                    return Some(tail.end);
                }
            }
        }
        None
    }

    /// `((?:(?<!\w)ID(?!\w))?)GAP(?:\[GAP]GAP)?(?=[\n),])` from `after`: the
    /// end of [`Lexer::over_qualified`].
    /// The name is tried longest first, then left out.
    /// The brackets are tried before they are left out.
    fn over_qualified_tail(&self, after: usize) -> Option<OverTail> {
        let idents = (!self.word_before(after)).then(|| self.ident_ends(after));
        let names = idents
            .into_iter()
            .flatten()
            .filter(|&end| self.word_at(end).is_none());
        for name in names.chain([after]) {
            for gap in self.gaps(name) {
                if self.at(gap.end) == b'[' && gap.end < self.lim() {
                    for inner in self.gaps(gap.end + 1) {
                        if self.at(inner.end) != b']' || inner.end >= self.lim() {
                            continue;
                        }
                        if let Some(last) = self
                            .gaps(inner.end + 1)
                            .find(|last| self.list_end(last.end))
                        {
                            let brackets = Some((inner, last));
                            return Some(OverTail {
                                name,
                                gap,
                                brackets,
                                end: last.end,
                            });
                        }
                    }
                }
                if self.list_end(gap.end) {
                    return Some(OverTail {
                        name,
                        gap,
                        brackets: None,
                        end: gap.end,
                    });
                }
            }
        }
        None
    }

    /// `(?=[\n),])` at `end`.
    fn list_end(&self, end: usize) -> bool {
        matches!(self.at(end), b'\n' | b')' | b',') && (end < self.lim() || self.nl)
    }

    /// `\s*+(?<!\w)(?:(LIST)|(LIST)|(pthread_…t)|([A-Z_a-z]\w*_t))(?!\w)`: the
    /// ways a builtin storage type matches at `p`, as a group, its end and the
    /// start after the white space.
    ///
    /// - A listed word that no word character follows is the whole run of word
    ///   characters at `start`. The lists are sorted.
    /// - `[A-Z_a-z]\w*_t`: the run of word characters has to end in `_t`.
    pub(super) fn builtin_types(&self, p: usize) -> Few<(usize, usize, usize), 4> {
        let start = self.skip_space(p);
        let mut out = Few::new();
        if self.word_before(start) {
            return out;
        }
        let run_end = self.word_end(start);
        if run_end == start {
            return out;
        }
        let run = self.text(start, run_end);
        if PRIMITIVE_KEYS.contains(run) {
            out.push((1, start, run_end));
        }
        if STDINT_KEYS.contains(run) {
            out.push((2, start, run_end));
        }
        if let Some(word) = self.lit(start, b"pthread_") {
            for suffix in [
                &b"rwlockattr_"[..],
                b"mutexattr_",
                b"condattr_",
                b"rwlock_",
                b"mutex_",
                b"cond_",
                b"attr_",
                b"once_",
                b"key_",
                b"",
            ] {
                let after = self.lit(word, suffix);
                if let Some(t) = after.and_then(|e| self.lit(e, b"t"))
                    && self.word_at(t).is_none()
                {
                    out.push((3, start, t));
                    break;
                }
            }
        }
        let lead = self.at(start);
        if (lead.is_ascii_alphabetic() || lead == b'_')
            && run_end >= start + 3
            && self.ends_with_at(run_end, b"_t")
        {
            out.push((4, start, run_end));
        }
        out
    }

    /// `\s*+(?<!\w)(?:…)(?!\w)`.
    pub(super) fn inline_builtin_type(&mut self, p: usize) -> Option<usize> {
        let (group, start, end) = *self.builtin_types(p).first()?;
        self.cap(group, start, end);
        Some(end)
    }

    /// `\s*+(?<!\w)(?:…)(?!\w)\s*+(?<!\w)(\()`.
    pub(super) fn builtin_type_call(&mut self, p: usize) -> Option<usize> {
        for (group, start, end) in self.builtin_types(p) {
            let open = self.skip_space(end);
            if self.at(open) == b'(' && open < self.lim() && !self.word_before(open) {
                self.cap(group, start, end);
                self.cap(5, open, open + 1);
                return Some(open + 1);
            }
        }
        None
    }

    /// `(TYPE)*GAP((?<!\w)ID(?!\w))` after a reference chain: the declarator of
    /// a typedef, groups 1 to 14.
    pub(super) fn typedef_declarator(&mut self, p: usize) -> Option<usize> {
        let declarator = |lexer: &Self, from: usize| {
            lexer.gaps(from).find_map(|gap| {
                if lexer.word_before(gap.end) {
                    return None;
                }
                Some((gap, lexer.ident_bounded(gap.end)?))
            })
        };
        let (chain, from, (gap, end)) = self
            .find_reference_chain(p, |lexer, chain| {
                Some((Some(chain), chain.0, declarator(lexer, chain.0)?))
            })
            .or_else(|| Some((None, p, declarator(self, p)?)))?;
        if let Some((cend, f, l)) = chain {
            self.cap(1, p, cend);
            if let Some(g) = f {
                self.put_gap(2, GapForm::Parts, p, &g);
            }
            if let Some((at, g)) = l {
                self.put_gap(6, GapForm::Parts, at, &g);
            }
        }
        self.put_gap(10, GapForm::Parts, from, &gap);
        self.cap(14, gap.end, end);
        Some(end)
    }

    /// `((?<!\w)ID(?!\w))((TEMPLATE)\s*+)?(\()`.
    pub(super) fn call_with_template(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        for name in self.ident_ends(p) {
            if self.word_at(name).is_some() {
                continue;
            }
            if let Some((end, comment)) = self.template_args(name) {
                let after = self.skip_space(end);
                if self.at(after) == b'(' && after < self.lim() {
                    self.cap(1, p, name);
                    self.cap(2, name, after);
                    self.cap(3, name, end);
                    if let Some(parts) = comment {
                        for (i, (a, b)) in parts.iter().enumerate() {
                            self.cap(4 + i, *a, *b);
                        }
                    }
                    self.cap(7, after, after + 1);
                    return Some(after + 1);
                }
            }
            if self.at(name) == b'(' && name < self.lim() {
                self.cap(1, p, name);
                self.cap(7, name, name + 1);
                return Some(name + 1);
            }
        }
        None
    }

    /// `(?<!\[)\[(?!\[)(?:[^]\[]*+\g<3>?)++]`: where a nested bracket group
    /// at `pos` ends.
    fn nested_brackets(&self, pos: usize, depth: usize) -> Option<usize> {
        if depth >= super::super::limits::MAX_CALL_DEPTH
            || self.at(pos) != b'['
            || pos >= self.lim()
        {
            return None;
        }
        if self.prev(pos) == Some(b'[') || self.at(pos + 1) == b'[' {
            return None;
        }
        let mut at = pos + 1;
        loop {
            let before = at;
            while at < self.lim() && !matches!(self.s[at], b']' | b'[') {
                at += 1;
            }
            if self.nl && at == self.lim() {
                at += 1;
            }
            if let Some(end) = self.nested_brackets(at, depth + 1) {
                at = end;
            }
            if at == before || at > self.lim() {
                break;
            }
            if self.at(at) == b']'
                || (self.at(at) == b'[' && self.nested_brackets(at, depth + 1).is_none())
            {
                break;
            }
        }
        (self.at(at) == b']' && at < self.lim()).then_some(at + 1)
    }

    /// The lambda introducer:
    /// `(?:(?<=\S|^)(?<![]"\&)*>\[\w])|(?<=(?:\W|^)return))\s+{0,1}(\[(?!\[|
    /// *+"| *+\d))((?:[^]\[]|(NESTED))*+)(](?!GAP[];=\[]))`.
    ///
    /// The body is a possessive run of characters and nested groups.
    pub(super) fn lambda_introducer(&mut self, p: usize) -> Option<usize> {
        let prev = self.ch_before(p);
        let plain = (p == 0 || prev.is_some_and(|c| !is_space(c)))
            && !prev.is_some_and(|c| {
                matches!(c, ']' | '"' | '&' | ')' | '*' | '>' | '[') || super::chars::is_word(c)
            });
        let after_return = self.ends_with_at(p, b"return") && (p == 6 || !self.word_before(p - 6));
        if !plain && !after_return {
            return None;
        }
        let open = self.skip_space(p);
        if self.at(open) != b'[' || open >= self.lim() || self.at(open + 1) == b'[' {
            return None;
        }
        let spaces = self.skip_space(open + 1);
        let blank_then = |b: u8| spaces <= self.lim() && self.at(spaces) == b;
        if blank_then(b'"') || self.digit_char(spaces).is_some() {
            return None;
        }
        let mut at = open + 1;
        loop {
            let b = self.at(at);
            if (at < self.lim() && !matches!(b, b']' | b'[')) || (self.nl && at == self.lim()) {
                at += 1;
            } else if let Some(end) = self.nested_brackets(at, 0) {
                at = end;
            } else {
                break;
            }
        }
        if self.at(at) != b']' || at >= self.lim() {
            return None;
        }
        let blocked = self.gaps(at + 1).any(|gap| {
            matches!(self.at(gap.end), b']' | b';' | b'=' | b'[') && gap.end < self.lim()
        });
        if blocked {
            return None;
        }
        self.cap(1, open, open + 1);
        self.cap(2, open + 1, at);
        self.cap(4, at, at + 1);
        Some(at + 1)
    }

    /// `((?<!\w)ID(?!\w))\s*(ATTR)?\s*(?:(=)\s*(.+?)\s*)?(?:(?:([,;](?!')|\
    /// n)|(?=}[^']))|(?=/[*/]))`.
    ///
    /// `\s?` takes all the white space it can and gives it back one character
    /// at a time.
    /// The form with `=` and a lazy value is tried first.
    pub(super) fn enumerator(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let tail = |lexer: &Self, q: usize| -> Option<(usize, Option<usize>)> {
            let b = lexer.at(q);
            if (b == b',' || b == b';') && q < lexer.lim() && lexer.at(q + 1) != b'\'' {
                return Some((q + 1, Some(q)));
            }
            if lexer.nl && q == lexer.lim() {
                return Some((q + 1, Some(q)));
            }
            if b == b'}' && q < lexer.lim() && lexer.at(q + 1) != b'\'' {
                return Some((q, None));
            }
            if lexer.starts(q, b"/*") || lexer.starts(q, b"//") {
                return Some((q, None));
            }
            None
        };
        let spaces = |lexer: &Self, at: usize| (at..=lexer.skip_space(at)).rev();
        for name in self.ident_ends(p) {
            if self.word_at(name).is_some() {
                continue;
            }
            for after in spaces(self, name) {
                for attr in self.attribute_options(after) {
                    for from in spaces(self, attr.unwrap_or(after)) {
                        if self.at(from) == b'=' && from < self.lim() {
                            for start in spaces(self, from + 1) {
                                for k in start + 1..=self.lim() {
                                    for q in spaces(self, k) {
                                        let Some((end, terminator)) = tail(self, q) else {
                                            continue;
                                        };
                                        self.cap(1, p, name);
                                        if let Some(a) = attr {
                                            self.cap(2, after, a);
                                        }
                                        self.cap(3, from, from + 1);
                                        self.cap(4, start, k);
                                        if let Some(t) = terminator {
                                            self.cap(5, t, end);
                                        }
                                        return Some(end);
                                    }
                                }
                            }
                        }
                        if let Some((end, terminator)) = tail(self, from) {
                            self.cap(1, p, name);
                            if let Some(a) = attr {
                                self.cap(2, after, a);
                            }
                            if let Some(t) = terminator {
                                self.cap(5, t, end);
                            }
                            return Some(end);
                        }
                    }
                }
            }
        }
        None
    }

    /// The banner of an Emacs file:
    /// `^(?:(\s*((//)\s*([#*/;=C~]++(?![#*/;=C~]))\s*.+\s*\4\s*(?:\n|$)))|(\s*
    /// ((/\*)…\8\s*\*/)))`.
    ///
    /// `\s*.+\s*\4\s*…`: the last occurrence of the run that only white space
    /// follows (or `*/` for the block form).
    pub(super) fn emacs_banner(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let lead = self.skip_space(0).min(self.lim());
        for (marker, groups, closing) in [
            (&b"//"[..], (1, 2, 3, 4), false),
            (b"/*", (5, 6, 7, 8), true),
        ] {
            let Some(after) = self.lit(lead, marker) else {
                continue;
            };
            let run_start = self.skip_space(after);
            let class = |b: u8| matches!(b, b'#' | b'*' | b'/' | b';' | b'=' | b'C' | b'~');
            let mut run_end = run_start;
            while run_end < self.lim() && class(self.s[run_end]) {
                run_end += 1;
            }
            if run_end == run_start {
                continue;
            }
            let line = self.s;
            let text = &line[run_start..run_end];
            let body = self.skip_space(run_end).min(self.lim());
            let mut found = None;
            for j in (body + 1..=self.lim()).rev() {
                let mut at = j;
                while at < self.lim() && is_space(char::from(self.s[at])) {
                    at += 1;
                }
                if !self.starts(at, text) {
                    continue;
                }
                let end = at + text.len();
                let tail = self.skip_space(end);
                if closing {
                    if let Some(close) = self.lit(tail, b"*/") {
                        found = Some(close);
                        break;
                    }
                } else if tail >= self.lim() {
                    found = Some(self.lim() + usize::from(self.nl));
                    break;
                }
            }
            let Some(end) = found else { continue };
            self.cap(groups.0, 0, end);
            self.cap(groups.1, lead, end);
            self.cap(groups.2, lead, after);
            self.cap(groups.3, run_start, run_end);
            return Some(end);
        }
        None
    }
}
