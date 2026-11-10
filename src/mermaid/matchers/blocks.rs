//! The Markdown blocks that hold a diagram, the first lines of the diagrams
//! and their ends.

use super::super::engine::Lexer;
use crate::classes::is_letter;
use crate::rules::Chars;

/// How a Markdown fence names Mermaid.
#[derive(Clone, Copy, Debug)]
pub(super) enum Fence {
    /// ```` ```mermaid ````.
    Plain,
    /// ```` ```{.mermaid} ````, with attributes.
    Attributes,
}

impl Lexer<'_> {
    /// ``(?i)(?<=[`~])\s*mermaid(\s+[^`~]*)?$``, or with `\{\s*\.?` before
    /// `mermaid` for [`Fence::Attributes`]: the info string of a fence.
    ///
    /// The group takes white space and the rest of the line when no backtick
    /// or tilde stands in it.
    /// Then `$` holds at the end, past the line feed.
    /// Otherwise `$` must hold right after `mermaid`.
    pub(super) fn code_block(&self, p: usize, fence: Fence) -> Option<usize> {
        if p == 0 || !matches!(self.at(p - 1), b'`' | b'~') {
            return None;
        }
        let mut name = self.ws(p);
        if let Fence::Attributes = fence {
            name = self.lit(name, b"{")?;
            name = self.ws(name);
            name += usize::from(self.at(name) == b'.');
        }
        let word = self.ci(name, b"mermaid")?;
        if self.space_at(word) > 0 {
            let rest = self.class_run(word, |c| c != '`' && c != '~');
            if rest >= self.lim() {
                return Some(rest);
            }
        }
        self.at_eol(word).then_some(word)
    }

    /// `(^|\G)(?=\s*[X]{3,}\s*$)`, for the fence characters `marks`: the end of
    /// a block or a diagram, before a line of three or more of them.
    pub(super) fn block_end(&self, p: usize, marks: &[u8]) -> Option<usize> {
        if p != 0 && !self.at_g(p) {
            return None;
        }
        let start = self.ws(p);
        let fence = self.skip(start, |b| marks.contains(&b));
        let after = self.ws(fence);
        (fence - start >= 3 && self.at_eol(after)).then_some(p)
    }

    /// `(?i)\s*:::\s*mermaid\s*$`: an Azure `DevOps` block.
    pub(super) fn ado_block(&self, p: usize) -> Option<usize> {
        let colons = self.lit(self.ws(p), b":::")?;
        let word = self.ci(self.ws(colons), b"mermaid")?;
        let end = self.ws(word);
        self.at_eol(end).then_some(end)
    }

    /// `\s*:::\s*`: the end of an Azure `DevOps` block.
    pub(super) fn ado_block_end(&self, p: usize) -> Option<usize> {
        let colons = self.lit(self.ws(p), b":::")?;
        Some(self.ws(colons))
    }

    /// `^\s*(word)`: the first line of a diagram, its word as written.
    pub(super) fn diagram(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.lit(start, word)?;
        self.capture(1, start, end);
        Some(end)
    }

    /// `^\s*(stateDiagram(?:-v2)?)`.
    pub(super) fn state_diagram(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.lit(start, b"stateDiagram")?;
        let end = self.lit(word, b"-v2").unwrap_or(word);
        self.capture(1, start, end);
        Some(end)
    }

    /// `^\s*(xychart(?:-beta)?(?:\s+horizontal)?)`.
    pub(super) fn xy_chart(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.lit(start, b"xychart")?;
        let beta = self.lit(word, b"-beta").unwrap_or(word);
        let end = self
            .ws1(beta)
            .and_then(|space| self.lit(space, b"horizontal"))
            .unwrap_or(beta);
        self.capture(1, start, end);
        Some(end)
    }

    /// `^\s*(graph|flowchart)\s+([ 0-9\p{L}]+)?`: a flowchart and its
    /// direction.
    /// The spaces are greedy, so the direction starts after them.
    pub(super) fn flowchart(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self
            .lit(start, b"graph")
            .or_else(|| self.lit(start, b"flowchart"))?;
        let space = self.ws1(word)?;
        self.capture(1, start, word);
        match self.class_run1(space, |c| c == ' ' || c.is_ascii_digit() || is_letter(c)) {
            Some(end) => {
                self.capture(2, space, end);
                Some(end)
            }
            None => Some(space),
        }
    }
}
