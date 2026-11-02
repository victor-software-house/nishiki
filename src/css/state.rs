//! How a rule is stored as a frame of the interned state.

use super::grammar::Rule;

/// The high byte that marks a frame as one of Css's.
const TAG: u16 = crate::tag::CSS;

impl Rule {
    /// The rule as a frame stores it: the Css tag in the high byte, the
    /// position in [`Rule::ALL`] in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// Css's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }
}
