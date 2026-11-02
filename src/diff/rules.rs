//! The grammar's rules as hand-written matchers.
//!
//! Every rule of the grammar is one regular expression over a single line,
//! and all but one start with `^`, so a line is the first rule that matches
//! its start, in the grammar's order, and then only the one unanchored rule
//! can match the rest of it. Each matcher below answers what the regular
//! expression would, including where its backtracking lands, and the scopes
//! of its capture groups.

use memchr::memmem;

use super::scope::S;
use crate::classes;
use crate::classes::is_ascii_space;
use crate::utf8::char_len;

/// A capture group that carries a scope of its own.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
}

/// A rule's match: the bytes `start..end` under `scope`, and up to three
/// capture groups, in order, that add a scope to theirs.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) scope: S,
    pub(super) caps: [Option<Cap>; 3],
}

impl Hit {
    fn whole(end: usize, scope: S) -> Hit {
        Hit {
            start: 0,
            end,
            scope,
            caps: [None; 3],
        }
    }

    fn with(mut self, index: usize, from: usize, to: usize, scope: S) -> Hit {
        self.caps[index] = Some(Cap { from, to, scope });
        self
    }
}

/// A line and the bytes its scans have read.
pub(super) struct Line<'a> {
    s: &'a [u8],
    /// Bytes read ahead of the cursor so far, for the work limit.
    pub(super) spent: u64,
}

impl<'a> Line<'a> {
    pub(super) fn new(s: &'a [u8]) -> Self {
        Line { s, spent: 0 }
    }

    fn spend(&mut self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.spent = self.spent.saturating_add(bytes);
    }

    /// The character at `i`, which is at a boundary, and its width.
    fn char_at(&self, i: usize) -> Option<(char, usize)> {
        let width = char_len(*self.s.get(i)?);
        let text = std::str::from_utf8(self.s.get(i..i + width)?).ok()?;
        Some((text.chars().next()?, width))
    }

    /// The width of the white space character at `i`, 0 when there is none.
    fn space_at(&self, i: usize) -> usize {
        let Some(&b) = self.s.get(i) else { return 0 };
        if b < 0x80 {
            return usize::from(is_ascii_space(b));
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_space(c) => width,
            _ => 0,
        }
    }

    /// The width of the character at `i` that `\d` matches, 0 when none.
    fn digit_at(&self, i: usize) -> usize {
        let Some(&b) = self.s.get(i) else { return 0 };
        if b < 0x80 {
            return usize::from(b.is_ascii_digit());
        }
        match self.char_at(i) {
            Some((c, width)) if classes::is_digit(c) => width,
            _ => 0,
        }
    }

    /// The end of the run of `\d` characters from `i`.
    fn digits(&mut self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.digit_at(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The end of the run of white space from `i`.
    fn spaces(&mut self, mut i: usize) -> usize {
        let from = i;
        while let width @ 1.. = self.space_at(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The start of the run of white space that ends at `i`, no earlier than
    /// `floor`.
    fn spaces_back(&self, mut i: usize, floor: usize) -> usize {
        while i > floor {
            let mut start = i - 1;
            while start > floor && self.s[start] & 0xc0 == 0x80 {
                start -= 1;
            }
            if self.space_at(start) == 0 {
                break;
            }
            i = start;
        }
        i
    }

    fn starts(&self, prefix: &[u8]) -> bool {
        self.s.starts_with(prefix)
    }

    fn len(&self) -> usize {
        self.s.len()
    }

    /// The rule that matches at the start of the line, the first in the
    /// grammar's order.
    pub(super) fn at_start(&mut self) -> Option<Hit> {
        let first = *self.s.first()?;
        match first {
            b'*' => self
                .separator()
                .or_else(|| self.context_range())
                .or_else(|| self.header_from()),
            b'=' => self.separator().or_else(|| self.header_from()),
            b'-' => self
                .separator()
                .or_else(|| self.context_range())
                .or_else(|| self.header_from())
                .or_else(|| self.deleted()),
            b'@' => self.unified_range(),
            b'd' => self.header_git().or_else(|| self.header_command()),
            b'+' => self.header_to().or_else(|| self.inserted()),
            b'>' => self.inserted(),
            b'!' => Some(Hit::whole(self.len(), S::Changed).with(0, 0, 1, S::PunctChanged)),
            b'<' => self.deleted(),
            b'#' => Some(Hit::whole(self.len(), S::Comment).with(0, 0, 1, S::PunctComment)),
            b'i' => self.index_git(),
            b'I' => self.index(),
            b'O' => self.only_in(),
            _ => self.normal_range(),
        }
    }

    /// `^((\*{15})|(={67})|(-{3}))$`
    fn separator(&self) -> Option<Hit> {
        let len = self.len();
        let (byte, count) = match self.s[0] {
            b'*' => (b'*', 15),
            b'=' => (b'=', 67),
            _ => (b'-', 3),
        };
        (len == count && self.s.iter().all(|&b| b == byte))
            .then(|| Hit::whole(len, S::MetaSeparator).with(0, 0, len, S::PunctSeparator))
    }

    /// `^\d+(,\d+)*(a|d|c)\d+(,\d+)*$`
    fn normal_range(&mut self) -> Option<Hit> {
        let mut i = self.digits(0);
        if i == 0 {
            return None;
        }
        i = self.number_list(i);
        if !matches!(self.s.get(i), Some(b'a' | b'd' | b'c')) {
            return None;
        }
        let digits = self.digits(i + 1);
        if digits == i + 1 {
            return None;
        }
        i = self.number_list(digits);
        (i == self.len()).then(|| Hit::whole(i, S::MetaRangeNormal))
    }

    /// `(,\d+)*` from `i`.
    fn number_list(&mut self, mut i: usize) -> usize {
        while self.s.get(i) == Some(&b',') {
            let end = self.digits(i + 1);
            if end == i + 1 {
                break;
            }
            i = end;
        }
        i
    }

    /// `^(@@)\s*(.+?)\s*(@@)($\n?)?`
    ///
    /// The first `\s*` is greedy and gives back one character only when the
    /// closing `@@` follows it directly. The lazy group ends at the first
    /// place where white space and then `@@` follow, which is the first `@@`
    /// at least one character past the group's start, with the white space
    /// before it left to the second `\s*`.
    fn unified_range(&mut self) -> Option<Hit> {
        if !self.starts(b"@@") {
            return None;
        }
        let start = self.spaces(2);
        let floor = start + char_len(*self.s.get(start)?);
        let found = memmem::find(self.s.get(floor..)?, b"@@").map(|at| at + floor);
        self.spend(self.len() - start);
        let (from, to, close) = if let Some(close) = found {
            (start, self.spaces_back(close, floor), close)
        } else if start > 2 && self.s[start..].starts_with(b"@@") {
            let mut last = start - 1;
            while self.s[last] & 0xc0 == 0x80 {
                last -= 1;
            }
            (last, start, start)
        } else {
            return None;
        };
        Some(
            Hit::whole(close + 2, S::MetaRangeUnified)
                .with(0, 0, 2, S::PunctRange)
                .with(1, from, to, S::LineNumber)
                .with(2, close, close + 2, S::PunctRange),
        )
    }

    /// `^(((\-{3}) .+ (\-{4}))|((\*{3}) .+ (\*{4})))$`
    fn context_range(&self) -> Option<Hit> {
        let len = self.len();
        let (open, close): (&[u8], &[u8]) = if self.starts(b"--- ") {
            (b"---", b" ----")
        } else if self.starts(b"*** ") {
            (b"***", b" ****")
        } else {
            return None;
        };
        (len >= 10 && self.s.ends_with(close)).then(|| {
            Hit::whole(len, S::MetaRangeContext)
                .with(0, 0, open.len(), S::PunctRange)
                .with(1, len - 4, len, S::PunctRange)
        })
    }

    /// `^diff --git a/.*$`
    fn header_git(&self) -> Option<Hit> {
        self.starts(b"diff --git a/")
            .then(|| Hit::whole(self.len(), S::MetaHeaderGit))
    }

    /// `^diff (-|\S+\s+\S+).*$`
    fn header_command(&mut self) -> Option<Hit> {
        if !self.starts(b"diff ") {
            return None;
        }
        let len = self.len();
        if self.s.get(5) != Some(&b'-') {
            let mut i = 5;
            while i < len && self.space_at(i) == 0 {
                i += char_len(self.s[i]);
            }
            self.spend(i - 5);
            if i == 5 {
                return None;
            }
            let after = self.spaces(i);
            if after == i || after >= len {
                return None;
            }
        }
        Some(Hit::whole(len, S::MetaHeaderCommand))
    }

    /// `(^(((-{3}) .+)|((\*{3}) .+))$|^(={4}) .+(?= - ))`
    fn header_from(&mut self) -> Option<Hit> {
        let len = self.len();
        if (self.starts(b"--- ") || self.starts(b"*** ")) && len >= 5 {
            return Some(Hit::whole(len, S::MetaFromFile).with(0, 0, 3, S::PunctFromFile));
        }
        if !self.starts(b"==== ") {
            return None;
        }
        self.spend(len);
        let at = memmem::rfind(self.s.get(6..)?, b" - ")? + 6;
        Some(Hit::whole(at, S::MetaFromFile).with(0, 0, 4, S::PunctFromFile))
    }

    /// `(^(\+{3}) .+$|...)`: the first alternative, anchored.
    fn header_to(&self) -> Option<Hit> {
        let len = self.len();
        (self.starts(b"+++ ") && len >= 5)
            .then(|| Hit::whole(len, S::MetaToFile).with(0, 0, 3, S::PunctToFile))
    }

    /// ` (-) .* (={4})$` from `from`: the grammar's one rule without `^`,
    /// which matches from the first ` - ` that leaves room for ` ====` at the
    /// end of the line.
    pub(super) fn header_to_inline(&mut self, from: usize) -> Option<Hit> {
        let len = self.len();
        self.spend(len.saturating_sub(from));
        if !self.s.ends_with(b" ====") || len < from + 8 {
            return None;
        }
        let at = memmem::find(&self.s[from..len - 5], b" - ")? + from;
        Some(Hit {
            start: at,
            end: len,
            scope: S::MetaToFile,
            caps: [
                Some(Cap {
                    from: at + 1,
                    to: at + 2,
                    scope: S::PunctToFile,
                }),
                Some(Cap {
                    from: len - 4,
                    to: len,
                    scope: S::PunctToFile,
                }),
                None,
            ],
        })
    }

    /// `^(((>)( .*)?)|((\+).*))$`
    fn inserted(&self) -> Option<Hit> {
        let len = self.len();
        let ok = match self.s[0] {
            b'+' => true,
            b'>' => len == 1 || self.s[1] == b' ',
            _ => false,
        };
        ok.then(|| Hit::whole(len, S::Inserted).with(0, 0, 1, S::PunctInserted))
    }

    /// `^(((<)( .*)?)|((-).*))$`
    fn deleted(&self) -> Option<Hit> {
        let len = self.len();
        let ok = match self.s[0] {
            b'-' => true,
            b'<' => len == 1 || self.s[1] == b' ',
            _ => false,
        };
        ok.then(|| Hit::whole(len, S::Deleted).with(0, 0, 1, S::PunctDeleted))
    }

    /// `^index [0-9a-f]{7,40}\.\.[0-9a-f]{7,40}.*$`
    fn index_git(&mut self) -> Option<Hit> {
        if !self.starts(b"index ") {
            return None;
        }
        let first = self.hex_run(6);
        if !(7..=40).contains(&(first - 6)) || !self.s[first..].starts_with(b"..") {
            return None;
        }
        let second = self.hex_run(first + 2);
        (second - first - 2 >= 7).then(|| Hit::whole(self.len(), S::MetaIndexGit))
    }

    /// The end of the run of `[0-9a-f]` from `i`.
    fn hex_run(&mut self, i: usize) -> usize {
        let run = self.s[i..]
            .iter()
            .take_while(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
            .count();
        self.spend(run);
        i + run
    }

    /// `^Index(:) (.+)$`
    fn index(&self) -> Option<Hit> {
        let len = self.len();
        (self.starts(b"Index: ") && len >= 8).then(|| {
            Hit::whole(len, S::MetaIndex)
                .with(0, 5, 6, S::PunctKeyValue)
                .with(1, 7, len, S::FileName)
        })
    }

    /// `^Only in .*: .*$`
    fn only_in(&mut self) -> Option<Hit> {
        if !self.starts(b"Only in ") {
            return None;
        }
        self.spend(self.len());
        memmem::find(&self.s[8..], b": ").map(|_| Hit::whole(self.len(), S::MetaOnlyIn))
    }
}
