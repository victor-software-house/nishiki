//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names, and the groups it
//! tokenizes again, with [`Lexer::cap`].
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or what giving them back leaves to test.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`].
//! `^` holds only at the start of the line, since Shiki tokenizes each line
//! alone.

mod expressions;
mod keywords;
mod strings;

use expressions::{Ahead, LoneAt};

use super::engine::{Coffee, Dialect, Lexer};
use super::grammar::{Pat, Rule};
use super::words;
use crate::rules::Chars;

/// Whether `b` is in the grammar's `[$A-Z_a-z]`.
fn is_ident_start(b: u8) -> bool {
    b == b'$' || b == b'_' || b.is_ascii_alphabetic()
}

/// The words that end a bare argument list, and that no bare call takes as
/// its first argument.
const ARGUMENT_STOPS: &[&[u8]] = &[
    b"of",
    b"in",
    b"then",
    b"is",
    b"isnt",
    b"and",
    b"or",
    b"for",
    b"else",
    b"when",
    b"if",
    b"unless",
    b"by",
    b"instanceof",
];

impl<const D: u8> Lexer<'_, D>
where
    Coffee<D>: Dialect,
{
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - strings, comments, heregexes and JSX;
    /// - keywords and the grammar's word lists;
    /// - functions, calls, objects, properties, numbers and operators;
    /// - brackets and punctuation.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn pat_matches(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::DoubleString | Pat::DoubleStringEnd => self.lit(p, b"\""),
            Pat::SingleString | Pat::SingleStringEnd => self.lit(p, b"'"),
            Pat::DoubleEscape => self.string_escape(p, strings::Quote::Double),
            Pat::SingleEscape => self.string_escape(p, strings::Quote::Single),
            Pat::SingleHeredoc | Pat::SingleHeredocEnd => self.lit(p, b"'''"),
            Pat::DoubleHeredoc | Pat::DoubleHeredocEnd => self.lit(p, b"\"\"\""),
            Pat::SingleHeredocEscape | Pat::DoubleHeredocEscape => {
                let end = self.backslash_any(p)?;
                self.cap(1, p, p + 1);
                Some(end)
            }
            Pat::Interpolation => self.lit(p, b"#{"),
            Pat::InterpolationEnd
            | Pat::DestructuredObjectEnd
            | Pat::BraceEnd
            | Pat::JsxExpressionEnd => self.lit(p, b"}"),
            Pat::Script => self.script(p),
            Pat::BlockComment => self.block_comment(p),
            Pat::BlockCommentEnd => self.lit(p, b"###"),
            Pat::Annotation => self.annotation(p),
            Pat::LineComment => self.lit(p, b"#"),
            Pat::LineCommentEnd => self.at_eol(p).then_some(p),
            Pat::EmbeddedComment => self.embedded_comment(p),
            Pat::Heregex => self.lit(p, b"///"),
            Pat::HeregexEnd => self.heregex_end(p),
            Pat::Anchor => self.anchor(p),
            Pat::BackReference => self.back_reference(p),
            Pat::Quantifier => self.quantifier(p),
            Pat::Or => self.lit(p, b"|"),
            Pat::Assertion => self.assertion(p),
            Pat::AssertionEnd => {
                let end = self.lit(p, b")")?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::Group => {
                let end = self.lit(p, b"(")?;
                Some(self.lit(end, b"?:").unwrap_or(end))
            }
            Pat::GroupEnd | Pat::ParamsEnd | Pat::ParenArgumentsEnd | Pat::ParenEnd => {
                self.lit(p, b")")
            }
            Pat::CharacterSet => self.character_set(p),
            Pat::CharacterSetEnd => {
                let end = self.lit(p, b"]")?;
                self.cap(1, p, end);
                Some(end)
            }
            Pat::CharacterRange => self.character_range(p),
            Pat::CharacterClass => self.character_class(p),
            Pat::NumericEscape => self.numeric_escape(p),
            Pat::ControlEscape => self.control_escape(p),
            Pat::BackslashEscape => self.backslash_any(p),
            Pat::JsxTag => self.jsx_tag(p, b"<"),
            Pat::JsxEndTag => self.jsx_tag(p, b"</"),
            Pat::JsxTagEnd | Pat::JsxEndTagEnd => self.jsx_tag_end(p),
            Pat::JsxAttribute => self.jsx_attribute(p),
            Pat::NewInstance => self.new_instance(p),
            Pat::KeywordControl => self.keyword_control(p),
            Pat::Delete => self.unkeyed_word(p, &[b"delete"]),
            Pat::Instanceof => self.unkeyed_word(p, &[b"instanceof"]),
            Pat::New => self.unkeyed_word(p, &[b"new"]),
            Pat::Typeof => self.unkeyed_word(p, &[b"typeof"]),
            Pat::Reserved => self.unkeyed_word(p, words::RESERVED),
            Pat::True => self.constant(p, &[b"on", b"true", b"yes"]),
            Pat::False => self.constant(p, &[b"false", b"no", b"off"]),
            Pat::Null => self.constant(p, &[b"null"]),
            Pat::Extends => self.extends(p),
            Pat::Super => self.language_variable(p, b"super"),
            Pat::This => self.language_variable(p, b"this"),
            Pat::Arguments => self.language_variable(p, b"arguments"),
            Pat::ClassExtends => self.class_extends(p),
            Pat::Class => self.class(p),
            Pat::Debugger => self.debugger(p),
            Pat::SupportClass => self.listed(p, words::SUPPORT_CLASSES),
            Pat::Console => self.keyword(p, b"console"),
            Pat::ConsoleMethod => self.word_after(p, b"console.", words::CONSOLE_METHODS),
            Pat::ArrayMethod => self.word_after(p, b".", words::ARRAY_METHODS),
            Pat::ArrayStatic => self.word_after(p, b"Array.", &[b"isArray"]),
            Pat::ObjectStatic => self.word_after(p, b"Object.", words::OBJECT_STATICS),
            Pat::MathStatic => self.word_after(p, b"Math.", words::MATH_STATICS),
            Pat::NumberStatic => self.word_after(p, b"Number.", words::NUMBER_STATICS),
            Pat::SupportVariable => self.support_variable(p),
            Pat::LanguageConstant => self.listed(p, &[b"Infinity", b"NaN", b"undefined"]),
            Pat::EventHandler => self.listed(p, words::EVENT_HANDLERS),
            Pat::SupportMethod => self.support_method(p),
            Pat::DomMethod => self.listed(p, words::DOM_METHODS),
            Pat::SupportFunction => self.listed(p, words::SUPPORT_FUNCTIONS),
            Pat::WordOperator => self.word_operator(p),
            Pat::FunctionAssignment => self.function_assignment(p),
            Pat::QuotedFunctionAssignment => self.quoted_function_assignment(p),
            Pat::InlineFunction => self.function_ahead(p).then_some(p),
            Pat::FunctionAssignmentEnd
            | Pat::QuotedFunctionAssignmentEnd
            | Pat::InlineFunctionEnd => self.arrow(p),
            Pat::Param => self.param(p),
            Pat::InstanceParam => self.instance_param(p),
            Pat::DestructuredObject => self.destructured(p, b'{', b'}'),
            Pat::DestructuredArray => self.destructured(p, b'[', b']'),
            Pat::DestructuredObjectName | Pat::DestructuredArrayName => self.short_ident(p),
            Pat::MethodName | Pat::FunctionName => self.ident(p),
            Pat::InvalidMethodName | Pat::InvalidFunctionName => self.digit_ident(p),
            Pat::MethodCall => self.method_call(p),
            Pat::BareMethodCall => self.bare_method_call(p),
            Pat::MethodCallEnd | Pat::FunctionCallEnd => (self.prev(p) == Some(b')')).then_some(p),
            Pat::BareMethodCallEnd | Pat::BareArgumentsEnd | Pat::BareFunctionCallEnd => {
                self.arguments_end(p).then_some(p)
            }
            Pat::BareArguments => self.argument_ahead(p, LoneAt::Taken).then_some(p),
            Pat::FunctionCall => self.function_call(p),
            Pat::BareFunctionCall => self.bare_function_call(p),
            Pat::Hex => self.prefixed_number(p, b'x', |b| b.is_ascii_hexdigit()),
            Pat::Binary => self.prefixed_number(p, b'b', |b| matches!(b, b'0' | b'1')),
            Pat::Octal => self.octal_number(p),
            Pat::Decimal => self.decimal(p),
            Pat::ConstantObject => self.constant_object(p),
            Pat::VariableObject => {
                let end = self.ident(p)?;
                self.object_ahead(end).then_some(end)
            }
            Pat::ConstantObjectProperty => self.constant_property(p, Ahead::Member),
            Pat::VariableObjectProperty => self.variable_property(p, Ahead::Member),
            Pat::ConstantProperty => self.constant_property(p, Ahead::Any),
            Pat::VariableProperty => self.variable_property(p, Ahead::Any),
            Pat::InvalidProperty => self.invalid_property(p),
            Pat::WordAssignment => self.word_assignment(p),
            Pat::CompoundAssignment => self.assignment(p, Self::compound_operator),
            Pat::BitwiseAssignment => self.assignment(p, Self::bitwise_operator),
            Pat::Assignment => self.assignment(p, Self::plain_assignment),
            Pat::Shift => self.shift(p),
            Pat::Comparison => self.comparison(p),
            Pat::Logical => self.logical(p),
            Pat::Bitwise => matches!(self.at(p), b'&' | b'^' | b'|' | b'~').then_some(p + 1),
            Pat::Decrement => self.lit(p, b"--"),
            Pat::Increment => self.lit(p, b"++"),
            Pat::Splat => self.lit(p, b"..."),
            Pat::Existential => self.lit(p, b"?"),
            Pat::Arithmetic => {
                matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
            }
            Pat::Prototype => self.lit(p, b"::"),
            Pat::InvalidIdentifier => self.invalid_identifier(p),
            Pat::Terminator => self.lit(p, b";"),
            Pat::Delimiter => self.lit(p, b","),
            Pat::Brace | Pat::JsxExpression => self.lit(p, b"{"),
            Pat::Bracket => self.lit(p, b"["),
            Pat::BracketEnd | Pat::DestructuredArrayEnd => self.lit(p, b"]"),
            Pat::SliceExclusive => self.slice(p, b"..."),
            Pat::SliceInclusive => self.slice(p, b".."),
            Pat::Params | Pat::ParenArguments | Pat::Paren => self.lit(p, b"("),
            Pat::InstanceVariable => {
                let end = self.lit(p, b"@")?;
                Some(self.short_ident(end).unwrap_or(end))
            }
            Pat::SvelteVariable => self.svelte_variable(p),
            Pat::SvelteProps => self.svelte_props(p),
        }
    }

    /// Svelte's `$` before a name:
    /// `(?<!["$'./_[:alnum:]])\$(?=[_[:alpha:]][$_[:alnum:]]*)`.
    /// The lookahead's tail can be empty, so a letter or `_` after the `$`
    /// decides.
    fn svelte_variable(&self, p: usize) -> Option<usize> {
        if !self.svelte_dollar(p) {
            return None;
        }
        let name = match self.s.get(p + 1) {
            Some(&b) if b < 0x80 => b == b'_' || b.is_ascii_alphabetic(),
            Some(_) => self
                .char_at(p + 1)
                .is_some_and(|(c, _)| crate::classes::is_alpha(c)),
            None => false,
        };
        name.then_some(p + 1)
    }

    /// The `$$` of Svelte's `$$props`, `$$restProps` and `$$slots`:
    /// `(?<!["$'./_[:alnum:]])(\$\$)(?=props|restProps|slots)`.
    fn svelte_props(&self, p: usize) -> Option<usize> {
        if !self.svelte_dollar(p) || self.s.get(p + 1) != Some(&b'$') {
            return None;
        }
        let after = self.s.get(p + 2..).unwrap_or_default();
        [b"props".as_slice(), b"restProps", b"slots"]
            .iter()
            .any(|name| after.starts_with(name))
            .then_some(p + 2)
    }

    /// A `$` at `p` with no `"`, `$`, `'`, `.`, `/`, `_` or `[:alnum:]`
    /// character before it.
    fn svelte_dollar(&self, p: usize) -> bool {
        if self.s.get(p) != Some(&b'$') {
            return false;
        }
        match p.checked_sub(1).and_then(|k| self.s.get(k)) {
            None => true,
            Some(&b) if b < 0x80 => {
                !(b.is_ascii_alphanumeric()
                    || matches!(b, b'"' | b'$' | b'\'' | b'.' | b'/' | b'_'))
            }
            Some(_) => !self.char_before(p).is_some_and(crate::classes::is_alnum),
        }
    }

    /// The end of the word at `p` when `\b` holds before it and the whole
    /// word is in `words`, a sorted list.
    /// That is what `\b(a|b|c)\b` accepts.
    fn listed(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        debug_assert!(words.is_sorted(), "a word list is sorted");
        if !self.boundary(p) {
            return None;
        }
        let end = self.name_word_end(p);
        let text = self.text(p, end);
        words
            .binary_search_by(|word| crate::bytes::order(word, text))
            .is_ok()
            .then_some(end)
    }

    /// Where the scan goes on after no pattern matched at `p`.
    ///
    /// At an ASCII letter or `_`, it goes on after the run of ASCII word
    /// characters there.
    /// No pattern can match inside that run:
    /// - a pattern that starts with a name reads the whole run of `[$\w]`, so
    ///   it needs the same text after the name at every place in the run, and
    ///   it missed at `p`;
    /// - `[A-Z][$0-9A-Z_]*` before a member reaches the run's end only where a
    ///   name before the same member matched at `p`;
    /// - every other pattern that can start with a word character needs `\b`,
    ///   or a blank, `.`, `(` or `[` before it.
    ///
    /// Two places are kept:
    /// - `(new)\s+` needs no `\b`, so a run that ends in `new` stops before it;
    /// - in a heregex's character set, `.-` can start a range at any character.
    pub(super) fn after_missed_word(&self, p: usize) -> usize {
        let b = self.at(p);
        if !(b.is_ascii_alphabetic() || b == b'_') || self.top_rule() == Rule::CharacterSet {
            return self.char_end(p);
        }
        let end = self.skip(p + 1, |b| b.is_ascii_alphanumeric() || b == b'_');
        if end - p > 3 && self.text(end - 3, end) == b"new" {
            return end - 3;
        }
        end
    }

    /// The width of the `[$\w]` character at `i`, when there is one.
    fn dollar_word_at(&self, i: usize) -> Option<usize> {
        if self.at(i) == b'$' {
            return Some(1);
        }
        self.word_at(i)
    }

    /// The end of the run of `[$\w]` characters from `i`.
    /// The run found last is remembered: several patterns read it at one
    /// place.
    fn dollar_word_end(&self, mut i: usize) -> usize {
        let key = self.lim() * 2 + usize::from(self.nl);
        if let Some(end) = self.x.names.get(key, i) {
            return end;
        }
        let from = i;
        while let Some(width) = self.dollar_word_at(i) {
            i += width;
        }
        self.spend(i - from);
        if i > from {
            self.x.names.set(key, from, i);
        }
        i
    }

    /// The end of the run of `\w` characters from `i`, remembered as
    /// [`Lexer::dollar_word_end`] remembers its run.
    fn name_word_end(&self, i: usize) -> usize {
        let key = self.lim() * 2 + usize::from(self.nl);
        if let Some(end) = self.x.words.get(key, i) {
            return end;
        }
        let end = self.word_end(i);
        if end > i {
            self.x.words.set(key, i, end);
        }
        end
    }

    /// Whether the character before `i` is in `[$\w]`.
    fn dollar_word_before(&self, i: usize) -> bool {
        self.prev(i) == Some(b'$') || self.word_before(i)
    }

    /// `[$A-Z_a-z][$\w]*`.
    /// Giving back the run leaves a `[$\w]` character next, which no pattern
    /// that follows the identifier takes.
    fn ident(&self, p: usize) -> Option<usize> {
        is_ident_start(self.at(p)).then(|| self.dollar_word_end(p + 1))
    }

    /// `[$A-Z_a-z]\w*`.
    fn short_ident(&self, p: usize) -> Option<usize> {
        is_ident_start(self.at(p)).then(|| self.word_end(p + 1))
    }

    /// `\d[$\w]*`.
    fn digit_ident(&self, p: usize) -> Option<usize> {
        (self.digit_end(p) > p).then(|| self.dollar_word_end(p))
    }

    /// Whether `\s` stands before `p`, or `p` starts the line: `(?<=\s|^)`.
    fn after_space(&self, p: usize) -> bool {
        p == 0 || self.char_before(p).is_some_and(crate::classes::is_space)
    }

    /// The end of the run of the byte `byte` from `i`.
    fn run_of(&self, i: usize, byte: u8) -> usize {
        self.skip(i, |b| b == byte)
    }
}
