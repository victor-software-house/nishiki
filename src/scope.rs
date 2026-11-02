//! Scope names of every language, numbered in one space, so that the stacks
//! of a language embedded in another (Rust in a Markdown fence) live in the
//! same arena.

/// A scope name: a language's table base plus the name's index in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Scope(u16);

/// Each language's scope names, with the base its scopes are numbered from.
/// Bases ascend; a new language's base is the previous one's end.
const TABLES: &[(u16, &[&str])] = &[
    (RUST, crate::rust::NAMES),
    (MARKDOWN, crate::markdown::NAMES),
    (TYPESCRIPT, crate::typescript::NAMES),
    (SHELL, crate::shell::NAMES),
];

/// The base of the Rust lexer's scope names.
pub(crate) const RUST: u16 = 1;

/// The base of the Markdown lexer's scope names: where Rust's end.
pub(crate) const MARKDOWN: u16 = after(RUST, crate::rust::NAMES);

/// The base of the TypeScript lexer's scope names: where Markdown's end.
pub(crate) const TYPESCRIPT: u16 = after(MARKDOWN, crate::markdown::NAMES);

/// The base of the table that follows `names`, which is numbered from `base`.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "`u16::try_from` is not const; the assert checks the length fits"
)]
const fn after(base: u16, names: &[&str]) -> u16 {
    assert!(names.len() <= (u16::MAX - base) as usize);
    base + names.len() as u16
}

/// The base of the shell lexer's scope names: where TypeScript's end.
pub(crate) const SHELL: u16 = after(TYPESCRIPT, crate::typescript::NAMES);

/// Scope numbers from here up name scopes made at run time, such as the
/// delimiter of a here-document; the arena resolves them.
pub(crate) const DYNAMIC: u16 = 0x8000;

// The tables end below the run-time scopes.
const _: () = assert!(after(SHELL, crate::shell::NAMES) <= DYNAMIC);

impl Scope {
    /// The placeholder scope of the empty stack.
    pub(crate) const NONE: Scope = Scope(0);

    /// The scope at `index` in the table numbered from `base`.
    pub(crate) const fn new(base: u16, index: u16) -> Scope {
        Scope(base + index)
    }

    /// The scope made at run time from the arena's string number `index`.
    pub(crate) const fn dynamic(index: u16) -> Scope {
        Scope(DYNAMIC + index)
    }

    pub(crate) fn id(self) -> u16 {
        self.0
    }

    /// The index of the arena string that names the scope, when it was made
    /// at run time.
    pub(crate) fn dynamic_index(self) -> Option<usize> {
        self.0.checked_sub(DYNAMIC).map(usize::from)
    }

    /// The scope's name, such as `"string.quoted.double.rust"`.
    pub(crate) fn name(self) -> &'static str {
        TABLES
            .iter()
            .rev()
            .find(|&&(base, _)| self.0 >= base && self.0 < DYNAMIC)
            .and_then(|&(base, names)| names.get(usize::from(self.0 - base)))
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
        assert_eq!(Scope::NONE.name(), "");
    }

    #[test]
    fn each_table_starts_where_the_last_ends() {
        assert_eq!(
            usize::from(MARKDOWN - RUST),
            crate::rust::NAMES.len(),
            "MARKDOWN is RUST plus the number of Rust scopes"
        );
        assert_eq!(
            usize::from(TYPESCRIPT - MARKDOWN),
            crate::markdown::NAMES.len(),
            "TYPESCRIPT is MARKDOWN plus the number of Markdown scopes"
        );
    }
}
