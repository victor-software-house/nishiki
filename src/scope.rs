//! Scope names of every language, numbered in one space, so that the stacks
//! of a language embedded in another (Rust in a Markdown fence) live in the
//! same arena.

/// A scope name: a language's table base plus the name's index in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Scope(u16);

/// Each language's scope names, with the base its scopes are numbered from.
/// Bases ascend; a new language's base is the previous one's end.
const TABLES: &[(u16, &[&str])] = &[(RUST, crate::rust::NAMES)];

/// The base of the Rust lexer's scope names.
pub(crate) const RUST: u16 = 1;

impl Scope {
    /// The placeholder scope of the empty stack.
    pub(crate) const NONE: Scope = Scope(0);

    /// The scope at `index` in the table numbered from `base`.
    pub(crate) const fn new(base: u16, index: u16) -> Scope {
        Scope(base + index)
    }

    pub(crate) fn id(self) -> u16 {
        self.0
    }

    /// The scope's name, such as `"string.quoted.double.rust"`.
    pub(crate) fn name(self) -> &'static str {
        TABLES
            .iter()
            .rev()
            .find(|&&(base, _)| self.0 >= base)
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
        assert_eq!(Scope::NONE.name(), "");
    }
}
