//! Lua's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, External, LonePattern, Reentry, RuleTables, Split,
};

/// Lua's tables, from `tests/engine/grammars/lua.json`.
pub(crate) const LUA: RuleTables = RuleTables {
    language: "lua",
    grammar: "lua",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    reentry: Reentry::WithinRule,
    externals: &[(
        "source.c",
        External::Begin {
            begin: "\\G",
            end: "(?=]\\2])",
        },
    )],
};
