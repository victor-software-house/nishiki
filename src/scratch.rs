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
    /// The Assembly lexer's, for Assembly documents.
    /// It is boxed and allocated by the first Assembly line.
    #[cfg(feature = "lang-asm")]
    pub(crate) asm: Option<Box<crate::asm::Scratch>>,
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
    /// The CoffeeScript lexer's, for CoffeeScript documents, Markdown
    /// CoffeeScript fences and the CoffeeScript that Vue and HAML embed.
    /// It is boxed and allocated by the first CoffeeScript line, as GraphQL's
    /// is.
    #[cfg(feature = "lang-coffee")]
    pub(crate) coffee: Option<Box<crate::coffee::Scratch>>,
    /// The CoffeeScript lexer's for the CoffeeScript that Svelte embeds, a
    /// dialect of its own for Svelte's `$` prefixes.
    /// It is boxed and allocated by the first such line.
    #[cfg(all(feature = "lang-coffee", feature = "lang-svelte"))]
    pub(crate) coffee_svelte: Option<Box<crate::coffee::Scratch<{ crate::coffee::SVELTE }>>>,
    /// The Common Lisp lexer's, for Common Lisp documents.
    /// It is boxed and allocated by the first Common Lisp line.
    #[cfg(feature = "lang-common-lisp")]
    pub(crate) common_lisp: Option<Box<crate::common_lisp::Scratch>>,
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
    /// The Elixir lexer's, for Elixir documents and Markdown Elixir fences.
    /// It is boxed and allocated by the first Elixir line, as Haskell's is.
    #[cfg(feature = "lang-elixir")]
    pub(crate) elixir: Option<Box<crate::elixir::Scratch>>,
    /// The GDScript lexer's, for GDScript documents.
    /// It is boxed and allocated by the first GDScript line, as Haskell's is.
    #[cfg(feature = "lang-gdscript")]
    pub(crate) gdscript: Option<Box<crate::gdscript::Scratch>>,
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
    /// The Jinja lexer's, for Jinja documents.
    /// It is boxed and allocated by the first Jinja line, as Haskell's is.
    #[cfg(feature = "lang-jinja")]
    pub(crate) jinja: Option<Box<crate::jinja::Scratch>>,
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
    /// The MATLAB lexer's, for MATLAB documents.
    /// It is boxed and allocated by the first MATLAB line, as Mermaid's is.
    #[cfg(feature = "lang-matlab")]
    pub(crate) matlab: Option<Box<crate::matlab::Scratch>>,
    /// The Mermaid lexer's, for Mermaid documents.
    /// It is boxed and allocated by the first Mermaid line, as Dart's is.
    #[cfg(feature = "lang-mermaid")]
    pub(crate) mermaid: Option<Box<crate::mermaid::Scratch>>,
    /// The Nix lexer's, for Nix documents.
    /// It is boxed and allocated by the first Nix line, as Dart's is.
    #[cfg(feature = "lang-nix")]
    pub(crate) nix: Option<Box<crate::nix::Scratch>>,
    /// The Objective-C lexer's, for Objective-C documents and Markdown
    /// Objective-C fences.
    /// It is boxed and allocated by the first Objective-C line, as Nix's is.
    #[cfg(feature = "lang-objective-c")]
    pub(crate) objective_c: Option<Box<crate::objective_c::Scratch>>,
    /// The Objective-C++ lexer's, for Objective-C++ documents.
    /// It is boxed and allocated by the first Objective-C++ line, as
    /// Objective-C's is.
    #[cfg(feature = "lang-objective-cpp")]
    pub(crate) objective_cpp: Option<Box<crate::objective_cpp::Scratch>>,
    /// The Pascal lexer's, for Pascal documents.
    /// It is boxed and allocated by the first Pascal line.
    #[cfg(feature = "lang-pascal")]
    pub(crate) pascal: Option<Box<crate::pascal::Scratch>>,
    /// The Perl lexer's, for Perl documents and Markdown Perl fences.
    /// It is boxed and allocated by the first Perl line, as PHP's is.
    #[cfg(feature = "lang-perl")]
    pub(crate) perl: Option<Box<crate::perl::Scratch>>,
    /// The PHP lexer's, for PHP documents and Markdown PHP fences.
    /// It is boxed and allocated by the first PHP line, as Dart's is.
    #[cfg(feature = "lang-php")]
    pub(crate) php: Option<Box<crate::php::Scratch>>,
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
    /// The R lexer's, for R documents and Markdown R fences.
    /// It is boxed and allocated by the first R line, as Pascal's is.
    #[cfg(feature = "lang-r")]
    pub(crate) r: Option<Box<crate::r::Scratch>>,
    /// The Ruby lexer's, for Ruby documents.
    #[cfg(feature = "lang-ruby")]
    pub(crate) ruby: crate::ruby::Scratch,
    /// The places where an embedding rule's own patterns start on the line,
    /// for a rule that lists them after the language it embeds
    /// ([`crate::rules::Embeds::embeds_after_own`]).
    /// The rule engine fills it once a line, and keeps its room for the next.
    #[cfg(feature = "_embedding")]
    pub(crate) rule_starts: Vec<usize>,
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
    /// The Svelte lexer's, for Svelte documents.
    /// It is boxed and allocated by the first Svelte line, as Dart's is.
    #[cfg(feature = "lang-svelte")]
    pub(crate) svelte: Option<Box<crate::svelte::Scratch>>,
    /// The Swift lexer's, for Swift documents and Markdown Swift fences.
    #[cfg(feature = "lang-swift")]
    pub(crate) swift: crate::swift::Scratch,
    /// The buffers of a test grammar of the rule engine, which has no field
    /// of its own here.
    #[cfg(all(test, feature = "_embedding"))]
    pub(crate) test_grammar: Option<Box<dyn std::any::Any>>,
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
    /// The Visual Basic lexer's, for Visual Basic documents and Markdown
    /// `vb` fences.
    /// It is boxed and allocated by the first Visual Basic line, as Haskell's
    /// is.
    #[cfg(feature = "lang-vb")]
    pub(crate) vb: Option<Box<crate::vb::Scratch>>,
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
