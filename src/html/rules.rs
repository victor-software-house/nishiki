//! What each rule is called, and the scopes of its captures.
//! This is the data of the grammar's begin, end and match rules, apart from
//! where they match.

use super::engine::{Hit, Lexer, Part, Pattern, packed};
use super::erb::Erb;
use super::scope::S;
use super::state::{
    AttributeRule, Category, ElementClass, Flavor, Namespace, Rule, TagClass, TagRule, name,
    payload_bits,
};
use super::vue::{self, TemplateTag, VuePattern, VueRule};

impl Lexer<'_> {
    /// The scopes of a rule's begin and end text and everything inside it,
    /// on `base`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule, in the order of the grammar"
    )]
    pub(super) fn name_stack(&mut self, base: u32, rule: Rule, payload: u32) -> u32 {
        let bits = payload_bits(payload);
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
            Rule::BladeComment
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
            | Rule::BladeArgsComment
            | Rule::BladeBalance
            | Rule::BladePhpSection
            | Rule::BladeAttribute => self.blade_name_stack(base, rule),
            Rule::Comment => self.path(base, &[S::CommentBlock]),
            Rule::Cdata => self.path(base, &[S::TagCdata]),
            Rule::Doctype => self.path(base, &[S::TagDoctype]),
            Rule::DoctypeString => self.path(base, &[S::StringDouble]),
            Rule::Processing => self.path(base, &[S::TagProcessing]),
            Rule::Tag => {
                let tag = TagClass::from_bits(bits);
                let suffix = match tag.rule {
                    TagRule::Start => ".start.html",
                    TagRule::End => ".end.html",
                    TagRule::Void => ".void.html",
                    TagRule::CustomStart => return self.path(base, &[S::TagCustomStart]),
                    TagRule::CustomEnd => return self.path(base, &[S::TagCustomEnd]),
                    TagRule::Derivative => return self.path(base, &[S::TagDerivative]),
                    TagRule::Unrecognized => {
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
                        Part::Lit(suffix),
                    ],
                )
            }
            Rule::Element => {
                let element = ElementClass::from_bits(bits);
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
                let element = ElementClass::from_bits(bits);
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
                let prefix = match AttributeRule::from_bits(bits) {
                    AttributeRule::Style => return self.path(base, &[S::AttributeStyle]),
                    AttributeRule::Known | AttributeRule::Deprecated => "meta.attribute.",
                    AttributeRule::Event => "meta.attribute.event-handler.",
                    AttributeRule::Data => "meta.attribute.data-x.",
                    AttributeRule::Unrecognized => "meta.attribute.unrecognized.",
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
            Rule::VueComment
            | Rule::VueCommentKv
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
            | Rule::VueCapitalizedAttrs
            | Rule::VueDirective
            | Rule::VueExprString
            | Rule::VueExprBare
            | Rule::VueExpr => self.vue_name_stack(base, rule, payload),
        }
    }

    /// The scopes of a rule's content only, on its name's stack.
    pub(super) fn content_stack(&mut self, name: u32, rule: Rule, payload: u32) -> u32 {
        match rule {
            Rule::VueComment
            | Rule::VueCommentKv
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
            | Rule::VueCapitalizedAttrs
            | Rule::VueDirective
            | Rule::VueExprString
            | Rule::VueExprBare
            | Rule::VueExpr => self.vue_content_stack(name, rule, payload),
            Rule::Cdata => self.path(name, &[S::InlineData]),
            Rule::StringCss => self.path(name, &[S::SourceCss]),
            Rule::StringJs => self.path(name, &[S::SourceJs]),
            Rule::ErbBlock | Rule::ErbLine => self.path(name, &[S::SourceRuby]),
            Rule::BladeComment
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
            | Rule::BladeArgsComment
            | Rule::BladeBalance
            | Rule::BladePhpSection
            | Rule::BladeAttribute => self.blade_content_stack(name, rule),
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
        Some(match hit.pattern {
            Pattern::Zero(Rule::ElementStart) => {
                (Rule::ElementStart, packed(payload_bits(top.payload), 0))
            }
            Pattern::Zero(rule) => (rule, 0),
            Pattern::Processing => (Rule::Processing, 0),
            Pattern::Comment => (Rule::Comment, 0),
            Pattern::Doctype => (Rule::Doctype, 0),
            Pattern::Cdata => (Rule::Cdata, 0),
            Pattern::StyleWrap => (Rule::StyleWrap, 0),
            Pattern::ScriptWrap => (Rule::ScriptWrap, 0),
            Pattern::Tag => self.tag_opens(hit, from, to),
            Pattern::Element => {
                let word = self.intern(from, to);
                (Rule::Element, packed(hit.bits, word))
            }
            Pattern::StyleTag => (Rule::StyleTag, 0),
            Pattern::ScriptTag => (Rule::ScriptTag, 0),
            Pattern::ScriptJs => (Rule::ScriptJsOuter, 0),
            Pattern::Attribute => self.attribute_opens(hit, from, to),
            Pattern::Equals => (value_rule(payload_bits(top.payload)), 0),
            Pattern::Quote => (string_rule(top.rule), hit.bits),
            Pattern::DoctypeQuote => (Rule::DoctypeString, 0),
            Pattern::JsLine => (Rule::JsLineComment, top.payload),
            Pattern::JsBlock => (Rule::JsBlockComment, top.payload),
            Pattern::Erb(erb) => (erb_rule(erb), 0),
            Pattern::Blade(blade) => (Self::blade_opens(blade)?, 0),
            Pattern::Vue(pattern) => return self.vue_opens(pattern, hit),
            Pattern::End
            | Pattern::Inject
            | Pattern::ElementVoid
            | Pattern::EntityNamed
            | Pattern::EntityDecimal
            | Pattern::EntityHexadecimal
            | Pattern::EntityAmbiguous
            | Pattern::AttributeIllegal
            | Pattern::EqualsIllegal
            | Pattern::CommentIllegal
            | Pattern::DoctypeKeyword
            | Pattern::DoctypeWord
            | Pattern::Unquoted
            | Pattern::UnquotedCss
            | Pattern::UnquotedJs
            | Pattern::JsText
            | Pattern::ErbHash
            | Pattern::BladeMatch(_) => return None,
        })
    }

    /// The tag rule a `Pattern::Tag` match opens, carrying its `TagRule` and,
    /// unless it is a custom element, its interned name.
    fn tag_opens(&mut self, hit: &Hit, from: usize, to: usize) -> (Rule, u32) {
        let tag = TagClass::from_bits(hit.bits & 0x3f);
        let word = match tag.rule {
            TagRule::CustomStart | TagRule::CustomEnd => 0,
            TagRule::Start
            | TagRule::End
            | TagRule::Void
            | TagRule::Unrecognized
            | TagRule::Derivative => self.intern(from, to),
        };
        (Rule::Tag, packed(hit.bits & 0x3f, word))
    }

    /// The attribute rule a `Pattern::Attribute` match opens, carrying its
    /// `AttributeRule` and, unless it is `style`, its interned name without
    /// leading dots.
    fn attribute_opens(&mut self, hit: &Hit, from: usize, to: usize) -> (Rule, u32) {
        let from = from + self.s[from..to].iter().take_while(|&&b| b == b'.').count();
        let word = if hit.bits == AttributeRule::Style.bits() {
            0
        } else {
            self.intern(from, to)
        };
        (Rule::Attribute, packed(hit.bits, word))
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
        match hit.pattern {
            Pattern::Processing => {
                self.cap(base, start, start + 2, &[S::TagPunctuation]);
                self.cap(base, start + 2, end, &[S::NameTag]);
            }
            Pattern::Comment => self.cap(base, start, end, &[S::CommentPunctuation]),
            Pattern::Doctype | Pattern::Cdata => self.cap(base, start, end, &[S::TagBegin]),
            Pattern::StyleWrap | Pattern::ScriptWrap => {
                self.cap(base, start, end, &[S::WhitespaceLeading]);
            }
            Pattern::Tag => {
                self.cap(base, start, from, &[S::TagBegin]);
                let rule = TagClass::from_bits(hit.bits & 0x3f).rule;
                let scopes: &[S] = match (rule, hit.bits >> 8) {
                    (TagRule::Unrecognized, _) => &[S::NameTag, S::UnrecognizedTag],
                    (_, 1) => &[S::NameTag, S::Deprecated],
                    (_, 2) => &[S::NameTag, S::NoLongerSupported],
                    _ => &[S::NameTag],
                };
                self.cap(base, from, to, scopes);
            }
            Pattern::Element => self.element_begin_captures(hit, frame),
            Pattern::StyleTag => self.embedded_start(base, start, end, S::TagStyleStart),
            Pattern::ScriptTag => self.embedded_start(base, start, end, S::TagScriptStart),
            Pattern::ScriptJs => {
                self.cap(base, start, end, &[S::TagScriptStart, S::TagEnd]);
            }
            Pattern::Attribute => {
                let scope = if AttributeRule::from_bits(hit.bits) == AttributeRule::Deprecated {
                    S::AttributeNameDeprecated
                } else {
                    S::AttributeName
                };
                self.cap(base, start, end, &[scope]);
            }
            Pattern::Equals => self.cap(base, start, end, &[S::KeyValue]),
            Pattern::Quote => self.cap(base, start, end, &[S::StringBegin]),
            Pattern::JsLine => self.cap(base, start, end, &[S::CommentPunctuationJs]),
            Pattern::JsBlock => self.cap(base, start, end, &[S::CommentBeginJs]),
            Pattern::Erb(erb) => {
                let scope = match erb {
                    Erb::CommentWrap => S::ErbCommentLeading,
                    Erb::TagWrap => S::ErbEmbeddedLeading,
                    Erb::Comment => S::ErbCommentBegin,
                    Erb::Block | Erb::Line => S::ErbBegin,
                };
                self.cap(base, start, end, &[scope]);
            }
            Pattern::Blade(blade) => self.blade_begin_captures(base, blade, start, end),
            Pattern::Vue(pattern) => self.vue_begin_captures(hit, pattern, frame),
            Pattern::End
            | Pattern::Inject
            | Pattern::Zero(_)
            | Pattern::ElementVoid
            | Pattern::DoctypeQuote
            | Pattern::EntityNamed
            | Pattern::EntityDecimal
            | Pattern::EntityHexadecimal
            | Pattern::EntityAmbiguous
            | Pattern::AttributeIllegal
            | Pattern::EqualsIllegal
            | Pattern::CommentIllegal
            | Pattern::DoctypeKeyword
            | Pattern::DoctypeWord
            | Pattern::Unquoted
            | Pattern::UnquotedCss
            | Pattern::UnquotedJs
            | Pattern::JsText
            | Pattern::ErbHash
            | Pattern::BladeMatch(_) => {}
        }
    }

    /// The scopes of a Vue rule's name on `base`, as [`Self::name_stack`] gives
    /// them.
    #[cold]
    #[inline(never)]
    fn vue_name_stack(&mut self, base: u32, rule: Rule, payload: u32) -> u32 {
        let Some(rule) = rule.vue() else {
            return base;
        };
        match rule {
            VueRule::Block
            | VueRule::Element
            | VueRule::ScriptStuff
            | VueRule::StyleStuff
            | VueRule::LangGroup
            | VueRule::CapitalizedAttrs
            | VueRule::ExprString
            | VueRule::ExprBare => base,
            VueRule::Comment | VueRule::CommentKv => self.path(base, &[S::VueComment]),
            VueRule::SelfClosing => self.path(base, &[S::SelfClosingTag]),
            VueRule::TagStuff | VueRule::ScriptTagStuff | VueRule::StyleTagStuff => {
                self.path(base, &[S::TagStuff])
            }
            VueRule::Body | VueRule::BodyWhile => {
                let parent = *self.top();
                let scope =
                    vue::body_lang(parent.rule, parent.payload).map_or(S::Text, vue::lang_scope);
                self.path(base, &[scope])
            }
            VueRule::TemplateBody => self.path(base, &[S::TextHtmlDerivative]),
            VueRule::TemplateTag => self.path(base, &[S::TemplateTagStart]),
            VueRule::TemplateTagEnd => self.path(base, &[S::TemplateTagEnd]),
            VueRule::Capitalized => self.dynamic(
                base,
                &[
                    Part::Lit("meta.tag.structure."),
                    Part::Name(payload),
                    Part::Lit(".start.html.vue"),
                ],
            ),
            VueRule::Directive => {
                let scope = if payload == 1 {
                    S::DirectiveControl
                } else {
                    S::DirectiveVue
                };
                self.path(base, &[scope])
            }
            VueRule::Expr => self.path(base, &[S::TsEmbedded]),
        }
    }

    /// The scopes of a Vue rule's content on `name`: an html block holds
    /// html-derivative.
    #[cold]
    #[inline(never)]
    fn vue_content_stack(&mut self, name: u32, rule: Rule, payload: u32) -> u32 {
        if rule.vue() == Some(VueRule::Element)
            && vue::Block::from_bits(payload_bits(payload)) == vue::Block::Lang(vue::Lang::Html)
        {
            return self.path(name, &[S::TextHtmlDerivative]);
        }
        name
    }

    /// The end captures of a Vue rule, as [`Self::end_captures`] emits them.
    #[cold]
    #[inline(never)]
    fn vue_end_captures(&mut self, top: &super::engine::Frame, hit: &Hit) {
        let base = top.name;
        let (start, end) = (hit.start, hit.end);
        let Some(rule) = top.rule.vue() else {
            return;
        };
        match rule {
            VueRule::Comment | VueRule::CommentKv => {
                self.cap(base, start, end, &[S::VueCommentPunctuation]);
            }
            VueRule::SelfClosing
            | VueRule::Block
            | VueRule::TagStuff
            | VueRule::ScriptTagStuff
            | VueRule::StyleTagStuff
            | VueRule::TemplateTag
            | VueRule::Capitalized => self.cap(base, start, end, &[S::VueTagEnd]),
            VueRule::Element => {
                let [name_start, name_end, ..] = hit.at;
                self.cap(base, start, start + 2, &[S::VueTagBegin]);
                let stack = self.vue_tag_name(base, name(top.payload));
                self.cap_stack(base, name_start, name_end, stack);
            }
            VueRule::TemplateTagEnd if end > start => {
                self.cap(base, start, start + 2, &[S::VueTagBegin]);
                let stack = self.dynamic(base, &[Part::Lit("entity.name.tag.template.html.vue")]);
                self.cap_stack(base, start + 2, end, stack);
            }
            VueRule::ExprString => self.cap(base, start, end, &[S::VueStringEnd]),
            VueRule::TemplateTagEnd
            | VueRule::ScriptStuff
            | VueRule::StyleStuff
            | VueRule::LangGroup
            | VueRule::Body
            | VueRule::BodyWhile
            | VueRule::TemplateBody
            | VueRule::CapitalizedAttrs
            | VueRule::Directive
            | VueRule::ExprBare
            | VueRule::Expr => {}
        }
    }

    /// The rule a begin match of one of Vue's patterns opens, as
    /// [`Self::opens`] gives it.
    /// Without the `lang-vue` feature it opens none, and its code folds away.
    #[cold]
    #[inline(never)]
    fn vue_opens(&mut self, pattern: VuePattern, hit: &Hit) -> Option<(Rule, u32)> {
        if !cfg!(feature = "lang-vue") {
            return None;
        }
        let [from, to, ..] = hit.at;
        Some(match pattern {
            VuePattern::CommentKv => (Rule::VueCommentKv, 0),
            VuePattern::Comment => (Rule::VueComment, 0),
            VuePattern::SelfClosing => (Rule::VueSelfClosing, 0),
            VuePattern::Block => (Rule::VueBlock, 0),
            VuePattern::Element => {
                let word = self.intern(from, to);
                (Rule::VueElement, packed(hit.bits, word))
            }
            VuePattern::LangGroup => (Rule::VueLangGroup, hit.bits),
            VuePattern::TemplateTag => (Rule::VueTemplateTag, hit.bits),
            VuePattern::TemplateTagEnd => (Rule::VueTemplateTagEnd, hit.bits),
            VuePattern::Capitalized => (Rule::VueCapitalized, self.intern(from, to)),
            VuePattern::DirectiveControl => (Rule::VueDirective, 1),
            VuePattern::DirectiveOriginal => (Rule::VueDirective, 0),
            VuePattern::ExprString => (Rule::VueExprString, hit.bits),
            VuePattern::ExprBare => (Rule::VueExprBare, 0),
            VuePattern::Modifier => return None,
        })
    }

    /// The scope `entity.name.tag.NAME.html.vue` of a Vue block's tag name
    /// on `base`, for the interned `word`.
    #[inline(never)]
    fn vue_tag_name(&mut self, base: u32, word: u32) -> u32 {
        self.dynamic(
            base,
            &[
                Part::Lit("entity.name.tag."),
                Part::Name(word),
                Part::Lit(".html.vue"),
            ],
        )
    }

    /// Emits the capture groups of a begin match of one of Vue's patterns.
    /// Without the `lang-vue` feature it emits none, and its code folds away.
    #[cold]
    #[inline(never)]
    fn vue_begin_captures(&mut self, hit: &Hit, pattern: VuePattern, frame: &super::engine::Frame) {
        if !cfg!(feature = "lang-vue") {
            return;
        }
        let base = frame.name;
        let (start, end) = (hit.start, hit.end);
        let at = hit.at;
        match pattern {
            VuePattern::CommentKv => {
                self.cap(base, start, start + 4, &[S::VueCommentPunctuation]);
                self.cap(base, at[0], at[1], &[S::VueBlockTag]);
                self.cap(base, at[1], end, &[S::VueStorageClass]);
            }
            VuePattern::Comment => self.cap(base, start, end, &[S::VueCommentPunctuation]),
            VuePattern::SelfClosing => {
                self.cap(base, start, start + 1, &[S::VueTagBegin]);
                let word = self.intern(at[0], at[1]);
                let stack = self.vue_tag_name(base, word);
                self.cap_stack(base, at[0], at[1], stack);
            }
            VuePattern::Block => self.cap(base, start, end, &[S::VueTagBegin]),
            VuePattern::Element => {
                let stack = self.vue_tag_name(base, name(frame.payload));
                self.cap_stack(base, start, end, stack);
            }
            VuePattern::TemplateTag => {
                let tag = TemplateTag::from_bits(hit.bits);
                self.cap(base, start, start + 1, &[S::VueTagBegin]);
                let stack = self.dynamic(base, &[Part::Lit("entity.name.tag.template.html.vue")]);
                self.cap_stack(base, start + 1, start + 9, stack);
                if tag == TemplateTag::Bare {
                    self.cap(base, start + 9, end, &[S::VueTagEnd]);
                }
            }
            VuePattern::Capitalized => {
                self.cap(base, start, start + 1, &[S::VueTagBegin]);
                self.cap(base, at[0], at[1], &[S::VueNameTag]);
            }
            VuePattern::DirectiveControl => {
                let scope = if hit.bits == 1 {
                    S::LoopVue
                } else {
                    S::ConditionalVue
                };
                self.cap(base, start, end, &[scope]);
            }
            VuePattern::DirectiveOriginal => {
                let [name_end, colon_end, argument_start, argument_end, argument] = at;
                match u8::try_from(hit.bits).unwrap_or(b'v') {
                    b'.' | b':' => self.cap(base, start, name_end, &[S::ShorthandBind]),
                    b'@' => self.cap(base, start, name_end, &[S::ShorthandEvent]),
                    b'#' => self.cap(base, start, name_end, &[S::ShorthandSlot]),
                    _ => {
                        self.cap(base, start, name_end, &[S::VueAttributeName]);
                        self.cap(base, name_end, colon_end, &[S::VueKeyValue]);
                    }
                }
                match vue::Argument::from_code(argument) {
                    vue::Argument::Bracketed => {
                        self.cap(base, argument_start, argument_start + 1, &[S::VueKeyValue]);
                        self.cap(base, argument_start + 1, argument_end, &[S::TsEmbedded]);
                        self.cap(base, argument_end, argument_end + 1, &[S::VueKeyValue]);
                    }
                    vue::Argument::Plain => {
                        self.cap(base, argument_start, argument_end, &[S::VueAttributeName]);
                    }
                    vue::Argument::Absent => {}
                }
            }
            VuePattern::ExprString => {
                self.cap(base, start, start + 1, &[S::VueKeyValue]);
                self.cap(base, at[0], end, &[S::VueStringBegin]);
            }
            VuePattern::ExprBare => self.cap(base, start, end, &[S::VueKeyValue]),
            VuePattern::LangGroup | VuePattern::TemplateTagEnd | VuePattern::Modifier => {}
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
        match hit.pattern {
            Pattern::EntityNamed => {
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
            Pattern::EntityDecimal | Pattern::EntityHexadecimal => {
                let scope = if hit.pattern == Pattern::EntityDecimal {
                    S::EntityDecimal
                } else {
                    S::EntityHexadecimal
                };
                let stack = self.path(base, &[scope]);
                let punctuation = self.path(stack, &[S::EntityPunctuation]);
                self.cap_stack(base, start, start + 1, punctuation);
                self.cap_stack(base, start + 1, end, stack);
            }
            Pattern::EntityAmbiguous => self.cap(base, start, end, &[S::AmbiguousAmpersand]),
            Pattern::AttributeIllegal => self.cap(base, start, end, &[S::CharacterNotAllowed]),
            Pattern::EqualsIllegal => self.cap(base, start, end, &[S::UnexpectedEquals]),
            Pattern::CommentIllegal => self.cap(base, start, end, &[S::CommentIllegal]),
            Pattern::DoctypeKeyword => self.cap(base, start, end, &[S::NameTag]),
            Pattern::DoctypeWord => self.cap(base, start, end, &[S::AttributeName]),
            Pattern::Unquoted => self.cap(base, start, end, &[S::Unquoted]),
            Pattern::UnquotedCss => self.cap(base, start, end, &[S::Unquoted, S::SourceCss]),
            Pattern::UnquotedJs => {
                let stack = self.path(base, &[S::Unquoted]);
                self.javascript(start, end, stack);
            }
            Pattern::JsText => self.javascript(start, end, base),
            Pattern::ErbHash => {
                let stack = self.path(base, &[S::ErbHashComment]);
                let punctuation = self.path(stack, &[S::ErbHashPunctuation]);
                self.cap_stack(base, start, start + 1, punctuation);
                self.cap_stack(base, start + 1, end, stack);
            }
            Pattern::BladeMatch(blade) => {
                let base = self.blade_base(top.rule, top.state, top.content);
                self.blade_match_captures(base, blade, start, end);
            }
            Pattern::Vue(VuePattern::Modifier) => {
                self.cap(base, start, start + 1, &[S::VueKeyValue]);
                self.cap(base, start + 1, end, &[S::VueAttributeName]);
            }
            Pattern::End
            | Pattern::Inject
            | Pattern::Zero(_)
            | Pattern::Processing
            | Pattern::Comment
            | Pattern::Doctype
            | Pattern::Cdata
            | Pattern::StyleWrap
            | Pattern::ScriptWrap
            | Pattern::Tag
            | Pattern::Element
            | Pattern::ElementVoid
            | Pattern::StyleTag
            | Pattern::ScriptTag
            | Pattern::ScriptJs
            | Pattern::Attribute
            | Pattern::Equals
            | Pattern::Quote
            | Pattern::DoctypeQuote
            | Pattern::JsLine
            | Pattern::JsBlock
            | Pattern::Erb(_)
            | Pattern::Blade(_)
            | Pattern::Vue(
                VuePattern::CommentKv
                | VuePattern::Comment
                | VuePattern::SelfClosing
                | VuePattern::Block
                | VuePattern::Element
                | VuePattern::LangGroup
                | VuePattern::TemplateTag
                | VuePattern::TemplateTagEnd
                | VuePattern::Capitalized
                | VuePattern::DirectiveControl
                | VuePattern::DirectiveOriginal
                | VuePattern::ExprString
                | VuePattern::ExprBare,
            ) => {}
        }
    }

    /// Emits the capture groups of an end match, on the rule's name stack.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per rule, in the order of the grammar"
    )]
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
            Rule::BladeComment
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
            | Rule::BladeArgsComment
            | Rule::BladeBalance
            | Rule::BladePhpSection
            | Rule::BladeAttribute => self.blade_end_captures(base, top.rule, start, end),
            Rule::VueComment
            | Rule::VueCommentKv
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
            | Rule::VueCapitalizedAttrs
            | Rule::VueDirective
            | Rule::VueExprString
            | Rule::VueExprBare
            | Rule::VueExpr => self.vue_end_captures(top, hit),
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
    /// `tag` (`start`, `end` or `void`).
    fn element_tag(&mut self, base: u32, element: ElementClass, word: u32, tag: &str) -> u32 {
        if element.flavor == Flavor::Invalid {
            let scope = match tag {
                "start" => S::TagOtherInvalidStart,
                "end" => S::TagOtherInvalidEnd,
                _ => S::TagOtherInvalidVoid,
            };
            return self.path(base, &[scope]);
        }
        let suffix = match tag {
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
                    Part::Lit(suffix),
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
                Part::Lit(suffix),
            ],
        )
    }

    /// The scopes of an element's tag name on the tag's stack.
    fn element_name(&mut self, tag: u32, element: ElementClass) -> u32 {
        match element.flavor {
            Flavor::Known => self.path(tag, &[S::NameTag]),
            Flavor::Deprecated => self.path(tag, &[S::NameTag, S::Deprecated]),
            Flavor::Invalid => self.path(tag, &[S::NameTag, S::UnrecognizedTag]),
        }
    }

    /// The code of the attribute rules a [`Rule::Region`] runs.
    fn region_payload(element: ElementClass) -> u32 {
        match element.namespace {
            Namespace::Svg => 1,
            Namespace::Math => 2,
        }
    }

    /// The begin of an element that opens a rule: the tag's text, and the
    /// attributes of a start tag the line holds whole.
    fn element_begin_captures(&mut self, hit: &Hit, frame: &super::engine::Frame) {
        let base = frame.name;
        let element = ElementClass::from_bits(hit.bits);
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
        let element = ElementClass::from_bits(hit.bits);
        let [name_start, name_end, attrs_start, attrs_end, _] = hit.at;
        let word = self.intern(name_start, name_end);
        let base = self.name_stack(top.content, Rule::Element, packed(hit.bits, word));
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
    /// of the end pattern.
    /// For `(/>)` group 2 did not match, so `$2` is empty.
    fn element_end_captures(&mut self, top: &super::engine::Frame, hit: &Hit) {
        let base = top.name;
        let element = ElementClass::from_bits(payload_bits(top.payload));
        let [name_start, name_end, gt, ..] = hit.at;
        match hit.bits {
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

/// The value rule that the `=` of an attribute with the rule `bits` opens.
fn value_rule(bits: u32) -> Rule {
    match AttributeRule::from_bits(bits) {
        AttributeRule::Style => Rule::ValueCss,
        AttributeRule::Event => Rule::ValueJs,
        AttributeRule::Known
        | AttributeRule::Data
        | AttributeRule::Deprecated
        | AttributeRule::Unrecognized => Rule::Value,
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
        | Rule::BladeArgsComment
        | Rule::BladeBalance
        | Rule::BladePhpSection
        | Rule::BladeAttribute
        | Rule::VueComment
        | Rule::VueCommentKv
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
        | Rule::VueCapitalizedAttrs
        | Rule::VueDirective
        | Rule::VueExprString
        | Rule::VueExprBare
        | Rule::VueExpr => Rule::StringPlain,
    }
}

/// The rule that a pattern of ERB's injection opens.
fn erb_rule(erb: Erb) -> Rule {
    match erb {
        Erb::CommentWrap => Rule::ErbCommentWrap,
        Erb::TagWrap => Rule::ErbTagWrap,
        Erb::Comment => Rule::ErbComment,
        Erb::Block => Rule::ErbBlock,
        Erb::Line => Rule::ErbLine,
    }
}
