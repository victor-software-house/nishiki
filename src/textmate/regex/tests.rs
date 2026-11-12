use std::collections::BTreeSet;

use regex_syntax::hir::Hir;

use super::ast::orbit;
use super::folds::{MULTI, UNFOLD};
use super::parse::{self, Dialect};
use super::{Context, Programs, Regex, Scratch, Sketch};

/// The whole match and the groups of the first match in `text`, from `from`,
/// with `\G` at `search` and `\A` at the start.
fn search(
    pattern: &str,
    text: &str,
    from: usize,
    search: Option<usize>,
) -> Option<Vec<Option<(usize, usize)>>> {
    let regex = Regex::new(pattern).unwrap_or_else(|e| panic!("{pattern}: {e:?}"));
    let mut scratch = Scratch::default();
    let mut steps = 100_000_000;
    let context = Context {
        search,
        text_start: true,
    };
    regex
        .find(&mut scratch, text.as_bytes(), from, context, &mut steps)
        .then(|| (0..=regex.groups()).map(|g| scratch.group(g)).collect())
}

fn find(pattern: &str, text: &str) -> Option<(usize, usize)> {
    search(pattern, text, 0, None).and_then(|groups| groups[0])
}

#[test]
fn literals_classes_and_quantifiers() {
    assert_eq!(find("a+b", "xaab"), Some((1, 4)));
    assert_eq!(find("[a-c]{2,3}", "zabcd"), Some((1, 4)));
    assert_eq!(find("a*?b", "aab"), Some((0, 3)));
    assert_eq!(find("(?i)HeLLo", "say hello"), Some((4, 9)));
    assert_eq!(find("\\bfoo\\b", "a foo b"), Some((2, 5)));
    assert_eq!(find("x{2}", "xxx"), Some((0, 2)));
    assert_eq!(find("x{2,}", "xxxx"), Some((0, 4)));
    assert_eq!(find("x{0,2}?y", "xxy"), Some((0, 3)));
    assert_eq!(find("é+", "caféé!"), Some((3, 7)));
}

#[test]
fn alternation_takes_the_first_that_matches() {
    assert_eq!(find("a|ab", "ab"), Some((0, 1)));
    assert_eq!(find("(a|ab)(c|bcd)", "abcd"), Some((0, 4)));
    assert_eq!(find("x(?:a|b|c)y", "xcy"), Some((0, 3)));
}

#[test]
fn groups_hold_the_last_iteration_and_unset_groups_are_none() {
    let found = search("(a)|(b)", "b", 0, None).unwrap();
    assert_eq!(found, [Some((0, 1)), None, Some((0, 1))]);
    let found = search("(?:(a)|b)*", "ab", 0, None).unwrap();
    assert_eq!(found[1], Some((0, 1)));
}

#[test]
fn lookaround_and_backrefs() {
    assert_eq!(find("foo(?=bar)", "foobar"), Some((0, 3)));
    assert_eq!(find("foo(?!bar)", "foobar foobaz"), Some((7, 10)));
    assert_eq!(find("(?<=a)b", "cb ab"), Some((4, 5)));
    assert_eq!(find("(?<!a)b", "ab cb"), Some((4, 5)));
    assert_eq!(find("(?<=a|bc)d", "xbcd"), Some((3, 4)));
    assert_eq!(find("(['\"]).*?\\1", "x 'a' y"), Some((2, 5)));
    assert_eq!(find("(?<q>a)\\k<q>", "baa"), Some((1, 3)));
    assert_eq!(find("(?i)(a)\\1", "aA"), Some((0, 2)));
}

#[test]
fn a_positive_lookahead_keeps_its_groups() {
    let found = search("(?=(a))a", "a", 0, None).unwrap();
    assert_eq!(found[1], Some((0, 1)));
    let found = search("(?!(b))a", "a", 0, None).unwrap();
    assert_eq!(found[1], None);
}

#[test]
fn atomic_possessive_and_calls() {
    assert_eq!(find("(?>a+)b", "aaab"), Some((0, 4)));
    assert_eq!(find("(?>a|ab)c", "abc"), None);
    assert_eq!(find("a++a", "aaa"), None);
    assert_eq!(find("a*+b", "aab"), Some((0, 3)));
    assert_eq!(
        find("(?<p>\\((?:[^()]|\\g<p>)*\\))", "x(a(b)c)y"),
        Some((1, 8))
    );
    assert_eq!(find("(a)\\g<1>", "aa"), Some((0, 2)));
}

#[test]
fn a_loop_over_an_empty_match_ends() {
    assert_eq!(find("(?:a*)*b", "aab"), Some((0, 3)));
    assert_eq!(find("(?:)*x", "x"), Some((0, 1)));
    assert_eq!(find("(a|)+b", "b"), Some((0, 1)));
}

/// `^` does not match after the final newline.
#[test]
fn anchors_and_search_position() {
    assert!(search("(^|\\G)ab", "ab", 0, None).is_some());
    assert!(search("(^|\\G)ab", "xab", 0, None).is_none());
    assert!(search("(^|\\G)ab", "xab", 0, Some(1)).is_some());
    assert_eq!(find("a$", "a\n"), Some((0, 1)));
    assert_eq!(find("^b", "a\nb"), Some((2, 3)));
    assert_eq!(find("^", "a\n"), Some((0, 0)));
    assert_eq!(search("^$", "\n", 1, None), None);
    assert_eq!(find("\\Aab", "ab"), Some((0, 2)));
    assert_eq!(find("b\\Z", "b\n"), Some((0, 1)));
}

#[test]
fn classes_with_intersection_and_posix() {
    assert_eq!(find("[a-z&&[^aeiou]]+", "aebcd"), Some((2, 5)));
    assert_eq!(find("[[:digit:]]+", "ab12"), Some((2, 4)));
    assert_eq!(find("\\h+", "xyzBEEFq"), Some((3, 7)));
    assert_eq!(find("[^\\s]+", "  ab "), Some((2, 4)));
    assert_eq!(find("\\p{Lu}\\p{Ll}", "xAb"), Some((1, 3)));
}

/// `{n,m}` with `n > m` is a possessive `{m,n}`, as in Oniguruma.
/// A `?` after it repeats it again instead of making it lazy.
#[test]
fn a_reversed_interval_is_a_possessive_one() {
    assert_eq!(find("x{1,0}y", "y"), Some((0, 1)));
    assert_eq!(find("a{2,1}", "aaa"), Some((0, 2)));
    assert_eq!(find("a{2,1}a", "aa"), None);
    assert_eq!(find("a{1,0}?", "a"), Some((0, 1)));
}

/// The class a pattern of one class parses to.
fn class_of(pattern: &str) -> super::ast::Class {
    let root = parse::parse(pattern, Dialect::Grammar).unwrap().root;
    let super::ast::Node::Class(class) = root else {
        panic!("{pattern:?} is not one class: {root:?}");
    };
    *class
}

#[test]
fn a_class_knows_its_ascii_members_as_it_matches_them() {
    for pattern in [
        "[a-z]",
        "[^a-z]",
        "[a-z&&[^aeiou]]",
        "[[:digit:][:space:]]",
        "[^\\s]",
        "[\\w-]",
        "\\h",
        "\\p{Lu}",
        "\\P{L}",
        "[^\\p{L}\\d_]",
        "[\\x00-\\x{7f}]",
        "[\\x{80}-\\x{10ffff}]",
        "[[^a][^b]&&[a-c]]",
    ] {
        let class = class_of(pattern);
        let tested = (0..128_u8).fold(0_u128, |bits, byte| {
            bits | u128::from(class.matches(char::from(byte))) << byte
        });
        assert_eq!(class.ascii(), tested, "{pattern:?}");
    }
}

#[test]
fn the_search_starts_where_it_is_told() {
    assert_eq!(search("a", "aXa", 1, None).unwrap()[0], Some((2, 3)));
    assert_eq!(find("a|b", "xxb"), Some((2, 3)));
    assert_eq!(find("zo+bar", "xx zooobar"), Some((3, 10)));
}

#[test]
fn a_search_that_spends_its_steps_gives_up() {
    let regex = Regex::new("(a*)*b").unwrap();
    let mut scratch = Scratch::default();
    let mut steps = 10_000;
    let text = format!("{}cb", "a".repeat(40));
    let context = Context {
        search: None,
        text_start: true,
    };
    assert!(!regex.find(&mut scratch, text.as_bytes(), 0, context, &mut steps));
    assert_eq!(steps, 0);
}

#[test]
fn a_long_line_does_not_need_a_deep_stack() {
    let text = "ab".repeat(200_000);
    assert_eq!(find("(?:ab)*c|(?:ab)+", &text), Some((0, 400_000)));
}

#[test]
fn bad_patterns_are_errors() {
    for pattern in [
        "(",
        "a)",
        "[a",
        "\\g<missing>",
        "\\k<missing>",
        "(?<n>a)\\3",
    ] {
        assert!(Regex::new(pattern).is_err(), "{pattern}");
    }
}

#[test]
fn a_lookahead_that_needs_a_character_limits_where_a_match_starts() {
    assert_eq!(find("(?=ab)", "xxab"), Some((2, 2)));
    assert_eq!(find("x*(?=<b)", "ccxx<b"), Some((2, 4)));
    assert_eq!(find("(?<=c)(?=d)", "ab cd"), Some((4, 4)));
    assert_eq!(find("a+(?=x<)", "aaxy"), None);
}

#[test]
fn a_required_character_missing_from_the_text_ends_the_search() {
    assert_eq!(find("a.*<", "aaa b"), None);
    assert_eq!(find("a.*<", "aaa < b"), Some((0, 5)));
    assert_eq!(find("(?:a|b)<", "zz<"), None);
}

#[test]
fn oniguruma_escapes_for_line_breaks_and_any_character() {
    assert_eq!(find("\\N+", "ab\ncd"), Some((0, 2)));
    assert_eq!(find("a\\Ob", "a\nb"), Some((0, 3)));
    assert_eq!(find("x\\R", "x\r\ny"), Some((0, 3)));
    assert_eq!(find("x\\R", "x\ny"), Some((0, 2)));
    assert_eq!(find("\\X", "e\u{301}x"), Some((0, 3)));
}

/// Compiles every pattern of every grammar Shiki bundles, from a file of hex
/// lines (`NISHIKI_PATTERNS`), and lists those the machine refuses.
#[test]
#[ignore = "a survey over a scratch file"]
fn survey_every_bundled_pattern() {
    let Some(path) = std::env::var_os("NISHIKI_PATTERNS") else {
        return;
    };
    let text = std::fs::read_to_string(path).unwrap();
    let mut refused: Vec<String> = Vec::new();
    let mut total = 0;
    for line in text.lines() {
        let bytes: Vec<u8> = (0..line.len() / 2)
            .map(|i| u8::from_str_radix(&line[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        let source = String::from_utf8(bytes).unwrap();
        total += 1;
        if let Err(error) = Regex::new(&source) {
            refused.push(format!("{error:?}  {source}"));
        }
    }
    refused.sort();
    for line in &refused {
        eprintln!("{}", line.chars().take(200).collect::<String>());
    }
    eprintln!("{} of {total} patterns refused", refused.len());
}

/// Per language, how many patterns the machine refuses.
/// The patterns come from a file of `id<TAB>hex` lines that `NISHIKI_BYLANG`
/// names.
/// Patterns with back-references are left out.
/// They compile at run time.
#[test]
#[ignore = "a survey over a scratch file"]
fn survey_refusals_by_language() {
    let Some(path) = std::env::var_os("NISHIKI_BYLANG") else {
        return;
    };
    let text = std::fs::read_to_string(path).unwrap();
    let mut by: std::collections::BTreeMap<String, (usize, usize)> =
        std::collections::BTreeMap::new();
    for line in text.lines() {
        let (id, hex) = line.split_once('\t').unwrap();
        let bytes: Vec<u8> = (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        let source = String::from_utf8(bytes).unwrap();
        let entry = by.entry(id.to_owned()).or_default();
        entry.0 += 1;
        if Regex::new(&source).is_err()
            && !source.contains("\\1")
            && !source.contains("\\2")
            && !source.contains("\\3")
        {
            entry.1 += 1;
        }
    }
    let clean = by.values().filter(|v| v.1 == 0).count();
    eprintln!("{clean} of {} languages compile every pattern", by.len());
    for (id, (total, refused)) in &by {
        if *refused > 0 {
            eprintln!("  {id}: {refused} of {total}");
        }
    }
}

#[test]
fn debug_ruby_lookahead() {
    assert_eq!(find("(?=[\\&*A-Z_a-z])", " *picks"), Some((1, 1)));
    assert_eq!(find("(?![(,|\\s])", " *picks"), Some((1, 1)));
}

/// Compares the machine with Oniguruma's recorded answers.
/// `NISHIKI_DIFF` names a directory with `patterns.hex`, `lines.hex`,
/// `cases.tsv` and `expected.tsv`.
/// A scratch script writes them.
#[test]
#[ignore = "a survey over a scratch directory"]
fn differential_against_oniguruma() {
    let Some(dir) = std::env::var_os("NISHIKI_DIFF") else {
        return;
    };
    let differ = differential(std::path::Path::new(&dir));
    for case in differ.iter().take(50) {
        eprintln!("{}", case.chars().take(400).collect::<String>());
    }
    eprintln!("{} cases differ", differ.len());
}

/// Under `(?i)`, a character matches the characters and strings its case
/// folds join it to.
/// This holds in literal runs, classes, lookbehinds and back-references, as
/// Oniguruma decides.
/// The cases are in tests/engine/case-folds, written by
/// `oracle/case-fold-probe.ts`.
///
/// A line of [`Programs::Lean`] is checked through both programs.
/// One of them is the lean program the engine runs on that line.
#[test]
fn case_insensitive_matching_answers_as_oniguruma() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/engine/case-folds");
    let differ = differential(&dir);
    assert!(
        differ.is_empty(),
        "{} cases differ from Oniguruma:\n{}",
        differ.len(),
        differ.join("\n")
    );
}

/// The absent operator `(?~X)` matches as Oniguruma does.
/// The cases include both patterns of C#'s rule for `/** */` documentation
/// comments.
/// They are in tests/engine/absent, written by `oracle/absent-probe.ts`.
#[test]
fn absent_operator_answers_as_oniguruma() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/engine/absent");
    let differ = differential(&dir);
    assert!(
        differ.is_empty(),
        "{} cases differ from Oniguruma:\n{}",
        differ.len(),
        differ.join("\n")
    );
}

/// The absent operator's other forms, which start `(?~|`, do not compile.
/// Nor does the operator inside a lookbehind or inside another absent
/// operator's body, where Oniguruma's answers differ from its reading
/// elsewhere.
#[test]
fn absent_operator_refuses_what_it_does_not_read() {
    for pattern in [
        "(?~|a)",
        "(?~|a|b)",
        "(?<=(?~a))b",
        "(?<!x(?~a))b",
        "^(?~a(?~b)c)",
        "(?~(?=(?~a))b)",
    ] {
        assert!(Regex::new(pattern).is_err(), "{pattern} compiled");
    }
}

/// Oniguruma's callouts do not compile, with their own error.
/// Shiki 4.6.0 runs each of these patterns as a callout.
/// Read as a group of literals, `(*COUNT)a` would match all of `*COUNTa`.
#[test]
fn callouts_are_refused() {
    for pattern in ["(*COUNT)a", "(*FAIL)|a", "a(*MAX{1})", "(*)"] {
        assert_eq!(
            Regex::new(pattern).err(),
            Some(parse::Error(parse::CALLOUTS.to_owned())),
            "{pattern}"
        );
    }
}

/// The guards on a fold's branches never turn a fold string away.
///
/// The test takes every fold of more than one character, such as `ß` and
/// `ss` or `ﬁ` and `fi`.
/// The character that folds to it matches the string as a literal and as a
/// class.
/// A class that also holds `\w` first tries only the string's first
/// character, and still matches the string through its fold strings.
#[test]
fn every_fold_string_passes_the_guards() {
    for &(fold, chars) in UNFOLD {
        let canonical: String = fold
            .iter()
            .filter_map(|&code| char::from_u32(code))
            .collect();
        let lower: String = canonical.chars().flat_map(char::to_lowercase).collect();
        for c in chars.iter().filter_map(|&code| char::from_u32(code)) {
            for text in [&canonical, &lower] {
                let whole = Some((0, text.len()));
                assert_eq!(find(&format!("(?i){c}"), text), whole, "{c:?} on {text:?}");
                assert_eq!(
                    find(&format!("(?i)[{c}]"), text),
                    whole,
                    "[{c:?}] on {text:?}"
                );
                assert_eq!(
                    find(&format!("(?i)[\\w{c}]x"), &format!("{text}x")),
                    Some((0, text.len() + 1)),
                    "[\\w{c:?}]x on {text:?}x"
                );
            }
        }
    }
}

/// Every case of a directory of recorded answers whose match differs from
/// Oniguruma's.
/// Each names its pattern, its line, the recorded answer and the machine's.
///
/// Oniguruma reports as many groups as the pattern has.
/// Those are the groups compared.
///
/// A case also differs when the scanner's automaton finds nothing where
/// Oniguruma matches.
/// The scanner skips such text.
fn differential(dir: &std::path::Path) -> Vec<String> {
    let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap();
    let unhex = |hex: &str| -> String {
        let bytes: Vec<u8> = (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        String::from_utf8(bytes).unwrap()
    };
    let patterns: Vec<String> = read("patterns.hex").lines().map(unhex).collect();
    let lines: Vec<String> = read("lines.hex").lines().map(unhex).collect();
    let cases = read("cases.tsv");
    let expected = read("expected.tsv");
    let compiled: Vec<Option<Regex>> = patterns.iter().map(|p| Regex::new(p).ok()).collect();
    let lean: Vec<Option<Regex>> = patterns.iter().map(|p| Regex::lean(p).ok()).collect();
    let relaxed: Vec<Option<Hir>> = patterns.iter().map(|p| relaxed_of(p)).collect();
    let mut differ = Vec::new();
    let mut scratch = Scratch::default();
    for (case, want) in cases.lines().zip(expected.lines()) {
        let mut parts = case.split('\t');
        let pattern: usize = parts.next().unwrap().parse().unwrap();
        let line: usize = parts.next().unwrap().parse().unwrap();
        let from: usize = parts.next().unwrap().parse().unwrap();
        let source = &patterns[pattern];
        let text = format!("{}\n", lines[line]);
        let programs = match Programs::of(lines[line].as_bytes()) {
            Programs::Lean => [
                Some((&compiled[pattern], &relaxed[pattern], "full")),
                Some((&lean[pattern], &relaxed[pattern], "lean")),
            ],
            Programs::Full => [Some((&compiled[pattern], &relaxed[pattern], "full")), None],
        };
        for (program, relaxed, kind) in programs.into_iter().flatten() {
            let Some(regex) = program else {
                differ.push(format!("{source} does not compile as {kind}"));
                continue;
            };
            if want != "-"
                && let Some(relaxed) = relaxed
                && !regex_automata::meta::Regex::builder()
                    .build_from_hir(relaxed)
                    .is_ok_and(|automaton| automaton.is_match(&text.as_bytes()[from..]))
            {
                differ.push(format!(
                    "{source} on {:?} from {from}: the automaton {relaxed} finds nothing",
                    lines[line]
                ));
            }
            let mut steps = 1_000_000_000;
            let context = Context {
                search: Some(from),
                text_start: true,
            };
            let found = regex.find(&mut scratch, text.as_bytes(), from, context, &mut steps);
            let got = if found {
                (0..=regex.groups())
                    .map(|g| {
                        scratch
                            .group(g)
                            .map_or_else(|| "x".to_owned(), |(s, e)| format!("{s},{e}"))
                    })
                    .collect::<Vec<_>>()
                    .join(";")
            } else {
                "-".to_owned()
            };
            let want_cut: Vec<&str> = want.split(';').collect();
            let got_cut: Vec<&str> = got.split(';').collect();
            let same = want == got
                || (want_cut.len() <= got_cut.len()
                    && want_cut[..] == got_cut[..want_cut.len()]
                    && got_cut[want_cut.len()..].iter().all(|g| *g == "x"));
            if !same {
                differ.push(format!(
                "{source} on {:?} from {from}: Oniguruma {want}, nishiki's {kind} program {got}",
                lines[line]
            ));
            }
        }
    }
    differ
}

/// `a(?i)b|c` is `a(?i:b|c)`: the `a` comes before either alternative.
#[test]
fn flags_without_a_colon_hold_across_the_alternatives_after_them() {
    assert_eq!(find("a(?i)b|c", "xc"), None);
    assert_eq!(find("a(?i)b|c", "aC"), Some((0, 2)));
    assert_eq!(find("(?<=x)(?i)do|\\n", "ab\n"), None);
    assert_eq!(find("(?<=x)(?i)do|\\n", "xDO"), Some((1, 3)));
    assert_eq!(find("(?i)ab|cd", "CD"), Some((0, 2)));
}

#[test]
fn a_group_repeated_no_times_can_still_be_called() {
    assert_eq!(find("a\\g<n>(?<n>b+){0}", "abbb"), Some((0, 4)));
    assert_eq!(
        search("x(?<n>[0-9]+){0}\\g<n>-\\g<n>", "x12-345", 0, None).and_then(|groups| groups[1]),
        Some((4, 7)),
        "a call sets the group it calls"
    );
}

/// A failed attempt leaves no group set for the attempt that matches: each
/// search clears the groups once, and a failure undoes what it captured.
#[test]
fn a_failed_attempt_leaves_no_group_behind() {
    assert_eq!(
        search("(a)(b)?c", "abxac", 0, None),
        Some(vec![Some((3, 5)), Some((3, 4)), None])
    );
    assert_eq!(
        search("(?=(a)b)ab|(x)c", "axxc", 0, None),
        Some(vec![Some((2, 4)), None, Some((2, 3))])
    );
}

/// The characters beyond ASCII in the orbit of a character that folds to a
/// string, or of a character of such a string.
/// `MULTI` lists them.
fn wide_characters() -> BTreeSet<char> {
    let mut wide = BTreeSet::new();
    for &(code, fold) in MULTI {
        for c in std::iter::once(code)
            .chain(fold.iter().copied())
            .filter_map(char::from_u32)
        {
            wide.extend(
                orbit(c)
                    .members()
                    .iter()
                    .copied()
                    .chain(std::iter::once(c))
                    .filter(|m| !m.is_ascii()),
            );
        }
    }
    wide
}

/// The lines of every UTF-8 lead byte classify as their characters do:
/// full exactly when one is a wide fold character.
#[test]
fn every_lead_byte_classifies() {
    let wide = wide_characters();
    for line in crate::textmate::tests::lines_beyond_ascii() {
        let expected = if line.chars().any(|c| wide.contains(&c)) {
            Programs::Full
        } else {
            Programs::Lean
        };
        assert_eq!(Programs::of(line.as_bytes()), expected, "{line:?}");
    }
}

/// A line is full exactly when it holds a character beyond ASCII, from one of
/// two orbit kinds, as `MULTI` lists them:
///
/// - the orbit of a character that folds to a string;
/// - the orbit of a character of such a string.
///
/// Every character beyond ASCII is checked alone.
/// The check shows that the table of first bytes, and of the bytes after
/// them, stands for that set.
/// For example, `é` shares `ß`'s first byte and stays lean.
#[test]
fn a_line_is_full_exactly_when_it_holds_a_wide_fold_character() {
    let wide = wide_characters();
    let mut buffer = [0; 4];
    for c in (0x80..=0x10_ffff).filter_map(char::from_u32) {
        let expected = if wide.contains(&c) {
            Programs::Full
        } else {
            Programs::Lean
        };
        assert_eq!(
            Programs::of(c.encode_utf8(&mut buffer).as_bytes()),
            expected,
            "{c:?}"
        );
    }
    assert_eq!(Programs::of("café".as_bytes()), Programs::Lean);
    assert_eq!(Programs::of("class first".as_bytes()), Programs::Lean);
    assert_eq!(Programs::of("straße".as_bytes()), Programs::Full);
    assert_eq!(Programs::of("the ﬁle".as_bytes()), Programs::Full);
    assert_eq!(Programs::of(b"x\xc3"), Programs::Lean);
}

/// A lean program leaves out the alternatives that need a wide character, and
/// keeps the rest.
#[test]
fn lean_programs_leave_out_the_wide_alternatives() {
    let lean = |pattern: &str, text: &str| {
        let regex = Regex::lean(pattern).unwrap_or_else(|e| panic!("{pattern}: {e:?}"));
        let mut scratch = Scratch::default();
        let mut steps = 100_000_000;
        let context = Context {
            search: None,
            text_start: true,
        };
        regex
            .find(&mut scratch, text.as_bytes(), 0, context, &mut steps)
            .then(|| scratch.group(0))
            .flatten()
    };
    assert_eq!(lean("(?i)ss", "SS"), Some((0, 2)));
    assert_eq!(lean("(?i)ss", "ß"), None);
    assert_eq!(find("(?i)ss", "ß"), Some((0, 2)));
    assert_eq!(lean("(?i)[\\w]+", "abc"), Some((0, 3)));
    assert_eq!(lean("(?i)fi", "ﬁ"), None);
    assert_eq!(find("(?i)fi", "ﬁ"), Some((0, 3)));
}

/// The relaxed form cuts a sequence at its first positive lookahead.
/// The lookahead's pattern takes the place of the lookahead and the rest.
/// Cutting follows groups and atomic groups from the root.
#[test]
fn the_relaxed_form_ends_with_the_first_positive_lookahead() {
    let cases = [
        ("a(?=bc)d", "abc", "abx"),
        ("(a(?=b))", "ab", "ac"),
        ("(?>p(?=q))", "pq", "pr"),
        ("(?=ab)c", "ab", "ac"),
        ("a(?!b)c", "ac", "ab"),
    ];
    for (pattern, matched, missed) in cases {
        let relaxed =
            relaxed_of(pattern).unwrap_or_else(|| panic!("{pattern} has no relaxed form"));
        let automaton = regex_automata::meta::Regex::builder()
            .build_from_hir(&relaxed)
            .unwrap_or_else(|e| panic!("{pattern}: {e}"));
        assert!(automaton.is_match(matched), "{pattern} misses {matched}");
        assert!(!automaton.is_match(missed), "{pattern} matches {missed}");
    }
}

/// A pattern whose relaxed form can match no text has no relaxed form.
/// A pattern with an empty class keeps one, as a class counts one character.
#[test]
fn a_relaxed_form_needs_a_character() {
    for pattern in ["(?<=a)", "^", "\\b", "(?=)", "a*"] {
        assert!(
            relaxed_of(pattern).is_none(),
            "{pattern} has a relaxed form"
        );
    }
    for pattern in ["a", "(?<=a)b", "[^\\x{0}-\\x{10ffff}]", "a+"] {
        assert!(
            relaxed_of(pattern).is_some(),
            "{pattern} has no relaxed form"
        );
    }
}

/// The relaxed form of `pattern` as Oniguruma reads it, from its sketch.
fn relaxed_of(pattern: &str) -> Option<Hir> {
    Sketch::oniguruma(pattern).ok()?.relaxed().cloned()
}
