//! Scope names of every language, numbered in one space.
//! The stacks of a language embedded in another, such as Rust in a Markdown
//! fence, then live in the same arena.
//!
//! A scope number is a table's slot in the high 16 bits and the name's index
//! in that table in the low 16.
//! A table's slot is the high byte of its language's frame tag
//! (`src/tag.rs`).
//! The slot therefore needs no number of its own.
//! It does not depend on any other table's length, or on which languages are
//! built.

/// A scope name: a language's table base plus the name's index in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Scope(u32);

/// Scope numbers from here up name scopes made at run time, such as the
/// delimiter of a here-document.
/// The arena resolves them.
pub(crate) const DYNAMIC: u32 = 0x8000_0000;

/// What the low 16 bits of a scope number index: the most names one table
/// can hold.
const TABLE_SPAN: usize = 1 << 16;

/// How many slots a frame tag's high byte can name.
const SLOTS: usize = 0x100;

/// The slot of the table whose language has the frame tag `tag`.
#[expect(
    clippy::as_conversions,
    reason = "`usize::from` is not const, and a `u16` widens without loss"
)]
const fn slot(tag: u16) -> usize {
    (tag >> 8) as usize
}

/// The base of the table whose language has the frame tag `tag`.
#[expect(
    clippy::as_conversions,
    reason = "`u32::from` is not const, and a `u16` widens without loss"
)]
const fn base(tag: u16) -> u32 {
    ((tag >> 8) as u32) << 16
}

/// Registers each language's scope table, one row per language.
/// A row holds the constant that names the table's base, and the language's
/// module.
/// The constant is also the name of the language's frame tag.
/// A row is on only when its language's feature is.
macro_rules! tables {
    ($($(#[cfg($($cfg:tt)*)])* $base:ident = $module:ident;)*) => {
        $(
            $(#[cfg($($cfg)*)])*
            /// The base of this language's scope names.
            pub(crate) const $base: u32 = base(crate::tag::$base);
        )*

        /// Every language's scope names, by slot.
        /// Each row rebinds the array rather than writing into one mutable
        /// binding.
        /// A build with no language that has a table then leaves nothing
        /// unused.
        const TABLES: [&[&str]; SLOTS] = {
            let tables: [&[&str]; SLOTS] = [&[]; SLOTS];
            $(
                $(#[cfg($($cfg)*)])*
                let tables = {
                    let mut tables = tables;
                    tables[slot(crate::tag::$base)] = crate::$module::NAMES;
                    tables
                };
            )*
            tables
        };
    };
}

tables! {
    #[cfg(feature = "lang-bat")]
    BAT = bat;
    #[cfg(feature = "lang-c")]
    C = c;
    #[cfg(feature = "lang-cmake")]
    CMAKE = cmake;
    #[cfg(feature = "lang-cpp")]
    CPP = cpp;
    #[cfg(feature = "lang-csharp")]
    CSHARP = csharp;
    #[cfg(feature = "lang-css")]
    CSS = css;
    #[cfg(feature = "lang-dart")]
    DART = dart;
    #[cfg(feature = "lang-diff")]
    DIFF = diff;
    #[cfg(feature = "lang-dockerfile")]
    DOCKERFILE = dockerfile;
    #[cfg(feature = "lang-go")]
    GO = go;
    #[cfg(feature = "lang-graphql")]
    GRAPHQL = graphql;
    #[cfg(feature = "lang-haskell")]
    HASKELL = haskell;
    #[cfg(feature = "lang-html")]
    HTML = html;
    #[cfg(feature = "lang-ini")]
    INI = ini;
    #[cfg(feature = "lang-java")]
    JAVA = java;
    #[cfg(feature = "lang-json")]
    JSON = json;
    #[cfg(feature = "lang-julia")]
    JULIA = julia;
    #[cfg(feature = "lang-kotlin")]
    KOTLIN = kotlin;
    #[cfg(feature = "lang-less")]
    LESS = less;
    #[cfg(feature = "lang-lua")]
    LUA = lua;
    #[cfg(feature = "lang-make")]
    MAKE = make;
    #[cfg(feature = "lang-markdown")]
    MARKDOWN = markdown;
    #[cfg(feature = "lang-mermaid")]
    MERMAID = mermaid;
    #[cfg(feature = "lang-nix")]
    NIX = nix;
    #[cfg(feature = "lang-objective-c")]
    OBJECTIVE_C = objective_c;
    #[cfg(feature = "lang-postcss")]
    POSTCSS = postcss;
    #[cfg(feature = "lang-powershell")]
    POWERSHELL = powershell;
    #[cfg(feature = "lang-proto")]
    PROTO = proto;
    #[cfg(feature = "lang-python")]
    PYTHON = python;
    #[cfg(feature = "lang-ruby")]
    RUBY = ruby;
    #[cfg(feature = "lang-rust")]
    RUST = rust;
    #[cfg(feature = "lang-sass")]
    SASS = sass;
    #[cfg(feature = "lang-scala")]
    SCALA = scala;
    #[cfg(feature = "lang-scss")]
    SCSS = scss;
    #[cfg(feature = "lang-shell")]
    SHELL = shell;
    #[cfg(feature = "lang-sql")]
    SQL = sql;
    #[cfg(feature = "lang-svelte")]
    SVELTE = svelte;
    #[cfg(feature = "lang-swift")]
    SWIFT = swift;
    #[cfg(feature = "lang-toml")]
    TOML = toml;
    #[cfg(any(
        feature = "lang-typescript",
        feature = "lang-javascript",
        feature = "lang-jsx",
        feature = "lang-tsx"
    ))]
    TYPESCRIPT = typescript;
    #[cfg(feature = "lang-xml")]
    XML = xml;
    #[cfg(feature = "lang-yaml")]
    YAML = yaml;
    #[cfg(feature = "lang-zig")]
    ZIG = zig;
}

// No table is longer than its slot.
const _: () = {
    let mut i = 0;
    while i < SLOTS {
        assert!(
            TABLES[i].len() < TABLE_SPAN,
            "a scope table outgrew its slot"
        );
        i += 1;
    }
};

/// The scope table of a language whose names are one list: the enum of
/// scopes, the table of their names in Shiki's spelling, and the conversion
/// to a [`Scope`] numbered from the language's base.
///
/// The marker `keyed` before the base adds the scope codes and the empty
/// list.
/// They key the scope lists made with [`scopes!`].
/// Only the tables of the languages on the rule engine, and shell's, carry
/// the marker.
#[cfg(feature = "_scope-table")]
macro_rules! scope_table {
    (
        $(#[$meta:meta])* keyed $base:ident;
        $($(#[cfg($($cfg:tt)*)])* $id:ident $name:literal)*
    ) => {
        crate::scope::scope_table! {
            $(#[$meta])* $base;
            $($(#[cfg($($cfg)*)])* $id $name)*
        }

        impl S {
            /// The code of the scope in its table, its discriminant, which
            /// keys the scope lists made with [`scopes!`](crate::scope::scopes).
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u16)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn code(self) -> u16 {
                self as u16
            }

            /// The empty scope list.
            pub(crate) const NO_SCOPES: crate::scope::Scopes<S> = &crate::scope::Keyed::new([], []);
        }
    };
    (
        $(#[$meta:meta])* $base:ident;
        $($(#[cfg($($cfg:tt)*)])* $id:ident $name:literal)*
    ) => {
        /// A scope name.
        /// The discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u16)]
        $(#[$meta])*
        pub(crate) enum S { $($(#[cfg($($cfg)*)])* $id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u16)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::$base, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($(#[cfg($($cfg)*)])* $name),*];
    };
}

#[cfg(feature = "_scope-table")]
pub(crate) use scope_table;

impl Scope {
    /// The placeholder scope of the empty stack.
    pub(crate) const NONE: Scope = Scope(0);

    /// The scope at `index` in the table numbered from `base`.
    #[expect(
        clippy::as_conversions,
        reason = "`u32::from` is not const, and a `u16` widens without loss"
    )]
    pub(crate) const fn new(base: u32, index: u16) -> Scope {
        Scope(base + index as u32)
    }

    /// The scope made at run time from the arena's string number `index`.
    #[expect(
        clippy::as_conversions,
        reason = "`u32::from` is not const, and a `u16` widens without loss"
    )]
    pub(crate) const fn dynamic(index: u16) -> Scope {
        Scope(DYNAMIC + index as u32)
    }

    pub(crate) fn id(self) -> u32 {
        self.0
    }

    /// The index of the arena string that names the scope, when it was made
    /// at run time.
    pub(crate) fn dynamic_index(self) -> Option<usize> {
        self.0
            .checked_sub(DYNAMIC)
            .and_then(|index| usize::try_from(index).ok())
    }

    /// The scope's name, such as `"string.quoted.double.rust"`.
    pub(crate) fn name(self) -> &'static str {
        let slot = usize::try_from(self.0 >> 16).ok();
        let index = usize::try_from(self.0 & 0xffff).ok();
        slot.and_then(|slot| TABLES.get(slot))
            .zip(index)
            .and_then(|(names, index)| names.get(index))
            .copied()
            .unwrap_or("")
    }
}

#[cfg(all(test, feature = "all"))]
mod tests {
    use super::*;

    #[test]
    fn names_resolve_through_their_table() {
        assert_eq!(Scope::new(RUST, 0).name(), crate::rust::NAMES[0]);
        assert_eq!(Scope::new(MARKDOWN, 0).name(), crate::markdown::NAMES[0]);
        assert_eq!(
            Scope::new(TYPESCRIPT, 0).name(),
            crate::typescript::NAMES[0]
        );
        assert_eq!(Scope::new(CSS, 0).name(), crate::css::NAMES[0]);
        assert_eq!(Scope::NONE.name(), "");
    }

    #[test]
    fn the_last_name_of_a_table_resolves_and_the_next_index_does_not() {
        let last = crate::typescript::NAMES.len() - 1;
        let last = u16::try_from(last).unwrap_or(u16::MAX);
        assert_eq!(
            Scope::new(TYPESCRIPT, last).name(),
            crate::typescript::NAMES[usize::from(last)]
        );
        assert_eq!(Scope::new(TYPESCRIPT, last + 1).name(), "");
    }
}

/// A scope list of a grammar's tables, with a key computed from its scopes
/// when the crate was compiled.
///
/// The stack caches find a list by its key, not by its address.
/// An address moves when another language's tables change, which moved the
/// caches' hits and misses with it.
/// A `&Keyed<[S]>` is as wide as the `&[S]` it replaces.
#[derive(Debug)]
pub(crate) struct Keyed<T: ?Sized> {
    /// The codes of the scopes, folded as the digits of a number in base
    /// [`KEY_STEP`], each code plus one.
    pub(crate) key: u16,
    /// The scopes.
    pub(crate) list: T,
}

/// A static scope list with its key; [`scopes!`] makes one.
pub(crate) type Scopes<S> = &'static Keyed<[S]>;

/// The base of the fold that keys a list.
/// It is odd, the high half of the 32-bit golden ratio.
///
/// A list of one scope keys by its code plus one.
/// Codes are numbered in order, so these keys are near consecutive.
/// The path caches' multiplicative hash spreads consecutive keys evenly.
/// A random key collides more.
/// An FNV key missed a third more often in CSS: 3,110 misses against 2,330.
/// The plus one keeps a list apart from the same list after a code 0.
const KEY_STEP: u16 = 0x9e37;

impl<S, const N: usize> Keyed<[S; N]> {
    /// The list `list`, keyed by `codes`, the codes of its scopes in order.
    pub(crate) const fn new(list: [S; N], codes: [u16; N]) -> Self {
        let mut key: u16 = 0;
        let mut i = 0;
        while i < N {
            key = key
                .wrapping_mul(KEY_STEP)
                .wrapping_add(codes[i].wrapping_add(1));
            i += 1;
        }
        Keyed { key, list }
    }
}

/// The slots of a path cache.
/// A path cache maps a stack and a scope list to the stack the list makes on
/// it.
/// At 16 bytes an entry, the slots take 16 KiB.
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
pub(crate) const PATH_SLOTS: usize = 1024;
/// The shift that leaves the top ten bits of a `u32`: a slot of the
/// [`PATH_SLOTS`].
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
const PATH_SHIFT: u32 = 22;
/// The multiplier of the slot hash, the 32-bit golden ratio.
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
const PATH_HASH: u32 = 0x9e37_79b9;
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
const _: () = assert!(PATH_SLOTS == 1 << (u32::BITS - PATH_SHIFT));

/// The slot of the list keyed `key` on the stack `base` in a path cache.
/// It is the top bits of a multiplicative hash of both.
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
pub(crate) fn path_slot(base: u32, key: u16) -> usize {
    let hash = (base ^ u32::from(key).rotate_left(16)).wrapping_mul(PATH_HASH);
    usize::try_from(hash >> PATH_SHIFT).unwrap_or(0)
}

/// The [`Scopes`] of the scopes given: `scopes![S::A, S::B]`.
///
/// A macro over [`Keyed::new`].
/// The key must be computed in a `const` block.
/// That block needs the list written out twice, as the scopes and as their
/// codes.
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
macro_rules! scopes {
    ($($scope:expr),+ $(,)?) => {
        const { &$crate::scope::Keyed::new([$($scope),*], [$($scope.code()),*]) }
    };
}

#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
pub(crate) use scopes;

#[cfg(test)]
impl Scope {
    /// The low half of the scope's number, which keys the scope lists of the
    /// engine's test grammars, whose scopes are made at run time.
    #[expect(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        reason = "a key only spreads lists over slots, so the high half may be dropped"
    )]
    pub(crate) const fn code(self) -> u16 {
        self.0 as u16
    }

    /// The empty scope list.
    pub(crate) const NO_SCOPES: Scopes<Scope> = &Keyed::new([], []);
}
