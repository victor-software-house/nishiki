//! Doc comments: the tags and inline tags of `JSDoc`.

use super::kind::Kind;
use super::lexer::{Lexer, is_ws};
use super::scope::S;

const TAG: S = S::StorageTypeClassJsdoc;
const AT: S = S::PunctuationDefinitionBlockTagJsdoc;
const INSTANCE: S = S::EntityNameTypeInstanceJsdoc;
const VARIABLE: S = S::VariableOtherJsdoc;

/// The tags the grammar scopes as a bare tag word, as the final catch-all of
/// its list spells them, with the optional suffixes written out.
const KNOWN_TAGS: &[&str] = &[
    "abstract",
    "access",
    "alias",
    "api",
    "arg",
    "argument",
    "async",
    "attribute",
    "augments",
    "author",
    "beta",
    "borrows",
    "bubbles",
    "callback",
    "chainable",
    "class",
    "classdesc",
    "code",
    "config",
    "const",
    "constant",
    "constructor",
    "constructs",
    "copyright",
    "default",
    "defaultvalue",
    "define",
    "deprecated",
    "desc",
    "description",
    "dict",
    "emits",
    "enum",
    "event",
    "example",
    "exception",
    "exports",
    "export",
    "extends",
    "extension",
    "extensionfor",
    "extension_for",
    "external",
    "externs",
    "file",
    "fileoverview",
    "final",
    "fires",
    "for",
    "func",
    "function",
    "generator",
    "global",
    "hideconstructor",
    "host",
    "ignore",
    "implements",
    "implicitCast",
    "inheritdoc",
    "inheritDoc",
    "inner",
    "instance",
    "interface",
    "internal",
    "kind",
    "lends",
    "license",
    "listens",
    "main",
    "member",
    "memberof",
    "method",
    "mixes",
    "mixins",
    "mixin",
    "modifies",
    "module",
    "name",
    "namespace",
    "noalias",
    "nocollapse",
    "nocompile",
    "nosideeffects",
    "override",
    "overview",
    "package",
    "param",
    "polymer",
    "polymerBehavior",
    "preserve",
    "private",
    "prop",
    "property",
    "protected",
    "public",
    "readonly",
    "readOnly",
    "record",
    "required",
    "requires",
    "returns",
    "return",
    "see",
    "since",
    "static",
    "struct",
    "submodule",
    "summary",
    "suppress",
    "template",
    "this",
    "throws",
    "todo",
    "tutorial",
    "type",
    "typedef",
    "unrestricted",
    "uses",
    "var",
    "variation",
    "version",
    "virtual",
    "writeOnce",
    "yields",
    "yield",
];

impl<const D: u8> Lexer<'_, D> {
    /// The rules of a doc comment: its tags and inline tags.
    pub(crate) fn docblock(&mut self, p: usize) -> Option<usize> {
        match self.b(p) {
            b'@' => self.doc_tag(p),
            b'[' => self.doc_bracket_link(p),
            b'{' => self.doc_inline_link(p),
            _ => None,
        }
    }

    /// `(?:[^S]|\*[^/])*` where `S` is `stop` and the star, run from `i`.
    fn doc_run(&self, i: usize, stop: impl Fn(u8) -> bool) -> usize {
        let mut j = i;
        while j <= self.len {
            let c = self.b(j);
            if c == b'*' {
                if self.b(j + 1) == b'/' {
                    break;
                }
                j += 2;
            } else if stop(c) {
                break;
            } else {
                j += self.char_len(j);
            }
        }
        j.min(self.len + 1)
    }

    /// `(?:[^*/@\s]|\*[^/])+`, as a run from `i` of at least one item.
    fn doc_word(&self, i: usize) -> Option<usize> {
        let e = self.doc_run(i, |c| matches!(c, b'/' | b'@') || is_ws(c));
        (e > i).then_some(e)
    }

    /// The `@tag` tokens: the `@` and the tag word, up to `end`.
    fn doc_tag_tokens(&mut self, p: usize, end: usize) {
        self.doc_tag_tokens_in(Kind::Top, p, end);
    }

    /// The `@tag` tokens of a rule whose context is `kind`.
    fn doc_tag_tokens_in(&mut self, kind: Kind, p: usize, end: usize) {
        self.flush(p);
        self.cap(kind, p, p + 1, &[TAG, AT]);
        self.cap(kind, p + 1, end, &[TAG]);
    }

    /// `[$A-Z_a-z][]$.\[\w]*`: a name that may hold brackets and dots.
    fn doc_name(&self, i: usize) -> Option<usize> {
        let c = self.b(i);
        if !(c == b'$' || c == b'_' || c.is_ascii_alphabetic()) {
            return None;
        }
        let mut j = i + 1;
        while j < self.len {
            let Some(c) = self.char_at(j) else { break };
            if matches!(c, ']' | '$' | '.' | '[' | '_') || c.is_alphanumeric() {
                j += c.len_utf8();
            } else {
                break;
            }
        }
        Some(j)
    }

    /// The ends of `@word` for each of `words` that the text starts with, in
    /// the order the grammar lists them.
    fn tag_ends(&self, p: usize, words: &[&str]) -> Vec<usize> {
        words
            .iter()
            .filter(|w| self.starts(p + 1, w))
            .map(|w| p + 1 + w.len())
            .collect()
    }

    fn doc_tag(&mut self, p: usize) -> Option<usize> {
        // @access and @api with a visibility.
        for e in self.tag_ends(p, &["access", "api"]) {
            let j = self.ws(e);
            if j > e {
                for word in ["private", "protected", "public"] {
                    if self.starts(j, word) && self.boundary(j + word.len()) {
                        self.doc_tag_tokens(p, e);
                        self.cap(
                            Kind::Top,
                            j,
                            j + word.len(),
                            &[S::ConstantLanguageAccessTypeJsdoc],
                        );
                        return Some(j + word.len());
                    }
                }
            }
        }
        if let Some(r) = self.doc_author(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_borrows(p) {
            return Some(r);
        }
        if let Some(e) = self.tag_ends(p, &["example"]).first().copied() {
            let j = self.ws(e);
            if j > e && !self.full() {
                self.doc_tag_tokens_in(Kind::JsExample, p, e);
                self.fin(Kind::JsExample, j);
                self.push(Kind::JsExample);
                return Some(j);
            }
        }
        if let Some(r) = self.doc_kind(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_see(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_template(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_param(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_typed_tags(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_named_tags(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_default(p) {
            return Some(r);
        }
        if let Some(r) = self.doc_known_tag(p) {
            return Some(r);
        }
        // `@name` followed by blanks.
        let e = self.ident(p + 1)?;
        (is_ws(self.b(e))).then(|| {
            self.doc_tag_tokens(p, e);
            Some(e)
        })?
    }

    fn doc_author(&mut self, p: usize) -> Option<usize> {
        let e = *self.tag_ends(p, &["author"]).first()?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let c = self.b(j);
        if matches!(c, b'*' | b'/' | b'<' | b'>' | b'@') || is_ws(c) {
            return None;
        }
        let name_end = self.doc_run(j + self.char_len(j), |c| {
            matches!(c, b'/' | b'<' | b'>' | b'@')
        });
        self.doc_tag_tokens(p, e);
        self.cap(Kind::Top, j, name_end.min(self.len), &[INSTANCE]);
        let k = self.ws(name_end);
        if self.b(k) == b'<' {
            let mut m = k + 1;
            while m < self.len && self.b(m) != b'>' && !is_ws(self.b(m)) {
                m += 1;
            }
            if m > k + 1 && self.b(m) == b'>' {
                self.cap(
                    Kind::Top,
                    k,
                    k + 1,
                    &[S::PunctuationDefinitionBracketAngleBeginJsdoc],
                );
                self.cap(
                    Kind::Top,
                    k + 1,
                    m,
                    &[S::ConstantOtherEmailLinkUnderlineJsdoc],
                );
                self.cap(
                    Kind::Top,
                    m,
                    m + 1,
                    &[S::PunctuationDefinitionBracketAngleEndJsdoc],
                );
                return Some(m + 1);
            }
        }
        Some(name_end.min(self.len))
    }

    fn doc_borrows(&mut self, p: usize) -> Option<usize> {
        let e = *self.tag_ends(p, &["borrows"]).first()?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let a = self.doc_word(j)?;
        let k = self.ws(a);
        if k == a || !self.starts(k, "as") {
            return None;
        }
        let m = self.ws(k + 2);
        if m == k + 2 {
            return None;
        }
        let z = self.doc_word(m)?;
        self.doc_tag_tokens(p, e);
        self.cap(Kind::Top, j, a, &[INSTANCE]);
        self.cap(Kind::Top, k, k + 2, &[S::KeywordOperatorControlJsdoc]);
        self.cap(Kind::Top, m, z, &[INSTANCE]);
        Some(z)
    }

    fn doc_kind(&mut self, p: usize) -> Option<usize> {
        let e = *self.tag_ends(p, &["kind"]).first()?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        for word in [
            "class",
            "constant",
            "event",
            "external",
            "file",
            "function",
            "member",
            "mixin",
            "module",
            "namespace",
            "typedef",
        ] {
            if self.starts(j, word) && self.boundary(j + word.len()) {
                self.doc_tag_tokens(p, e);
                self.cap(
                    Kind::Top,
                    j,
                    j + word.len(),
                    &[S::ConstantLanguageSymbolTypeJsdoc],
                );
                return Some(j + word.len());
            }
        }
        None
    }

    fn doc_see(&mut self, p: usize) -> Option<usize> {
        let e = *self.tag_ends(p, &["see"]).first()?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        if self.starts(j, "http://") || self.starts(j, "https://") {
            let z = self.doc_run(j, is_ws);
            if z > j {
                self.doc_tag_tokens(p, e);
                self.cap(Kind::Top, j, z, &[S::VariableOtherLinkUnderlineJsdoc]);
                return Some(z);
            }
        }
        if !self.starts(j, "http://")
            && !self.starts(j, "https://")
            && !self.inline_link_ahead(j)
            && let Some(z) = self.doc_word(j)
        {
            self.doc_tag_tokens(p, e);
            self.cap(Kind::Top, j, z, &[INSTANCE]);
            return Some(z);
        }
        None
    }

    /// `(?:\[[^]\[]*])?\{@(?:link|linkcode|linkplain|tutorial)\b`.
    fn inline_link_ahead(&self, i: usize) -> bool {
        let mut j = i;
        if self.b(j) == b'[' {
            let mut k = j + 1;
            while k < self.len && !matches!(self.b(k), b']' | b'[') {
                k += 1;
            }
            if self.b(k) == b']' {
                j = k + 1;
            }
        }
        self.starts(j, "{@") && self.inline_tag_name(j + 2).is_some()
    }

    fn inline_tag_name(&self, i: usize) -> Option<usize> {
        for word in ["linkcode", "linkplain", "link", "tutorial"] {
            if self.starts(i, word) && self.boundary(i + word.len()) {
                return Some(i + word.len());
            }
        }
        None
    }

    fn doc_template(&mut self, p: usize) -> Option<usize> {
        let e = *self.tag_ends(p, &["template"]).first()?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        if let Some(n) = self.doc_name(j) {
            // `X, Y, Z`
            let mut end = n;
            loop {
                let k = self.ws(end);
                if self.b(k) != b',' {
                    break;
                }
                let m = self.ws(k + 1);
                match self.doc_name(m) {
                    Some(z) => end = z,
                    None => break,
                }
            }
            self.doc_tag_tokens(p, e);
            self.cap(Kind::Top, j, end, &[VARIABLE]);
            return Some(end);
        }
        if self.b(j) == b'{' && !self.full() {
            self.doc_tag_tokens(p, e);
            self.fin(Kind::Top, j);
            self.push(Kind::JsTypeTag);
            return Some(j);
        }
        None
    }

    fn doc_param(&mut self, p: usize) -> Option<usize> {
        let ends = self.tag_ends(
            p,
            &[
                "arg",
                "argument",
                "const",
                "constant",
                "member",
                "namespace",
                "param",
                "var",
            ],
        );
        for e in ends {
            let j = self.ws(e);
            if j > e
                && let Some(n) = self.doc_name(j)
            {
                self.doc_tag_tokens(p, e);
                self.cap(Kind::Top, j, n, &[VARIABLE]);
                return Some(n);
            }
        }
        None
    }

    /// Tags followed by `{type}`: they open a context that ends after it.
    fn doc_typed_tags(&mut self, p: usize) -> Option<usize> {
        let sets: [&[&str]; 3] = [
            &["typedef"],
            &[
                "arg",
                "argument",
                "const",
                "constant",
                "member",
                "namespace",
                "param",
                "prop",
                "property",
                "var",
            ],
            &[
                "define",
                "enum",
                "exception",
                "export",
                "extends",
                "lends",
                "implements",
                "modifies",
                "namespace",
                "private",
                "protected",
                "returns",
                "return",
                "satisfies",
                "suppress",
                "this",
                "throws",
                "type",
                "yields",
                "yield",
            ],
        ];
        let kinds = [Kind::JsTypedefTag, Kind::JsParamTag, Kind::JsRetTag];
        for (words, kind) in sets.into_iter().zip(kinds) {
            for e in self.tag_ends(p, words) {
                let j = self.ws(e);
                if j > e && self.b(j) == b'{' && !self.full() {
                    self.doc_tag_tokens(p, e);
                    self.fin(Kind::Top, j);
                    self.push(kind);
                    return Some(j);
                }
            }
        }
        None
    }

    fn doc_named_tags(&mut self, p: usize) -> Option<usize> {
        let ends = self.tag_ends(
            p,
            &[
                "alias",
                "augments",
                "callback",
                "constructs",
                "emits",
                "event",
                "fires",
                "exports",
                "export",
                "extends",
                "external",
                "function",
                "func",
                "host",
                "lends",
                "listens",
                "interface",
                "memberof",
                "method",
                "module",
                "mixes",
                "mixin",
                "name",
                "requires",
                "see",
                "this",
                "typedef",
                "uses",
            ],
        );
        for mut e in ends {
            if self.starts(e, "!") && self.starts(p + 1, "memberof") {
                e += 1;
            }
            let j = self.ws(e);
            if j > e {
                let z = self.doc_run(j, |c| matches!(c, b'@' | b'{' | b'}') || is_ws(c));
                if z > j {
                    self.doc_tag_tokens(p, e);
                    self.cap(Kind::Top, j, z, &[INSTANCE]);
                    return Some(z);
                }
            }
        }
        None
    }

    fn doc_default(&mut self, p: usize) -> Option<usize> {
        for e in self.tag_ends(p, &["defaultvalue", "default", "license", "version"]) {
            let j = self.ws(e);
            if j == e {
                continue;
            }
            let q = self.b(j);
            if (q == b'"' || q == b'\'') && !self.full() {
                self.doc_tag_tokens(p, e);
                self.cap(
                    Kind::Top,
                    j,
                    j + 1,
                    &[VARIABLE, S::PunctuationDefinitionStringBeginJsdoc],
                );
                self.push_with(Kind::JsDefaultQuote, u32::from(q));
                return Some(j + 1);
            }
        }
        for e in self.tag_ends(
            p,
            &[
                "defaultvalue",
                "default",
                "license",
                "tutorial",
                "variation",
                "version",
            ],
        ) {
            let j = self.ws(e);
            if j == e {
                continue;
            }
            let mut z = j;
            while z <= self.len && self.b(z) != b'*' && !is_ws(self.b(z)) {
                z += self.char_len(z);
            }
            if z > j {
                self.doc_tag_tokens(p, e);
                self.cap(Kind::Top, j, z, &[VARIABLE]);
                return Some(z);
            }
        }
        None
    }

    fn doc_known_tag(&mut self, p: usize) -> Option<usize> {
        let word_end = {
            let mut e = p + 1;
            while self.b(e) == b'_' || self.b(e).is_ascii_alphanumeric() {
                e += 1;
            }
            e
        };
        let word = self.text(p + 1, word_end);
        if !KNOWN_TAGS.contains(&word) {
            return None;
        }
        self.one(p, p + 1, &[TAG, AT]);
        self.cap(Kind::Top, p + 1, word_end, &[TAG]);
        Some(word_end)
    }

    /// `(\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))`.
    fn doc_bracket_link(&mut self, p: usize) -> Option<usize> {
        let mut e = p + 1;
        while e < self.len && self.b(e) != b']' {
            e += 1;
        }
        if e == p + 1 || self.b(e) != b']' {
            return None;
        }
        let after = e + 1;
        if !(self.starts(after, "{@") && self.inline_tag_name(after + 2).is_some()) {
            return None;
        }
        let d = S::ConstantOtherDescriptionJsdoc;
        self.one(
            p,
            p + 1,
            &[d, S::PunctuationDefinitionBracketSquareBeginJsdoc],
        );
        self.cap(Kind::Top, p + 1, e, &[d]);
        self.cap(
            Kind::Top,
            e,
            after,
            &[d, S::PunctuationDefinitionBracketSquareEndJsdoc],
        );
        Some(after)
    }

    /// `{@link …}`.
    fn doc_inline_link(&mut self, p: usize) -> Option<usize> {
        if self.b(p + 1) != b'@' || self.full() {
            return None;
        }
        let name_end = self.inline_tag_name(p + 2)?;
        let e = self.ws(name_end);
        self.flush(p);
        self.cap(
            Kind::JsInlineLink,
            p,
            p + 1,
            &[S::PunctuationDefinitionBracketCurlyBeginJsdoc],
        );
        self.cap(
            Kind::JsInlineLink,
            p + 1,
            p + 2,
            &[TAG, S::PunctuationDefinitionInlineTagJsdoc],
        );
        self.cap(Kind::JsInlineLink, p + 2, name_end, &[TAG]);
        self.fin(Kind::JsInlineLink, e);
        self.push(Kind::JsInlineLink);
        Some(e)
    }

    /// Inside `{@link …}`: the link or description just after the tag.
    pub(crate) fn doc_inline_link_rules(&mut self, p: usize) -> Option<usize> {
        if p != self.anchor {
            return None;
        }
        if self.starts(p, "http://") || self.starts(p, "https://") {
            let mut e = p;
            while e <= self.len {
                let c = self.b(e);
                if self.starts(e, "*/") {
                    e += 2;
                } else if c == b'*' || c == b'|' || c == b'}' || is_ws(c) {
                    break;
                } else {
                    e += self.char_len(e);
                }
            }
            if e > p {
                return Some(self.doc_link_part(p, e, S::VariableOtherLinkUnderlineJsdoc));
            }
        }
        let e = self.doc_run(p, |c| matches!(c, b'@' | b'{' | b'|' | b'}') || is_ws(c));
        if e > p {
            return Some(self.doc_link_part(p, e, S::VariableOtherDescriptionJsdoc));
        }
        None
    }

    fn doc_link_part(&mut self, p: usize, e: usize, scope: S) -> usize {
        self.flush(p);
        self.cap(Kind::Top, p, e.min(self.len), &[scope]);
        if self.b(e) == b'|' {
            self.cap(Kind::Top, e, e + 1, &[S::PunctuationSeparatorPipeJsdoc]);
            return e + 1;
        }
        e
    }

    /// Inside `@example`: the line prefix, a caption and the code.
    pub(crate) fn doc_example_rules(&mut self, p: usize) -> Option<usize> {
        if self.bol(p) && is_ws(self.b(0)) {
            let j = 1;
            if self.b(j) == b'*' {
                let e = self.ws(j + 1);
                if e > j + 1 {
                    return Some(e);
                }
            }
        }
        if p == self.anchor && self.b(p) == b'<' && self.starts(p + 1, "caption>") && !self.full() {
            let e = p + 9;
            self.flush(p);
            let tag = S::EntityNameTagInlineJsdoc;
            self.cap(
                Kind::Top,
                p,
                p + 1,
                &[tag, S::PunctuationDefinitionBracketAngleBeginJsdoc],
            );
            self.cap(Kind::Top, p + 1, e - 1, &[tag]);
            self.cap(
                Kind::Top,
                e - 1,
                e,
                &[tag, S::PunctuationDefinitionBracketAngleEndJsdoc],
            );
            self.push(Kind::JsCaption);
            return Some(e);
        }
        let c = self.b(p);
        if !(matches!(c, b'*' | b'@') || is_ws(c)) && p <= self.len {
            let e = self.doc_run_to_star(p);
            self.flush(p);
            self.cap(Kind::Top, p, e, &[S::SourceEmbedded]);
            return Some(e);
        }
        None
    }

    /// `(?:[^*]|\*[^/])*`.
    fn doc_run_to_star(&self, i: usize) -> usize {
        let mut j = i;
        while j <= self.len {
            if self.b(j) == b'*' {
                if self.b(j + 1) == b'/' {
                    break;
                }
                j += 2;
            } else {
                j += self.char_len(j);
            }
        }
        j.min(self.len)
    }

    /// Inside a `{type}` of a tag: brackets nest.
    pub(crate) fn doc_type_tag_rules(&mut self, p: usize) -> Option<usize> {
        if self.b(p) == b'{' && p == self.anchor && !self.full() {
            self.flush(p);
            self.cap(
                Kind::Top,
                p,
                p + 1,
                &[INSTANCE, S::PunctuationDefinitionBracketCurlyBeginJsdoc],
            );
            self.push(Kind::JsType);
            return Some(p + 1);
        }
        None
    }

    /// The names inside tags that take a type and then a name.
    pub(crate) fn doc_type_tag_names(&mut self, p: usize) -> Option<usize> {
        let n = self.doc_name(p)?;
        self.tok(p, n, VARIABLE)
    }

    /// The `{`…`}` and `[`…`]` groups inside a doc type.
    pub(crate) fn doc_brackets(&mut self, p: usize) -> Option<usize> {
        match self.b(p) {
            b'{' => self.open1(Kind::JsBraces, p, p + 1, &[]),
            b'[' => self.open1(Kind::JsBrackets, p, p + 1, &[]),
            _ => None,
        }
    }
}
