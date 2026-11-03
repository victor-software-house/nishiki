//! Expressions: operators, literals, names, calls, arrow functions and
//! object literals.

use super::bytes::{
    ANY, ARROW_START, BLANK, Bytes, CALL_START, COMMENT_START, DIGIT, LITERAL_START, NAME_START,
    OPERATOR_START, STRING_START,
};
use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// The `expression` rule group: everything that is not an identifier,
    /// then identifiers, then punctuation.
    pub(crate) fn expression(&mut self, p: usize) -> Option<usize> {
        self.expression_without_identifiers(p)
            .or_else(|| self.identifiers(p))
            .or_else(|| self.expression_punctuation(p))
    }

    pub(crate) fn expression_without_identifiers(&mut self, p: usize) -> Option<usize> {
        self.only("e.jsx", BLANK.with(b'<'), p, Self::jsx)
            .or_else(|| self.only("e.string", STRING_START, p, Self::string))
            .or_else(|| self.only("e.regex", BLANK.with(b'/'), p, Self::regex))
            .or_else(|| self.only("e.comment", COMMENT_START, p, Self::comment))
            .or_else(|| {
                self.only(
                    "e.function_expression",
                    Bytes::of(b"af"),
                    p,
                    Self::function_expression,
                )
            })
            .or_else(|| {
                self.only(
                    "e.class_expression",
                    Bytes::of(b"ac"),
                    p,
                    Self::class_expression,
                )
            })
            .or_else(|| self.only("e.arrow_function", ARROW_START, p, Self::arrow_function))
            .or_else(|| {
                self.only(
                    "e.paren_expression_possibly_arrow",
                    ANY,
                    p,
                    Self::paren_expression_possibly_arrow,
                )
            })
            .or_else(|| self.only("e.cast", BLANK.with(b'<'), p, Self::cast))
            .or_else(|| {
                self.only(
                    "e.ternary_expression",
                    Bytes::of(b"?"),
                    p,
                    Self::ternary_expression,
                )
            })
            .or_else(|| self.only("e.new_expr", Bytes::of(b"n"), p, Self::new_expr))
            .or_else(|| {
                self.only(
                    "e.instanceof_expr",
                    Bytes::of(b"i"),
                    p,
                    Self::instanceof_expr,
                )
            })
            .or_else(|| self.only("e.object_literal", Bytes::of(b"{"), p, Self::object_literal))
            .or_else(|| {
                self.only(
                    "e.expression_operators",
                    OPERATOR_START,
                    p,
                    Self::expression_operators,
                )
            })
            .or_else(|| self.only("e.function_call", CALL_START, p, Self::function_call))
            .or_else(|| self.only("e.literal", LITERAL_START, p, Self::literal))
            .or_else(|| {
                self.only(
                    "e.support_objects",
                    Bytes::of(b"aPinem.?"),
                    p,
                    Self::support_objects,
                )
            })
            .or_else(|| {
                self.only(
                    "e.paren_expression",
                    Bytes::of(b"("),
                    p,
                    Self::paren_expression,
                )
            })
    }

    pub(crate) fn expression_punctuation(&mut self, p: usize) -> Option<usize> {
        self.punctuation_comma(p)
            .or_else(|| self.punctuation_accessor(p))
    }

    pub(crate) fn punctuation_comma(&mut self, p: usize) -> Option<usize> {
        (self.byte(p) == b',').then(|| self.token_rule(p, p + 1, S::PunctuationSeparatorComma))?
    }

    pub(crate) fn punctuation_semicolon(&mut self, p: usize) -> Option<usize> {
        (self.byte(p) == b';')
            .then(|| self.token_rule(p, p + 1, S::PunctuationTerminatorStatement))?
    }

    /// `(\.)|(\?\.(?!\s*\d))`.
    pub(crate) fn punctuation_accessor(&mut self, p: usize) -> Option<usize> {
        let e = self.accessor_at(p)?;
        self.token_rule(p, e, Self::accessor_scope(p, e))
    }

    /// The keyword and symbol operators of an expression.
    ///
    /// In `yield /* comment */ *` the asterisk follows a comment.
    pub(crate) fn expression_operators(&mut self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        if b.is_ascii_lowercase() {
            if let Some(e) = self.kw(p, "await") {
                return self.token_rule(p, e, S::KeywordControlFlow);
            }
            if let Some(e) = self.kw(p, "yield") {
                let j = self.ws(e);
                if let Some(c) = self.block_comment(j)
                    && self.byte(self.ws(c)) == b'*'
                {
                    return self.open_token(Kind::YieldComment, p, e, &[S::KeywordControlFlow]);
                }
                self.token(p, e, &[S::KeywordControlFlow]);
                let j = self.ws(e);
                if self.byte(j) == b'*' {
                    self.capture(Kind::Top, j, j + 1, &[S::KeywordGeneratorAsterisk]);
                    return Some(j + 1);
                }
                return Some(e);
            }
            for (word, scope) in [
                ("delete", S::KeywordOperatorExpressionDelete),
                ("in", S::KeywordOperatorExpressionIn),
                ("of", S::KeywordOperatorExpressionOf),
                ("instanceof", S::KeywordOperatorExpressionInstanceof),
                ("new", S::KeywordOperatorNew),
            ] {
                if let Some(e) = self.kw(p, word) {
                    if (word == "in" || word == "of") && self.byte(e) == b'(' {
                        continue;
                    }
                    return self.token_rule(p, e, scope);
                }
            }
            if let Some(r) = self.typeof_operator(p) {
                return Some(r);
            }
            if let Some(e) = self.kw(p, "void") {
                return self.token_rule(p, e, S::KeywordOperatorExpressionVoid);
            }
            if let Some(r) = self.as_expression(p) {
                return Some(r);
            }
            return None;
        }
        self.symbol_operators(p)
    }

    /// `as const`, `as Type` and `satisfies Type`.
    fn as_expression(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.kw(p, "as") {
            let j = self.ws(e);
            if j > e {
                if let Some(c) = self.kw(j, "const") {
                    let k = self.ws(c);
                    if matches!(self.byte(k), b']' | b')' | b',' | b':' | b';' | b'}')
                        || self.eol(k)
                    {
                        self.token(p, e, &[S::KeywordControlAs]);
                        self.capture(Kind::Top, j, c, &[S::StorageModifier]);
                        return Some(c);
                    }
                }
                if self.full() {
                    return None;
                }
                self.flush(p);
                self.capture(Kind::Top, p, e, &[S::KeywordControlAs]);
                self.finish(Kind::Top, j);
                self.push(Kind::AsType);
                return Some(j);
            }
        }
        if let Some(e) = self.kw(p, "satisfies") {
            let j = self.ws(e);
            if j > e && !self.full() {
                self.flush(p);
                self.capture(Kind::Top, p, e, &[S::KeywordControlSatisfies]);
                self.finish(Kind::Top, j);
                self.push(Kind::AsType);
                return Some(j);
            }
        }
        None
    }

    /// The operators made of symbols, in the grammar's order: the spread
    /// `...`; the compound assignments `*=`, `/=` (not after `(`), `-=`, `%=`
    /// and `+=`; the bitwise ones `&=`, `^=`, `<<=`, `>>=`, `>>>=` and `|=`;
    /// `a! /= b` and `a! / b`; and the arithmetic and relational operators.
    ///
    /// A `/` after a name may follow blanks and comments, because the regex
    /// and comment rules have claimed the other cases, and a `/` or `/=` after
    /// a closing bracket or a name may have blanks before it.
    fn symbol_operators(&mut self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        let b1 = self.byte(p + 1);
        let b2 = self.byte(p + 2);
        let prev = if p > 0 {
            self.bytes.get(p - 1).copied()
        } else {
            None
        };
        if self.starts(p, "...") {
            return self.token_rule(p, p + 3, S::KeywordOperatorSpread);
        }
        if b1 == b'=' && matches!(b, b'*' | b'-' | b'%' | b'+') {
            return self.token_rule(p, p + 2, S::KeywordOperatorAssignmentCompound);
        }
        if b == b'/' && b1 == b'=' && prev != Some(b'(') {
            return self.token_rule(p, p + 2, S::KeywordOperatorAssignmentCompound);
        }
        if let Some(len) = self.bitwise_assign_len(p) {
            return self.token_rule(p, p + len, S::KeywordOperatorAssignmentCompoundBitwise);
        }
        if self.starts(p, "<<") || self.starts(p, ">>>") || self.starts(p, ">>") {
            let len = if self.starts(p, ">>>") { 3 } else { 2 };
            return self.token_rule(p, p + len, S::KeywordOperatorBitwiseShift);
        }
        if matches!(b, b'!' | b'=') && b1 == b'=' {
            let len = if b2 == b'=' { 3 } else { 2 };
            return self.token_rule(p, p + len, S::KeywordOperatorComparison);
        }
        if (b == b'<' || b == b'>') && b1 == b'=' {
            return self.token_rule(p, p + 2, S::KeywordOperatorRelational);
        }
        if b == b'<' && b1 == b'>' {
            return self.token_rule(p, p + 2, S::KeywordOperatorRelational);
        }
        if b == b'<' || b == b'>' {
            return self.token_rule(p, p + 1, S::KeywordOperatorRelational);
        }
        if b == b'!' && self.alnum_before(p) {
            let j = self.ws(p + 1);
            if self.starts(j, "/=") {
                self.token(p, p + 1, &[S::KeywordOperatorLogical]);
                self.capture(Kind::Top, j, j + 2, &[S::KeywordOperatorAssignmentCompound]);
                return Some(j + 2);
            }
            if self.byte(j) == b'/' && !matches!(self.byte(j + 1), b'*' | b'/') {
                self.token(p, p + 1, &[S::KeywordOperatorLogical]);
                self.capture(Kind::Top, j, j + 1, &[S::KeywordOperatorArithmetic]);
                return Some(j + 1);
            }
        }
        if b == b'!' {
            return self.token_rule(p, p + 1, S::KeywordOperatorLogical);
        }
        if (b == b'&' && b1 == b'&') || (b == b'|' && b1 == b'|') || (b == b'?' && b1 == b'?') {
            return self.token_rule(p, p + 2, S::KeywordOperatorLogical);
        }
        if matches!(b, b'&' | b'^' | b'|' | b'~') {
            return self.token_rule(p, p + 1, S::KeywordOperatorBitwise);
        }
        if b == b'=' {
            return self.token_rule(p, p + 1, S::KeywordOperatorAssignment);
        }
        if b == b'-' && b1 == b'-' {
            return self.token_rule(p, p + 2, S::KeywordOperatorDecrement);
        }
        if b == b'+' && b1 == b'+' {
            return self.token_rule(p, p + 2, S::KeywordOperatorIncrement);
        }
        if matches!(b, b'-' | b'%' | b'*' | b'+' | b'/') {
            return self.token_rule(p, p + 1, S::KeywordOperatorArithmetic);
        }
        if self.space_at(p) && self.divide_after_name(p) {
            return Some(self.divide_with_blanks(p));
        }
        None
    }

    fn bitwise_assign_len(&self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        if matches!(b, b'&' | b'^' | b'|') && self.byte(p + 1) == b'=' {
            return Some(2);
        }
        if self.starts(p, "<<=") || self.starts(p, ">>=") {
            return Some(3);
        }
        if self.starts(p, ">>>=") {
            return Some(4);
        }
        None
    }

    /// `(?<=[]$)_[:alnum:]])\s*(?:(/=)|(/)(?![*/]))` starting at blanks.
    fn divide_after_name(&self, p: usize) -> bool {
        let before = self.prev_char(p);
        if !before.is_some_and(|c| c == ']' || c == ')' || super::chars::is_alnum_char(c)) {
            return false;
        }
        let j = self.ws(p);
        self.starts(j, "/=") || (self.byte(j) == b'/' && !matches!(self.byte(j + 1), b'*' | b'/'))
    }

    fn divide_with_blanks(&mut self, p: usize) -> usize {
        let j = self.ws(p);
        if self.starts(j, "/=") {
            self.capture(Kind::Top, j, j + 2, &[S::KeywordOperatorAssignmentCompound]);
            return j + 2;
        }
        self.capture(Kind::Top, j, j + 1, &[S::KeywordOperatorArithmetic]);
        j + 1
    }

    /// `typeof` up to the end of its operand.
    pub(crate) fn typeof_operator(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "typeof")?;
        self.open_token(Kind::Typeof, p, e, &[S::KeywordOperatorExpressionTypeof])
    }

    pub(crate) fn literal(&mut self, p: usize) -> Option<usize> {
        self.only("l.number", DIGIT.with(b'.'), p, Self::numeric_literal)
            .or_else(|| self.only("l.keyword", Bytes::of(b"tfnuNIs"), p, Self::keyword_literal))
            .or_else(|| self.only("l.array", BLANK.with(b'['), p, Self::array_literal))
    }

    fn keyword_literal(&mut self, p: usize) -> Option<usize> {
        if !self.byte(p).is_ascii_alphabetic() {
            return None;
        }
        for (word, scope) in [
            ("true", S::ConstantLanguageBooleanTrue),
            ("false", S::ConstantLanguageBooleanFalse),
            ("null", S::ConstantLanguageNull),
            ("undefined", S::ConstantLanguageUndefined),
            ("NaN", S::ConstantLanguageNan),
            ("Infinity", S::ConstantLanguageInfinity),
        ] {
            if let Some(e) = self.kw(p, word) {
                return self.token_rule(p, e, scope);
            }
        }
        if self.starts(p, "this") {
            let e = p + "this".len();
            if self.kw_before(p) && self.boundary(e) && self.byte(e) != b'$' {
                return self.token_rule(p, e, S::VariableLanguageThis);
            }
        }
        if self.starts(p, "super") {
            let e = p + "super".len();
            if self.kw_before(p) && self.boundary(e) && self.byte(e) != b'$' {
                return self.token_rule(p, e, S::VariableLanguageSuper);
            }
        }
        None
    }

    /// `\s*(\[)`.
    pub(crate) fn array_literal(&mut self, p: usize) -> Option<usize> {
        let j = self.ws(p);
        if self.byte(j) != b'[' || self.full() {
            return None;
        }
        self.flush(p);
        self.capture(Kind::ArrLit, j, j + 1, &[S::MetaBraceSquare]);
        self.push(Kind::ArrLit);
        Some(j + 1)
    }

    pub(crate) fn object_literal(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'{' {
            return None;
        }
        self.open_token(Kind::ObjLit, p, p + 1, &[S::PunctuationDefinitionBlock])
    }

    pub(crate) fn paren_expression(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'(' {
            return None;
        }
        self.open_token(Kind::ParenExpr, p, p + 1, &[S::MetaBraceRound])
    }

    /// The built-in objects and their members, such as `arguments`, `Promise`
    /// and the `Number` constants.
    ///
    /// For a `Number` constant the grammar maps a scope to capture 4, but
    /// numbering counts the groups inside the negative lookahead before it, so
    /// the constant gets none. Shiki's output has none, and so has ours.
    pub(crate) fn support_objects(&mut self, p: usize) -> Option<usize> {
        let b = self.byte(p);
        if b.is_ascii_lowercase() || b.is_ascii_uppercase() {
            if let Some(e) = self.word_guard(p, "arguments") {
                return self.token_rule(p, e, S::VariableLanguageArguments);
            }
            if let Some(e) = self.word_guard(p, "Promise") {
                return self.token_rule(p, e, S::SupportClassPromise);
            }
            for (word, member, scope, member_scope) in [
                (
                    "import",
                    "meta",
                    S::KeywordControlImport,
                    S::SupportVariablePropertyImportmeta,
                ),
                (
                    "new",
                    "target",
                    S::KeywordOperatorNew,
                    S::SupportVariablePropertyTarget,
                ),
            ] {
                if let Some(e) = self.word_guard(p, word) {
                    let j = self.ws(e);
                    if let Some(a) = self.accessor_at(j) {
                        let k = self.ws(a);
                        if self.starts(k, member)
                            && self.boundary(k + member.len())
                            && self.byte(k + member.len()) != b'$'
                        {
                            self.token(p, e, &[scope]);
                            self.capture(Kind::Top, j, a, &[Self::accessor_scope(j, a)]);
                            self.capture(Kind::Top, k, k + member.len(), &[member_scope]);
                            return Some(k + member.len());
                        }
                    }
                }
            }
            if let Some(r) = self.module_exports(p) {
                return Some(r);
            }
        }
        if let Some(a) = self.accessor_at(p) {
            let k = self.ws(a);
            for word in ["constructor", "length", "prototype", "__proto__"] {
                if self.starts(k, word)
                    && self.boundary(k + word.len())
                    && self.byte(k + word.len()) != b'$'
                    && !self.call_after_type_args(k + word.len())
                {
                    self.token(p, a, &[Self::accessor_scope(p, a)]);
                    self.capture(Kind::Top, k, k + word.len(), &[S::SupportVariableProperty]);
                    return Some(k + word.len());
                }
            }
            for word in [
                "EPSILON",
                "MAX_SAFE_INTEGER",
                "MAX_VALUE",
                "MIN_SAFE_INTEGER",
                "MIN_VALUE",
                "NEGATIVE_INFINITY",
                "POSITIVE_INFINITY",
            ] {
                if self.starts(k, word)
                    && self.boundary(k + word.len())
                    && self.byte(k + word.len()) != b'$'
                {
                    self.token(p, a, &[Self::accessor_scope(p, a)]);
                    return Some(k + word.len());
                }
            }
        }
        None
    }

    /// `\s*(<…>\s*)?\(` after a name: the negative lookahead that keeps a
    /// method named `length` from being the property.
    fn call_after_type_args(&self, e: usize) -> bool {
        let j = self.ws(e);
        self.tp_paren(j).is_some()
    }

    /// `B word \b (?!\$)`.
    fn word_guard(&self, p: usize, word: &str) -> Option<usize> {
        let e = p + word.len();
        (self.starts(p, word) && self.kw_before(p) && self.boundary(e) && self.byte(e) != b'$')
            .then_some(e)
    }

    fn module_exports(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.word_guard(p, "exports") {
            return self.token_rule(p, e, S::SupportTypeObjectModule);
        }
        let e = self.word_guard(p, "module")?;
        let j = e;
        if let Some(a) = self.accessor_at(self.ws(j)) {
            let at = self.ws(j);
            let k = self.ws(a);
            for word in ["exports", "id", "filename", "loaded", "parent", "children"] {
                if self.starts(k, word)
                    && self.boundary(k + word.len())
                    && self.byte(k + word.len()) != b'$'
                {
                    self.token(p, e, &[S::SupportTypeObjectModule]);
                    self.capture(Kind::Top, at, a, &[Self::accessor_scope(at, a)]);
                    self.capture(Kind::Top, k, k + word.len(), &[S::SupportTypeObjectModule]);
                    return Some(k + word.len());
                }
            }
        }
        self.token_rule(p, e, S::SupportTypeObjectModule)
    }

    pub(crate) fn identifiers(&mut self, p: usize) -> Option<usize> {
        self.only(
            "i.object",
            NAME_START.or(Bytes::of(b".?")),
            p,
            Self::object_identifiers,
        )
        .or_else(|| {
            self.only(
                "i.assigned_function",
                NAME_START.or(Bytes::of(b".?")),
                p,
                Self::assigned_function_name,
            )
        })
        .or_else(|| self.only("i.property", Bytes::of(b".?"), p, Self::property_name))
        .or_else(|| self.only("i.variable", NAME_START, p, Self::variable_name))
    }

    /// The names that start a member chain: `Name.prototype`, and a property
    /// followed by another, `.property.` or `?.property.`.
    pub(crate) fn object_identifiers(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.ident(p) {
            let j = self.ws(e);
            let q = if self.byte(j) == b'?' { j + 1 } else { j };
            if self.byte(q) == b'.' {
                let k = self.ws(q + 1);
                if self.starts(k, "prototype")
                    && self.boundary(k + "prototype".len())
                    && self.byte(k + "prototype".len()) != b'$'
                {
                    return self.token_rule(p, e, S::SupportClass);
                }
            }
        }
        if let Some(a) = self.accessor_at(p) {
            let k = self.ws(a);
            let (name_end, scope) = match self.hash_constant_name(k) {
                Some(e) => (e, S::VariableOtherConstantObjectProperty),
                None => match self.hash_ident(k) {
                    Some(e) => (e, S::VariableOtherObjectProperty),
                    None => return self.object_head(p),
                },
            };
            if self.member_ahead(name_end) {
                self.token(p, a, &[Self::accessor_scope(p, a)]);
                self.capture(Kind::Top, k, name_end, &[scope]);
                return Some(name_end);
            }
            return None;
        }
        self.object_head(p)
    }

    /// `Name` or `NAME` followed by `.member`.
    fn object_head(&mut self, p: usize) -> Option<usize> {
        let (e, scope) = match self.constant_name(p) {
            Some(e) if self.member_ahead(e) => (e, S::VariableOtherConstantObject),
            _ => {
                let e = self.ident(p)?;
                (e, S::VariableOtherObject)
            }
        };
        self.member_ahead(e).then(|| self.token_rule(p, e, scope))?
    }

    /// `name = function` and similar: a name the grammar scopes as a function.
    ///
    /// The regex is `\s*=\s*FUNCTION`.
    fn assigned_function_name(&mut self, p: usize) -> Option<usize> {
        let (start, accessor) = match self.accessor_at(p) {
            Some(a) => (self.ws(a), Some(a)),
            None => (p, None),
        };
        let e = self.ident(start)?;
        let j = self.ws(e);
        if self.byte(j) != b'=' {
            return None;
        }
        if !self.fn_value_ahead(self.ws(j + 1)) {
            return None;
        }
        if let Some(a) = accessor {
            self.token(p, a, &[Self::accessor_scope(p, a)]);
            self.capture(Kind::Top, start, e, &[S::EntityNameFunction]);
        } else {
            self.token(start, e, &[S::EntityNameFunction]);
        }
        Some(e)
    }

    /// `.name`: a property.
    fn property_name(&mut self, p: usize) -> Option<usize> {
        let a = self.accessor_at(p)?;
        let k = self.ws(a);
        let (e, scope) = match self.hash_constant_name(k) {
            Some(e) => (e, S::VariableOtherConstantProperty),
            None => (self.hash_ident(k)?, S::VariableOtherProperty),
        };
        self.token(p, a, &[Self::accessor_scope(p, a)]);
        self.capture(Kind::Top, k, e, &[scope]);
        Some(e)
    }

    fn variable_name(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.constant_name(p) {
            return self.token_rule(p, e, S::VariableOtherConstant);
        }
        let e = self.ident(p)?;
        self.token_rule(p, e, S::VariableOtherReadwrite)
    }

    /// `.` or `?.` (not before a digit) at `i`: the end of the accessor.
    pub(crate) fn accessor_at(&self, i: usize) -> Option<usize> {
        match self.byte(i) {
            b'.' => Some(i + 1),
            b'?' if self.byte(i + 1) == b'.' && !self.digit_after_ws(i + 2) => Some(i + 2),
            _ => None,
        }
    }

    /// The `(?!\s*\d)` that follows `?.`, as a positive test.
    fn digit_after_ws(&self, i: usize) -> bool {
        self.byte(self.ws(i)).is_ascii_digit()
    }

    /// The scope of an accessor token of width `end - at`.
    pub(crate) fn accessor_scope(at: usize, end: usize) -> S {
        if end - at == 1 {
            S::PunctuationAccessor
        } else {
            S::PunctuationAccessorOptional
        }
    }

    /// `#?` and a [`Lexer::constant_name`]: its end.
    pub(crate) fn hash_constant_name(&self, i: usize) -> Option<usize> {
        let j = if self.byte(i) == b'#' { i + 1 } else { i };
        self.constant_name(j)
    }

    /// `\p{upper}[$_\d[:upper:]]*` not followed by a name character: its end.
    pub(crate) fn constant_name(&self, i: usize) -> Option<usize> {
        let mut j = i;
        let first = self.char_at(j)?;
        if !first.is_uppercase() {
            return None;
        }
        j += first.len_utf8();
        while let Some(c) = self.char_at(j) {
            if c == '$' || c == '_' || c.is_ascii_digit() || c.is_uppercase() {
                j += c.len_utf8();
            } else {
                break;
            }
        }
        self.spend(j - i);
        (!self.alnum_at(j)).then_some(j)
    }

    /// `\s*\??\.\s*#?ID` at `i`.
    pub(crate) fn member_ahead(&self, i: usize) -> bool {
        let mut j = self.ws(i);
        if self.byte(j) == b'?' {
            j += 1;
        }
        self.byte(j) == b'.' && self.hash_ident(self.ws(j + 1)).is_some()
    }
}
