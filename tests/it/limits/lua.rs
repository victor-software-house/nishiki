//! Hostile lines for the lua lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit};

use super::lex;

#[test]
fn hostile_lua_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::Lua);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "\"", 300_000, ""),
        line("", "'", 300_000, ""),
        line("", "`", 300_000, ""),
        line("\"", "\\", 300_000, ""),
        line("\"", "\\z ", 100_000, ""),
        line("\"", "\\u{", 100_000, ""),
        line("", "[", 300_000, ""),
        line("", "[=", 150_000, ""),
        line("[", "=", 300_000, ""),
        line("[[", "]", 300_000, ""),
        line("[==[", "]=]", 100_000, ""),
        line("", "--[[", 100_000, ""),
        line("--[==[", "]=]", 100_000, ""),
        line("", "--", 150_000, ""),
        line("", "-", 300_000, ""),
        line("---", "@param ", 50_000, ""),
        line("---@param ", "a ", 100_000, ""),
        line("---@type ", "fun(", 60_000, ""),
        line("---@type ", "a,", 150_000, ""),
        line("---@class ", "A:", 100_000, ""),
        line("---@diagnostic ", "a-", 100_000, ""),
        line("---", "|", 300_000, ""),
        line("", "/*", 150_000, ""),
        line("", "0x", 150_000, ""),
        line("0x", "A", 300_000, "."),
        line("", "1", 300_000, "."),
        line("", "1.", 150_000, ""),
        line("", ".5", 150_000, ""),
        line("", "1e", 150_000, ""),
        line("", "0x1p", 70_000, ""),
        line("", "local function f(", 20_000, ""),
        line("function f(", "a, ", 100_000, ""),
        line("function f(", "...", 100_000, ""),
        line("", "function(", 30_000, ""),
        line("", "local ", 50_000, ""),
        line("", "goto ", 50_000, ""),
        line("", "::", 150_000, ""),
        line("", "<", 300_000, ""),
        line("", "<const", 50_000, ""),
        line("", "a.", 150_000, ""),
        line("", "a:", 150_000, ""),
        line("", "math.", 50_000, ""),
        line("", "string.", 50_000, "len"),
        line("", "x = ", 70_000, "function"),
        line("", "x ", 150_000, "= function"),
        line("", "..", 150_000, ""),
        line("", "...", 100_000, ""),
        line("", "a||", 100_000, "b"),
        line("", " ", 300_000, "x"),
        line("x", " ", 300_000, "("),
        line("", "\u{e9}", 150_000, ""),
        line("", "x", 300_000, ""),
        line("", "x", 300_000, "("),
    ];
    let mut failures = Vec::new();
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        if !report.limits().all(|limit| limit == Limit::Depth) {
            let start: String = line.chars().take(12).collect();
            failures.push(format!(
                "{start:?} reached {:?}",
                report.limits().collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
