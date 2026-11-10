//! The grammar's patterns, which rule tries which and in what order, and the
//! bytes a pattern can start with.

use memchr::{memchr, memchr2, memchr3};

use super::scope::S;
use super::state::Rule;

/// A pattern of the grammar: a rule it opens, or a match that adds a scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Pattern {
    /// `<?name`.
    Pi,
    /// `<!DOCTYPE name`.
    Doctype,
    /// `<%--`.
    CommentJsp,
    /// `<!--`.
    Comment,
    /// `<name ...></name>`: a tag with nothing between its two halves.
    NoContent,
    /// `<name` and `</name`.
    Tag,
    /// `&name;`, `&#1;` and `&#x1;`.
    Entity,
    /// An `&` that starts no entity.
    Ampersand,
    /// `<%@`.
    JspProps,
    /// `<%`, `<%!` and `<%=`.
    Jsp,
    /// `<![CDATA[`.
    Cdata,
    /// A space and a name in a processing instruction.
    PiAttr,
    /// `"`.
    Double,
    /// `'`.
    Single,
    /// `[` of a doctype.
    Subset,
    /// `<!ENTITY name`.
    EntityDecl,
    /// `%name;`.
    ParameterEntity,
    /// `--` in a comment that does not end it.
    CommentBad,
    /// `name=` in a tag.
    Attribute,
    /// `page`, `include` and `taglib` in a page directive.
    PageProp,
}

impl Pattern {
    /// The rule the pattern opens.
    /// It is `None` for a match.
    pub(super) fn opens(self) -> Option<Rule> {
        match self {
            Pattern::Pi => Some(Rule::Pi),
            Pattern::Doctype => Some(Rule::Doctype),
            Pattern::CommentJsp => Some(Rule::CommentJsp),
            Pattern::Comment => Some(Rule::Comment),
            Pattern::NoContent => Some(Rule::NoContent),
            Pattern::Tag => Some(Rule::Tag),
            Pattern::JspProps => Some(Rule::JspProps),
            Pattern::Jsp => Some(Rule::Jsp),
            Pattern::Cdata => Some(Rule::Cdata),
            Pattern::Double => Some(Rule::Double),
            Pattern::Single => Some(Rule::Single),
            Pattern::Subset => Some(Rule::Subset),
            Pattern::EntityDecl => Some(Rule::EntityDecl),
            Pattern::CommentBad => Some(Rule::CommentBad),
            Pattern::Entity
            | Pattern::Ampersand
            | Pattern::PiAttr
            | Pattern::ParameterEntity
            | Pattern::Attribute
            | Pattern::PageProp => None,
        }
    }

    /// The scope a match adds to the whole of its text.
    pub(super) fn name(self) -> Option<S> {
        match self {
            Pattern::Entity => Some(S::ConstantEntity),
            Pattern::Ampersand => Some(S::InvalidAmpersand),
            Pattern::PiAttr => Some(S::AttrName),
            Pattern::ParameterEntity => Some(S::ConstantParameterEntity),
            Pattern::PageProp => Some(S::PageProps),
            Pattern::Attribute
            | Pattern::Pi
            | Pattern::Doctype
            | Pattern::CommentJsp
            | Pattern::Comment
            | Pattern::NoContent
            | Pattern::Tag
            | Pattern::JspProps
            | Pattern::Jsp
            | Pattern::Cdata
            | Pattern::Double
            | Pattern::Single
            | Pattern::Subset
            | Pattern::EntityDecl
            | Pattern::CommentBad => None,
        }
    }
}

/// The patterns of `rule`, in the grammar's order.
pub(super) fn patterns(rule: Rule) -> &'static [Pattern] {
    match rule {
        Rule::Root => &[
            Pattern::Pi,
            Pattern::Doctype,
            Pattern::CommentJsp,
            Pattern::Comment,
            Pattern::NoContent,
            Pattern::Tag,
            Pattern::Entity,
            Pattern::Ampersand,
            Pattern::JspProps,
            Pattern::Jsp,
            Pattern::Cdata,
        ],
        Rule::Pi => &[Pattern::PiAttr, Pattern::Double, Pattern::Single],
        Rule::Doctype => &[Pattern::Subset],
        Rule::Subset => &[
            Pattern::EntityDecl,
            Pattern::ParameterEntity,
            Pattern::CommentJsp,
            Pattern::Comment,
        ],
        Rule::EntityDecl => &[Pattern::Double, Pattern::Single],
        Rule::Comment => &[Pattern::CommentBad],
        Rule::NoContent | Rule::Tag => &[Pattern::Attribute, Pattern::Double, Pattern::Single],
        Rule::JspProps => &[Pattern::PageProp],
        Rule::Double | Rule::Single => &[Pattern::Entity, Pattern::Ampersand],
        Rule::CommentBad | Rule::CommentJsp | Rule::Jsp | Rule::Cdata => &[],
    }
}

/// Whether `b` can start a `\s`: an ASCII space, or the lead byte of one of
/// the Unicode spaces that `src/space.rs` reads.
fn space_lead(b: u8) -> bool {
    matches!(
        b,
        b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | 0xc2 | 0xe1 | 0xe2 | 0xe3
    )
}

/// In a tag: the end `>` and `/>`, an attribute after white space, a string.
fn in_tag(b: u8) -> bool {
    matches!(b, b'>' | b'/' | b'"' | b'\'') || space_lead(b)
}

/// In a doctype: the end after white space, and the subset.
fn in_doctype(b: u8) -> bool {
    matches!(b, b'>' | b'[') || space_lead(b)
}

/// In a processing instruction: the end, a space, a string.
fn in_instruction(b: u8) -> bool {
    matches!(b, b'?' | b'"' | b'\'' | b' ')
}

/// In a page directive: the end, and the first letter of each property.
fn in_directive(b: u8) -> bool {
    matches!(b, b'%' | b'p' | b'i' | b't')
}

/// The first byte at or after `from` that a pattern or the end of `rule` can
/// start with, or `None` when no byte of the line can.
///
/// A tag also starts where the line does, because its attribute pattern
/// reads `^`.
pub(super) fn next_start(rule: Rule, s: &[u8], from: usize) -> Option<usize> {
    let rest = s.get(from..)?;
    let found = match rule {
        Rule::Root => memchr2(b'<', b'&', rest),
        Rule::Double => memchr2(b'"', b'&', rest),
        Rule::Single => memchr2(b'\'', b'&', rest),
        Rule::Comment => memchr(b'-', rest),
        Rule::Cdata => memchr(b']', rest),
        Rule::Subset => memchr3(b'<', b'%', b']', rest),
        Rule::EntityDecl => memchr3(b'>', b'"', b'\'', rest),
        Rule::Tag | Rule::NoContent => {
            if from == 0 {
                return Some(0);
            }
            return rest.iter().position(|&b| in_tag(b)).map(|at| from + at);
        }
        Rule::Doctype => rest.iter().position(|&b| in_doctype(b)),
        Rule::Pi => rest.iter().position(|&b| in_instruction(b)),
        Rule::JspProps => rest.iter().position(|&b| in_directive(b)),
        Rule::CommentBad | Rule::CommentJsp | Rule::Jsp => None,
    };
    found.map(|at| from + at)
}
