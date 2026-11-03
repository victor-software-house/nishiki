//! Interned scope stacks and lexer states, shared by every lexer of one
//! highlighter.
//!
//! Both are persistent linked lists, hash-consed: a node is its parent's id
//! and one step, created once. A push is one hash lookup, a pop follows the
//! parent link, and two equal stacks or states have the same id, so comparing
//! them is one integer comparison. Memory grows with the number of distinct
//! nodes, not with depth times the number of lines.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use crate::limits::{Limit, LineReport};
use crate::scope::Scope;

/// The id of the empty scope stack, and of the document-start state.
pub(crate) const ROOT: u32 = 0;

/// A hasher for the arena's integer keys, after rustc's `FxHasher`:
/// one rotate, xor and multiply per word. The keys are ids the arena made, so
/// no adversary picks them, and the default `SipHash` would cost several
/// times more on the hot path of every token.
#[derive(Default)]
struct IdHasher(u64);

/// The multiplier of rustc's `FxHasher`.
const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u64(u64::from(byte));
        }
    }

    fn write_u64(&mut self, word: u64) {
        self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(SEED);
    }

    fn write_u16(&mut self, word: u16) {
        self.write_u64(u64::from(word));
    }

    fn write_u32(&mut self, word: u32) {
        self.write_u64(u64::from(word));
    }
}

type IdMap<K> = HashMap<K, u32, BuildHasherDefault<IdHasher>>;

/// A scope stack's key: its parent and its top scope.
type ScopeKey = (u32, u32);

/// A state's key: parent, kind, payload and scope stack.
type FrameKey = (u32, u16, u32, u32);

/// One interned scope stack: its parent stack plus one scope.
#[derive(Clone, Copy, Debug)]
struct ScopeNode {
    parent: u32,
    scope: Scope,
}

/// One interned lexer state: an open context on top of its parent state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Frame {
    /// The state this context was opened in.
    pub(crate) parent: u32,
    /// The lexer's context kind, tagged with its language.
    pub(crate) kind: u16,
    /// Data the context carries, such as a raw string's `#` count.
    pub(crate) payload: u32,
    /// The scope stack of text inside the context.
    pub(crate) stack: u32,
    /// Open contexts below and including this one.
    pub(crate) depth: u32,
}

/// The scope stacks and states of one highlighter.
#[derive(Debug)]
pub(crate) struct Arena {
    scopes: Vec<ScopeNode>,
    scope_ids: IdMap<ScopeKey>,
    frames: Vec<Frame>,
    frame_ids: IdMap<FrameKey>,
    /// Strings lexers intern: payloads such as a here-document's delimiter,
    /// and the names of scopes made from source text. Number 0 is empty.
    strings: Vec<Box<str>>,
    string_ids: HashMap<Box<str>, u32>,
    /// Most nodes each list may hold; `None` for no limit.
    limit: Option<usize>,
}

impl Arena {
    pub(crate) fn new(limit: Option<usize>) -> Self {
        let root = Frame {
            parent: ROOT,
            kind: 0,
            payload: 0,
            stack: ROOT,
            depth: 0,
        };
        Arena {
            scopes: vec![ScopeNode {
                parent: ROOT,
                scope: Scope::NONE,
            }],
            scope_ids: IdMap::default(),
            frames: vec![root],
            frame_ids: IdMap::default(),
            strings: vec![Box::from("")],
            string_ids: HashMap::new(),
            limit,
        }
    }

    fn has_room(&self, len: usize) -> bool {
        self.limit.is_none_or(|limit| len < limit)
    }

    /// The stack `parent` with `scope` on top, or `None` when the node limit
    /// keeps a new stack from being made.
    pub(crate) fn child(&mut self, parent: u32, scope: Scope) -> Option<u32> {
        let key = (parent, scope.id());
        if let Some(&id) = self.scope_ids.get(&key) {
            return Some(id);
        }
        if !self.has_room(self.scopes.len()) {
            return None;
        }
        let id = u32::try_from(self.scopes.len()).ok()?;
        self.scopes.push(ScopeNode { parent, scope });
        self.scope_ids.insert(key, id);
        Some(id)
    }

    /// The stack `base` with `scopes` on top, in order. The node limit keeps
    /// a new stack from being made: the scope it stops is left off with every
    /// scope after it, and `report` records the limit.
    pub(crate) fn path(
        &mut self,
        base: u32,
        scopes: impl IntoIterator<Item = Scope>,
        report: &mut LineReport,
    ) -> u32 {
        let mut stack = base;
        for scope in scopes {
            let Some(child) = self.child(stack, scope) else {
                report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        stack
    }

    /// The number of `text`, which stays the same for equal text, or `None`
    /// when the node limit keeps a new string from being made.
    pub(crate) fn intern(&mut self, text: &str) -> Option<u32> {
        if let Some(&id) = self.string_ids.get(text) {
            return Some(id);
        }
        if !self.has_room(self.strings.len()) {
            return None;
        }
        let id = u32::try_from(self.strings.len()).ok()?;
        self.strings.push(Box::from(text));
        self.string_ids.insert(Box::from(text), id);
        Some(id)
    }

    /// The text of string `id`; empty for a number this arena never made.
    pub(crate) fn string(&self, id: u32) -> &str {
        usize::try_from(id)
            .ok()
            .and_then(|index| self.strings.get(index))
            .map_or("", |text| text)
    }

    /// The scope named `name`, made at run time, or `None` when a limit keeps
    /// it from being made.
    pub(crate) fn named_scope(&mut self, name: &str) -> Option<Scope> {
        let index = u16::try_from(self.intern(name)?).ok()?;
        Some(Scope::dynamic(index))
    }

    /// The stack `stack` without its top scope; the empty stack for an id
    /// this arena never made.
    pub(crate) fn parent(&self, stack: u32) -> u32 {
        usize::try_from(stack)
            .ok()
            .and_then(|index| self.scopes.get(index))
            .map_or(ROOT, |node| node.parent)
    }

    /// The scopes of `stack`, innermost first; empty for an id this arena
    /// never made.
    pub(crate) fn scopes(&self, stack: u32) -> Scopes<'_> {
        let valid = usize::try_from(stack).is_ok_and(|index| index < self.scopes.len());
        Scopes {
            arena: self,
            node: if valid { stack } else { ROOT },
        }
    }

    /// The frame of `state`; the document start for an id this arena never
    /// made.
    pub(crate) fn frame(&self, state: u32) -> Frame {
        usize::try_from(state)
            .ok()
            .and_then(|index| self.frames.get(index))
            .copied()
            .unwrap_or(self.frames[0])
    }

    /// The state `parent` with a context of `kind` opened on it, or `None`
    /// when the node limit keeps a new state from being made.
    pub(crate) fn push(&mut self, parent: u32, kind: u16, payload: u32, stack: u32) -> Option<u32> {
        let key = (parent, kind, payload, stack);
        if let Some(&id) = self.frame_ids.get(&key) {
            return Some(id);
        }
        if !self.has_room(self.frames.len()) {
            return None;
        }
        let id = u32::try_from(self.frames.len()).ok()?;
        let depth = self.frame(parent).depth + 1;
        self.frames.push(Frame {
            parent,
            kind,
            payload,
            stack,
            depth,
        });
        self.frame_ids.insert(key, id);
        Some(id)
    }
}

/// The scope names of one stack, innermost first, as theme rules match them.
#[derive(Clone, Debug)]
pub struct Scopes<'a> {
    arena: &'a Arena,
    node: u32,
}

impl<'a> Iterator for Scopes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.node == ROOT {
            return None;
        }
        let node = self.arena.scopes[usize::try_from(self.node).ok()?];
        self.node = node.parent;
        Some(match node.scope.dynamic_index() {
            Some(index) => self.arena.strings.get(index).map_or("", |text| text),
            None => node.scope.name(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_stacks_share_one_id() {
        let mut arena = Arena::new(None);
        let a = arena.child(ROOT, Scope::new(1, 0)).unwrap();
        let b = arena.child(ROOT, Scope::new(1, 0)).unwrap();
        assert_eq!(a, b);
        let deeper = arena.child(a, Scope::new(1, 1)).unwrap();
        assert_ne!(deeper, a);
        assert_eq!(arena.scopes(deeper).count(), 2);
    }

    #[test]
    fn equal_states_share_one_id() {
        let mut arena = Arena::new(None);
        let a = arena.push(ROOT, 3, 7, ROOT).unwrap();
        let b = arena.push(ROOT, 3, 7, ROOT).unwrap();
        assert_eq!(a, b);
        assert_ne!(arena.push(ROOT, 3, 8, ROOT).unwrap(), a);
        assert_eq!(arena.frame(a).depth, 1);
        assert_eq!(arena.frame(a).parent, ROOT);
    }

    #[test]
    fn strings_and_dynamic_scopes_are_interned() {
        let mut arena = Arena::new(None);
        let a = arena.intern("EOF").unwrap();
        assert_eq!(arena.intern("EOF"), Some(a));
        assert_eq!(arena.string(a), "EOF");
        let scope = arena.named_scope("string.quoted.heredoc.EOF").unwrap();
        let stack = arena.child(ROOT, scope).unwrap();
        assert_eq!(
            arena.scopes(stack).collect::<Vec<_>>(),
            ["string.quoted.heredoc.EOF"]
        );
    }

    #[test]
    fn the_node_limit_stops_new_nodes_only() {
        let mut arena = Arena::new(Some(2));
        let a = arena.child(ROOT, Scope::new(1, 0)).unwrap();
        assert_eq!(arena.child(ROOT, Scope::new(1, 0)), Some(a));
        assert_eq!(arena.child(ROOT, Scope::new(1, 1)), None);
    }
}
