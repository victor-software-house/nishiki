//! Rust, with the scopes of the Rust grammar that Shiki bundles.
//!
//! The lexer is a hand-written state machine over bytes. [`State`] is the stack
//! of contexts left open at the end of a line (strings, comments, calls,
//! generics and so on); it is small and `Copy`. Scope stacks are interned once
//! per highlighter, so lexing allocates only for stacks it has not seen.
//!
//! It reproduces Shiki's output, quirks included: `||` is two `|` operators,
//! `a!=b` reads `a!` as a macro, and a camel-case name such as `fooBar` gets no
//! scope.
//!
//! Limits: a context nested deeper than 32 is not tracked (a block comment is
//! only counted, anything else is lexed as plain text); a raw string with more
//! than 255 `#` is not a raw string; and every non-ASCII byte counts as a word
//! character.

mod code;
mod lexer;
mod numbers;
mod operators;
mod scope;
mod state;
mod text;

use scope::{NAMES, S};
pub use state::State;

use crate::{Stack, Token};

/// One interned scope stack.
#[derive(Debug)]
struct Node {
    /// The full stack, outermost first.
    names: Vec<&'static str>,
    /// Child node per scope, 0 when not yet created.
    children: Box<[usize]>,
}

/// Interns scope stacks as a trie, so a token's stack is found by following
/// one child link per scope.
#[derive(Debug)]
struct Interner {
    nodes: Vec<Node>,
}

impl Interner {
    fn new() -> Self {
        let root = Node {
            names: vec![NAMES[S::Source.index()]],
            children: vec![0; NAMES.len()].into_boxed_slice(),
        };
        Interner { nodes: vec![root] }
    }

    fn child(&mut self, parent: usize, scope: S) -> usize {
        let known = self.nodes[parent].children[scope.index()];
        if known != 0 {
            return known;
        }
        let mut names = self.nodes[parent].names.clone();
        names.push(NAMES[scope.index()]);
        let id = self.nodes.len();
        self.nodes.push(Node {
            names,
            children: vec![0; NAMES.len()].into_boxed_slice(),
        });
        self.nodes[parent].children[scope.index()] = id;
        id
    }

    fn path(&mut self, mut node: usize, scopes: &[S]) -> usize {
        for &scope in scopes {
            node = self.child(node, scope);
        }
        node
    }
}

/// The Rust highlighter. It owns the interned scope stacks its tokens name.
#[derive(Debug)]
pub struct Highlighter {
    interner: Interner,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter {
    /// A highlighter with no scope stacks interned yet.
    #[must_use]
    pub fn new() -> Self {
        Highlighter {
            interner: Interner::new(),
        }
    }

    /// Lexes `line`, which excludes its line ending, starting from `state`.
    ///
    /// Appends tokens that cover every byte of `line` in order (an empty line
    /// may append none), and leaves in `state` the state for the next line.
    /// Start a document from [`State::default`].
    pub fn line(&mut self, line: &str, state: &mut State, out: &mut Vec<Token>) {
        let mut lexer = lexer::Lexer::new(&mut self.interner, line.as_bytes(), out, *state);
        lexer.run();
        *state = lexer.into_state();
    }

    /// The scope names of `stack`, outermost first, such as
    /// `["source.rust", "string.quoted.double.rust"]`.
    ///
    /// Empty when `stack` is unknown to this highlighter. A stack from another
    /// highlighter resolves to an unrelated stack or to nothing.
    #[must_use]
    pub fn scopes(&self, stack: Stack) -> &[&'static str] {
        self.interner
            .nodes
            .get(stack.0)
            .map_or(&[], |node| &node.names)
    }
}

#[cfg(test)]
mod tests;
