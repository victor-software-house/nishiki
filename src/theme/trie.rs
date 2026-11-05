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

use super::{Color, FontStyle, RuleScope, Style, ThemeRule};
use crate::fnv::FnvMap;

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
    scope
        .strip_prefix(selector)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}

/// One node: its own rule, the rules that need parent scopes, and its
/// children by the next dot-separated part.
#[derive(Clone, Debug)]
struct Node {
    main: Rule,
    with_parents: Vec<Rule>,
    children: FnvMap<String, usize>,
    /// `with_parents` and `main` in specificity order, filled once the trie
    /// is built.
    ordered: Vec<Rule>,
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
                children: FnvMap::default(),
                ordered: Vec::new(),
            }],
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
        if let Some(&child) = self.nodes[node].children.get(part) {
            return child;
        }
        let parent = &self.nodes[node];
        let copy = Node {
            main: parent.main.clone(),
            with_parents: parent.with_parents.clone(),
            children: FnvMap::default(),
            ordered: Vec::new(),
        };
        let child = self.nodes.len();
        self.nodes.push(copy);
        self.nodes[node].children.insert(part.to_owned(), child);
        child
    }

    /// Applies the rule for the innermost scope of `path`, outermost first,
    /// to `style`: the attributes the rule sets replace those in `style`.
    pub(super) fn apply(&self, path: &[&str], style: &mut Style) {
        let Some((scope, outer)) = path.split_last() else {
            return;
        };
        let mut node = &self.nodes[0];
        for part in scope.split('.') {
            match node.children.get(part) {
                Some(&child) => node = &self.nodes[child],
                None => break,
            }
        }
        if let Some(rule) = node
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
