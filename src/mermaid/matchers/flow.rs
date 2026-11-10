//! The flowchart.

use super::super::engine::Lexer;
use super::is_dash_word;
use crate::classes::{is_letter, is_space, is_word};
use crate::rules::Chars;

/// `RB|BT|RL|TD|LR`.
pub(super) const FLOW_DIRECTIONS: &[&[u8]] = &[b"RB", b"BT", b"RL", b"TD", b"LR"];

/// The shapes that open a node, in the grammar's order:
/// `\(\[|\[\[|\[\(?|\(+|[>{]|\(\(`.
/// `\(+` takes every `(`, so `\(\(` is never reached.
const SHAPES: &[&[u8]] = &[b"([", b"[[", b"[(", b"[", b"(", b">", b"{"];

/// The shapes that close a node, in the grammar's order:
/// `]\)|]]|\)]|]|\)+|}|\)\)`.
/// `\)+` takes every `)`, so `\)\)` is never reached.
const SHAPE_ENDS: &[&[u8]] = &[b"])", b"]]", b")]", b"]", b")", b"}"];

/// ``[!-'*-/:<-?\[-^`\w\s]``: the text of a link.
/// It holds `-`, `.`, `=` and the `"`.
fn is_link_text(c: char) -> bool {
    matches!(c, '!'..='\'' | '*'..='/' | ':' | '<'..='?' | '['..='^' | '`')
        || is_word(c)
        || is_space(c)
}

/// `[!#-'*+,./:;<>?\\^_\w\s]`: the text of a node or a link's label.
/// It holds neither `-` nor `=`.
fn is_label(c: char) -> bool {
    matches!(
        c,
        '!' | '#'
            ..='\'' | '*' | '+' | ',' | '.' | '/' | ':' | ';' | '<' | '>' | '?' | '\\' | '^' | '_'
    ) || is_word(c)
        || is_space(c)
}

/// ``[!#-'*-/:<-?\\^`\w\s]``: the title of a subgraph in brackets.
/// It holds neither `"` nor `]`.
fn is_subgraph_title(c: char) -> bool {
    matches!(c, '!' | '#'..='\'' | '*'..='/' | ':' | '<'..='?' | '\\' | '^' | '`')
        || is_word(c)
        || is_space(c)
}

/// The part of a click handler after its name.
#[derive(Clone, Copy, Debug)]
struct ClickRest {
    /// Where the name's spaces end, and the callback starts.
    spaces: usize,
    /// Where the callback ends, when it is taken.
    callback: Option<usize>,
    /// Where the tooltip starts.
    tip: usize,
}

/// `[<ox]` under `(?i)`: the tail of an arrow.
fn is_tail(b: u8) -> bool {
    matches!(b, b'<' | b'o' | b'O' | b'x' | b'X')
}

/// `[>ox]` under `(?i)`: the head of an arrow.
fn is_head(b: u8) -> bool {
    matches!(b, b'>' | b'o' | b'O' | b'x' | b'X')
}

/// The ends from `top` down to `low`, as a greedy run gives them back.
fn descending(low: usize, top: Option<usize>) -> impl Iterator<Item = usize> {
    top.into_iter().flat_map(move |top| (low..=top).rev())
}

impl Lexer<'_> {
    /// `(?i)^\s*(subgraph)\s+(\w+)(\[)("?[…]*"?)(])`: a subgraph with a title
    /// in brackets.
    /// The title's class holds no `"` and no `]`.
    /// So each `"?` takes a quote when it can, and giving one back never
    /// lets `]` follow.
    pub(super) fn subgraph_titled(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"subgraph")?;
        let id = self.ws1(word)?;
        let id_end = self.class_run1(id, is_word)?;
        let open = self.lit(id_end, b"[")?;
        let mut title = open + usize::from(self.at(open) == b'"');
        title = self.class_run(title, is_subgraph_title);
        title += usize::from(self.at(title) == b'"');
        let end = self.lit(title, b"]")?;
        for (group, from, to) in [
            (1, start, word),
            (2, id, id_end),
            (3, id_end, open),
            (4, open, title),
            (5, title, end),
        ] {
            self.capture(group, from, to);
        }
        Some(end)
    }

    /// `^\s*(subgraph)\s+([ 0-9<>\p{L}]+)`: a subgraph and its title.
    /// The title's class holds the space, so the spaces before it can give
    /// one back.
    pub(super) fn subgraph(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.lit(start, b"subgraph")?;
        let (title, end) = self.spaced_run(word, 1, |c| {
            matches!(c, ' ' | '0'..='9' | '<' | '>') || is_letter(c)
        })?;
        self.capture(1, start, word);
        self.capture(2, title, end);
        Some(end)
    }

    /// `\b(end)\b`.
    pub(super) fn end_word(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"end")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(?:(?!--|==)[-\w])+` from `p`: the greedy end of a node's name,
    /// which stops before `--`.
    /// No `=` is a name character, so `(?!==)` never stops it.
    fn node_name(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut q = p;
        while let Some((c, end)) = self.ch(q)
            && is_dash_word(c)
            && !self.starts(q, b"--")
        {
            q = end;
        }
        self.spend(q - p);
        (q > p).then_some(q)
    }

    /// `(?i)\b((?:(?!--|==)[-\w])+\b\s*)(SHAPE)`: a node and the shape that
    /// opens its text.
    /// A name given back leaves a name character where `\s*` and the shape
    /// must start.
    /// So only the greedy end can match.
    pub(super) fn node(&mut self, p: usize) -> Option<usize> {
        let name = self.node_name(p)?;
        if !self.boundary(name) {
            return None;
        }
        let shape = self.ws(name);
        let mut end = SHAPES.iter().find_map(|s| self.lit(shape, s))?;
        if self.at(shape) == b'(' && end == shape + 1 {
            end = self.skip(shape, |b| b == b'(');
        }
        self.capture(1, p, shape);
        self.capture(2, shape, end);
        Some(end)
    }

    /// `(?i)\b((?:(?!--|==)[-\w])+\b\s*)`: a node with no shape.
    /// Where `\b` fails at the greedy end, the name gives back one character
    /// at a time until it holds.
    pub(super) fn node_text(&mut self, p: usize) -> Option<usize> {
        let mut name = self.node_name(p)?;
        while !self.boundary(name) {
            name = self.char_start_before(name);
            if name <= p {
                return None;
            }
        }
        let end = self.ws(name);
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?i)(]\)|]]|\)]|]|\)+|}|\)\))`: the shape that closes a node's text.
    pub(super) fn node_end(&mut self, p: usize) -> Option<usize> {
        let mut end = SHAPE_ENDS.iter().find_map(|s| self.lit(p, s))?;
        if self.at(p) == b')' && end == p + 1 {
            end = self.skip(p, |b| b == b')');
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `\s*(")`.
    pub(super) fn quote_open(&mut self, p: usize) -> Option<usize> {
        let quote = self.ws(p);
        self.group_one(quote, b"\"")
    }

    /// `([^"]*)`: text up to a quote, the line feed included.
    pub(super) fn unquoted(&mut self, p: usize) -> usize {
        let end = self.class_run(p, |c| c != '"');
        self.capture(1, p, end);
        end
    }

    /// `(?i)\s*([!#-'*+,./:;<>?\\^_\w\s]+)`: a node's text or a link's label.
    pub(super) fn label(&mut self, p: usize) -> Option<usize> {
        let (start, end) = self.spaced_run(p, 0, is_label)?;
        self.capture(1, start, end);
        Some(end)
    }

    /// `b{least,most}` at `p`, greedy: the end of the run.
    fn counted(&self, p: usize, b: u8, least: usize, most: usize) -> Option<usize> {
        let mut q = p;
        while q - p < most && self.at(q) == b {
            q += 1;
        }
        (q - p >= least).then_some(q)
    }

    /// `-?\.{1,4}-` at `p`.
    /// Giving back a dot leaves a dot where `-` must be.
    /// Skipping the first `-` leaves it where a dot must be.
    fn dotted_line(&self, p: usize) -> Option<usize> {
        let dots = p + usize::from(self.at(p) == b'-');
        let end = self.counted(dots, b'.', 1, 4)?;
        self.lit(end, b"-")
    }

    /// `(?i)\s*((?:-?\.{1,4}-|-{2,5}|={2,5})[>ox]?\|)`: a link with its label
    /// between bars.
    /// At most one of the three lines matches at a place: `-.` or `.`, `--`
    /// and `==` start them.
    /// Giving back a `-` or `=` leaves one where `[>ox]?\|` must start.
    pub(super) fn piped_link(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let line = self
            .dotted_line(start)
            .or_else(|| self.counted(start, b'-', 2, 5))
            .or_else(|| self.counted(start, b'=', 2, 5))?;
        let head = line + usize::from(is_head(self.at(line)));
        let end = self.lit(head, b"|")?;
        self.capture(1, start, end);
        Some(end)
    }

    /// `(?i)\s*([<ox]?(?:-?\.{1,4}-|-{1,4}|={1,4})[>ox]?)`: a link.
    /// Nothing follows it, so the first line that matches wins.
    /// Skipping the tail leaves it where every line must start, so the tail
    /// is taken when it is there.
    pub(super) fn link(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let from = start + usize::from(is_tail(self.at(start)));
        let line = self
            .dotted_line(from)
            .or_else(|| self.counted(from, b'-', 1, 4))
            .or_else(|| self.counted(from, b'=', 1, 4))?;
        let end = line + usize::from(is_head(self.at(line)));
        self.capture(1, start, end);
        Some(end)
    }

    /// The end of `(?:-{2,5}|={2,5}|\.{1,3}-|\.-)[>ox]?` at `p`, the closing
    /// line of a link with text.
    /// Nothing follows it, so the first alternative that matches wins, at
    /// its greedy end.
    fn close_line(&self, p: usize) -> Option<usize> {
        let line = self
            .counted(p, b'-', 2, 5)
            .or_else(|| self.counted(p, b'=', 2, 5))
            .or_else(|| {
                self.counted(p, b'.', 1, 3)
                    .and_then(|end| self.lit(end, b"-"))
            })
            .or_else(|| self.lit(p, b".-"))?;
        Some(line + usize::from(is_head(self.at(line))))
    }

    /// The text of a link from `open` and the closing line after it: the
    /// text's end and the line's end.
    ///
    /// The text is ``(?:(?!--|==)[!-'*-/:<-?\[-^`\w\s])*``, which stops before
    /// `--` and `==`.
    /// Its class holds `-`, `.` and `=`.
    /// So it gives back one character at a time until a closing line
    /// follows.
    fn link_text(&self, open: usize) -> Option<(usize, usize)> {
        let mut text = open;
        while let Some((c, end)) = self.ch(text)
            && is_link_text(c)
            && !self.starts(text, b"--")
            && !self.starts(text, b"==")
        {
            text = end;
        }
        self.spend(text - open);
        loop {
            if let Some(end) = self.close_line(text) {
                return Some((text, end));
            }
            if text <= open {
                return None;
            }
            text = self.char_start_before(text);
        }
    }

    /// `(?i)\s*([<ox]?OPEN)(TEXT)(CLOSE[>ox]?)`: a link with its text between
    /// two lines, as in `-- text -->`.
    ///
    /// The opening line is `-{2,5}|={2,5}|-\.{1,3}|-\.`.
    /// Each of its ends is tried in turn, from the greedy one down, and
    /// [`Lexer::link_text`] searches the rest after it.
    /// `-\.` is the shortest end of `-\.{1,3}`, so it adds nothing.
    /// Skipping the tail leaves it where every opening line must start, so
    /// the tail is taken when it is there.
    pub(super) fn texted_link(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let from = start + usize::from(is_tail(self.at(start)));
        let dots = self
            .lit(from, b"-")
            .and_then(|dash| self.counted(dash, b'.', 1, 3));
        let (open, (text, end)) = descending(from + 2, self.counted(from, b'-', 2, 5))
            .chain(descending(from + 2, self.counted(from, b'=', 2, 5)))
            .chain(descending(from + 2, dots))
            .find_map(|open| Some((open, self.link_text(open)?)))?;
        self.capture(1, start, open);
        self.capture(2, open, text);
        self.capture(3, text, end);
        Some(end)
    }

    /// `\s+\b(C+)` from `p`, for a class `C` whose first character must be a
    /// word character: the span of the group.
    /// The spaces are greedy; `\b` holds after them only before a word
    /// character, and inside them never.
    fn word_led(&self, p: usize, class: impl Fn(char) -> bool) -> Option<(usize, usize)> {
        let start = self.ws1(p)?;
        self.word_at(start)?;
        Some((start, self.class_run(start, class)))
    }

    /// `(?i)\s*(class)\s+\b([-,\w]+)\s+\b(\w+)\b`: classes given to nodes.
    /// Giving back the names' run leaves a name character where `\s+` must
    /// start.
    pub(super) fn class_statement(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"class")?;
        let (names, names_end) = self.word_led(word, |c| c == '-' || c == ',' || is_word(c))?;
        let (class, end) = self.word_led(names_end, is_word)?;
        self.capture(1, start, word);
        self.capture(2, names, names_end);
        self.capture(3, class, end);
        Some(end)
    }

    /// `(?i)\s*(classDef)\s+\b(\w+)\b\s+\b([-#,:;\w]+)`: a class definition.
    pub(super) fn class_def(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"classDef")?;
        let (name, name_end) = self.word_led(word, is_word)?;
        let (style, end) = self.word_led(name_end, |c| {
            matches!(c, '-' | '#' | ',' | ':' | ';') || is_word(c)
        })?;
        self.capture(1, start, word);
        self.capture(2, name, name_end);
        self.capture(3, style, end);
        Some(end)
    }

    /// `(?i)\s*(click)\s+\b([-\w]+\b\s*)(\b\w+\b)?\s("*.*")`: a click
    /// handler.
    ///
    /// This backtracks for real.
    /// - The name gives back characters until `\b` holds after it.
    /// - Its spaces give back one at a time.
    /// - At each place the callback is taken when a white space follows it, and
    ///   skipped otherwise.
    /// - The tooltip, `"*.*`, matches anything after one white space.
    ///
    /// So `click A "tip"` takes `A` and gives the space to the `\s`.
    pub(super) fn click(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"click")?;
        let (name, greedy) = self.word_led(word, is_dash_word)?;
        let mut name_end = greedy;
        let ClickRest {
            spaces,
            callback,
            tip,
        } = loop {
            if self.boundary(name_end)
                && let Some(found) = self.click_rest(name_end)
            {
                break found;
            }
            name_end = self.char_start_before(name_end);
            if name_end <= name {
                return None;
            }
        };
        self.capture(1, start, word);
        self.capture(2, name, spaces);
        if let Some(to) = callback {
            self.capture(3, spaces, to);
        }
        let end = tip.max(self.lim());
        self.capture(4, tip, end);
        Some(end)
    }

    /// The part of a click handler after its name, from `p`.
    /// It holds the end of the name's spaces, the callback's span and where the
    /// tooltip starts.
    fn click_rest(&self, p: usize) -> Option<ClickRest> {
        let mut spaces = self.ws(p);
        loop {
            if self.boundary(spaces) && self.word_at(spaces).is_some() {
                let callback = self.word_end(spaces);
                let gap = self.space_at(callback);
                if gap > 0 {
                    return Some(ClickRest {
                        spaces,
                        callback: Some(callback),
                        tip: callback + gap,
                    });
                }
            }
            let gap = self.space_at(spaces);
            if gap > 0 {
                return Some(ClickRest {
                    spaces,
                    callback: None,
                    tip: spaces + gap,
                });
            }
            if spaces <= p {
                return None;
            }
            spaces = self.char_start_before(spaces);
        }
    }

    /// `\s*(@\{)`.
    pub(super) fn attributes_open(&mut self, p: usize) -> Option<usize> {
        let open = self.ws(p);
        self.group_one(open, b"@{")
    }

    /// `(?i)\s*(NAME\s*:)([^,}]*)(,)?`: one attribute of a node.
    /// The value can take the line feed.
    pub(super) fn attribute(&mut self, p: usize, name: &[u8]) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, name)?;
        let colon = self.lit(self.ws(word), b":")?;
        let value = self.class_run(colon, |c| c != ',' && c != '}');
        let end = self.lit(value, b",").unwrap_or(value);
        self.capture(1, start, colon);
        self.capture(2, colon, value);
        self.capture(3, value, end);
        Some(end)
    }
}
