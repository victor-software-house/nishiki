//! Comments: block, line and documentation comments.

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;

impl<const D: u8> Lexer<'_, D> {
    /// A block, documentation or line comment at `p`.
    ///
    /// A line comment also takes the blanks `^[\t ]+` before its `//`.
    pub(crate) fn comment(&mut self, p: usize) -> Option<usize> {
        match self.byte(p) {
            b'/' => {
                if self.starts(p, "/**") && self.byte(p + "/**".len()) != b'/' {
                    return self.open_token(
                        Rule::CommentDoc,
                        p,
                        p + "/**".len(),
                        &[S::PunctuationDefinitionComment],
                    );
                }
                if self.byte(p + 1) == b'*' {
                    return self.block_comment_begin(p);
                }
                if self.byte(p + 1) == b'/' {
                    return self.line_comment_begin(p, p, Rule::CommentLine);
                }
                None
            }
            b' ' | b'\t' => {
                if !Self::bol(p) {
                    return None;
                }
                let mut j = p;
                while matches!(self.byte(j), b' ' | b'\t') {
                    j += 1;
                }
                (self.starts(j, "//")).then(|| self.line_comment_begin(p, j, Rule::CommentLine))?
            }
            _ => None,
        }
    }

    /// The two comment rules that HTML's grammar puts before the JavaScript
    /// of a script, at `p`:
    /// - `(^[\t ]+)?(?=//)` then `//`, a line comment that ends with the line
    ///   or before `</script`;
    /// - `/\*`, a block comment that ends at `*/` or before `</script`.
    ///
    /// The rule around the line comment has no scope.
    /// It ends as soon as the comment does, so the comment is the one context.
    pub(crate) fn script_comment(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        let mut slashes = p;
        if Self::bol(p) {
            while matches!(self.byte(slashes), b' ' | b'\t') {
                slashes += 1;
            }
            self.spend(slashes - p);
        }
        if self.starts(slashes, "//") {
            self.flush(p);
            self.capture(
                Rule::Script,
                p,
                slashes,
                &[S::PunctuationWhitespaceCommentLeading],
            );
            return self.open_token(
                Rule::ScriptLineComment,
                slashes,
                slashes + "//".len(),
                &[S::PunctuationDefinitionComment],
            );
        }
        if self.starts(p, "/*") {
            return self.open_token(
                Rule::ScriptBlockComment,
                p,
                p + "/*".len(),
                &[S::PunctuationDefinitionComment],
            );
        }
        None
    }

    /// `(/\*)(?:\s*((@)internal)(?=\s|(\*/)))?`.
    fn block_comment_begin(&mut self, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(p);
        self.capture(
            Rule::CommentBlock,
            p,
            p + 2,
            &[S::PunctuationDefinitionComment],
        );
        let mut e = p + 2;
        let j = self.ws(e);
        if self.starts(j, "@internal") {
            let after = j + "@internal".len();
            if self.space_at(after) || self.starts(after, "*/") || after >= self.len {
                self.capture(
                    Rule::CommentBlock,
                    j,
                    j + 1,
                    &[
                        S::StorageTypeInternaldeclaration,
                        S::PunctuationDecoratorInternaldeclaration,
                    ],
                );
                self.capture(
                    Rule::CommentBlock,
                    j + 1,
                    after,
                    &[S::StorageTypeInternaldeclaration],
                );
                e = after;
            }
        }
        self.push(Rule::CommentBlock);
        Some(e)
    }

    /// `(^[\t ]+)?((//)(?:\s*((@)internal)(?=\s|$))?)`: `start` is where the
    /// match starts and `slashes` where the `//` is.
    pub(crate) fn line_comment_begin(
        &mut self,
        start: usize,
        slashes: usize,
        rule: Rule,
    ) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(start);
        let line = S::CommentLineDoubleSlash;
        if slashes > start {
            self.capture(
                rule,
                start,
                slashes,
                &[S::PunctuationWhitespaceCommentLeading],
            );
        }
        self.capture(
            rule,
            slashes,
            slashes + 2,
            &[line, S::PunctuationDefinitionComment],
        );
        let mut e = slashes + 2;
        let j = self.ws(e);
        if self.starts(j, "@internal") {
            let after = j + "@internal".len();
            if self.space_at(after) || after >= self.len {
                self.capture(rule, e, j, &[line]);
                self.capture(
                    rule,
                    j,
                    j + 1,
                    &[
                        line,
                        S::StorageTypeInternaldeclaration,
                        S::PunctuationDecoratorInternaldeclaration,
                    ],
                );
                self.capture(
                    rule,
                    j + 1,
                    after,
                    &[line, S::StorageTypeInternaldeclaration],
                );
                e = after;
            }
        }
        self.push(rule);
        Some(e)
    }

    /// `(^[\t ]+)?((//)…)` that runs to the start of the next line.
    pub(crate) fn single_line_comment_consuming_line_ending(&mut self, p: usize) -> Option<usize> {
        if self.byte(p) == b'/' && self.byte(p + 1) == b'/' {
            return self.line_comment_begin(p, p, Rule::CommentLineNl);
        }
        if Self::bol(p) && matches!(self.byte(p), b' ' | b'\t') {
            let mut j = p;
            while matches!(self.byte(j), b' ' | b'\t') {
                j += 1;
            }
            if self.starts(j, "//") {
                return self.line_comment_begin(p, j, Rule::CommentLineNl);
            }
        }
        None
    }
}
