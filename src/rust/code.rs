//! The rule groups of the grammar, and which contexts use which, in order.

use super::lexer::Lexer;
use super::numbers::NUMERIC;
use super::scope::S;
use super::state::Rule;
use crate::classes::is_ascii_word;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Group {
    BlockComments,
    Comments,
    Attributes,
    Lvariables,
    Constants,
    Gtypes,
    Functions,
    Types,
    Keywords,
    Lifetimes,
    Macros,
    Namespaces,
    Punctuation,
    Strings,
    Variables,
}

use Group::{
    Attributes, BlockComments, Comments, Constants, Functions, Gtypes, Keywords, Lifetimes,
    Lvariables, Macros, Namespaces, Punctuation, Strings, Types, Variables,
};

impl Group {
    /// Whether the rules of the group can match at a byte `b`.
    /// The lexer then asks only the groups that can.
    /// Every rule begins by testing this byte.
    /// A group that answers false would have declined anyway.
    pub(crate) fn can_start(self, b: u8) -> bool {
        match self {
            BlockComments | Comments => b == b'/',
            Attributes => b == b'#',
            Lvariables | Constants | Gtypes | Functions | Types | Namespaces | Variables => {
                is_ascii_word(b)
            }
            Macros => b.is_ascii_alphabetic() || b == b'_',
            Lifetimes => matches!(b, b'\'' | b'&'),
            Strings => matches!(b, b'"' | b'\'' | b'b' | b'r'),
            Punctuation => matches!(
                b,
                b',' | b'{' | b'}' | b'(' | b')' | b';' | b'[' | b']' | b'<' | b'>'
            ),
            Keywords => true,
        }
    }
}

/// The rules active in a context, in the grammar's priority order.
pub(crate) fn groups(rule: Rule) -> &'static [Group] {
    match rule {
        Rule::Top => &[
            BlockComments,
            Comments,
            Attributes,
            Lvariables,
            Constants,
            Gtypes,
            Functions,
            Types,
            Keywords,
            Lifetimes,
            Macros,
            Namespaces,
            Punctuation,
            Strings,
            Variables,
        ],
        Rule::FunctionDefinition => &[
            BlockComments,
            Comments,
            Keywords,
            Lvariables,
            Constants,
            Gtypes,
            Functions,
            Lifetimes,
            Macros,
            Namespaces,
            Punctuation,
            Strings,
            Types,
            Variables,
        ],
        Rule::Call => &[
            BlockComments,
            Comments,
            Attributes,
            Keywords,
            Lvariables,
            Constants,
            Gtypes,
            Functions,
            Lifetimes,
            Macros,
            Namespaces,
            Punctuation,
            Strings,
            Types,
            Variables,
        ],
        Rule::Generic => &[
            BlockComments,
            Comments,
            Keywords,
            Lvariables,
            Lifetimes,
            Punctuation,
            Types,
            Variables,
        ],
        Rule::AngleSquare => &[
            BlockComments,
            Comments,
            Gtypes,
            Lvariables,
            Lifetimes,
            Punctuation,
            Types,
        ],
        Rule::Attribute => &[
            BlockComments,
            Comments,
            Keywords,
            Lifetimes,
            Punctuation,
            Strings,
            Gtypes,
            Types,
        ],
        Rule::Use => &[
            BlockComments,
            Comments,
            Keywords,
            Namespaces,
            Punctuation,
            Types,
            Lvariables,
        ],
        Rule::Import => &[BlockComments, Comments, Keywords, Punctuation],
        Rule::String | Rule::Raw | Rule::Char | Rule::Block | Rule::Doc => &[],
    }
}

const SPECIFIERS: [&[u8]; 15] = [
    b"block",
    b"expr",
    b"expr_2021",
    b"ident",
    b"item",
    b"lifetime",
    b"literal",
    b"meta",
    b"pat",
    b"pat_param",
    b"path",
    b"stmt",
    b"tt",
    b"ty",
    b"vis",
];

impl Lexer<'_> {
    pub(crate) fn group(&mut self, group: Group, p: usize, b: u8) -> Option<usize> {
        match group {
            BlockComments => self.block_comment_start(p),
            Comments => self.line_comment(p),
            Attributes => self.attribute(p, b),
            Lvariables => self.language_variable(p),
            Constants => self.constants(p, b),
            Gtypes => self.gtypes(p),
            Functions => self.functions(p, b),
            Types => self.types(p, b),
            Keywords => self.keywords(p, b),
            Lifetimes => self.lifetime(p, b),
            Macros => self.macro_call(p, b),
            Namespaces => self.namespace(p, b),
            Punctuation => self.punctuation(p, b),
            Strings => self.string_start(p, b),
            Variables => self.variable(p, b),
        }
    }

    // Top-level only rules.

    pub(crate) fn top_rules(&mut self, p: usize, b: u8) -> Option<usize> {
        match b {
            b'<' if self.at(p + 1) == b'[' => {
                self.gap(p);
                if !self.push(Rule::AngleSquare) {
                    return None;
                }
                self.tok(p + 1, &[S::Angle]);
                self.tok(p + 2, &[S::Square]);
                Some(p + 2)
            }
            b'$' => self.metavariable(p),
            b'm' => self.macro_rules(p).or_else(|| self.module(p)),
            b'e' => self.extern_crate(p),
            b'u' => self.use_declaration(p),
            _ => None,
        }
    }

    fn metavariable(&mut self, p: usize) -> Option<usize> {
        let c = self.at(p + 1);
        let (rule, name_scope, name_end) = if self.starts_with(p + 1, b"crate") {
            (S::MetaMetavarType, S::KwCrate, p + 6)
        } else if c.is_ascii_uppercase() {
            (S::MetaMetavarType, S::EntTypeMetavar, self.word_run(p + 2))
        } else if c.is_ascii_lowercase() {
            (S::MetaMetavar, S::VarMetaName, self.word_run(p + 2))
        } else {
            return None;
        };
        let colon = self.blanks(name_end);
        let spec = (self.at(colon) == b':').then(|| {
            let start = self.blanks(colon + 1);
            (start, self.word_run(start))
        });
        let spec = spec.filter(|&(start, end)| SPECIFIERS.contains(&&self.s[start..end]));
        self.gap(p);
        self.tok(p + 1, &[rule, S::OpMacroDollar]);
        self.tok(name_end, &[rule, name_scope]);
        match spec {
            Some((start, end)) => {
                self.tok(colon, &[rule]);
                self.tok(colon + 1, &[rule, S::OpKeyValue]);
                self.tok(start, &[rule]);
                self.tok(end, &[rule, S::VarMetaSpec]);
                Some(end)
            }
            None => Some(name_end),
        }
    }

    fn macro_rules(&mut self, p: usize) -> Option<usize> {
        if !self.word_start(p) || !self.starts_with(p, b"macro_rules!") {
            return None;
        }
        let name = self.blanks(p + 12);
        if name == p + 12 {
            return None;
        }
        let lower = |b: u8| matches!(b, b'0'..=b'9' | b'_' | b'a'..=b'z');
        let (name_end, scope) = if lower(self.at(name)) {
            (self.skip(name, lower), S::EntMacroFn)
        } else if self.at(name).is_ascii_uppercase() {
            (self.skip(name + 1, lower), S::EntMacroType)
        } else {
            return None;
        };
        let brace = self.blanks(name_end);
        if brace == name_end || self.at(brace) != b'{' {
            return None;
        }
        let rule = S::MetaMacroRules;
        self.gap(p);
        self.tok(p + 12, &[rule, S::EntMacroRulesFn]);
        self.tok(name, &[rule]);
        self.tok(name_end, &[rule, scope]);
        self.tok(brace, &[rule]);
        self.tok(brace + 1, &[rule, S::Curly]);
        Some(brace + 1)
    }

    fn module(&mut self, p: usize) -> Option<usize> {
        if !self.starts_with(p, b"mod") {
            return None;
        }
        let name = self.blanks(p + 3);
        if name == p + 3 {
            return None;
        }
        let start = if self.raw_prefix(name) {
            name + 2
        } else {
            name
        };
        if !self.at(start).is_ascii_lowercase() {
            return None;
        }
        let end = self.skip(start, is_ascii_word);
        self.gap(p);
        self.tok(p + 3, &[S::StorageType]);
        self.tok(name, &[]);
        self.tok(end, &[S::EntModule]);
        Some(end)
    }

    fn extern_crate(&mut self, p: usize) -> Option<usize> {
        if !self.word_start(p) || !self.starts_with(p, b"extern") {
            return None;
        }
        let krate = self.blanks(p + 6);
        if krate == p + 6 || !self.starts_with(krate, b"crate") {
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Import) {
            return None;
        }
        self.tok(p + 6, &[S::StorageType]);
        self.tok(krate, &[]);
        self.tok(krate + 5, &[S::KwCrate]);
        Some(krate + 5)
    }

    fn use_declaration(&mut self, p: usize) -> Option<usize> {
        if !self.word_start(p) || !self.starts_with(p, b"use") {
            return None;
        }
        if p + 3 < self.s.len() && self.blanks(p + 3) == p + 3 {
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Use) {
            return None;
        }
        self.tok(p + 3, &[S::KwOther]);
        Some(p + 3)
    }

    // Identifiers.

    /// `r#` not followed by `crate`, `self`, `Self` or `super`.
    fn raw_prefix(&self, i: usize) -> bool {
        self.starts_with(i, b"r#")
            && ![&b"crate"[..], b"self", b"Self", b"super"]
                .iter()
                .any(|keyword| self.starts_with(i + 2, keyword))
    }

    /// The end of `(r#)?[0-9A-Za-z_]+` at `i`.
    fn ident_at(&self, i: usize) -> Option<usize> {
        if self.raw_prefix(i) {
            let end = self.skip(i + 2, is_ascii_word);
            if end > i + 2 {
                return Some(end);
            }
        }
        let end = self.skip(i, is_ascii_word);
        (end > i).then_some(end)
    }

    fn language_variable(&mut self, p: usize) -> Option<usize> {
        if !is_ascii_word(self.s[p]) || !self.word_start(p) {
            return None;
        }
        if !matches!(self.word_end - p, 4 | 5) || !matches!(self.s[p], b's' | b'S') {
            return None;
        }
        let scope = if self.word_is(p, b"self") || self.word_is(p, b"Self") {
            S::VarSelf
        } else if self.word_is(p, b"super") {
            S::VarSuper
        } else {
            return None;
        };
        self.gap(p);
        self.tok(self.word_end, &[scope]);
        Some(self.word_end)
    }

    fn gtypes(&mut self, p: usize) -> Option<usize> {
        if !is_ascii_word(self.s[p]) || !self.word_start(p) {
            return None;
        }
        if !matches!(self.s[p], b'S' | b'N' | b'O' | b'E') || self.word_end - p > 4 {
            return None;
        }
        let scope = match &self.s[p..self.word_end] {
            b"Some" | b"None" => S::TypeOption,
            b"Ok" | b"Err" => S::TypeResult,
            _ => return None,
        };
        self.gap(p);
        self.tok(self.word_end, &[scope]);
        Some(self.word_end)
    }

    fn variable(&mut self, p: usize, b: u8) -> Option<usize> {
        if !is_ascii_word(b) || !self.word_start(p) {
            return None;
        }
        if p > 0 && self.s[p - 1] == b'.' && !(p > 1 && self.s[p - 2] == b'.') {
            return None;
        }
        let lower = |b: u8| matches!(b, b'0'..=b'9' | b'_' | b'a'..=b'z');
        let mut end = None;
        if self.raw_prefix(p) {
            let e = self.skip(p + 2, lower);
            if e > p + 2 && self.boundary(e) {
                end = Some(e);
            }
        }
        let end = end.or_else(|| {
            let e = self.skip(p, lower);
            (e > p && self.boundary(e)).then_some(e)
        })?;
        self.gap(p);
        self.tok(end, &[S::VarOther]);
        Some(end)
    }

    fn lifetime(&mut self, p: usize, b: u8) -> Option<usize> {
        let quote = match b {
            b'\'' => p,
            b'&' => p + 1,
            _ => return None,
        };
        if self.at(quote) != b'\''
            || !(self.at(quote + 1).is_ascii_alphabetic() || self.at(quote + 1) == b'_')
        {
            return None;
        }
        let end = self.skip(quote + 1, is_ascii_word);
        if self.at(end) == b'\'' || !self.boundary(end) {
            return None;
        }
        self.gap(p);
        if quote > p {
            self.tok(quote, &[S::OpBorrow]);
        }
        self.tok(quote + 1, &[S::LifetimePunct]);
        self.tok(end, &[S::LifetimeName]);
        Some(end)
    }

    fn macro_call(&mut self, p: usize, b: u8) -> Option<usize> {
        if !(b.is_ascii_alphabetic() || b == b'_') || self.at(self.ident_end) != b'!' {
            return None;
        }
        let scope = if b.is_ascii_uppercase() {
            S::EntMacroType
        } else {
            S::EntMacroFn
        };
        self.gap(p);
        self.tok(self.ident_end + 1, &[S::MetaMacro, scope]);
        Some(self.ident_end + 1)
    }

    fn namespace(&mut self, p: usize, b: u8) -> Option<usize> {
        if !is_ascii_word(b) || (p > 0 && is_ascii_word(self.s[p - 1])) {
            return None;
        }
        let end = self.ident_end;
        let name = &self.s[p..end];
        if !self.starts_with(end, b"::") || name.ends_with(b"self") || name.ends_with(b"super") {
            return None;
        }
        self.gap(p);
        self.tok(end, &[S::EntNamespace]);
        self.tok(end + 2, &[S::OpNamespace]);
        Some(end + 2)
    }

    fn punctuation(&mut self, p: usize, b: u8) -> Option<usize> {
        let scope = match b {
            b',' => S::Comma,
            b'{' | b'}' => S::Curly,
            b'(' | b')' => S::Round,
            b';' => S::Semi,
            b'[' | b']' => S::Square,
            b'<' | b'>' if p == 0 || self.s[p - 1] != b'=' => S::Angle,
            _ => return None,
        };
        self.gap(p);
        self.tok(p + 1, &[scope]);
        Some(p + 1)
    }

    fn attribute(&mut self, p: usize, b: u8) -> Option<usize> {
        if b != b'#' {
            return None;
        }
        let bang = usize::from(self.at(p + 1) == b'!');
        if self.at(p + 1 + bang) != b'[' {
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Attribute) {
            return None;
        }
        self.tok(p + 1, &[S::PunctDefAttr]);
        self.tok(p + 1 + bang, &[]);
        self.tok(p + 2 + bang, &[S::AttrBracket]);
        Some(p + 2 + bang)
    }

    // Constants and numbers.

    fn constants(&mut self, p: usize, b: u8) -> Option<usize> {
        if !is_ascii_word(b) || !self.word_start(p) {
            return None;
        }
        let upper = |b: u8| b.is_ascii_digit() || b.is_ascii_uppercase() || b == b'_';
        if b.is_ascii_uppercase() && self.at(p + 1).is_ascii_uppercase() {
            let end = self.skip(p + 2, upper);
            if self.boundary(end) {
                self.gap(p);
                self.tok(end, &[S::ConstCaps]);
                return Some(end);
            }
        }
        if self.word_is(p, b"const") {
            let name = self.blanks(p + 5);
            if name > p + 5 && self.at(name).is_ascii_uppercase() {
                let end = self.skip(name, is_ascii_word);
                if self.boundary(end) {
                    self.gap(p);
                    self.tok(p + 5, &[S::StorageType]);
                    self.tok(name, &[]);
                    self.tok(end, &[S::ConstCaps]);
                    return Some(end);
                }
            }
        }
        if let Some(end) = self.decimal(p).or_else(|| self.radix(p)) {
            return Some(end);
        }
        if self.word_is(p, b"true") || self.word_is(p, b"false") {
            self.gap(p);
            self.tok(self.word_end, &[S::LangBool]);
            return Some(self.word_end);
        }
        None
    }

    // Functions.

    fn functions(&mut self, p: usize, b: u8) -> Option<usize> {
        if !is_ascii_word(b) {
            return None;
        }
        if self.word_start(p) && self.word_is(p, b"pub") {
            self.gap(p);
            self.tok(p + 3, &[S::StorageVis]);
            return Some(p + 3);
        }
        if self.word_start(p) && self.word_is(p, b"fn") {
            let name = self.blanks(p + 2);
            if name > p + 2
                && let Some(end) = self.ident_at(name)
                && matches!(self.at(end), b'(' | b'<')
            {
                self.gap(p);
                if self.push(Rule::FunctionDefinition) {
                    let open = if self.at(end) == b'(' {
                        S::Round
                    } else {
                        S::Angle
                    };
                    self.tok(p + 2, &[S::KwFn]);
                    self.tok(name, &[]);
                    self.tok(end, &[S::EntFunction]);
                    self.tok(end + 1, &[open]);
                    return Some(end + 1);
                }
            }
        }
        let end = self.ident_at(p)?;
        let call = self.at(end) == b'(';
        let turbofish = !call && self.starts_with(end, b"::<") && self.call_ahead(end + 3);
        if !call && !turbofish {
            return None;
        }
        self.gap(p);
        if !self.push(Rule::Call) {
            return None;
        }
        self.tok(end, &[S::EntFunction]);
        if call {
            self.tok(end + 1, &[S::Round]);
            return Some(end + 1);
        }
        Some(end)
    }

    // Types.

    fn types(&mut self, p: usize, b: u8) -> Option<usize> {
        if !is_ascii_word(b) {
            return None;
        }
        if matches!(b, b'f' | b'i' | b'u')
            && (p == 0 || !self.s[p - 1].is_ascii_alphabetic())
            && NUMERIC
                .iter()
                .any(|n| self.starts_with(p, n) && self.boundary(p + n.len()))
        {
            self.gap(p);
            self.tok(self.word_end, &[S::TypeNumeric]);
            return Some(self.word_end);
        }
        if !self.word_start(p) {
            return None;
        }
        let start = p + usize::from(b == b'_');
        if self.at(start).is_ascii_uppercase() {
            let end = self.skip(start, is_ascii_word);
            if self.at(end) == b'<' && self.boundary(end) {
                self.gap(p);
                if !self.push(Rule::Generic) {
                    return None;
                }
                self.tok(end, &[S::EntType]);
                self.tok(end + 1, &[S::Angle]);
                return Some(end + 1);
            }
        }
        let word = &self.s[p..self.word_end];
        if word.len() > 6 {
            return self.type_name(p, start);
        }
        if matches!(word, b"bool" | b"char" | b"str") {
            self.gap(p);
            self.tok(self.word_end, &[S::EntTypePrimitive]);
            return Some(self.word_end);
        }
        let declaration = match &self.s[p..self.word_end] {
            b"trait" => Some((S::KwDeclTrait, S::EntTypeTrait)),
            b"struct" => Some((S::KwDeclStruct, S::EntTypeStruct)),
            b"enum" => Some((S::KwDeclEnum, S::EntTypeEnum)),
            b"type" => Some((S::KwDeclType, S::EntTypeDecl)),
            _ => None,
        };
        if let Some((keyword, name_scope)) = declaration {
            let name = self.blanks(self.word_end);
            let n = name + usize::from(self.at(name) == b'_');
            if name > self.word_end && self.at(n).is_ascii_uppercase() {
                let end = self.skip(n, is_ascii_word);
                if self.boundary(end) {
                    self.gap(p);
                    self.tok(self.word_end, &[keyword, S::StorageType]);
                    self.tok(name, &[]);
                    self.tok(end, &[name_scope]);
                    return Some(end);
                }
            }
            return None;
        }
        self.type_name(p, start)
    }

    /// A capitalised name that is not a macro call.
    fn type_name(&mut self, p: usize, start: usize) -> Option<usize> {
        if self.at(start).is_ascii_uppercase() {
            let end = self.skip(start, is_ascii_word);
            if self.boundary(end) && self.at(end) != b'!' {
                self.gap(p);
                self.tok(end, &[S::EntType]);
                return Some(end);
            }
        }
        None
    }
}
