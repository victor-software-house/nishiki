//! The matchers of code outside strings: statements, names, operators,
//! numbers, brackets and comments.

use super::chars;
use crate::classes::{self, is_alpha};
use crate::python::engine::{Ahead, Lexer};
use crate::python::grammar::Pat;
use crate::rules::Chars;

/// The keywords of `illegal-names` that name flow control.
const FLOW_KEYWORDS: &[&[u8]] = &[
    b"and",
    b"assert",
    b"async",
    b"await",
    b"break",
    b"class",
    b"continue",
    b"def",
    b"del",
    b"elif",
    b"else",
    b"except",
    b"finally",
    b"for",
    b"from",
    b"global",
    b"if",
    b"in",
    b"is",
    b"nonlocal",
    b"not",
    b"or",
    b"pass",
    b"raise",
    b"return",
    b"try",
    b"while",
    b"with",
    b"yield",
];

/// The words of the `statement-keyword` flow-control match.
const STATEMENT_FLOW: &[&[u8]] = &[
    b"assert",
    b"async",
    b"break",
    b"continue",
    b"del",
    b"elif",
    b"else",
    b"except",
    b"finally",
    b"for",
    b"from",
    b"if",
    b"pass",
    b"raise",
    b"return",
    b"try",
    b"while",
    b"with",
];

/// The typehint names a `# type:` comment scopes.
const TYPEHINT_NAMES: &[&[u8]] = &[
    b"Any",
    b"Callable",
    b"Dict",
    b"FrozenSet",
    b"Iterable",
    b"List",
    b"None",
    b"Sequence",
    b"Set",
    b"Tuple",
    b"Union",
    b"bool",
    b"bytes",
    b"float",
    b"int",
    b"object",
    b"str",
];

/// The words of `codetags`.
const CODETAGS: &[&[u8]] = &[b"BUG", b"FIXME", b"HACK", b"NOTE", b"TODO", b"XXX"];

impl Lexer<'_> {
    #[expect(
        clippy::too_many_lines,
        clippy::wildcard_enum_match_arm,
        reason = "one arm per pattern, a table; the dispatch sends this function its own patterns only"
    )]
    pub(super) fn code_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::ImportAt0 => self.import_from(p),
            Pat::ImportAt0End => (self.at_eol(p) || self.starts(p, b"import")).then_some(p),
            Pat::ImportAt0At0 => self.dots(p),
            Pat::ImportAt1 => self.keyword_import(p),
            Pat::CommentsAt0 => self.typehint_begin(p),
            Pat::CommentsAt0End => (self.at_eol(p) || self.at(p) == b'#').then_some(p),
            Pat::CommentsAt0At0 => self.typehint_ignore(p),
            Pat::CommentsAt0At1 => {
                if self.prev_is(p, b'.') {
                    return None;
                }
                self.word_in(p, TYPEHINT_NAMES)
            }
            Pat::CommentsAt0At2 => self.typehint_punctuation(p),
            Pat::CommentsAt0At3 => self.ident_end(p),
            Pat::CommentsBase => self.hash(p),
            Pat::ImportAt1End | Pat::CommentsBaseEnd => self.at_eol(p).then_some(p),
            Pat::Codetags => {
                let end = self.word_in(p, CODETAGS)?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::Backticks => (self.at(p) == b'`').then_some(p + 1),
            Pat::BackticksEnd => self.backtick_end(p),
            Pat::IllegalAnno => self.lit(p, b"->"),
            Pat::LiteralAt0 => self.word_in(
                p,
                &[b"Ellipsis", b"False", b"None", b"NotImplemented", b"True"],
            ),
            Pat::NumberFloat => self.number_float(p),
            Pat::NumberDec => self.number_dec(p),
            Pat::NumberHex => self.number_radix(p, b"Xx", |b| b.is_ascii_hexdigit()),
            Pat::NumberOct => self.number_radix(p, b"Oo", |b| matches!(b, b'0'..=b'7')),
            Pat::NumberBin => self.number_radix(p, b"Bb", |b| matches!(b, b'0' | b'1')),
            Pat::NumberLong => self.number_long(p),
            Pat::NumberAt6 => self.number_name(p),
            Pat::LambdaAt0 => self.lambda_member(p),
            Pat::LambdaAt1 => self.lambda_bare(p),
            Pat::LambdaAt2 => self.lambda_begin(p),
            Pat::LambdaAt2End => self.lambda_end(p),
            Pat::LambdaNestedIncomplete => self.lambda_incomplete(p, b"),:"),
            Pat::LambdaAt2At4 => self.lambda_parameter(p),
            Pat::LambdaParameterWithDefault => self.name_equals(p, false),
            Pat::LambdaParameterWithDefaultEnd => self.lambda_default_end(p),
            Pat::Generator => self.keyword_zero(p, b"for"),
            Pat::GeneratorEnd => self.keyword_zero(p, b"in"),
            Pat::IllegalNames => self.illegal_names(p),
            Pat::IllegalOperatorAt0 => (self.starts(p, b"&&")
                || self.starts(p, b"||")
                || self.starts(p, b"--")
                || self.starts(p, b"++"))
            .then_some(p + 2),
            Pat::IllegalOperatorAt1 => matches!(self.at(p), b'$' | b'?').then_some(p + 1),
            Pat::IllegalOperatorAt2 => {
                (self.at(p) == b'!' && self.boundary(p + 1)).then_some(p + 1)
            }
            Pat::Operator => self.operator(p),
            Pat::CurlyBraces => (self.at(p) == b'{' && p < self.lim()).then_some(p + 1),
            Pat::List => (self.at(p) == b'[' && p < self.lim()).then_some(p + 1),
            Pat::RoundBraces => (self.at(p) == b'(' && p < self.lim()).then_some(p + 1),
            Pat::CurlyBracesEnd => (self.at(p) == b'}').then_some(p + 1),
            Pat::ListEnd => (self.at(p) == b']').then_some(p + 1),
            Pat::RoundBracesEnd => (self.at(p) == b')').then_some(p + 1),
            Pat::CurlyBracesAt0 | Pat::ItemIndexAt0 | Pat::PunctuationAt0 => {
                (self.at(p) == b':').then_some(p + 1)
            }
            Pat::PunctuationAt1 | Pat::ClassInheritanceAt1 => (self.at(p) == b',').then_some(p + 1),
            Pat::ItemAccessAt0 => self.item_access_begin(p),
            Pat::ItemAccessAt0End => self.char_cap(p, b']'),
            Pat::SpecialVariables => self.special_variables(p),
            Pat::BuiltinFunctionsAt0 => self.builtin(p, chars::FUNCTION),
            Pat::BuiltinFunctionsAt1 => self.builtin(p, chars::LEGACY),
            Pat::BuiltinTypes => self.builtin(p, chars::TYPE),
            Pat::BuiltinExceptions => self.builtin(p, chars::EXCEPTION),
            Pat::MagicFunctionNames => self.magic(p, chars::MAGIC_FUNCTION),
            Pat::MagicVariableNames => self.magic(p, chars::MAGIC_VARIABLE),
            Pat::SpecialNames => self.special_names(p),
            Pat::ItemNameAt3
            | Pat::FunctionNameAt1
            | Pat::ExpressionAt2
            | Pat::FExpressionAt2
            | Pat::MemberAccessAttribute
            | Pat::ClassNameAt2
            | Pat::FunctionDefNameAt2
            | Pat::InheritanceIdentifier => self.bounded_ident(p),
            Pat::ItemIndex => self.char_cap(p, b'['),
            Pat::ItemIndexEnd => (self.at(p) == b']').then_some(p),
            Pat::OddFunctionCall => self.odd_call_begin(p),
            Pat::OddFunctionCallEnd | Pat::FunctionCallEnd | Pat::CallWrapperInheritanceEnd => {
                self.char_cap(p, b')')
            }
            Pat::FunctionCall | Pat::CallWrapperInheritance => self.call_begin(p),
            Pat::FunctionArgumentsEnd => self.arguments_end(p),
            Pat::FunctionArgumentsAt0 => self.char_cap(p, b','),
            Pat::FunctionArgumentsAt1 => self.argument_unpacking(p),
            Pat::LambdaIncomplete => self.lambda_incomplete(p, b"),"),
            Pat::FunctionArgumentsAt4 | Pat::ClassKwarg => self.name_equals(p, true),
            Pat::FunctionArgumentsAt5 | Pat::ClassInheritanceAt2 | Pat::AnnotatedParameterAt1 => {
                self.equals_alone(p)
            }
            Pat::FunctionArgumentsAt7 => self.close_open(p),
            Pat::IllegalObjectName => self.word_in(p, &[b"False", b"None", b"True"]),
            Pat::Ellipsis => self.lit(p, b"..."),
            Pat::LineContinuationAt0 => self.continuation_junk(p),
            Pat::LineContinuationAt1 => self.continuation_begin(p),
            Pat::LineContinuationAt1End => self.continuation_end(p),
            Pat::MemberAccess | Pat::MemberAccessClass => self.member_access_begin(p),
            Pat::MemberAccessEnd => self.member_access_end(p, true),
            Pat::MemberAccessClassEnd => self.member_access_end(p, false),
            Pat::ClassDeclarationAt0 => self.class_begin(p),
            Pat::ClassDeclarationAt0End | Pat::ReturnAnnotationEnd => {
                self.class_end(p, pat == Pat::ReturnAnnotationEnd)
            }
            Pat::ClassInheritanceAt3 => self.word_in(p, &[b"metaclass"]),
            Pat::FunctionDeclaration => self.def_begin(p),
            Pat::FunctionDeclarationEnd => self.def_end(p),
            Pat::FunctionArguments | Pat::ClassInheritance | Pat::Parameters => {
                self.char_cap(p, b'(')
            }
            Pat::ClassInheritanceEnd | Pat::ParametersEnd => self.char_cap(p, b')'),
            Pat::LambdaAt2At0 | Pat::ParametersAt0 => (self.at(p) == b'/').then_some(p + 1),
            Pat::LambdaAt2At1 | Pat::ClassInheritanceAt0 | Pat::ParametersAt1 => self.star_run(p),
            Pat::ParameterSpecial => self.parameter_special(p),
            Pat::ParametersAt6 => self.parameter_name(p),
            Pat::LooseDefault => self.char_cap(p, b'='),
            Pat::LooseDefaultEnd | Pat::AnnotatedParameterEnd => self.parameter_end(p),
            Pat::AnnotatedParameter => self.annotated_begin(p),
            Pat::ReturnAnnotation => self.arrow_begin(p),
            Pat::StatementKeywordAt0 => self.def_keyword(p),
            Pat::StatementKeywordAt1 => self.as_flow(p),
            Pat::ImportAt1At0 | Pat::StatementKeywordAt2 => self.as_keyword(p),
            Pat::StatementKeywordAt3 => self.statement_flow(p),
            Pat::StatementKeywordAt4 => self
                .word_in(p, &[b"global", b"nonlocal"])
                .filter(|_| !self.prev_is(p, b'.')),
            Pat::StatementKeywordAt5 => self
                .word_in(p, &[b"class"])
                .filter(|_| !self.prev_is(p, b'.')),
            Pat::StatementKeywordAt6 => self.case_match(p),
            Pat::AssignmentOperator => self.assignment_operator(p),
            Pat::Decorator => self.decorator_begin(p),
            Pat::DecoratorEnd => self.decorator_end(p),
            Pat::DecoratorNameAt2 => self.decorator_name(p),
            Pat::DecoratorNameAt4 => self.decorator_junk(p),
            Pat::SemicolonAt0 => (self.at(p) == b';' && self.at_eol(p + 1)).then_some(p + 1),
            _ => None,
        }
    }

    /// A one-byte match of `byte` whose capture group 1 is that byte.
    fn char_cap(&mut self, p: usize, byte: u8) -> Option<usize> {
        if self.at(p) != byte || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// The regex `(#)`.
    fn hash(&mut self, p: usize) -> Option<usize> {
        self.char_cap(p, b'#')
    }

    /// The regex `\.+`.
    fn dots(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| b == b'.');
        (end > p).then_some(end)
    }

    /// The regex `(\*\*?)`.
    fn star_run(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'*' || p >= self.lim() {
            return None;
        }
        Some(p + 1 + usize::from(self.at(p + 1) == b'*' && p + 1 < self.lim()))
    }

    /// The regex `=(?!=)`.
    fn equals_alone(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'=' && p < self.lim() && self.at(p + 1) != b'=').then_some(p + 1)
    }

    /// The regex `\b(?<!\.)(from)\b(?=.+import)`.
    fn import_from(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p)
            || self.prev_is(p, b'.')
            || !self.starts(p, b"from")
            || !self.boundary(p + 4)
        {
            return None;
        }
        // `.+` takes at least one character before `import`.
        if self.last(Ahead::Import)? < p + 5 {
            return None;
        }
        self.cap(1, p, p + 4);
        Some(p + 4)
    }

    /// The regex `\b(?<!\.)(import)\b`.
    fn keyword_import(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.')
            || !self.boundary(p)
            || !self.starts(p, b"import")
            || !self.boundary(p + 6)
        {
            return None;
        }
        self.cap(1, p, p + 6);
        Some(p + 6)
    }

    /// The regex `\b(?<!\.)as\b`.
    fn as_keyword(&self, p: usize) -> Option<usize> {
        (!self.prev_is(p, b'.')
            && self.boundary(p)
            && self.starts(p, b"as")
            && self.boundary(p + 2))
        .then_some(p + 2)
    }

    /// The regex `\b(?<!\.)as\b(?=.*[:\\])`.
    fn as_flow(&self, p: usize) -> Option<usize> {
        let end = self.as_keyword(p)?;
        (self.last(Ahead::ColonOrBackslash)? >= end).then_some(end)
    }

    /// The regex `\b(?<!\.)(async|continue|...)\b`.
    fn statement_flow(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.word_in(p, STATEMENT_FLOW)?;
        Some(end)
    }

    /// A keyword with `\b` on both sides, as `\bfor\b`.
    fn keyword_zero(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(0, p, end);
        Some(end)
    }

    /// The begin `#\s*(type:)\s*+(?!$|#)`.
    fn typehint_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        let q = self.space_end(p + 1);
        if !self.starts(q, b"type:") {
            return None;
        }
        let r = self.space_end(q + 5);
        if self.at_eol(r) || self.at(r) == b'#' {
            return None;
        }
        self.cap(1, q, q + 5);
        Some(r)
    }

    /// The match `\Gignore(?=\s*(?:$|#))`.
    fn typehint_ignore(&self, p: usize) -> Option<usize> {
        if !(self.g && p == self.pos && self.starts(p, b"ignore")) {
            return None;
        }
        let q = self.space_end(p + 6);
        (self.at_eol(q) || self.at(q) == b'#').then_some(p + 6)
    }

    /// The match `([]()*,.=\[]|(->))`.
    fn typehint_punctuation(&self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        if matches!(
            self.at(p),
            b']' | b'(' | b')' | b'*' | b',' | b'.' | b'=' | b'['
        ) {
            return Some(p + 1);
        }
        self.lit(p, b"->")
    }

    /// The end `` `|(?<!\\)(\n) ``.
    fn backtick_end(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'`' && p < self.lim() {
            return Some(p + 1);
        }
        (p == self.lim() && !self.prev_is(p, b'\\')).then_some(p + 1)
    }

    /// `[0-9](?:_?[0-9])*` at `i`: where it ends.
    fn digits(&self, i: usize) -> Option<usize> {
        if !self.at(i).is_ascii_digit() || i >= self.lim() {
            return None;
        }
        let mut k = i + 1;
        loop {
            if self.at(k).is_ascii_digit() && k < self.lim() {
                k += 1;
            } else if self.at(k) == b'_' && self.at(k + 1).is_ascii_digit() && k + 1 < self.lim() {
                k += 2;
            } else {
                return Some(k);
            }
        }
    }

    /// `[Ee][-+]?[0-9](?:_?[0-9])*` at `i`: where it ends.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') || i >= self.lim() {
            return None;
        }
        let sign = usize::from(matches!(self.at(i + 1), b'-' | b'+'));
        self.digits(i + 1 + sign)
    }

    /// The match of `number-float`:
    /// `(?<!\w)(?:(?:\.D|D\.D|D\.)(?:E)?|DE)([Jj])?\b`.
    fn number_float(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        // The alternatives in the grammar's order, each with its exponent
        // and imaginary suffix tried present before absent.
        let mut bases = [None; 4];
        bases[0] = (self.at(p) == b'.' && p < self.lim())
            .then(|| self.digits(p + 1))
            .flatten();
        bases[1] = self.digits(p).and_then(|e| {
            (self.at(e) == b'.' && e < self.lim())
                .then(|| self.digits(e + 1))
                .flatten()
        });
        bases[2] = self
            .digits(p)
            .and_then(|e| (self.at(e) == b'.' && e < self.lim()).then_some(e + 1));
        let tail = |me: &Self, base: usize| -> Option<(usize, Option<(usize, usize)>)> {
            let with_exponent = me.exponent(base);
            for e in [with_exponent, Some(base)].into_iter().flatten() {
                if matches!(me.at(e), b'J' | b'j') && e < me.lim() && me.boundary(e + 1) {
                    return Some((e + 1, Some((e, e + 1))));
                }
                if me.boundary(e) {
                    return Some((e, None));
                }
            }
            None
        };
        for base in bases.into_iter().take(3).flatten() {
            if let Some((end, j)) = tail(self, base) {
                if let Some((from, to)) = j {
                    self.cap(1, from, to);
                }
                return Some(end);
            }
        }
        // `DE`: digits then an exponent, which is not optional.
        let e = self.digits(p).and_then(|e| self.exponent(e))?;
        if matches!(self.at(e), b'J' | b'j') && e < self.lim() && self.boundary(e + 1) {
            self.cap(1, e, e + 1);
            return Some(e + 1);
        }
        self.boundary(e).then_some(e)
    }

    /// The match of `number-dec`.
    fn number_dec(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || self.word_before(p) {
            return None;
        }
        let b = self.at(p);
        // `[1-9](?:_?[0-9])*`
        if matches!(b, b'1'..=b'9') && p < self.lim() {
            let end = self.digits(p)?;
            if self.boundary(end) {
                return Some(end);
            }
        }
        // `0+`
        if b == b'0' && p < self.lim() {
            let end = self.skip(p, |c| c == b'0');
            if self.boundary(end) {
                return Some(end);
            }
        }
        // `[0-9](?:_?[0-9])*([Jj])`
        if let Some(end) = self.digits(p)
            && matches!(self.at(end), b'J' | b'j')
            && end < self.lim()
            && self.boundary(end + 1)
        {
            self.cap(1, end, end + 1);
            return Some(end + 1);
        }
        // `0([0-9]+)(?![.Ee])`
        if b == b'0' && p < self.lim() {
            let end = self.skip(p + 1, |c| c.is_ascii_digit());
            if end > p + 1 && !matches!(self.at(end), b'.' | b'E' | b'e') && self.boundary(end) {
                self.cap(2, p + 1, end);
                return Some(end);
            }
        }
        None
    }

    /// `(?<![.\w])(0[Xx])(_?\h)+\b` and its octal and binary kin: `letters`
    /// are the two cases of the radix letter and `digit` the digit class.
    fn number_radix(&mut self, p: usize, letters: &[u8], digit: fn(u8) -> bool) -> Option<usize> {
        if self.prev_is(p, b'.') || self.word_before(p) {
            return None;
        }
        if self.at(p) != b'0' || !letters.contains(&self.at(p + 1)) || p + 1 >= self.lim() {
            return None;
        }
        let mut k = p + 2;
        let mut any = false;
        loop {
            if digit(self.at(k)) && k < self.lim() {
                k += 1;
            } else if self.at(k) == b'_' && digit(self.at(k + 1)) && k + 1 < self.lim() {
                k += 2;
            } else {
                break;
            }
            any = true;
        }
        if !any || !self.boundary(k) {
            return None;
        }
        self.cap(1, p, p + 2);
        Some(k)
    }

    /// The match `(?<![.\w])([1-9][0-9]*|0)([Ll])\b`.
    fn number_long(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') || self.word_before(p) || p >= self.lim() {
            return None;
        }
        let digits_end = match self.at(p) {
            b'1'..=b'9' => self.skip(p + 1, |c| c.is_ascii_digit()),
            b'0' => p + 1,
            _ => return None,
        };
        if matches!(self.at(digits_end), b'L' | b'l')
            && digits_end < self.lim()
            && self.boundary(digits_end + 1)
        {
            self.cap(2, digits_end, digits_end + 1);
            return Some(digits_end + 1);
        }
        None
    }

    /// The match `\b[0-9]+\w+`: a number that runs into letters. `[0-9]+`
    /// gives digits back to `\w+`, so a word of two digits or more matches.
    fn number_name(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_digit() || p >= self.lim() {
            return None;
        }
        let end = self.word_end(p);
        (end - p >= 2).then_some(end)
    }

    /// The match `((?<=\.)lambda|lambda(?=\s*[.=]))`.
    fn lambda_member(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"lambda") {
            return None;
        }
        let after = self.space_end(p + 6);
        if self.prev_is(p, b'.') || matches!(self.at(after), b'.' | b'=') && after < self.lim() {
            self.cap(1, p, p + 6);
            return Some(p + 6);
        }
        None
    }

    /// The match `\b(lambda)\s*?(?=[\n,]|$)`.
    fn lambda_bare(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"lambda") {
            return None;
        }
        let mut q = p + 6;
        loop {
            if self.at(q) == b'\n' && q == self.lim()
                || self.at(q) == b',' && q < self.lim()
                || self.at_eol(q)
            {
                self.cap(1, p, p + 6);
                return Some(q);
            }
            let width = self.space_at(q);
            if width == 0 {
                return None;
            }
            q += width;
        }
    }

    /// The begin `\b(lambda)\b`.
    fn lambda_begin(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"lambda") || !self.boundary(p + 6) {
            return None;
        }
        self.cap(1, p, p + 6);
        Some(p + 6)
    }

    /// The end `(:)|(\n)`.
    fn lambda_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b':' && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        (p == self.lim()).then_some(p + 1)
    }

    /// The match `\blambda(?=\s*[),:])` for the set `closers`.
    fn lambda_incomplete(&self, p: usize, closers: &[u8]) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"lambda") {
            return None;
        }
        let q = self.space_end(p + 6);
        (q < self.lim() && closers.contains(&self.at(q))).then_some(p + 6)
    }

    /// The match `([_[:alpha:]]\w*)\s*(?:(,)|(?=:|$))`.
    fn lambda_parameter(&mut self, p: usize) -> Option<usize> {
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        self.cap(1, p, e);
        if q < self.lim() && self.at(q) == b',' {
            self.cap(2, q, q + 1);
            return Some(q + 1);
        }
        (q < self.lim() && self.at(q) == b':' || self.at_eol(q)).then_some(q)
    }

    /// The begin `\b([_[:alpha:]]\w*)\s*(=)`, followed by `(?!=)` when
    /// `not_double`.
    fn name_equals(&mut self, p: usize, not_double: bool) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        if self.at(q) != b'=' || q >= self.lim() {
            return None;
        }
        if not_double && self.at(q + 1) == b'=' && q + 1 < self.lim() {
            return None;
        }
        self.cap(1, p, e);
        self.cap(2, q, q + 1);
        Some(q + 1)
    }

    /// The end `(,)|(?=:|$)`.
    fn lambda_default_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b',' && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        (self.at(p) == b':' && p < self.lim() || self.at_eol(p)).then_some(p)
    }

    /// The match of `illegal-names`.
    fn illegal_names(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        debug_assert!(FLOW_KEYWORDS.is_sorted(), "FLOW_KEYWORDS not sorted");
        if FLOW_KEYWORDS.binary_search(&word).is_ok() {
            self.cap(1, p, end);
            return Some(end);
        }
        if word == b"lambda" {
            let after = self.space_end(end);
            if self.prev_is(p, b'.') || matches!(self.at(after), b'.' | b'=') && after < self.lim()
            {
                self.cap(1, p, end);
                return Some(end);
            }
            return None;
        }
        if word == b"as" || word == b"import" {
            self.cap(2, p, end);
            return Some(end);
        }
        None
    }

    /// The match of `operator`.
    fn operator(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword_operator(p) {
            return Some(end);
        }
        let b = self.at(p);
        if p >= self.lim() {
            return None;
        }
        let two = |me: &Self, text: &[u8]| me.starts(p, text);
        // `(<<|>>|[\&^|~])`
        if two(self, b"<<") || two(self, b">>") {
            self.cap(3, p, p + 2);
            return Some(p + 2);
        }
        if matches!(b, b'&' | b'^' | b'|' | b'~') {
            self.cap(3, p, p + 1);
            return Some(p + 1);
        }
        // `(\*\*|[-%*+]|//|[/@])`
        if two(self, b"**") || two(self, b"//") {
            self.cap(4, p, p + 2);
            return Some(p + 2);
        }
        if matches!(b, b'-' | b'%' | b'*' | b'+' | b'/' | b'@') {
            self.cap(4, p, p + 1);
            return Some(p + 1);
        }
        // `(!=|==|>=|<=|[<>])`
        if two(self, b"!=") || two(self, b"==") || two(self, b">=") || two(self, b"<=") {
            self.cap(5, p, p + 2);
            return Some(p + 2);
        }
        if matches!(b, b'<' | b'>') {
            self.cap(5, p, p + 1);
            return Some(p + 1);
        }
        // `(:=)`
        if two(self, b":=") {
            self.cap(6, p, p + 2);
            return Some(p + 2);
        }
        None
    }

    /// The first alternative of `operator`:
    /// `\b(?<!\.)(?:(and|or|not|in|is)|(for|if|else|await|yield(?:\s+from)?))(?
    /// !\s*:)\b`.
    fn keyword_operator(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev_is(p, b'.') {
            return None;
        }
        let end = self.word_end(p);
        let word = self.text(p, end);
        let not_before_colon = |me: &Self, at: usize| {
            let q = me.space_end(at);
            !(q < me.lim() && me.at(q) == b':')
        };
        if matches!(word, b"and" | b"or" | b"not" | b"in" | b"is") {
            if !not_before_colon(self, end) {
                return None;
            }
            self.cap(1, p, end);
            return Some(end);
        }
        if matches!(word, b"for" | b"if" | b"else" | b"await") {
            if !not_before_colon(self, end) {
                return None;
            }
            self.cap(2, p, end);
            return Some(end);
        }
        if word == b"yield" {
            // `(?:\s+from)?` first, then without it.
            let q = self.space_end(end);
            if q > end && self.starts(q, b"from") {
                let e = q + 4;
                if self.boundary(e) && not_before_colon(self, e) {
                    self.cap(2, p, e);
                    return Some(e);
                }
            }
            if !not_before_colon(self, end) {
                return None;
            }
            self.cap(2, p, end);
            return Some(end);
        }
        None
    }

    /// The begin `\b(?=[_[:alpha:]]\w*\s*\[)`.
    fn item_access_begin(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        (self.at(q) == b'[' && q < self.lim()).then_some(p)
    }

    /// The match `\b(?<!\.)(?:(self)|(cls))\b`.
    fn special_variables(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev_is(p, b'.') {
            return None;
        }
        let end = self.word_end(p);
        match self.text(p, end) {
            b"self" => self.cap(1, p, end),
            b"cls" => self.cap(2, p, end),
            _ => return None,
        }
        Some(end)
    }

    /// A match `(?<!\.)\b(words)\b` of a builtin.
    fn builtin(&mut self, p: usize, list: u8) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.word_listed(p, list)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// A match `\b(__(?:names)__)\b`.
    fn magic(&mut self, p: usize, list: u8) -> Option<usize> {
        let end = self.word_listed(p, list)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The match `\b(_*\p{upper}[_\d]*\p{upper})[[:upper:]\d]*(_\w*)?\b`.
    fn special_names(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let upper = classes::is_upper;
        let mut i = self.skip(p, |b| b == b'_');
        // The first capital, then underscores and digits, then a second one.
        let (c, next) = self.char_at(i)?;
        if !upper(c) || next > end {
            return None;
        }
        i = next;
        while let Some((c, after)) = self.char_at(i)
            && (c == '_' || classes::is_digit(c))
        {
            i = after;
        }
        let (c, next) = self.char_at(i)?;
        if !upper(c) || next > end {
            return None;
        }
        i = next;
        // Capitals and digits, then an underscore ends the match at the word's
        // end.
        while i < end {
            let (c, next) = self.char_at(i)?;
            if upper(c) || classes::is_digit(c) {
                i = next;
            } else {
                return (c == '_').then_some(end);
            }
        }
        Some(end)
    }

    /// The match `\b([_[:alpha:]]\w*)\b`.
    fn bounded_ident(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ident_end(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The begin `(?<=[])])\s*(?=\()`.
    fn odd_call_begin(&self, p: usize) -> Option<usize> {
        if !(self.prev_is(p, b']') || self.prev_is(p, b')')) {
            return None;
        }
        let q = self.space_end(p);
        (self.at(q) == b'(' && q < self.lim()).then_some(q)
    }

    /// The begin `\b(?=([_[:alpha:]]\w*)\s*(\())`.
    fn call_begin(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        (self.at(q) == b'(' && q < self.lim()).then_some(p)
    }

    /// The end `(?=\))(?!\)\s*\()`.
    fn arguments_end(&self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let q = self.space_end(p + 1);
        (!(self.at(q) == b'(' && q < self.lim())).then_some(p)
    }

    /// The match `(?:(?<=[(,])|^)\s*(\*{1,2})`.
    fn argument_unpacking(&mut self, p: usize) -> Option<usize> {
        if !(self.prev_is(p, b'(') || self.prev_is(p, b',') || p == 0) {
            return None;
        }
        let q = self.space_end(p);
        let end = self.star_run(q)?;
        self.cap(1, q, end);
        Some(end)
    }

    /// The match `\s*(\))\s*(\()`.
    fn close_open(&mut self, p: usize) -> Option<usize> {
        let q = self.space_end(p);
        if self.at(q) != b')' || q >= self.lim() {
            return None;
        }
        let r = self.space_end(q + 1);
        if self.at(r) != b'(' || r >= self.lim() {
            return None;
        }
        self.cap(1, q, q + 1);
        self.cap(2, r, r + 1);
        Some(r + 1)
    }

    /// The match `(\\)\s*(\S.*$\n?)`.
    fn continuation_junk(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let q = self.space_end(p + 1);
        if q >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, q, self.lim() + 1);
        Some(self.lim() + 1)
    }

    /// The begin `(\\)\s*$\n?`.
    fn continuation_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let q = self.space_end(p + 1);
        if q <= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(self.lim() + 1)
    }

    /// The end `(?=^\s*$)|(?!(\s*[Rr]?('''|"""|["']))|\G()$)`.
    fn continuation_end(&self, p: usize) -> Option<usize> {
        if p == 0 && self.space_end(0) > self.lim() {
            return Some(p);
        }
        // `(\s*[Rr]?('''|"""|["']))`
        let q = self.space_end(p);
        if matches!(self.at(q), b'R' | b'r') && q < self.lim() {
            // `[Rr]?` is optional, so a quote may follow either way.
            if matches!(self.at(q + 1), b'\'' | b'"') && q + 1 < self.lim() {
                return None;
            }
        }
        if matches!(self.at(q), b'\'' | b'"') && q < self.lim() {
            return None;
        }
        // `\G()$`
        if self.g && p == self.pos && self.at_eol(p) {
            return None;
        }
        Some(p)
    }

    /// The begin `(\.)\s*(?!\.)`.
    fn member_access_begin(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'.' || p >= self.lim() {
            return None;
        }
        let q = self.space_end(p + 1);
        let end = if self.at(q) == b'.' && q < self.lim() {
            // The spaces give one back so that the lookahead sees a space.
            if q == p + 1 {
                return None;
            }
            q - 1
        } else {
            q
        };
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// The end `(?<=\S)(?=\W)|(^|(?<=\s))(?=[^\\\w\s])|$` of `member-access`,
    /// or `(?<=\S)(?=\W)|$` of `member-access-class` when `second` is false.
    fn member_access_end(&self, p: usize, second: bool) -> Option<usize> {
        let prev_non_space = p > 0 && self.char_before(p).is_some_and(|c| !classes::is_space(c));
        let word = self.word_at(p).is_some();
        // `(?=\W)`: any character that is not a word character, the line feed
        // included.
        if prev_non_space && !word && p <= self.lim() {
            return Some(p);
        }
        if second && p < self.lim() {
            let prev_space = p == 0 || self.char_before(p).is_some_and(classes::is_space);
            if prev_space && !word && self.space_at(p) == 0 && self.at(p) != b'\\' {
                return Some(p);
            }
        }
        self.at_eol(p).then_some(p)
    }

    /// The begin `\s*(class)\s+(?=[_[:alpha:]]\w*\s*([(:]))`.
    fn class_begin(&mut self, p: usize) -> Option<usize> {
        let class_at = self.space_end(p);
        if !self.starts(class_at, b"class") {
            return None;
        }
        let name_at = self.space_end(class_at + 5);
        if name_at == class_at + 5 {
            return None;
        }
        let name_end = self.ident_end(name_at)?;
        let after = self.space_end(name_end);
        if !(matches!(self.at(after), b'(' | b':') && after < self.lim()) {
            return None;
        }
        self.cap(1, class_at, class_at + 5);
        Some(name_at)
    }

    /// The end `(:)` of a class, or `(?=:)` of a return annotation.
    fn class_end(&mut self, p: usize, lookahead: bool) -> Option<usize> {
        if self.at(p) != b':' || p >= self.lim() {
            return None;
        }
        if lookahead {
            return Some(p);
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// The begin `\s*(?:\b(async)\s+)?\b(def)\s+(?=[_[:alpha:]]\p{word}*\s*\
    /// ()`.
    fn def_begin(&mut self, p: usize) -> Option<usize> {
        let q = self.space_end(p);
        // With `async` first, then without it.
        if self.boundary(q)
            && self.starts(q, b"async")
            && let r = self.space_end(q + 5)
            && r > q + 5
            && let Some(end) = self.def_after(r)
        {
            self.cap(1, q, q + 5);
            self.cap(2, end.0, end.0 + 3);
            return Some(end.1);
        }
        let end = self.def_after(q)?;
        self.cap(2, end.0, end.0 + 3);
        Some(end.1)
    }

    /// `\b(def)\s+(?=[_[:alpha:]]\p{word}*\s*\()` at `q`: where `def` starts
    /// and where the match ends.
    fn def_after(&self, q: usize) -> Option<(usize, usize)> {
        if !self.boundary(q) || !self.starts(q, b"def") {
            return None;
        }
        let r = self.space_end(q + 3);
        if r == q + 3 {
            return None;
        }
        let e = self.ident_end(r)?;
        let t = self.space_end(e);
        (self.at(t) == b'(' && t < self.lim()).then_some((q, r))
    }

    /// The end `(:|(?=[\n"#']))` of a function declaration.
    fn def_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b':' && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        let b = self.at(p);
        (p == self.lim() || matches!(b, b'"' | b'#' | b'\'')).then_some(p)
    }

    /// The match `\b((self)|(cls))\b\s*(?:(,)|(?=\)))`.
    fn parameter_special(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.word_end(p);
        let which = match self.text(p, end) {
            b"self" => 2,
            b"cls" => 3,
            _ => return None,
        };
        let q = self.space_end(end);
        if self.at(q) == b',' && q < self.lim() {
            self.cap(4, q, q + 1);
        } else if !(self.at(q) == b')' && q < self.lim()) {
            return None;
        }
        self.cap(1, p, end);
        self.cap(which, p, end);
        Some(if self.at(q) == b',' { q + 1 } else { q })
    }

    /// The match `([_[:alpha:]]\w*)\s*(?:(,)|(?=[\n#)=]))`.
    fn parameter_name(&mut self, p: usize) -> Option<usize> {
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        self.cap(1, p, e);
        if q < self.lim() {
            match self.at(q) {
                b',' => {
                    self.cap(2, q, q + 1);
                    return Some(q + 1);
                }
                b'#' | b')' | b'=' => return Some(q),
                _ => return None,
            }
        }
        // The spaces took the line feed, which the lookahead sees again.
        Some(self.lim())
    }

    /// The end `(,)|(?=\))`.
    fn parameter_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b',' && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        (self.at(p) == b')' && p < self.lim()).then_some(p)
    }

    /// The begin `\b([_[:alpha:]]\w*)\s*(:)`.
    fn annotated_begin(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let e = self.ident_end(p)?;
        let q = self.space_end(e);
        if self.at(q) != b':' || q >= self.lim() {
            return None;
        }
        self.cap(1, p, e);
        self.cap(2, q, q + 1);
        Some(q + 1)
    }

    /// The begin `(->)`.
    fn arrow_begin(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"->") {
            return None;
        }
        self.cap(1, p, p + 2);
        Some(p + 2)
    }

    /// The match `\b((async\s+)?\s*def)\b`.
    fn def_keyword(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let try_def =
            |me: &Self, q: usize| (me.starts(q, b"def") && me.boundary(q + 3)).then_some(q + 3);
        if self.starts(p, b"async") {
            let r = self.space_end(p + 5);
            if r > p + 5 {
                let r = self.space_end(r);
                if let Some(end) = try_def(self, r) {
                    return Some(end);
                }
            }
        }
        // Without `async`, the `\s*` can still take blanks after the boundary.
        try_def(self, self.space_end(p))
    }

    /// The match `^\s*(case|match)(?=\s*([-"#'(+:\[{\w\d]|$))\b`.
    fn case_match(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let q = self.space_end(0);
        let end = self.word_end(q);
        if !matches!(self.text(q, end), b"case" | b"match") {
            return None;
        }
        let r = self.space_end(end);
        let next = self.at(r);
        let ok = r >= self.lim() || b"-\"#'(+:[{".contains(&next) || self.word_at(r).is_some();
        if !ok {
            return None;
        }
        self.cap(1, q, end);
        Some(end)
    }

    /// The match `<<=|>>=|//=|\*\*=|\+=|-=|/=|@=|\*=|%=|~=|\^=|&=|\|=|=(?!=)`.
    fn assignment_operator(&self, p: usize) -> Option<usize> {
        for op in [
            &b"<<="[..],
            b">>=",
            b"//=",
            b"**=",
            b"+=",
            b"-=",
            b"/=",
            b"@=",
            b"*=",
            b"%=",
            b"~=",
            b"^=",
            b"&=",
            b"|=",
        ] {
            if self.starts(p, op) {
                return Some(p + op.len());
            }
        }
        self.equals_alone(p)
    }

    /// The begin `^\s*((@))\s*(?=[_[:alpha:]]\w*)`.
    fn decorator_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let q = self.space_end(0);
        if self.at(q) != b'@' || q >= self.lim() {
            return None;
        }
        let r = self.space_end(q + 1);
        let (c, _) = self.char_at(r)?;
        if c != '_' && !is_alpha(c) {
            return None;
        }
        self.cap(1, q, q + 1);
        self.cap(2, q, q + 1);
        Some(r)
    }

    /// The end `(\))(.*?)(?=\s*(?:#|$))|(?=[\n#])`.
    fn decorator_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b')' && p < self.lim() {
            // The lazy `.*?` stops at the first place that only white space
            // separates from a `#` or the end of the line: before the first
            // `#`, or before the trailing white space.
            let from = p + 1;
            let back = |me: &Self, mut at: usize| {
                while at > from
                    && let Some(c) = me.char_before(at)
                    && classes::is_space(c)
                {
                    at -= c.len_utf8();
                }
                at
            };
            self.spend(self.lim() - from.min(self.lim()));
            let trailing = back(self, self.lim());
            let k = match memchr::memchr(b'#', self.text(from, self.lim())) {
                Some(hash) => back(self, from + hash).min(trailing),
                None => trailing,
            };
            self.cap(1, p, p + 1);
            self.cap(2, p + 1, k);
            return Some(k);
        }
        (p == self.lim() || self.at(p) == b'#').then_some(p)
    }

    /// The match `([_[:alpha:]]\w*)|(\.)` of a decorator's name.
    fn decorator_name(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.ident_end(p) {
            return Some(end);
        }
        if self.at(p) == b'.' && p < self.lim() {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        None
    }

    /// The match `\s*([^#(.\\_[:alpha:]\s].*?)(?=#|$)`.
    fn decorator_junk(&mut self, p: usize) -> Option<usize> {
        let q = self.space_end(p);
        if q >= self.lim() {
            return None;
        }
        let (c, next) = self.char_at(q)?;
        if matches!(c, '#' | '(' | '.' | '\\' | '_') || is_alpha(c) || classes::is_space(c) {
            return None;
        }
        let mut k = next;
        while k < self.lim() && self.at(k) != b'#' {
            k = self.char_end(k);
        }
        self.cap(1, q, k);
        Some(k)
    }
}
