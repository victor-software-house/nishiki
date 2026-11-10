//! The parser: `Oniguruma`'s syntax, as `TextMate` grammars use it, to a tree.

use std::collections::HashMap;

use super::ast::{
    Anchor, Case, Class, Greediness, Member, Node, Polarity, Property, builtin, insensitive_run,
    property_named, prune_repeated_folds,
};

/// A pattern that does not compile: what is wrong with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Error(pub(crate) String);

type Result<T> = std::result::Result<T, Error>;

fn err<T>(message: impl Into<String>) -> Result<T> {
    Err(Error(message.into()))
}

/// A parsed pattern.
#[derive(Debug)]
pub(super) struct Parsed {
    pub(super) root: Node,
    /// How many capture groups the pattern has.
    pub(super) groups: usize,
    /// The index of each named group.
    pub(super) names: HashMap<String, usize>,
}

/// Which reading of a pattern applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Dialect {
    /// Oniguruma's own.
    /// Only the tests read it.
    #[cfg(test)]
    Oniguruma,
    /// A grammar's pattern as Shiki runs it, where `\z` never matches.
    Grammar,
}

/// Parses `pattern`.
pub(super) fn parse(pattern: &str, dialect: Dialect) -> Result<Parsed> {
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
    Ok(Parsed {
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
    /// Set when the atom just read was `(?i)` or another group of flags only.
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
            Node::Alt(alts)
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
                        Err(nodes) => Node::Seq(nodes),
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
            _ => Node::Seq(items),
        })
    }

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
    /// A quantifier takes only the character before it out of a run of
    /// literals.
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
            '(' => return Ok(self.group()?.map(Atom::Node)),
            '[' => {
                let class = self.class()?;
                Atom::Node(self.class_node(class))
            }
            '.' => Atom::Node(Node::Any(self.flags.dotall)),
            '^' => Atom::Node(Node::Anchor(Anchor::Caret)),
            '$' => Atom::Node(Node::Anchor(Anchor::Dollar)),
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

    /// The absent operator `(?~X)`, after its `(?`, read as [`absent`] says.
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

    /// Flags with no colon, such as `(?i)`, stay on for the rest of the
    /// enclosing group.
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
                    Node::Look {
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
                    Node::Look {
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
                parts: vec![vec![Member::Property(prop, polarity)]],
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
            'N' => Node::Any(false),
            'O' => Node::Any(true),
            'R' => line_break(),
            'X' => Node::Atomic(Box::new(Node::Seq(vec![
                Node::Any(true),
                Node::Repeat {
                    node: Box::new(prop_class(builtin("m"), Polarity::Positive)),
                    min: 0,
                    max: None,
                    greediness: Greediness::Greedy,
                },
            ]))),
            'b' => Node::Anchor(Anchor::WordBoundary),
            'B' => Node::Anchor(Anchor::NotWordBoundary),
            'A' => Node::Anchor(Anchor::StartText),
            'z' if self.dialect == Dialect::Grammar => Node::Look {
                ahead: true,
                negative: true,
                node: Box::new(Node::Empty),
            },
            'z' => Node::Anchor(Anchor::EndText),
            'Z' => Node::Anchor(Anchor::EndTextNewline),
            'G' => Node::Anchor(Anchor::Search),
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
                    Ok(n) => Node::Backref(n, case),
                    Err(_) => Node::NamedBackref(name, case),
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
                    Ok(n) => Node::Call(n),
                    Err(_) => Node::NamedCall(name),
                }
            }
            '1'..='9' => Node::Backref(
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
            'x' => self.hex_escape(2)?,
            'u' => self.hex_escape(4)?,
            other => other,
        })
    }

    fn hex_escape(&mut self, digits: usize) -> Result<char> {
        let braced = self.eat('{');
        let start = self.at;
        let limit = if braced { usize::MAX } else { digits };
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
        Ok(Class { negated, parts })
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
        node: Box::new(Node::Seq(vec![
            Node::Look {
                ahead: true,
                negative: true,
                node: Box::new(body),
            },
            Node::Any(true),
        ])),
        min: 0,
        max: None,
        greediness: Greediness::Greedy,
    }
}

/// `\R`: a line ending, `\r\n` or any one character that ends a line.
fn line_break() -> Node {
    let separator = |code: u32| char::from_u32(code).unwrap_or('\n');
    Node::Atomic(Box::new(Node::Alt(vec![
        Node::Seq(vec![Node::Char('\r'), Node::Char('\n')]),
        Node::Class(Box::new(Class {
            negated: false,
            parts: vec![vec![
                Member::Range('\n', '\r'),
                Member::Range(separator(0x85), separator(0x85)),
                Member::Range(separator(0x2028), separator(0x2029)),
            ]],
        })),
    ])))
}
