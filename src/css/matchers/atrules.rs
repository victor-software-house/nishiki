//! At-rules.

use super::super::ahead::Find;
use super::super::engine::CssLike;
use super::chars::{Letters, Words};
use crate::rules::{Chars, Lexer};

#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: CssLike> Lexer<'_, G> {
    /// The lookahead `(["';{\s]|/\*|$)` and its kin at `p`: white space, a
    /// comment, the end, or one of `set`.
    fn at_follows(&self, p: usize, set: &[u8]) -> bool {
        self.space_at(p) > 0
            || self.starts(p, b"/*")
            || self.at_eol(p)
            || (p < self.lim() && set.contains(&self.s[p]))
    }

    /// `\G` at `p`.
    fn at_anchor(&self, p: usize) -> bool {
        self.g && p == self.pos
    }

    /// `(?<=})(?!\G)` at `p`.
    fn after_brace_not_anchor(&self, p: usize) -> bool {
        p > 0 && self.s.get(p - 1) == Some(&b'}') && !self.at_anchor(p)
    }

    /// `@` and `word`, in any case, at `p`: the end of the word.
    fn at_word(&self, p: usize, word: &str) -> Option<usize> {
        self.lit(p, b"@")?;
        self.lit_ci(p + 1, word)
    }

    /// `(?=@word(set))`: the zero-width begin of an at-rule that lists its
    /// header and body as separate rules.
    fn at_ahead(&self, p: usize, word: &str, set: &[u8]) -> Option<usize> {
        let end = self.at_word(p, word)?;
        self.at_follows(end, set).then_some(p)
    }

    /// `\G(@)word`: the header of such an at-rule, with `@` as group 1.
    fn at_header(&mut self, p: usize, word: &str) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        let end = self.at_word(p, word)?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// `@` and the name of a keyframes rule at `p`:
    /// `@(?:-(?:webkit|moz|o|ms)-)?keyframes`.
    fn keyframes_word(&self, p: usize) -> Option<usize> {
        self.lit(p, b"@")?;
        let start = p + 1;
        if self.at(start) == b'-' {
            let letters = self.letters_ci(start + 1, Letters::Plain);
            if self.in_list(
                start + 1,
                letters,
                Words::Spelled(&["moz", "ms", "o", "webkit"]),
            ) && self.at(letters) == b'-'
                && let Some(end) = self.lit_ci(letters + 1, "keyframes")
            {
                return Some(end);
            }
        }
        self.lit_ci(start, "keyframes")
    }

    /// `\A﻿?(?i:(?=\s*@charset\b))`: the start of the document.
    pub(in crate::css) fn at_rules_at0(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.first_line {
            return None;
        }
        let start = if self.starts(0, "\u{feff}".as_bytes()) {
            3
        } else {
            0
        };
        let word = self.spaces(start);
        let end = self.lit_ci(word, "@charset")?;
        self.boundary(end).then_some(start)
    }

    /// `;|(?=$)`
    pub(in crate::css) fn at_rules_at0_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b";").or_else(|| self.at_eol(p).then_some(p))
    }

    /// The seven ways a `@charset` goes wrong.
    ///
    /// They are tried in this order:
    /// - `\G((?!@charset)@\w+)`;
    /// - `\G(\s+)`;
    /// - `(@charset\S[^;]*)`;
    /// - `(?<=@charset)( {2,}|\t+)`;
    /// - `(?<=@charset )([^";]+)`;
    /// - `("[^"]+)$`;
    /// - `(?<=")([^;]+)`.
    pub(in crate::css) fn at_rules_at0_at0(&mut self, p: usize) -> Option<usize> {
        let lim = self.lim();
        if self.at_anchor(p) && self.at(p) == b'@' && p < lim && !self.starts(p, b"@charset") {
            let end = self.word_run(p + 1);
            if end > p + 1 {
                self.capture(1, p, end);
                return Some(end);
            }
        }
        if self.at_anchor(p) {
            let end = self.spaces(p);
            if end > p {
                self.capture(2, p, end);
                return Some(end);
            }
        }
        if self.starts(p, b"@charset") && p + 8 < lim && self.space_at(p + 8) == 0 {
            let end = self.skip_to(p + 9, b';');
            self.capture(3, p, end);
            return Some(end);
        }
        if p >= 8 && self.text(p - 8, p) == b"@charset" {
            let spaces = self.skip(p, |b| b == b' ');
            if spaces >= p + 2 {
                self.capture(4, p, spaces);
                return Some(spaces);
            }
            let tabs = self.skip(p, |b| b == b'\t');
            if tabs > p {
                self.capture(4, p, tabs);
                return Some(tabs);
            }
        }
        if p >= 9 && self.text(p - 9, p) == b"@charset " {
            let end = self.skip_to_any(p, b"\";");
            if end > p {
                self.capture(5, p, end);
                return Some(end);
            }
        }
        if self.at(p) == b'"' && p < lim && self.find(Find::Quote, p + 1).is_none() {
            self.capture(6, p, lim + 1);
            return Some(lim + 1);
        }
        if p > 0 && self.s.get(p - 1) == Some(&b'"') {
            let end = self.skip_to(p, b';');
            if end > p {
                self.capture(7, p, end);
                return Some(end);
            }
        }
        None
    }

    /// The end of the run of characters from `p` other than `stop`.
    /// The line feed after the line is one of them.
    fn skip_to(&self, p: usize, stop: u8) -> usize {
        self.skip_to_any(p, &[stop])
    }

    /// The end of the run of characters from `p` other than those in `stops`;
    /// the line feed after the line is one of them.
    fn skip_to_any(&self, p: usize, stops: &[u8]) -> usize {
        let lim = self.lim();
        let mut i = p;
        while i < lim && !stops.contains(&self.s[i]) {
            i += 1;
        }
        self.spend(i - p + 1);
        if i == lim { lim + 1 } else { i }
    }

    /// `((@)charset)(?=\s)`
    pub(in crate::css) fn at_rules_at0_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"@charset")?;
        (self.space_at(end) > 0).then(|| {
            self.capture(1, p, end);
            self.capture(2, p, p + 1);
            end
        })
    }

    /// `"|$`
    pub(in crate::css) fn at_rules_at0_at2_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"\"").or_else(|| self.at_eol(p).then_some(p))
    }

    /// `(?:\G|^)(?=[^"]+$)`
    pub(in crate::css) fn at_rules_at0_at2_at0(&mut self, p: usize) -> Option<usize> {
        let anchored = self.at_anchor(p) || p == 0;
        (anchored && self.find(Find::Quote, p).is_none()).then_some(p)
    }

    /// `(?i)((@)import)(?:\s+|$|(?=["']|/\*))`
    pub(in crate::css) fn at_rules_at1(&mut self, p: usize) -> Option<usize> {
        let word = self.at_word(p, "import")?;
        let end = if self.space_at(word) > 0 {
            self.spaces(word)
        } else if matches!(self.s.get(word), Some(b'"' | b'\'')) || self.starts(word, b"/*") {
            word
        } else {
            return None;
        };
        self.capture(1, p, word);
        self.capture(2, p, p + 1);
        Some(end)
    }

    /// `;`
    pub(in crate::css) fn at_rules_at1_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b";")
    }

    /// `\G\s*(?=/\*)`
    pub(in crate::css) fn at_rules_at1_at0(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        let end = self.spaces(p);
        self.starts(end, b"/*").then_some(end)
    }

    /// `(?<=\*/)\s*`
    pub(in crate::css) fn at_rules_at1_at0_end(&mut self, p: usize) -> Option<usize> {
        (p >= 2 && self.text(p - 2, p) == b"*/").then(|| self.spaces(p))
    }

    /// `\s+`
    pub(in crate::css) fn at_rules_at1_at3(&mut self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        (end > p).then_some(end)
    }

    /// `(?i)((@)font-face)(?=\s*|\{|/\*|$)`
    pub(in crate::css) fn at_rules_at2(&mut self, p: usize) -> Option<usize> {
        let end = self.at_word(p, "font-face")?;
        self.capture(1, p, end);
        self.capture(2, p, p + 1);
        Some(end)
    }

    /// `(?!\G)`
    pub(in crate::css) fn at_rules_at2_end(&mut self, p: usize) -> Option<usize> {
        (!self.at_anchor(p)).then_some(p)
    }

    /// `(?i)((@)(word))(?=["';@{\s]|/\*|$)`
    fn font_feature(&mut self, p: usize, word: &str) -> Option<usize> {
        let end = self.at_word(p, word)?;
        self.at_follows(end, b"\"';@{").then(|| {
            self.capture(1, p, end);
            self.capture(2, p, p + 1);
            end
        })
    }

    /// `annotation`
    pub(in crate::css) fn font_features_at0(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "annotation")
    }

    /// `character-variant`
    pub(in crate::css) fn font_features_at1(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "character-variant")
    }

    /// `ornaments`
    pub(in crate::css) fn font_features_at2(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "ornaments")
    }

    /// `styleset`
    pub(in crate::css) fn font_features_at3(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "styleset")
    }

    /// `stylistic`
    pub(in crate::css) fn font_features_at4(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "stylistic")
    }

    /// `swash`
    pub(in crate::css) fn font_features_at5(&mut self, p: usize) -> Option<usize> {
        self.font_feature(p, "swash")
    }

    /// `(?<=})`
    pub(in crate::css) fn font_features_at0_end(&mut self, p: usize) -> Option<usize> {
        (p > 0 && self.s.get(p - 1) == Some(&b'}')).then_some(p)
    }

    /// `(?i)(@)page(?=[:{\s]|/\*|$)`
    pub(in crate::css) fn at_rules_at3(&mut self, p: usize) -> Option<usize> {
        let end = self.at_word(p, "page")?;
        self.at_follows(end, b":{").then(|| {
            self.capture(1, p, p + 1);
            end
        })
    }

    /// `(?=\s*($|[:;{]))`
    ///
    /// Group 1 sits inside the lookahead, and the grammar's `captures` name it
    /// all the same.
    /// They name it only when it starts where the match ends.
    /// vscode-textmate skips a capture that starts beyond the match.
    pub(in crate::css) fn at_rules_at3_end(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        if after > self.lim() {
            return Some(p);
        }
        matches!(self.at(after), b':' | b';' | b'{').then(|| {
            if after == p {
                self.capture(1, after, after + 1);
            }
            p
        })
    }

    /// `(?i)(?=@media([(\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at4(&mut self, p: usize) -> Option<usize> {
        self.at_ahead(p, "media", b"(")
    }

    /// `(?<=})(?!\G)`
    pub(in crate::css) fn at_rules_at4_end(&mut self, p: usize) -> Option<usize> {
        self.after_brace_not_anchor(p).then_some(p)
    }

    /// `(?i)\G(@)media`
    pub(in crate::css) fn at_rules_at4_at0(&mut self, p: usize) -> Option<usize> {
        self.at_header(p, "media")
    }

    /// `(?i)(?=@counter-style(["';{\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at5(&mut self, p: usize) -> Option<usize> {
        self.at_ahead(p, "counter-style", b"\"';{")
    }

    /// `(?i)\G(@)counter-style`
    pub(in crate::css) fn at_rules_at5_at0(&mut self, p: usize) -> Option<usize> {
        self.at_header(p, "counter-style")
    }

    /// `(?=\s*\{)`
    pub(in crate::css) fn at_rules_at5_at0_end(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        (self.at(after) == b'{' && after < self.lim()).then_some(p)
    }

    /// `(?i)(?=@document(["';{\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at6(&mut self, p: usize) -> Option<usize> {
        self.at_ahead(p, "document", b"\"';{")
    }

    /// `(?i)\G(@)document`
    pub(in crate::css) fn at_rules_at6_at0(&mut self, p: usize) -> Option<usize> {
        self.at_header(p, "document")
    }

    /// `(?i)(?<![-\w])(url-prefix|domain|regexp)(\()`
    pub(in crate::css) fn at_rules_at6_at0_at0(&mut self, p: usize) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let name = self.wordish_run(p);
        let ok = self.at(name) == b'('
            && name < self.lim()
            && self.in_list(p, name, Words::Spelled(&["domain", "regexp", "url-prefix"]));
        ok.then(|| {
            self.capture(1, p, name);
            self.capture(2, name, name + 1);
            name + 1
        })
    }

    /// `(?i)(?=@(?:-(?:webkit|moz|o|ms)-)?keyframes(["';{\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at7(&mut self, p: usize) -> Option<usize> {
        let end = self.keyframes_word(p)?;
        self.at_follows(end, b"\"';{").then_some(p)
    }

    /// `(?i)\G(@)(?:-(?:webkit|moz|o|ms)-)?keyframes`
    pub(in crate::css) fn at_rules_at7_at0(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        let end = self.keyframes_word(p)?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// `(?i)(?<![-\w])(from|to)(?![-\w])|([-+]?(?:\d+(?:\.\d+)?|\.\d+)%)`
    pub(in crate::css) fn at_rules_at7_at1_at2(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.standalone_word(p, Words::Spelled(&["from", "to"])) {
            self.capture(1, p, end);
            return Some(end);
        }
        let sign = usize::from(matches!(self.at(p), b'+' | b'-') && p < self.lim());
        let whole = self.digit_end(p + sign);
        let mut end = p + sign;
        if whole > end {
            end = whole;
            if self.at(end) == b'.' && self.digit_at(end + 1).is_some() {
                end = self.digit_end(end + 1);
            }
        } else if self.at(end) == b'.' && self.digit_at(end + 1).is_some() {
            end = self.digit_end(end + 1);
        } else {
            return None;
        }
        (self.at(end) == b'%' && end < self.lim()).then(|| {
            self.capture(2, p, end + 1);
            end + 1
        })
    }

    /// `(?i)(?=@supports([(\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at8(&mut self, p: usize) -> Option<usize> {
        self.at_ahead(p, "supports", b"(")
    }

    /// `(?<=})(?!\G)|(?=;)`
    pub(in crate::css) fn at_rules_at8_end(&mut self, p: usize) -> Option<usize> {
        (self.after_brace_not_anchor(p) || (self.at(p) == b';' && p < self.lim())).then_some(p)
    }

    /// `(?i)\G(@)supports`
    pub(in crate::css) fn at_rules_at8_at0(&mut self, p: usize) -> Option<usize> {
        self.at_header(p, "supports")
    }

    /// `(?i)(?=@layer(?:[;{\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at9(&mut self, p: usize) -> Option<usize> {
        self.at_ahead(p, "layer", b";{")
    }

    /// `(?<=})(?!\G)|;`
    pub(in crate::css) fn at_rules_at9_end(&mut self, p: usize) -> Option<usize> {
        if self.after_brace_not_anchor(p) {
            return Some(p);
        }
        self.lit(p, b";")
    }

    /// `(?i)\G(@)layer`
    pub(in crate::css) fn at_rules_at9_at0(&mut self, p: usize) -> Option<usize> {
        self.at_header(p, "layer")
    }

    /// `(?i)((@)(-(ms|o)-)?viewport)(?=["';{\s]|/\*|$)`
    pub(in crate::css) fn at_rules_at10(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"@")?;
        let mut end = None;
        if self.at(p + 1) == b'-' {
            let letters = self.letters_ci(p + 2, Letters::Plain);
            if self.in_list(p + 2, letters, Words::Spelled(&["ms", "o"]))
                && self.at(letters) == b'-'
            {
                end = self.lit_ci(letters + 1, "viewport");
            }
        }
        let end = end.or_else(|| self.lit_ci(p + 1, "viewport"))?;
        self.at_follows(end, b"\"';{").then(|| {
            self.capture(1, p, end);
            self.capture(2, p, p + 1);
            end
        })
    }

    /// `(?=\s*[;@{])`
    pub(in crate::css) fn at_rules_at10_end(&mut self, p: usize) -> Option<usize> {
        let after = self.spaces(p);
        (matches!(self.at(after), b';' | b'@' | b'{') && after < self.lim()).then_some(p)
    }

    /// `(?i)((@)font-feature-values)(?=["';{\s]|/\*|$)\s*`
    pub(in crate::css) fn at_rules_at11(&mut self, p: usize) -> Option<usize> {
        let end = self.at_word(p, "font-feature-values")?;
        self.at_follows(end, b"\"';{").then(|| {
            self.capture(1, p, end);
            self.capture(2, p, p + 1);
            self.spaces(end)
        })
    }

    /// `(?i)((@)namespace)(?=["';\s]|/\*|$)`
    pub(in crate::css) fn at_rules_at13(&mut self, p: usize) -> Option<usize> {
        let end = self.at_word(p, "namespace")?;
        self.at_follows(end, b"\"';").then(|| {
            self.capture(1, p, end);
            self.capture(2, p, p + 1);
            end
        })
    }

    /// `;|(?=[@{])`
    pub(in crate::css) fn at_rules_at13_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b";")
            .or_else(|| (matches!(self.at(p), b'@' | b'{') && p < self.lim()).then_some(p))
    }

    /// The namespace prefix and its name.
    pub(in crate::css) fn at_rules_at13_at1(&mut self, p: usize) -> Option<usize> {
        let blank_before = p == 0 || self.space_before(p);
        if !(self.at_anchor(p) || p == 0 || blank_before) {
            return None;
        }
        let ahead = (blank_before && self.ident_start(p).is_some()) || {
            let after = self.spaces(p);
            self.comment_like(after).is_some()
        };
        if !ahead {
            return None;
        }
        let mut name = p;
        while name < self.lim() {
            if let Some(end) = self.name_end(name) {
                self.capture(1, p, name);
                self.capture(2, name, end);
                return Some(end);
            }
            name = self.char_end(name);
        }
        None
    }

    /// `(?i)(?=@[-\w]+[^;]+;s*$)`
    pub(in crate::css) fn at_rules_at14(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"@")?;
        if !self.wordish_at(p + 1) {
            return None;
        }
        let lim = self.lim();
        let semicolon = self.find(Find::Semicolon, p + 1)?;
        self.spend(semicolon - p);
        let trailing = self.skip(semicolon + 1, |b| b == b's');
        (semicolon >= p + 3 && trailing == lim).then_some(p)
    }

    /// `(?<=;)(?!\G)`
    pub(in crate::css) fn at_rules_at14_end(&mut self, p: usize) -> Option<usize> {
        (p > 0 && self.s.get(p - 1) == Some(&b';') && !self.at_anchor(p)).then_some(p)
    }

    /// `(?i)\G(@)[-\w]+`
    pub(in crate::css) fn at_rules_at14_at0(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) || self.at(p) != b'@' || p >= self.lim() {
            return None;
        }
        let end = self.wordish_run(p + 1);
        (end > p + 1).then(|| {
            self.capture(1, p, p + 1);
            end
        })
    }

    /// `(?i)(?=@[-\w]+([({\s]|/\*|$))`
    pub(in crate::css) fn at_rules_at15(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"@")?;
        let end = self.wordish_run(p + 1);
        (end > p + 1 && self.at_follows(end, b"({")).then_some(p)
    }
}
