//! The highlighter: one language, its interned stacks and states, and its
//! limits.

use crate::arena::{Arena, ROOT, Scopes};
use crate::json::Dialect;
use crate::limits::{Limit, Limits, LineReport};
use crate::scratch::Scratch;
use crate::{Language, Stack, State, Token, json, markdown, python, rust, shell, typescript};

/// Highlights one language, a line at a time.
///
/// It owns the scope stacks and states its tokens and [`State`]s name, so
/// both belong to the highlighter that made them. Lines are lexed in order
/// from [`State::default`], and the state after one line is the input of the
/// next, so an editor can keep each line's state and re-highlight from any
/// line.
#[derive(Debug)]
pub struct Highlighter {
    language: Language,
    limits: Limits,
    arena: Arena,
    /// Buffers and caches of the lexers, one field per language.
    scratch: Scratch,
    /// The state at the start of a document; [`State::default`] stands for
    /// it.
    start: u32,
}

impl Highlighter {
    /// A highlighter for `language` with [`Limits::default`].
    #[must_use]
    pub fn new(language: Language) -> Self {
        Self::new_with_limits(language, Limits::default())
    }

    /// A highlighter for `language` with `limits`.
    #[must_use]
    pub fn new_with_limits(language: Language, limits: Limits) -> Self {
        let mut arena = Arena::new(limits.nodes);
        let start = match language {
            Language::Rust => rust::start(&mut arena, ROOT),
            Language::Markdown => markdown::start(&mut arena, ROOT),
            Language::TypeScript => typescript::start(&mut arena, ROOT),
            Language::Shell => shell::start(&mut arena, ROOT),
            Language::Json => json::start(&mut arena, ROOT, Dialect::Json),
            Language::Jsonc => json::start(&mut arena, ROOT, Dialect::Jsonc),
            Language::Jsonl => json::start(&mut arena, ROOT, Dialect::Jsonl),
            Language::Python => python::start(&mut arena, ROOT),
        };
        Highlighter {
            language,
            limits,
            arena,
            scratch: Scratch::default(),
            start,
        }
    }

    /// The language this highlighter lexes.
    #[must_use]
    pub fn language(&self) -> Language {
        self.language
    }

    /// The limits this highlighter applies to every line.
    #[must_use]
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Lexes `line`, which excludes its line ending, starting from `state`.
    ///
    /// Appends tokens that cover every byte of `line` in order (an empty line
    /// appends none), leaves in `state` the state for the next line, and
    /// reports the limits the line reached.
    pub fn line(&mut self, line: &str, state: &mut State, out: &mut Vec<Token>) -> LineReport {
        let mut report = LineReport::default();
        let from = if state.0 == ROOT { self.start } else { state.0 };
        if self.limits.line_bytes.is_some_and(|max| line.len() > max) {
            report.record(Limit::LineBytes);
            let stack = Stack(self.arena.frame(from).stack);
            out.push(Token {
                end: line.len(),
                stack,
            });
            return report;
        }
        let to = match self.language {
            Language::Rust => rust::line(
                &mut self.arena,
                &self.limits,
                line.as_bytes(),
                from,
                out,
                &mut report,
            ),
            Language::Markdown => markdown::line(
                &mut self.scratch,
                &mut self.arena,
                &self.limits,
                line,
                from,
                out,
                &mut report,
            ),
            Language::TypeScript => {
                typescript::line(&mut self.arena, &self.limits, line, from, out, &mut report)
            }
            Language::Shell => shell::line(
                &mut self.scratch,
                &mut self.arena,
                &self.limits,
                shell::Text {
                    bytes: line.as_bytes(),
                    start: 0,
                    anchored: false,
                },
                from,
                out,
                &mut report,
            ),
            Language::Json => self.json_line(Dialect::Json, line, from, out, &mut report),
            Language::Jsonc => self.json_line(Dialect::Jsonc, line, from, out, &mut report),
            Language::Jsonl => self.json_line(Dialect::Jsonl, line, from, out, &mut report),
            Language::Python => python::line(
                &mut self.scratch,
                &mut self.arena,
                &self.limits,
                python::Text {
                    bytes: line.as_bytes(),
                    start: 0,
                },
                from,
                out,
                &mut report,
            ),
        };
        *state = State(if to == self.start { ROOT } else { to });
        report
    }

    fn json_line(
        &mut self,
        dialect: Dialect,
        line: &str,
        from: u32,
        out: &mut Vec<Token>,
        report: &mut LineReport,
    ) -> u32 {
        json::line(
            &mut self.arena,
            &self.limits,
            dialect,
            line.as_bytes(),
            from,
            out,
            report,
        )
    }

    /// The scope names of `stack`, innermost first, such as
    /// `"string.quoted.double.rust"` then `"source.rust"`.
    ///
    /// Empty when `stack` is unknown to this highlighter. A stack from another
    /// highlighter resolves to an unrelated stack or to nothing.
    #[must_use]
    pub fn scopes(&self, stack: Stack) -> Scopes<'_> {
        self.arena.scopes(stack.0)
    }

    /// The open context of `state`, for the lexers' tests.
    #[cfg(test)]
    pub(crate) fn frame(&self, state: State) -> crate::arena::Frame {
        self.arena
            .frame(if state.0 == ROOT { self.start } else { state.0 })
    }
}
