//! The other diagrams: architecture, entity relationship, Gantt, Git graph,
//! mind map, pie, quadrant, requirement, user journey and XY charts.
//!
//! Most of their patterns end in optional parts or in a class that holds
//! white space, so their greedy reading is the match.
//! The doc comments name the few that give characters back.

use super::super::engine::Lexer;
use super::{is_dash_word, is_text};
use crate::classes::{is_digit, is_space, is_word};
use crate::rules::Chars;

/// What separates a title's keyword from its text.
#[derive(Clone, Copy, Debug)]
pub(super) enum Gap {
    /// `\s+`.
    Some,
    /// `\s*`.
    Any,
}

/// The value of a Git graph option.
#[derive(Clone, Copy, Debug)]
pub(super) enum GitValue {
    /// `"[^\n"]*"`.
    Quoted,
    /// `NORMAL|REVERSE|HIGHLIGHT`.
    Type,
    /// `"[!#-(*-/:-?\\^\w\s]*"`.
    Tag,
    /// `\d+`.
    Order,
}

/// `crit|done|active|after`: the tags of a Gantt task.
pub(super) const TASK_TAGS: &[&[u8]] = &[b"crit", b"done", b"active", b"after"];

/// `low|medium|high`.
pub(super) const RISKS: &[&[u8]] = &[b"low", b"medium", b"high"];

/// `analysis|inspection|test|demonstration`.
pub(super) const METHODS: &[&[u8]] = &[b"analysis", b"inspection", b"test", b"demonstration"];

/// The days `excludes` names, beside `[-,\d\s]`.
const DAYS: &[&[u8]] = &[
    b"monday",
    b"tuesday",
    b"wednesday",
    b"thursday",
    b"friday",
    b"saturday",
    b"sunday",
    b"weekends",
];

/// `millisecond|second|minute|hour|day|week|month`.
const UNITS: &[&[u8]] = &[
    b"millisecond",
    b"second",
    b"minute",
    b"hour",
    b"day",
    b"week",
    b"month",
];

/// `NORMAL|REVERSE|HIGHLIGHT`.
const COMMIT_TYPES: &[&[u8]] = &[b"NORMAL", b"REVERSE", b"HIGHLIGHT"];

/// The grammar's `(?:contain|…|trace)s`, written out as whole words.
/// The stems are distinct, so the first word that matches is the stem's.
const VERBS: &[&[u8]] = &[
    b"contains",
    b"copies",
    b"derives",
    b"satisfies",
    b"verifies",
    b"refines",
    b"traces",
];

/// The cardinalities at the left of an entity relationship, in the grammar's
/// order: `\|o|\|\||}o|}\||one or …|zero or …|many\([01]\)|only one|0\+|1\+?`.
const LEFT_ENDS: &[&[u8]] = &[
    b"|o",
    b"||",
    b"}o",
    b"}|",
    b"one or zero",
    b"one or more",
    b"one or many",
    b"zero or one",
    b"zero or more",
    b"zero or many",
    b"many(0)",
    b"many(1)",
    b"only one",
    b"0+",
    b"1+",
    b"1",
];

/// The cardinalities at the right of an entity relationship, in the
/// grammar's order: `o\||\|\||o\{|\|\{` and then the words of [`LEFT_ENDS`].
const RIGHT_ENDS: &[&[u8]] = &[
    b"o|",
    b"||",
    b"o{",
    b"|{",
    b"one or zero",
    b"one or more",
    b"one or many",
    b"zero or one",
    b"zero or more",
    b"zero or many",
    b"many(0)",
    b"many(1)",
    b"only one",
    b"0+",
    b"1+",
    b"1",
];

/// `[-%./\\\w]`: a Gantt axis format.
pub(super) fn is_axis_format(c: char) -> bool {
    matches!(c, '-' | '%' | '.' | '/' | '\\') || is_word(c)
}

/// `[!#-'*+,./:;<>?\\^_\w\s]`: a requirement's property.
pub(super) fn is_property(c: char) -> bool {
    matches!(
        c,
        '!' | '#'
            ..='\'' | '*' | '+' | ',' | '.' | '/' | ':' | ';' | '<' | '>' | '?' | '\\' | '^' | '_'
    ) || is_word(c)
        || is_space(c)
}

/// `[!-'*+,./:;<>?\\^_\w\s]`: an element's type, which may hold `"`.
pub(super) fn is_type_property(c: char) -> bool {
    c == '"' || is_property(c)
}

/// `[-\w\s]`.
fn is_dash_word_space(c: char) -> bool {
    is_dash_word(c) || is_space(c)
}

/// `[!#-(*-/:-?\\^\w]`: a word on an axis, or with `\s` the text of a tag.
fn is_tag_word(c: char) -> bool {
    matches!(c, '!' | '#'..='(' | '*'..='/' | ':'..='?' | '\\' | '^') || is_word(c)
}

/// `[!#-(*-/:-?\\^\w\s]`.
fn is_tag(c: char) -> bool {
    is_tag_word(c) || is_space(c)
}

/// `[!-$\&'*-/;-?\\^\w\s]`: a mind map class.
fn is_mindmap_class(c: char) -> bool {
    matches!(c, '!'..='$' | '&' | '\'' | '*'..='/' | ';'..='?' | '\\' | '^')
        || is_word(c)
        || is_space(c)
}

/// `[!-$\&'*-/:-?\\^\w\s]`: a mind map node's text, which may hold `:`.
fn is_mindmap_text(c: char) -> bool {
    c == ':' || is_mindmap_class(c)
}

/// `[!#-'*-/=?\\\w\s]`: a quadrant chart's text.
fn is_quadrant_text(c: char) -> bool {
    matches!(c, '!' | '#'..='\'' | '*'..='/' | '=' | '?' | '\\') || is_word(c) || is_space(c)
}

/// `[!"$-/<-?\\^\w\s]`: a journey task.
fn is_journey_task(c: char) -> bool {
    matches!(c, '!' | '"' | '$'..='/' | '<'..='?' | '\\' | '^') || is_word(c) || is_space(c)
}

/// `[-!#-(*+./:-?\\^\w\s]`: an XY chart category.
fn is_category(c: char) -> bool {
    matches!(c, '-' | '!' | '#'..='(' | '*' | '+' | '.' | '/' | ':'..='?' | '\\' | '^')
        || is_word(c)
        || is_space(c)
}

impl Lexer<'_> {
    /// `(?i)\s*(group|service)\s+([-\w]+)` and its optional parts: an
    /// architecture group or service.
    ///
    /// The parts are `\s*(\()?([-\w\s]+)?(:)?([-\w\s]+)?(\))?`, then
    /// `\s*(\[)?([-\w\s]+)?\s*(])?` and `\s*(in)?\s*([-\w]+)?`.
    /// All are optional and nothing follows, so each takes what it can.
    pub(super) fn architecture_item(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self
            .ci(start, b"group")
            .or_else(|| self.ci(start, b"service"))?;
        let name = self.ws1(word)?;
        let name_end = self.class_run1(name, is_dash_word)?;
        self.cap(1, start, word);
        self.cap(2, name, name_end);
        let mut q = self.ws(name_end);
        q = self.optional_lit(3, q, b"(");
        q = self.optional_run(4, q, is_dash_word_space);
        q = self.optional_lit(5, q, b":");
        q = self.optional_run(6, q, is_dash_word_space);
        q = self.optional_lit(7, q, b")");
        q = self.ws(q);
        q = self.optional_lit(8, q, b"[");
        q = self.optional_run(9, q, is_dash_word_space);
        q = self.ws(q);
        q = self.optional_lit(10, q, b"]");
        q = self.ws(q);
        if let Some(end) = self.ci(q, b"in") {
            self.cap(11, q, end);
            q = end;
        }
        q = self.ws(q);
        Some(self.optional_run(12, q, is_dash_word))
    }

    /// `(literal)?` at `p`, with the group `group`.
    fn optional_lit(&mut self, group: usize, p: usize, literal: &[u8]) -> usize {
        match self.lit(p, literal) {
            Some(end) => {
                self.cap(group, p, end);
                end
            }
            None => p,
        }
    }

    /// `(C+)?` at `p`, with the group `group`.
    fn optional_run(&mut self, group: usize, p: usize, class: impl Fn(char) -> bool) -> usize {
        let end = self.class_run(p, class);
        self.cap(group, p, end);
        end
    }

    /// `([-\w]+)\s*(\{)?\s*(group)?(})?` at `p`, with groups from `group`:
    /// one side of an architecture edge.
    fn edge_end(&mut self, group: usize, p: usize) -> Option<usize> {
        let name = self.class_run1(p, is_dash_word)?;
        self.cap(group, p, name);
        let mut q = self.ws(name);
        q = self.optional_lit(group + 1, q, b"{");
        q = self.ws(q);
        if let Some(end) = self.ci(q, b"group") {
            self.cap(group + 2, q, end);
            q = end;
        }
        Some(self.optional_lit(group + 3, q, b"}"))
    }

    /// `(?i)\s*SIDE\s*(:)\s*([BLRT])\s+(<?-->?)\s+([BLRT])\s*(:)\s*SIDE`: an
    /// architecture edge, where each `SIDE` is [`Lexer::edge_end`].
    ///
    /// The name, the brace, `group` and `}` cannot hold the colon.
    /// So every reading of the first side reaches the same colon, and the
    /// greedy one is the match.
    pub(super) fn architecture_edge(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let first = self.edge_end(1, start)?;
        let colon = self.ws(first);
        let colon_end = self.lit(colon, b":")?;
        let side = self.ws(colon_end);
        let side_end = self.one(side, |c| {
            matches!(c, 'B' | 'L' | 'R' | 'T' | 'b' | 'l' | 'r' | 't')
        })?;
        let arrow = self.ws1(side_end)?;
        let tail = arrow + usize::from(self.at(arrow) == b'<');
        let line = self.lit(tail, b"--")?;
        let arrow_end = line + usize::from(self.at(line) == b'>');
        let other = self.ws1(arrow_end)?;
        let other_end = self.one(other, |c| {
            matches!(c, 'B' | 'L' | 'R' | 'T' | 'b' | 'l' | 'r' | 't')
        })?;
        let target_colon = self.ws(other_end);
        let target_colon_end = self.lit(target_colon, b":")?;
        let second = self.ws(target_colon_end);
        let end = self.edge_end(10, second)?;
        for (group, from, to) in [
            (5, colon, colon_end),
            (6, side, side_end),
            (7, arrow, arrow_end),
            (8, other, other_end),
            (9, target_colon, target_colon_end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// `(?i)\s*(junction)\s+([-\w]+)\s*(in)?\s*([-\w]+)?`.
    pub(super) fn junction(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"junction")?;
        let name = self.ws1(word)?;
        let name_end = self.class_run1(name, is_dash_word)?;
        self.cap(1, start, word);
        self.cap(2, name, name_end);
        let mut q = self.ws(name_end);
        if let Some(end) = self.ci(q, b"in") {
            self.cap(3, q, end);
            q = end;
        }
        q = self.ws(q);
        Some(self.optional_run(4, q, is_dash_word))
    }

    /// `([-\w]+)\s*(\[)?\s*([-\w]+|"[-\w\s]+")?\s*(])?` at `p`: an entity and
    /// its alias, with groups 1 to 4.
    /// A part given back is taken by the next one, which ends at the same
    /// place.
    fn entity_head(&mut self, p: usize) -> Option<usize> {
        let name = self.class_run1(p, is_dash_word)?;
        self.cap(1, p, name);
        let mut q = self.ws(name);
        q = self.optional_lit(2, q, b"[");
        q = self.ws(q);
        let alias = if self.at(q) == b'"' {
            let close = self.class_run1(q + 1, is_dash_word_space);
            close
                .filter(|&c| self.at(c) == b'"' && c < self.lim())
                .map(|c| c + 1)
        } else {
            self.class_run1(q, is_dash_word)
        };
        if let Some(end) = alias {
            self.cap(3, q, end);
            q = end;
        }
        q = self.ws(q);
        Some(self.optional_lit(4, q, b"]"))
    }

    /// `(?i)^\s*ENTITY\s*$`: an entity alone on its line.
    pub(super) fn entity(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.entity_head(start)?;
        self.at_eol(end).then_some(end)
    }

    /// `(?i)\s*ENTITY\s*(\{)`: an entity and the brace that opens its
    /// attributes.
    pub(super) fn entity_open(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let head = self.entity_head(start)?;
        let brace = self.ws(head);
        let end = self.lit(brace, b"{")?;
        self.cap(5, brace, end);
        Some(end)
    }

    /// `(?i)[FPU]K` at `p`.
    fn key(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'F' | b'P' | b'U' | b'f' | b'p' | b'u') {
            return None;
        }
        self.ci(p + 1, b"k")
    }

    /// `(?i)\s*([-\w]+)\s+([-\w]+)\s+([FPU]K(?:,\s*[FPU]K){0,2})?\s*("[^\n\r"]*
    /// ")?\s*`: an attribute of an entity.
    pub(super) fn attribute_row(&mut self, p: usize) -> Option<usize> {
        let ty = self.ws(p);
        let ty_end = self.class_run1(ty, is_dash_word)?;
        let name = self.ws1(ty_end)?;
        let name_end = self.class_run1(name, is_dash_word)?;
        let mut q = self.ws1(name_end)?;
        self.cap(1, ty, ty_end);
        self.cap(2, name, name_end);
        if let Some(mut keys) = self.key(q) {
            for _ in 0..2 {
                let Some(next) = self
                    .lit(keys, b",")
                    .and_then(|comma| self.key(self.ws(comma)))
                else {
                    break;
                };
                keys = next;
            }
            self.cap(3, q, keys);
            q = keys;
        }
        q = self.ws(q);
        if self.at(q) == b'"' {
            let close = self.class_run(q + 1, |c| !matches!(c, '\n' | '\r' | '"'));
            if self.at(close) == b'"' && close < self.lim() {
                self.cap(4, q, close + 1);
                q = close + 1;
            }
        }
        Some(self.ws(q))
    }

    /// `(?i)\s*([-\w]+)\s*(LEFT(?:..|--)RIGHT)\s*([-\w]+)\s*(:)\s*(LABEL)`: a
    /// relationship between two entities.
    ///
    /// The first name takes `-`, `o` and digits, which start cardinalities.
    /// So it gives back one character at a time, and at each end every
    /// left and right cardinality is tried in order.
    /// `..` takes any two characters, so `--` is never reached.
    pub(super) fn er_relation(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let mut name = self.class_run1(start, is_dash_word)?;
        let (line, rest) = loop {
            let line = self.ws(name);
            if let Some(rest) = self.er_line(line) {
                break (line, rest);
            }
            name = self.char_start_before(name);
            if name <= start {
                return None;
            }
        };
        let [line_end, other, other_end, colon, label, end] = rest;
        for (group, from, to) in [
            (1, start, name),
            (2, line, line_end),
            (3, other, other_end),
            (4, colon, colon + 1),
            (5, label, end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// The rest of an entity relationship from `p`, its cardinalities on.
    /// It returns where the cardinalities end, where the second name starts
    /// and ends, the colon, and where the label starts and ends.
    fn er_line(&self, p: usize) -> Option<[usize; 6]> {
        LEFT_ENDS.iter().find_map(|left| {
            let line = self.any_two(self.ci(p, left)?)?;
            RIGHT_ENDS.iter().find_map(|right| {
                let line_end = self.ci(line, right)?;
                let other = self.ws(line_end);
                let other_end = self.class_run1(other, is_dash_word)?;
                let colon = self.ws(other_end);
                self.lit(colon, b":")?;
                let label = self.ws(colon + 1);
                let end = self.er_label(label)?;
                Some([line_end, other, other_end, colon, label, end])
            })
        })
    }

    /// `"[\w\s]*"|[-\w]+` at `p`: a relationship's label.
    fn er_label(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'"' {
            let close = self.class_run(p + 1, |c| is_word(c) || is_space(c));
            if self.at(close) == b'"' && close < self.lim() {
                return Some(close + 1);
            }
        }
        self.class_run1(p, is_dash_word)
    }

    /// `(?i)^\s*(WORD)\s+(C+)`: a Gantt setting and its value.
    pub(super) fn setting(
        &mut self,
        p: usize,
        word: &[u8],
        class: impl Fn(char) -> bool,
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word_end = self.ci(start, word)?;
        let value = self.ws1(word_end)?;
        let end = self.class_run1(value, class)?;
        self.cap(1, start, word_end);
        self.cap(2, value, end);
        Some(end)
    }

    /// `(?i)(tickInterval)\s+(([1-9][0-9]*)(UNIT))`.
    pub(super) fn tick_interval(&mut self, p: usize) -> Option<usize> {
        let word = self.ci(p, b"tickInterval")?;
        let value = self.ws1(word)?;
        if !matches!(self.at(value), b'1'..=b'9') {
            return None;
        }
        let number = self.skip(value + 1, |b| b.is_ascii_digit());
        let end = self.ci_first(number, UNITS)?;
        self.cap(1, p, word);
        self.cap(2, value, end);
        self.cap(3, value, number);
        self.cap(4, number, end);
        Some(end)
    }

    /// `(?i)^\s*(WORD)\s+(\s*[!-/:-?\\^\w\s]*)`, or `\s*` in place of `\s+`
    /// for [`Gap::Any`]: a title or a section.
    /// The text's class holds white space, so the group starts after the
    /// spaces.
    pub(super) fn titled(&mut self, p: usize, words: &[&[u8]], gap: Gap) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci_first(start, words)?;
        let text = match gap {
            Gap::Some => self.ws1(word)?,
            Gap::Any => self.ws(word),
        };
        let end = self.class_run(text, is_text);
        self.cap(1, start, word);
        self.cap(2, text, end);
        Some(end)
    }

    /// `(?i)^\s*(excludes)\s+((?:[-,\d\s]|DAY)+)`: the days a Gantt chart
    /// leaves out.
    /// The repetition holds white space.
    /// Where nothing it takes follows the spaces, they give back their last
    /// one.
    pub(super) fn excludes(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"excludes")?;
        let spaces = self.ws1(word)?;
        let mut end = spaces;
        loop {
            if let Some((c, next)) = self.ch(end)
                && (matches!(c, '-' | ',') || is_digit(c) || is_space(c))
            {
                end = next;
            } else if let Some(next) = self.ci_first(end, DAYS) {
                end = next;
            } else {
                break;
            }
        }
        let days = if end > spaces {
            spaces
        } else {
            let last = self.char_start_before(spaces);
            if last <= word {
                return None;
            }
            last
        };
        self.cap(1, start, word);
        self.cap(2, days, end);
        Some(end)
    }

    /// `(?i)^\s+(todayMarker)\s+(.*)$`.
    /// `.*` stops before the line feed, where `$` holds, unless the spaces
    /// took it.
    pub(super) fn today_marker(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws1(p)?;
        let word = self.ci(start, b"todayMarker")?;
        let text = self.ws1(word)?;
        let end = text.max(self.lim());
        self.cap(1, start, word);
        self.cap(2, text, end);
        Some(end)
    }

    /// `(?i)\s(.*)(:)`: a pie slice's label, up to the line's last colon.
    pub(super) fn slice(&mut self, p: usize) -> Option<usize> {
        let space = self.space_at(p);
        if space == 0 || p + space > self.lim() {
            return None;
        }
        let text = p + space;
        let colon = text + self.s[text..self.lim()].iter().rposition(|&b| b == b':')?;
        self.spend(self.lim() - text);
        self.cap(1, text, colon);
        self.cap(2, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(?i)^\s(.*)(:)`: a Gantt task, up to the line's last colon.
    pub(super) fn task(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.slice(p)
    }

    /// `(?i)^\s*(commit)`.
    pub(super) fn git_command(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.ci(start, b"commit")?;
        self.cap(1, start, end);
        Some(end)
    }

    /// The end of a Git value of the form `value` at `p`.
    fn git_value(&self, p: usize, value: GitValue) -> Option<usize> {
        let quoted = |class: fn(char) -> bool| {
            if self.at(p) != b'"' {
                return None;
            }
            let close = self.class_run(p + 1, class);
            (self.at(close) == b'"' && close < self.lim()).then_some(close + 1)
        };
        match value {
            GitValue::Quoted => quoted(|c| c != '\n' && c != '"'),
            GitValue::Tag => quoted(is_tag),
            GitValue::Type => self.ci_first(p, COMMIT_TYPES),
            GitValue::Order => self.class_run1(p, is_digit),
        }
    }

    /// `(?i)\s*(NAME)(:)\s?(VALUE)` at `p`, with groups from `group`: a Git
    /// option.
    /// Giving back `\s?` leaves a space where no value starts.
    fn git_pair(&mut self, group: usize, p: usize, name: &[u8], value: GitValue) -> Option<usize> {
        let name_end = self.ci(p, name)?;
        let colon = self.lit(name_end, b":")?;
        let start = self.ws01(colon);
        let end = self.git_value(start, value)?;
        self.cap(group, p, name_end);
        self.cap(group + 1, name_end, colon);
        self.cap(group + 2, start, end);
        Some(end)
    }

    /// `(?i)\s*(NAME)(:)\s?(VALUE)`: an option of a commit.
    pub(super) fn git_option(&mut self, p: usize, name: &[u8], value: GitValue) -> Option<usize> {
        let start = self.ws(p);
        self.git_pair(1, start, name, value)
    }

    /// `(?i)^\s*(checkout)\s*([^"\s]*)`.
    pub(super) fn checkout(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"checkout")?;
        let name = self.ws(word);
        let end = self.class_run(name, |c| c != '"' && !is_space(c));
        self.cap(1, start, word);
        self.cap(2, name, end);
        Some(end)
    }

    /// `(?i)^\s*(WORD)\s*([^"\s]*)\s*(?:(OPTION)(:)\s?(VALUE))?`: a branch
    /// with its order, or a merge with its tag.
    pub(super) fn branch_or_merge(
        &mut self,
        p: usize,
        word: &[u8],
        option: &[u8],
        value: GitValue,
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word_end = self.ci(start, word)?;
        let name = self.ws(word_end);
        let name_end = self.class_run(name, |c| c != '"' && !is_space(c));
        self.cap(1, start, word_end);
        self.cap(2, name, name_end);
        let rest = self.ws(name_end);
        Some(self.git_pair(3, rest, option, value).unwrap_or(rest))
    }

    /// `(?i)^\s*(cherry-pick)\s+(id)(:)\s*("[^\n"]*")`.
    pub(super) fn cherry_pick(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"cherry-pick")?;
        let id = self.ws1(word)?;
        let id_end = self.ci(id, b"id")?;
        let colon = self.lit(id_end, b":")?;
        let text = self.ws(colon);
        let end = self.git_value(text, GitValue::Quoted)?;
        for (group, from, to) in [
            (1, start, word),
            (2, id, id_end),
            (3, id_end, colon),
            (4, text, end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// `(?i)(\s*:::)(\s*[!-$\&'*-/;-?\\^\w\s]*)`: a mind map class.
    pub(super) fn mindmap_class(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(self.ws(p), b":::")?;
        let end = self.class_run(colons, is_mindmap_class);
        self.cap(1, p, colons);
        self.cap(2, colons, end);
        Some(end)
    }

    /// `(?i)(\s*::icon)(\s*\()(\s*[!-$\&'*-/;-?\\^\w\s]*)(\s*\))`: a mind map
    /// icon.
    /// The name's class holds white space but not `)`.
    /// So the name stops at `)` or nowhere useful, and giving back never
    /// helps.
    pub(super) fn mindmap_icon(&mut self, p: usize) -> Option<usize> {
        let word = self.ci(self.ws(p), b"::icon")?;
        let open = self.lit(self.ws(word), b"(")?;
        let name = self.class_run(open, is_mindmap_class);
        let end = self.lit(name, b")")?;
        self.cap(1, p, word);
        self.cap(2, word, open);
        self.cap(3, open, name);
        self.cap(4, name, end);
        Some(end)
    }

    /// `(?i)(\s*TEXT)(OPEN)(\s*TEXT)(CLOSE)`: a mind map node with a shape.
    ///
    /// The open shape is `\s*\({1,2}|\){1,2}|\{\{|\[` and the close one
    /// `\s*\){1,2}|\({1,2}|}}|]`.
    /// The text's class holds white space but no bracket.
    /// So each text stops at a bracket, and only the open shape gives
    /// back: `((` becomes `(`, so that the second `(` can close the node.
    pub(super) fn mindmap_shape(&mut self, p: usize) -> Option<usize> {
        let open = self.class_run(p, is_mindmap_text);
        let counted = |b: u8| {
            let one = open + 1;
            [(self.at(one) == b).then_some(one + 1), Some(one)]
        };
        let opens = match self.at(open) {
            b'(' => counted(b'('),
            b')' => counted(b')'),
            b'{' => [self.lit(open, b"{{"), None],
            b'[' => [Some(open + 1), None],
            _ => return None,
        };
        let (open_end, text_end, end) = opens.into_iter().flatten().find_map(|open_end| {
            let text_end = self.class_run(open_end, is_mindmap_text);
            let end = match self.at(text_end) {
                b @ (b')' | b'(') => text_end + 1 + usize::from(self.at(text_end + 1) == b),
                b'}' => self.lit(text_end, b"}}")?,
                b']' => text_end + 1,
                _ => return None,
            };
            Some((open_end, text_end, end))
        })?;
        self.cap(1, p, open);
        self.cap(2, open, open_end);
        self.cap(3, open_end, text_end);
        self.cap(4, text_end, end);
        Some(end)
    }

    /// `^(\s*[!-$\&'*-/:-?\\^\w\s]*)`: a mind map node's text.
    /// It can match empty.
    pub(super) fn mindmap_text(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.class_run(p, is_mindmap_text);
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i)^\s*([xy]-axis)\s+((?:(?!-->)[!#-'*-/=?\\\w\s])*)`: a quadrant
    /// chart's axis, up to its arrow.
    pub(super) fn quadrant_axis(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        if !matches!(self.at(start), b'x' | b'X' | b'y' | b'Y') {
            return None;
        }
        let word = self.ci(start + 1, b"-axis")?;
        let text = self.ws1(word)?;
        let mut end = text;
        while let Some((c, next)) = self.ch(end)
            && is_quadrant_text(c)
            && !self.starts(end, b"-->")
        {
            end = next;
        }
        self.spend(end - text);
        self.cap(1, start, word);
        self.cap(2, text, end);
        Some(end)
    }

    /// `(?i)\s*(-->)\s*([!#-'*-/=?\\\w\s]*)`.
    pub(super) fn axis_arrow(&mut self, p: usize) -> Option<usize> {
        let arrow = self.ws(p);
        let arrow_end = self.lit(arrow, b"-->")?;
        let text = self.ws(arrow_end);
        let end = self.class_run(text, is_quadrant_text);
        self.cap(1, arrow, arrow_end);
        self.cap(2, text, end);
        Some(end)
    }

    /// `(?i)^\s*(quadrant-[1-4])\s*([!-/:-?\\^\w\s]*)`.
    pub(super) fn quadrant(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"quadrant-")?;
        if !matches!(self.at(word), b'1'..=b'4') {
            return None;
        }
        let text = self.ws(word + 1);
        let end = self.class_run(text, is_text);
        self.cap(1, start, word + 1);
        self.cap(2, text, end);
        Some(end)
    }

    /// `\d\.\d+` at `p`.
    fn fraction(&self, p: usize) -> Option<usize> {
        let dot = self.one(p, is_digit)?;
        let digits = self.lit(dot, b".")?;
        self.class_run1(digits, is_digit)
    }

    /// `(?i)\s*(TEXT)\s*(:)\s*(\[)\s*(\d\.\d+)\s*(,)\s*(\d\.\d+)\s*(])`: a
    /// point of a quadrant chart.
    /// The text's class holds white space but not the colon, so it stops at
    /// the colon or nowhere useful.
    pub(super) fn point(&mut self, p: usize) -> Option<usize> {
        let text = self.ws(p);
        let text_end = self.class_run(text, is_quadrant_text);
        let colon = self.lit(text_end, b":")?;
        let open_at = self.ws(colon);
        let open = self.lit(open_at, b"[")?;
        let x = self.ws(open);
        let x_end = self.fraction(x)?;
        let comma_at = self.ws(x_end);
        let comma = self.lit(comma_at, b",")?;
        let y = self.ws(comma);
        let y_end = self.fraction(y)?;
        let close_at = self.ws(y_end);
        let end = self.lit(close_at, b"]")?;
        for (group, from, to) in [
            (1, text, text_end),
            (2, text_end, colon),
            (3, open_at, open),
            (4, x, x_end),
            (5, comma_at, comma),
            (6, y, y_end),
            (7, close_at, end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// `(?i)^\s*(WORD)\s*([!-/:-?\\^\w\s]*)\s*(\{)` at `p` once `WORD` has
    /// ended at `word`: a requirement's or an element's head.
    /// The text's class holds white space but not `{`.
    /// So it stops at the brace or nowhere useful.
    fn block_head(&mut self, start: usize, word: usize) -> Option<usize> {
        let text = self.ws(word);
        let text_end = self.class_run(text, is_text);
        let end = self.lit(text_end, b"{")?;
        self.cap(1, start, word);
        self.cap(2, text, text_end);
        self.cap(3, text_end, end);
        Some(end)
    }

    /// `(?i)^\s*((?:functional|interface|performance|physical)?
    /// requirement|designConstraint)…`: a requirement's head.
    pub(super) fn requirement(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let prefix = [
            b"functional".as_slice(),
            b"interface",
            b"performance",
            b"physical",
        ]
        .iter()
        .find_map(|prefix| self.ci(start, prefix))
        .and_then(|prefix| self.ci(prefix, b"requirement"));
        let word = prefix
            .or_else(|| self.ci(start, b"requirement"))
            .or_else(|| self.ci(start, b"designConstraint"))?;
        self.block_head(start, word)
    }

    /// `(?i)\s*(})`.
    pub(super) fn requirement_end(&mut self, p: usize) -> Option<usize> {
        let brace = self.ws(p);
        self.group_one(brace, b"}")
    }

    /// `(?i)^\s*(element)\s*([!-/:-?\\^\w\s]*)\s*(\{)`.
    pub(super) fn element(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"element")?;
        self.block_head(start, word)
    }

    /// `(?i)\s*(NAME)\s*(C+)`, for a class `C` that holds white space: a
    /// requirement's property.
    pub(super) fn property(
        &mut self,
        p: usize,
        name: &[u8],
        class: impl Fn(char) -> bool + Copy,
    ) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, name)?;
        let (value, end) = self.spaced_run(word, 0, class)?;
        self.cap(1, start, word);
        self.cap(2, value, end);
        Some(end)
    }

    /// `(?i)\s*(NAME)\s*(CHOICE)\s*$`: a requirement's risk or verification
    /// method.
    pub(super) fn choice(&mut self, p: usize, name: &[u8], choices: &[&[u8]]) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, name)?;
        let value = self.ws(word);
        let value_end = self.ci_first(value, choices)?;
        let end = self.ws(value_end);
        if !self.at_eol(end) {
            return None;
        }
        self.cap(1, start, word);
        self.cap(2, value, value_end);
        Some(end)
    }

    /// `(?i)^\s*(\w+)\s*(FROM)\s*(VERBs)\s*(TO)\s*(\w+)\s*$`: a relation
    /// between requirements, with `-` and `->` or `<-` and `-` around its
    /// verb.
    pub(super) fn requirement_link(&mut self, p: usize, from: &[u8], to: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let name = self.class_run1(start, is_word)?;
        let left = self.ws(name);
        let left_end = self.lit(left, from)?;
        let verb = self.ws(left_end);
        let verb_end = self.ci_first(verb, VERBS)?;
        let right = self.ws(verb_end);
        let right_end = self.lit(right, to)?;
        let other = self.ws(right_end);
        let other_end = self.class_run1(other, is_word)?;
        let end = self.ws(other_end);
        if !self.at_eol(end) {
            return None;
        }
        for (group, from, to) in [
            (1, start, name),
            (2, left, left_end),
            (3, verb, verb_end),
            (4, right, right_end),
            (5, other, other_end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// `(?i)\s*([!"$-/<-?\\^\w\s]*)\s*(:)\s*(\d+)\s*(:)`: a journey task and
    /// its score.
    /// The task's class holds white space but not the colon.
    pub(super) fn journey_task(&mut self, p: usize) -> Option<usize> {
        let task = self.ws(p);
        let task_end = self.class_run(task, is_journey_task);
        let colon = self.lit(task_end, b":")?;
        let score = self.ws(colon);
        let score_end = self.class_run1(score, is_digit)?;
        let score_colon = self.ws(score_end);
        let end = self.lit(score_colon, b":")?;
        for (group, from, to) in [
            (1, task, task_end),
            (2, task_end, colon),
            (3, score, score_end),
            (4, score_colon, end),
        ] {
            self.cap(group, from, to);
        }
        Some(end)
    }

    /// `(?i)\s*,?\s*([^\n#,]+)`: an actor of a journey task.
    /// The name's class holds white space.
    /// So where none of it follows the spaces, they give back their last
    /// one, and then the comma gives way.
    pub(super) fn journey_actor(&mut self, p: usize) -> Option<usize> {
        let class = |c: char| !matches!(c, '\n' | '#' | ',');
        let comma = self.ws(p);
        let (name, end) = self
            .lit(comma, b",")
            .and_then(|after| self.spaced_run(after, 0, class))
            .or_else(|| self.spaced_run(p, 0, class))?;
        self.cap(1, name, end);
        Some(end)
    }

    /// `(?i)^\s*(WORD)`: an XY chart axis.
    pub(super) fn axis(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let end = self.ci(start, word)?;
        self.cap(1, start, end);
        Some(end)
    }

    /// `[-+]?\d+\.?\d*` at `p`.
    /// Giving back the digits or the dot leaves one where the rest of every
    /// pattern after a number must start.
    fn number(&self, p: usize) -> Option<usize> {
        let digits = p + usize::from(matches!(self.at(p), b'-' | b'+'));
        let whole = self.class_run1(digits, is_digit)?;
        let dot = whole + usize::from(self.at(whole) == b'.');
        Some(self.class_run(dot, is_digit))
    }

    /// `(?i)\s*([-+]?\d+\.?\d*)\s*(-->)\s*([-+]?\d+\.?\d*)`: an axis range.
    pub(super) fn range(&mut self, p: usize) -> Option<usize> {
        let low = self.ws(p);
        let low_end = self.number(low)?;
        let arrow = self.ws(low_end);
        let arrow_end = self.lit(arrow, b"-->")?;
        let high = self.ws(arrow_end);
        let end = self.number(high)?;
        self.cap(1, low, low_end);
        self.cap(2, arrow, arrow_end);
        self.cap(3, high, end);
        Some(end)
    }

    /// `"[!#-(*-/:-?\\^\w\s]*"` at `p`.
    fn tag_string(&self, p: usize) -> Option<usize> {
        self.git_value(p, GitValue::Tag)
    }

    /// `(?i)\s+("[!#-(*-/:-?\\^\w\s]*")`: an axis title.
    pub(super) fn axis_title(&mut self, p: usize) -> Option<usize> {
        let text = self.ws1(p)?;
        let end = self.tag_string(text)?;
        self.cap(1, text, end);
        Some(end)
    }

    /// `(?i)\s+([!#-(*-/:-?\\^\w]*)`: an axis title as a word.
    /// It can be empty.
    pub(super) fn axis_word(&mut self, p: usize) -> Option<usize> {
        let text = self.ws1(p)?;
        let end = self.class_run(text, is_tag_word);
        self.cap(1, text, end);
        Some(end)
    }

    /// `\s*(B)`, for one byte `B`.
    pub(super) fn spaced_open(&mut self, p: usize, b: u8) -> Option<usize> {
        let at = self.ws(p);
        self.group_one(at, &[b])
    }

    /// `(?i)\s*([-+]?\d+\.?\d*)`.
    pub(super) fn spaced_number(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let end = self.number(start)?;
        self.cap(1, start, end);
        Some(end)
    }

    /// `(?i)\s*("[!#-(*-/:-?\\^\w\s]*")`.
    pub(super) fn spaced_string(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let end = self.tag_string(start)?;
        self.cap(1, start, end);
        Some(end)
    }

    /// `(?i)\s*([-!#-(*+./:-?\\^\w\s]+)`: a category on an axis.
    pub(super) fn category(&mut self, p: usize) -> Option<usize> {
        let (start, end) = self.spaced_run(p, 0, is_category)?;
        self.cap(1, start, end);
        Some(end)
    }

    /// `(?i)^\s*(line|bar)\s*(\[)`: a data series.
    pub(super) fn series(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"line").or_else(|| self.ci(start, b"bar"))?;
        let open = self.ws(word);
        let end = self.lit(open, b"[")?;
        self.cap(1, start, word);
        self.cap(2, open, end);
        Some(end)
    }
}
