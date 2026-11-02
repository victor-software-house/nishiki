//! A backtracking regular-expression engine for the Oniguruma dialect that
//! TextMate grammars are written in: classes with `&&` and POSIX brackets,
//! lookahead and lookbehind, atomic groups, lazy and possessive quantifiers,
//! backreferences, named groups, subexpression calls (`\g<name>`), inline
//! flags and the anchors `\A`, `\G`, `^` and `$`.
//!
//! The matcher is written in continuation-passing style, which keeps every
//! construct short at the cost of stack depth per matched character; callers
//! run it on a thread with a large stack. It decodes UTF-8 as it goes.
//! Unicode properties are approximated with the standard library's character
//! tests.

use std::collections::HashMap;
use std::rc::Rc;

/// A compile error: the pattern and what is wrong with it.
#[derive(Debug)]
pub struct Error(pub String);

type Result<T> = std::result::Result<T, Error>;

fn err<T>(message: impl Into<String>) -> Result<T> {
    Err(Error(message.into()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Anchor {
    Caret,
    Dollar,
    StartText,
    EndText,
    EndTextNewline,
    Search,
    WordBoundary,
    NotWordBoundary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Greedy,
    Lazy,
    Possessive,
}

#[derive(Clone, Debug)]
enum Item {
    Range(char, char),
    Prop(Prop, bool),
    Nested(Class),
}

/// A character class: the intersection of unions of items, negated or not.
#[derive(Clone, Debug)]
struct Class {
    negated: bool,
    parts: Vec<Vec<Item>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Prop {
    Digit,
    Word,
    Space,
    Hex,
    Alpha,
    Alnum,
    Upper,
    Lower,
    Punct,
    Cntrl,
    Graph,
    Print,
    Blank,
    Ascii,
    Number,
    Mark,
    Symbol,
    Separator,
    Any,
}

fn prop_named(name: &str) -> Option<Prop> {
    let lower = name.to_ascii_lowercase().replace(['_', ' ', '-'], "");
    Some(match lower.as_str() {
        // The escaped name is the Unicode general category for decimal digits.
        "digit" | "n\x64" => Prop::Digit,
        "word" | "w" => Prop::Word,
        "space" | "whitespace" => Prop::Space,
        "xdigit" | "hex" | "asciihexdigit" => Prop::Hex,
        "alpha" | "alphabetic" | "l" | "letter" | "lo" | "lm" | "lt" | "idstart" | "xidstart" => {
            Prop::Alpha
        }
        "alnum" => Prop::Alnum,
        "upper" | "lu" | "uppercase" => Prop::Upper,
        "lower" | "ll" | "lowercase" => Prop::Lower,
        "punct" | "p" | "punctuation" => Prop::Punct,
        "cntrl" | "cc" | "control" => Prop::Cntrl,
        "graph" => Prop::Graph,
        "print" => Prop::Print,
        "blank" => Prop::Blank,
        "ascii" => Prop::Ascii,
        "n" | "number" | "nl" | "no" => Prop::Number,
        "m" | "mark" | "mn" | "mc" | "me" => Prop::Mark,
        "s" | "symbol" | "sm" | "sc" | "sk" | "so" => Prop::Symbol,
        "z" | "separator" | "zs" => Prop::Separator,
        "any" => Prop::Any,
        "idcontinue" | "xidcontinue" => Prop::Word,
        _ => return None,
    })
}

fn prop_matches(prop: &Prop, c: char) -> bool {
    match prop {
        Prop::Digit => c.is_ascii_digit() || (!c.is_ascii() && c.is_numeric()),
        Prop::Word => is_word(c),
        Prop::Space => c.is_whitespace(),
        Prop::Hex => c.is_ascii_hexdigit(),
        Prop::Alpha => c.is_alphabetic(),
        Prop::Alnum => c.is_alphanumeric(),
        Prop::Upper => c.is_uppercase(),
        Prop::Lower => c.is_lowercase(),
        Prop::Punct => c.is_ascii_punctuation() && !"$+<=>^`|~".contains(c),
        Prop::Cntrl => c.is_control(),
        Prop::Graph => !c.is_whitespace() && !c.is_control(),
        Prop::Print => !c.is_control(),
        Prop::Blank => c == ' ' || c == '\t',
        Prop::Ascii => c.is_ascii(),
        Prop::Number => c.is_numeric(),
        Prop::Mark => ('\u{300}'..='\u{36f}').contains(&c),
        Prop::Symbol => "$+<=>^`|~".contains(c),
        Prop::Separator => c.is_whitespace() && !c.is_control(),
        Prop::Any => true,
    }
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || ('\u{300}'..='\u{36f}').contains(&c)
}

fn fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

impl Class {
    fn matches(&self, c: char, icase: bool) -> bool {
        let hit = |c: char| {
            self.parts.iter().all(|union| {
                union.iter().any(|item| match item {
                    Item::Range(lo, hi) => (*lo..=*hi).contains(&c),
                    Item::Prop(prop, negated) => prop_matches(prop, c) != *negated,
                    Item::Nested(class) => class.matches(c, false),
                })
            })
        };
        let found = hit(c)
            || (icase && {
                let lower = fold(c);
                let upper = c.to_uppercase().next().unwrap_or(c);
                hit(lower) || hit(upper)
            });
        found != self.negated
    }
}

#[derive(Clone, Debug)]
enum Node {
    Empty,
    Char(char, bool),
    Any(bool),
    Class(Box<Class>, bool),
    Seq(Vec<Node>),
    Alt(Vec<Node>),
    Repeat {
        node: Box<Node>,
        min: u32,
        max: Option<u32>,
        kind: Kind,
    },
    Group {
        index: Option<usize>,
        node: Rc<Node>,
    },
    Look {
        ahead: bool,
        negative: bool,
        node: Box<Node>,
    },
    Atomic(Box<Node>),
    Backref(usize, bool),
    NamedBackref(String, bool),
    Call(usize),
    NamedCall(String),
    Anchor(Anchor),
}

#[derive(Clone, Copy)]
struct Flags {
    icase: bool,
    dotall: bool,
    extended: bool,
}

struct Parser {
    chars: Vec<char>,
    at: usize,
    flags: Flags,
    groups: usize,
    names: HashMap<String, usize>,
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

    fn sequence(&mut self) -> Result<Node> {
        let mut items = Vec::new();
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
            let Some(atom) = self.atom()? else { continue };
            items.push(self.quantified(atom)?);
        }
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
            let kind = if self.eat('?') {
                Kind::Lazy
            } else if self.peek() == Some('+') && self.chars[self.at - 1] != '}' {
                self.at += 1;
                Kind::Possessive
            } else {
                Kind::Greedy
            };
            atom = Node::Repeat {
                node: Box::new(atom),
                min,
                max,
                kind,
            };
        }
    }

    /// `{n}`, `{n,}`, `{,m}` and `{n,m}`; none when the brace is a literal.
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

    fn atom(&mut self) -> Result<Option<Node>> {
        let Some(c) = self.peek() else {
            return Ok(None);
        };
        self.at += 1;
        Ok(Some(match c {
            '(' => return self.group(),
            '[' => {
                let class = self.class()?;
                Node::Class(Box::new(class), self.flags.icase)
            }
            '.' => Node::Any(self.flags.dotall),
            '^' => Node::Anchor(Anchor::Caret),
            '$' => Node::Anchor(Anchor::Dollar),
            '\\' => self.escape()?,
            c => Node::Char(c, self.flags.icase),
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

    fn group(&mut self) -> Result<Option<Node>> {
        let outer = self.flags;
        let node = if self.eat('?') {
            match self.peek() {
                Some(':') => {
                    self.at += 1;
                    self.close_group(None)?
                }
                Some('=') | Some('!') => {
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
                Some('#') => {
                    while self.peek().is_some_and(|c| c != ')') {
                        self.at += 1;
                    }
                    self.expect_close()?;
                    return Ok(None);
                }
                Some('<') if matches!(self.chars.get(self.at + 1), Some('=') | Some('!')) => {
                    let negative = self.chars[self.at + 1] == '!';
                    self.at += 2;
                    let inner = self.alternation()?;
                    self.expect_close()?;
                    Node::Look {
                        ahead: false,
                        negative,
                        node: Box::new(inner),
                    }
                }
                Some('<') | Some('\'') => {
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
                            Some('i') => flags.icase = on,
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
                                // The flags stay for the rest of the enclosing group.
                                self.flags = flags;
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
            node: Rc::new(inner),
        })
    }

    fn escape(&mut self) -> Result<Node> {
        let Some(c) = self.peek() else {
            return err("trailing backslash");
        };
        self.at += 1;
        let icase = self.flags.icase;
        let prop_class = |prop: Prop, negated: bool| {
            Node::Class(
                Box::new(Class {
                    negated: false,
                    parts: vec![vec![Item::Prop(prop, negated)]],
                }),
                false,
            )
        };
        Ok(match c {
            'd' => prop_class(Prop::Digit, false),
            'D' => prop_class(Prop::Digit, true),
            'w' => prop_class(Prop::Word, false),
            'W' => prop_class(Prop::Word, true),
            's' => prop_class(Prop::Space, false),
            'S' => prop_class(Prop::Space, true),
            'h' => prop_class(Prop::Hex, false),
            'H' => prop_class(Prop::Hex, true),
            'b' => Node::Anchor(Anchor::WordBoundary),
            'B' => Node::Anchor(Anchor::NotWordBoundary),
            'A' => Node::Anchor(Anchor::StartText),
            'z' => Node::Anchor(Anchor::EndText),
            'Z' => Node::Anchor(Anchor::EndTextNewline),
            'G' => Node::Anchor(Anchor::Search),
            'p' | 'P' => {
                let (prop, negated) = self.property(c == 'P')?;
                prop_class(prop, negated)
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
                    Ok(n) => Node::Backref(n, icase),
                    Err(_) => Node::NamedBackref(name, icase),
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
            '1'..='9' => Node::Backref(usize::from(c as u8 - b'0'), icase),
            _ => Node::Char(self.simple_escape(c)?, icase),
        })
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

    fn property(&mut self, negated: bool) -> Result<(Prop, bool)> {
        if !self.eat('{') {
            return err("bad \\p");
        }
        let negated = negated ^ self.eat('^');
        let name = self.name_until('}')?;
        match prop_named(&name) {
            Some(prop) => Ok((prop, negated)),
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
                    let neg = self.eat('^');
                    let start = self.at;
                    while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
                        self.at += 1;
                    }
                    let name: String = self.chars[start..self.at].iter().collect();
                    if self.starts(":]")
                        && let Some(prop) = prop_named(&name)
                    {
                        self.at += 2;
                        if let Some(part) = parts.last_mut() {
                            part.push(Item::Prop(prop, neg));
                        }
                        continue;
                    }
                    self.at = save;
                }
                self.at += 1;
                let nested = self.class()?;
                if let Some(part) = parts.last_mut() {
                    part.push(Item::Nested(nested));
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
                    'd' => Item::Prop(Prop::Digit, false),
                    'D' => Item::Prop(Prop::Digit, true),
                    'w' => Item::Prop(Prop::Word, false),
                    'W' => Item::Prop(Prop::Word, true),
                    's' => Item::Prop(Prop::Space, false),
                    'S' => Item::Prop(Prop::Space, true),
                    'h' => Item::Prop(Prop::Hex, false),
                    'H' => Item::Prop(Prop::Hex, true),
                    'p' | 'P' => {
                        let (prop, negated) = self.property(e == 'P')?;
                        Item::Prop(prop, negated)
                    }
                    'b' => Item::Range('\u{8}', '\u{8}'),
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
    fn range_from(&mut self, lo: char) -> Result<Item> {
        if self.peek() == Some('-')
            && self.chars.get(self.at + 1).is_some_and(|&c| c != ']' && c != '[')
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
            return Ok(Item::Range(lo, hi));
        }
        Ok(Item::Range(lo, lo))
    }
}

/// Where a match can start, beyond the character at the start.
#[derive(Clone, Copy, Debug, Default)]
struct Starts {
    caret: bool,
    text: bool,
    search: bool,
}

/// A compiled regular expression.
#[derive(Debug)]
pub struct Regex {
    root: Node,
    groups: Vec<Option<Rc<Node>>>,
    names: HashMap<String, usize>,
    count: usize,
    first: Option<Box<[bool; 256]>>,
    anchored: Option<Starts>,
}

/// The groups of a match, `0` being the whole match.
pub type Captures = Vec<Option<(usize, usize)>>;

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex> {
        let mut parser = Parser {
            chars: pattern.chars().collect(),
            at: 0,
            flags: Flags {
                icase: false,
                dotall: false,
                extended: false,
            },
            groups: 0,
            names: HashMap::new(),
        };
        let root = parser.alternation()?;
        if parser.at != parser.chars.len() {
            return err("unbalanced parenthesis");
        }
        let mut groups = vec![None; parser.groups + 1];
        collect_groups(&root, &mut groups);
        let mut set = [false; 256];
        let nullable = first_bytes(&root, &mut set);
        let first = (!nullable && set.iter().any(|&b| !b)).then(|| Box::new(set));
        let anchored = anchors(&root);
        Ok(Regex {
            root,
            groups,
            names: parser.names,
            count: parser.groups,
            first,
            anchored,
        })
    }

    /// The groups the pattern has.
    pub fn group_count(&self) -> usize {
        self.count
    }

    /// The leftmost match that starts at or after `from`. `search` is where
    /// `\G` matches, and `text_start` whether `\A` matches at offset 0.
    /// `steps` is a budget of matcher steps, which the search spends; when it
    /// reaches zero the search gives up.
    pub fn find_at(
        &self,
        text: &str,
        from: usize,
        search: Option<usize>,
        text_start: bool,
        steps: &mut u64,
    ) -> Option<Captures> {
        let mut cx = Cx {
            re: self,
            text,
            bytes: text.as_bytes(),
            search,
            text_start,
            caps: vec![None; self.count + 1],
            steps: *steps,
            calls: 0,
        };
        let found = self.search(&mut cx, from);
        *steps = cx.steps;
        found
    }

    fn search(&self, cx: &mut Cx<'_>, from: usize) -> Option<Captures> {
        let len = cx.bytes.len();
        if let Some(starts) = self.anchored {
            let mut candidates = [None; 3];
            if starts.text || starts.caret {
                candidates[0] = Some(0);
            }
            if starts.caret {
                // After a newline: the line holds one, at its end.
                candidates[1] = len.checked_sub(1).filter(|&i| cx.bytes[i] == b'\n').map(|i| i + 1);
            }
            if starts.search {
                candidates[2] = cx.search;
            }
            let mut list: Vec<usize> = candidates.iter().flatten().copied().collect();
            list.sort_unstable();
            list.dedup();
            return list
                .into_iter()
                .filter(|&p| p >= from && p <= len)
                .find_map(|p| self.try_at(cx, p));
        }
        let mut p = from;
        while p <= len {
            if let Some(set) = &self.first {
                match cx.bytes[p..].iter().position(|&b| set[usize::from(b)]) {
                    Some(skip) => p += skip,
                    None => return None,
                }
            }
            if let Some(found) = self.try_at(cx, p) {
                return Some(found);
            }
            if cx.steps == 0 {
                return None;
            }
            p += 1;
            while p < len && cx.bytes[p] & 0xc0 == 0x80 {
                p += 1;
            }
        }
        None
    }

    fn try_at(&self, cx: &mut Cx<'_>, p: usize) -> Option<Captures> {
        for cap in &mut cx.caps {
            *cap = None;
        }
        let mut end = None;
        let matched = cx.m(&self.root, p, &mut |e, _| {
            end = Some(e);
            true
        });
        if matched {
            let mut caps = cx.caps.clone();
            caps[0] = Some((p, end.unwrap_or(p)));
            Some(caps)
        } else {
            None
        }
    }
}

fn collect_groups(node: &Node, table: &mut [Option<Rc<Node>>]) {
    match node {
        Node::Seq(items) | Node::Alt(items) => items.iter().for_each(|n| collect_groups(n, table)),
        Node::Repeat { node, .. } | Node::Look { node, .. } => collect_groups(node, table),
        Node::Atomic(node) => collect_groups(node, table),
        Node::Group { index, node } => {
            if let Some(i) = index {
                table[*i] = Some(Rc::new(node.as_ref().clone()));
            }
            collect_groups(node, table);
        }
        _ => {}
    }
}

/// Adds the bytes a match of `node` can start with to `set`; true when the
/// node can match without consuming anything, so what follows counts too.
fn first_bytes(node: &Node, set: &mut [bool; 256]) -> bool {
    let all = |set: &mut [bool; 256]| set.iter_mut().for_each(|b| *b = true);
    match node {
        Node::Empty | Node::Anchor(_) | Node::Look { .. } => true,
        Node::Char(c, icase) => {
            let mut buffer = [0; 4];
            set[usize::from(c.encode_utf8(&mut buffer).as_bytes()[0])] = true;
            if *icase {
                for other in c.to_lowercase().chain(c.to_uppercase()) {
                    set[usize::from(other.encode_utf8(&mut buffer).as_bytes()[0])] = true;
                }
            }
            false
        }
        Node::Any(_) | Node::Class(..) | Node::Backref(..) | Node::NamedBackref(..) => {
            if let Node::Class(class, icase) = node {
                for b in 0..128u8 {
                    if class.matches(char::from(b), *icase) {
                        set[usize::from(b)] = true;
                    }
                }
                for b in 0xc0..=0xffu8 {
                    set[usize::from(b)] = true;
                }
                false
            } else {
                all(set);
                matches!(node, Node::Backref(..) | Node::NamedBackref(..))
            }
        }
        Node::Call(_) | Node::NamedCall(_) => {
            all(set);
            true
        }
        Node::Seq(items) => {
            for item in items {
                if !first_bytes(item, set) {
                    return false;
                }
            }
            true
        }
        Node::Alt(alts) => {
            let mut nullable = false;
            for alt in alts {
                nullable |= first_bytes(alt, set);
            }
            nullable
        }
        Node::Repeat { node, min, .. } => {
            let nullable = first_bytes(node, set);
            nullable || *min == 0
        }
        Node::Group { node, .. } => first_bytes(node, set),
        Node::Atomic(node) => first_bytes(node, set),
    }
}

/// The anchors every match of `node` begins at, when it has one.
fn anchors(node: &Node) -> Option<Starts> {
    match node {
        Node::Anchor(Anchor::Caret) => Some(Starts {
            caret: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::StartText) => Some(Starts {
            text: true,
            ..Starts::default()
        }),
        Node::Anchor(Anchor::Search) => Some(Starts {
            search: true,
            ..Starts::default()
        }),
        Node::Seq(items) => items.first().and_then(anchors),
        Node::Group { node, .. } => anchors(node),
        Node::Atomic(node) => anchors(node),
        Node::Alt(alts) => {
            let mut all = Starts::default();
            for alt in alts {
                let s = anchors(alt)?;
                all.caret |= s.caret;
                all.text |= s.text;
                all.search |= s.search;
            }
            Some(all)
        }
        _ => None,
    }
}

struct Cx<'a> {
    re: &'a Regex,
    text: &'a str,
    bytes: &'a [u8],
    search: Option<usize>,
    text_start: bool,
    caps: Captures,
    steps: u64,
    calls: u32,
}

type Cont<'k> = &'k mut dyn FnMut(usize, &mut Cx<'_>) -> bool;

impl Cx<'_> {
    fn char_at(&self, pos: usize) -> Option<char> {
        self.text.get(pos..)?.chars().next()
    }

    fn char_before(&self, pos: usize) -> Option<char> {
        self.text.get(..pos)?.chars().next_back()
    }

    fn single(&self, node: &Node, pos: usize) -> Option<usize> {
        let c = self.char_at(pos)?;
        let ok = match node {
            Node::Char(want, false) => c == *want,
            Node::Char(want, true) => fold(c) == fold(*want),
            Node::Any(dotall) => *dotall || c != '\n',
            Node::Class(class, icase) => class.matches(c, *icase),
            _ => return None,
        };
        ok.then(|| pos + c.len_utf8())
    }

    fn m(&mut self, node: &Node, pos: usize, k: Cont<'_>) -> bool {
        if self.steps == 0 {
            return false;
        }
        self.steps -= 1;
        match node {
            Node::Empty => k(pos, self),
            Node::Char(..) | Node::Any(_) | Node::Class(..) => match self.single(node, pos) {
                Some(next) => k(next, self),
                None => false,
            },
            Node::Seq(items) => self.seq(items, pos, k),
            Node::Alt(alts) => {
                for alt in alts {
                    if self.m(alt, pos, &mut *k) {
                        return true;
                    }
                }
                false
            }
            Node::Group { index, node } => match index {
                None => self.m(node, pos, k),
                Some(i) => {
                    let i = *i;
                    self.m(node, pos, &mut |p, cx| {
                        let old = cx.caps[i];
                        cx.caps[i] = Some((pos, p));
                        if k(p, cx) {
                            true
                        } else {
                            cx.caps[i] = old;
                            false
                        }
                    })
                }
            },
            Node::Repeat {
                node,
                min,
                max,
                kind,
            } => self.repeat(node, *min, *max, *kind, pos, k),
            Node::Look {
                ahead,
                negative,
                node,
            } => {
                let snapshot = self.caps.clone();
                let found = if *ahead {
                    self.m(node, pos, &mut |_, _| true)
                } else {
                    self.behind(node, pos)
                };
                if *negative {
                    self.caps = snapshot;
                    !found && k(pos, self)
                } else if found {
                    if k(pos, self) {
                        true
                    } else {
                        self.caps = snapshot;
                        false
                    }
                } else {
                    false
                }
            }
            Node::Atomic(node) => {
                let snapshot = self.caps.clone();
                let mut end = None;
                if self.m(node, pos, &mut |p, _| {
                    end = Some(p);
                    true
                }) && let Some(end) = end
                {
                    if k(end, self) {
                        return true;
                    }
                    self.caps = snapshot;
                }
                false
            }
            Node::Backref(i, icase) => self.backref(*i, *icase, pos, k),
            Node::NamedBackref(name, icase) => match self.re.names.get(name) {
                Some(&i) => self.backref(i, *icase, pos, k),
                None => false,
            },
            Node::Call(i) => self.call(*i, pos, k),
            Node::NamedCall(name) => match self.re.names.get(name) {
                Some(&i) => self.call(i, pos, k),
                None => false,
            },
            Node::Anchor(anchor) => self.anchor(*anchor, pos) && k(pos, self),
        }
    }

    fn call(&mut self, index: usize, pos: usize, k: Cont<'_>) -> bool {
        let Some(Some(group)) = self.re.groups.get(index) else {
            return false;
        };
        if self.calls > 200 {
            return false;
        }
        let group = Rc::clone(group);
        self.calls += 1;
        let matched = self.m(&group, pos, &mut |p, cx| {
            cx.calls -= 1;
            let ok = k(p, cx);
            cx.calls += 1;
            ok
        });
        self.calls -= 1;
        matched
    }

    fn backref(&mut self, index: usize, icase: bool, pos: usize, k: Cont<'_>) -> bool {
        let Some(Some((s, e))) = self.caps.get(index).copied() else {
            return false;
        };
        let want = &self.text[s..e];
        let Some(have) = self.text.get(pos..pos + want.len()) else {
            return false;
        };
        let same = if icase {
            want.chars().map(fold).eq(have.chars().map(fold))
        } else {
            want == have
        };
        same && k(pos + want.len(), self)
    }

    fn anchor(&self, anchor: Anchor, pos: usize) -> bool {
        let len = self.bytes.len();
        match anchor {
            // Oniguruma: not at the end of the text after a final newline.
            Anchor::Caret => pos == 0 || (self.bytes[pos - 1] == b'\n' && pos < len),
            Anchor::Dollar => pos == len || self.bytes[pos] == b'\n',
            Anchor::StartText => pos == 0 && self.text_start,
            Anchor::EndText => pos == len,
            Anchor::EndTextNewline => {
                pos == len || (pos + 1 == len && self.bytes[pos] == b'\n')
            }
            Anchor::Search => self.search == Some(pos),
            Anchor::WordBoundary | Anchor::NotWordBoundary => {
                let before = self.char_before(pos).is_some_and(is_word);
                let after = self.char_at(pos).is_some_and(is_word);
                (before != after) == (anchor == Anchor::WordBoundary)
            }
        }
    }

    fn seq(&mut self, items: &[Node], pos: usize, k: Cont<'_>) -> bool {
        match items.split_first() {
            None => k(pos, self),
            Some((first, [])) => self.m(first, pos, k),
            Some((first, rest)) => self.m(first, pos, &mut |p, cx| cx.seq(rest, p, &mut *k)),
        }
    }

    fn repeat(
        &mut self,
        node: &Node,
        min: u32,
        max: Option<u32>,
        kind: Kind,
        pos: usize,
        k: Cont<'_>,
    ) -> bool {
        if matches!(node, Node::Char(..) | Node::Any(_) | Node::Class(..)) {
            // One character per step: walk without recursing.
            let mut ends = vec![pos];
            let limit = max.map_or(usize::MAX, |m| usize::try_from(m).unwrap_or(usize::MAX));
            let mut at = pos;
            let need = usize::try_from(min).unwrap_or(usize::MAX);
            let lazy = kind == Kind::Lazy;
            if lazy {
                // Extend only as far as the continuation needs.
                let mut count = 0usize;
                loop {
                    if count >= need && k(at, self) {
                        return true;
                    }
                    if count >= limit {
                        return false;
                    }
                    match self.single(node, at) {
                        Some(next) => {
                            at = next;
                            count += 1;
                        }
                        None => return false,
                    }
                    if self.steps == 0 {
                        return false;
                    }
                    self.steps -= 1;
                }
            }
            while ends.len() - 1 < limit {
                match self.single(node, at) {
                    Some(next) => {
                        at = next;
                        ends.push(at);
                    }
                    None => break,
                }
            }
            self.steps = self.steps.saturating_sub(u64::try_from(ends.len()).unwrap_or(0));
            if ends.len() - 1 < need {
                return false;
            }
            if kind == Kind::Possessive {
                return k(at, self);
            }
            for &end in ends[need..].iter().rev() {
                if k(end, self) {
                    return true;
                }
                if self.steps == 0 {
                    return false;
                }
            }
            return false;
        }
        self.repeat_general(node, min, max, kind, 0, pos, k)
    }

    #[allow(clippy::too_many_arguments)]
    fn repeat_general(
        &mut self,
        node: &Node,
        min: u32,
        max: Option<u32>,
        kind: Kind,
        count: u32,
        pos: usize,
        k: Cont<'_>,
    ) -> bool {
        if kind == Kind::Possessive {
            let snapshot = self.caps.clone();
            let mut at = pos;
            let mut done = 0;
            while max.is_none_or(|m| done < m) {
                let mut next = None;
                if !self.m(node, at, &mut |p, _| {
                    next = Some(p);
                    true
                }) {
                    break;
                }
                match next {
                    Some(p) if p != at => {
                        at = p;
                        done += 1;
                    }
                    _ => break,
                }
            }
            if done >= min && k(at, self) {
                return true;
            }
            self.caps = snapshot;
            return false;
        }
        let can_more = max.is_none_or(|m| count < m);
        let try_more = |cx: &mut Cx<'_>, k: Cont<'_>| {
            can_more
                && cx.m(node, pos, &mut |p, cx| {
                    if p == pos && count >= min {
                        return false;
                    }
                    cx.repeat_general(node, min, max, kind, count + 1, p, &mut *k)
                })
        };
        if kind == Kind::Lazy {
            if count >= min && k(pos, self) {
                return true;
            }
            try_more(self, k)
        } else {
            if try_more(self, &mut *k) {
                return true;
            }
            count >= min && k(pos, self)
        }
    }

    fn behind(&mut self, node: &Node, pos: usize) -> bool {
        let (min, max) = width(node);
        let mut start = pos;
        let mut back = 0usize;
        loop {
            if back >= min {
                let matched = self.m(node, start, &mut |p, _| p == pos);
                if matched {
                    return true;
                }
            }
            if max.is_some_and(|m| back >= m) {
                return false;
            }
            match self.char_before(start) {
                Some(c) => {
                    start -= c.len_utf8();
                    back += 1;
                }
                None => return false,
            }
        }
    }
}

/// The fewest and most characters `node` matches; none for unbounded.
fn width(node: &Node) -> (usize, Option<usize>) {
    match node {
        Node::Char(..) | Node::Any(_) | Node::Class(..) => (1, Some(1)),
        Node::Seq(items) => items.iter().fold((0, Some(0)), |(lo, hi), n| {
            let (l, h) = width(n);
            (lo + l, hi.zip(h).map(|(a, b)| a + b))
        }),
        Node::Alt(alts) => alts.iter().map(width).fold((usize::MAX, Some(0)), |(lo, hi), (l, h)| {
            (lo.min(l), hi.zip(h).map(|(a, b)| a.max(b)))
        }),
        Node::Repeat { node, min, max, .. } => {
            let (l, h) = width(node);
            let min = l * usize::try_from(*min).unwrap_or(0);
            let max = match (h, max) {
                (Some(h), Some(m)) => Some(h * usize::try_from(*m).unwrap_or(0)),
                _ => None,
            };
            (min, max)
        }
        Node::Group { node, .. } => width(node),
        Node::Atomic(node) => width(node),
        Node::Backref(..) | Node::NamedBackref(..) | Node::Call(_) | Node::NamedCall(_) => (0, None),
        Node::Empty | Node::Look { .. } | Node::Anchor(_) => (0, Some(0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(pattern: &str, text: &str) -> Option<(usize, usize)> {
        let re = Regex::new(pattern).unwrap();
        let mut steps = 1_000_000;
        re.find_at(text, 0, None, true, &mut steps).and_then(|c| c[0])
    }

    #[test]
    fn literals_classes_and_quantifiers() {
        assert_eq!(find("a+b", "xaab"), Some((1, 4)));
        assert_eq!(find("[a-c]{2,3}", "zabcd"), Some((1, 4)));
        assert_eq!(find("a*?b", "aab"), Some((0, 3)));
        assert_eq!(find("(?i)HeLLo", "say hello"), Some((4, 9)));
        assert_eq!(find("\\bfoo\\b", "a foo b"), Some((2, 5)));
    }

    #[test]
    fn lookaround_and_backrefs() {
        assert_eq!(find("foo(?=bar)", "foobar"), Some((0, 3)));
        assert_eq!(find("foo(?!bar)", "foobar foobaz"), Some((7, 10)));
        assert_eq!(find("(?<=a)b", "cb ab"), Some((4, 5)));
        assert_eq!(find("(?<!a)b", "ab cb"), Some((4, 5)));
        assert_eq!(find("(['\"]).*?\\1", "x 'a' y"), Some((2, 5)));
    }

    #[test]
    fn atomic_possessive_and_calls() {
        assert_eq!(find("(?>a+)b", "aaab"), Some((0, 4)));
        assert_eq!(find("a++a", "aaa"), None);
        assert_eq!(find("(?<p>\\((?:[^()]|\\g<p>)*\\))", "x(a(b)c)y"), Some((1, 8)));
    }

    #[test]
    fn anchors_and_search_position() {
        let re = Regex::new("(^|\\G)ab").unwrap();
        let mut steps = 1000;
        assert!(re.find_at("ab", 0, None, true, &mut steps).is_some());
        assert!(re.find_at("xab", 0, None, true, &mut steps).is_none());
        assert!(re.find_at("xab", 0, Some(1), true, &mut steps).is_some());
        assert_eq!(find("a$", "a\n"), Some((0, 1)));
    }

    #[test]
    fn classes_with_intersection_and_posix() {
        assert_eq!(find("[a-z&&[^aeiou]]+", "aebcd"), Some((2, 5)));
        assert_eq!(find("[[:digit:]]+", "ab12"), Some((2, 4)));
        assert_eq!(find("\\h+", "xyzBEEFq"), Some((3, 7)));
    }
}
