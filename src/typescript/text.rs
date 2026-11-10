//! Strings, template literals, regular expressions and comments.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn string(&mut self, p: usize) -> Option<usize> {
        match self.byte(p) {
            b'\'' => self.open_token(
                Rule::StrSingle,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            b'"' => self.open_token(
                Rule::StrDouble,
                p,
                p + 1,
                &[S::PunctuationDefinitionStringBegin],
            ),
            _ => self.template(p),
        }
    }

    /// `\\(x\h{2}|u\h{4}|u\{\h+}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-7]?
    /// |[4-7][0-7]?|.|$)`.
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn string_character_escape(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'\\' {
            return None;
        }
        let j = p + 1;
        let hex = |i: usize, n: usize| (0..n).all(|k| self.byte(i + k).is_ascii_hexdigit());
        let oct = |i: usize| (b'0'..=b'7').contains(&self.byte(i));
        let c = self.byte(j);
        let e = if c == b'x' && hex(j + 1, 2) {
            j + 3
        } else if c == b'u' && hex(j + 1, 4) {
            j + 5
        } else if c == b'u' && self.byte(j + 1) == b'{' && self.byte(j + 2).is_ascii_hexdigit() {
            let mut k = j + 2;
            while self.byte(k).is_ascii_hexdigit() {
                k += 1;
            }
            if self.byte(k) == b'}' { k + 1 } else { j + 1 }
        } else if matches!(c, b'0'..=b'2') {
            let mut k = j + 1;
            let mut n = 0;
            while n < 2 && oct(k) {
                k += 1;
                n += 1;
            }
            k
        } else if c == b'3' && (b'0'..=b'7').contains(&self.byte(j + 1)) {
            if oct(j + 2) { j + 3 } else { j + 2 }
        } else if matches!(c, b'4'..=b'7') {
            if oct(j + 1) { j + 2 } else { j + 1 }
        } else if j >= self.len {
            j
        } else {
            j + self.char_len(j)
        };
        self.token_rule(p, e, S::ConstantCharacterEscape)
    }

    /// A template literal, with or without a tag.
    fn template(&mut self, p: usize) -> Option<usize> {
        if self.tagged_template_ahead(p) && !self.full() {
            return self.open_empty(Rule::TemplateCall, p);
        }
        self.template_type_arguments(p)
            .or_else(|| self.template_begin(p, Rule::Template))
    }

    /// A template tag whose type arguments follow blanks, or that has no name:
    /// an optional name, blanks, and a `<…>` group before the backtick.
    fn template_type_arguments(&mut self, p: usize) -> Option<usize> {
        if !self.has_backtick_after(p) || self.full() {
            return None;
        }
        let tag = self.ident(p);
        let j = self.ws(tag.unwrap_or(p));
        let end = self.type_args(j).filter(|_| self.byte(j) == b'<')?;
        if self.byte(self.ws(end)) != b'`' {
            return None;
        }
        self.flush(p);
        if let Some(e) = tag {
            self.capture(Rule::Top, p, e, &[S::EntityNameFunctionTaggedTemplate]);
        }
        self.finish(Rule::TemplateCall2, j);
        self.push(Rule::TemplateCall2);
        Some(j)
    }

    /// The opening of a template literal, with an optional tag name before the
    /// backtick.
    fn template_begin(&mut self, p: usize, rule: Rule) -> Option<usize> {
        if self.full() {
            return None;
        }
        let (tag_end, tick) = match self.ident(p) {
            Some(e) if self.byte(e) == b'`' => (Some(e), e),
            _ if self.byte(p) == b'`' => (None, p),
            _ => return None,
        };
        self.flush(p);
        if let Some(e) = tag_end {
            self.capture(Rule::Top, p, e, &[S::EntityNameFunctionTaggedTemplate]);
        }
        self.capture(
            Rule::Top,
            tick,
            tick + 1,
            &[
                S::StringTemplate,
                S::PunctuationDefinitionStringTemplateBegin,
            ],
        );
        self.push(rule);
        Some(tick + 1)
    }

    /// A template literal in a type position.
    pub(crate) fn template_type(&mut self, p: usize) -> Option<usize> {
        if self.tagged_template_ahead(p) && !self.full() {
            return self.open_empty(Rule::TemplateCall, p);
        }
        self.template_type_arguments(p)
            .or_else(|| self.template_begin(p, Rule::TemplateType))
    }

    /// `${` inside a template.
    pub(crate) fn template_substitution(&mut self, p: usize, rule: Rule) -> Option<usize> {
        if self.byte(p) != b'$' || self.byte(p + 1) != b'{' {
            return None;
        }
        self.open_token(
            rule,
            p,
            p + 2,
            &[S::PunctuationDefinitionTemplateExpressionBegin],
        )
    }

    /// The lookahead of a tagged template: a name chain, optionally with type
    /// arguments, then a backtick.
    fn tagged_template_ahead(&self, p: usize) -> bool {
        if !self.has_backtick_after(p) {
            return false;
        }
        let mut s = p;
        if matches!(self.byte(s), b'?' | b'.') {
            let mut j = s;
            if self.byte(j) == b'?' {
                j += 1;
            }
            if self.byte(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        let limit = self.window();
        while s < limit {
            self.spend(1);
            let Some(e) = self.ident(s) else {
                return false;
            };
            if self.byte(e) == b'`' {
                return true;
            }
            if self.byte(e) == b'<'
                && self
                    .type_args(e)
                    .is_some_and(|t| self.byte(self.ws(t)) == b'`')
            {
                return true;
            }
            let mut j = self.ws(e);
            if self.byte(j) == b'?' {
                j += 1;
            }
            if self.byte(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        false
    }

    /// `(?=((ID\s*\??\.\s*)*|(\??\.\s*)?)(ID))`: where a template tag starts.
    pub(crate) fn template_target_start(&self, p: usize) -> bool {
        let mut s = p;
        if matches!(self.byte(s), b'?' | b'.') {
            let mut j = s;
            if self.byte(j) == b'?' {
                j += 1;
            }
            if self.byte(j) != b'.' {
                return false;
            }
            s = self.ws(j + 1);
        }
        self.ident(s).is_some()
    }
}
