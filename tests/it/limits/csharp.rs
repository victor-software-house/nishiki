//! Hostile lines and fences for the csharp lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_csharp_lines_finish_within_the_default_limits() {
    let mut highlighter = Highlighter::new(Language::CSharp);
    // `prefix`, then `unit` `count` times, then `suffix`.
    let line = |prefix: &str, unit: &str, count: usize, suffix: &str| {
        format!("{prefix}{}{suffix}", unit.repeat(count))
    };
    let lines = [
        line("", "{", 100_000, ""),
        line("", "(", 100_000, ""),
        line("", "[", 100_000, ""),
        line("", "<", 100_000, ""),
        line("", "a<", 100_000, ""),
        line("", "A<", 100_000, ">"),
        line("", "A<B, ", 50_000, ">"),
        line("", "(a, ", 50_000, ")"),
        line("", "}", 100_000, ""),
        line("", ">", 100_000, ""),
        line("", "?", 300_000, ""),
        line("", "??", 100_000, ""),
        line("", "x?[", 100_000, ""),
        line("", "a?.b", 100_000, ""),
        line("", "new ", 50_000, ""),
        line("", "new A(", 50_000, ""),
        line("", "int ", 50_000, ""),
        line("", "int ", 50_000, "x;"),
        line("", "A ", 100_000, "x;"),
        line("", "A a, ", 50_000, ";"),
        line("", "int[] ", 50_000, "x;"),
        line("", "int x = 1; ", 20_000, ""),
        line("", "A.", 100_000, "x"),
        line("", "a.", 100_000, "b y;"),
        line("", "a.b ", 50_000, ""),
        line("", "A.B<", 50_000, ""),
        line("", "A::", 100_000, ""),
        line("", "x(", 100_000, ""),
        line("", "x()", 100_000, ""),
        line("", ".x()", 100_000, ""),
        line("", "a b ", 50_000, "("),
        line("", "void m(", 50_000, ""),
        line("", "int a, ", 50_000, ") {"),
        line("", "ref ", 50_000, "int x"),
        line("", "ref readonly ", 30_000, "int x"),
        line("", "params ", 50_000, "int[] x"),
        line("", "static ", 50_000, "int a;"),
        line("", "async ", 50_000, "x => 1"),
        line("", "(int a, ", 30_000, ") => 1"),
        line("", "x => ", 50_000, "1"),
        line("", "delegate ", 50_000, ""),
        line("", "where T : ", 30_000, ""),
        line("", "class ", 50_000, ""),
        line("", "record ", 40_000, ""),
        line("", "struct ", 50_000, ""),
        line("", "enum ", 50_000, ""),
        line("", "interface ", 30_000, ""),
        line("", "namespace ", 40_000, ""),
        line("", "using ", 40_000, ""),
        line("", "using static ", 30_000, ""),
        line("", "global ", 40_000, "using A;"),
        line("", "[", 100_000, "]"),
        line("", "[A(", 50_000, ""),
        line("", "is not ", 50_000, ""),
        line("", "is { A: ", 30_000, ""),
        line("", "switch { ", 30_000, ""),
        line("", "case ", 50_000, ""),
        line("", "from x in ", 30_000, ""),
        line("", "join ", 40_000, ""),
        line("", "yield ", 50_000, ""),
        line("", "\"", 100_000, ""),
        line("", "'", 100_000, ""),
        line("", "$\"{", 50_000, ""),
        line("", "$\"\"\"{", 30_000, ""),
        line("", "\"\"\"", 100_000, ""),
        line("", "@\"", 100_000, ""),
        line("\"", "\\", 100_000, ""),
        line("", "{{", 100_000, ""),
        line("$\"", "{{", 50_000, ""),
        line("", "/*", 100_000, ""),
        line("", "/**", 100_000, ""),
        line("", "///", 100_000, ""),
        line("", "//", 100_000, ""),
        line("/// ", "<a ", 50_000, ""),
        line("/// ", "&", 100_000, ""),
        line("/// ", "<![CDATA[", 50_000, ""),
        line("", "#", 100_000, ""),
        line("", "#if ", 50_000, ""),
        line("", "#pragma warning disable ", 30_000, ""),
        line("", "0x", 100_000, ""),
        line("", "0b1", 100_000, ""),
        line("", "1_", 100_000, ""),
        line("", "1e", 100_000, ""),
        line("", "1.", 100_000, ""),
        line("", ".5", 100_000, ""),
        line("", "1f", 100_000, ""),
        line("", "1_0", 100_000, "x"),
        line("", "1.5e+", 50_000, ""),
        line("", "x", 300_000, ""),
        line("", "@x", 100_000, ""),
        line("", "_", 300_000, ""),
        line("", "=>", 100_000, ""),
        line("", "<<", 100_000, "="),
        line("", "?:", 100_000, ""),
        line("", " ", 300_000, ""),
        line("x", " ", 300_000, "y"),
        line("", "\t", 300_000, "//"),
        line("", "\u{a0}", 100_000, ""),
        line("", "日", 100_000, ""),
        line("", "try ", 50_000, ""),
        line("", "catch (", 50_000, ""),
        line("", "for (int i = 0; ", 30_000, ""),
        line("", "foreach (var ", 30_000, ""),
        line("", "(A) ", 50_000, ""),
        line("", "(A)-", 50_000, ""),
        line("", "x as ", 50_000, ""),
        line("", "stackalloc ", 50_000, ""),
        line("", "event ", 50_000, ""),
        line("", "operator ", 50_000, ""),
        line("", "implicit operator ", 30_000, ""),
        line("", "this[", 50_000, ""),
        line("", "~", 100_000, ""),
        line("", "A.B.", 50_000, "C()"),
        line("", "A<B>.", 50_000, "C()"),
    ];
    let failures: Vec<String> = lines
        .iter()
        .filter_map(|line| {
            let (rows, _) = lex(&mut highlighter, &[line]);
            let limits: Vec<Limit> = rows[0].1.limits().collect();
            let start: String = line.chars().take(12).collect();
            (!limits.iter().all(|&limit| limit == Limit::Depth))
                .then(|| format!("{start:?} reached {limits:?}"))
        })
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn a_markdown_fence_runs_its_csharp_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let csharp = format!("F({});", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```csharp", &csharp, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
