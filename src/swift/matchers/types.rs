//! Matchers: types.

use std::cell::Cell;

use super::super::engine::Lexer;
use super::names::{Ident, is_op_ascii};
use crate::classes::{is_digit, is_letter};
use crate::rules::Chars;

/// What the matchers of this file remember about a line.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    /// The last run of blanks that a lookahead scanned: where it starts and
    /// where it ends.
    /// A search that starts inside the run ends at the same place.
    /// A long run of blanks is then read once.
    blanks: Cell<Option<(usize, usize)>>,
}

/// The blanks between a declaration's keyword and its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Gap {
    /// `\s*`.
    Optional,
    /// `\s+`.
    Required,
}

/// Whether a declaration head takes the blanks after its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tail {
    /// The match ends at the name.
    Bare,
    /// `\s*` after the name.
    Blanks,
}

impl Lexer<'_> {
    /// Whether the character at `p` is one of `set`, with the line feed at
    /// the end of the line.
    fn at_in(&self, p: usize, set: &[u8]) -> bool {
        p <= self.lim() && set.contains(&self.at(p))
    }

    /// `//|/\*`: a comment starts at `p`.
    fn comment_at(&self, p: usize) -> bool {
        self.starts(p, b"//") || self.starts(p, b"/*")
    }

    /// `(?!\G)\bwhere\b`.
    fn where_ahead(&self, p: usize) -> bool {
        !self.at_g(p) && self.keyword(p, b"where").is_some()
    }

    /// `(?=[={}]|(?!\G)\bwhere\b)`.
    /// Several clauses end on it.
    fn clause_stop(&self, p: usize) -> bool {
        self.at_in(p, b"={}") || self.where_ahead(p)
    }

    /// The end of the `\s*` from `p`.
    /// A lookahead asks it at every position of a run of blanks.
    /// The run is therefore read once.
    fn blanks_end(&self, p: usize) -> usize {
        let memo = &self.x.types.blanks;
        if let Some((from, to)) = memo.get()
            && from <= p
            && p <= to
            && to <= self.lim() + 1
        {
            return to;
        }
        let end = self.space_end(p);
        memo.set(Some((p, end)));
        end
    }

    /// `(?=\s*[SET])` where the set contains the line feed.
    /// `\s*` gives back the line feed it took.
    /// A run of blanks that takes the line feed therefore always matches.
    /// Past the line feed, or where none follows the text, the run must end
    /// at a character of the set.
    fn blanks_then(&self, p: usize, set: &[u8]) -> bool {
        let end = self.blanks_end(p);
        (p..end).contains(&self.lim()) || self.at_in(end, set)
    }

    /// Records the groups of an identifier that starts at `p`.
    /// The groups in `whole` each get its text.
    /// `ticks` is the pair of groups that hold the opening and the closing
    /// backtick, empty when there are none.
    pub(in crate::swift) fn cap_ident(
        &mut self,
        p: usize,
        id: Ident,
        whole: &[usize],
        ticks: Option<(usize, usize)>,
    ) {
        for &group in whole {
            self.cap(group, p, id.end);
        }
        if let Some((open, close)) = ticks {
            let width = id.quote();
            self.cap(open, p, p + width);
            self.cap(close, id.end - width, id.end);
        }
    }

    /// `\b(KEYWORD)SEP(name)BLANKS`, the shape of a declaration head, once the
    /// keyword is known to end at `keyword_end`.
    /// Group 1 is the keyword, group 2 the name, and groups 3 and 4 its
    /// backticks.
    fn declaration(&mut self, p: usize, keyword_end: usize, gap: Gap, tail: Tail) -> Option<usize> {
        let name = self.space_end(keyword_end);
        if gap == Gap::Required && name == keyword_end {
            return None;
        }
        let id = self.ident_at(name)?;
        self.cap(1, p, keyword_end);
        self.cap_ident(name, id, &[2], Some((3, 4)));
        Some(match tail {
            Tail::Blanks => self.space_end(id.end),
            Tail::Bare => id.end,
        })
    }

    /// `\b(KEYWORD)\b\s*(name)` for struct and actor.
    fn word_declaration(&mut self, p: usize, keyword: &[u8]) -> Option<usize> {
        let end = self.keyword(p, keyword)?;
        self.declaration(p, end, Gap::Optional, Tail::Bare)
    }

    /// `\b(KEYWORD)\s+(name)\s*`.
    fn spaced_declaration(&mut self, p: usize, keyword: &[u8], tail: Tail) -> Option<usize> {
        let end = self.lit(p, keyword).filter(|_| self.boundary(p))?;
        self.declaration(p, end, Gap::Required, tail)
    }

    /// `<`, for `DeclarationsGenericParameterClause`,
    /// `DeclarationsGenericArgumentClause`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"<")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `` >|(?=[^\&,:<=>`\w\d\s]) ``, for
    /// `DeclarationsGenericParameterClauseEnd`.
    ///
    /// The line feed is a `\s`.
    /// The second branch therefore never matches at the end of the line.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if let Some(end) = self.lit(p, b">") {
            self.cap(0, p, end);
            return Some(end);
        }
        if p >= self.lim() || self.at_in(p, b"&,:<=>`") {
            return None;
        }
        let (c, _) = self.char_at(p)?;
        if self.word_at(p).is_some() || is_digit(c) || self.space_at(p) > 0 {
            return None;
        }
        self.cap(0, p, p);
        Some(p)
    }

    /// `\b(where)\b\s*`, for `DeclarationsGenericWhereClause`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.keyword(p, b"where")?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `(?!\G)$|(?=[\n;>{}]|//|/\*)`, for `DeclarationsGenericWhereClauseEnd`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let stop =
            (!self.at_g(p) && self.at_eol(p)) || self.at_in(p, b"\n;>{}") || self.comment_at(p);
        stop.then_some(p)
    }

    /// `\G|,\s*`, for `DeclarationsGenericWhereClauseRequirementList`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at_g(p) {
            return Some(p);
        }
        let comma = self.lit(p, b",")?;
        Some(self.space_end(comma))
    }

    /// `(?=[\n,;>{}]|//|/\*)`, for
    /// `DeclarationsGenericWhereClauseRequirementListEnd`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_in(p, b"\n,;>{}") || self.comment_at(p)).then_some(p)
    }

    /// `>|(?=[]){}])`, for `DeclarationsGenericArgumentClauseEnd`.
    pub(in crate::swift) fn rx_declarations_generic_argument_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if let Some(end) = self.lit(p, b">") {
            self.cap(0, p, end);
            return Some(end);
        }
        if self.at_in(p, b"]){}") {
            self.cap(0, p, p);
            return Some(p);
        }
        None
    }

    /// `` \b(?:(async)\s+)?(let|var)\b\s+(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\
    /// k<q>)\s*: ``, for `DeclarationsTypedVariableDeclaration`.
    ///
    /// Without the `async` branch the keyword would have to start at `p`,
    /// where `async` stands.
    /// One path therefore decides.
    pub(in crate::swift) fn rx_declarations_typed_variable_declaration(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let mut at = p;
        let mut asynchronous = None;
        if let Some(end) = self.lit(p, b"async") {
            let blanks = self.space_end(end);
            if blanks > end {
                asynchronous = Some((p, end));
                at = blanks;
            }
        }
        let end = self
            .keyword_here(at, b"let")
            .or_else(|| self.keyword_here(at, b"var"))?;
        let blanks = self.space_end(end);
        if blanks == end {
            return None;
        }
        let id = self.ident_at(blanks)?;
        let colon = self.space_end(id.end);
        if !self.at_in(colon, b":") || colon >= self.lim() {
            return None;
        }
        if let Some((a, b)) = asynchronous {
            self.cap(1, a, b);
        }
        self.cap(2, at, end);
        Some(colon + 1)
    }

    /// The end of `text` at `i` when no word character follows it: the `\b`
    /// after a keyword whose start has already been checked.
    fn keyword_here(&self, i: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(i, text)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `(?=$|[={])`, for `DeclarationsTypedVariableDeclarationEnd`.
    pub(in crate::swift) fn rx_declarations_typed_variable_declaration_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_eol(p) || self.at_in(p, b"={")).then_some(p)
    }

    /// `\b(?:BitwiseShift|Assignment|RangeFormation|Casting|Addition|NilCoalescing|Comparison|LogicalConjunction|LogicalDisjunction|Default|Ternary|Multiplication|FunctionArrow)Precedence\b`,
    /// for `DeclarationsTypesPrecedencegroupAt0`.
    ///
    /// No name is a prefix of another, and `\b` ends the word.
    /// The whole word must therefore be one of the names.
    pub(in crate::swift) fn rx_declarations_types_precedencegroup_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_in(
            p,
            &[
                b"BitwiseShiftPrecedence",
                b"AssignmentPrecedence",
                b"RangeFormationPrecedence",
                b"CastingPrecedence",
                b"AdditionPrecedence",
                b"NilCoalescingPrecedence",
                b"ComparisonPrecedence",
                b"LogicalConjunctionPrecedence",
                b"LogicalDisjunctionPrecedence",
                b"DefaultPrecedence",
                b"TernaryPrecedence",
                b"MultiplicationPrecedence",
                b"FunctionArrowPrecedence",
            ],
        )
    }

    /// `` \b(protocol)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)) ``, for
    /// `DeclarationsProtocolProtocol`.
    pub(in crate::swift) fn rx_declarations_protocol_protocol(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.spaced_declaration(p, b"protocol", Tail::Bare)
    }

    /// `(?<=})`, for `DeclarationsProtocolProtocolEnd`,
    /// `DeclarationsTypeAt0ClassEnd`, `DeclarationsTypeAt0StructEnd`,
    /// `DeclarationsTypeAt0ActorEnd`, `DeclarationsTypeEnumEnumEnd`,
    /// `DeclarationsExtensionExtensionEnd`.
    pub(in crate::swift) fn rx_declarations_protocol_protocol_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.prev_is(p, b'}').then_some(p)
    }

    /// `(:)(?=\s*\{)|(:)\s*`, for `DeclarationsInheritanceClause`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let colon = self.lit(p, b":")?;
        let blanks = self.space_end(colon);
        if self.at_in(blanks, b"{") && blanks < self.lim() {
            self.cap(1, p, colon);
            return Some(colon);
        }
        self.cap(2, p, colon);
        Some(blanks)
    }

    /// `(?!\G)$|(?=[={}]|(?!\G)\bwhere\b)`, for
    /// `DeclarationsInheritanceClauseEnd`,
    /// `DeclarationsInheritanceClauseAt1End`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        ((!self.at_g(p) && self.at_eol(p)) || self.clause_stop(p)).then_some(p)
    }

    /// `\bclass\b`, for `DeclarationsInheritanceClauseAt0`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.keyword(p, b"class")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `(?=[={}]|(?!\G)\bwhere\b)`, for `DeclarationsInheritanceClauseAt0End`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_at_0_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.clause_stop(p).then_some(p)
    }

    /// `,\s*`, for `DeclarationsInheritanceClauseMoreTypes`,
    /// `DeclarationsTypeEnumMoreCases`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_more_types(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let comma = self.lit(p, b",")?;
        Some(self.space_end(comma))
    }

    /// `(?!\G)(?!/[*/])|(?=[,={}]|(?!\G)\bwhere\b)`, for
    /// `DeclarationsInheritanceClauseMoreTypesEnd`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_more_types_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        ((!self.at_g(p) && !self.comment_at(p)) || self.at_in(p, b",") || self.clause_stop(p))
            .then_some(p)
    }

    /// `` (?=[_`\p{L}]) ``, for `DeclarationsInheritanceClauseInheritedType`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_inherited_type(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        let (c, _) = self.char_at(p)?;
        (c == '_' || c == '`' || is_letter(c)).then_some(p)
    }

    /// `` ((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s* ``, for
    /// `DeclarationsTypeIdentifier`, `DeclarationsTypeEnumEnumCase`.
    pub(in crate::swift) fn rx_declarations_type_identifier(&mut self, p: usize) -> Option<usize> {
        let id = self.ident_at(p)?;
        self.cap_ident(p, id, &[1], Some((2, 3)));
        Some(self.space_end(id.end))
    }

    /// `(?!<)`, for `DeclarationsTypeIdentifierEnd`.
    pub(in crate::swift) fn rx_declarations_type_identifier_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (!self.at_in(p, b"<")).then_some(p)
    }

    /// `(?=<)`, for `DeclarationsTypeIdentifierAt0`.
    pub(in crate::swift) fn rx_declarations_type_identifier_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.at_in(p, b"<").then_some(p)
    }

    /// `(?<![-!%\&*+./<=>^|~])(~)(?![-!%\&*+./<=>^|~])`, for
    /// `DeclarationsTypeOperatorsAt1`.
    pub(in crate::swift) fn rx_declarations_type_operators_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lone_operator(p, b"~")
    }

    /// `(?<![OPS])(TEXT)(?![OPS])` with the groups 1: the operator `text`
    /// stands alone.
    fn lone_operator(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        let before = p.checked_sub(1).and_then(|i| self.s.get(i));
        if before.is_some_and(|&b| is_op_ascii(b)) || is_op_ascii(self.at(end)) {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `\G`, for `DeclarationsInheritanceClauseAt1`,
    /// `DeclarationsGenericParameterClauseAt6At0`.
    pub(in crate::swift) fn rx_declarations_inheritance_clause_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.at_g(p).then_some(p)
    }

    /// `$|(?=;|//|/\*|})`, for `DeclarationsProtocolProtocolMethodEnd`,
    /// `DeclarationsProtocolProtocolInitializerEnd`.
    pub(in crate::swift) fn rx_declarations_protocol_protocol_method_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_eol(p) || self.at_in(p, b";}") || self.comment_at(p)).then_some(p)
    }

    /// `` \b(class(?!\s+(?:func|var|let)\b))\b\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\
    /// p{M}]*(\k<q>)) ``, for `DeclarationsTypeAt0Class`.
    pub(in crate::swift) fn rx_declarations_type_at_0_class(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"class")?;
        let next = self.space_end(end);
        if next > end
            && [b"func".as_slice(), b"var", b"let"]
                .iter()
                .any(|word| self.keyword_here(next, word).is_some())
        {
            return None;
        }
        self.declaration(p, end, Gap::Optional, Tail::Bare)
    }

    /// `` \b(struct)\b\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)) ``, for
    /// `DeclarationsTypeAt0Struct`.
    pub(in crate::swift) fn rx_declarations_type_at_0_struct(&mut self, p: usize) -> Option<usize> {
        self.word_declaration(p, b"struct")
    }

    /// `` \b(actor)\b\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)) ``, for
    /// `DeclarationsTypeAt0Actor`.
    pub(in crate::swift) fn rx_declarations_type_at_0_actor(&mut self, p: usize) -> Option<usize> {
        self.word_declaration(p, b"actor")
    }

    /// `` \b(enum)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)) ``, for
    /// `DeclarationsTypeEnumEnum`.
    pub(in crate::swift) fn rx_declarations_type_enum_enum(&mut self, p: usize) -> Option<usize> {
        self.spaced_declaration(p, b"enum", Tail::Bare)
    }

    /// `\b(case)\b\s*`, for `DeclarationsTypeEnumEnumCaseClause`.
    pub(in crate::swift) fn rx_declarations_type_enum_enum_case_clause(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.keyword(p, b"case")?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `(?=[;}])|(?!\G)(?!/[*/])(?=[^,\s])`, for
    /// `DeclarationsTypeEnumEnumCaseClauseEnd`.
    pub(in crate::swift) fn rx_declarations_type_enum_enum_case_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_in(p, b";}") || self.case_continues(p)).then_some(p)
    }

    /// `(?!\G)(?!/[*/])(?=[^,\s])`: something other than a comma or a blank
    /// follows.
    /// The line feed is a blank.
    fn case_continues(&self, p: usize) -> bool {
        !self.at_g(p)
            && !self.comment_at(p)
            && p < self.lim()
            && self.at(p) != b','
            && self.space_at(p) == 0
    }

    /// `(?<=\))|(?![(=])`, for `DeclarationsTypeEnumEnumCaseEnd`.
    pub(in crate::swift) fn rx_declarations_type_enum_enum_case_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.prev_is(p, b')') || !self.at_in(p, b"(=")).then_some(p)
    }

    /// `\G\(`, for `DeclarationsTypeEnumAssociatedValues`.
    pub(in crate::swift) fn rx_declarations_type_enum_associated_values(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let end = self.lit(p, b"(")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// ``
    /// (?:(_)|((?<q1>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*\k<q1>))\s+(((?<q2>`?)[_\
    /// p{L}][_\p{L}\p{N}\p{M}]*\k<q2>))\s*(:)
    /// ``, for `DeclarationsTypeEnumAssociatedValuesAt1`.
    ///
    /// A lone `_` is the first branch.
    /// When it fails, the second branch reads the same text and fails too.
    /// The identifier therefore decides which group holds the label.
    pub(in crate::swift) fn rx_declarations_type_enum_associated_values_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let label = self.ident_at(p)?;
        let name = self.space_end(label.end);
        if name == label.end {
            return None;
        }
        let local = self.ident_at(name)?;
        let colon = self.space_end(local.end);
        if colon >= self.lim() || self.at(colon) != b':' {
            return None;
        }
        let underscore = label.end == p + 1 && self.at(p) == b'_';
        self.cap(if underscore { 1 } else { 2 }, p, label.end);
        self.cap(5, name, local.end);
        self.cap(7, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(?=[]),])`, for `DeclarationsTypeEnumAssociatedValuesAt1End`,
    /// `DeclarationsTypeEnumAssociatedValuesAt2End`,
    /// `DeclarationsTypeEnumAssociatedValuesAt3End`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesExpressionElementListAt1End`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesExpressionElementListAt2End`.
    pub(in crate::swift) fn rx_declarations_type_enum_associated_values_at_1_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.at_in(p, b"]),").then_some(p)
    }

    /// `` (((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*\k<q>))\s*(:) ``, for
    /// `DeclarationsTypeEnumAssociatedValuesAt2`.
    pub(in crate::swift) fn rx_declarations_type_enum_associated_values_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let id = self.ident_at(p)?;
        let colon = self.space_end(id.end);
        if colon >= self.lim() || self.at(colon) != b':' {
            return None;
        }
        self.cap_ident(p, id, &[1, 2], None);
        self.cap(4, colon, colon + 1);
        Some(colon + 1)
    }

    /// `(?![]),])(?=\S)`, for `DeclarationsTypeEnumAssociatedValuesAt3`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesExpressionElementListAt2`.
    pub(in crate::swift) fn rx_declarations_type_enum_associated_values_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (p < self.lim() && !self.at_in(p, b"]),") && self.space_at(p) == 0).then_some(p)
    }

    /// `(=)\s*`, for `DeclarationsTypeEnumRawValueAssignment`,
    /// `DeclarationsTypealiasAssignment`.
    pub(in crate::swift) fn rx_declarations_type_enum_raw_value_assignment(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"=")?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `(?!\G)(?!/[*/])(?=[;}[^,\s]])`, for `DeclarationsTypeEnumMoreCasesEnd`.
    ///
    /// The class is `;`, `}` or anything but a comma or a blank.
    /// The third member already holds the first two.
    /// The class is therefore the case clause's lookahead.
    pub(in crate::swift) fn rx_declarations_type_enum_more_cases_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.case_continues(p).then_some(p)
    }

    /// `\b(extension)\s+`, for `DeclarationsExtensionExtension`.
    pub(in crate::swift) fn rx_declarations_extension_extension(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"extension").filter(|_| self.boundary(p))?;
        let blanks = self.space_end(end);
        if blanks == end {
            return None;
        }
        self.cap(1, p, end);
        Some(blanks)
    }

    /// `\G(?!\s*[\n:{])`, for `DeclarationsExtensionExtensionAt0`.
    pub(in crate::swift) fn rx_declarations_extension_extension_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_g(p) && !self.blanks_then(p, b":{")).then_some(p)
    }

    /// `(?=\s*[\n:{])|(?!\G)(?=\s*where\b)`, for
    /// `DeclarationsExtensionExtensionAt0End`.
    pub(in crate::swift) fn rx_declarations_extension_extension_at_0_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let found = self.blanks_then(p, b":{")
            || (!self.at_g(p) && self.keyword_here(self.blanks_end(p), b"where").is_some());
        found.then_some(p)
    }

    /// `` \b(typealias)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s* ``,
    /// for `DeclarationsTypealias`.
    pub(in crate::swift) fn rx_declarations_typealias(&mut self, p: usize) -> Option<usize> {
        self.spaced_declaration(p, b"typealias", Tail::Blanks)
    }

    /// `(?!\G)$|(?=;|//|/\*|$)`, for `DeclarationsTypealiasEnd`,
    /// `DeclarationsTypealiasAssignmentEnd`.
    ///
    /// The first branch is part of the second.
    pub(in crate::swift) fn rx_declarations_typealias_end(&mut self, p: usize) -> Option<usize> {
        (self.at_eol(p) || self.at_in(p, b";") || self.comment_at(p)).then_some(p)
    }

    /// `\G(?=<)`, for `DeclarationsTypealiasAt0`.
    pub(in crate::swift) fn rx_declarations_typealias_at_0(&mut self, p: usize) -> Option<usize> {
        (self.at_g(p) && self.at_in(p, b"<")).then_some(p)
    }

    /// `` \b(associatedtype)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*
    /// ``, for `DeclarationsProtocolAssociatedType`.
    pub(in crate::swift) fn rx_declarations_protocol_associated_type(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.spaced_declaration(p, b"associatedtype", Tail::Blanks)
    }

    /// `(?!\G)$|(?=[;}]|$)`, for `DeclarationsProtocolAssociatedTypeEnd`.
    ///
    /// The first branch is part of the second.
    pub(in crate::swift) fn rx_declarations_protocol_associated_type_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_eol(p) || self.at_in(p, b";}")).then_some(p)
    }

    /// `(?<![-!%\&*+./<=>^|~])(==)(?![-!%\&*+./<=>^|~])`, for
    /// `DeclarationsGenericWhereClauseRequirementListAt3`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lone_operator(p, b"==")
    }

    /// `(?=\s*[\n,;>{}]|//|/\*)`, for
    /// `DeclarationsGenericWhereClauseRequirementListAt3End`,
    /// `DeclarationsGenericWhereClauseRequirementListAt4End`,
    /// `DeclarationsGenericWhereClauseRequirementListAt4At0End`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list_at_3_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.blanks_then(p, b",;>{}") || self.comment_at(p)).then_some(p)
    }

    /// `(?<![-!%\&*+./<=>^|~])(:)(?![-!%\&*+./<=>^|~])`, for
    /// `DeclarationsGenericWhereClauseRequirementListAt4`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list_at_4(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lone_operator(p, b":")
    }

    /// `\G\s*`, for `DeclarationsGenericWhereClauseRequirementListAt4At0`.
    pub(in crate::swift) fn rx_declarations_generic_where_clause_requirement_list_at_4_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.at_g(p).then(|| self.space_end(p))
    }

    /// `\blet\b`, for `DeclarationsGenericParameterClauseAt2`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"let")
    }

    /// `\beach\b`, for `DeclarationsGenericParameterClauseAt3`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"each")
    }

    /// `\b((?!\d)\w[\w\d]*)\b`, for `DeclarationsGenericParameterClauseAt4`.
    ///
    /// Giving characters back would end the run inside a word, where `\b`
    /// fails.
    /// The whole run is therefore the answer.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_4(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let (c, _) = self.char_at(p)?;
        if is_digit(c) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        self.cap(1, p, end);
        Some(end)
    }

    /// `,`, for `DeclarationsGenericParameterClauseAt5`, `Nested1At2`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_5(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b",")
    }

    /// `(:)\s*`, for `DeclarationsGenericParameterClauseAt6`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_6(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b":")?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `(?=[,>]|(?!\G)\bwhere\b)`, for
    /// `DeclarationsGenericParameterClauseAt6End`,
    /// `DeclarationsGenericParameterClauseAt6At0End`.
    pub(in crate::swift) fn rx_declarations_generic_parameter_clause_at_6_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at_in(p, b",>") || self.where_ahead(p)).then_some(p)
    }
}
