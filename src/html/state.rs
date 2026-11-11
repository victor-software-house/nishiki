//! The rules the lexer opens, which are frames of the interned state, and
//! what a frame carries.

/// The high byte that marks a frame as one of the HTML lexer's.
const TAG: u16 = crate::tag::HTML;

/// A begin/end rule of the grammar, which can stay open at the end of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule.
    /// The carried value is [`ROOT_EMBEDDED`] or [`ROOT_DERIVATIVE_FIRST`]
    /// where another grammar embeds HTML.
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
    /// tag, up to its `>`. The carried value is a [`TagClass`].
    Tag,
    /// An `svg` or `math` element, up to its end tag. The carried value is an
    /// [`ElementClass`].
    Element,
    /// The start tag of an element whose attributes the line did not hold.
    ElementStart,
    /// The leading blanks and line around a `<style>` element.
    StyleWrap,
    /// The `<style>` element itself.
    StyleTag,
    /// The start tag of a `<style>` element, up to `>`.
    OpenStyle,
    /// The CSS of a `<style>` element. The carried value is the CSS lexer's
    /// state.
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
    /// The JavaScript of a `<script>` element. The carried value is the lexer's
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
    /// An attribute, up to the next one. The carried value is an
    /// [`AttributeInfo`].
    Attribute,
    /// `=` and the value after it.
    AttributeValue,
    /// `=` and the value of a `style` attribute.
    AttributeValueCss,
    /// `=` and the value of an event handler.
    AttributeValueJs,
    /// The first value of a `style` attribute.
    WrapCss,
    /// The first value of an event handler.
    WrapJs,
    /// A quoted value. The carried value is the quote.
    StringPlain,
    /// A quoted value of a `style` attribute.
    StringCss,
    /// A quoted value of an event handler.
    StringJs,
    /// A `//` comment in an event handler. The carried value is the quote.
    JsLineComment,
    /// A `/* */` comment in an event handler. The carried value is the quote.
    JsBlockComment,
    /// The text of a capture group that the grammar tokenizes with the
    /// attribute rules. It never outlives the line.
    Region,
    /// ERB: the blanks at the start of a line, before a comment that the line
    /// does not close, up to the comment's end.
    ErbCommentWrap,
    /// ERB: the blanks at the start of a line, before code that the line does
    /// not close, up to the code's end.
    ErbTagWrap,
    /// ERB: `<%#` up to `%>`.
    ErbComment,
    /// ERB: code that its line does not close, up to `%>`. The carried value is
    /// the Ruby lexer's state.
    ErbBlock,
    /// ERB: code, up to `%>`. The carried value is the Ruby lexer's state.
    ErbLine,
    /// Vue: `<!--` up to `-->`.
    VueComment,
    /// Vue: `<!-- @name` up to `-->`.
    VueCommentKeyValue,
    /// Vue: a tag that its line closes with `/>`, up to `/>`.
    VueSelfClosing,
    /// Vue: the `<` of a block up to its `>`.
    VueBlock,
    /// Vue: a block from its tag name to its end tag. The carried value packs a
    /// [`Block`](super::vue::Block) and the interned name.
    VueElement,
    /// Vue: `meta.tag-stuff`, a start tag's attributes up to `>`.
    VueTagStuff,
    /// Vue: a multi-line `<script>` block's attributes up to `>` or a `lang`
    /// of its own.
    VueScriptTagStuff,
    /// Vue: a multi-line `<style>` block's attributes up to `>` or a `lang` of
    /// its own.
    VueStyleTagStuff,
    /// Vue: a `<script>` block's rules after its tag name.
    VueScriptStuff,
    /// Vue: a `<style>` block's rules after its tag name.
    VueStyleStuff,
    /// Vue: the rule that a `lang` opens in a multi-line block. The carried
    /// value is the [`Lang`](super::vue::Lang).
    VueLangGroup,
    /// Vue: a block's body, which its language's lexer reads. The carried value
    /// is that lexer's state.
    /// Inside a key-value comment, it is the comment's JSON, with no scope.
    VueBody,
    /// Vue: a TypeScript body that runs while no line starts with `</script`.
    /// The carried value is the lexer's state.
    VueBodyWhile,
    /// Vue: a template's HTML.
    VueTemplateBody,
    /// Vue: a nested `<template>` up to its start tag's end. The carried value
    /// is its [`TemplateTag`](super::vue::TemplateTag).
    VueTemplateTag,
    /// Vue: a nested template's inside up to `</template`. The carried value is
    /// its [`TemplateTag`](super::vue::TemplateTag).
    VueTemplateTagEnd,
    /// Vue: a tag whose name starts with a capital, up to `/?>`. The carried
    /// value is the interned name.
    VueCapitalized,
    /// Vue: the attributes of such a tag.
    VueCapitalizedAttributes,
    /// Vue: a directive and its value. The carried value is 1 for a control
    /// directive.
    VueDirective,
    /// Vue: a directive's quoted value. The carried value is the quote.
    VueExpressionString,
    /// Vue: a directive's value that no quote opened.
    VueExpressionBare,
    /// Vue: the TypeScript of a directive's value. The carried value is the
    /// lexer's state.
    VueExpression,
    /// Blade: `{{--` up to `--}}`.
    BladeComment,
    /// Blade: the blanks before `<?` in a comment, when the line does not close
    /// the tag, up to the tag's end.
    BladeCommentPhpWrap,
    /// Blade: `<?php` in a comment that the line does not close, up to `?>`.
    /// The carried value is the PHP lexer's state.
    BladeCommentPhpBlock,
    /// Blade: `<?php` in a comment, up to `>`.
    BladeCommentPhpLine,
    /// Blade: the blanks before `<?` that the line does not close, up to the
    /// tag's end.
    BladePhpWrap,
    /// Blade: `<?php` that the line does not close, up to `?>`. The carried
    /// value is the PHP lexer's state.
    BladePhpBlock,
    /// Blade: `<?php` up to `>`.
    BladePhpLine,
    /// Blade: the code of such a tag, up to `?` before `>`. The carried value
    /// is the PHP lexer's state.
    BladePhpLineCode,
    /// Blade: `{{{` up to `}}}`. The carried value is the PHP lexer's state.
    BladeEchoTriple,
    /// Blade: `{{` up to `}}`. The carried value is the PHP lexer's state.
    BladeEcho,
    /// Blade: `{!!` up to `!!}`. The carried value is the PHP lexer's state.
    BladeEchoRaw,
    /// Blade: `@{{` up to `}}`. The carried value is the PHP lexer's state.
    BladeEchoEscaped,
    /// Blade: a directive of the grammar's list and its `(`, up to `)`. The
    /// carried value is the PHP lexer's state.
    BladeDirective,
    /// Blade: any other directive and its `(`, up to `)`. The carried value is
    /// the PHP lexer's state.
    BladeDirectiveCustom,
    /// Blade: the arguments of a closing directive, such as `@endif(`, up to
    /// `)`, as `comment.blade`.
    BladeArgumentsComment,
    /// Blade: a `(` in those arguments, up to its `)`.
    BladeBalance,
    /// Blade: `@php` up to `@endphp`. The carried value is the PHP lexer's
    /// state.
    BladePhpSection,
    /// Blade: a `:name="` binding up to its `"`. The carried value is the PHP
    /// lexer's state.
    BladeAttribute,
    /// Vue: the component inside a block in `lang="vue"`, up to `</`.
    /// Its patterns are Vue's root's.
    /// It runs on the stack of the component around it, as in vscode-textmate.
    VueComponent,
}

impl Rule {
    /// The rules in code order, which [`Rule::code`] numbers.
    const ALL: [Rule; 82] = [
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
        Rule::AttributeValue,
        Rule::AttributeValueCss,
        Rule::AttributeValueJs,
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
        Rule::VueComment,
        Rule::VueCommentKeyValue,
        Rule::VueSelfClosing,
        Rule::VueBlock,
        Rule::VueElement,
        Rule::VueTagStuff,
        Rule::VueScriptTagStuff,
        Rule::VueStyleTagStuff,
        Rule::VueScriptStuff,
        Rule::VueStyleStuff,
        Rule::VueLangGroup,
        Rule::VueBody,
        Rule::VueBodyWhile,
        Rule::VueTemplateBody,
        Rule::VueTemplateTag,
        Rule::VueTemplateTagEnd,
        Rule::VueCapitalized,
        Rule::VueCapitalizedAttributes,
        Rule::VueDirective,
        Rule::VueExpressionString,
        Rule::VueExpressionBare,
        Rule::VueExpression,
        Rule::BladeComment,
        Rule::BladeCommentPhpWrap,
        Rule::BladeCommentPhpBlock,
        Rule::BladeCommentPhpLine,
        Rule::BladePhpWrap,
        Rule::BladePhpBlock,
        Rule::BladePhpLine,
        Rule::BladePhpLineCode,
        Rule::BladeEchoTriple,
        Rule::BladeEcho,
        Rule::BladeEchoRaw,
        Rule::BladeEchoEscaped,
        Rule::BladeDirective,
        Rule::BladeDirectiveCustom,
        Rule::BladeArgumentsComment,
        Rule::BladeBalance,
        Rule::BladePhpSection,
        Rule::BladeAttribute,
        Rule::VueComponent,
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
            Rule::Comment
                | Rule::StyleTag
                | Rule::ScriptTag
                | Rule::WrapCss
                | Rule::WrapJs
                | Rule::VueComment
                | Rule::VueCommentKeyValue
        )
    }

    /// Whether a template's injection applies inside the rule, opened where
    /// it applies as `parent` says: not under a `comment` scope.
    /// It reads [`ERB_SHUT_OUT`] by the rule's discriminant.
    pub(crate) fn injection(self, parent: Injection) -> Injection {
        if ERB_SHUT_OUT[usize::from(self.code() & 0xff)] {
            Injection::ShutOut
        } else {
            parent
        }
    }

    /// Whether ERB's injection stays out of the rule's text, for
    /// [`ERB_SHUT_OUT`].
    const fn erb_shuts_out(self) -> bool {
        match self {
            Rule::Comment
            | Rule::ErbComment
            | Rule::JsLineComment
            | Rule::JsBlockComment
            | Rule::VueComment
            | Rule::VueCommentKeyValue => true,
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
            | Rule::AttributeValue
            | Rule::AttributeValueCss
            | Rule::AttributeValueJs
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::StringPlain
            | Rule::StringCss
            | Rule::StringJs
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap
            | Rule::ErbBlock
            | Rule::ErbLine
            | Rule::BladeComment
            | Rule::BladeCommentPhpWrap
            | Rule::BladeCommentPhpBlock
            | Rule::BladeCommentPhpLine
            | Rule::BladePhpWrap
            | Rule::BladePhpBlock
            | Rule::BladePhpLine
            | Rule::BladePhpLineCode
            | Rule::BladeEchoTriple
            | Rule::BladeEcho
            | Rule::BladeEchoRaw
            | Rule::BladeEchoEscaped
            | Rule::BladeDirective
            | Rule::BladeDirectiveCustom
            | Rule::BladeArgumentsComment
            | Rule::BladeBalance
            | Rule::BladePhpSection
            | Rule::BladeAttribute
            | Rule::VueSelfClosing
            | Rule::VueBlock
            | Rule::VueElement
            | Rule::VueTagStuff
            | Rule::VueScriptTagStuff
            | Rule::VueStyleTagStuff
            | Rule::VueScriptStuff
            | Rule::VueStyleStuff
            | Rule::VueLangGroup
            | Rule::VueBody
            | Rule::VueBodyWhile
            | Rule::VueTemplateBody
            | Rule::VueTemplateTag
            | Rule::VueTemplateTagEnd
            | Rule::VueCapitalized
            | Rule::VueCapitalizedAttributes
            | Rule::VueDirective
            | Rule::VueExpressionString
            | Rule::VueExpressionBare
            | Rule::VueExpression
            | Rule::VueComponent => false,
        }
    }
}

/// [`Rule::erb_shuts_out`] by the rule's discriminant, the low byte of its
/// code.
/// A load replaces a `match` over more rules than a 64-bit mask holds,
/// which LLVM lowers to a jump table on every frame push.
/// Every byte indexes it, so the read needs no bounds check.
const ERB_SHUT_OUT: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < Rule::ALL.len() {
        table[i] = Rule::ALL[i].erb_shuts_out();
        i += 1;
    }
    table
};

impl Rule {
    /// Whether Blade's injection applies inside the rule, opened where it
    /// applies as `parent` says, and how it ranks there.
    ///
    /// The injection has two selectors:
    /// - `text.html.php.blade - (meta.embedded | meta.tag |
    ///   comment.block.blade)`, which loses a tie;
    /// - `L:(text.html.php.blade meta.tag - (comment.block.blade |
    ///   meta.embedded.block.blade))`, which wins one.
    ///
    /// So inside a tag it leads, even in an attribute's embedded CSS or
    /// JavaScript, until a Blade comment or an `@php` section shuts it out.
    pub(crate) fn blade_injection(self, parent: Injection) -> Injection {
        match self {
            Rule::BladeComment | Rule::BladePhpSection => Injection::ShutOut,
            Rule::Cdata
            | Rule::Doctype
            | Rule::Processing
            | Rule::Tag
            | Rule::ElementStart
            | Rule::OpenStyle
            | Rule::OpenScript
            | Rule::ScriptAttributes => Injection::Leads,
            Rule::StyleTag
            | Rule::ScriptTag
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::BladePhpBlock
            | Rule::BladePhpLine
            | Rule::BladeCommentPhpWrap
            | Rule::BladeCommentPhpBlock
            | Rule::BladeCommentPhpLine => {
                if parent == Injection::Leads {
                    Injection::Leads
                } else {
                    Injection::ShutOut
                }
            }
            Rule::Root
            | Rule::Comment
            | Rule::DoctypeString
            | Rule::Element
            | Rule::StyleWrap
            | Rule::BodyCss
            | Rule::ScriptWrap
            | Rule::ScriptHead
            | Rule::ScriptJsOuter
            | Rule::BodyJs
            | Rule::ScriptTemplateOuter
            | Rule::BodyTemplate
            | Rule::ScriptOtherOuter
            | Rule::BodyOther
            | Rule::Attribute
            | Rule::AttributeValue
            | Rule::AttributeValueCss
            | Rule::AttributeValueJs
            | Rule::StringPlain
            | Rule::StringCss
            | Rule::StringJs
            | Rule::JsLineComment
            | Rule::JsBlockComment
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap
            | Rule::ErbComment
            | Rule::ErbBlock
            | Rule::ErbLine
            | Rule::BladePhpWrap
            | Rule::BladePhpLineCode
            | Rule::BladeEchoTriple
            | Rule::BladeEcho
            | Rule::BladeEchoRaw
            | Rule::BladeEchoEscaped
            | Rule::BladeDirective
            | Rule::BladeDirectiveCustom
            | Rule::BladeArgumentsComment
            | Rule::BladeBalance
            | Rule::BladeAttribute
            | Rule::VueComment
            | Rule::VueCommentKeyValue
            | Rule::VueSelfClosing
            | Rule::VueBlock
            | Rule::VueElement
            | Rule::VueTagStuff
            | Rule::VueScriptTagStuff
            | Rule::VueStyleTagStuff
            | Rule::VueScriptStuff
            | Rule::VueStyleStuff
            | Rule::VueLangGroup
            | Rule::VueBody
            | Rule::VueBodyWhile
            | Rule::VueTemplateBody
            | Rule::VueTemplateTag
            | Rule::VueTemplateTagEnd
            | Rule::VueCapitalized
            | Rule::VueCapitalizedAttributes
            | Rule::VueDirective
            | Rule::VueExpressionString
            | Rule::VueExpressionBare
            | Rule::VueExpression
            | Rule::VueComponent => parent,
        }
    }
}

/// Whether a template language's injection, such as ERB's, applies to a
/// rule's text.
/// It also marks how its matches rank against the rule's own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Injection {
    /// It applies, and loses a tie: a match of the rule that starts at the
    /// same place comes first.
    /// A selector without `L:` ranks an injection after the rule's patterns.
    Applies,
    /// It applies, and wins a tie: an `L:` selector ranks the injection
    /// before the rule's patterns.
    /// Blade's injection leads this way inside a `meta.tag` scope.
    Leads,
    /// It does not: the text sits under a `comment` scope.
    ShutOut,
}

impl Injection {
    /// Whether the injection's match at `at` comes before the rule's match
    /// at `candidate`.
    pub(crate) fn precedes(self, at: usize, candidate: usize) -> bool {
        match self {
            Injection::Applies | Injection::ShutOut => at < candidate,
            Injection::Leads => at <= candidate,
        }
    }
}

/// The carried value of the root frame of a Markdown fence.
/// The injection of the grammar does not apply there, as it does not when
/// another grammar embeds this one.
pub(crate) const ROOT_EMBEDDED: u32 = 1;

/// The carried value of the root frame of HTML embedded in a document whose
/// grammar compiles html-derivative's rules first.
/// Such HTML sits in a Markdown fence, or in PHP or Perl inside one, also in
/// a Svelte `@component` comment.
/// The injection does not apply, as with [`ROOT_EMBEDDED`].
/// The document is lexed as [`Document::DerivativeFirst`].
///
/// [`Document::DerivativeFirst`]: super::engine::Document::DerivativeFirst
pub(crate) const ROOT_DERIVATIVE_FIRST: u32 = 2;

/// The carried value of the root frame of HTML in a Svelte document: the HTML
/// of an `@component` comment's Markdown.
/// It is [`ROOT_DERIVATIVE_FIRST`] in every way but one.
/// Svelte's grammar injects its `$` prefixes into the document's every
/// `source.js`, so a `<script>`'s JavaScript takes them.
pub(crate) const ROOT_SVELTE: u32 = 3;

/// The carried value of the root frame of the HTML of a Blade heredoc,
/// `<<<BLADE`, in Blade's PHP.
/// The heredoc's patterns are `text.html.basic` and then Blade's own rules,
/// without the PHP tags that only Blade's injection holds.
/// Its `meta.embedded.blade` scope shuts the injection out of every rule it
/// opens but the tags, where the injection's `L:` selector still leads.
/// The document is lexed as [`Document::Blade`].
///
/// [`Document::Blade`]: super::engine::Document::Blade
pub(crate) const ROOT_BLADE_HEREDOC: u32 = 4;

/// The carried value of the root frame of a Blade heredoc's HTML under an
/// `@php` section or a Blade comment.
/// Their scopes shut Blade's injection out of the tags too.
/// It is [`ROOT_BLADE_HEREDOC`] in every other way.
pub(crate) const ROOT_BLADE_HEREDOC_SEALED: u32 = 5;

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

/// The tag rule that matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum TagRule {
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

impl TagRule {
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    const fn bits(self) -> u32 {
        self as u32
    }

    const fn from_bits(bits: u32) -> TagRule {
        match bits {
            0 => TagRule::Start,
            1 => TagRule::End,
            2 => TagRule::Void,
            3 => TagRule::CustomStart,
            4 => TagRule::CustomEnd,
            5 => TagRule::Unrecognized,
            _ => TagRule::Derivative,
        }
    }
}

/// How the grammar classes a tag: what a [`Rule::Tag`] frame carries besides
/// its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TagClass {
    pub(crate) category: Category,
    pub(crate) rule: TagRule,
}

impl TagClass {
    pub(crate) const fn bits(self) -> u32 {
        self.category.bits() | self.rule.bits() << 3
    }

    pub(crate) const fn from_bits(bits: u32) -> TagClass {
        TagClass {
            category: Category::from_bits(bits & 7),
            rule: TagRule::from_bits(bits >> 3 & 7),
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

/// How the grammar classes an element: what a [`Rule::Element`] or
/// [`Rule::ElementStart`] frame carries besides its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ElementClass {
    pub(crate) namespace: Namespace,
    pub(crate) category: Category,
    pub(crate) flavor: Flavor,
    /// The `svg` or `math` element itself, whose rule differs from its
    /// children's.
    pub(crate) root: bool,
}

impl ElementClass {
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

    pub(crate) const fn from_bits(bits: u32) -> ElementClass {
        ElementClass {
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

/// The attribute rule that matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum AttributeRule {
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

impl AttributeRule {
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn bits(self) -> u32 {
        self as u32
    }

    pub(crate) const fn from_bits(bits: u32) -> AttributeRule {
        match bits {
            0 => AttributeRule::Known,
            1 => AttributeRule::Style,
            2 => AttributeRule::Event,
            3 => AttributeRule::Data,
            4 => AttributeRule::Deprecated,
            _ => AttributeRule::Unrecognized,
        }
    }
}

/// Packs the bits a rule carries with the interned id of the name the
/// rule's scope is built from.
pub(crate) const fn pack(bits: u32, name: u32) -> u32 {
    bits | name << 8
}

/// The bits of a carried value made by [`pack`].
pub(crate) const fn carried_bits(carried: u32) -> u32 {
    carried & 0xff
}

/// The interned name of a carried value made by [`pack`].
pub(crate) const fn name(carried: u32) -> u32 {
    carried >> 8
}
