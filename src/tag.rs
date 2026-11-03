//! The high byte of every frame kind, one constant per lexer module.
//!
//! A frame of the shared arena (`src/arena.rs`) carries a kind code, and the
//! tag in its high bits says which lexer's enum the low bits belong to.
//! Lexers are written in parallel, so the tags are allocated here and nowhere
//! else; a lexer module takes its constant and never a "next free byte".
//! A lexer that needs more than the low byte for its own bits (a flag next to
//! the kind) claims a wider range by giving its row a `mask` that covers
//! fewer bits of the high byte, and the check below counts the whole range.

/// A claimed range of the code space: the codes `c` with `c & mask == value`.
struct Tag {
    value: u16,
    mask: u16,
}

/// Claims each lexer's range, one row per language: the constant, its value
/// and, for a lexer that claims a wider range for flag bits, the mask that
/// covers it. A language's row is on only when its feature is, so the overlap
/// check below counts the languages that are built.
macro_rules! tags {
    (@mask) => { 0xff00 };
    (@mask $mask:literal) => { $mask };
    ($(
        $(#[doc = $doc:literal])*
        $(#[cfg($($cfg:tt)*)])*
        $name:ident = $value:literal $(mask $mask:literal)?,
    )*) => {
        $(
            $(#[doc = $doc])*
            $(#[cfg($($cfg)*)])*
            pub(crate) const $name: u16 = $value;
        )*

        /// Every claimed range.
        const CLAIMED: &[Tag] = &[
            $(
                $(#[cfg($($cfg)*)])*
                Tag { value: $name, mask: tags!(@mask $($mask)?) },
            )*
        ];
    };
}

tags! {
    /// Rust.
    #[cfg(feature = "lang-rust")]
    RUST = 0x0100,
    /// Markdown.
    #[cfg(feature = "lang-markdown")]
    MARKDOWN = 0x0200,
    /// TypeScript, JavaScript, JSX and TSX.
    #[cfg(any(
    feature = "lang-typescript",
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx"
))]
    TYPESCRIPT = 0x0300,
    /// Shell.
    #[cfg(feature = "lang-shell")]
    SHELL = 0x0400,
    /// JSON.
    #[cfg(feature = "lang-json")]
    JSON = 0x0500,
    /// Python.
    #[cfg(feature = "lang-python")]
    PYTHON = 0x0600,
    /// YAML.
    #[cfg(feature = "lang-yaml")]
    YAML = 0x0800,
    /// TOML.
    #[cfg(feature = "lang-toml")]
    TOML = 0x0900,
    /// Diff.
    #[cfg(feature = "lang-diff")]
    DIFF = 0x0e00,
    /// Java.
    #[cfg(feature = "lang-java")]
    JAVA = 0x0a00,
    /// Makefiles.
    #[cfg(feature = "lang-make")]
    MAKE = 0x0b00,
    /// Dockerfiles.
    #[cfg(feature = "lang-dockerfile")]
    DOCKERFILE = 0x2000,
    /// C, with the next byte for its end-of-line flag.
    #[cfg(feature = "lang-c")]
    C = 0x0c00 mask 0xfe00,
    /// SQL.
    #[cfg(feature = "lang-sql")]
    SQL = 0x0f00,
    /// INI files.
    #[cfg(feature = "lang-ini")]
    INI = 0x2100,
    /// GraphQL.
    #[cfg(feature = "lang-graphql")]
    GRAPHQL = 0x2200,
    /// Go.
    #[cfg(feature = "lang-go")]
    GO = 0x2300,
    /// CSS.
    #[cfg(feature = "lang-css")]
    CSS = 0x2600,
    /// Ruby.
    #[cfg(feature = "lang-ruby")]
    RUBY = 0x2700,
    /// Kotlin.
    #[cfg(feature = "lang-kotlin")]
    KOTLIN = 0x2a00,
}

/// Two ranges overlap when some code lies in both: when their values agree on
/// every bit that both masks look at.
const fn all_distinct() -> bool {
    let mut i = 0;
    while i < CLAIMED.len() {
        let mut j = i + 1;
        while j < CLAIMED.len() {
            let both = CLAIMED[i].mask & CLAIMED[j].mask;
            if (CLAIMED[i].value ^ CLAIMED[j].value) & both == 0 {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const _: () = assert!(all_distinct(), "two lexers claim the same frame tag");
