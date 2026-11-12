//! The parser: `Oniguruma`'s syntax, as `TextMate` grammars use it, to a tree.
//!
//! [`parse`] is visible to the whole crate, beside the tree it returns
//! ([`super::ast::Node`]).

use std::collections::HashMap;

use super::ast::{
    Anchor, Case, Class, Greediness, Member, Node, Polarity, Property, builtin, insensitive_run,
    property_named, prune_repeated_folds,
};

/// A pattern that does not compile: what is wrong with it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub struct Error(
    /// What is wrong with the pattern.
    pub(crate) String,
);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

fn err<T>(message: impl Into<String>) -> Result<T> {
    Err(Error(message.into()))
}

/// The error for a group that opens with `(*`: Oniguruma reads it as a
/// callout, such as `(*FAIL)` or `(*COUNT)`, and the engine has none.
pub(super) const CALLOUTS: &str = "callouts are not supported";

/// A parsed pattern.
#[derive(Debug)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub struct ParsedPattern {
    /// The tree of the whole pattern.
    pub root: Node,
    /// How many capture groups the pattern has.
    pub groups: usize,
    /// The index of each named group.
    pub names: HashMap<String, usize>,
}

/// Which reading of a pattern applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub enum Dialect {
    /// Oniguruma's own.
    /// Only the tests read it.
    #[cfg(test)]
    Oniguruma,
    /// A grammar's pattern as Shiki runs it, where `\z` never matches.
    Grammar,
}

/// Parses `pattern` into its tree.
///
/// # Errors
///
/// Returns an [`Error`] when the pattern is not valid in `dialect`, or uses
/// syntax the engine refuses, such as a callout.
#[cfg_attr(
    not(all(nishiki_unstable, feature = "_rule-engine")),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
pub fn parse(pattern: &str, dialect: Dialect) -> std::result::Result<ParsedPattern, Error> {
    let mut parser = Parser {
        dialect,
        chars: pattern.chars().collect(),
        at: 0,
        flags: Flags {
            case: Case::Sensitive,
            dotall: false,
            extended: false,
        },
        isolated: false,
        lookbehind: false,
        absent: false,
        groups: 0,
        names: HashMap::new(),
    };
    let mut root = parser.alternation()?;
    if parser.at != parser.chars.len() {
        return err("unbalanced parenthesis");
    }
    prune_repeated_folds(&mut root);
    Ok(ParsedPattern {
        root,
        groups: parser.groups,
        names: parser.names,
    })
}

#[derive(Clone, Copy)]
struct Flags {
    case: Case,
    dotall: bool,
    extended: bool,
}

struct Parser {
    /// How the pattern is read.
    dialect: Dialect,
    chars: Vec<char>,
    at: usize,
    flags: Flags,
    /// Set when the last atom read was `(?i)` or another group of flags only.
    isolated: bool,
    /// Set inside a lookbehind, where `(?i)` folds only one character to one.
    lookbehind: bool,
    /// Set inside an absent operator's body.
    /// Oniguruma reads another absent operator there otherwise.
    absent: bool,
    groups: usize,
    names: HashMap<String, usize>,
}

/// What `atom` reads.
/// A literal character under `(?i)` joins the run of literals around it.
/// Any other node stands alone.
enum Atom {
    Folded(char),
    Node(Node),
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn starts(&self, s: &str) -> bool {
        s.chars()
            .enumerate()
            .all(|(i, c)| self.chars.get(self.at + i) == Some(&c))
    }

    fn alternation(&mut self) -> Result<Node> {
        let mut alts = vec![self.sequence()?];
        while self.eat('|') {
            alts.push(self.sequence()?);
        }
        Ok(if alts.len() == 1 {
            alts.pop().unwrap_or(Node::Empty)
        } else {
            Node::Alternation(alts)
        })
    }

    /// Flags with no colon hold to the end of the enclosing group, across its
    /// alternatives.
    /// For example, `a(?i)b|c` is `a(?i:b|c)`.
    ///
    /// Literal characters under `(?i)` gather into a run.
    /// The run folds as a whole when anything else comes.
    /// A quantified character folds alone.
    fn sequence(&mut self) -> Result<Node> {
        let mut items = Vec::new();
        let mut run = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            if self.flags.extended {
                if c.is_whitespace() {
                    self.at += 1;
                    continue;
                }
                if c == '#' {
                    while let Some(c) = self.peek() {
                        self.at += 1;
                        if c == '\n' {
                            break;
                        }
                    }
                    continue;
                }
            }
            let Some(atom) = self.atom()? else {
                if std::mem::take(&mut self.isolated) {
                    items.extend(insensitive_run(&std::mem::take(&mut run), self.lookbehind));
                    items.push(self.alternation()?);
                    break;
                }
                continue;
            };
            match atom {
                Atom::Folded(c) if !self.quantifier_follows() => run.push(c),
                Atom::Folded(c) => {
                    items.extend(insensitive_run(&std::mem::take(&mut run), self.lookbehind));
                    let single = insensitive_run(&[c], self.lookbehind);
                    let node = match <[Node; 1]>::try_from(single) {
                        Ok([node]) => node,
                        Err(nodes) => Node::Sequence(nodes),
                    };
                    items.push(self.quantified(node)?);
                }
                Atom::Node(node) => {
                    items.extend(insensitive_run(&std::mem::take(&mut run), self.lookbehind));
                    items.push(self.quantified(node)?);
                }
            }
        }
        items.extend(insensitive_run(&run, self.lookbehind));
        Ok(match items.len() {
            0 => Node::Empty,
            1 => items.pop().unwrap_or(Node::Empty),
            _ => Node::Sequence(items),
        })
    }

    /// `atom` with the quantifiers that follow it.
    ///
    /// Oniguruma reads `{n,m}` with `n > m` as a possessive `{m,n}`.
    /// A `?` after it repeats it again instead of making it lazy.
    /// Its `fetch_interval` in `regparse.c` swaps the bounds, and
    /// `doc/RE` lists the form under possessive quantifiers.
    fn quantified(&mut self, mut atom: Node) -> Result<Node> {
        loop {
            let (min, max) = match self.peek() {
                Some('*') => {
                    self.at += 1;
                    (0, None)
                }
                Some('+') => {
                    self.at += 1;
                    (1, None)
                }
                Some('?') => {
                    self.at += 1;
                    (0, Some(1))
                }
                Some('{') => match self.braces() {
                    Some((low, Some(high))) if high < low => {
                        atom = Node::Repeat {
                            node: Box::new(atom),
                            min: high,
                            max: Some(low),
                            greediness: Greediness::Possessive,
                        };
                        continue;
                    }
                    Some(range) => range,
                    None => return Ok(atom),
                },
                _ => return Ok(atom),
            };
            let greediness = if self.eat('?') {
                Greediness::Lazy
            } else if self.peek() == Some('+') && self.chars[self.at - 1] != '}' {
                self.at += 1;
                Greediness::Possessive
            } else {
                Greediness::Greedy
            };
            atom = Node::Repeat {
                node: Box::new(atom),
                min,
                max,
                greediness,
            };
        }
    }

    /// `{n}`, `{n,}`, `{,m}` and `{n,m}`.
    /// `None` when the brace is a literal.
    fn braces(&mut self) -> Option<(u32, Option<u32>)> {
        let save = self.at;
        self.at += 1;
        let number = |p: &mut Parser| {
            let start = p.at;
            while p.peek().is_some_and(|c| c.is_ascii_digit()) {
                p.at += 1;
            }
            p.chars[start..p.at]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .ok()
        };
        let low = number(self);
        let range = if self.eat(',') {
            let high = number(self);
            if low.is_none() && high.is_none() {
                None
            } else {
                Some((low.unwrap_or(0), high))
            }
        } else {
            low.map(|n| (n, Some(n)))
        };
        if let Some(range) = range
            && self.eat('}')
        {
            return Some(range);
        }
        self.at = save;
        None
    }

    /// A literal character, as the flags read it.
    /// Under `(?i)`, it joins the run of literals around it.
    /// The run folds as a whole.
    fn char_atom(&self, c: char) -> Atom {
        match self.flags.case {
            Case::Sensitive => Atom::Node(Node::Char(c)),
            Case::Insensitive => Atom::Folded(c),
        }
    }

    /// A bracketed class, as the flags read it.
    fn class_node(&self, class: Class) -> Node {
        match self.flags.case {
            Case::Sensitive => Node::Class(Box::new(class)),
            Case::Insensitive => class.insensitive_node(),
        }
    }

    /// Whether a quantifier comes next.
    /// A quantifier takes only the last character of a literal
    /// run.
    fn quantifier_follows(&mut self) -> bool {
        match self.peek() {
            Some('*' | '+' | '?') => true,
            Some('{') => {
                let save = self.at;
                let found = self.braces().is_some();
                self.at = save;
                found
            }
            _ => false,
        }
    }

    fn atom(&mut self) -> Result<Option<Atom>> {
        let Some(c) = self.peek() else {
            return Ok(None);
        };
        self.at += 1;
        Ok(Some(match c {
            '(' if self.peek() == Some('*') => return err(CALLOUTS),
            '(' => return Ok(self.group()?.map(Atom::Node)),
            '[' => {
                let class = self.class()?;
                Atom::Node(self.class_node(class))
            }
            '.' => Atom::Node(Node::Any {
                newline: self.flags.dotall,
            }),
            '^' => Atom::Node(Node::Anchor(Anchor::LineStart)),
            '$' => Atom::Node(Node::Anchor(Anchor::LineEnd)),
            '\\' => self.escape()?,
            c => self.char_atom(c),
        }))
    }

    fn name_until(&mut self, close: char) -> Result<String> {
        let start = self.at;
        while self.peek().is_some_and(|c| c != close) {
            self.at += 1;
        }
        if self.peek() != Some(close) {
            return err("unterminated group name");
        }
        let name: String = self.chars[start..self.at].iter().collect();
        self.at += 1;
        Ok(name)
    }

    /// The absent operator `(?~X)`, after its `(?`, read as [`absent`]
    /// describes.
    ///
    /// Its other forms, which start `(?~|`, are refused.
    /// So is the operator inside a lookbehind or inside another absent
    /// operator's body, where Oniguruma gives other answers.
    /// For example, Oniguruma's `^(?~a(?~b))` takes all of `bad cab`: the body
    /// never matches.
    fn absent_operator(&mut self) -> Result<Node> {
        if self.chars.get(self.at + 1) == Some(&'|') {
            return err("unsupported absent expression");
        }
        if self.lookbehind {
            return err("absent operator in a lookbehind");
        }
        if self.absent {
            return err("absent operator in another's body");
        }
        self.at += 1;
        self.absent = true;
        let inner = self.alternation();
        self.absent = false;
        let inner = inner?;
        self.expect_close()?;
        Ok(absent(inner))
    }

    /// Flags with no colon, such as `(?i)`, stay on until the
    /// enclosing group ends.
    fn group(&mut self) -> Result<Option<Node>> {
        let outer = self.flags;
        let node = if self.eat('?') {
            match self.peek() {
                Some(':') => {
                    self.at += 1;
                    self.close_group(None)?
                }
                Some('=' | '!') => {
                    let negative = self.peek() == Some('!');
                    self.at += 1;
                    let inner = self.alternation()?;
                    self.expect_close()?;
                    Node::Lookaround {
                        ahead: true,
                        negative,
                        node: Box::new(inner),
                    }
                }
                Some('>') => {
                    self.at += 1;
                    let inner = self.alternation()?;
                    self.expect_close()?;
                    Node::Atomic(Box::new(inner))
                }
                Some('~') => self.absent_operator()?,
                Some('#') => {
                    while self.peek().is_some_and(|c| c != ')') {
                        self.at += 1;
                    }
                    self.expect_close()?;
                    return Ok(None);
                }
                Some('<') if matches!(self.chars.get(self.at + 1), Some('=' | '!')) => {
                    let negative = self.chars[self.at + 1] == '!';
                    self.at += 2;
                    let outer_lookbehind = std::mem::replace(&mut self.lookbehind, true);
                    let inner = self.alternation();
                    self.lookbehind = outer_lookbehind;
                    let inner = inner?;
                    self.expect_close()?;
                    Node::Lookaround {
                        ahead: false,
                        negative,
                        node: Box::new(inner),
                    }
                }
                Some('<' | '\'') => {
                    let close = if self.peek() == Some('<') { '>' } else { '\'' };
                    self.at += 1;
                    let name = self.name_until(close)?;
                    self.groups += 1;
                    let index = self.groups;
                    self.names.insert(name, index);
                    self.close_group(Some(index))?
                }
                Some(c) if c == '-' || c.is_ascii_alphabetic() => {
                    let mut on = true;
                    let mut flags = self.flags;
                    loop {
                        match self.peek() {
                            Some('-') => on = false,
                            Some('i') => {
                                flags.case = if on {
                                    Case::Insensitive
                                } else {
                                    Case::Sensitive
                                };
                            }
                            Some('m') => flags.dotall = on,
                            Some('x') => flags.extended = on,
                            Some('a' | 'u' | 'd' | 'l') => {}
                            Some(':') => {
                                self.at += 1;
                                self.flags = flags;
                                let node = self.close_group(None)?;
                                self.flags = outer;
                                return Ok(Some(node));
                            }
                            Some(')') => {
                                self.at += 1;
                                self.flags = flags;
                                self.isolated = true;
                                return Ok(None);
                            }
                            _ => return err("unknown group flag"),
                        }
                        self.at += 1;
                    }
                }
                _ => return err("unsupported group syntax"),
            }
        } else {
            self.groups += 1;
            let index = self.groups;
            self.close_group(Some(index))?
        };
        self.flags = outer;
        Ok(Some(node))
    }

    fn expect_close(&mut self) -> Result<()> {
        if self.eat(')') {
            Ok(())
        } else {
            err("unterminated group")
        }
    }

    fn close_group(&mut self, index: Option<usize>) -> Result<Node> {
        let inner = self.alternation()?;
        self.expect_close()?;
        Ok(Node::Group {
            index,
            node: Box::new(inner),
        })
    }

    /// In a grammar's pattern, as Shiki runs it, `\z` never matches.
    /// It does not match even after a line's newline.
    /// A rule that ends at `\z` therefore stays open from line to line.
    ///
    /// A property escape outside brackets keeps its case under `(?i)`, as in
    /// Oniguruma.
    /// For example, `(?i)\p{Lu}` refuses `a`.
    /// `(?i)[\p{Lu}]` takes it.
    fn escape(&mut self) -> Result<Atom> {
        let Some(c) = self.peek() else {
            return err("trailing backslash");
        };
        self.at += 1;
        let case = self.flags.case;
        let prop_class = |prop: Property, polarity: Polarity| {
            Node::Class(Box::new(Class {
                negated: false,
                unions: vec![vec![Member::Property(prop, polarity)]],
            }))
        };
        Ok(Atom::Node(match c {
            'd' => prop_class(builtin("digit"), Polarity::Positive),
            'D' => prop_class(builtin("digit"), Polarity::Negated),
            'w' => prop_class(builtin("word"), Polarity::Positive),
            'W' => prop_class(builtin("word"), Polarity::Negated),
            's' => prop_class(builtin("space"), Polarity::Positive),
            'S' => prop_class(builtin("space"), Polarity::Negated),
            'h' => prop_class(builtin("xdigit"), Polarity::Positive),
            'H' => prop_class(builtin("xdigit"), Polarity::Negated),
            'N' => Node::Any { newline: false },
            'O' => Node::Any { newline: true },
            'R' => line_break(),
            'X' => Node::Atomic(Box::new(Node::Sequence(vec![
                Node::Any { newline: true },
                Node::Repeat {
                    node: Box::new(prop_class(builtin("m"), Polarity::Positive)),
                    min: 0,
                    max: None,
                    greediness: Greediness::Greedy,
                },
            ]))),
            'b' => Node::Anchor(Anchor::WordBoundary),
            'B' => Node::Anchor(Anchor::NotWordBoundary),
            'A' => Node::Anchor(Anchor::TextStart),
            'z' if self.dialect == Dialect::Grammar => Node::Lookaround {
                ahead: true,
                negative: true,
                node: Box::new(Node::Empty),
            },
            'z' => Node::Anchor(Anchor::TextEnd),
            'Z' => Node::Anchor(Anchor::TextEndOrFinalNewline),
            'G' => Node::Anchor(Anchor::LastMatchEnd),
            'p' | 'P' => {
                let (prop, polarity) = self.property(if c == 'P' {
                    Polarity::Negated
                } else {
                    Polarity::Positive
                })?;
                prop_class(prop, polarity)
            }
            'k' => {
                let close = match self.peek() {
                    Some('<') => '>',
                    Some('\'') => '\'',
                    _ => return err("bad \\k"),
                };
                self.at += 1;
                let name = self.name_until(close)?;
                match name.parse::<usize>() {
                    Ok(n) => Node::BackReference(n, case),
                    Err(_) => Node::NamedBackReference(name, case),
                }
            }
            'g' => {
                let close = match self.peek() {
                    Some('<') => '>',
                    Some('\'') => '\'',
                    _ => return err("bad \\g"),
                };
                self.at += 1;
                let name = self.name_until(close)?;
                match name.parse::<usize>() {
                    Ok(n) => Node::Subroutine(n),
                    Err(_) => Node::NamedSubroutine(name),
                }
            }
            '1'..='9' => Node::BackReference(
                c.to_digit(10)
                    .map_or(0, |d| usize::try_from(d).unwrap_or(0)),
                case,
            ),
            _ => {
                let c = self.simple_escape(c)?;
                return Ok(self.char_atom(c));
            }
        }))
    }

    /// The character an escape such as `\n` or `\x41` stands for.
    fn simple_escape(&mut self, c: char) -> Result<char> {
        Ok(match c {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            'f' => '\u{c}',
            'v' => '\u{b}',
            'e' => '\u{1b}',
            'a' => '\u{7}',
            '0' => '\0',
            'x' => self.hex_escape()?,
            'u' => self.unicode_escape()?,
            other => other,
        })
    }

    /// `\x{H…}`, or `\xHH` with one or two digits.
    fn hex_escape(&mut self) -> Result<char> {
        let braced = self.eat('{');
        let start = self.at;
        let limit = if braced { usize::MAX } else { 2 };
        while self.at - start < limit && self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
            self.at += 1;
        }
        let text: String = self.chars[start..self.at].iter().collect();
        if braced && !self.eat('}') {
            return err("unterminated \\x{");
        }
        u32::from_str_radix(&text, 16)
            .ok()
            .and_then(char::from_u32)
            .map_or_else(|| err("bad hex escape"), Ok)
    }

    /// `\uHHHH`: exactly four hex digits.
    ///
    /// Oniguruma refuses the pattern when fewer follow.
    /// Its `fetch_token` in `regparse.c` reads at least four and at most four.
    /// A `\u` that ends the pattern is a literal `u` there.
    fn unicode_escape(&mut self) -> Result<char> {
        if self.at == self.chars.len() {
            return Ok('u');
        }
        let end = self.at + 4;
        let Some(digits) = self
            .chars
            .get(self.at..end)
            .filter(|digits| digits.iter().all(char::is_ascii_hexdigit))
        else {
            return err("\\u takes four hex digits");
        };
        let text: String = digits.iter().collect();
        self.at = end;
        u32::from_str_radix(&text, 16)
            .ok()
            .and_then(char::from_u32)
            .map_or_else(|| err("bad hex escape"), Ok)
    }

    fn property(&mut self, polarity: Polarity) -> Result<(Property, Polarity)> {
        if !self.eat('{') {
            return err("bad \\p");
        }
        let polarity = if self.eat('^') {
            polarity.inverse()
        } else {
            polarity
        };
        let name = self.name_until('}')?;
        match property_named(&name) {
            Some(prop) => Ok((prop, polarity)),
            None => err(format!("unknown property {name}")),
        }
    }

    fn class(&mut self) -> Result<Class> {
        let negated = self.eat('^');
        let mut parts = vec![Vec::new()];
        let mut first = true;
        loop {
            let Some(c) = self.peek() else {
                return err("unterminated class");
            };
            if c == ']' && !first {
                self.at += 1;
                break;
            }
            first = false;
            if self.starts("&&") {
                self.at += 2;
                parts.push(Vec::new());
                continue;
            }
            if c == '[' {
                if self.starts("[:") {
                    let save = self.at;
                    self.at += 2;
                    let polarity = if self.eat('^') {
                        Polarity::Negated
                    } else {
                        Polarity::Positive
                    };
                    let start = self.at;
                    while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
                        self.at += 1;
                    }
                    let name: String = self.chars[start..self.at].iter().collect();
                    if self.starts(":]")
                        && let Some(prop) = property_named(&name)
                    {
                        self.at += 2;
                        if let Some(part) = parts.last_mut() {
                            part.push(Member::Property(prop, polarity));
                        }
                        continue;
                    }
                    self.at = save;
                }
                self.at += 1;
                let nested = self.class()?;
                if let Some(part) = parts.last_mut() {
                    part.push(Member::Nested(nested));
                }
                continue;
            }
            self.at += 1;
            let item = if c == '\\' {
                let Some(e) = self.peek() else {
                    return err("trailing backslash in class");
                };
                self.at += 1;
                match e {
                    'd' => Member::Property(builtin("digit"), Polarity::Positive),
                    'D' => Member::Property(builtin("digit"), Polarity::Negated),
                    'w' => Member::Property(builtin("word"), Polarity::Positive),
                    'W' => Member::Property(builtin("word"), Polarity::Negated),
                    's' => Member::Property(builtin("space"), Polarity::Positive),
                    'S' => Member::Property(builtin("space"), Polarity::Negated),
                    'h' => Member::Property(builtin("xdigit"), Polarity::Positive),
                    'H' => Member::Property(builtin("xdigit"), Polarity::Negated),
                    'p' | 'P' => {
                        let (prop, polarity) = self.property(if e == 'P' {
                            Polarity::Negated
                        } else {
                            Polarity::Positive
                        })?;
                        Member::Property(prop, polarity)
                    }
                    'b' => Member::Range('\u{8}', '\u{8}'),
                    other => {
                        let ch = self.simple_escape(other)?;
                        self.range_from(ch)?
                    }
                }
            } else {
                self.range_from(c)?
            };
            if let Some(part) = parts.last_mut() {
                part.push(item);
            }
        }
        Ok(Class {
            negated,
            unions: parts,
        })
    }

    /// `lo` alone, or `lo-hi` when a range follows.
    fn range_from(&mut self, lo: char) -> Result<Member> {
        if self.peek() == Some('-')
            && self
                .chars
                .get(self.at + 1)
                .is_some_and(|&c| c != ']' && c != '[')
        {
            self.at += 1;
            let hi = self.peek().unwrap_or(lo);
            self.at += 1;
            let hi = if hi == '\\' {
                let e = self.peek().unwrap_or('\\');
                self.at += 1;
                self.simple_escape(e)?
            } else {
                hi
            };
            return Ok(Member::Range(lo, hi));
        }
        Ok(Member::Range(lo, lo))
    }
}

/// The absent operator `(?~X)`, read as `(?:(?!X)\O)*`.
///
/// It takes characters, newlines too, while `X` does not match where the next
/// one starts.
/// A match of `X` that starts inside the range ends it, even when that match
/// runs past the range's end.
/// The loop is greedy and gives characters back on backtracking.
///
/// Oniguruma documents the operator as the longest string with no match of
/// `X` inside it.
/// Its answers follow this reading instead.
/// On 2026-11-10, Shiki 4.6.0's Oniguruma gave the same spans for both forms
/// on 124,800 random cases.
/// They put 26 bodies in 16 surrounding patterns, none of them nested.
/// `tests/engine/absent/` holds its answers for the regex tests.
fn absent(body: Node) -> Node {
    Node::Repeat {
        node: Box::new(Node::Sequence(vec![
            Node::Lookaround {
                ahead: true,
                negative: true,
                node: Box::new(body),
            },
            Node::Any { newline: true },
        ])),
        min: 0,
        max: None,
        greediness: Greediness::Greedy,
    }
}

/// `\R`: a line ending, `\r\n` or any one character that ends a line.
fn line_break() -> Node {
    let separator = |code: u32| char::from_u32(code).unwrap_or('\n');
    Node::Atomic(Box::new(Node::Alternation(vec![
        Node::Sequence(vec![Node::Char('\r'), Node::Char('\n')]),
        Node::Class(Box::new(Class {
            negated: false,
            unions: vec![vec![
                Member::Range('\n', '\r'),
                Member::Range(separator(0x85), separator(0x85)),
                Member::Range(separator(0x2028), separator(0x2029)),
            ]],
        })),
    ])))
}
