//! Hostile lines and fences for the C++ lexer, under the default limits.

use nishiki::{Highlighter, Language, Limit, Limits, Work};

use super::lex;

#[test]
fn hostile_cpp_lines_finish_within_the_default_limits() {
    let lines = [
        "(".repeat(100_000),
        "[".repeat(100_000),
        "{".repeat(100_000),
        "x[".repeat(50_000),
        "f(".repeat(50_000),
        "a.".repeat(100_000),
        "a->".repeat(60_000),
        "a.b.c(".repeat(40_000),
        "a .b ".repeat(40_000),
        "/*".repeat(100_000),
        "//".repeat(100_000),
        "/**".repeat(60_000),
        "/** @a ".repeat(30_000),
        "\"".repeat(100_000),
        "'".repeat(100_000),
        "\\".repeat(100_000),
        format!("\"{}", "\\".repeat(100_000)),
        format!("\"{}", "%1$".repeat(50_000)),
        format!("\"{}", "%-5.5ld".repeat(30_000)),
        format!("\"{}", "%0000000000".repeat(20_000)),
        "#define ".repeat(50_000),
        "#define X(a,".repeat(30_000),
        "#if defined(".repeat(30_000),
        "#error \"".repeat(30_000),
        "#include <".repeat(40_000),
        "0x".repeat(100_000),
        "1e".repeat(100_000),
        "1'".repeat(100_000),
        "0x1.p".repeat(40_000),
        "...".repeat(60_000),
        " ".repeat(300_000),
        format!("x{}", " ".repeat(300_000)),
        format!("{}x", " ".repeat(300_000)),
        "x".repeat(300_000),
        "x or(".repeat(40_000),
        "switch(".repeat(40_000),
        "switch ".repeat(40_000),
        "case ".repeat(60_000),
        "case 1:".repeat(40_000),
        "asm(".repeat(50_000),
        "asm volatile ".repeat(30_000),
        "?".repeat(100_000),
        "? : ".repeat(40_000),
        "int (".repeat(40_000),
        "foo(bar(".repeat(30_000),
        "\u{a0}".repeat(100_000),
        "/*\u{a0}".repeat(60_000),
        "<".repeat(25_000),
        "<<".repeat(15_000),
        ">".repeat(25_000),
        "a<".repeat(12_500),
        "a<b<c<".repeat(7_500),
        "a<b>::".repeat(7_500),
        "std::".repeat(12_500),
        "::".repeat(25_000),
        "a::b::c(".repeat(7_500),
        "template<".repeat(7_500),
        "template <class T, ".repeat(5_000),
        "operator".repeat(10_000),
        "operator<<(".repeat(7_500),
        "[[".repeat(15_000),
        "[[x]] ".repeat(7_500),
        "alignas(".repeat(10_000),
        "R\"(".repeat(12_500),
        format!("R\"x({}", "a".repeat(25_000)),
        format!("R\"{}(", "d".repeat(25_000)),
        "using X = ".repeat(7_500),
        "enum class E : ".repeat(5_000),
        "namespace a::b {".repeat(5_000),
        "auto f() -> ".repeat(5_000),
        "[&](int x) {".repeat(7_500),
        "int x = ".repeat(10_000),
        "int *".repeat(10_000),
        "int & ".repeat(10_000),
        "a, ".repeat(15_000),
        "a b ".repeat(15_000),
        "const ".repeat(12_500),
        "static const int ".repeat(5_000),
        "A::A(".repeat(7_500),
        "~A(".repeat(10_000),
        "x ? ".repeat(10_000),
    ];
    let mut highlighter = Highlighter::new(Language::Cpp);
    for line in lines {
        let (rows, _) = lex(&mut highlighter, &[&line]);
        let report = rows[0].1;
        assert!(
            report
                .limits()
                .all(|limit| matches!(limit, Limit::Depth | Limit::Work)),
            "{:?} reached {:?}",
            &line[..4],
            report.limits().collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_scope_chain_nested_deeper_than_the_lexer_bound_finishes() {
    let line = format!("{}b x;", "a::".repeat(500));
    let limits = Limits {
        depth: None,
        work: None,
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Cpp, limits);
    let (rows, _) = lex(&mut highlighter, &[&line]);
    assert!(rows[0].1.reached(Limit::Depth));
}

#[test]
fn a_markdown_fence_runs_its_cpp_lines_under_the_same_limits() {
    let limits = Limits {
        work: Some(Work {
            per_byte: 0,
            base: 64,
        }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let c = format!("int x[] = {{{}}};", "1, ".repeat(200));
    let (rows, _) = lex(&mut highlighter, &["```cpp", &c, "```"]);
    assert!(rows[1].1.reached(Limit::Work));
    assert!(rows[2].1.is_complete());
}
