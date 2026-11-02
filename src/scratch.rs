//! What the `Highlighter` keeps from one line to the next, one field per
//! language.
//!
//! A language's `line` takes the whole of it and uses its own field and those
//! of the languages it embeds, so a Markdown line can lex a shell fence
//! without a parameter per embedded language.

use crate::{css, java, markdown, python, shell};

/// Buffers and caches of every lexer that keeps any.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The Markdown lexer's.
    pub(crate) markdown: markdown::Scratch,
    /// The shell lexer's, for shell documents and Markdown shell fences.
    pub(crate) shell: shell::Scratch,
    /// The Python lexer's, for Python documents and Markdown Python fences.
    pub(crate) python: python::Scratch,
    /// The Java lexer's, for Java documents.
    pub(crate) java: java::Scratch,
    /// The CSS lexer's, for CSS documents and Markdown CSS fences.
    pub(crate) css: css::Scratch,
}
