//! The names of the package, import and export rules.

use super::super::engine::Lexer;
use super::names::is_upper_start;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\s(as)\s`.
    pub(super) fn import_as(&mut self, p: usize) -> Option<usize> {
        let before = self.space_at(p);
        if before == 0 {
            return None;
        }
        let word = self.lit(p + before, b"as")?;
        let after = self.space_at(word);
        if after == 0 {
            return None;
        }
        self.capture(1, p + before, word);
        Some(word + after)
    }

    /// `[A-Z\p{Lt}\p{Lu}]idrest`: an upper-case name.
    pub(super) fn upper_name(&self, p: usize) -> Option<usize> {
        self.name_with(p, is_upper_start)
    }

    /// `(quoted|(?:plain|[op]+))`: a package, import or export name.
    pub(super) fn path_name(&self, p: usize) -> Option<usize> {
        self.quoted_name(p).or_else(|| self.id(p))
    }

    /// `(given\s+)?(?:(upper)|(quoted|plain|[op]+))`: a selector.
    ///
    /// `given\s+` comes first.
    /// Its `\s+` gives blanks back only to a name that starts with a blank,
    /// which none does.
    /// Without a name after it, the selector starts again at `given`.
    pub(super) fn selector(&mut self, p: usize) -> Option<usize> {
        let with_given = self
            .given_prefix_end(p, true)
            .and_then(|given| self.selector_name(given).map(|found| (Some(given), found)));
        let (given, (start, group, end)) = match with_given {
            Some(found) => found,
            None => (None, self.selector_name(p)?),
        };
        if let Some(given) = given {
            self.capture(1, p, given);
        }
        self.capture(group, start, end);
        Some(end)
    }

    /// `(upper)|(quoted|plain|[op]+)` at `start`: where it starts, its group
    /// and its end.
    fn selector_name(&self, start: usize) -> Option<(usize, usize, usize)> {
        self.upper_name(start)
            .map(|end| (start, 2, end))
            .or_else(|| self.path_name(start).map(|end| (start, 3, end)))
    }

    /// The end of `given\s` or `given\s+` at `p`.
    fn given_prefix_end(&self, p: usize, plus: bool) -> Option<usize> {
        let word = self.lit(p, b"given")?;
        let width = self.space_at(word);
        if width == 0 {
            return None;
        }
        Some(if plus {
            self.space_end(word)
        } else {
            word + width
        })
    }

    /// `(given\s)?\s*(?:(upper)|(quoted|plain|[op]+))\s*(=>)\s*(?:
    /// (upper)|(quoted|plain|[op]+))\s*`: a renaming selector.
    ///
    /// The first name backtracks: an operator name or tail can take the
    /// `=>`.
    /// Without `given\s`, the name starts after the blanks.
    pub(super) fn renaming(&mut self, p: usize) -> Option<usize> {
        let with_given = self.given_prefix_end(p, false).and_then(|given| {
            self.renaming_from(self.space_end(given))
                .map(|found| (Some(given), found))
        });
        let (given, (start, group, name_end, arrow)) = match with_given {
            Some(found) => found,
            None => (None, self.renaming_from(self.space_end(p))?),
        };
        let target = self.space_end(arrow + 2);
        let (to_group, to_end) = self.renamed_to(target)?;
        let end = self.space_end(to_end);
        if let Some(given) = given {
            self.capture(1, p, given);
        }
        self.capture(group, start, name_end);
        self.capture(4, arrow, arrow + 2);
        self.capture(to_group, target, to_end);
        Some(end)
    }

    /// `(upper)|(quoted|plain|[op]+)` after a renaming's `=>`: the group and
    /// the end.
    fn renamed_to(&self, target: usize) -> Option<(usize, usize)> {
        self.upper_name(target)
            .map(|end| (5, end))
            .or_else(|| self.path_name(target).map(|end| (6, end)))
    }

    /// The first name of a renaming at `start` and the `=>` after it: the
    /// name's group, its end and where `=>` starts.
    /// A first name is accepted only where `=>` and a second name follow it.
    fn renaming_from(&self, start: usize) -> Option<(usize, usize, usize, usize)> {
        let arrow_at = |end: usize| {
            let arrow = self.space_end(end);
            self.starts(arrow, b"=>") && self.renamed_to(self.space_end(arrow + 2)).is_some()
        };
        let (group, end) = self
            .plain_end_where(start, is_upper_start, arrow_at)
            .map(|end| (2, end))
            .or_else(|| self.quoted_first_where(start, arrow_at).map(|end| (3, end)))?;
        Some((start, group, end, self.space_end(end)))
    }
}
