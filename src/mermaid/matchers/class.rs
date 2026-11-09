//! The class diagram.

use super::super::engine::Lexer;
use super::is_name;
use crate::classes::{is_digit, is_word};
use crate::rules::Chars;

/// Which of the grammar's three `class` heads a pattern is.
#[derive(Clone, Copy, Debug)]
pub(super) enum Head {
    /// Inside a namespace:
    /// ``(?i)(class)\s+(?:(`)([^`]+)(`)|NAME(~)?…(~)?)\s*(\{)``.
    /// Only a plain name takes generics.
    InNamespace,
    /// At the top: ``(?i)(class)\s+(?:(`)([^`]+)(`)|NAME)(~)?…(~)?\s?(\{)``.
    Body,
    /// At the top, with no body: the same without `\s?(\{)`.
    Bare,
}

/// What may follow a stereotype's `>>`.
#[derive(Clone, Copy, Debug)]
pub(super) enum After {
    /// `\s?([-.\w]+)?`, in a class body.
    Name,
    /// ``\s?(?:(`)([^`]+)(`)|([-.\w]+))?``, at the top.
    Class,
}

/// The arrows of a relation, in the grammar's order.
/// The grammar's `--\*?` tries `--*` first, which an earlier alternative
/// already tried, so it stands here as `--`.
const ARROWS: &[&[u8]] = &[
    b"--o", b"--*", b"<--", b"-->", b"<..", b"..>", b"<|..", b"..|>", b"<|--", b"--|>", b"--",
    b"..", b"*--", b"o--",
];

/// `[-#+~]`: the visibility a member may start with.
fn is_visibility(b: u8) -> bool {
    matches!(b, b'-' | b'#' | b'+' | b'~')
}

/// `[.\w]`.
fn is_dotted(c: char) -> bool {
    c == '.' || is_word(c)
}

/// Where the parts of a relation after its first name sit.
#[derive(Clone, Copy, Debug)]
struct Relation {
    /// The first cardinality.
    from: Option<(usize, usize)>,
    /// The arrow.
    arrow: (usize, usize),
    /// The second cardinality.
    to: Option<(usize, usize)>,
    /// The second name, and whether it sits in backticks.
    name: (usize, usize, bool),
    /// The colon.
    colon: Option<(usize, usize)>,
    /// The label.
    label: (usize, usize),
}

impl Lexer<'_> {
    /// ``(?:(`)([^`]+)(`)|([-.\w]+))`` at `p`: a name in backticks, or a
    /// plain name.
    /// It returns the longest end and whether the name sits in backticks.
    fn quoted_name(&self, p: usize) -> Option<(usize, bool)> {
        if self.at(p) == b'`' {
            let close = self.class_run1(p + 1, |c| c != '`')?;
            return (self.at(close) == b'`' && close < self.lim()).then_some((close + 1, true));
        }
        Some((self.class_run1(p, is_name)?, false))
    }

    /// Records the groups of [`Lexer::quoted_name`], which count from `group`.
    fn name_groups(&mut self, group: usize, p: usize, end: usize, quoted: bool) {
        if quoted {
            self.cap(group, p, p + 1);
            self.cap(group + 1, p + 1, end - 1);
            self.cap(group + 2, end - 1, end);
        } else {
            self.cap(group + 3, p, end);
        }
    }

    /// [`Lexer::quoted_name`] with its groups recorded.
    fn named(&mut self, group: usize, p: usize) -> Option<usize> {
        let (end, quoted) = self.quoted_name(p)?;
        self.name_groups(group, p, end, quoted);
        Some(end)
    }

    /// `(?i)^\s*(note)\s+(for)\s+NAME\s+("[^"]*")\s*$`: a note for a class.
    pub(super) fn class_note(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let note = self.ci(start, b"note")?;
        let word = self.ws1(note)?;
        let r#for = self.ci(word, b"for")?;
        let name = self.ws1(r#for)?;
        let (name_end, quoted) = self.quoted_name(name)?;
        let text = self.ws1(name_end)?;
        let text_end = self.plain_string(text)?;
        let end = self.ws(text_end);
        if !self.at_eol(end) {
            return None;
        }
        self.cap(1, start, note);
        self.cap(2, word, r#for);
        self.name_groups(3, name, name_end, quoted);
        self.cap(7, text, text_end);
        Some(end)
    }

    /// `(?i)^\s*(namespace)\s+([-.\w]+)\s*(\{)`.
    pub(super) fn namespace(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"namespace")?;
        let name = self.ws1(word)?;
        let name_end = self.class_run1(name, is_name)?;
        let brace = self.ws(name_end);
        let end = self.lit(brace, b"{")?;
        self.cap(1, start, word);
        self.cap(2, name, name_end);
        self.cap(3, brace, end);
        Some(end)
    }

    /// `(?i)^\s*(})`.
    pub(super) fn namespace_end(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let brace = self.ws(p);
        self.group_one(brace, b"}")
    }

    /// `(~)?([-.\w]+)?(~)?` from `p`, with groups `group` to `group + 2`:
    /// generics.
    /// Each part is greedy.
    /// Giving back would leave the matcher at a `~` or a name character.
    /// Nothing after generics in the grammar starts with one of those.
    fn generics(&mut self, group: usize, p: usize) -> usize {
        let mut q = p;
        if self.at(q) == b'~' {
            self.cap(group, q, q + 1);
            q += 1;
        }
        if let Some(end) = self.class_run1(q, is_name) {
            self.cap(group + 1, q, end);
            q = end;
        }
        if self.at(q) == b'~' {
            self.cap(group + 2, q, q + 1);
            q += 1;
        }
        q
    }

    /// A `class` head (see [`Head`]).
    /// Group 1 is `class`, groups 2 to 5 the name, 6 to 8 the generics and 9
    /// the brace.
    /// No `\b` stands before `class`, so it can end another word.
    pub(super) fn class_head(&mut self, p: usize, head: Head) -> Option<usize> {
        let word = self.ci(p, b"class")?;
        let name = self.ws1(word)?;
        let (name_end, quoted) = self.quoted_name(name)?;
        let after = match (head, quoted) {
            (Head::InNamespace, true) => name_end,
            (Head::InNamespace | Head::Body | Head::Bare, _) => self.generics(6, name_end),
        };
        let end = match head {
            Head::InNamespace => self.group_at(9, self.ws(after), b"{")?,
            Head::Body => self.group_at(9, self.ws01(after), b"{")?,
            Head::Bare => after,
        };
        self.cap(1, p, word);
        self.name_groups(2, name, name_end, quoted);
        Some(end)
    }

    /// `(literal)` at `p`, with the literal the group `group`.
    fn group_at(&mut self, group: usize, p: usize, literal: &[u8]) -> Option<usize> {
        let end = self.lit(p, literal)?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `([-#+~])?([-.\w]+)` at `p` and then `next`, with groups `group` and
    /// `group + 1`: a member's visibility and name.
    /// A `-` is a visibility first.
    /// When no name follows it, the `-` is the name.
    /// When the name follows but `next` fails, giving back reaches the same
    /// end, so `next` fails there too.
    fn member(
        &mut self,
        group: usize,
        p: usize,
        next: impl Fn(&mut Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        if is_visibility(self.at(p))
            && let Some(name) = self.class_run1(p + 1, is_name)
        {
            let end = next(self, name)?;
            self.cap(group, p, p + 1);
            self.cap(group + 1, p + 1, name);
            return Some(end);
        }
        let name = self.class_run1(p, is_name)?;
        let end = next(self, name)?;
        self.cap(group + 1, p, name);
        Some(end)
    }

    /// `(?i)\s([-#+~])?([-.\w]+)(\()`: a method's head in a class body.
    pub(super) fn method_head(&mut self, p: usize) -> Option<usize> {
        let space = self.space_at(p);
        if space == 0 {
            return None;
        }
        self.member(1, p + space, |lexer, name| lexer.group_at(3, name, b"("))
    }

    /// `(\))([$*]{0,2})\s?([-.\w]+)?(~)?([-.\w]+)?(~)?$` from `p`, with
    /// groups from `group`: the end of a method and its return type.
    /// Every part is greedy.
    /// Giving any of them back leaves the matcher at a character that is no
    /// end of the line.
    /// So `$` holds after the greedy parts or nowhere.
    fn method_tail(&mut self, group: usize, p: usize) -> Option<usize> {
        let close = self.lit(p, b")")?;
        let mut marks = close;
        while marks - close < 2 && matches!(self.at(marks), b'$' | b'*') {
            marks += 1;
        }
        let returns = self.ws01(marks);
        let returns_end = self.class_run(returns, is_name);
        let end = self.generics(group + 3, returns_end);
        if !self.at_eol(end) {
            return None;
        }
        self.cap(group, p, close);
        self.cap(group + 1, close, marks);
        self.cap(group + 2, returns, returns_end);
        Some(end)
    }

    /// `(?i)(\))([$*]{0,2})\s?([-.\w]+)?(~)?([-.\w]+)?(~)?$`.
    pub(super) fn method_end(&mut self, p: usize) -> Option<usize> {
        self.method_tail(1, p)
    }

    /// `(?i)\s*,?\s*([-.\w]+)?(~)?([-.\w]+)?(~)?\s?([-.\w]+)?`: a parameter.
    /// Every part is optional, so it can match empty.
    pub(super) fn parameter(&mut self, p: usize) -> usize {
        let mut q = self.ws(p);
        q += usize::from(self.at(q) == b',' && q < self.lim());
        q = self.ws(q);
        let name = self.class_run(q, is_name);
        self.cap(1, q, name);
        q = self.generics(2, name);
        q = self.ws01(q);
        let last = self.class_run(q, is_name);
        self.cap(5, q, last);
        last
    }

    /// `(?i)^\s*(map)(~)([^,~]+)(,)([^~]+)(~)\s*$`.
    /// `[^~]+` can take the line feed, but a `~` must follow it.
    pub(super) fn map_field(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"map")?;
        let open = self.lit(word, b"~")?;
        let key = self.class_run1(open, |c| c != ',' && c != '~')?;
        let comma = self.lit(key, b",")?;
        let value = self.class_run1(comma, |c| c != '~')?;
        if value > self.lim() {
            return None;
        }
        let close = self.lit(value, b"~")?;
        let end = self.ws(close);
        if !self.at_eol(end) {
            return None;
        }
        for (group, from, to) in [
            (1, start, word),
            (2, word, open),
            (3, open, key),
            (4, key, comma),
            (5, comma, value),
            (6, value, close),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// The ends `(?:\[]\*?|\*|\*?\*?)` can take at `p`, in the order the
    /// alternation tries them.
    /// That is `[]` with and without a star, one star, then none, one or two
    /// stars, as the lazy `\*?\*?` tries them.
    fn type_prefixes(&self, p: usize) -> [Option<usize>; 6] {
        let brackets = self.starts(p, b"[]");
        let star = |q: usize| self.at(q) == b'*';
        [
            (brackets && star(p + 2)).then_some(p + 3),
            brackets.then_some(p + 2),
            star(p).then_some(p + 1),
            Some(p),
            star(p).then_some(p + 1),
            (star(p) && star(p + 1)).then_some(p + 2),
        ]
    }

    /// `(?i)\s([-#+~])?\s*(TYPE)\s+([.\w]+)\s*$`: a field with its type first.
    ///
    /// The type is `(?:\[]\*?|\*|\*?\*?)[.\w]+(?:\.[.\w]+)*(?:\[])?`.
    /// - Its `[.\w]+` takes every dot, so `(?:\.[.\w]+)*` takes nothing.
    /// - Giving the run back leaves the matcher at a character `\s+` cannot
    ///   take.
    /// - No visibility can start the type, so the visibility is never given
    ///   back.
    pub(super) fn typed_field(&mut self, p: usize) -> Option<usize> {
        let space = self.space_at(p);
        if space == 0 {
            return None;
        }
        let mut q = p + space;
        let visibility = is_visibility(self.at(q)).then_some(q);
        q += usize::from(visibility.is_some());
        let start = self.ws(q);
        for prefix in self.type_prefixes(start).into_iter().flatten() {
            let Some(base) = self.class_run1(prefix, is_dotted) else {
                continue;
            };
            let ty = if self.starts(base, b"[]") {
                base + 2
            } else {
                base
            };
            let Some(name) = self.ws1(ty) else {
                continue;
            };
            let Some(name_end) = self.class_run1(name, is_dotted) else {
                continue;
            };
            let end = self.ws(name_end);
            if !self.at_eol(end) {
                continue;
            }
            if let Some(v) = visibility {
                self.cap(1, v, v + 1);
            }
            self.cap(2, start, ty);
            self.cap(3, name, name_end);
            return Some(end);
        }
        None
    }

    /// `([-#+~])?([-.\w]+)(~)?([-.\w]+)?(~)?\s([-.\w]+)?$` at `p`, with groups
    /// from `group`: a field with its type first and its name after one white
    /// space.
    /// The white space can be the line feed, and then the match ends past it.
    fn field_tail(&mut self, group: usize, p: usize) -> Option<usize> {
        self.member(group, p, |lexer, name| {
            let after = lexer.generics(group + 2, name);
            let gap = lexer.space_at(after);
            if gap == 0 {
                return None;
            }
            let field = after + gap;
            let end = lexer.class_run(field, is_name);
            if !lexer.at_eol(end) {
                return None;
            }
            lexer.cap(group + 5, field, end);
            Some(end)
        })
    }

    /// `(?i)\s([-#+~])?([-.\w]+)(~)?([-.\w]+)?(~)?\s([-.\w]+)?$`: a field in a
    /// class body.
    pub(super) fn field(&mut self, p: usize) -> Option<usize> {
        let space = self.space_at(p);
        if space == 0 {
            return None;
        }
        self.field_tail(1, p + space)
    }

    /// `(?i)(<<)([-\w]+)(>>)` and what [`After`] says follows it: a
    /// stereotype.
    pub(super) fn stereotype(&mut self, p: usize, after: After) -> Option<usize> {
        let open = self.lit(p, b"<<")?;
        let name = self.class_run1(open, |c| c == '-' || is_word(c))?;
        let close = self.lit(name, b">>")?;
        self.cap(1, p, open);
        self.cap(2, open, name);
        self.cap(3, name, close);
        let q = self.ws01(close);
        match after {
            After::Name => {
                let end = self.class_run(q, is_name);
                self.cap(4, q, end);
                Some(end)
            }
            After::Class => Some(self.named(4, q).unwrap_or(q)),
        }
    }

    /// Any character but the line feed, twice, from `p`: the grammar's `..`.
    pub(super) fn any_two(&self, p: usize) -> Option<usize> {
        let first = self.ch(p).filter(|&(c, _)| c != '\n')?.1;
        self.ch(first)
            .filter(|&(c, _)| c != '\n')
            .map(|(_, end)| end)
    }

    /// `"(?:\d+|\*|0..\d+|1..\d+|1..\*)"` at `p`: a cardinality.
    /// Each alternative is tried in turn with the closing quote.
    /// A greedy `\d+` given back leaves a digit where the quote must be.
    fn cardinality(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'"' {
            return None;
        }
        let q = p + 1;
        let closed = |end: Option<usize>| {
            end.filter(|&e| self.at(e) == b'"' && e < self.lim())
                .map(|e| e + 1)
        };
        let after = |digit: &[u8]| self.lit(q, digit).and_then(|d| self.any_two(d));
        closed(self.class_run1(q, is_digit))
            .or_else(|| closed(self.lit(q, b"*")))
            .or_else(|| closed(after(b"0").and_then(|a| self.class_run1(a, is_digit))))
            .or_else(|| closed(after(b"1").and_then(|a| self.class_run1(a, is_digit))))
            .or_else(|| closed(after(b"1").and_then(|a| self.lit(a, b"*"))))
    }

    /// The rest of a relation from `p`, after its first name.
    ///
    /// A cardinality the matcher skips leaves a `"`, and no arrow starts with
    /// one.
    /// The same holds for the second cardinality and the second name.
    /// So each cardinality matches when it can.
    /// The arrows are tried in turn: one that leaves no second name after it
    /// gives way to the next.
    /// The tail after the second name, `\s*(:)?\s*(.*)$`, always matches.
    fn relation_rest(&self, p: usize) -> Option<Relation> {
        let before = self.ws(p);
        let from = self.cardinality(before).map(|end| (before, end));
        let arrow = from.map_or(before, |(_, end)| self.ws(end));
        ARROWS.iter().find_map(|shape| {
            let arrow_end = self.ci(arrow, shape)?;
            let after = self.ws(arrow_end);
            let to = self.cardinality(after).map(|end| (after, end));
            let name = to.map_or(after, |(_, end)| self.ws(end));
            let (name_end, quoted) = self.quoted_name(name)?;
            let colon = self.ws(name_end);
            let colon_end = (self.at(colon) == b':' && colon < self.lim()).then_some(colon + 1);
            let label = self.ws(colon_end.unwrap_or(colon));
            Some(Relation {
                from,
                arrow: (arrow, arrow_end),
                to,
                name: (name, name_end, quoted),
                colon: colon_end.map(|end| (colon, end)),
                label: (label, label.max(self.lim())),
            })
        })
    }

    /// `(?i)NAME\s*(CARD)?\s*(ARROW)\s*(CARD)?\s*NAME\s*(:)?\s*(.*)$`: a
    /// relation between two classes.
    ///
    /// The first plain name takes `-` and `.`, which start arrows.
    /// So it gives back one character at a time until the rest matches, as
    /// in `A-->B`, where the name gives back `--`.
    /// The groups:
    /// - 1 to 4, the first name;
    /// - 5 to 7, the cardinalities and the arrow;
    /// - 8 to 11, the second name;
    /// - 12, the colon;
    /// - 13, the label.
    pub(super) fn relation(&mut self, p: usize) -> Option<usize> {
        let (longest, quoted) = self.quoted_name(p)?;
        let (name_end, rest) = if quoted {
            (longest, self.relation_rest(longest)?)
        } else {
            let mut end = longest;
            loop {
                if let Some(rest) = self.relation_rest(end) {
                    break (end, rest);
                }
                end = self.char_start_before(end);
                if end <= p {
                    return None;
                }
            }
        };
        self.name_groups(1, p, name_end, quoted);
        for (group, span) in [
            (5, rest.from),
            (6, Some(rest.arrow)),
            (7, rest.to),
            (12, rest.colon),
        ] {
            if let Some((from, to)) = span {
                self.cap(group, from, to);
            }
        }
        let (name, second_end, second_quoted) = rest.name;
        self.name_groups(8, name, second_end, second_quoted);
        self.cap(13, rest.label.0, rest.label.1);
        Some(rest.label.1)
    }

    /// `NAME\s?(:)\s` at `p`: a class name and the colon before one of its
    /// members, with groups 1 to 5.
    /// The plain name given back would leave a name character where `\s?(:)`
    /// must start.
    fn member_owner(&mut self, p: usize) -> Option<usize> {
        let (name_end, quoted) = self.quoted_name(p)?;
        let colon = self.ws01(name_end);
        let colon_end = self.lit(colon, b":")?;
        let space = self.space_at(colon_end);
        if space == 0 {
            return None;
        }
        self.name_groups(1, p, name_end, quoted);
        self.cap(5, colon, colon_end);
        Some(colon_end + space)
    }

    /// `(?i)NAME\s?(:)\s([-#+~])?([-.\w]+)(\()PARAMETER(\))…$`: a method added
    /// to a class from outside it.
    ///
    /// The parameter is `([-.\w]+)?(~)?([-.\w]+)?(~)?\s?([-.\w]+)?`, groups 9
    /// to 13.
    /// Each of its parts is greedy, and a part given back goes to the next
    /// one, which ends at the same place.
    pub(super) fn member_method(&mut self, p: usize) -> Option<usize> {
        let start = self.member_owner(p)?;
        self.member(6, start, |lexer, name| {
            let open = lexer.group_at(8, name, b"(")?;
            let first = lexer.class_run(open, is_name);
            let generic = lexer.generics(10, first);
            let last = lexer.ws01(generic);
            let last_end = lexer.class_run(last, is_name);
            let end = lexer.method_tail(14, last_end)?;
            lexer.cap(9, open, first);
            lexer.cap(13, last, last_end);
            Some(end)
        })
    }

    /// `(?i)NAME\s?(:)\s([-#+~])?([-.\w]+)(~)?([-.\w]+)?(~)?\s([-.\w]+)?$`: a
    /// field added to a class from outside it.
    pub(super) fn member_field(&mut self, p: usize) -> Option<usize> {
        let start = self.member_owner(p)?;
        self.field_tail(6, start)
    }
}
