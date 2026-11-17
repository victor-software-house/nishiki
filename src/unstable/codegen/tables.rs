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
    /// What a pattern named after a group, such as `$1`, becomes.
    pub split: Split,
    /// The id each pattern of a split takes.
    pub split_ids: SplitIds,
    /// What a match becomes when only a capture's name holds a group's text.
    pub capture_names: CaptureNames,
    /// The id of a list entry's one inline pattern.
    pub lone_pattern: LonePattern,
    /// How the walk finds that it reaches a pattern again.
    pub identity: Identity,
    /// Where a list that reaches itself again adds nothing.
    pub reentry: Reentry,
    /// What each include of another grammar adds, by the include, such as
    /// Lua's `source.c`.
    /// Any other include of another grammar is an error.
    /// An include of a repository key the grammar lacks, such as Assembly's
    /// `#string_escaped_char`, can take an entry too.
    pub externals: &'static [(&'static str, External)],
    /// The matches another grammar injects into this one, when the language
    /// takes them.
    pub injected: Option<Injection>,
    /// The rules whose patterns are a list in another grammar.
    pub added_rules: &'static [AddedRule],
    /// The matches at the start of `Root` that merge into one, when the
    /// language merges them.
    pub merged_words: Option<MergedWords>,
}

/// The entries at the start of `Root` whose matches merge into one, such as
/// Assembly's registers and mnemonics.
///
/// Each such match is `(?i)\b(…)\b` over plain words, made of ASCII `\w`:
///
/// - it matches where the whole run of `\w` from a `\b` is one of its words;
/// - at a run's start, the first of them that holds the run wins.
///
/// So one match of all their words, in their order, matches the same text,
/// and the language's code names that text by the first match that lists
/// the word.
///
/// The merged match follows every other pattern.
/// `Root` tries it first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MergedWords {
    /// The merged match's id, such as `Word`.
    pub id: &'static str,
    /// How many of `Root`'s first entries merge.
    pub entries: usize,
}

/// A rule whose patterns are a list in another grammar, such as CSS's rule
/// for the string of Vue's `style` attribute.
///
/// The rule follows `Root`, and has no begin and no end.
/// An include of the language's own scope with a key, such as
/// `source.css#escapes`, adds the language's own entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddedRule {
    /// The rule's id, such as `VueStyleAttribute`.
    pub id: &'static str,
    /// The grammar's file name without `.json`, such as `vue`.
    pub grammar: &'static str,
    /// The repository key where the path starts, such as
    /// `vue-directives-style-attr`.
    pub key: &'static str,
    /// The position to follow in each rule's patterns, from the entry to the
    /// rule whose list it is.
    /// `&[0]` takes the entry's first pattern.
    pub path: &'static [usize],
}

/// The matches another grammar injects into a language under one selector,
/// such as Svelte's `$` prefixes in CoffeeScript.
///
/// They follow every other pattern, and no rule lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Injection {
    /// The grammar's file name without `.json`, such as `svelte`.
    pub grammar: &'static str,
    /// The selector of the injection in that grammar's `injections`.
    pub selector: &'static str,
    /// The id of each injected match, in the grammar's order.
    pub ids: &'static [&'static str],
}

/// How the walk finds that it reaches a pattern again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Identity {
    /// By its id.
    Id,
    /// By its contents, before its id.
    /// A pattern reached again with the same contents keeps its first id,
    /// whatever id the new place would give it.
    /// A rule the walk dropped stays dropped.
    Content,
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
    /// A match from the repository of another grammar, such as HAML's
    /// `source.ruby#escaped_char`.
    /// Its id is the grammar's name and the key in Pascal case, such as
    /// `RubyEscapedChar`.
    /// The caller passes that grammar to [`super::flatten::flatten`].
    Pattern {
        /// The grammar's file name without `.json`, such as `ruby`.
        grammar: &'static str,
        /// The repository key, such as `escaped_char`.
        key: &'static str,
    },
    /// Nothing in the list.
    /// The rule whose own list holds the include embeds the grammar.
    ///
    /// When the rule's end repeats one group of its begin, the rule lists
    /// one begin, `<rule>Body`, and nothing else.
    /// That begin is `\G`, its end is `(?=<the rule's end>)`, and its rule
    /// has no patterns.
    Embeds,
    /// Nothing, from a grammar Shiki does not load, or from a repository
    /// key the grammar lacks.
    /// A rule whose own list holds the include is dropped when it lists no
    /// pattern and embeds no grammar, as vscode-textmate drops it.
    Missing,
    /// Nothing, from a grammar Shiki does not load.
    /// A list entry whose own patterns are this include alone is left out of
    /// its list, and takes no place there.
    DropsItsEntry,
    /// Nothing in a list.
    /// A capture list of this include alone becomes the rule `id`, with no
    /// patterns.
    /// The language's code lexes such a capture with the other grammar.
    CaptureRule {
        /// The rule's id, such as `Javascript`.
        id: &'static str,
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

/// How a match or begin whose scope name holds a group's text, such as
/// `$1`, becomes one pattern for each alternative of that group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Split {
    /// The alternative stays in its group: `\b(extends|super)\b` gives
    /// `\b(extends)\b`.
    KeepsGroup,
    /// The alternative takes the group's place: `\b(extends|super)\b` gives
    /// `\bextends\b`.
    DropsGroup,
}

/// The id each pattern of a split takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitIds {
    /// Its position.
    ///
    /// - In a list, each pattern takes the next place, such as `RootAt3` and
    ///   `RootAt4`.
    /// - A repository entry's patterns take the entry's id, `At` and their
    ///   position, such as `FontFeaturesAt2`.
    Positions,
    /// The split pattern's id, then the letters that start its alternative,
    /// the first in upper case.
    ///
    /// - In a list, `class` at place 13 of `Keywords` gives
    ///   `KeywordsAt13Class`, and the split takes one place.
    /// - A repository entry `declarations-type-enum` gives
    ///   `DeclarationsTypeEnumEnum`.
    Words,
}

/// What a match becomes when only a capture's scope name holds a group's
/// text, as `keyword.other.unit.${2:/downcase}.css` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureNames {
    /// It stays one pattern.
    Kept,
    /// The names of `captures` and `beginCaptures` split the group as the
    /// pattern's own name does.
    /// vscode-textmate fills a capture's `$1` from the same match, so each
    /// pattern gets a fixed name.
    SplitsGroup,
    /// The innermost optional group `(?:…)?` around the named group splits.
    /// Each alternative gives one pattern, and a last one leaves the group
    /// out.
    /// The language's code names the group's text.
    SplitsOptionalGroup,
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
    /// As [`CaptureLists::Rules`], except that a list of one include of a
    /// repository key takes the key in Pascal case, such as `FunctionNames`.
    NamedRules,
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
