//! Type alias declarations.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;

impl<const D: u8> Lexer<'_, D> {
    #[expect(
        clippy::many_single_char_names,
        reason = "positions are named by role: `p` where the rule starts, `e` where a part ends, `j` and `k` for the cursor"
    )]
    pub(crate) fn type_alias_declaration(&mut self, p: usize) -> Option<usize> {
        let pre = self.declaration_start(p)?;
        let k = pre.at;
        if !self.starts(k, "type")
            || !self.boundary(k)
            || !self.boundary(k + "type".len())
            || self.full()
        {
            return None;
        }
        let e = k + "type".len();
        let j = self.ws(e);
        if j == e {
            return None;
        }
        let n = self.ident(j)?;
        let end = self.ws(n);
        self.flush(p);
        self.prefix_tokens(Rule::TypeAlias, &pre);
        self.capture(Rule::TypeAlias, k, e, &[S::StorageTypeType]);
        self.capture(Rule::TypeAlias, j, n, &[S::EntityNameTypeAlias]);
        self.finish(Rule::TypeAlias, end);
        self.push(Rule::TypeAlias);
        Some(end)
    }

    pub(crate) fn type_alias_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(r) = self.type_parameters(p) {
            return Some(r);
        }
        if self.byte(p) == b'=' && !self.full() {
            let j = self.ws(p + 1);
            let mut intrinsic = None;
            if self.starts(j, "intrinsic") && self.keyword_after(j + "intrinsic".len()) {
                intrinsic = Some((j, j + "intrinsic".len()));
            }
            self.flush(p);
            self.capture(
                Rule::TypeAliasEquals,
                p,
                p + 1,
                &[S::KeywordOperatorAssignment],
            );
            if let Some((a, b)) = intrinsic {
                self.capture(Rule::TypeAliasEquals, a, b, &[S::KeywordControlIntrinsic]);
            } else {
                self.finish(Rule::TypeAliasEquals, j);
            }
            self.push(Rule::TypeAliasEquals);
            return Some(intrinsic.map_or(j, |(_, e)| e));
        }
        None
    }
}
