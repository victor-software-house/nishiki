//! The contexts the Rust lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of Rust's.
const TAG: u16 = crate::tag::RUST;

/// An open context.
/// Each one is a TextMate-style begin/end rule of the Rust grammar.
/// It adds a scope, though some add none, and limits which rules apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside any context.
    Top,
    /// `fn name(` up to `{` or `;`.
    FnDef,
    /// `name(` up to `)`.
    Call,
    /// `Type<` up to `>`.
    Generic,
    /// `<[` up to `>`.
    AngleSq,
    /// `#[` up to `]`.
    Attr,
    /// `use ` up to `;`.
    Use,
    /// `extern crate` up to `;`.
    Import,
    /// `"` or `b"` up to `"`.
    Str,
    /// `r#"` up to `"#`.
    /// The hash count is the frame's payload.
    Raw,
    /// `'` or `b'` up to `'`.
    Char,
    /// `/*` up to `*/`.
    /// It nests.
    Block,
    /// `/**` up to `*/`.
    /// It nests.
    Doc,
}

impl Rule {
    /// The `Rule` as a frame stores it: the Rust tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The `Rule` a frame's code names.
    /// It is [`Rule::Top`] for a code that is not Rust's, such as the empty
    /// state's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Top;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Top)
    }

    const ALL: [Rule; 13] = [
        Rule::Top,
        Rule::FnDef,
        Rule::Call,
        Rule::Generic,
        Rule::AngleSq,
        Rule::Attr,
        Rule::Use,
        Rule::Import,
        Rule::Str,
        Rule::Raw,
        Rule::Char,
        Rule::Block,
        Rule::Doc,
    ];

    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Rule::Top | Rule::Generic | Rule::AngleSq => None,
            Rule::FnDef => Some(S::MetaFnDef),
            Rule::Call => Some(S::MetaFnCall),
            Rule::Attr => Some(S::MetaAttr),
            Rule::Use => Some(S::MetaUse),
            Rule::Import => Some(S::MetaImport),
            Rule::Str | Rule::Raw => Some(S::StrDouble),
            Rule::Char => Some(S::StrChar),
            Rule::Block => Some(S::CommentBlock),
            Rule::Doc => Some(S::CommentBlockDoc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Rule::ALL` is a hand copy of the enum: each entry must sit at its own
    /// discriminant, or a frame's code reads back as another rule.
    #[test]
    fn every_rule_round_trips_through_its_code() {
        for (index, rule) in Rule::ALL.iter().enumerate() {
            assert_eq!(usize::from(rule.code() & 0xff), index, "{rule:?}");
            assert_eq!(Rule::from_code(rule.code()), *rule);
        }
    }
}
