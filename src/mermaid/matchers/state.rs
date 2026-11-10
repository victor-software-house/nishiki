//! The state diagram.

use super::super::engine::Lexer;
use super::is_dash_word;
use crate::rules::Chars;

/// `BT|RL|TB|LR`.
pub(super) const STATE_DIRECTIONS: &[&[u8]] = &[b"BT", b"RL", b"TB", b"LR"];

/// `[^\n:]`: a description's text.
fn is_description(c: char) -> bool {
    c != '\n' && c != ':'
}

impl Lexer<'_> {
    /// `\s+(literal)\s+`.
    pub(super) fn spaced(&mut self, p: usize, literal: &[u8]) -> Option<usize> {
        let word = self.ws1(p)?;
        let word_end = self.lit(word, literal)?;
        let end = self.ws1(word_end)?;
        self.capture(1, word, word_end);
        Some(end)
    }

    /// `^\s*([-\w]+)$`: a state alone on its line.
    pub(super) fn lone_state(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.class_run1(start, is_dash_word)?;
        if !self.at_eol(end) {
            return None;
        }
        self.capture(1, start, end);
        Some(end)
    }

    /// `(?i)([-\w]+)\s*(:)\s*(\s*[^:]+)`: a state and its description.
    /// The text's class holds white space and the line feed.
    /// So where a colon follows the spaces, they give back their last one
    /// to the text.
    pub(super) fn state_description(&mut self, p: usize) -> Option<usize> {
        let name = self.class_run1(p, is_dash_word)?;
        let colon = self.ws(name);
        let colon_end = self.lit(colon, b":")?;
        let (text, end) = self.spaced_run(colon_end, 0, |c| c != ':')?;
        self.capture(1, p, name);
        self.capture(2, colon, colon_end);
        self.capture(3, text, end);
        Some(end)
    }

    /// `(?i)^\s*(state)\s+`.
    pub(super) fn state_keyword(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"state")?;
        let end = self.ws1(word)?;
        self.capture(1, start, word);
        Some(end)
    }

    /// `(?i)\s*("[^"]+")\s*(as)\s+([-\w]+)\s*(\{)?`: a state with a quoted
    /// description and an alias.
    pub(super) fn state_alias(&mut self, p: usize) -> Option<usize> {
        let text = self.ws(p);
        if self.at(text) != b'"' {
            return None;
        }
        let close = self.class_run1(text + 1, |c| c != '"')?;
        let text_end = self.lit(close, b"\"")?;
        let word = self.ws(text_end);
        let word_end = self.ci(word, b"as")?;
        let name = self.ws1(word_end)?;
        let name_end = self.class_run1(name, is_dash_word)?;
        let brace = self.ws(name_end);
        let end = self.lit(brace, b"{").unwrap_or(brace);
        for (group, from, to) in [
            (1, text, text_end),
            (2, word, word_end),
            (3, name, name_end),
            (4, brace, end),
        ] {
            self.capture(group, from, to);
        }
        Some(end)
    }

    /// `(?i)\s*([-\w]+)\s+(\{)`: a composite state.
    pub(super) fn composite(&mut self, p: usize) -> Option<usize> {
        let name = self.ws(p);
        let name_end = self.class_run1(name, is_dash_word)?;
        let brace = self.ws1(name_end)?;
        let end = self.lit(brace, b"{")?;
        self.capture(1, name, name_end);
        self.capture(2, brace, end);
        Some(end)
    }

    /// `(?i)\s*([-\w]+)\s+(<<(?:fork|join)>>)`: a fork or a join.
    pub(super) fn fork(&mut self, p: usize) -> Option<usize> {
        let name = self.ws(p);
        let name_end = self.class_run1(name, is_dash_word)?;
        let mark = self.ws1(name_end)?;
        let open = self.lit(mark, b"<<")?;
        let word = self.ci(open, b"fork").or_else(|| self.ci(open, b"join"))?;
        let end = self.lit(word, b">>")?;
        self.capture(1, name, name_end);
        self.capture(2, mark, end);
        Some(end)
    }

    /// `(?i)([-\w]+)\s*(-->)`: the head of a transition.
    /// The name takes `-`, so it gives back one character at a time until
    /// `-->` follows, as in `A-->B`.
    pub(super) fn transition_head(&mut self, p: usize) -> Option<usize> {
        let mut name = self.class_run1(p, is_dash_word)?;
        loop {
            let arrow = self.ws(name);
            if let Some(end) = self.lit(arrow, b"-->") {
                self.capture(1, p, name);
                self.capture(2, arrow, end);
                return Some(end);
            }
            name = self.char_start_before(name);
            if name <= p {
                return None;
            }
        }
    }

    /// `\s*(:)?\s*([^\n:]+)?` from `p`, with groups `group` and `group + 1`:
    /// a transition's label.
    /// Every part is optional and greedy, and nothing follows.
    fn transition_label(&mut self, group: usize, p: usize) -> usize {
        let colon = self.ws(p);
        let mut q = colon;
        if self.at(colon) == b':' {
            q = self.ws(colon + 1);
            self.capture(group, colon, colon + 1);
        }
        let end = self.class_run(q, is_description);
        self.capture(group + 1, q, end);
        end
    }

    /// `(?i)\s*([-\w]+)\s*(:)?\s*([^\n:]+)?`: the target of a transition.
    pub(super) fn transition_target(&mut self, p: usize) -> Option<usize> {
        let name = self.ws(p);
        let name_end = self.class_run1(name, is_dash_word)?;
        self.capture(1, name, name_end);
        Some(self.transition_label(2, name_end))
    }

    /// `(?i)(\[\*])\s*(:)?\s*([^\n:]+)?`: the final state as a target.
    pub(super) fn terminal_target(&mut self, p: usize) -> Option<usize> {
        let end = self.group_one(p, b"[*]")?;
        Some(self.transition_label(2, end))
    }

    /// `(?i)(\[\*])\s*(-->)\s*([-\w]+)\s*(:)?\s*([^\n:]+)?`: a transition from
    /// the initial state.
    pub(super) fn initial_transition(&mut self, p: usize) -> Option<usize> {
        let star = self.lit(p, b"[*]")?;
        let arrow = self.ws(star);
        let arrow_end = self.lit(arrow, b"-->")?;
        let name = self.ws(arrow_end);
        let name_end = self.class_run1(name, is_dash_word)?;
        self.capture(1, p, star);
        self.capture(2, arrow, arrow_end);
        self.capture(3, name, name_end);
        Some(self.transition_label(4, name_end))
    }

    /// `(?i)^\s*(note (?:left|right) of)\s+([-\w]+)` at `p`: the head of a
    /// note.
    /// It returns where its words start and end, and where the name starts
    /// and ends.
    fn note_head(&self, p: usize) -> Option<[usize; 4]> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"note ")?;
        let side = self
            .ci(word, b"left ")
            .or_else(|| self.ci(word, b"right "))?;
        let words = self.ci(side, b"of")?;
        let name = self.ws1(words)?;
        let name_end = self.class_run1(name, is_dash_word)?;
        Some([start, words, name, name_end])
    }

    /// `(?i)^\s*(note (?:left|right) of)\s+([-\w]+)\s*(:)\s*([^\n:]+)`: a
    /// note on one line.
    /// The text's class holds white space but not the line feed.
    /// So where the spaces reach a colon or the line feed, they give back
    /// their last one to the text.
    pub(super) fn state_note(&mut self, p: usize) -> Option<usize> {
        let [start, words, name, name_end] = self.note_head(p)?;
        let colon = self.ws(name_end);
        let colon_end = self.lit(colon, b":")?;
        let (text, end) = self.spaced_run(colon_end, 0, is_description)?;
        self.capture(1, start, words);
        self.capture(2, name, name_end);
        self.capture(3, colon, colon_end);
        self.capture(4, text, end);
        Some(end)
    }

    /// `(?i)^\s*(note (?:left|right) of)\s+([-\w]+)(.|\n)`: the head of a
    /// note on several lines.
    /// The last group takes any character.
    /// So the name gives one back only at the end of a text with no line
    /// feed.
    pub(super) fn state_note_block(&mut self, p: usize) -> Option<usize> {
        let [start, words, name, greedy] = self.note_head(p)?;
        let (name_end, end) = if let Some((_, end)) = self.ch(greedy) {
            (greedy, end)
        } else {
            let last = self.char_start_before(greedy);
            if last <= name {
                return None;
            }
            (last, greedy)
        };
        self.capture(1, start, words);
        self.capture(2, name, name_end);
        self.capture(3, name_end, end);
        Some(end)
    }

    /// `(?i)(end note)`.
    pub(super) fn state_note_end(&mut self, p: usize) -> Option<usize> {
        let end = self.ci(p, b"end note")?;
        self.capture(1, p, end);
        Some(end)
    }
}
