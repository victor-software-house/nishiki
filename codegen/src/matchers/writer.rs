//! Writes the matcher of one pattern from its parsed tree.
//!
//! The code passes the match on in continuation style.
//! Each node's code ends by running what follows it, from `pos` and `caps`.
//! `caps` holds two slots per recorded group: its start and its end.

use std::collections::HashSet;

use nishiki::unstable::regex::{Anchor, Case, Class, Dialect, Greediness, Node, parse};
use proc_macro2::{Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};

use super::chars::Chars;
use super::shape::{
    Follow, ScanSkip, WordTest, back_reference_groups, drop_covered_arms, first_byte, first_of,
    group_body, keep_read_groups, leading_anchor, one_character_set, scan_skip, unique, word_test,
};

/// Why a pattern has no generated matcher.
#[derive(Debug)]
pub(super) struct Refusal(pub(super) String);

/// The fewest distinct first bytes for which an alternation dispatches on
/// the byte at its start.
/// With fewer, its arms fail at their first byte as fast as the dispatch.
const DISPATCH_BYTES: usize = 3;

/// The most tokens the body of a matcher with no loop and no lookbehind may
/// hold and still be offered to the language's `matcher` inline.
///
/// - One read of a byte takes about 40 tokens. So the bound covers a test of
///   one or two literal bytes, such as `//`.
/// - Every other matcher stays out of line. Its registers then never widen the
///   matcher's frame.
/// - A lookbehind's code gave Lua's `matcher` a frame when inlined.
const SMALL_BODY: usize = 100;

/// The most runs a lookbehind may spell out.
const LOOKBEHIND_RUNS: usize = 64;

/// The test on one character that a node takes.
#[derive(Clone)]
enum Predicate {
    /// The character `c`.
    Is(char),
    /// Any character.
    Any,
    /// Any character but a line feed.
    NotNewline,
    /// A character the module-level function of this name takes.
    Class(Ident),
}

impl Predicate {
    /// The pattern of a `char_at` read that this test takes.
    /// A test that no pattern spells adds a guard.
    /// `width` names the bound width, or `_` for none.
    fn arm(&self, width: &TokenStream) -> TokenStream {
        match self {
            Predicate::Is(c) => {
                let c = Literal::character(*c);
                quote!(Some((#c, #width)))
            }
            Predicate::Any => quote!(Some((_, #width))),
            Predicate::NotNewline => quote!(Some((character, #width)) if character != '\n'),
            Predicate::Class(name) => quote!(Some((character, #width)) if #name(character)),
        }
    }
}

/// How the code reads one character at a position.
struct OneCharacter {
    /// The match arm's pattern, with its test on what was read.
    arm: TokenStream,
    /// The call that reads at `pos`: `byte_at` or `char_at`.
    read: Ident,
    /// The bytes the character takes: `1` or `w`.
    width: TokenStream,
    /// Whether every character the node takes is ASCII, so one byte wide.
    ascii: bool,
}

/// What a node passes the match on to.
#[derive(Clone)]
enum Next {
    /// Code that reads `pos` and `caps` and evaluates to the result.
    Inline(TokenStream),
    /// A closure that takes `pos` and `caps`.
    Call(Ident),
}

impl Next {
    /// The code that runs the rest of the match from `pos` and `caps`.
    fn code(&self) -> TokenStream {
        match self {
            Next::Inline(code) => code.clone(),
            Next::Call(name) => quote!(#name(pos, caps)),
        }
    }
}

/// How a pattern's code takes its repeats of whole groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Path {
    /// No such repeat: the code backtracks as the regex engine does.
    Exact,
    /// Each turn keeps its first match, which the analysis proves exact.
    Loop,
    /// Turns are given back from an explicit stack.
    Stack,
}

/// A class function: a test on one character, written at module level.
pub(super) struct ClassFunction {
    /// The function's name.
    pub(super) name: Ident,
    /// The characters it takes.
    pub(super) chars: Chars,
}

/// The writer of one pattern's matcher.
struct Writer {
    /// The width of the groups array: two slots per group, the match included.
    width: usize,
    /// The counter that names locals, closures and class functions.
    fresh: usize,
    /// Class functions, written at module level.
    classes: Vec<ClassFunction>,
    /// The pattern's function, which names its class functions.
    function: Ident,
    /// The most costly way the pattern takes a repeat of a whole group.
    path: Path,
    /// Whether the pattern holds a lookbehind.
    lookbehind: bool,
    /// The groups a back-reference may read: each takes exactly one
    /// character.
    one_character: Vec<usize>,
    /// The groups around the node being written.
    open: Vec<usize>,
}

/// A whole number as an unsuffixed literal.
fn number(n: usize) -> Literal {
    Literal::usize_unsuffixed(n)
}

/// `value` in hexadecimal, padded to `digits` digits, with an underscore
/// between each group of four from the right, and `suffix` after it.
pub(super) fn hex(value: u128, digits: usize, suffix: &str) -> Literal {
    let text = format!("{value:0digits$x}");
    let mut grouped = String::new();
    for (i, digit) in text.chars().enumerate() {
        if i > 0 && (text.len() - i) % 4 == 0 {
            grouped.push('_');
        }
        grouped.push(digit);
    }
    syn::LitInt::new(&format!("0x{grouped}{suffix}"), Span::call_site()).token()
}

/// The turn counter of a lazy repeat, and the statement that counts a turn.
/// A repeat with no least and no most turns reads no count, so it has none.
fn counter(min: u32, max: Option<u32>) -> (TokenStream, TokenStream) {
    if min == 0 && max.is_none() {
        (TokenStream::new(), TokenStream::new())
    } else {
        (quote!(let mut count = 0u32;), quote!(count += 1;))
    }
}

/// The code of a repeat's bound: `u32::MAX` when it has none.
fn bound(max: Option<u32>) -> TokenStream {
    max.map_or_else(
        || quote!(u32::MAX),
        |max| {
            let max = Literal::u32_unsuffixed(max);
            quote!(#max)
        },
    )
}

/// Why a pattern with a text anchor, such as `\A`, has no matcher.
///
/// The lexer reads one line, and holds no mark of the first line.
/// The regex engine matches `\A` only at the start of the first line.
/// A grammar's `\z` parses as a lookahead, not as a text anchor.
fn text_anchor() -> Refusal {
    Refusal("a text anchor, such as `\\A`, which the line's lexer cannot test".to_owned())
}

/// The test of `anchor` at `pos`, on the lexer `lx`.
///
/// # Errors
///
/// [`text_anchor`] for the text start and end anchors.
pub(super) fn anchor_test(anchor: Anchor, lx: &Ident, pos: &Ident) -> Result<TokenStream, Refusal> {
    Ok(match anchor {
        Anchor::LineStart => quote!(#pos == 0),
        Anchor::LineEnd | Anchor::TextEndOrFinalNewline => {
            quote!(#pos == #lx.s.len() || (#lx.nl && #pos == #lx.s.len() + 1))
        }
        Anchor::TextStart | Anchor::TextEnd => return Err(text_anchor()),
        Anchor::LastMatchEnd => quote!(#lx.g && #pos == #lx.pos),
        Anchor::WordBoundary => quote!(#lx.boundary(#pos)),
        Anchor::NotWordBoundary => quote!(!#lx.boundary(#pos)),
    })
}

/// The name of the placeholder macro that marks where code leaves its
/// context with a value.
const EXIT: &str = "exit_with";

/// The code that leaves the current context with `value`.
///
/// It stays a placeholder until [`resolve`] reads the context.
/// A function or a closure returns the value; a labelled block breaks with it.
fn exit(value: &TokenStream) -> TokenStream {
    let name = format_ident!("{EXIT}");
    quote!(#name!(#value))
}

/// The code that leaves the current context with no match.
fn fail() -> TokenStream {
    exit(&quote!(None))
}

/// The name of the placeholder macro that binds what an option holds, or
/// leaves the context with no match when it holds nothing.
const EXIT_UNLESS: &str = "exit_unless";

/// The statement that binds `pattern` to what `option` holds, or leaves the
/// current context with no match.
///
/// It stays a placeholder until [`resolve`] reads the context.
/// A function or a closure uses `?`; a labelled block breaks.
fn bind_or_fail(pattern: &TokenStream, option: &Ident) -> TokenStream {
    let name = format_ident!("{EXIT_UNLESS}");
    quote!(#name!(#pattern, #option);)
}

/// The resolved form of an [`bind_or_fail`] placeholder whose arguments are
/// `arguments`: a pattern group, a comma, then the option.
fn resolve_bind(arguments: TokenStream, label: Option<&TokenStream>) -> TokenStream {
    let mut trees = arguments.into_iter();
    let pattern = trees.next();
    let option = trees.skip(1).collect::<TokenStream>();
    label.map_or_else(
        || quote!(let #pattern = #option?),
        |label| quote!(let Some(#pattern) = #option else { break #label None; }),
    )
}

/// `code` with each exit placeholder made a `return`, or a `break` to `label`.
///
/// Every context inside `code` resolved its own exits when it was written.
/// So each placeholder left in `code` leaves `code`'s own context.
fn resolve(code: TokenStream, label: Option<&TokenStream>) -> TokenStream {
    let mut out = TokenStream::new();
    let mut trees = code.into_iter();
    while let Some(tree) = trees.next() {
        match tree {
            TokenTree::Ident(ident) if ident == EXIT => {
                let (_, value) = (trees.next(), trees.next());
                if let Some(TokenTree::Group(value)) = value {
                    let value = value.stream();
                    out.extend(label.map_or_else(
                        || quote!(return #value),
                        |label| quote!(break #label #value),
                    ));
                }
            }
            TokenTree::Ident(ident) if ident == EXIT_UNLESS => {
                let (_, arguments) = (trees.next(), trees.next());
                if let Some(TokenTree::Group(arguments)) = arguments {
                    out.extend(resolve_bind(arguments.stream(), label));
                }
            }
            TokenTree::Group(group) => {
                let mut resolved = Group::new(group.delimiter(), resolve(group.stream(), label));
                resolved.set_span(group.span());
                out.extend([TokenTree::Group(resolved)]);
            }
            TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => {
                out.extend([tree]);
            }
        }
    }
    out
}

/// Whether `code` holds an exit placeholder.
fn has_exit(code: &TokenStream) -> bool {
    code.clone().into_iter().any(|tree| match tree {
        TokenTree::Ident(ident) => ident == EXIT || ident == EXIT_UNLESS,
        TokenTree::Group(group) => has_exit(&group.stream()),
        TokenTree::Punct(_) | TokenTree::Literal(_) => false,
    })
}

/// Whether `code` holds a statement outside its groups: a `;`.
fn has_statements(code: &TokenStream) -> bool {
    code.clone()
        .into_iter()
        .any(|tree| matches!(tree, TokenTree::Punct(punct) if punct.as_char() == ';'))
}

/// A block label made from `name`, such as `'behind3`.
fn label(name: &Ident) -> TokenStream {
    let lifetime = Punct::new('\'', Spacing::Joint);
    quote!(#lifetime #name)
}

/// The test that `anchor` does not hold at `pos`.
///
/// # Errors
///
/// [`text_anchor`] for the text start and end anchors.
fn anchor_fails(anchor: Anchor) -> Result<TokenStream, Refusal> {
    Ok(match anchor {
        Anchor::LineStart => quote!(pos != 0),
        Anchor::LineEnd | Anchor::TextEndOrFinalNewline => {
            quote!(pos != lx.s.len() && (!lx.nl || pos != lx.s.len() + 1))
        }
        Anchor::TextStart | Anchor::TextEnd => return Err(text_anchor()),
        Anchor::LastMatchEnd => quote!(!lx.g || pos != lx.pos),
        Anchor::WordBoundary => quote!(!lx.boundary(pos)),
        Anchor::NotWordBoundary => quote!(lx.boundary(pos)),
    })
}

/// The pattern of the bytes an ASCII mask takes, such as `b'0'..=b'9' | b'_'`.
/// An empty mask takes no byte, so it has none.
fn byte_pattern(ascii: u128) -> Option<TokenStream> {
    let mut runs: Vec<(u8, u8)> = Vec::new();
    for code in 0..0x80_u8 {
        if (ascii >> code) & 1 == 1 {
            match runs.last_mut() {
                Some(run) if run.1 + 1 == code => run.1 = code,
                _ => runs.push((code, code)),
            }
        }
    }
    let arms = runs.iter().map(|&(first, last)| {
        let low = Literal::byte_character(first);
        if first == last {
            quote!(#low)
        } else {
            let high = Literal::byte_character(last);
            quote!(#low..=#high)
        }
    });
    (!runs.is_empty()).then(|| quote!(#(#arms)|*))
}

impl Writer {
    /// A fresh name that starts with `stem`.
    fn name(&mut self, stem: &str) -> Ident {
        self.fresh += 1;
        format_ident!("{stem}{}", self.fresh)
    }

    /// The type of the result a matcher's code gives.
    fn found(&self) -> TokenStream {
        let width = number(self.width);
        quote!(Found<#width>)
    }

    /// The type of the groups array.
    fn caps(&self) -> TokenStream {
        let width = number(self.width);
        quote!([usize; #width])
    }

    /// The continuation as a closure, when it is code that would be copied,
    /// and the closure's name.
    fn bind(&mut self, next: &Next) -> (TokenStream, Ident) {
        match next {
            Next::Call(name) => (TokenStream::new(), name.clone()),
            Next::Inline(code) => {
                let name = self.name("rest");
                let (caps, found) = (self.caps(), self.found());
                let code = resolve(code.clone(), None);
                (
                    quote!(let #name = |pos: usize, caps: #caps| -> #found { #code };),
                    name,
                )
            }
        }
    }

    /// The test on one character, for a node that takes exactly one.
    fn predicate(&mut self, node: &Node) -> Option<Predicate> {
        match node {
            Node::Char(c) => Some(Predicate::Is(*c)),
            Node::Any { newline: true } => Some(Predicate::Any),
            Node::Any { newline: false } => Some(Predicate::NotNewline),
            Node::Class(class) => Some(Predicate::Class(self.class(class))),
            Node::Group { index: None, node } => self.predicate(node),
            Node::Empty
            | Node::Sequence(_)
            | Node::Alternation(_)
            | Node::Folds(_)
            | Node::Repeat { .. }
            | Node::Group { .. }
            | Node::Lookaround { .. }
            | Node::Atomic(_)
            | Node::BackReference(..)
            | Node::NamedBackReference(..)
            | Node::Subroutine(_)
            | Node::NamedSubroutine(_)
            | Node::Anchor(_) => None,
        }
    }

    /// The ASCII mask of a node that takes exactly one character and only
    /// ASCII ones.
    /// A byte from 0x80 up never matches such a node, and starts no ASCII
    /// character, so the byte decides alone.
    fn ascii_only(node: &Node) -> Option<u128> {
        match node {
            Node::Char(c) if c.is_ascii() => Some(1 << u32::from(*c)),
            Node::Class(class) => {
                let chars = Chars::of_class(class);
                chars.wide.is_empty().then_some(chars.ascii)
            }
            Node::Group { index: None, node } => Self::ascii_only(node),
            Node::Empty
            | Node::Char(_)
            | Node::Any { .. }
            | Node::Sequence(_)
            | Node::Alternation(_)
            | Node::Folds(_)
            | Node::Repeat { .. }
            | Node::Group { .. }
            | Node::Lookaround { .. }
            | Node::Atomic(_)
            | Node::BackReference(..)
            | Node::NamedBackReference(..)
            | Node::Subroutine(_)
            | Node::NamedSubroutine(_)
            | Node::Anchor(_) => None,
        }
    }

    /// How one character of `node` is read at a position.
    /// A node that takes some ASCII characters and no other reads a byte.
    /// Any other decodes a character.
    fn one(&mut self, node: &Node) -> Option<OneCharacter> {
        if let Some(bytes) = Self::ascii_only(node).and_then(byte_pattern) {
            return Some(OneCharacter {
                arm: quote!(Some(#bytes)),
                read: format_ident!("byte_at"),
                width: quote!(1),
                ascii: true,
            });
        }
        let predicate = self.predicate(node)?;
        Some(OneCharacter {
            arm: predicate.arm(&quote!(width)),
            read: format_ident!("char_at"),
            width: quote!(width),
            ascii: false,
        })
    }

    /// A class as a module-level function, and that function's name.
    fn class(&mut self, class: &Class) -> Ident {
        let name = self.name("class");
        let name = format_ident!("{}_{name}", self.function);
        self.classes.push(ClassFunction {
            name: name.clone(),
            chars: Chars::of_class(class),
        });
        name
    }

    /// The code that matches `node` at `pos` and then runs `next`.
    /// `follow` describes `next` for the analysis.
    ///
    /// The code is a run of statements and a last expression, flat in the
    /// block it lands in.
    /// Where the node fails, the code leaves its context with no match
    /// ([`fail`]).
    fn node(&mut self, node: &Node, next: &Next, follow: &Follow) -> Result<TokenStream, Refusal> {
        if let Some(OneCharacter {
            arm, read, width, ..
        }) = self.one(node)
        {
            let (code, fail) = (next.code(), fail());
            return Ok(quote! {
                let pos = match #read(lx, pos) {
                    #arm => pos + #width,
                    Some(_) | None => #fail,
                };
                #code
            });
        }
        match node {
            Node::Empty => Ok(next.code()),
            Node::Sequence(nodes) => self.sequence(nodes, next, follow),
            Node::Alternation(nodes) => self.alternation(nodes, true, next, follow),
            Node::Folds(nodes) => self.alternation(nodes, false, next, follow),
            Node::Group { index: None, node } => self.node(node, next, follow),
            Node::Group {
                index: Some(group),
                node,
            } => self.group(*group, node, next, follow),
            Node::Repeat {
                node,
                min,
                max,
                greediness,
            } => self.repeat(node, *min, *max, *greediness, next, follow),
            Node::Lookaround {
                ahead: true,
                negative,
                node,
            } => {
                let inner = self.first_match(node)?;
                let looked = self.attempt(inner);
                let (code, fail) = (next.code(), fail());
                Ok(if *negative {
                    quote! {
                        let looked = #looked;
                        if looked.is_some() {
                            #fail;
                        }
                        #code
                    }
                } else {
                    let bind = bind_or_fail(&quote!((_, caps)), &format_ident!("looked"));
                    quote! {
                        let looked = #looked;
                        #bind
                        #code
                    }
                })
            }
            Node::Lookaround {
                ahead: false,
                negative,
                node,
            } => self.lookbehind(node, *negative, next),
            Node::Atomic(node) => {
                let inner = self.first_match(node)?;
                let looked = self.attempt(inner);
                let code = next.code();
                let bind = bind_or_fail(&quote!((pos, caps)), &format_ident!("looked"));
                Ok(quote! {
                    let looked = #looked;
                    #bind
                    #code
                })
            }
            Node::Anchor(anchor) => {
                let fails = anchor_fails(*anchor)?;
                let (code, fail) = (next.code(), fail());
                Ok(quote! {
                    if #fails {
                        #fail;
                    }
                    #code
                })
            }
            Node::BackReference(group, Case::Sensitive)
                if self.one_character.contains(group) && !self.open.contains(group) =>
            {
                Ok(self.back_reference(*group, next))
            }
            Node::BackReference(..) | Node::NamedBackReference(..) => {
                Err(Refusal("a back-reference".to_owned()))
            }
            Node::Subroutine(_) | Node::NamedSubroutine(_) => {
                Err(Refusal("a subexpression call".to_owned()))
            }
            Node::Char(_) | Node::Any { .. } | Node::Class(_) => {
                Err(Refusal("a one-character node with no read".to_owned()))
            }
        }
    }

    /// `code` as an expression whose exits stay inside it.
    ///
    /// Code that can fail runs in a labelled block, and fails out of that
    /// block.
    /// Code with no exit stays as it is, in braces when it holds statements.
    fn attempt(&mut self, code: TokenStream) -> TokenStream {
        if has_exit(&code) {
            let label = label(&self.name("attempt"));
            let code = resolve(code, Some(&label));
            quote!(#label: { #code })
        } else if has_statements(&code) {
            quote!({ #code })
        } else {
            code
        }
    }

    /// The code that tries `arms` in order and keeps the first that matches.
    ///
    /// Each arm but the last runs as an [`attempt`](Writer::attempt).
    /// When it matches, the code leaves its context with the arm's match.
    /// The last arm runs in the context itself.
    fn try_in_order(&mut self, arms: Vec<TokenStream>) -> TokenStream {
        let mut arms = arms;
        let Some(last) = arms.pop() else {
            return fail();
        };
        let mut code = TokenStream::new();
        let leave = exit(&quote!(found));
        for arm in arms {
            let attempt = self.attempt(arm);
            code.extend(quote! {
                let found = #attempt;
                if found.is_some() {
                    #leave;
                }
            });
        }
        code.extend(last);
        code
    }

    /// A sequence, written from its last node back.
    /// A word boundary with one side settled tests only the other side.
    fn sequence(
        &mut self,
        nodes: &[Node],
        next: &Next,
        follow: &Follow,
    ) -> Result<TokenStream, Refusal> {
        let mut code = next.clone();
        for (i, node) in nodes.iter().enumerate().rev() {
            let after = follow.after(nodes.get(i + 1..).unwrap_or(&[]));
            let one_sided = match node {
                Node::Anchor(anchor @ (Anchor::WordBoundary | Anchor::NotWordBoundary)) => {
                    word_test(
                        *anchor == Anchor::WordBoundary,
                        nodes.get(..i).unwrap_or(&[]),
                        &after,
                    )
                }
                Node::Anchor(_)
                | Node::Empty
                | Node::Char(_)
                | Node::Any { .. }
                | Node::Class(_)
                | Node::Sequence(_)
                | Node::Alternation(_)
                | Node::Folds(_)
                | Node::Repeat { .. }
                | Node::Group { .. }
                | Node::Lookaround { .. }
                | Node::Atomic(_)
                | Node::BackReference(..)
                | Node::NamedBackReference(..)
                | Node::Subroutine(_)
                | Node::NamedSubroutine(_) => None,
            };
            code = Next::Inline(match one_sided {
                Some(test) => {
                    let fails = match test {
                        WordTest::NoWordAt => quote!(lx.word_at(pos).is_some()),
                        WordTest::WordAt => quote!(lx.word_at(pos).is_none()),
                        WordTest::NoWordBefore => quote!(lx.word_before(pos)),
                        WordTest::WordBefore => quote!(!lx.word_before(pos)),
                    };
                    let (rest, fail) = (code.code(), fail());
                    quote! {
                        if #fails {
                            #fail;
                        }
                        #rest
                    }
                }
                None => self.node(node, &code, &after)?,
            });
        }
        Ok(code.code())
    }

    /// An alternation, or a set of folds, with its arms tried in order.
    /// An alternation whose arms each start with a literal byte reads that
    /// byte once and tries only the arms that start with it.
    fn alternation(
        &mut self,
        nodes: &[Node],
        dispatch: bool,
        next: &Next,
        follow: &Follow,
    ) -> Result<TokenStream, Refusal> {
        let (prelude, next) = self.bind(next);
        let next = Next::Call(next);
        let mut arms = Vec::new();
        for node in nodes {
            arms.push(self.node(node, &next, follow)?);
        }
        let firsts = if dispatch {
            nodes.iter().map(first_byte).collect::<Option<Vec<_>>>()
        } else {
            None
        };
        let code = match firsts {
            Some(firsts) if firsts.iter().collect::<HashSet<_>>().len() >= DISPATCH_BYTES => {
                let mut buckets: Vec<(u8, Vec<TokenStream>)> = Vec::new();
                for (byte, arm) in firsts.into_iter().zip(arms) {
                    match buckets.iter_mut().find(|(first, _)| *first == byte) {
                        Some((_, bucket)) => bucket.push(arm),
                        None => buckets.push((byte, vec![arm])),
                    }
                }
                let mut cases = Vec::new();
                for (byte, bucket) in buckets {
                    let byte = Literal::byte_character(byte);
                    let tries = self.try_in_order(bucket);
                    cases.push(quote!(Some(#byte) => { #tries }));
                }
                let fail = fail();
                quote! {
                    match byte_at(lx, pos) {
                        #(#cases)*
                        Some(_) | None => #fail,
                    }
                }
            }
            _ => self.try_in_order(arms),
        };
        Ok(quote!(#prelude #code))
    }

    /// A capturing group: it notes its start, and its end before `next`.
    fn group(
        &mut self,
        group: usize,
        node: &Node,
        next: &Next,
        follow: &Follow,
    ) -> Result<TokenStream, Refusal> {
        let start = self.name("group_start");
        let (low, high) = (number(2 * group), number(2 * group + 1));
        let code = next.code();
        let after = Next::Inline(quote! {
            let caps = {
                let mut caps = caps;
                caps[#low] = #start;
                caps[#high] = pos;
                caps
            };
            #code
        });
        self.open.push(group);
        let inner = self.node(node, &after, follow);
        self.open.pop();
        let inner = inner?;
        Ok(quote! {
            let #start = pos;
            #inner
        })
    }

    /// A back-reference to a group that took exactly one character.
    fn back_reference(&mut self, group: usize, next: &Next) -> TokenStream {
        let start = self.name("taken_at");
        let low = number(2 * group);
        let (code, fail) = (next.code(), fail());
        quote! {
            let #start = caps[#low];
            if #start == usize::MAX {
                #fail;
            }
            let pos = match (char_at(lx, #start), char_at(lx, pos)) {
                (Some((taken, width)), Some((here, _))) if taken == here => pos + width,
                _ => #fail,
            };
            #code
        }
    }

    /// A lookbehind of fixed runs, each read backwards from the position.
    fn lookbehind(
        &mut self,
        node: &Node,
        negative: bool,
        next: &Next,
    ) -> Result<TokenStream, Refusal> {
        self.lookbehind = true;
        let alternatives = self.behind(node)?;
        let mut tests = Vec::new();
        for predicates in &alternatives {
            let label = label(&self.name("behind"));
            let steps = predicates.iter().rev().map(|predicate| {
                let arm = predicate.arm(&quote!(_));
                quote! {
                    if back == 0 {
                        break #label false;
                    }
                    let before = prev_boundary(lx, back);
                    match char_at(lx, before) {
                        #arm => back = before,
                        Some(_) | None => break #label false,
                    }
                }
            });
            tests.push(quote!(#label: { let mut back = pos; #(#steps)* true }));
        }
        let (code, fail) = (next.code(), fail());
        let fails = if negative {
            quote!(behind)
        } else {
            quote!(!behind)
        };
        Ok(quote! {
            let behind = #(#tests)||*;
            if #fails {
                #fail;
            }
            #code
        })
    }

    /// The alternatives of a lookbehind, each a fixed run of one-character
    /// tests.
    fn behind(&mut self, node: &Node) -> Result<Vec<Vec<Predicate>>, Refusal> {
        if let Some(predicate) = self.predicate(node) {
            return Ok(vec![vec![predicate]]);
        }
        match node {
            Node::Empty => Ok(vec![Vec::new()]),
            Node::Alternation(nodes) | Node::Folds(nodes) => {
                let mut all = Vec::new();
                for node in nodes {
                    all.extend(self.behind(node)?);
                }
                Ok(all)
            }
            Node::Sequence(nodes) => {
                let mut runs = vec![Vec::new()];
                for node in nodes {
                    runs = joined(&runs, &self.behind(node)?)?;
                }
                Ok(runs)
            }
            Node::Group { node, .. } => self.behind(node),
            Node::Repeat {
                node,
                min,
                max: Some(max),
                ..
            } if min == max => {
                let one = self.behind(node)?;
                let mut runs = vec![Vec::new()];
                for _ in 0..*min {
                    runs = joined(&runs, &one)?;
                }
                Ok(runs)
            }
            Node::Char(_)
            | Node::Any { .. }
            | Node::Class(_)
            | Node::Repeat { .. }
            | Node::Lookaround { .. }
            | Node::Atomic(_)
            | Node::BackReference(..)
            | Node::NamedBackReference(..)
            | Node::Subroutine(_)
            | Node::NamedSubroutine(_)
            | Node::Anchor(_) => Err(Refusal("a lookbehind of varying width".to_owned())),
        }
    }

    /// The code that gives the first match of `node`, as a lookaround or an
    /// atomic group keeps it.
    fn first_match(&mut self, node: &Node) -> Result<TokenStream, Refusal> {
        self.node(
            node,
            &Next::Inline(quote!(Some((pos, caps)))),
            &Follow::always(),
        )
    }

    /// A repeat, written without recursion per turn.
    ///
    /// - One character a turn loops ([`Writer::repeat_one`]).
    /// - An optional node tries it, then what follows.
    /// - A whole group a turn loops over a closure ([`Writer::repeat_group`]).
    ///
    /// A repeated group with a least of one or more whose turn can match empty
    /// has no generated matcher.
    /// The regex engine counts an empty turn toward the least.
    /// The group loop stops at an empty turn and does not count it.
    fn repeat(
        &mut self,
        node: &Node,
        min: u32,
        max: Option<u32>,
        greediness: Greediness,
        next: &Next,
        follow: &Follow,
    ) -> Result<TokenStream, Refusal> {
        if let Some(one) = self.one(node) {
            return Ok(self.repeat_one(node, &one, min, max, greediness, next, follow));
        }
        if max == Some(1) && greediness != Greediness::Possessive {
            let (prelude, next) = self.bind(next);
            let next = Next::Call(next);
            let once = self.node(node, &next, follow)?;
            let skip = next.code();
            let tries = match (greediness, min) {
                (_, 1) => once,
                (Greediness::Lazy, _) => self.try_in_order(vec![skip, once]),
                (Greediness::Greedy | Greediness::Possessive, _) => {
                    self.try_in_order(vec![once, skip])
                }
            };
            return Ok(quote!(#prelude #tries));
        }
        if min > 0 && first_of(node).1 {
            return Err(Refusal(
                "a repeated group whose turn can match empty, with a least of one or more"
                    .to_owned(),
            ));
        }
        self.repeat_group(node, min, max, greediness, next, follow)
    }

    /// A repeat of one character a turn.
    ///
    /// - It loops, and gives back one character at a time.
    /// - A greedy one gives back nothing when what follows must read a
    ///   character the body cannot take. At each given-back position the body
    ///   took that character, so what follows fails there.
    /// - Such a `\s*` or `\s+` calls the lexer's `space_end`, which tests ASCII
    ///   by byte and decodes only above it.
    ///
    /// A loop that takes turns spends the bytes it took when it ends.
    /// A lazy loop spends each turn's bytes as it takes the turn.
    /// Each turn given back spends one step.
    /// A loop that gives back, or tries the continuation first, checks the
    /// budget on each turn.
    /// Once the budget is spent, the loop stops with no match.
    #[expect(
        clippy::too_many_arguments,
        reason = "the repeat's own fields, then the continuation and its analysis"
    )]
    fn repeat_one(
        &mut self,
        node: &Node,
        one: &OneCharacter,
        min: u32,
        max: Option<u32>,
        greediness: Greediness,
        next: &Next,
        follow: &Follow,
    ) -> TokenStream {
        let OneCharacter {
            arm,
            read,
            width,
            ascii,
        } = one;
        let limit = bound(max);
        let least = Literal::u32_unsuffixed(min);
        let fail = fail();
        let give_back = if *ascii {
            quote!(end -= 1;)
        } else {
            quote!(end = prev_boundary(lx, end);)
        };
        let take = quote! {
            let start = pos;
            let mut end = pos;
            let mut count = 0u32;
            while count < #limit {
                match #read(lx, end) {
                    #arm => {
                        end += #width;
                        count += 1;
                    }
                    Some(_) | None => break,
                }
            }
            lx.spend(end - start);
        };
        let (set, _) = first_of(node);
        let keeps = match greediness {
            Greediness::Possessive => true,
            Greediness::Greedy => follow.shuns(&set),
            Greediness::Lazy => false,
        };
        let space = min <= 1
            && max.is_none()
            && one_character_set(node).is_some_and(|set| set.same(&Chars::space()));
        if keeps && space {
            let code = next.code();
            let short = (min > 0).then(|| quote!(if end == start { #fail; }));
            return quote! {
                let start = pos;
                let end = lx.space_end(pos);
                lx.spend(end - start);
                #short
                let pos = end;
                #code
            };
        }
        if keeps {
            let code = next.code();
            let short = (min > 0).then(|| quote!(if count < #least { #fail; }));
            return quote! {
                #take
                #short
                let pos = end;
                #code
            };
        }
        let (prelude, next) = self.bind(next);
        match greediness {
            Greediness::Greedy | Greediness::Possessive => {
                let short = (min > 0).then(|| quote!(if count < #least { break None; }));
                quote! {
                    #prelude
                    #take
                    loop {
                        if lx.over_budget() {
                            break None;
                        }
                        #short
                        if let matched @ Some(_) = #next(end, caps) {
                            break matched;
                        }
                        if count == #least {
                            break None;
                        }
                        #give_back
                        count -= 1;
                        lx.spend(1);
                    }
                }
            }
            Greediness::Lazy => Self::lazy_one(one, min, max, &prelude, &next),
        }
    }

    /// A lazy repeat of one character a turn: it tries what follows first,
    /// then takes one more character.
    fn lazy_one(
        one: &OneCharacter,
        min: u32,
        max: Option<u32>,
        prelude: &TokenStream,
        next: &Ident,
    ) -> TokenStream {
        let OneCharacter {
            arm, read, width, ..
        } = one;
        let least = Literal::u32_unsuffixed(min);
        let tried = quote! {
            if let matched @ Some(_) = #next(end, caps) {
                break matched;
            }
        };
        let tried = if min == 0 {
            tried
        } else {
            quote!(if count >= #least { #tried })
        };
        let at_most = max.map(|max| {
            let max = Literal::u32_unsuffixed(max);
            quote!(if count >= #max { break None; })
        });
        let (counter, tick) = counter(min, max);
        quote! {
            #prelude
            let mut end = pos;
            #counter
            loop {
                if lx.over_budget() {
                    break None;
                }
                #tried
                #at_most
                match #read(lx, end) {
                    #arm => {
                        end += #width;
                        #tick
                        lx.spend(#width);
                    }
                    Some(_) | None => break None,
                }
            }
        }
    }

    /// A repeat of a whole node a turn, over a closure that matches one turn.
    ///
    /// - A possessive repeat keeps each turn's first match.
    /// - Any other repeat keeps each turn's first match only when [`unique`]
    ///   proves no other match of the turn can succeed. A greedy one gives
    ///   turns back from an explicit stack, unless what follows must read a
    ///   character the body cannot start with.
    /// - Otherwise the pattern has no generated matcher.
    fn repeat_group(
        &mut self,
        node: &Node,
        min: u32,
        max: Option<u32>,
        greediness: Greediness,
        next: &Next,
        follow: &Follow,
    ) -> Result<TokenStream, Refusal> {
        let (set, empty) = first_of(node);
        let limit = bound(max);
        let least = Literal::u32_unsuffixed(min);
        let (caps, found) = (self.caps(), self.found());
        let turn = self.name("turn");
        let once = resolve(self.first_match(node)?, None);
        let step = quote!(let #turn = |pos: usize, caps: #caps| -> #found { #once };);
        let take = quote! {
            let (mut end, mut ecaps) = (pos, caps);
            let mut count = 0u32;
            while count < #limit {
                match #turn(end, ecaps) {
                    Some((turn_end, turn_caps)) if turn_end != end => {
                        end = turn_end;
                        ecaps = turn_caps;
                        count += 1;
                    }
                    Some(_) | None => break,
                }
            }
            lx.spend(end - pos);
        };
        let fail = fail();
        let short = (min > 0).then(|| quote!(if count < #least { #fail; }));
        if greediness == Greediness::Possessive {
            self.path = self.path.max_with(Path::Loop);
            let code = next.code();
            return Ok(quote! {
                #step
                #take
                #short
                let (pos, caps) = (end, ecaps);
                #code
            });
        }
        if !follow.always && !unique(node, &follow.after_turn(&set, empty)) {
            return Err(Refusal(
                "a repeated group whose turn can match more than one way".to_owned(),
            ));
        }
        let (prelude, next) = self.bind(next);
        Ok(match greediness {
            Greediness::Lazy => {
                self.path = self.path.max_with(Path::Loop);
                let tried = quote! {
                    if let matched @ Some(_) = #next(end, ecaps) {
                        break matched;
                    }
                };
                let tried = if min == 0 {
                    tried
                } else {
                    quote!(if count >= #least { #tried })
                };
                let at_most = max.map(|max| {
                    let max = Literal::u32_unsuffixed(max);
                    quote!(if count >= #max { break None; })
                });
                let (counter, tick) = counter(min, max);
                quote! {
                    #prelude
                    #step
                    let (mut end, mut ecaps) = (pos, caps);
                    #counter
                    loop {
                        if lx.over_budget() {
                            break None;
                        }
                        #tried
                        #at_most
                        match #turn(end, ecaps) {
                            Some((turn_end, turn_caps)) if turn_end != end => {
                                lx.spend(turn_end - end);
                                end = turn_end;
                                ecaps = turn_caps;
                                #tick
                            }
                            Some(_) | None => break None,
                        }
                    }
                }
            }
            Greediness::Greedy | Greediness::Possessive
                if follow.always || (!empty && follow.shuns(&set)) =>
            {
                self.path = self.path.max_with(Path::Loop);
                quote! {
                    #prelude
                    #step
                    #take
                    #short
                    #next(end, ecaps)
                }
            }
            Greediness::Greedy | Greediness::Possessive => {
                self.path = Path::Stack;
                Self::stack_turns(&step, &turn, &caps, min, max, &prelude, &next)
            }
        })
    }

    /// A greedy repeat of a whole node a turn that gives turns back from an
    /// explicit stack.
    fn stack_turns(
        step: &TokenStream,
        turn: &Ident,
        groups: &TokenStream,
        min: u32,
        max: Option<u32>,
        prelude: &TokenStream,
        next: &Ident,
    ) -> TokenStream {
        let limit = bound(max);
        let least = Literal::u32_unsuffixed(min);
        let short = (min > 0).then(|| quote!(if count < #least { break None; }));
        quote! {
            #prelude
            #step
            let mut turns: Vec<(usize, #groups)> = Vec::new();
            let (mut end, mut ecaps) = (pos, caps);
            let mut count = 0u32;
            while count < #limit {
                match #turn(end, ecaps) {
                    Some((turn_end, turn_caps)) if turn_end != end => {
                        turns.push((end, ecaps));
                        end = turn_end;
                        ecaps = turn_caps;
                        count += 1;
                    }
                    Some(_) | None => break,
                }
            }
            lx.spend(end - pos);
            loop {
                if lx.over_budget() {
                    break None;
                }
                #short
                if let matched @ Some(_) = #next(end, ecaps) {
                    break matched;
                }
                match turns.pop() {
                    Some((turn_end, turn_caps)) => {
                        end = turn_end;
                        ecaps = turn_caps;
                        count -= 1;
                        lx.spend(1);
                    }
                    None => break None,
                }
            }
        }
    }

    /// The code that keeps a failed match's scan skip in the field `skips`,
    /// when the pattern `root` earns a [`ScanSkip`].
    fn skip(&mut self, root: &Node, skips: &TokenStream) -> Option<TokenStream> {
        let ScanSkip { run, reach } = scan_skip(root)?;
        let OneCharacter {
            arm, read, width, ..
        } = self.one(run)?;
        let (longer, reach) = (number(reach + 1), number(reach));
        Some(quote! {
            if found.is_none() && !lx.over_budget() {
                let mut end = p;
                loop {
                    match #read(lx, end) {
                        #arm => end += #width,
                        Some(_) | None => break,
                    }
                }
                if end > p + #longer {
                    #skips = Skip::after(lx, p, end, #reach);
                }
            }
        })
    }
}

impl Path {
    /// The costlier of this path and `other`.
    fn max_with(self, other: Path) -> Path {
        match (self, other) {
            (Path::Stack, _) | (_, Path::Stack) => Path::Stack,
            (Path::Loop, _) | (_, Path::Loop) => Path::Loop,
            (Path::Exact, Path::Exact) => Path::Exact,
        }
    }
}

/// Each run of `runs` followed by each run of `tails`.
fn joined(
    runs: &[Vec<Predicate>],
    tails: &[Vec<Predicate>],
) -> Result<Vec<Vec<Predicate>>, Refusal> {
    let joined = runs
        .iter()
        .flat_map(|run| {
            tails.iter().map(move |tail| {
                let mut joined = run.clone();
                joined.extend(tail.iter().cloned());
                joined
            })
        })
        .collect::<Vec<_>>();
    if joined.len() > LOOKBEHIND_RUNS {
        return Err(Refusal("a lookbehind with too many runs".to_owned()));
    }
    Ok(joined)
}

/// Whether `stream` names `name` anywhere, counting into its groups.
fn mentions(stream: &TokenStream, name: &Ident) -> bool {
    stream.clone().into_iter().any(|tree| match tree {
        TokenTree::Ident(ident) => ident == *name,
        TokenTree::Group(group) => mentions(&group.stream(), name),
        TokenTree::Punct(_) | TokenTree::Literal(_) => false,
    })
}

/// The stems of the numbered locals the writer binds.
const BINDINGS: [&str; 4] = ["group_start", "rest", "taken_at", "turn"];

/// `stream` with the number of each numbered local padded to `digits` digits.
///
/// clippy's `similar_names` passes names that differ only in their digits.
/// It flags two that differ by one inserted digit, such as `rest1` and
/// `rest13`.
fn padded(stream: TokenStream, digits: usize) -> TokenStream {
    stream
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Ident(ident) => {
                let text = ident.to_string();
                let number = BINDINGS.iter().find_map(|stem| {
                    text.strip_prefix(stem)
                        .filter(|rest| !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()))
                        .map(|rest| (stem, rest))
                });
                match number {
                    Some((stem, number)) => {
                        TokenTree::Ident(format_ident!("{stem}{number:0>digits$}"))
                    }
                    None => TokenTree::Ident(ident),
                }
            }
            TokenTree::Group(group) => {
                let mut padded_group =
                    Group::new(group.delimiter(), padded(group.stream(), digits));
                padded_group.set_span(group.span());
                TokenTree::Group(padded_group)
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => tree,
        })
        .collect()
}

/// How many tokens `stream` holds, counting into its groups.
fn tokens(stream: &TokenStream) -> usize {
    stream
        .clone()
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Group(group) => 1 + tokens(&group.stream()),
            TokenTree::Ident(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => 1,
        })
        .sum()
}

/// One pattern's generated code.
pub(super) struct Matcher {
    /// The pattern's functions.
    pub(super) items: TokenStream,
    /// The class functions the pattern's functions call.
    pub(super) classes: Vec<ClassFunction>,
    /// Whether the pattern keeps a scan skip in the field named after its
    /// function.
    pub(super) skips: bool,
    /// The position anchor every match starts with, which the dispatch tests
    /// before the call.
    pub(super) leading: Option<Anchor>,
}

/// Writes the matcher of `regex` as the function `function`, or says why it
/// has none.
///
/// `reads` holds the groups the lexer reads after a match.
/// The matcher records them, and the groups a back-reference reads.
///
/// `skips` is the path from a matcher's lexer to the pattern's field of the
/// language's `Skips`, such as `lx.x.kv`, when the language keeps scan skips.
/// When the pattern earns a [`ScanSkip`], the matcher keeps its last failure
/// in that field.
/// It keeps a failure only when the run is longer than `k + 1` bytes.
/// A shorter run holds no start the skip could pass.
pub(super) fn matcher(
    regex: &str,
    function: &Ident,
    reads: &[usize],
    skips: Option<&TokenStream>,
) -> Result<Matcher, Refusal> {
    let mut parsed = parse(regex, Dialect::Grammar).map_err(|error| Refusal(error.to_string()))?;
    let leading = leading_anchor(&parsed.root);
    let mut referenced = Vec::new();
    back_reference_groups(&parsed.root, &mut referenced);
    let one_character = referenced
        .iter()
        .copied()
        .filter(|&group| {
            group_body(&parsed.root, group).is_some_and(|body| body.width() == (1, Some(1)))
        })
        .collect::<Vec<_>>();
    let tracked = reads.iter().copied().chain(referenced).collect::<Vec<_>>();
    keep_read_groups(&mut parsed.root, &tracked);
    drop_covered_arms(&mut parsed.root);
    let last = tracked
        .iter()
        .copied()
        .filter(|&group| group <= parsed.groups)
        .max()
        .unwrap_or(0);
    let mut writer = Writer {
        width: 2 * (last + 1),
        fresh: 0,
        classes: Vec::new(),
        function: function.clone(),
        path: Path::Exact,
        lookbehind: false,
        one_character,
        open: Vec::new(),
    };
    let body = resolve(writer.first_match(&parsed.root)?, None);
    let skip = skips.and_then(|skips| writer.skip(&parsed.root, skips));
    let (found_type, caps) = (writer.found(), number(writer.width));
    let found_function = format_ident!("{function}_found");
    let found = quote! {
        #[inline(always)]
        fn #found_function(lx: &Lexer<'_>, p: usize) -> #found_type {
            let pos = p;
            let caps = [usize::MAX; #caps];
            #body
        }
    };
    let skipping = skip.is_some();
    let function_item = if let (Some(skip), Some(skips)) = (skip, skips) {
        quote! {
            #[inline(never)]
            fn #function(lx: &mut Lexer<'_>, p: usize) -> Option<usize> {
                if #skips.covers(lx, p) {
                    return None;
                }
                let found = #found_function(lx, p);
                #skip
                record(lx, found)
            }
        }
    } else {
        let small = writer.path == Path::Exact && !writer.lookbehind && tokens(&body) <= SMALL_BODY;
        let inline = if small {
            quote!(#[inline])
        } else {
            quote!(#[inline(never)])
        };
        quote! {
            #inline
            fn #function(lx: &mut Lexer<'_>, p: usize) -> Option<usize> {
                let found = #found_function(lx, p);
                record(lx, found)
            }
        }
    };
    let items = quote!(#found #function_item);
    let digits = writer.fresh.to_string().len();
    Ok(Matcher {
        classes: writer
            .classes
            .into_iter()
            .filter(|class| mentions(&items, &class.name))
            .collect(),
        items: padded(items, digits),
        skips: skipping,
        leading,
    })
}
