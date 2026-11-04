//! Function definitions and operator overloads.

use super::super::engine::Lexer;
use super::directives::Reference;
use super::gaps::{Gap, GapForm};
use super::heads::{Head, Pointer, Pointers, Shape};
use super::qualified::{Layout, Step};
use super::scopes::Chain;
use crate::rules::Chars;

/// The head of `OperatorOverload`: the return type and its pointer marks.
const RETURN: Shape = Shape {
    storage: None,
    outer: Some(5),
    layout: Layout {
        attribute: Some(6),
        gap_a: 7,
        gap_b: 11,
        scope: 15,
        scope_template: (16, 17),
        gap_c: 18,
        template: 22,
        gap_form: GapForm::Parts,
    },
    pointer: Pointer {
        outer: 23,
        first: 24,
        each: 28,
        gap_form: GapForm::Parts,
    },
};

/// The head of `FunctionDefinition`: the return type and its pointer marks.
const DEFINED: Shape = Shape {
    storage: None,
    outer: Some(13),
    layout: Layout {
        attribute: Some(14),
        gap_a: 15,
        gap_b: 19,
        scope: 23,
        scope_template: (24, 25),
        gap_c: 26,
        template: 30,
        gap_form: GapForm::Parts,
    },
    pointer: Pointer {
        outer: 31,
        first: 32,
        each: 36,
        gap_form: GapForm::Parts,
    },
};

/// The words that may stand before the return type of a definition.
const SPECIFIERS: &[&[u8]] = &[
    b"constexpr",
    b"consteval",
    b"explicit",
    b"mutable",
    b"virtual",
    b"inline",
    b"friend",
    b"thread_local",
    b"volatile",
    b"register",
    b"restrict",
    b"static",
    b"extern",
    b"const",
];

/// The last iteration of the specifier loop: the word and the gap after it.
type Specified = (usize, usize, Gap);

/// The pointer marks of a conversion operator's type.
const CONVERSION: Pointer = Pointer {
    outer: 63,
    first: 64,
    each: 68,
    gap_form: GapForm::Parts,
};

/// The operators, in the order the expression tries them.
/// The characters that follow stand alone.
const OPERATORS: &[&[u8]] = &[
    b"delete[]",
    b"delete",
    b"new[]",
    b"<=>",
    b"<<=",
    b"new",
    b">>=",
    b"->*",
    b"/=",
    b"%=",
    b"&=",
    b">=",
    b"|=",
    b"++",
    b"--",
    b"()",
    b"[]",
    b"->",
    b"<<",
    b">>",
    b"<=",
    b"^=",
    b"==",
    b"!=",
    b"&&",
    b"||",
    b"+=",
    b"-=",
    b"*=",
];

/// `[!%\&*-\-/<=>^|~]`.
const OPERATOR_CHARS: &[u8] = b"!%&*+,-/<=>^|~";

/// What names the operator.
#[derive(Clone, Copy)]
enum Named {
    /// `delete[]`, `+` and the like: where it ends.
    Operator(usize),
    /// A conversion: the type name's end, its pointer marks, the gap after
    /// them, and where its optional `[]` ends.
    Conversion(usize, Option<Reference>, Gap, usize),
    /// A literal operator `""`: the gap after it, and the suffix's end.
    Literal(Gap, usize),
}

impl Lexer<'_> {
    /// `(?<=^|\))\s?(->)GAP(TYPE)`: the trailing return type of a function.
    pub(super) fn trailing_return(&mut self, p: usize) -> Option<usize> {
        if !(p == 0 || self.prev(p) == Some(b')')) {
            return None;
        }
        let arrow = self.skip_space(p);
        let after = self.lit(arrow, b"->")?;
        let layout = Layout {
            attribute: Some(7),
            gap_a: 8,
            gap_b: 12,
            scope: 16,
            scope_template: (17, 18),
            gap_c: 19,
            template: 23,
            gap_form: GapForm::Parts,
        };
        for gap in self.gaps(after) {
            let from = gap.end;
            let start = self.skip_space(from);
            if let Some(found) = self.first_type(start, &layout) {
                self.cap(1, arrow, after);
                self.put_gap(2, GapForm::Parts, after, &gap);
                self.cap(6, from, found.end);
                self.put_type(&layout, &found);
                return Some(found.end);
            }
        }
        None
    }

    /// The ways the name after `operator` and its chain matches at `p`, in
    /// order: the first for which `rest` matches what follows, given the name
    /// and where it ends, with what `rest` found.
    fn find_operator_name<R>(
        &self,
        p: usize,
        mut rest: impl FnMut(&Self, Named, usize) -> Option<R>,
    ) -> Option<R> {
        for op in OPERATORS {
            if let Some(end) = self.lit(p, op)
                && let Some(found) = rest(self, Named::Operator(end), end)
            {
                return Some(found);
            }
        }
        if p < self.lim()
            && OPERATOR_CHARS.contains(&self.s[p])
            && let Some(found) = rest(self, Named::Operator(p + 1), p + 1)
        {
            return Some(found);
        }
        if !self.word_before(p) {
            for id in self.ident_ends(p) {
                if self.word_at(id).is_some() {
                    continue;
                }
                let found = self.find_pointer_run(id, |lexer, run| {
                    let at = run.map_or(id, |r| r.0);
                    lexer.gaps(at).find_map(|gap| {
                        let bracketed = lexer.starts(gap.end, b"[]").then_some(gap.end + 2);
                        bracketed
                            .into_iter()
                            .chain([gap.end])
                            .find_map(|end| rest(lexer, Named::Conversion(id, run, gap, end), end))
                    })
                });
                if found.is_some() {
                    return found;
                }
            }
        }
        if self.starts(p, b"\"\"") {
            for gap in self.gaps(p + 2) {
                if self.word_before(gap.end) {
                    continue;
                }
                for id in self.ident_ends(gap.end) {
                    if self.word_at(id).is_none()
                        && let Some(found) = rest(self, Named::Literal(gap, id), id)
                    {
                        return Some(found);
                    }
                }
            }
        }
        None
    }

    /// `GAP(RETURN GAP)?(CALL)GAPGAP(CHAIN)(operator)(?:GAP(const))?GAP(CHAIN)
    /// NAME GAP(?=[(;<])`: the declaration of an operator overload.
    ///
    /// The return type with the gap after it is tried first, then none.
    pub(super) fn operator_overload(&mut self, p: usize) -> Option<usize> {
        let found = self.gaps(p).find_map(|first| {
            self.find_head(first.end, &RETURN, |lexer, head| {
                lexer
                    .gaps(head.end)
                    .find_map(|gap| lexer.overload_after(first, Some(&(*head, gap))))
            })
            .or_else(|| self.overload_after(first, None))
        })?;
        self.put_overload(p, &found);
        Some(found.end)
    }

    /// The rest of an operator overload after its first gap and its return
    /// type, if it has one.
    fn overload_after(&self, first: Gap, given: Option<&(Head, Gap)>) -> Option<Overload> {
        let start = given.map_or(first.end, |(_, gap)| gap.end);
        for convention in self.convention_ends(start) {
            for second in self.gaps(convention) {
                for third in self.gaps(second.end) {
                    for (chain, template) in self.scope_chain(third.end) {
                        let Some(word) = self.lit(chain, b"operator") else {
                            continue;
                        };
                        let consts = self.gaps(word).filter_map(|gap| {
                            let e = self.word_lit(gap.end, b"const")?;
                            Some((Some((gap, e)), e))
                        });
                        for (konst, after) in consts.chain([(None, word)]) {
                            for fourth in self.gaps(after) {
                                for (chain2, template2) in self.scope_chain(fourth.end) {
                                    let name = self.find_operator_name(
                                        chain2,
                                        |lexer, named, name_end| {
                                            let last = lexer.gaps(name_end).find(|last| {
                                                last.end < lexer.lim()
                                                    && matches!(
                                                        lexer.at(last.end),
                                                        b'(' | b';' | b'<'
                                                    )
                                            })?;
                                            Some((named, name_end, last))
                                        },
                                    );
                                    if let Some((named, name_end, last)) = name {
                                        return Some(Overload {
                                            first,
                                            given: given.copied(),
                                            start,
                                            convention,
                                            second,
                                            third,
                                            scope: (chain, template),
                                            word,
                                            konst,
                                            after,
                                            fourth,
                                            scope2: (chain2, template2),
                                            named,
                                            name_end,
                                            last,
                                            end: last.end,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Records the groups of an operator overload that began at `p`.
    fn put_overload(&mut self, p: usize, found: &Overload) {
        let first = found.first;
        self.put_gap(1, GapForm::Parts, p, &first);
        if let Some((head, gap)) = &found.given {
            self.put_head(first.end, &RETURN, head);
            self.put_gap(32, GapForm::Parts, head.end, gap);
        }
        self.cap(36, found.start, found.convention);
        self.put_gap(37, GapForm::Parts, found.convention, &found.second);
        self.put_gap(41, GapForm::Parts, found.second.end, &found.third);
        let (chain, template) = found.scope;
        self.cap(45, found.third.end, chain);
        if let Some((a, tpl_end, spaced)) = template {
            self.cap(46, a, spaced);
            self.cap(47, a, tpl_end);
        }
        self.cap(48, chain, found.word);
        if let Some((gap, e)) = found.konst {
            self.put_gap(49, GapForm::Parts, found.word, &gap);
            self.cap(53, gap.end, e);
        }
        self.put_gap(54, GapForm::Parts, found.after, &found.fourth);
        let (chain2, template2) = found.scope2;
        self.cap(58, found.fourth.end, chain2);
        if let Some((a, tpl_end, spaced)) = template2 {
            self.cap(59, a, spaced);
            self.cap(60, a, tpl_end);
        }
        match found.named {
            Named::Operator(end) => self.cap(61, chain2, end),
            Named::Conversion(id, run, gap, end) => {
                self.cap(62, chain2, end);
                let at = run.map_or(id, |r| r.0);
                if let Some(run) = run {
                    self.put_pointer(&CONVERSION, id, &run);
                }
                self.put_gap(72, GapForm::Parts, at, &gap);
                self.cap(76, gap.end, end);
            }
            Named::Literal(gap, id) => {
                self.cap(77, chain2, chain2 + 2);
                self.put_gap(78, GapForm::Parts, chain2 + 2, &gap);
                self.cap(82, gap.end, id);
            }
        }
        self.put_gap(83, GapForm::Parts, found.name_end, &found.last);
    }

    /// `(?:(WORD)GAP)*` at `p`, every way, the longest first.
    /// It finds the first way for which `rest` matches what follows, given
    /// where the loop ends and its last iteration.
    /// It returns what `rest` found.
    /// `last` is the iteration before `p`.
    fn find_definition_specifiers<R>(
        &self,
        p: usize,
        last: Option<Specified>,
        rest: &mut impl FnMut(&Self, usize, Option<Specified>) -> Option<R>,
    ) -> Option<R> {
        let Some(_depth) = self.deeper() else {
            return rest(self, p, last);
        };
        if let Some(word) = self.word_in(p, SPECIFIERS) {
            for gap in self.gaps(word) {
                if gap.end > p
                    && let Some(found) =
                        self.find_definition_specifiers(gap.end, Some((p, word, gap)), rest)
                {
                    return Some(found);
                }
            }
        }
        rest(self, p, last)
    }

    /// The definition of a function: its return type, name and the `(` that
    /// opens its parameters.
    ///
    /// It may start with `(template)GAP`.
    pub(super) fn function_definition(&mut self, p: usize) -> Option<usize> {
        let context = p == 0
            || self.at_g(p)
            || matches!(self.prev(p), Some(b';' | b'}' | b'>'))
            || self.ends_with_at(p, b"*/");
        if !context || !self.may_start_declaration(p, Pointers::Continue) {
            return None;
        }
        let start = self.skip_space(p);
        let templates = self
            .word_lit(start, b"template")
            .into_iter()
            .flat_map(|word| self.gaps(word).map(move |gap| (Some((word, gap)), gap.end)))
            .chain([(None, start)]);
        let found = templates.into_iter().find_map(|(template, after)| {
            self.attribute_options(after)
                .into_iter()
                .find_map(|attribute| {
                    let loop_start = attribute.unwrap_or(after);
                    self.find_definition_specifiers(
                        loop_start,
                        None,
                        &mut |lexer, loop_end, last| {
                            lexer.find_head(loop_end, &DEFINED, |lexer, head| {
                                let name = lexer.definition_name(head.end)?;
                                Some(Definition {
                                    template,
                                    after,
                                    attribute,
                                    loop_start,
                                    loop_end,
                                    last,
                                    head: *head,
                                    name,
                                })
                            })
                        },
                    )
                })
        })?;
        if let Some((word, gap)) = found.template {
            self.cap(1, start, word);
            self.put_gap(2, GapForm::Parts, word, &gap);
        }
        if let Some(end) = found.attribute {
            self.cap(6, found.after, end);
        }
        self.cap(7, found.loop_start, found.loop_end);
        if let Some((from, word, gap)) = found.last {
            self.cap(8, from, word);
            self.put_gap(9, GapForm::Parts, word, &gap);
        }
        let head = found.head;
        let name = found.name;
        self.put_head(found.loop_end, &DEFINED, &head);
        self.put_gap(40, GapForm::Parts, head.end, &name.fifth);
        self.cap(44, name.fifth.end, name.convention);
        self.put_gap(45, GapForm::Parts, name.convention, &name.sixth);
        self.cap(49, name.sixth.end, name.chain.end);
        if let Some(at) = name.chain.colons {
            self.cap(50, at, at + 2);
        }
        if let Some((a, tpl_end, spaced)) = name.chain.template {
            self.cap(51, a, spaced);
            self.cap(52, a, tpl_end);
        }
        self.cap(53, name.chain.end, name.end);
        self.put_gap(54, GapForm::Parts, name.end, &name.seventh);
        Some(name.seventh.end)
    }

    /// The name of a function definition after its return type, which ends at
    /// `at`: the gap, calling convention, gap, scope chain, name and gap
    /// before the `(`.
    fn definition_name(&self, at: usize) -> Option<DefinitionName> {
        for fifth in self.gaps(at) {
            for convention in self.convention_ends(fifth.end) {
                for sixth in self.gaps(convention) {
                    let found = self.find_scope_chain(sixth.end, |lexer, chain| {
                        lexer.ident_ends(chain.end).find_map(|end| {
                            if !lexer.boundary(end) || !lexer.not_behind_listed(end) {
                                return None;
                            }
                            let seventh = lexer.gaps(end).find(|seventh| {
                                seventh.end < lexer.lim() && lexer.at(seventh.end) == b'('
                            })?;
                            Some(DefinitionName {
                                fifth,
                                convention,
                                sixth,
                                chain: *chain,
                                end,
                                seventh,
                            })
                        })
                    });
                    if found.is_some() {
                        return found;
                    }
                }
            }
        }
        None
    }
}

/// One way an operator overload matches, with what [`Lexer::put_overload`]
/// records.
struct Overload {
    first: Gap,
    given: Option<(Head, Gap)>,
    start: usize,
    convention: usize,
    second: Gap,
    third: Gap,
    scope: Step,
    word: usize,
    konst: Option<(Gap, usize)>,
    after: usize,
    fourth: Gap,
    scope2: Step,
    named: Named,
    name_end: usize,
    last: Gap,
    end: usize,
}

/// One way a function definition matches, up to its name.
struct Definition {
    template: Option<(usize, Gap)>,
    after: usize,
    attribute: Option<usize>,
    loop_start: usize,
    loop_end: usize,
    last: Option<Specified>,
    head: Head,
    name: DefinitionName,
}

/// The name of a function definition and what lies around it.
struct DefinitionName {
    fifth: Gap,
    convention: usize,
    sixth: Gap,
    chain: Chain,
    end: usize,
    seventh: Gap,
}
