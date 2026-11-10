//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
use super::grammar::Pattern;
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
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::LiquidHaskell => self.lit(p, b"{-@"),
            Pattern::LiquidHaskellEnd => self.lit(p, b"@-}"),
            Pattern::Cpp => self.cpp(p),
            Pattern::Pragma => self.lit(p, b"{-#"),
            Pattern::PragmaEnd => self.lit(p, b"#-}"),
            Pattern::PragmaAt0 => self.pragma_word(p, b"language"),
            Pattern::PragmaAt0End
            | Pattern::PragmaAt1End
            | Pattern::PragmaAt2End
            | Pattern::PragmaAt4End => self.starts(p, b"#-}").then_some(p),
            Pattern::PragmaAt0At0 => self.deprecated_extension(p),
            Pattern::PragmaAt0At1 => self.extension(p),
            Pattern::PragmaAt1 => self.specialise_instance(p),
            Pattern::PragmaAt2 => self.specialise(p),
            Pattern::PragmaAt3 => self.pragma_name(p),
            Pattern::PragmaAt4 => self.deprecated(p),
            Pattern::InlinePhase => self.lit(p, b"["),
            Pattern::InlinePhaseEnd | Pattern::TypeApplicationAt1End => self.lit(p, b"]"),
            Pattern::InlinePhaseAt0 => self.lit(p, b"~"),
            Pattern::InlinePhaseAt2 => Some(self.word_end(p)),
            Pattern::CommentsAt0 => self.haddock_line(p),
            Pattern::CommentsAt0End => self.haddock_line_end(p),
            Pattern::CommentsAt1 => self.haddock_mark(p),
            Pattern::CommentsAt1End | Pattern::CommentsAt3At0End => self.line_feed(p),
            Pattern::CommentsAt2 => self.haddock_block(p),
            Pattern::CommentsAt2End | Pattern::BlockCommentEnd => self.lit(p, b"-}"),
            Pattern::BlockComment => self.lit(p, b"{-"),
            Pattern::CommentsAt3 => self.line_comment(p),
            Pattern::CommentsAt3End => (!self.at_g(p)).then_some(p),
            Pattern::CommentsAt3At0 => self.lit(p, b"--"),

            Pattern::IntegerLiterals => self.integer(p),
            Pattern::FloatLiterals => self.float(p),
            Pattern::CharLiteral => self.char_literal(p),
            Pattern::StringLiteral | Pattern::StringLiteralEnd => self.lit(p, b"\""),
            Pattern::StringLiteralAt0 => self.named_escape(p),
            Pattern::StringLiteralAt1 => self.numeric_escape(p),
            Pattern::StringLiteralAt2 => self.control_escape(p),
            Pattern::StringLiteralAt3 => self.gap_open(p),
            Pattern::StringLiteralAt3End => self.lit(p, b"\\"),
            Pattern::StringLiteralAt3At0 => self.non_blanks(p),
            Pattern::FfiAt2At3 => self.quoted(p),

            Pattern::RootAt11 => self.module_head(p),
            Pattern::RootAt11End => self.where_ahead(p, b'\''),
            Pattern::RootAt11At3 => {
                let end = self.skip(p, |b| b.is_ascii_lowercase());
                (end > p).then_some(end)
            }
            Pattern::ModuleName => self.module_name(p),
            Pattern::ModuleExports | Pattern::ModuleExportsAt4 => self.lit(p, b"("),
            Pattern::ModuleExportsEnd
            | Pattern::ModuleExportsAt4End
            | Pattern::TypeApplicationAt0End => self.lit(p, b")"),
            Pattern::ModuleExportsAt1 => self.keyword_group(p, b"module"),
            Pattern::ExportConstructsAt1 => self.keyword_group(p, b"pattern"),
            Pattern::ExportConstructsAt1End => self.constructor_or_operator(p, true),
            Pattern::ExportConstructsAt2 => self.keyword_group(p, b"type"),
            Pattern::ExportConstructsAt2End => self.constructor_or_operator(p, false),
            Pattern::ExportConstructsAt3 => self.variable(p),
            Pattern::ExportConstructsAt4 => self.constructor(p),
            Pattern::RootAt13 => self.declaration_head(p, b"class"),
            Pattern::RootAt21 => self.declaration_head(p, b"instance"),
            Pattern::RootAt13End | Pattern::RootAt21End | Pattern::RootAt22End => {
                self.declaration_end(p, Some(b"where"), b"'")
            }
            Pattern::RootAt22 => self.declaration_head(p, b"import"),
            Pattern::RootAt22At2 => self.import_word(p),
            Pattern::RootAt14Data => self.data_head(p, b"data", true),
            Pattern::RootAt14Newtype => self.data_head(p, b"newtype", true),
            Pattern::RootAt14End => self.declaration_end(p, Some(b"deriving"), b"'"),
            Pattern::RootAt18Data => self.data_head(p, b"data", false),
            Pattern::RootAt18Newtype => self.data_head(p, b"newtype", false),
            Pattern::RootAt19 => self.type_family_head(p),
            Pattern::RootAt20 => self.type_head(p),
            Pattern::FfiEnd
            | Pattern::DerivingAt0End
            | Pattern::RoleAnnotationAt0End
            | Pattern::RootAt16End
            | Pattern::RootAt17End
            | Pattern::RootAt18End
            | Pattern::RootAt19End
            | Pattern::RootAt20End => self.declaration_end(p, None, b"'"),
            Pattern::GadtConstructorAt0End => self.declaration_end(p, Some(b"deriving"), b"''"),
            Pattern::RootAt16 => self.pattern_signature(p),
            Pattern::RootAt17 => self.pattern_head(p),
            Pattern::RoleAnnotationAt0 => self.role_head(p),
            Pattern::RoleAnnotationAt0At2 => {
                self.keyword_in(p, &[b"nominal", b"representational", b"phantom"])
            }
            Pattern::FfiImport => self.foreign_head(p, b"import"),
            Pattern::FfiExport => self.foreign_head(p, b"export"),
            Pattern::FfiAt1 => self.calling_convention(p),
            Pattern::FfiAt2 => self.foreign_name_ahead(p),
            Pattern::FfiAt2End => self.double_colon_ahead(p),
            Pattern::FfiAt2At1 => self.safety_and_name(p),
            Pattern::FfiAt2At2 => self.safety_to_end(p),
            Pattern::FfiAt2At4 => self.foreign_name(p),
            Pattern::WhereAt0 | Pattern::RootAt14At1 => self.where_brace(p),
            Pattern::WhereAt0End | Pattern::RootAt14At1End | Pattern::LayoutHeraldEnd => {
                self.group_byte(p, b'}')
            }
            Pattern::WhereAt0At1 | Pattern::RootAt14At1At2 | Pattern::LayoutHeraldAt1 => {
                self.lit(p, b";")
            }
            Pattern::WhereAt1 | Pattern::RootAt14At2 => self.keyword_group(p, b"where"),
            Pattern::GadtConstructorAt0 => self.gadt_head(p),
            Pattern::GadtConstructorAt1 => self.gadt_constructor(p),
            Pattern::GadtConstructorAt1End => self.at_eol(p).then_some(p),
            Pattern::DerivingAt0 => self.deriving_head(p),
            Pattern::DerivingAt0At1 => self.keyword_group(p, b"instance"),
            Pattern::DerivingAt0At2 => {
                self.keyword_in(p, &[b"via", b"stock", b"newtype", b"anyclass"])
            }
            Pattern::DerivingAt1 => self.deriving_paren(p),
            Pattern::DerivingAt1End | Pattern::TypeSignatureAt18End | Pattern::RootAt37End => {
                self.group_byte(p, b')')
            }
            Pattern::DerivingAt2 => self.deriving_class(p),
            Pattern::DerivingAt3 => self.keyword_group(p, b"via"),
            Pattern::RecordDecl | Pattern::Record => self.record_open(p),
            Pattern::RecordDeclEnd | Pattern::RecordEnd => self.record_close(p),
            Pattern::RecordDeclField => self.record_decl_field(p),
            Pattern::RecordDeclFieldEnd | Pattern::RecordFieldAt0End => self.field_end(p),
            Pattern::RecordFieldAt0 => self.record_field(p),
            Pattern::AdtConstructorAt1 => self.adt_bar(p),
            Pattern::AdtConstructorAt1End => self.adt_constructor(p),
            Pattern::Context => self.context(p),
            Pattern::LayoutHerald => self.layout_herald(p),
            Pattern::Keyword => self.keyword(p),
            Pattern::RootAt26 => self.fixity(p),

            Pattern::TypeSignatureAt1 => self.unit(p, true),
            Pattern::TypeSignatureAt2 | Pattern::TypeSignatureAt4 | Pattern::RootAt7 => {
                self.unboxed_unit(p)
            }
            Pattern::TypeSignatureAt3 => self.tuple(p, true),
            Pattern::TypeSignatureAt5 | Pattern::RootAt9 => self.unboxed_tuple(p),
            Pattern::TypeSignatureAt6 => self.empty_list(p, true),
            Pattern::TypeSignatureAt8 => self.double_colon(p),
            Pattern::Forall => self.forall(p),
            Pattern::ForallEnd => self.forall_end(p),
            Pattern::TypeVariable => self.type_variable(p),
            Pattern::TypeSignatureAt10 => {
                self.lit(p, b"=>").or_else(|| self.lit(p, "⇒".as_bytes()))
            }
            Pattern::TypeSignatureAt12 => self.promoted_char(p),
            Pattern::TypeApplicationAt0 => self.type_application(p, b'('),
            Pattern::TypeApplicationAt1 => self.type_application(p, b'['),
            Pattern::TypeApplicationAt2 => self.type_application_string(p),
            Pattern::TypeApplicationAt2End => self.prev_is(p, b'"').then_some(p),
            Pattern::TypeApplicationAt3 => self.type_application_name(p),
            Pattern::TypeApplicationAt3End => {
                (!self.char_here(p).is_some_and(|(c, _)| is_name(c))).then_some(p)
            }
            Pattern::TypeOperatorAt0 => self.type_operator(p),
            Pattern::TypeOperatorAt1 => self.type_operator_backticks(p),
            Pattern::TypeConstructorAt0 => self.type_constructor(p),
            Pattern::TypeConstructorAt1 => self.type_constructor_operator(p),
            Pattern::TypeSignatureAt17 => self.hash_paren(p),
            Pattern::TypeSignatureAt17End | Pattern::RootAt36End => self.hash_close(p),
            Pattern::TypeSignatureAt18 => self.promoted_open(p, b'('),
            Pattern::TypeSignatureAt19 => self.promoted_open(p, b'['),
            Pattern::TypeSignatureAt19End | Pattern::RootAt39End => self.group_byte(p, b']'),
            Pattern::DoubleColon => self.spaced_double_colon(p),
            Pattern::FunDecl => self.fun_decl(p),
            Pattern::FunDeclEnd => self.fun_decl_end(p),
            Pattern::StartTypeSignatureAt0 => self.signature_line(p),
            Pattern::StartTypeSignatureAt0End => self.signature_end(p, true),
            Pattern::StartTypeSignatureAt1 => self.inline_signature(p),
            Pattern::StartTypeSignatureAt1End => self.signature_end(p, false),

            Pattern::Comma => self.lit(p, b","),
            Pattern::RootAt5 => self.stray_close(p),
            Pattern::RootAt6 => self.unit(p, false),
            Pattern::RootAt8 => self.tuple(p, false),
            Pattern::RootAt10 => self.empty_list(p, false),
            Pattern::RecordWildcard => self.record_wildcard(p),
            Pattern::PrefixOpAt0 => self.prefix_op(p),
            Pattern::ReservedSymbolAt0 => self.reserved_symbol(p),
            Pattern::ReservedSymbolAt1 => self.hashes(p),
            Pattern::ReservedSymbolAt2 => self.as_pattern(p),
            Pattern::ReservedSymbolAt3 => self.prefix_symbol(p),
            Pattern::DataConstructor => self.data_constructor(p),
            Pattern::InfixOpAt0 => self.infix_op(p),
            Pattern::InfixOpAt1 => self.infix_backticks(p),
            Pattern::OverloadedLabelAt0 => self.overloaded_label(p),
            Pattern::Qualifier => self.qualifier(p),
            Pattern::RootAt36 => self.unboxed_open(p),
            Pattern::RootAt37 => self.group_byte(p, b'('),
            Pattern::RootAt39 => self.group_byte(p, b'['),
            Pattern::QuasiQuoteAt0 => self.quasi_quote(p, b"dep", true),
            Pattern::QuasiQuoteAt1 => self.quasi_quote(p, b"t", false),
            Pattern::QuasiQuoteAt2 => self.quasi_quoter(p),
            Pattern::QuasiQuoteAt0End | Pattern::QuasiQuoteAt1End | Pattern::QuasiQuoteAt2End => {
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
        self.capture(1, p, end);
        Some(end)
    }

    /// `\b(?<!')(a|b|…)\b(?!')`, with group 1 around the first of `words`
    /// that matches.
    fn keyword_in(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = words.iter().find_map(|word| self.keyword_at(p, word))?;
        self.capture(1, p, end);
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
        self.capture(1, p, p + 1);
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
    fn capture_paren(
        &mut self,
        groups: [usize; 3],
        open: usize,
        (start, end, close): (usize, usize, usize),
    ) -> usize {
        self.capture(groups[0], open, open + 1);
        self.capture(groups[1], start, end);
        self.capture(groups[2], close, close + 1);
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
