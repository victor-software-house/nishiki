use super::fill_name;

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
