//! What the `Highlighter` keeps from one line to the next, one field per
//! language.
//!
//! A language's `line` takes the whole of it and uses its own field and those
//! of the languages it embeds, so a Markdown line can lex a shell fence
//! without a parameter per embedded language.

/// Buffers and caches of every lexer that keeps any.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The number of the current line (see [`Scratch::line`]).
    line: u64,
    /// The Batch lexer's, for Batch documents and Markdown Batch fences.
    /// It is boxed and allocated by the first Batch line, as Dart's is.
    #[cfg(feature = "lang-bat")]
    pub(crate) bat: Option<Box<crate::bat::Scratch>>,
    /// The C lexer's, for C documents and Markdown C fences.
    #[cfg(feature = "lang-c")]
    pub(crate) c: crate::c::Scratch,
    /// The CMake lexer's, for CMake documents.
    /// It is boxed and allocated by the first CMake line, as GraphQL's is.
    #[cfg(feature = "lang-cmake")]
    pub(crate) cmake: Option<Box<crate::cmake::Scratch>>,
    /// The C++ lexer's, for C++ documents and Markdown C++ fences.
    #[cfg(feature = "lang-cpp")]
    pub(crate) cpp: crate::cpp::Scratch,
    /// The C# lexer's, for C# documents and Markdown C# fences.
    #[cfg(feature = "lang-csharp")]
    pub(crate) csharp: crate::csharp::Scratch,
    /// The CSS lexer's, for CSS documents and Markdown CSS fences.
    #[cfg(feature = "lang-css")]
    pub(crate) css: crate::css::Scratch,
    /// The Dart lexer's, for Dart documents and Markdown Dart fences.
    /// It is boxed and allocated by the first Dart line, as GraphQL's is.
    #[cfg(feature = "lang-dart")]
    pub(crate) dart: Option<Box<crate::dart::Scratch>>,
    /// The Go lexer's, for Go documents.
    #[cfg(feature = "lang-go")]
    pub(crate) go: crate::go::Scratch,
    /// The GraphQL lexer's, for GraphQL documents and Ruby's GraphQL
    /// heredocs.
    /// It is boxed and allocated by the first GraphQL line.
    /// A highlighter for another language keeps an 8-byte field, which
    /// `Highlighter::new` moves with the rest.
    #[cfg(feature = "lang-graphql")]
    pub(crate) graphql: Option<Box<crate::graphql::Scratch>>,
    /// The Haskell lexer's, for Haskell documents.
    /// It is boxed and allocated by the first Haskell line, as Dart's is.
    #[cfg(feature = "lang-haskell")]
    pub(crate) haskell: Option<Box<crate::haskell::Scratch>>,
    /// The HTML lexer's, for HTML documents and Markdown HTML fences.
    #[cfg(feature = "lang-html")]
    pub(crate) html: crate::html::Scratch,
    /// The Java lexer's, for Java documents.
    #[cfg(feature = "lang-java")]
    pub(crate) java: crate::java::Scratch,
    /// The Julia lexer's, for Julia documents and Markdown Julia fences.
    /// It is boxed and allocated by the first Julia line, as GraphQL's is.
    #[cfg(feature = "lang-julia")]
    pub(crate) julia: Option<Box<crate::julia::Scratch>>,
    /// The Kotlin lexer's, for Kotlin documents.
    #[cfg(feature = "lang-kotlin")]
    pub(crate) kotlin: crate::kotlin::Scratch,
    /// The Less lexer's, for Less documents and Markdown Less fences.
    #[cfg(feature = "lang-less")]
    pub(crate) less: crate::less::Scratch,
    /// The Lua lexer's, for Lua documents and Markdown Lua fences.
    #[cfg(feature = "lang-lua")]
    pub(crate) lua: crate::lua::Scratch,
    /// The Makefile lexer's.
    #[cfg(feature = "lang-make")]
    pub(crate) make: crate::make::Scratch,
    /// The Markdown lexer's.
    #[cfg(feature = "lang-markdown")]
    pub(crate) markdown: crate::markdown::Scratch,
    /// The Mermaid lexer's, for Mermaid documents.
    /// It is boxed and allocated by the first Mermaid line, as Dart's is.
    #[cfg(feature = "lang-mermaid")]
    pub(crate) mermaid: Option<Box<crate::mermaid::Scratch>>,
    /// The Nix lexer's, for Nix documents.
    /// It is boxed and allocated by the first Nix line, as Dart's is.
    #[cfg(feature = "lang-nix")]
    pub(crate) nix: Option<Box<crate::nix::Scratch>>,
    /// The PowerShell lexer's, for PowerShell documents and Markdown
    /// PowerShell fences.
    /// It is boxed and allocated by the first PowerShell line, as GraphQL's is.
    /// The PostCSS lexer's, for PostCSS documents.
    /// It is boxed and allocated by the first PostCSS line, as Protobuf's is.
    #[cfg(feature = "lang-postcss")]
    pub(crate) postcss: Option<Box<crate::postcss::Scratch>>,
    #[cfg(feature = "lang-powershell")]
    pub(crate) powershell: Option<Box<crate::powershell::Scratch>>,
    /// The Protobuf lexer's, for Protobuf documents.
    /// It is boxed and allocated by the first Protobuf line, as Dart's is.
    #[cfg(feature = "lang-proto")]
    pub(crate) proto: Option<Box<crate::proto::Scratch>>,
    /// The Python lexer's, for Python documents, Markdown Python fences and
    /// Julia's `py"""` string macros.
    #[cfg(feature = "lang-python")]
    pub(crate) python: crate::python::Scratch,
    /// The Ruby lexer's, for Ruby documents.
    #[cfg(feature = "lang-ruby")]
    pub(crate) ruby: crate::ruby::Scratch,
    /// The Sass lexer's, for Sass documents, Vue's Sass styles and HAML's Sass
    /// filters.
    /// It is boxed and allocated by the first Sass line, as Dart's is.
    #[cfg(feature = "lang-sass")]
    pub(crate) sass: Option<Box<crate::sass::Scratch>>,
    /// The Scala lexer's, for Scala documents and Markdown Scala fences.
    /// It is boxed and allocated by the first Scala line, as Dart's is.
    #[cfg(feature = "lang-scala")]
    pub(crate) scala: Option<Box<crate::scala::Scratch>>,
    /// The SCSS lexer's, for SCSS documents and Markdown SCSS fences.
    #[cfg(feature = "lang-scss")]
    pub(crate) scss: crate::scss::Scratch,
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
    /// Boxed, so a line of a grammar that hands off swaps it for its spare by
    /// swapping two pointers.
    #[cfg(feature = "_textmate")]
    pub(crate) textmate: Box<crate::textmate::Scratch>,
    /// The TypeScript lexer's, for every dialect and the JavaScript of
    /// GraphQL interpolations and Markdown fences.
    #[cfg(any(
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    ))]
    pub(crate) typescript: crate::typescript::Scratch,
    /// The XML lexer's, for XML documents and Markdown XML fences.
    #[cfg(feature = "lang-xml")]
    pub(crate) xml: crate::xml::Scratch,
    /// The Zig lexer's, for Zig documents.
    /// It is boxed and allocated by the first Zig line, as Dart's is.
    #[cfg(feature = "lang-zig")]
    pub(crate) zig: Option<Box<crate::zig::Scratch>>,
}

impl Scratch {
    /// The number of the current line. [`Scratch::start_line`] advances it.
    /// An embedding language may lex many texts of one line. A lexer that
    /// keeps an answer for every text of one line keys it by this number.
    pub(crate) fn line(&self) -> u64 {
        self.line
    }

    /// Starts the next line, so the lexers forget what they kept for every
    /// text of the line before.
    pub(crate) fn start_line(&mut self) {
        self.line = self.line.wrapping_add(1);
    }
}
