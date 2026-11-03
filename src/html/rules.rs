//! What each rule is called, and the scopes of its captures: the data of the
//! grammar's begin, end and match rules, apart from where they match.

use super::engine::{Hit, Lexer, Part, Pat, packed};
use super::erb::Erb;
use super::scope::S;
use super::state::{
    AttributeKind, Category, ElementInfo, Flavor, Namespace, Rule, TagInfo, TagKind, info, name,
};

impl Lexer<'_> {
    /// The scopes of a rule's begin and end text and everything inside it,
    /// on `base`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule, in the order of the grammar"
    )]
    pub(super) fn name_stack(&mut self, base: u32, rule: Rule, payload: u32) -> u32 {
        let bits = info(payload);
        let word = name(payload);
        match rule {
            Rule::Root
            | Rule::StyleWrap
            | Rule::ScriptWrap
            | Rule::ScriptHead
            | Rule::ScriptJsOuter
            | Rule::ScriptTemplateOuter
            | Rule::ScriptOtherOuter
            | Rule::Value
            | Rule::ValueCss
            | Rule::ValueJs
            | Rule::BodyJs
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap => base,
            Rule::ErbComment => self.path(base, &[S::ErbComment]),
            Rule::ErbBlock => self.path(base, &[S::ErbBlock]),
            Rule::ErbLine => self.path(base, &[S::ErbLine]),
            Rule::Comment => self.path(base, &[S::CommentBlock]),
            Rule::Cdata => self.path(base, &[S::TagCdata]),
            Rule::Doctype => self.path(base, &[S::TagDoctype]),
            Rule::DoctypeString => self.path(base, &[S::StringDouble]),
            Rule::Processing => self.path(base, &[S::TagProcessing]),
            Rule::Tag => {
                let tag = TagInfo::from_bits(bits);
                let kind = match tag.kind {
                    TagKind::Start => ".start.html",
                    TagKind::End => ".end.html",
                    TagKind::Void => ".void.html",
                    TagKind::CustomStart => return self.path(base, &[S::TagCustomStart]),
                    TagKind::CustomEnd => return self.path(base, &[S::TagCustomEnd]),
                    TagKind::Derivative => return self.path(base, &[S::TagDerivative]),
                    TagKind::Unrecognized => {
                        return self.dynamic(
                            base,
                            &[
                                Part::Lit("meta.tag.other."),
                                Part::Name(word),
                                Part::Lit(".html"),
                            ],
                        );
                    }
                };
                self.dynamic(
                    base,
                    &[
                        Part::Lit("meta.tag."),
                        Part::Lit(tag.category.word()),
                        Part::Lit("."),
                        Part::Name(word),
                        Part::Lit(kind),
                    ],
                )
            }
            Rule::Element => {
                let element = ElementInfo::from_bits(bits);
                if element.flavor == Flavor::Invalid {
                    return self.path(base, &[S::ElementOtherInvalid]);
                }
                if element.root {
                    return self.dynamic(
                        base,
                        &[
                            Part::Lit("meta.element."),
                            Part::Lit(element.category.word()),
                            Part::Lit("."),
                            Part::Name(word),
                            Part::Lit(".html"),
                        ],
                    );
                }
                self.dynamic(
                    base,
                    &[
                        Part::Lit("meta.element."),
                        Part::Lit(element.category.word()),
                        Part::Lit("."),
                        Part::Lit(element.namespace.word()),
                        Part::Lit("."),
                        Part::Name(word),
                        Part::Lit(".html"),
                    ],
                )
            }
            Rule::ElementStart => {
                let element = ElementInfo::from_bits(bits);
                let scope = if element.root {
                    S::TagStartStructure
                } else if element.flavor == Flavor::Invalid {
                    S::TagOtherInvalidStart
                } else if element.flavor == Flavor::Deprecated {
                    S::TagStartOther
                } else {
                    match element.category {
                        Category::Metadata => S::TagStartMetadata,
                        Category::Structure => S::TagStartStructure,
                        Category::Inline => S::TagStartInline,
                        Category::Object => S::TagStartObject,
                        Category::Other => S::TagStartOther,
                    }
                };
                self.path(base, &[scope])
            }
            Rule::StyleTag | Rule::ScriptTag => self.path(base, &[S::Embedded]),
            Rule::OpenStyle => self.path(base, &[S::TagStyleStart]),
            Rule::OpenScript | Rule::ScriptAttributes => self.path(base, &[S::TagScriptStart]),
            Rule::BodyCss => self.path(base, &[S::SourceCss]),
            Rule::BodyTemplate => self.path(base, &[S::TextHtmlBasic]),
            Rule::BodyOther => self.path(base, &[S::SourceUnknown]),
            Rule::Attribute => {
                let prefix = match AttributeKind::from_bits(bits) {
                    AttributeKind::Style => return self.path(base, &[S::AttributeStyle]),
                    AttributeKind::Known | AttributeKind::Deprecated => "meta.attribute.",
                    AttributeKind::Event => "meta.attribute.event-handler.",
                    AttributeKind::Data => "meta.attribute.data-x.",
                    AttributeKind::Unrecognized => "meta.attribute.unrecognized.",
                };
                self.dynamic(
                    base,
                    &[Part::Lit(prefix), Part::Name(word), Part::Lit(".html")],
                )
            }
            Rule::WrapCss => self.path(base, &[S::EmbeddedLineCss]),
            Rule::WrapJs => self.path(base, &[S::EmbeddedLineJs]),
            Rule::StringPlain | Rule::StringCss | Rule::StringJs => {
                let quote = if payload == u32::from(b'"') {
                    S::StringDouble
                } else {
                    S::StringSingle
                };
                self.path(base, &[quote])
            }
            Rule::JsLineComment => self.path(base, &[S::CommentLineJs]),
            Rule::JsBlockComment => self.path(base, &[S::CommentBlockJs]),
        }
    }

    /// The scopes of a rule's content only, on its name's stack.
    pub(super) fn content_stack(&mut self, name: u32, rule: Rule) -> u32 {
        match rule {
            Rule::Cdata => self.path(name, &[S::InlineData]),
            Rule::StringCss => self.path(name, &[S::SourceCss]),
            Rule::StringJs => self.path(name, &[S::SourceJs]),
            Rule::ErbBlock | Rule::ErbLine => self.path(name, &[S::SourceRuby]),
            Rule::Root
            | Rule::Comment
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
            | Rule::JsLineComment
            | Rule::JsBlockComment
            | Rule::Region
            | Rule::ErbCommentWrap
            | Rule::ErbTagWrap
            | Rule::ErbComment => name,
        }
    }

    /// The rule that a begin match opens and what the rule carries.
    /// `$1` loses its leading dots, which would make the scope invalid.
    pub(super) fn opens(&mut self, hit: &Hit, top: &super::engine::Frame) -> Option<(Rule, u32)> {
        let [from, to, ..] = hit.at;
        Some(match hit.pat {
            Pat::Zero(Rule::ElementStart) => (Rule::ElementStart, packed(info(top.payload), 0)),
            Pat::Zero(rule) => (rule, 0),
            Pat::Processing => (Rule::Processing, 0),
            Pat::Comment => (Rule::Comment, 0),
            Pat::Doctype => (Rule::Doctype, 0),
            Pat::Cdata => (Rule::Cdata, 0),
            Pat::StyleWrap => (Rule::StyleWrap, 0),
            Pat::ScriptWrap => (Rule::ScriptWrap, 0),
            Pat::Tag => self.tag_opens(hit, from, to),
            Pat::Element => {
                let word = self.intern(from, to);
                (Rule::Element, packed(hit.info, word))
            }
            Pat::StyleTag => (Rule::StyleTag, 0),
            Pat::ScriptTag => (Rule::ScriptTag, 0),
            Pat::ScriptJs => (Rule::ScriptJsOuter, 0),
            Pat::Attribute => self.attribute_opens(hit, from, to),
            Pat::Equals => (value_rule(info(top.payload)), 0),
            Pat::Quote => (string_rule(top.rule), hit.info),
            Pat::DoctypeQuote => (Rule::DoctypeString, 0),
            Pat::JsLine => (Rule::JsLineComment, top.payload),
            Pat::JsBlock => (Rule::JsBlockComment, top.payload),
            Pat::Erb(kind) => (erb_rule(kind), 0),
            Pat::End
            | Pat::Inject
            | Pat::ElementVoid
            | Pat::EntityNamed
            | Pat::EntityDecimal
            | Pat::EntityHexadecimal
            | Pat::EntityAmbiguous
            | Pat::AttributeIllegal
            | Pat::EqualsIllegal
            | Pat::CommentIllegal
            | Pat::DoctypeKeyword
            | Pat::DoctypeWord
            | Pat::Unquoted
            | Pat::UnquotedCss
            | Pat::UnquotedJs
            | Pat::JsText
            | Pat::ErbHash => return None,
        })
    }

    /// The tag rule a `Pat::Tag` match opens, carrying its kind and, unless
    /// it is a custom element, its interned name.
    fn tag_opens(&mut self, hit: &Hit, from: usize, to: usize) -> (Rule, u32) {
        let tag = TagInfo::from_bits(hit.info & 0x3f);
        let word = match tag.kind {
            TagKind::CustomStart | TagKind::CustomEnd => 0,
            TagKind::Start
            | TagKind::End
            | TagKind::Void
            | TagKind::Unrecognized
            | TagKind::Derivative => self.intern(from, to),
        };
        (Rule::Tag, packed(hit.info & 0x3f, word))
    }

    /// The attribute rule a `Pat::Attribute` match opens, carrying its kind
    /// and, unless it is `style`, its interned name without leading dots.
    fn attribute_opens(&mut self, hit: &Hit, from: usize, to: usize) -> (Rule, u32) {
        let from = from + self.s[from..to].iter().take_while(|&&b| b == b'.').count();
        let word = if hit.info == AttributeKind::Style.bits() {
            0
        } else {
            self.intern(from, to)
        };
        (Rule::Attribute, packed(hit.info, word))
    }

    /// Emits the capture groups of a begin match, on the rule's name stack
    /// `frame.name`.
    pub(super) fn begin_captures(
        &mut self,
        hit: &Hit,
        frame: &super::engine::Frame,
        _parent: &super::engine::Frame,
    ) {
        let base = frame.name;
        let (start, end) = (hit.start, hit.end);
        let [from, to, ..] = hit.at;
        match hit.pat {
            Pat::Processing => {
                self.cap(base, start, start + 2, &[S::TagPunctuation]);
                self.cap(base, start + 2, end, &[S::NameTag]);
            }
            Pat::Comment => self.cap(base, start, end, &[S::CommentPunctuation]),
            Pat::Doctype | Pat::Cdata => self.cap(base, start, end, &[S::TagBegin]),
            Pat::StyleWrap | Pat::ScriptWrap => {
                self.cap(base, start, end, &[S::WhitespaceLeading]);
            }
            Pat::Tag => {
                self.cap(base, start, from, &[S::TagBegin]);
                let kind = TagInfo::from_bits(hit.info & 0x3f).kind;
                let scopes: &[S] = match (kind, hit.info >> 8) {
                    (TagKind::Unrecognized, _) => &[S::NameTag, S::UnrecognizedTag],
                    (_, 1) => &[S::NameTag, S::Deprecated],
                    (_, 2) => &[S::NameTag, S::NoLongerSupported],
                    _ => &[S::NameTag],
                };
                self.cap(base, from, to, scopes);
            }
            Pat::Element => self.element_begin_captures(hit, frame),
            Pat::StyleTag => self.embedded_start(base, start, end, S::TagStyleStart),
            Pat::ScriptTag => self.embedded_start(base, start, end, S::TagScriptStart),
            Pat::ScriptJs => {
                self.cap(base, start, end, &[S::TagScriptStart, S::TagEnd]);
            }
            Pat::Attribute => {
                let scope = if AttributeKind::from_bits(hit.info) == AttributeKind::Deprecated {
                    S::AttributeNameDeprecated
                } else {
                    S::AttributeName
                };
                self.cap(base, start, end, &[scope]);
            }
            Pat::Equals => self.cap(base, start, end, &[S::KeyValue]),
            Pat::Quote => self.cap(base, start, end, &[S::StringBegin]),
            Pat::JsLine => self.cap(base, start, end, &[S::CommentPunctuationJs]),
            Pat::JsBlock => self.cap(base, start, end, &[S::CommentBeginJs]),
            Pat::Erb(kind) => {
                let scope = match kind {
                    Erb::CommentWrap => S::ErbCommentLeading,
                    Erb::TagWrap => S::ErbEmbeddedLeading,
                    Erb::Comment => S::ErbCommentBegin,
                    Erb::Block | Erb::Line => S::ErbBegin,
                };
                self.cap(base, start, end, &[scope]);
            }
            Pat::End
            | Pat::Inject
            | Pat::Zero(_)
            | Pat::ElementVoid
            | Pat::DoctypeQuote
            | Pat::EntityNamed
            | Pat::EntityDecimal
            | Pat::EntityHexadecimal
            | Pat::EntityAmbiguous
            | Pat::AttributeIllegal
            | Pat::EqualsIllegal
            | Pat::CommentIllegal
            | Pat::DoctypeKeyword
            | Pat::DoctypeWord
            | Pat::Unquoted
            | Pat::UnquotedCss
            | Pat::UnquotedJs
            | Pat::JsText
            | Pat::ErbHash => {}
        }
    }

    /// The begin of a `<style>` or `<script>` element: the whole match is
    /// the start tag, with the bracket and the name in it.
    fn embedded_start(&mut self, base: u32, start: usize, end: usize, tag: S) {
        let tag_stack = self.path(base, &[tag]);
        let stack = self.path(tag_stack, &[S::TagBegin]);
        self.cap_stack(base, start, start + 1, stack);
        let stack = self.path(tag_stack, &[S::NameTag]);
        self.cap_stack(base, start + 1, end, stack);
    }

    /// Emits a match rule's text under its scopes.
    /// The group that the JavaScript lexes does not carry the scope of the
    /// group around it.
    pub(super) fn match_captures(&mut self, hit: &Hit, top: &super::engine::Frame) {
        let base = top.content;
        let (start, end) = (hit.start, hit.end);
        let [from, to, semi, ..] = hit.at;
        match hit.pat {
            Pat::EntityNamed => {
                let word = self.intern(from, to);
                let stack = self.dynamic(
                    base,
                    &[
                        Part::Lit("constant.character.entity.named."),
                        Part::Name(word),
                        Part::Lit(".html"),
                    ],
                );
                let punctuation = self.path(stack, &[S::EntityPunctuation]);
                self.cap_stack(base, start, start + 1, punctuation);
                self.cap_stack(base, start + 1, semi, stack);
                self.cap_stack(base, semi, end, punctuation);
            }
            Pat::EntityDecimal | Pat::EntityHexadecimal => {
                let scope = if hit.pat == Pat::EntityDecimal {
                    S::EntityDecimal
                } else {
                    S::EntityHexadecimal
                };
                let stack = self.path(base, &[scope]);
                let punctuation = self.path(stack, &[S::EntityPunctuation]);
                self.cap_stack(base, start, start + 1, punctuation);
                self.cap_stack(base, start + 1, end, stack);
            }
            Pat::EntityAmbiguous => self.cap(base, start, end, &[S::AmbiguousAmpersand]),
            Pat::AttributeIllegal => self.cap(base, start, end, &[S::CharacterNotAllowed]),
            Pat::EqualsIllegal => self.cap(base, start, end, &[S::UnexpectedEquals]),
            Pat::CommentIllegal => self.cap(base, start, end, &[S::CommentIllegal]),
            Pat::DoctypeKeyword => self.cap(base, start, end, &[S::NameTag]),
            Pat::DoctypeWord => self.cap(base, start, end, &[S::AttributeName]),
            Pat::Unquoted => self.cap(base, start, end, &[S::Unquoted]),
            Pat::UnquotedCss => self.cap(base, start, end, &[S::Unquoted, S::SourceCss]),
            Pat::UnquotedJs => {
                let stack = self.path(base, &[S::Unquoted]);
                self.javascript(start, end, stack);
            }
            Pat::JsText => self.javascript(start, end, base),
            Pat::ErbHash => {
                let stack = self.path(base, &[S::ErbHashComment]);
                let punctuation = self.path(stack, &[S::ErbHashPunctuation]);
                self.cap_stack(base, start, start + 1, punctuation);
                self.cap_stack(base, start + 1, end, stack);
            }
            Pat::End
            | Pat::Inject
            | Pat::Zero(_)
            | Pat::Processing
            | Pat::Comment
            | Pat::Doctype
            | Pat::Cdata
            | Pat::StyleWrap
            | Pat::ScriptWrap
            | Pat::Tag
            | Pat::Element
            | Pat::ElementVoid
            | Pat::StyleTag
            | Pat::ScriptTag
            | Pat::ScriptJs
            | Pat::Attribute
            | Pat::Equals
            | Pat::Quote
            | Pat::DoctypeQuote
            | Pat::JsLine
            | Pat::JsBlock
            | Pat::Erb(_) => {}
        }
    }

    /// Emits the capture groups of an end match, on the rule's name stack.
    pub(super) fn end_captures(&mut self, top: &super::engine::Frame, hit: &Hit) {
        let base = top.name;
        let (start, end) = (hit.start, hit.end);
        match top.rule {
            Rule::Comment => self.cap(base, start, end, &[S::CommentPunctuation]),
            Rule::Processing => self.cap(base, start, end, &[S::TagPunctuation]),
            Rule::Cdata | Rule::Doctype | Rule::OpenStyle | Rule::OpenScript | Rule::Tag => {
                self.cap(base, start, end, &[S::TagEnd]);
            }
            Rule::ElementStart => self.cap(base, start, end, &[S::TagEnd]),
            Rule::Element => self.element_end_captures(top, hit),
            Rule::StyleWrap | Rule::ScriptWrap => {
                self.cap(base, start, end, &[S::WhitespaceTrailing]);
            }
            Rule::StyleTag => {
                let [name_start, name_end, gt, ..] = hit.at;
                let tag = self.path(base, &[S::TagStyleEnd]);
                let open = self.path(tag, &[S::TagBegin]);
                let ignored = self.path(open, &[S::CssIgnored]);
                self.cap_stack(base, start, start + 1, ignored);
                self.cap_stack(base, start + 1, start + 2, open);
                let name_stack = self.path(tag, &[S::NameTag]);
                self.cap_stack(base, name_start, name_end, name_stack);
                self.tok(tag, gt);
                let close = self.path(tag, &[S::TagEnd]);
                self.cap_stack(base, gt, end, close);
                self.tok(tag, end);
            }
            Rule::ScriptTag => {
                let tag = self.path(base, &[S::TagScriptEnd]);
                let open = self.path(tag, &[S::TagBegin]);
                self.cap_stack(base, start, start + 1, open);
                let name_stack = self.path(tag, &[S::NameTag]);
                self.cap_stack(base, start + 1, end - 1, name_stack);
                let close = self.path(tag, &[S::TagEnd]);
                self.cap_stack(base, end - 1, end, close);
            }
            Rule::ScriptJsOuter => {
                self.cap(
                    base,
                    start,
                    end,
                    &[S::TagScriptEnd, S::TagBegin, S::JsIgnored],
                );
            }
            Rule::ScriptTemplateOuter => {
                self.cap(
                    base,
                    start,
                    end,
                    &[S::TagScriptEnd, S::TagBegin, S::TextHtmlBasic],
                );
            }
            Rule::ScriptOtherOuter => {
                self.cap(base, start, end, &[S::TagScriptEnd, S::TagBegin]);
            }
            Rule::StringPlain => self.cap(base, start, end, &[S::StringEnd]),
            Rule::StringCss => self.cap(base, start, end, &[S::StringEnd, S::SourceCss]),
            Rule::StringJs => self.cap(base, start, end, &[S::StringEnd, S::SourceJs]),
            Rule::JsBlockComment if end > start => {
                self.cap(base, start, end, &[S::CommentEndJs]);
            }
            Rule::ErbCommentWrap => self.cap(base, start, end, &[S::ErbCommentTrailing]),
            Rule::ErbTagWrap => self.cap(base, start, end, &[S::ErbEmbeddedTrailing]),
            Rule::ErbComment => self.cap(base, start, end, &[S::ErbCommentEnd]),
            Rule::ErbBlock | Rule::ErbLine => {
                let gt = hit.at[0];
                self.cap(base, start, gt, &[S::ErbEnd, S::SourceRuby]);
                self.cap(base, gt, end, &[S::ErbEnd]);
            }
            Rule::JsBlockComment
            | Rule::Root
            | Rule::DoctypeString
            | Rule::BodyCss
            | Rule::ScriptHead
            | Rule::ScriptAttributes
            | Rule::BodyJs
            | Rule::BodyTemplate
            | Rule::BodyOther
            | Rule::Attribute
            | Rule::Value
            | Rule::ValueCss
            | Rule::ValueJs
            | Rule::WrapCss
            | Rule::WrapJs
            | Rule::JsLineComment
            | Rule::Region => {}
        }
    }

    /// The scopes of an element's tag text on `base`: `meta.tag.…` for the
    /// kind of tag (`start`, `end`, `void`).
    fn element_tag(&mut self, base: u32, element: ElementInfo, word: u32, kind: &str) -> u32 {
        if element.flavor == Flavor::Invalid {
            let scope = match kind {
                "start" => S::TagOtherInvalidStart,
                "end" => S::TagOtherInvalidEnd,
                _ => S::TagOtherInvalidVoid,
            };
            return self.path(base, &[scope]);
        }
        let kind = match kind {
            "start" => ".start.html",
            "end" => ".end.html",
            _ => ".void.html",
        };
        if element.root {
            return self.dynamic(
                base,
                &[
                    Part::Lit("meta.tag."),
                    Part::Lit(element.category.word()),
                    Part::Lit("."),
                    Part::Name(word),
                    Part::Lit(kind),
                ],
            );
        }
        self.dynamic(
            base,
            &[
                Part::Lit("meta.tag."),
                Part::Lit(element.category.word()),
                Part::Lit("."),
                Part::Lit(element.namespace.word()),
                Part::Lit("."),
                Part::Name(word),
                Part::Lit(kind),
            ],
        )
    }

    /// The scopes of an element's tag name on the tag's stack.
    fn element_name(&mut self, tag: u32, element: ElementInfo) -> u32 {
        match element.flavor {
            Flavor::Known => self.path(tag, &[S::NameTag]),
            Flavor::Deprecated => self.path(tag, &[S::NameTag, S::Deprecated]),
            Flavor::Invalid => self.path(tag, &[S::NameTag, S::UnrecognizedTag]),
        }
    }

    /// The code of the attribute rules a [`Rule::Region`] runs.
    fn region_payload(element: ElementInfo) -> u32 {
        match element.namespace {
            Namespace::Svg => 1,
            Namespace::Math => 2,
        }
    }

    /// The begin of an element that opens a rule: the tag's text, and the
    /// attributes of a start tag the line holds whole.
    fn element_begin_captures(&mut self, hit: &Hit, frame: &super::engine::Frame) {
        let base = frame.name;
        let element = ElementInfo::from_bits(hit.info);
        let [name_start, name_end, attrs_start, attrs_end, _] = hit.at;
        let word = name(frame.payload);
        let tag = self.element_tag(base, element, word, "start");
        let bracket = self.path(tag, &[S::TagBegin]);
        self.cap_stack(base, hit.start, name_start, bracket);
        let name_stack = self.element_name(tag, element);
        self.cap_stack(base, name_start, name_end, name_stack);
        if attrs_start != usize::MAX {
            self.region(attrs_start, attrs_end, base, Self::region_payload(element));
            let close = self.path(tag, &[S::TagEnd]);
            self.cap_stack(base, attrs_end, hit.end, close);
        }
    }

    /// An element that is one match: a start tag with the attributes and
    /// the `/>` of the line.
    pub(super) fn element_void(&mut self, hit: &Hit) -> bool {
        let top = *self.top();
        self.tok(top.content, hit.start);
        let element = ElementInfo::from_bits(hit.info);
        let [name_start, name_end, attrs_start, attrs_end, _] = hit.at;
        let word = self.intern(name_start, name_end);
        let base = self.name_stack(top.content, Rule::Element, packed(hit.info, word));
        let tag = self.element_tag(base, element, word, "void");
        let bracket = self.path(tag, &[S::TagBegin]);
        self.cap_stack(base, hit.start, name_start, bracket);
        let name_stack = self.element_name(tag, element);
        self.cap_stack(base, name_start, name_end, name_stack);
        self.region(attrs_start, attrs_end, base, Self::region_payload(element));
        let close = self.path(tag, &[S::TagEnd]);
        self.cap_stack(base, attrs_end, hit.end, close);
        self.pos = self.pos.max(hit.end);
        false
    }

    /// The end of an element: its end tag, the `/>` of an unfinished start
    /// tag, or the lookahead for any end tag.
    /// For `(</)(\2)\s*(>)` the name in the scope is the end tag's own, as `$2`
    /// of the end pattern; for `(/>)` group 2 did not match, so `$2` is empty.
    fn element_end_captures(&mut self, top: &super::engine::Frame, hit: &Hit) {
        let base = top.name;
        let element = ElementInfo::from_bits(info(top.payload));
        let [name_start, name_end, gt, ..] = hit.at;
        match hit.info {
            0 => {
                let word = self.intern(name_start, name_end);
                let tag = self.element_tag(base, element, word, "end");
                let bracket = self.path(tag, &[S::TagBegin]);
                self.cap_stack(base, hit.start, name_start, bracket);
                let name_stack = self.element_name(tag, element);
                self.cap_stack(base, name_start, name_end, name_stack);
                self.cap_stack(base, name_end, gt, tag);
                let close = self.path(tag, &[S::TagEnd]);
                self.cap_stack(base, gt, hit.end, close);
            }
            1 => {
                let tag = self.element_tag(base, element, 0, "end");
                let close = self.path(tag, &[S::TagEnd]);
                self.cap_stack(base, hit.start, hit.end, close);
            }
            _ => {}
        }
    }
}

/// The value rule that the `=` of an attribute of kind `bits` opens.
fn value_rule(bits: u32) -> Rule {
    match AttributeKind::from_bits(bits) {
        AttributeKind::Style => Rule::ValueCss,
        AttributeKind::Event => Rule::ValueJs,
        AttributeKind::Known
        | AttributeKind::Data
        | AttributeKind::Deprecated
        | AttributeKind::Unrecognized => Rule::Value,
    }
}

/// The string rule that a quote opens inside `rule`.
fn string_rule(rule: Rule) -> Rule {
    match rule {
        Rule::WrapCss => Rule::StringCss,
        Rule::WrapJs => Rule::StringJs,
        Rule::Root
        | Rule::Comment
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
        | Rule::ErbLine => Rule::StringPlain,
    }
}

/// The rule that a pattern of ERB's injection opens.
fn erb_rule(kind: Erb) -> Rule {
    match kind {
        Erb::CommentWrap => Rule::ErbCommentWrap,
        Erb::TagWrap => Rule::ErbTagWrap,
        Erb::Comment => Rule::ErbComment,
        Erb::Block => Rule::ErbBlock,
        Erb::Line => Rule::ErbLine,
    }
}
