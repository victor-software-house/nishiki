//! ShaderLab on the shared rule engine ([`crate::rules`]): the grammar's
//! types, what its matchers remember about a line, and where HLSL hands the
//! line back.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::arena::Arena;
use crate::classes::{word_before, word_width};
use crate::rules::{self, Act, Captures, FnHalt, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The ShaderLab lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, ShaderLab>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<ShaderLab>;

/// The word at a place, as the word patterns ask about it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Word {
    /// Where the word starts.
    pub(super) start: usize,
    /// Where it ends, or `None` when `\b` does not hold before it.
    pub(super) end: Option<usize>,
    /// The [`super::words::bit`]s of the word patterns that list it.
    pub(super) bits: u64,
}

impl Default for Word {
    fn default() -> Self {
        Word {
            start: usize::MAX,
            end: None,
            bits: 0,
        }
    }
}

/// What the ShaderLab matchers remember about a line.
/// Up to 25 word patterns ask about the same word, so the last word looked up
/// is kept.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last word looked up.
    pub(super) word: Cell<Word>,
}

/// The `shaderlab` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ShaderLab;

impl Grammar for ShaderLab {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "ShaderLab";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 2;
    /// A Unity word that matches inside a block gives the line back to the
    /// block mid-line, which hands it to HLSL again.
    const EMBEDS: bool = true;
    const SKIPS_PLACES: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.data().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        pat.data().act
    }

    fn pat_captures(pat: Pat) -> Captures<Self> {
        let data = pat.data();
        (data.captures, data.nested)
    }

    fn pat_data(pat: Pat) -> (Act<Self>, Captures<Self>) {
        let data = pat.data();
        (data.act, (data.captures, data.nested))
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        table_scan(lexer)
    }

    fn next_place(lexer: &Lexer<'_>, p: usize) -> usize {
        lexer.after_missed_word(p)
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}

/// A block hands HLSL's line back at its end, `ENDCG` or `ENDHLSL`, and
/// where one of Unity's patterns starts and none of HLSL's matches.
impl rules::Embeds for ShaderLab {
    type End = FnHalt;

    /// Every embedding rule picks its halt by the rule
    /// ([`rules::Embeds::embed_halts_for`]); this one never holds.
    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (never, None)
    }

    fn embed_halts_for(rule: Rule, _: &rules::Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        let end: FnHalt = match rule {
            Rule::CgBlock => ends_cg,
            Rule::HlslBlock => ends_hlsl,
            Rule::Root | Rule::LineComment | Rule::Attribute | Rule::String => never,
        };
        (end, None)
    }

    /// Both blocks list Unity's patterns after HLSL.
    fn embeds_after_own(rule: Rule) -> bool {
        rule.embeds_after_own()
    }
}

/// `\b(ENDCG)\b` at `p` of `s`.
fn ends_cg(s: &[u8], p: usize) -> bool {
    keyword_at(s, p, b"ENDCG")
}

/// `\b(ENDHLSL)\b` at `p` of `s`.
fn ends_hlsl(s: &[u8], p: usize) -> bool {
    keyword_at(s, p, b"ENDHLSL")
}

/// Whether the word `word`, made of ASCII `\w`, stands at `p` of `s` with
/// `\b` before and after it.
fn keyword_at(s: &[u8], p: usize, word: &[u8]) -> bool {
    s.get(p..).is_some_and(|rest| rest.starts_with(word))
        && !word_before(s, p)
        && word_width(s, p + word.len()) == 0
}

/// A halt that never holds, for the rules that embed nothing.
fn never(_: &[u8], _: usize) -> bool {
    false
}
