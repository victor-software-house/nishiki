//! For each rule, the patterns that can match at a byte, in the order the
//! grammar tries them, so the engine tries only those where a byte stands.

use std::sync::OnceLock;

use super::grammar::{Pat, Rule};

/// Where a position lies, for the patterns that need a word boundary or the
/// start of the line.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Place {
    /// Anywhere else.
    Away,
    /// The start of the line.
    LineStart,
    /// Between two ASCII word characters.
    InWord,
}

impl Place {
    fn index(self) -> usize {
        match self {
            Place::Away => 0,
            Place::LineStart => 1,
            Place::InWord => 2,
        }
    }
}

/// The candidate patterns of one rule.
pub(crate) struct Tables {
    /// All patterns, the end pattern first: for the text after the line feed.
    all: Box<[Pat]>,
    /// The candidates for each byte, away from the start of the line (the
    /// first 256), at it (the next 256) and inside a word (the last 256).
    by_byte: Box<[Box<[Pat]>]>,
}

impl Tables {
    fn build(rule: Rule) -> Tables {
        let data = rule.data();
        let all: Vec<Pat> = data
            .end
            .into_iter()
            .chain(data.patterns.iter().copied())
            .collect();
        let mut by_byte = Vec::with_capacity(768);
        for place in [Place::Away, Place::LineStart, Place::InWord] {
            for byte in 0..=255_u8 {
                let list: Vec<Pat> = all
                    .iter()
                    .copied()
                    .filter(|pat| {
                        let start = pat.start();
                        (place == Place::LineStart || !start.bol)
                            && (place != Place::InWord || !start.wb)
                            && start.bytes[usize::from(byte >> 6)] >> (byte & 63) & 1 == 1
                    })
                    .collect();
                by_byte.push(list.into_boxed_slice());
            }
        }
        Tables {
            all: all.into_boxed_slice(),
            by_byte: by_byte.into_boxed_slice(),
        }
    }

    /// The patterns that can match at a position where byte `byte` stands.
    pub(crate) fn candidates(&self, place: Place, byte: u8) -> &[Pat] {
        &self.by_byte[place.index() * 256 + usize::from(byte)]
    }

    /// Every pattern, the end pattern first.
    pub(crate) fn all(&self) -> &[Pat] {
        &self.all
    }
}

/// The tables of `rule`, built on first use.
pub(crate) fn tables(rule: Rule) -> &'static Tables {
    static TABLES: [OnceLock<Tables>; Rule::ALL.len()] =
        [const { OnceLock::new() }; Rule::ALL.len()];
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl"
    )]
    let index = rule as usize;
    TABLES[index].get_or_init(|| Tables::build(rule))
}
