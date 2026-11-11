//! Names: the lazy `[7A-Z_a-z][0-9A-Z_a-z]*?\b` of the grammar, and the
//! patterns built on it.

use super::super::engine::Lexer;
use crate::case_fold;
use crate::rules::Chars;

/// Which characters a name's class takes.
#[derive(Clone, Copy)]
enum Class {
    /// `[7A-Z_a-z]` and `[0-9A-Z_a-z]` as written, outside `(?i)`.
    Ascii,
    /// The same classes under `(?i)`.
    /// They also take `ſ` and the Kelvin sign, whose folds are one ASCII
    /// letter.
    /// `ß` and the ligatures fold to two letters, and a class takes none of
    /// them.
    Folded,
}

impl Lexer<'_> {
    /// The width of the character at `p` when the name class takes it.
    /// `start` gives `[7A-Z_a-z]`, else `[0-9A-Z_a-z]`.
    fn name_char(&self, p: usize, class: Class, start: bool) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let b = self.at(p);
        if b.is_ascii() {
            let takes = b == b'_'
                || b.is_ascii_alphabetic()
                || if start { b == b'7' } else { b.is_ascii_digit() };
            return takes.then_some(1);
        }
        match class {
            Class::Ascii => None,
            Class::Folded => {
                let (c, end) = self.char_at(p)?;
                (case_fold::letters(c)?.len() == 1).then_some(end - p)
            }
        }
    }

    /// The end of `[7A-Z_a-z][0-9A-Z_a-z]*?\b` at `p`.
    ///
    /// Every character the class takes is `\w`, so `\b` holds only where the
    /// class run ends, before a character that is not `\w`.
    /// The lazy run therefore ends with the class run.
    /// A `\w` that the class does not take, such as `é`, ends the run where
    /// `\b` fails, and nothing matches at `p`.
    fn lazy_name(&self, p: usize, class: Class) -> Option<usize> {
        let mut end = p + self.name_char(p, class, true)?;
        while let Some(width) = self.name_char(end, class, false) {
            end += width;
        }
        self.spend(end - p);
        self.word_at(end).is_none().then_some(end)
    }

    /// `\b` and then a folded lazy name at `p`.
    fn bounded_name(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.lazy_name(p, Class::Folded)
    }

    /// `^\s*((?i:function|sub))\s*([A-Z_a-z]\w*)\s*(\()([^)]*)(\)).*\n?`: a
    /// function head on one line.
    ///
    /// The keyword needs no `\b`, so `Subroutine(` reads as `Sub` and
    /// `routine`.
    /// Only the keyword is under `(?i)`; the name's class is ASCII.
    /// `[^)]*` runs to the first `)`, and the head fails without one.
    pub(super) fn function_head(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let keyword = self.space_end(p).min(self.lim());
        let text = self.text(0, self.lim());
        let keyword_end = case_fold::literal_end(text, keyword, b"function")
            .or_else(|| case_fold::literal_end(text, keyword, b"sub"))?;
        let name = self.space_end(keyword_end);
        if !(name < self.lim() && (self.at(name) == b'_' || self.at(name).is_ascii_alphabetic())) {
            return None;
        }
        let name_end = self.word_end(name + 1);
        let open = self.space_end(name_end);
        let params = self.lit(open, b"(")?;
        let close = self.skip(params, |b| b != b')');
        let after = self.lit(close, b")")?;
        self.capture(1, keyword, keyword_end);
        self.capture(2, name, name_end);
        self.capture(3, open, params);
        self.capture(4, params, close);
        self.capture(5, close, after);
        let end = self.lim();
        Some(if self.at(end) == b'\n' { end + 1 } else { end })
    }

    /// `(?i:(dim)\s*\b(name)\b\s*(,?))`: a `Dim` and the name it declares.
    ///
    /// `dim` needs no `\b` before it.
    /// The `\b` before the name needs a blank after `dim`, since both are
    /// `\w`.
    /// The trailing `\s*` can take the line feed.
    pub(super) fn dim_name(&mut self, p: usize) -> Option<usize> {
        let keyword = case_fold::literal_end(self.text(0, self.lim()), p, b"dim")?;
        let name = self.space_end(keyword);
        if name == keyword {
            return None;
        }
        let name_end = self.lazy_name(name, Class::Folded)?;
        let comma = self.space_end(name_end);
        let end = if self.starts(comma, b",") {
            comma + 1
        } else {
            comma
        };
        self.capture(1, p, keyword);
        self.capture(2, name, name_end);
        self.capture(3, comma, end);
        Some(end)
    }

    /// `(\$)[7A-Z_a-z][0-9A-Z_a-z]*?\b\s*`: a variable after `$`.
    /// It sits outside `(?i)`, so its classes are ASCII.
    pub(super) fn dollar_variable(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"$")?;
        let name_end = self.lazy_name(name, Class::Ascii)?;
        self.capture(1, p, name);
        Some(self.space_end(name_end))
    }

    /// `(?i:(?<=as )\b(name)\b)`: the type after `As`.
    /// The lookbehind reads `as ` under `(?i)`, where `ſ` stands for `s`.
    pub(super) fn as_type(&mut self, p: usize) -> Option<usize> {
        let before = |width: usize| p.checked_sub(width).map(|start| self.text(start, p));
        let plain = before(3).is_some_and(|text| text.eq_ignore_ascii_case(b"as "));
        let long_s = before(4).is_some_and(|text| {
            text.first().is_some_and(|b| b.eq_ignore_ascii_case(&b'a'))
                && text.get(1..) == Some("ſ ".as_bytes())
        });
        if !(plain || long_s) {
            return None;
        }
        let end = self.bounded_name(p)?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i:\b(name)\b(?=\(\)?))`: a name before `(`.
    pub(super) fn function_call(&mut self, p: usize) -> Option<usize> {
        let end = self.bounded_name(p)?;
        if !self.starts(end, b"(") {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i:((?<=([-\&(+,/<=>\\]))\s*\b(name)\b(?!([(.]))|\b(name)\b(?=\s*([-\&
    /// ()+/<=>\\]))))`: a variable beside an operator.
    ///
    /// The first branch starts right after an operator character, blanks
    /// included.
    /// Only the first blank of a run has the operator before it.
    /// The second branch needs an operator character after the name and its
    /// blanks.
    /// A blank is never one, so the blanks give nothing back.
    pub(super) fn variable(&self, p: usize) -> Option<usize> {
        if matches!(
            self.prev(p),
            Some(b'-' | b'&' | b'(' | b'+' | b',' | b'/' | b'<' | b'=' | b'>' | b'\\')
        ) {
            let name = self.space_end(p);
            if let Some(end) = self.bounded_name(name)
                && !(end < self.lim() && matches!(self.at(end), b'(' | b'.'))
            {
                return Some(end);
            }
        }
        let end = self.bounded_name(p)?;
        let next = self.space_end(end).min(self.lim());
        (next < self.lim()
            && matches!(
                self.at(next),
                b'-' | b'&' | b'(' | b')' | b'+' | b'/' | b'<' | b'=' | b'>' | b'\\'
            ))
        .then_some(end)
    }
}
