//! Node paths: `$Node/Child`, `%Unique`, `$"quoted"`, `NodePath("…")` and
//! `get_node("…")`.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The functions that take a node path, in the grammar's order.
/// `has_node` comes before `has_node_and_resource`, so the longer name
/// matches only where `\s*\(` does not follow the shorter one.
const NODE_PATH_FUNCTIONS: [&[u8]; 5] = [
    b"get_node_or_null",
    b"has_node",
    b"has_node_and_resource",
    b"find_node",
    b"get_node",
];

impl Lexer<'_> {
    /// `(?:([$%])|([\&@^]))(["'])`: a quoted node path, or a quoted string
    /// name.
    pub(super) fn quoted_node(&mut self, p: usize) -> Option<usize> {
        let sigil = self.at(p);
        let group = match sigil {
            b'$' | b'%' => 1,
            b'&' | b'@' | b'^' => 2,
            _ => return None,
        };
        if !matches!(self.at(p + 1), b'"' | b'\'') {
            return None;
        }
        self.capture(group, p, p + 1);
        self.capture(3, p + 1, p + 2);
        Some(p + 2)
    }

    /// `\3`, `\2` or `\1`: the quote that opened the rule.
    pub(super) fn repeated(&self, p: usize) -> Option<usize> {
        self.payload_at(p).map(|width| p + width)
    }

    /// The end of `(\$\s*|%|\$%\s*)` at `p` whose rest can match: the
    /// alternatives in order.
    ///
    /// `\$\s*` before `%` leaves a `%` where `/` or a name must follow, so the
    /// third alternative takes `$%`.
    /// A `$` followed by blanks and then `%` matches no alternative.
    fn node_sigil(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'%' if p < self.lim() => Some(p + 1),
            b'$' if p < self.lim() => {
                if self.starts(p + 1, b"%") {
                    Some(self.space_end(p + 2))
                } else {
                    Some(self.space_end(p + 1))
                }
            }
            _ => None,
        }
    }

    /// `(sigil)(/\s*)?([A-Z_a-z]\w*)` at `p`: the head of a bare node path,
    /// with groups 1 to 3.
    /// It returns the end of the name.
    fn node_name(&mut self, p: usize) -> Option<usize> {
        let sigil = self.node_sigil(p)?;
        let (slash, name) = if self.starts(sigil, b"/") {
            (Some(sigil), self.space_end(sigil + 1))
        } else {
            (None, sigil)
        };
        let end = self.name(name)?;
        self.capture(1, p, sigil);
        if let Some(slash) = slash {
            self.capture(2, slash, name);
        }
        self.capture(3, name, end);
        Some(end)
    }

    /// `(?<!/\s*)(\$\s*|%|\$%\s*)(/\s*)?([A-Z_a-z]\w*)\b(?!\s*/)`: a bare node
    /// path of one name.
    ///
    /// Oniguruma compiles the lookbehind `(?<!/\s*)` although its length
    /// varies: no `/` may come before `p` across any run of `\s`.
    /// The name ends where `\b` holds, so nothing gives back to
    /// `(?!\s*/)`.
    pub(super) fn bare_node(&mut self, p: usize) -> Option<usize> {
        if self.slash_before(p) {
            return None;
        }
        let end = self.node_name(p)?;
        (!self.starts(self.space_end(end), b"/")).then_some(end)
    }

    /// Whether a `/` comes before `p` across a run of `\s`: `(?<=/\s*)`.
    fn slash_before(&self, p: usize) -> bool {
        let mut at = p;
        while at > 0 {
            let start = self.char_start_before(at);
            if self.at(start) == b'/' {
                return true;
            }
            if self.space_at(start) == 0 {
                return false;
            }
            at = start;
        }
        false
    }

    /// `(\$\s*|%|\$%\s*)(/\s*)?([A-Z_a-z]\w*)`: the head of a node path of
    /// several names.
    pub(super) fn node_head(&mut self, p: usize) -> Option<usize> {
        self.node_name(p)
    }

    /// `(?!\s*/\s*%?\s*[A-Z_a-z]\w*)`: the end of a node path of several
    /// names, where no further `/name` follows.
    /// Without the name, `%?` gives way, and the name then fails at the `%`.
    pub(super) fn node_path_end(&self, p: usize) -> Option<usize> {
        let slash = self.space_end(p);
        if !self.starts(slash, b"/") {
            return Some(p);
        }
        let mut name = self.space_end(slash + 1);
        if self.starts(name, b"%") {
            name = self.space_end(name + 1);
        }
        self.name(name).is_none().then_some(p)
    }

    /// `(/)\s*(%)?\s*([A-Z_a-z]\w*)\s*`: one more name of a node path.
    pub(super) fn node_segment(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"/")?;
        let mut name = self.space_end(after);
        let percent = self.starts(name, b"%").then_some(name);
        if let Some(percent) = percent {
            name = self.space_end(percent + 1);
        }
        let end = self.name(name)?;
        self.capture(1, p, after);
        if let Some(percent) = percent {
            self.capture(2, percent, percent + 1);
        }
        self.capture(3, name, end);
        Some(self.space_end(end))
    }

    /// `(NodePath)\s*\(`.
    pub(super) fn node_path_object(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"NodePath")?;
        let end = self.lit(self.space_end(name), b"(")?;
        self.capture(1, p, name);
        Some(end)
    }

    /// `(["'])`: the quote that opens a node path.
    pub(super) fn quote_group(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() || !matches!(self.at(p), b'"' | b'\'') {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 1)
    }

    /// `(get_node_or_null|has_node|…|get_node)\s*(\()`: the first name of
    /// the alternation that `\s*\(` follows.
    pub(super) fn node_path_function(&mut self, p: usize) -> Option<usize> {
        let (name, open) = NODE_PATH_FUNCTIONS.iter().find_map(|&name| {
            let end = self.lit(p, name)?;
            let open = self.space_end(end);
            self.starts(open, b"(").then_some((end, open))
        })?;
        self.capture(1, p, name);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }
}
