//! The high byte of every frame rule, one constant per lexer module.
//!
//! A frame of the shared arena (`src/arena.rs`) carries a `rule` code.
//! The tag in its high bits marks which lexer's enum the low bits belong to.
//! Lexers are written in parallel, so the tags are allocated here and nowhere
//! else.
//! A lexer module takes its constant and never a "next free byte".
//!
//! A lexer may need more than the low byte for its own bits.
//! It may keep a flag next to its enum's variant, or have more variants than a
//! byte numbers, as C++'s 686 rules do.
//!
//! Such a lexer claims a wider range by giving its row a `mask` that covers
//! fewer bits of the high byte.
//! The check below counts the whole range.
//!
//! Byte `0x10` belongs to the `TextMate` engine, for every language it runs.

/// A claimed range of the code space: the codes `c` with `c & mask == value`.
struct Tag {
    value: u16,
    mask: u16,
}

/// Claims each lexer's range, one row per language.
///
/// A row holds the constant and its value.
/// A lexer that claims a wider range for flag bits adds the mask that covers
/// it.
/// Each language's row is on only when its feature is.
/// So the overlap check below counts the languages that are built.
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
    /// Assembly.
    #[cfg(feature = "lang-asm")]
    ASM = 0x4a00,
    /// Astro.
    #[cfg(feature = "lang-astro")]
    ASTRO = 0x5000,
    /// Batch.
    #[cfg(feature = "lang-bat")]
    BAT = 0x3500,
    /// Blade's PHP.
    #[cfg(feature = "lang-blade")]
    BLADE = 0x5200,
    /// C, with the next byte for its end-of-line flag.
    #[cfg(feature = "lang-c")]
    C = 0x0c00 mask 0xfe00,
    /// CMake.
    #[cfg(feature = "lang-cmake")]
    CMAKE = 0x3300,
    /// CoffeeScript.
    #[cfg(feature = "lang-coffee")]
    COFFEE = 0x3e00,
    /// Common Lisp.
    #[cfg(feature = "lang-common-lisp")]
    COMMON_LISP = 0x4900,
    /// C++, with the next byte for the number of its rule.
    #[cfg(feature = "lang-cpp")]
    CPP = 0x2400 mask 0xfe00,
    /// C#.
    #[cfg(feature = "lang-csharp")]
    CSHARP = 0x2c00,
    /// CSS.
    #[cfg(feature = "lang-css")]
    CSS = 0x2600,
    /// Dart.
    #[cfg(feature = "lang-dart")]
    DART = 0x3000,
    /// Diff.
    #[cfg(feature = "lang-diff")]
    DIFF = 0x0e00,
    /// Dockerfiles.
    #[cfg(feature = "lang-dockerfile")]
    DOCKERFILE = 0x2000,
    /// Elixir.
    #[cfg(feature = "lang-elixir")]
    ELIXIR = 0x4300,
    /// GDScript.
    #[cfg(feature = "lang-gdscript")]
    GDSCRIPT = 0x4600,
    /// Go.
    #[cfg(feature = "lang-go")]
    GO = 0x2300,
    /// GraphQL.
    #[cfg(feature = "lang-graphql")]
    GRAPHQL = 0x2200,
    /// Hack.
    #[cfg(feature = "lang-hack")]
    HACK = 0x5100,
    /// HAML.
    #[cfg(feature = "lang-haml")]
    HAML = 0x4800,
    /// Haskell.
    #[cfg(feature = "lang-haskell")]
    HASKELL = 0x3600,
    /// HCL.
    #[cfg(feature = "lang-hcl")]
    HCL = 0x4F00,
    /// HLSL.
    #[cfg(feature = "lang-hlsl")]
    HLSL = 0x5300,
    /// HTML.
    #[cfg(feature = "lang-html")]
    HTML = 0x2800,
    /// INI files.
    #[cfg(feature = "lang-ini")]
    INI = 0x2100,
    /// Java.
    #[cfg(feature = "lang-java")]
    JAVA = 0x0a00,
    /// Jinja.
    #[cfg(feature = "lang-jinja")]
    JINJA = 0x4e00,
    /// JSON.
    #[cfg(feature = "lang-json")]
    JSON = 0x0500,
    /// Julia.
    #[cfg(feature = "lang-julia")]
    JULIA = 0x3400,
    /// Kotlin.
    #[cfg(feature = "lang-kotlin")]
    KOTLIN = 0x2a00,
    /// LaTeX.
    #[cfg(feature = "lang-latex")]
    LATEX = 0x4b00,
    /// Less.
    #[cfg(feature = "lang-less")]
    LESS = 0x2f00,
    /// Lua.
    #[cfg(feature = "lang-lua")]
    LUA = 0x2d00,
    /// Makefiles.
    #[cfg(feature = "lang-make")]
    MAKE = 0x0b00,
    /// Markdown.
    #[cfg(feature = "lang-markdown")]
    MARKDOWN = 0x0200,
    /// MATLAB.
    #[cfg(feature = "lang-matlab")]
    MATLAB = 0x4400,
    /// Mermaid.
    #[cfg(feature = "lang-mermaid")]
    MERMAID = 0x3d00,
    /// Nix.
    #[cfg(feature = "lang-nix")]
    NIX = 0x3700,
    /// Objective-C.
    #[cfg(feature = "lang-objective-c")]
    OBJECTIVE_C = 0x4000,
    /// Objective-C++.
    #[cfg(feature = "lang-objective-cpp")]
    OBJECTIVE_CPP = 0x4100,
    /// Pascal.
    #[cfg(feature = "lang-pascal")]
    PASCAL = 0x4500,
    /// Perl.
    #[cfg(feature = "lang-perl")]
    PERL = 0x4200,
    /// PHP.
    #[cfg(feature = "lang-php")]
    PHP = 0x3F00,
    /// PostCSS.
    #[cfg(feature = "lang-postcss")]
    POSTCSS = 0x3B00,
    /// PowerShell.
    #[cfg(feature = "lang-powershell")]
    POWERSHELL = 0x3100,
    /// Protobuf.
    #[cfg(feature = "lang-proto")]
    PROTO = 0x3900,
    /// Python.
    #[cfg(feature = "lang-python")]
    PYTHON = 0x0600,
    /// R.
    #[cfg(feature = "lang-r")]
    R = 0x4D00,
    /// Ruby.
    #[cfg(feature = "lang-ruby")]
    RUBY = 0x2700,
    /// Rust.
    #[cfg(feature = "lang-rust")]
    RUST = 0x0100,
    /// Sass.
    #[cfg(feature = "lang-sass")]
    SASS = 0x3a00,
    /// Scala.
    #[cfg(feature = "lang-scala")]
    SCALA = 0x3200,
    /// SCSS.
    #[cfg(feature = "lang-scss")]
    SCSS = 0x2e00,
    /// ShaderLab.
    #[cfg(feature = "lang-shaderlab")]
    SHADERLAB = 0x5400,
    /// Shell.
    #[cfg(feature = "lang-shell")]
    SHELL = 0x0400,
    /// SQL.
    #[cfg(feature = "lang-sql")]
    SQL = 0x0f00,
    /// Svelte.
    #[cfg(feature = "lang-svelte")]
    SVELTE = 0x3c00,
    /// Swift.
    #[cfg(feature = "lang-swift")]
    SWIFT = 0x2b00,
    /// TeX.
    #[cfg(feature = "lang-tex")]
    TEX = 0x4c00,
    /// The `TextMate` engine's, for every language it runs.
    #[cfg(feature = "_textmate")]
    TEXTMATE = 0x1000,
    /// TOML.
    #[cfg(feature = "lang-toml")]
    TOML = 0x0900,
    /// TypeScript, JavaScript, JSX and TSX.
    #[cfg(any(
    feature = "lang-typescript",
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx"
))]
    TYPESCRIPT = 0x0300,
    /// Visual Basic.
    #[cfg(feature = "lang-vb")]
    VB = 0x4700,
    /// XML.
    #[cfg(feature = "lang-xml")]
    XML = 0x2900,
    /// YAML.
    #[cfg(feature = "lang-yaml")]
    YAML = 0x0800,
    /// Zig.
    #[cfg(feature = "lang-zig")]
    ZIG = 0x3800,
}

/// Two ranges overlap when some code lies in both: when their values match on
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

/// The bits of a frame code that name the variant within a lexer's enum; the
/// bits above them hold the lexer's tag.
#[cfg(feature = "_frame-rules")]
pub(crate) const RULE_MASK: u16 = 0x00ff;

/// Declares a lexer's frame rules: a fieldless enum tagged with `$tag`.
///
/// The enum gets `ALL`, `code` and `from_code`.
/// A test checks that every variant survives the round trip through its code.
///
/// The first variant stands for the level outside every context.
/// `from_code` returns it for a code another lexer wrote, such as the empty
/// state's.
/// A lexer started on a foreign state therefore reads it as its top level.
/// A variant's code is its position in the declaration, so variants are only
/// ever added at the end.
#[cfg(feature = "_frame-rules")]
macro_rules! frame_rules {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident: $tag:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl $name {
            /// Every variant, in code order.
            $vis const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The variant as a frame stores it: the lexer's tag in the high
            /// byte, the variant's position in the low one.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
            )]
            $vis const fn code(self) -> u16 {
                $tag | self as u16
            }

            /// The variant a frame's code names, or the first variant for a
            /// code that is not this lexer's.
            $vis fn from_code(code: u16) -> $name {
                const TOP: $name = $name::ALL[0];
                if code & !$crate::tag::RULE_MASK != $tag {
                    return TOP;
                }
                $name::ALL
                    .get(usize::from(code & $crate::tag::RULE_MASK))
                    .copied()
                    .unwrap_or(TOP)
            }
        }

        #[cfg(test)]
        mod frame_rules_round_trip {
            use super::$name;

            #[test]
            fn every_rule_survives_its_code() {
                for (index, &rule) in $name::ALL.iter().enumerate() {
                    let code = rule.code();
                    assert_eq!(usize::from(code & $crate::tag::RULE_MASK), index, "{rule:?}");
                    assert_eq!($name::from_code(code), rule);
                }
                assert_eq!($name::from_code(0), $name::ALL[0]);
            }
        }
    };
}

#[cfg(feature = "_frame-rules")]
pub(crate) use frame_rules;
