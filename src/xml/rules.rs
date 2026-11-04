//! The grammar's patterns, which rule tries which and in what order, and the
//! bytes a pattern can start with.

use memchr::{memchr, memchr2, memchr3};

use super::scope::S;
use super::state::Rule;

/// A pattern of the grammar: a rule it opens, or a match that adds a scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Pat {
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

impl Pat {
    /// The rule the pattern opens.
    /// It is `None` for a match.
    pub(super) fn opens(self) -> Option<Rule> {
        match self {
            Pat::Pi => Some(Rule::Pi),
            Pat::Doctype => Some(Rule::Doctype),
            Pat::CommentJsp => Some(Rule::CommentJsp),
            Pat::Comment => Some(Rule::Comment),
            Pat::NoContent => Some(Rule::NoContent),
            Pat::Tag => Some(Rule::Tag),
            Pat::JspProps => Some(Rule::JspProps),
            Pat::Jsp => Some(Rule::Jsp),
            Pat::Cdata => Some(Rule::Cdata),
            Pat::Double => Some(Rule::Double),
            Pat::Single => Some(Rule::Single),
            Pat::Subset => Some(Rule::Subset),
            Pat::EntityDecl => Some(Rule::EntityDecl),
            Pat::CommentBad => Some(Rule::CommentBad),
            Pat::Entity
            | Pat::Ampersand
            | Pat::PiAttr
            | Pat::ParameterEntity
            | Pat::Attribute
            | Pat::PageProp => None,
        }
    }

    /// The scope a match adds to the whole of its text.
    pub(super) fn name(self) -> Option<S> {
        match self {
            Pat::Entity => Some(S::ConstantEntity),
            Pat::Ampersand => Some(S::InvalidAmpersand),
            Pat::PiAttr => Some(S::AttrName),
            Pat::ParameterEntity => Some(S::ConstantParameterEntity),
            Pat::PageProp => Some(S::PageProps),
            Pat::Attribute
            | Pat::Pi
            | Pat::Doctype
            | Pat::CommentJsp
            | Pat::Comment
            | Pat::NoContent
            | Pat::Tag
            | Pat::JspProps
            | Pat::Jsp
            | Pat::Cdata
            | Pat::Double
            | Pat::Single
            | Pat::Subset
            | Pat::EntityDecl
            | Pat::CommentBad => None,
        }
    }
}

/// The patterns of `rule`, in the grammar's order.
pub(super) fn patterns(rule: Rule) -> &'static [Pat] {
    match rule {
        Rule::Root => &[
            Pat::Pi,
            Pat::Doctype,
            Pat::CommentJsp,
            Pat::Comment,
            Pat::NoContent,
            Pat::Tag,
            Pat::Entity,
            Pat::Ampersand,
            Pat::JspProps,
            Pat::Jsp,
            Pat::Cdata,
        ],
        Rule::Pi => &[Pat::PiAttr, Pat::Double, Pat::Single],
        Rule::Doctype => &[Pat::Subset],
        Rule::Subset => &[
            Pat::EntityDecl,
            Pat::ParameterEntity,
            Pat::CommentJsp,
            Pat::Comment,
        ],
        Rule::EntityDecl => &[Pat::Double, Pat::Single],
        Rule::Comment => &[Pat::CommentBad],
        Rule::NoContent | Rule::Tag => &[Pat::Attribute, Pat::Double, Pat::Single],
        Rule::JspProps => &[Pat::PageProp],
        Rule::Double | Rule::Single => &[Pat::Entity, Pat::Ampersand],
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
