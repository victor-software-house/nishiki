//! The lookaheads the grammar decides rules with, written as scanners.
//!
//! The grammar spells each one as a regular expression of bracket groups
//! nested a fixed number of levels.
//! A scanner here accepts the same text, including the level at which the
//! regular expression gives up.
//! The rules then agree with Shiki on where an arrow function or a call with
//! type arguments starts.
//! Every scan stays within [`Lexer::window`].

use super::lexer::Lexer;

/// Which annotations make a name a function in [`Lexer::colon_function_ahead`].
#[derive(Clone, Copy, Debug)]
pub(crate) enum Annotation {
    /// A function type or `Function`.
    FunctionType,
    /// Those, or any type followed by a function-valued initialiser.
    OrValue,
}

/// The call lookahead that holds, as [`Lexer::call_lookahead`] reports it.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CallLookahead {
    /// A call on this line.
    Call,
    /// Type arguments that the line leaves open before the call.
    Open,
}
use super::limits::{GENERIC_NESTING, GRAMMAR_NESTING};

impl<const D: u8> Lexer<'_, D> {
    /// A balanced `open … close` group at `i`, nested at most
    /// [`GRAMMAR_NESTING`] levels: its end.
    pub(crate) fn balanced(&self, i: usize, open: u8, close: u8) -> Option<usize> {
        self.balanced_at(i, open, close, 1, self.window())
    }

    fn balanced_at(
        &self,
        i: usize,
        open: u8,
        close: u8,
        depth: usize,
        end: usize,
    ) -> Option<usize> {
        let mut j = i + 1;
        while j < end {
            self.spend(1);
            let c = self.byte(j);
            if c == close {
                return Some(j + 1);
            }
            if c == open {
                if depth >= GRAMMAR_NESTING {
                    return None;
                }
                j = self.balanced_at(j, open, close, depth + 1, end)?;
            } else {
                j += self.char_len(j);
            }
        }
        None
    }

    /// `(…)`, `{…}` or `[…]` at `i`: its end.
    pub(crate) fn bracket_group(&self, i: usize) -> Option<usize> {
        match self.byte(i) {
            b'(' => self.balanced(i, b'(', b')'),
            b'{' => self.balanced(i, b'{', b'}'),
            b'[' => self.balanced(i, b'[', b']'),
            _ => None,
        }
    }

    /// A quoted string or template literal at `i`, with backslash escapes and
    /// no nesting: its end.
    pub(crate) fn quoted(&self, i: usize) -> Option<usize> {
        let quote = self.byte(i);
        if !matches!(quote, b'\'' | b'"' | b'`') {
            return None;
        }
        let end = self.window();
        let mut j = i + 1;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'\\' => j += 1 + self.char_len(j + 1),
                c if c == quote => return Some(j + 1),
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// `/*…*/` at `i`: its end.
    pub(crate) fn block_comment(&self, i: usize) -> Option<usize> {
        if !self.starts(i, "/*") {
            return None;
        }
        let rest = self.bytes.get(i + 2..)?;
        let end = memchr::memmem::find(rest, b"*/");
        self.spend(end.map_or(rest.len(), |at| at + 2));
        end.map(|at| i + 2 + at + 2)
    }

    /// `\s*(/*…*/\s*)*`: skips blanks and block comments.
    pub(crate) fn ws_comments(&self, i: usize) -> usize {
        let mut j = self.ws(i);
        while let Some(e) = self.block_comment(j) {
            j = self.ws(e);
        }
        j
    }

    /// The first element of a type-parameter list: `(const\s+)?` and a name's
    /// first character, or a bracket group.
    fn type_parameters_first(&self, i: usize) -> Option<usize> {
        if self.starts(i, "const") {
            let j = self.ws(i + "const".len());
            if j > i + "const".len() && self.id_start(j) {
                return Some(j + self.char_len(j));
            }
        }
        if self.id_start(i) {
            return Some(i + self.char_len(i));
        }
        self.bracket_group(i)
    }

    /// A `<…>` group of the arrow-function lookahead at `i`, nested at most
    /// [`GRAMMAR_NESTING`] levels: its end after trailing blanks.
    pub(crate) fn arrow_type_parameters(&self, i: usize) -> Option<usize> {
        let e = self.arrow_type_parameters_group(i, 1, self.window())?;
        Some(self.ws(e))
    }

    fn arrow_type_parameters_group(&self, i: usize, depth: usize, end: usize) -> Option<usize> {
        if self.byte(i) != b'<' {
            return None;
        }
        let mut j = self.type_parameters_first(self.ws(i + 1))?;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'>' => return Some(j + 1),
                b'<' => {
                    if depth >= GRAMMAR_NESTING {
                        return None;
                    }
                    j = self.arrow_type_parameters_group(j, depth + 1, end)?;
                }
                b'=' => {
                    if self.byte(j + 1) == b'<' || j + 1 > self.len {
                        return None;
                    }
                    j += 1 + self.char_len(j + 1);
                }
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// `(<…>\s*)?\(`: the position after the `(`.
    pub(crate) fn type_parameters_paren(&self, i: usize) -> Option<usize> {
        let j = if self.byte(i) == b'<' {
            self.arrow_type_parameters(i)?
        } else {
            i
        };
        (self.byte(j) == b'(').then_some(j + 1)
    }

    /// `[^<>]` items and `<…>` groups up to `>`, as `<[^<>]*>` does.
    fn flat_angle(&self, i: usize) -> Option<usize> {
        let end = self.window();
        let mut j = i + 1;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'>' => return Some(j + 1),
                b'<' => return None,
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// `<([^<>]|<([^<>]|<[^<>]+>)+>)+>` at `i`: its end.
    fn type_angle(&self, i: usize) -> Option<usize> {
        self.type_angle_at(i, 1)
    }

    fn type_angle_at(&self, i: usize, depth: usize) -> Option<usize> {
        let end = self.window();
        let mut j = i + 1;
        let mut items = 0;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'>' => return (items > 0).then_some(j + 1),
                b'<' => {
                    j = if depth >= GRAMMAR_NESTING {
                        return None;
                    } else if depth + 1 == GRAMMAR_NESTING {
                        let e = self.flat_angle(j)?;
                        if e == j + 2 {
                            return None;
                        }
                        e
                    } else {
                        self.type_angle_at(j, depth + 1)?
                    };
                    items += 1;
                }
                _ => {
                    j += self.char_len(j);
                    items += 1;
                }
            }
        }
        None
    }

    /// `\([^()]+\)` at `i`: its end.
    fn flat_paren(&self, i: usize) -> Option<usize> {
        let end = self.window();
        let mut j = i + 1;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b')' => return (j > i + 1).then_some(j + 1),
                b'(' => return None,
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// `\{[^{}]+}` at `i`: its end.
    fn flat_curly(&self, i: usize) -> Option<usize> {
        let end = self.window();
        let mut j = i + 1;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'}' => return (j > i + 1).then_some(j + 1),
                b'{' => return None,
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// The return type of an arrow function, `([^()<>{}]|<…>|\(…\)|\{…})+`, up
    /// to where the text stops being one: its end, with at least one item.
    fn arrow_return_items(&self, i: usize) -> Option<usize> {
        let end = self.window();
        let mut j = i;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'(' => j = self.flat_paren(j)?,
                b'{' => j = self.flat_curly(j)?,
                b'<' => j = self.type_angle(j)?,
                b')' | b'}' | b'>' => break,
                _ => j += self.char_len(j),
            }
        }
        (j > i).then_some(j)
    }

    /// `(\s*:\s*RETURN)?\s*=>` after the closing parenthesis at `i`.
    ///
    /// The type's last item may be the `=` of `=>`: it gives it back.
    fn arrow_tail(&self, i: usize) -> bool {
        let j = self.ws(i);
        if self.byte(j) == b':' {
            let start = self.ws(j + 1);
            let Some(end) = self.arrow_return_items(start) else {
                return false;
            };
            return self.byte(end) == b'>' && end > start && self.byte(end - 1) == b'=';
        }
        self.starts(j, "=>")
    }

    /// The first form of a parameter in an arrow function lookahead: `()` then
    /// `:`, or a name then `:`, after the `(` that ends at `i`.
    fn arrow_typed(&self, i: usize) -> bool {
        let j = self.ws_comments(i);
        if self.byte(j) == b')' {
            return self.byte(self.ws(j + 1)) == b':';
        }
        let mut k = j;
        if self.starts(k, "...") {
            k = self.ws(k + 3);
        }
        self.ident(k).is_some_and(|e| self.byte(self.ws(e)) == b':')
    }

    /// The second form: any parameter list closed by `)` then the arrow, after
    /// the `(` that ends at `i`.
    fn arrow_untyped(&self, i: usize) -> bool {
        let mut j = self.ws_comments(i);
        let end = self.window();
        let first = if self.id_start(j) {
            Some(j + self.char_len(j))
        } else if matches!(self.byte(j), b'{' | b'[') {
            self.bracket_group(j)
        } else if self.starts(j, "...") {
            let k = self.ws(j + 3);
            self.id_start(k).then(|| k + self.char_len(k))
        } else {
            None
        };
        if let Some(f) = first {
            j = f;
            while j < end {
                self.spend(1);
                match self.byte(j) {
                    b'"' | b'\'' | b'`' => match self.quoted(j) {
                        Some(e) => j = e,
                        None => return false,
                    },
                    b'(' => match self.balanced(j, b'(', b')') {
                        Some(e) => j = e,
                        None => return false,
                    },
                    b')' => break,
                    _ => j += self.char_len(j),
                }
            }
        }
        self.byte(j) == b')' && self.arrow_tail(j + 1)
    }

    /// Whether an arrow function's parameters and `=>` start at `i`, as
    /// `(<…>)?\(` followed by either typed or untyped parameters.
    pub(crate) fn arrow_ahead(&self, i: usize) -> bool {
        self.type_parameters_paren(i)
            .is_some_and(|j| self.arrow_typed(j) || self.arrow_untyped(j))
            || (Self::DIALECT.jsx() && self.generic_extends_ahead(i))
    }

    /// `<\s*ID\s+extends\s*[^=>]`: a type parameter list that cannot be a
    /// tag.
    /// The JSX grammars add it to the arrow function lookahead.
    fn generic_extends_ahead(&self, i: usize) -> bool {
        if self.byte(i) != b'<' {
            return false;
        }
        let Some(name_end) = self.ident(self.ws(i + 1)) else {
            return false;
        };
        let after = self.ws(name_end);
        after > name_end
            && self.starts(after, "extends")
            && !matches!(self.byte(after + "extends".len()), b'=' | b'>')
    }

    /// An arrow function's return type, of at least one item, followed by
    /// `=>`: the text after the colon.
    pub(crate) fn arrow_return_ahead(&self, i: usize) -> bool {
        self.arrow_return_items(i)
            .is_some_and(|end| self.byte(end) == b'>' && end > i + 1 && self.byte(end - 1) == b'=')
    }

    /// The first form only: `(<…>)?\(` then `)` or a name, then `:`.
    pub(crate) fn arrow_typed_ahead(&self, i: usize) -> bool {
        self.type_parameters_paren(i)
            .is_some_and(|j| self.arrow_typed(j))
    }

    /// Whether a function-valued initialiser starts at `i`, as the grammar
    /// tests after `=` or `:`.
    /// It is a function expression, or an arrow function whose parameters are
    /// seen, or whose line ends inside them.
    ///
    /// The forms are `(async\s+)?` then `function` or `name =>`, and
    /// `(async\s*)?` then an opening, a typed arrow or an arrow.
    pub(crate) fn function_value_ahead(&self, i: usize) -> bool {
        let async_end = self.starts(i, "async").then(|| i + "async".len());
        let after_async = async_end
            .filter(|&a| self.space_at(a) || a >= self.len)
            .map(|a| self.ws(a));
        for j in std::iter::once(i).chain(after_async) {
            if self.function_keyword_head(j)
                || self.ident(j).is_some_and(|e| self.starts(self.ws(e), "=>"))
            {
                return true;
            }
        }
        std::iter::once(i)
            .chain(async_end.map(|a| self.ws(a)))
            .any(|j| {
                self.function_open_ahead(j) || self.arrow_typed_ahead(j) || self.arrow_ahead(j)
            })
    }

    /// `function\s*[(*<]` or `function\s+` at `i`.
    fn function_keyword_head(&self, i: usize) -> bool {
        if !self.starts(i, "function") {
            return false;
        }
        let e = i + "function".len();
        let j = self.ws(e);
        j > e || matches!(self.byte(j), b'(' | b'*' | b'<')
    }

    /// `(<\s*)$`, or `(<…>)?\(` followed by a line end or by a destructured
    /// parameter that the line leaves open or assigns.
    ///
    /// The JSX grammars drop the type arguments before the parenthesis, which
    /// a tag would otherwise be read as.
    pub(crate) fn function_open_ahead(&self, i: usize) -> bool {
        (self.byte(i) == b'<' && self.eol(self.ws(i + 1)))
            || if Self::DIALECT.jsx() {
                self.byte(i) == b'(' && self.paren_open_rest(i + 1)
            } else {
                self.paren_open_ahead(i)
            }
    }

    /// `(<…>)?\(` followed by a line end or by a destructured parameter that
    /// the line leaves open or assigns.
    pub(crate) fn paren_open_ahead(&self, i: usize) -> bool {
        self.type_parameters_paren(i)
            .is_some_and(|j| self.paren_open_rest(j))
    }

    /// What follows the `(` that ends at `j`: a line end, or a destructured
    /// parameter that the line leaves open or assigns.
    fn paren_open_rest(&self, j: usize) -> bool {
        let m = self.ws(j);
        if self.eol(m) {
            return true;
        }
        if matches!(self.byte(m), b'[' | b'{') && self.eol(self.ws(m + 1)) {
            return true;
        }
        match self.byte(m) {
            b'{' => self
                .balanced(m, b'{', b'}')
                .is_some_and(|e| self.parameter_default_tail(e, b'{')),
            b'[' => self
                .balanced(m, b'[', b']')
                .is_some_and(|e| self.parameter_default_tail(e, b'[')),
            _ => false,
        }
    }

    /// After a destructured parameter: `\s*((:\s*\{?)$|((\s*TYPE\s*)?=\s*))`.
    pub(crate) fn parameter_default_tail(&self, e: usize, open: u8) -> bool {
        let j = self.ws(e);
        if self.byte(j) == b':' {
            let mut k = self.ws(j + 1);
            if self.byte(k) == open {
                k = self.ws(k + 1);
            }
            if self.eol(k) {
                return true;
            }
        }
        self.type_then_equals(j)
    }

    /// `(\s*TYPE+\s*)?=`: an `=` reached over type text.
    fn type_then_equals(&self, i: usize) -> bool {
        let end = self.window();
        let mut j = i;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'=' => return true,
                b'(' => match self.flat_paren(j) {
                    Some(e) => j = e,
                    None => return false,
                },
                b'{' => match self.flat_curly(j) {
                    Some(e) => j = e,
                    None => return false,
                },
                b'<' => match self.type_angle(j) {
                    Some(e) => j = e,
                    None => return false,
                },
                b')' | b'}' | b'>' => return false,
                _ => j += self.char_len(j),
            }
        }
        false
    }

    /// The type-annotation forms of a function-valued name, at the `:` at
    /// `i`.
    /// They are a function type, `Function`, and with [`Annotation::OrValue`]
    /// a type followed by `=` and a function-valued initialiser.
    ///
    /// A function type: `<` or `(` and a parameter.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn colon_function_ahead(&self, i: usize, forms: Annotation) -> bool {
        if self.byte(i) != b':' {
            return false;
        }
        let j = self.ws(i + 1);
        if self.byte(j) == b'<' {
            return true;
        }
        if self.byte(j) == b'(' {
            let k = self.ws(j + 1);
            if self.byte(k) == b')' || self.starts(k, "...") {
                return true;
            }
            let mut e = k;
            while self.alnum_at(e) {
                e += self.char_len(e);
            }
            if e > k {
                let m = self.ws(e);
                if matches!(self.byte(m), b',' | b':' | b'=' | b'?')
                    || (self.byte(m) == b')' && self.starts(self.ws(m + 1), "=>"))
                {
                    return true;
                }
            }
        }
        if self.starts(j, "Function")
            && self.keyword_before(j)
            && self.keyword_after(j + "Function".len())
        {
            return true;
        }
        if self.function_open_ahead(j) {
            return true;
        }
        match forms {
            Annotation::FunctionType => false,
            Annotation::OrValue => self.type_then_function_value(j),
        }
    }

    /// `(=>|\(…\)|<[^<>]*>|[^(),<=>])+=\s*FUNCTION`.
    fn type_then_function_value(&self, i: usize) -> bool {
        let end = self.window();
        let mut j = i;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'=' if self.byte(j + 1) == b'>' => j += 2,
                b'=' => {
                    return j > i && self.function_value_ahead(self.ws(j + 1));
                }
                b'(' => match self.balanced(j, b'(', b')') {
                    Some(e) => j = e,
                    None => return false,
                },
                b'<' => match self.flat_angle(j) {
                    Some(e) => j = e,
                    None => return false,
                },
                b')' | b',' | b'>' => return false,
                _ => j += self.char_len(j),
            }
        }
        false
    }

    /// `=\s*FUNCTION` at `i`, or a type-annotated form: the lookahead of a
    /// name the grammar scopes as a function.
    /// `eq_ws` is the position of the `\s*` that precedes `=`.
    pub(crate) fn named_function_ahead(&self, eq_ws: usize) -> bool {
        let j = self.ws(eq_ws);
        if self.byte(j) == b'=' && self.function_value_ahead(self.ws(j + 1)) {
            return true;
        }
        self.colon_function_ahead(eq_ws, Annotation::OrValue)
    }

    /// The first element of a type-argument list: `keyof `, `infer `,
    /// `typeof `, `readonly `, or a name, bracket group or string followed by
    /// something that continues a type.
    fn targ_first(&self, i: usize) -> Option<usize> {
        for word in ["keyof", "infer", "typeof", "readonly"] {
            if self.starts(i, word) {
                let e = i + word.len();
                let j = self.ws(e);
                if j > e {
                    return Some(j);
                }
            }
        }
        let e = self
            .ident(i)
            .or_else(|| self.bracket_group(i))
            .or_else(|| self.quoted(i))?;
        let k = self.ws(e);
        let ok = match self.byte(k) {
            b',' | b'.' | b'<' | b'>' | b'[' => true,
            b'=' => self.byte(k + 1) == b'>',
            b'&' => self.byte(k + 1) != b'&',
            b'|' => self.byte(k + 1) != b'|',
            _ => false,
        };
        ok.then_some(e)
    }

    /// A `<…>` type-argument group at `i`, as a call or tagged template
    /// spells it: its end after `>`.
    pub(crate) fn type_arguments_end(&self, i: usize) -> Option<usize> {
        self.type_arguments_end_at(i, 1, self.window())
    }

    fn type_arguments_end_at(&self, i: usize, depth: usize, end: usize) -> Option<usize> {
        if self.byte(i) != b'<' {
            return None;
        }
        let mut j = self.targ_first(self.ws(i + 1))?;
        while j < end {
            self.spend(1);
            match self.byte(j) {
                b'(' => j = self.balanced(j, b'(', b')')?,
                b'<' => {
                    if depth >= GENERIC_NESTING {
                        return None;
                    }
                    j = self.type_arguments_end_at(j, depth + 1, end)?;
                }
                b'>' => {
                    if j > 0 && self.bytes.get(j - 1) == Some(&b'=') {
                        j += 1;
                    } else {
                        return Some(j + 1);
                    }
                }
                _ => j += self.char_len(j),
            }
        }
        None
    }

    /// `\s*(?:(\?\.\s*)|(!))?(<…>\s*)?\(`: what must follow a call target.
    pub(crate) fn call_rest(&self, i: usize) -> bool {
        let mut j = self.ws(i);
        if self.starts(j, "?.") {
            j = self.ws(j + 2);
        } else if self.byte(j) == b'!' {
            j += 1;
        }
        if self.byte(j) == b'<' {
            match self.type_arguments_end(j) {
                Some(e) => j = self.ws(e),
                None => return false,
            }
        }
        self.byte(j) == b'('
    }

    /// The end of the longest `ID(\s*\??\.\s*#?ID)*` at `i`.
    ///
    /// The regular expression may backtrack to a shorter chain.
    /// Every shorter end is followed by `.` or `?.` and a name.
    /// Neither what follows a call target ([`Lexer::call_rest`]) nor an open
    /// `<` accepts that.
    /// Only the longest end can therefore match.
    fn chain_end(&self, i: usize) -> Option<usize> {
        let mut e = self.ident(i)?;
        let limit = self.window();
        while e < limit {
            self.spend(1);
            let mut j = self.ws(e);
            if self.byte(j) == b'?' {
                j += 1;
            }
            if self.byte(j) != b'.' {
                break;
            }
            j = self.ws(j + 1);
            match self.hash_ident(j) {
                Some(n) => e = n,
                None => break,
            }
        }
        Some(e)
    }

    /// `\??\.\s*#?ID` at `i`: its end.
    fn dot_member(&self, i: usize) -> Option<usize> {
        let mut j = i;
        if self.byte(j) == b'?' {
            j += 1;
        }
        if self.byte(j) != b'.' {
            return None;
        }
        self.hash_ident(self.ws(j + 1))
    }

    /// The call lookahead: a name chain, a `.member` or the end of a
    /// parenthesis, then what must follow a call target.
    pub(crate) fn call_ahead(&self, i: usize) -> bool {
        self.call_ahead_in(i, self.chain_end(i))
    }

    fn call_ahead_in(&self, i: usize, chain: Option<usize>) -> bool {
        chain.is_some_and(|e| self.call_rest(e))
            || self.dot_member(i).is_some_and(|e| self.call_rest(e))
            || (self.prev_char(i) == Some(')') && self.call_rest(i))
    }

    /// The call target alone, as the inner rule begins on it.
    pub(crate) fn call_target_start(&self, i: usize) -> bool {
        self.ident(i).is_some() || self.dot_member(i).is_some()
    }

    /// The same lookahead with type arguments that the line leaves open:
    /// `<` then `(`, `[` or `{` at the end of the line.
    pub(crate) fn call_open_ahead(&self, i: usize) -> bool {
        self.call_open_ahead_in(i, self.chain_end(i))
    }

    fn call_open_ahead_in(&self, i: usize, chain: Option<usize>) -> bool {
        let open_at = |e: usize| {
            self.byte(e) == b'<' && {
                let j = self.ws(e + 1);
                matches!(self.byte(j), b'(' | b'[' | b'{') && self.eol(self.ws(j + 1))
            }
        };
        chain.is_some_and(open_at)
            || self.dot_member(i).is_some_and(open_at)
            || (self.prev_char(i) == Some(')') && open_at(i))
    }

    /// Which call lookahead holds at `i`, if either.
    /// Both read the name chain, and it is scanned once.
    pub(crate) fn call_lookahead(&self, i: usize) -> Option<CallLookahead> {
        if !self.has_open_after(i) {
            return None;
        }
        let chain = self.chain_end(i);
        if self.call_ahead_in(i, chain) {
            Some(CallLookahead::Call)
        } else if self.call_open_ahead_in(i, chain) {
            Some(CallLookahead::Open)
        } else {
            None
        }
    }
}
