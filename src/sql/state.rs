//! The rules the lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the SQL lexer's.
const TAG: u16 = crate::tag::SQL;

/// A rule of the grammar that can stay open at the end of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule.
    Root,
    /// `[` up to `]`.
    Bracket,
    /// The outer rule of a `--` comment, which closes right after it.
    CommentDash,
    /// The outer rule of a `#` comment, which holds nothing and closes at
    /// once.
    CommentHash,
    /// `--` up to the line feed.
    LineComment,
    /// `/*` up to `*/`, which nests.
    BlockComment,
    /// `'` up to `'`.
    StrSingle,
    /// `` ` `` up to `` ` ``.
    StrBacktick,
    /// `"` up to `"`.
    StrDouble,
    /// `%{` up to `}`.
    StrBrace,
    /// `%r{` up to `}`.
    StrModr,
}

impl Rule {
    /// The rules in code order, which [`Rule::code`] numbers.
    pub(super) const ALL: [Rule; 11] = [
        Rule::Root,
        Rule::Bracket,
        Rule::CommentDash,
        Rule::CommentHash,
        Rule::LineComment,
        Rule::BlockComment,
        Rule::StrSingle,
        Rule::StrBacktick,
        Rule::StrDouble,
        Rule::StrBrace,
        Rule::StrModr,
    ];

    /// The rule as a frame stores it: the SQL tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// the SQL lexer's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }

    /// The scopes the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> &'static [S] {
        match self {
            Rule::Bracket => &[S::Bracketed],
            Rule::LineComment => &[S::CommentDash],
            Rule::BlockComment => &[S::CommentBlock],
            Rule::StrSingle => &[S::StrSingle],
            Rule::StrBacktick => &[S::StrBacktick],
            Rule::StrDouble => &[S::StrDouble],
            Rule::StrBrace => &[S::StrBraces],
            Rule::StrModr => &[S::StrModr],
            Rule::Root | Rule::CommentDash | Rule::CommentHash => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_codes_to_its_place_in_all() {
        for (index, rule) in Rule::ALL.iter().enumerate() {
            assert_eq!(usize::from(rule.code() & 0xff), index, "{rule:?}");
            assert_eq!(Rule::from_code(rule.code()), *rule);
        }
    }
}
