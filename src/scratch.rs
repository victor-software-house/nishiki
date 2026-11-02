//! What the `Highlighter` keeps from one line to the next, one field per
//! language.
//!
//! A language's `line` takes the whole of it and uses its own field and those
//! of the languages it embeds, so a Markdown line can lex a shell fence
//! without a parameter per embedded language.

/// Buffers and caches of every lexer that keeps any.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The Markdown lexer's.
    #[cfg(feature = "lang-markdown")]
    pub(crate) markdown: crate::markdown::Scratch,
    /// The shell lexer's, for shell documents and Markdown shell fences.
    #[cfg(feature = "lang-shell")]
    pub(crate) shell: crate::shell::Scratch,
    /// The Python lexer's, for Python documents and Markdown Python fences.
    #[cfg(feature = "lang-python")]
    pub(crate) python: crate::python::Scratch,
    /// The Java lexer's, for Java documents.
    #[cfg(feature = "lang-java")]
    pub(crate) java: crate::java::Scratch,
    /// The CSS lexer's, for CSS documents and Markdown CSS fences.
    #[cfg(feature = "lang-css")]
    pub(crate) css: crate::css::Scratch,
}
