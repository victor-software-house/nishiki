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
use super::grammar::{Pattern, Rule};
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
    /// Whether `pattern` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - strings, comments, heregexes and JSX;
    /// - keywords and the grammar's word lists;
    /// - functions, calls, objects, properties, numbers and operators;
    /// - brackets and punctuation.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn pattern_matches(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::DoubleString | Pattern::DoubleStringEnd => self.lit(p, b"\""),
            Pattern::SingleString | Pattern::SingleStringEnd => self.lit(p, b"'"),
            Pattern::DoubleEscape => self.string_escape(p, strings::Quote::Double),
            Pattern::SingleEscape => self.string_escape(p, strings::Quote::Single),
            Pattern::SingleHeredoc | Pattern::SingleHeredocEnd => self.lit(p, b"'''"),
            Pattern::DoubleHeredoc | Pattern::DoubleHeredocEnd => self.lit(p, b"\"\"\""),
            Pattern::SingleHeredocEscape | Pattern::DoubleHeredocEscape => {
                let end = self.backslash_any(p)?;
                self.cap(1, p, p + 1);
                Some(end)
            }
            Pattern::Interpolation => self.lit(p, b"#{"),
            Pattern::InterpolationEnd
            | Pattern::DestructuredObjectEnd
            | Pattern::BraceEnd
            | Pattern::JsxExpressionEnd => self.lit(p, b"}"),
            Pattern::Script => self.script(p),
            Pattern::BlockComment => self.block_comment(p),
            Pattern::BlockCommentEnd => self.lit(p, b"###"),
            Pattern::Annotation => self.annotation(p),
            Pattern::LineComment => self.lit(p, b"#"),
            Pattern::LineCommentEnd => self.at_eol(p).then_some(p),
            Pattern::EmbeddedComment => self.embedded_comment(p),
            Pattern::Heregex => self.lit(p, b"///"),
            Pattern::HeregexEnd => self.heregex_end(p),
            Pattern::Anchor => self.anchor(p),
            Pattern::BackReference => self.back_reference(p),
            Pattern::Quantifier => self.quantifier(p),
            Pattern::Or => self.lit(p, b"|"),
            Pattern::Assertion => self.assertion(p),
            Pattern::AssertionEnd => {
                let end = self.lit(p, b")")?;
                self.cap(1, p, end);
                Some(end)
            }
            Pattern::Group => {
                let end = self.lit(p, b"(")?;
                Some(self.lit(end, b"?:").unwrap_or(end))
            }
            Pattern::GroupEnd
            | Pattern::ParamsEnd
            | Pattern::ParenArgumentsEnd
            | Pattern::ParenEnd => self.lit(p, b")"),
            Pattern::CharacterSet => self.character_set(p),
            Pattern::CharacterSetEnd => {
                let end = self.lit(p, b"]")?;
                self.cap(1, p, end);
                Some(end)
            }
            Pattern::CharacterRange => self.character_range(p),
            Pattern::CharacterClass => self.character_class(p),
            Pattern::NumericEscape => self.numeric_escape(p),
            Pattern::ControlEscape => self.control_escape(p),
            Pattern::BackslashEscape => self.backslash_any(p),
            Pattern::JsxTag => self.jsx_tag(p, b"<"),
            Pattern::JsxEndTag => self.jsx_tag(p, b"</"),
            Pattern::JsxTagEnd | Pattern::JsxEndTagEnd => self.jsx_tag_end(p),
            Pattern::JsxAttribute => self.jsx_attribute(p),
            Pattern::NewInstance => self.new_instance(p),
            Pattern::KeywordControl => self.keyword_control(p),
            Pattern::Delete => self.unkeyed_word(p, &[b"delete"]),
            Pattern::Instanceof => self.unkeyed_word(p, &[b"instanceof"]),
            Pattern::New => self.unkeyed_word(p, &[b"new"]),
            Pattern::Typeof => self.unkeyed_word(p, &[b"typeof"]),
            Pattern::Reserved => self.unkeyed_word(p, words::RESERVED),
            Pattern::True => self.constant(p, &[b"on", b"true", b"yes"]),
            Pattern::False => self.constant(p, &[b"false", b"no", b"off"]),
            Pattern::Null => self.constant(p, &[b"null"]),
            Pattern::Extends => self.extends(p),
            Pattern::Super => self.language_variable(p, b"super"),
            Pattern::This => self.language_variable(p, b"this"),
            Pattern::Arguments => self.language_variable(p, b"arguments"),
            Pattern::ClassExtends => self.class_extends(p),
            Pattern::Class => self.class(p),
            Pattern::Debugger => self.debugger(p),
            Pattern::SupportClass => self.listed(p, words::SUPPORT_CLASSES),
            Pattern::Console => self.keyword(p, b"console"),
            Pattern::ConsoleMethod => self.word_after(p, b"console.", words::CONSOLE_METHODS),
            Pattern::ArrayMethod => self.word_after(p, b".", words::ARRAY_METHODS),
            Pattern::ArrayStatic => self.word_after(p, b"Array.", &[b"isArray"]),
            Pattern::ObjectStatic => self.word_after(p, b"Object.", words::OBJECT_STATICS),
            Pattern::MathStatic => self.word_after(p, b"Math.", words::MATH_STATICS),
            Pattern::NumberStatic => self.word_after(p, b"Number.", words::NUMBER_STATICS),
            Pattern::SupportVariable => self.support_variable(p),
            Pattern::LanguageConstant => self.listed(p, &[b"Infinity", b"NaN", b"undefined"]),
            Pattern::EventHandler => self.listed(p, words::EVENT_HANDLERS),
            Pattern::SupportMethod => self.support_method(p),
            Pattern::DomMethod => self.listed(p, words::DOM_METHODS),
            Pattern::SupportFunction => self.listed(p, words::SUPPORT_FUNCTIONS),
            Pattern::WordOperator => self.word_operator(p),
            Pattern::FunctionAssignment => self.function_assignment(p),
            Pattern::QuotedFunctionAssignment => self.quoted_function_assignment(p),
            Pattern::InlineFunction => self.function_ahead(p).then_some(p),
            Pattern::FunctionAssignmentEnd
            | Pattern::QuotedFunctionAssignmentEnd
            | Pattern::InlineFunctionEnd => self.arrow(p),
            Pattern::Param => self.param(p),
            Pattern::InstanceParam => self.instance_param(p),
            Pattern::DestructuredObject => self.destructured(p, b'{', b'}'),
            Pattern::DestructuredArray => self.destructured(p, b'[', b']'),
            Pattern::DestructuredObjectName | Pattern::DestructuredArrayName => self.short_ident(p),
            Pattern::MethodName | Pattern::FunctionName => self.ident(p),
            Pattern::InvalidMethodName | Pattern::InvalidFunctionName => self.digit_ident(p),
            Pattern::MethodCall => self.method_call(p),
            Pattern::BareMethodCall => self.bare_method_call(p),
            Pattern::MethodCallEnd | Pattern::FunctionCallEnd => {
                (self.prev(p) == Some(b')')).then_some(p)
            }
            Pattern::BareMethodCallEnd
            | Pattern::BareArgumentsEnd
            | Pattern::BareFunctionCallEnd => self.arguments_end(p).then_some(p),
            Pattern::BareArguments => self.argument_ahead(p, LoneAt::Taken).then_some(p),
            Pattern::FunctionCall => self.function_call(p),
            Pattern::BareFunctionCall => self.bare_function_call(p),
            Pattern::Hex => self.prefixed_number(p, b'x', |b| b.is_ascii_hexdigit()),
            Pattern::Binary => self.prefixed_number(p, b'b', |b| matches!(b, b'0' | b'1')),
            Pattern::Octal => self.octal_number(p),
            Pattern::Decimal => self.decimal(p),
            Pattern::ConstantObject => self.constant_object(p),
            Pattern::VariableObject => {
                let end = self.ident(p)?;
                self.object_ahead(end).then_some(end)
            }
            Pattern::ConstantObjectProperty => self.constant_property(p, Ahead::Member),
            Pattern::VariableObjectProperty => self.variable_property(p, Ahead::Member),
            Pattern::ConstantProperty => self.constant_property(p, Ahead::Any),
            Pattern::VariableProperty => self.variable_property(p, Ahead::Any),
            Pattern::InvalidProperty => self.invalid_property(p),
            Pattern::WordAssignment => self.word_assignment(p),
            Pattern::CompoundAssignment => self.assignment(p, Self::compound_operator),
            Pattern::BitwiseAssignment => self.assignment(p, Self::bitwise_operator),
            Pattern::Assignment => self.assignment(p, Self::plain_assignment),
            Pattern::Shift => self.shift(p),
            Pattern::Comparison => self.comparison(p),
            Pattern::Logical => self.logical(p),
            Pattern::Bitwise => matches!(self.at(p), b'&' | b'^' | b'|' | b'~').then_some(p + 1),
            Pattern::Decrement => self.lit(p, b"--"),
            Pattern::Increment => self.lit(p, b"++"),
            Pattern::Splat => self.lit(p, b"..."),
            Pattern::Existential => self.lit(p, b"?"),
            Pattern::Arithmetic => {
                matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
            }
            Pattern::Prototype => self.lit(p, b"::"),
            Pattern::InvalidIdentifier => self.invalid_identifier(p),
            Pattern::Terminator => self.lit(p, b";"),
            Pattern::Delimiter => self.lit(p, b","),
            Pattern::Brace | Pattern::JsxExpression => self.lit(p, b"{"),
            Pattern::Bracket => self.lit(p, b"["),
            Pattern::BracketEnd | Pattern::DestructuredArrayEnd => self.lit(p, b"]"),
            Pattern::SliceExclusive => self.slice(p, b"..."),
            Pattern::SliceInclusive => self.slice(p, b".."),
            Pattern::Params | Pattern::ParenArguments | Pattern::Paren => self.lit(p, b"("),
            Pattern::InstanceVariable => {
                let end = self.lit(p, b"@")?;
                Some(self.short_ident(end).unwrap_or(end))
            }
            Pattern::SvelteVariable => self.svelte_variable(p),
            Pattern::SvelteProps => self.svelte_props(p),
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
    /// - a pattern that starts with a name reads the whole run of `[$\w]`. It
    ///   needs the same text after the name at every place in the run, and it
    ///   missed at `p`;
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
