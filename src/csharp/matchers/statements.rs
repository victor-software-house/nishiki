//! Statements, directives, attributes and patterns.

use super::super::engine::Lexer;
use super::chars::Body;
use super::types::{Shape, Ty};
use crate::rules::Chars;

/// `[_[:alpha:]]`-led characters and the class `[]\&(),:;=@^_{|}[:alpha:]]`
/// that ends a type pattern: the ASCII members of the class.
const TYPE_PATTERN_END: &[u8] = b"]&(),:;=@^_{|}";

/// What may precede `var` in a local declaration: `ref`, `ref readonly` or
/// nothing.
#[derive(Clone, Copy, Debug)]
struct VarPrefix {
    /// The end of `ref`.
    ref_end: Option<usize>,
    /// Where `readonly` starts and ends.
    readonly: Option<(usize, usize)>,
    /// Where `var` would start.
    var_at: usize,
}

impl Lexer<'_> {
    /// `(@?ident)\s*(::)` and `(@?ident)\s*(\.)`: a name and the separator
    /// that follows it, with the separator fixed by the pattern.
    pub(in crate::csharp) fn name_then(&mut self, p: usize, separator: &[u8]) -> Option<usize> {
        let name_end = self.ident(p)?;
        let at = self.blanks(name_end);
        let end = self.lit(at, separator)?;
        self.cap(1, p, name_end);
        self.cap(2, at, end);
        Some(end)
    }

    /// `(\.)\s*(@?ident)`
    pub(in crate::csharp) fn dot_then_name(&mut self, p: usize) -> Option<usize> {
        let dot = self.lit(p, b".")?;
        let start = self.blanks(dot);
        let end = self.ident(start)?;
        self.cap(1, p, dot);
        self.cap(2, start, end);
        Some(end)
    }

    /// `\b(extern)\s+(alias)\b`
    pub(in crate::csharp) fn extern_alias(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let a = self.lit(p, b"extern")?;
        let b = self.blanks1(a)?;
        let c = self.lit(b, b"alias")?;
        if self.word_at(c).is_some() {
            return None;
        }
        self.cap(1, p, a);
        self.cap(2, b, c);
        Some(c)
    }

    /// `\b(?:(global)\s+)?`: the end of the optional `global`, trying it
    /// first, then nothing.
    fn global_choices(&self, p: usize) -> [Option<(Option<usize>, usize)>; 2] {
        let with = self
            .keyword(p, b"global")
            .and_then(|end| self.blanks1(end).map(|after| (Some(end), after)));
        [with, (!self.word_before(p)).then_some((None, p))]
    }

    /// `\b(?:(global)\s+)?(using)\s+(static)\b\s*(?:(unsafe)\b\s*)?`
    pub(in crate::csharp) fn using_static(&mut self, p: usize) -> Option<usize> {
        for (global, at) in self.global_choices(p).into_iter().flatten() {
            let Some(using_end) = self.lit(at, b"using") else {
                continue;
            };
            let Some(static_at) = self.blanks1(using_end) else {
                continue;
            };
            let Some(static_end) = self
                .lit(static_at, b"static")
                .filter(|&end| self.word_at(end).is_none())
            else {
                continue;
            };
            let mut end = self.blanks(static_end);
            if let Some(unsafe_end) = self.keyword(end, b"unsafe") {
                self.cap(4, end, unsafe_end);
                end = self.blanks(unsafe_end);
            }
            if let Some(global_end) = global {
                self.cap(1, p, global_end);
            }
            self.cap(2, at, using_end);
            self.cap(3, static_at, static_end);
            return Some(end);
        }
        None
    }

    /// `\b(?:(global)\s+)?(using)\b\s*(?:(unsafe)\b\s*)?(@?ident)\s*(=)`: a
    /// using alias.
    pub(in crate::csharp) fn using_alias(&mut self, p: usize) -> Option<usize> {
        for (global, at) in self.global_choices(p).into_iter().flatten() {
            let Some(using_end) = self.keyword(at, b"using") else {
                continue;
            };
            let after_using = self.blanks(using_end);
            let with_unsafe = self
                .keyword(after_using, b"unsafe")
                .map(|end| (Some((after_using, end)), self.blanks(end)));
            for (unsafe_span, name_start) in [with_unsafe, Some((None, after_using))]
                .into_iter()
                .flatten()
            {
                let Some(name_end) = self.ident(name_start) else {
                    continue;
                };
                let equals = self.blanks(name_end);
                let Some(end) = self.lit(equals, b"=") else {
                    continue;
                };
                if let Some(global_end) = global {
                    self.cap(1, p, global_end);
                }
                self.cap(2, at, using_end);
                if let Some((from, to)) = unsafe_span {
                    self.cap(3, from, to);
                }
                self.cap(4, name_start, name_end);
                self.cap(5, equals, end);
                return Some(end);
            }
        }
        None
    }

    /// `\b(?:(global)\s+)?(using)\b\s*+(?!\(|var\b)`
    pub(in crate::csharp) fn using_namespace(&mut self, p: usize) -> Option<usize> {
        for (global, at) in self.global_choices(p).into_iter().flatten() {
            let Some(using_end) = self.keyword(at, b"using") else {
                continue;
            };
            let end = self.blanks(using_end);
            if self.ahead_in(end, b"(") || self.keyword(end, b"var").is_some() {
                continue;
            }
            if let Some(global_end) = global {
                self.cap(1, p, global_end);
            }
            self.cap(2, at, using_end);
            return Some(end);
        }
        None
    }

    /// `(\[)(assembly|module|field|event|method|param|property|return|typevar|type)?(:)?`
    pub(in crate::csharp) fn attribute_section(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"[")?;
        self.cap(1, p, open);
        let mut end = open;
        for word in [
            &b"assembly"[..],
            b"module",
            b"field",
            b"event",
            b"method",
            b"param",
            b"property",
            b"return",
            b"typevar",
            b"type",
        ] {
            if let Some(word_end) = self.lit(open, word) {
                self.cap(2, open, word_end);
                end = word_end;
                break;
            }
        }
        if let Some(colon) = self.lit(end, b":") {
            self.cap(3, end, colon);
            end = colon;
        }
        Some(end)
    }

    /// `(?=([),]))` and its kin: a lookahead that records nothing.
    pub(in crate::csharp) fn ahead_of_class(&self, p: usize, set: &[u8]) -> Option<usize> {
        self.ahead_in(p, set).then_some(p)
    }

    /// `(?=(\[))` and `(?=(]))`: a lookahead that names the bracket.
    pub(in crate::csharp) fn ahead_of_bracket(&mut self, p: usize, bracket: u8) -> Option<usize> {
        if !self.ahead_in(p, &[bracket]) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p)
    }

    /// `T(?:\s*\?\s*)?(?:\s*\[...)*)(?:(\g<identifier>)\b)?`: a tuple's
    /// element.
    pub(in crate::csharp) fn tuple_element(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::STD, &mut |lx, ty: Ty| {
            lx.cap(1, ty.start, ty.end);
            if let Some(name_end) = lx.ident(ty.end).filter(|&end| lx.word_at(end).is_none()) {
                lx.cap(6, ty.end, name_end);
                return Some(name_end);
            }
            Some(ty.end)
        })
    }

    /// `(?<!\.)\b(word)\b\s*(?=\()`, for `if` and `while`.
    pub(in crate::csharp) fn keyword_then_paren(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.non_member_keyword(p, word)?;
        let paren = self.blanks(end);
        if !self.ahead_in(paren, b"(") {
            return None;
        }
        self.cap(1, p, end);
        Some(paren)
    }

    /// `(?<!\.)\b(when)\b\s*(\()`
    pub(in crate::csharp) fn when_clause(&mut self, p: usize) -> Option<usize> {
        let end = self.non_member_keyword(p, b"when")?;
        let open = self.blanks(end);
        let after = self.lit(open, b"(")?;
        self.cap(1, p, end);
        self.cap(2, open, after);
        Some(after)
    }

    /// `(?<!\.)\b(yield)\b\s*\b(return|break)\b`
    pub(in crate::csharp) fn yield_statement(&mut self, p: usize, second: &[u8]) -> Option<usize> {
        let end = self.non_member_keyword(p, b"yield")?;
        let at = self.blanks(end);
        let second_end = self.keyword(at, second)?;
        self.cap(1, p, end);
        self.cap(2, at, second_end);
        Some(second_end)
    }

    /// `\b(checked|unchecked|unsafe)\b(?!\s*[(@_[:alpha:]])`
    pub(in crate::csharp) fn context_control(&self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        let at = self.blanks(end);
        let blocked = self.ahead_in(at, b"(@_") || self.alpha_at(at).is_some();
        (!blocked).then_some(end)
    }

    /// `T\s*(?:(\g<identifier>)\b)?`, the type of a `catch`.
    pub(in crate::csharp) fn catch_declaration(&mut self, p: usize) -> Option<usize> {
        self.type_name(p, Shape::STD, &mut |lx, ty: Ty| {
            let at = lx.blanks(ty.end);
            lx.cap(1, ty.start, ty.end);
            if let Some(name_end) = lx.ident(at).filter(|&end| lx.word_at(end).is_none()) {
                lx.cap(6, at, name_end);
                return Some(name_end);
            }
            Some(at)
        })
    }

    /// `\b(?<const_keyword>const)\b\s*T\s+(\g<identifier>)\s*(?=[,;=])`
    pub(in crate::csharp) fn local_constant(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"const")?;
        let start = self.blanks(end);
        self.type_name(start, Shape::STD, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let name_end = lx.ident(name_start)?;
            let after = lx.blanks(name_end);
            if !lx.ahead_in(after, b",;=") {
                return None;
            }
            lx.cap(1, p, end);
            lx.cap(2, ty.start, ty_end);
            lx.cap(7, name_start, name_end);
            Some(after)
        })
    }

    /// `\s+(\g<identifier>)\s*(?!=>)(?=[),;=])` after a local's type or `var`:
    /// the name, and the end.
    fn local_name(&self, after_type: usize) -> Option<(usize, usize, usize)> {
        let name_start = self.blanks1(after_type)?;
        let name_end = self.ident(name_start)?;
        let end = self.blanks(name_end);
        if self.starts(end, b"=>") || !self.ahead_in(end, b"),;=") {
            return None;
        }
        Some((name_start, name_end, end))
    }

    /// `(?:(?:\b(ref)\s+(?:\b(readonly)\s+)?)?\b(var)\b|T)\s+(\g<identifier>)\
    /// s*(?!=>)(?=[),;=])`
    ///
    /// The prefixes of `var` are tried in the grammar's order: `ref readonly`,
    /// `ref`, then none.
    pub(in crate::csharp) fn local_variable(&mut self, p: usize) -> Option<usize> {
        let none = VarPrefix {
            ref_end: None,
            readonly: None,
            var_at: p,
        };
        let mut prefixes = [None, None, Some(none)];
        if let Some(ref_end) = self.keyword(p, b"ref")
            && let Some(after_ref) = self.blanks1(ref_end)
        {
            if let Some(readonly_end) = self.keyword(after_ref, b"readonly")
                && let Some(after_readonly) = self.blanks1(readonly_end)
            {
                prefixes[0] = Some(VarPrefix {
                    ref_end: Some(ref_end),
                    readonly: Some((after_ref, readonly_end)),
                    var_at: after_readonly,
                });
            }
            prefixes[1] = Some(VarPrefix {
                ref_end: Some(ref_end),
                readonly: None,
                var_at: after_ref,
            });
        }
        for prefix in prefixes.into_iter().flatten() {
            let at = prefix.var_at;
            let Some(var_end) = self.keyword(at, b"var") else {
                continue;
            };
            let Some((name_start, name_end, end)) = self.local_name(var_end) else {
                continue;
            };
            if let Some(ref_end) = prefix.ref_end {
                self.cap(1, p, ref_end);
            }
            if let Some((from, to)) = prefix.readonly {
                self.cap(2, from, to);
            }
            self.cap(3, at, var_end);
            self.cap(9, name_start, name_end);
            return Some(end);
        }
        self.type_name(p, Shape::LOCAL, &mut |lx, ty: Ty| {
            let (name_start, ty_end) = lx.plus_blanks(ty)?;
            let name_end = lx.ident(name_start)?;
            let end = lx.blanks(name_end);
            if lx.starts(end, b"=>") || !lx.ahead_in(end, b"),;=") {
                return None;
            }
            lx.cap(4, ty.start, ty_end);
            lx.cap(9, name_start, name_end);
            Some(end)
        })
    }

    /// `(?<![!=])(=)(?![=>])`
    pub(in crate::csharp) fn variable_initializer(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"=")?;
        if matches!(self.prev(p), Some(b'!' | b'=')) || self.byte_in(end, b"=>") {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(var)\b\s*(?<tuple>\((?:[^()]|\g<tuple>)+\))\s*(?=[);=])`
    pub(in crate::csharp) fn local_tuple_var(&mut self, p: usize) -> Option<usize> {
        let var_end = self.keyword(p, b"var")?;
        let open = self.blanks(var_end);
        let close = self.balanced(open, b'(', b')', Body::NonEmpty)?;
        let end = self.blanks(close);
        if !self.ahead_in(end, b");=") {
            return None;
        }
        self.cap(1, p, var_end);
        self.cap(2, open, close);
        Some(end)
    }

    /// `(?:(?:\b(ref)\s+)?\b(var)\b|T)\s+(\g<identifier>)\s+\b(in)\b`
    pub(in crate::csharp) fn foreach_variable(&mut self, p: usize) -> Option<usize> {
        let finish = |lx: &Self, name_start: usize| -> Option<(usize, usize, usize)> {
            let name_end = lx.ident(name_start)?;
            let in_start = lx.blanks1(name_end)?;
            let in_end = lx.keyword(in_start, b"in")?;
            Some((name_end, in_start, in_end))
        };
        let with_ref = self
            .keyword(p, b"ref")
            .and_then(|end| self.blanks1(end).map(|after| (Some(end), after)));
        for (ref_end, at) in [with_ref, Some((None, p))].into_iter().flatten() {
            let Some(var_end) = self.keyword(at, b"var") else {
                continue;
            };
            let Some(name_start) = self.blanks1(var_end) else {
                continue;
            };
            let Some((name_end, in_start, in_end)) = finish(self, name_start) else {
                continue;
            };
            if let Some(end) = ref_end {
                self.cap(1, p, end);
            }
            self.cap(2, at, var_end);
            self.cap(8, name_start, name_end);
            self.cap(9, in_start, in_end);
            return Some(in_end);
        }
        self.type_name(p, Shape::REF, &mut |lx, ty: Ty| {
            let (name_start, type_end) = lx.plus_blanks(ty)?;
            let (name_end, in_start, in_end) = finish(lx, name_start)?;
            lx.cap(3, ty.start, type_end);
            lx.cap(8, name_start, name_end);
            lx.cap(9, in_start, in_end);
            Some(in_end)
        })
    }

    /// `(?:\b(var)\b\s*)?(?<tuple>\((?:[^()]|\g<tuple>)+\))\s+\b(in)\b`
    pub(in crate::csharp) fn foreach_tuple(&mut self, p: usize) -> Option<usize> {
        let with_var = self
            .keyword(p, b"var")
            .map(|end| (Some(end), self.blanks(end)));
        for (var_end, open) in [with_var, Some((None, p))].into_iter().flatten() {
            let Some(close) = self.balanced(open, b'(', b')', Body::NonEmpty) else {
                continue;
            };
            let Some(in_start) = self.blanks1(close) else {
                continue;
            };
            let Some(in_end) = self.keyword(in_start, b"in") else {
                continue;
            };
            if let Some(end) = var_end {
                self.cap(1, p, end);
            }
            self.cap(2, open, close);
            self.cap(3, in_start, in_end);
            return Some(in_end);
        }
        None
    }

    /// `\b((?:(?:async|unsafe|static|extern)\s+)*)T\s+(\g<identifier>)\s*
    /// (<[^<>]+>)?\s*(?=\()`
    pub(in crate::csharp) fn local_function(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut ends = [p; 9];
        let mut count = 0;
        let mut at = p;
        'iterations: while count < 8 {
            for word in [&b"async"[..], b"unsafe", b"static", b"extern"] {
                if let Some(end) = self.lit(at, word)
                    && let Some(after) = self.blanks1(end)
                {
                    at = after;
                    count += 1;
                    ends[count] = at;
                    continue 'iterations;
                }
            }
            break;
        }
        for used in (0..=count).rev() {
            let start = ends[used];
            let found = self.type_name(start, Shape::LOCAL_FUNCTION, &mut |lx, ty: Ty| {
                let (name_start, ty_end) = lx.plus_blanks(ty)?;
                let name_end = lx.ident(name_start)?;
                let end = lx.type_parameters_then_paren(name_end, 8)?;
                if used > 0 {
                    lx.cap(1, p, start);
                }
                lx.cap(2, ty.start, ty_end);
                lx.cap(7, name_start, name_end);
                Some(end)
            });
            if found.is_some() {
                return found;
            }
        }
        None
    }

    /// `(\()([.:@_\s[:alnum:]]+)(\))(?=[-!+~\s]*@?["'(_[:alnum:]]+)`
    pub(in crate::csharp) fn casted_constant(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"(")?;
        let mut end = open;
        while let Some((c, next)) = self.cp(end) {
            if matches!(c, '.' | ':' | '@' | '_')
                || crate::classes::is_space(c)
                || c.is_alphanumeric()
            {
                end = next;
            } else {
                break;
            }
        }
        if end == open && !(self.avail(end) && self.space_width(end) > 0) {
            return None;
        }
        let close = self.lit(end, b")")?;
        let mut at = close;
        while let Some((c, next)) = self.cp(at) {
            if matches!(c, '-' | '!' | '+' | '~') || crate::classes::is_space(c) {
                at = next;
            } else {
                break;
            }
        }
        if self.at(at) == b'@' && at < self.lim() {
            at += 1;
        }
        if !(self.ahead_in(at, b"\"'(_") || self.alnum_run(at) > at) {
            return None;
        }
        self.cap(1, p, open);
        self.cap(2, open, end);
        self.cap(3, end, close);
        Some(close)
    }

    /// `(@?ident(?:\s*\.\s*@?ident)*)\s*(:)`
    pub(in crate::csharp) fn subpattern(&mut self, p: usize) -> Option<usize> {
        let mut end = self.ident(p)?;
        loop {
            let dot = self.blanks(end);
            if self.at(dot) != b'.' || dot >= self.lim() {
                break;
            }
            let Some(next) = self.ident(self.blanks(dot + 1)) else {
                break;
            };
            end = next;
        }
        let colon = self.blanks(end);
        let after = self.lit(colon, b":")?;
        self.cap(1, p, end);
        self.cap(2, colon, after);
        Some(after)
    }

    /// `(?=@?[_[:alpha:]][_[:alnum:]]*)`
    pub(in crate::csharp) fn ahead_of_name(&self, p: usize) -> Option<usize> {
        self.ident(p).map(|_| p)
    }

    /// `(?=[(@_{[:alpha:]])`
    pub(in crate::csharp) fn type_pattern_body_ahead(&self, p: usize) -> Option<usize> {
        (self.ahead_in(p, b"(@_{") || self.alpha_at(p).is_some()).then_some(p)
    }

    /// `\G`
    pub(in crate::csharp) fn at_anchor(&self, p: usize) -> Option<usize> {
        self.at_g(p).then_some(p)
    }

    /// `(?!\G[@_[:alpha:]])(?=[]\&(),:;=@^_{|}[:alpha:]]|(?:\s|^)\?|!=|\
    /// b(and|or|when)\b)`
    pub(in crate::csharp) fn type_pattern_end(&self, p: usize) -> Option<usize> {
        if self.at_g(p) && (self.ahead_in(p, b"@_") || self.alpha_at(p).is_some()) {
            return None;
        }
        let class = self.ahead_in(p, TYPE_PATTERN_END) || self.alpha_at(p).is_some();
        let question = (self.space_width(p) > 0 || p == 0)
            && self.at(p + self.space_width(p)) == b'?'
            && p + self.space_width(p) < self.lim();
        (class || question || self.starts(p, b"!=") || self.pattern_word(p)).then_some(p)
    }

    /// `(?<=[_[:alnum:]])|(?=[class]|!=|\b(and|or|when)\b)`: the end of a
    /// qualified name in a type pattern, with the bytes of the class.
    pub(in crate::csharp) fn type_subpattern_end(&self, p: usize, set: &[u8]) -> Option<usize> {
        let behind = self
            .char_before(p)
            .is_some_and(|c| c == '_' || c.is_alphabetic() || crate::classes::is_digit(c));
        (behind || self.ahead_in(p, set) || self.starts(p, b"!=") || self.pattern_word(p))
            .then_some(p)
    }

    /// `(?<!\s)\?`
    pub(in crate::csharp) fn tight_question(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"?")?;
        let spaced = self.char_before(p).is_some_and(crate::classes::is_space);
        (!spaced).then_some(end)
    }

    /// `(?!\s)`
    pub(in crate::csharp) fn not_blank_ahead(&self, p: usize) -> Option<usize> {
        (!(self.avail(p) && self.space_width(p) > 0)).then_some(p)
    }

    /// `(?=[class]|=>|\bwhen\b)` and its kin: a byte of `set`, one of
    /// `texts`, or one of the keywords `words`.
    pub(in crate::csharp) fn word_or_class_ahead(
        &self,
        p: usize,
        set: &[u8],
        texts: &[&[u8]],
        words: &[&[u8]],
    ) -> Option<usize> {
        (self.ahead_in(p, set)
            || texts.iter().any(|text| self.starts(p, text))
            || words.iter().any(|word| self.keyword(p, word).is_some()))
        .then_some(p)
    }
}
