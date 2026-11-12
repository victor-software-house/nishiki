use nishiki::unstable::codegen::starts::{
    BackrefStart, Start, can_match_empty, start, takes_one_blank,
};

/// Where a match of `regex` can start, with back references at a quote.
fn quoted(regex: &str) -> Start {
    start(regex, BackrefStart::Quote)
}

/// Where a match of `regex` can start, with back references at any byte.
fn any(regex: &str) -> Start {
    start(regex, BackrefStart::Any)
}

/// Whether some alternative of `regex` takes one blank, then a non-blank.
fn one_blank(regex: &str) -> bool {
    takes_one_blank(regex, BackrefStart::Quote)
}

/// A single blank, then a character that is not a blank, takes one blank.
#[test]
fn a_single_blank_then_a_character_that_is_not_a_blank_takes_one_blank() {
    assert!(one_blank(r"\s-\s"));
    assert!(!one_blank(r"\s+x"));
}

/// A line feed after the blank counts as a character that is not a blank.
#[test]
fn a_line_feed_after_the_blank_counts_as_a_character_that_is_not_a_blank() {
    assert!(one_blank(r".\n"));
}

/// A negative lookbehind whose body cannot match a blank is stepped over.
#[test]
fn a_negative_lookbehind_whose_body_cannot_match_a_blank_is_stepped_over() {
    assert!(one_blank(r"(\)|(?<!\.\.\.).\n)"));
}

/// A negative lookbehind whose body can match a blank is not stepped over.
#[test]
fn a_negative_lookbehind_whose_body_can_match_a_blank_is_not_stepped_over() {
    assert!(!one_blank(r"(?<! ).\n"));
    assert!(!one_blank(r"(?<!\s\s).\n"));
    assert!(!one_blank(r"(?<!(a)\1).\n"));
}

/// Each alternative of a leading group runs on into the rest.
#[test]
fn a_leading_groups_alternatives_each_run_on_with_the_rest() {
    assert!(one_blank(r"(;|(^|\s)#\s).*$"));
    assert!(!one_blank(r"(;|#)\s.*$"));
}

/// A quantified leading group does not distribute.
#[test]
fn a_quantified_leading_group_does_not_distribute() {
    assert!(!one_blank(r"(a|\s)+x"));
}

/// A leading group with an anchor counts only through its blank branch.
#[test]
fn a_leading_group_with_an_anchor_counts_only_through_its_blank_branch() {
    assert!(one_blank(r"(^|\s)#\s"));
    assert!(!one_blank(r"(^|a)#\s"));
}

/// A match of only a lookahead, or an optional atom, can match the empty text.
#[test]
fn a_lookahead_or_an_optional_atom_can_match_the_empty_text() {
    assert!(can_match_empty(r"(;)|(?=[)}])"));
    assert!(can_match_empty("a?"));
    assert!(!can_match_empty(r"(;)|[)}]"));
    assert!(!can_match_empty(r"\b\w+"));
}

/// An anchor, a negative lookahead or an optional atom can match where no
/// byte follows.
#[test]
fn an_anchor_a_negative_lookahead_or_an_optional_atom_can_match_at_the_end() {
    for regex in [
        "$",
        r"\z",
        "(?=$)",
        "(?!x)",
        "a?",
        r"(;)|(?=[)}]|$)",
        r"(\w*)\1",
    ] {
        assert!(quoted(regex).matches_at_end, "{regex}");
    }
}

/// A lookahead that needs a byte cannot match where no byte follows.
#[test]
fn a_lookahead_that_needs_a_byte_cannot_match_at_the_end() {
    for regex in ["(?=<<)", r"(?=\n)", "(?=[)}])", r"\b\w+", r"(?<=\{)(\s+)"] {
        assert!(!quoted(regex).matches_at_end, "{regex}");
    }
}

/// A lookbehind that needs a byte cannot match at the start of the line.
#[test]
fn a_lookbehind_that_needs_a_byte_cannot_match_at_the_line_start() {
    for regex in [r"(?<=\.)foo", r"(?<=\w)x", "(?<=ab)x", "(?:(?<=,)|(?<=;))x"] {
        assert!(!quoted(regex).places.line_start, "{regex}");
    }
    for regex in [r"(?<=^|\s)x", r"(?<=\.)?x", r"x|(?<=\.)y", r"(?<!\.)x"] {
        assert!(quoted(regex).places.line_start, "{regex}");
    }
}

/// A lookbehind of one character with no word character cannot match inside
/// a word.
#[test]
fn a_lookbehind_of_one_non_word_character_cannot_match_inside_a_word() {
    for regex in [
        r"(?<=\.)foo",
        r"(?<=[,(])\s*x",
        r"(?<=\{)(\s+)",
        r"(?<=\W)x",
    ] {
        assert!(!quoted(regex).places.in_word, "{regex}");
    }
    for regex in [r"(?<=\w)x", "(?<=[a.])x", "(?<=..)x", "(?<=.)x"] {
        assert!(quoted(regex).places.in_word, "{regex}");
    }
}

/// A lookbehind or `\b` after an item that can take the next byte leaves the
/// start free.
#[test]
fn a_lookbehind_or_a_boundary_after_an_item_that_can_take_a_byte_leaves_the_start_free() {
    assert!(quoted("x?(?<=x)y").places.line_start);
    assert!(!quoted(r"\s*(?<=\.)x").places.in_word);
    assert!(quoted(r"\s*(?<=\.)x").places.line_start);
    assert!(!quoted(r"\s*\bx").places.in_word);
    assert!(quoted(r"a?\bx").places.in_word);
    assert!(!quoted(r"(?<=\.)\s*x").places.line_start);
}

/// A back reference or a call that can match empty can match where no byte
/// follows.
#[test]
fn a_back_reference_or_a_call_that_can_match_empty_can_match_at_the_end() {
    for regex in [
        r"(\w*)\1",
        r"(?<n>a*)\k<n>",
        r"(?<n>a*)\g<n>",
        r"(a*)(?=\1)",
    ] {
        assert!(any(regex).matches_at_end, "{regex}");
    }
}

/// A back reference in a lookahead that needs a byte cannot match at the end.
#[test]
fn a_back_reference_in_a_lookahead_that_needs_a_byte_cannot_match_at_the_end() {
    assert!(!any(r#"(?=(?><<[-~]("?)(SQL)\b\1))"#).matches_at_end);
}

/// A back reference behind a byte the match needs cannot match where no byte
/// follows.
#[test]
fn a_back_reference_behind_a_needed_byte_cannot_match_at_the_end() {
    assert!(!any(r"<(\w*)\1").matches_at_end);
}

/// A back reference that can repeat an empty capture can match at the end.
#[test]
fn a_back_reference_that_can_repeat_an_empty_capture_can_match_at_the_end() {
    for regex in [r"(a?)\1", r"(?:x|())\1", r"(?=(a?))\1"] {
        assert!(any(regex).matches_at_end, "{regex}");
    }
}

/// Every form of a call or a back reference keeps a match where no byte
/// follows.
#[test]
fn every_form_of_a_call_or_a_back_reference_keeps_a_match_at_the_end() {
    for regex in [
        r"(?<n>a*)\g'n'",
        r"(?<n>a*)\k'n'",
        r"(a*)\g<-1>",
        r"(a*)\g<1>",
        r"a*\g<0>",
    ] {
        assert!(any(regex).matches_at_end, "{regex}");
    }
}

/// A call of the group it sits in is not followed.
#[test]
fn a_call_of_the_group_it_sits_in_is_not_followed() {
    assert!(!any(r"(?<n>a\g<n>?b)").matches_at_end);
    assert!(any(r"(?<n>a\g<n>|)").matches_at_end);
}

/// A `\G` lets a match start anywhere only before the first item that
/// consumes a byte.
#[test]
fn a_last_match_anchor_frees_the_start_only_before_the_first_consumed_byte() {
    assert_eq!(any(r"\Gx").bytes, [u64::MAX; 4]);
    assert_eq!(any(r"x?\Gy").bytes, [u64::MAX; 4]);
    let hack = any(r"(?i)(\\)?\bG(ender\Gender|lobIterator)\b");
    assert_ne!(hack.bytes, [u64::MAX; 4]);
    assert!(!hack.matches_at_end);
    assert!(!any(r"a\Gb").matches_at_end);
    assert!(any(r"x?\G$").matches_at_end);
}

/// A named back reference counts as empty where a numbered one does.
#[test]
fn a_named_back_reference_counts_as_empty_where_a_numbered_one_does() {
    assert!(any(r"(?<n>a?)\k<n>").matches_at_end);
    assert!(!any(r"(?=<<(?<n>x?)\k<n>)").matches_at_end);
}

/// An item before `\b` is safe in a word unless a match starts there and
/// none earlier.
#[test]
fn an_item_before_a_boundary_is_safe_in_a_word_unless_no_earlier_start_exists() {
    assert!(quoted(r"a?\b-").places.in_word);
    assert!(quoted(r"(?:a\b)?-").places.in_word);
    assert!(
        !quoted(r"(?:([-:\\_\p{L}\d])*\\)?\b(?i:Add|Get)")
            .places
            .in_word
    );
    let names = r"(')?((?:\b[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*\.)*)\b([\p{Lu}\p{Lt}])";
    assert!(!quoted(names).places.in_word);
    assert!(!quoted(r"[\w-]*\bx").places.in_word);
    assert!(any(r"(\w*)\1\bx").places.in_word);
    assert!(quoted(r"\G[\w-]*\bx").places.in_word);
}

/// A pattern kept out of words for an earlier start is tried where the scan
/// starts.
#[test]
fn a_pattern_kept_out_of_words_for_an_earlier_start_is_tried_where_the_scan_starts() {
    assert!(quoted(r"(?:([-:\\_\p{L}\d])*\\)?\b(?i:Add|Get)").in_word_first);
    assert!(!quoted(r"\s*\bx").in_word_first);
    let names = r"(')?((?:\b[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*\.)*)\b([\p{Lu}\p{Lt}])";
    assert!(!quoted(names).in_word_first);
    assert!(!quoted(r"a?\b-").in_word_first);
}

/// A run after a lookaround cannot move a match inside a word to the word's
/// start.
#[test]
fn a_run_after_a_lookaround_cannot_move_a_match_to_the_words_start() {
    for regex in [
        r"(?=b)\w*\b-",
        r"(?<=a)\w*\b-",
        r"(?=b)(?:\w*)\b-",
        r"(?:(?=b)|-)\w*\b-",
        r"(?:-?(?=b))\w*\b-",
    ] {
        assert!(any(regex).places.in_word, "{regex}");
    }
    for regex in [r"-?\w*\b-", r"\w*\b-", r"(?:\w*)\b-"] {
        assert!(!any(regex).places.in_word, "{regex}");
    }
}

/// A reversed interval such as `{1,0}` is an optional repeat, as Oniguruma
/// reads it.
#[test]
fn a_reversed_interval_is_an_optional_repeat() {
    let comment = quoted(r"^\s+{1,0}(//[!/]+)");
    assert_ne!(comment.bytes[0] & (1 << b'/'), 0);
    assert_ne!(comment.bytes[0] & (1 << b' '), 0);
}

/// A range above ASCII starts with its own lead bytes and no others.
#[test]
fn a_range_above_ascii_starts_with_its_own_lead_bytes() {
    assert_eq!(quoted("[à-ÿ]").bytes, [0, 0, 0, 1 << (0xc3 - 0xc0)]);
    let names = quoted(r"[_\x7F-\x{10FFFF}]");
    assert_eq!(names.bytes[2], 0);
    assert_eq!(names.bytes[3], (1 << (0xf5 - 0xc0)) - (1 << (0xc2 - 0xc0)));
}

/// Under `(?i)`, a lookbehind class that holds `\w` still keeps the match
/// out of words.
#[test]
fn an_insensitive_lookbehind_that_holds_a_word_class_keeps_out_of_words() {
    assert!(!quoted(r"(?i)(?<![-\w])ab").places.in_word);
    assert!(!quoted(r"(?<![-\w])ab").places.in_word);
}
