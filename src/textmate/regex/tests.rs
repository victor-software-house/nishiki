use super::{Context, Regex, Scratch};

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

#[test]
fn anchors_and_search_position() {
    assert!(search("(^|\\G)ab", "ab", 0, None).is_some());
    assert!(search("(^|\\G)ab", "xab", 0, None).is_none());
    assert!(search("(^|\\G)ab", "xab", 0, Some(1)).is_some());
    assert_eq!(find("a$", "a\n"), Some((0, 1)));
    assert_eq!(find("^b", "a\nb"), Some((2, 3)));
    // `^` does not match after the final newline.
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
