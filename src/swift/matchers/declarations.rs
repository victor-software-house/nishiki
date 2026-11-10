//! Matchers: declarations.

use std::cell::Cell;

use super::super::engine::Lexer;
use super::names::{Dots, is_op_ascii, is_op_continue, is_op_head};
use crate::classes::{is_digit, is_letter, is_mark, is_number, is_space};
use crate::rules::Chars;

/// What the matchers of this file remember about a line: the last run of
/// white space, as `(from, to)`.
/// A scan that starts inside a run ends where the run ends.
/// A line of thousands of blanks is then walked once, not from every start
/// the engine tries.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    blank: Cell<(usize, usize)>,
}

/// The declaration keywords an import may name: `import struct A.B`.
const IMPORT_KEYWORDS: [&[u8]; 8] = [
    b"typealias",
    b"struct",
    b"class",
    b"actor",
    b"enum",
    b"protocol",
    b"var",
    b"func",
];

impl Lexer<'_> {
    /// The end of the `\s*` from `i`, remembering the run.
    pub(in crate::swift) fn blank_end(&self, i: usize) -> usize {
        let (lo, hi) = self.x.declarations.blank.get();
        if lo <= i && i < hi {
            return hi;
        }
        let mut end = i;
        loop {
            let width = self.space_at(end);
            if width == 0 {
                break;
            }
            end += width;
        }
        self.spend(end - i);
        self.x.declarations.blank.set((i, end));
        end
    }

    /// The width of the operator head character at `i`.
    pub(in crate::swift) fn op_head_at(&self, i: usize) -> Option<usize> {
        let (c, end) = self.char_at(i)?;
        is_op_head(c).then_some(end - i)
    }

    /// The width of the operator head, continuation or dot character at `i`.
    pub(in crate::swift) fn op_elem_at(&self, i: usize) -> Option<usize> {
        let (c, end) = self.char_at(i)?;
        (is_op_head(c) || is_op_continue(c) || c == '.').then_some(end - i)
    }

    /// The end of the run of operator head and continuation characters from
    /// `i`, with dots too when `dots` takes them.
    pub(in crate::swift) fn op_run_end(&self, from: usize, dots: Dots) -> usize {
        let mut i = from;
        while let Some((c, end)) = self.char_at(i)
            && (is_op_head(c) || is_op_continue(c) || (dots == Dots::Taken && c == '.'))
        {
            i = end;
        }
        self.spend(i - from);
        i
    }

    /// The start of the character before `i`.
    fn prev_boundary(&self, i: usize) -> usize {
        let mut k = i.saturating_sub(1);
        while k > 0 && self.s.get(k).is_some_and(|b| b & 0xc0 == 0x80) {
            k -= 1;
        }
        k
    }

    /// An operator run `(head (head|continue)*)` or `(. (head|continue|.)+)`
    /// that ends where `after` accepts.
    /// The run is greedy and gives back one character at a time, down to
    /// `min`, the end of its first element.
    /// It returns where the operator ends and what `after` made of that end.
    fn operator_then(
        &self,
        min: usize,
        max: usize,
        after: impl Fn(&Self, usize) -> Option<usize>,
    ) -> Option<(usize, usize)> {
        let mut end = max;
        loop {
            if let Some(done) = after(self, end) {
                return Some((end, done));
            }
            if end <= min {
                return None;
            }
            end = self.prev_boundary(end);
            self.spend(1);
        }
    }

    /// The `\G` of a lookbehind `(?<=\G|\.)`.
    fn after_anchor_or_dot(&self, p: usize) -> bool {
        (self.g && p == self.pos) || self.prev_is(p, b'.')
    }

    /// `;|$|//|/\*`.
    fn at_statement_end(&self, i: usize) -> bool {
        self.at(i) == b';'
            || self.at_eol(i)
            || (self.at(i) == b'/' && matches!(self.at(i + 1), b'/' | b'*'))
    }

    /// The `\b(word)\s+` that several declarations start with.
    fn word_and_blank(&self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        let next = self.blank_end(end);
        (next > end).then_some(next)
    }

    /// The `(?=[(<])` after `\s*`.
    fn open_paren_or_angle(&self, i: usize) -> Option<usize> {
        let next = self.blank_end(i);
        matches!(self.at(next), b'(' | b'<').then_some(next)
    }

    /// `(?<!\.)\b(word)\s*(?=[(<])`.
    fn declaration_word(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.keyword(p, word)?;
        let next = self.open_paren_or_angle(end)?;
        self.capture(1, p, end);
        Some(next)
    }

    /// `(?<![-!%\&*+./<=>^|~])` before `p`.
    fn not_after_op_ascii(&self, p: usize) -> bool {
        !p.checked_sub(1)
            .and_then(|k| self.s.get(k))
            .is_some_and(|&b| is_op_ascii(b))
    }

    /// `(?![-!%\&*+./<=>^|~])` at `p`.
    fn not_before_op_ascii(&self, p: usize) -> bool {
        !is_op_ascii(self.at(p))
    }

    /// `(?<![set])(text)(?![set])`, the operators that must stand alone.
    fn lone_decl_operator(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        if !(self.not_after_op_ascii(p) && self.not_before_op_ascii(end)) {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?<=\G|\.)` then `\$[0-9]+`.
    fn dollar_number(&self, p: usize) -> Option<usize> {
        if !self.after_anchor_or_dot(p) || self.at(p) != b'$' {
            return None;
        }
        let mut i = p + 1;
        while self.at(i).is_ascii_digit() && i < self.lim() {
            i += 1;
        }
        self.spend(i - p);
        (i > p + 1).then_some(i)
    }

    /// `(?:left|right|none)`-style words after `\s*:\s*`, with `\b` after.
    fn colon_word(&self, p: usize, words: &[&[u8]]) -> Option<(usize, usize)> {
        let colon = self.blank_end(p);
        if self.at(colon) != b':' {
            return None;
        }
        let from = self.blank_end(colon + 1);
        let end = self.word_in(from, words)?;
        Some((from, end))
    }

    /// `\b(word)\b(?:\s*:\s*(a|b)\b)?`.
    fn word_with_value(&mut self, p: usize, word: &[u8], values: &[&[u8]]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.capture(1, p, end);
        match self.colon_word(end, values) {
            Some((from, to)) => {
                self.capture(2, from, to);
                Some(to)
            }
            None => Some(end),
        }
    }

    /// `(?<=\s)` before `p`.
    fn after_space(&self, p: usize) -> bool {
        self.char_before(p).is_some_and(is_space)
    }

    /// `[(\[_\p{L}\d\p{N}\p{M}]` at `i`.
    fn starts_argument(&self, i: usize) -> bool {
        self.char_at(i).is_some_and(|(c, _)| {
            matches!(c, '(' | '[' | '_')
                || is_letter(c)
                || is_digit(c)
                || is_number(c)
                || is_mark(c)
        })
    }

    /// The rest of the operator declaration after its optional fixity word:
    /// `\b(operator)\s+(name)\s*`.
    /// The name's repetition is possessive.
    /// It takes every operator character, dots included, and never gives one
    /// back.
    fn operator_rest(&mut self, p: usize, fix: Option<(usize, usize)>) -> Option<usize> {
        let word_end = self.keyword(p, b"operator")?;
        let name = self.blank_end(word_end);
        if name == word_end {
            return None;
        }
        let (end, alt_one) = if let Some(width) = self.op_head_at(name) {
            (self.op_run_end(name + width, Dots::Taken), true)
        } else if self.at(name) == b'.' {
            let end = self.op_run_end(name + 1, Dots::Taken);
            if end == name + 1 {
                return None;
            }
            (end, false)
        } else {
            return None;
        };
        if let Some((from, to)) = fix {
            self.capture(1, from, to);
        }
        self.capture(2, p, word_end);
        self.capture(3, name, end);
        if alt_one {
            self.capture(4, name, end);
        }
        Some(self.blank_end(end))
    }
}

impl Lexer<'_> {
    /// Pattern ids: `DeclarationsFunction`,
    /// `DeclarationsProtocolProtocolMethod`.
    ///
    /// ```text
    /// \b(func)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)|(?:((?<oph>[-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷‖‗†-‧‰-‾⁁-⁓⁕-⁞←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰])(\g<oph>|(?<opc>[̀-ͯ᷀-᷿⃐-⃿︀-️︠-︯\x{E0100}-\x{E01EF}]))*)|(\.(\g<oph>|\g<opc>|\.)+)))\s*(?=[(<])
    /// ```
    ///
    /// No operator starts with an identifier character.
    /// A failed lookahead therefore ends the match.
    ///
    /// An operator gives back one character at a time.
    /// Before the end of the run, only a `<` can follow it, since the next
    /// character is an operator one.
    pub(in crate::swift) fn rx_declarations_function(&mut self, p: usize) -> Option<usize> {
        let keyword = self.keyword(p, b"func")?;
        self.capture(1, p, keyword);
        let name = self.blank_end(keyword);
        if name == keyword {
            return None;
        }
        if let Some(id) = self.ident_at(name) {
            let next = self.open_paren_or_angle(id.end)?;
            self.capture(2, name, id.end);
            self.capture_quotes(3, name, id);
            return Some(next);
        }
        let (min, max) = if let Some(width) = self.op_head_at(name) {
            (name + width, self.op_run_end(name + width, Dots::Refused))
        } else if self.at(name) == b'.' {
            let max = self.op_run_end(name + 1, Dots::Taken);
            let width = self.op_elem_at(name + 1)?;
            (name + 1 + width, max)
        } else {
            return None;
        };
        let (end, next) = self.operator_then(min, max, Self::open_paren_or_angle)?;
        self.capture(2, name, end);
        Some(next)
    }

    /// Pattern ids: `DeclarationsFunctionEnd`,
    /// `DeclarationsFunctionInitializerEnd`,
    /// `DeclarationsFunctionSubscriptEnd`.
    ///
    /// ```text
    /// (?<=})|$
    /// ```
    pub(in crate::swift) fn rx_declarations_function_end(&mut self, p: usize) -> Option<usize> {
        (self.prev_is(p, b'}') || self.at_eol(p)).then_some(p)
    }

    /// Pattern ids: `DeclarationsFunctionInitializer`,
    /// `DeclarationsProtocolProtocolInitializer`.
    ///
    /// ```text
    /// (?<!\.)\b(init[!?]*)\s*(?=[(<])
    /// ```
    pub(in crate::swift) fn rx_declarations_function_initializer(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.prev_is(p, b'.') || !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, b"init")?;
        let mut end = word;
        while matches!(self.at(end), b'!' | b'?') {
            end += 1;
        }
        let next = self.open_paren_or_angle(end)?;
        self.capture(1, p, end);
        Some(next)
    }

    /// Pattern ids: `DeclarationsParameterClause`.
    ///
    /// ```text
    /// (\()
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_clause(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'(').then(|| {
            self.capture(1, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsParameterClauseEnd`.
    ///
    /// ```text
    /// (\))(?:\s*(async)\b)?
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_clause_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b')' {
            return None;
        }
        self.capture(1, p, p + 1);
        let word = self.blank_end(p + 1);
        if let Some(end) = self.keyword(word, b"async") {
            self.capture(2, word, end);
            return Some(end);
        }
        Some(p + 1)
    }

    /// Pattern ids: `DeclarationsParameterListAt0`.
    ///
    /// ```text
    /// ((?<q1>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q1>))\s+((?<q2>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q2>))(?=\s*:)
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_list_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let first = self.ident_at(p)?;
        let from = self.blank_end(first.end);
        if from == first.end {
            return None;
        }
        let second = self.ident_at(from)?;
        let colon = self.blank_end(second.end);
        if self.at(colon) != b':' {
            return None;
        }
        self.capture(1, p, first.end);
        self.capture_quotes(2, p, first);
        self.capture(4, from, second.end);
        self.capture_quotes(5, from, second);
        Some(second.end)
    }

    /// Pattern ids: `DeclarationsParameterListAt1`.
    ///
    /// ```text
    /// (((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)))(?=\s*:)
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_list_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let id = self.ident_at(p)?;
        let colon = self.blank_end(id.end);
        if self.at(colon) != b':' {
            return None;
        }
        self.capture(1, p, id.end);
        self.capture(2, p, id.end);
        self.capture_quotes(3, p, id);
        Some(id.end)
    }

    /// Pattern ids: `DeclarationsParameterListAt2`.
    ///
    /// ```text
    /// :\s*(?!\s)
    /// ```
    ///
    /// `\s*` takes every blank.
    /// `(?!\s)` therefore holds where it stops.
    pub(in crate::swift) fn rx_declarations_parameter_list_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b':' {
            return None;
        }
        Some(self.blank_end(p + 1))
    }

    /// Pattern ids: `DeclarationsParameterListAt2End`,
    /// `DeclarationsParameterListAt2At3End`.
    ///
    /// ```text
    /// (?=[),])
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_list_at_2_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        matches!(self.at(p), b')' | b',').then_some(p)
    }

    /// Pattern ids: `DeclarationsParameterListAt2At0`,
    /// `DeclarationsFunctionResultAt0`.
    ///
    /// ```text
    /// \bsending\b
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_list_at_2_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"sending")
    }

    /// Pattern ids: `CodeBlock`, `DeclarationsPrecedencegroupAt0`,
    /// `DeclarationsProtocolProtocolAt3`,
    /// `DeclarationsProtocolProtocolMethodAt6`, `DeclarationsTypeAt0ClassAt4`,
    /// `DeclarationsTypeAt0StructAt4`, `DeclarationsTypeAt0ActorAt4`,
    /// `DeclarationsTypeEnumEnumAt4`, `DeclarationsExtensionExtensionAt4`,
    /// `DeclarationsProtocolProtocolInitializerAt5`.
    ///
    /// ```text
    /// \{
    /// ```
    pub(in crate::swift) fn rx_code_block(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'{').then(|| {
            self.capture(0, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `CodeBlockEnd`, `DeclarationsPrecedencegroupAt0End`,
    /// `DeclarationsProtocolProtocolAt3End`,
    /// `DeclarationsProtocolProtocolMethodAt6End`,
    /// `DeclarationsTypeAt0ClassAt4End`, `DeclarationsTypeAt0StructAt4End`,
    /// `DeclarationsTypeAt0ActorAt4End`, `DeclarationsTypeEnumEnumAt4End`,
    /// `DeclarationsExtensionExtensionAt4End`,
    /// `DeclarationsProtocolProtocolInitializerAt5End`.
    ///
    /// ```text
    /// }
    /// ```
    pub(in crate::swift) fn rx_code_block_end(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'}').then(|| {
            self.capture(0, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsFunctionSubscript`.
    ///
    /// ```text
    /// (?<!\.)\b(subscript)\s*(?=[(<])
    /// ```
    pub(in crate::swift) fn rx_declarations_function_subscript(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.declaration_word(p, b"subscript")
    }

    /// Pattern ids: `DeclarationsFunctionResult`.
    ///
    /// ```text
    /// (?<![-!%\&*+./<=>^|~])(->)(?![-!%\&*+./<=>^|~])\s*
    /// ```
    pub(in crate::swift) fn rx_declarations_function_result(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_op_ascii(p) {
            return None;
        }
        let arrow = self.lit(p, b"->")?;
        if !self.not_before_op_ascii(arrow) {
            return None;
        }
        self.capture(1, p, arrow);
        Some(self.blank_end(arrow))
    }

    /// Pattern ids: `DeclarationsFunctionResultEnd`.
    ///
    /// ```text
    /// (?!\G)(?=\{|\bwhere\b|[;=])|$
    /// ```
    pub(in crate::swift) fn rx_declarations_function_result_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !(self.g && p == self.pos)
            && (matches!(self.at(p), b'{' | b';' | b'=') || self.keyword(p, b"where").is_some())
        {
            return Some(p);
        }
        self.at_eol(p).then_some(p)
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt3`.
    ///
    /// ```text
    /// \basync\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"async")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt4`.
    ///
    /// ```text
    /// \b(?:|re)throws\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_4(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"throws")
            .or_else(|| self.keyword(p, b"rethrows"))
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt5`.
    ///
    /// ```text
    /// \bsome\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_5(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"some")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt6`.
    ///
    /// ```text
    /// \bany\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_6(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"any")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt7`.
    ///
    /// ```text
    /// \b(?:repeat|each)\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_7(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_in(p, &[b"repeat", b"each"])
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt8`.
    ///
    /// ```text
    /// \b(?:inout|isolated|borrowing|consuming)\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_8(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_in(p, &[b"inout", b"isolated", b"borrowing", b"consuming"])
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt9`.
    ///
    /// ```text
    /// \bnonisolated(?:\(nonsending\)|\b)
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_9(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, b"nonisolated")?;
        if let Some(end) = self.lit(word, b"(nonsending)") {
            return Some(end);
        }
        self.word_at(word).is_none().then_some(word)
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt10`.
    ///
    /// ```text
    /// \bSelf\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_10(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"Self")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt11`.
    ///
    /// ```text
    /// (?<![-!%\&*+./<=>^|~])(->)(?![-!%\&*+./<=>^|~])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_11(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lone_decl_operator(p, b"->")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt12`,
    /// `DeclarationsTypeOperatorsAt0`.
    ///
    /// ```text
    /// (?<![-!%\&*+./<=>^|~])(&)(?![-!%\&*+./<=>^|~])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_12(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lone_decl_operator(p, b"&")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt13`.
    ///
    /// ```text
    /// [!?]
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_13(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        matches!(self.at(p), b'!' | b'?').then_some(p + 1)
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt14`.
    ///
    /// ```text
    /// \.\.\.
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_14(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.lit(p, b"...")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt15`.
    ///
    /// ```text
    /// \bprotocol\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_15(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.keyword(p, b"protocol")
    }

    /// Pattern ids: `DeclarationsAvailableTypesAt16`.
    ///
    /// ```text
    /// (?<=\.)(?:Protocol|Type)\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_at_16(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let end = self.lit(p, b"Protocol").or_else(|| self.lit(p, b"Type"))?;
        self.word_at(end).is_none().then_some(end)
    }

    /// Pattern ids: `DeclarationsAvailableTypesTupleTypeEnd`.
    ///
    /// ```text
    /// \)|(?=[]>{}])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_tuple_type_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = if self.at(p) == b')' {
            p + 1
        } else if matches!(self.at(p), b']' | b'>' | b'{' | b'}') {
            p
        } else {
            return None;
        };
        self.capture(0, p, end);
        Some(end)
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionType`.
    ///
    /// ```text
    /// \[
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b'[').then(|| {
            self.capture(0, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionTypeEnd`.
    ///
    /// ```text
    /// ]|(?=[)>{}])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = if self.at(p) == b']' {
            p + 1
        } else if matches!(self.at(p), b')' | b'>' | b'{' | b'}') {
            p
        } else {
            return None;
        };
        self.capture(0, p, end);
        Some(end)
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionTypeAt2`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAt18`.
    ///
    /// ```text
    /// \b_\b
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b'_' && self.boundary(p) && self.boundary(p + 1)).then_some(p + 1)
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionTypeAt3`,
    /// `KeywordsAt3`.
    ///
    /// ```text
    /// (?<=\s)\bof\b(?=\s+[(\[_\p{L}\d\p{N}\p{M}])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.after_space(p) {
            return None;
        }
        let end = self.keyword(p, b"of")?;
        let next = self.blank_end(end);
        (next > end && self.starts_argument(next)).then_some(end)
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionTypeAt4`,
    /// `DeclarationsAvailableTypesCollectionTypeAt4At0`,
    /// `DeclarationsTypeEnumAssociatedValuesAt3At1`, `OperatorsAt2`,
    /// `DeclarationsParameterListAt2At2`.
    ///
    /// ```text
    /// :
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type_at_4(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b':').then(|| {
            self.capture(0, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsAvailableTypesCollectionTypeAt4End`.
    ///
    /// ```text
    /// (?=[])>{}])
    /// ```
    pub(in crate::swift) fn rx_declarations_available_types_collection_type_at_4_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        matches!(self.at(p), b']' | b')' | b'>' | b'{' | b'}').then_some(p)
    }

    /// Pattern ids: `DeclarationsFunctionSubscriptAt6`,
    /// `DeclarationsFunctionInitializerAt5`, `DeclarationsFunctionAt6`.
    ///
    /// ```text
    /// (\{)
    /// ```
    pub(in crate::swift) fn rx_declarations_function_subscript_at_6(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b'{').then(|| {
            self.capture(1, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsFunctionSubscriptAt6End`,
    /// `DeclarationsOperatorSwift2End`,
    /// `DeclarationsFunctionInitializerAt5End`, `DeclarationsFunctionAt6End`.
    ///
    /// ```text
    /// (})
    /// ```
    pub(in crate::swift) fn rx_declarations_function_subscript_at_6_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b'}').then(|| {
            self.capture(1, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsImport`.
    ///
    /// ```text
    /// (?<!\.)\b(import)\s+
    /// ```
    pub(in crate::swift) fn rx_declarations_import(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        let end = self.keyword(p, b"import")?;
        let next = self.blank_end(end);
        if next == end {
            return None;
        }
        self.capture(1, p, end);
        Some(next)
    }

    /// Pattern ids: `DeclarationsImportEnd`, `DeclarationsOperatorEnd`.
    ///
    /// ```text
    /// (;)|$\n?|(?=/[*/])
    /// ```
    ///
    /// `\n?` takes the line feed when it is still there.
    pub(in crate::swift) fn rx_declarations_import_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b';' {
            self.capture(1, p, p + 1);
            return Some(p + 1);
        }
        if self.at_eol(p) {
            return Some(p + usize::from(p == self.lim() && self.nl));
        }
        (self.at(p) == b'/' && matches!(self.at(p + 1), b'/' | b'*')).then_some(p)
    }

    /// Pattern ids: `DeclarationsImportAt0`.
    ///
    /// ```text
    /// \G(?!;|$|//|/\*)(?:(typealias|struct|class|actor|enum|protocol|var|func)\s+)?
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0(&mut self, p: usize) -> Option<usize> {
        if !(self.g && p == self.pos) || self.at_statement_end(p) {
            return None;
        }
        for keyword in IMPORT_KEYWORDS {
            if let Some(end) = self.lit(p, keyword) {
                let next = self.blank_end(end);
                if next > end {
                    self.capture(1, p, end);
                    return Some(next);
                }
            }
        }
        Some(p)
    }

    /// Pattern ids: `DeclarationsImportAt0End`.
    ///
    /// ```text
    /// (?=;|$|//|/\*)
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0_end(&mut self, p: usize) -> Option<usize> {
        self.at_statement_end(p).then_some(p)
    }

    /// Pattern ids: `DeclarationsImportAt0At0`.
    ///
    /// ```text
    /// (?<=\G|\.)(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.after_anchor_or_dot(p) {
            return None;
        }
        let id = self.ident_at(p)?;
        self.capture_quotes(1, p, id);
        Some(id.end)
    }

    /// Pattern ids: `DeclarationsImportAt0At1`.
    ///
    /// ```text
    /// (?<=\G|\.)\$[0-9]+
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0_at_1(&mut self, p: usize) -> Option<usize> {
        self.dollar_number(p)
    }

    /// Pattern ids: `DeclarationsImportAt0At2`.
    ///
    /// ```text
    /// (?<=\G|\.)(?:((?<oph>[-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷‖‗†-‧‰-‾⁁-⁓⁕-⁞←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰])(\g<oph>|(?<opc>[̀-ͯ᷀-᷿⃐-⃿︀-️︠-︯\x{E0100}-\x{E01EF}]))*)|(\.(\g<oph>|\g<opc>|\.)+))(?=[.;]|$|//|/\*|\s)
    /// ```
    ///
    /// The run gives characters back until a dot, a semicolon, the end of
    /// the line, a comment start or a blank follows.
    pub(in crate::swift) fn rx_declarations_import_at_0_at_2(&mut self, p: usize) -> Option<usize> {
        if !self.after_anchor_or_dot(p) {
            return None;
        }
        let look = |this: &Self, i: usize| {
            let c = this.at(i);
            (matches!(c, b'.' | b';') || this.at_statement_end(i) || this.space_at(i) > 0)
                .then_some(i)
        };
        if let Some(width) = self.op_head_at(p) {
            let max = self.op_run_end(p + width, Dots::Refused);
            let (end, _) = self.operator_then(p + width, max, look)?;
            self.capture(1, p, end);
            return Some(end);
        }
        if self.at(p) != b'.' {
            return None;
        }
        let max = self.op_run_end(p + 1, Dots::Taken);
        let width = self.op_elem_at(p + 1)?;
        let (end, _) = self.operator_then(p + 1 + width, max, look)?;
        Some(end)
    }

    /// Pattern ids: `DeclarationsImportAt0At4`.
    ///
    /// ```text
    /// (?!\s*(;|$|//|/\*))
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0_at_4(&mut self, p: usize) -> Option<usize> {
        let end = self.blank_end(p);
        (!self.at_statement_end(end)).then_some(p)
    }

    /// Pattern ids: `DeclarationsImportAt0At4End`.
    ///
    /// ```text
    /// (?=\s*(;|$|//|/\*))
    /// ```
    pub(in crate::swift) fn rx_declarations_import_at_0_at_4_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.blank_end(p);
        self.at_statement_end(end).then_some(p)
    }

    /// Pattern ids: `DeclarationsOperator`.
    ///
    /// ```text
    /// (?:\b((?:pre|in|post)fix)\s+)?\b(operator)\s+(((?<oph>[-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷‖‗†-‧‰-‾⁁-⁓⁕-⁞←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰])(\g<oph>|\.|(?<opc>[̀-ͯ᷀-᷿⃐-⃿︀-️︠-︯\x{E0100}-\x{E01EF}]))*+)|(\.(\g<oph>|\g<opc>|\.)++))\s*
    /// ```
    pub(in crate::swift) fn rx_declarations_operator(&mut self, p: usize) -> Option<usize> {
        let fix = if self.boundary(p) {
            [&b"prefix"[..], b"infix", b"postfix"]
                .into_iter()
                .find_map(|word| self.lit(p, word))
        } else {
            None
        };
        if let Some(fix_end) = fix {
            let word = self.blank_end(fix_end);
            if word > fix_end
                && let Some(end) = self.operator_rest(word, Some((p, fix_end)))
            {
                return Some(end);
            }
        }
        self.operator_rest(p, None)
    }

    /// Pattern ids: `DeclarationsOperatorSwift2`.
    ///
    /// ```text
    /// \G(\{)
    /// ```
    pub(in crate::swift) fn rx_declarations_operator_swift_2(&mut self, p: usize) -> Option<usize> {
        (self.g && p == self.pos && self.at(p) == b'{').then(|| {
            self.capture(1, p, p + 1);
            p + 1
        })
    }

    /// Pattern ids: `DeclarationsOperatorSwift2At1`.
    ///
    /// ```text
    /// \b(associativity)\s+(left|right)\b
    /// ```
    pub(in crate::swift) fn rx_declarations_operator_swift_2_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let word = self.keyword(p, b"associativity")?;
        let from = self.blank_end(word);
        if from == word {
            return None;
        }
        let end = self.word_in(from, &[b"left", b"right"])?;
        self.capture(1, p, word);
        self.capture(2, from, end);
        Some(end)
    }

    /// Pattern ids: `DeclarationsOperatorSwift2At2`.
    ///
    /// ```text
    /// \b(precedence)\s+([0-9]+)\b
    /// ```
    ///
    /// Fewer digits would leave a digit after the number, where `\b` fails.
    pub(in crate::swift) fn rx_declarations_operator_swift_2_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let word = self.keyword(p, b"precedence")?;
        let from = self.blank_end(word);
        if from == word {
            return None;
        }
        let mut end = from;
        while self.at(end).is_ascii_digit() && end < self.lim() {
            end += 1;
        }
        if end == from || self.word_at(end).is_some() {
            return None;
        }
        self.capture(1, p, word);
        self.capture(2, from, end);
        Some(end)
    }

    /// Pattern ids: `DeclarationsOperatorSwift2At3`.
    ///
    /// ```text
    /// \b(assignment)\b
    /// ```
    pub(in crate::swift) fn rx_declarations_operator_swift_2_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.keyword(p, b"assignment")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// Pattern ids: `DeclarationsOperatorSwift3`.
    ///
    /// ```text
    /// \G(:)\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))
    /// ```
    pub(in crate::swift) fn rx_declarations_operator_swift_3(&mut self, p: usize) -> Option<usize> {
        if !(self.g && p == self.pos) || self.at(p) != b':' {
            return None;
        }
        let from = self.blank_end(p + 1);
        let id = self.ident_at(from)?;
        self.capture(1, p, p + 1);
        self.capture(2, from, id.end);
        self.capture_quotes(3, from, id);
        Some(id.end)
    }

    /// Pattern ids: `DeclarationsOperatorAt2`.
    ///
    /// ```text
    /// ((?!$|;|//|/\*)\S)+
    /// ```
    pub(in crate::swift) fn rx_declarations_operator_at_2(&mut self, p: usize) -> Option<usize> {
        let mut i = p;
        while let Some((c, end)) = self.char_at(i) {
            if c == ';' || is_space(c) || (c == '/' && matches!(self.at(i + 1), b'/' | b'*')) {
                break;
            }
            i = end;
        }
        self.spend(i - p);
        (i > p).then_some(i)
    }

    /// Pattern ids: `DeclarationsPrecedencegroup`.
    ///
    /// ```text
    /// \b(precedencegroup)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*(?=\{)
    /// ```
    pub(in crate::swift) fn rx_declarations_precedencegroup(&mut self, p: usize) -> Option<usize> {
        let from = self.word_and_blank(p, b"precedencegroup")?;
        let id = self.ident_at(from)?;
        let brace = self.blank_end(id.end);
        if self.at(brace) != b'{' {
            return None;
        }
        self.capture(1, p, p + b"precedencegroup".len());
        self.capture(2, from, id.end);
        self.capture_quotes(3, from, id);
        Some(brace)
    }

    /// Pattern ids: `DeclarationsPrecedencegroupAt0At1`.
    ///
    /// ```text
    /// \b((?:high|low)erThan)\s*:\s*((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))
    /// ```
    pub(in crate::swift) fn rx_declarations_precedencegroup_at_0_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self
            .lit(p, b"higherThan")
            .or_else(|| self.lit(p, b"lowerThan"))?;
        let colon = self.blank_end(word);
        if self.at(colon) != b':' {
            return None;
        }
        let from = self.blank_end(colon + 1);
        let id = self.ident_at(from)?;
        self.capture(1, p, word);
        self.capture(2, from, id.end);
        self.capture_quotes(3, from, id);
        Some(id.end)
    }

    /// Pattern ids: `DeclarationsPrecedencegroupAt0At2`.
    ///
    /// ```text
    /// \b(associativity)\b(?:\s*:\s*(right|left|none)\b)?
    /// ```
    pub(in crate::swift) fn rx_declarations_precedencegroup_at_0_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_with_value(p, b"associativity", &[b"right", b"left", b"none"])
    }

    /// Pattern ids: `DeclarationsPrecedencegroupAt0At3`.
    ///
    /// ```text
    /// \b(assignment)\b(?:\s*:\s*(true|false)\b)?
    /// ```
    pub(in crate::swift) fn rx_declarations_precedencegroup_at_0_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_with_value(p, b"assignment", &[b"true", b"false"])
    }

    /// Pattern ids: `DeclarationsMacro`.
    ///
    /// ```text
    /// \b(macro)\s+((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*(?=[(<=])
    /// ```
    pub(in crate::swift) fn rx_declarations_macro(&mut self, p: usize) -> Option<usize> {
        let from = self.word_and_blank(p, b"macro")?;
        let id = self.ident_at(from)?;
        let next = self.blank_end(id.end);
        if !matches!(self.at(next), b'(' | b'<' | b'=') {
            return None;
        }
        self.capture(1, p, p + b"macro".len());
        self.capture(2, from, id.end);
        self.capture_quotes(3, from, id);
        Some(next)
    }

    /// Pattern ids: `DeclarationsMacroEnd`.
    ///
    /// ```text
    /// $|(?=;|//|/\*|[=}])
    /// ```
    pub(in crate::swift) fn rx_declarations_macro_end(&mut self, p: usize) -> Option<usize> {
        (self.at_eol(p) || matches!(self.at(p), b';' | b'=' | b'}') || self.at_statement_end(p))
            .then_some(p)
    }

    /// Pattern ids: `DeclarationsParameterListAt2At3`.
    ///
    /// ```text
    /// =
    /// ```
    pub(in crate::swift) fn rx_declarations_parameter_list_at_2_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        (self.at(p) == b'=').then(|| {
            self.capture(0, p, p + 1);
            p + 1
        })
    }
}
