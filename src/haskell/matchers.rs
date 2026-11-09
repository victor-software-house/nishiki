//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.
//!
//! `^` holds only at the start of the line: Shiki's engine does not match it
//! after the line feed that ends the line.
//! `(?i)` compares words through [`crate::case_fold`], so `ſ`, the Kelvin sign,
//! `ß` and the Latin ligatures match the letters they fold to.

mod comments;
mod decls;
mod literals;
mod symbols;
mod types;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::case_fold;
use crate::classes::{is_digit, is_lower, is_symbol_or_punctuation, is_title_or_upper};
use crate::rules::Chars;

/// `['_\p{Ll}\p{Lu}\p{Lt}\d]`: a character of a name after its first.
fn is_name(c: char) -> bool {
    c == '\'' || c == '_' || is_lower(c) || is_title_or_upper(c) || is_digit(c)
}

/// `[_\p{Ll}]`: the first character of a variable's name.
fn is_var_start(c: char) -> bool {
    c == '_' || is_lower(c)
}

/// ``[[\p{S}\p{P}]&&[^]"'(),;\[_`{}]]``: an operator character.
fn is_op(c: char) -> bool {
    is_symbol_or_punctuation(c)
        && !matches!(
            c,
            ']' | '"' | '\'' | '(' | ')' | ',' | ';' | '[' | '_' | '`' | '{' | '}'
        )
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - comments, pragmas and the C preprocessor;
    /// - literals;
    /// - declarations: modules, imports, classes, data, types, instances,
    ///   foreign functions, deriving clauses and records;
    /// - type signatures and their parts;
    /// - symbols, operators, names and brackets.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::LiquidHaskell => self.lit(p, b"{-@"),
            Pat::LiquidHaskellEnd => self.lit(p, b"@-}"),
            Pat::Cpp => self.cpp(p),
            Pat::Pragma => self.lit(p, b"{-#"),
            Pat::PragmaEnd => self.lit(p, b"#-}"),
            Pat::PragmaAt0 => self.pragma_word(p, b"language"),
            Pat::PragmaAt0End | Pat::PragmaAt1End | Pat::PragmaAt2End | Pat::PragmaAt4End => {
                self.starts(p, b"#-}").then_some(p)
            }
            Pat::PragmaAt0At0 => self.deprecated_extension(p),
            Pat::PragmaAt0At1 => self.extension(p),
            Pat::PragmaAt1 => self.specialise_instance(p),
            Pat::PragmaAt2 => self.specialise(p),
            Pat::PragmaAt3 => self.pragma_name(p),
            Pat::PragmaAt4 => self.deprecated(p),
            Pat::InlinePhase => self.lit(p, b"["),
            Pat::InlinePhaseEnd | Pat::TypeApplicationAt1End => self.lit(p, b"]"),
            Pat::InlinePhaseAt0 => self.lit(p, b"~"),
            Pat::InlinePhaseAt2 => Some(self.word_end(p)),
            Pat::CommentsAt0 => self.haddock_line(p),
            Pat::CommentsAt0End => self.haddock_line_end(p),
            Pat::CommentsAt1 => self.haddock_mark(p),
            Pat::CommentsAt1End | Pat::CommentsAt3At0End => self.line_feed(p),
            Pat::CommentsAt2 => self.haddock_block(p),
            Pat::CommentsAt2End | Pat::BlockCommentEnd => self.lit(p, b"-}"),
            Pat::BlockComment => self.lit(p, b"{-"),
            Pat::CommentsAt3 => self.line_comment(p),
            Pat::CommentsAt3End => (!self.at_g(p)).then_some(p),
            Pat::CommentsAt3At0 => self.lit(p, b"--"),

            Pat::IntegerLiterals => self.integer(p),
            Pat::FloatLiterals => self.float(p),
            Pat::CharLiteral => self.char_literal(p),
            Pat::StringLiteral | Pat::StringLiteralEnd => self.lit(p, b"\""),
            Pat::StringLiteralAt0 => self.named_escape(p),
            Pat::StringLiteralAt1 => self.numeric_escape(p),
            Pat::StringLiteralAt2 => self.control_escape(p),
            Pat::StringLiteralAt3 => self.gap_open(p),
            Pat::StringLiteralAt3End => self.lit(p, b"\\"),
            Pat::StringLiteralAt3At0 => self.non_blanks(p),
            Pat::FfiAt2At3 => self.quoted(p),

            Pat::RootAt11 => self.module_head(p),
            Pat::RootAt11End => self.where_ahead(p, b'\''),
            Pat::RootAt11At3 => {
                let end = self.skip(p, |b| b.is_ascii_lowercase());
                (end > p).then_some(end)
            }
            Pat::ModuleName => self.module_name(p),
            Pat::ModuleExports | Pat::ModuleExportsAt4 => self.lit(p, b"("),
            Pat::ModuleExportsEnd | Pat::ModuleExportsAt4End | Pat::TypeApplicationAt0End => {
                self.lit(p, b")")
            }
            Pat::ModuleExportsAt1 => self.keyword_group(p, b"module"),
            Pat::ExportConstructsAt1 => self.keyword_group(p, b"pattern"),
            Pat::ExportConstructsAt1End => self.constructor_or_operator(p, true),
            Pat::ExportConstructsAt2 => self.keyword_group(p, b"type"),
            Pat::ExportConstructsAt2End => self.constructor_or_operator(p, false),
            Pat::ExportConstructsAt3 => self.variable(p),
            Pat::ExportConstructsAt4 => self.constructor(p),
            Pat::RootAt13 => self.declaration_head(p, b"class"),
            Pat::RootAt21 => self.declaration_head(p, b"instance"),
            Pat::RootAt13End | Pat::RootAt21End | Pat::RootAt22End => {
                self.declaration_end(p, Some(b"where"), b"'")
            }
            Pat::RootAt22 => self.declaration_head(p, b"import"),
            Pat::RootAt22At2 => self.import_word(p),
            Pat::RootAt14Data => self.data_head(p, b"data", true),
            Pat::RootAt14Newtype => self.data_head(p, b"newtype", true),
            Pat::RootAt14End => self.declaration_end(p, Some(b"deriving"), b"'"),
            Pat::RootAt18Data => self.data_head(p, b"data", false),
            Pat::RootAt18Newtype => self.data_head(p, b"newtype", false),
            Pat::RootAt19 => self.type_family_head(p),
            Pat::RootAt20 => self.type_head(p),
            Pat::FfiEnd
            | Pat::DerivingAt0End
            | Pat::RoleAnnotationAt0End
            | Pat::RootAt16End
            | Pat::RootAt17End
            | Pat::RootAt18End
            | Pat::RootAt19End
            | Pat::RootAt20End => self.declaration_end(p, None, b"'"),
            Pat::GadtConstructorAt0End => self.declaration_end(p, Some(b"deriving"), b"''"),
            Pat::RootAt16 => self.pattern_signature(p),
            Pat::RootAt17 => self.pattern_head(p),
            Pat::RoleAnnotationAt0 => self.role_head(p),
            Pat::RoleAnnotationAt0At2 => {
                self.keyword_in(p, &[b"nominal", b"representational", b"phantom"])
            }
            Pat::FfiImport => self.foreign_head(p, b"import"),
            Pat::FfiExport => self.foreign_head(p, b"export"),
            Pat::FfiAt1 => self.calling_convention(p),
            Pat::FfiAt2 => self.foreign_name_ahead(p),
            Pat::FfiAt2End => self.double_colon_ahead(p),
            Pat::FfiAt2At1 => self.safety_and_name(p),
            Pat::FfiAt2At2 => self.safety_to_end(p),
            Pat::FfiAt2At4 => self.foreign_name(p),
            Pat::WhereAt0 | Pat::RootAt14At1 => self.where_brace(p),
            Pat::WhereAt0End | Pat::RootAt14At1End | Pat::LayoutHeraldEnd => {
                self.group_byte(p, b'}')
            }
            Pat::WhereAt0At1 | Pat::RootAt14At1At2 | Pat::LayoutHeraldAt1 => self.lit(p, b";"),
            Pat::WhereAt1 | Pat::RootAt14At2 => self.keyword_group(p, b"where"),
            Pat::GadtConstructorAt0 => self.gadt_head(p),
            Pat::GadtConstructorAt1 => self.gadt_constructor(p),
            Pat::GadtConstructorAt1End => self.at_eol(p).then_some(p),
            Pat::DerivingAt0 => self.deriving_head(p),
            Pat::DerivingAt0At1 => self.keyword_group(p, b"instance"),
            Pat::DerivingAt0At2 => self.keyword_in(p, &[b"via", b"stock", b"newtype", b"anyclass"]),
            Pat::DerivingAt1 => self.deriving_paren(p),
            Pat::DerivingAt1End | Pat::TypeSignatureAt18End | Pat::RootAt37End => {
                self.group_byte(p, b')')
            }
            Pat::DerivingAt2 => self.deriving_class(p),
            Pat::DerivingAt3 => self.keyword_group(p, b"via"),
            Pat::RecordDecl | Pat::Record => self.record_open(p),
            Pat::RecordDeclEnd | Pat::RecordEnd => self.record_close(p),
            Pat::RecordDeclField => self.record_decl_field(p),
            Pat::RecordDeclFieldEnd | Pat::RecordFieldAt0End => self.field_end(p),
            Pat::RecordFieldAt0 => self.record_field(p),
            Pat::AdtConstructorAt1 => self.adt_bar(p),
            Pat::AdtConstructorAt1End => self.adt_constructor(p),
            Pat::Context => self.context(p),
            Pat::LayoutHerald => self.layout_herald(p),
            Pat::Keyword => self.keyword(p),
            Pat::RootAt26 => self.fixity(p),

            Pat::TypeSignatureAt1 => self.unit(p, true),
            Pat::TypeSignatureAt2 | Pat::TypeSignatureAt4 | Pat::RootAt7 => self.unboxed_unit(p),
            Pat::TypeSignatureAt3 => self.tuple(p, true),
            Pat::TypeSignatureAt5 | Pat::RootAt9 => self.unboxed_tuple(p),
            Pat::TypeSignatureAt6 => self.empty_list(p, true),
            Pat::TypeSignatureAt8 => self.double_colon(p),
            Pat::Forall => self.forall(p),
            Pat::ForallEnd => self.forall_end(p),
            Pat::TypeVariable => self.type_variable(p),
            Pat::TypeSignatureAt10 => self.lit(p, b"=>").or_else(|| self.lit(p, "⇒".as_bytes())),
            Pat::TypeSignatureAt12 => self.promoted_char(p),
            Pat::TypeApplicationAt0 => self.type_application(p, b'('),
            Pat::TypeApplicationAt1 => self.type_application(p, b'['),
            Pat::TypeApplicationAt2 => self.type_application_string(p),
            Pat::TypeApplicationAt2End => self.prev_is(p, b'"').then_some(p),
            Pat::TypeApplicationAt3 => self.type_application_name(p),
            Pat::TypeApplicationAt3End => {
                (!self.char_here(p).is_some_and(|(c, _)| is_name(c))).then_some(p)
            }
            Pat::TypeOperatorAt0 => self.type_operator(p),
            Pat::TypeOperatorAt1 => self.type_operator_backticks(p),
            Pat::TypeConstructorAt0 => self.type_constructor(p),
            Pat::TypeConstructorAt1 => self.type_constructor_operator(p),
            Pat::TypeSignatureAt17 => self.hash_paren(p),
            Pat::TypeSignatureAt17End | Pat::RootAt36End => self.hash_close(p),
            Pat::TypeSignatureAt18 => self.promoted_open(p, b'('),
            Pat::TypeSignatureAt19 => self.promoted_open(p, b'['),
            Pat::TypeSignatureAt19End | Pat::RootAt39End => self.group_byte(p, b']'),
            Pat::DoubleColon => self.spaced_double_colon(p),
            Pat::FunDecl => self.fun_decl(p),
            Pat::FunDeclEnd => self.fun_decl_end(p),
            Pat::StartTypeSignatureAt0 => self.signature_line(p),
            Pat::StartTypeSignatureAt0End => self.signature_end(p, true),
            Pat::StartTypeSignatureAt1 => self.inline_signature(p),
            Pat::StartTypeSignatureAt1End => self.signature_end(p, false),

            Pat::Comma => self.lit(p, b","),
            Pat::RootAt5 => self.stray_close(p),
            Pat::RootAt6 => self.unit(p, false),
            Pat::RootAt8 => self.tuple(p, false),
            Pat::RootAt10 => self.empty_list(p, false),
            Pat::RecordWildcard => self.record_wildcard(p),
            Pat::PrefixOpAt0 => self.prefix_op(p),
            Pat::ReservedSymbolAt0 => self.reserved_symbol(p),
            Pat::ReservedSymbolAt1 => self.hashes(p),
            Pat::ReservedSymbolAt2 => self.as_pattern(p),
            Pat::ReservedSymbolAt3 => self.prefix_symbol(p),
            Pat::DataConstructor => self.data_constructor(p),
            Pat::InfixOpAt0 => self.infix_op(p),
            Pat::InfixOpAt1 => self.infix_backticks(p),
            Pat::OverloadedLabelAt0 => self.overloaded_label(p),
            Pat::Qualifier => self.qualifier(p),
            Pat::RootAt36 => self.unboxed_open(p),
            Pat::RootAt37 => self.group_byte(p, b'('),
            Pat::RootAt39 => self.group_byte(p, b'['),
            Pat::QuasiQuoteAt0 => self.quasi_quote(p, b"dep", true),
            Pat::QuasiQuoteAt1 => self.quasi_quote(p, b"t", false),
            Pat::QuasiQuoteAt2 => self.quasi_quoter(p),
            Pat::QuasiQuoteAt0End | Pat::QuasiQuoteAt1End | Pat::QuasiQuoteAt2End => {
                self.quasi_quote_end(p)
            }
        }
    }

    /// The character at `i`, before the line feed, and its end.
    fn char_here(&self, i: usize) -> Option<(char, usize)> {
        if i < self.lim() {
            self.char_at(i)
        } else {
            None
        }
    }

    /// The end of the run of characters from `i` that `class` accepts.
    fn class_end(&self, i: usize, class: fn(char) -> bool) -> usize {
        let mut at = i;
        while let Some((c, next)) = self.char_here(at)
            && class(c)
        {
            at = next;
        }
        self.spend(at - i);
        at
    }

    /// Whether the character before `p` is in `class`.
    fn before_is(&self, p: usize, class: fn(char) -> bool) -> bool {
        p > 0 && self.char_before(p).is_some_and(class)
    }

    /// Whether the character at `i` is in `class`.
    fn here_is(&self, i: usize, class: fn(char) -> bool) -> bool {
        self.char_here(i).is_some_and(|(c, _)| class(c))
    }

    /// `\b(?<!')word\b(?!')` at `i`: the word, not part of a longer name nor
    /// next to a quote.
    fn keyword_at(&self, i: usize, word: &[u8]) -> Option<usize> {
        if self.prev_is(i, b'\'') || self.word_before(i) {
            return None;
        }
        let end = self.lit(i, word)?;
        (self.word_at(end).is_none() && self.at(end) != b'\'').then_some(end)
    }

    /// [`Lexer::keyword_at`] with group 1 around the word.
    fn keyword_group(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword_at(p, word)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\b(?<!')(a|b|…)\b(?!')`, with group 1 around the first of `words`
    /// that matches.
    fn keyword_in(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = words.iter().find_map(|word| self.keyword_at(p, word))?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?i:word)` at `i`, for the lowercase `word`, and its end.
    fn folded(&self, i: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s, i, word)
    }

    /// `\n` at `p`.
    fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.at(p) == b'\n').then_some(p + 1)
    }

    /// `(x)` for the byte `x`, `byte`, with group 1 around it.
    fn group_byte(&mut self, p: usize, byte: u8) -> Option<usize> {
        if !(p < self.lim() && self.at(p) == byte) {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `(?=\b(?<!x)where\b(?!x))` for the quote `x`: `where` ahead.
    fn where_ahead(&self, p: usize, quote: u8) -> Option<usize> {
        if self.prev_is(p, quote) || self.word_before(p) {
            return None;
        }
        let end = self.lit(p, b"where")?;
        (self.word_at(end).is_none() && self.at(end) != quote).then_some(p)
    }

    /// The end of a declaration that runs while its lines are indented:
    /// `(?=\b(?<!x)word\b(?!'))|(?=[;}])|^(?!\1\s+\S|\s*(?:$|\{-[^@]|--+(?!
    /// [OP]).*$))`.
    /// The first alternative applies only for a `word`.
    /// In it, `x` stands for the quotes `before` the word must not follow.
    /// `\1` is the declaration's own indentation.
    ///
    /// The last alternative holds at the start of a line that is neither
    /// indented past the declaration, nor blank, nor only a comment.
    fn declaration_end(&self, p: usize, word: Option<&[u8]>, before: &[u8]) -> Option<usize> {
        if let Some(word) = word
            && !(p >= before.len() && self.text(p - before.len(), p) == before)
            && !self.word_before(p)
            && let Some(end) = self.lit(p, word)
            && self.word_at(end).is_none()
            && self.at(end) != b'\''
        {
            return Some(p);
        }
        if p < self.lim() && matches!(self.at(p), b';' | b'}') {
            return Some(p);
        }
        (p == 0 && !self.continues_declaration()).then_some(p)
    }

    /// `\1\s+\S|\s*(?:$|\{-[^@]|--+(?![OP]).*$)` at the start of the line:
    /// whether the line is indented past the declaration, blank, or only a
    /// comment.
    fn continues_declaration(&self) -> bool {
        let indent = self.payload_text().as_bytes();
        if self.s.starts_with(indent) {
            let blanks = self.space_end(indent.len());
            if blanks > indent.len() && blanks < self.lim() {
                return true;
            }
        }
        let start = self.space_end(0);
        if self.at_eol(start) {
            return true;
        }
        if self.starts(start, b"{-") && self.avail(start + 2) && self.at(start + 2) != b'@' {
            return true;
        }
        let dashes = self.skip(start, |b| b == b'-');
        dashes - start >= 2 && !self.here_is(dashes, is_op)
    }

    /// The end of `[x]['_\p{Ll}\p{Lu}\p{Lt}\d]*` at `i`, for the class `first`
    /// of the first character.
    fn name_from(&self, i: usize, first: fn(char) -> bool) -> Option<usize> {
        let (c, next) = self.char_here(i)?;
        first(c).then(|| self.class_end(next, is_name))
    }

    /// The end of `X\b(?!')`, for a run that `X` took greedily up to `end`.
    ///
    /// The run gives characters back, down to `min`, until a boundary with no
    /// quote after it holds.
    fn tail_end(&self, min: usize, end: usize) -> Option<usize> {
        let mut e = end;
        loop {
            if self.boundary(e) && self.at(e) != b'\'' {
                return Some(e);
            }
            if e <= min {
                return None;
            }
            e = self.char_start_before(e);
        }
    }

    /// `\(\s*X\s*\)` at `i`, for the operator `op` that gives the end of `X`:
    /// where `X` starts and ends, and where the `)` stands.
    ///
    /// No blank given back helps: `X` starts and ends with no blank.
    fn paren_op(
        &self,
        i: usize,
        op: impl Fn(&Self, usize) -> Option<usize>,
    ) -> Option<(usize, usize, usize)> {
        if !(i < self.lim() && self.at(i) == b'(') {
            return None;
        }
        let start = self.space_end(i + 1);
        let end = op(self, start)?;
        let close = self.space_end(end);
        (close < self.lim() && self.at(close) == b')').then_some((start, end, close))
    }

    /// Groups `groups` around the `(`, the operator and the `)` that
    /// [`Lexer::paren_op`] found from `open`, and the end of the match.
    fn cap_paren(
        &mut self,
        groups: [usize; 3],
        open: usize,
        (start, end, close): (usize, usize, usize),
    ) -> usize {
        self.cap(groups[0], open, open + 1);
        self.cap(groups[1], start, end);
        self.cap(groups[2], close, close + 1);
        close + 1
    }

    /// The end of `[OP]+` at `i`, or of `[OP]*` when `empty`.
    fn ops(&self, i: usize, empty: bool) -> Option<usize> {
        let end = self.class_end(i, is_op);
        (empty || end > i).then_some(end)
    }

    /// The end of `:[OP]+` at `i`, or of `:[OP]*` when `empty`.
    fn colon_ops(&self, i: usize, empty: bool) -> Option<usize> {
        let colon = self.lit(i, b":")?;
        self.ops(colon, empty)
    }

    /// Whether `--+\)` matches at `i`.
    ///
    /// Giving a dash back leaves a dash before the `)`.
    fn dashes_close(&self, i: usize) -> bool {
        let end = self.skip(i, |b| b == b'-');
        end - i >= 2 && self.at(end) == b')'
    }

    /// Whether `x(?![OP])` matches at `i` for one of `texts`, and where.
    fn alone_after(&self, i: usize, texts: &[&[u8]]) -> Option<usize> {
        texts
            .iter()
            .find_map(|text| self.lit(i, text).filter(|&end| !self.here_is(end, is_op)))
    }
}
