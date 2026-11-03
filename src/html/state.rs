//! The rules the lexer opens, which are frames of the interned state, and
//! what a frame's payload carries.

/// The high byte that marks a frame as one of the HTML lexer's.
const TAG: u16 = crate::tag::HTML;

/// A begin/end rule of the grammar, which can stay open at the end of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule. The payload is [`ROOT_EMBEDDED`] in a fence.
    Root,
    /// `<!--` up to `-->`.
    Comment,
    /// `<![CDATA[` up to `]]>`.
    Cdata,
    /// `<!DOCTYPE` up to `>`.
    Doctype,
    /// A `"` string of a doctype.
    DoctypeString,
    /// `<?xml` up to `?>`.
    Processing,
    /// A tag of the grammar's own list, a custom element or an unrecognized
    /// tag, up to its `>`. The payload is a [`TagInfo`].
    Tag,
    /// An `svg` or `math` element, up to its end tag. The payload is an
    /// [`ElementInfo`].
    Element,
    /// The start tag of an element whose attributes the line did not hold.
    ElementStart,
    /// The leading blanks and line around a `<style>` element.
    StyleWrap,
    /// The `<style>` element itself.
    StyleTag,
    /// The start tag of a `<style>` element, up to `>`.
    OpenStyle,
    /// The CSS of a `<style>` element. The payload is the CSS lexer's state.
    BodyCss,
    /// The leading blanks and line around a `<script>` element.
    ScriptWrap,
    /// The `<script>` element itself.
    ScriptTag,
    /// What follows `<script` up to its end tag.
    ScriptHead,
    /// The attributes of a `<script>` tag up to the `>` or a `type` that is
    /// not JavaScript.
    ScriptAttributes,
    /// A `>` up to the `<` of the end tag, around JavaScript.
    ScriptJsOuter,
    /// The JavaScript of a `<script>` element. The payload is the lexer's
    /// state.
    BodyJs,
    /// A `<script>` whose `type` is an HTML template.
    ScriptTemplateOuter,
    /// The start tag of a `<script>` that holds a template or another type.
    OpenScript,
    /// The HTML of a template.
    BodyTemplate,
    /// A `<script>` whose `type` is neither JavaScript nor a template.
    ScriptOtherOuter,
    /// The text of such a `<script>`.
    BodyOther,
    /// An attribute, up to the next one. The payload is an [`AttributeInfo`].
    Attribute,
    /// `=` and the value after it.
    Value,
    /// `=` and the value of a `style` attribute.
    ValueCss,
    /// `=` and the value of an event handler.
    ValueJs,
    /// The first value of a `style` attribute.
    WrapCss,
    /// The first value of an event handler.
    WrapJs,
    /// A quoted value. The payload is the quote.
    StringPlain,
    /// A quoted value of a `style` attribute.
    StringCss,
    /// A quoted value of an event handler.
    StringJs,
    /// A `//` comment in an event handler. The payload is the quote.
    JsLineComment,
    /// A `/* */` comment in an event handler. The payload is the quote.
    JsBlockComment,
    /// The text of a capture group that the grammar tokenizes with the
    /// attribute rules. It never outlives the line.
    Region,
    /// ERB: the blanks at the start of a line before a comment that the line
    /// does not close, up to the comment's end.
    ErbCommentWrap,
    /// ERB: the blanks at the start of a line before code that the line does
    /// not close, up to the code's end.
    ErbTagWrap,
    /// ERB: `<%#` up to `%>`.
    ErbComment,
    /// ERB: code that its line does not close, up to `%>`. The payload is the
    /// Ruby lexer's state.
    ErbBlock,
    /// ERB: code, up to `%>`. The payload is the Ruby lexer's state.
    ErbLine,
}

impl Rule {
    /// The rules in code order, which [`Rule::code`] numbers.
    const ALL: [Rule; 41] = [
        Rule::Root,
        Rule::Comment,
        Rule::Cdata,
        Rule::Doctype,
        Rule::DoctypeString,
        Rule::Processing,
        Rule::Tag,
        Rule::Element,
        Rule::ElementStart,
        Rule::StyleWrap,
        Rule::StyleTag,
        Rule::OpenStyle,
        Rule::BodyCss,
        Rule::ScriptWrap,
        Rule::ScriptTag,
        Rule::ScriptHead,
        Rule::ScriptAttributes,
        Rule::ScriptJsOuter,
        Rule::BodyJs,
        Rule::ScriptTemplateOuter,
        Rule::OpenScript,
        Rule::BodyTemplate,
        Rule::ScriptOtherOuter,
        Rule::BodyOther,
        Rule::Attribute,
        Rule::Value,
        Rule::ValueCss,
        Rule::ValueJs,
        Rule::WrapCss,
        Rule::WrapJs,
        Rule::StringPlain,
        Rule::StringCss,
        Rule::StringJs,
        Rule::JsLineComment,
        Rule::JsBlockComment,
        Rule::Region,
        Rule::ErbCommentWrap,
        Rule::ErbTagWrap,
        Rule::ErbComment,
        Rule::ErbBlock,
        Rule::ErbLine,
    ];

    /// The rule as a frame stores it: the HTML tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// the HTML lexer's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }

    /// Whether the text inside the rule sits under a scope that the
    /// grammar's injection leaves alone: a comment, or `meta.embedded`.
    pub(crate) fn shuts_out_injection(self) -> bool {
        matches!(
            self,
            Rule::Comment | Rule::StyleTag | Rule::ScriptTag | Rule::WrapCss | Rule::WrapJs
        )
    }

    /// Whether ERB's injection applies inside the rule, opened where it
    /// applies as `parent` says: not under a `comment` scope.
    pub(crate) fn erb_injection(self, parent: ErbInjection) -> ErbInjection {
        match self {
            Rule::Comment | Rule::ErbComment | Rule::JsLineComment | Rule::JsBlockComment => {
                ErbInjection::ShutOut
            }
            Rule::Root
            | Rule::Cdata
            | Rule::Doctype
            | Rule::DoctypeString
            | Rule::Processing
            | Rule::Tag
            | Rule::Element
            | Rule::ElementStart
            | Rule::StyleWrap
            | Rule::StyleTag
            | Rule::OpenStyle
            | Rule::BodyCss
            | Rule::ScriptWrap
            | Rule::ScriptTag
            | Rule::ScriptHead
            | Rule::ScriptAttributes
            | Rule::ScriptJsOuter
            | Rule::BodyJs
            | Rule::ScriptTemplateOuter
            | Rule::OpenScript
            | Rule::BodyTemplate
            | Rule::ScriptOtherOuter
            | Rule::BodyOther
            | Rule::Attribute
            | Rule::Value
            | Rule::ValueCss
            | Rule::ValueJs
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::StringPlain
            | Rule::StringCss
            | Rule::StringJs
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap
            | Rule::ErbBlock
            | Rule::ErbLine => parent,
        }
    }
}

/// Whether ERB's injection applies to a rule's text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ErbInjection {
    /// It applies.
    Applies,
    /// It does not: the text sits under a `comment` scope.
    ShutOut,
}

/// The payload of the root frame of a Markdown fence, where the injection
/// of the grammar does not apply, as it does not when another grammar
/// embeds this one.
pub(crate) const ROOT_EMBEDDED: u32 = 1;

/// The category the grammar sorts a tag into, which names its scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Category {
    Metadata,
    Structure,
    Inline,
    Object,
    Other,
}

impl Category {
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Category::Metadata => "metadata",
            Category::Structure => "structure",
            Category::Inline => "inline",
            Category::Object => "object",
            Category::Other => "other",
        }
    }

    const fn from_bits(bits: u32) -> Category {
        match bits {
            0 => Category::Metadata,
            1 => Category::Structure,
            2 => Category::Inline,
            3 => Category::Object,
            _ => Category::Other,
        }
    }

    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    const fn bits(self) -> u32 {
        self as u32
    }
}

/// Which tag rule matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum TagKind {
    /// `<name`, ending at `>`.
    Start,
    /// `</name`, ending at `>`.
    End,
    /// `<name` of a tag without content, ending at `>` or `/>`.
    Void,
    /// `<name-name`, ending at `>` or `/>`.
    CustomStart,
    /// `</name-name`, ending at `>`.
    CustomEnd,
    /// A tag the grammar does not know, ending at `>`, `/>` or ` />`.
    Unrecognized,
    /// A tag that html-derivative's own rule takes, `(</?)(\w[^<>\s]*)`,
    /// ending as an unrecognized tag does.
    Derivative,
}

impl TagKind {
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    const fn bits(self) -> u32 {
        self as u32
    }

    const fn from_bits(bits: u32) -> TagKind {
        match bits {
            0 => TagKind::Start,
            1 => TagKind::End,
            2 => TagKind::Void,
            3 => TagKind::CustomStart,
            4 => TagKind::CustomEnd,
            5 => TagKind::Unrecognized,
            _ => TagKind::Derivative,
        }
    }
}

/// What a [`Rule::Tag`] frame carries besides its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TagInfo {
    pub(crate) category: Category,
    pub(crate) kind: TagKind,
}

impl TagInfo {
    pub(crate) const fn bits(self) -> u32 {
        self.category.bits() | self.kind.bits() << 3
    }

    pub(crate) const fn from_bits(bits: u32) -> TagInfo {
        TagInfo {
            category: Category::from_bits(bits & 7),
            kind: TagKind::from_bits(bits >> 3 & 7),
        }
    }
}

/// The grammar of an element: `svg` or `math`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Namespace {
    Svg,
    Math,
}

impl Namespace {
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Namespace::Svg => "svg",
            Namespace::Math => "math",
        }
    }
}

/// How an element's tag is sorted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Flavor {
    /// A tag of the namespace's list.
    Known,
    /// An SVG tag that is deprecated.
    Deprecated,
    /// A tag the namespace does not list.
    Invalid,
}

/// What a [`Rule::Element`] or [`Rule::ElementStart`] frame carries besides
/// its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ElementInfo {
    pub(crate) namespace: Namespace,
    pub(crate) category: Category,
    pub(crate) flavor: Flavor,
    /// The `svg` or `math` element itself, whose rule differs from its
    /// children's.
    pub(crate) root: bool,
}

impl ElementInfo {
    #[expect(
        clippy::as_conversions,
        reason = "`From<bool>` is not callable in a `const fn`"
    )]
    pub(crate) const fn bits(self) -> u32 {
        let namespace = match self.namespace {
            Namespace::Svg => 0,
            Namespace::Math => 1,
        };
        let flavor = match self.flavor {
            Flavor::Known => 0,
            Flavor::Deprecated => 1,
            Flavor::Invalid => 2,
        };
        namespace | self.category.bits() << 1 | flavor << 4 | (self.root as u32) << 6
    }

    pub(crate) const fn from_bits(bits: u32) -> ElementInfo {
        ElementInfo {
            namespace: if bits & 1 == 0 {
                Namespace::Svg
            } else {
                Namespace::Math
            },
            category: Category::from_bits(bits >> 1 & 7),
            flavor: match bits >> 4 & 3 {
                0 => Flavor::Known,
                1 => Flavor::Deprecated,
                _ => Flavor::Invalid,
            },
            root: bits >> 6 & 1 == 1,
        }
    }
}

/// Which attribute rule matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum AttributeKind {
    /// A name of the namespace's list.
    Known,
    /// `style`, whose value is CSS.
    Style,
    /// `on…`, whose value is JavaScript.
    Event,
    /// `data-…`.
    Data,
    /// `align`, `bgcolor` and `border`.
    Deprecated,
    /// Any other name.
    Unrecognized,
}

impl AttributeKind {
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn bits(self) -> u32 {
        self as u32
    }

    pub(crate) const fn from_bits(bits: u32) -> AttributeKind {
        match bits {
            0 => AttributeKind::Known,
            1 => AttributeKind::Style,
            2 => AttributeKind::Event,
            3 => AttributeKind::Data,
            4 => AttributeKind::Deprecated,
            _ => AttributeKind::Unrecognized,
        }
    }
}

/// Packs the bits a rule carries with the interned id of the name the
/// rule's scope is built from.
pub(crate) const fn pack(info: u32, name: u32) -> u32 {
    info | name << 8
}

/// The bits of a payload made by [`pack`].
pub(crate) const fn info(payload: u32) -> u32 {
    payload & 0xff
}

/// The interned name of a payload made by [`pack`].
pub(crate) const fn name(payload: u32) -> u32 {
    payload >> 8
}
