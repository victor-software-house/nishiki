//! The sequence diagram.
//!
//! Its actors are runs of `[!-*./<-?\\^\w\s]`, which holds white space.
//! A run ends where `\b` holds, so the matchers give characters back to
//! reach one.
//! [`Lexer::sequence_actor`] and [`Lexer::spaces_then`] try every end in the
//! order a backtracking matcher does, and hand each to the rest of the pattern.

use super::super::engine::Lexer;
use crate::classes::{is_space, is_word};
use crate::rules::Chars;

/// What may follow a block's keyword.
#[derive(Clone, Copy, Debug)]
pub(super) enum Tail {
    /// `(?:\s+([^#;]*))?`.
    Open,
    /// `(?:\s+([^#;]*))?$`.
    ToEnd,
}

/// `alt|else|option|par|and|rect|autonumber|critical|opt`, in the grammar's
/// order.
pub(super) const BLOCKS: &[&[u8]] = &[
    b"alt",
    b"else",
    b"option",
    b"par",
    b"and",
    b"rect",
    b"autonumber",
    b"critical",
    b"opt",
];

/// `[!-*./<-?\\^\w\s]`: a character of an actor's name.
/// It holds `(`, `)`, `>` and white space, but no `-`, `+`, `,` or `:`.
fn is_actor(c: char) -> bool {
    matches!(c, '!'..='*' | '.' | '/' | '<'..='?' | '\\' | '^') || is_word(c) || is_space(c)
}

/// `[!-/:<-?\\^\w\s]`: the text of a title.
fn is_title(c: char) -> bool {
    matches!(c, '!'..='/' | ':' | '<'..='?' | '\\' | '^') || is_word(c) || is_space(c)
}

/// `[^#;]`.
fn is_text(c: char) -> bool {
    c != '#' && c != ';'
}

impl Lexer<'_> {
    /// `(?i)(title)\s*(:)?\s+(\s*[!-/:<-?\\^\w\s]*)`: a title.
    ///
    /// The `\s+` needs one white space after the optional colon.
    /// Without a colon, `\s*` gives its last white space to `\s+`.
    /// So `title Foo` and `title` at the end of a line match.
    pub(super) fn sequence_title(&mut self, p: usize) -> Option<usize> {
        let word = self.ci(p, b"title")?;
        let spaces = self.ws(word);
        let colon = (self.at(spaces) == b':').then_some(spaces + 1);
        let text = match colon {
            Some(after) if self.space_at(after) > 0 => self.ws(after),
            _ if spaces > word => spaces,
            _ => return None,
        };
        let end = self.class_run(text, is_title);
        self.capture(1, p, word);
        if let Some(after) = colon
            && text > after
        {
            self.capture(2, spaces, after);
        }
        self.capture(3, text, end);
        Some(end)
    }

    /// `(?i)\s*(participant|actor)\s+((?:(?! as )C)+)\s*(as)?\s(C'+)?`: a
    /// participant, where `C` is an actor's class and `C'` adds `,`.
    ///
    /// The name stops before ` as `, under `(?i)`.
    /// It gives back one character at a time until `\s*(as)?\s` follows.
    /// - Where `as` and a white space follow, the alias comes after them.
    /// - Otherwise `\s*` gives its last white space to `\s`.
    ///
    /// So a name that runs to the end of the line keeps all but the line
    /// feed.
    pub(super) fn participant(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self
            .ci(start, b"participant")
            .or_else(|| self.ci(start, b"actor"))?;
        let name = self.ws1(word)?;
        let mut name_end = name;
        while let Some((c, end)) = self.ch(name_end)
            && is_actor(c)
            && self.ci(name_end, b" as ").is_none()
        {
            name_end = end;
        }
        self.spend(name_end - name);
        let (alias_word, alias) = loop {
            if name_end <= name {
                return None;
            }
            if let Some(found) = self.participant_tail(name_end) {
                break found;
            }
            name_end = self.char_start_before(name_end);
        };
        let alias_end = self.class_run(alias, |c| c == ',' || is_actor(c));
        self.capture(1, start, word);
        self.capture(2, name, name_end);
        if let Some((from, to)) = alias_word {
            self.capture(3, from, to);
        }
        self.capture(4, alias, alias_end);
        Some(alias_end)
    }

    /// `\s*(as)?\s` at `p`: the span of `as`, when it is taken, and where the
    /// alias starts.
    fn participant_tail(&self, p: usize) -> Option<(Option<(usize, usize)>, usize)> {
        let spaces = self.ws(p);
        if let Some(word) = self.ci(spaces, b"as") {
            let gap = self.space_at(word);
            if gap > 0 {
                return Some((Some((spaces, word)), word + gap));
            }
        }
        (spaces > p).then_some((None, spaces))
    }

    /// `[!-*./<-?\\^\w\s]+\b\)?` from `p`, an actor, and then `then`.
    /// The run gives back one character at a time; at each end where `\b`
    /// holds, `\)?` takes a `)` first and then none.
    /// It returns the actor's end and what `then` found after it.
    fn sequence_actor<T>(&self, p: usize, then: impl Fn(usize) -> Option<T>) -> Option<(usize, T)> {
        let greedy = self.class_run(p, is_actor);
        let mut end = greedy;
        while end > p {
            if self.boundary(end) {
                if self.at(end) == b')'
                    && let Some(found) = then(end + 1)
                {
                    return Some((end + 1, found));
                }
                if let Some(found) = then(end) {
                    return Some((end, found));
                }
            }
            end = self.char_start_before(end);
        }
        None
    }

    /// `\s*` from `p` and then `then`: the spaces give back one at a time.
    /// It returns where the spaces end and what `then` found after them.
    fn spaces_then<T>(&self, p: usize, then: impl Fn(usize) -> Option<T>) -> Option<(usize, T)> {
        let mut end = self.ws(p);
        loop {
            if let Some(found) = then(end) {
                return Some((end, found));
            }
            if end <= p {
                return None;
            }
            end = self.char_start_before(end);
        }
    }

    /// `(?i)\s*((?:de)?activate)\s+\b([!-*./<-?\\^\w\s]+\b\)?\s*)`.
    /// Nothing follows the group, so its first end wins.
    pub(super) fn activation(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self
            .ci(start, b"activate")
            .or_else(|| self.ci(start, b"deactivate"))?;
        let name = self.ws1(word)?;
        self.word_at(name)?;
        let (_, end) = self.sequence_actor(name, |q| Some(self.ws(q)))?;
        self.capture(1, start, word);
        self.capture(2, name, end);
        Some(end)
    }

    /// `(?i)(?:left|right)\sof|over` at `p`.
    fn note_place(&self, p: usize) -> Option<usize> {
        let side = self.ci(p, b"left").or_else(|| self.ci(p, b"right"));
        side.and_then(|side| {
            let gap = self.space_at(side);
            (gap > 0).then(|| self.ci(side + gap, b"of")).flatten()
        })
        .or_else(|| self.ci(p, b"over"))
    }

    /// `(?i)\s*(Note)\s+(PLACE)\s+\b(ACTOR\s*)(,)?(\bACTOR\s*)?(:)(?:\s+([^#;]*
    /// ))?`: a note over one or two actors.
    ///
    /// Each actor is tried at each of its ends, and then its spaces at each
    /// of theirs, until a colon follows.
    /// No actor holds `,`, so the comma is taken where it stands.
    pub(super) fn note(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"Note")?;
        let place = self.ws1(word)?;
        let place_end = self.note_place(place)?;
        let first = self.ws1(place_end)?;
        self.word_at(first)?;
        let (_, (first_end, (comma, second, colon))) = self.sequence_actor(first, |q| {
            self.spaces_then(q, |w| {
                let comma = (self.at(w) == b',').then_some(w + 1);
                let second = comma.unwrap_or(w);
                let found = self
                    .boundary(second)
                    .then(|| {
                        self.sequence_actor(second, |q| self.spaces_then(q, |c| self.lit(c, b":")))
                    })
                    .flatten();
                match found {
                    Some((_, (second_end, colon))) => {
                        Some((comma, Some((second, second_end)), colon))
                    }
                    None => self.lit(second, b":").map(|colon| (comma, None, colon)),
                }
            })
        })?;
        let text = (self.space_at(colon) > 0).then(|| self.ws(colon));
        let end = text.map_or(colon, |text| self.class_run(text, is_text));
        self.capture(1, start, word);
        self.capture(2, place, place_end);
        self.capture(3, first, first_end);
        if let Some(after) = comma {
            self.capture(4, after - 1, after);
        }
        if let Some((from, to)) = second {
            self.capture(5, from, to);
        }
        self.capture(6, colon - 1, colon);
        if let Some(text) = text {
            self.capture(7, text, end);
        }
        Some(end)
    }

    /// `(?i)\s*(WORD)(?:\s+([^#;]*))?`, with `$` after it for [`Tail::ToEnd`]:
    /// a block's keyword and its text.
    ///
    /// `[^#;]*` takes the line feed, so it reaches the end unless a `#` or
    /// `;` stops it.
    /// Under `$`, a keyword whose text stops early gives way to the next
    /// one, as `option` gives way to `opt`.
    pub(super) fn keyword_rest(&mut self, p: usize, words: &[&[u8]], tail: Tail) -> Option<usize> {
        let start = self.ws(p);
        for word in words {
            let Some(word_end) = self.ci(start, word) else {
                continue;
            };
            let text = (self.space_at(word_end) > 0).then(|| self.ws(word_end));
            let end = text.map_or(word_end, |text| self.class_run(text, is_text));
            let found = match tail {
                Tail::Open => Some(end),
                Tail::ToEnd if self.at_eol(end) => Some(end),
                Tail::ToEnd => self.at_eol(word_end).then_some(word_end),
            };
            if let Some(found) = found {
                self.capture(1, start, word_end);
                if let Some(text) = text
                    && found == end
                {
                    self.capture(2, text, end);
                }
                return Some(found);
            }
        }
        None
    }

    /// `\s*(end)`.
    pub(super) fn sequence_end(&mut self, p: usize) -> Option<usize> {
        let word = self.ws(p);
        self.group_one(word, b"end")
    }

    /// `-?-[)>x]>?[-+]?` at `p`, an arrow, and then `then`.
    /// The arrow's `>?` gives its `>` back, since an actor can start with
    /// one.
    /// Giving back `[-+]?` leaves a character no actor holds.
    fn arrow<T>(&self, p: usize, then: impl Fn(usize) -> Option<T>) -> Option<(usize, T)> {
        let line = if self.starts(p, b"--") {
            p + 2
        } else {
            self.lit(p, b"-")?
        };
        if !matches!(self.at(line), b')' | b'>' | b'x' | b'X') {
            return None;
        }
        let head = line + 1;
        let heads = [(self.at(head) == b'>').then_some(head + 1), Some(head)];
        heads.into_iter().flatten().find_map(|q| {
            let end = q + usize::from(matches!(self.at(q), b'-' | b'+'));
            then(end).map(|found| (end, found))
        })
    }

    /// `(?i)\s*\b(ACTOR)\s*(ARROW)\s*(ACTOR)\s*(:)\s*([^#;]*)`: a message.
    ///
    /// The leading `\s*` gives back white space until `\b` holds.
    /// Then each actor, the arrow's head and the spaces between them are
    /// tried at each of their ends until a colon follows.
    pub(super) fn message(&mut self, p: usize) -> Option<usize> {
        let mut from = self.ws(p);
        let (from_end, (arrow, (arrow_end, (to, (to_end, (_, colon)))))) = loop {
            if self.boundary(from)
                && let Some(found) = self.sequence_actor(from, |q| {
                    self.spaces_then(q, |a| {
                        self.arrow(a, |e| {
                            self.spaces_then(e, |t| {
                                self.sequence_actor(t, |q| {
                                    self.spaces_then(q, |c| self.lit(c, b":"))
                                })
                            })
                        })
                    })
                })
            {
                break found;
            }
            if from <= p {
                return None;
            }
            from = self.char_start_before(from);
        };
        let text = self.ws(colon);
        let end = self.class_run(text, is_text);
        self.capture(1, from, from_end);
        self.capture(2, arrow, arrow_end);
        self.capture(3, to, to_end);
        self.capture(4, colon - 1, colon);
        self.capture(5, text, end);
        Some(end)
    }

    /// `(?i)\s*(box)\s+(transparent)(?:\s+([^#;]*))?`.
    pub(super) fn transparent_box(&mut self, p: usize) -> Option<usize> {
        let start = self.ws(p);
        let word = self.ci(start, b"box")?;
        let transparent = self.ws1(word)?;
        let transparent_end = self.ci(transparent, b"transparent")?;
        let text = (self.space_at(transparent_end) > 0).then(|| self.ws(transparent_end));
        let end = text.map_or(transparent_end, |text| self.class_run(text, is_text));
        self.capture(1, start, word);
        self.capture(2, transparent, transparent_end);
        if let Some(text) = text {
            self.capture(3, text, end);
        }
        Some(end)
    }
}
