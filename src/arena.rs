//! Interned scope stacks and lexer states, shared by every lexer of one
//! highlighter.
//!
//! Both are persistent linked lists, hash-consed.
//! A node is its parent's id and one step, created once.
//! A push is one hash lookup, and a pop follows the parent link.
//! Two equal stacks or states have the same id.
//! Comparing them is therefore one integer comparison.
//!
//! Memory grows with the number of distinct nodes, not with depth times the
//! number of lines.

use std::collections::HashMap;
#[cfg(feature = "_language")]
use std::collections::hash_map::Entry;
use std::hash::{BuildHasherDefault, Hasher};

#[cfg(feature = "_language")]
use crate::limits::{Limit, LineReport};
use crate::scope::Scope;

/// The id of the empty scope stack, and of the document-start state.
pub(crate) const ROOT: u32 = 0;

/// A hasher for the arena's integer keys, after rustc's `FxHasher`:
/// one rotate, xor and multiply per word.
/// The keys are ids the arena made, so no adversary picks them.
/// The default `SipHash` would cost several times more on the hot path of
/// every token.
#[derive(Default)]
pub(crate) struct IdHasher(u64);

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

/// A map keyed by ids an arena made, such as scope stacks, states or
/// interned strings.
/// No adversary picks the key.
/// Its probes are the same in every process, and so are the instruction
/// counts of lexing.
pub(crate) type IdMap<K, V = u32> = HashMap<K, V, BuildHasherDefault<IdHasher>>;

/// A scope stack's key: its parent and its top scope.
type ScopeKey = (u32, u32);

/// A state's key: parent, rule, payload and scope stack.
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
    /// The rule of the lexer's open context, tagged with its language.
    pub(crate) rule: u16,
    /// Data the context carries, such as a raw string's `#` count.
    pub(crate) payload: u32,
    /// The scope stack of text inside the context.
    pub(crate) stack: u32,
    /// Open contexts below and including this one.
    pub(crate) depth: u32,
}

/// The scope stacks and states of one highlighter.
#[derive(Debug)]
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub struct Arena {
    scopes: Vec<ScopeNode>,
    scope_ids: IdMap<ScopeKey>,
    frames: Vec<Frame>,
    frame_ids: IdMap<FrameKey>,
    /// Strings lexers intern: payloads such as a here-document's delimiter,
    /// and the names of scopes made from source text.
    /// Number 0 is empty.
    strings: Vec<Box<str>>,
    string_ids: HashMap<Box<str>, u32>,
    /// Most nodes each list may hold.
    /// It is `None` for no limit.
    limit: Option<usize>,
}

impl Arena {
    pub(crate) fn new(limit: Option<usize>) -> Self {
        let root = Frame {
            parent: ROOT,
            rule: 0,
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

    /// The stack `parent` with `scope` on top, or `None` when the node limit
    /// keeps a new stack from being made.
    #[inline(never)]
    pub(crate) fn child(&mut self, parent: u32, scope: Scope) -> Option<u32> {
        let key = (parent, scope.id());
        if let Some(&id) = self.scope_ids.get(&key) {
            return Some(id);
        }
        self.new_child(key, scope)
    }

    /// A [`child`](Self::child) the arena does not hold yet: makes the node
    /// keyed `key`.
    ///
    /// Out of line, so the map's insert never joins the lookup.
    /// Where it joined, at sixteen codegen units, each lookup cost more.
    #[inline(never)]
    fn new_child(&mut self, key: (u32, u32), scope: Scope) -> Option<u32> {
        let (parent, _) = key;
        let id = append(&mut self.scopes, self.limit, ScopeNode { parent, scope })?;
        self.scope_ids.insert(key, id);
        Some(id)
    }

    /// A [`child`](Self::child) for a caller that expects to make the node,
    /// such as the walk after a path cache misses.
    /// One hash of the key serves the lookup and the insert.
    /// A hit costs more than `child`'s, so a caller that usually hits calls
    /// `child`.
    #[cfg(feature = "_language")]
    #[inline(never)]
    #[cfg_attr(
        not(feature = "_hand-lexers"),
        expect(
            dead_code,
            reason = "only hand-written lexers use it, and the TextMate engine alone turns none on"
        )
    )]
    pub(crate) fn child_or_insert(&mut self, parent: u32, scope: Scope) -> Option<u32> {
        match self.scope_ids.entry((parent, scope.id())) {
            Entry::Occupied(found) => Some(*found.get()),
            Entry::Vacant(slot) => {
                let id = append(&mut self.scopes, self.limit, ScopeNode { parent, scope })?;
                Some(*slot.insert(id))
            }
        }
    }

    /// The stack `base` with `scopes` on top, in order.
    /// The node limit can keep a new stack from being made.
    /// The scope it stops is then left off with every scope after it.
    /// `report` records the limit.
    #[cfg(feature = "_language")]
    #[cfg_attr(
        not(feature = "_hand-lexers"),
        expect(
            dead_code,
            reason = "only hand-written lexers use it, and the TextMate engine alone turns none on"
        )
    )]
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

    /// The number of `text`, which stays the same for equal text.
    /// It is `None` when the node limit keeps a new string from being made.
    pub(crate) fn intern(&mut self, text: &str) -> Option<u32> {
        if let Some(&id) = self.string_ids.get(text) {
            return Some(id);
        }
        let id = append(&mut self.strings, self.limit, Box::from(text))?;
        self.string_ids.insert(Box::from(text), id);
        Some(id)
    }

    /// The text of string `id`.
    /// It is empty for a number this arena never made.
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

    /// The stack `stack` without its top scope.
    /// It is the empty stack for an id this arena never made.
    #[cfg(feature = "_textmate")]
    pub(crate) fn parent(&self, stack: u32) -> u32 {
        usize::try_from(stack)
            .ok()
            .and_then(|index| self.scopes.get(index))
            .map_or(ROOT, |node| node.parent)
    }

    /// The outermost scope of `stack`, which names the document's grammar.
    /// It is `None` for the empty stack and for an id this arena never made.
    #[cfg(feature = "lang-html")]
    pub(crate) fn outermost(&self, stack: u32) -> Option<Scope> {
        if stack == ROOT {
            return None;
        }
        let mut node = *self.scopes.get(usize::try_from(stack).ok()?)?;
        while node.parent != ROOT {
            node = *self.scopes.get(usize::try_from(node.parent).ok()?)?;
        }
        Some(node.scope)
    }

    /// The name of `scope`, whether a table or this arena made it.
    #[cfg(feature = "lang-html")]
    #[inline]
    pub(crate) fn scope_name(&self, scope: Scope) -> &str {
        match scope.dynamic_index() {
            Some(index) => self.strings.get(index).map_or("", |text| text),
            None => scope.name(),
        }
    }

    /// How many stacks this arena made, the empty one included.
    /// Every stack id is below it.
    pub(crate) fn stack_count(&self) -> usize {
        self.scopes.len()
    }

    /// The scopes of `stack`, innermost first.
    /// They are empty for an id this arena never made.
    pub(crate) fn scopes(&self, stack: u32) -> Scopes<'_> {
        let valid = usize::try_from(stack).is_ok_and(|index| index < self.scopes.len());
        Scopes {
            arena: self,
            node: if valid { stack } else { ROOT },
        }
    }

    /// The frame of `state`.
    /// It is the document start for an id this arena never made.
    pub(crate) fn frame(&self, state: u32) -> Frame {
        usize::try_from(state)
            .ok()
            .and_then(|index| self.frames.get(index))
            .copied()
            .unwrap_or(self.frames[0])
    }

    /// The state `parent` with the context of `rule` opened on it, or `None`
    /// when the node limit keeps a new state from being made.
    #[inline(never)]
    pub(crate) fn push(&mut self, parent: u32, rule: u16, payload: u32, stack: u32) -> Option<u32> {
        let key = (parent, rule, payload, stack);
        if let Some(&id) = self.frame_ids.get(&key) {
            return Some(id);
        }
        self.new_frame(key)
    }

    /// A [`push`](Self::push) the arena does not hold yet: makes the state
    /// keyed `key`.
    ///
    /// Out of line for the reason [`new_child`](Self::new_child) is.
    #[inline(never)]
    fn new_frame(&mut self, key: (u32, u16, u32, u32)) -> Option<u32> {
        let frame = self.frame_on(key);
        let id = append(&mut self.frames, self.limit, frame)?;
        self.frame_ids.insert(key, id);
        Some(id)
    }

    /// A [`push`](Self::push) for a caller that expects to make the state,
    /// such as a lexer after its push cache misses.
    /// One hash of the key serves the lookup and the insert.
    #[cfg(feature = "_language")]
    #[inline(never)]
    #[cfg_attr(
        not(feature = "_hand-lexers"),
        expect(
            dead_code,
            reason = "only hand-written lexers use it, and the TextMate engine alone turns none on"
        )
    )]
    pub(crate) fn push_or_insert(
        &mut self,
        parent: u32,
        rule: u16,
        payload: u32,
        stack: u32,
    ) -> Option<u32> {
        let key = (parent, rule, payload, stack);
        let frame = self.frame_on(key);
        match self.frame_ids.entry(key) {
            Entry::Occupied(found) => Some(*found.get()),
            Entry::Vacant(slot) => {
                let id = append(&mut self.frames, self.limit, frame)?;
                Some(*slot.insert(id))
            }
        }
    }

    /// The state that the key's context makes on its parent.
    fn frame_on(&self, (parent, rule, payload, stack): (u32, u16, u32, u32)) -> Frame {
        Frame {
            parent,
            rule,
            payload,
            stack,
            depth: self.frame(parent).depth + 1,
        }
    }
}

/// Adds `node` to `nodes` and returns its number.
/// It is `None` when `limit` keeps `nodes` from growing.
fn append<T>(nodes: &mut Vec<T>, limit: Option<usize>, node: T) -> Option<u32> {
    if limit.is_some_and(|limit| nodes.len() >= limit) {
        return None;
    }
    let id = u32::try_from(nodes.len()).ok()?;
    nodes.push(node);
    Some(id)
}

/// The scope names of one stack, innermost first, as theme rules match them.
#[derive(Clone, Debug)]
pub struct Scopes<'a> {
    arena: &'a Arena,
    node: u32,
}

impl<'a> Iterator for Scopes<'a> {
    type Item = &'a str;

    /// Reads the name in place, not through `Arena::scope_name`.
    /// Measured at one codegen unit, the call made `Store::style_miss` run
    /// 75 more instructions a call, 0.29% of Ruby's colour pass.
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

    #[cfg(feature = "lang-html")]
    #[test]
    fn the_outermost_scope_is_the_first_one_pushed() {
        let mut arena = Arena::new(None);
        let document = arena.child(ROOT, Scope::new(1, 0)).unwrap();
        let deeper = arena.child(document, Scope::new(2, 5)).unwrap();
        assert_eq!(arena.outermost(deeper), Some(Scope::new(1, 0)));
        assert_eq!(arena.outermost(document), Some(Scope::new(1, 0)));
        assert_eq!(arena.outermost(ROOT), None);
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
