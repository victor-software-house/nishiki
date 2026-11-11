//! The short expressions: keywords, operators, punctuation, literals and
//! the names the nested rules of captures use.

use super::super::engine::Lexer;
use crate::rules::{CachedRuns, Chars};

/// The words of the grammar's `keywords` that stand for themselves.
const CONTROL: &[&[u8]] = &[
    b"break",
    b"case",
    b"continue",
    b"default",
    b"defer",
    b"else",
    b"fallthrough",
    b"for",
    b"go",
    b"goto",
    b"if",
    b"range",
    b"return",
    b"select",
    b"switch",
];

/// The names of the sized numeric types.
const NUMERIC: &[&[u8]] = &[
    b"complex128",
    b"complex64",
    b"float32",
    b"float64",
    b"int",
    b"int16",
    b"int32",
    b"int64",
    b"int8",
    b"uint",
    b"uint16",
    b"uint32",
    b"uint64",
    b"uint8",
];

impl Lexer<'_> {
    /// `\G` at `p`.
    pub(in crate::go) fn at_anchor(&self, p: usize) -> bool {
        self.g && p == self.pos
    }

    /// `\b(word)\s+`: a keyword and the white space after it.
    fn word_and_spaces(&mut self, p: usize, word: &str) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.lit(p, word.as_bytes())?;
        let after = self.spaces(end);
        (after > end).then(|| {
            self.capture(1, p, end);
            after
        })
    }

    /// `\b(package)\s+`
    pub(in crate::go) fn package_name_at0(&mut self, p: usize) -> Option<usize> {
        self.word_and_spaces(p, "package")
    }

    /// `\b(import)\s+`
    pub(in crate::go) fn import_at0(&mut self, p: usize) -> Option<usize> {
        self.word_and_spaces(p, "import")
    }

    /// `(?!\G)`
    pub(in crate::go) fn never_anchor_end(&mut self, p: usize) -> Option<usize> {
        (!self.at_anchor(p)).then_some(p)
    }

    /// `\d\w*`
    pub(in crate::go) fn digit_word(&mut self, p: usize) -> Option<usize> {
        (self.digit_end(p) > p).then(|| self.word_run(p))
    }

    /// `\w+`
    pub(in crate::go) fn word(&mut self, p: usize) -> Option<usize> {
        (p < self.lim() && self.word_at(p).is_some()).then(|| self.word_run(p))
    }

    /// `,`
    pub(in crate::go) fn comma(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b",")
    }

    /// `;`
    pub(in crate::go) fn semicolon(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b";")
    }

    /// `\(`
    pub(in crate::go) fn open_paren(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"(")
    }

    /// `\)`
    pub(in crate::go) fn close_paren(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b")")
    }

    /// `\[`
    pub(in crate::go) fn open_square(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"[")
    }

    /// `]`
    pub(in crate::go) fn close_square(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"]")
    }

    /// `\{`
    pub(in crate::go) fn open_curly(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"{")
    }

    /// `}`
    pub(in crate::go) fn close_curly(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"}")
    }

    /// `:`
    pub(in crate::go) fn colon_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b":")
    }

    /// `'`
    pub(in crate::go) fn rune_quote(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"'")
    }

    /// `"`
    pub(in crate::go) fn double_quote(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"\"")
    }

    /// `` ` ``
    pub(in crate::go) fn backquote(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"`")
    }

    /// `\.(?!\.\.)`
    pub(in crate::go) fn delimiters_at1(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'.' && p < self.lim() && !self.starts(p + 1, b"..")).then_some(p + 1)
    }

    /// `:(?!=)`
    pub(in crate::go) fn delimiters_at2(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b':' && p < self.lim() && self.at(p + 1) != b'=').then_some(p + 1)
    }

    /// `(\s*[.\w]+)?\s*((")([^"]*)("))`: an import with an optional alias.
    pub(in crate::go) fn imports_at0(&mut self, p: usize) -> Option<usize> {
        let alias_start = self.spaces(p);
        let mut alias_end = alias_start;
        while alias_end < self.lim()
            && (self.at(alias_end) == b'.' || self.word_at(alias_end).is_some())
        {
            alias_end = self.char_end(alias_end);
        }
        let (alias, quote) = if alias_end > alias_start {
            let quote = self.spaces(alias_end);
            if self.at(quote) == b'"' && quote < self.lim() {
                (Some(alias_end), quote)
            } else {
                (None, alias_start)
            }
        } else {
            (None, alias_start)
        };
        if self.at(quote) != b'"' || quote >= self.lim() {
            return None;
        }
        let close = self.skip(quote + 1, |b| b != b'"');
        if close >= self.lim() {
            return None;
        }
        if let Some(end) = alias {
            self.capture(1, p, end);
        }
        self.capture(2, quote, close + 1);
        self.capture(3, quote, quote + 1);
        self.capture(4, quote + 1, close);
        self.capture(5, close, close + 1);
        Some(close + 1)
    }

    /// `(/\*)`
    pub(in crate::go) fn comment_block_begin(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/*")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(\*/)`
    pub(in crate::go) fn comment_block_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"*/")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(//)`
    pub(in crate::go) fn comment_line_begin(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"//")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\n|$`
    pub(in crate::go) fn line_end(&mut self, p: usize) -> Option<usize> {
        self.eol_end(p)
    }

    /// The end of `(?:\n|$)` at `p`.
    /// The line feed is there on a line.
    /// It is not there in the text of a capture group, where `$` is
    /// zero-width.
    pub(in crate::go) fn eol_end(&self, p: usize) -> Option<usize> {
        (p == self.lim()).then(|| p + usize::from(self.nl))
    }

    /// `\[](\s+)`
    pub(in crate::go) fn syntax_errors_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"[]")?;
        let after = self.spaces(end);
        (after > end).then(|| {
            self.capture(1, end, after);
            after
        })
    }

    /// `\b0[0-7]*[89]\d*\b`
    pub(in crate::go) fn syntax_errors_at1(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.at(p) != b'0' || p >= self.lim() {
            return None;
        }
        let octal = self.skip(p + 1, |b| matches!(b, b'0'..=b'7'));
        if !matches!(self.at(octal), b'8' | b'9') || octal >= self.lim() {
            return None;
        }
        let end = self.digit_end(octal + 1);
        self.word_at(end).is_none().then_some(end)
    }

    /// `\b(?:(true|false)|(nil)|(iota))\b`
    pub(in crate::go) fn language_constants(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(p, &[b"false", b"iota", b"nil", b"true"])?;
        self.capture(1, p, end);
        match &self.s[p..end] {
            b"true" | b"false" => {}
            b"nil" => {
                self.capture(1, 0, 0);
                self.capture(2, p, end);
            }
            _ => {
                self.capture(1, 0, 0);
                self.capture(3, p, end);
            }
        }
        Some(end)
    }

    /// `\b(map)\b(\[)`
    pub(in crate::go) fn map_types(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"map")?;
        let open = self.lit(end, b"[")?;
        self.capture(1, p, end);
        self.capture(2, end, open);
        Some(open)
    }

    /// `\b(break|case|...)\b`
    pub(in crate::go) fn keywords_at0(&mut self, p: usize) -> Option<usize> {
        self.word_in(p, CONTROL)
    }

    /// `\bchan\b`
    pub(in crate::go) fn chan_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"chan")
    }

    /// `\bconst\b`
    pub(in crate::go) fn const_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"const")
    }

    /// `\bvar\b`
    pub(in crate::go) fn var_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"var")
    }

    /// `\bfunc\b`
    pub(in crate::go) fn func_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"func")
    }

    /// `\binterface\b`
    pub(in crate::go) fn interface_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"interface")
    }

    /// `\bmap\b`
    pub(in crate::go) fn map_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"map")
    }

    /// `\bstruct\b`
    pub(in crate::go) fn struct_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"struct")
    }

    /// `\bimport\b`
    pub(in crate::go) fn import_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"import")
    }

    /// `\btype\b`
    pub(in crate::go) fn type_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"type")
    }

    /// `\bcase\b`
    pub(in crate::go) fn case_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"case")
    }

    /// `\binvalid\b\s+\btype\b`
    pub(in crate::go) fn invalid_type(&mut self, p: usize) -> Option<usize> {
        let word = self.keyword(p, b"invalid")?;
        let next = self.spaces(word);
        if next == word {
            return None;
        }
        self.keyword(next, b"type")
    }

    /// `\bbool\b`
    pub(in crate::go) fn bool_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"bool")
    }

    /// `\bbyte\b`
    pub(in crate::go) fn byte_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"byte")
    }

    /// `\berror\b`
    pub(in crate::go) fn error_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"error")
    }

    /// `\brune\b`
    pub(in crate::go) fn rune_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"rune")
    }

    /// `\bstring\b`
    pub(in crate::go) fn string_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"string")
    }

    /// `\buintptr\b`
    pub(in crate::go) fn uintptr_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"uintptr")
    }

    /// `\bany\b`
    pub(in crate::go) fn any_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"any")
    }

    /// `\bcomparable\b`
    pub(in crate::go) fn comparable_word(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"comparable")
    }

    /// `\b(complex(64|128)|float(32|64)|u?int(8|16|32|64)?)\b`
    pub(in crate::go) fn storage_types_at3(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(p, NUMERIC)?;
        self.capture(1, p, end);
        if self.s[p..end].starts_with(b"complex") {
            self.capture(2, p + 7, end);
        }
        Some(end)
    }

    /// `<-`
    pub(in crate::go) fn arrow(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"<-")
    }

    /// `--`
    pub(in crate::go) fn decrement(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"--")
    }

    /// `\+\+`
    pub(in crate::go) fn increment(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"++")
    }

    /// `\.\.\.`
    pub(in crate::go) fn ellipsis(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"...")
    }

    /// `(?<!\w)[\&*]+(?!\d)(?=[]\[\w]|<-)`
    pub(in crate::go) fn operators_at0(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let end = self.skip(p, |b| b == b'&' || b == b'*');
        if end == p || self.digit_end(end) > end {
            return None;
        }
        let ahead = matches!(self.at(end), b']' | b'[') && end < self.lim()
            || self.word_at(end).is_some()
            || self.starts(end, b"<-");
        ahead.then_some(end)
    }

    /// `(==|!=|<=|>=|<(?!<)|>(?!>))`
    pub(in crate::go) fn operators_at4(&mut self, p: usize) -> Option<usize> {
        let end = if self.starts(p, b"==")
            || self.starts(p, b"!=")
            || self.starts(p, b"<=")
            || self.starts(p, b">=")
        {
            p + 2
        } else if p < self.lim()
            && matches!(self.at(p), b'<' | b'>')
            && self.at(p + 1) != self.at(p)
        {
            p + 1
        } else {
            return None;
        };
        self.capture(1, p, end);
        Some(end)
    }

    /// `(&&|\|\||!)`
    pub(in crate::go) fn operators_at5(&mut self, p: usize) -> Option<usize> {
        let end = if self.starts(p, b"&&") || self.starts(p, b"||") {
            p + 2
        } else if self.at(p) == b'!' && p < self.lim() {
            p + 1
        } else {
            return None;
        };
        self.capture(1, p, end);
        Some(end)
    }

    /// `((?:|[-%*+/:^|]|<<|>>|&\^?)=)`
    pub(in crate::go) fn operators_at6(&mut self, p: usize) -> Option<usize> {
        let end = if self.at(p) == b'=' && p < self.lim() {
            p + 1
        } else if matches!(
            self.at(p),
            b'-' | b'%' | b'*' | b'+' | b'/' | b':' | b'^' | b'|'
        ) && self.at(p + 1) == b'='
            && p + 1 < self.lim()
        {
            p + 2
        } else if (self.starts(p, b"<<") || self.starts(p, b">>"))
            && self.at(p + 2) == b'='
            && p + 2 < self.lim()
        {
            p + 3
        } else if self.at(p) == b'&' && p < self.lim() {
            let caret = usize::from(self.at(p + 1) == b'^' && p + 1 < self.lim());
            if self.at(p + 1 + caret) == b'=' && p + 1 + caret < self.lim() {
                p + 2 + caret
            } else {
                return None;
            }
        } else {
            return None;
        };
        self.capture(1, p, end);
        Some(end)
    }

    /// `([-%*+/])`
    pub(in crate::go) fn operators_at7(&mut self, p: usize) -> Option<usize> {
        (matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/') && p < self.lim()).then(|| {
            self.capture(1, p, p + 1);
            p + 1
        })
    }

    /// `(&(?!\^)|[\^|]|&\^|<<|>>|~)`
    pub(in crate::go) fn operators_at8(&mut self, p: usize) -> Option<usize> {
        let single = p < self.lim()
            && (matches!(self.at(p), b'^' | b'|' | b'~')
                || self.at(p) == b'&' && self.at(p + 1) != b'^');
        let end = if single {
            p + 1
        } else if self.starts(p, b"&^") || self.starts(p, b"<<") || self.starts(p, b">>") {
            p + 2
        } else {
            return None;
        };
        self.capture(1, p, end);
        Some(end)
    }

    /// The ends of an escape `\\` and what follows it, as the rune and string
    /// escapes list them.
    /// The escapes are `\\([0-7]{3}|["'\\abfnrtv]|x\h{2}|u\h{4}|U\h{8})`.
    /// Ends come in the order of the alternatives, `None` where one does
    /// not match.
    /// `after` is the position they are tried from.
    fn escape_alternatives(&self, after: usize) -> [Option<usize>; 5] {
        let octal =
            (0..3).all(|i| matches!(self.at(after + i), b'0'..=b'7') && after + i < self.lim());
        let single = after < self.lim()
            && matches!(
                self.at(after),
                b'"' | b'\'' | b'\\' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'v'
            );
        let hex = [(b'x', 2), (b'u', 4), (b'U', 8)].map(|(letter, digits)| {
            (self.at(after) == letter
                && after < self.lim()
                && (1..=digits).all(|i| self.hex_at(after + i)))
            .then_some(after + 1 + digits)
        });
        let [x, u, big_u] = hex;
        [
            octal.then_some(after + 3),
            single.then_some(after + 1),
            x,
            u,
            big_u,
        ]
    }

    /// `\G(\\([0-7]{3}|["'\\abfnrtv]|x\h{2}|u\h{4}|U\h{8})|.)(?=')`
    pub(in crate::go) fn runes_at0_at0(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        if self.at(p) == b'\\' && p < self.lim() {
            for end in self.escape_alternatives(p + 1).into_iter().flatten() {
                if self.at(end) == b'\'' && end < self.lim() {
                    self.capture(1, p, end);
                    self.capture(2, p + 1, end);
                    return Some(end);
                }
            }
        }
        if p < self.lim() {
            let end = self.char_end(p);
            if self.at(end) == b'\'' && end < self.lim() {
                self.capture(1, p, end);
                return Some(end);
            }
        }
        None
    }

    /// `[^']+`: everything up to the next quote, the line feed included.
    pub(in crate::go) fn runes_at0_at1(&mut self, p: usize) -> Option<usize> {
        if p > self.lim() {
            return None;
        }
        let end = self.skip(p, |b| b != b'\'');
        let end = if end >= self.lim() {
            self.lim() + 1
        } else {
            end
        };
        (end > p).then_some(end)
    }

    /// `\\([0-7]{3}|["'\\abfnrtv]|x\h{2}|u\h{4}|U\h{8})`
    pub(in crate::go) fn string_escaped_char_at0(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let end = self
            .escape_alternatives(p + 1)
            .into_iter()
            .flatten()
            .next()?;
        self.capture(1, p + 1, end);
        Some(end)
    }

    /// `\\[^"'0-7Uabfnrtuvx]`
    pub(in crate::go) fn string_escaped_char_at1(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() || p + 1 > self.lim() {
            return None;
        }
        let next = self.at(p + 1);
        let listed = p + 1 < self.lim()
            && matches!(
                next,
                b'"' | b'\'' | b'0'
                    ..=b'7' | b'U' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'u' | b'v' | b'x'
            );
        if listed {
            return None;
        }
        Some(if p + 1 >= self.lim() {
            p + 2
        } else {
            self.char_end(p + 1)
        })
    }

    /// `(?=.)`
    pub(in crate::go) fn nested4_at0(&mut self, p: usize) -> Option<usize> {
        (p < self.lim()).then_some(p)
    }

    /// `(?<=\h)_(?=\h)`
    pub(in crate::go) fn nested5_at0(&mut self, p: usize) -> Option<usize> {
        let behind = p > 0 && self.hex_at(p - 1);
        (behind && self.at(p) == b'_' && self.hex_at(p + 1)).then_some(p + 1)
    }

    /// `(?:[.0-9A-Z_a-z]|(?<=[EPep])[-+])+`
    pub(in crate::go) fn nested4_at0_at2(&mut self, p: usize) -> Option<usize> {
        let end = self.number_tail(p);
        (end > p).then_some(end)
    }

    /// The end of `(?:[.0-9A-Z_a-z]|(?<=[EPep])[-+])*` from `p`.
    pub(in crate::go) fn number_tail(&self, p: usize) -> usize {
        let mut i = p;
        while i < self.lim() {
            let b = self.at(i);
            let sign = matches!(b, b'-' | b'+')
                && i > 0
                && matches!(self.at(i - 1), b'E' | b'P' | b'e' | b'p');
            if b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || sign {
                i += 1;
            } else {
                break;
            }
        }
        i
    }

    /// `(?<!\w)\.?\d(?:[.0-9A-Z_a-z]|(?<=[EPep])[-+])*`
    pub(in crate::go) fn numeric_literals(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let dotted = p < self.lim() && self.at(p) == b'.' && self.digit_end(p + 1) > p + 1;
        let digit = if dotted { p + 1 } else { p };
        (self.digit_end(digit) > digit).then(|| self.number_tail(self.char_end(digit)))
    }
}
