//! Scope names of every language, numbered in one space, so that the stacks
//! of a language embedded in another (Rust in a Markdown fence) live in the
//! same arena.
//!
//! A scope number is a table's slot in the high 16 bits and the name's index
//! in that table in the low 16. A table's slot is its position in [`tables!`]
//! below, so it does not depend on any other table's length.

/// A scope name: a language's table base plus the name's index in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Scope(u32);

/// Scope numbers from here up name scopes made at run time, such as the
/// delimiter of a here-document; the arena resolves them.
pub(crate) const DYNAMIC: u32 = 0x8000_0000;

/// What the low 16 bits of a scope number index: the most names one table
/// can hold.
const TABLE_SPAN: usize = 1 << 16;

/// The base of the table in `slot`. Slot 0 is the placeholder scope of the
/// empty stack, so tables start at slot 1.
#[expect(
    clippy::as_conversions,
    reason = "`u32::from` is not const, and a `repr(u16)` enum has no `From` for its discriminant"
)]
const fn base(slot: Slot) -> u32 {
    (slot as u32 + 1) << 16
}

/// Registers each language's scope table, one line per language: the constant
/// that names its base, and its module. The order sets the slots, so a new
/// language is appended.
macro_rules! tables {
    ($($base:ident $module:ident)*) => {
        /// A table's position in [`TABLES`], named for its module.
        #[repr(u16)]
        #[expect(
            non_camel_case_types,
            reason = "each variant is its language's module name"
        )]
        enum Slot { $($module),* }

        $(
            /// The base of this language's scope names.
            pub(crate) const $base: u32 = base(Slot::$module);
        )*

        /// Every language's scope names, by slot.
        const TABLES: &[&[&str]] = &[$(crate::$module::NAMES),*];
    };
}

tables! {
    RUST rust
    MARKDOWN markdown
    TYPESCRIPT typescript
    SHELL shell
    JSON json
    PYTHON python
    YAML yaml
    TOML toml
    MAKE make
    DIFF diff
    DOCKERFILE dockerfile
    JAVA java
    C c
    SQL sql
    INI ini
    GRAPHQL graphql
    CSS css
}

// No table is longer than its slot, and the last slot ends below the
// run-time scopes, so no two scopes share a number.
const _: () = {
    let mut i = 0;
    while i < TABLES.len() {
        assert!(
            TABLES[i].len() < TABLE_SPAN,
            "a scope table outgrew its slot"
        );
        i += 1;
    }
    assert!(
        TABLES.len() < 0x7fff,
        "the scope tables run into the run-time scopes"
    );
};

/// The scope table of a language whose names are one list: the enum of
/// scopes, the table of their names in Shiki's spelling, and the conversion
/// to a [`Scope`] numbered from the language's base.
macro_rules! scope_table {
    ($(#[$meta:meta])* $base:ident; $($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        $(#[$meta])*
        pub(crate) enum S { $($id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::$base, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

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
        let slot = usize::try_from(self.0 >> 16)
            .ok()
            .and_then(|slot| slot.checked_sub(1));
        let index = usize::try_from(self.0 & 0xffff).ok();
        slot.and_then(|slot| TABLES.get(slot))
            .zip(index)
            .and_then(|(names, index)| names.get(index))
            .copied()
            .unwrap_or("")
    }
}

#[cfg(test)]
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
