//! Variable declarations and assignments, aliases and range `for`
//! declarations: a head (storage, type, pointer marks), a name, and what
//! follows it.

use super::super::engine::Lexer;
use super::directives::Reference;
use super::gaps::{Gap, GapForm};
use super::heads::{Head, Pointer, Pointers, Shape, Storage};
use super::qualified::Layout;
use crate::rules::Chars;

/// The head of `NormalVariableDeclaration` and `NormalVariableAssignment`.
const NORMAL: Shape = Shape {
    storage: Some(Storage {
        group: 6,
        inline: 7,
        parts: true,
        after: Some(10),
    }),
    outer: Some(14),
    layout: Layout {
        attribute: Some(15),
        gap_a: 16,
        gap_b: 20,
        scope: 24,
        scope_template: (25, 26),
        gap_c: 27,
        template: 31,
        gap_form: GapForm::Parts,
    },
    pointer: Pointer {
        outer: 32,
        first: 33,
        each: 37,
        gap_form: GapForm::Parts,
    },
};

/// The head of `VariableAssignment`.
const ASSIGNED: Shape = Shape {
    storage: Some(Storage {
        group: 1,
        inline: 2,
        parts: false,
        after: Some(3),
    }),
    outer: Some(5),
    layout: Layout {
        attribute: Some(6),
        gap_a: 7,
        gap_b: 9,
        scope: 11,
        scope_template: (12, 13),
        gap_c: 14,
        template: 16,
        gap_form: GapForm::Plain,
    },
    pointer: Pointer {
        outer: 17,
        first: 18,
        each: 20,
        gap_form: GapForm::Plain,
    },
};

/// The head of `SimpleArrayAssignment`.
const ARRAY: Shape = Shape {
    storage: None,
    outer: Some(1),
    layout: Layout {
        attribute: Some(2),
        gap_a: 3,
        gap_b: 5,
        scope: 7,
        scope_template: (8, 9),
        gap_c: 10,
        template: 12,
        gap_form: GapForm::Plain,
    },
    pointer: Pointer {
        outer: 13,
        first: 14,
        each: 16,
        gap_form: GapForm::Plain,
    },
};

/// The head of the range `for` expressions.
const RANGE: Shape = Shape {
    storage: Some(Storage {
        group: 2,
        inline: 3,
        parts: true,
        after: Some(6),
    }),
    outer: Some(10),
    layout: Layout {
        attribute: Some(11),
        gap_a: 12,
        gap_b: 16,
        scope: 20,
        scope_template: (21, 22),
        gap_c: 23,
        template: 27,
        gap_form: GapForm::Parts,
    },
    pointer: Pointer {
        outer: 28,
        first: 29,
        each: 33,
        gap_form: GapForm::Parts,
    },
};

/// The two expressions of a variable's declaration.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VariableForm {
    /// Ends at a `,`, `;` or `[` with no `=` in the rest of the line.
    Declaration,
    /// Ends at an assignment operator.
    Assignment,
}

impl Lexer<'_> {
    /// `^GAP(…head GAP name GAP)` and then a `,`, `;` or `[` with no `=` in
    /// the rest of the line, or an assignment operator.
    pub(super) fn normal_variable(&mut self, p: usize, form: VariableForm) -> Option<usize> {
        if p != 0 || !self.may_start_declaration(0, Pointers::Continue) {
            return None;
        }
        let (first, head, second, kind, name, third, end, op) = self.gaps(p).find_map(|first| {
            self.find_head(first.end, &NORMAL, |lexer, head| {
                lexer.gaps(head.end).find_map(|second| {
                    lexer
                        .variable_names(second.end)
                        .into_iter()
                        .find_map(|(kind, name)| {
                            lexer.gaps(name).find_map(|third| {
                                let (end, op) = lexer.variable_end(third.end, form)?;
                                Some((first, *head, second, kind, name, third, end, op))
                            })
                        })
                })
            })
        })?;
        self.put_gap(1, GapForm::Parts, p, &first);
        self.cap(5, first.end, end);
        self.put_head(first.end, &NORMAL, &head);
        self.put_gap(41, GapForm::Parts, head.end, &second);
        self.cap(44 + kind, second.end, name);
        self.put_gap(50, GapForm::Parts, name, &third);
        if let Some(group) = op {
            self.cap(group, third.end, end);
        }
        Some(end)
    }

    /// Where a normal variable's expression ends after its name and gap, at
    /// `at`.
    /// It ends at an assignment operator, or at a `,`, `;` or `[` with no `=`
    /// in the rest of the line.
    ///
    /// It gives where the match ends and, for an assignment, the group of
    /// the operator.
    fn variable_end(&self, at: usize, form: VariableForm) -> Option<(usize, Option<usize>)> {
        match form {
            VariableForm::Assignment => {
                let (group, end) = self.assign_op(at)?;
                Some((end, Some(54 + group)))
            }
            VariableForm::Declaration => {
                let ahead = at < self.lim() && matches!(self.at(at), b',' | b';' | b'[');
                let equals = self
                    .s
                    .get(at + 1..)
                    .is_some_and(|rest| rest.contains(&b'='));
                (ahead && !equals).then_some((at, None))
            }
        }
    }

    /// `(?:(STORAGE)GAP)?(TYPE)(POINTER)?GAP name GAP op` for an assignment
    /// to a variable of a known type.
    pub(super) fn variable_assignment(&mut self, p: usize) -> Option<usize> {
        if !self.may_start_declaration(p, Pointers::Continue) {
            return None;
        }
        let (head, second, kind, name, third, op, end) =
            self.find_head(p, &ASSIGNED, |lexer, head| {
                lexer.gaps(head.end).find_map(|second| {
                    lexer
                        .variable_names(second.end)
                        .into_iter()
                        .find_map(|(kind, name)| {
                            lexer.gaps(name).find_map(|third| {
                                let (op, end) = lexer.assign_op(third.end)?;
                                Some((*head, second, kind, name, third, op, end))
                            })
                        })
                })
            })?;
        self.put_head(p, &ASSIGNED, &head);
        self.put_gap(22, GapForm::Plain, head.end, &second);
        self.cap(23 + kind, second.end, name);
        self.put_gap(29, GapForm::Plain, name, &third);
        self.cap(31 + op, third.end, end);
        Some(end)
    }

    /// `(TYPE)(POINTER)?GAP name(\[) *(\])GAP op`: an assignment to an array.
    pub(super) fn simple_array_assignment(&mut self, p: usize) -> Option<usize> {
        let (head, second, kind, name, close, third, op, end) =
            self.find_head(p, &ARRAY, |lexer, head| {
                lexer.gaps(head.end).find_map(|second| {
                    lexer
                        .variable_names(second.end)
                        .into_iter()
                        .find_map(|(kind, name)| {
                            if lexer.at(name) != b'[' || name >= lexer.lim() {
                                return None;
                            }
                            let mut close = name + 1;
                            while lexer.at(close) == b' ' && close < lexer.lim() {
                                close += 1;
                            }
                            if lexer.at(close) != b']' || close >= lexer.lim() {
                                return None;
                            }
                            lexer.gaps(close + 1).find_map(|third| {
                                let (op, end) = lexer.assign_op(third.end)?;
                                Some((*head, second, kind, name, close, third, op, end))
                            })
                        })
                })
            })?;
        self.put_head(p, &ARRAY, &head);
        self.put_gap(18, GapForm::Plain, head.end, &second);
        self.cap(19 + kind, second.end, name);
        self.cap(25, name, name + 1);
        self.cap(26, close, close + 1);
        self.put_gap(27, GapForm::Plain, close + 1, &third);
        self.cap(29 + op, third.end, end);
        Some(end)
    }

    /// `(HEAD)GAP(ID)GAP(:)(?!:)`: the declaration of a range `for` loop.
    pub(super) fn range_for_variable(&mut self, p: usize) -> Option<usize> {
        let (head, second, name, third) = self.find_head(p, &RANGE, |lexer, head| {
            lexer.gaps(head.end).find_map(|second| {
                lexer.ident_ends(second.end).find_map(|name| {
                    lexer.gaps(name).find_map(|third| {
                        let colon = third.end;
                        (lexer.at(colon) == b':'
                            && colon < lexer.lim()
                            && lexer.at(colon + 1) != b':')
                            .then_some((*head, second, name, third))
                    })
                })
            })
        })?;
        let colon = third.end;
        self.cap(1, p, head.end);
        self.put_head(p, &RANGE, &head);
        self.put_gap(37, GapForm::Parts, head.end, &second);
        self.cap(41, second.end, name);
        self.put_gap(42, GapForm::Parts, name, &third);
        self.cap(46, colon, colon + 1);
        Some(colon + 1)
    }
}

/// The head of `TypeAlias`.
/// It has no storage loop in it.
const ALIAS: Shape = Shape {
    storage: None,
    outer: Some(8),
    layout: Layout {
        attribute: Some(9),
        gap_a: 10,
        gap_b: 12,
        scope: 14,
        scope_template: (15, 16),
        gap_c: 17,
        template: 19,
        gap_form: GapForm::Plain,
    },
    pointer: Pointer {
        outer: 21,
        first: 22,
        each: 24,
        gap_form: GapForm::Plain,
    },
};

/// What follows the first iteration of a structured binding: more names, `]`
/// and the `:`.
struct Bindings {
    /// The last `,` and its two gaps, and the name after it.
    last: Option<(usize, usize, Gap, Gap, usize, usize)>,
    close_gap: (usize, Gap),
    close: usize,
    colon_gap: Gap,
    colon: usize,
}

impl Lexer<'_> {
    /// `(?:GAP(,)GAP(ID))*GAP(])GAP(:)(?!:)` from `pos`, the loop taking as
    /// many names as it can and giving them back one at a time.
    fn bindings(
        &self,
        pos: usize,
        last: Option<(usize, usize, Gap, Gap, usize, usize)>,
    ) -> Option<Bindings> {
        let _depth = self.deeper()?;
        for gap in self.gaps(pos) {
            if self.at(gap.end) != b',' || gap.end >= self.lim() {
                continue;
            }
            for next in self.gaps(gap.end + 1) {
                for name in self.ident_ends(next.end) {
                    let step = Some((pos, gap.end, gap, next, next.end, name));
                    if let Some(found) = self.bindings(name, step) {
                        return Some(found);
                    }
                }
            }
        }
        for close_gap in self.gaps(pos) {
            let close = close_gap.end;
            if self.at(close) != b']' || close >= self.lim() {
                continue;
            }
            for colon_gap in self.gaps(close + 1) {
                let colon = colon_gap.end;
                if self.at(colon) == b':' && colon < self.lim() && self.at(colon + 1) != b':' {
                    return Some(Bindings {
                        last,
                        close_gap: (pos, close_gap),
                        close,
                        colon_gap,
                        colon,
                    });
                }
            }
        }
        None
    }

    /// `(HEAD)GAP(\[)GAP(ID)(?:GAP(,)GAP(ID))*GAP(])GAP(:)(?!:)`: the
    /// structured binding of a range `for` loop.
    pub(super) fn range_for_binding(&mut self, p: usize) -> Option<usize> {
        let (head, second, third, first, found) = self.find_head(p, &RANGE, |lexer, head| {
            lexer.gaps(head.end).find_map(|second| {
                let open = second.end;
                if lexer.at(open) != b'[' || open >= lexer.lim() {
                    return None;
                }
                lexer.gaps(open + 1).find_map(|third| {
                    lexer.ident_ends(third.end).find_map(|first| {
                        let found = lexer.bindings(first, None)?;
                        Some((*head, second, third, first, found))
                    })
                })
            })
        })?;
        let open = second.end;
        self.cap(1, p, head.end);
        self.put_head(p, &RANGE, &head);
        self.put_gap(37, GapForm::Parts, head.end, &second);
        self.cap(41, open, open + 1);
        self.put_gap(42, GapForm::Parts, open + 1, &third);
        self.cap(46, third.end, first);
        if let Some((at, comma, gap_a, gap_b, id, id_end)) = found.last {
            self.put_gap(47, GapForm::Parts, at, &gap_a);
            self.cap(51, comma, comma + 1);
            self.put_gap(52, GapForm::Parts, comma + 1, &gap_b);
            self.cap(56, id, id_end);
        }
        self.put_gap(57, GapForm::Parts, found.close_gap.0, &found.close_gap.1);
        self.cap(61, found.close, found.close + 1);
        self.put_gap(62, GapForm::Parts, found.close + 1, &found.colon_gap);
        self.cap(66, found.colon, found.colon + 1);
        Some(found.colon + 1)
    }
}

/// The brackets of an alias, taken or not, and the end with its `;`: where
/// the match ends, the `[` with its `]`, and the `;`.
type Bracketed = (usize, Option<(usize, usize)>, Option<usize>);

/// How an alias ends after its target.
#[derive(Clone, Copy)]
struct Tail {
    end: usize,
    /// The pointer marks with the gap after them.
    pointer: Option<(Reference, Gap)>,
    /// The array brackets: the `[` and the `]`.
    brackets: Option<(usize, usize)>,
    semicolon: Option<usize>,
}

impl Lexer<'_> {
    /// `\s?` then a `;` or the line feed, from `at`: where the match ends and
    /// the `;`.
    /// The white space gives back what it took until one fits.
    fn alias_end(&self, at: usize) -> Option<(usize, Option<usize>)> {
        let full = self.skip_space(at);
        for q in (at..=full).rev() {
            if self.at(q) == b';' && q < self.lim() {
                return Some((q + 1, Some(q)));
            }
            if self.nl && q == self.lim() {
                return Some((q + 1, None));
            }
        }
        None
    }

    /// `(?:(\[)(\w*)(\])\s?)?\s?(?:(;)|\n)` from `pos`: the brackets, taken
    /// or not, and the end.
    fn alias_brackets(&self, pos: usize) -> Option<Bracketed> {
        if self.at(pos) == b'[' && pos < self.lim() {
            let word = self.word_end(pos + 1);
            if self.at(word) == b']'
                && word < self.lim()
                && let Some((end, semicolon)) = self.alias_end(self.skip_space(word + 1))
            {
                return Some((end, Some((pos, word)), semicolon));
            }
        }
        let (end, semicolon) = self.alias_end(pos)?;
        Some((end, None, semicolon))
    }

    /// How an alias ends after the type of `head`: its pointer marks and the
    /// gap after them if it has some, then the brackets and the `;`.
    fn alias_tail(&self, head: &Head) -> Option<Tail> {
        if let Some(run) = head.run {
            for gap in self.gaps(run.0) {
                if let Some((end, brackets, semicolon)) = self.alias_brackets(gap.end) {
                    return Some(Tail {
                        end,
                        pointer: Some((run, gap)),
                        brackets,
                        semicolon,
                    });
                }
            }
            return None;
        }
        let (end, brackets, semicolon) = self.alias_brackets(head.end)?;
        Some(Tail {
            end,
            pointer: None,
            brackets,
            semicolon,
        })
    }

    /// Records the groups of an alias tail whose type ended at `after`.
    fn put_tail(&mut self, after: usize, tail: &Tail) {
        if let Some((run, gap)) = &tail.pointer {
            self.cap(21, after, run.0);
            if let Some(first) = run.1 {
                self.put_gap(22, GapForm::Plain, after, &first);
            }
            if let Some((from, last)) = run.2 {
                self.put_gap(24, GapForm::Plain, from, &last);
            }
            self.put_gap(26, GapForm::Plain, run.0, gap);
        }
        if let Some((open, close)) = tail.brackets {
            self.cap(28, open, open + 1);
            self.cap(29, open + 1, close);
            self.cap(30, close, close + 1);
        }
        if let Some(q) = tail.semicolon {
            self.cap(31, q, q + 1);
        }
    }

    /// `(using)\s+(?!namespace)(ID)\s?(ATTR)?\s?(=)\s?(typename?)\s?(STORAGE)?`
    /// then a type or the rest of the line, and the end: an alias
    /// declaration.
    ///
    /// The white space after `=` gives back what it took, line feed first,
    /// until the rest fits.
    pub(super) fn type_alias(&mut self, p: usize) -> Option<usize> {
        let using = self.lit(p, b"using")?;
        let name_at = self.skip_space(using);
        if name_at == using || self.starts(name_at, b"namespace") {
            return None;
        }
        let lexer = &*self;
        let (
            name,
            after_name,
            attribute,
            equals,
            keyword_at,
            keyword,
            type_at,
            storage,
            at,
            end,
            target,
        ) = lexer.ident_ends(name_at).find_map(|name| {
            let after_name = lexer.skip_space(name);
            lexer
                .attribute_options(after_name)
                .into_iter()
                .find_map(|attribute| {
                    let equals = lexer.skip_space(attribute.unwrap_or(after_name));
                    if lexer.at(equals) != b'=' || equals >= lexer.lim() {
                        return None;
                    }
                    lexer
                        .alias_keywords(equals)
                        .find_map(|(keyword_at, keyword, type_at)| {
                            let with_storage =
                                lexer.find_storage_run(type_at, &mut |lexer, stored, iteration| {
                                    let (end, target) = lexer.alias_target(stored)?;
                                    Some((Some((stored, iteration)), stored, end, target))
                                });
                            let (storage, at, end, target) = with_storage.or_else(|| {
                                let (end, target) = lexer.alias_target(type_at)?;
                                Some((None, type_at, end, target))
                            })?;
                            Some((
                                name, after_name, attribute, equals, keyword_at, keyword, type_at,
                                storage, at, end, target,
                            ))
                        })
                })
        })?;
        self.cap(1, p, using);
        self.cap(2, name_at, name);
        if let Some(end) = attribute {
            self.cap(3, after_name, end);
        }
        self.cap(4, equals, equals + 1);
        self.cap(5, keyword_at, keyword.unwrap_or(keyword_at));
        if let Some((stored, iteration)) = storage {
            self.cap(6, type_at, stored);
            if iteration.gap.run {
                self.cap(7, iteration.from, iteration.gap.end);
            }
        }
        match target {
            Target::Type(head, tail) => {
                self.put_head(at, &ALIAS, &head);
                self.put_tail(head.type_end(), &tail);
            }
            Target::Text(text_end, semicolon) => {
                self.cap(20, at, text_end);
                if let Some(q) = semicolon {
                    self.cap(31, q, q + 1);
                }
            }
        }
        Some(end)
    }

    /// `\s?(typename)?\s?` after the `=` at `c`, every way: where `typename`
    /// would start, where it ends if it is there, and where the type starts.
    /// Each white space takes as much as it can.
    /// It gives it back one character at a time.
    fn alias_keywords(&self, c: usize) -> impl Iterator<Item = (usize, Option<usize>, usize)> + '_ {
        (c + 1..=self.skip_space(c + 1)).rev().flat_map(move |d| {
            self.lit(d, b"typename")
                .into_iter()
                .flat_map(move |e| {
                    (e..=self.skip_space(e))
                        .rev()
                        .map(move |after| (d, Some(e), after))
                })
                .chain(std::iter::once((d, None, d)))
        })
    }

    /// The target of an alias at `at`: a type with its tail.
    /// Otherwise it is the rest of the line up to its last character that is
    /// no `;`.
    fn alias_target(&self, at: usize) -> Option<(usize, Target)> {
        if let Some(found) = self.find_head(at, &ALIAS, |lexer, head| {
            let tail = lexer.alias_tail(head)?;
            Some((tail.end, Target::Type(*head, tail)))
        }) {
            return Some(found);
        }
        for text_end in (at..=self.lim()).rev() {
            if self.prev(text_end) == Some(b';') {
                continue;
            }
            if let Some((end, semicolon)) = self.alias_end(text_end) {
                return Some((end, Target::Text(text_end, semicolon)));
            }
        }
        None
    }
}

/// What an alias names.
#[expect(
    clippy::large_enum_variant,
    reason = "one is built per match, on the stack, instead of on the heap"
)]
enum Target {
    Type(Head, Tail),
    Text(usize, Option<usize>),
}
