//! Expressions: member access, calls, creations, casts, lambdas, tuples and
//! queries.

use super::super::engine::Lexer;
use super::super::limits::{MAX_CALL_NAMES, MAX_NESTING};
use super::chars::Body;
use super::types::{Shape, Ty};
use crate::rules::Chars;

/// The words that may lead an anonymous function's parameter list.
const FUNCTION_MODIFIERS: &[&[u8]] = &[b"async", b"static"];

/// An access operator: `?.`, `.` or `->`.
#[derive(Clone, Copy, Debug)]
struct Access {
    /// The end of the operator and the blanks after it.
    end: usize,
    question: Option<usize>,
    dot: Option<(usize, usize)>,
    arrow: Option<(usize, usize)>,
}

impl Lexer<'_> {
    /// `[^()<>]`: whether the byte at `i` is none of the four brackets.
    fn plain_at(&self, i: usize) -> bool {
        self.avail(i) && !matches!(self.at(i), b'(' | b')' | b'<' | b'>')
    }

    /// The end of the character at `i`, which a `[^...]` class accepts.
    fn step(&self, i: usize) -> usize {
        if i >= self.lim() {
            i + 1
        } else {
            self.char_end(i)
        }
    }

    /// `\([^()<>]*\)` or `<[^()<>]*>` at `i`: the end of one level of
    /// brackets with plain text inside.
    fn flat_group(&self, i: usize, close: u8) -> Option<usize> {
        let mut j = i + 1;
        while self.plain_at(j) {
            j = self.step(j);
        }
        (self.at(j) == close && j < self.lim()).then_some(j + 1)
    }

    /// `\((?:[^()<>]|<[^()<>]*>|\([^()<>]*\))*\)` at `i`: the end of a
    /// parenthesis in the grammar's call type arguments.
    fn call_parenthesis(&self, i: usize) -> Option<usize> {
        let mut j = i + 1;
        loop {
            if !self.avail(j) {
                return None;
            }
            match self.at(j) {
                b')' if j < self.lim() => return Some(j + 1),
                b'(' if j < self.lim() => j = self.flat_group(j, b')')?,
                b'<' if j < self.lim() => j = self.flat_group(j, b'>')?,
                b'>' if j < self.lim() => return None,
                _ => j = self.step(j),
            }
        }
    }

    /// `<(?<type_args>[^()<>]|\(...\)|<\g<type_args>*>)*>` at `i`: the end of
    /// the angle brackets of a call's type arguments.
    fn call_type_arguments(&self, i: usize, depth: u32) -> Option<usize> {
        if self.at(i) != b'<' || i >= self.lim() || depth >= MAX_NESTING {
            return None;
        }
        let mut j = i + 1;
        loop {
            if !self.avail(j) {
                self.spend(j - i);
                return None;
            }
            match self.at(j) {
                b'>' if j < self.lim() => return Some(j + 1),
                b')' if j < self.lim() => return None,
                b'<' if j < self.lim() => j = self.call_type_arguments(j, depth + 1)?,
                b'(' if j < self.lim() => j = self.call_parenthesis(j)?,
                _ => j = self.step(j),
            }
        }
    }

    /// `(<(?<type_args>...)*>\s*)?` after a name that ends at `w`: the end
    /// of the arguments and the blanks after them.
    fn optional_call_arguments(&self, w: usize) -> Option<(usize, usize)> {
        let close = self.call_type_arguments(w, 0)?;
        Some((close, self.blanks(close)))
    }

    /// `(?:(?:(\?)\s*)?(\.)\s*|(->)\s*)` at `p`: the access operator that
    /// starts there, without recording its groups.
    fn access_prefix(&self, p: usize) -> Option<Access> {
        if let Some(arrow) = self.lit(p, b"->") {
            return Some(Access {
                end: self.blanks(arrow),
                question: None,
                dot: None,
                arrow: Some((p, arrow)),
            });
        }
        let (dot_at, question) = if self.at(p) == b'?' && p < self.lim() {
            (self.blanks(p + 1), Some(p))
        } else {
            (p, None)
        };
        let dot = self.lit(dot_at, b".")?;
        Some(Access {
            end: self.blanks(dot),
            question,
            dot: Some((dot_at, dot)),
            arrow: None,
        })
    }

    /// Records the groups of an access operator: 1 the question mark, 2 the
    /// dot and 3 the arrow.
    fn record_access(&mut self, access: &Access) {
        if let Some(at) = access.question {
            self.cap(1, at, at + 1);
        }
        if let Some((from, to)) = access.dot {
            self.cap(2, from, to);
        }
        if let Some((from, to)) = access.arrow {
            self.cap(3, from, to);
        }
    }

    /// `(?:(?:(\?)\s*)?(\.)\s*|(->)\s*)?(@?ident)\s*(<...>\s*)?(?=\()`
    pub(in crate::csharp) fn invocation(&mut self, p: usize) -> Option<usize> {
        let access = self.access_prefix(p);
        let starts = [access.map(|a| (Some(a), a.end)), Some((None, p))];
        for (access, start) in starts.into_iter().flatten() {
            let Some(name_end) = self.ident(start) else {
                continue;
            };
            let w = self.blanks(name_end);
            let mut found = None;
            if let Some((_, end)) = self.optional_call_arguments(w)
                && self.at(end) == b'('
                && end < self.lim()
            {
                found = Some((Some((w, end)), end));
            }
            if found.is_none() && self.at(w) == b'(' && w < self.lim() {
                found = Some((None, w));
            }
            let Some((arguments, end)) = found else {
                continue;
            };
            if let Some(access) = access {
                self.record_access(&access);
            }
            self.cap(4, start, name_end);
            if let Some((from, to)) = arguments {
                self.cap(5, from, to);
            }
            return Some(end);
        }
        None
    }

    /// The shared head of a base-class constructor call: dotted names, each
    /// `(@?ident)\s*(\.)`, then the last name and its type arguments.
    ///
    /// The ends of the iterations of `(?:(@?ident)\s*(\.))*` are kept, longest
    /// last.
    /// A shorter run can then be tried when the rest fails.
    /// Groups 1 and 2 hold the last iteration kept, as in Oniguruma.
    pub(in crate::csharp) fn call_head(&mut self, p: usize) -> Option<usize> {
        let mut ends = [p; MAX_CALL_NAMES + 1];
        let mut count = 0;
        let mut at = p;
        while count < MAX_CALL_NAMES {
            let Some(name_end) = self.ident(at) else {
                break;
            };
            let dot = self.blanks(name_end);
            if self.at(dot) != b'.' || dot >= self.lim() {
                break;
            }
            at = dot + 1;
            count += 1;
            ends[count] = at;
        }
        for used in (0..=count).rev() {
            let start = ends[used];
            let Some(name_end) = self.ident(start) else {
                continue;
            };
            let w = self.blanks(name_end);
            let mut found = None;
            if let Some((_, end)) = self.optional_call_arguments(w)
                && self.at(end) == b'('
                && end < self.lim()
            {
                found = Some((Some(w), end));
            }
            if found.is_none() && self.at(w) == b'(' && w < self.lim() {
                found = Some((None, w));
            }
            let Some((args, end)) = found else {
                continue;
            };
            if used > 0 {
                let from = ends[used - 1];
                let prev_end = self.ident(from).unwrap_or(from);
                self.cap(1, from, prev_end);
                let dot = self.blanks(prev_end);
                self.cap(2, dot, dot + 1);
            }
            self.cap(3, start, name_end);
            if let Some(w) = args {
                self.cap(4, w, end);
            }
            return Some(end);
        }
        None
    }

    /// The characters `(?![(_[:alnum:]]|(\?)?\[|<)` rules out after a member
    /// name.
    fn after_member_name(&self, q: usize) -> bool {
        let b = self.at(q);
        if !self.avail(q) {
            return true;
        }
        if b == b'(' || b == b'_' || b == b'[' || b == b'<' {
            return false;
        }
        if b == b'?' && self.at(q + 1) == b'[' && q + 1 < self.lim() {
            return false;
        }
        self.alnum_run(q) <= q
    }

    /// `(?:(?:(\?)\s*)?(\.)\s*|(->)\s*)(@?ident)\s*(?![(_[:alnum:]]|(\?)?\[|<)`
    ///
    /// When the lookahead fails after the blanks, the match gives back the last
    /// blank, which then follows the name.
    pub(in crate::csharp) fn member_access(&mut self, p: usize) -> Option<usize> {
        let access = self.access_prefix(p)?;
        let name_start = access.end;
        let name_end = self.ident(name_start)?;
        let w = self.blanks(name_end);
        let end = if self.after_member_name(w) {
            w
        } else if w > name_end {
            w - self.char_before(w).map_or(1, char::len_utf8)
        } else {
            return None;
        };
        self.record_access(&access);
        self.cap(4, name_start, name_end);
        Some(end)
    }

    /// `(\.)?\s*(@?ident)(?<type_params>\s*<([^<>]|\g<type_params>)+>\s*)(?=(\
    /// s*\?)?\s*\.\s*@?ident)`
    pub(in crate::csharp) fn member_with_type_arguments(&mut self, p: usize) -> Option<usize> {
        let dot = self.lit(p, b".");
        let starts = [dot.map(|after| (Some(p), after)), Some((None, p))];
        for (dot_at, after) in starts.into_iter().flatten() {
            let name_start = self.blanks(after);
            let Some(name_end) = self.ident(name_start) else {
                continue;
            };
            let open = self.blanks(name_end);
            let Some(close) = self.balanced(open, b'<', b'>', Body::NonEmpty) else {
                continue;
            };
            let end = self.blanks(close);
            let mut at = end;
            if self.at(at) == b'?' && at < self.lim() {
                let after_question = self.blanks(at + 1);
                if self.lit(after_question, b".").is_some() {
                    at = after_question;
                }
            }
            let dot2 = self.lit(self.blanks(at), b".");
            let found = dot2.is_some_and(|d| self.ident(self.blanks(d)).is_some());
            if !found {
                continue;
            }
            if let Some(d) = dot_at {
                self.cap(1, d, d + 1);
            }
            self.cap(2, name_start, name_end);
            self.cap(3, name_end, end);
            return Some(end);
        }
        None
    }

    /// `(@?ident)(?=\s*(?:(?:\?\s*)?\.|->)\s*@?ident)`
    pub(in crate::csharp) fn member_object(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p)?;
        let q = self.blanks(name_end);
        let mut after_op = None;
        if self.at(q) == b'?' && q < self.lim() {
            let r = self.blanks(q + 1);
            after_op = self.lit(r, b".");
        }
        if after_op.is_none() {
            after_op = self.lit(q, b".").or_else(|| self.lit(q, b"->"));
        }
        let next = self.blanks(after_op?);
        self.ident(next)?;
        self.cap(1, p, name_end);
        Some(name_end)
    }

    /// `(?:access)?(?:(@?ident)\s*)?(?:(\?)\s*)?(?=\[)`: an element access,
    /// which may be only the bracket's position.
    ///
    /// Each optional part is tried present first and absent after, as Oniguruma
    /// backtracks.
    pub(in crate::csharp) fn element_access(&mut self, p: usize) -> Option<usize> {
        let access = self.access_prefix(p);
        let starts = [access.map(|a| (Some(a), a.end)), Some((None, p))];
        for (access, start) in starts.into_iter().flatten() {
            let name = self
                .ident(start)
                .map(|name_end| (start, name_end, self.blanks(name_end)));
            for name in [name, None] {
                let after_name = name.map_or(start, |(_, _, end)| end);
                let question = (self.at(after_name) == b'?' && after_name < self.lim())
                    .then(|| (after_name, self.blanks(after_name + 1)));
                for question in [question, None] {
                    let at = question.map_or(after_name, |(_, end)| end);
                    if self.at(at) != b'[' || at >= self.lim() {
                        continue;
                    }
                    if let Some(access) = access {
                        self.record_access(&access);
                    }
                    if let Some((from, to, _)) = name {
                        self.cap(4, from, to);
                    }
                    if let Some((from, _)) = question {
                        self.cap(5, from, from + 1);
                    }
                    return Some(at);
                }
            }
        }
        None
    }

    /// `(?<=])(?!\s*\[)`
    pub(in crate::csharp) fn element_access_end(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b']') {
            return None;
        }
        let after = self.blanks(p);
        (!(self.at(after) == b'[' && after < self.lim())).then_some(p)
    }

    /// `(@?ident)\s*(:)`
    pub(in crate::csharp) fn named_argument(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p)?;
        let colon = self.blanks(name_end);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        self.cap(1, p, name_end);
        self.cap(2, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(@?ident)\s*(?==)`
    pub(in crate::csharp) fn attribute_named_argument(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p)?;
        let equals = self.blanks(name_end);
        if self.at(equals) != b'=' || equals >= self.lim() {
            return None;
        }
        self.cap(1, p, name_end);
        Some(equals)
    }

    /// `(@?ident)\s*(?=:)`
    pub(in crate::csharp) fn tuple_element_name(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p)?;
        let colon = self.blanks(name_end);
        if self.at(colon) != b':' || colon >= self.lim() {
            return None;
        }
        self.cap(1, p, name_end);
        Some(colon)
    }

    /// `(\()(?=.*[,:])`
    pub(in crate::csharp) fn tuple_literal(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"(")?;
        if self.last_comma_or_colon()? < open {
            return None;
        }
        self.cap(1, p, open);
        Some(open)
    }

    /// `(\()\s*T\s*(\))(?=\s*-*!*@?[(_[:alnum:]])`
    pub(in crate::csharp) fn cast_expression(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"(")?;
        let start = self.blanks(open);
        self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
            let close = lx.blanks(ty.end);
            let end = lx.lit(close, b")")?;
            let mut at = lx.blanks(end);
            at = lx.skip(at, |b| b == b'-');
            at = lx.skip(at, |b| b == b'!');
            if lx.at(at) == b'@' && at < lx.lim() {
                at += 1;
            }
            let class = lx.ahead_in(at, b"(_") || lx.alnum_run(at) > at;
            if !class {
                return None;
            }
            lx.cap(1, p, open);
            lx.cap(2, ty.start, ty.end);
            lx.cap(7, close, end);
            Some(end)
        })
    }

    /// `(new)(?:\s+T)?\s*(?=\()`
    pub(in crate::csharp) fn object_creation_with_arguments(&mut self, p: usize) -> Option<usize> {
        let new_end = self.lit(p, b"new")?;
        if let Some(start) = self.blanks1(new_end) {
            let found = self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
                let end = lx.blanks(ty.end);
                if lx.at(end) != b'(' || end >= lx.lim() {
                    return None;
                }
                lx.cap(1, p, new_end);
                lx.cap(2, ty.start, ty.end);
                Some(end)
            });
            if found.is_some() {
                return found;
            }
        }
        let end = self.blanks(new_end);
        if self.at(end) != b'(' || end >= self.lim() {
            return None;
        }
        self.cap(1, p, new_end);
        Some(end)
    }

    /// `(?=\{|//|/\*|$)` at `p`.
    fn initializer_ahead(&self, p: usize) -> bool {
        self.ahead_in(p, b"{") || self.starts(p, b"//") || self.starts(p, b"/*") || self.at_eol(p)
    }

    /// `(new)\s+T\s*(?=\{|//|/\*|$)`
    pub(in crate::csharp) fn object_creation_without_arguments(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let new_end = self.lit(p, b"new")?;
        let start = self.blanks1(new_end)?;
        self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
            let end = lx.blanks(ty.end);
            if !lx.initializer_ahead(end) {
                return None;
            }
            lx.cap(1, p, new_end);
            lx.cap(2, ty.start, ty.end);
            Some(end)
        })
    }

    /// `\b(new|stackalloc)\b\s*T?\s*(?=\[)`, the word fixed by the pattern.
    pub(in crate::csharp) fn array_creation(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let word_end = self.keyword(p, word)?;
        let start = self.blanks(word_end);
        let found = self.type_name(start, Shape::POINTER, &mut |lx, ty: Ty| {
            let end = lx.blanks(ty.end);
            if lx.at(end) != b'[' || end >= lx.lim() {
                return None;
            }
            lx.cap(1, p, word_end);
            lx.cap(2, ty.start, ty.end);
            Some(end)
        });
        if found.is_some() {
            return found;
        }
        if self.at(start) != b'[' || start >= self.lim() {
            return None;
        }
        self.cap(1, p, word_end);
        Some(start)
    }

    /// `\b(new)\b\s*(?=\{|//|/\*|$)`
    pub(in crate::csharp) fn anonymous_object_creation(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"new")?;
        let after = self.blanks(end);
        if !self.initializer_ahead(after) {
            return None;
        }
        self.cap(1, p, end);
        Some(after)
    }

    /// `(?<!\.)\b(with)\b\s*(?=\{|//|/\*|$)`
    pub(in crate::csharp) fn with_expression(&mut self, p: usize) -> Option<usize> {
        let end = self.non_member_keyword(p, b"with")?;
        let after = self.blanks(end);
        if !self.initializer_ahead(after) {
            return None;
        }
        self.cap(1, p, end);
        Some(after)
    }

    /// `((?:\b(?:async|static)\b\s*)*)`: the end of each iteration, so a
    /// caller can give iterations back.
    fn function_modifiers(&self, p: usize, ends: &mut [usize; 9]) -> usize {
        let mut count = 0;
        let mut at = p;
        'iterations: while count < 8 {
            for word in FUNCTION_MODIFIERS {
                if let Some(end) = self.keyword(at, word) {
                    at = self.blanks(end);
                    count += 1;
                    ends[count] = at;
                    continue 'iterations;
                }
            }
            break;
        }
        ends[0] = p;
        count
    }

    /// `((?:\b(?:async|static)\b\s*)*)(?:(@?ident)\b|(\()(?<tuple>...)(\)))\s*
    /// (=>)`
    pub(in crate::csharp) fn lambda_start(&mut self, p: usize) -> Option<usize> {
        let mut ends = [p; 9];
        let count = self.function_modifiers(p, &mut ends);
        for used in (0..=count).rev() {
            let at = ends[used];
            if let Some(name_end) = self.ident(at).filter(|&end| self.word_at(end).is_none()) {
                let arrow = self.blanks(name_end);
                if let Some(end) = self.lit(arrow, b"=>") {
                    if used > 0 {
                        self.cap(1, p, at);
                    }
                    self.cap(2, at, name_end);
                    self.cap(6, arrow, end);
                    return Some(end);
                }
            }
            if self.at(at) == b'('
                && at < self.lim()
                && let Some(close) = self.balanced(at, b'(', b')', Body::MayBeEmpty)
            {
                let arrow = self.blanks(close);
                if let Some(end) = self.lit(arrow, b"=>") {
                    if used > 0 {
                        self.cap(1, p, at);
                    }
                    self.cap(3, at, at + 1);
                    self.cap(4, at + 1, close - 1);
                    self.cap(5, close - 1, close);
                    self.cap(6, arrow, end);
                    return Some(end);
                }
            }
        }
        None
    }

    /// `((?:\b(?:async|static)\b\s*)*)\b(delegate)\b\s*`
    pub(in crate::csharp) fn delegate_start(&mut self, p: usize) -> Option<usize> {
        let mut ends = [p; 9];
        let count = self.function_modifiers(p, &mut ends);
        for used in (0..=count).rev() {
            let at = ends[used];
            if let Some(end) = self.keyword(at, b"delegate") {
                if used > 0 {
                    self.cap(1, p, at);
                }
                self.cap(2, at, end);
                return Some(self.blanks(end));
            }
        }
        None
    }

    /// `\b(ref)\b|(?=\S)`
    pub(in crate::csharp) fn ref_or_non_blank(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword(p, b"ref") {
            self.cap(1, p, end);
            return Some(end);
        }
        (self.avail(p) && self.space_width(p) == 0).then_some(p)
    }

    /// `@?ident\b`, the name of an anonymous function's parameter.
    pub(in crate::csharp) fn name_then_boundary(&self, p: usize) -> Option<usize> {
        self.ident(p).filter(|&end| self.word_at(end).is_none())
    }

    /// `(@?ident)\b`, a type parameter's name.
    pub(in crate::csharp) fn type_parameter_name(&mut self, p: usize) -> Option<usize> {
        let end = self.name_then_boundary(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?:\b(word)\b\s*)?T\s*\b(\g<identifier>)\b`: a parameter of an
    /// explicitly typed anonymous function, with the modifier fixed by the
    /// pattern.
    ///
    /// With a word the modifier is required; the pattern without one is the
    /// grammar's last choice and has no word.
    pub(in crate::csharp) fn explicit_function_parameter(
        &mut self,
        p: usize,
        word: &[u8],
    ) -> Option<usize> {
        let with_word = self
            .keyword(p, word)
            .map(|end| (Some((p, end)), self.blanks(end)));
        let starts = if word.is_empty() {
            [Some((None, p)), None]
        } else {
            [with_word, None]
        };
        for (modifier, start) in starts.into_iter().flatten() {
            let found = self.type_name(start, Shape::LOOSE, &mut |lx, ty: Ty| {
                let name_start = lx.blanks(ty.end);
                if !lx.boundary(name_start) {
                    return None;
                }
                let name_end = lx
                    .ident(name_start)
                    .filter(|&end| lx.word_at(end).is_none())?;
                if let Some((from, to)) = modifier {
                    lx.cap(1, from, to);
                }
                lx.cap(2, ty.start, ty.end);
                lx.cap(7, name_start, name_end);
                Some(name_end)
            });
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// `(?:\b(word)\b\s+)?T\s+(\g<identifier>)`: a parameter, with the modifier
    /// fixed by the pattern.
    pub(in crate::csharp) fn parameter(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let with_word = self
            .keyword(p, word)
            .and_then(|end| self.blanks1(end).map(|start| (Some((p, end)), start)));
        let starts = if word.is_empty() {
            [Some((None, p)), None]
        } else {
            [with_word, None]
        };
        for (modifier, start) in starts.into_iter().flatten() {
            let found = self.type_name(start, Shape::REF, &mut |lx, ty: Ty| {
                let (name_start, ty_end) = lx.plus_blanks(ty)?;
                let name_end = lx.ident(name_start)?;
                if let Some((from, to)) = modifier {
                    lx.cap(1, from, to);
                }
                lx.cap(2, ty.start, ty_end);
                lx.cap(7, name_start, name_end);
                Some(name_end)
            });
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// `(?:\b(var)\b|T)\s+(\g<identifier>)\b\s*(?=[]),])`: a declaration
    /// expression, with the lookahead's characters given.
    pub(in crate::csharp) fn declaration_expression(
        &mut self,
        p: usize,
        ahead: &[u8],
        name_group: usize,
    ) -> Option<usize> {
        let finish = |lx: &mut Self, name_start: usize| -> Option<(usize, usize)> {
            let name_end = lx
                .ident(name_start)
                .filter(|&end| lx.word_at(end).is_none())?;
            let end = lx.blanks(name_end);
            lx.ahead_in(end, ahead).then_some((name_end, end))
        };
        if let Some(var_end) = self.keyword(p, b"var")
            && let Some(name_start) = self.blanks1(var_end)
            && let Some((name_end, end)) = finish(self, name_start)
        {
            self.cap(1, p, var_end);
            self.cap(name_group, name_start, name_end);
            return Some(end);
        }
        self.type_name(p, Shape::STD, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let (name_end, end) = finish(lx, name_start)?;
            lx.cap(2, ty.start, ty_end);
            lx.cap(name_group, name_start, name_end);
            Some(end)
        })
    }

    /// `(@?ident)\b\s*(?=[),])`
    pub(in crate::csharp) fn tuple_element_variable(&mut self, p: usize) -> Option<usize> {
        let name_end = self.ident(p).filter(|&end| self.word_at(end).is_none())?;
        let end = self.blanks(name_end);
        if !self.ahead_in(end, b"),") {
            return None;
        }
        self.cap(1, p, name_end);
        Some(end)
    }

    /// `(?<tuple>\s*\((?:[^()]|\g<tuple>)+\))\s*(?!=[=>])(?==)`: a tuple,
    /// after blanks, on the left of an assignment.
    pub(in crate::csharp) fn tuple_assignment(&mut self, p: usize) -> Option<usize> {
        let open = self.blanks(p);
        self.tuple_assigned(p, open)
    }

    /// `(?<tuple>\((?:[^()]|\g<tuple>)+\))\s*(?!=[=>])(?==)`: a tuple of
    /// declarations on the left of an assignment.
    pub(in crate::csharp) fn local_tuple_assignment(&mut self, p: usize) -> Option<usize> {
        self.tuple_assigned(p, p)
    }

    /// The tuple that opens at `open`, then `=` but not `==` or `=>`, as
    /// group 1 from `p`: the position of the `=`.
    fn tuple_assigned(&mut self, p: usize, open: usize) -> Option<usize> {
        let close = self.balanced(open, b'(', b')', Body::NonEmpty)?;
        let at = self.blanks(close);
        if self.at(at) != b'=' || at >= self.lim() || self.byte_in(at + 1, b"=>") {
            return None;
        }
        self.cap(1, p, close);
        Some(at)
    }

    /// `\b(from|join)\b\s*T?\s+(\g<identifier>)\b\s*\b(in)\b\s*`: the head of
    /// a `from` or `join` clause.
    ///
    /// Without the type, `\s*` gives a blank back to `\s+`, so the name needs a
    /// blank after the keyword.
    pub(in crate::csharp) fn range_clause(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let word_end = self.keyword(p, word)?;
        let start = self.blanks(word_end);
        let finish = |lx: &mut Self, after_blanks: usize| -> Option<(usize, usize, usize, usize)> {
            let name_end = lx
                .ident(after_blanks)
                .filter(|&end| lx.word_at(end).is_none())?;
            let in_start = lx.blanks(name_end);
            let in_end = lx.keyword(in_start, b"in")?;
            Some((name_end, in_start, in_end, lx.blanks(in_end)))
        };
        let found = self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let (name_end, in_start, in_end, end) = finish(lx, name_start)?;
            lx.cap(1, p, word_end);
            lx.cap(2, ty.start, ty_end);
            lx.cap(7, name_start, name_end);
            lx.cap(8, in_start, in_end);
            Some(end)
        });
        if found.is_some() {
            return found;
        }
        if start == word_end {
            return None;
        }
        let (name_end, in_start, in_end, end) = finish(self, start)?;
        self.cap(1, p, word_end);
        self.cap(7, start, name_end);
        self.cap(8, in_start, in_end);
        Some(end)
    }

    /// `\b(word)\b\s*`, a keyword and the blanks after it.
    pub(in crate::csharp) fn keyword_and_blanks(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        Some(self.blanks(end))
    }

    /// `\b(let)\b\s*(@?ident)\b\s*(=)\s*`
    pub(in crate::csharp) fn let_clause(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"let")?;
        let name_start = self.blanks(end);
        let name_end = self
            .ident(name_start)
            .filter(|&end| self.word_at(end).is_none())?;
        let equals = self.blanks(name_end);
        let after = self.lit(equals, b"=")?;
        self.cap(1, p, end);
        self.cap(2, name_start, name_end);
        self.cap(3, equals, after);
        Some(self.blanks(after))
    }

    /// `\b(into)\b\s*(@?ident)\b\s*`
    pub(in crate::csharp) fn join_into(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"into")?;
        let name_start = self.blanks(end);
        let name_end = self
            .ident(name_start)
            .filter(|&end| self.word_at(end).is_none())?;
        self.cap(1, p, end);
        self.cap(2, name_start, name_end);
        Some(self.blanks(name_end))
    }

    /// `(?<!\.)\b(as)\b\s*T?`
    pub(in crate::csharp) fn as_expression(&mut self, p: usize) -> Option<usize> {
        let end = self.non_member_keyword(p, b"as")?;
        self.cap(1, p, end);
        let start = self.blanks(end);
        let found = self.type_name(start, Shape::AS, &mut |lx, ty: Ty| {
            lx.cap(2, ty.start, ty.end);
            Some(ty.end)
        });
        Some(found.unwrap_or(start))
    }
}
