//! The rule-engine tables of one language, as a code generator builds them.
//!
//! [`RuleTables`] holds what a language needs beyond its grammar.
//! [`super::flatten::flatten`] reads the grammar with it and returns
//! [`Tables`]: every pattern once, and every rule with the patterns it tries.

/// What a language needs beyond its grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuleTables {
    /// The language's module, such as `go` for `src/go/`.
    pub language: &'static str,
    /// The grammar's file name without `.json`, such as `common-lisp`.
    pub grammar: &'static str,
    /// Each misspelling in the grammar's repository keys, with the spelling
    /// the pattern ids take instead.
    pub key_spellings: &'static [(&'static str, &'static str)],
    /// What a capture's own pattern list becomes.
    pub capture_lists: CaptureLists,
    /// A name for each inline pattern whose derived id the language replaces,
    /// such as `("RootAt0", "Shebang")`.
    /// The patterns inside it take their ids from the name.
    pub pattern_names: &'static [(&'static str, &'static str)],
    /// What a pattern named after its first group, `$1`, becomes.
    pub split: Split,
    /// The id of a list entry's one inline pattern.
    pub lone_pattern: LonePattern,
    /// Where a list that reaches itself again adds nothing.
    pub reentry: Reentry,
    /// What each include of another grammar adds, by the include, such as
    /// Lua's `source.c`.
    /// Any other include of another grammar is an error.
    pub externals: &'static [(&'static str, External)],
}

/// What an include of another grammar adds to its list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum External {
    /// Nothing.
    /// Shiki does not load the grammar, or the language embeds it by hand.
    AddsNothing,
    /// A begin that the script makes up in the include's place.
    /// It takes the include's inline id, and its rule has no patterns.
    Begin {
        /// The begin's expression.
        begin: &'static str,
        /// The end's expression.
        end: &'static str,
    },
}

/// Where a list reached again while it is being expanded adds nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reentry {
    /// Within one rule's list.
    /// A rule a begin opens lists its patterns afresh, so a list being
    /// expanded around it still counts in full inside it.
    WithinRule,
    /// Anywhere in the walk, rules included.
    /// A rule cut short that way gets its full list from a second pass,
    /// once every pattern has its id.
    WithinWalk,
}

/// How a match whose scope name holds `$1` becomes one pattern for each
/// alternative of its first group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Split {
    /// The alternative stays in its group: `\b(extends|super)\b` gives
    /// `\b(extends)\b`.
    KeepsGroup,
    /// The alternative takes the group's place: `\b(extends|super)\b` gives
    /// `\bextends\b`.
    DropsGroup,
}

/// The id of the one pattern of a list entry that has no expression of its
/// own and holds one inline pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LonePattern {
    /// It takes its place in the entry's list, such as `StringAt0`.
    OwnId,
    /// It takes the entry's id, such as `String`.
    ListId,
}

/// What a capture's own pattern list becomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureLists {
    /// Each distinct list becomes a rule, `Nested0`, `Nested1` and so on, in
    /// the order the walk finds them.
    /// Its patterns join the tables.
    Rules,
    /// The language's hand-written code lexes a capture's text.
    /// The lists add no rule and no pattern.
    ByHand,
}

/// Every pattern and rule of one language.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tables {
    /// Each pattern once, in the order the walk first reaches it.
    pub patterns: Vec<Pattern>,
    /// Each rule once, `Root` last.
    pub rules: Vec<Rule>,
}

/// One regular expression of the grammar, and what a match of it does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern {
    /// The pattern's name, such as `RootAt3` or `StringEnd`.
    pub id: String,
    /// The expression, as the grammar spells it.
    pub regex: String,
    /// What a match does.
    pub act: Act,
}

/// What a match of a pattern does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    /// It adds its scopes, and the rule stays.
    Match,
    /// It opens the rule whose id is the pattern's id.
    Begin,
    /// It closes the rule that lists it as its end.
    End,
}

impl Act {
    /// The act's name in a grammar's `match`, `begin` or `end` key.
    #[must_use]
    pub const fn role(self) -> &'static str {
        match self {
            Act::Match => "match",
            Act::Begin => "begin",
            Act::End => "end",
        }
    }
}

/// One rule: the patterns it tries, and how it ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The rule's name: its begin pattern's id, `Root`, or `Nested` and a
    /// number.
    pub id: String,
    /// The index of its end pattern in [`Tables::patterns`], for a rule that a
    /// begin opens.
    pub end: Option<usize>,
    /// The indices of the patterns it tries, in [`Tables::patterns`], in the
    /// grammar's order with its includes flattened.
    pub patterns: Vec<usize>,
}
