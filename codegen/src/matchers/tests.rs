//! The writer's choices, read from the code it writes for small expressions.

use nishiki::unstable::regex::{Anchor, Dialect, parse};
use quote::{format_ident, quote};

use super::chars::Chars;
use super::language;
use super::shape::{drop_covered_arms, leading_anchor, one_character_set, scan_skip};
use super::writer::{Refusal, matcher};

/// The most characters a written line holds, as `check:line-width` allows.
const WIDTH: usize = 100;

/// How many arms `drop_covered_arms` drops from `regex`.
fn covered_arms(regex: &str) -> usize {
    let mut parsed = parse(regex, Dialect::Grammar).expect("the test expression parses");
    drop_covered_arms(&mut parsed.root)
}

/// The characters of a one-character expression.
fn characters(regex: &str) -> Chars {
    let parsed = parse(regex, Dialect::Grammar).expect("the test expression parses");
    one_character_set(&parsed.root).expect("the expression takes one character")
}

/// The code the writer gives `regex` as the function `m`, with no group read
/// and its scan skip in `lx.x.m`, without whitespace.
fn written(regex: &str) -> String {
    let skips = quote!(lx.x.m);
    let code = matcher(regex, &format_ident!("m"), &[], Some(&skips))
        .unwrap_or_else(|Refusal(reason)| panic!("{regex} has no matcher: {reason}"));
    code.items.to_string().split_whitespace().collect()
}

/// The position anchor every match of `regex` starts with.
fn leading(regex: &str) -> Option<Anchor> {
    let parsed = parse(regex, Dialect::Grammar).expect("the test expression parses");
    leading_anchor(&parsed.root)
}

/// The `k` of the scan skip `regex` earns, when it earns one.
fn skip_reach(regex: &str) -> Option<usize> {
    let mut parsed = parse(regex, Dialect::Grammar).expect("the test expression parses");
    drop_covered_arms(&mut parsed.root);
    scan_skip(&parsed.root).map(|skip| skip.reach)
}

#[test]
fn the_space_class_is_the_lexers() {
    assert!(
        characters(r"\s").same(&Chars::space()),
        "a `\\s*` that calls `space_end` takes what the regex takes"
    );
    assert!(!characters(r"[ \t]").same(&Chars::space()));
}

#[test]
fn an_alternation_of_words_dispatches_on_its_first_byte() {
    let code = written(r"\b(?:assert|lua|ipairs|lpeg)\b");
    assert!(
        code.contains("matchbyte_at(lx,pos){Some(b'a')=>{"),
        "{code}"
    );
    let ell = code.find("Some(b'l')=>").expect("a bucket for l");
    let rest = code.get(ell..).unwrap_or_default();
    let (lua, lpeg) = (rest.find("b'u'"), rest.find("b'p'"));
    assert!(
        lua.is_some() && lua < lpeg,
        "the l bucket tries lua before lpeg: {code}"
    );
    assert!(
        !written(r"(?:ab|ac|bd)x").contains("Some(b'a')=>{"),
        "two first bytes give no dispatch"
    );
}

#[test]
fn a_pattern_that_starts_with_an_anchor_is_tested_before_its_call() {
    assert_eq!(
        leading(r"\G[\t ]*(@)(alias|author)\b"),
        Some(Anchor::LastMatchEnd)
    );
    assert_eq!(leading(r"(?:\G)a"), Some(Anchor::LastMatchEnd));
    assert_eq!(leading(r"$\n?"), Some(Anchor::LineEnd));
    assert_eq!(leading(r"^\s*#"), Some(Anchor::LineStart));
    assert_eq!(leading(r"[\t ]*\G"), None);
    assert_eq!(leading(r"\Ga|b"), None);
    assert_eq!(leading(r"\bfoo"), None);
}

#[test]
fn a_boundary_with_a_settled_side_reads_the_other() {
    let code = written(r"\b([A-Z_a-z][0-9A-Z_a-z]*)\b(?=\s*[({])");
    assert!(code.contains("iflx.word_before(pos){"), "{code}");
    assert!(code.contains("lx.word_at(pos).is_some()"), "{code}");
    assert!(!code.contains("lx.boundary(pos)"), "{code}");
}

#[test]
fn a_boundary_with_no_settled_side_reads_both() {
    let code = written(r"a|\b(?:x|\.)?");
    assert!(code.contains("lx.boundary(pos)"), "{code}");
}

#[test]
fn the_field_pattern_skips_past_its_run() {
    let field = r"\s*(optional|repeated|required)?\s*(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\h+|[0-9]+)";
    assert_eq!(skip_reach(field), Some(8), "a keyword can start the run");
    let code = written(field);
    assert!(code.contains("lx.x.m.covers(lx,p)"), "{code}");
    assert!(code.contains("Skip::after(lx,p,end,8)"), "{code}");
    assert_eq!(skip_reach(r"\s*([.\w]+)\s+x"), Some(0));
    assert_eq!(skip_reach(r"(?:ab){0,2}([.\w]+)x"), Some(4));
    assert_eq!(skip_reach(r"(?:ab)?a?([.\w]+)x"), Some(3));
    let two_bytes = char::from(0xe9_u8);
    assert_eq!(
        skip_reach(&format!(r"{two_bytes}?([.\w]+)x")),
        Some(2),
        "the reach counts bytes"
    );
    assert_eq!(skip_reach(r".?([.\w]+)x"), Some(4));
}

#[test]
fn a_pattern_with_no_exact_skip_has_none() {
    for (regex, why) in [
        (r"\G\s*([.\w]+)\s+x", "`\\G` reads where the search began"),
        (r"(a)?([.\w]+)\1", "a back-reference reads a group"),
        (
            r"(?!a)([.\w]+)x",
            "a lookahead in `P` holds at some starts only",
        ),
        (
            r"\b([.\w]+)x",
            "an assertion in `P` holds at some starts only",
        ),
        (
            r"(?>\s*)([.\w]+)x",
            "an atomic group can refuse the empty path",
        ),
        (r"([.\w]++)x", "a possessive run gives nothing back"),
        (r"(?:ab)*([.\w]+)x", "`k` has no bound"),
        (r"a([.\w]+)x", "`P` does not match empty"),
        (r"([.\w]{1,9})x", "the run has an upper bound"),
        (
            r"\s*(syntax)\s*(=)",
            "the scan already passes a run of blanks",
        ),
    ] {
        assert_eq!(skip_reach(regex), None, "{regex}: {why}");
        assert!(!written(regex).contains("Skip::after"), "{regex}: {why}");
    }
}

#[test]
fn drops_the_dot_arm_of_the_field_pattern() {
    assert_eq!(
        covered_arms(
            r"\s*(optional|repeated|required)?\s*(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\h+|[0-9]+)"
        ),
        1
    );
    assert_eq!(covered_arms(r"\.??[.\w]+?"), 1);
    assert_eq!(covered_arms(r"a?[a-z]*"), 1);
}

#[test]
fn keeps_an_arm_the_run_does_not_cover() {
    for (regex, why) in [
        (r"\.?\w+", "the dot is outside the run's class"),
        (r"(\.)?[.\w]+", "a group sits around the arm"),
        (r"\.?([.\w]+)", "a group sits around the run"),
        (r"\.?[.\w]{1,3}", "the run has an upper bound"),
        (r"\.?[.\w]+?", "the arm is preferred but the run is lazy"),
        (r"\.?[.\w]++", "the run is possessive"),
        (r"(?<=\.?[.\w]+)a", "a lookbehind is left alone"),
    ] {
        assert_eq!(covered_arms(regex), 0, "{regex}: {why}");
    }
}

#[test]
fn every_written_line_fits_the_width() {
    let root = crate::root().expect("the repository root");
    let file = language(&root, "proto", "src/proto/matchers.rs").expect("the matchers");
    let long = file
        .text
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().count() > WIDTH)
        .map(|(index, line)| format!("{}: {line}", index + 1))
        .collect::<Vec<_>>();
    assert!(
        long.is_empty(),
        "lines past {WIDTH} characters:\n{}",
        long.join("\n")
    );
}

#[test]
fn a_pattern_the_lexer_cannot_test_has_no_matcher() {
    for (regex, why) in [
        (r"\Afoo", "`\\A` holds only at the start of the first line"),
        (
            r"(?:a|)+b",
            "the regex engine counts an empty turn toward the least",
        ),
        (r"b(?:c?){2,}d", "the same, for a least of two"),
        (
            r"(?:xa*?)*c",
            "a turn's first match takes no `a`, and only the longest run of `a` can succeed",
        ),
    ] {
        assert!(
            matcher(regex, &format_ident!("m"), &[], None).is_err(),
            "{regex}: {why}"
        );
    }
}
