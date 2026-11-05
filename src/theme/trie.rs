//! A theme's rules, resolved into a trie on the dot-separated parts of each
//! rule's last scope, with the specificity order VS Code's `TextMate` engine
//! uses.
//!
//! Rules are inserted in sorted order (scope, then parent scopes, then file
//! order), so a rule for `string` is in place before `string.quoted` gets
//! its node, and a new node starts from a copy of its parent's rules: a
//! rule for `string` keeps applying under `string.quoted` until a more
//! specific one replaces what it set.

use std::cmp::Ordering;
use std::hash::Hasher;

use super::{Color, FontStyle, RuleScope, Style, ThemeRule};
use crate::arena::IdMap;
use crate::fnv::Fnv;

/// One resolved rule: its specificity and the attributes it sets.
#[derive(Clone, Debug)]
struct Rule {
    /// How many dot-separated parts of the scope the rule names.
    depth: u32,
    /// Selectors of enclosing scopes, innermost first; `>` makes the next
    /// one the direct parent.
    parents: Vec<String>,
    font_style: Option<FontStyle>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Rule {
    /// Takes the attributes `other` sets, at its depth.
    fn overwrite(&mut self, depth: u32, other: &Parsed) {
        self.depth = self.depth.max(depth);
        self.font_style = other.font_style.or(self.font_style);
        self.foreground = other.foreground.or(self.foreground);
        self.background = other.background.or(self.background);
    }

    /// Whether the enclosing scopes, innermost first, satisfy the rule's
    /// parent selectors.
    fn matches<'a>(&self, mut scopes: impl Iterator<Item = &'a str>) -> bool {
        let mut selectors = self.parents.iter();
        while let Some(selector) = selectors.next() {
            let (selector, direct) = if selector == ">" {
                match selectors.next() {
                    Some(next) => (next, true),
                    None => return false,
                }
            } else {
                (selector, false)
            };
            loop {
                let Some(scope) = scopes.next() else {
                    return false;
                };
                if scope_matches(scope, selector) {
                    break;
                }
                if direct {
                    return false;
                }
            }
        }
        true
    }

    /// VS Code's specificity order, most specific first: deeper scope
    /// match, then longer parent selectors pairwise, then more of them.
    fn specificity(a: &Rule, b: &Rule) -> Ordering {
        b.depth.cmp(&a.depth).then_with(|| {
            let without_combinators = |rule: &Rule| -> Vec<usize> {
                rule.parents
                    .iter()
                    .filter(|selector| *selector != ">")
                    .map(String::len)
                    .collect()
            };
            let (a_lengths, b_lengths) = (without_combinators(a), without_combinators(b));
            a_lengths
                .iter()
                .zip(&b_lengths)
                .map(|(a_len, b_len)| b_len.cmp(a_len))
                .find(|order| order.is_ne())
                .unwrap_or_else(|| b.parents.len().cmp(&a.parents.len()))
        })
    }
}

/// Whether `scope` is `selector` or extends it by dot-separated parts.
fn scope_matches(scope: &str, selector: &str) -> bool {
    let (scope, selector) = (scope.as_bytes(), selector.as_bytes());
    scope
        .get(..selector.len())
        .is_some_and(|head| same_bytes(head, selector))
        && scope.get(selector.len()).is_none_or(|&byte| byte == b'.')
}

/// Whether `a` and `b` hold the same bytes, compared in a loop.
///
/// Scope parts are a few bytes long.
/// libc's `memcmp`, which `==` on strings calls, costs more there.
/// Its count of instructions also follows where the strings lie, so styling
/// would move with code layout.
fn same_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (x, y) in a.iter().zip(b) {
        if x != y {
            return false;
        }
    }
    true
}

/// The key of the child of `parent` for `part`: the part's FNV-1a hash,
/// mixed with the parent's index.
/// Two children can share a key, so a lookup checks the parent and the part.
fn child_key(parent: usize, part: &str) -> u64 {
    let mut hasher = Fnv::default();
    hasher.write(part.as_bytes());
    let parent = u64::try_from(parent).unwrap_or(u64::MAX);
    hasher.finish() ^ parent.wrapping_mul(0x9e37_79b9_7f4a_7c15)
}

/// One node: its own rule, the rules that need parent scopes, and where it
/// sits under its parent.
#[derive(Clone, Debug)]
struct Node {
    main: Rule,
    with_parents: Vec<Rule>,
    /// `with_parents` and `main` in specificity order, filled once the trie
    /// is built.
    ordered: Vec<Rule>,
    /// The node this one is a child of; the root's is itself.
    parent: usize,
    /// The dot-separated part that leads here from the parent.
    part: Box<str>,
    /// The next node whose [`child_key`] is this one's.
    next: Option<usize>,
}

/// A rule as a theme file states it, one per selector.
struct Parsed {
    scope: String,
    /// `None` when the selector names no enclosing scope.
    parents: Option<Vec<String>>,
    index: usize,
    font_style: Option<FontStyle>,
    foreground: Option<Color>,
    background: Option<Color>,
}

/// The selectors a rule's scope names, after VS Code: a string is split at
/// commas once leading and trailing commas are dropped; a missing scope is
/// the empty selector.
fn selectors(scope: &RuleScope) -> Vec<String> {
    match scope {
        RuleScope::Defaults => vec![String::new()],
        RuleScope::Selectors(text) => text
            .trim_matches(',')
            .split(',')
            .map(ToOwned::to_owned)
            .collect(),
        RuleScope::List(list) => list.clone(),
    }
}

fn parse(rules: &[ThemeRule]) -> Vec<Parsed> {
    let mut parsed = Vec::new();
    for (index, rule) in rules.iter().enumerate() {
        let font_style = rule.font_style.as_deref().map(FontStyle::parse);
        let foreground = rule.foreground.as_deref().and_then(Color::from_hex);
        let background = rule.background.as_deref().and_then(Color::from_hex);
        for selector in selectors(&rule.scope) {
            let mut parts: Vec<String> =
                selector.trim().split(' ').map(ToOwned::to_owned).collect();
            let scope = parts.pop().unwrap_or_default();
            let parents = (!parts.is_empty()).then(|| {
                parts.reverse();
                parts
            });
            parsed.push(Parsed {
                scope,
                parents,
                index,
                font_style,
                foreground,
                background,
            });
        }
    }
    parsed.sort_by(|a, b| {
        a.scope
            .cmp(&b.scope)
            .then_with(|| match (&a.parents, &b.parents) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(a), Some(b)) => a.len().cmp(&b.len()).then_with(|| a.cmp(b)),
            })
            .then(a.index.cmp(&b.index))
    });
    parsed
}

/// The resolved rules of a theme.
#[derive(Clone, Debug)]
pub(super) struct Trie {
    nodes: Vec<Node>,
    /// The first node of each [`child_key`].
    /// The key is an integer, so a lookup compares no strings in libc.
    children: IdMap<u64, usize>,
}

impl Trie {
    /// The theme's default style and its trie, from rules whose first entry
    /// sets the defaults.
    pub(super) fn build(rules: &[ThemeRule]) -> (Style, Trie) {
        let mut parsed = parse(rules).into_iter().peekable();
        let mut defaults = Style {
            foreground: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            background: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            font: FontStyle::NONE,
        };
        while let Some(rule) = parsed.next_if(|rule| rule.scope.is_empty()) {
            defaults.font = rule.font_style.unwrap_or(defaults.font);
            defaults.foreground = rule.foreground.unwrap_or(defaults.foreground);
            defaults.background = rule.background.unwrap_or(defaults.background);
        }
        let empty = Rule {
            depth: 0,
            parents: Vec::new(),
            font_style: None,
            foreground: None,
            background: None,
        };
        let mut trie = Trie {
            nodes: vec![Node {
                main: empty,
                with_parents: Vec::new(),
                ordered: Vec::new(),
                parent: 0,
                part: Box::default(),
                next: None,
            }],
            children: IdMap::default(),
        };
        for rule in parsed {
            trie.insert(&rule);
        }
        for node in &mut trie.nodes {
            let mut ordered = node.with_parents.clone();
            ordered.push(node.main.clone());
            ordered.sort_by(Rule::specificity);
            node.ordered = ordered;
        }
        (defaults, trie)
    }

    fn insert(&mut self, rule: &Parsed) {
        let mut node = 0;
        let mut depth = 0;
        if !rule.scope.is_empty() {
            for part in rule.scope.split('.') {
                depth += 1;
                node = self.child(node, part);
            }
        }
        let target = &mut self.nodes[node];
        let Some(parents) = &rule.parents else {
            target.main.overwrite(depth, rule);
            return;
        };
        if let Some(existing) = target
            .with_parents
            .iter_mut()
            .find(|r| &r.parents == parents)
        {
            existing.overwrite(depth, rule);
            return;
        }
        target.with_parents.push(Rule {
            depth,
            parents: parents.clone(),
            font_style: rule.font_style.or(target.main.font_style),
            foreground: rule.foreground.or(target.main.foreground),
            background: rule.background.or(target.main.background),
        });
    }

    /// The child of `node` for `part`, created from a copy of `node`'s rules
    /// when it does not exist yet.
    fn child(&mut self, node: usize, part: &str) -> usize {
        if let Some(child) = self.find(node, part) {
            return child;
        }
        let key = child_key(node, part);
        let child = self.nodes.len();
        let next = self.children.insert(key, child);
        let parent = &self.nodes[node];
        let copy = Node {
            main: parent.main.clone(),
            with_parents: parent.with_parents.clone(),
            ordered: Vec::new(),
            parent: node,
            part: part.into(),
            next,
        };
        self.nodes.push(copy);
        child
    }

    /// The child of `node` for `part`, if the trie has one.
    /// It follows the nodes that share the key, and checks the parent and
    /// the part byte by byte.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: once Highlighter::style instantiated Theme::style in the crate, this stopped being inlined into apply, and colouring ran 4% more instructions"
    )]
    #[inline(always)]
    fn find(&self, node: usize, part: &str) -> Option<usize> {
        let mut at = self.children.get(&child_key(node, part)).copied();
        while let Some(index) = at {
            let candidate = self.nodes.get(index)?;
            if candidate.parent == node && same_bytes(candidate.part.as_bytes(), part.as_bytes()) {
                return Some(index);
            }
            at = candidate.next;
        }
        None
    }

    /// Applies the rule for the innermost scope of `path`, outermost first,
    /// to `style`: the attributes the rule sets replace those in `style`.
    ///
    /// It walks the scope's dot-separated parts with a byte loop.
    /// `str::split` searches with `memchr`, whose call cost more than the few
    /// bytes of a part.
    pub(super) fn apply(&self, path: &[&str], style: &mut Style) {
        let Some((scope, outer)) = path.split_last() else {
            return;
        };
        let mut index = 0;
        let mut rest = *scope;
        loop {
            let end = rest
                .bytes()
                .position(|byte| byte == b'.')
                .unwrap_or(rest.len());
            match rest.get(..end).and_then(|part| self.find(index, part)) {
                Some(child) => index = child,
                None => break,
            }
            match rest.get(end + 1..) {
                Some(next) => rest = next,
                None => break,
            }
        }
        if let Some(rule) = self.nodes[index]
            .ordered
            .iter()
            .find(|rule| rule.matches(outer.iter().rev().copied()))
        {
            style.font = rule.font_style.unwrap_or(style.font);
            style.foreground = rule.foreground.unwrap_or(style.foreground);
            style.background = rule.background.unwrap_or(style.background);
        }
    }
}
