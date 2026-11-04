//! Keywords, operators, strings, calls and the small rules of code.

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// One of `set` at `p`.
    pub(super) fn one_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        (p < self.lim() && set.contains(&self.s[p])).then_some(p + 1)
    }

    /// `\b(return|yield|break|case|continue|default|do|while|for|switch|if|else)\b`.
    pub(super) fn control_keyword(&self, p: usize) -> Option<usize> {
        self.keyword_of(
            p,
            &[
                b"return",
                b"yield",
                b"break",
                b"case",
                b"continue",
                b"default",
                b"do",
                b"while",
                b"for",
                b"switch",
                b"if",
                b"else",
            ],
        )
    }

    /// `(<<|>>>?|[\^~])`.
    pub(super) fn bitwise_operator(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"<<") {
            return Some(p + 2);
        }
        if self.starts(p, b">>") {
            return Some(p + if self.at(p + 2) == b'>' { 3 } else { 2 });
        }
        self.one_of(p, b"^~")
    }

    /// `(([\&^|]|<<|>>>?)=)`.
    pub(super) fn bitwise_assignment(&self, p: usize) -> Option<usize> {
        let operator = if self.starts(p, b">>>") && self.at(p + 3) == b'=' {
            p + 3
        } else if self.starts(p, b">>") || self.starts(p, b"<<") {
            p + 2
        } else {
            self.one_of(p, b"&^|")?
        };
        (self.at(operator) == b'=').then_some(operator + 1)
    }

    /// `(===?|!=|<=|>=|<>|[<>])`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"==") {
            return Some(p + if self.at(p + 2) == b'=' { 3 } else { 2 });
        }
        for text in [&b"!="[..], b"<=", b">=", b"<>"] {
            if let Some(end) = self.lit(p, text) {
                return Some(end);
            }
        }
        self.one_of(p, b"<>")
    }

    /// `([-%*+/]=)`.
    pub(super) fn arithmetic_assignment(&self, p: usize) -> Option<usize> {
        let end = self.one_of(p, b"-%*+/")?;
        (self.at(end) == b'=').then_some(end + 1)
    }

    /// `(--|\+\+)`.
    pub(super) fn increment(&self, p: usize) -> Option<usize> {
        self.lit(p, b"--").or_else(|| self.lit(p, b"++"))
    }

    /// `(!|&&|\|\|)`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        self.lit(p, b"&&")
            .or_else(|| self.lit(p, b"||"))
            .or_else(|| self.lit(p, b"!"))
    }

    /// A string's escape: `\\.`, and in a text block `(\\""")(?!")` first.
    ///
    /// `.` matches any character but the line feed.
    pub(super) fn escape(&self, p: usize, escapes: Escapes) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        if escapes == Escapes::TextBlock && self.starts(p + 1, b"\"\"\"") && self.at(p + 4) != b'"'
        {
            return Some(p + 4);
        }
        (p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `([$A-Z_a-z][$\w]*)\s*(\()`: a call, with the name and the parenthesis
    /// as groups 1 and 2.
    pub(super) fn call(&mut self, p: usize) -> Option<usize> {
        let name = self.ident_end(p)?;
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, name);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `(\.)\s*([$A-Z_a-z][$\w]*)\s*(\()`: a method call.
    pub(super) fn method_call(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'.' {
            return None;
        }
        let name = self.space_end(p + 1);
        let name_end = self.ident_end(name)?;
        let open = self.space_end(name_end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, name, name_end);
        self.cap(3, open, open + 1);
        Some(open + 1)
    }

    /// `(?<![$\w])[$A-Z_a-z][$\w]*(?=\s*\.\s*[$\w]+)`: an object before a dot.
    pub(super) fn object(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') || self.word_before(p) {
            return None;
        }
        let end = self.ident_end(p)?;
        let dot = self.space_end(end);
        if self.at(dot) != b'.' {
            return None;
        }
        let next = self.space_end(dot + 1);
        (self.name_end(next) > next).then_some(end)
    }

    /// `(\.)\s*(new)`, and the property patterns `(\.)\s*(name)`, with the
    /// kind of name that follows the dot.
    pub(super) fn property(&mut self, p: usize, kind: Property) -> Option<usize> {
        if self.at(p) != b'.' {
            return None;
        }
        let name = self.space_end(p + 1);
        let end = match kind {
            Property::New => self.lit(name, b"new")?,
            Property::Chained | Property::Plain => self.ident_end(name)?,
            Property::Digit => {
                if !self.at(name).is_ascii_digit() {
                    return None;
                }
                self.name_end(name + 1)
            }
        };
        if kind == Property::Chained {
            let dot = self.space_end(end);
            if self.at(dot) != b'.' {
                return None;
            }
            let next = self.space_end(dot + 1);
            self.ident_end(next)?;
        }
        self.cap(1, p, p + 1);
        self.cap(2, name, end);
        Some(end)
    }

    /// `([$A-Z_a-z][$\w]*)(?=\s*(\[])*\s*([,:;=]))` and its `var` twin: a
    /// name that a declaration ends at.
    /// `terminators` are the bytes that may end it.
    pub(super) fn definition(&mut self, p: usize, terminators: &[u8]) -> Option<usize> {
        let end = self.ident_end(p)?;
        let mut i = self.space_end(end);
        while self.starts(i, b"[]") {
            i += 2;
        }
        i = self.space_end(i);
        self.one_of(i, terminators)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(assert)\s`: the keyword and one white space character.
    pub(super) fn assert_begin(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"assert")?;
        let width = self.space_at(end);
        if width == 0 {
            return None;
        }
        self.cap(1, p, end);
        Some(end + width)
    }

    /// `$`: before the line feed or at the end of the text.
    pub(super) fn end_of_line(&self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// The end of the lookahead `(?=[])-.:;?}]|/(?![*/])|[!%\&=^|])` that
    /// ends a `new` expression.
    pub(super) fn new_expression_end(&self, p: usize) -> Option<usize> {
        let b = self.at(p);
        let stops = b"])*+,-.:;?}!%&=^|";
        if p < self.lim() && stops.contains(&b) {
            return Some(p);
        }
        (b == b'/' && p < self.lim() && !matches!(self.at(p + 1), b'*' | b'/')).then_some(p)
    }
}

/// Which escapes a string takes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Escapes {
    /// `\\.` only, in a string on one line.
    String,
    /// `(\\""")(?!")` too, in a text block.
    TextBlock,
}

/// Which property pattern is being matched.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Property {
    /// `(\.)\s*(new)`.
    New,
    /// A name that another dotted name follows.
    Chained,
    /// Any name.
    Plain,
    /// A name that starts with a digit.
    /// Such a name is invalid.
    Digit,
}
