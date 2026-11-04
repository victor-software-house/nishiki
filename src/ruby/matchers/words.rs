//! Keywords, the methods the grammar names, constants and variables.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The words of `keyword.control.ruby`.
const CONTROL: &[&[u8]] = &[
    b"BEGIN", b"begin", b"case", b"class", b"else", b"elsif", b"END", b"end", b"ensure", b"for",
    b"if", b"in", b"module", b"rescue", b"then", b"unless", b"until", b"when", b"while",
];

/// The words of `keyword.control.pseudo-method.ruby` that `\b` and `(?![!?])`
/// bound.
const PSEUDO_METHODS: &[&[u8]] = &[
    b"alias",
    b"alias_method",
    b"break",
    b"next",
    b"redo",
    b"retry",
    b"return",
    b"super",
    b"undef",
    b"yield",
];

/// The methods that `keyword.other.special-method.ruby` names.
const SPECIAL_METHODS: &[&[u8]] = &[
    b"initialize",
    b"new",
    b"loop",
    b"include",
    b"extend",
    b"prepend",
    b"raise",
    b"fail",
    b"attr_reader",
    b"attr_writer",
    b"attr_accessor",
    b"attr",
    b"catch",
    b"throw",
    b"private",
    b"private_class_method",
    b"module_function",
    b"public",
    b"public_class_method",
    b"protected",
    b"refine",
    b"using",
];

/// The methods of `Kernel` that `support.function.kernel.ruby` names.
const KERNEL: &[&[u8]] = &[
    b"abort",
    b"at_exit",
    b"autoload",
    b"binding",
    b"callcc",
    b"caller",
    b"caller_locations",
    b"chomp",
    b"chop",
    b"eval",
    b"exec",
    b"exit",
    b"fork",
    b"format",
    b"gets",
    b"global_variables",
    b"gsub",
    b"lambda",
    b"load",
    b"local_variables",
    b"open",
    b"p",
    b"print",
    b"printf",
    b"proc",
    b"putc",
    b"puts",
    b"rand",
    b"readline",
    b"readlines",
    b"select",
    b"set_trace_func",
    b"sleep",
    b"spawn",
    b"sprintf",
    b"srand",
    b"sub",
    b"syscall",
    b"system",
    b"test",
    b"trace_var",
    b"trap",
    b"untrace_var",
    b"warn",
];

impl Lexer<'_> {
    /// `\b(words)\b(?![!?])`, with `(?<!\.)` before it when `dot` is
    /// [`Dot::Refused`].
    fn listed_word(&self, p: usize, words: &[&[u8]], dot: Dot) -> Option<usize> {
        if dot == Dot::Refused && !self.not_after_dot(p) {
            return None;
        }
        self.word_in(p, words).filter(|&end| self.no_bang(end))
    }

    /// `\bword\b(?![!?])`.
    fn plain_word(&self, p: usize, word: &[u8]) -> Option<usize> {
        self.keyword_out_of_line(p, word)
            .filter(|&end| self.no_bang(end))
    }

    /// `(?<!\.)\belse(\s)+if\b`
    pub(in crate::ruby) fn else_if(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_dot(p) {
            return None;
        }
        let blank = self.word_then(p, b"else")?;
        let word = self.space_end(blank);
        if word == blank {
            return None;
        }
        self.keyword_out_of_line(word, b"if")
    }

    /// `(?<!\.)\b(BEGIN|begin|…|while)\b(?![!?])`
    pub(in crate::ruby) fn control_word(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, CONTROL, Dot::Refused)
    }

    /// `(?<!\.)\bdo\b`
    pub(in crate::ruby) fn do_word(&mut self, p: usize) -> Option<usize> {
        self.not_after_dot(p)
            .then(|| self.keyword_out_of_line(p, b"do"))
            .flatten()
    }

    /// `(?<!\.)\b(alias|…|yield)\b(?![!?])|\bdefined\?|\
    /// b(block_given|iterator)\?`
    pub(in crate::ruby) fn pseudo_method(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, PSEUDO_METHODS, Dot::Refused)
            .or_else(|| self.word_then(p, b"defined?"))
            .or_else(|| self.word_then(p, b"block_given?"))
            .or_else(|| self.word_then(p, b"iterator?"))
    }

    /// `\bnil\b(?![!?])`
    pub(in crate::ruby) fn nil_word(&mut self, p: usize) -> Option<usize> {
        self.plain_word(p, b"nil")
    }

    /// `\b(true|false)\b(?![!?])`
    pub(in crate::ruby) fn boolean_word(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, &[b"true", b"false"], Dot::Allowed)
    }

    /// `\b(__(FILE|LINE)__)\b(?![!?])`
    pub(in crate::ruby) fn file_line_word(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, &[b"__FILE__", b"__LINE__"], Dot::Allowed)
    }

    /// `\bself\b(?![!?])`
    pub(in crate::ruby) fn self_word(&mut self, p: usize) -> Option<usize> {
        self.plain_word(p, b"self")
    }

    /// `\b(initialize|new|…|using)\b(?![!?])`
    pub(in crate::ruby) fn special_method(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, SPECIAL_METHODS, Dot::Allowed)
    }

    /// `\b(?<!\.|::)(require(?:|_relative))\b(?![!?])`
    pub(in crate::ruby) fn require_word(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_dot(p) || self.prev_is(p, b':') && self.prev_is(p - 1, b':') {
            return None;
        }
        let end = self.listed_word(p, &[b"require", b"require_relative"], Dot::Allowed)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `$|(?=[#}])`
    pub(in crate::ruby) fn require_end(&mut self, p: usize) -> Option<usize> {
        (self.at_eol(p) || self.byte_in(p, |b| b == b'#' || b == b'}')).then_some(p)
    }

    /// `\b((abort|…|warn)\b(?![!?])|autoload\?|exit!)`
    pub(in crate::ruby) fn kernel_method(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, KERNEL, Dot::Allowed)
            .or_else(|| self.word_then(p, b"autoload?"))
            .or_else(|| self.word_then(p, b"exit!"))
    }

    /// `(?<!\.)\b(and|not|or)\b(?![!?])`
    pub(in crate::ruby) fn logical_word(&mut self, p: usize) -> Option<usize> {
        self.listed_word(p, &[b"and", b"not", b"or"], Dot::Refused)
    }

    /// `(sigil)[A-Z_a-z]\w*`: an instance, class or global variable.
    pub(in crate::ruby) fn sigil_name(&mut self, p: usize, sigil: &[u8]) -> Option<usize> {
        let start = self.lit(p, sigil)?;
        let end = self.name_end(start)?;
        self.cap(1, p, start);
        Some(end)
    }

    /// ``(\$)([!\&'+@`]|\d+|["$*,./:-?\\_~]|-[0FIadilpv])``
    pub(in crate::ruby) fn special_global(&mut self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'$') {
            return None;
        }
        let next = p + 1;
        let end = if self.byte_in(next, |b| {
            matches!(b, b'!' | b'&' | b'\'' | b'+' | b'@' | b'`')
        }) {
            next + 1
        } else if self.digit_end(next) > next {
            self.digit_end(next)
        } else if self.byte_in(next, |b| {
            matches!(
                b,
                b'"' | b'$' | b'*' | b',' | b'.' | b'/' | b':'..=b'?' | b'\\' | b'_' | b'~'
            )
        }) {
            next + 1
        } else if self.byte_in(next, |b| b == b'-')
            && self.byte_in(next + 1, |b| {
                matches!(
                    b,
                    b'0' | b'F' | b'I' | b'a' | b'd' | b'i' | b'l' | b'p' | b'v'
                )
            })
        {
            next + 2
        } else {
            return None;
        };
        self.cap(1, p, next);
        Some(end)
    }

    /// `\b(ENV)\[`
    pub(in crate::ruby) fn env_index(&mut self, p: usize) -> Option<usize> {
        let end = self.word_then(p, b"ENV[")?;
        self.cap(1, p, p + 3);
        Some(end)
    }

    /// `\b[A-Z]\w*(?=((\.|::)[A-Za-z]|\[))`
    pub(in crate::ruby) fn constant_before_call(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.byte_in(p, |b| b.is_ascii_uppercase()) {
            return None;
        }
        let end = self.word_end(p + 1);
        let member = |at: usize| self.byte_in(at, |b| b.is_ascii_alphabetic());
        let follows = self.byte_in(end, |b| b == b'[')
            || self.byte_in(end, |b| b == b'.') && member(end + 1)
            || self.starts(end, b"::") && member(end + 2);
        follows.then_some(end)
    }

    /// `\b[A-Z_]\w*\b`
    pub(in crate::ruby) fn constant(&mut self, p: usize) -> Option<usize> {
        (self.boundary(p) && self.byte_in(p, |b| b.is_ascii_uppercase() || b == b'_'))
            .then(|| self.word_end(p + 1))
    }
}

/// Whether a word may follow a `.`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dot {
    /// `(?<!\.)` stands before the word.
    Refused,
    /// No lookbehind stands before it.
    Allowed,
}
