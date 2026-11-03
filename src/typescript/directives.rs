//! Triple-slash directives and the attributes they carry.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// `^(///)\s*(?=<(reference|amd-dependency|amd-module)(…)+\s*/>\s*$)`.
    pub(crate) fn directive(&mut self, p: usize) -> Option<usize> {
        if !self.bol(p) || !self.starts(0, "///") || self.full() {
            return None;
        }
        let j = self.ws(3);
        if !self.directive_ahead(j) {
            return None;
        }
        self.flush(0);
        self.capture(Kind::Directive, 0, 3, &[S::PunctuationDefinitionComment]);
        self.finish(Kind::Directive, j);
        self.push(Kind::Directive);
        Some(j)
    }

    /// `<name( attr = "value")+\s*/>\s*$`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    fn directive_ahead(&self, i: usize) -> bool {
        if self.byte(i) != b'<' {
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
            if self.byte(eq) != b'=' {
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
        if self.byte(p) != b'<' || self.full() {
            return None;
        }
        let name = ["reference", "amd-dependency", "amd-module"]
            .iter()
            .find(|w| self.starts(p + 1, w))?;
        let e = p + 1 + name.len();
        self.flush(p);
        self.capture(
            Kind::DirectiveTag,
            p,
            p + 1,
            &[S::PunctuationDefinitionTagDirective],
        );
        self.capture(Kind::DirectiveTag, p + 1, e, &[S::EntityNameTagDirective]);
        self.push(Kind::DirectiveTag);
        Some(e)
    }

    pub(crate) fn directive_tag_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.directive_attribute(p) {
            return self.token_rule(p, e, S::EntityOtherAttributeNameDirective);
        }
        if self.byte(p) == b'=' {
            return self.token_rule(p, p + 1, S::KeywordOperatorAssignment);
        }
        self.string(p)
    }
}
