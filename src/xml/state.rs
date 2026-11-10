//! The rules the lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the XML lexer's.
const TAG: u16 = crate::tag::XML;

/// A begin/end rule of the grammar.
/// It can stay open at the end of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule.
    Root,
    /// `<?name` up to `?>`.
    Instruction,
    /// `<!DOCTYPE name` up to `>`.
    Doctype,
    /// `[` of a doctype up to `]`.
    Subset,
    /// `<!ENTITY name` up to `>`.
    EntityDeclaration,
    /// `<!--` up to `-->`.
    Comment,
    /// A `--` inside a comment.
    /// The grammar gives it no end.
    CommentBad,
    /// `<%--`.
    /// The grammar gives it no end.
    CommentJsp,
    /// `<name` that closes right away as `></name>`.
    NoContent,
    /// `<name` and `</name` up to `>`.
    Tag,
    /// `<%@` up to `%>`.
    JspDirective,
    /// `<%` up to `%>`, around Java.
    Jsp,
    /// `<![CDATA[` up to `]]>`.
    Cdata,
    /// `"` up to the next `"`.
    Double,
    /// `'` up to the next `'`.
    Single,
}

impl Rule {
    /// The rules in code order.
    /// [`Rule::code`] numbers them.
    const ALL: [Rule; 15] = [
        Rule::Root,
        Rule::Instruction,
        Rule::Doctype,
        Rule::Subset,
        Rule::EntityDeclaration,
        Rule::Comment,
        Rule::CommentBad,
        Rule::CommentJsp,
        Rule::NoContent,
        Rule::Tag,
        Rule::JspDirective,
        Rule::Jsp,
        Rule::Cdata,
        Rule::Double,
        Rule::Single,
    ];

    /// The rule as a frame stores it: the XML tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names.
    /// It is [`Rule::Root`] for a code that is not the XML lexer's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }

    /// The scope the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> Option<S> {
        match self {
            Rule::Instruction => Some(S::MetaPreprocessor),
            Rule::Doctype => Some(S::MetaDoctype),
            Rule::Subset => Some(S::MetaSubset),
            Rule::Comment => Some(S::CommentBlock),
            Rule::NoContent => Some(S::MetaNoContent),
            Rule::Tag => Some(S::MetaTag),
            Rule::JspDirective => Some(S::JavaProps),
            Rule::Jsp => Some(S::JavaEmbedded),
            Rule::Cdata => Some(S::Cdata),
            Rule::Double => Some(S::StringDouble),
            Rule::Single => Some(S::StringSingle),
            Rule::Root | Rule::EntityDeclaration | Rule::CommentBad | Rule::CommentJsp => None,
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
