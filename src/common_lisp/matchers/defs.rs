//! The matchers of the `def-name` patterns: a defining word, blanks and the
//! name it defines.
//!
//! Each starts with `(?i)(?<=^|[(\s])(word)\s+`.
//! No word holds a delimiter, so the word is the run before the next one, and
//! that delimiter must be a blank.
//! Giving back a blank of `\s+` would start what follows at a blank, which
//! none of them takes.

use super::super::engine::Lexer;
use super::super::words;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The defining word of `words` at `p` and where the blanks after it end.
    fn head(&mut self, p: usize, words: &[&[u8]]) -> Option<(usize, usize)> {
        let word = self.listed(p, self.after_open(p), words)?;
        (self.space_at(word) > 0).then(|| (word, self.space_end(word)))
    }

    /// `((,(?:@|\.?))?)` and a delimited symbol at `i`: where the symbol
    /// starts and ends.
    ///
    /// The comma takes its `@` or `.` first.
    /// When no symbol follows that, it gives the character back, and the
    /// symbol starts at it: `(defun ,. x)` names `.`.
    /// A comma is no symbol character, so the group cannot be left empty
    /// before one.
    fn prefixed_symbol(&self, i: usize) -> Option<(usize, usize)> {
        if self.at(i) != b',' {
            return self.delimited_symbol(i).map(|end| (i, end));
        }
        if matches!(self.at(i + 1), b'@' | b'.')
            && let Some(end) = self.delimited_symbol(i + 2)
        {
            return Some((i + 2, end));
        }
        self.delimited_symbol(i + 1).map(|end| (i + 1, end))
    }

    /// `DefNameAt0`: a function definition, `(def…)\s+` and either
    /// `\(\s*(sym)\s*((,…)?)(sym+?)` or `((,…)?)(sym+?)`, before a
    /// delimiter.
    /// The groups are:
    /// - 1, the word;
    /// - 3, the head of a list name such as `setf`;
    /// - 4 and 7, a comma prefix;
    /// - 6 and 9, the name, which is lexed again.
    ///
    /// In the list form, group 3 is greedy.
    /// When no name follows it, it gives back its last character, which
    /// becomes the name when a delimiter follows: `(defun (foo) …)` names
    /// `o`.
    /// It cannot give back more: every shorter split ends the name at the
    /// same place.
    pub(super) fn def_function(&mut self, p: usize) -> Option<usize> {
        let (word, blanks) = self.head(p, words::DEF_FUNCTIONS)?;
        let end = if self.at(blanks) == b'(' {
            let inner = self.space_end(blanks + 1);
            let head = self.symbol_end(inner);
            if head == inner {
                return None;
            }
            let prefix = self.space_end(head);
            if let Some((name, end)) = self.prefixed_symbol(prefix) {
                self.cap(3, inner, head);
                if name > prefix {
                    self.cap(4, prefix, name);
                }
                self.cap(6, name, end);
                end
            } else {
                let last = self.char_start_before(head);
                if last == inner || !self.delimiter(head) {
                    return None;
                }
                self.cap(3, inner, last);
                self.cap(6, last, head);
                head
            }
        } else {
            let (name, end) = self.prefixed_symbol(blanks)?;
            if name > blanks {
                self.cap(7, blanks, name);
            }
            self.cap(9, name, end);
            end
        };
        self.cap(1, p, word);
        Some(end)
    }

    /// `DefNameAt1`, a type definition, or `DefNameAt2`, a constant:
    /// `(def…)\s+(sym+?)` before a delimiter, the word as group 1 and the
    /// name as group 2.
    pub(super) fn def_named(&mut self, p: usize, list: &[&[u8]]) -> Option<usize> {
        let (word, blanks) = self.head(p, list)?;
        let end = self.delimited_symbol(blanks)?;
        self.cap(1, p, word);
        self.cap(2, blanks, end);
        Some(end)
    }

    /// `(?i)(?<=^|[(\s])(def(?:var|parameter))\s+(?=([()\s]))`: the word
    /// as group 1 and its blanks.
    ///
    /// The greedy `\s+` takes the line feed too.
    /// Unless a parenthesis follows the blanks, it gives back the last one,
    /// which the lookahead then reads, when it has more than one.
    pub(super) fn def_variable(&mut self, p: usize) -> Option<usize> {
        let (word, blanks) = self.head(p, words::DEF_VARIABLES)?;
        let end = if matches!(self.at(blanks), b'(' | b')') {
            blanks
        } else {
            let last = if blanks > self.lim() {
                self.lim()
            } else {
                self.char_start_before(blanks)
            };
            if last == word {
                return None;
            }
            last
        };
        self.cap(1, p, word);
        Some(end)
    }

    /// `(?i)(?<=^|[(\s])(defstruct)\s+\(?\s*(sym+?)(?=([()\s]))`: the word
    /// as group 1 and the structure's name as group 2.
    ///
    /// Without the optional `(`, the name would start at it, which no symbol
    /// does.
    pub(super) fn def_struct(&mut self, p: usize) -> Option<usize> {
        let (word, blanks) = self.head(p, words::DEF_STRUCT)?;
        let name = if self.at(blanks) == b'(' {
            self.space_end(blanks + 1)
        } else {
            blanks
        };
        let end = self.delimited_symbol(name)?;
        self.cap(1, p, word);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(?i)(?<=^|[(\s])(macrolet|labels|flet)\s+\(\s*\(\s*(sym+?)(?=([()\
    /// s]))`: the word as group 1 and the first local function's name as group
    /// 2, which is lexed again.
    pub(super) fn local_function(&mut self, p: usize) -> Option<usize> {
        let (word, blanks) = self.head(p, words::LOCAL_FUNCTIONS)?;
        let outer = self.lit(blanks, b"(")?;
        let inner = self.lit(self.space_end(outer), b"(")?;
        let name = self.space_end(inner);
        let end = self.delimited_symbol(name)?;
        self.cap(1, p, word);
        self.cap(2, name, end);
        Some(end)
    }
}
