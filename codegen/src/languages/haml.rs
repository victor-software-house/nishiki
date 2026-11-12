//! HAML's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, Reentry, RuleTables, Split};

/// HAML's tables, from `tests/engine/grammars/haml.json`.
pub(crate) const HAML: RuleTables = RuleTables {
    language: "haml",
    grammar: "haml",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[
        ("RootAt0", "InterpolatedText"),
        ("RootAt1", "RubyFilter"),
        ("RootAt2", "Prolog"),
        ("RootAt3", "JavascriptFilter"),
        ("RootAt4", "Script"),
        ("RootAt5", "RubyFilterAlone"),
        ("RootAt6", "ConditionalComment"),
        ("RootAt7", "Comment"),
        ("RootAt8", "Tag"),
        ("RootAt9", "RubyOrOpalFilter"),
        ("RootAt10", "RubyFilterAloneAgain"),
        ("RootAt11", "SassStyleFilter"),
        ("RootAt12", "CoffeeFilter"),
        ("RootAt13", "PlainFilter"),
        ("RootAt14", "RubyFilterKeyword"),
        ("RootAt15", "SassFilter"),
        ("RootAt16", "SassStylesFilter"),
        ("RootAt17", "PlainFilterAgain"),
        ("RootAt18", "Escape"),
        ("RootAt19", "RubyStart"),
        ("RootAt20", "PhpFilter"),
        ("RootAt21", "MarkdownFilter"),
        ("RootAt22", "CssFilter"),
        ("RootAt23", "SassFilterAlone"),
        ("RootAt24", "ScssFilter"),
    ],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::ListId,
    reentry: Reentry::WithinRule,
};
