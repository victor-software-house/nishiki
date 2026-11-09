//! The matchers, one for each regular expression of the grammar.
//! Each is the code that decides whether its expression matches at a
//! position, where it ends, and the capture groups the grammar names.
//!
//! Each follows its expression as Oniguruma's backtracking would.
//! A word followed by `\b` in an alternation of words is the whole run of word
//! characters there, so a list test takes that run.
//! `\w`, `\s` and `\d` take non-ASCII characters, as in Oniguruma, through
//! [`Chars`] and [`crate::classes`].

use super::c_lang::Last;
use super::engine::ObjcMemory;
use super::grammar::Rx;
use super::preproc::{ELSE_ELIF_ENDIF, ELSE_ENDIF, IF_WORDS, Truth};
use super::words;
use crate::rules::{Chars, Grammar, Lexer};

/// The property attributes of `PropertiesAt0At0`.
const PROPERTY_ATTRIBUTES: &[&[u8]] = &[
    b"getter",
    b"setter",
    b"readonly",
    b"readwrite",
    b"assign",
    b"retain",
    b"copy",
    b"nonatomic",
    b"atomic",
    b"strong",
    b"weak",
    b"nonnull",
    b"nullable",
    b"null_resettable",
    b"null_unspecified",
    b"class",
    b"direct",
];

/// The qualifiers of `ProtocolTypeQualifier`.
const QUALIFIERS: &[&[u8]] = &[
    b"in",
    b"out",
    b"inout",
    b"oneway",
    b"bycopy",
    b"byref",
    b"nonnull",
    b"nullable",
    b"_Nonnull",
    b"_Nullable",
    b"_Null_unspecified",
];

/// The macros of `AppleFoundationFunctionalMacros`.
const AVAILABILITY: &[&[u8]] = &[
    b"API_AVAILABLE",
    b"API_DEPRECATED",
    b"API_UNAVAILABLE",
    b"NS_AVAILABLE",
    b"NS_AVAILABLE_MAC",
    b"NS_AVAILABLE_IOS",
    b"NS_DEPRECATED",
    b"NS_DEPRECATED_MAC",
    b"NS_DEPRECATED_IOS",
    b"NS_SWIFT_NAME",
];

/// The types of `AnonymousPattern12`.
const OBJC_TYPES: &[&[u8]] = &[
    b"IBOutlet",
    b"IBAction",
    b"BOOL",
    b"SEL",
    b"id",
    b"unichar",
    b"IMP",
    b"Class",
    b"instancetype",
];

/// The control words of `CLangAt5`.
const CONTROL: &[&[u8]] = &[
    b"break",
    b"continue",
    b"do",
    b"else",
    b"for",
    b"goto",
    b"if",
    b"_Pragma",
    b"return",
    b"while",
];

/// The modifiers of `CLangAt9`.
const MODIFIERS: &[&[u8]] = &[
    b"const",
    b"extern",
    b"register",
    b"restrict",
    b"static",
    b"volatile",
    b"inline",
    b"__block",
];

/// The storage types of `CStorageTypesAt0`.
const PRIMITIVES: &[&[u8]] = &[
    b"void",
    b"char",
    b"short",
    b"int",
    b"signed",
    b"unsigned",
    b"long",
    b"float",
    b"double",
    b"bool",
    b"_Bool",
];

/// The system types of `CLangAt25`.
const SYSTEM_TYPES: &[&[u8]] = &[
    b"u_char",
    b"u_short",
    b"u_int",
    b"u_long",
    b"ushort",
    b"uint",
    b"u_quad_t",
    b"quad_t",
    b"qaddr_t",
    b"caddr_t",
    b"daddr_t",
    b"div_t",
    b"dev_t",
    b"fixpt_t",
    b"blkcnt_t",
    b"blksize_t",
    b"gid_t",
    b"in_addr_t",
    b"in_port_t",
    b"ino_t",
    b"key_t",
    b"mode_t",
    b"nlink_t",
    b"id_t",
    b"pid_t",
    b"off_t",
    b"segsz_t",
    b"swblk_t",
    b"uid_t",
    b"clock_t",
    b"size_t",
    b"ssize_t",
    b"time_t",
    b"useconds_t",
    b"suseconds_t",
];

/// The `pthread_` types of `CLangAt26`.
const PTHREAD_TYPES: &[&[u8]] = &[
    b"pthread_attr_t",
    b"pthread_cond_t",
    b"pthread_condattr_t",
    b"pthread_mutex_t",
    b"pthread_mutexattr_t",
    b"pthread_once_t",
    b"pthread_rwlock_t",
    b"pthread_rwlockattr_t",
    b"pthread_t",
    b"pthread_key_t",
];

/// The integer prefixes of `CLangAt27`, each followed by `_t`.
const STDINT: &[&[u8]] = &[
    b"int8",
    b"int16",
    b"int32",
    b"int64",
    b"uint8",
    b"uint16",
    b"uint32",
    b"uint64",
    b"int_least8",
    b"int_least16",
    b"int_least32",
    b"int_least64",
    b"uint_least8",
    b"uint_least16",
    b"uint_least32",
    b"uint_least64",
    b"int_fast8",
    b"int_fast16",
    b"int_fast32",
    b"int_fast64",
    b"uint_fast8",
    b"uint_fast16",
    b"uint_fast32",
    b"uint_fast64",
    b"intptr",
    b"uintptr",
    b"intmax",
    b"uintmax",
];

impl<G: Grammar> Lexer<'_, G>
where
    G::Extra: ObjcMemory,
{
    /// Whether `rx` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per expression: the table of what each matches"
    )]
    pub(crate) fn matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        match rx {
            Rx::AnonymousPattern1 => self.interface_begin(p),
            Rx::AnonymousPattern1End => {
                let end = self.lit(p, b"@end")?;
                self.boundary(end).then_some(())?;
                self.cap(1, p, end);
                self.cap(2, p, p + 1);
                Some(end)
            }
            Rx::PreprocessorRuleEnabledInterface => {
                let (hash, digits, end) = self.interface_if(p, Truth::One)?;
                self.cap(1, hash, end);
                self.cap(2, hash + 1, hash + 3);
                self.cap(3, digits, end);
                Some(end)
            }
            Rx::PreprocessorRuleEnabledInterfaceEnd => {
                let (hash, start, end) = self.directive_one(p, b"endif")?;
                let stop = self.to_comment(end);
                self.cap(1, hash, stop);
                self.cap(2, start, end);
                Some(stop)
            }
            Rx::PreprocessorRuleEnabledInterfaceAt0 => {
                self.hash_word_inner(p, &[b"else"])?;
                Some(self.lim())
            }
            Rx::PreprocessorRuleEnabledInterfaceAt0End => {
                self.directive_one(p, b"endif").map(|_| p)
            }
            Rx::Disabled => self.directive(p, IF_WORDS).map(|_| self.lim()),
            Rx::DisabledEnd => self.directive_one(p, b"endif").map(|_| self.lim()),
            Rx::PragmaMark => self.pragma_mark(p, false),
            Rx::PreprocessorRuleEnabledInterfaceAt1 => Some(p),
            Rx::PreprocessorRuleEnabledInterfaceAt1End => self.directive(p, ELSE_ENDIF).map(|_| p),
            Rx::PreprocessorRuleDisabledInterface => {
                let (hash, digits, end) = self.interface_if(p, Truth::Zero)?;
                self.cap(1, hash, end);
                self.cap(2, hash + 1, hash + 3);
                self.cap(3, digits, end);
                Some(self.lim())
            }
            Rx::PreprocessorRuleDisabledInterfaceAt0 => self.hash_word_inner(p, &[b"else"]),
            Rx::PreprocessorRuleOtherInterface => {
                let (hash, start, end) = self.directive(p, IF_WORDS)?;
                let stop = self.to_comment(end);
                self.cap(1, hash, stop);
                self.cap(2, start, end);
                Some(stop)
            }
            Rx::PreprocessorRuleOtherInterfaceEnd => {
                let end = self.hash_word_inner(p, &[b"endif"])?;
                Some(self.to_comment(end))
            }
            Rx::PropertiesAt0 => {
                let end = self.lit(p, b"@property")?;
                let paren = self.space_end(end);
                if self.at(paren) != b'(' || paren >= self.lim() {
                    return None;
                }
                self.cap(1, p, end);
                self.cap(2, p, p + 1);
                self.cap(3, paren, paren + 1);
                Some(paren + 1)
            }
            Rx::PropertiesAt0End | Rx::MethodAt0End => {
                self.close_paren_word(p, rx == Rx::MethodAt0End)
            }
            Rx::PropertiesAt0At0 => self.word_in(p, PROPERTY_ATTRIBUTES),
            Rx::PropertiesAt1 => {
                let end = self.lit(p, b"@property")?;
                self.boundary(end).then_some(())?;
                self.cap(1, p, end);
                self.cap(2, p, p + 1);
                Some(end)
            }
            Rx::ProtocolList => self.one_char(p, b'<'),
            Rx::ProtocolListEnd => self.one_char(p, b'>'),
            Rx::ProtocolListAt0 => self.listed_word(p, words::PROTOCOL),
            Rx::Method => (p == 0 && matches!(self.at(0), b'-' | b'+') && self.lim() > 0)
                .then(|| self.space_end(1)),
            Rx::MethodEnd => match self.at(p) {
                b'#' | b'{' if p < self.lim() => Some(p),
                b';' if p < self.lim() => Some(p + 1),
                _ => None,
            },
            Rx::MethodAt0 => self.one_char(p, b'('),
            Rx::ProtocolTypeQualifier => self.word_in(p, QUALIFIERS),
            Rx::AnonymousPattern2 => self.implementation_begin(p),
            Rx::PropertyDirective => self.at_sign_word(p, &[b"dynamic", b"synthesize"]),
            Rx::MethodSuper => {
                (p == 0 && matches!(self.at(0), b'-' | b'+') && self.lim() > 0).then_some(0)
            }
            Rx::MethodSuperEnd => {
                (self.prev_is(p, b'}') || (self.at(p) == b'#' && p < self.lim())).then_some(p)
            }
            Rx::AnonymousPattern3 => self.lit(p, b"@\""),
            Rx::AnonymousPattern3End => self.lit(p, b"\""),
            Rx::StringEscapedCharAt0 => self.escape(p, true),
            Rx::StringEscapedCharAt1 => {
                (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
            }
            Rx::AnonymousPattern3At1 => self.format_at(p),
            Rx::StringPlaceholderAt0 => self.placeholder(p),
            Rx::StringPlaceholderAt1 => {
                let end = self.lit(p, b"%")?;
                if self.at(end) == b'"' {
                    let q = self.space_end(end + 1);
                    if self.starts(q, b"PRI") || self.starts(q, b"SCN") {
                        return None;
                    }
                }
                self.cap(1, p, end);
                Some(end)
            }
            Rx::AnonymousPattern4 => {
                if !self.boundary(p) {
                    return None;
                }
                let end = self.lit(p, b"id")?;
                let q = self.space_end(end);
                if self.at(q) != b'<' || q >= self.lim() {
                    return None;
                }
                self.cap(1, p, end);
                Some(q)
            }
            Rx::AnonymousPattern4End => self.prev_is(p, b'>').then_some(p),
            Rx::AnonymousPattern5 => {
                self.word_in(p, &[b"NS_DURING", b"NS_HANDLER", b"NS_ENDHANDLER"])
            }
            Rx::AppleFoundationFunctionalMacros => {
                if !self.boundary(p) || self.word_at(p).is_none() {
                    return None;
                }
                let end = self.word_end(p);
                if !crate::bytes::contains(AVAILABILITY, self.text(p, end)) {
                    return None;
                }
                let paren = self.space_end(end);
                if self.at(paren) != b'(' || paren >= self.lim() {
                    return None;
                }
                self.cap(1, p, end);
                self.cap(2, paren, paren + 1);
                Some(paren + 1)
            }
            Rx::AppleFoundationFunctionalMacrosEnd => self.lit(p, b")"),
            Rx::CPreprocessorRuleEnabledAt0 => self.hash_word_if(p, b"if", Truth::One),
            Rx::CPreprocessorRuleEnabledAt0End => {
                self.hash_word(p, &[b"endif"]).map(|(_, end)| end)
            }
            Rx::CPreprocessorRuleEnabledAt0At0 => {
                let comment = self.at(p) == b'/'
                    && (self.at(p + 1) == b'/'
                        || (self.at(p + 1) == b'*' && !self.continued_from(p + 2)));
                (self.at_g(p) && p < self.lim() && !comment).then_some(p)
            }
            Rx::CPreprocessorRuleEnabledAt0At0End => (self.starts(p, b"//")
                || self.block_comment_ahead(p)
                || (p == self.lim() && self.at(p) == b'\n'))
                .then_some(p),
            Rx::CPreprocessorRuleConditionalLineAt0 => self.defined_test(p),
            Rx::CPreprocessorRuleConditionalLineAt1 => self.keyword(p, b"defined"),
            Rx::CCommentsAt0 => self.banner_block(p),
            Rx::CCommentsAt1 => self.lit(p, b"/*"),
            Rx::CCommentsAt1End => self.lit(p, b"*/"),
            Rx::CCommentsAt2 => self.banner_line(p),
            Rx::CCommentsAt3 => {
                if p == 0 {
                    let q = self.skip(0, |b| b == b' ' || b == b'\t');
                    if q > 0 && self.starts(q, b"//") {
                        self.cap(1, 0, q);
                        return Some(q);
                    }
                }
                self.starts(p, b"//").then_some(p)
            }
            Rx::CCommentsAt3End => (!self.at_g(p)).then_some(p),
            Rx::CCommentsAt3At0 => self.lit(p, b"//"),
            Rx::CCommentsAt3At0End => (p == self.lim() && self.at(p) == b'\n').then_some(p),
            Rx::CLineContinuationCharacterAt0 => {
                if self.at(p) == b'\\' && p + 1 == self.lim() && self.at(p + 1) == b'\n' {
                    self.cap(1, p, p + 1);
                    return Some(p + 2);
                }
                None
            }
            Rx::CStringsAt1 => self.lit(p, b"'"),
            Rx::CNumbers => self.number_begin(p),
            Rx::CNumbersEnd => (!self.number_char(p)).then_some(p),
            Rx::CNumbersAt0 => self.number_pattern(p, Self::hex_float),
            Rx::CNumbersAt0Cap2At0 => self.digit_sep(p).then_some(p + 1),
            Rx::CNumbersAt1 => self.number_pattern(p, Self::dec_float),
            Rx::CNumbersAt2 => self.number_pattern(p, Self::binary),
            Rx::CNumbersAt3 => self.number_pattern(p, Self::octal),
            Rx::CNumbersAt4 => self.number_pattern(p, Self::hex_int),
            Rx::CNumbersAt5 => self.number_pattern(p, Self::dec_int),
            Rx::CNumbersAt6 => self.invalid_number(p),
            Rx::CPreprocessorRuleConditionalLineAt5 => self.lit(p, b"?"),
            Rx::CPreprocessorRuleConditionalLineAt5End => self.lit(p, b":"),
            Rx::COperatorsAt0 => {
                if self.word_before(p) || self.prev_is(p, b'$') {
                    return None;
                }
                let end = self.lit(p, b"sizeof")?;
                (self.word_at(end).is_none() && self.at(end) != b'$').then_some(end)
            }
            Rx::COperatorsAt1 => self.lit(p, b"--"),
            Rx::COperatorsAt2 => self.lit(p, b"++"),
            Rx::COperatorsAt3 => {
                let ok = match self.at(p) {
                    b'-' | b'%' | b'*' | b'+' => true,
                    b'/' => !self.prev_is(p, b'('),
                    _ => false,
                };
                (ok && self.at(p + 1) == b'=' && p + 1 < self.lim()).then_some(p + 2)
            }
            Rx::COperatorsAt4 => {
                let len = match self.at(p) {
                    b'&' | b'^' | b'|' => 1,
                    b'<' if self.at(p + 1) == b'<' => 2,
                    b'>' if self.at(p + 1) == b'>' => 2,
                    _ => return None,
                };
                (self.at(p + len) == b'=' && p + len < self.lim()).then_some(p + len + 1)
            }
            Rx::COperatorsAt5 => self.lit(p, b"<<").or_else(|| self.lit(p, b">>")),
            Rx::COperatorsAt6 => {
                for op in [&b"!="[..], b"<=", b">=", b"=="] {
                    if let Some(end) = self.lit(p, op) {
                        return Some(end);
                    }
                }
                (matches!(self.at(p), b'<' | b'>') && p < self.lim()).then_some(p + 1)
            }
            Rx::COperatorsAt7 => self
                .lit(p, b"&&")
                .or_else(|| self.lit(p, b"!"))
                .or_else(|| self.lit(p, b"||")),
            Rx::COperatorsAt8 => {
                (matches!(self.at(p), b'&' | b'^' | b'|' | b'~') && p < self.lim()).then_some(p + 1)
            }
            Rx::COperatorsAt9 => self.lit(p, b"="),
            Rx::COperatorsAt10 => (matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/')
                && p < self.lim())
            .then_some(p + 1),
            Rx::COperatorsAt11 => self.one_char(p, b'?'),
            Rx::COperatorsAt11End => self.one_char(p, b':'),
            Rx::CStorageTypesAt0 => self.free_word_in(p, PRIMITIVES),
            Rx::CStorageTypesAt1 => {
                if self.word_before(p) {
                    return None;
                }
                let end = self.word_end(p);
                (end > p && self.lists(p, end) & words::STORAGE != 0).then_some(end)
            }
            Rx::CStorageTypesAt2 => {
                let end = self.word_in(p, &[b"asm", b"__asm__", b"enum", b"struct", b"union"])?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::CMethodAccess => {
                let end = self.access_chain(p, Last::Method)?;
                self.cap(6, end, end + 1);
                Some(end + 1)
            }
            Rx::SpecialVariablesAt0 => self.keyword(p, b"_cmd"),
            Rx::SpecialVariablesAt1 => self.word_in(p, &[b"self", b"super"]),
            Rx::CMethodAccessCap1At1 => (p < self.lim()).then(|| self.lim()),
            Rx::CMemberAccess => self.access_chain(p, Last::Member),
            Rx::CMemberAccessCap4At2 => self.access_head(p),
            Rx::CFunctionCallInnardsAt5 => self.call_innards(p),
            Rx::CFunctionCallInnardsAt6 => self.lit(p, b"("),
            Rx::CPreprocessorRuleEnabledBlockAt0At2 => {
                self.hash_word(p, &[b"else"]).map(|(_, end)| end)
            }
            Rx::CPreprocessorRuleEnabledBlockAt0At2End => {
                self.directive_one(p, b"endif").map(|_| p)
            }
            Rx::CDisabledEnd => self.directive_one(p, b"endif").map(|(_, _, end)| end),
            Rx::CPragmaMark => self.pragma_mark(p, true),
            Rx::CPreprocessorRuleEnabledBlockAt0At3 => {
                self.hash_word(p, &[b"elif"]).map(|(_, end)| end)
            }
            Rx::CPreprocessorRuleEnabledBlockAt0At3End | Rx::CPreprocessorRuleDisabledElifEnd => {
                self.directive(p, ELSE_ELIF_ENDIF).map(|_| p)
            }
            Rx::CPreprocessorRuleEnabledBlockAt0At4 => {
                (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
            }
            Rx::CPreprocessorRuleDisabledBlockAt0 => self.hash_word_if(p, b"if", Truth::Zero),
            Rx::CPreprocessorRuleEnabledElifBlock => self.hash_word_if(p, b"elif", Truth::One),
            Rx::CPreprocessorRuleEnabledElifBlockAt0End
            | Rx::CPreprocessorRuleConditionalLineAt10End => {
                if rx == Rx::CPreprocessorRuleConditionalLineAt10End
                    && self.at(p) == b')'
                    && p < self.lim()
                {
                    return Some(p + 1);
                }
                (self.starts(p, b"//") || self.block_comment_ahead(p) || self.line_feed_here(p))
                    .then_some(p)
            }
            Rx::CPreprocessorRuleEnabledElifBlockAt2At0 => {
                let (start, end) = self.hash_word(p, &[b"else"])?;
                self.cap(3, start, end);
                Some(end)
            }
            Rx::CPreprocessorRuleEnabledElifBlockAt2At1 => {
                let (start, end) = self.hash_word(p, &[b"elif"])?;
                self.cap(3, start, end);
                Some(end)
            }
            Rx::CPreprocessorRuleDisabledElif => self.hash_word_if(p, b"elif", Truth::Zero),
            Rx::CPreprocessorRuleConditionalBlockAt0 => {
                self.hash_word(p, IF_WORDS).map(|(_, end)| end)
            }
            Rx::CPreprocessorRuleConditionalBlockAt1 => {
                let (_, start, end) = self.directive(p, ELSE_ELIF_ENDIF)?;
                self.cap(1, start, end);
                Some(end)
            }
            Rx::CCFunctionCall => self.call_target(p, words::FUNCTION_EXCLUDED).map(|_| p),
            Rx::CCFunctionCallEnd => {
                (self.prev_is(p, b')') && self.word_at(p).is_none()).then_some(p)
            }
            Rx::CBlockInnardsAt6 => self.operator_word_call(p),
            Rx::CBlockInnardsAt7 => self.lit(p, b"{"),
            Rx::CBlockInnardsAt7End | Rx::CPreprocessorRuleDefineLineContentsAt1End => {
                if let Some(end) = self.lit(p, b"}") {
                    return Some(end);
                }
                if self.else_ahead(p) {
                    return Some(p);
                }
                (rx == Rx::CPreprocessorRuleDefineLineContentsAt1End && self.blank_to_line_feed(p))
                    .then_some(p)
            }
            Rx::CParensBlockAt1 => (!self.prev_is(p, b':')
                && self.at(p) == b':'
                && p < self.lim()
                && self.at(p + 1) != b':')
                .then_some(p + 1),
            Rx::CPreprocessorRuleConditionalLineAt7 => {
                self.word_in(p, &[b"NULL", b"true", b"false", b"TRUE", b"FALSE"])
            }
            Rx::CPreprocessorRuleConditionalLineAt8 => self.dollar_name(p, false),
            Rx::CSwitchStatement => {
                let end = self.free_word_in(p, &[b"switch"])?;
                self.cap(1, p, end);
                self.cap(2, p, end);
                Some(end)
            }
            Rx::CSwitchStatementEnd => (self.prev_is(p, b'}')
                || (matches!(self.at(p), b']' | b';' | b'=' | b'>' | b'[') && p < self.lim()))
            .then_some(p),
            Rx::CSwitchStatementAt0 => {
                self.at_g(p).then_some(())?;
                Some(if self.at(p) == b' ' && p < self.lim() {
                    p + 1
                } else {
                    p
                })
            }
            Rx::CSwitchStatementAt0End => {
                if self.at(p) == b'{' && p < self.lim() {
                    self.cap(1, p, p + 1);
                    return Some(p + 1);
                }
                if self.at(p) == b';' && p < self.lim() {
                    self.cap(1, p, p);
                    return Some(p);
                }
                None
            }
            Rx::CSwitchStatementAt1 => self.prev_is(p, b'{').then_some(p),
            Rx::CSwitchStatementAt1End => self.one_char(p, b'}'),
            Rx::CDefaultStatement => {
                let end = self.free_word_in(p, &[b"default"])?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::CCaseStatement => {
                let end = self.free_word_in(p, &[b"case"])?;
                self.cap(1, p, end);
                Some(end)
            }
            Rx::CSwitchStatementAt2 => self.prev_is(p, b'}').then(|| self.space_end(p)),
            Rx::CSwitchStatementAt2End => {
                let q = self.space_end(p);
                (self.at(q) == b';' && q < self.lim()).then_some(q)
            }
            Rx::CLangAt5 => self.word_in(p, CONTROL),
            Rx::CLangAt7 => self.lit(p, b"typedef"),
            Rx::CLangAt8 => self.keyword(p, b"in"),
            Rx::CLangAt9 => self.word_in(p, MODIFIERS),
            Rx::CLangAt10 => self.prefixed_name(p, b'k'),
            Rx::CLangAt11 => self.prefixed_name(p, b'g'),
            Rx::CLangAt12 => self.prefixed_name(p, b's'),
            Rx::CLangAt18 => self.define(p),
            Rx::CLangAt18End => {
                (self.starts(p, b"//") || self.starts(p, b"/*") || self.line_feed_here(p))
                    .then_some(p)
            }
            Rx::CVarargEllipses => {
                (!self.prev_is(p, b'.') && self.starts(p, b"...") && self.at(p + 3) != b'.')
                    .then_some(p + 3)
            }
            Rx::CPreprocessorRuleDefineLineContentsAt4 => {
                self.call_target(p, words::DEFINE_EXCLUDED).map(|_| p)
            }
            Rx::CPreprocessorRuleDefineLineContentsAt4End => {
                ((self.prev_is(p, b')') && self.word_at(p).is_none()) || self.blank_to_line_feed(p))
                    .then_some(p)
            }
            Rx::CPreprocessorRuleDefineLineFunctionsAt6End => {
                if self.at(p) == b')' && p < self.lim() {
                    self.cap(1, p, p + 1);
                    return Some(p + 1);
                }
                self.blank_to_line_feed(p).then_some(p)
            }
            Rx::CPreprocessorRuleDefineLineContentsAt5End => self
                .lit(p, b"\"")
                .or_else(|| self.blank_to_line_feed(p).then_some(p)),
            Rx::CPreprocessorRuleDefineLineContentsAt6End => self
                .lit(p, b"'")
                .or_else(|| self.blank_to_line_feed(p).then_some(p)),
            Rx::CLangAt19 => self.directive_with_word(p, &[b"error", b"warning"]),
            Rx::CLangAt19End => self.line_feed_here(p).then_some(p),
            Rx::CLangAt19At2 => match self.at(p) {
                b'"' | b'\'' => None,
                b'\n' if p == self.lim() => Some(p + 1),
                _ if p < self.lim() => Some(self.char_end(p)),
                _ => None,
            },
            Rx::CLangAt19At2End => self.blank_to_line_feed(p).then_some(p),
            Rx::CLangAt20 => self.directive_with_word(p, &[b"include", b"include_next", b"import"]),
            Rx::CLangAt20At2 => self.lit(p, b"<"),
            Rx::CLangAt20At2End => self.lit(p, b">"),
            Rx::CLangAt22 => self.hash_word(p, &[b"line"]).map(|(_, end)| end),
            Rx::CLangAt23 => self.hash_word(p, &[b"undef"]).map(|(_, end)| end),
            Rx::CLangAt24 => self.hash_word(p, &[b"pragma"]).map(|(_, end)| end),
            Rx::CLangAt24At1 => self.dollar_name(p, true),
            Rx::CLangAt25 => self.word_in(p, SYSTEM_TYPES),
            Rx::CLangAt26 => self.word_in(p, PTHREAD_TYPES),
            Rx::CLangAt27 => {
                if !self.boundary(p) || self.word_at(p).is_none() {
                    return None;
                }
                let end = self.word_end(p);
                let word = self.text(p, end);
                let ok = word
                    .strip_suffix(b"_t")
                    .is_some_and(|stem| crate::bytes::contains(STDINT, stem));
                ok.then_some(end)
            }
            Rx::CLangAt28 => self.word_in(
                p,
                &[
                    b"noErr",
                    b"kNilOptions",
                    b"kInvalidID",
                    b"kVariableLengthArray",
                ],
            ),
            Rx::CLangAt29 => self.listed_word(p, words::MAC_TYPE),
            Rx::CLangAt30 => self.underscore_t(p),
            Rx::CLangAt33 => self.call_ahead(p),
            Rx::CLangAt33End => self.prev_is(p, b')').then_some(p),
            Rx::CProbablyAParameter => self.parameter(p),
            Rx::CLangAt35 => self.bracket_begin(p),
            Rx::CLangAt35End => self.lit(p, b"]"),
            Rx::CLangAt36 => {
                let end = self.lit(p, b"[")?;
                let close = self.space_end(end);
                (self.at(close) == b']' && close < self.lim()).then_some(close + 1)
            }
            Rx::CLangAt37 => self.lit(p, b";"),
            Rx::CLangAt38 => self.lit(p, b","),
            Rx::AnonymousPattern7 => {
                self.at_sign_word(p, &[b"try", b"catch", b"finally", b"throw"])
            }
            Rx::AnonymousPattern8 => self.at_sign_word(p, &[b"synchronized"]),
            Rx::AnonymousPattern9 => self.at_sign_word(p, &[b"required", b"optional"]),
            Rx::AnonymousPattern10 => self.at_sign_word(p, &[b"defs", b"encode"]),
            Rx::AnonymousPattern11 => self.keyword(p, b"id"),
            Rx::AnonymousPattern12 => self.word_in(p, OBJC_TYPES),
            Rx::AnonymousPattern13 => self.at_sign_word(p, &[b"class", b"protocol"]),
            Rx::AnonymousPattern14 => {
                let end = self.lit(p, b"@selector")?;
                let paren = self.space_end(end);
                if self.at(paren) != b'(' || paren >= self.lim() {
                    return None;
                }
                self.cap(1, p, end);
                self.cap(2, p, p + 1);
                self.cap(3, paren, paren + 1);
                Some(paren + 1)
            }
            Rx::AnonymousPattern14At0 => {
                let b = self.at(p);
                if !(b == b':' || b == b'_' || b.is_ascii_alphabetic())
                    || p >= self.lim()
                    || !self.boundary(p)
                {
                    return None;
                }
                let mut i = p + 1;
                loop {
                    if self.at(i) == b':' && i < self.lim() {
                        i += 1;
                    } else if let Some(width) = self.word_at(i) {
                        i += width;
                    } else {
                        break;
                    }
                }
                self.spend(i - p);
                Some(i)
            }
            Rx::AnonymousPattern15 => self.at_sign_word(
                p,
                &[
                    b"synchronized",
                    b"public",
                    b"package",
                    b"private",
                    b"protected",
                ],
            ),
            Rx::AnonymousPattern16 => self.word_in(p, &[b"YES", b"NO", b"Nil", b"nil"]),
            Rx::AnonymousPattern17 => self.keyword(p, b"NSApp"),
            Rx::AnonymousPattern18 => self.spaced_word(p, words::LEOPARD_FUNCTION),
            Rx::AnonymousPattern19 => self.spaced_word(p, words::FUNCTION),
            Rx::AnonymousPattern20 => self.listed_word(p, words::LEOPARD_CLASS),
            Rx::AnonymousPattern21 => self.listed_word(p, words::CLASS),
            Rx::AnonymousPattern22 => self.listed_word(p, words::LEOPARD_TYPE),
            Rx::AnonymousPattern23 => self.listed_word(p, words::QUARTZ_CLASS),
            Rx::AnonymousPattern24 => self.listed_word(p, words::QUARTZ_TYPE),
            Rx::AnonymousPattern25 => self.listed_word(p, words::TYPE),
            Rx::AnonymousPattern26 => self.listed_word(p, words::CONSTANT),
            Rx::AnonymousPattern27 => self.listed_word(p, words::LEOPARD_NOTIFICATION),
            Rx::AnonymousPattern28 => self.listed_word(p, words::NOTIFICATION),
            Rx::AnonymousPattern29 => self.listed_word(p, words::LEOPARD_CONSTANT),
            Rx::AnonymousPattern30 => self.listed_word(p, words::COCOA_CONSTANT),
            Rx::BracketedContent => self.lit(p, b"["),
            Rx::BracketedContentAt0 => {
                let end = self.lit(p, b"predicateWithFormat:")?;
                if p < 12 || self.text(p - 12, p) != b"NSPredicate " {
                    return None;
                }
                self.cap(1, p, end);
                Some(end)
            }
            Rx::BracketedContentAt0End => (self.at(p) == b']' && p < self.lim()).then_some(p),
            Rx::BracketedContentAt0At0 => {
                if !self.boundary(p) {
                    return None;
                }
                let word = self.lit(p, b"argument")?;
                let end = self.lit(word, b"Array").or_else(|| self.lit(word, b"s"))?;
                if self.at(end) != b':' || end >= self.lim() {
                    return None;
                }
                self.cap(1, word, end);
                self.cap(2, end, end + 1);
                Some(end + 1)
            }
            Rx::BracketedContentAt0At1 => {
                if !self.boundary(p) || self.word_at(p).is_none() {
                    return None;
                }
                let end = self.word_end(p);
                if self.at(end) != b':' || end >= self.lim() {
                    return None;
                }
                self.cap(1, end, end + 1);
                Some(end + 1)
            }
            Rx::BracketedContentAt0At2At0 => self.word_in(p, &[b"AND", b"OR", b"NOT", b"IN"]),
            Rx::BracketedContentAt0At2At1 => self.word_in(p, &[b"ALL", b"ANY", b"SOME", b"NONE"]),
            Rx::BracketedContentAt0At2At2 => self.word_in(
                p,
                &[
                    b"NULL", b"NIL", b"SELF", b"TRUE", b"YES", b"FALSE", b"NO", b"FIRST", b"LAST",
                    b"SIZE",
                ],
            ),
            Rx::BracketedContentAt0At2At3 => self.word_in(
                p,
                &[
                    b"MATCHES",
                    b"CONTAINS",
                    b"BEGINSWITH",
                    b"ENDSWITH",
                    b"BETWEEN",
                ],
            ),
            Rx::BracketedContentAt0At2At4 => self.word_in(p, &[b"CASEINSENSITIVE", b"CI"]),
            Rx::BracketedContentAt0At2At5 => self.word_in(
                p,
                &[
                    b"ANYKEY",
                    b"SUBQUERY",
                    b"CAST",
                    b"TRUEPREDICATE",
                    b"FALSEPREDICATE",
                ],
            ),
            Rx::BracketedContentAt0At2At6 => self.escape(p, false),
            Rx::CFunctionsAt0 => self.spaced_word(p, words::C99_FUNCTION),
            Rx::CFunctionsAt1 => self.objc_call(p),
            Rx::BracketedContentAt1 => {
                if self.word_at(p).is_none() || p < 2 || self.at(p - 1) != b' ' {
                    return None;
                }
                let before = p - 1;
                let ok = matches!(self.prev(before), Some(b']' | b'"' | b')'))
                    || self.word_before(before);
                if !ok {
                    return None;
                }
                let end = self.word_end(p);
                if self.at(end) == b':' && end < self.lim() {
                    self.cap(1, p, end + 1);
                    self.cap(2, end, end + 1);
                    return Some(end + 1);
                }
                if self.at(end) == b']' && end < self.lim() {
                    self.cap(1, p, end);
                    return Some(end);
                }
                None
            }
            Rx::MethodAt1 => {
                if !self.boundary(p) || self.word_at(p).is_none() {
                    return None;
                }
                let end = self.word_end(p);
                (self.at(end) == b':' && end < self.lim()).then_some(end)
            }
            Rx::MethodAt2 => {
                let end = self.lit(p, b":")?;
                let paren = self.space_end(end);
                if self.at(paren) != b'(' || paren >= self.lim() {
                    return None;
                }
                self.cap(1, p, end);
                self.cap(2, p, end);
                self.cap(3, paren, paren + 1);
                Some(paren + 1)
            }
            Rx::MethodAt2End => {
                let end = self.lit(p, b")")?;
                self.cap(1, p, end);
                let word = self.space_end(end);
                if self.word_at(word).is_some() {
                    let word_end = self.word_end(word);
                    self.cap(2, word, word_end);
                    return Some(word_end);
                }
                Some(word)
            }
            Rx::CommentAt1At0At0 => {
                if self.at(p) != b'\\' || p >= self.lim() {
                    return None;
                }
                (self.blank_end(p + 1) == self.lim() && self.at(self.lim()) == b'\n')
                    .then_some(self.lim() + 1)
            }
        }
    }

    /// One byte `byte` at `p`, recorded as group 1.
    fn one_char(&mut self, p: usize, byte: u8) -> Option<usize> {
        if self.at(p) != byte || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `(\))`, and with `word`, `(\))\s*(\w+)\b`.
    fn close_paren_word(&mut self, p: usize, word: bool) -> Option<usize> {
        let end = self.lit(p, b")")?;
        if !word {
            self.cap(1, p, end);
            return Some(end);
        }
        let start = self.space_end(end);
        self.word_at(start)?;
        let word_end = self.word_end(start);
        self.cap(1, p, end);
        self.cap(2, start, word_end);
        Some(word_end)
    }

    /// `(@)(words)\b`: `@` recorded as group 1 and a word of `words` right
    /// after it.
    fn at_sign_word(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let start = self.lit(p, b"@")?;
        let end = self.word_end(start);
        if !crate::bytes::contains(words, self.text(start, end)) {
            return None;
        }
        self.cap(1, p, start);
        Some(end)
    }

    /// `(?<!\w)(words)(?!\w)` at `p`.
    fn free_word_in(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if self.word_before(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        crate::bytes::contains(words, self.text(p, end)).then_some(end)
    }

    /// `[$A-Z_a-z][$\w]*`, with `-` among the later characters when `dash`
    /// holds.
    fn dollar_name(&self, p: usize, dash: bool) -> Option<usize> {
        let b = self.at(p);
        if !(b == b'$' || b == b'_' || b.is_ascii_alphabetic()) || p >= self.lim() {
            return None;
        }
        let mut i = p + 1;
        loop {
            let c = self.at(i);
            if (c == b'$' || (dash && c == b'-')) && i < self.lim() {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - p);
        Some(i)
    }

    /// `(?=\s*#\s*e(?:lif|lse|ndif)\b)` at `p`.
    fn else_ahead(&self, p: usize) -> bool {
        let hash = self.blank_end(p);
        if self.at(hash) != b'#' {
            return false;
        }
        let start = self.blank_end(hash + 1);
        let end = self.word_end(start);
        crate::bytes::contains(ELSE_ELIF_ENDIF, self.text(start, end))
    }

    /// `^\s*((#)\s*(words))\b\s*`, with the word recorded as group 3: the
    /// end of the blanks after it.
    fn directive_with_word(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let (start, end) = self.hash_word(p, words)?;
        self.cap(3, start, end);
        Some(self.space_end(end))
    }

    /// `\\(["'?\\abefnprtv]|[0-3]\d{0,2}|[4-7]\d?|x\h{0,2}|u\h{0,4}|U\h{0,8})`,
    /// or with `string` off the predicate's
    /// `\\(["'?\\abefnrtv]|[0-3]\d{0,2}|[4-7]\d?|x[0-9A-Za-z]+)`.
    fn escape(&self, p: usize, string: bool) -> Option<usize> {
        let i = self.lit(p, b"\\")?;
        let c = self.at(i);
        if i >= self.lim() {
            return None;
        }
        let simple: &[u8] = if string {
            b"\"'?\\abefnprtv"
        } else {
            b"\"'?\\abefnrtv"
        };
        if simple.contains(&c) {
            return Some(i + 1);
        }
        match c {
            b'0'..=b'3' => Some(self.digits_up_to(i + 1, 2)),
            b'4'..=b'7' => Some(self.digits_up_to(i + 1, 1)),
            b'x' if string => Some(self.hex_up_to(i + 1, 2)),
            b'u' if string => Some(self.hex_up_to(i + 1, 4)),
            b'U' if string => Some(self.hex_up_to(i + 1, 8)),
            b'x' => {
                let end = self.skip(i + 1, |b| b.is_ascii_alphanumeric());
                (end > i + 1).then_some(end)
            }
            _ => None,
        }
    }

    /// The end of up to `max` `\d` characters from `i`.
    fn digits_up_to(&self, mut i: usize, max: usize) -> usize {
        for _ in 0..max {
            match self.char_at(i) {
                Some((c, end)) if crate::classes::is_digit(c) => i = end,
                _ => break,
            }
        }
        i
    }

    /// The end of up to `max` hex digits from `i`.
    fn hex_up_to(&self, mut i: usize, max: usize) -> usize {
        for _ in 0..max {
            if i < self.lim() && self.at(i).is_ascii_hexdigit() {
                i += 1;
            } else {
                break;
            }
        }
        i
    }

    /// `-?\d+` at `i`: where it ends.
    fn signed_digits(&self, i: usize) -> Option<usize> {
        let start = if self.at(i) == b'-' { i + 1 } else { i };
        let end = self.digit_end(start);
        (end > start).then_some(end)
    }

    /// `((-?\d+)|\*(-?\d+\$)?)?` at `i`: a width or precision, or nothing.
    fn width(&self, i: usize) -> usize {
        if let Some(end) = self.signed_digits(i) {
            return end;
        }
        if self.at(i) == b'*' && i < self.lim() {
            if let Some(end) = self.signed_digits(i + 1)
                && self.at(end) == b'$'
            {
                return end + 1;
            }
            return i + 1;
        }
        i
    }

    /// What follows `%` up to the length modifier: `(\d+\$)?[- #'+0]*`, then
    /// `[,:;_]?` when `separator` holds, then a width and a precision.
    fn format_spec(&self, p: usize, separator: bool) -> usize {
        let mut i = p;
        let digits = self.digit_end(i);
        if digits > i && self.at(digits) == b'$' {
            i = digits + 1;
        }
        i = self.skip(i, |b| matches!(b, b'-' | b' ' | b'#' | b'\'' | b'+' | b'0'));
        if separator && matches!(self.at(i), b',' | b':' | b';' | b'_') && i < self.lim() {
            i += 1;
        }
        i = self.width(i);
        if self.at(i) == b'.' && i < self.lim() {
            i = self.width(i + 1);
        }
        i
    }

    /// `AnonymousPattern3At1`, a `%@` format.
    fn format_at(&self, p: usize) -> Option<usize> {
        let i = self.format_spec(self.lit(p, b"%")?, false);
        self.lit(i, b"@")
    }

    /// `StringPlaceholderAt0`, a `printf` placeholder.
    fn placeholder(&self, p: usize) -> Option<usize> {
        let i = self.format_spec(self.lit(p, b"%")?, true);
        let conversion = |j: usize| {
            let c = self.at(j);
            j < self.lim()
                && (matches!(
                    c,
                    b'%' | b'A'
                        | b'O'
                        | b'S'
                        | b'U'
                        | b'X'
                        | b'a'
                        | b'i'
                        | b'n'
                        | b'o'
                        | b'p'
                        | b's'
                        | b'u'
                        | b'x'
                ) || (b'C'..=b'G').contains(&c)
                    || (b'c'..=b'g').contains(&c))
        };
        for modifier in [
            &b"hh"[..],
            b"h",
            b"ll",
            b"L",
            b"j",
            b"l",
            b"q",
            b"t",
            b"z",
            b"vh",
            b"vl",
            b"v",
            b"hv",
            b"hl",
            b"",
        ] {
            let end = i + modifier.len();
            if self.text(i, end) == modifier && end <= self.lim() && conversion(end) {
                return Some(end + 1);
            }
        }
        None
    }

    /// `((@)(interface|protocol))(?!.+;)\s+([A-Z_a-z][0-9A-Z_a-z]*)\s*
    /// ((:)\s*([A-Za-z][0-9A-Za-z]*))?([\n\s])?`.
    fn interface_begin(&mut self, p: usize) -> Option<usize> {
        let start = self.lit(p, b"@")?;
        let word = self
            .lit(start, b"interface")
            .or_else(|| self.lit(start, b"protocol"))?;
        if memchr::memchr(b';', self.text(word + 1, self.lim())).is_some() {
            return None;
        }
        let name = self.space_end(word);
        if name == word {
            return None;
        }
        let name_end = self.ascii_ident_end(name)?;
        self.cap(1, p, word);
        self.cap(2, p, start);
        self.cap(3, start, word);
        self.cap(4, name, name_end);
        let colon = self.space_end(name_end);
        if self.at(colon) == b':' && colon < self.lim() {
            let parent = self.space_end(colon + 1);
            if self.at(parent).is_ascii_alphabetic() && parent < self.lim() {
                let parent_end = self.skip(parent + 1, |b| b.is_ascii_alphanumeric());
                self.cap(5, colon, parent_end);
                self.cap(6, colon, colon + 1);
                self.cap(7, parent, parent_end);
                let width = self.space_at(parent_end);
                if width > 0 {
                    self.cap(8, parent_end, parent_end + width);
                }
                return Some(parent_end + width);
            }
        }
        Some(colon)
    }

    /// `((@)(implementation))\s+([A-Z_a-z][0-9A-Z_a-z]*)\s*
    /// (?::\s*([A-Za-z][0-9A-Za-z]*))?`.
    fn implementation_begin(&mut self, p: usize) -> Option<usize> {
        let start = self.lit(p, b"@")?;
        let word = self.lit(start, b"implementation")?;
        let name = self.space_end(word);
        if name == word {
            return None;
        }
        let name_end = self.ascii_ident_end(name)?;
        self.cap(1, p, word);
        self.cap(2, p, start);
        self.cap(3, start, word);
        self.cap(4, name, name_end);
        let colon = self.space_end(name_end);
        if self.at(colon) == b':' && colon < self.lim() {
            let parent = self.space_end(colon + 1);
            if self.at(parent).is_ascii_alphabetic() && parent < self.lim() {
                let parent_end = self.skip(parent + 1, |b| b.is_ascii_alphanumeric());
                self.cap(5, parent, parent_end);
                return Some(parent_end);
            }
        }
        Some(colon)
    }

    /// `^\s*((#)\s*define)\s+((?<id>[$A-Z_a-z][$\w]*))(?:(\()(\s*\g<id>\s*
    /// ((,)\s*\g<id>\s*)*(?:\.\.\.)?)(\)))?`.
    fn define(&mut self, p: usize) -> Option<usize> {
        let (_, end) = self.hash_word(p, &[b"define"])?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let name_end = self.dollar_name(name, false)?;
        self.cap(3, name, name_end);
        self.cap(4, name, name_end);
        if self.at(name_end) != b'(' || name_end >= self.lim() {
            return Some(name_end);
        }
        let open = name_end;
        let mut i = self.space_end(open + 1);
        let Some(first) = self.dollar_name(i, false) else {
            return Some(name_end);
        };
        let mut last_id = (i, first);
        i = self.space_end(first);
        let mut comma = None;
        while self.at(i) == b',' && i < self.lim() {
            let next = self.space_end(i + 1);
            let Some(next_end) = self.dollar_name(next, false) else {
                break;
            };
            let after = self.space_end(next_end);
            comma = Some((i, after));
            last_id = (next, next_end);
            i = after;
        }
        if self.starts(i, b"...") {
            i += 3;
        }
        if self.at(i) != b')' || i >= self.lim() {
            return Some(name_end);
        }
        self.cap(4, last_id.0, last_id.1);
        self.cap(5, open, open + 1);
        self.cap(6, open + 1, i);
        if let Some((at, after)) = comma {
            self.cap(7, at, after);
            self.cap(8, at, at + 1);
        }
        self.cap(9, i, i + 1);
        Some(i + 1)
    }
}
