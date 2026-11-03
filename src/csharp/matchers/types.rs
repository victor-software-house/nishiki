//! The grammar's `type_name` expression, which nearly every declaration
//! begins with, and the continuations that follow it.
//!
//! Oniguruma backtracks through the optional and repeated parts of the type
//! (a leading `ref`, an alias, type arguments, further dotted names, a
//! nullable mark, array suffixes) when what follows it fails. The matcher
//! offers each way the type can end to a continuation, in the order
//! Oniguruma tries them, and returns the first the continuation accepts.
//! Giving back the blanks that the last part took is not a separate way: it
//! ends at the same text, so a continuation sees the end of the last
//! character and the end of the blanks after it ([`Ty`]) and decides.

use super::super::engine::Lexer;
use super::super::limits::MAX_TYPE_PARTS;
use super::chars::Body;
use crate::rules::Chars;

/// The leading `ref` of a type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::csharp) enum Prefix {
    /// No prefix.
    None,
    /// `(?:ref\s+)?`
    Ref,
    /// `(?:ref\s+(?:readonly\s+)?)?`
    RefReadonly,
}

/// Whether a tuple type is one of the type's two forms, and if its body may
/// be empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::csharp) enum Tuple {
    /// There is no tuple form.
    None,
    /// `\s*\((?:[^()]|\g<tuple>)+\)`
    Plus,
    /// `\s*\((?:[^()]|\g<tuple>)*\)`
    Star,
}

/// What follows the type's name: the nullable mark and the array suffixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::csharp) enum Tail {
    /// `(?:\s*\?\s*)?(?:\s*\[(?:\s*,\s*)*]\s*\??\s*)*`
    Std,
    /// `(?:\s*\?(?!\?))?(?:\s*\[\s*(?:,\s*)*](?:\s*\?(?!\?))?)*`
    AsExpression,
    /// `(?:\s*\?)?(?:\s*\[\s*(?:,\s*)*](?:\s*\?)?)*`
    LocalFunction,
    /// Nothing.
    Bare,
}

/// The variants of the `type_name` expression the grammar spells out.
#[derive(Clone, Copy, Debug)]
pub(in crate::csharp) struct Shape {
    pub(in crate::csharp) prefix: Prefix,
    pub(in crate::csharp) tuple: Tuple,
    /// Whether the type arguments may be empty: `<(?:...)*>`.
    pub(in crate::csharp) args: Body,
    /// `(?:\s*\*\s*)*` before the nullable mark.
    pub(in crate::csharp) pointers: bool,
    pub(in crate::csharp) tail: Tail,
}

impl Shape {
    /// The type of a field, a tuple element, a cast and the like.
    pub(in crate::csharp) const STD: Shape = Shape {
        prefix: Prefix::None,
        tuple: Tuple::Plus,
        args: Body::NonEmpty,
        pointers: false,
        tail: Tail::Std,
    };
    /// The type of a member that may be a `ref` or `ref readonly` return.
    pub(in crate::csharp) const REF_READONLY: Shape = Shape {
        prefix: Prefix::RefReadonly,
        ..Shape::STD
    };
    /// The type of a parameter or a `foreach` variable.
    pub(in crate::csharp) const REF: Shape = Shape {
        prefix: Prefix::Ref,
        ..Shape::STD
    };
    /// The type of a local variable.
    pub(in crate::csharp) const LOCAL: Shape = Shape {
        prefix: Prefix::RefReadonly,
        pointers: true,
        ..Shape::STD
    };
    /// The type of an array creation.
    pub(in crate::csharp) const POINTER: Shape = Shape {
        pointers: true,
        ..Shape::STD
    };
    /// The type after `as`.
    pub(in crate::csharp) const AS: Shape = Shape {
        tail: Tail::AsExpression,
        ..Shape::STD
    };
    /// The return type of a local function.
    pub(in crate::csharp) const LOCAL_FUNCTION: Shape = Shape {
        prefix: Prefix::RefReadonly,
        tail: Tail::LocalFunction,
        ..Shape::STD
    };
    /// The type of an explicitly typed anonymous function's parameter.
    pub(in crate::csharp) const LOOSE: Shape = Shape {
        tuple: Tuple::Star,
        args: Body::MayBeEmpty,
        ..Shape::STD
    };
    /// The element type of a fixed-size buffer: a name and nothing else.
    pub(in crate::csharp) const NAME_ONLY: Shape = Shape {
        tuple: Tuple::None,
        tail: Tail::Bare,
        ..Shape::STD
    };
}

/// A way the type can end.
#[derive(Clone, Copy, Debug)]
pub(in crate::csharp) struct Ty {
    pub(in crate::csharp) start: usize,
    /// The end of the last character that is not a blank.
    pub(in crate::csharp) core: usize,
    /// The end of the blanks the last part of the type took.
    pub(in crate::csharp) end: usize,
}

/// What the type's continuation is.
pub(in crate::csharp) type Cont<'k, 'a> = &'k mut dyn FnMut(&mut Lexer<'a>, Ty) -> Option<usize>;

impl<'a> Lexer<'a> {
    /// Matches the type at `p` and returns what the first way it ends that
    /// `k` accepts returns.
    pub(in crate::csharp) fn type_name(
        &mut self,
        p: usize,
        shape: Shape,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        let with_ref = match shape.prefix {
            Prefix::None => None,
            Prefix::Ref | Prefix::RefReadonly => self.lit(p, b"ref").and_then(|e| self.blanks1(e)),
        };
        if let Some(after_ref) = with_ref {
            if shape.prefix == Prefix::RefReadonly {
                let readonly = self
                    .lit(after_ref, b"readonly")
                    .and_then(|e| self.blanks1(e));
                if let Some(q) = readonly
                    && let found @ Some(_) = self.body(p, q, shape, k)
                {
                    return found;
                }
            }
            if let found @ Some(_) = self.body(p, after_ref, shape, k) {
                return found;
            }
        }
        self.body(p, p, shape, k)
    }

    /// The two forms of the type: a dotted name, or a tuple.
    fn body(&mut self, start: usize, q: usize, shape: Shape, k: Cont<'_, 'a>) -> Option<usize> {
        if let Some(found) = self.alias_and_names(start, q, shape, k) {
            return Some(found);
        }
        if shape.tuple == Tuple::None {
            return None;
        }
        let open = self.blanks(q);
        let body = match shape.tuple {
            Tuple::Star => Body::MayBeEmpty,
            Tuple::Plus | Tuple::None => Body::NonEmpty,
        };
        let end = self.balanced(open, b'(', b')', body)?;
        self.pointers(start, end, end, shape, 1, k)
    }

    /// `(?:ID\s*::\s*)?` and the names after it.
    fn alias_and_names(
        &mut self,
        start: usize,
        q: usize,
        shape: Shape,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        if let Some(after_alias) = self
            .ident(q)
            .map(|e| self.blanks(e))
            .and_then(|w| self.lit(w, b"::").map(|c| self.blanks(c)))
            && let found @ Some(_) = self.name(start, after_alias, shape, 1, k)
        {
            return found;
        }
        self.name(start, q, shape, 1, k)
    }

    /// One name with its type arguments, and the dotted names after it;
    /// `parts` counts the names, pointer marks and array suffixes so far.
    fn name(
        &mut self,
        start: usize,
        at: usize,
        shape: Shape,
        parts: u32,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        let id_end = self.ident(at)?;
        let w = self.blanks(id_end);
        if let Some(close) = self.balanced(w, b'<', b'>', shape.args) {
            let end = self.blanks(close);
            if let found @ Some(_) = self.after_name(start, close, end, shape, parts, k) {
                return found;
            }
        }
        self.after_name(start, id_end, w, shape, parts, k)
    }

    /// `(?:\s*\.\s*NAT)*` greedily, then what follows the names.
    fn after_name(
        &mut self,
        start: usize,
        core: usize,
        end: usize,
        shape: Shape,
        parts: u32,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        if parts < MAX_TYPE_PARTS && self.at(end) == b'.' && end < self.lim() {
            let next = self.blanks(end + 1);
            if let found @ Some(_) = self.name(start, next, shape, parts + 1, k) {
                return found;
            }
        }
        self.pointers(start, core, end, shape, parts, k)
    }

    /// `(?:\s*\*\s*)*`, greedily.
    ///
    /// Each suffix matcher reads its leading `\s*` from `end`: after a name
    /// `end` already follows its blanks, while a tuple, which the grammar
    /// does not end with `\s*`, ends at its `)`.
    fn pointers(
        &mut self,
        start: usize,
        core: usize,
        end: usize,
        shape: Shape,
        parts: u32,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        let look = self.blanks(end);
        if shape.pointers && parts < MAX_TYPE_PARTS && self.at(look) == b'*' && look < self.lim() {
            let next = self.blanks(look + 1);
            if let found @ Some(_) = self.pointers(start, look + 1, next, shape, parts + 1, k) {
                return found;
            }
        }
        self.nullable(start, core, end, shape, parts, k)
    }

    /// The nullable mark.
    fn nullable(
        &mut self,
        start: usize,
        core: usize,
        end: usize,
        shape: Shape,
        parts: u32,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        let look = self.blanks(end);
        let mark = self.at(look) == b'?' && look < self.lim();
        match shape.tail {
            Tail::Bare => return k(self, Ty { start, core, end }),
            Tail::Std if mark => {
                let next = self.blanks(look + 1);
                if let found @ Some(_) = self.arrays(start, look + 1, next, shape, parts, k) {
                    return found;
                }
            }
            Tail::AsExpression if mark && self.at(look + 1) != b'?' => {
                if let found @ Some(_) = self.arrays(start, look + 1, look + 1, shape, parts, k) {
                    return found;
                }
            }
            Tail::LocalFunction if mark => {
                if let found @ Some(_) = self.arrays(start, look + 1, look + 1, shape, parts, k) {
                    return found;
                }
            }
            Tail::Std | Tail::AsExpression | Tail::LocalFunction => {}
        }
        self.arrays(start, core, end, shape, parts, k)
    }

    /// The array suffixes, greedily.
    ///
    /// The standard tail reads `\[(?:\s*,\s*)*]` and the others
    /// `\[\s*(?:,\s*)*]`.
    fn arrays(
        &mut self,
        start: usize,
        core: usize,
        end: usize,
        shape: Shape,
        parts: u32,
        k: Cont<'_, 'a>,
    ) -> Option<usize> {
        let look = self.blanks(end);
        if parts < MAX_TYPE_PARTS && self.at(look) == b'[' && look < self.lim() {
            let parts = parts + 1;
            let std = shape.tail == Tail::Std;
            let mut i = if std { look + 1 } else { self.blanks(look + 1) };
            loop {
                let j = if std { self.blanks(i) } else { i };
                if self.at(j) == b',' && j < self.lim() {
                    i = self.blanks(j + 1);
                } else {
                    break;
                }
            }
            if self.at(i) == b']' && i < self.lim() {
                let close = i + 1;
                if std {
                    let w = self.blanks(close);
                    if self.at(w) == b'?' && w < self.lim() {
                        let next = self.blanks(w + 1);
                        if let found @ Some(_) = self.arrays(start, w + 1, next, shape, parts, k) {
                            return found;
                        }
                    }
                    if let found @ Some(_) = self.arrays(start, close, w, shape, parts, k) {
                        return found;
                    }
                } else {
                    let w = self.blanks(close);
                    let mark = self.at(w) == b'?'
                        && w < self.lim()
                        && (shape.tail == Tail::LocalFunction || self.at(w + 1) != b'?');
                    if mark
                        && let found @ Some(_) = self.arrays(start, w + 1, w + 1, shape, parts, k)
                    {
                        return found;
                    }
                    if let found @ Some(_) = self.arrays(start, close, close, shape, parts, k) {
                        return found;
                    }
                }
            }
        }
        k(self, Ty { start, core, end })
    }

    /// `\s+` after a type that ended as `ty`: the end of the blanks, and
    /// where the type's capture group ends. The blanks that the type's last
    /// part took stand for the `\s+`, as Oniguruma gives one of them back,
    /// so the group ends one blank early.
    pub(in crate::csharp) fn plus_blanks(&self, ty: Ty) -> Option<(usize, usize)> {
        if ty.end > ty.core {
            let width = self.char_before(ty.end).map_or(1, char::len_utf8);
            Some((ty.end, ty.end - width))
        } else {
            self.blanks1(ty.end).map(|after| (after, ty.end))
        }
    }
}
