use super::treats_wide_spaces_as_spaces;

/// Whether `regex`, as a match or a begin, treats a wide space as a space.
fn alike(regex: &str) -> bool {
    treats_wide_spaces_as_spaces(regex, false)
}

/// A pattern that takes any blank treats a wide space as a space.
#[test]
fn a_pattern_that_takes_any_blank_treats_a_wide_space_as_a_space() {
    assert!(alike(r"\s*(//)"));
    assert!(alike("[[:space:]]+x"));
    assert!(alike(r"^\s*$"));
}

/// A pattern that takes spaces only fails on a wide space.
#[test]
fn a_pattern_that_takes_spaces_only_fails_on_a_wide_space() {
    assert!(!alike(r"[ \t]+$"));
    assert!(!alike(r"^[\t ]+"));
    assert!(!alike(" {2,}$"));
}

/// A class that holds only some wide spaces fails on the others.
#[test]
fn a_class_that_holds_only_some_wide_spaces_fails_on_the_others() {
    assert!(!alike(r"[^\x00-\x7F]+"));
    assert!(!alike(r"[\x{80}-\x{10FFFF}]"));
}

/// What follows a character that leaves the run is not read.
#[test]
fn what_follows_a_character_that_leaves_the_run_is_not_read() {
    assert!(alike(r"\s*x[ \t]"));
    assert!(!alike(r"(?:\s|a) "));
}

/// A lookbehind is read in full, wherever it stands.
#[test]
fn a_lookbehind_is_read_in_full_wherever_it_stands() {
    assert!(!alike("(?<= )x"));
    assert!(!alike("x(?<=[ ]x)"));
    assert!(alike(r"(?<=\S)x"));
}

/// A back reference to a group with no blank repeats the same text.
#[test]
fn a_back_reference_to_a_group_with_no_blank_repeats_the_same_text() {
    assert!(alike(r#"(["'])(?:\\.|[^\\])*?\1"#));
    assert!(alike(r"(a)x\1"));
}

/// A back reference that can repeat a blank fails the pattern.
#[test]
fn a_back_reference_that_can_repeat_a_blank_fails_the_pattern() {
    assert!(!alike(r"(\s)x\1"));
    assert!(!alike(r"(a\1)"));
    assert!(!alike(r"\1"));
}

/// A named reference to a group with no blank repeats the same text.
#[test]
fn a_named_reference_to_a_group_with_no_blank_repeats_the_same_text() {
    assert!(alike(r"(?<q>'?)x\k<q>"));
    assert!(!alike(r"(?<q>\s?)x\k<q>"));
}

/// A call passes once the match has left the run.
#[test]
fn a_call_passes_once_the_match_has_left_the_run() {
    assert!(alike(r"(?<b>\[(?:[^]\[]|\g<b>)*])"));
}

/// In an end or a while pattern, a back reference repeats the begin's text
/// and fails.
#[test]
fn a_back_reference_that_repeats_the_begin_fails() {
    assert!(!treats_wide_spaces_as_spaces(r"(a)x\1", true));
}

/// A call inside the run fails the pattern, and so does a pattern the parser
/// refuses.
#[test]
fn a_call_inside_the_run_or_a_refused_pattern_fails() {
    assert!(!alike(r"(?<n>\s)\g<n>"));
    assert!(!alike("(a"));
}

/// A class intersection is read as the engine reads it.
/// `[a&&b]` matches no character, so it reads no blank.
#[test]
fn a_class_intersection_is_read_as_the_engine_reads_it() {
    assert!(alike("[a&&b]"));
    assert!(!alike(r"[\s&&[^\x{3000}]]"));
}

/// Extended mode skips its blanks and comments.
#[test]
fn extended_mode_skips_its_blanks_and_comments() {
    assert!(alike(
        r"(?x) \s* # a comment
 x"
    ));
    assert!(!alike(r"(?x: \s* ) "));
}
