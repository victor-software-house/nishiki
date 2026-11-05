//! Svelte's injection into the TypeScript and JavaScript it embeds:
//! `L:(source.ts, source.js, source.coffee)`, which the grammar tries before
//! the embedded language's own rules, in every one of its contexts.

use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    /// A `$` before a name, as in `$count`, or the `$$` of `$$props`,
    /// `$$restProps` and `$$slots`, not after a quote, a `$`, a dot, a slash
    /// or a name character: `(?<!["$'./_[:alnum:]])\$(?=[_[:alpha:]])` and
    /// `(?<!["$'./_[:alnum:]])(\$\$)(?=props|restProps|slots)`.
    pub(crate) fn svelte_variable(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) != b'$'
            || p >= self.len
            || self.alnum_before(p)
            || matches!(
                p.checked_sub(1).map(|i| self.byte(i)),
                Some(b'"' | b'\'' | b'.' | b'/')
            )
        {
            return None;
        }
        if self.byte(p + 1) != b'$' && self.id_start(p + 1) {
            return self.token_rule(p, p + 1, S::PunctuationDefinitionVariableSvelte);
        }
        let names = ["props", "restProps", "slots"];
        (self.byte(p + 1) == b'$' && names.iter().any(|name| self.starts(p + 2, name)))
            .then(|| self.token_rule(p, p + 2, S::PunctuationDefinitionVariableSvelte))
            .flatten()
    }

    /// Where the plain text of a comment or string ends, from `step`, for
    /// the TypeScript that Svelte embeds: at the next `$` before `target`,
    /// where Svelte's injection can match, since it applies in every
    /// context.
    pub(crate) fn svelte_body_end(&self, step: usize, target: usize) -> usize {
        if D & super::Dialect::SVELTE == 0 {
            return target;
        }
        self.bytes
            .get(step..target)
            .and_then(|text| memchr::memchr(b'$', text))
            .map_or(target, |at| step + at)
    }
}
