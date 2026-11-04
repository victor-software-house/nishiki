//! The lookaheads that decide whether a `{` opens an attribute set or a
//! function head, and whether a name starts a function.

use super::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of bytes from `p` that are none of `stops`.
    /// It ends at the line's end, after the line feed when there is one.
    fn run_until(&self, p: usize, stops: &[u8]) -> usize {
        let mut i = p;
        while self.avail(i) && !stops.contains(&self.at(i)) {
            i += 1;
        }
        self.spend(i - p);
        i
    }

    /// Whether `[,?]` follows some part of the run of bytes from `p` that are
    /// none of `stops`, with `stops` free of `,` and `?`.
    ///
    /// The run is greedy and gives bytes back until `[,?]` follows it.
    /// That happens when the run holds a `,` or a `?`.
    fn run_holds(&self, p: usize, stops: &[u8], wanted: &[u8]) -> bool {
        let end = self.run_until(p, stops);
        (p..end).any(|i| wanted.contains(&self.at(i)))
    }

    /// `(?=(\b[A-Z_a-z][-'0-9A-Z_a-z]*\s*[:@]|\{[^"'}]*}\s*:|\{[^"#'/=}]*[,?
    /// ]))`.
    ///
    /// The first branch: a name, then `:` or `@`.
    /// The second: a brace, a run free of quotes and braces, its `}` and a `:`.
    /// The third: a brace and a run free of quotes, `#`, `/`, `=` and `}` that
    /// holds a `,` or a `?`.
    pub(super) fn function_ahead(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.name(p) {
            let next = self.space_end(end);
            if matches!(self.at(next), b':' | b'@') {
                return Some(p);
            }
        }
        if self.at(p) != b'{' {
            return None;
        }
        let close = self.run_until(p + 1, b"\"'}");
        if self.at(close) == b'}' && self.at(self.space_end(close + 1)) == b':' {
            return Some(p);
        }
        self.run_holds(p + 1, b"\"#'/=}", b",?").then_some(p)
    }

    /// `(?=\{\s*(}|[^,?]*([;=])))`.
    ///
    /// After the brace and its blanks: a `}`, or a run free of `,` and `?`
    /// that holds a `;` or a `=`.
    /// The blanks the `\s*` gives back fall in that run too.
    pub(super) fn attrset_for_sure(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        if self.at(self.space_end(p + 1)) == b'}' {
            return Some(p);
        }
        self.run_holds(p + 1, b",?", b";=").then_some(p)
    }

    /// `(?=(\s*}|"|\binherit\b|\$\{|\b[A-Z_a-z][-'0-9A-Z_a-z]*(\s*\.|\s*
    /// =[^=])))`.
    ///
    /// Inside a brace, what an attribute set holds and a function head does
    /// not: a closing brace, a string, `inherit`, an interpolation, or a name
    /// before a `.` or a `=` that is not `==`.
    pub(super) fn attrset_ahead(&self, p: usize) -> Option<usize> {
        if self.at(self.space_end(p)) == b'}'
            || self.at(p) == b'"'
            || self.keyword(p, b"inherit").is_some()
            || self.starts(p, b"${")
        {
            return Some(p);
        }
        let end = self.name(p)?;
        let next = self.space_end(end);
        let binds = match self.at(next) {
            b'.' => true,
            b'=' => self.avail(next + 1) && self.at(next + 1) != b'=',
            _ => false,
        };
        binds.then_some(p)
    }

    /// `(?=(\.\.\.|\b[A-Z_a-z][-'0-9A-Z_a-z]*\s*[,?]))`: what only a function
    /// head holds, `...` or a parameter before a `,` or a `?`.
    pub(super) fn parameters_ahead(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"...") {
            return Some(p);
        }
        let end = self.name(p)?;
        matches!(self.at(self.space_end(end)), b',' | b'?').then_some(p)
    }
}
