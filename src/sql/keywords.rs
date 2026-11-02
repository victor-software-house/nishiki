//! The keyword rules of the grammar that take several words, or a type's
//! size: each is a regular expression of alternatives that start with a word
//! and end at a word boundary, and the first word picks the alternative.

use super::engine::{Hit, Lexer};
use super::matchers::LONGEST;
use super::scope::S;

/// The types of the first alternative of the type rule.
const PLAIN_TYPES: &[&str] = &[
    "bigint",
    "bigserial",
    "bit",
    "boolean",
    "box",
    "bytea",
    "cidr",
    "circle",
    "date",
    "inet",
    "int",
    "integer",
    "line",
    "lseg",
    "macaddr",
    "money",
    "oid",
    "path",
    "point",
    "polygon",
    "real",
    "serial",
    "smallint",
    "sysdate",
    "text",
];

/// The two numbers of `(\d+,\d+)` as their starts and ends, and the end of the
/// parentheses.
type Pair = ((usize, usize), (usize, usize), usize);

/// A list of end positions, in the order a regular expression tries them.
struct Ends {
    ends: [usize; 5],
    len: usize,
}

impl Ends {
    fn new() -> Ends {
        Ends {
            ends: [0; 5],
            len: 0,
        }
    }

    fn push(&mut self, end: Option<usize>) {
        if let (Some(end), Some(slot)) = (end, self.ends.get_mut(self.len)) {
            *slot = end;
            self.len += 1;
        }
    }

    fn all(&self) -> &[usize] {
        &self.ends[..self.len]
    }
}

impl Lexer<'_> {
    /// Whether the word run `p..e` is `word`, without regard to case.
    fn run_is(&self, p: usize, e: usize, word: &str) -> bool {
        self.lit(p, word) == Some(e)
    }

    /// The end of one white space character at `i`, which is the grammar's
    /// `\s`.
    fn one_space(&self, i: usize) -> Option<usize> {
        Some(i + self.space_w(i)).filter(|&end| end > i)
    }

    /// The word `word` after one white space character at `i`.
    fn space_then(&self, i: usize, word: &str) -> Option<usize> {
        self.one_space(i).and_then(|at| self.lit(at, word))
    }

    /// The word `word` after white space at `i`: `\s+word`.
    fn spaces_then(&self, i: usize, word: &str) -> Option<usize> {
        self.spaces1(i).and_then(|at| self.lit(at, word))
    }

    /// `\((\d+)\)` at `i`: where the digits start and end, and the end.
    fn paren_digits(&self, i: usize) -> Option<(usize, usize, usize)> {
        if self.at(i) != b'(' || i >= self.lim {
            return None;
        }
        let end = self.digits(i + 1);
        (end > i + 1 && self.at(end) == b')' && end < self.lim).then_some((i + 1, end, end + 1))
    }

    /// `\((\d+),(\d+)\)` at `i`: the two numbers and the end.
    fn paren_pair(&self, i: usize) -> Option<Pair> {
        if self.at(i) != b'(' || i >= self.lim {
            return None;
        }
        let first = self.digits(i + 1);
        if first == i + 1 || self.at(first) != b',' || first >= self.lim {
            return None;
        }
        let second = self.digits(first + 1);
        (second > first + 1 && self.at(second) == b')' && second < self.lim).then_some((
            (i + 1, first),
            (first + 1, second),
            second + 1,
        ))
    }

    /// `\s(with|without)\stime\szone\b` at `i`: the end.
    fn time_zone(&self, i: usize) -> Option<usize> {
        let at = self.one_space(i)?;
        [self.lit(at, "with"), self.lit(at, "without")]
            .into_iter()
            .flatten()
            .find_map(|end| {
                let end = self.space_then(end, "time")?;
                let end = self.space_then(end, "zone")?;
                self.bnd(end).then_some(end)
            })
    }

    /// The type rule: six alternatives, each starting with its words.
    pub(super) fn types(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let named = Hit::new(p, e).cap(0, p, e, S::StorageType);
        // `\b(bigint|...|double\sprecision|...)\b`
        if PLAIN_TYPES.iter().any(|word| self.run_is(p, e, word)) {
            return Some(named);
        }
        if self.run_is(p, e, "double") {
            let end = self.space_then(e, "precision").filter(|&end| self.bnd(end));
            if let Some(end) = end {
                return Some(Hit::new(p, end).cap(0, p, end, S::StorageType));
            }
        }
        if let Some(hit) = self.sized_type(p, e) {
            return Some(hit);
        }
        // `\b(char|number|varchar\d?)\b(?:\((\d+)\))?`
        let varchar = self.run_is(p, p + 7, "varchar")
            && (e == p + 7 || (self.digit_w(p + 7) > 0 && e == p + 7 + self.digit_w(p + 7)));
        if self.run_is(p, e, "char") || self.run_is(p, e, "number") || varchar {
            return Some(match self.paren_digits(e) {
                Some((from, to, end)) => {
                    Hit::new(p, end)
                        .cap(0, p, e, S::StorageType)
                        .cap(1, from, to, S::Numeric)
                }
                None => named,
            });
        }
        // `\b(numeric|decimal)\b(?:\((\d+),(\d+)\))?`
        if self.run_is(p, e, "numeric") || self.run_is(p, e, "decimal") {
            return Some(match self.paren_pair(e) {
                Some((first, second, end)) => Hit::new(p, end)
                    .cap(0, p, e, S::StorageType)
                    .cap(1, first.0, first.1, S::Numeric)
                    .cap(2, second.0, second.1, S::Numeric),
                None => named,
            });
        }
        self.time_type(p, e)
    }

    /// `\b(bit\svarying|character\s(?:varying)?|tinyint|var\
    /// schar|float|interval)\((\d+)\)`
    fn sized_type(&self, p: usize, e: usize) -> Option<Hit> {
        let mut names = Ends::new();
        if self.run_is(p, e, "bit") {
            names.push(self.space_then(e, "varying"));
        } else if self.run_is(p, e, "character") {
            let at = self.one_space(e)?;
            names.push(self.lit(at, "varying"));
            names.push(Some(at));
        } else if self.run_is(p, e, "var") {
            names.push(self.space_then(e, "char"));
        } else if ["tinyint", "float", "interval"]
            .iter()
            .any(|word| self.run_is(p, e, word))
        {
            names.push(Some(e));
        }
        names.all().iter().find_map(|&name| {
            let (from, to, end) = self.paren_digits(name)?;
            Some(
                Hit::new(p, end)
                    .cap(0, p, name, S::StorageType)
                    .cap(1, from, to, S::Numeric),
            )
        })
    }

    /// The time and timestamp alternatives: an optional size, and an optional
    /// time zone clause.
    fn time_type(&self, p: usize, e: usize) -> Option<Hit> {
        let stamp = self.lit(p, "timestamp").unwrap_or(p);
        let plain = ["time", "times", "timestamp"]
            .iter()
            .any(|word| self.run_is(p, e, word));
        let (word_end, suffix) = if plain {
            (e, None)
        } else if self.run_is(p, e, "timestamps") || self.run_is(p, e, "timestamptz") {
            (stamp, Some((stamp, e)))
        } else {
            return None;
        };
        let mut hit = Hit::new(p, e).cap(0, p, word_end, S::StorageType);
        if let Some((from, to)) = suffix {
            hit = hit.cap(1, from, to, S::StorageType);
        }
        let mut end = e;
        if let Some((from, to, after)) = self.paren_digits(e) {
            hit = hit.cap(2, from, to, S::Numeric);
            end = after;
        }
        if let Some(after) = self.time_zone(end) {
            hit = hit.cap(3, end, after, S::StorageType);
            end = after;
        }
        hit.end = end;
        Some(hit)
    }

    /// `\b((?:primary|foreign)\s+key|references|on\s+(delete|update)(\
    /// s+cascade)?|nocheck|check|constraint|collate|default)\b`
    pub(super) fn modifier(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let end = if self.run_is(p, e, "primary") || self.run_is(p, e, "foreign") {
            self.spaces_then(e, "key")
        } else if self.run_is(p, e, "on") {
            let action = self
                .spaces1(e)
                .and_then(|at| self.lit(at, "delete").or_else(|| self.lit(at, "update")));
            let mut ends = Ends::new();
            ends.push(action.and_then(|end| self.spaces_then(end, "cascade")));
            ends.push(action);
            ends.all().iter().copied().find(|&end| self.bnd(end))
        } else if [
            "references",
            "nocheck",
            "check",
            "constraint",
            "collate",
            "default",
        ]
        .iter()
        .any(|word| self.run_is(p, e, word))
        {
            Some(e)
        } else {
            None
        }?;
        self.bnd(end).then(|| Hit::new(p, end))
    }

    /// The first of `ends` that a word boundary follows.
    fn first_bounded(&self, ends: &Ends) -> Option<usize> {
        ends.all().iter().copied().find(|&end| self.bnd(end))
    }

    /// The query keywords: `select(\s+(all|distinct))?`, `group\s+by`, the
    /// joins and the rest.
    pub(super) fn dml(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let mut buf = [0; LONGEST];
        let word = self.lower(p, e, &mut buf)?;
        let mut ends = Ends::new();
        match word {
            b"select" => {
                ends.push(
                    self.spaces_then(e, "all")
                        .or_else(|| self.spaces_then(e, "distinct")),
                );
                ends.push(Some(e));
            }
            b"insert" => {
                let at = self.spaces1(e)?;
                ends.push(
                    self.lit(at, "ignore")
                        .and_then(|end| self.spaces_then(end, "into")),
                );
                ends.push(self.lit(at, "into"));
            }
            b"update" | b"delete" | b"from" | b"set" | b"where" | b"or" | b"like" | b"and"
            | b"having" | b"limit" | b"join" | b"straight_join" => ends.push(Some(e)),
            b"group" | b"order" => ends.push(self.spaces_then(e, "by")),
            b"union" => {
                ends.push(self.spaces_then(e, "all"));
                ends.push(Some(e));
            }
            b"cross" | b"inner" => ends.push(self.spaces_then(e, "join")),
            b"left" | b"right" | b"full" => self.outer_join(e, &mut ends),
            b"natural" => {
                ends.push(
                    self.spaces_then(e, "inner")
                        .and_then(|end| self.spaces_then(end, "join")),
                );
                let side = self.spaces1(e).and_then(|at| {
                    ["left", "right", "full"]
                        .iter()
                        .find_map(|word| self.lit(at, word))
                });
                if let Some(side) = side {
                    self.outer_join(side, &mut ends);
                }
                ends.push(self.spaces_then(e, "join"));
            }
            _ => return None,
        }
        self.first_bounded(&ends).map(|end| Hit::new(p, end))
    }

    /// `(\s+outer)?\s+join` after `left`, `right` or `full`.
    fn outer_join(&self, side: usize, ends: &mut Ends) {
        ends.push(
            self.spaces_then(side, "outer")
                .and_then(|end| self.spaces_then(end, "join")),
        );
        ends.push(self.spaces_then(side, "join"));
    }

    /// `\b(on|off|((is\s+)?not\s+)?null)\b`
    pub(super) fn ddl(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let end =
            if self.run_is(p, e, "on") || self.run_is(p, e, "off") || self.run_is(p, e, "null") {
                Some(e)
            } else if self.run_is(p, e, "not") {
                self.spaces_then(e, "null")
            } else if self.run_is(p, e, "is") {
                self.spaces_then(e, "not")
                    .and_then(|end| self.spaces_then(end, "null"))
            } else {
                None
            }?;
        self.bnd(end).then(|| Hit::new(p, end))
    }

    /// `\b(begin(\s+work)?|start\s+transaction|commit(\s+work)?|rollback(\
    /// s+work)?)\b`
    pub(super) fn luw(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let mut ends = Ends::new();
        if self.run_is(p, e, "start") {
            ends.push(self.spaces_then(e, "transaction"));
        } else if ["begin", "commit", "rollback"]
            .iter()
            .any(|word| self.run_is(p, e, word))
        {
            ends.push(self.spaces_then(e, "work"));
            ends.push(Some(e));
        }
        self.first_bounded(&ends).map(|end| Hit::new(p, end))
    }

    /// `\b(grant(\swith\sgrant\soption)?|revoke)\b`
    pub(super) fn authorization(&self, p: usize) -> Option<Hit> {
        if !self.word_start(p) {
            return None;
        }
        let e = self.word_run(p);
        let mut ends = Ends::new();
        if self.run_is(p, e, "grant") {
            ends.push(
                self.space_then(e, "with")
                    .and_then(|end| self.space_then(end, "grant"))
                    .and_then(|end| self.space_then(end, "option")),
            );
            ends.push(Some(e));
        } else if self.run_is(p, e, "revoke") {
            ends.push(Some(e));
        }
        self.first_bounded(&ends).map(|end| Hit::new(p, end))
    }
}
