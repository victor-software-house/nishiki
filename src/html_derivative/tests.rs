use crate::Language;
use crate::test_support::{leaf, scopes_of};

/// A tag that no HTML rule knows is html-derivative's own, not an invalid
/// tag, and its name is a plain tag name.
#[test]
fn an_unknown_tag_is_the_grammar_s_own() {
    for (text, name) in [("<Foo/>", "Foo"), ("<x:y>", "x:y"), ("</Bar >", "Bar")] {
        assert_eq!(
            scopes_of(Language::HtmlDerivative, text, name),
            [
                "text.html.derivative",
                "meta.tag.other.unrecognized.html.derivative",
                "entity.name.tag.html"
            ],
            "{text}"
        );
    }
}

/// The tags HTML knows keep HTML's rules: a custom element stays one.
#[test]
fn a_custom_element_stays_custom() {
    assert_eq!(
        scopes_of(Language::HtmlDerivative, "<foo-bar x=\"1\">", "foo-bar")[1],
        "meta.tag.custom.start.html"
    );
}

/// Its tags read HTML's attribute list, so `onclick` is an event handler.
#[test]
fn its_tags_read_html_attributes() {
    assert_eq!(
        scopes_of(
            Language::HtmlDerivative,
            "<Foo onclick=\"go()\">",
            "onclick"
        )[2],
        "meta.attribute.event-handler.click.html"
    );
}

/// The grammar's own injection flags a bare `<` in a document of its own.
#[test]
fn a_bare_less_than_is_flagged() {
    assert_eq!(
        leaf(Language::HtmlDerivative, "a < b", "<"),
        "invalid.illegal.bad-angle-bracket.html"
    );
}

/// A tag name stops at a `<`, which HTML's rule for invalid tags reads on.
#[test]
fn a_name_stops_at_a_less_than() {
    assert_eq!(
        leaf(Language::HtmlDerivative, "<x:y<b>", "x:y"),
        "entity.name.tag.html"
    );
}

/// In html-derivative the grammar's own tag rule compiles HTML's attribute
/// rule first.
/// A tag HTML does not know therefore reads HTML's attribute list.
/// That holds even inside a template, where HTML's grammar reads the `math`
/// list.
#[test]
fn an_unknown_tag_reads_html_attributes_everywhere() {
    let text = "<script type=\"text/x-template\"><Foo class=\"x\"></script>";
    assert_eq!(
        scopes_of(Language::HtmlDerivative, text, "class")[4],
        "meta.attribute.class.html"
    );
    assert_eq!(
        scopes_of(Language::Html, text, "class")[4],
        "meta.attribute.unrecognized.class.html"
    );
}
