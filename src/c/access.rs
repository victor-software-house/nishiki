//! Member access, calls, function definitions and the brackets around them.

use super::engine::{Lexer, Retok, Run};
use super::scope::S;
use super::state::R;
use super::wordset;

/// The punctuation a call-like begin gives its `(`.
#[derive(Clone, Copy, Debug)]
pub(super) enum CallKind {
    /// A call in a function-call context.
    Args,
    /// The parameters of a function definition.
    Params,
}

/// What the last name of a member access chain is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Last {
    /// A method, followed by its `(`.
    Method,
    /// A member, not followed by `(`.
    Member,
}

/// The pieces of `obj.chain.name`, up to the last name.
struct Chain {
    /// The end of the first object and the white space after it.
    object_end: usize,
    /// Where the first operator starts and ends, and whether it is `->`.
    op: (usize, usize, bool),
    /// The repeated objects between the operator and the last name.
    middle: (usize, usize),
    /// The last name.
    name: (usize, usize),
}

impl Lexer<'_> {
    /// `[.]\*?` or `->\*?` at `i`: where it ends, and whether it is the arrow.
    fn access_op(&self, i: usize) -> Option<(usize, bool)> {
        if i >= self.len_all() {
            return None;
        }
        let (len, arrow) = match self.b(i) {
            b'.' => (1, false),
            b'-' if self.b(i + 1) == b'>' => (2, true),
            _ => return None,
        };
        let star = self.b(i + len) == b'*' && i + len < self.len_all();
        Some((i + len + usize::from(star), arrow))
    }

    /// `((?:[A-Za-z_]\w*|(?<=[\])]))\s*)`: where the object and the white
    /// space after it end.
    fn access_object(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.ident_end(p) {
            return Some(self.skip_space(end));
        }
        matches!(self.prev(p), Some(b']' | b')')).then(|| self.skip_space(p))
    }

    /// `((?:[A-Za-z_]\w*\s*(?:\.\*?|->\*?)\s*)*)`: where the chain after the
    /// first operator ends after each repetition it can take, the fewest
    /// first.
    fn access_reps(&self, from: usize, ends: &mut Vec<usize>) {
        let mut i = from;
        ends.clear();
        ends.push(i);
        while let Some(name_end) = self.ident_end(i) {
            let after = self.skip_space(name_end);
            let Some((op_end, _)) = self.access_op(after) else {
                break;
            };
            i = self.skip_space(op_end);
            ends.push(i);
        }
    }

    /// The chain at `p` that ends in a name of the kind `last`.
    fn access_chain(&mut self, p: usize, last: Last) -> Option<Chain> {
        let object_end = self.access_object(p)?;
        let (op_end, _) = self.access_op(object_end)?;
        let arrow = self.b(object_end) == b'-';
        let mut ends = std::mem::take(&mut self.ends);
        self.access_reps(op_end, &mut ends);
        let mut found = None;
        for &end in ends.iter().rev() {
            let name_start = self.skip_space(end);
            let name_end = match last {
                Last::Method => self.method_name(name_start),
                Last::Member => self.member_name(name_start),
            };
            if let Some(name_end) = name_end {
                found = Some(Chain {
                    object_end,
                    op: (object_end, op_end, arrow),
                    middle: (op_end, end),
                    name: (name_start, name_end),
                });
                break;
            }
        }
        self.ends = ends;
        found
    }

    /// The end of a final identifier of a method call: `([A-Za-z_]\w*)(\()`.
    fn method_name(&self, q: usize) -> Option<usize> {
        let end = self.ident_end(q)?;
        (self.b(end) == b'(' && end < self.len_all()).then_some(end)
    }

    /// The final identifier of a member access:
    /// `\b((?!(?:types)\b)[A-Z_a-z]\w*\b(?!\())`.
    fn member_name(&self, q: usize) -> Option<usize> {
        if !self.boundary(q) {
            return None;
        }
        let end = self.ident_end(q)?;
        if wordset::of(self.text(q, end)) & wordset::MEMBER_EXCL != 0 {
            return None;
        }
        (self.b(end) != b'(' || end >= self.len_all()).then_some(end)
    }

    fn push_chain(&mut self, p: usize, chain: &Chain) {
        let (op_start, op_end, arrow) = chain.op;
        self.cap(p, chain.object_end, &[S::VariableObjectAccess]);
        let scope = if arrow {
            &[S::PunctPointerAccess]
        } else {
            &[S::PunctDotAccess]
        };
        self.cap(op_start, op_end, scope);
        self.cap_retok(chain.middle.0, chain.middle.1, &[], Retok::Access);
    }

    /// The begin of a method call, `object.method(`.
    pub(super) fn method_access(&mut self, p: usize) -> bool {
        let Some(chain) = self.access_chain(p, Last::Method) else {
            return false;
        };
        self.push_chain(p, &chain);
        let (a, b) = chain.name;
        self.cap(a, b, &[S::EntityFunctionMember]);
        self.cap(b, b + 1, &[S::PunctArgsBeginMember]);
        self.begin(p, b + 1, R::Method)
    }

    /// `object.member`.
    pub(super) fn member_access(&mut self, p: usize) -> bool {
        let Some(chain) = self.access_chain(p, Last::Member) else {
            return false;
        };
        self.push_chain(p, &chain);
        let (a, b) = chain.name;
        self.cap(a, b, &[S::VariableMember]);
        self.plain(p, b, &[])
    }

    /// `object.`: the object and its operator.
    pub(super) fn plain_access(&mut self, p: usize) -> bool {
        let Some(object_end) = self.access_object(p) else {
            return false;
        };
        let Some((op_end, arrow)) = self.access_op(object_end) else {
            return false;
        };
        self.cap(p, object_end, &[S::VariableObjectAccess]);
        let scope = if arrow {
            &[S::PunctPointerAccess]
        } else {
            &[S::PunctDotAccess]
        };
        self.cap(object_end, op_end, scope);
        self.plain(p, op_end, &[])
    }

    /// `(?<=operator)(?:[-!&*+<=>]+|\(\)|\[])` at `i`: where it ends.
    fn operator_name(&self, i: usize) -> Option<usize> {
        if i < 8 || self.text(i - 8, i) != b"operator" {
            return None;
        }
        let mut j = i;
        while matches!(
            self.b(j),
            b'-' | b'!' | b'&' | b'*' | b'+' | b'<' | b'=' | b'>'
        ) && j < self.len_all()
        {
            j += 1;
        }
        self.spend(j - i);
        if j > i {
            return Some(j);
        }
        if self.starts(i, b"()") || self.starts(i, b"[]") {
            return Some(i + 2);
        }
        None
    }

    /// `(?:[A-Za-z_][0-9A-Za-z_]*+|::)++` at `i`: where the run ends.
    ///
    /// The call patterns ask at every byte of a run, so a walk past a plain
    /// name is cached. A run of colons takes its pairs from wherever it is
    /// entered, so it passes when an even number of colons is left and stops
    /// before the last one otherwise. Every position a step lands on after the
    /// first starts a name or a run of colons, which the cached walk, if it
    /// covers that position, also lands on: the search then ends where the
    /// cached one did.
    fn scoped_name(&self, i: usize) -> Option<usize> {
        let mut j = i;
        if let Some(end) = self.ascii_ident_end(i) {
            if self.b(end) != b':' {
                return Some(end);
            }
            j = end;
        } else if self.b(i) != b':' {
            return None;
        }
        let walk = self.scoped.get();
        loop {
            if let Some(end) = self.ascii_ident_end(j) {
                j = end;
            } else if self.b(j) == b':' && j < self.len_all() {
                let end = self.colons_end(j);
                if (end - j) % 2 == 1 {
                    j = end - 1;
                    break;
                }
                j = end;
            } else {
                break;
            }
            if walk.holds(j) {
                j = walk.end;
                break;
            }
        }
        if j == i {
            return None;
        }
        self.scoped.set(Run { start: i, end: j });
        Some(j)
    }

    /// Whether a name, or an operator name, and a `(` start at `p`: the
    /// part of a call that does not depend on the words it excludes.
    pub(super) fn call_shape(&self, p: usize) -> bool {
        let Some(end) = self.scoped_name(p).or_else(|| self.operator_name(p)) else {
            return false;
        };
        let paren = self.skip_space(end);
        self.b(paren) == b'(' && paren < self.len_all()
    }

    /// The name of a call at `p` and where its `(` is, with `exclude` the
    /// word lists whose words may not be called: `(?!(?:…)\s*\()(name)\s*(\()`.
    pub(super) fn call_target(&self, p: usize, exclude: u32) -> Option<(usize, usize)> {
        let (name_end, paren) = if let Some(end) = self.scoped_name(p) {
            (end, self.skip_space(end))
        } else {
            let end = self.operator_name(p)?;
            (end, self.skip_space(end))
        };
        if !(self.b(paren) == b'(' && paren < self.len_all()) {
            return None;
        }
        if let Some(word_end) = self.ascii_ident_end(p) {
            let after = self.skip_space(word_end);
            if self.b(after) == b'('
                && after < self.len_all()
                && wordset::of(self.text(p, word_end)) & exclude != 0
            {
                return None;
            }
        }
        Some((name_end, paren))
    }

    /// A call at `p` that opens `rule`, with the `(` scoped `punct`.
    pub(super) fn call_like(&mut self, p: usize, rule: R, kind: CallKind) -> bool {
        let Some((name_end, paren)) = self.call_target(p, wordset::CALL_EXCL) else {
            return false;
        };
        let scope = match kind {
            CallKind::Args => &[S::PunctArgsBegin],
            CallKind::Params => &[S::PunctParamBegin],
        };
        self.cap(p, name_end, &[S::EntityFunction]);
        self.cap(paren, paren + 1, scope);
        self.begin(p, paren + 1, rule)
    }

    /// The lookahead that starts a call: `meta.function-call.c`.
    pub(super) fn function_call(&mut self, p: usize) -> bool {
        self.call_target(p, wordset::CALL_EXCL).is_some() && self.begin(p, p, R::FuncCall)
    }

    /// `(?<!\w)(?!\s*(?:…)\s*\()(?=[A-Za-z_]\w*\s*\()`: a function
    /// definition or declaration.
    pub(super) fn func_def(&mut self, p: usize) -> bool {
        if self.word_before(p) {
            return false;
        }
        let Some(end) = self.ident_end(p) else {
            return false;
        };
        let after = self.skip_space(end);
        if !(self.b(after) == b'(' && after < self.len_all()) {
            return false;
        }
        if wordset::of(self.text(p, end)) & wordset::FUNCDEF_EXCL != 0 {
            return false;
        }
        self.begin(p, p, R::FuncDef)
    }

    /// `(?<=[0-9A-Z_a-z] |[]&)*>])\s*([A-Z_a-z]\w*)\s*(?=(?:\[]\s*)?[),])`.
    pub(super) fn probably(&mut self, p: usize) -> bool {
        let lead = match self.prev(p) {
            Some(b']' | b'&' | b')' | b'*' | b'>') => true,
            Some(b' ') => {
                p >= 2 && matches!(self.b(p - 2), b'0'..=b'9' | b'A'..=b'Z' | b'_' | b'a'..=b'z')
            }
            _ => false,
        };
        if !lead {
            return false;
        }
        let q = self.skip_space(p);
        let Some(end) = self.ident_end(q) else {
            return false;
        };
        let r = self.skip_space(end);
        let mut look = r;
        if self.starts(look, b"[]") {
            look = self.skip_space(look + 2);
        }
        if !(matches!(self.b(look), b')' | b',') && look < self.len_all()) {
            return false;
        }
        self.cap(q, end, &[S::VariableParamProbably]);
        self.plain(p, r, &[])
    }

    /// `([A-Z_a-z][0-9A-Z_a-z]*|(?<=[])]))?(\[)(?!])`: the start of an
    /// indexing.
    pub(super) fn bracket_begin(&mut self, p: usize) -> bool {
        if let Some(end) = self.ascii_ident_end(p)
            && self.b(end) == b'['
            && end < self.len_all()
            && self.b(end + 1) != b']'
        {
            self.cap(p, end, &[S::VariableObject]);
            self.cap(end, end + 1, &[S::PunctBeginSquare]);
            return self.begin(p, end + 1, R::Bracket);
        }
        if self.b(p) == b'[' && p < self.len_all() && self.b(p + 1) != b']' {
            self.cap(p, p + 1, &[S::PunctBeginSquare]);
            return self.begin(p, p + 1, R::Bracket);
        }
        false
    }

    /// `\[\s*]`.
    pub(super) fn bracket_empty(&mut self, p: usize) -> bool {
        if self.b(p) != b'[' || p >= self.len_all() {
            return false;
        }
        let q = self.skip_space(p + 1);
        self.b(q) == b']' && q < self.len_all() && self.plain(p, q + 1, &[S::StorageArray])
    }

    /// The begin of an initialization, `x or_eq(`: after a word and
    /// white space, a command word that the grammar reads as an operator and
    /// a call.
    pub(super) fn init_begin(&mut self, p: usize) -> bool {
        const WORDS: &[&str] = &[
            "and", "and_eq", "bitand", "bitor", "compl", "not", "not_eq", "or", "or_eq", "typeid",
            "xor", "xor_eq", "alignof", "alignas",
        ];
        if self.space_at(p) == 0 || !self.word_before(p) {
            return false;
        }
        for tail in ["else", "new", "return"] {
            if p >= tail.len() && self.text(p - tail.len(), p) == tail.as_bytes() {
                return false;
            }
        }
        let q = self.skip_space(p);
        for word in WORDS {
            if !self.starts(q, word.as_bytes()) {
                continue;
            }
            let g1_end = q + word.len();
            let Some(g2_end) = self.scoped_name(g1_end) else {
                continue;
            };
            let paren = self.skip_space(g2_end);
            if self.b(paren) != b'(' || paren >= self.len_all() {
                continue;
            }
            self.cap(q, g1_end, &[S::VariableOther]);
            self.cap(g1_end, g2_end, &[S::PunctParensBeginInit]);
            return self.begin(p, paren + 1, R::Init);
        }
        false
    }
}
