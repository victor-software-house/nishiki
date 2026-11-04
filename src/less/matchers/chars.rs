//! The character classes, escapes and names the grammar's expressions are
//! built from.

use super::super::engine::Lexer;
use crate::case_fold;
use crate::rules::Chars;

/// A character class of the grammar's names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::less) enum Class {
    /// `[-\w[^\x00-\x7F]]`.
    DashWord,
    /// `[A-Z_a-z[^\x00-\x7F]]`.
    Alpha,
    /// `[_a-z[^\x00-\x7F]]`.
    Lower,
    /// `[-0-9_a-z[^\x00-\x7F]]`.
    LowerRest,
    /// `[-0-9A-Z_a-z[^\x00-\x7F]]`.
    AlphaRest,
    /// `[-\w[^\x00-\x{9F}]]`.
    DashWordWide,
    /// `[A-Z_a-z[^\x00-\x{9F}]]`.
    AlphaWide,
    /// The start of a CSS name,
    /// `[A-Z_a-z·À-ÖØ-öø-ͽͿ-῿‌‍‿⁀⁰-↏Ⰰ-⿯、-퟿豈-﷏ﷰ-�𐀀-\x{EFFFF}]`.
    NameStart,
    /// The rest of a CSS name: [`Class::NameStart`], `-` and `\d`.
    NameRest,
}

/// How a name escapes a character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::less) enum Escape {
    /// `\\(?:\h{1,6}[\t\n\f\s]?|[^\n\f\h])`.
    Hex,
    /// `\\(?:\N|\H|\h{1,6}[R\s])`: a backslash and any one character.
    Any,
    /// `(:?\\[0-9a-f]{1,6}(\r\n|[\t\n\f\r\s])?)|\\[^\n\f\r0-9a-f]`, the escape
    /// of the first character of a keyframe name.
    KeyframeStart,
    /// `(:?\\[0-9a-f]{1,6}(\r\n|[\t\n\f\r])?)|\\[^\n\f\r0-9a-f]`, the escape of
    /// the rest of one.
    KeyframeRest,
}

/// Whether `c` is in the CSS name-start ranges beyond ASCII.
fn name_start_wide(c: char) -> bool {
    matches!(
        u32::from(c),
        0xb7 | 0xc0..=0xd6
            | 0xd8..=0xf6
            | 0xf8..=0x37d
            | 0x37f..=0x1fff
            | 0x200c..=0x200d
            | 0x203f..=0x2040
            | 0x2070..=0x218f
            | 0x2c00..=0x2fef
            | 0x3001..=0xd7ff
            | 0xf900..=0xfdcf
            | 0xfdf0..=0xfffd
            | 0x1_0000..=0xe_ffff
    )
}

impl Lexer<'_> {
    /// The width of the character at `i` when it is in `class`.
    pub(super) fn class_at(&self, i: usize, class: Class) -> Option<usize> {
        let (c, end) = self.char_at(i)?;
        let ascii = |b: u8| match class {
            Class::DashWord | Class::DashWordWide | Class::AlphaRest | Class::NameRest => {
                b == b'-' || b == b'_' || b.is_ascii_alphanumeric()
            }
            Class::Alpha | Class::AlphaWide | Class::NameStart => {
                b == b'_' || b.is_ascii_alphabetic()
            }
            Class::Lower => b == b'_' || b.is_ascii_lowercase(),
            Class::LowerRest => {
                b == b'-' || b == b'_' || b.is_ascii_lowercase() || b.is_ascii_digit()
            }
        };
        let fits = match u8::try_from(c) {
            Ok(b) if b.is_ascii() => ascii(b),
            _ => match class {
                Class::DashWord
                | Class::Alpha
                | Class::Lower
                | Class::LowerRest
                | Class::AlphaRest => true,
                Class::DashWordWide | Class::AlphaWide => u32::from(c) > 0x9f,
                Class::NameStart => name_start_wide(c),
                Class::NameRest => name_start_wide(c) || self.digit_end(i) > i,
            },
        };
        fits.then_some(end - i)
    }

    /// The end of the escape `escape` at `i`. A keyframe escape's optional
    /// `:` belongs to its first alternative only; the second takes none.
    pub(super) fn escape_at(&self, i: usize, escape: Escape) -> Option<usize> {
        match escape {
            Escape::Hex => {
                if self.at(i) != b'\\' || i >= self.lim() {
                    return None;
                }
                let hex = self.bounded(i + 1, 6, |b| b.is_ascii_hexdigit());
                if hex > i + 1 {
                    return Some(hex + self.space_at(hex));
                }
                (!matches!(self.at(i + 1), b'\n' | b'\x0c') && i + 1 < self.lim())
                    .then(|| self.char_end(i + 1))
            }
            Escape::Any => (self.at(i) == b'\\'
                && i < self.lim()
                && (i + 1 < self.lim() || self.nl))
                .then(|| {
                    if i + 1 < self.lim() {
                        self.char_end(i + 1)
                    } else {
                        i + 2
                    }
                }),
            Escape::KeyframeStart | Escape::KeyframeRest => {
                let slash = if self.at(i) == b':' && self.at(i + 1) == b'\\' {
                    i + 1
                } else {
                    i
                };
                if self.at(slash) != b'\\' || slash >= self.lim() {
                    return None;
                }
                let hex = self.bounded(slash + 1, 6, |b| {
                    b.is_ascii_digit() || (b'a'..=b'f').contains(&b)
                });
                if hex > slash + 1 {
                    if self.starts(hex, b"\r\n") {
                        return Some(hex + 2);
                    }
                    let blank = match self.at(hex) {
                        b'\t' | b'\x0c' | b'\r' => 1,
                        b'\n' if hex == self.lim() && self.nl => 1,
                        _ => match escape {
                            Escape::KeyframeStart => self.space_at(hex),
                            Escape::Hex | Escape::Any | Escape::KeyframeRest => 0,
                        },
                    };
                    return Some(hex + blank);
                }
                (slash == i
                    && slash + 1 < self.lim()
                    && !matches!(self.at(slash + 1), b'\n' | b'\x0c' | b'\r' | b'0'..=b'9' | b'a'..=b'f'))
                .then(|| self.char_end(slash + 1))
            }
        }
    }

    /// The end of one character of `class`, or one escape, at `i`.
    pub(super) fn unit_at(&self, i: usize, class: Class, escape: Escape) -> Option<usize> {
        self.class_at(i, class)
            .map(|w| i + w)
            .or_else(|| self.escape_at(i, escape))
    }

    /// The end of the run of characters of `class` and escapes from `i`. A
    /// run without an escape is remembered, and a later read from inside it
    /// ends where it does.
    pub(super) fn units_end(&self, i: usize, class: Class, escape: Escape) -> usize {
        let (known, how, from, to) = self.x.units.get();
        if known == class && how == escape && from < i && i < to {
            return to;
        }
        let mut end = i;
        let mut plain = true;
        while let Some(next) = self.unit_at(end, class, escape) {
            plain &= self.at(end) != b'\\';
            end = next;
        }
        self.spend(end - i);
        if plain {
            self.x.units.set((class, escape, i, end));
        }
        end
    }

    /// `-?(?:first|escape)(?:rest|escape)*` at `p`: the optional `-` gives
    /// itself back when the name cannot start after it.
    pub(super) fn name(
        &self,
        p: usize,
        first: Class,
        rest: Class,
        escape: Escape,
    ) -> Option<usize> {
        self.name_with(p, (first, escape), (rest, escape))
    }

    /// [`Lexer::name`] with an escape of its own for the first character.
    pub(super) fn name_with(
        &self,
        p: usize,
        first: (Class, Escape),
        rest: (Class, Escape),
    ) -> Option<usize> {
        let start = if self.at(p) == b'-' && self.unit_at(p + 1, first.0, first.1).is_some() {
            p + 1
        } else {
            p
        };
        let head = self.unit_at(start, first.0, first.1)?;
        Some(self.units_end(head, rest.0, rest.1))
    }

    /// The last `k` in `from + 1..=to` where `\b` holds: where a run of
    /// `from..to` that a `\b` follows ends, the run giving characters back
    /// until one does.
    pub(super) fn boundary_back(&self, from: usize, to: usize) -> Option<usize> {
        let mut k = to;
        while k > from {
            if self.boundary(k) {
                self.spend(to - k);
                return Some(k);
            }
            k = self.char_start_before(k);
        }
        None
    }

    /// `\s*` from `p`. Most patterns start with it, so a read from inside
    /// the last run of white space ends where that run does, and a long run
    /// is read once, not once at each of its places.
    pub(super) fn spaces(&self, p: usize) -> usize {
        let (from, to) = self.x.spaces.get();
        if from < p && p < to && self.at(p) & 0xc0 != 0x80 {
            return to;
        }
        let end = self.space_end(p);
        if end > p {
            self.spend(end - p);
            self.x.spaces.set((p, end));
        }
        end
    }

    /// The end of the run of `\d` from `p`, which a read from inside the last
    /// run shares.
    pub(super) fn digits(&self, p: usize) -> usize {
        let (from, to) = self.x.digits.get();
        if from < p && p < to {
            return to;
        }
        let end = self.digit_end(p);
        if end > p {
            self.spend(end - p);
            self.x.digits.set((p, end));
        }
        end
    }

    /// The end of the run of `[0-9]` from `p`, which a read from inside the
    /// last run shares.
    pub(super) fn ascii_digits(&self, p: usize) -> usize {
        let (from, to) = self.x.ascii_digits.get();
        if from < p && p < to {
            return to;
        }
        let end = self.skip(p, |b| b.is_ascii_digit());
        if end > p {
            self.x.ascii_digits.set((p, end));
        }
        end
    }

    /// The end of the run of bytes of `class` from `p`, read no further than
    /// `p + most`.
    pub(super) fn bounded(&self, p: usize, most: usize, class: impl Fn(u8) -> bool) -> usize {
        let stop = (p + most).min(self.lim());
        let end = (p..stop)
            .find(|&i| !class(self.at(i)))
            .unwrap_or(stop.max(p));
        self.spend(end - p);
        end
    }

    /// `[\t\s]*` from `p`, which is `\s*`.
    pub(super) fn blanks(&self, p: usize) -> usize {
        self.spaces(p)
    }

    /// `\s+` at `p`.
    pub(super) fn spaces_run(&self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        (end > p).then_some(end)
    }

    /// `[-\w]+` at `p`.
    pub(super) fn dash_word_run(&self, p: usize) -> Option<usize> {
        let end = self.dash_word_end(p);
        (end > p).then_some(end)
    }

    /// The end of the run of `[-\w]` from `p`. A read from the start or the
    /// inside of the last run reuses that run's end.
    pub(super) fn dash_word_end(&self, p: usize) -> usize {
        let (from, to) = self.x.dash_words.get();
        if from <= p && p < to {
            return to;
        }
        let mut end = p;
        loop {
            if self.at(end) == b'-' && end < self.lim() {
                end += 1;
            } else if let Some(width) = self.word_at(end) {
                end += width;
            } else {
                break;
            }
        }
        self.spend(end - p);
        self.x.dash_words.set((p, end));
        end
    }

    /// `(?=\s*X)` for a byte of `set`: the white space from `p` and whether
    /// one of them follows.
    pub(super) fn spaces_then(&self, p: usize, set: &[u8]) -> Option<usize> {
        let end = self.spaces(p);
        set.contains(&self.at(end)).then_some(end)
    }

    /// The end of `word` at `p` as `(?i)` reads it: without regard to ASCII
    /// case, a character above ASCII standing for the letters it folds to,
    /// as `ſ` for `s` ([`case_fold::literal_end`]).
    pub(super) fn folded_end(&self, p: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s.get(..self.lim())?, p, word)
    }
}
