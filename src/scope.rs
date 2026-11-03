//! Scope names of every language, numbered in one space, so that the stacks
//! of a language embedded in another (Rust in a Markdown fence) live in the
//! same arena.
//!
//! A scope number is a table's slot in the high 16 bits and the name's index
//! in that table in the low 16. A table's slot is the high byte of its
//! language's frame tag (`src/tag.rs`), so it needs no number of its own and
//! does not depend on any other table's length, or on which languages are
//! built.

/// A scope name: a language's table base plus the name's index in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Scope(u32);

/// Scope numbers from here up name scopes made at run time, such as the
/// delimiter of a here-document; the arena resolves them.
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

/// Registers each language's scope table, one row per language: the constant
/// that names its base, which is also the name of its frame tag, and its
/// module. A row is on only when its language's feature is.
macro_rules! tables {
    ($($(#[cfg($($cfg:tt)*)])* $base:ident = $module:ident;)*) => {
        $(
            $(#[cfg($($cfg)*)])*
            /// The base of this language's scope names.
            pub(crate) const $base: u32 = base(crate::tag::$base);
        )*

        /// Every language's scope names, by slot. Each row rebinds the array
        /// rather than writing into one mutable binding, so a build with no
        /// language that has a table leaves nothing unused.
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
    #[cfg(feature = "lang-c")]
    C = c;
    #[cfg(feature = "lang-cpp")]
    CPP = cpp;
    #[cfg(feature = "lang-csharp")]
    CSHARP = csharp;
    #[cfg(feature = "lang-css")]
    CSS = css;
    #[cfg(feature = "lang-diff")]
    DIFF = diff;
    #[cfg(feature = "lang-dockerfile")]
    DOCKERFILE = dockerfile;
    #[cfg(feature = "lang-go")]
    GO = go;
    #[cfg(feature = "lang-graphql")]
    GRAPHQL = graphql;
    #[cfg(feature = "lang-html")]
    HTML = html;
    #[cfg(feature = "lang-ini")]
    INI = ini;
    #[cfg(feature = "lang-java")]
    JAVA = java;
    #[cfg(feature = "lang-json")]
    JSON = json;
    #[cfg(feature = "lang-kotlin")]
    KOTLIN = kotlin;
    #[cfg(feature = "lang-lua")]
    LUA = lua;
    #[cfg(feature = "lang-make")]
    MAKE = make;
    #[cfg(feature = "lang-markdown")]
    MARKDOWN = markdown;
    #[cfg(feature = "lang-python")]
    PYTHON = python;
    #[cfg(feature = "lang-ruby")]
    RUBY = ruby;
    #[cfg(feature = "lang-rust")]
    RUST = rust;
    #[cfg(feature = "lang-scss")]
    SCSS = scss;
    #[cfg(feature = "lang-shell")]
    SHELL = shell;
    #[cfg(feature = "lang-sql")]
    SQL = sql;
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
#[cfg(feature = "_scope-table")]
macro_rules! scope_table {
    (
        $(#[$meta:meta])* $base:ident;
        $($(#[cfg($($cfg:tt)*)])* $id:ident $name:literal)*
    ) => {
        /// A scope name; the discriminant indexes [`NAMES`].
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
