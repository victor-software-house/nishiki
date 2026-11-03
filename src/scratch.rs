//! What the `Highlighter` keeps from one line to the next, one field per
//! language.
//!
//! A language's `line` takes the whole of it and uses its own field and those
//! of the languages it embeds, so a Markdown line can lex a shell fence
//! without a parameter per embedded language.

/// Buffers and caches of every lexer that keeps any.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The C lexer's, for C documents and Markdown C fences.
    #[cfg(feature = "lang-c")]
    pub(crate) c: crate::c::Scratch,
    /// The C++ lexer's, for C++ documents and Markdown C++ fences.
    #[cfg(feature = "lang-cpp")]
    pub(crate) cpp: crate::cpp::Scratch,
    /// The C# lexer's, for C# documents and Markdown C# fences.
    #[cfg(feature = "lang-csharp")]
    pub(crate) csharp: crate::csharp::Scratch,
    /// The CSS lexer's, for CSS documents and Markdown CSS fences.
    #[cfg(feature = "lang-css")]
    pub(crate) css: crate::css::Scratch,
    /// The Go lexer's, for Go documents.
    #[cfg(feature = "lang-go")]
    pub(crate) go: crate::go::Scratch,
    /// The GraphQL lexer's, for GraphQL documents.
    #[cfg(feature = "lang-graphql")]
    pub(crate) graphql: crate::graphql::Scratch,
    /// The HTML lexer's, for HTML documents and Markdown HTML fences.
    #[cfg(feature = "lang-html")]
    pub(crate) html: crate::html::Scratch,
    /// The Java lexer's, for Java documents.
    #[cfg(feature = "lang-java")]
    pub(crate) java: crate::java::Scratch,
    /// The Kotlin lexer's, for Kotlin documents.
    #[cfg(feature = "lang-kotlin")]
    pub(crate) kotlin: crate::kotlin::Scratch,
    /// The Lua lexer's, for Lua documents and Markdown Lua fences.
    #[cfg(feature = "lang-lua")]
    pub(crate) lua: crate::lua::Scratch,
    /// The Makefile lexer's.
    #[cfg(feature = "lang-make")]
    pub(crate) make: crate::make::Scratch,
    /// The Markdown lexer's.
    #[cfg(feature = "lang-markdown")]
    pub(crate) markdown: crate::markdown::Scratch,
    /// The Python lexer's, for Python documents and Markdown Python fences.
    #[cfg(feature = "lang-python")]
    pub(crate) python: crate::python::Scratch,
    /// The Ruby lexer's, for Ruby documents.
    #[cfg(feature = "lang-ruby")]
    pub(crate) ruby: crate::ruby::Scratch,
    /// The shell lexer's, for shell documents and Markdown shell fences.
    #[cfg(feature = "lang-shell")]
    pub(crate) shell: crate::shell::Scratch,
    /// The SQL lexer's, for SQL documents and Markdown SQL fences.
    #[cfg(feature = "lang-sql")]
    pub(crate) sql: crate::sql::Scratch,
    /// The Swift lexer's, for Swift documents and Markdown Swift fences.
    #[cfg(feature = "lang-swift")]
    pub(crate) swift: crate::swift::Scratch,
    /// The `TextMate` engine's, for every language it runs.
    #[cfg(feature = "_textmate")]
    pub(crate) textmate: crate::textmate::Scratch,
    /// The TypeScript lexer's, for every dialect and the JavaScript of
    /// GraphQL interpolations and Markdown fences.
    #[cfg(any(
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    ))]
    pub(crate) typescript: Vec<crate::typescript::Opened>,
    /// The XML lexer's, for XML documents and Markdown XML fences.
    #[cfg(feature = "lang-xml")]
    pub(crate) xml: crate::xml::Scratch,
}
