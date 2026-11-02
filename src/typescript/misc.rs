//! Directives, doc-comment parameters and template tags.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

/// The pieces of an optional parameter, `[name=default]`.
struct OptionalParam {
    /// The `=`, the start of the default and its end.
    default: Option<(usize, usize, usize)>,
    /// The closing bracket, when there is one.
    close: Option<usize>,
    /// Where the match ends.
    end: usize,
}

impl Lexer<'_> {
    /// `^(///)\s*(?=<(reference|amd-dependency|amd-module)(…)+\s*/>\s*$)`.
    pub(crate) fn directive(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.starts(0, "///") || self.full() {
            return None;
        }
        let j = self.ws(3);
        if !self.directive_ahead(j) {
            return None;
        }
        self.flush(0);
        self.cap(Kind::Directive, 0, 3, &[S::PunctuationDefinitionComment]);
        self.fin(Kind::Directive, j);
        self.push(Kind::Directive);
        Some(j)
    }

    /// `<name( attr = "value")+\s*/>\s*$`.
    fn directive_ahead(&self, i: usize) -> bool {
        if self.b(i) != b'<' {
            return false;
        }
        let name = ["reference", "amd-dependency", "amd-module"]
            .iter()
            .find(|w| self.starts(i + 1, w));
        let Some(name) = name else {
            return false;
        };
        let mut j = i + 1 + name.len();
        let mut attrs = 0;
        loop {
            let k = self.ws(j);
            if k == j {
                break;
            }
            let Some(a) = self.directive_attribute(k) else {
                break;
            };
            let eq = self.ws(a);
            if self.b(eq) != b'=' {
                break;
            }
            let v = self.ws(eq + 1);
            let Some(e) = self.quoted(v) else {
                break;
            };
            j = e;
            attrs += 1;
        }
        if attrs == 0 {
            return false;
        }
        let k = self.ws(j);
        self.starts(k, "/>") && self.eol(self.ws(k + 2))
    }

    fn directive_attribute(&self, i: usize) -> Option<usize> {
        [
            "path",
            "types",
            "no-default-lib",
            "lib",
            "name",
            "resolution-mode",
        ]
        .iter()
        .find(|w| self.starts(i, w))
        .map(|w| i + w.len())
    }

    pub(crate) fn directive_rules(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'<' || self.full() {
            return None;
        }
        let name = ["reference", "amd-dependency", "amd-module"]
            .iter()
            .find(|w| self.starts(p + 1, w))?;
        let e = p + 1 + name.len();
        self.flush(p);
        self.cap(
            Kind::DirectiveTag,
            p,
            p + 1,
            &[S::PunctuationDefinitionTagDirective],
        );
        self.cap(Kind::DirectiveTag, p + 1, e, &[S::EntityNameTagDirective]);
        self.push(Kind::DirectiveTag);
        Some(e)
    }

    pub(crate) fn directive_tag_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.directive_attribute(p) {
            return self.tok(p, e, S::EntityOtherAttributeNameDirective);
        }
        if self.b(p) == b'=' {
            return self.tok(p, p + 1, S::KeywordOperatorAssignment);
        }
        self.string(p)
    }

    /// `(?:[^*/@\s]|\*[^/])+` as the name of a `@typedef`.
    pub(crate) fn doc_typedef_name(&mut self, p: usize) -> Option<usize> {
        let mut j = p;
        while j <= self.len {
            let c = self.b(j);
            if c == b'*' {
                if self.b(j + 1) == b'/' {
                    break;
                }
                j += 2;
            } else if matches!(c, b'/' | b'@') || super::lexer::is_ws(c) {
                break;
            } else {
                j += self.char_len(j);
            }
        }
        if j == p {
            return None;
        }
        self.one(p, j.min(self.len), &[S::EntityNameTypeInstanceJsdoc]);
        Some(j)
    }

    /// `[name=default]`, an optional parameter in a doc comment, checked in
    /// full before anything is emitted: the grammar's pattern has to match to
    /// the closing bracket or to the end of the comment.
    fn optional_param_ahead(&self, p: usize) -> Option<OptionalParam> {
        let name_char = |b: u8| b == b'$' || b == b'_' || b.is_ascii_alphanumeric();
        let mut j = self.ws(p + 1);
        let name_start = j;
        while name_char(self.b(j)) {
            j += 1;
        }
        if j == name_start {
            return None;
        }
        // `(?:(?:\[])?\.[$\w]+)*`
        loop {
            let k = if self.starts(j, "[]") { j + 2 } else { j };
            if self.b(k) != b'.' {
                break;
            }
            let mut e = k + 1;
            while name_char(self.b(e)) {
                e += 1;
            }
            if e == k + 1 {
                break;
            }
            j = e;
        }
        let mut end = j;
        let mut default = None;
        let k = self.ws(j);
        if self.b(k) == b'=' {
            let value = self.ws(k + 1);
            let mut e = value;
            let mut depth = 0usize;
            while e < self.len && !self.starts(e, "*/") {
                match self.b(e) {
                    b'[' => depth += 1,
                    b']' if depth == 0 => break,
                    b']' => depth -= 1,
                    b'"' | b'\'' => {
                        if let Some(q) = self.quoted(e) {
                            e = q;
                            continue;
                        }
                    }
                    _ => {}
                }
                e += 1;
            }
            // Blanks before the closing bracket are not part of the value.
            let mut value_end = e;
            while value_end > value && self.b(value_end - 1).is_ascii_whitespace() {
                value_end -= 1;
            }
            default = Some((k, value, value_end));
            end = e;
        }
        let m = self.ws(end);
        if self.b(m) == b']' {
            // `((?:[^*\s]|\*[^/\s])+)?`
            let mut t = m + 1;
            while t < self.len && !self.b(t).is_ascii_whitespace() && self.b(t) != b'*' {
                t += 1;
            }
            return Some(OptionalParam {
                default,
                close: Some(m),
                end: t,
            });
        }
        self.starts(m, "*/").then_some(OptionalParam {
            default,
            close: None,
            end: m,
        })
    }

    pub(crate) fn doc_optional_param(&mut self, p: usize) -> Option<usize> {
        if self.b(p) != b'[' {
            return None;
        }
        let found = self.optional_param_ahead(p)?;
        let scope = S::VariableOtherJsdoc;
        self.flush(p);
        self.cap(
            Kind::Top,
            p,
            p + 1,
            &[
                scope,
                S::PunctuationDefinitionOptionalValueBeginBracketSquareJsdoc,
            ],
        );
        if let Some((eq, value, value_end)) = found.default {
            self.cap(Kind::Top, p + 1, eq, &[scope]);
            self.cap(
                Kind::Top,
                eq,
                eq + 1,
                &[scope, S::KeywordOperatorAssignmentJsdoc],
            );
            self.cap(Kind::Top, eq + 1, value, &[scope]);
            self.cap(Kind::Top, value, value_end, &[scope, S::SourceEmbedded]);
        }
        if let Some(close) = found.close {
            self.fill(Kind::Top, close, &[scope]);
            self.cap(
                Kind::Top,
                close,
                close + 1,
                &[
                    scope,
                    S::PunctuationDefinitionOptionalValueEndBracketSquareJsdoc,
                ],
            );
            if found.end > close + 1 {
                self.cap(
                    Kind::Top,
                    close + 1,
                    found.end,
                    &[scope, S::InvalidIllegalSyntaxJsdoc],
                );
            }
        } else {
            self.fill(Kind::Top, found.end, &[scope]);
        }
        Some(found.end)
    }

    /// `(?=((ID\s*\??\.\s*)*|(\??\.\s*)?)(ID))`: where a template tag starts.
    pub(crate) fn template_target_start(&self, p: usize) -> bool {
        let mut s = p;
        if matches!(self.b(s), b'?' | b'.') {
            let mut j = s;
            if self.b(j) == b'?' {
                j += 1;
            }
            if self.b(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        self.ident(s).is_some()
    }

    pub(crate) fn possibly_arrow_return_type_rule(&mut self, p: usize) -> Option<usize> {
        self.possibly_arrow_return_type(p)
    }
}
