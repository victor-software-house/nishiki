//! Media queries, feature queries and layer names.

use super::super::engine::CssLike;
use super::chars::{Letters, Words};
use crate::rules::{Chars, Lexer};

/// The words of the grammar's media features, without a `min-` or `max-`.
const FEATURES: &[&str] = &[
    "aspect-ratio",
    "color",
    "color-index",
    "height",
    "monochrome",
    "resolution",
    "width",
];

/// The features the grammar lists as they are, with no `min-` or `max-`.
const PLAIN_FEATURES: &[&str] = &["display-mode", "grid", "hover", "orientation", "scan"];

/// The features of devices, without a `min-` or `max-`.
const DEVICE_FEATURES: &[&str] = &["device-aspect-ratio", "device-height", "device-width"];

/// The keywords of media features.
const FEATURE_KEYWORDS: &[&str] = &[
    "browser",
    "fullscreen",
    "hover",
    "interlace",
    "landscape",
    "minimal-ui",
    "portrait",
    "progressive",
    "standalone",
];

impl<G: CssLike> Lexer<'_, G> {
    /// `/\*(?:[^*]|\*[^/])*\*/` at `p`: the end of the comment.
    /// As the expression reads it, a `*` takes the character after it too.
    /// A `*/` that starts at such a second character is therefore not seen.
    pub(super) fn comment_like(&self, p: usize) -> Option<usize> {
        self.lit(p, b"/*")?;
        let lim = self.lim();
        let mut i = p + 2;
        let end = loop {
            if i > lim {
                break None;
            }
            if i < lim && self.s[i] == b'*' {
                if self.at(i + 1) == b'/' && i + 1 < lim {
                    break Some(i + 2);
                }
                if i + 1 > lim {
                    break None;
                }
                i = if i + 1 == lim {
                    lim + 1
                } else {
                    self.char_end(i + 1)
                };
            } else {
                i = if i == lim { lim + 1 } else { self.char_end(i) };
            }
        };
        self.spend(end.unwrap_or(lim) - p);
        end
    }

    /// The lookbehind `\s|^|\*/` at `p`, with `extra` more characters.
    fn behind_blank_or_comment(&self, p: usize, extra: &[u8]) -> bool {
        p == 0
            || self.space_before(p)
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/"))
            || (p > 0 && self.s.get(p - 1).is_some_and(|b| extra.contains(b)))
    }

    /// The lookahead `\s|/\*|$` at `p`, with `extra` more characters.
    fn ahead_blank_or_comment(&self, p: usize, extra: &[u8]) -> bool {
        self.space_at(p) > 0
            || self.starts(p, b"/*")
            || self.at_eol(p)
            || self.s.get(p).is_some_and(|b| extra.contains(b))
    }

    /// `(?i)(?<![-\w])(layer)(\()`
    pub(in crate::css) fn layer_import_at0(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let name = self.lit_ci(p, "layer")?;
        (self.at(name) == b'(' && name < self.lim()).then(|| {
            self.cap(1, p, name);
            self.cap(2, name, name + 1);
            name + 1
        })
    }

    /// `\.`
    pub(in crate::css) fn layer_name_list_at2(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b".")
    }

    /// A name: `[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,
    /// 6}|.))*`
    pub(in crate::css) fn layer_name_list_at3(&mut self, p: usize) -> Option<usize> {
        self.name_end(p)
    }

    /// `(?i)(?<![-\w])(layer)(?![-\w])`
    pub(in crate::css) fn layer_import_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.standalone_word(p, Words::Spelled(&["layer"]))?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?=\s*[^;{])`
    pub(in crate::css) fn media_query_list(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        let ok = (after <= self.lim() && !matches!(self.at(after), b';' | b'{')) || after > p;
        ok.then_some(p)
    }

    /// `(?=\s*[;{])`
    pub(in crate::css) fn media_query_list_end(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        (matches!(self.at(after), b';' | b'{') && after < self.lim()).then_some(p)
    }

    /// `\G`
    pub(in crate::css) fn media_query(&mut self, p: usize) -> Option<usize> {
        (self.g && p == self.pos).then_some(p)
    }

    /// `(?i)(?<=^|[,\s]|\*/)(?:(all|print|screen|speech)|(aural|...|tv))(?=$|[,
    /// ;{\s]|/\*)`
    pub(in crate::css) fn media_types(&mut self, p: usize) -> Option<usize> {
        if !self.behind_blank_or_comment(p, b",") {
            return None;
        }
        let end = self.wordish_run(p);
        if end == p {
            return None;
        }
        let group = if self.in_list(
            p,
            end,
            Words::Spelled(&["all", "print", "screen", "speech"]),
        ) {
            1
        } else if self.in_list(
            p,
            end,
            Words::Spelled(&[
                "aural",
                "braille",
                "embossed",
                "handheld",
                "projection",
                "tty",
                "tv",
            ]),
        ) {
            2
        } else {
            return None;
        };
        self.ahead_blank_or_comment(end, b",;{").then(|| {
            self.cap(group, p, end);
            end
        })
    }

    /// `(?i)(?<=\s|^|,|\*/)(word)(?=[{\s]|/\*|$)`
    fn media_logical(&self, p: usize, word: &str) -> Option<usize> {
        if !self.behind_blank_or_comment(p, b",") {
            return None;
        }
        let end = self.lit_ci(p, word)?;
        self.ahead_blank_or_comment(end, b"{").then_some(end)
    }

    /// `only`
    pub(in crate::css) fn media_query_at3(&mut self, p: usize) -> Option<usize> {
        self.media_logical(p, "only")
    }

    /// `not`
    pub(in crate::css) fn media_query_at4(&mut self, p: usize) -> Option<usize> {
        self.media_logical(p, "not")
    }

    /// `(?i)(?<=\s|^|\*/|\))and(?=\s|/\*|$)`
    pub(in crate::css) fn media_query_at5(&mut self, p: usize) -> Option<usize> {
        if !self.behind_blank_or_comment(p, b")") {
            return None;
        }
        let end = self.lit_ci(p, "and")?;
        self.ahead_blank_or_comment(end, b"").then_some(end)
    }

    /// `,(?:(?:\s*,)+|(?=\s*[);{]))`
    pub(in crate::css) fn media_query_at6(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b",")?;
        let mut end = p + 1;
        let mut commas = 0;
        loop {
            let after = self.spaces(end);
            if self.at(after) == b',' && after < self.lim() {
                end = after + 1;
                commas += 1;
            } else {
                break;
            }
        }
        if commas > 0 {
            return Some(end);
        }
        let after = self.spaces(p + 1);
        (matches!(self.at(after), b')' | b';' | b'{') && after < self.lim()).then_some(p + 1)
    }

    /// The group of a media feature at `p` and its end.
    ///
    /// The vendor-prefixed form:
    ///
    /// ```text
    /// (?:[-_](?:webkit|...)|(?:mso|prince))-[-\w]+(?=\s*(?:/\*...\*/)?\s*[):])
    /// ```
    fn media_feature(&self, p: usize) -> Option<(usize, usize)> {
        let run = self.wordish_run(p);
        let (min_max, rest) = if self
            .lit_ci(p, "min-")
            .or_else(|| self.lit_ci(p, "max-"))
            .is_some()
        {
            (true, p + 4)
        } else {
            (false, p)
        };
        if run > p {
            if self.in_list(p, run, Words::Spelled(PLAIN_FEATURES))
                || self.in_list(p, run, Words::Spelled(FEATURES))
                || (min_max && self.in_list(rest, run, Words::Spelled(FEATURES)))
            {
                return Some((1, run));
            }
            if self.in_list(p, run, Words::Spelled(DEVICE_FEATURES))
                || (min_max && self.in_list(rest, run, Words::Spelled(DEVICE_FEATURES)))
            {
                return Some((2, run));
            }
        }
        let dash = matches!(self.at(p), b'-' | b'_');
        let start = if dash { p + 1 } else { p };
        let letters = self.letters_ci(start, Letters::Plain);
        let vendor_ok = if dash {
            self.in_list(
                start,
                letters,
                Words::Spelled(&[
                    "ah", "apple", "atsc", "epub", "hp", "khtml", "moz", "ms", "o", "rim", "ro",
                    "tc", "wap", "webkit", "xv",
                ]),
            )
        } else {
            self.is_word_ci(start, letters, "mso") || self.is_word_ci(start, letters, "prince")
        };
        if !vendor_ok || self.at(letters) != b'-' {
            return None;
        }
        let end = self.wordish_run(letters + 1);
        if end == letters + 1 {
            return None;
        }
        let before = self.spaces(end);
        let after = self.comment_like(before).map_or(before, |c| self.spaces(c));
        (matches!(self.at(after), b')' | b':') && after < self.lim()).then_some((3, end))
    }

    /// The media features.
    pub(in crate::css) fn media_features(&mut self, p: usize) -> Option<usize> {
        if !(p == 0
            || self.s.get(p - 1) == Some(&b'(')
            || self.space_before(p)
            || (p >= 2 && self.s.get(p - 2..p) == Some(b"*/")))
        {
            return None;
        }
        let (group, end) = self.media_feature(p)?;
        let ahead = self.space_at(end) > 0
            || self.at_eol(end)
            || matches!(self.s.get(end), Some(b')' | b':' | b'<' | b'=' | b'>'))
            || self.starts(end, b"/*");
        ahead.then(|| {
            self.cap(group, p, end);
            end
        })
    }

    /// `(?i)(?<=^|[:\s]|\*/)(?:portrait|...)(?=[)\s]|$)`
    pub(in crate::css) fn media_feature_keywords(&mut self, p: usize) -> Option<usize> {
        if !self.behind_blank_or_comment(p, b":") {
            return None;
        }
        let end = self.wordish_run(p);
        let ahead =
            self.at(end) == b')' && end < self.lim() || self.space_at(end) > 0 || self.at_eol(end);
        (end > p && self.in_list(p, end, Words::Spelled(FEATURE_KEYWORDS)) && ahead).then_some(end)
    }

    /// `:`
    pub(in crate::css) fn media_query_at8_at2(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b":")
    }

    /// `>=|<=|[<=>]`
    pub(in crate::css) fn media_query_at8_at3(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b">=")
            .or_else(|| self.lit(p, b"<="))
            .or_else(|| matches!(self.s.get(p), Some(b'<' | b'=' | b'>')).then_some(p + 1))
    }

    /// `(\d+)\s*(/)\s*(\d+)`
    pub(in crate::css) fn media_query_at8_at4(&mut self, p: usize) -> Option<usize> {
        let first = self.digit_end(p);
        if first == p {
            return None;
        }
        let slash = self.spaces(first);
        if self.at(slash) != b'/' || slash >= self.lim() {
            return None;
        }
        let second = self.spaces(slash + 1);
        let end = self.digit_end(second);
        (end > second).then(|| {
            self.cap(1, p, first);
            self.cap(2, slash, slash + 1);
            self.cap(3, second, end);
            end
        })
    }

    /// `(?i)(?<=[()\s]|^|\*/)(word)(?=[()\s]|/\*|$)`
    fn feature_operator(&self, p: usize, word: &str) -> Option<usize> {
        if !self.behind_blank_or_comment(p, b"()") {
            return None;
        }
        let end = self.lit_ci(p, word)?;
        self.ahead_blank_or_comment(end, b"()").then_some(end)
    }

    /// `and`
    pub(in crate::css) fn feature_query_operators_at0(&mut self, p: usize) -> Option<usize> {
        self.feature_operator(p, "and")
    }

    /// `not`
    pub(in crate::css) fn feature_query_operators_at1(&mut self, p: usize) -> Option<usize> {
        self.feature_operator(p, "not")
    }

    /// `or`
    pub(in crate::css) fn feature_query_operators_at2(&mut self, p: usize) -> Option<usize> {
        self.feature_operator(p, "or")
    }
}
