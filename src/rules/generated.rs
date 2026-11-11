//! What the generated matchers call: reads of the line, the recording of a
//! match's groups and the scan skip.
//!
//! `codegen/src/matchers.rs` writes each language's matchers.
//! A matcher reads the line through [`byte_at`] and [`char_at`], and records
//! its groups with [`record`].

use super::{Grammar, Lexer};

/// What a generated matcher returns: the end and each group's bounds.
/// `usize::MAX` marks a group that did not take part.
pub(crate) type Found<const W: usize> = Option<(usize, [usize; W])>;

/// The character at `pos` and its width in bytes.
/// At the end of a line the line feed stands there.
/// Reading spends no step: the loops of the generated code spend what they
/// take.
#[inline]
pub(crate) fn char_at<G: Grammar>(lexer: &Lexer<'_, G>, pos: usize) -> Option<(char, usize)> {
    let s = lexer.s;
    match s.get(pos) {
        Some(&b) if b < 0x80 => Some((char::from(b), 1)),
        Some(&b) => {
            let width = match b {
                0xf0.. => 4,
                0xe0.. => 3,
                _ => 2,
            };
            let bytes = s.get(pos..pos + width)?;
            let c = std::str::from_utf8(bytes).ok()?.chars().next()?;
            Some((c, width))
        }
        None if pos == s.len() && lexer.nl => Some(('\n', 1)),
        None => None,
    }
}

/// The byte at `pos`.
/// At the end of a line the line feed stands there.
/// Reading spends no step, as for [`char_at`].
#[inline]
pub(crate) fn byte_at<G: Grammar>(lexer: &Lexer<'_, G>, pos: usize) -> Option<u8> {
    match lexer.s.get(pos) {
        Some(&b) => Some(b),
        None if pos == lexer.s.len() && lexer.nl => Some(b'\n'),
        None => None,
    }
}

/// The start of the character that ends at `pos`.
#[inline]
pub(crate) fn prev_boundary<G: Grammar>(lexer: &Lexer<'_, G>, pos: usize) -> usize {
    let s = lexer.s;
    if pos > s.len() {
        return pos - 1;
    }
    let mut at = pos - 1;
    while at > 0 && s.get(at).is_some_and(|&b| b & 0xc0 == 0x80) {
        at -= 1;
    }
    at
}

/// Whether `c` is in `ranges`, sorted and disjoint.
#[inline]
pub(crate) fn in_ranges(ranges: &[(u32, u32)], c: char) -> bool {
    let code = u32::from(c);
    let at = ranges.partition_point(|&(_, last)| last < code);
    ranges.get(at).is_some_and(|&(first, _)| first <= code)
}

/// Records a generated matcher's groups past the whole match, and returns
/// its end.
///
/// It leaves group 0 to the lexer, which writes it before any reader runs.
/// `tokenize` writes it after the scan picks a match.
/// `emit_captures` writes it for every act and while pattern.
#[expect(
    clippy::inline_always,
    reason = "at one codegen unit, Lua's matcher left wider copies out of line, a call per try"
)]
#[inline(always)]
pub(crate) fn record<G: Grammar, const W: usize>(
    lexer: &mut Lexer<'_, G>,
    found: Found<W>,
) -> Option<usize> {
    let (end, groups) = found?;
    for group in 1..(W / 2).min(G::GROUPS) {
        let (start, stop) = (groups[2 * group], groups[2 * group + 1]);
        if start != usize::MAX {
            lexer.capture(group, start, stop);
        }
    }
    Some(end)
}

/// The starts a generated matcher's last failure proves fail too.
///
/// It is the scan skip of the `field-pattern-fixes` design, section 3.
/// It holds for the text it was found on, which `text` names.
/// One lexer reads one line, or a prefix of it while it tokenizes again.
///
/// Each field takes 32 bits, so a skip takes 12 bytes.
/// The lexer holds one per skipping pattern.
/// With seven 32-byte skips, Protocol Buffers' `lex` gained 2,211 calls to
/// `memcpy`.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Skip {
    /// The start that failed.
    from: u32,
    /// The first start the failure does not cover.
    to: u32,
    /// The text's length, times two, plus one when a line feed follows it.
    text: u32,
}

/// The text a skip holds for: its length, times two, plus one when a line
/// feed follows it.
/// A text past `u32::MAX / 2` bytes gives `u32::MAX`, which no skip holds.
#[inline]
fn skip_text<G: Grammar>(lexer: &Lexer<'_, G>) -> u32 {
    u32::try_from(lexer.s.len())
        .ok()
        .and_then(|len| len.checked_mul(2))
        .map_or(u32::MAX, |len| len | u32::from(lexer.nl))
}

impl Skip {
    /// The skip after a failure at `p`, where the run from `p` ends at
    /// `run_end`.
    /// A match of the prefix `P` begins with at most `reach` bytes of run
    /// characters.
    /// So every start within `reach` bytes of `run_end` is still tried.
    /// A text past `u32::MAX / 2` bytes gets no skip.
    pub(crate) fn after<G: Grammar>(
        lexer: &Lexer<'_, G>,
        p: usize,
        run_end: usize,
        reach: usize,
    ) -> Skip {
        let to = run_end.saturating_sub(reach).max(p + 1);
        let text = skip_text(lexer);
        match (u32::try_from(p), u32::try_from(to)) {
            (Ok(from), Ok(to)) if text != u32::MAX => Skip { from, to, text },
            _ => Skip::default(),
        }
    }

    /// Whether a try at `p` fails, by the last failure on this text.
    /// With no failure kept, `to` is 0.
    /// So a try first compares `p` with `to` alone.
    /// The start and the text are read only inside the range.
    #[expect(
        clippy::inline_always,
        reason = "one test before each try of a skipping pattern"
    )]
    #[inline(always)]
    pub(crate) fn covers<G: Grammar>(self, lexer: &Lexer<'_, G>, p: usize) -> bool {
        usize::try_from(self.to).is_ok_and(|to| p < to)
            && u32::try_from(p).is_ok_and(|p| self.from <= p)
            && self.text == skip_text(lexer)
    }
}
