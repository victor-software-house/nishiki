use super::{fill_backrefs, fill_name};

/// A back-reference is a `\` and every digit after it, as vscode-textmate
/// reads it.
/// A `\` before it still leaves it a reference.
/// `\0` is the whole match, and a group the begin lacks writes nothing.
#[test]
fn a_backreference_reads_every_digit() {
    let text = b"ab";
    let mut caps = vec![Some((0, 2)), Some((0, 1))];
    caps.resize(11, None);
    caps[10] = Some((1, 2));
    let mut out = String::new();
    fill_backrefs(r"\1|\10|\0|\9|\w", text, &caps, &mut out);
    assert_eq!(out, r"a|b|ab||\w");
    fill_backrefs(r"\\1", text, &caps, &mut out);
    assert_eq!(out, r"\a");
}

/// A group's text is escaped where JavaScript's `\s` matches, as
/// vscode-textmate escapes it, and nowhere else.
#[test]
fn a_backreference_escapes_javascript_white_space() {
    let text = "a\u{a0}\u{feff}\u{85}.".as_bytes();
    let caps = [Some((0, text.len())), Some((0, text.len()))];
    let mut out = String::new();
    fill_backrefs(r"\1", text, &caps, &mut out);
    assert_eq!(out, "a\\\u{a0}\\\u{feff}\u{85}\\.");
}

/// A `$` form names a group of the pattern that matched.
/// A group the pattern does not have keeps the form as written, as in
/// vscode-textmate.
/// Any group that took no part writes nothing.
#[test]
fn a_dollar_form_without_its_group_stays_as_written() {
    let text = b"namespace x";
    let caps = [Some((0, 11)), Some((0, 9)), None];
    let mut out = String::new();
    fill_name("keyword.control.namespace.$1.objcpp", text, &caps, &mut out);
    assert_eq!(out, "keyword.control.namespace.namespace.objcpp");
    fill_name("keyword.$2.x", text, &caps, &mut out);
    assert_eq!(out, "keyword..x");
    fill_name("keyword.control.namespace.$3.objcpp", text, &caps, &mut out);
    assert_eq!(out, "keyword.control.namespace.$3.objcpp");
    fill_name("keyword.${3:/downcase}.x", text, &caps, &mut out);
    assert_eq!(out, "keyword.${3:/downcase}.x");
}
