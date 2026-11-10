//! Doc comments: the tags and inline tags of `JSDoc`.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::classes;

const TAG: S = S::StorageTypeClassJsdoc;
const AT: S = S::PunctuationDefinitionBlockTagJsdoc;
const INSTANCE: S = S::EntityNameTypeInstanceJsdoc;
const VARIABLE: S = S::VariableOtherJsdoc;

/// The tags the grammar scopes as a bare tag word, as the final catch-all of
/// its list spells them.
/// The optional suffixes are written out.
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
        match self.byte(p) {
            b'@' => self.doc_tag(p),
            b'[' => self.doc_bracket_link(p),
            b'{' => self.doc_inline_link(p),
            _ => None,
        }
    }

    /// `(?:[^S]|\*[^/])*` where `S` is the star and what `stop` holds at a
    /// position, run from `i`.
    fn doc_run(&self, i: usize, stop: impl Fn(usize) -> bool) -> usize {
        let mut j = i;
        while j <= self.len {
            let c = self.byte(j);
            if c == b'*' {
                if self.byte(j + 1) == b'/' {
                    break;
                }
                j += 2;
            } else if stop(j) {
                break;
            } else {
                j += self.char_len(j);
            }
        }
        j.min(self.window())
    }

    /// `(?:[^*/@\s]|\*[^/])+`, as a run from `i` of at least one item.
    fn doc_word(&self, i: usize) -> Option<usize> {
        let e = self.doc_run(i, |j| {
            matches!(self.byte(j), b'/' | b'@') || self.space_at(j)
        });
        (e > i).then_some(e)
    }

    /// The `@tag` tokens: the `@` and the tag word, up to `end`.
    fn doc_tag_tokens(&mut self, p: usize, end: usize) {
        self.doc_tag_tokens_in(Rule::Top, p, end);
    }

    /// The `@tag` tokens of a tag rule inside `rule`.
    fn doc_tag_tokens_in(&mut self, rule: Rule, p: usize, end: usize) {
        self.flush(p);
        self.capture(rule, p, p + 1, &[TAG, AT]);
        self.capture(rule, p + 1, end, &[TAG]);
    }

    /// `[$A-Z_a-z][]$.\[\w]*`: a name that may hold brackets and dots.
    fn doc_name(&self, i: usize) -> Option<usize> {
        let c = self.byte(i);
        if !(c == b'$' || c == b'_' || c.is_ascii_alphabetic()) {
            return None;
        }
        let mut j = i + 1;
        while j < self.len {
            let Some(c) = self.char_at(j) else { break };
            if matches!(c, ']' | '$' | '.' | '[') || classes::is_word(c) {
                j += c.len_utf8();
            } else {
                break;
            }
        }
        self.spend(j - i);
        Some(j)
    }

    /// The end of `@word` at `p`, if the text there starts with it.
    /// Callers try a tag's words in the order the grammar lists them.
    fn tag_end(&self, p: usize, word: &str) -> Option<usize> {
        self.starts(p + 1, word).then(|| p + 1 + word.len())
    }

    /// A block tag in a doc comment: `@access` and `@api` with a visibility,
    /// the other known tags, and `@name` followed by blanks.
    fn doc_tag(&mut self, p: usize) -> Option<usize> {
        for word in ["access", "api"] {
            let Some(e) = self.tag_end(p, word) else {
                continue;
            };
            let j = self.ws(e);
            if j > e {
                for word in ["private", "protected", "public"] {
                    if self.starts(j, word) && self.boundary(j + word.len()) {
                        self.doc_tag_tokens(p, e);
                        self.capture(
                            Rule::Top,
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
        if let Some(e) = self.tag_end(p, "example") {
            let j = self.ws(e);
            if j > e && !self.full() {
                self.doc_tag_tokens_in(Rule::JsExample, p, e);
                self.finish(Rule::JsExample, j);
                self.push(Rule::JsExample);
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
        let e = self.ident(p + 1)?;
        self.space_at(e).then(|| {
            self.doc_tag_tokens(p, e);
            Some(e)
        })?
    }

    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn doc_author(&mut self, p: usize) -> Option<usize> {
        let e = self.tag_end(p, "author")?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        if matches!(self.byte(j), b'*' | b'/' | b'<' | b'>' | b'@') || self.space_at(j) {
            return None;
        }
        let name_end = self.doc_run(j + self.char_len(j), |k| {
            matches!(self.byte(k), b'/' | b'<' | b'>' | b'@')
        });
        self.doc_tag_tokens(p, e);
        self.capture(Rule::Top, j, name_end.min(self.len), &[INSTANCE]);
        let k = self.ws(name_end);
        if self.byte(k) == b'<' {
            let mut m = k + 1;
            while m < self.len && self.byte(m) != b'>' && !self.space_at(m) {
                m += self.char_len(m);
            }
            self.spend(m - k);
            if m > k + 1 && self.byte(m) == b'>' {
                self.capture(
                    Rule::Top,
                    k,
                    k + 1,
                    &[S::PunctuationDefinitionBracketAngleBeginJsdoc],
                );
                self.capture(
                    Rule::Top,
                    k + 1,
                    m,
                    &[S::ConstantOtherEmailLinkUnderlineJsdoc],
                );
                self.capture(
                    Rule::Top,
                    m,
                    m + 1,
                    &[S::PunctuationDefinitionBracketAngleEndJsdoc],
                );
                return Some(m + 1);
            }
        }
        Some(name_end.min(self.len))
    }

    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn doc_borrows(&mut self, p: usize) -> Option<usize> {
        let e = self.tag_end(p, "borrows")?;
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
        self.capture(Rule::Top, j, a, &[INSTANCE]);
        self.capture(Rule::Top, k, k + 2, &[S::KeywordOperatorControlJsdoc]);
        self.capture(Rule::Top, m, z, &[INSTANCE]);
        Some(z)
    }

    fn doc_kind(&mut self, p: usize) -> Option<usize> {
        let e = self.tag_end(p, "kind")?;
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
                self.capture(
                    Rule::Top,
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
        let e = self.tag_end(p, "see")?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        if self.starts(j, "http://") || self.starts(j, "https://") {
            let z = self.doc_run(j, |k| self.space_at(k));
            if z > j {
                self.doc_tag_tokens(p, e);
                self.capture(Rule::Top, j, z, &[S::VariableOtherLinkUnderlineJsdoc]);
                return Some(z);
            }
        }
        if !self.starts(j, "http://")
            && !self.starts(j, "https://")
            && !self.inline_link_ahead(j)
            && let Some(z) = self.doc_word(j)
        {
            self.doc_tag_tokens(p, e);
            self.capture(Rule::Top, j, z, &[INSTANCE]);
            return Some(z);
        }
        None
    }

    /// `(?:\[[^]\[]*])?\{@(?:link|linkcode|linkplain|tutorial)\b`.
    fn inline_link_ahead(&self, i: usize) -> bool {
        let mut j = i;
        if self.byte(j) == b'[' {
            let mut k = j + 1;
            while k < self.len && !matches!(self.byte(k), b']' | b'[') {
                k += 1;
            }
            self.spend(k - j);
            if self.byte(k) == b']' {
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

    /// `@template` and its names, `X, Y, Z`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn doc_template(&mut self, p: usize) -> Option<usize> {
        let e = self.tag_end(p, "template")?;
        let j = self.ws(e);
        if j == e {
            return None;
        }
        if let Some(n) = self.doc_name(j) {
            let mut end = n;
            loop {
                let k = self.ws(end);
                if self.byte(k) != b',' {
                    break;
                }
                let m = self.ws(k + 1);
                match self.doc_name(m) {
                    Some(z) => end = z,
                    None => break,
                }
            }
            self.doc_tag_tokens(p, e);
            self.capture(Rule::Top, j, end, &[VARIABLE]);
            return Some(end);
        }
        if self.byte(j) == b'{' && !self.full() {
            self.doc_tag_tokens(p, e);
            self.finish(Rule::Top, j);
            self.push(Rule::JsTypeTag);
            return Some(j);
        }
        None
    }

    fn doc_param(&mut self, p: usize) -> Option<usize> {
        let words = [
            "arg",
            "argument",
            "const",
            "constant",
            "member",
            "namespace",
            "param",
            "var",
        ];
        for word in words {
            let Some(e) = self.tag_end(p, word) else {
                continue;
            };
            let j = self.ws(e);
            if j > e
                && let Some(n) = self.doc_name(j)
            {
                self.doc_tag_tokens(p, e);
                self.capture(Rule::Top, j, n, &[VARIABLE]);
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
        let rules = [Rule::JsTypedefTag, Rule::JsParamTag, Rule::JsRetTag];
        for (words, rule) in sets.into_iter().zip(rules) {
            for word in words {
                let Some(e) = self.tag_end(p, word) else {
                    continue;
                };
                let j = self.ws(e);
                if j > e && self.byte(j) == b'{' && !self.full() {
                    self.doc_tag_tokens(p, e);
                    self.finish(Rule::Top, j);
                    self.push(rule);
                    return Some(j);
                }
            }
        }
        None
    }

    fn doc_named_tags(&mut self, p: usize) -> Option<usize> {
        let words = [
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
        ];
        for word in words {
            let Some(mut e) = self.tag_end(p, word) else {
                continue;
            };
            if self.starts(e, "!") && self.starts(p + 1, "memberof") {
                e += 1;
            }
            let j = self.ws(e);
            if j > e {
                let z = self.doc_run(j, |k| {
                    matches!(self.byte(k), b'@' | b'{' | b'}') || self.space_at(k)
                });
                if z > j {
                    self.doc_tag_tokens(p, e);
                    self.capture(Rule::Top, j, z, &[INSTANCE]);
                    return Some(z);
                }
            }
        }
        None
    }

    fn doc_default(&mut self, p: usize) -> Option<usize> {
        for word in ["defaultvalue", "default", "license", "version"] {
            let Some(e) = self.tag_end(p, word) else {
                continue;
            };
            let j = self.ws(e);
            if j == e {
                continue;
            }
            let q = self.byte(j);
            if (q == b'"' || q == b'\'') && !self.full() {
                self.doc_tag_tokens(p, e);
                self.capture(
                    Rule::Top,
                    j,
                    j + 1,
                    &[VARIABLE, S::PunctuationDefinitionStringBeginJsdoc],
                );
                self.push_with(Rule::JsDefaultQuote, u32::from(q));
                return Some(j + 1);
            }
        }
        for word in [
            "defaultvalue",
            "default",
            "license",
            "tutorial",
            "variation",
            "version",
        ] {
            let Some(e) = self.tag_end(p, word) else {
                continue;
            };
            let j = self.ws(e);
            if j == e {
                continue;
            }
            let mut z = j;
            while z <= self.len && self.byte(z) != b'*' && !self.space_at(z) {
                z += self.char_len(z);
            }
            self.spend(z - j);
            if z > j {
                self.doc_tag_tokens(p, e);
                self.capture(Rule::Top, j, z, &[VARIABLE]);
                return Some(z);
            }
        }
        None
    }

    fn doc_known_tag(&mut self, p: usize) -> Option<usize> {
        let word_end = {
            let mut e = p + 1;
            while self.byte(e) == b'_' || self.byte(e).is_ascii_alphanumeric() {
                e += 1;
            }
            self.spend(e - p);
            e
        };
        let word = self.text(p + 1, word_end);
        if !KNOWN_TAGS.contains(&word) || !self.boundary(word_end) {
            return None;
        }
        self.token(p, p + 1, &[TAG, AT]);
        self.capture(Rule::Top, p + 1, word_end, &[TAG]);
        Some(word_end)
    }

    /// `(\[)[^]]+(])(?=\{@(?:link|linkcode|linkplain|tutorial))`.
    fn doc_bracket_link(&mut self, p: usize) -> Option<usize> {
        let e = self.bracket_from(p + 1)?;
        if e == p + 1 {
            return None;
        }
        let after = e + 1;
        if !(self.starts(after, "{@") && self.inline_tag_name(after + 2).is_some()) {
            return None;
        }
        let d = S::ConstantOtherDescriptionJsdoc;
        self.token(
            p,
            p + 1,
            &[d, S::PunctuationDefinitionBracketSquareBeginJsdoc],
        );
        self.capture(Rule::Top, p + 1, e, &[d]);
        self.capture(
            Rule::Top,
            e,
            after,
            &[d, S::PunctuationDefinitionBracketSquareEndJsdoc],
        );
        Some(after)
    }

    /// `{@link …}`.
    fn doc_inline_link(&mut self, p: usize) -> Option<usize> {
        if self.byte(p + 1) != b'@' || self.full() {
            return None;
        }
        let name_end = self.inline_tag_name(p + 2)?;
        let e = self.ws(name_end);
        self.flush(p);
        self.capture(
            Rule::JsInlineLink,
            p,
            p + 1,
            &[S::PunctuationDefinitionBracketCurlyBeginJsdoc],
        );
        self.capture(
            Rule::JsInlineLink,
            p + 1,
            p + 2,
            &[TAG, S::PunctuationDefinitionInlineTagJsdoc],
        );
        self.capture(Rule::JsInlineLink, p + 2, name_end, &[TAG]);
        self.finish(Rule::JsInlineLink, e);
        self.push(Rule::JsInlineLink);
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
                let c = self.byte(e);
                if self.starts(e, "*/") {
                    e += 2;
                } else if c == b'*' || c == b'|' || c == b'}' || self.space_at(e) {
                    break;
                } else {
                    e += self.char_len(e);
                }
            }
            if e > p {
                return Some(self.doc_link_part(p, e, S::VariableOtherLinkUnderlineJsdoc));
            }
        }
        let e = self.doc_run(p, |k| {
            matches!(self.byte(k), b'@' | b'{' | b'|' | b'}') || self.space_at(k)
        });
        if e > p {
            return Some(self.doc_link_part(p, e, S::VariableOtherDescriptionJsdoc));
        }
        None
    }

    fn doc_link_part(&mut self, p: usize, e: usize, scope: S) -> usize {
        self.flush(p);
        self.capture(Rule::Top, p, e.min(self.len), &[scope]);
        if self.byte(e) == b'|' {
            self.capture(Rule::Top, e, e + 1, &[S::PunctuationSeparatorPipeJsdoc]);
            return e + 1;
        }
        e
    }

    /// Inside `@example`: the line prefix, a caption and the code.
    pub(crate) fn doc_example_rules(&mut self, p: usize) -> Option<usize> {
        if Self::bol(p) && self.space_at(0) {
            let j = 1;
            if self.byte(j) == b'*' {
                let e = self.ws(j + 1);
                if e > j + 1 {
                    return Some(e);
                }
            }
        }
        if p == self.anchor
            && self.byte(p) == b'<'
            && self.starts(p + 1, "caption>")
            && !self.full()
        {
            let e = p + 9;
            self.flush(p);
            let tag = S::EntityNameTagInlineJsdoc;
            self.capture(
                Rule::Top,
                p,
                p + 1,
                &[tag, S::PunctuationDefinitionBracketAngleBeginJsdoc],
            );
            self.capture(Rule::Top, p + 1, e - 1, &[tag]);
            self.capture(
                Rule::Top,
                e - 1,
                e,
                &[tag, S::PunctuationDefinitionBracketAngleEndJsdoc],
            );
            self.push(Rule::JsCaption);
            return Some(e);
        }
        if !(matches!(self.byte(p), b'*' | b'@') || self.space_at(p)) && p <= self.len {
            let e = self.doc_run_to_star(p);
            self.flush(p);
            self.capture(Rule::Top, p, e, &[S::SourceEmbedded]);
            return Some(e);
        }
        None
    }

    /// `(?:[^*]|\*[^/])*`.
    fn doc_run_to_star(&self, i: usize) -> usize {
        let mut j = i;
        while j <= self.len {
            if self.byte(j) == b'*' {
                if self.byte(j + 1) == b'/' {
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
        if self.byte(p) == b'{' && p == self.anchor && !self.full() {
            self.flush(p);
            self.capture(
                Rule::Top,
                p,
                p + 1,
                &[INSTANCE, S::PunctuationDefinitionBracketCurlyBeginJsdoc],
            );
            self.push(Rule::JsType);
            return Some(p + 1);
        }
        None
    }

    /// The names inside tags that take a type and then a name.
    pub(crate) fn doc_type_tag_names(&mut self, p: usize) -> Option<usize> {
        let n = self.doc_name(p)?;
        self.token_rule(p, n, VARIABLE)
    }

    /// The `{`…`}` and `[`…`]` groups inside a doc type.
    pub(crate) fn doc_brackets(&mut self, p: usize) -> Option<usize> {
        match self.byte(p) {
            b'{' => self.open_token(Rule::JsBraces, p, p + 1, &[]),
            b'[' => self.open_token(Rule::JsBrackets, p, p + 1, &[]),
            _ => None,
        }
    }
}
